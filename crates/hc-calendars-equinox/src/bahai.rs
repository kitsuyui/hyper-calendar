//! The Badíʿ calendar, astronomical — `bahai-astronomical`.
//!
//! The rules the Universal House of Justice unified the calendar on from
//! Naw-Rúz 172 BE (2015), applied to any year: Naw-Rúz is the Badíʿ day —
//! sunset to sunset at Tehran — in which the March equinox falls; the year
//! runs to the next Naw-Rúz, and Ayyám-i-Há is as long as it takes to get
//! there; and the Twin Holy Birthdays are "the first and the second day
//! following the occurrence of the eighth new moon after Naw-Rúz", again
//! reckoned at Tehran. Source: the Universal House of Justice, letter of
//! 10 July 2014.
//!
//! # The table is the test
//!
//! The Bahá'í World Centre published the resulting dates for 172–221 BE,
//! and `hc-calendars-solar`'s `bahai` carries that table as the calendar
//! as kept. This module reproduces every row of it — Naw-Rúz, the length of
//! Ayyám-i-Há and both birthdays — save the two Naw-Rúzes that fell within
//! [`TOLERANCE_MINUTES`] of Tehran's sunset, which the test names rather
//! than claims. One of them, 183 BE, is the sharpest edge in the table: on
//! 20 March 2026 the equinox and the sunset fall within seconds of each
//! other, and the row is decided by the committee's ephemeris and its
//! definition of sunset, which no model reproduces to that precision. The
//! test says exactly that, and it is the check on the astronomy
//! underneath, not only on this module. Past
//! 221 BE this calendar continues where the table stops, and that is what
//! it is for.
//!
//! # What it is not
//!
//! Not the calendar as kept before 172 BE, when Naw-Rúz was 21 March by
//! rule in the West and the Iranian equinox day in the East; this module
//! applies the 2015 rule to those years too, so use `bahai` for a date in
//! them. Not exact beyond the astronomy either: a year whose equinox falls
//! within [`TOLERANCE_MINUTES`] of Tehran's sunset is decided here by a
//! model. [`new_year_margin`] says how close the call was.
//!
//! A date here names the fixed day the Badíʿ day *ends* in, as the table
//! does; the day began at the previous sunset.

use hc_astro::lunar::new_moon_at_or_after;
use hc_astro::riseset::sunset;
use hc_astro::solar::{Equinox, equinox};
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_solar::bahai::{self, AYYAM_I_HA, BahaiDate, GREGORIAN_YEAR_OFFSET, NINETEEN};
use hc_calendars_solar::gregorian;

use crate::places::TEHRAN;

/// The identifier of the astronomical Badíʿ calendar.
pub const ID: CalendarId = CalendarId("bahai-astronomical");

/// The earliest year this calendar converts: the epoch.
pub const MIN_YEAR: i64 = 1;

/// The latest year this calendar converts, whose Naw-Rúz falls in
/// Gregorian 3000 — as far as the astronomy behind it is worth asking.
pub const MAX_YEAR: i64 = 3_000 - GREGORIAN_YEAR_OFFSET;

/// The number of new moons after Naw-Rúz that fixes the Twin Holy
/// Birthdays.
pub const TWIN_BIRTHDAYS_NEW_MOON: u8 = 8;

/// How close, in minutes, an equinox may fall to Tehran's sunset before
/// this calendar is deciding by a model rather than by the sky: the
/// equinox is placed to seconds, but a sunset is good to a minute or two
/// of its own geometry and depends on the horizon assumed — see
/// [`crate::places::TEHRAN`].
pub const TOLERANCE_MINUTES: f64 = 3.0;

/// The Badíʿ day containing `moment`, named by the fixed day it ends in:
/// the first day whose sunset at Tehran is after the moment.
///
/// Tehran has a sunset every day of the year, so this always answers.
#[must_use]
pub fn badi_day_containing(moment: Moment) -> Rd {
    let day = moment.day();
    match sunset(day, TEHRAN) {
        Some(set) if set.0 > moment.0 => day,
        _ => Rd(day.0 + 1),
    }
}

/// The moment a Badíʿ day begins: sunset at Tehran on the day before the
/// fixed day it ends in.
fn start_of(day: Rd) -> Moment {
    sunset(Rd(day.0 - 1), TEHRAN).unwrap_or(Moment(day.0 as f64 - 0.5))
}

/// The instant of the March equinox that begins `year`, in Universal Time.
fn equinox_of(year: i64) -> Moment {
    equinox(year + GREGORIAN_YEAR_OFFSET, Equinox::March)
}

