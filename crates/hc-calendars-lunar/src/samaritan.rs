//! The Samaritan calendar — `samaritan`.
//!
//! The system is written up in `docs/systems/samaritan.md` in the
//! repository: who keeps it and who issues it, the month from the
//! conjunction at Mount Gerizim, the first month tied to Julian 11 March,
//! the year counted from the entry into Canaan and changing at the sixth
//! month, the Passover of 2019 worked by hand, how the rule was checked against the
//! community's published dates and where it fails, and the sources, keyed
//! in `docs/references.bib`. This page summarises it and states the code's
//! own facts.
//!
//! # What this is
//!
//! The lunisolar calendar of the Israelite Samaritans, a sibling of
//! [`crate::hebrew`] with none of its arithmetic: the rule here has no
//! molad, no nineteen-year table and no postponements. The calendar as
//! kept is issued each year by the priesthood, from a calculation the late
//! High Priest Avraham b. Pinchas put on a computer in the twentieth
//! century (`samaritan-institute-calendar`), and this module is not that
//! calculation, which this library has not seen. It is Reingold and Dershowitz's
//! "modern calculation" of it, the `samaritan-*` functions of their
//! published `calendar.l` (`reingold2018code`, Apache License 2.0, read
//! 2026-09-26), from the true conjunction:
//!
//! 1. **A month begins on the day of the conjunction** when the conjunction
//!    falls at or before apparent noon at Mount Gerizim ([`GERIZIM`],
//!    32.1994° N, 35.2728° E, 881 m), and on the day after otherwise.
//! 2. **The first month** — the month of Passover — is the first to begin
//!    after apparent noon on 11 March of the Julian calendar, which in the
//!    twentieth and twenty-first centuries is 24 March Gregorian.
//! 3. **A year has twelve or thirteen months**, as many as fit before the
//!    next first month; a thirteenth month is intercalated when the
//!    months run out before the next Julian 11 March.
//! 4. **The year is counted from the entry into Canaan** with Joshua, the
//!    Entry Era of [`EPOCH`], Julian 15 March 1639 BCE, and the count moves
//!    on at the **sixth** month, not the first: Passover 2016 is in year
//!    3654 and the autumn of 2016 begins 3655.
//!
//! # Month numbering
//!
//! As [`crate::hebrew`] does, and for the same reason, the public months are
//! numbered from where the year number changes: 1 is the Sixth Month,
//! 7 the Twelfth, 8 the First Month and 12 the Fifth. The Thirteenth Month
//! of an intercalary year falls between the Twelfth and the First, so it is
//! [`Month::leap(7)`](hc_calendar::Month::leap). The sources name every
//! month by its ordinal from the First Month, the month of Passover, and
//! [`SamaritanDate::biblical_month`] gives that ordinal, 1 to 13.
//!
//! # Accuracy and range
//!
//! The rule agrees with the community's published Passover sacrifices of
//! 2017 to 2020 and with its new moons and festivals of autumn 2026, and
//! puts the sacrifice of 2016 a day after the published one. It lets the
//! sacrifice fall on 7 April in six years of the range, where the community
//! says it is never earlier than the 8th. It is
//! bounded to the Samaritan years [`MIN_YEAR`] to [`MAX_YEAR`], autumn 1900
//! to autumn 2100, which is this library's choice and not a claim of the
//! book's: the code is marked as a modern calculation and states no range,
//! and the community's own calculation is of the twentieth century. The
//! astronomy is `hc-astro`'s, so a conjunction within a few minutes of
//! noon at Gerizim is decided by that model.

use hc_astro::Location;
use hc_astro::lunar::{new_moon_at_or_after, new_moon_before};
use hc_astro::riseset::solar_noon;
use hc_astro::solar::equation_of_time;
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_solar::julian;

/// The calendar's identifier.
pub const ID: CalendarId = CalendarId("samaritan");

/// The era code of the Entry Era, the years since the entry into Canaan.
pub const ERA: &str = "entry";

/// Mount Gerizim, where the months were observed and are reckoned:
/// 32.1994° N, 35.2728° E, 881 m, as `samaritan-location` gives it
/// (`reingold2018code`).
pub const GERIZIM: Location = Location::new(32.1994, 35.2728, 881.0);

