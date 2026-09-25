//! The Babylonian calendar in its regular form — `babylonian`.
//!
//! The system is written up in `docs/systems/babylonian.md` in the
//! repository: what it is, how the cycle and the visibility criterion work
//! with a worked example, what is carried and what is not, and the
//! measurement against Parker and Dubberstein's table. This page summarises
//! it and states the code's own facts.
//!
//! # What this is
//!
//! The lunisolar calendar of Babylonia as it ran from the fourth century
//! BCE: a month begins on the evening the new crescent is first seen from
//! Babylon, and seven years of every nineteen carry a thirteenth month, a
//! second Addaru in the years ≡ 1, 4, 7, 9, 12 and 15 (mod 19) of the
//! Seleucid count and a second Ulūlu in the years ≡ 18. The rules are those
//! Reingold and Dershowitz give in *Calendrical Calculations* (4th ed.,
//! Cambridge, 2018), following the `babylonian-*` functions of their
//! published source, `calendar.l` in the `calendar-code2` repository
//! (Apache License 2.0), read 2026-09-25; they in turn follow Parker and
//! Dubberstein, *Babylonian
//! Chronology 626 B.C.–A.D. 75* (Brown University Press, 1956; rev. 1971).
//!
//! # The year count
//!
//! Years are counted in the Seleucid era, whose year 1 begins on
//! 1 Nisanu = 3 April 311 BCE in the Julian calendar (RD −113 502). That is
//! the count the tablets themselves carry from the reign of Seleucus I on.
//! Years before SE 1 are the era continued backwards, so that year 0 is
//! 312/311 BCE and year −71 is 383/382 BCE: a modern convention, the one
//! Parker and Dubberstein's transcribers use, and not a count any Babylonian
//! wrote. The tablets of those years are dated by regnal years, and those
//! labels are not carried here.
//!
//! # The range, and why
//!
//! SE −71 to SE 386: from 1 Nisanu of 383 BCE to the end of Addaru of
//! 76 CE. The lower bound is where Parker and Dubberstein's table starts
//! following the nineteen-year rule without exception — its last
//! intercalation outside the rule is in SE −73 — and the earlier centuries,
//! in which the king intercalated by decree, would need their table rather
//! than a rule. The upper bound is where their table ends, with the last
//! cuneiform texts; the Seleucid count went on in Syria for centuries, but
//! on other calendars. A date outside the range is refused
//! ([`CalendarError::BeforeEpoch`] or [`CalendarError::AfterSupportedRange`]),
//! not extrapolated.
//!
//! # The month
//!
//! The day begins at sunset. A month begins on the day whose eve passed the
//! *moonlag* criterion at Babylon (32.4794° N, 44.4328° E, 26 m): at sunset
//! the Moon is at least 24 hours past conjunction and short of first quarter,
//! and it sets more than 48 minutes after the Sun. This is a forecast of an
//! observation, as [`crate::islamic_observational`] is, and Parker and
//! Dubberstein's own month starts are likewise *computed* first visibilities
//! (by Schoch's criterion) rather than the sightings the tablets record.
//! Between the two computations the difference is measured, not assumed:
//! see the accuracy section.
//!
//! Where Reingold and Dershowitz look for the eve's moonset within the
//! standard-time day of UT+3:30, this module's [`hc_astro::moonset`] bounds
//! the day by Babylon's local mean time, 2:58 ahead of UT. The two windows
//! differ by half an hour at each end, and a moonset in that sliver is one
//! the criterion would in any case count as a lag of over twenty hours.
//!
//! # Month numbering
//!
//! Months are 1 Nīsannu, 2 Ayyāru, 3 Sīmannu, 4 Duʾūzu, 5 Ābu, 6 Ulūlu,
//! 7 Tašrītu, 8 Araḫsamna, 9 Kisilīmu, 10 Ṭebētu, 11 Šabāṭu, 12 Addāru, in
//! the normalisation van Gent's converter of Parker and Dubberstein's table
//! prints; `hc-i18n` carries them for English. The intercalary month
//! repeats the month it follows, so a second Addaru is
//! [`Month::leap(12)`](hc_calendar::Month::leap) and a second Ulūlu is
//! `Month::leap(6)`, the same convention the Chinese and Hebrew calendars use
//! here.
//!
//! The days run from 1 to whatever the criterion gives. It is applied to
//! each evening on its own, so a month here runs 29 or 30 days almost
//! always, but 31 when one first evening just clears the 48-minute lag and
//! the thirtieth evening after it just misses, and 28 in the opposite case.
//! The Babylonian rule that a month not seen to end on its thirtieth
//! evening ends anyway is not applied: neither Reingold and Dershowitz nor
//! Parker and Dubberstein apply it to their computed months — the 1971
//! table prints a lunation of 31 days — and applying it would make each
//! month's start depend on the month before it, all the way back. The
//! count of such months in the range is in the accuracy section.
//!
//! # Accuracy
//!
//! Measured against Parker and Dubberstein's table (1971 edition, in
//! R. H. van Gent's transcription at `webspace.science.uu.nl/~gent0113/
//! babylon/`, read 2026-09-25) over its 5 664 months from SE −71: the
//! intercalary month falls in the year and the place the rule gives in
//! every one of the 168 cases, and the first day of the month agrees with
//! the table on the share of months stated in [`PARKER_DUBBERSTEIN_AGREEMENT`],
//! never differing by more than a day. The transcription is not carried in
//! this repository; the test that measures against it,
//! `measured_against_parker_dubberstein`, is ignored unless the environment
//! variable `HC_PD_TABLE` names a copy, and the spot checks in the module's
//! tests are the rows of it that the documentation cites.

