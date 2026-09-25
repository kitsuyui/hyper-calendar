//! The French Republican calendar, arithmetic (Romme) variant.
//!
//! Twelve months of thirty days, each divided into three *décades* of ten
//! days, followed by five complementary days — the *sansculottides* — and a
//! sixth in a leap year. The year began at the autumn equinox, which is why
//! An I, the year of the Republic, begins on 22 September 1792.
//!
//! # Which variant this is, and which it is not
//!
//! The decree of 5 October 1793 defined the new year **astronomically**: the
//! year begins on the day the true autumn equinox falls at the Paris
//! observatory. That is an observation, and it is not implemented here.
//!
//! What is implemented is the arithmetic rule Charles-Gilbert Romme proposed
//! in 1795 and never got adopted, in the form Reingold and Dershowitz give
//! as `fixed-from-arithmetic-french`: a leap year every fourth year, with
//! the Gregorian century exception and a further exception every four
//! thousand years. Its mean year is 365.24225 days.
//!
//! **The two variants disagree, and early.** The equinox put the sextile day
//! at the end of An III, An VII and An XI; the Romme rule puts it at the end
//! of An IV, An VIII and An XII. So An IV began on 23 September 1795 in
//! practice and on 22 September 1795 here, and the calendars stay one day
//! apart until they resynchronise. Anyone converting a dated document from
//! the twelve years the calendar was actually in force — 1793 to 1805 —
//! needs the equinox variant, `french-republican-equinox` in
//! `hc-calendars-equinox`. This module is for the arithmetic extension,
//! forwards and backwards, of the *idea*.
//!
//! The month names and the day names of the *décade* are [`MONTHS`] and
//! [`DECADE_DAYS`], declared with the calendar's shape: the calendar was
//! defined in French and every other language borrows the words, so they
//! are the calendar's own rather than a locale's.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, gregorian};

/// The fixed day of 1 Vendémiaire An I, 22 September 1792.
pub const EPOCH: Rd = match gregorian::to_fixed(1792, 9, 22) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The era code of the Republican era.
pub const ERA: &str = "RE";

/// The number of days in a *décade*, the ten-day week that replaced the
/// seven-day one.
pub const DAYS_IN_DECADE: u8 = 10;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// The year of the decree that abolished the calendar, An XIV; it ended on
/// 31 December 1805 by Napoleon's decree of 9 September 1805.
pub const ABOLITION_YEAR: i64 = 14;

