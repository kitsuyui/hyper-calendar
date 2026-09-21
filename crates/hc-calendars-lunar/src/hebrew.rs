//! The arithmetic Hebrew calendar — CLDR `hebrew`.
//!
//! A lunisolar calendar with no astronomy in it. Since Hillel II the months
//! have been fixed by calculation from the *molad*, the mean conjunction, and
//! the calculation is exact arithmetic on integers: this module computes no
//! solar longitude and calls nothing in `hc-astro`.
//!
//! # How a year is built
//!
//! 1. **The molad.** The mean synodic month is taken as exactly 29 days,
//!    12 hours and 793 *ḥalakim* (parts), a part being 1/1080 of an hour.
//!    That is 29.530594 days, 0.4 seconds longer than the true mean synodic
//!    month, so the molad drifts about a day later every 216 years.
//! 2. **The Metonic cycle.** 235 months are fitted into 19 years, with years
//!    3, 6, 8, 11, 14, 17 and 19 of each cycle carrying a thirteenth month.
//!    235 mean months are 6939.69 days against 19 mean tropical years of
//!    6939.60, so Passover creeps later by about a day per 216 years too.
//! 3. **The four dehiyyot.** Rosh Hashanah is then postponed, by the rules
//!    listed on [`new_year`], until it falls on a permitted weekday and
//!    leaves the year a permitted length.
//! 4. **The year's length follows.** A common year runs 353, 354 or 355 days
//!    and a leap year 383, 384 or 385, called *deficient*, *regular* and
//!    *complete*. The three are distinguished by Ḥeshvan (29 or 30 days) and
//!    Kislev (30 or 29); every other month has a fixed length.
//!
//! # Month numbering
//!
//! The year number changes at Tishrei, so months here are numbered from
//! Tishrei: 1 Tishrei, 2 Ḥeshvan, 3 Kislev, 4 Ṭevet, 5 Shevaṭ, 6 Adar,
//! 7 Nisan, 8 Iyyar, 9 Sivan, 10 Tammuz, 11 Av, 12 Elul.
//!
//! In a leap year the extra month is inserted after Shevaṭ, so it is
//! [`Month::leap(5)`](hc_calendar::Month::leap) — CLDR's `M05L` — and it is
//! called **Adar I**. The ordinary month 6 is then **Adar II**, the one that
//! carries Purim and the one the Talmud treats as "the" Adar. Writing Adar I
//! as the intercalary repetition of Shevaṭ rather than of Adar is what keeps
//! a single [`Month`](hc_calendar::Month) type usable for this calendar and
//! for the Chinese one, where the leap month likewise follows the month it
//! is named after.
//!
//! Note that the religious year is counted from Nisan, which is why the
//! internal arithmetic below — and Reingold and Dershowitz's presentation of
//! it — numbers Nisan 1 and Tishrei 7. Only the internal numbering does; the
//! public API is the Tishrei-first one throughout.
//!
//! # Accuracy
//!
//! Exact. There is nothing to approximate: the rules are arithmetic and this
//! is those rules. What the calendar is not is *astronomically* right, and
//! the drift figures above say by how much it is not.

use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};

/// The machine identifier CLDR uses for this calendar.
pub const ID: CalendarId = CalendarId("hebrew");

/// The era code of the Anno Mundi era.
pub const ERA: &str = "AM";

/// The fixed day of 1 Tishrei AM 1, which is 7 October 3761 BCE in the Julian
/// calendar.
///
/// This is *epoch*, not observation: the year count was fixed retrospectively
/// and AM 1 has no events in it.
pub const EPOCH: Rd = Rd(-1_373_427);

/// The earliest Hebrew year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest Hebrew year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// Parts (1/1080 of an hour) in a day.
pub const PARTS_PER_DAY: i64 = 25_920;

/// The mean synodic month in parts: 29 days, 12 hours and 793 parts.
pub const MOLAD_INTERVAL_PARTS: i64 = 29 * PARTS_PER_DAY + 13_753;

/// Years in the Metonic cycle.
pub const METONIC_YEARS: i64 = 19;

/// Months in the Metonic cycle.
pub const METONIC_MONTHS: i64 = 235;

/// The seven years of each Metonic cycle that carry a thirteenth month.
pub const LEAP_YEARS_IN_CYCLE: [u8; 7] = [3, 6, 8, 11, 14, 17, 19];

