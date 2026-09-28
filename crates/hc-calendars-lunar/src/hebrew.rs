//! The arithmetic Hebrew calendar — CLDR `hebrew`.
//!
//! The system is written up in `docs/systems/hebrew.md` in the repository:
//! the molad and its parts, the nineteen-year cycle, the four dehiyyot and
//! why each exists, the six year lengths, Rosh Hashanah 5784 worked by hand
//! through the rules, how far the arithmetic drifts from the sky, what is
//! carried and what is not, and the sources. This page summarises it and
//! states the code's own facts.
//!
//! # What this is
//!
//! A lunisolar calendar with no astronomy in it. The months are fixed by
//! calculation from the *molad*, the mean conjunction, taken as exactly
//! 29 days, 12 hours and 793 parts of 1/1080 hour; seven years of every
//! nineteen carry a thirteenth month; and Rosh Hashanah is moved off
//! Sunday, Wednesday and Friday and away from a molad at or after noon by
//! the four *dehiyyot*, so that every year runs 353, 354 or 355 days, or
//! 383, 384 or 385 in a leap year. The rules are Maimonides' statement of
//! them in *Hilchot Kiddush HaChodesh*, chapters 6–8, in the arithmetic of
//! Reingold and Dershowitz, *Calendrical Calculations* (4th ed., Cambridge,
//! 2018), following the `hebrew-*` functions of their published source,
//! `calendar.l` in the `calendar-code2` repository (Apache License 2.0),
//! read 2026-09-25. The calculation is exact arithmetic on integers: this
//! module computes no solar longitude and calls nothing in `hc-astro`.
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
//! carries Purim. Writing Adar I as the intercalary repetition of Shevaṭ
//! rather than of Adar is what keeps a single [`hc_calendar::Month`] type
//! usable for this calendar and for the Chinese one, where the leap month
//! likewise follows the month it is named after. The internal arithmetic
//! below numbers the months from Nisan, as Reingold and Dershowitz do; the
//! public API is the Tishrei-first one throughout.
//!
//! # Accuracy
//!
//! Exact. There is nothing to approximate: the rules are arithmetic and this
//! is those rules. What the calendar is not is *astronomically* right: the
//! molad interval is 0.46 seconds longer than the mean synodic month, a day
//! in about 15 000 years, and the mean year of 365.2468 days is 0.0046 days
//! longer than the tropical year, so the festivals move later through the
//! seasons by about a day every 216 years. The document derives both.

use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};

/// The machine identifier CLDR uses for this calendar.
pub const ID: CalendarId = CalendarId("hebrew");

/// The era code of the Anno Mundi era.
pub const ERA: &str = "am";

/// The fixed day of 1 Tishrei AM 1, which is 7 October 3761 BCE in the Julian
/// calendar.
///
/// This is *epoch*, not observation: the year count was fixed retrospectively
/// and AM 1 has no events in it.
pub const EPOCH: Rd = Rd(-1_373_427);

/// The earliest Hebrew year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest Hebrew year this implementation converts.
///
/// The library's choice, not a limit of the calendar: neither Maimonides
/// nor Reingold and Dershowitz bound the year, and the rules run on without
/// end. AM 9 999 is where this implementation stops, and it is the span over
/// which the structural tests check every year.
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
/// All four dehiyyot are applied here: two inside `elapsed_days` and two
/// inside `new_year_delay`.
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

/// The year tradition attributes the fixed calendar to Hillel II in: 670 of
/// the Seleucid era, 358/9 CE, which is 4119 AM.
pub const TRADITIONAL_ADOPTION_YEAR: i64 = 4_119;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Tradition, first recorded in a responsum of Hai Gaon of 992, attributes the fixed calendar \
    to Hillel II in 670 of the Seleucid era, 358/9 CE, 4119 AM; the modern reading has it \
    reach its exact form in 922–924 [wikipedia-hillel-ii], and docs/systems/hebrew.md \
    carries both; the calendar of Jewish religious life and one of Israel's two civil \
    calendars today";

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
///
/// Reingold and Dershowitz compute the same day as 30 Paremhat, the last
/// day of the Coptic seventh month, in a Coptic year that is 17 modulo 28
/// (*Calendrical Calculations*, 4th ed., 2018, `birkath-ha-hama` in their
/// published `calendar.l`, read 2026-09-26); a test holds this function to
/// that rule. The dates the tests pin,
/// 7 April 1897 to 9 April 2149, are those of Wikipedia's "Birkat
/// Hachamah" (retrieved 2026-09-26), which gives no source for them.
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