use hc_astro::{Location, MEAN_SYNODIC_MONTH, moonset, new_moon_before, sunset};
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_core::math::{floor, round};

/// The machine identifier of this calendar. CLDR has none for it.
pub const ID: CalendarId = CalendarId("babylonian");

/// The era code of the Seleucid era.
pub const ERA: &str = "SE";

/// The fixed day of 1 Nisanu SE 1: 3 April 311 BCE Julian, which is 29 March
/// of the proleptic Gregorian year −310.
pub const EPOCH: Rd = Rd(-113_502);

/// Babylon: 32°28′46″ N, 44°25′58″ E, 26 m, as *Calendrical Calculations*
/// places it.
pub const BABYLON: Location = Location::new(32.4794, 44.4328, 26.0);

/// The earliest Seleucid year converted, 383/382 BCE.
pub const MIN_YEAR: i64 = -71;

/// The latest Seleucid year converted, 75/76 CE.
pub const MAX_YEAR: i64 = 386;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Parker and Dubberstein 1956 [parker1956]: the nineteen-year rule followed without exception \
    from 1 Nisanu of SE −71, 383 BCE, to 29 Addaru of SE 386, 76 CE, where their table ends \
    with the last dated cuneiform texts; earlier the king intercalated by decree, and later \
    the Seleucid count went on in Syria on other calendars";

/// The earliest fixed day converted: 1 Nisanu SE −71, 18 April 383 BCE
/// Julian, as Parker and Dubberstein's table has it and as the criterion
/// places it.
pub const EARLIEST: Rd = Rd(-139_785);

/// The latest fixed day converted: 29 Addaru SE 386, 25 March 76 CE Julian,
/// the day before the criterion's 1 Nisanu SE 387.
pub const LATEST: Rd = Rd(27_475);

/// How many of the 5 664 month starts from SE −71 in Parker and
/// Dubberstein's table this module places on the same day, as the ignored
/// measurement test computes it: `(agreeing, total)`. Of the rest, 941 are a
/// day later here and 29 a day earlier; none is further off.
pub const PARKER_DUBBERSTEIN_AGREEMENT: (u32, u32) = (4_694, 5_664);

/// Years in the intercalation cycle.
pub const CYCLE_YEARS: i64 = 19;

/// Months in the intercalation cycle.
pub const CYCLE_MONTHS: i64 = 235;

/// The least lag of moonset behind sunset, in days, for the crescent to
/// count as seen: 48 minutes.
const MINIMUM_MOONLAG: f64 = 48.0 / (24.0 * 60.0);

