//! The Darian calendar for Mars, after Thomas Gangale (1985, revised 1998 and
//! 2006).
//!
//! # The shape of it
//!
//! * 24 months, alternating a Latin zodiac name with its Sanskrit counterpart:
//!   Sagittarius, Dhanus, Capricornus, Makara, and so on round to Vrishika.
//! * Four quarters of six months. The first five months of a quarter have 28
//!   sols and the sixth has 27, giving 4 × 167 = **668 sols**; in a leap year
//!   the final month, Vrishika, keeps its 28th sol and the year is **669**.
//! * Seven-sol weeks named for the same planets as the terrestrial week — Sol
//!   Solis, Sol Lunae, Sol Martis, Sol Mercurii, Sol Jovis, Sol Veneris, Sol
//!   Saturni. A 28-sol month is exactly four weeks, so **every month begins on
//!   Sol Solis**, and a 27-sol month simply has no final Sol Saturni. The
//!   weekday is therefore a function of the day of the month alone, which is
//!   the calendar's neatest property and the reason it has no week that
//!   straddles a month boundary.
//! * Leap rule: a year is long if it is **odd**, or **divisible by 10**,
//!   except that years divisible by 100 are not long unless they are also
//!   divisible by 500. Gangale writes the count of long years up to `Y` as
//!   `(Y−1)\2 + Y\10 − Y\100 + Y\500`. That gives 296 long years per 500, a
//!   mean year of **668.592 sols** against Mars's tropical year of 668.5921 —
//!   an error of about one sol in ten thousand Mars years.
//!
//!   Some secondary sources state the last term as `Y\1000`, giving 668.591;
//!   this module follows the `\500` form published on Gangale's own site,
//!   which is the better fit.
//! * Epoch: the northern spring equinox of **1609**, chosen for the telescopic
//!   era. Year 0 sol 1 — 1 Sagittarius 0 — is the sol at Mars Sol Date
//!   **−94 129**, so Darian year `Y` corresponds to Clancy Mars Year `Y − 183`.
//!
//! # Why this calendar does not use `Rd` as other calendars do
//!
//! [`hc_calendar::Calendar`] pivots every conversion through [`Rd`], the Rata
//! Die fixed **day** number, and a Rata Die day is an Earth day. A sol is 2.75%
//! longer. There is no honest way to map 1 Sagittarius 214 onto an Earth day:
//! the Darian year is 668.592 sols but 687 Earth days, so any such map either
//! loses sols or invents them, and the round-trip contract that
//! [`hc_calendar::Calendar`] requires could not hold.
//!
//! So this module does the other thing. Its own conversions are **sol-indexed**
//! throughout — [`sol_from_date`], [`date_from_sol`] and
//! [`DarianCalendar::date_at`] all speak in sols, and that is the API to use.
//! The [`hc_calendar::Calendar`] implementation is provided so that a Darian
//! date can travel through the registry, the formatters and the FFI boundary
//! like any other calendar's, and in it **the `Rd` is a Darian sol number, not
//! an Earth day**. Both round-trip contracts then hold exactly, because the
//! pivot is a day count on the body the calendar belongs to.
//!
//! The consequence is a real footgun and is stated here rather than buried: an
//! `Rd` obtained from this calendar must never be handed to a terrestrial
//! calendar, and [`hc_calendar::Calendar::convert_to`] across the Earth–Mars
//! boundary will produce nonsense. To cross that boundary, go through
//! [`sol_mars_sol_date`] and [`super::MarsMoment`], which know what a sol is.
//!
//! Source: Gangale, T., "The Architecture of Time, Part 2: The Darian System
//! for Mars", SAE 2006-01-2249, and the calendar description at
//! `ops-alaska.com/time/gangale_mst/darian.htm`.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_core::math::floor;

use super::MarsMoment;

/// The 24 Darian month names, in order.
pub const MONTH_NAMES: [&str; 24] = [
    "Sagittarius",
    "Dhanus",
    "Capricornus",
    "Makara",
    "Aquarius",
    "Kumbha",
    "Pisces",
    "Mina",
    "Aries",
    "Mesha",
    "Taurus",
    "Rishabha",
    "Gemini",
    "Mithuna",
    "Cancer",
    "Karka",
    "Leo",
    "Simha",
    "Virgo",
    "Kanya",
    "Libra",
    "Tula",
    "Scorpius",
    "Vrishika",
];