/// The fixed day of the start of the Entry Era: 15 March 1639 BCE Julian,
/// `samaritan-epoch` (`reingold2018code`), which the book's errata give as
/// 1 March −1638 Gregorian (`reingold2018errata`).
pub const EPOCH: Rd = match julian::to_fixed(-1_638, 3, 15) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The first Samaritan year this implementation converts, which begins in
/// the autumn of 1900.
pub const MIN_YEAR: i64 = 3_539;

/// The last Samaritan year this implementation converts, which ends in the
/// autumn of 2100.
pub const MAX_YEAR: i64 = 3_738;

/// The Julian month and day after whose noon the first month begins.
const FIRST_MONTH_AFTER: (u8, u8) = (3, 11);

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The calendar of the Israelite Samaritans, kept on Mount Gerizim and issued by the \
    priesthood; its computed form from the twentieth century, when the High Priest Avraham \
    b. Pinchas moved the calculation to a computer [samaritan-institute-calendar]; older than \
    any source read dates, as docs/systems/samaritan.md states";

/// The Sixth Month, where the year number changes, in the sources'
/// numbering from the First.
const SIXTH: u8 = 6;

/// A date in the Samaritan calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamaritanDate {
    /// The year of the Entry Era, which changes at the Sixth Month.
    pub year: i64,
    /// The month, numbered from the Sixth Month as 1, with the Thirteenth
    /// Month as [`Month::leap(7)`](Month::leap).
    pub month: Month,
    /// The day of the month, from 1.
    pub day: u8,
}

impl SamaritanDate {
    /// The month's ordinal as the sources give it: 1 for the First Month,
    /// the month of Passover, through 13 for the Thirteenth.
    ///
    /// Returns 0 for a month the calendar does not have.
    #[must_use]
    pub const fn biblical_month(self) -> u8 {
        from_public(self.month)
    }
}

/// The sources' ordinal of a public month, or 0 if there is none.
const fn from_public(month: Month) -> u8 {
    if month.leap {
        return if month.ordinal == 7 { 13 } else { 0 };
    }
    match month.ordinal {
        1..=7 => month.ordinal + 5,
        8..=12 => month.ordinal - 7,
        _ => 0,
    }
}

/// The public month of one of the sources' ordinals.
const fn to_public(biblical: u8) -> Month {
    match biblical {
        13 => Month::leap(7),
        6..=12 => Month::regular(biblical - 5),
        _ => Month::regular(biblical + 7),
    }
}

/// Apparent solar time at Gerizim of a moment in Universal Time, as
/// `apparent-from-universal` computes it: local mean time plus the
/// equation of time.
fn apparent_at_gerizim(moment: Moment) -> f64 {
    moment.0 + GERIZIM.longitude_degrees / 360.0 + equation_of_time(moment)
}

/// The first day of the month begun by the conjunction at `conjunction`:
/// its apparent day at Gerizim, or the next when it falls after noon.
fn month_start(conjunction: Moment) -> Rd {
    Rd(hc_core::math::ceil(apparent_at_gerizim(conjunction) - 0.5) as i64)
}

/// The first day of the first month to begin after `moment`,
/// `samaritan-new-moon-after`.
fn month_start_after(moment: Moment) -> Rd {
    month_start(new_moon_at_or_after(moment))
}

/// The first day of the month the conjunction before `moment` began,
/// `samaritan-new-moon-at-or-before`.
fn month_start_before(moment: Moment) -> Rd {
    month_start(new_moon_before(moment))
}

/// Apparent noon at Gerizim on a fixed day, `samaritan-noon`.
fn noon(day: Rd) -> Moment {
    solar_noon(day, GERIZIM)
}

/// The first day of the First Month that follows Julian 11 March of the
/// Julian year `year`, which is the Gregorian year it falls in.
fn first_month_of(year: i64) -> CalendarResult<Rd> {
    let (month, day) = FIRST_MONTH_AFTER;
    Ok(month_start_after(noon(julian::to_fixed(year, month, day)?)))
}