/// Whether `year` carries a thirteenth month.
///
/// `(7y + 1) mod 19 < 7` is the closed form of
/// [`LEAP_YEARS_IN_CYCLE`]; the test in
/// [`the_closed_form_matches_the_listed_leap_years`](self) checks that.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    (7 * year + 1).rem_euclid(METONIC_YEARS) < 7
}

/// The highest *internal* month number of `year`: 13 in a leap year, else 12.
///
/// Internal numbering runs Nisan 1 to Adar 12, with Adar II as 13.
const fn last_internal_month(year: i64) -> u8 {
    if is_leap_year(year) { 13 } else { 12 }
}

/// Months elapsed between the molad of Tishrei AM 1 and the molad of Tishrei
/// of `year`.
const fn months_before_year(year: i64) -> i64 {
    (METONIC_MONTHS * year - 234).div_euclid(METONIC_YEARS)
}

/// Days elapsed from the epoch to Rosh Hashanah of `year`, with the first two
/// dehiyyot applied and the other two still to come.
///
/// The constant 12 084 is two things added together: 5 604 parts, the molad
/// of Tishrei AM 1 (*BaHaRaD* — Monday, 5 hours and 204 parts after the
/// evening), and 6 480 parts, which is six hours. Adding those six hours
/// before truncating to whole days *is* the first dehiyyah, **Molad Zaken**:
/// a molad at or after noon pushes Rosh Hashanah to the next day.
///
/// The weekday test that follows is the second, **Lo ADU Rosh**: Rosh
/// Hashanah may not fall on Sunday, Wednesday or Friday, so that Yom Kippur
/// never abuts a Sabbath and Hoshana Rabbah never falls on one.
const fn elapsed_days(year: i64) -> i64 {
    let months = months_before_year(year);
    let parts = 12_084 + 13_753 * months;
    let day = 29 * months + parts.div_euclid(PARTS_PER_DAY);
    // `day + 1` is congruent to the fixed day modulo 7, and 3x mod 7 < 3
    // selects exactly x congruent to 0, 3 or 5 — Sunday, Wednesday, Friday.
    if (3 * (day + 1)).rem_euclid(7) < 3 {
        day + 1
    } else {
        day
    }
}

/// The last two dehiyyot, as a correction in days applied to `year`'s start.
///
/// Both exist to stop a year taking an impossible length.
///
/// * **GaTaRaD** (two days): without it `year` would run 356 days, which no
///   Hebrew year may. It bites when the molad of a common year falls on a
///   Tuesday at or after 9 hours and 204 parts.
/// * **BeTuTaKaPoT** (one day): without it the *preceding* year would run
///   382 days. It bites when the molad of the year after a leap year falls
///   on a Monday at or after 15 hours and 589 parts.
const fn new_year_delay(year: i64) -> i64 {
    let before = elapsed_days(year - 1);
    let this = elapsed_days(year);
    let after = elapsed_days(year + 1);
    if after - this == 356 {
        2
    } else if this - before == 382 {
        1
    } else {
        0
    }
}

/// The fixed day of Rosh Hashanah, 1 Tishrei of `year`.
///
/// All four dehiyyot are applied here: two inside [`elapsed_days`] and two
/// inside [`new_year_delay`].
#[must_use]
pub const fn new_year(year: i64) -> Rd {
    Rd(EPOCH.0 + elapsed_days(year) + new_year_delay(year))
}

/// The number of days in `year`: 353, 354 or 355, or 383, 384 or 385 in a
/// leap year.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    (new_year(year + 1).0 - new_year(year).0) as u16
}

/// How a year's variable months are set, which is what makes up the three
/// lengths a common or leap year can take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YearKindOfLength {
    /// 353 or 383 days: Kislev is short, at 29 days.
    Deficient,
    /// 354 or 384 days: Ḥeshvan 29 and Kislev 30, the ordinary arrangement.
    Regular,
    /// 355 or 385 days: Ḥeshvan is long, at 30 days.
    Complete,
}

/// Whether `year` is deficient, regular or complete.
///
/// The three are told apart by the year length modulo ten, which is 3, 4 or 5
/// for both a common and a leap year.
#[must_use]
pub const fn year_kind(year: i64) -> YearKindOfLength {
    match days_in_year(year) % 10 {
        3 => YearKindOfLength::Deficient,
        5 => YearKindOfLength::Complete,
        _ => YearKindOfLength::Regular,
    }
}

