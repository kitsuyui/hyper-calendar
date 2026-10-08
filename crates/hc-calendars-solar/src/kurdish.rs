//! The Kurdish calendar as the English Wikipedia reckons it: the Solar
//! Hijri day and month under Kurdish month names, and the Solar Hijri year
//! plus 1321 — `kurdish`.
//!
//! The one source read, Wikipedia's "Kurdish calendar", describes a solar
//! year that begins on Newroz, 21 March, "corresponding to the spring
//! equinox", with twelve months of 31, 31, 31, 31, 31, 31, 30, 30, 30, 30,
//! 30 and 29 or 30 days, Xakelêwe to Reşeme, and gives each month an
//! "Approximate Gregorian Span". It states no leap rule and no rule for the
//! year number. The date it displays for the day it is read is produced by
//! `Template:Kurdish_calendar_date_today`, whose wikitext is
//! `{{#time:xij}}`, the month name switched on `{{#time:xin}}`, and
//! `{{#expr: {{#time:xiY}} + 1321}}`: MediaWiki's `xi` codes are the
//! Iranian calendar, computed by `Language::tsToIranian`, the Pournader and
//! Toossi algorithm, which is the 33-year rule of
//! [`crate::persian_33`] counted from 1 Farvardin 979 on 20 March 1600.
//! So the page's reckoning is: the Solar Hijri date under the 33-year rule,
//! the months renamed, and the year 1321 ahead. That is what this module
//! carries, and the test `the_template_is_the_pournader_toossi_algorithm`
//! holds the two algorithms together day by day. The system document is
//! `docs/systems/kurdish.md`.
//!
//! The article's own statement of its epoch, "the Battle of Nineveh … in
//! 612 BC", gives 2638 for the year from 21 March 2026, not the 2726 the
//! page displays, so it is not carried; the year number is the template's.
//! The article says the calendar "is formally recognized for cultural and
//! official use in the Kurdistan Region of Iraq" and gives no date for
//! that, so the period of use is unrecorded.
//!
//! # What is not carried
//!
//! * A year on fixed Gregorian dates, every month beginning on the same
//!   Gregorian day each year with Reşeme taking its 30th day from the
//!   Gregorian February: no source read states it. The article's spans are
//!   headed approximate, and in the years the Solar Hijri Nowruz falls on
//!   20 March, 2024 and 2028 among them, that reading and the page disagree
//!   for the whole year.
//! * A Kurdish year over the astronomical Solar Hijri calendar
//!   (`persian`, in `hc-calendars-equinox`), the Iranian civil rule: it
//!   would need a source stating that the Kurdistan Region reckons the year
//!   by the equinox, and none was read. The two agree on every day from
//!   1 Farvardin 1178 to 29 Esfand 1634 — Kurdish 2499 to 2955 — and part
//!   company on the 33-year rule's leap day, 30 Esfand 1634, which the
//!   equinox calendar has as 1 Farvardin 1635. The span is the one Borkowski
//!   gives, not read here, as Heydari-Malayeri reports it; the day-by-day
//!   agreement is checked in `hc-calendars-equinox`.
//! * The Sorani spellings, خاکەلێوە to ڕەشەمە, for want of a Kurdish
//!   locale; the romanised forms are the shape's names.
//! * The Kurdistan Region's own instrument, which was not found and would
//!   replace the article.
//!
//! # Sources
//!
//! * Wikipedia, "Kurdish calendar",
//!   <https://en.wikipedia.org/wiki/Kurdish_calendar>, retrieved
//!   2026-10-04 and again, as wikitext, 2026-10-05
//!   (`wikipedia-kurdish-calendar`): the months, their lengths and
//!   approximate spans, Newroz on 21 March, the recognition in the
//!   Kurdistan Region, the 612 BC statement, and the date it displayed on
//!   4 October 2026, 12 Rezber 2726. A secondary source.
//! * Wikipedia, "Template:Kurdish calendar date today", wikitext read
//!   2026-10-05 (`wikipedia-template-kurdish-calendar-date-today`): the
//!   `#time:xi` codes and the `+ 1321`.
//! * MediaWiki, `includes/Language/Language.php`, `tsToIranian`, read
//!   2026-10-05 (`mediawiki-language-tstoiranian`): the algorithm the `xi`
//!   codes compute, and MediaWiki's Help:Extension:ParserFunctions
//!   (`mediawiki-parserfunctions-time`) for the codes being the Iranian
//!   calendar.
//!
//! # Exactness
//!
//! Exact to the date the page's template displays, for every day from
//! 20 March 1600, the first day the template's arithmetic is defined for.
//! Before it nothing is answered: [`from_fixed`] refuses a day before
//! [`EARLIEST`] with [`CalendarError::BeforeEpoch`], and [`to_fixed`]
//! refuses a year before [`MIN_YEAR`] with [`CalendarError::YearOutOfRange`].
//! The 33-year rule carried back past 1600 would give dates no page
//! displays, and a gap is the answer ADR 0013 gives to a year the sources
//! do not reach. Whether the Kurdistan Region reckons the year the same
//! way is not established by any source read.