/// A *birkat hachama*: Wednesday 8 April 2009, 14 Nisan 5769, as Wikipedia's
/// "Birkat Hachamah" lists it (retrieved 2026-09-26) and as the rule of
/// Reingold and Dershowitz gives it.
pub const BIRKAT_HACHAMA_ANCHOR: Rd = Rd(733_505);

/// One of the four *tekufot*, the seasons of the solar year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tekufah {
    /// *Tekufat Tishrei*, the Sun at the head of Libra, the autumn
    /// equinox that opens the year.
    Tishrei,
    /// *Tekufat Tevet*, the Sun at the head of Capricorn, the winter
    /// solstice.
    Tevet,
    /// *Tekufat Nisan*, the Sun at the head of Aries, the spring equinox.
    Nisan,
    /// *Tekufat Tammuz*, the Sun at the head of Cancer, the summer solstice.
    Tammuz,
}

impl Tekufah {
    /// The four in the order a Hebrew year meets them.
    pub const ALL: [Self; 4] = [Self::Tishrei, Self::Tevet, Self::Nisan, Self::Tammuz];

    /// Minutes from this *tekufah* of a Hebrew year to the same year's
    /// *tekufat Nisan*, which is the reckoning's anchor: a season is 91
    /// days and 7½ hours.
    const fn minutes_before_nisan(self) -> i64 {
        match self {
            Self::Tishrei => 2 * SHMUEL_SEASON_MINUTES,
            Self::Tevet => SHMUEL_SEASON_MINUTES,
            Self::Nisan => 0,
            Self::Tammuz => -SHMUEL_SEASON_MINUTES,
        }
    }
}

/// Minutes in a season of Shmuel's year, 91 days and 7½ hours: "between
/// the start of each of the successive seasons of the year, there will be
/// ninety-one days and seven and one-half hours" (Maimonides, *Hilkhot
/// Kiddush HaChodesh* 9:2).
pub const SHMUEL_SEASON_MINUTES: i64 = 91 * 1_440 + 450;

/// Minutes in Shmuel's year of 365¼ days.
pub const SHMUEL_YEAR_MINUTES: i64 = 4 * SHMUEL_SEASON_MINUTES;

/// *Tekufat Nisan* 5769: Tuesday 7 April 2009 at six in the evening,
/// Jerusalem mean time, "the beginning of the night of the fourth day" at
/// which the 28-year cycle begins again (Maimonides, *Hilkhot Berakhot*
/// 10:18), as minutes from RD 0.
const SHMUEL_NISAN_5769_MINUTES: i64 = (BIRKAT_HACHAMA_ANCHOR.0 - 1) * 1_440 + 18 * 60;

/// The Hebrew year whose *tekufat Nisan* is the anchor.
const SHMUEL_ANCHOR_YEAR: i64 = 5_769;

/// A *tekufah* of Shmuel's reckoning as minutes from midnight at the start
/// of RD 0, in Jerusalem mean solar time.
const fn shmuel_tekufah_minutes(year: i64, tekufah: Tekufah) -> i64 {
    SHMUEL_NISAN_5769_MINUTES + (year - SHMUEL_ANCHOR_YEAR) * SHMUEL_YEAR_MINUTES
        - tekufah.minutes_before_nisan()
}

/// The moment of a *tekufah* of the Hebrew year `year` by Shmuel's
/// reckoning, in Jerusalem mean solar time as [`molad`] is.
///
/// Shmuel's solar year is 365¼ days, and "between the start of each of the
/// successive seasons of the year, there will be ninety-one days and seven
/// and one-half hours"; the first *tekufat Nisan* "took place at the
/// beginning of the fourth day", the night that began on Tuesday evening
/// (Maimonides, *Hilkhot Kiddush HaChodesh* 9:2–9:4, in Wikisource's Hebrew
/// and Touger's English on Sefaria, read 2026-09-29). The year's
/// *tekufot* are Tishrei and Tevet before its Nisan and Tammuz after it, so
/// that Tishrei 5786 is 7 October 2025. The hours are the reckoning's equal
/// hours, the night beginning at six in the evening of mean time (Simmons,
/// *Sinai* 111, secondary); Hebrew Wikipedia's "ארבע התקופות" computes
/// its dated table the same way, in Jerusalem mean time. Simmons's own
/// clock times, thirteen minutes later, are another reading of the hour
/// and are not this one.
///
/// [`birkat_hachama_on_or_after`] is the day after the *tekufat Nisan*
/// that begins the 28-year cycle.
#[must_use]
pub fn shmuel_tekufah(year: i64, tekufah: Tekufah) -> Moment {
    let minutes = shmuel_tekufah_minutes(year, tekufah);
    Moment(minutes.div_euclid(1_440) as f64 + minutes.rem_euclid(1_440) as f64 / 1_440.0)
}