/// The seven Darian weekday names, in order, starting with the one that
/// begins every month.
pub const WEEKDAY_NAMES: [&str; 7] = [
    "Sol Solis",
    "Sol Lunae",
    "Sol Martis",
    "Sol Mercurii",
    "Sol Jovis",
    "Sol Veneris",
    "Sol Saturni",
];

/// The Mars Sol Date of Darian sol 0, which is 1 Sagittarius 0: the Airy
/// midnight beginning the sol of the northern spring equinox of 1609.
pub const DARIAN_EPOCH_MARS_SOL_DATE: i64 = -94_129;

/// The offset between Darian years and Clancy Mars years.
///
/// Darian year 0 is Mars Year −183, so `mars_year = darian_year − 183`.
pub const DARIAN_YEAR_MINUS_MARS_YEAR: i64 = 183;

/// Sols in a common Darian year.
pub const COMMON_YEAR_SOLS: u16 = 668;

/// Sols in a long Darian year.
pub const LEAP_YEAR_SOLS: u16 = 669;

/// The earliest year this implementation converts.
///
/// The bound is arbitrary but generous — it is about 1.9 million Earth years
/// either side of the epoch — and exists so that `668 * year` cannot overflow
/// and so that a nonsense year is rejected rather than wrapped.
pub const MIN_YEAR: i64 = -1_000_000;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 1_000_000;

/// A date in the Darian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DarianDate {
    /// The Darian year. Negative years are before the 1609 epoch; there is no
    /// year-zero gap, because year 0 is a real year.
    pub year: i64,
    /// The month, in `1..=24`.
    pub month: u8,
    /// The sol of the month, in `1..=28`.
    pub day: u8,
}

impl DarianDate {
    /// A Darian date, unchecked; [`sol_from_date`] is what validates it.
    #[must_use]
    pub const fn new(year: i64, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// The name of this date's month.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] when the month is not in
    /// `1..=24`.
    pub fn month_name(&self) -> CalendarResult<&'static str> {
        MONTH_NAMES
            .get(self.month as usize - 1)
            .copied()
            .ok_or(CalendarError::MonthOutOfRange)
    }

    /// The name of this date's weekday.
    ///
    /// Every Darian month begins on Sol Solis, so the weekday follows from the
    /// sol of the month with no reference to any continuous count.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when the day is not in
    /// `1..=28`.
    pub fn weekday_name(&self) -> CalendarResult<&'static str> {
        if self.day == 0 || self.day > 28 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(WEEKDAY_NAMES[(self.day as usize - 1) % 7])
    }

    /// The Clancy Mars year this Darian year corresponds to.
    ///
    /// The two counts run in step because both are anchored on an `Ls = 0`
    /// crossing; they differ only in which one they call the first.
    #[must_use]
    pub const fn mars_year(&self) -> i64 {
        self.year - DARIAN_YEAR_MINUS_MARS_YEAR
    }
}

/// Whether `year` is a long Darian year of 669 sols.
///
/// Odd, or divisible by ten; but not if divisible by a hundred, unless also
/// divisible by five hundred.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    if year.rem_euclid(2) != 0 {
        return true;
    }
    if year.rem_euclid(10) != 0 {
        return false;
    }
    if year.rem_euclid(100) != 0 {
        return true;
    }
    year.rem_euclid(500) == 0
}

/// Gangale's count of long years in `1..=year`, `(Y−1)\2 + Y\10 − Y\100 +
/// Y\500`, extended to negative years with floor division so that the
/// difference of two counts is always the number of long years between them.
const fn long_years_through(year: i64) -> i64 {
    (year - 1).div_euclid(2) + year.div_euclid(10) - year.div_euclid(100) + year.div_euclid(500)
}

/// The number of long years in `0..year`, i.e. strictly before `year`.
const fn long_years_before(year: i64) -> i64 {
    // `long_years_through(-1)` is -2; subtracting it rebases the count on
    // year 0, which is itself a long year.
    long_years_through(year - 1) + 2
}