/// Whether Ḥeshvan has 30 days rather than 29 in `year`.
#[must_use]
pub const fn is_long_heshvan(year: i64) -> bool {
    days_in_year(year) % 10 == 5
}

/// Whether Kislev has 29 days rather than 30 in `year`.
#[must_use]
pub const fn is_short_kislev(year: i64) -> bool {
    days_in_year(year) % 10 == 3
}

/// The length of an internal month (Nisan 1 … Adar 12, Adar II 13).
const fn internal_month_length(year: i64, month: u8) -> u8 {
    match month {
        // Iyyar, Tammuz, Elul, Ṭevet and Adar II are always short.
        2 | 4 | 6 | 10 | 13 => 29,
        // Adar I, which is always 30, in a leap year; plain Adar otherwise.
        12 if is_leap_year(year) => 30,
        12 => 29,
        8 if is_long_heshvan(year) => 30,
        8 => 29,
        9 if is_short_kislev(year) => 29,
        _ => 30,
    }
}

/// Translate a public, Tishrei-first month into the internal Nisan-first
/// number, or `None` when the month does not exist in `year`.
const fn internal_month(year: i64, month: Month) -> Option<u8> {
    let leap = is_leap_year(year);
    let ordinal = month.ordinal;
    if month.leap {
        // The only intercalary month is Adar I, written as the repetition of
        // Shevaṭ, and it exists only in a leap year.
        return if leap && ordinal == 5 { Some(12) } else { None };
    }
    if ordinal == 0 || ordinal > 12 {
        return None;
    }
    if leap {
        match ordinal {
            1..=5 => Some(ordinal + 6),
            6 => Some(13),
            _ => Some(ordinal - 6),
        }
    } else if ordinal <= 6 {
        Some(ordinal + 6)
    } else {
        Some(ordinal - 6)
    }
}

/// Translate an internal month number back into the public one.
const fn public_month(year: i64, month: u8) -> Month {
    let leap = is_leap_year(year);
    if leap && month == 12 {
        return Month::leap(5);
    }
    if leap && month == 13 {
        return Month::regular(6);
    }
    if month >= 7 {
        Month::regular(month - 6)
    } else {
        Month::regular(month + 6)
    }
}

/// The number of days in `month` of `year`, or `None` when that month does
/// not exist in that year.
///
/// Asking for `Month::leap(5)` in a common year is a legitimate question with
/// the answer "there is no such month", so it returns `None` rather than
/// failing.
#[must_use]
pub const fn days_in_month(year: i64, month: Month) -> Option<u8> {
    match internal_month(year, month) {
        None => None,
        Some(internal) => Some(internal_month_length(year, internal)),
    }
}

/// The number of months in `year`: 12, or 13 in a leap year.
#[must_use]
pub const fn months_in_year(year: i64) -> u8 {
    last_internal_month(year)
}

/// Days elapsed since Rosh Hashanah before the first of an internal month.
const fn days_before_internal_month(year: i64, month: u8) -> i64 {
    let last = last_internal_month(year);
    let mut total = 0i64;
    // A month before Tishrei falls in the second half of the year, so the
    // whole autumn-to-Adar stretch precedes it.
    let mut current = 7u8;
    while current <= last {
        if month >= 7 && current >= month {
            break;
        }
        total += internal_month_length(year, current) as i64;
        current += 1;
    }
    if month < 7 {
        let mut spring = 1u8;
        while spring < month {
            total += internal_month_length(year, spring) as i64;
            spring += 1;
        }
    }
    total
}

/// The fixed day of a Hebrew date, taking the internal month number.
const fn to_fixed_internal(year: i64, month: u8, day: u8) -> i64 {
    new_year(year).0 + days_before_internal_month(year, month) + day as i64 - 1
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = new_year(MIN_YEAR);

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year(MAX_YEAR + 1).0 - 1);

/// The fixed day of a Hebrew date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`], [`CalendarError::MonthOutOfRange`] when the
/// month does not exist in that year — asking for Adar I in a common year is
/// the usual case — and [`CalendarError::DayOutOfRange`] otherwise.
pub const fn to_fixed(year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    let internal = match internal_month(year, month) {
        None => return Err(CalendarError::MonthOutOfRange),
        Some(internal) => internal,
    };
    let length = internal_month_length(year, internal);
    if day == 0 || day > length {
        return Err(CalendarError::DayOutOfRange);
    }
    Ok(Rd(to_fixed_internal(year, internal, day)))
}