/// The fixed day of Naw-Rúz, without validation.
fn naw_ruz_raw(year: i64) -> Rd {
    badi_day_containing(equinox_of(year))
}

/// The fixed day of Naw-Rúz, the first day of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn new_year(year: i64) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(naw_ruz_raw(year))
}

/// How far the equinox that begins `year` fell from the Tehran sunset that
/// decides Naw-Rúz, in minutes: positive when it fell before that sunset
/// (Naw-Rúz is the equinox's own day), negative when after (Naw-Rúz is the
/// day after).
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn new_year_margin(year: i64) -> CalendarResult<f64> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    let instant = equinox_of(year);
    let set = sunset(instant.day(), TEHRAN).unwrap_or(Moment(instant.day().0 as f64 + 0.5));
    Ok((set.0 - instant.0) * 24.0 * 60.0)
}

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The Universal House of Justice, letter of 10 July 2014: the calendar unified worldwide on \
    astronomical rules from Naw-Rúz 172 BE, 21 March 2015 (*Badíʿ dates 172 to 221 BE*, \
    Bahá'í World Centre, 2014)";

/// The earliest fixed day this calendar converts.
#[must_use]
pub fn earliest() -> Rd {
    naw_ruz_raw(MIN_YEAR)
}

/// The latest fixed day this calendar converts: the day before the Naw-Rúz
/// after [`MAX_YEAR`].
#[must_use]
pub fn latest() -> Rd {
    Rd(naw_ruz_raw(MAX_YEAR + 1).0 - 1)
}

/// The number of days in `year`, or `None` outside [`MIN_YEAR`]..=[`MAX_YEAR`].
#[must_use]
pub fn days_in_year(year: i64) -> Option<u16> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return None;
    }
    Some((naw_ruz_raw(year + 1).0 - naw_ruz_raw(year).0) as u16)
}

/// The length of Ayyám-i-Há in `year`, or `None` outside the range.
fn intercalary(year: i64) -> Option<i64> {
    days_in_year(year).map(|days| i64::from(days) - 19 * NINETEEN)
}

/// The number of days in `month` of `year`, or `None` when the year is out
/// of range or `month` is not in `0..=19`.
#[must_use]
pub fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        AYYAM_I_HA => intercalary(year).map(|days| days as u8),
        1..=19 => days_in_year(year).map(|_| 19),
        _ => None,
    }
}

/// The fixed day of a Badíʿ date.
///
/// `month` is `1..=19`, or [`AYYAM_I_HA`] for the intercalary days.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    let Some(length) = days_in_month(year, month) else {
        return Err(CalendarError::MonthOutOfRange);
    };
    if day == 0 || day > length {
        return Err(CalendarError::DayOutOfRange);
    }
    let Some(intercalary) = intercalary(year) else {
        return Err(CalendarError::YearOutOfRange);
    };
    Ok(Rd(naw_ruz_raw(year).0
        + bahai::days_before_month_with(intercalary, month)
        + i64::from(day)
        - 1))
}

/// The Badíʿ year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside
/// [`earliest`]..=[`latest`].
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < earliest() {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > latest() {
        return Err(CalendarError::AfterSupportedRange);
    }
    let (gregorian_year, _, _) = gregorian::from_fixed(rd)?;
    let mut year = gregorian_year - GREGORIAN_YEAR_OFFSET;
    if year > MAX_YEAR {
        year = MAX_YEAR;
    }
    if rd < naw_ruz_raw(year) {
        year -= 1;
    }
    let day_of_year = rd.0 - naw_ruz_raw(year).0;
    let Some(intercalary) = intercalary(year) else {
        return Err(CalendarError::YearOutOfRange);
    };
    let (month, day) = bahai::split_day_of_year(day_of_year, intercalary);
    Ok((year, month, day))
}

/// The Twin Holy Birthdays of `year`: the Birth of the Báb and, the day
/// after, the Birth of Bahá'u'lláh — the first and second day following
/// the Badíʿ day that contains the eighth new moon after Naw-Rúz.
///
/// New moons are counted from the moment Naw-Rúz begins, sunset at Tehran
/// on the evening before.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn twin_holy_birthdays(year: i64) -> CalendarResult<(Rd, Rd)> {
    let naw_ruz = new_year(year)?;
    let mut moon = start_of(naw_ruz);
    for _ in 0..TWIN_BIRTHDAYS_NEW_MOON {
        moon = new_moon_at_or_after(Moment(moon.0 + 1.0));
    }
    let day = badi_day_containing(moon);
    Ok((Rd(day.0 + 1), Rd(day.0 + 2)))
}