use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{gregorian, persian_33};

/// The calendar identifier.
pub const ID: &str = "kurdish";

/// The month names, romanised as the source gives them, in the order of
/// the Solar Hijri months they stand for, Farvardin to Esfand.
pub const MONTHS: [&str; 12] = [
    "Xakelêwe",
    "Gulan",
    "Cozerdan",
    "Pûşper",
    "Gelawêj",
    "Xermanan",
    "Rezber",
    "Gelarêzan",
    "Sermawez",
    "Befranbar",
    "Rêbendan",
    "Reşeme",
];

/// How far the Kurdish year runs ahead of the Solar Hijri year: the
/// template's `+ 1321`.
pub const YEAR_OFFSET: i64 = 1321;

/// The month whose length varies, Reşeme, which stands for Esfand.
pub const RESHEME: u8 = 12;

/// The Solar Hijri year the template's arithmetic begins in: 979, which
/// begins on [`TEMPLATE_FROM`].
const TEMPLATE_YEAR: i64 = 979;

/// The earliest year this implementation converts: Solar Hijri 979, which
/// the template's 1600 begins, so 2300 in the Kurdish count. No source read
/// displays a date before it.
pub const MIN_YEAR: i64 = TEMPLATE_YEAR + YEAR_OFFSET;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// The first day the template's arithmetic is defined for: 20 March 1600,
/// 1 Farvardin 979, from which `tsToIranian` counts its days.
pub const TEMPLATE_FROM: Rd = match gregorian::to_fixed(1600, 3, 20) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Whether `year` has a 30th of Reşeme: its Solar Hijri year is a leap
/// year of the 33-year rule.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    persian_33::is_leap_year(year - YEAR_OFFSET)
}

/// The number of days in `month` of `year`, or `None` outside `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    persian_33::days_in_month(year - YEAR_OFFSET, month)
}

/// The number of days in `year`, 365 or 366.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    persian_33::days_in_year(year - YEAR_OFFSET)
}

/// The fixed day of Newroz, 1 Xakelêwe, which is 1 Farvardin.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    to_fixed(year, 1, 1)
}

/// The earliest fixed day this implementation converts: [`TEMPLATE_FROM`],
/// 1 Xakelêwe 2300, which is 1 Farvardin 979.
pub const EARLIEST: Rd = TEMPLATE_FROM;

/// The latest fixed day this implementation converts, the last day of
/// [`MAX_YEAR`].
pub const LATEST: Rd = match persian_33::to_fixed(MAX_YEAR - YEAR_OFFSET + 1, 1, 1) {
    Ok(rd) => Rd(rd.0 - 1),
    Err(_) => Rd(0),
};

/// The fixed day of a Kurdish date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    persian_33::to_fixed(year - YEAR_OFFSET, month, day)
}

/// The Kurdish year, month and day of a fixed day.
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
    match persian_33::from_fixed(rd) {
        Ok((year, month, day)) => Ok((year + YEAR_OFFSET, month, day)),
        Err(error) => Err(error),
    }
}