/// The Hebrew year containing a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside the supported range.
pub const fn year_from_fixed(rd: Rd) -> CalendarResult<i64> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    // The mean Hebrew year is 35 975 351/98 496 days, a shade over 365.2468.
    // Dividing by it can only undershoot by a year or two, so the loop that
    // follows is short.
    let mut year = 1 + (rd.0 - EPOCH.0) * 98_496 / 35_975_351;
    if year < MIN_YEAR {
        year = MIN_YEAR;
    }
    while year > MIN_YEAR && new_year(year).0 > rd.0 {
        year -= 1;
    }
    while new_year(year + 1).0 <= rd.0 {
        year += 1;
    }
    Ok(year)
}

/// The Hebrew year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside the supported range.
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, Month, u8)> {
    let year = match year_from_fixed(rd) {
        Err(error) => return Err(error),
        Ok(year) => year,
    };
    // A date before 1 Nisan is in the Tishrei-to-Adar half of the year.
    let mut internal = if rd.0 < to_fixed_internal(year, 1, 1) {
        7u8
    } else {
        1u8
    };
    while internal <= 13
        && rd.0 > to_fixed_internal(year, internal, internal_month_length(year, internal))
    {
        internal += 1;
    }
    let day = (rd.0 - to_fixed_internal(year, internal, 1) + 1) as u8;
    Ok((year, public_month(year, internal), day))
}

/// The molad — the mean conjunction — of an internal month of `year`, as a
/// moment in Jerusalem mean solar time.
///
/// The Hebrew day begins at sunset, and the molad is quoted from 6 pm the
/// previous evening; both conventions are folded into the fractional part
/// here, which is why the value is a moment and not a fixed day. The result
/// is a *mean* conjunction and can sit up to about fifteen hours from the
/// true one.
fn molad_internal(year: i64, month: u8) -> Moment {
    let shifted = if month < 7 { year + 1 } else { year };
    let months = month as i64 - 7 + months_before_year(shifted);
    Moment(
        EPOCH.0 as f64 - 876.0 / PARTS_PER_DAY as f64
            + months as f64 * (MOLAD_INTERVAL_PARTS as f64 / PARTS_PER_DAY as f64),
    )
}

/// The molad of a month, as a moment in Jerusalem mean solar time.
///
/// # Errors
///
/// Returns [`CalendarError::MonthOutOfRange`] when the month does not exist
/// in `year`.
pub fn molad(year: i64, month: Month) -> CalendarResult<Moment> {
    let internal = internal_month(year, month).ok_or(CalendarError::MonthOutOfRange)?;
    Ok(molad_internal(year, internal))
}

/// The fixed day of 15 Nisan, the first day of Passover, in `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside the supported range.
pub const fn passover(year: i64) -> CalendarResult<Rd> {
    to_fixed(year, Month::regular(7), 15)
}

/// How many days of the Omer are counted between Passover and Shavuot.
pub const OMER_DAYS: u8 = 49;

/// The day of the Omer that a fixed day carries, from 1 on 16 Nisan to 49 on
/// 5 Sivan, or `None` outside the count.
///
/// The count is recited on the evening that begins each of those days, so a
/// caller displaying it for an evening should ask about the following day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside the supported range.
pub const fn omer_day(rd: Rd) -> CalendarResult<Option<u8>> {
    let year = match year_from_fixed(rd) {
        Err(error) => return Err(error),
        Ok(year) => year,
    };
    let start = match passover(year) {
        Err(error) => return Err(error),
        Ok(start) => start,
    };
    let count = rd.0 - start.0;
    if count >= 1 && count <= OMER_DAYS as i64 {
        Ok(Some(count as u8))
    } else {
        Ok(None)
    }
}

/// The Omer count of a fixed day as the weeks and days it is spoken in.
///
/// Day 8, for example, is "one week and one day", so this returns `(1, 1)`.
///
/// # Errors
///
/// Propagates the range check of [`omer_day`].
pub const fn omer_weeks_and_days(rd: Rd) -> CalendarResult<Option<(u8, u8)>> {
    match omer_day(rd) {
        Err(error) => Err(error),
        Ok(None) => Ok(None),
        Ok(Some(count)) => Ok(Some((count / 7, count % 7))),
    }
}