/// The first day of the First Month on or before `day`,
/// `samaritan-new-year-on-or-before`.
fn first_month_on_or_before(day: Rd) -> CalendarResult<Rd> {
    let year = hc_calendar::gregorian::year_from_fixed(day);
    let this = first_month_of(year)?;
    if this <= day {
        Ok(this)
    } else {
        first_month_of(year - 1)
    }
}

/// `round`, as `calendar.l` defines it: the floor of `numerator /
/// denominator + 1/2`.
const fn round_ratio(numerator: i64, denominator: i64) -> i64 {
    (2 * numerator + denominator).div_euclid(2 * denominator)
}

/// 1 when a month in the sources' numbering belongs to the year that
/// begins at the Sixth Month after its First, else 0: `(ceiling (- month 5)
/// 8)`.
const fn year_shift(biblical: u8) -> i64 {
    if biblical >= SIXTH { 1 } else { 0 }
}

/// The year, the month in the sources' numbering and the day of a fixed
/// day, with no range check: `samaritan-from-fixed`.
fn from_fixed_unchecked(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    let moon = month_start_before(noon(rd));
    let new_year = first_month_on_or_before(moon)?;
    // 1 + round((moon - new-year) / 29.5).
    let month = 1 + round_ratio(2 * (moon.0 - new_year.0), 59);
    // round((new-year - epoch) / 365.25) + ceiling((month - 5) / 8).
    let biblical = u8::try_from(month).map_err(|_| CalendarError::MonthOutOfRange)?;
    let year = round_ratio(4 * (new_year.0 - EPOCH.0), 1_461) + year_shift(biblical);
    let day = u8::try_from(rd.0 - moon.0 + 1).map_err(|_| CalendarError::DayOutOfRange)?;
    Ok((year, biblical, day))
}

/// The first day of a month in the sources' numbering of `year`, with no
/// check that the month exists: the month-start part of
/// `fixed-from-samaritan`.
fn month_start_of(year: i64, biblical: u8) -> CalendarResult<Rd> {
    let elapsed = year - year_shift(biblical);
    let guess = Rd(EPOCH.0 + 50 + (1_461 * elapsed).div_euclid(4));
    let new_year = first_month_on_or_before(guess)?;
    let middle = new_year.0 as f64 + 29.5 * f64::from(biblical - 1) + 15.0;
    Ok(month_start_before(Moment(middle)))
}

/// The first day of year [`MIN_YEAR`].
///
/// # Errors
///
/// Returns an error only if the astronomy cannot place the year, which it
/// can.
pub fn earliest() -> CalendarResult<Rd> {
    month_start_of(MIN_YEAR, SIXTH)
}

/// The last day of year [`MAX_YEAR`].
///
/// # Errors
///
/// As [`earliest`].
pub fn latest() -> CalendarResult<Rd> {
    Ok(Rd(month_start_of(MAX_YEAR + 1, SIXTH)?.0 - 1))
}

/// The number of days in a month, in the sources' numbering, of a year,
/// or `None` when the year has no such month.
fn month_length(year: i64, biblical: u8) -> CalendarResult<Option<u8>> {
    if !(1..=13).contains(&biblical) {
        return Ok(None);
    }
    let first = month_start_of(year, biblical)?;
    let (found_year, found_month, found_day) = from_fixed_unchecked(first)?;
    if (found_year, found_month, found_day) != (year, biblical, 1) {
        return Ok(None);
    }
    let next = month_start_after(Moment(first.0 as f64 + 1.0));
    // The day after the month's last is the first of the next; the next
    // month begins after the next conjunction, whichever day that is.
    let length = next.0 - first.0;
    Ok(u8::try_from(length).ok())
}

/// The fixed day of a Samaritan date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`], [`CalendarError::MonthOutOfRange`] for a
/// month the year does not have — the Thirteenth in a year of twelve — and
/// [`CalendarError::DayOutOfRange`] for a day past the month's last.
pub fn to_fixed(date: SamaritanDate) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&date.year) {
        return Err(CalendarError::YearOutOfRange);
    }
    let biblical = from_public(date.month);
    let length = month_length(date.year, biblical)?.ok_or(CalendarError::MonthOutOfRange)?;
    if date.day == 0 || date.day > length {
        return Err(CalendarError::DayOutOfRange);
    }
    Ok(Rd(month_start_of(date.year, biblical)?.0
        + i64::from(date.day)
        - 1))
}

