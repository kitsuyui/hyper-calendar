//! A *prediction* of the observational Hebrew calendar of the Second Temple
//! period — `hebrew-observational`.
//!
//! The system is written up in `docs/systems/hebrew-observational.md` in
//! the repository: what the calendar was, the rule Reingold and Dershowitz
//! give for predicting it, Nisan and Passover of 2024 worked by hand, what
//! is carried, and what the tests can and cannot say about it. This page
//! summarises it and states the code's own facts.
//!
//! # Read this before using it
//!
//! Before the fixed calendar of [`crate::hebrew`], each month was declared
//! by the court on the testimony of witnesses to the new crescent, and the
//! year was made leap by decision. No record of those declarations is
//! carried. What this module computes is Reingold and Dershowitz's
//! *prediction* of that calendar: a month begins on the first evening the
//! crescent should have been visible from Haifa by Shaukat's criterion, and
//! the year begins with the first such month whose fifteenth day is not
//! before the spring equinox. It is a forecast of an observation, of the
//! kind [`crate::islamic_observational`] is, and no published date of the
//! calendar was found to test it against.
//!
//! The rule is `observational-hebrew-first-of-nisan`,
//! `fixed-from-observational-hebrew`, `observational-hebrew-from-fixed` and
//! `classical-passover-eve` in the published code of *Calendrical
//! Calculations* (4th ed., Cambridge, 2018), `calendar.l` in the
//! `calendar-code2` repository (Apache License 2.0), read 2026-09-26, with
//! its `hebrew-location` for Haifa.
//!
//! # Dates and months
//!
//! Dates are [`HebrewDate`]s numbered as [`crate::hebrew`] numbers them,
//! from Tishrei, with Adar I as [`Month::leap(5)`](Month::leap), and the
//! year counted in the era of creation as the fixed calendar counts it.
//! Whether a year has Adar I is the prediction's, not the fixed
//! calendar's: [`HebrewDate::new`] validates against the fixed calendar,
//! so a date of this one is validated by [`to_fixed`] instead.
//!
//! A predicted month runs 29 or 30 days, or 31 when a first evening that
//! just clears the criterion is followed thirty evenings later by one that
//! just misses; the published code's main version keeps such months, and
//! so does this one, with the observational Hijri calendar
//! ([`MAXIMUM_MONTH_LENGTH`]). The 31st of such a month is a date here.
//!
//! # Range
//!
//! From [`EARLIEST`], 1 January 383 BCE, the year from which this library
//! has measured its crescent computation against a published table of
//! first visibilities (the Babylonian calendar's comparison with Parker and
//! Dubberstein), to [`LATEST`], 31 December 2100, where the observational
//! Hijri prediction also stops. Both bounds are this library's choice.

use hc_astro::solar::{Equinox, equinox};
use hc_astro::{Location, sunset};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_core::math::{floor, round};

use crate::civil;
use crate::hebrew::{self, ERA, HebrewDate};
use crate::islamic_observational::{
    MAXIMUM_MONTH_LENGTH, ObservationSite, VisibilityCriterion, read_back_error,
};

/// The machine identifier of this calendar. CLDR has none for it.
pub const ID: CalendarId = CalendarId("hebrew-observational");

/// Haifa: 32.82° N, 35° E, sea level, the `hebrew-location` of the
/// published code of *Calendrical Calculations*, its sample location for
/// the observational Hebrew calendar.
pub const HAIFA: Location = Location::new(32.82, 35.0, 0.0);

/// Where the crescent is looked for: Haifa, by Shaukat's criterion, the
/// published code's `visible-crescent`.
pub const SITE: ObservationSite = ObservationSite::new(HAIFA, VisibilityCriterion::SHAUKAT);

/// The earliest fixed day this calendar converts: 1 January of the
/// proleptic Gregorian year −382, 383 BCE.
pub const EARLIEST: Rd = civil::to_rd(-382, 1, 1);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = civil::to_rd(2100, 12, 31);

/// The mean month the published code rounds by: 29.5 days.
const ROUNDING_MONTH: f64 = 29.5;

/// Tishrei in the Nisan-first count: the seventh month, at which the year
/// number changes.
const TISHREI: u8 = 7;