/// The astronomical Badíʿ calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AstronomicalBahaiCalendar;

impl Calendar for AstronomicalBahaiCalendar {
    type Date = BahaiDate;

    /// The rules in force from Naw-Rúz 172 BE, 21 March 2015; before that the
    /// calendar was kept by other rules, which `bahai` carries, and this one
    /// is proleptic.
    fn usage(&self) -> hc_calendar::Usage {
        match gregorian::to_fixed(2015, 3, 21) {
            Ok(rd) => hc_calendar::Usage::since(rd, USAGE_SOURCE),
            Err(_) => hc_calendar::Usage::UNRECORDED,
        }
    }

    /// Nineteen months of nineteen days, and a seven-day week; Ayyám-i-Há is
    /// not a position in the month cycle.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        bahai::SHAPE
    }

    /// A year whose Ayyám-i-Há has five days.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        days_in_month(year, AYYAM_I_HA)
            .map(|days| days == 5)
            .ok_or(CalendarError::YearOutOfRange)
    }

    /// The Bahá'í day begins at sunset.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Badíʿ (astronomical)",
            year_kind: YearKind::EpochForward,
            // Ayyám-i-Há is carried as an intercalary repetition of month
            // 18, which is what makes this true.
            has_leap_months: true,
            is_astronomical: true,
            earliest: Some(earliest()),
            latest: Some(latest()),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(BahaiDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        bahai::fields_of(date)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let (year, month, day) = bahai::ordinal_from_fields(fields)?;
        to_fixed(year, month, day)?;
        Ok(BahaiDate { year, month, day })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::bahai_kept as kept;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    /// The Birth of the Báb, 2015–2064, from the Bahá'í World Centre's
    /// table *Badíʿ dates 172 to 221 BE*; the Birth of Bahá'u'lláh is the
    /// day after in every row.
    const BIRTH_OF_THE_BAB: [(i64, u8, u8); 50] = [
        (2015, 11, 13),
        (2016, 11, 1),
        (2017, 10, 21),
        (2018, 11, 9),
        (2019, 10, 29),
        (2020, 10, 18),
        (2021, 11, 6),
        (2022, 10, 26),
        (2023, 10, 16),
        (2024, 11, 2),
        (2025, 10, 22),
        (2026, 11, 10),
        (2027, 10, 30),
        (2028, 10, 19),
        (2029, 11, 7),
        (2030, 10, 28),
        (2031, 10, 17),
        (2032, 11, 4),
        (2033, 10, 24),
        (2034, 11, 12),
        (2035, 11, 1),
        (2036, 10, 20),
        (2037, 11, 8),
        (2038, 10, 29),
        (2039, 10, 19),
        (2040, 11, 6),
        (2041, 10, 26),
        (2042, 10, 15),
        (2043, 11, 3),
        (2044, 10, 22),
        (2045, 11, 10),
        (2046, 10, 30),
        (2047, 10, 20),
        (2048, 11, 7),
        (2049, 10, 28),
        (2050, 10, 17),
        (2051, 11, 5),
        (2052, 10, 24),
        (2053, 11, 11),
        (2054, 11, 1),
        (2055, 10, 21),
        (2056, 11, 8),
        (2057, 10, 29),
        (2058, 10, 18),
        (2059, 11, 6),
        (2060, 10, 25),
        (2061, 10, 14),
        (2062, 11, 2),
        (2063, 10, 23),
        (2064, 11, 10),
    ];

    /// Whether the model can claim the Naw-Rúz of `year`: whether the
    /// equinox fell farther from Tehran's sunset than the model is good to.
    fn decidable(year: i64) -> bool {
        new_year_margin(year).is_ok_and(|margin| margin.abs() >= TOLERANCE_MINUTES)
    }

    #[test]
    fn every_naw_ruz_of_the_world_centres_table_the_model_can_decide_is_reproduced() {
        let mut undecidable = alloc::vec::Vec::new();
        for year in kept::FIRST_TABULATED_YEAR..=kept::LAST_TABULATED_YEAR {
            if !decidable(year) {
                undecidable.push(year);
                continue;
            }
            assert_eq!(
                new_year(year),
                kept::new_year(year),
                "{year} BE: margin {:?} minutes",
                new_year_margin(year)
            );
            if decidable(year + 1) {
                assert_eq!(
                    days_in_month(year, AYYAM_I_HA),
                    kept::days_in_month(year, AYYAM_I_HA),
                    "{year} BE Ayyám-i-Há"
                );
            }
        }
        // Two of the fifty fall inside the tolerance. 183 BE (2026): the
        // equinox at 14:45:55 UT and Tehran's sea-level sunset within
        // seconds of it; the table says 21 March, and so does the model,
        // with the equinox six seconds after the sunset. That agreement is
        // ΔT's: with the observed ΔT of 2026 the equinox lands here, and
        // with the Espenak–Meeus polynomial's, five seconds larger, it
        // landed under a second before the sunset and the model said the
        // 20th. 216 BE (2059): about two minutes before sunset by the
        // model, which agrees with the table. Neither is a row a model gets
        // to decide, so neither is claimed.
        assert_eq!(undecidable, [183, 216], "the marginal years changed");
        assert_eq!(kept::new_year(183), gregorian::to_fixed(2026, 3, 21));
        assert_eq!(new_year(183), kept::new_year(183));
        let margin_183 = new_year_margin(183).unwrap();
        assert!(
            (-0.2..0.0).contains(&margin_183),
            "183 BE margin {margin_183} minutes"
        );
        assert_eq!(new_year(216), kept::new_year(216));
    }

    #[test]
    fn every_twin_birthday_of_the_world_centres_table_is_reproduced() {
        for (index, (gregorian_year, month, day)) in BIRTH_OF_THE_BAB.iter().enumerate() {
            let year = kept::FIRST_TABULATED_YEAR + index as i64;
            let bab = ymd(*gregorian_year, *month, *day);
            assert_eq!(
                twin_holy_birthdays(year),
                Ok((bab, Rd(bab.0 + 1))),
                "{year} BE ({gregorian_year})"
            );
        }
    }

    #[test]
    fn the_margin_is_measured_from_sunset() {
        // 172 BE: the equinox of 2015 at 22:45 UT on 20 March, eight hours
        // after Tehran's sunset, so Naw-Rúz is the 21st.
        let margin = new_year_margin(172).unwrap();
        assert!(margin < -400.0 && margin > -540.0, "172 BE margin {margin}");
        assert_eq!(new_year(172), Ok(ymd(2015, 3, 21)));
        // 181 BE: 03:06 UT on 20 March 2024, well before sunset.
        let margin = new_year_margin(181).unwrap();
        assert!(margin > 600.0, "181 BE margin {margin}");
        assert_eq!(new_year(181), Ok(ymd(2024, 3, 20)));
    }

    #[test]
    fn before_172_be_the_rule_is_proleptic_and_says_so_by_its_name() {
        // 173 BE by the 2015 rule begins 20 March 2016; the arithmetic
        // calendar kept in the West began it on the 21st.
        assert_eq!(new_year(173), Ok(ymd(2016, 3, 20)));
        assert_eq!(
            hc_calendars_solar::bahai::new_year(173),
            Ok(ymd(2016, 3, 21))
        );
        // And the era's own first day: by the equinox rule 1 BE begins on
        // 20 March 1844, the day before the 21 March the calendar was
        // founded on. That is what "proleptic" costs.
        assert_eq!(new_year(1), Ok(ymd(1844, 3, 20)));
        assert_eq!(hc_calendars_solar::bahai::EPOCH, ymd(1844, 3, 21));
    }

    #[test]
    fn a_sample_of_days_round_trips() {
        let start = new_year(150).unwrap().0;
        let end = new_year(250).unwrap().0;
        for rd in (start..end).step_by(53) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = AstronomicalBahaiCalendar;
        for rd in (new_year(170).unwrap().0..new_year(190).unwrap().0).step_by(367) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(bahai::ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, ID);
        assert!(calendar.meta().is_astronomical);
        assert!(calendar.meta().has_leap_months);
    }

    #[test]
    fn the_range_is_stated_and_refused_outside() {
        assert_eq!(new_year(0), Err(CalendarError::YearOutOfRange));
        assert_eq!(new_year(MAX_YEAR + 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(twin_holy_birthdays(0), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            from_fixed(Rd(earliest().0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(latest().0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        // 184 BE has four days of Ayyám-i-Há, by the table and by the model.
        assert_eq!(days_in_month(184, AYYAM_I_HA), Some(4));
        assert_eq!(
            to_fixed(184, AYYAM_I_HA, 5),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(to_fixed(184, 20, 1), Err(CalendarError::MonthOutOfRange));
    }
}