/// The number of sols in `year`.
#[must_use]
pub const fn sols_in_year(year: i64) -> u16 {
    if is_leap_year(year) {
        LEAP_YEAR_SOLS
    } else {
        COMMON_YEAR_SOLS
    }
}

/// The number of sols in `month` of `year`, or `None` when the month is not in
/// `1..=24`.
#[must_use]
pub const fn sols_in_month(year: i64, month: u8) -> Option<u8> {
    if month == 0 || month > 24 {
        return None;
    }
    if !month.is_multiple_of(6) {
        return Some(28);
    }
    if month == 24 && is_leap_year(year) {
        Some(28)
    } else {
        Some(27)
    }
}

/// The Darian sol number of 1 Sagittarius of `year`.
#[must_use]
pub const fn year_start_sol(year: i64) -> i64 {
    668 * year + long_years_before(year)
}

/// The Darian sol number of a date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`] when
/// the date does not exist — including 27 Vrishika's missing 28th sol in a
/// common year.
pub const fn sol_from_date(date: DarianDate) -> CalendarResult<i64> {
    if date.year < MIN_YEAR || date.year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    let length = match sols_in_month(date.year, date.month) {
        Some(value) => value,
        None => return Err(CalendarError::MonthOutOfRange),
    };
    if date.day == 0 || date.day > length {
        return Err(CalendarError::DayOutOfRange);
    }
    let mut sol = year_start_sol(date.year);
    let mut month = 1u8;
    while month < date.month {
        sol += match sols_in_month(date.year, month) {
            Some(value) => value as i64,
            None => return Err(CalendarError::MonthOutOfRange),
        };
        month += 1;
    }
    Ok(sol + date.day as i64 - 1)
}

/// The Darian date of a sol number.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] for a sol outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn date_from_sol(sol: i64) -> CalendarResult<DarianDate> {
    // 668 is a lower bound on the year length, so this never overshoots by
    // more than one year, and the correction below is a single step.
    let mut year = sol.div_euclid(669);
    while year_start_sol(year + 1) <= sol {
        year += 1;
    }
    while year_start_sol(year) > sol {
        year -= 1;
    }
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    let mut remaining = sol - year_start_sol(year);
    let mut month = 1u8;
    loop {
        let length = match sols_in_month(year, month) {
            Some(value) => value as i64,
            None => return Err(CalendarError::MonthOutOfRange),
        };
        if remaining < length {
            return Ok(DarianDate::new(year, month, (remaining + 1) as u8));
        }
        remaining -= length;
        month += 1;
    }
}

/// The Darian sol number containing a Mars Sol Date.
#[must_use]
pub fn sol_from_mars_sol_date(msd: f64) -> i64 {
    floor(msd) as i64 - DARIAN_EPOCH_MARS_SOL_DATE
}

/// The Mars Sol Date at which a Darian sol begins.
#[must_use]
pub const fn sol_mars_sol_date(sol: i64) -> i64 {
    sol + DARIAN_EPOCH_MARS_SOL_DATE
}

/// The Darian calendar.
///
/// See the module documentation before using the [`Calendar`] implementation:
/// the `Rd` values it produces are Darian **sol** numbers, not Earth days.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DarianCalendar;

impl DarianCalendar {
    /// The Darian date at a Martian moment, at the prime meridian.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] for a moment outside the
    /// supported year range.
    pub fn date_at(&self, moment: MarsMoment) -> CalendarResult<DarianDate> {
        date_from_sol(sol_from_mars_sol_date(moment.mars_sol_date()))
    }
}

impl Calendar for DarianCalendar {
    type Date = DarianDate;