/// The fixed day whose Hebrew day a *tekufah* of Shmuel's reckoning falls
/// in: the civil day of the moment, or the next one from six in the
/// evening, when the reckoning's night begins, together with the minutes
/// of Jerusalem mean time since that civil midnight.
///
/// Maimonides' own example: *tekufat Nisan* of 4930 "on the night of the
/// fifth day at midnight", and "on the eighth of Nisan" (9:5–9:7).
#[must_use]
pub const fn shmuel_tekufah_day(year: i64, tekufah: Tekufah) -> (Rd, i64) {
    let minutes = shmuel_tekufah_minutes(year, tekufah);
    let civil = minutes.div_euclid(1_440);
    let clock = minutes.rem_euclid(1_440);
    if clock >= 18 * 60 {
        (Rd(civil + 1), clock)
    } else {
        (Rd(civil), clock)
    }
}

/// Years in the sabbatical cycle.
pub const SABBATICAL_CYCLE_YEARS: u8 = 7;

/// The place of a Hebrew year in the seven-year sabbatical cycle, 1 to 7,
/// the seventh being the sabbatical year, *shemittah*.
///
/// The count in use makes the years divisible by seven sabbatical years.
/// Wikipedia's "Shmita" (retrieved 2026-09-27) lists 5712 (1951–52) to
/// 5782 (2021–22), seven years apart, and Chabad.org's "What Is
/// Shemitah?" (retrieved 2026-09-27) names 5789, "which runs from
/// September 20, 2028 to September 9, 2029", as the next. Both count the
/// sabbatical year from Rosh Hashanah, so the year is the Hebrew year of
/// this calendar. The same Chabad.org page says the cycle's first year
/// was 3829; counting sevens from there would make 5788 the sabbatical
/// year and not the 5789 it names, so the named years are the ones
/// carried. The Jubilee is not carried: whether it is the forty-ninth year
/// or the fiftieth is disputed, and no source read dates a Jubilee year
/// of the present count.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn sabbatical_cycle_year(year: i64) -> CalendarResult<u8> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    let cycle = SABBATICAL_CYCLE_YEARS as i64;
    Ok(hc_core::math::amod(year, cycle) as u8)
}

/// Whether a Hebrew year is a sabbatical year, *shemittah*: the seventh
/// of [`sabbatical_cycle_year`]'s count.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn is_sabbatical_year(year: i64) -> CalendarResult<bool> {
    match sabbatical_cycle_year(year) {
        Ok(place) => Ok(place == SABBATICAL_CYCLE_YEARS),
        Err(error) => Err(error),
    }
}

/// The anniversary in `year` of a death on `death`: the *yahrzeit*, by
/// Reingold and Dershowitz's `yahrzeit` (`calendar.l`, read 2026-09-26).
///
/// A date the later year has is kept on that date. The four that a later
/// year may lack follow the rules the book's code states:
///
/// - **30 Ḥeshvan.** If the year after the death had no 30 Ḥeshvan, the
///   anniversary is the day before 1 Kislev every year; if it had one, the
///   30th is kept, and in a year without it the day after the 29th.
/// - **30 Kislev.** The same, with the day before 1 Ṭevet.
/// - **Adar II.** Kept in Adar in a common year and Adar II in a leap one —
///   the last month of the year.
/// - **Adar of a common year, or Adar I.** Kept in Adar I in a leap year and
///   Adar in a common one; 30 Adar I, in a common year, on 30 Shevaṭ.
///
/// Customs differ, and the rules are the published code's, not a ruling;
/// one yahrzeit and one Adar birthday are checked against Hebcal's
/// anniversary calculator (`hebcal-yahrzeit`, queried 2026-09-27).
///
/// # Errors
///
/// Returns a [`CalendarError`] when `death` is not a Hebrew date, or when
/// `year` is outside [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn yahrzeit(death: HebrewDate, year: i64) -> CalendarResult<Rd> {
    let HebrewDate {
        year: died,
        month,
        day,
    } = death;
    to_fixed(died, month, day)?;
    if month == Month::regular(2) && day == 30 && !is_long_heshvan(died + 1) {
        return Ok(Rd(to_fixed(year, Month::regular(3), 1)?.0 - 1));
    }
    if month == Month::regular(3) && day == 30 && is_short_kislev(died + 1) {
        return Ok(Rd(to_fixed(year, Month::regular(4), 1)?.0 - 1));
    }
    let adar = Month::regular(6);
    if month == adar && is_leap_year(died) {
        return to_fixed(year, adar, day);
    }
    // Adar of a common year and Adar I are one month to the book's code.
    let first_adar = month == Month::leap(5) || month == adar;
    if first_adar && day == 30 && !is_leap_year(year) {
        return to_fixed(year, Month::regular(5), 30);
    }
    anniversary(
        year,
        if first_adar {
            first_adar_in(year)
        } else {
            month
        },
        day,
    )
}