/// 1 Nisan of the observational calendar in the spring of a proleptic
/// Gregorian year (`observational-hebrew-first-of-nisan`).
///
/// The first day on or after a starting day whose eve shows the crescent at
/// Haifa, where the starting day is fourteen days before the day of the
/// March equinox (in Universal Time) when the equinox falls before that
/// day's sunset at Haifa, and thirteen days before it otherwise: so that
/// the fifteenth of Nisan, the first day of Passover, is not before the
/// equinox.
///
/// # Errors
///
/// [`CalendarError::AstronomicalModelFailure`] when the Sun does not set at
/// Haifa or no crescent is found, which would be a failure of the model.
pub fn first_of_nisan(gregorian_year: i64) -> CalendarResult<Rd> {
    let spring = equinox(gregorian_year, Equinox::March);
    let day = Rd(floor(spring.0) as i64);
    let set = sunset(day, HAIFA).ok_or(CalendarError::AstronomicalModelFailure)?;
    let back = if spring.0 < set.0 { 14 } else { 13 };
    SITE.month_start_on_or_after(Rd(day.0 - back))
}

/// The eve of Passover, 14 Nisan, in the spring of a proleptic Gregorian
/// year (`classical-passover-eve`).
///
/// # Errors
///
/// As [`first_of_nisan`].
pub fn classical_passover_eve(gregorian_year: i64) -> CalendarResult<Rd> {
    Ok(Rd(first_of_nisan(gregorian_year)?.0 + 13))
}

/// Whether the months from 1 Nisan of a Gregorian year's spring to the
/// next run to thirteen.
fn thirteen_months_from(gregorian_year: i64) -> CalendarResult<bool> {
    let this = first_of_nisan(gregorian_year)?;
    let next = first_of_nisan(gregorian_year + 1)?;
    Ok(next.0 - this.0 > 370)
}

/// The Gregorian year whose spring holds 1 Nisan of Hebrew year `year`, by
/// the fixed calendar's Nisan (as `fixed-from-observational-hebrew` finds
/// it: sixty days after the fixed 1 Nisan is always in the same Gregorian
/// year as the observational one).
fn spring_of(year: i64) -> CalendarResult<i64> {
    let nisan = hebrew::to_fixed(year, Month::regular(7), 1)?;
    Ok(civil::year_from_rd(Rd(nisan.0 + 60)))
}

/// Whether Hebrew year `year` of this calendar has thirteen months: whether
/// the Nisan after its Tishrei comes thirteen lunations after the one
/// before.
///
/// # Errors
///
/// As [`first_of_nisan`], and [`CalendarError::YearOutOfRange`] for a year
/// the fixed calendar cannot place.
pub fn is_leap_year(year: i64) -> CalendarResult<bool> {
    thirteen_months_from(spring_of(year - 1)?)
}

/// A public, Tishrei-first month as the Nisan-first number, given whether
/// the year has thirteen months.
const fn internal_month(month: Month, leap: bool) -> Option<u8> {
    let ordinal = month.ordinal;
    if month.leap {
        return if leap && ordinal == 5 { Some(12) } else { None };
    }
    match ordinal {
        1..=5 => Some(ordinal + 6),
        6 if leap => Some(13),
        6 => Some(12),
        7..=12 => Some(ordinal - 6),
        _ => None,
    }
}

/// A Nisan-first month number as the public, Tishrei-first month.
const fn public_month(month: u8, leap: bool) -> Month {
    match month {
        12 if leap => Month::leap(5),
        13 => Month::regular(6),
        TISHREI..=12 => Month::regular(month - 6),
        _ => Month::regular(month + 6),
    }
}

/// Range check.
fn check_range(rd: Rd) -> CalendarResult<()> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    Ok(())
}