/// A Kurdish date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KurdishDate {
    /// The year, 1321 ahead of the Solar Hijri year.
    pub year: i64,
    /// The month, 1 for Xakelêwe through 12 for Reşeme.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl KurdishDate {
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

    /// The name of the month.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }
}

/// The Kurdish calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KurdishCalendar;

/// Twelve named months and the seven-day week.
const SHAPE: &[CycleShape] = &[
    CycleShape::named(MONTH, &MONTHS),
    CycleShape::fixed(WEEKDAY, 7),
];

impl Calendar for KurdishCalendar {
    type Date = KurdishDate;

    /// Unrecorded: the source says the calendar is recognised in the
    /// Kurdistan Region of Iraq and gives no day it was recognised from.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve named months and the week.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A year with a 30th of Reşeme.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Kurdish",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(KurdishDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        KurdishDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    /// MediaWiki's `Language::tsToIranian`, the algorithm "by Roozbeh
    /// Pournader and Mohammad Toossi", written out as the PHP has it: days
    /// since 1 January 1600, less 79, in cycles of 12 053 days of 33 years
    /// from 979, whose first four-year block opens with a 366-day year.
    fn ts_to_iranian(year: i64, month: u8, day: u8) -> (i64, u8, u8) {
        const GREG_DAYS: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        const IRANIAN_DAYS: [i64; 12] = [31, 31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 29];
        let gy = year - 1600;
        let gm = i64::from(month) - 1;
        let gd = i64::from(day) - 1;
        let mut g_day_no = 365 * gy + (gy + 3).div_euclid(4) - (gy + 99).div_euclid(100)
            + (gy + 399).div_euclid(400);
        for days in &GREG_DAYS[..gm as usize] {
            g_day_no += days;
        }
        if gm > 1 && ((gy % 4 == 0 && gy % 100 != 0) || gy % 400 == 0) {
            g_day_no += 1;
        }
        g_day_no += gd;
        let mut j_day_no = g_day_no - 79;
        assert!(
            j_day_no >= 0,
            "the template's arithmetic begins on 20 March 1600"
        );
        let j_np = j_day_no / 12_053;
        j_day_no %= 12_053;
        let mut jy = 979 + 33 * j_np + 4 * (j_day_no / 1461);
        j_day_no %= 1461;
        if j_day_no >= 366 {
            jy += (j_day_no - 1) / 365;
            j_day_no = (j_day_no - 1) % 365;
        }
        let mut i = 0;
        while i < 11 && j_day_no >= IRANIAN_DAYS[i] {
            j_day_no -= IRANIAN_DAYS[i];
            i += 1;
        }
        (jy, i as u8 + 1, j_day_no as u8 + 1)
    }

    #[test]
    fn the_template_is_the_pournader_toossi_algorithm() {
        // Every day from the first the template's arithmetic is defined for
        // to the end of Gregorian 2600: the page's reckoning and this module
        // agree, the month under its Kurdish name and the year 1321 ahead.
        let end = gregorian(2600, 12, 31);
        assert_eq!(TEMPLATE_FROM, gregorian(1600, 3, 20));
        assert_eq!(from_fixed(TEMPLATE_FROM), Ok((979 + YEAR_OFFSET, 1, 1)));
        for rd in TEMPLATE_FROM.0..=end.0 {
            let (y, m, d) = gregorian::from_fixed(Rd(rd)).unwrap();
            let (jy, jm, jd) = ts_to_iranian(y, m, d);
            assert_eq!(
                from_fixed(Rd(rd)),
                Ok((jy + YEAR_OFFSET, jm, jd)),
                "{y}-{m}-{d}"
            );
        }
    }