/// The Samaritan date of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside the years converted.
pub fn from_fixed(rd: Rd) -> CalendarResult<SamaritanDate> {
    let (year, biblical, day) = from_fixed_unchecked(rd)?;
    if year < MIN_YEAR {
        return Err(CalendarError::BeforeEpoch);
    }
    if year > MAX_YEAR {
        return Err(CalendarError::AfterSupportedRange);
    }
    Ok(SamaritanDate {
        year,
        month: to_public(biblical),
        day,
    })
}

/// Whether a Samaritan year has a Thirteenth Month.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn is_leap_year(year: i64) -> CalendarResult<bool> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(month_length(year, 13)?.is_some())
}

/// The fixed day of the Passover sacrifice, the fourteenth of the First
/// Month, in a Samaritan year.
///
/// The First Month is the eighth public month of the year, which began at
/// the Sixth Month the autumn before.
///
/// # Errors
///
/// As [`to_fixed`].
pub fn passover_sacrifice(year: i64) -> CalendarResult<Rd> {
    to_fixed(SamaritanDate {
        year,
        month: to_public(1),
        day: 14,
    })
}

/// The Samaritan calendar, by Reingold and Dershowitz's modern calculation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SamaritanCalendar;

impl Calendar for SamaritanCalendar {
    type Date = SamaritanDate;