/// Whether `year` carries a thirteenth month.
///
/// `(7y + 13) mod 19 < 7` selects the years ≡ 1, 4, 7, 9, 12, 15 and 18
/// (mod 19); the test `the_closed_form_matches_the_listed_leap_years`
/// checks that.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    (7 * year + 13).rem_euclid(CYCLE_YEARS) < 7
}

/// Whether `year`'s thirteenth month is a second Ulūlu rather than a second
/// Addaru — true in the years ≡ 18 (mod 19), all of which are leap years.
#[must_use]
pub const fn has_second_ululu(year: i64) -> bool {
    year.rem_euclid(CYCLE_YEARS) == 18
}

/// The ordinal of the month that `year`'s thirteenth month repeats, or
/// `None` in a common year.
#[must_use]
pub const fn leap_month(year: i64) -> Option<u8> {
    if !is_leap_year(year) {
        None
    } else if has_second_ululu(year) {
        Some(6)
    } else {
        Some(12)
    }
}

/// Months elapsed from the epoch to 1 Nisanu of `year`.
const fn months_before_year(year: i64) -> i64 {
    ((year - 1) * CYCLE_MONTHS + 13).div_euclid(CYCLE_YEARS)
}

/// The moonlag between the eve's sunset and the Moon's setting, in days.
///
/// A day on which the Moon does not set is given a lag of a full day, as
/// the source does; the criterion then passes, since a Moon still up at
/// midnight is a Moon that was up at dusk.
fn moonlag(eve: Rd, set: Moment) -> f64 {
    match moonset(eve, BABYLON) {
        Some(moon) => moon.0 - set.0,
        None => 1.0,
    }
}

/// Whether the crescent counts as seen on the evening that begins the day
/// `rd` — the sunset of `rd - 1`.
#[must_use]
pub fn crescent_visible_on_the_eve_of(rd: Rd) -> bool {
    let eve = Rd(rd.0 - 1);
    let Some(set) = sunset(eve, BABYLON) else {
        return false;
    };
    let phase = hc_astro::lunar_phase(set);
    if !(0.0..90.0).contains(&phase) {
        return false;
    }
    if new_moon_before(set).0 > set.0 - 1.0 {
        return false;
    }
    moonlag(eve, set) > MINIMUM_MOONLAG
}

/// The first day of the month containing `rd`.
///
/// # Errors
///
/// Returns [`CalendarError::AstronomicalModelFailure`] when no evening
/// within a lunation and a half passes the criterion, which the model does
/// not do at Babylon's latitude.
pub fn month_start_on_or_before(rd: Rd) -> CalendarResult<Rd> {
    let moon = floor(new_moon_before(Moment(rd.0 as f64)).0) as i64;
    let age = rd.0 - moon;
    // Within three days of the conjunction the crescent may not yet count
    // as seen, in which case the month began a lunation earlier.
    let first = if age <= 3 && !crescent_visible_on_the_eve_of(rd) {
        moon - 30
    } else {
        moon
    };
    for step in 0..45 {
        let candidate = Rd(first + step);
        if crescent_visible_on_the_eve_of(candidate) {
            return Ok(candidate);
        }
    }
    Err(CalendarError::AstronomicalModelFailure)
}

/// The fixed day of a Babylonian date, with no range check on the year.
///
/// # Errors
///
/// Returns [`CalendarError::MonthOutOfRange`] for a month that the year
/// does not have, [`CalendarError::DayOutOfRange`] for a day past 31, or
/// the criterion's failure.
fn to_fixed_unchecked(year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
    if month.ordinal == 0 || month.ordinal > 12 {
        return Err(CalendarError::MonthOutOfRange);
    }
    if month.leap && leap_month(year) != Some(month.ordinal) {
        return Err(CalendarError::MonthOutOfRange);
    }
    if day == 0 || day > 31 {
        return Err(CalendarError::DayOutOfRange);
    }
    // Months of the year elapsed before this one: the intercalary month
    // and, in a year with a second Ulūlu, every month after it come one
    // later than their ordinal says.
    let elapsed_this_year = if month.leap || (has_second_ululu(year) && month.ordinal > 6) {
        i64::from(month.ordinal)
    } else {
        i64::from(month.ordinal) - 1
    };
    let months = months_before_year(year) + elapsed_this_year;
    let midmonth = EPOCH.0 + round(MEAN_SYNODIC_MONTH * months as f64) as i64 + 15;
    let start = month_start_on_or_before(Rd(midmonth))?;
    Ok(Rd(start.0 + i64::from(day) - 1))
}