/// The predicted Hebrew date of a fixed day
/// (`observational-hebrew-from-fixed`).
///
/// # Errors
///
/// [`CalendarError::BeforeEpoch`] or [`CalendarError::AfterSupportedRange`]
/// outside the range, and [`CalendarError::AstronomicalModelFailure`] when
/// the crescent search does not converge.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, Month, u8)> {
    check_range(rd)?;
    let crescent = SITE.month_start_on_or_before(rd)?;
    let gregorian_year = civil::year_from_rd(rd);
    let this_spring = first_of_nisan(gregorian_year)?;
    let (spring_year, new_year) = if rd < this_spring {
        (gregorian_year - 1, first_of_nisan(gregorian_year - 1)?)
    } else {
        (gregorian_year, this_spring)
    };
    let elapsed = round((crescent.0 - new_year.0) as f64 / ROUNDING_MONTH) as i64;
    if !(0..13).contains(&elapsed) {
        return Err(CalendarError::AstronomicalModelFailure);
    }
    let month = elapsed as u8 + 1;
    let year = hebrew::year_from_fixed(new_year)? + i64::from(month >= TISHREI);
    let leap = month == 12 && thirteen_months_from(spring_year)?;
    let day = (rd.0 - crescent.0 + 1) as u8;
    Ok((year, public_month(month, leap), day))
}

/// The fixed day of a predicted Hebrew date
/// (`fixed-from-observational-hebrew`).
///
/// # Errors
///
/// [`CalendarError::MonthOutOfRange`] for a month the year does not have,
/// Adar I in a year of twelve months among them;
/// [`CalendarError::DayOutOfRange`] for a day past the month's last; the
/// range errors; and [`CalendarError::AstronomicalModelFailure`].
pub fn to_fixed(year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
    if month.ordinal == 0 || month.ordinal > 12 || (month.leap && month.ordinal != 5) {
        return Err(CalendarError::MonthOutOfRange);
    }
    if day == 0 || day > MAXIMUM_MONTH_LENGTH {
        return Err(CalendarError::DayOutOfRange);
    }
    let winter = month.leap || (1..=6).contains(&month.ordinal);
    let spring_year = spring_of(if winter { year - 1 } else { year })?;
    let leap = winter && thirteen_months_from(spring_year)?;
    let internal = internal_month(month, leap).ok_or(CalendarError::MonthOutOfRange)?;
    let new_year = first_of_nisan(spring_year)?;
    let midmonth = new_year.0 + round(ROUNDING_MONTH * f64::from(internal - 1)) as i64 + 15;
    let start = SITE.month_start_on_or_before(Rd(midmonth))?;
    let rd = Rd(start.0 + i64::from(day) - 1);
    check_range(rd)?;
    // An observed month's length is not known in advance, so the date is
    // validated by reading it back.
    let (round_year, round_month, round_day) = from_fixed(rd)?;
    if (round_year, round_month) != (year, month) {
        return Err(read_back_error(
            from_fixed(start).is_ok_and(|(y, m, _)| (y, m) == (year, month)),
        ));
    }
    if round_day != day {
        return Err(CalendarError::DayOutOfRange);
    }
    Ok(rd)
}

/// The predicted observational Hebrew calendar at Haifa.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ObservationalHebrewCalendar;

impl Calendar for ObservationalHebrewCalendar {
    type Date = HebrewDate;