    /// Twenty-four months and the seven-sol week.
    ///
    /// The week is not Earth's: its sols are named Sol Solis through Sol
    /// Saturni and no terrestrial weekday name applies, so its kind is
    /// `sol-of-week` rather than `weekday`.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        const SHAPE: &[hc_calendar::shape::CycleShape] = &[
            hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTH_NAMES),
            hc_calendar::shape::CycleShape::named("sol-of-week", &WEEKDAY_NAMES),
        ];
        SHAPE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("darian"),
            english_name: "Darian (Mars)",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(Rd(year_start_sol(MIN_YEAR))),
            latest: Some(Rd(year_start_sol(MAX_YEAR + 1) - 1)),
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
        // Validate before describing, so that an impossible date cannot be
        // laundered into a field set.
        sol_from_date(date)?;
        Ok(DateFields::ymd(date.year, date.month, date.day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = DarianDate::new(fields.year, month.ordinal, fields.require_day()?);
        sol_from_date(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tai_from_utc_fields;

    fn moment(year: i64, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> MarsMoment {
        MarsMoment::from_tai(tai_from_utc_fields(year, month, day, hour, minute, second).unwrap())
    }

    #[test]
    fn there_are_twenty_four_months_and_seven_weekdays() {
        assert_eq!(MONTH_NAMES.len(), 24);
        assert_eq!(WEEKDAY_NAMES.len(), 7);
        assert_eq!(MONTH_NAMES[0], "Sagittarius");
        assert_eq!(MONTH_NAMES[23], "Vrishika");
        assert_eq!(WEEKDAY_NAMES[0], "Sol Solis");
        assert_eq!(WEEKDAY_NAMES[6], "Sol Saturni");
    }

    #[test]
    fn the_quarters_are_five_long_months_and_one_short_one() {
        for quarter in 0..4u8 {
            for offset in 1..=5u8 {
                assert_eq!(sols_in_month(3, quarter * 6 + offset), Some(28));
            }
            assert_eq!(sols_in_month(2, quarter * 6 + 6), Some(27));
        }
        assert_eq!(sols_in_month(1, 0), None);
        assert_eq!(sols_in_month(1, 25), None);
    }

    #[test]
    fn the_leap_sol_is_the_last_sol_of_vrishika() {
        assert!(is_leap_year(3));
        assert_eq!(sols_in_month(3, 24), Some(28));
        assert!(!is_leap_year(2));
        assert_eq!(sols_in_month(2, 24), Some(27));
        // Only the twenty-fourth month ever gains it.
        assert_eq!(sols_in_month(3, 6), Some(27));
        assert_eq!(sols_in_month(3, 18), Some(27));
    }

    #[test]
    fn the_leap_rule_is_gangales() {
        assert!(is_leap_year(1), "odd years are long");
        assert!(!is_leap_year(2));
        assert!(is_leap_year(10), "multiples of ten are long");
        assert!(!is_leap_year(100), "except multiples of a hundred");
        assert!(is_leap_year(500), "unless also multiples of five hundred");
        assert!(is_leap_year(1_000), "1000 is also a multiple of 500");
        assert!(
            !is_leap_year(1_100),
            "1100 is a hundred but not a five hundred"
        );
        assert!(is_leap_year(0), "year zero is divisible by 500");
        assert!(is_leap_year(-1), "negative odd years are long too");
        assert!(!is_leap_year(-100));
        assert!(is_leap_year(-500));
    }

    #[test]
    fn the_leap_rule_makes_the_mean_year_track_the_tropical_year() {
        // 296 long years per 500 gives 668.592 sols; Mars's tropical year is
        // 668.5921 sols.
        let long = (0..500).filter(|year| is_leap_year(*year)).count();
        assert_eq!(long, 296);
        let mean = (year_start_sol(500) - year_start_sol(0)) as f64 / 500.0;
        assert!((mean - 668.592).abs() < 1e-9, "{mean}");
        assert!((mean - super::super::MARS_TROPICAL_YEAR_SOLS).abs() < 2e-4);
        // Over ten thousand Mars years the calendar slips by about one sol.
        let slip = (year_start_sol(10_000) - year_start_sol(0)) as f64
            - 10_000.0 * super::super::MARS_TROPICAL_YEAR_SOLS;
        assert!(slip.abs() < 2.0, "{slip}");
    }

    #[test]
    fn the_month_lengths_add_up_to_the_year_length() {
        for year in -600..600 {
            let total: u16 = (1..=24)
                .map(|month| sols_in_month(year, month).unwrap() as u16)
                .sum();
            assert_eq!(total, sols_in_year(year), "year {year}");
            assert_eq!(
                (year_start_sol(year + 1) - year_start_sol(year)) as u16,
                sols_in_year(year),
                "year {year}"
            );
        }
    }

    #[test]
    fn dates_round_trip_through_sol_numbers_across_many_years() {
        let mut sol = year_start_sol(-50);
        let end = year_start_sol(350);
        while sol < end {
            let date = date_from_sol(sol).unwrap();
            assert_eq!(sol_from_date(date).unwrap(), sol, "{sol} -> {date:?}");
            sol += 1;
        }
    }

    #[test]
    fn every_month_begins_on_sol_solis() {
        for year in [0, 1, 2, 214, -7] {
            for month in 1..=24u8 {
                let date = DarianDate::new(year, month, 1);
                assert_eq!(date.weekday_name().unwrap(), "Sol Solis");
                let last = DarianDate::new(year, month, sols_in_month(year, month).unwrap());
                let expected = if last.day == 28 {
                    "Sol Saturni"
                } else {
                    "Sol Veneris"
                };
                assert_eq!(last.weekday_name().unwrap(), expected);
            }
        }
    }

    #[test]
    fn the_week_never_straddles_a_month_boundary() {
        // The whole point of dropping the 28th sol of a short month: the
        // weekday depends only on the day of the month.
        for day in 1..=28u8 {
            let a = DarianDate::new(5, 3, day).weekday_name().unwrap();
            let b = DarianDate::new(-1_000, 19, day).weekday_name().unwrap();
            assert_eq!(a, b, "day {day}");
        }
    }

    #[test]
    fn the_epoch_sol_is_one_sagittarius_of_year_zero() {
        let date = date_from_sol(0).unwrap();
        assert_eq!(date, DarianDate::new(0, 1, 1));
        assert_eq!(date.month_name().unwrap(), "Sagittarius");
        assert_eq!(sol_mars_sol_date(0), DARIAN_EPOCH_MARS_SOL_DATE);
    }

    /// Two published Darian dates for events whose Mars Sol Date this crate
    /// computes independently, from the comparison table in the Wikipedia
    /// article on the Darian calendar.
    #[test]
    fn the_published_event_conversions_reproduce() {
        let viking = super::super::missions::mission("Viking 1")
            .unwrap()
            .landing_moment()
            .unwrap();
        let date = DarianCalendar.date_at(viking).unwrap();
        assert_eq!(date, DarianDate::new(195, 8, 14));
        assert_eq!(date.month_name().unwrap(), "Mina");
        assert_eq!(sol_from_mars_sol_date(viking.mars_sol_date()), 130_584);

        let perseverance = super::super::missions::mission("Perseverance")
            .unwrap()
            .landing_moment()
            .unwrap();
        let date = DarianCalendar.date_at(perseverance).unwrap();
        assert_eq!(date, DarianDate::new(219, 1, 13));
        assert_eq!(date.month_name().unwrap(), "Sagittarius");
        assert_eq!(
            sol_from_mars_sol_date(perseverance.mars_sol_date()),
            146_433
        );
    }

    #[test]
    fn darian_years_are_mars_years_plus_one_hundred_and_eighty_three() {
        // Curiosity landed in Mars Year 31, which is Darian year 214.
        let landing = super::super::missions::mission("Curiosity")
            .unwrap()
            .landing_moment()
            .unwrap();
        let date = DarianCalendar.date_at(landing).unwrap();
        assert_eq!(date.year, 214);
        assert_eq!(date.mars_year(), 31);
        assert_eq!(date.mars_year(), landing.mars_year());
    }

    #[test]
    fn the_darian_year_and_the_mars_year_stay_in_step_for_two_centuries() {
        let mut msd = -90_000.0;
        while msd < 60_000.0 {
            let moment = MarsMoment::from_mars_sol_date(msd);
            let date = DarianCalendar.date_at(moment).unwrap();
            // The two counts are both anchored on Ls = 0 but solved by
            // different means — one arithmetic, one from the orbital series —
            // so they may disagree by a sol at a boundary, never more.
            let difference = date.mars_year() - moment.mars_year();
            assert!(difference.abs() <= 1, "MSD {msd}: {difference}");
            msd += 337.0;
        }
    }

    #[test]
    fn the_calendar_trait_round_trips_in_both_directions() {
        let calendar = DarianCalendar;
        let mut sol = -2_000;
        while sol < 4_000 {
            let date = calendar.from_fixed(Rd(sol)).unwrap();
            assert_eq!(calendar.to_fixed(date).unwrap(), Rd(sol));
            sol += 1;
        }
        for date in [
            DarianDate::new(0, 1, 1),
            DarianDate::new(214, 12, 27),
            DarianDate::new(3, 24, 28),
            DarianDate::new(-77, 6, 27),
        ] {
            let rd = calendar.to_fixed(date).unwrap();
            assert_eq!(calendar.from_fixed(rd).unwrap(), date);
        }
    }

    #[test]
    fn the_fixed_day_this_calendar_emits_is_a_sol_not_an_earth_day() {
        // The documented footgun, pinned by a test so it cannot drift: the Rd
        // is the Darian sol number, and a Darian year is 668 or 669 of them,
        // not 365 or 366.
        let calendar = DarianCalendar;
        let start = calendar.to_fixed(DarianDate::new(100, 1, 1)).unwrap();
        let next = calendar.to_fixed(DarianDate::new(101, 1, 1)).unwrap();
        assert_eq!(next.get() - start.get(), sols_in_year(100) as i64);
        assert_eq!(start.get(), year_start_sol(100));
    }

    #[test]
    fn impossible_dates_are_rejected_with_the_right_error() {
        assert_eq!(
            sol_from_date(DarianDate::new(1, 25, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            sol_from_date(DarianDate::new(1, 0, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            sol_from_date(DarianDate::new(1, 1, 29)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            sol_from_date(DarianDate::new(1, 1, 0)),
            Err(CalendarError::DayOutOfRange)
        );
        // 2 is a common year, so Vrishika has only 27 sols.
        assert_eq!(
            sol_from_date(DarianDate::new(2, 24, 28)),
            Err(CalendarError::DayOutOfRange)
        );
        assert!(sol_from_date(DarianDate::new(3, 24, 28)).is_ok());
        // Every short month loses its 28th sol, leap year or not.
        assert_eq!(
            sol_from_date(DarianDate::new(3, 6, 28)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            sol_from_date(DarianDate::new(MAX_YEAR + 1, 1, 1)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            sol_from_date(DarianDate::new(MIN_YEAR - 1, 1, 1)),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_fields_interface_rejects_what_the_calendar_rejects() {
        let calendar = DarianCalendar;
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(2, 24, 28)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd_leap_month(2, 3, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert!(calendar.from_fields(&DateFields::new(5)).is_err());
        let fields = calendar.to_fields(DarianDate::new(214, 11, 28)).unwrap();
        assert_eq!(fields.year, 214);
        assert_eq!(fields.month.unwrap().ordinal, 11);
        assert_eq!(fields.day.unwrap(), 28);
        assert_eq!(
            calendar.from_fields(&fields).unwrap(),
            DarianDate::new(214, 11, 28)
        );
    }

    #[test]
    fn the_metadata_describes_an_arithmetic_calendar_with_no_leap_months() {
        let meta = DarianCalendar.meta();
        assert_eq!(meta.id, CalendarId("darian"));
        assert_eq!(meta.english_name, "Darian (Mars)");
        assert!(!meta.has_leap_months);
        assert!(!meta.is_astronomical);
        assert_eq!(meta.year_kind, YearKind::Astronomical);
        assert!(meta.supports(Rd(0)));
        assert!(!meta.supports(Rd(i64::MAX)));
    }

    #[test]
    fn month_and_weekday_names_refuse_impossible_indices() {
        assert_eq!(
            DarianDate::new(1, 25, 1).month_name(),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            DarianDate::new(1, 1, 29).weekday_name(),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn a_martian_moment_lands_on_the_sol_that_contains_it() {
        // The sol boundary is Airy midnight, the same boundary the Mars Sol
        // Date uses.
        let before = moment(2021, 2, 18, 20, 43, 48);
        let sol = sol_from_mars_sol_date(before.mars_sol_date());
        let start = MarsMoment::from_mars_sol_date(sol_mars_sol_date(sol) as f64);
        assert_eq!(start.coordinated_mars_time().to_string(), "00:00:00");
        assert!(start.mars_sol_date() <= before.mars_sol_date());
        assert!(start.mars_sol_date() + 1.0 > before.mars_sol_date());
    }
}
