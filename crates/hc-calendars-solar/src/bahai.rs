//! The Badíʿ (Bahá'í) calendar, arithmetic Western variant.
//!
//! Nineteen months of nineteen days, an intercalary period of four or five
//! days — the *Ayyám-i-Há* — inserted before the last month, and the last
//! month, ʻAláʼ, kept as the fast. Nineteen nineteens is 361, so a
//! *Váḥid* is nineteen years and a *Kull-i-Shayʼ* is nineteen Váḥids, 361
//! years. The era begins with the declaration of the Báb in 1844.
//!
//! # Which variant this is, and which it is not
//!
//! Since Naw-Rúz 172 B.E. (2015) the calendar has been unified worldwide on
//! **astronomical** rules: the year begins on the day whose sunset follows
//! the March equinox as measured at Tehran, and the length of Ayyám-i-Há
//! follows from where the next Naw-Rúz lands. That determination needs the
//! apparent solar longitude and a sunset time for a specific meridian.
//! **It is a documented gap in this crate**: `hc-astro` will supply it, and
//! until then no arithmetic here can claim to be the Badíʿ calendar as
//! currently observed.
//!
//! What this module implements is the calendar as it was kept in the West
//! before 2015: Naw-Rúz pinned to 21 March in the Gregorian calendar, and
//! Ayyám-i-Há taking five days exactly when the following Gregorian year is
//! a leap year — which is the same thing as saying that the Badíʿ year
//! inherits the Gregorian year's length. That rule is exact, and it is what
//! Bahá'í publications in Europe and the Americas used for over a century.
//!
//! A second astronomical fact is not modelled either: a Badíʿ day runs from
//! sunset to sunset, so a date here names the fixed day a Badíʿ day *ends*
//! in, as is conventional when tabulating the calendar.
//!
//! Month names — Bahá, Jalál, Jamál … — belong to `hc-i18n`.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};

use crate::gregorian;

/// The Gregorian year in which Badíʿ year zero would begin, so that Badíʿ
/// year 1 begins in 1844.
pub const GREGORIAN_YEAR_OFFSET: i64 = 1_843;

/// The month number this crate uses for the intercalary days.
///
/// Ayyám-i-Há is not a month: it has no name of the nineteen, no fixed
/// length, and it sits between months 18 and 19. Zero is the conventional
/// number for it in tabulations, and [`DateFields`] carries it as an
/// intercalary repetition of month 18.
pub const AYYAM_I_HA: u8 = 0;

/// The era code of the Badíʿ era.
pub const ERA: &str = "BE";

/// The number of months in a year, and of years in a Váḥid.
pub const NINETEEN: i64 = 19;

/// The number of years in a Kull-i-Shayʼ.
pub const KULL_I_SHAY_YEARS: i64 = 361;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// The first Badíʿ year governed by the astronomical rules this module does
/// not implement.
pub const FIRST_ASTRONOMICAL_YEAR: i64 = 172;

/// The fixed day of Naw-Rúz of `year`, without validation.
const fn new_year_raw(year: i64) -> i64 {
    match gregorian::to_fixed(year + GREGORIAN_YEAR_OFFSET, 3, 21) {
        Ok(rd) => rd.0,
        // Unreachable inside this calendar's year bounds.
        Err(_) => 0,
    }
}

/// The fixed day of Naw-Rúz, the first day of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(Rd(new_year_raw(year)))
}

/// The fixed day of the first Naw-Rúz, 21 March 1844.
pub const EPOCH: Rd = Rd(new_year_raw(1));

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = EPOCH;

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    (new_year_raw(year + 1) - new_year_raw(year)) as u16
}

/// Whether Ayyám-i-Há takes a fifth day in `year`.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    days_in_year(year) == 366
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `0..=19`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        AYYAM_I_HA => Some(if is_leap_year(year) { 5 } else { 4 }),
        1..=19 => Some(19),
        _ => None,
    }
}

/// Days elapsed in the year before the first day of `month`.
const fn days_before_month(year: i64, month: u8) -> i64 {
    match month {
        AYYAM_I_HA => 18 * NINETEEN,
        19 => 18 * NINETEEN + if is_leap_year(year) { 5 } else { 4 },
        _ => (month as i64 - 1) * NINETEEN,
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
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match crate::common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(new_year_raw(year)
            + days_before_month(year, month)
            + day as i64
            - 1)),
    }
}