/// The Babylonian year, month and day of a fixed day, with no range check.
fn from_fixed_unchecked(rd: Rd) -> CalendarResult<(i64, Month, u8)> {
    let crescent = month_start_on_or_before(rd)?;
    let months = round((crescent.0 - EPOCH.0) as f64 / MEAN_SYNODIC_MONTH) as i64;
    let year = (CYCLE_YEARS * months + 5).div_euclid(CYCLE_MONTHS) + 1;
    let approx = EPOCH.0 + round(months_before_year(year) as f64 * MEAN_SYNODIC_MONTH) as i64;
    let new_year = month_start_on_or_before(Rd(approx + 15))?;
    let position = 1 + round((crescent.0 - new_year.0) as f64 / 29.5) as i64;
    let special = has_second_ululu(year);
    let leap = if special {
        position == 7
    } else {
        position == 13
    };
    let ordinal = if leap || (special && position > 6) {
        position - 1
    } else {
        position
    };
    let month = Month {
        ordinal: ordinal as u8,
        leap,
    };
    let day = (rd.0 - crescent.0 + 1) as u8;
    Ok((year, month, day))
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

/// The fixed day of a Babylonian date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside SE −71 to 386,
/// [`CalendarError::MonthOutOfRange`] for a month the year does not have —
/// a leap month in a common year, or the wrong one — and
/// [`CalendarError::DayOutOfRange`] for a day the month does not have.
pub fn to_fixed(year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    let rd = to_fixed_unchecked(year, month, day)?;
    check_range(rd)?;
    // The length of a month is not knowable before the sky is asked, so a
    // 30th day is validated by reading the date back.
    let (round_year, round_month, round_day) = from_fixed_unchecked(rd)?;
    if (round_year, round_month) != (year, month) {
        return Err(CalendarError::DayOutOfRange);
    }
    if round_day != day {
        return Err(CalendarError::DayOutOfRange);
    }
    Ok(rd)
}

/// The Babylonian year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before 1 Nisanu SE −71 and
/// [`CalendarError::AfterSupportedRange`] after 30 Addaru SE 386.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, Month, u8)> {
    check_range(rd)?;
    from_fixed_unchecked(rd)
}

/// A date in the Babylonian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BabylonianDate {
    /// The Seleucid year, negative before the era's first year.
    pub year: i64,
    /// The month, Nisanu first, with a second Ulūlu as `Month::leap(6)` and a
    /// second Addaru as `Month::leap(12)`.
    pub month: Month,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl BabylonianDate {
    /// A validated Babylonian date.
    ///
    /// # Errors
    ///
    /// The errors of [`to_fixed`].
    pub fn new(year: i64, month: Month, day: u8) -> CalendarResult<Self> {
        to_fixed(year, month, day)?;
        Ok(Self { year, month, day })
    }
}

/// The Babylonian calendar of the Seleucid era.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BabylonianCalendar;

impl Calendar for BabylonianCalendar {
    type Date = BabylonianDate;