    /// In use today and older than any source read dates, so undated at the
    /// start; the computed form this module approximates is of the
    /// twentieth century.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(USAGE_SOURCE)
    }

    /// Twelve months with a thirteenth in an intercalary year, and the
    /// seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// A year with a Thirteenth Month.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        is_leap_year(year)
    }

    /// The Samaritan day begins at the preceding sunset
    /// (`samaritans-net-calendar`).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Samaritan",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: earliest().ok(),
            latest: latest().ok(),
            native_locales: &["smp", "he"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        from_fixed(rd)
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
        let date = SamaritanDate {
            year: fields.year,
            month: fields.require_month()?,
            day: fields.require_day()?,
        };
        to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::civil;

    fn first_month(year: i64, day: u8) -> SamaritanDate {
        SamaritanDate {
            year,
            month: Month::regular(8),
            day,
        }
    }

    #[test]
    fn the_epoch_is_the_first_of_march_1638_bce_gregorian() {
        // `samaritan-epoch` is Julian 15 March 1639 BCE; the errata to the
        // 4th edition correct its Gregorian form to 1 March −1638.
        assert_eq!(civil::from_rd(EPOCH), (-1_638, 3, 1));
    }

    #[test]
    fn the_published_passover_sacrifices_of_2017_to_2020() {
        // The Israelite Samaritan Information Institute's calendar page
        // lists the Passover sacrifice on Mount Gerizim for five years; the
        // rule gives four of them. 2016 is the next test.
        for (year, rd) in [
            (3_655, civil::to_rd(2017, 4, 10)),
            (3_656, civil::to_rd(2018, 4, 29)),
            (3_657, civil::to_rd(2019, 4, 18)),
            (3_658, civil::to_rd(2020, 5, 6)),
        ] {
            assert_eq!(passover_sacrifice(year), Ok(rd), "{year}");
            let date = from_fixed(rd).expect("in range");
            assert_eq!(date, first_month(year, 14));
            assert_eq!(date.biblical_month(), 1);
        }
    }

    #[test]
    fn the_rule_puts_the_passover_of_2016_a_day_after_the_community() {
        // Published: Wednesday 20 April 2016. The conjunction of 7 April
        // 2016 came at about 11:24 UT, after apparent noon at Gerizim, so
        // the rule begins the First Month on the 8th and puts the
        // fourteenth on the 21st. Held as a known disagreement.
        let published = civil::to_rd(2016, 4, 20);
        assert_eq!(passover_sacrifice(3_654), Ok(Rd(published.0 + 1)));
        assert_eq!(from_fixed(published), Ok(first_month(3_654, 13)));
    }

    #[test]
    fn the_first_month_of_2020_waited_for_the_conjunction_after_noon_on_24_march() {
        // The conjunction of 24 March 2020 came at about 09:28 UT, before
        // apparent noon at Gerizim on Julian 11 March, so it could not open
        // the year: the First Month began a lunation later, and the
        // sacrifice fell on 6 May, as published.
        assert_eq!(
            from_fixed(civil::to_rd(2020, 4, 23)),
            Ok(first_month(3_658, 1))
        );
        let before = from_fixed(civil::to_rd(2020, 4, 22)).expect("in range");
        assert_eq!(before.biblical_month(), 13);
        assert!(is_leap_year(3_658).expect("in range"));
    }

    #[test]
    fn the_months_of_autumn_2026_are_the_published_ones() {
        // the-samaritans.net's calendar for 2026: the new moon of the Sixth
        // Month on 11 September, the festival of the Seventh Month on
        // 11 October, the Day of Atonement on 20 October, Sukkot on
        // 25 October, the new moons of the Eighth and Ninth Months on
        // 9 November and 9 December. The Sixth Month begins year 3665.
        for ((y, m, d), (month, day)) in [
            ((2026, 9, 11), (1, 1)),
            ((2026, 10, 11), (2, 1)),
            ((2026, 10, 20), (2, 10)),
            ((2026, 10, 25), (2, 15)),
            ((2026, 11, 9), (3, 1)),
            ((2026, 12, 9), (4, 1)),
        ] {
            let date = from_fixed(civil::to_rd(y, m, d)).expect("in range");
            assert_eq!(
                (date.year, date.month, date.day),
                (3_665, Month::regular(month), day),
                "{y}-{m}-{d}"
            );
        }
        let eve = from_fixed(civil::to_rd(2026, 9, 10)).expect("in range");
        assert_eq!((eve.year, eve.biblical_month()), (3_664, 5));
    }

    #[test]
    fn year_3656_began_at_the_sixth_month_of_2017() {
        // Benyamim Tsedaka, 24 January 2017: "Samaritan year 3656 will
        // start later this year, in the 6th month".
        let spring = from_fixed(civil::to_rd(2017, 4, 10)).expect("in range");
        assert_eq!(spring.year, 3_655);
        let start = month_start_of(3_656, SIXTH).expect("in range");
        let (y, m, _) = civil::from_rd(start);
        assert_eq!((y, m), (2017, 8));
        assert_eq!(
            from_fixed(start),
            Ok(SamaritanDate {
                year: 3_656,
                month: Month::regular(1),
                day: 1
            })
        );
        assert_eq!(from_fixed(Rd(start.0 - 1)).map(|d| d.year), Ok(3_655));
    }

    #[test]
    fn the_rule_allows_a_passover_on_7_april_where_the_community_says_8() {
        // Julian 11 March is Gregorian 24 March throughout the range, and
        // the First Month begins after its noon, so the fourteenth is 7 April
        // at the earliest and 7 May at the latest. The community's page says
        // its sages fixed Passover "so that it will not be earlier than
        // April 8"; the rule reaches 7 April when the conjunction falls just
        // after noon on 24 March, in these six years of the two centuries.
        // No published date for any of them was read, so which of the two
        // statements the priesthood's calendar follows there is unknown.
        let mut on_the_seventh = alloc::vec::Vec::new();
        for year in MIN_YEAR..=MAX_YEAR {
            let passover = passover_sacrifice(year).expect("in range");
            let (g, m, d) = civil::from_rd(passover);
            assert!((m, d) >= (4, 7), "{year}: {g}-{m}-{d}");
            assert!((m, d) <= (5, 7), "{year}: {g}-{m}-{d}");
            if (m, d) == (4, 7) {
                on_the_seventh.push(g);
            }
        }
        assert_eq!(on_the_seventh, [1906, 1925, 1944, 2001, 2039, 2058]);
    }

    #[test]
    fn years_have_twelve_or_thirteen_months_of_twenty_nine_or_thirty_days() {
        let mut leaps = 0;
        for year in 3_640..3_678 {
            let mut months = 0;
            let mut total = 0i64;
            for biblical in 1..=13u8 {
                if let Some(length) = month_length(year, biblical).expect("in range") {
                    assert!((29..=30).contains(&length), "{year} {biblical}: {length}");
                    months += 1;
                    total += i64::from(length);
                }
            }
            assert!(months == 12 || months == 13, "{year}: {months}");
            assert_eq!(is_leap_year(year), Ok(months == 13));
            if months == 13 {
                leaps += 1;
                assert!((381..=387).contains(&total), "{year}: {total}");
            } else {
                assert!((351..=357).contains(&total), "{year}: {total}");
            }
        }
        // Seven intercalary years in nineteen, twice over.
        assert_eq!(leaps, 14);
    }

    #[test]
    fn every_day_of_four_years_round_trips() {
        let calendar = SamaritanCalendar;
        let start = month_start_of(3_654, SIXTH).expect("in range");
        let end = month_start_of(3_658, SIXTH).expect("in range");
        for day in start.0..end.0 {
            let rd = Rd(day);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "{date:?}");
            let fields = calendar.to_fields(date).expect("fields");
            assert_eq!(calendar.from_fields(&fields), Ok(date));
        }
    }

    #[test]
    fn a_sample_of_the_whole_range_round_trips_and_the_edges_refuse() {
        let first = earliest().expect("placed");
        let last = latest().expect("placed");
        for day in (first.0..=last.0).step_by(97) {
            let rd = Rd(day);
            let date = from_fixed(rd).expect("in range");
            assert_eq!(to_fixed(date), Ok(rd), "{date:?}");
        }
        assert_eq!(
            from_fixed(first).map(|d| (d.year, d.day)),
            Ok((MIN_YEAR, 1))
        );
        assert_eq!(from_fixed(last).map(|d| d.year), Ok(MAX_YEAR));
        assert_eq!(from_fixed(Rd(first.0 - 1)), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            from_fixed(Rd(last.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        let (y, _, _) = civil::from_rd(first);
        assert_eq!(y, 1900);
        let (y, _, _) = civil::from_rd(last);
        assert_eq!(y, 2100);
    }

    #[test]
    fn impossible_dates_are_refused_by_name() {
        let common = (3_640..3_660)
            .find(|year| !is_leap_year(*year).expect("in range"))
            .expect("a common year");
        assert_eq!(
            to_fixed(SamaritanDate {
                year: common,
                month: Month::leap(7),
                day: 1
            }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(SamaritanDate {
                year: 3_660,
                month: Month::regular(13),
                day: 1
            }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(SamaritanDate {
                year: 3_660,
                month: Month::leap(3),
                day: 1
            }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(first_month(3_660, 31)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(first_month(3_660, 0)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(first_month(MAX_YEAR + 1, 1)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            is_leap_year(MIN_YEAR - 1),
            Err(CalendarError::YearOutOfRange)
        );
        let calendar = SamaritanCalendar;
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(3_660, 1, 1).with_era("am")),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn the_public_months_run_from_the_sixth_and_the_thirteenth_is_leap_seven() {
        for biblical in 1..=13u8 {
            let public = to_public(biblical);
            assert_eq!(from_public(public), biblical);
        }
        assert_eq!(to_public(6), Month::regular(1));
        assert_eq!(to_public(12), Month::regular(7));
        assert_eq!(to_public(13), Month::leap(7));
        assert_eq!(to_public(1), Month::regular(8));
        assert_eq!(to_public(5), Month::regular(12));
        assert_eq!(from_public(Month::regular(0)), 0);
        assert_eq!(from_public(Month::leap(6)), 0);
    }

    #[test]
    fn the_metadata_says_what_the_calendar_is() {
        let meta = SamaritanCalendar.meta();
        assert_eq!(meta.id, ID);
        assert!(meta.has_leap_months && meta.is_astronomical);
        assert_eq!(meta.native_locales, &["smp", "he"]);
        assert_eq!(
            SamaritanCalendar.day_boundary(),
            hc_calendar::DayBoundary::Sunset
        );
    }
}