/// The Badíʿ year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    let (gregorian_year, _, _) = gregorian::from_fixed(rd)?;
    let mut year = gregorian_year - GREGORIAN_YEAR_OFFSET;
    if year > MAX_YEAR {
        year = MAX_YEAR;
    }
    if rd.0 < new_year_raw(year) {
        year -= 1;
    }
    let day_of_year = rd.0 - new_year_raw(year);
    let intercalary = i64::from(days_in_month(year, AYYAM_I_HA).unwrap_or(4));
    let (month, day) = if day_of_year < 18 * NINETEEN {
        (
            (day_of_year.div_euclid(NINETEEN) + 1) as u8,
            day_of_year.rem_euclid(NINETEEN) + 1,
        )
    } else if day_of_year < 18 * NINETEEN + intercalary {
        (AYYAM_I_HA, day_of_year - 18 * NINETEEN + 1)
    } else {
        (19, day_of_year - 18 * NINETEEN - intercalary + 1)
    };
    Ok((year, month, day as u8))
}

/// A Badíʿ date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BahaiDate {
    /// The year of the Badíʿ era, counting from 1.
    pub year: i64,
    /// The month, 1 through 19, or [`AYYAM_I_HA`] for the intercalary days.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl BahaiDate {
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

    /// The Kull-i-Shayʼ, Váḥid and year within the Váḥid, each counting
    /// from 1.
    #[must_use]
    pub const fn cycles(self) -> (i64, i64, i64) {
        let elapsed = self.year - 1;
        (
            elapsed.div_euclid(KULL_I_SHAY_YEARS) + 1,
            elapsed.rem_euclid(KULL_I_SHAY_YEARS).div_euclid(NINETEEN) + 1,
            elapsed.rem_euclid(NINETEEN) + 1,
        )
    }

    /// Whether this date falls in the intercalary days.
    #[must_use]
    pub const fn is_intercalary(self) -> bool {
        self.month == AYYAM_I_HA
    }

    /// Whether this date falls in the month of the fast, ʻAláʼ.
    #[must_use]
    pub const fn is_fast(self) -> bool {
        self.month == 19
    }

    /// Whether the astronomical rules adopted in 2015 govern this year, in
    /// which case this calendar is an approximation rather than an answer.
    #[must_use]
    pub const fn needs_astronomical_rules(self) -> bool {
        self.year >= FIRST_ASTRONOMICAL_YEAR
    }
}

/// The arithmetic Western Badíʿ calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ArithmeticBahaiCalendar;

impl Calendar for ArithmeticBahaiCalendar {
    type Date = BahaiDate;

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("bahai-arithmetic"),
            english_name: "Badíʿ (arithmetic)",
            year_kind: YearKind::EpochForward,
            // Ayyám-i-Há is carried as an intercalary repetition of month
            // 18, which is what makes this true.
            has_leap_months: true,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
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
        let (major, cycle, year_of_cycle) = date.cycles();
        let month = if date.is_intercalary() {
            Month::leap(18)
        } else {
            Month::regular(date.month)
        };
        let mut fields = DateFields::ymd(date.year, 1, date.day).with_era(ERA);
        fields.month = Some(month);
        fields
            .with_extra("kull-i-shay", major)?
            .with_extra("vahid", cycle)?
            .with_extra("year-of-vahid", year_of_cycle)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        let ordinal = if month.leap {
            if month.ordinal != 18 {
                return Err(CalendarError::MonthOutOfRange);
            }
            AYYAM_I_HA
        } else {
            month.ordinal
        };
        BahaiDate::new(fields.year, ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_era_begins_with_the_declaration_of_the_bab() {
        // 1 Bahá 1 B.E. is 21 March 1844.
        assert_eq!(EPOCH, gregorian::to_fixed(1844, 3, 21).unwrap());
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        assert_eq!(from_fixed(EPOCH), Ok((1, 1, 1)));
    }

    #[test]
    fn naw_ruz_is_pinned_to_the_twenty_first_of_march() {
        // The defining simplification of the Western variant.
        for year in [1, 100, 171, 172, 180] {
            let naw_ruz = new_year(year).unwrap();
            let (gregorian_year, month, day) = gregorian::from_fixed(naw_ruz).unwrap();
            assert_eq!((month, day), (3, 21), "B.E. {year}");
            assert_eq!(gregorian_year, year + GREGORIAN_YEAR_OFFSET);
        }
    }

    #[test]
    fn the_year_is_nineteen_nineteens_plus_the_intercalary_days() {
        for year in (1..500).step_by(3) {
            let length = i64::from(days_in_year(year));
            let intercalary = i64::from(days_in_month(year, AYYAM_I_HA).unwrap());
            assert_eq!(length, 19 * 19 + intercalary, "B.E. {year}");
            assert!(intercalary == 4 || intercalary == 5);
        }
    }

    #[test]
    fn the_fifth_intercalary_day_appears_before_a_gregorian_leap_year() {
        // Ayyám-i-Há runs across the end of February, so it gains its fifth
        // day exactly when the February it spans has twenty-nine.
        for year in 150..250i64 {
            let gregorian_year = year + GREGORIAN_YEAR_OFFSET + 1;
            assert_eq!(
                is_leap_year(year),
                gregorian::is_leap_year(gregorian_year),
                "B.E. {year} against {gregorian_year}"
            );
        }
        assert_eq!(days_in_month(156, AYYAM_I_HA), Some(5)); // 2000 is leap
        assert_eq!(days_in_month(56, AYYAM_I_HA), Some(4)); // 1900 is not
    }

    #[test]
    fn the_fast_is_the_nineteen_days_before_naw_ruz() {
        for year in [150, 171, 200] {
            let first_of_ala = to_fixed(year, 19, 1).unwrap();
            let next_naw_ruz = new_year(year + 1).unwrap();
            assert_eq!(next_naw_ruz.0 - first_of_ala.0, 19, "B.E. {year}");
            let (_, month, day) = gregorian::from_fixed(first_of_ala).unwrap();
            assert_eq!((month, day), (3, 2), "B.E. {year}");
        }
        assert!(BahaiDate::new(180, 19, 1).unwrap().is_fast());
    }

    #[test]
    fn the_cycles_are_nineteen_and_nineteen_squared() {
        assert_eq!(BahaiDate::new(1, 1, 1).unwrap().cycles(), (1, 1, 1));
        assert_eq!(BahaiDate::new(19, 1, 1).unwrap().cycles(), (1, 1, 19));
        assert_eq!(BahaiDate::new(20, 1, 1).unwrap().cycles(), (1, 2, 1));
        assert_eq!(BahaiDate::new(361, 1, 1).unwrap().cycles(), (1, 19, 19));
        assert_eq!(BahaiDate::new(362, 1, 1).unwrap().cycles(), (2, 1, 1));
        // The year opening in 2026 is B.E. 183: still the first
        // Kull-i-Shayʼ, its tenth Váḥid, the twelfth year of that Váḥid.
        assert_eq!(BahaiDate::new(183, 1, 1).unwrap().cycles(), (1, 10, 12));
    }

    #[test]
    fn the_intercalary_days_sit_between_months_eighteen_and_nineteen() {
        let year = 180;
        let last_of_eighteen = to_fixed(year, 18, 19).unwrap();
        let first_intercalary = to_fixed(year, AYYAM_I_HA, 1).unwrap();
        let first_of_nineteen = to_fixed(year, 19, 1).unwrap();
        assert_eq!(first_intercalary.0, last_of_eighteen.0 + 1);
        let length = i64::from(days_in_month(year, AYYAM_I_HA).unwrap());
        assert_eq!(first_of_nineteen.0, first_intercalary.0 + length);
        assert!(
            BahaiDate::new(year, AYYAM_I_HA, 1)
                .unwrap()
                .is_intercalary()
        );
        assert_eq!(days_in_month(year, 20), None);
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(113) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = new_year(150).unwrap().0;
        let end = new_year(200).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = ArithmeticBahaiCalendar;
        for rd in (EARLIEST.0..=LATEST.0).step_by(367) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("bahai-arithmetic"));
        assert!(calendar.meta().has_leap_months);
    }

    #[test]
    fn the_astronomical_years_are_flagged_rather_than_guessed_at() {
        assert!(
            !BahaiDate::new(171, 1, 1)
                .unwrap()
                .needs_astronomical_rules()
        );
        assert!(
            BahaiDate::new(172, 1, 1)
                .unwrap()
                .needs_astronomical_rules()
        );
        assert!(
            BahaiDate::new(183, 1, 1)
                .unwrap()
                .needs_astronomical_rules()
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
        assert_eq!(to_fixed(180, 1, 20), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(180, 20, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(
            ArithmeticBahaiCalendar.from_fields(&DateFields::ymd_leap_month(180, 5, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
    }
}