    /// SE −71 to SE 386, which is also the whole of the range it converts:
    /// the years in which the nineteen-year rule was followed without
    /// exception and the cuneiform record runs.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(EARLIEST, LATEST, USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// A year with a second Addaru or a second Ulūlu.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    /// The Babylonian day begins at sunset, which is when its month is
    /// decided.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Babylonian (Seleucid era)",
            year_kind: YearKind::Astronomical,
            has_leap_months: true,
            is_astronomical: true,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(BabylonianDate { year, month, day })
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
        BabylonianDate::new(fields.year, fields.require_month()?, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::civil;

    /// Parker and Dubberstein's first days of the month, as fixed days, for
    /// the rows the module documentation cites: 1 Nisanu of every
    /// nineteenth year from SE −56, and every intercalary month of the first
    /// two cycles of the era. `(year, month, leap, fixed day)`; the comment
    /// is the table's Julian date.
    const PARKER_DUBBERSTEIN: &[(i64, u8, bool, i64)] = &[
        (-56, 1, false, -134_322), // -367-04-02
        (-37, 1, false, -127_382), // -348-04-02
        (-18, 1, false, -120_442), // -329-04-03
        (1, 1, false, -113_502),   // -310-04-03
        (1, 12, true, -113_148),   // -309-03-23
        (4, 12, true, -112_056),   // -306-03-19
        (7, 12, true, -110_963),   // -303-03-16
        (9, 12, true, -110_224),   // -301-03-25
        (12, 12, true, -109_132),  // -298-03-21
        (15, 12, true, -108_040),  // -295-03-17
        (18, 6, true, -107_124),   // -293-09-19
        (20, 1, false, -106_562),  // -291-04-03
        (20, 12, true, -106_208),  // -290-03-23
        (23, 12, true, -105_116),  // -287-03-19
        (26, 12, true, -104_023),  // -284-03-16
        (28, 12, true, -103_284),  // -282-03-25
        (31, 12, true, -102_193),  // -279-03-20
        (34, 12, true, -101_100),  // -276-03-17
        (37, 6, true, -100_185),   // -274-09-18
        (39, 1, false, -99_623),   // -272-04-02
        (58, 1, false, -92_683),   // -253-04-03
        (77, 1, false, -85_744),   // -234-04-02
        (96, 1, false, -78_804),   // -215-04-02
        (115, 1, false, -71_864),  // -196-04-02
        (134, 1, false, -64_924),  // -177-04-03
        (153, 1, false, -57_984),  // -158-04-03
        (172, 1, false, -51_045),  // -139-04-02
        (191, 1, false, -44_106),  // -120-04-01
        (210, 1, false, -37_166),  // -101-04-02
        (229, 1, false, -30_226),  // -82-04-02
        (248, 1, false, -23_286),  // -63-04-02
        (267, 1, false, -16_346),  // -44-04-02
        (286, 1, false, -9_407),   // -25-04-02
        (305, 1, false, -2_467),   // -6-04-02
        (324, 1, false, 4_472),    // 13-04-01
        (343, 1, false, 11_412),   // 32-04-01
        (362, 1, false, 18_351),   // 51-04-01
        (381, 1, false, 25_291),   // 70-04-01
    ];

    fn month(ordinal: u8, leap: bool) -> Month {
        Month { ordinal, leap }
    }

    #[test]
    fn the_epoch_is_the_third_of_april_311_bce() {
        // 3 April 311 BCE Julian is 29 March of the proleptic Gregorian
        // year −310, five days earlier, as every date of that century is.
        assert_eq!(EPOCH, civil::to_rd(-310, 3, 29));
        assert_eq!(from_fixed(EPOCH), Ok((1, month(1, false), 1)));
        assert_eq!(to_fixed(1, month(1, false), 1), Ok(EPOCH));
    }

    #[test]
    fn the_closed_form_matches_the_listed_leap_years() {
        let leap_years: [i64; 7] = [1, 4, 7, 9, 12, 15, 18];
        for position in 0..19 {
            assert_eq!(
                is_leap_year(position),
                leap_years.contains(&position),
                "year ≡ {position}"
            );
            assert_eq!(is_leap_year(position - 19), is_leap_year(position));
            assert_eq!(is_leap_year(position + 380), is_leap_year(position));
        }
        assert_eq!(leap_month(18), Some(6));
        assert_eq!(leap_month(-1), Some(6));
        assert_eq!(leap_month(1), Some(12));
        assert_eq!(leap_month(2), None);
    }

    #[test]
    fn the_cited_rows_of_parker_and_dubberstein() {
        let mut exact = 0;
        for &(year, ordinal, leap, table) in PARKER_DUBBERSTEIN {
            let computed = to_fixed(year, month(ordinal, leap), 1).expect("in range");
            let difference = computed.0 - table;
            assert!(
                (0..=1).contains(&difference),
                "SE {year} month {ordinal}{}: {difference} days off",
                if leap { "b" } else { "" }
            );
            if difference == 0 {
                exact += 1;
            }
            // Whatever the day, the month is the table's month.
            let (round_year, round_month, _) = from_fixed(Rd(table + 5)).expect("in range");
            assert_eq!((round_year, round_month), (year, month(ordinal, leap)));
        }
        // Six of the thirty-eight cited rows are a day later here: the
        // intercalary months of SE 15, 31, 34 and 37 and the new years of
        // SE 362 and 381.
        assert_eq!(exact, PARKER_DUBBERSTEIN.len() - 6, "rows on the same day");
    }

    #[test]
    fn the_range_is_the_tables_regular_span() {
        assert_eq!(from_fixed(EARLIEST), Ok((MIN_YEAR, month(1, false), 1)));
        assert_eq!(
            from_fixed(LATEST).map(|(y, m, _)| (y, m)),
            Ok((MAX_YEAR, month(12, false)))
        );
        assert_eq!(
            from_fixed_unchecked(Rd(LATEST.0 + 1)),
            Ok((MAX_YEAR + 1, month(1, false), 1))
        );
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            to_fixed(MIN_YEAR - 1, month(12, false), 29),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(MAX_YEAR + 1, month(1, false), 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_calendar_round_trips_at_both_ends_and_across_the_span() {
        let mut days = (0..400i64)
            .map(|offset| EARLIEST.0 + offset)
            .collect::<std::vec::Vec<_>>();
        days.extend((0..400i64).map(|offset| LATEST.0 - offset));
        days.extend((EARLIEST.0..=LATEST.0).step_by(97));
        for rd in days.into_iter().map(Rd) {
            let (year, month, day) = from_fixed(rd).expect("in range");
            assert!(
                (MIN_YEAR..=MAX_YEAR).contains(&year),
                "RD {rd} gave SE {year}"
            );
            assert_eq!(to_fixed(year, month, day), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn years_have_twelve_or_thirteen_months_in_the_cycles_places() {
        for year in 1..=38 {
            let end = to_fixed(year + 1, month(1, false), 1).expect("in range");
            let mut cursor = to_fixed(year, month(1, false), 1).expect("in range");
            let mut count = 0u8;
            let mut second_ululu = false;
            while cursor < end {
                let (_, this, day) = from_fixed(cursor).expect("in range");
                assert_eq!(day, 1, "RD {cursor} is not a month start");
                second_ululu |= this == month(6, true);
                count += 1;
                let next = month_start_on_or_before(Rd(cursor.0 + 32)).expect("converges");
                let length = next.0 - cursor.0;
                assert!(
                    (28..=31).contains(&length),
                    "SE {year}: a month of {length} days"
                );
                cursor = next;
            }
            assert_eq!(cursor, end, "SE {year}");
            assert_eq!(count, if is_leap_year(year) { 13 } else { 12 }, "SE {year}");
            assert_eq!(second_ululu, has_second_ululu(year), "SE {year}");
        }
    }

    #[test]
    fn a_leap_month_is_refused_where_the_cycle_has_none() {
        assert_eq!(
            to_fixed(2, month(12, true), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(1, month(6, true), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(18, month(12, true), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert!(to_fixed(18, month(6, true), 1).is_ok());
        assert_eq!(
            to_fixed(1, month(13, false), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(1, month(1, false), 31),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(1, month(1, false), 0),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn a_day_exists_only_where_the_month_reaches_it() {
        // The month's length is whatever the criterion gave, so the last day
        // is accepted and the one after it refused, month by month.
        let mut start = EPOCH;
        for _ in 0..12 {
            let next = month_start_on_or_before(Rd(start.0 + 32)).expect("converges");
            let length = (next.0 - start.0) as u8;
            let (year, month, _) = from_fixed(start).expect("in range");
            assert_eq!(to_fixed(year, month, length), Ok(Rd(next.0 - 1)));
            assert_eq!(
                to_fixed(year, month, length + 1),
                Err(CalendarError::DayOutOfRange)
            );
            start = next;
        }
        // Nisanu SE 1 ran 29 days: the table's 1 Aiaru is 2 May, and so is this one's.
        assert_eq!(
            to_fixed(1, month(1, false), 30),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(to_fixed(1, month(2, false), 1), Ok(Rd(EPOCH.0 + 29)));
    }

    #[test]
    fn fields_round_trip_through_the_generic_interface() {
        let calendar = BabylonianCalendar;
        let date = calendar.from_fixed(Rd(-107_124)).expect("in range");
        assert_eq!(
            date,
            BabylonianDate {
                year: 18,
                month: month(6, true),
                day: 1
            }
        );
        let fields = calendar.to_fields(date).expect("fields");
        assert_eq!(fields.era, Some("SE"));
        assert_eq!(fields.month, Some(month(6, true)));
        assert_eq!(calendar.from_fields(&fields), Ok(date));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(18, 1, 1).with_era("AH")),
            Err(CalendarError::UnknownEra)
        );
        let meta = calendar.meta();
        assert_eq!(meta.id, CalendarId("babylonian"));
        assert!(meta.is_astronomical);
        assert!(meta.has_leap_months);
        assert_eq!(meta.year_kind, YearKind::Astronomical);
    }

    /// Measures every month start from SE −71 against a copy of Parker and
    /// Dubberstein's table, one line per month, `<SE year> <month> <leap
    /// 0|1> <fixed day>`, named by `HC_PD_TABLE`. Run with
    /// `HC_PD_TABLE=... cargo test -p hc-calendars-lunar --release
    /// -- --ignored --nocapture measured_against`.
    #[test]
    #[ignore = "needs a copy of the table, named by HC_PD_TABLE"]
    fn measured_against_parker_dubberstein() {
        let Ok(path) = std::env::var("HC_PD_TABLE") else {
            return;
        };
        let table = std::fs::read_to_string(path).expect("the table is readable");
        let mut total = 0u32;
        let mut same = 0u32;
        let mut early = 0u32;
        let mut late = 0u32;
        let mut worst = 0i64;
        let mut leap_wrong = 0u32;
        let mut leaps = 0u32;
        let mut lengths = [0u32; 32];
        let mut previous: Option<i64> = None;
        for line in table.lines() {
            let fields: Vec<i64> = line
                .split_whitespace()
                .map(|f| f.parse().unwrap())
                .collect();
            let [year, ordinal, leap, rd] = fields[..] else {
                continue;
            };
            if year < MIN_YEAR {
                continue;
            }
            total += 1;
            let month = month(ordinal as u8, leap == 1);
            if month.leap {
                leaps += 1;
                if leap_month(year) != Some(month.ordinal) {
                    leap_wrong += 1;
                    continue;
                }
            }
            let computed = to_fixed(year, month, 1).expect("in range");
            if let Some(last) = previous {
                lengths[(computed.0 - last) as usize] += 1;
            }
            previous = Some(computed.0);
            let difference = computed.0 - rd;
            worst = worst.max(difference.abs());
            match difference {
                0 => same += 1,
                d if d < 0 => early += 1,
                _ => late += 1,
            }
            if difference != 0 {
                std::println!(
                    "SE {year} month {ordinal}{}: {difference:+}",
                    if leap == 1 { "b" } else { "" }
                );
            }
        }
        std::println!(
            "months {total}, same day {same}, earlier {early}, later {late}, worst {worst}; intercalary {leaps}, misplaced {leap_wrong}"
        );
        std::println!(
            "month lengths: 28 × {}, 29 × {}, 30 × {}, 31 × {}",
            lengths[28],
            lengths[29],
            lengths[30],
            lengths[31]
        );
        assert_eq!(total, PARKER_DUBBERSTEIN_AGREEMENT.1);
        assert_eq!(same, PARKER_DUBBERSTEIN_AGREEMENT.0);
        assert_eq!(leap_wrong, 0);
        assert!(worst <= 1);
    }
}