    /// Unrecorded: this is a prediction of sightings under one criterion,
    /// never a calendar anyone declared, so there is no period in which it
    /// was in force.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// A year with Adar I: thirteen predicted months from Tishrei to Elul.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        is_leap_year(year)
    }

    /// The day begins at sunset and is named by the civil day it ends on:
    /// the crescent that opens a month is looked for on the eve of its
    /// first day, as the published code's `phasis-on-or-before` has it.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset(hc_calendar::DayNaming::ByEnd)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Hebrew (observational, predicted)",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["he"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(HebrewDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut fields = DateFields::new(date.year).with_era(ERA);
        fields.month = Some(date.month);
        fields.day = Some(date.day);
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let date = HebrewDate {
            year: fields.year,
            month: fields.require_month()?,
            day: fields.require_day()?,
        };
        to_fixed(date.year, date.month, date.day)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::Weekday;

    /// Nisan, the seventh month of the public Tishrei-first count.
    const NISAN: Month = Month::regular(7);

    #[test]
    fn nisan_2024_is_worked_in_the_document() {
        // docs/systems/hebrew-observational.md, "Worked example".
        let first = first_of_nisan(2024).expect("converges");
        assert_eq!(first, civil::to_rd(2024, 3, 12));
        assert_eq!(Weekday::from_rd(first), Weekday::Tuesday);
        assert_eq!(classical_passover_eve(2024), Ok(civil::to_rd(2024, 3, 25)));
        assert_eq!(from_fixed(first), Ok((5_784, NISAN, 1)));
        // On the fixed calendar the same day is 2 Adar II 5784, and its
        // 1 Nisan four weeks later (Hebcal, "Jewish Holidays 5784").
        assert_eq!(hebrew::from_fixed(first), Ok((5_784, Month::regular(6), 2)));
        assert_eq!(
            hebrew::to_fixed(5_784, Month::regular(7), 1),
            Ok(civil::to_rd(2024, 4, 9))
        );
        // The fixed calendar made 5784 leap; the prediction makes 5785 so.
        assert_eq!(first_of_nisan(2025), Ok(civil::to_rd(2025, 3, 31)));
        assert_eq!(is_leap_year(5_784), Ok(false));
        assert_eq!(is_leap_year(5_785), Ok(true));
        assert!(hebrew::is_leap_year(5_784) && !hebrew::is_leap_year(5_785));
    }

    #[test]
    fn the_fifteenth_of_nisan_is_the_first_on_or_after_the_equinox() {
        for year in (1_900..=2_100i64).chain(-382..-282) {
            let spring = equinox(year, Equinox::March);
            let day = floor(spring.0) as i64;
            let set = sunset(Rd(day), HAIFA).expect("the Sun sets at Haifa");
            // The equinox after sunset belongs to the next Hebrew day.
            let equinox_day = if spring.0 < set.0 { day } else { day + 1 };
            let first = first_of_nisan(year).expect("converges");
            assert!(
                first.0 + 14 >= equinox_day,
                "{year}: 15 Nisan before the equinox"
            );
            let previous = SITE
                .month_start_on_or_before(Rd(first.0 - 1))
                .expect("converges");
            assert!(
                previous.0 + 14 < equinox_day,
                "{year}: the month before would also have done"
            );
        }
    }

    #[test]
    fn passover_eve_is_the_fourteenth_of_the_first_month() {
        for year in 1_900..=2_100i64 {
            let first = first_of_nisan(year).expect("converges");
            let eve = classical_passover_eve(year).expect("converges");
            let hebrew_year = hebrew::year_from_fixed(first).expect("in range");
            assert_eq!(from_fixed(first), Ok((hebrew_year, NISAN, 1)), "{year}");
            assert_eq!(from_fixed(eve), Ok((hebrew_year, NISAN, 14)), "{year}");
            assert_eq!(SITE.month_start_on_or_before(first), Ok(first));
        }
    }

    #[test]
    fn months_run_twenty_nine_or_thirty_days_but_for_a_few_of_thirty_one() {
        // Each evening is judged by itself, so a first evening that just
        // clears the criterion followed thirty evenings later by one that
        // just misses makes a month of 31 days. The published code keeps
        // such months (its capped alternative is not carried), and so does
        // this module: four in the 2 486 months of 1900–2100.
        let mut lengths = [0u32; 3];
        let mut long_months = Vec::new();
        let last = civil::to_rd(2_101, 1, 1);
        let mut cursor = SITE
            .month_start_on_or_after(civil::to_rd(1_900, 1, 1))
            .expect("converges");
        while cursor < last {
            let next = SITE
                .month_start_on_or_after(Rd(cursor.0 + 1))
                .expect("converges");
            let length = next.0 - cursor.0;
            assert!(
                (29..=31).contains(&length),
                "month at {cursor} ran {length}"
            );
            lengths[(length - 29) as usize] += 1;
            if length == 31 {
                long_months.push(civil::from_rd(cursor));
            }
            cursor = next;
        }
        assert_eq!(lengths, [1_171, 1_311, 4]);
        assert_eq!(
            long_months,
            [
                (1_917, 7, 21),
                (1_933, 7, 24),
                (1_971, 7, 24),
                (2_042, 9, 16)
            ]
        );
        // The 31st of each is a date, and round-trips.
        for (year, month, day) in long_months {
            let last = Rd(civil::to_rd(year, month, day).0 + 30);
            let (hebrew_year, hebrew_month, hebrew_day) = from_fixed(last).expect("in range");
            assert_eq!(hebrew_day, 31, "{year}-{month}-{day}");
            assert_eq!(to_fixed(hebrew_year, hebrew_month, 31), Ok(last));
        }
    }

    #[test]
    fn years_run_twelve_or_thirteen_months() {
        for year in (5_750..5_790i64).chain(3_661..3_700) {
            let start = to_fixed(year, Month::regular(1), 1).expect("Tishrei exists");
            let next = to_fixed(year + 1, Month::regular(1), 1).expect("Tishrei exists");
            let length = next.0 - start.0;
            let leap = is_leap_year(year).expect("converges");
            let expected = if leap { 383..=386 } else { 353..=356 };
            assert!(
                expected.contains(&length),
                "{year} ran {length} days, leap {leap}"
            );
            assert_eq!(to_fixed(year, Month::leap(5), 1).is_ok(), leap, "{year}");
        }
    }

    #[test]
    fn the_calendar_round_trips_over_four_years_of_days() {
        let calendar = ObservationalHebrewCalendar;
        let start = civil::to_rd(2022, 6, 1);
        // Every day in a release build, every eleventh in a debug one.
        for offset in (0..1_500i64).step_by(crate::sweep_stride(11)) {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn the_calendar_round_trips_at_both_ends_of_its_range() {
        let calendar = ObservationalHebrewCalendar;
        for start in [EARLIEST.0, LATEST.0 - 800] {
            // Every day in a release build, every eleventh in a debug one.
            for offset in (0..=800i64).step_by(crate::sweep_stride(11)) {
                let rd = Rd(start + offset);
                let date = calendar.from_fixed(rd).expect("in range");
                assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
            }
        }
    }

    #[test]
    fn the_prediction_and_the_fixed_calendar_and_the_crate_says_how_far_apart() {
        // No published date of the observational calendar exists to check
        // it against; this is the measure the document reports instead.
        let mut differences = [0u32; 4];
        for year in 1_900..=2_100i64 {
            let predicted = first_of_nisan(year).expect("converges");
            let fixed = hebrew::to_fixed(year + 3_760, Month::regular(7), 1).expect("in range");
            match predicted.0 - fixed.0 {
                0 => differences[0] += 1,
                1 => differences[1] += 1,
                2 => differences[2] += 1,
                -29..=-27 => differences[3] += 1,
                other => panic!("{year}: {other} days apart"),
            }
        }
        assert_eq!(differences, [48, 58, 44, 51]);
        let agreeing = (5_661..5_861i64)
            .filter(|&year| is_leap_year(year) == Ok(hebrew::is_leap_year(year)))
            .count();
        assert_eq!(agreeing, 99);
    }

    #[test]
    fn the_range_is_refused_rather_than_extrapolated() {
        let calendar = ObservationalHebrewCalendar;
        assert_eq!(
            calendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            calendar.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn out_of_range_fields_name_the_field_that_is_wrong() {
        // 5784 has twelve months here, so no Adar I; 5785 has thirteen.
        assert_eq!(
            to_fixed(5_784, Month::leap(5), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert!(to_fixed(5_785, Month::leap(5), 1).is_ok());
        assert_eq!(
            to_fixed(5_785, Month::leap(6), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(5_785, Month::regular(13), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(5_785, Month::regular(1), 0),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(5_785, Month::regular(1), 32),
            Err(CalendarError::DayOutOfRange)
        );
        // A day past the end of a month that exists is a wrong day, not a
        // wrong month.
        assert_eq!(
            to_fixed(5_785, Month::regular(1), 31),
            Err(CalendarError::DayOutOfRange)
        );
        let calendar = ObservationalHebrewCalendar;
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(5_785, 1, 1).with_era("ah")),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn the_metadata_admits_the_calendar_is_a_prediction() {
        let calendar = ObservationalHebrewCalendar;
        let meta = calendar.meta();
        assert_eq!(meta.id, CalendarId("hebrew-observational"));
        assert!(meta.is_astronomical && meta.has_leap_months);
        assert_eq!(calendar.usage(), hc_calendar::Usage::UNRECORDED);
        assert_eq!(SITE.location, HAIFA);
    }
}