/// The fixed day of a *birkat hachama* on or after `rd`.
///
/// The blessing of the sun is recited when the vernal *tekufah* of Shmuel
/// returns to the hour and weekday it held at creation, which the Talmudic
/// reckoning — a solar year of exactly 365¼ days — makes every 28 years
/// exactly. 28 × 365¼ is 10 227 days, and 10 227 is divisible by 7, so the
/// blessing always falls on a Wednesday.
///
/// Because the underlying year is the Julian one, the Gregorian date drifts:
/// it was 7 April through the nineteenth century, has been 8 April since
/// 1925, and will move to 9 April in 2121.
#[must_use]
pub const fn birkat_hachama_on_or_after(rd: Rd) -> Rd {
    let offset = rd.0 - BIRKAT_HACHAMA_ANCHOR.0;
    let cycles = offset.div_euclid(BIRKAT_HACHAMA_CYCLE_DAYS);
    let candidate = BIRKAT_HACHAMA_ANCHOR.0 + cycles * BIRKAT_HACHAMA_CYCLE_DAYS;
    if candidate >= rd.0 {
        Rd(candidate)
    } else {
        Rd(candidate + BIRKAT_HACHAMA_CYCLE_DAYS)
    }
}

/// Whether *birkat hachama* is recited on `rd`.
#[must_use]
pub const fn is_birkat_hachama(rd: Rd) -> bool {
    (rd.0 - BIRKAT_HACHAMA_ANCHOR.0).rem_euclid(BIRKAT_HACHAMA_CYCLE_DAYS) == 0
}

/// Days in the *birkat hachama* cycle: 28 Julian years.
pub const BIRKAT_HACHAMA_CYCLE_DAYS: i64 = 10_227;

/// A *birkat hachama* the world watched: 8 April 2009.
pub const BIRKAT_HACHAMA_ANCHOR: Rd = Rd(733_505);

/// A Hebrew date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HebrewDate {
    /// The year of the Anno Mundi era, counting from 1.
    pub year: i64,
    /// The month, numbered from Tishrei, with Adar I as `Month::leap(5)`.
    pub month: Month,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl HebrewDate {
    /// A validated Hebrew date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, month: Month, day: u8) -> CalendarResult<Self> {
        match to_fixed(year, month, day) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self { year, month, day }),
        }
    }

    /// Whether this date falls in Adar I, the intercalary month.
    #[must_use]
    pub const fn is_adar_i(self) -> bool {
        self.month.leap && self.month.ordinal == 5
    }

    /// Whether this date falls in Adar of a common year or Adar II of a leap
    /// year — the Adar that carries Purim.
    #[must_use]
    pub const fn is_adar_ii(self) -> bool {
        !self.month.leap && self.month.ordinal == 6
    }
}

/// The arithmetic Hebrew calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HebrewCalendar;

impl Calendar for HebrewCalendar {
    type Date = HebrewDate;