/// Whether `year` is a leap year under the Romme rule.
///
/// Every fourth year, minus the centuries that are not multiples of four
/// hundred, minus every four-thousandth year.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    if year.rem_euclid(4) != 0 {
        return false;
    }
    let within_four_centuries = year.rem_euclid(400);
    if within_four_centuries == 100 || within_four_centuries == 200 || within_four_centuries == 300
    {
        return false;
    }
    year.rem_euclid(4_000) != 0
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=13`. Month 13 is the *sansculottides*.
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

/// The fixed day of a date, without validation.
const fn to_fixed_raw(year: i64, month: u8, day: u8) -> i64 {
    let prior = year - 1;
    EPOCH.0 - 1 + 365 * prior + prior.div_euclid(4) - prior.div_euclid(100) + prior.div_euclid(400)
        - prior.div_euclid(4_000)
        + 30 * (month as i64 - 1)
        + day as i64
}

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The decrees of 14 Vendémiaire and 3 Brumaire An II (5 and 24 October 1793) \
    establishing the calendar, and its abolition from 1 January 1806 at the end of An XIV, \
    as Wikipedia, \"French Republican calendar\" (retrieved 2026-09-22) renders them";

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(to_fixed_raw(MIN_YEAR, 1, 1));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(to_fixed_raw(MAX_YEAR + 1, 1, 1) - 1);

/// The fixed day of a French Republican date.
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
        Ok(()) => Ok(Rd(to_fixed_raw(year, month, day))),
    }
}

/// The French Republican year, month and day of a fixed day.
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
    // 4 000 Republican years are 1 460 969 days, which inverts the leap rule
    // to within one year; the comparison below fixes the remaining case.
    let approximate = ((rd.0 - EPOCH.0 + 2) * 4_000).div_euclid(1_460_969) + 1;
    let year = if rd.0 < to_fixed_raw(approximate, 1, 1) {
        approximate - 1
    } else {
        approximate
    };
    let day_of_year = rd.0 - to_fixed_raw(year, 1, 1);
    let month = (day_of_year.div_euclid(30) + 1) as u8;
    let day = (rd.0 - to_fixed_raw(year, month, 1) + 1) as u8;
    Ok((year, month, day))
}

/// A French Republican date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrenchRepublicanDate {
    /// The year of the Republic, counting from 1.
    pub year: i64,
    /// The month, 1 for Vendémiaire through 12 for Fructidor; month 13 is
    /// the *sansculottides*.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl FrenchRepublicanDate {
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

    /// Whether this date is one of the complementary days.
    #[must_use]
    pub const fn is_complementary_day(self) -> bool {
        self.month == 13
    }

    /// The *décade* within the month, 1 to 3, and the day within it, 1 to
    /// 10. The complementary days belong to no *décade*.
    #[must_use]
    pub const fn decade(self) -> Option<(u8, u8)> {
        if self.is_complementary_day() {
            return None;
        }
        Some((
            (self.day - 1) / DAYS_IN_DECADE + 1,
            (self.day - 1) % DAYS_IN_DECADE + 1,
        ))
    }
}

/// The arithmetic French Republican calendar.
///
/// The name carries the variant on purpose: see the module documentation for
/// how it differs from the calendar France actually used.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ArithmeticFrenchRepublicanCalendar;

/// The twelve months and, as a thirteenth position, the complementary
/// days — named as Fabre d'Églantine proposed and the Convention adopted
/// on 3 Brumaire An II (24 October 1793), in their modern French
/// orthography.
///
/// The complementary days were the *sansculottides* in the decree of 1793
/// and the *jours complémentaires* from An III; the earlier name is used
/// here because the calendar's month numbering in this crate follows the
/// 1793 form.
pub const MONTHS: [&str; 13] = [
    "Vendémiaire",
    "Brumaire",
    "Frimaire",
    "Nivôse",
    "Pluviôse",
    "Ventôse",
    "Germinal",
    "Floréal",
    "Prairial",
    "Messidor",
    "Thermidor",
    "Fructidor",
    "Sansculottides",
];

/// The ten days of the *décade*, Primidi through Décadi, from the same
/// decree.
pub const DECADE_DAYS: [&str; 10] = [
    "Primidi", "Duodi", "Tridi", "Quartidi", "Quintidi", "Sextidi", "Septidi", "Octidi", "Nonidi",
    "Décadi",
];

impl Calendar for ArithmeticFrenchRepublicanCalendar {
    type Date = FrenchRepublicanDate;

    /// Twelve months of thirty days, a thirteenth of complementary days,
    /// and a *décade* of ten — not a week of seven.
    ///
    /// The ten-day cycle is the other reason `hc_calendar::shape` exists:
    /// the seven-valued `hc_calendar::Weekday` has nowhere to put Primidi
    /// through Décadi, so the *décade* is declared as a cycle of its own.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        use hc_calendar::shape::{CycleShape, MONTH};
        const SHAPE: &[CycleShape] = &[
            CycleShape::named(MONTH, &MONTHS),
            CycleShape::named("decade-day", &DECADE_DAYS),
        ];
        SHAPE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    /// In force from the decree of 1793 until Napoleon abolished it at the
    /// end of An XIV, 31 December 1805. The Paris Commune revived it for
    /// eighteen days in 1871, which this does not model. Everything outside
    /// those twelve years is the arithmetic extension of an idea, which is
    /// what this module is for.
    fn usage(&self) -> hc_calendar::Usage {
        match (
            gregorian::to_fixed(1793, 10, 24),
            gregorian::to_fixed(1805, 12, 31),
        ) {
            (Ok(from), Ok(until)) => hc_calendar::Usage::between(from, until, USAGE_SOURCE),
            _ => hc_calendar::Usage::UNRECORDED,
        }
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("french-republican-arithmetic"),
            english_name: "French Republican (arithmetic)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["fr"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(FrenchRepublicanDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let fields = DateFields::ymd(date.year, date.month, date.day).with_era(ERA);
        match date.decade() {
            None => Ok(fields),
            Some((decade, day)) => fields
                .with_extra("decade", i64::from(decade))?
                .with_extra("day-of-decade", i64::from(day)),
        }
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        FrenchRepublicanDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_republic_begins_on_the_autumn_equinox_of_1792() {
        // 1 Vendémiaire An I is 22 September 1792, the day the Republic was
        // proclaimed and, that year, the day of the true equinox at Paris.
        assert_eq!(EPOCH, gregorian::to_fixed(1792, 9, 22).unwrap());
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        assert_eq!(from_fixed(EPOCH), Ok((1, 1, 1)));
    }

    #[test]
    fn the_first_three_years_match_the_calendar_france_used() {
        // The equinox and the arithmetic rule agree until the first sextile.
        assert_eq!(to_fixed(2, 1, 1), gregorian::to_fixed(1793, 9, 22));
        assert_eq!(to_fixed(3, 1, 1), gregorian::to_fixed(1794, 9, 22));
    }

    #[test]
    fn the_arithmetic_rule_diverges_from_the_equinox_at_an_iv() {
        // France put the sextile at the end of An III, so An IV began on
        // 23 September 1795. The Romme rule puts it at the end of An IV, so
        // this calendar says 22 September. Both are recorded here because
        // the difference is the whole reason the variant is named.
        assert!(!is_leap_year(3));
        assert!(is_leap_year(4));
        assert_eq!(to_fixed(4, 1, 1), gregorian::to_fixed(1795, 9, 22));
        let historical = gregorian::to_fixed(1795, 9, 23).unwrap();
        assert_eq!(to_fixed(4, 1, 1).unwrap().0 + 1, historical.0);
    }

    #[test]
    fn the_leap_rule_and_the_year_length_agree_everywhere() {
        for year in MIN_YEAR..MAX_YEAR {
            let length = to_fixed(year + 1, 1, 1).unwrap().0 - to_fixed(year, 1, 1).unwrap().0;
            assert_eq!(length, i64::from(days_in_year(year)), "An {year}");
        }
    }

    #[test]
    fn the_century_and_millennium_exceptions_are_both_exercised() {
        assert!(is_leap_year(400)); // multiple of 400, not of 4000
        assert!(!is_leap_year(100));
        assert!(!is_leap_year(200));
        assert!(!is_leap_year(300));
        assert!(!is_leap_year(4_000));
        assert!(is_leap_year(800));
        // The mean year the rule produces, for comparison with the
        // Gregorian 365.2425.
        let mean: f64 = 1_460_969.0 / 4_000.0;
        assert!((mean - 365.242_25).abs() < 1e-9, "mean year {mean}");
    }

    #[test]
    fn the_sansculottides_are_five_days_or_six() {
        assert_eq!(days_in_month(3, 13), Some(5));
        assert_eq!(days_in_month(4, 13), Some(6));
        assert_eq!(days_in_month(4, 12), Some(30));
        assert_eq!(days_in_month(4, 14), None);
        assert_eq!(to_fixed(3, 13, 6), Err(CalendarError::DayOutOfRange));
        assert!(to_fixed(4, 13, 6).is_ok());
        assert!(
            FrenchRepublicanDate::new(4, 13, 6)
                .unwrap()
                .is_complementary_day()
        );
    }

    #[test]
    fn the_month_divides_into_three_decades() {
        let date = FrenchRepublicanDate::new(1, 1, 1).unwrap();
        assert_eq!(date.decade(), Some((1, 1)));
        assert_eq!(
            FrenchRepublicanDate::new(1, 1, 10).unwrap().decade(),
            Some((1, 10))
        );
        assert_eq!(
            FrenchRepublicanDate::new(1, 1, 11).unwrap().decade(),
            Some((2, 1))
        );
        assert_eq!(
            FrenchRepublicanDate::new(1, 1, 30).unwrap().decade(),
            Some((3, 10))
        );
        assert_eq!(FrenchRepublicanDate::new(1, 13, 1).unwrap().decade(), None);
    }

    #[test]
    fn the_abolition_year_is_an_xiv() {
        // Napoleon's decree returned France to the Gregorian calendar on
        // 1 January 1806, part-way through An XIV.
        let last = gregorian::to_fixed(1805, 12, 31).unwrap();
        let (year, _, _) = from_fixed(last).unwrap();
        assert_eq!(year, ABOLITION_YEAR);
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(109) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_the_years_it_was_in_force_round_trips() {
        let start = to_fixed(1, 1, 1).unwrap().0;
        let end = to_fixed(20, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = ArithmeticFrenchRepublicanCalendar;
        for rd in (EARLIEST.0..=LATEST.0).step_by(877) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(
            calendar.meta().id,
            CalendarId("french-republican-arithmetic")
        );
    }

    #[test]
    fn dates_outside_the_supported_range_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            ArithmeticFrenchRepublicanCalendar
                .from_fields(&DateFields::ymd(1, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }
}