/// The anniversary in `year` of a birth on `birth`, by Reingold and
/// Dershowitz's `hebrew-birthday` (`calendar.l`, read 2026-09-26): the
/// same date, except that a birth in the last month of the year — Adar, or
/// Adar II — is kept in the last month of the later year, and a birth in
/// Adar I in Adar of a common year. A day the later month lacks runs on
/// into the next month.
///
/// # Errors
///
/// Returns a [`CalendarError`] when `birth` is not a Hebrew date, or when
/// `year` is outside [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn birthday(birth: HebrewDate, year: i64) -> CalendarResult<Rd> {
    let HebrewDate {
        year: born,
        month,
        day,
    } = birth;
    to_fixed(born, month, day)?;
    let adar = Month::regular(6);
    if month == adar {
        return to_fixed(year, adar, day);
    }
    let kept = if month == Month::leap(5) {
        first_adar_in(year)
    } else {
        month
    };
    anniversary(year, kept, day)
}

/// Adar I in a leap year, Adar in a common one.
const fn first_adar_in(year: i64) -> Month {
    if is_leap_year(year) {
        Month::leap(5)
    } else {
        Month::regular(6)
    }
}

/// The day that is `day − 1` days after the first of `month` in `year`,
/// which runs on into the next month when the month is shorter.
fn anniversary(year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
    Ok(Rd(to_fixed(year, month, 1)?.0 + i64::from(day) - 1))
}

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

    /// From Rosh Hashanah of 4119 AM, the autumn of 358 CE, the year
    /// tradition gives for Hillel II's fixed calendar, and never abandoned.
    /// The tradition is carried as a tradition: the calendar's exact modern
    /// form is later, and the years before 4119 are the fixed rules
    /// projected back.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(new_year(TRADITIONAL_ADOPTION_YEAR), USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// A year with Adar I. A common year is 353, 354 or 355 days long and
    /// none of the three is leap.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    /// The Hebrew day begins at sunset, so a Hebrew date covers the second
    /// half of one civil day and the first half of the next, and is named
    /// by the next: Rosh Hashanah 5784 began at sunset on Friday
    /// 15 September 2023 and is Saturday the 16th (Hebcal, "Jewish Holidays
    /// 5784", retrieved 2026-09-25), the fixed day this calendar gives it.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset(hc_calendar::DayNaming::ByEnd)
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
        HebrewDate::new(fields.year, fields.require_month()?, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::Weekday;
    use hc_calendar::gregorian;

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
        let rd = gregorian::to_fixed_saturating(2023, 9, 16);
        assert_eq!(new_year(5_784), rd);
        assert_eq!(to_fixed(5_784, Month::regular(1), 1), Ok(rd));
        assert_eq!(from_fixed(rd), Ok((5_784, Month::regular(1), 1)));
    }

    #[test]
    fn passover_5784_was_the_twenty_third_of_april_2024() {
        // A published anchor: 15 Nisan 5784 fell on 2024-04-23.
        let rd = gregorian::to_fixed_saturating(2024, 4, 23);
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

    /// Maimonides, *Hilkhot Kiddush HaChodesh* 9:4–9:7: the hours each
    /// *tekufah* can fall at, the weekday rule and the example of 4930;
    /// the *birkat hachama* of 2009; and Hebrew Wikipedia's table of
    /// 5786–5791 in Jerusalem mean time.
    #[test]
    fn shmuels_tekufot_are_maimonides_and_the_tables() {
        use Tekufah::*;
        // 9:4: Nisan at the start of the night or day, at midnight or
        // midday; Tammuz at 1½ or 7½ hours, Tishrei at 3 or 9, Tevet at 4½
        // or 10½, of the day or the night, counted from six o'clock.
        for year in 5_600..5_900 {
            for (tekufah, hours) in [
                (Nisan, [0.0, 6.0]),
                (Tammuz, [1.5, 7.5]),
                (Tishrei, [3.0, 9.0]),
                (Tevet, [4.5, 10.5]),
            ] {
                let (_, clock) = shmuel_tekufah_day(year, tekufah);
                let from_six = ((clock - 360).rem_euclid(720)) as f64 / 60.0;
                assert!(hours.contains(&from_six), "{year} {tekufah:?}: {from_six}");
            }
        }
        // 9:5–9:7: Nisan 4930 "on the night of the fifth day at midnight",
        // "on the eighth of Nisan"; Tammuz on Thursday at 1½ hours of the
        // day, Tevet 4½ hours into the night of the sixth day.
        let (day, clock) = shmuel_tekufah_day(4_930, Nisan);
        assert_eq!(clock, 0);
        assert_eq!(
            hc_calendar::Weekday::from_rd(day),
            hc_calendar::Weekday::Thursday
        );
        assert_eq!(from_fixed(day), Ok((4_930, Month::regular(7), 8)));
        let (day, clock) = shmuel_tekufah_day(4_930, Tammuz);
        assert_eq!(
            (hc_calendar::Weekday::from_rd(day), clock),
            (hc_calendar::Weekday::Thursday, 7 * 60 + 30)
        );
        let (day, clock) = shmuel_tekufah_day(4_931, Tevet);
        assert_eq!(
            (hc_calendar::Weekday::from_rd(day), clock),
            (hc_calendar::Weekday::Friday, 22 * 60 + 30)
        );
        // Berakhot 10:18 and the blessing of 8 April 2009: the tekufah at
        // six on Tuesday evening, the blessing the next morning.
        let (day, clock) = shmuel_tekufah_day(5_769, Nisan);
        assert_eq!((day, clock), (BIRKAT_HACHAMA_ANCHOR, 18 * 60));
        for cycle in -3..=5 {
            let (day, clock) = shmuel_tekufah_day(5_769 + 28 * cycle, Nisan);
            assert_eq!(clock, 18 * 60);
            assert!(is_birkat_hachama(day));
        }
        // Hebrew Wikipedia, "ארבע התקופות", the table of the coming years
        // (retrieved 2026-09-29), less its 21 minutes of Israel time:
        // Tishrei 5786 on 7 October 2025 at 9:00, Tevet on 6 January 2026
        // at 16:30, Nisan on 8 April 2026 at 0:00, Tammuz on 8 July 2026 at
        // 7:30; Tishrei 5788 on 7 October 2027 at 21:00, whose Hebrew day
        // is the 8th.
        let at = |year: i64, tekufah: Tekufah| {
            let minutes = shmuel_tekufah_minutes(year, tekufah);
            (
                gregorian::ymd(Rd(minutes.div_euclid(1_440))),
                minutes.rem_euclid(1_440),
            )
        };
        assert_eq!(at(5_786, Tishrei), ((2025, 10, 7), 9 * 60));
        assert_eq!(at(5_786, Tevet), ((2026, 1, 6), 16 * 60 + 30));
        assert_eq!(at(5_786, Nisan), ((2026, 4, 8), 0));
        assert_eq!(at(5_786, Tammuz), ((2026, 7, 8), 7 * 60 + 30));
        assert_eq!(at(5_787, Tishrei), ((2026, 10, 7), 15 * 60));
        assert_eq!(at(5_791, Tammuz), ((2031, 7, 8), 13 * 60 + 30));
        assert_eq!(at(5_788, Tishrei), ((2027, 10, 7), 21 * 60));
        assert_eq!(
            gregorian::ymd(shmuel_tekufah_day(5_788, Tishrei).0),
            (2027, 10, 8)
        );
        let moment = shmuel_tekufah(5_786, Tishrei);
        assert_eq!(moment.day(), gregorian::to_fixed_saturating(2025, 10, 7));
        assert!((moment.0 - moment.day().0 as f64 - 0.375).abs() < 1e-9);
    }

    /// The prayer for rain outside the Land of Israel from the sixtieth
    /// day of *tekufat Tishrei*, the *tekufah*'s own day the first (Simmons,
    /// *Sinai* 111, after R. Yose): on the Julian calendar "22 November in
    /// most years and 23 November before a leap year" (Hebrew Wikipedia,
    /// "שאלת גשמים", after Bar Ḥiyya, secondary), and "until the year 2100,
    /// in a regular year we start saying the prayer for rain on the night
    /// of December 4, and in the year before a (civil) leap year ... on the
    /// night of December 5" (Shurpin, Chabad.org, secondary). 2099, before
    /// a Julian leap year and a Gregorian common one, already has the later
    /// day.
    #[test]
    fn the_sixtieth_day_of_tekufat_tishrei_is_the_diaspora_s_prayer_for_rain() {
        for gregorian_year in 1_583..2_400 {
            let (day, _) = shmuel_tekufah_day(gregorian_year + 3_761, Tekufah::Tishrei);
            let sixtieth = Rd(day.0 + 59);
            let (year, month, date) = hc_calendars_solar::julian::from_fixed(sixtieth).unwrap();
            let julian_leap = hc_calendars_solar::julian::is_leap_year(gregorian_year + 1);
            assert_eq!(
                (year, month, date),
                (gregorian_year, 11, if julian_leap { 23 } else { 22 }),
                "{gregorian_year}"
            );
            if (2001..2099).contains(&gregorian_year) {
                let eve = gregorian::ymd(Rd(sixtieth.0 - 1));
                let expected = if gregorian::is_leap_year(gregorian_year + 1) {
                    5
                } else {
                    4
                };
                assert_eq!(eve, (gregorian_year, 12, expected), "{gregorian_year}");
            }
        }
        let (day, _) = shmuel_tekufah_day(2_099 + 3_761, Tekufah::Tishrei);
        assert_eq!(gregorian::ymd(Rd(day.0 + 59)), (2099, 12, 6));
    }

    #[test]
    fn birkat_hachama_falls_on_a_wednesday_every_twenty_eight_years() {
        assert_eq!(gregorian::ymd(BIRKAT_HACHAMA_ANCHOR), (2009, 4, 8));
        assert!(is_birkat_hachama(BIRKAT_HACHAMA_ANCHOR));
        for cycle in -60..60i64 {
            let rd = Rd(BIRKAT_HACHAMA_ANCHOR.0 + cycle * BIRKAT_HACHAMA_CYCLE_DAYS);
            assert!(is_birkat_hachama(rd));
            assert_eq!(Weekday::from_rd(rd), Weekday::Wednesday, "cycle {cycle}");
        }
    }

    /// Reingold and Dershowitz's `birkath-ha-hama` (`reingold2018code`):
    /// the blessing falls on 30 Paremhat, Coptic month 7 day 30, in a
    /// Coptic year that is 17 modulo 28. The Coptic day is their
    /// `fixed-from-coptic`, which `hc_calendars_solar::coptic` implements.
    #[test]
    fn birkat_hachama_is_reingold_and_dershowitzs_thirtieth_of_paremhat() {
        let paremhat_30 = |year: i64| hc_calendars_solar::coptic::to_fixed(year, 7, 30).unwrap();
        assert_eq!(paremhat_30(1725), BIRKAT_HACHAMA_ANCHOR);
        for year in 1..3_000i64 {
            assert_eq!(
                is_birkat_hachama(paremhat_30(year)),
                year.rem_euclid(28) == 17,
                "Coptic year {year}"
            );
        }
    }

    #[test]
    fn the_recent_and_next_birkat_hachama_are_the_published_ones() {
        // Wikipedia, "Birkat Hachamah", retrieved 2026-09-26: Wednesday
        // 7 April 1897, 8 April 1925 to 2093, 9 April 2121 and 2149.
        for pair in [
            (1897, 4, 7),
            (1925, 4, 8),
            (1953, 4, 8),
            (1981, 4, 8),
            (2009, 4, 8),
            (2037, 4, 8),
            (2065, 4, 8),
            (2093, 4, 8),
            (2121, 4, 9),
            (2149, 4, 9),
        ]
        .windows(2)
        {
            let ((y0, m0, d0), (y1, m1, d1)) = (pair[0], pair[1]);
            let rd = gregorian::to_fixed_saturating(y0, m0, d0);
            assert!(is_birkat_hachama(rd), "{y0}-{m0}-{d0}");
            assert_eq!(
                birkat_hachama_on_or_after(Rd(rd.0 + 1)),
                gregorian::to_fixed_saturating(y1, m1, d1)
            );
        }
        assert_eq!(
            birkat_hachama_on_or_after(gregorian::to_fixed_saturating(2009, 4, 9)),
            gregorian::to_fixed_saturating(2037, 4, 8)
        );
        assert_eq!(
            birkat_hachama_on_or_after(gregorian::to_fixed_saturating(1982, 1, 1)),
            gregorian::to_fixed_saturating(2009, 4, 8)
        );
        assert_eq!(
            birkat_hachama_on_or_after(gregorian::to_fixed_saturating(1981, 4, 8)),
            gregorian::to_fixed_saturating(1981, 4, 8)
        );
        assert_eq!(
            birkat_hachama_on_or_after(gregorian::to_fixed_saturating(1870, 1, 1)),
            gregorian::to_fixed_saturating(1897, 4, 7)
        );
    }

    #[test]
    fn the_sabbatical_years_are_the_published_ones() {
        // Wikipedia, "Shmita", retrieved 2026-09-27: 5712 (1951–52) to
        // 5782 (2021–22).
        for year in (5_712..=5_782).step_by(7) {
            assert_eq!(is_sabbatical_year(year), Ok(true), "{year}");
            assert_eq!(sabbatical_cycle_year(year), Ok(7), "{year}");
        }
        // Chabad.org, "What Is Shemitah?", retrieved 2026-09-27: 5789, "from
        // September 20, 2028 to September 9, 2029", each the evening before
        // the day this calendar starts the year on.
        assert_eq!(is_sabbatical_year(5_789), Ok(true));
        assert_eq!(new_year(5_789), gregorian::to_fixed_saturating(2028, 9, 21));
        assert_eq!(new_year(5_790), gregorian::to_fixed_saturating(2029, 9, 10));
        for year in 5_783..=5_788 {
            assert_eq!(is_sabbatical_year(year), Ok(false), "{year}");
            assert_eq!(sabbatical_cycle_year(year), Ok((year - 5_782) as u8));
        }
        assert_eq!(sabbatical_cycle_year(1), Ok(1));
        assert_eq!(sabbatical_cycle_year(0), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            is_sabbatical_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
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

    #[test]
    fn a_yahrzeit_keeps_the_date_when_the_later_year_has_it() {
        let death = HebrewDate::new(5780, Month::regular(4), 10).expect("10 Tevet 5780");
        for year in 5781..=5800 {
            assert_eq!(yahrzeit(death, year), to_fixed(year, Month::regular(4), 10));
            assert_eq!(birthday(death, year), to_fixed(year, Month::regular(4), 10));
        }
    }

    #[test]
    fn the_adar_anniversaries_agree_with_hebcals_calculator() {
        // A check against a published calculator, not a source of the
        // rules: Hebcal's Hebrew anniversary calculator (`hebcal-yahrzeit`),
        // queried 2026-09-27 for a death and a birth on 8 March 2023,
        // 15 Adar 5783, in a common year. In the leap year 5787 it keeps the
        // yahrzeit on 15 Adar I, 22 February 2027, and the birthday on
        // 15 Adar II, 24 March 2027; in the common years 5786 and 5788 both
        // on 15 Adar, 4 March 2026 and 13 March 2028.
        let date = HebrewDate::new(5783, Month::regular(6), 15).expect("15 Adar 5783");
        assert_eq!(
            to_fixed(5783, Month::regular(6), 15),
            Ok(gregorian::to_fixed_saturating(2023, 3, 8))
        );
        for (year, yahrzeit_day, birthday_day) in [
            (5786, (2026, 3, 4), (2026, 3, 4)),
            (5787, (2027, 2, 22), (2027, 3, 24)),
            (5788, (2028, 3, 13), (2028, 3, 13)),
        ] {
            let (y, m, d) = yahrzeit_day;
            assert_eq!(
                yahrzeit(date, year),
                Ok(gregorian::to_fixed_saturating(y, m, d)),
                "{year}"
            );
            let (y, m, d) = birthday_day;
            assert_eq!(
                birthday(date, year),
                Ok(gregorian::to_fixed_saturating(y, m, d)),
                "{year}"
            );
        }
    }

    #[test]
    fn a_yahrzeit_in_adar_follows_the_books_rules() {
        let leap = (5780..5800)
            .find(|&y| is_leap_year(y))
            .expect("a leap year");
        let common = (5780..5800)
            .find(|&y| !is_leap_year(y))
            .expect("a common year");
        // Adar II: the last month of every later year.
        let adar_ii = HebrewDate::new(5784, Month::regular(6), 7).expect("5784 is leap");
        assert!(is_leap_year(5784));
        assert_eq!(
            yahrzeit(adar_ii, leap),
            to_fixed(leap, Month::regular(6), 7)
        );
        assert_eq!(
            yahrzeit(adar_ii, common),
            to_fixed(common, Month::regular(6), 7)
        );
        // Adar of a common year: Adar I in a leap year.
        let adar = HebrewDate::new(5783, Month::regular(6), 7).expect("5783 is common");
        assert!(!is_leap_year(5783));
        assert_eq!(yahrzeit(adar, leap), to_fixed(leap, Month::leap(5), 7));
        assert_eq!(
            yahrzeit(adar, common),
            to_fixed(common, Month::regular(6), 7)
        );
        // 30 Adar I: 30 Shevat when the later year has no Adar I.
        let thirtieth = HebrewDate::new(5784, Month::leap(5), 30).expect("Adar I has 30 days");
        assert_eq!(
            yahrzeit(thirtieth, common),
            to_fixed(common, Month::regular(5), 30)
        );
        assert_eq!(
            yahrzeit(thirtieth, leap),
            to_fixed(leap, Month::leap(5), 30)
        );
    }

    #[test]
    fn a_birthday_in_adar_follows_the_books_rules() {
        let leap = (5780..5800)
            .find(|&y| is_leap_year(y))
            .expect("a leap year");
        let common = (5780..5800)
            .find(|&y| !is_leap_year(y))
            .expect("a common year");
        // Adar of a common year and Adar II: the last month, Adar II in a
        // leap year — where Adar of a common year's yahrzeit is Adar I.
        let adar = HebrewDate::new(5783, Month::regular(6), 7).expect("5783 is common");
        assert_eq!(birthday(adar, leap), to_fixed(leap, Month::regular(6), 7));
        assert_ne!(birthday(adar, leap), yahrzeit(adar, leap));
        let adar_i = HebrewDate::new(5784, Month::leap(5), 7).expect("5784 is leap");
        assert_eq!(
            birthday(adar_i, common),
            to_fixed(common, Month::regular(6), 7)
        );
        assert_eq!(birthday(adar_i, leap), to_fixed(leap, Month::leap(5), 7));
        // 30 Adar I in a common year runs on to 1 Nisan.
        let thirtieth = HebrewDate::new(5784, Month::leap(5), 30).expect("Adar I has 30 days");
        assert_eq!(
            birthday(thirtieth, common),
            to_fixed(common, Month::regular(7), 1)
        );
    }

    #[test]
    fn the_thirtieth_of_heshvan_and_kislev_depend_on_the_first_anniversary() {
        let long = |y: i64| is_long_heshvan(y);
        // A death on 30 Heshvan whose first anniversary year had no 30th is
        // kept on the day before 1 Kislev, whatever the later year.
        let died = (5700..5800)
            .find(|&y| long(y) && !long(y + 1))
            .expect("such a year");
        let death = HebrewDate::new(died, Month::regular(2), 30).expect("a long Heshvan");
        for year in died + 1..died + 20 {
            assert_eq!(
                yahrzeit(death, year).map(|rd| rd.0 + 1),
                to_fixed(year, Month::regular(3), 1).map(|rd| rd.0)
            );
        }
        // One whose first anniversary had a 30th keeps the 30th, and in a
        // year without it runs on to 1 Kislev.
        let died = (5700..5800)
            .find(|&y| long(y) && long(y + 1))
            .expect("such a year");
        let death = HebrewDate::new(died, Month::regular(2), 30).expect("a long Heshvan");
        let short_year = (died + 2..died + 30)
            .find(|&y| !long(y))
            .expect("a short Heshvan");
        assert_eq!(
            yahrzeit(death, short_year),
            to_fixed(short_year, Month::regular(3), 1)
        );
        // 30 Kislev with a short Kislev in the next year: the day before
        // 1 Tevet.
        let died = (5700..5800)
            .find(|&y| !is_short_kislev(y) && is_short_kislev(y + 1))
            .expect("such a year");
        let death = HebrewDate::new(died, Month::regular(3), 30).expect("a full Kislev");
        let full_year = (died + 2..died + 30)
            .find(|&y| !is_short_kislev(y))
            .expect("a full Kislev");
        assert_eq!(
            yahrzeit(death, full_year).map(|rd| rd.0 + 1),
            to_fixed(full_year, Month::regular(4), 1).map(|rd| rd.0)
        );
    }

    #[test]
    fn an_anniversary_of_a_date_that_does_not_exist_is_an_error() {
        let bad = HebrewDate {
            year: 5783,
            month: Month::leap(5),
            day: 1,
        };
        assert!(yahrzeit(bad, 5790).is_err());
        assert!(birthday(bad, 5790).is_err());
        let good = HebrewDate::new(5780, Month::regular(1), 1).expect("Rosh Hashanah");
        assert!(yahrzeit(good, MAX_YEAR + 1).is_err());
    }
}