    /// The Hebrew day begins at sunset, so a Hebrew date covers the second
    /// half of one civil day and the first half of the next.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Hebrew",
            year_kind: YearKind::EpochForward,
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
        HebrewDate::new(fields.year, fields.require_month()?, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::civil;
    use hc_calendar::Weekday;

    fn date(year: i64, month: u8, day: u8) -> HebrewDate {
        HebrewDate {
            year,
            month: Month::regular(month),
            day,
        }
    }

    #[test]
    fn the_closed_form_matches_the_listed_leap_years() {
        for cycle in 0..50i64 {
            for position in 1..=19i64 {
                let year = cycle * 19 + position;
                let listed = LEAP_YEARS_IN_CYCLE.contains(&(position as u8));
                assert_eq!(is_leap_year(year), listed, "year {year}");
            }
        }
    }

    #[test]
    fn seven_years_in_nineteen_carry_a_thirteenth_month() {
        let leaps = (1..=19i64).filter(|year| is_leap_year(*year)).count();
        assert_eq!(leaps, 7);
        for year in 5_780..5_800i64 {
            assert_eq!(
                months_in_year(year),
                if is_leap_year(year) { 13 } else { 12 }
            );
        }
    }

    #[test]
    fn rosh_hashanah_5784_was_the_sixteenth_of_september_2023() {
        // A published anchor: 1 Tishrei 5784 fell on 2023-09-16.
        let rd = civil::to_rd(2023, 9, 16);
        assert_eq!(new_year(5_784), rd);
        assert_eq!(to_fixed(5_784, Month::regular(1), 1), Ok(rd));
        assert_eq!(from_fixed(rd), Ok((5_784, Month::regular(1), 1)));
    }

    #[test]
    fn passover_5784_was_the_twenty_third_of_april_2024() {
        // A published anchor: 15 Nisan 5784 fell on 2024-04-23.
        let rd = civil::to_rd(2024, 4, 23);
        assert_eq!(passover(5_784), Ok(rd));
        assert_eq!(from_fixed(rd), Ok((5_784, Month::regular(7), 15)));
    }

    #[test]
    fn the_year_5784_was_a_deficient_leap_year_of_383_days() {
        assert!(is_leap_year(5_784));
        assert_eq!(days_in_year(5_784), 383);
        assert_eq!(year_kind(5_784), YearKindOfLength::Deficient);
        assert!(is_short_kislev(5_784));
        assert!(!is_long_heshvan(5_784));
        assert_eq!(days_in_month(5_784, Month::regular(3)), Some(29));
        assert_eq!(days_in_month(5_784, Month::regular(2)), Some(29));
    }

    #[test]
    fn rosh_hashanah_never_falls_on_sunday_wednesday_or_friday() {
        for year in 1..=9_999i64 {
            let weekday = Weekday::from_rd(new_year(year));
            assert!(
                !matches!(
                    weekday,
                    Weekday::Sunday | Weekday::Wednesday | Weekday::Friday
                ),
                "year {year} began on {weekday:?}"
            );
        }
    }

    #[test]
    fn every_year_takes_one_of_the_six_permitted_lengths() {
        for year in 1..=9_999i64 {
            let length = days_in_year(year);
            if is_leap_year(year) {
                assert!((383..=385).contains(&length), "year {year} was {length}");
            } else {
                assert!((353..=355).contains(&length), "year {year} was {length}");
            }
        }
    }

    #[test]
    fn deficient_regular_and_complete_years_differ_only_in_heshvan_and_kislev() {
        for year in 5_700..5_800i64 {
            let heshvan = days_in_month(year, Month::regular(2)).expect("exists");
            let kislev = days_in_month(year, Month::regular(3)).expect("exists");
            match year_kind(year) {
                YearKindOfLength::Deficient => assert_eq!((heshvan, kislev), (29, 29)),
                YearKindOfLength::Regular => assert_eq!((heshvan, kislev), (29, 30)),
                YearKindOfLength::Complete => assert_eq!((heshvan, kislev), (30, 30)),
            }
            // Every other month is fixed.
            for ordinal in [1u8, 4, 5, 6, 7, 8, 9, 10, 11, 12] {
                let length = days_in_month(year, Month::regular(ordinal)).expect("exists");
                assert!((29..=30).contains(&length));
            }
        }
    }

    #[test]
    fn all_three_kinds_of_year_occur() {
        let mut deficient = 0;
        let mut regular = 0;
        let mut complete = 0;
        for year in 5_700..5_800i64 {
            match year_kind(year) {
                YearKindOfLength::Deficient => deficient += 1,
                YearKindOfLength::Regular => regular += 1,
                YearKindOfLength::Complete => complete += 1,
            }
        }
        assert!(deficient > 0 && regular > 0 && complete > 0);
        assert_eq!(deficient + regular + complete, 100);
    }

    #[test]
    fn adar_one_exists_only_in_a_leap_year_and_always_has_thirty_days() {
        for year in 5_700..5_800i64 {
            let adar_one = days_in_month(year, Month::leap(5));
            if is_leap_year(year) {
                assert_eq!(adar_one, Some(30), "year {year}");
                assert_eq!(days_in_month(year, Month::regular(6)), Some(29));
            } else {
                assert_eq!(adar_one, None, "year {year}");
                assert_eq!(days_in_month(year, Month::regular(6)), Some(29));
            }
        }
    }

    #[test]
    fn adar_one_falls_between_shevat_and_adar_two() {
        // 5784 is a leap year; Adar I runs from 30 Shevaṭ to 29 Adar I and
        // Adar II begins the next day.
        let shevat_end = to_fixed(5_784, Month::regular(5), 30).expect("Shevaṭ has 30 days");
        let adar_one = to_fixed(5_784, Month::leap(5), 1).expect("exists in a leap year");
        let adar_two = to_fixed(5_784, Month::regular(6), 1).expect("exists");
        assert_eq!(adar_one.0, shevat_end.0 + 1);
        assert_eq!(adar_two.0, adar_one.0 + 30);
        let date = HebrewDate {
            year: 5_784,
            month: Month::leap(5),
            day: 1,
        };
        assert!(date.is_adar_i());
        assert!(!date.is_adar_ii());
    }

    #[test]
    fn asking_for_adar_one_in_a_common_year_is_refused() {
        assert!(!is_leap_year(5_783));
        assert_eq!(
            to_fixed(5_783, Month::leap(5), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(5_784, Month::leap(4), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(5_784, Month::regular(13), 1),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn the_calendar_round_trips_over_forty_thousand_modern_days() {
        let calendar = HebrewCalendar;
        let start = new_year(5_700).0;
        for offset in 0..40_000i64 {
            let rd = Rd(start + offset);
            let hebrew = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(hebrew), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn the_calendar_round_trips_near_the_epoch_and_the_end_of_the_range() {
        let calendar = HebrewCalendar;
        for start in [EARLIEST.0, LATEST.0 - 5_000] {
            for offset in 0..5_000i64 {
                let rd = Rd(start + offset);
                let hebrew = calendar.from_fixed(rd).expect("in range");
                assert_eq!(calendar.to_fixed(hebrew), Ok(rd), "RD {rd}");
            }
        }
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
    fn months_run_to_twenty_nine_or_thirty_days_and_years_to_twelve_or_thirteen_months() {
        let calendar = HebrewCalendar;
        for year in 5_600..5_900i64 {
            let mut months = 0u8;
            let mut total = 0i64;
            let mut cursor = new_year(year);
            let end = new_year(year + 1);
            while cursor < end {
                let first = calendar.from_fixed(cursor).expect("in range");
                assert_eq!(first.day, 1, "expected the first of a month at {cursor}");
                let length = days_in_month(year, first.month).expect("exists") as i64;
                assert!((29..=30).contains(&length));
                months += 1;
                total += length;
                cursor = Rd(cursor.0 + length);
            }
            assert_eq!(cursor, end);
            assert_eq!(months, months_in_year(year), "year {year}");
            assert!(months == 12 || months == 13);
            assert_eq!(total, days_in_year(year) as i64);
        }
    }

    #[test]
    fn the_metonic_cycle_closes_after_two_hundred_and_thirty_five_months() {
        for cycle in 300..310i64 {
            let start = cycle * 19 + 1;
            let months: i64 = (start..start + 19).map(|y| months_in_year(y) as i64).sum();
            assert_eq!(months, METONIC_MONTHS, "cycle starting {start}");
        }
    }

    #[test]
    fn the_molad_advances_by_one_mean_synodic_month() {
        let interval = MOLAD_INTERVAL_PARTS as f64 / PARTS_PER_DAY as f64;
        assert!(
            (interval - 29.530_594).abs() < 1e-6,
            "interval was {interval}"
        );
        let first = molad(5_784, Month::regular(1)).expect("exists");
        let second = molad(5_784, Month::regular(2)).expect("exists");
        assert!((second.0 - first.0 - interval).abs() < 1e-9);
    }

    #[test]
    fn the_molad_stays_within_a_day_of_rosh_hashanah() {
        // The dehiyyot move Rosh Hashanah at most two days past the molad,
        // and never before it.
        for year in 5_700..5_800i64 {
            let molad_tishrei = molad(year, Month::regular(1)).expect("exists");
            let start = new_year(year).0 as f64;
            let delay = start - molad_tishrei.0;
            assert!(
                (-0.5..=2.5).contains(&delay),
                "year {year} was delayed by {delay}"
            );
        }
    }

    #[test]
    fn the_omer_runs_forty_nine_days_from_sixteen_nisan_to_five_sivan() {
        let first = to_fixed(5_784, Month::regular(7), 16).expect("16 Nisan exists");
        assert_eq!(omer_day(first), Ok(Some(1)));
        assert_eq!(omer_weeks_and_days(first), Ok(Some((0, 1))));
        let last = to_fixed(5_784, Month::regular(9), 5).expect("5 Sivan exists");
        assert_eq!(omer_day(last), Ok(Some(OMER_DAYS)));
        assert_eq!(omer_weeks_and_days(last), Ok(Some((7, 0))));
        assert_eq!(last.0 - first.0, 48);
        // Shavuot, 6 Sivan, is the day after the count ends.
        let shavuot = to_fixed(5_784, Month::regular(9), 6).expect("6 Sivan exists");
        assert_eq!(omer_day(shavuot), Ok(None));
        // 15 Nisan, the first day of Passover, is before the count starts.
        assert_eq!(omer_day(Rd(first.0 - 1)), Ok(None));
    }

    #[test]
    fn the_omer_is_forty_nine_days_long_in_every_year() {
        for year in 5_700..5_800i64 {
            let counted = (0..60i64)
                .map(|offset| Rd(passover(year).expect("valid").0 + offset))
                .filter(|rd| matches!(omer_day(*rd), Ok(Some(_))))
                .count();
            assert_eq!(counted, OMER_DAYS as usize, "year {year}");
        }
    }

    #[test]
    fn the_omer_speaks_weeks_and_days() {
        let start = to_fixed(5_784, Month::regular(7), 16).expect("exists");
        assert_eq!(omer_weeks_and_days(Rd(start.0 + 6)), Ok(Some((1, 0))));
        assert_eq!(omer_weeks_and_days(Rd(start.0 + 7)), Ok(Some((1, 1))));
        assert_eq!(omer_weeks_and_days(Rd(start.0 + 32)), Ok(Some((4, 5))));
    }

    #[test]
    fn birkat_hachama_falls_on_a_wednesday_every_twenty_eight_years() {
        assert_eq!(civil::from_rd(BIRKAT_HACHAMA_ANCHOR), (2009, 4, 8));
        assert!(is_birkat_hachama(BIRKAT_HACHAMA_ANCHOR));
        for cycle in -60..60i64 {
            let rd = Rd(BIRKAT_HACHAMA_ANCHOR.0 + cycle * BIRKAT_HACHAMA_CYCLE_DAYS);
            assert!(is_birkat_hachama(rd));
            assert_eq!(Weekday::from_rd(rd), Weekday::Wednesday, "cycle {cycle}");
        }
    }

    #[test]
    fn the_recent_and_next_birkat_hachama_are_the_published_ones() {
        // Widely reported: 1981-04-08, 2009-04-08, next 2037-04-08; before
        // the Gregorian century rule bit, 1897-04-07.
        assert_eq!(
            birkat_hachama_on_or_after(civil::to_rd(2009, 4, 9)),
            civil::to_rd(2037, 4, 8)
        );
        assert_eq!(
            birkat_hachama_on_or_after(civil::to_rd(1982, 1, 1)),
            civil::to_rd(2009, 4, 8)
        );
        assert_eq!(
            birkat_hachama_on_or_after(civil::to_rd(1981, 4, 8)),
            civil::to_rd(1981, 4, 8)
        );
        assert_eq!(
            birkat_hachama_on_or_after(civil::to_rd(1870, 1, 1)),
            civil::to_rd(1897, 4, 7)
        );
    }

    #[test]
    fn fields_round_trip_through_the_generic_interface() {
        let calendar = HebrewCalendar;
        for offset in (0..120_000i64).step_by(53) {
            let rd = Rd(new_year(5_500).0 + offset);
            let hebrew = calendar.from_fixed(rd).expect("in range");
            let fields = calendar.to_fields(hebrew).expect("describable");
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(hebrew));
        }
    }

    #[test]
    fn out_of_range_fields_name_the_field_that_is_wrong() {
        assert_eq!(
            to_fixed(0, Month::regular(1), 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(MAX_YEAR + 1, Month::regular(1), 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(5_784, Month::regular(0), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(5_784, Month::regular(1), 31),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(5_784, Month::regular(1), 0),
            Err(CalendarError::DayOutOfRange)
        );
        let calendar = HebrewCalendar;
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(5_784, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::new(5_784)),
            Err(CalendarError::MissingField("month"))
        );
        assert_eq!(
            HebrewDate::new(5_784, Month::regular(1), 1),
            Ok(date(5_784, 1, 1))
        );
    }

    #[test]
    fn the_metadata_says_the_calendar_has_leap_months_and_no_astronomy() {
        let meta = HebrewCalendar.meta();
        assert_eq!(meta.id, CalendarId("hebrew"));
        assert!(meta.has_leap_months);
        assert!(!meta.is_astronomical);
        assert_eq!(meta.earliest, Some(EARLIEST));
    }
}