    #[test]
    fn the_source_wrote_12_rezber_2726_for_4_october_2026() {
        // 4 October 2026 is 12 Mehr 1405; the page displayed 12 Rezber 2726.
        assert_eq!(from_fixed(gregorian(2026, 10, 4)), Ok((2726, 7, 12)));
        assert_eq!(
            KurdishDate::new(2726, 7, 12).unwrap().month_name(),
            "Rezber"
        );
        // Newroz 2726 is 1 Farvardin 1405, 21 March 2026.
        assert_eq!(new_year(2726), Ok(gregorian(2026, 3, 21)));
        assert_eq!(from_fixed(gregorian(2026, 3, 20)), Ok((2725, 12, 29)));
        // Where the Solar Hijri Nowruz falls on 20 March, so does Newroz:
        // 1 Xakelêwe 2724 on 20 March 2024, 30 Reşeme 2724 on 20 March 2025
        // and 1 Xakelêwe 2728 on 20 March 2028, as the template computes them.
        assert_eq!(from_fixed(gregorian(2024, 3, 20)), Ok((2724, 1, 1)));
        assert_eq!(from_fixed(gregorian(2025, 3, 20)), Ok((2724, 12, 30)));
        assert_eq!(from_fixed(gregorian(2028, 3, 20)), Ok((2728, 1, 1)));
        assert_eq!(from_fixed(gregorian(2028, 3, 19)), Ok((2727, 12, 29)));
    }

    #[test]
    fn resheme_has_thirty_days_in_the_rules_leap_years() {
        for year in MIN_YEAR..=MAX_YEAR {
            assert_eq!(
                is_leap_year(year),
                persian_33::is_leap_year(year - YEAR_OFFSET),
                "{year}"
            );
            let length =
                new_year(year + 1).map_or(LATEST.0 + 1, |rd| rd.0) - new_year(year).unwrap().0;
            assert_eq!(length, i64::from(days_in_year(year)), "{year}");
            assert_eq!(
                days_in_month(year, RESHEME),
                Some(if is_leap_year(year) { 30 } else { 29 })
            );
        }
        // Solar Hijri 1403 is a leap year of the rule and 1404 to 1407 are
        // not; 1408 is.
        assert!(is_leap_year(2724));
        assert!(!is_leap_year(2725));
        assert!(!is_leap_year(2726));
        assert!(!is_leap_year(2727));
        assert!(is_leap_year(2729));
        assert_eq!(to_fixed(2724, RESHEME, 30), Ok(gregorian(2025, 3, 20)));
        assert_eq!(
            to_fixed(2726, RESHEME, 30),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(days_in_month(2726, 13), None);
        assert_eq!(to_fixed(2726, 0, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(
            to_fixed(MIN_YEAR - 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            KurdishCalendar.from_fields(&DateFields::ymd_leap_month(2726, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            KurdishCalendar.is_leap_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_range_begins_where_the_template_does() {
        // The template's first day, 20 March 1600, is 1 Farvardin 979 and
        // 1 Xakelêwe 2300; the days and years before it are out of range,
        // not carried back by the 33-year rule (ADR 0013, policy §4).
        assert_eq!(EARLIEST, TEMPLATE_FROM);
        assert_eq!(MIN_YEAR, 979 + YEAR_OFFSET);
        assert_eq!(to_fixed(MIN_YEAR, 1, 1), Ok(gregorian(1600, 3, 20)));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            to_fixed(MIN_YEAR - 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            KurdishCalendar.meta().earliest,
            Some(gregorian(1600, 3, 20))
        );
    }

    #[test]
    fn every_day_round_trips() {
        let recent = new_year(2600).unwrap().0;
        for rd in recent..recent + 40_000 {
            let date = KurdishCalendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(KurdishCalendar.to_fixed(date), Ok(Rd(rd)), "{rd}");
            let fields = KurdishCalendar.to_fields(date).unwrap();
            assert_eq!(KurdishCalendar.from_fields(&fields), Ok(date));
        }
        // Every day in a release build; every 9 973rd in a debug one, with
        // each year's first and last day.
        let year_starts = (MIN_YEAR..=MAX_YEAR).map(|year| new_year(year).unwrap().0);
        for rd in crate::sweep_days(EARLIEST.0, LATEST.0, 9_973, year_starts) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
        }
        assert_eq!(from_fixed(EARLIEST), Ok((MIN_YEAR, 1, 1)));
        assert_eq!(from_fixed(LATEST), Ok((MAX_YEAR, RESHEME, 29)));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}
