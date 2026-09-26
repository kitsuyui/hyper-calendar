//! Pattern-driven formatting and parsing, in both vocabularies.
//!
//! # Why both
//!
//! There are two pattern languages in use and neither is going away.
//! `strftime` is what every C library, every shell script and most log
//! formats speak; CLDR's field patterns ([Unicode TR 35]) are what every
//! localisation stack speaks, and are the only one of the two that can
//! express "the month name in the form this language uses inside a date". A
//! library that implements one of them forces its callers to translate, and
//! translating between them is lossy, so both are here:
//! [`strftime`] and [`cldr`].
//!
//! [Unicode TR 35]: https://unicode.org/reports/tr35/tr35-dates.html
//!
//! # Locales
//!
//! Every name — month, weekday, day period, era, quarter — comes from
//! [`hc_i18n`] when a [`Locale`] is supplied on the [`FormatContext`]. With
//! no locale the names are the English ones of the POSIX `C` locale, which
//! is what `strftime` without `setlocale` produces and what an RFC-defined
//! protocol field needs. The `C` names live in this crate rather than being
//! looked up as the `en` locale, because they are part of the *format*, not
//! a translation of it: CLDR's own root locale names months `M01`…`M12`.
//!
//! ```
//! use hc_calendar::{CivilDateTime, CivilTime, Rd};
//! use hc_format::patterns::{FormatContext, strftime};
//!
//! let noon = CivilDateTime::new(Rd(739_880), CivilTime::hms(12, 0, 0)?);
//! let mut out = String::new();
//! strftime::format(&mut out, "%Y-%m-%d %H:%M:%S", &FormatContext::new(noon))?;
//! assert_eq!(out, "2026-09-21 12:00:00");
//! # Ok::<(), Box<dyn core::error::Error>>(())
//! ```

pub mod cldr;
pub mod strftime;

use hc_calendar::{CivilDateTime, Weekday};
use hc_calendars_solar::{gregorian, iso_week};
use hc_i18n::names::{DayPeriod, NameContext, NameWidth};
use hc_i18n::{Locale, names};

use crate::error::{ValueError, ValueResult};
use crate::value::ZoneInfo;

/// The CLDR calendar identifier every pattern in this module formats in.
///
/// Patterns are a Gregorian vocabulary: `%m` is a Gregorian month and `MMMM`
/// is a Gregorian month name. Other calendars have their own field sets and
/// belong behind [`hc_calendar::Calendar`], not behind a `%` escape.
const CALENDAR: hc_calendar::CalendarId = hc_calendar::CalendarId("gregory");

/// The month names of the POSIX `C` locale.
const C_MONTHS_WIDE: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// The abbreviated month names of the POSIX `C` locale.
const C_MONTHS_ABBREVIATED: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// The weekday names of the POSIX `C` locale, in ISO order.
const C_WEEKDAYS_WIDE: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

/// The abbreviated weekday names of the POSIX `C` locale, in ISO order.
const C_WEEKDAYS_ABBREVIATED: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

/// The narrow month initials of the POSIX `C` locale.
const C_MONTHS_NARROW: [&str; 12] = ["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"];

/// The narrow weekday initials, in ISO order.
const C_WEEKDAYS_NARROW: [&str; 7] = ["M", "T", "W", "T", "F", "S", "S"];

/// The day periods of the POSIX `C` locale.
const C_DAY_PERIODS: [&str; 2] = ["AM", "PM"];

/// The era names of the POSIX `C` locale, abbreviated then wide.
const C_ERAS_ABBREVIATED: [&str; 2] = ["BC", "AD"];

/// The wide era names.
const C_ERAS_WIDE: [&str; 2] = ["Before Christ", "Anno Domini"];

/// The quarter names of the POSIX `C` locale, abbreviated then wide.
const C_QUARTERS_ABBREVIATED: [&str; 4] = ["Q1", "Q2", "Q3", "Q4"];

/// The wide quarter names.
const C_QUARTERS_WIDE: [&str; 4] = ["1st quarter", "2nd quarter", "3rd quarter", "4th quarter"];

/// Everything a pattern might want to write, plus the vocabulary to write it
/// in.
///
/// A zone abbreviation and a zone name are carried separately from the
/// offset because they cannot be derived from it: `+09:00` is `JST` in Tokyo
/// and `KST` in Seoul, and only the zone that produced the reading knows
/// which. Supply them from [`hc_tz::TimeZone::abbreviation_at`].
#[derive(Debug, Clone, Copy)]
pub struct FormatContext<'a> {
    /// The local civil reading to format.
    pub date_time: CivilDateTime,
    /// What is known about the zone.
    pub zone: ZoneInfo,
    /// The short zone name, such as `JST`, for `%Z` and CLDR `z`.
    pub zone_abbreviation: Option<&'a str>,
    /// The long zone name, such as `Japan Standard Time`, for CLDR `zzzz`.
    pub zone_name: Option<&'a str>,
    /// The locale whose vocabulary to use, or `None` for POSIX `C`.
    pub locale: Option<&'a Locale>,
}

impl<'a> FormatContext<'a> {
    /// A context for a local reading with no zone and no locale.
    #[must_use]
    pub const fn new(date_time: CivilDateTime) -> Self {
        Self {
            date_time,
            zone: ZoneInfo::Unspecified,
            zone_abbreviation: None,
            zone_name: None,
            locale: None,
        }
    }

    /// The same context, with a zone designator.
    #[must_use]
    pub const fn with_zone(mut self, zone: ZoneInfo) -> Self {
        self.zone = zone;
        self
    }

    /// The same context, with a short zone name.
    #[must_use]
    pub const fn with_zone_abbreviation(mut self, abbreviation: &'a str) -> Self {
        self.zone_abbreviation = Some(abbreviation);
        self
    }

    /// The same context, with a long zone name.
    #[must_use]
    pub const fn with_zone_name(mut self, name: &'a str) -> Self {
        self.zone_name = Some(name);
        self
    }

    /// The same context, with a locale to take names from.
    #[must_use]
    pub const fn with_locale(mut self, locale: &'a Locale) -> Self {
        self.locale = Some(locale);
        self
    }

    /// The derived calendar and clock fields.
    ///
    /// # Errors
    ///
    /// [`ValueError::Calendar`] when the day is outside the Gregorian range.
    pub fn fields(&self) -> ValueResult<Fields> {
        Fields::from_civil(self.date_time)
    }
}

/// Every field a pattern can ask for, worked out once.
///
/// Deriving these repeatedly inside a formatting loop would mean running the
/// Gregorian and ISO-week conversions once per `%` escape, so they are done
/// once and handed round.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Fields {
    /// The astronomical Gregorian year.
    pub year: i64,
    /// The month, 1 through 12.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
    /// The day of the year, counting from 1.
    pub day_of_year: u16,
    /// The weekday.
    pub weekday: Weekday,
    /// The ISO week-numbering year.
    pub iso_year: i64,
    /// The ISO week, 1 through 53.
    pub iso_week: u8,
    /// The hour, 0 through 23.
    pub hour: u8,
    /// The minute.
    pub minute: u8,
    /// The second, possibly 60.
    pub second: u8,
    /// The sub-second remainder in attoseconds.
    pub subsec_attos: u64,
}

impl Fields {
    /// Derive every field from a civil reading.
    ///
    /// # Errors
    ///
    /// [`ValueError::Calendar`] when the day is outside the Gregorian range.
    pub fn from_civil(date_time: CivilDateTime) -> ValueResult<Self> {
        let (year, month, day) = gregorian::from_fixed(date_time.day)?;
        let (iso_year, iso_week, _) = iso_week::from_fixed(date_time.day)?;
        Ok(Self {
            year,
            month,
            day,
            day_of_year: gregorian::day_of_year(year, month, day)?,
            weekday: Weekday::from_rd(date_time.day),
            iso_year,
            iso_week,
            hour: date_time.time.hour(),
            minute: date_time.time.minute(),
            second: date_time.time.second(),
            subsec_attos: date_time.time.subsec_attos(),
        })
    }

    /// The quarter, 1 through 4.
    #[must_use]
    pub const fn quarter(&self) -> u8 {
        (self.month - 1) / 3 + 1
    }

    /// The hour on a 12-hour clock, 1 through 12.
    #[must_use]
    pub const fn hour12(&self) -> u8 {
        match self.hour % 12 {
            0 => 12,
            other => other,
        }
    }

    /// Which day period the hour falls in.
    #[must_use]
    pub const fn day_period(&self) -> DayPeriod {
        DayPeriod::from_hour(self.hour)
    }

    /// The week of the year counting from the first Sunday, `strftime`'s
    /// `%U`. Days before that Sunday are in week 0.
    #[must_use]
    pub const fn week_of_year_sunday(&self) -> u8 {
        let day_index = self.day_of_year as u32 - 1;
        let weekday = self.weekday.sunday_first_number() as u32;
        ((day_index + 7 - weekday) / 7) as u8
    }

    /// The week of the year counting from the first Monday, `strftime`'s
    /// `%W`.
    #[must_use]
    pub const fn week_of_year_monday(&self) -> u8 {
        let day_index = self.day_of_year as u32 - 1;
        let weekday = self.weekday.iso_number() as u32 - 1;
        ((day_index + 7 - weekday) / 7) as u8
    }

    /// The week of the month, CLDR's `W`, counting from 1.
    ///
    /// This uses the ISO rule — weeks begin on Monday and week 1 is the one
    /// holding the 4th — rather than a locale-dependent one.
    #[must_use]
    pub const fn week_of_month(&self) -> u8 {
        let day_index = self.day as u32 - 1;
        let weekday = self.weekday.iso_number() as u32 - 1;
        ((day_index + 7 - weekday) / 7) as u8 + 1
    }

    /// Milliseconds elapsed since midnight, CLDR's `A`.
    #[must_use]
    pub const fn millis_in_day(&self) -> u32 {
        (self.hour as u32 * 3_600 + self.minute as u32 * 60 + self.second as u32) * 1_000
            + (self.subsec_attos / 1_000_000_000_000_000) as u32
    }

    /// The era index: 0 for years at or before zero, 1 after.
    ///
    /// ISO 8601 counts year 0 as 1 BC, so the boundary is `year < 1`.
    #[must_use]
    pub const fn era_index(&self) -> usize {
        if self.year < 1 { 0 } else { 1 }
    }

    /// The year within its era, which is what `G`-bearing patterns pair with.
    #[must_use]
    pub const fn era_year(&self) -> i64 {
        if self.year < 1 {
            1 - self.year
        } else {
            self.year
        }
    }
}

// --- vocabulary ------------------------------------------------------------

/// The month name, from the locale when there is one.
pub(crate) fn month_name(
    locale: Option<&Locale>,
    month: u8,
    width: NameWidth,
    context: NameContext,
) -> &'static str {
    let index = usize::from(month).saturating_sub(1);
    if let Some(locale) = locale
        && let Some(name) = names::month_name(
            locale,
            CALENDAR,
            hc_calendar::Month::regular(month),
            width,
            context,
        )
    {
        return name;
    }
    let table = match width {
        NameWidth::Wide => &C_MONTHS_WIDE[..],
        NameWidth::Narrow => &C_MONTHS_NARROW[..],
        _ => &C_MONTHS_ABBREVIATED[..],
    };
    table.get(index).copied().unwrap_or("")
}

/// The weekday name, from the locale when there is one.
pub(crate) fn weekday_name(
    locale: Option<&Locale>,
    weekday: Weekday,
    width: NameWidth,
    context: NameContext,
) -> &'static str {
    if let Some(locale) = locale
        && let Some(name) = names::weekday_name(locale, weekday, width, context)
    {
        return name;
    }
    let index = usize::from(weekday.iso_number()) - 1;
    let table = match width {
        NameWidth::Wide => &C_WEEKDAYS_WIDE[..],
        NameWidth::Narrow => &C_WEEKDAYS_NARROW[..],
        _ => &C_WEEKDAYS_ABBREVIATED[..],
    };
    table.get(index).copied().unwrap_or("")
}

/// The day-period name, from the locale when there is one.
pub(crate) fn day_period_name(
    locale: Option<&Locale>,
    period: DayPeriod,
    width: NameWidth,
) -> &'static str {
    if let Some(locale) = locale
        && let Some(name) = names::day_period_name(locale, period, width, NameContext::Format)
    {
        return name;
    }
    let index = usize::from(matches!(period, DayPeriod::Pm));
    C_DAY_PERIODS.get(index).copied().unwrap_or("")
}

/// The era name, from the locale when there is one.
pub(crate) fn era_name(locale: Option<&Locale>, index: usize, width: NameWidth) -> &'static str {
    if let Some(locale) = locale
        && let Some(name) = names::era_name(locale, CALENDAR, index, width)
    {
        return name;
    }
    let table = match width {
        NameWidth::Wide => &C_ERAS_WIDE[..],
        _ => &C_ERAS_ABBREVIATED[..],
    };
    table.get(index).copied().unwrap_or("")
}

/// The quarter name, from the locale when there is one.
pub(crate) fn quarter_name(
    locale: Option<&Locale>,
    quarter: u8,
    width: NameWidth,
    context: NameContext,
) -> &'static str {
    if let Some(locale) = locale
        && let Some(name) = names::quarter_name(locale, CALENDAR, quarter, width, context)
    {
        return name;
    }
    let index = usize::from(quarter).saturating_sub(1);
    let table = match width {
        NameWidth::Wide => &C_QUARTERS_WIDE[..],
        _ => &C_QUARTERS_ABBREVIATED[..],
    };
    table.get(index).copied().unwrap_or("")
}

/// Every candidate spelling of a month, for a parser to match against.
///
/// Both the locale's names and the `C` ones are offered: input written by
/// one program and read by another rarely agrees about which was in force.
pub(crate) fn month_candidates(locale: Option<&Locale>, month: u8) -> [&'static str; 6] {
    [
        month_name(locale, month, NameWidth::Wide, NameContext::Format),
        month_name(locale, month, NameWidth::Abbreviated, NameContext::Format),
        month_name(locale, month, NameWidth::Wide, NameContext::Standalone),
        month_name(
            locale,
            month,
            NameWidth::Abbreviated,
            NameContext::Standalone,
        ),
        C_MONTHS_WIDE[usize::from(month) - 1],
        C_MONTHS_ABBREVIATED[usize::from(month) - 1],
    ]
}

/// Every candidate spelling of a weekday, for a parser to match against.
pub(crate) fn weekday_candidates(locale: Option<&Locale>, weekday: Weekday) -> [&'static str; 6] {
    let index = usize::from(weekday.iso_number()) - 1;
    [
        weekday_name(locale, weekday, NameWidth::Wide, NameContext::Format),
        weekday_name(locale, weekday, NameWidth::Abbreviated, NameContext::Format),
        weekday_name(locale, weekday, NameWidth::Wide, NameContext::Standalone),
        weekday_name(
            locale,
            weekday,
            NameWidth::Abbreviated,
            NameContext::Standalone,
        ),
        C_WEEKDAYS_WIDE[index],
        C_WEEKDAYS_ABBREVIATED[index],
    ]
}

/// Every candidate spelling of a day period, for a parser to match against.
pub(crate) fn day_period_candidates(
    locale: Option<&Locale>,
    period: DayPeriod,
) -> [&'static str; 4] {
    let index = usize::from(matches!(period, DayPeriod::Pm));
    [
        day_period_name(locale, period, NameWidth::Wide),
        day_period_name(locale, period, NameWidth::Abbreviated),
        day_period_name(locale, period, NameWidth::Narrow),
        C_DAY_PERIODS[index],
    ]
}

/// Every candidate spelling of an era, for a parser to match against.
pub(crate) fn era_candidates(locale: Option<&Locale>, index: usize) -> [&'static str; 4] {
    [
        era_name(locale, index, NameWidth::Wide),
        era_name(locale, index, NameWidth::Abbreviated),
        C_ERAS_WIDE[index],
        C_ERAS_ABBREVIATED[index],
    ]
}

/// What a pattern-driven parse found.
///
/// The fields are kept as they were read rather than resolved on the spot,
/// because a pattern may name a date three different ways — calendar,
/// ordinal or week — and which one wins is a decision that needs the whole
/// set. [`ParsedFields::to_offset_date_time`] makes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct ParsedFields {
    /// A full year, from `%Y`, `yyyy` or `u`.
    pub year: Option<i64>,
    /// A century, from `%C`.
    pub century: Option<i64>,
    /// A year within a century, from `%y` or `yy`.
    pub year_of_century: Option<i64>,
    /// The month.
    pub month: Option<u8>,
    /// The day of the month.
    pub day: Option<u8>,
    /// The day of the year, from `%j` or `D`.
    pub day_of_year: Option<u16>,
    /// The ISO week-numbering year, from `%G` or `Y`.
    pub iso_year: Option<i64>,
    /// The ISO week, from `%V` or `w`.
    pub iso_week: Option<u8>,
    /// The ISO weekday, from `%u` or `e`.
    pub iso_weekday: Option<u8>,
    /// The hour on a 24-hour clock.
    pub hour: Option<u8>,
    /// The hour on a 12-hour clock, which needs a day period to be useful.
    pub hour12: Option<u8>,
    /// The day period, from `%p` or `a`.
    pub day_period: Option<DayPeriod>,
    /// The minute.
    pub minute: Option<u8>,
    /// The second.
    pub second: Option<u8>,
    /// The sub-second remainder in attoseconds.
    pub subsec_attos: Option<u64>,
    /// What the zone field said.
    pub zone: Option<ZoneInfo>,
    /// A POSIX timestamp, from `%s`, which names an instant on its own.
    pub unix_seconds: Option<i64>,
    /// The era index, from `%` nothing or CLDR `G`: 0 before year 1, 1 after.
    pub era: Option<usize>,
    /// The week of the year counted from the first Sunday, from `%U`.
    pub week_of_year_sunday: Option<u8>,
    /// The week of the year counted from the first Monday, from `%W`.
    pub week_of_year_monday: Option<u8>,
}

impl ParsedFields {
    /// Resolve the fields into a reading plus whatever zone was stated.
    ///
    /// The date is taken from the first of these that is complete: a POSIX
    /// timestamp, an ISO week date, an ordinal date, a `%U` or `%W` week
    /// number with a weekday and a year, a calendar date.
    ///
    /// # Errors
    ///
    /// [`ValueError::MissingField`] when no complete date is present, and
    /// [`ValueError::Calendar`] when the fields do not name a real day.
    pub fn to_offset_date_time(&self) -> ValueResult<crate::OffsetDateTime> {
        if let Some(seconds) = self.unix_seconds {
            // `%s` is defined as seconds since the POSIX epoch in UTC, so an
            // absent zone means UTC here rather than "unknown".
            let zone = self.zone.unwrap_or(ZoneInfo::Zulu);
            let unix = hc_core::UnixTime::new(seconds, self.subsec_attos.unwrap_or(0))?;
            return crate::OffsetDateTime::from_unix(unix, zone);
        }
        let day = self.resolve_day()?;
        let time = self.resolve_time()?;
        Ok(crate::OffsetDateTime {
            local: CivilDateTime::new(day, time),
            zone: self.zone.unwrap_or(ZoneInfo::Unspecified),
            written_as_end_of_day: false,
        })
    }

    fn resolved_year(&self) -> Option<i64> {
        if let Some(year) = self.year {
            return Some(match self.era {
                // Year 1 BC is astronomical year 0.
                Some(0) => 1 - year,
                _ => year,
            });
        }
        let within = self.year_of_century?;
        if let Some(century) = self.century {
            return Some(century * 100 + within);
        }
        // POSIX strptime: a bare two-digit year 69-99 is 1969-1999 and 0-68
        // is 2000-2068.
        Some(if within >= 69 {
            1_900 + within
        } else {
            2_000 + within
        })
    }

    fn resolve_day(&self) -> ValueResult<hc_calendar::Rd> {
        if let (Some(year), Some(week), Some(weekday)) =
            (self.iso_year, self.iso_week, self.iso_weekday)
        {
            return Ok(iso_week::to_fixed(year, week, weekday)?);
        }
        let year = self
            .resolved_year()
            .ok_or(ValueError::MissingField("year"))?;
        if let Some(day_of_year) = self.day_of_year {
            return Ok(hc_calendars_solar::ordinal::to_fixed(year, day_of_year)?);
        }
        if let Some(day) = self.resolve_week_of_year(year)? {
            return Ok(day);
        }
        let month = self.month.ok_or(ValueError::MissingField("month"))?;
        let day = self.day.ok_or(ValueError::MissingField("day"))?;
        Ok(gregorian::to_fixed(year, month, day)?)
    }

    /// The day named by a `%U` or `%W` week number and a weekday.
    ///
    /// Week 1 starts on the year's first Sunday (`%U`) or Monday (`%W`), and
    /// the days before it are week 0, so week 0 may reach back into the
    /// previous year — which is what Python's `strptime` computes too.
    fn resolve_week_of_year(&self, year: i64) -> ValueResult<Option<hc_calendar::Rd>> {
        let Some(iso_weekday) = self.iso_weekday else {
            return Ok(None);
        };
        let (week, first_weekday) = match (self.week_of_year_sunday, self.week_of_year_monday) {
            (Some(week), _) => (week, Weekday::Sunday),
            (None, Some(week)) => (week, Weekday::Monday),
            (None, None) => return Ok(None),
        };
        let weekday = Weekday::from_iso_number(iso_weekday).ok_or(ValueError::Calendar(
            hc_calendar::CalendarError::DayOutOfRange,
        ))?;
        let week_one = first_weekday.on_or_after(gregorian::to_fixed(year, 1, 1)?);
        let into_week = match first_weekday {
            Weekday::Sunday => weekday.sunday_first_number(),
            _ => weekday.monday_first_number(),
        };
        let offset = 7 * (i64::from(week) - 1) + i64::from(into_week);
        Ok(Some(week_one.checked_add_days(offset)?))
    }

    /// The same fields with a date filled in wherever the text named none,
    /// which is how Python's `strptime` resolves a pattern such as `%H:%M`:
    /// against 1900-01-01.
    ///
    /// Only a field that nothing else determines is filled: a year when no
    /// year, century or ISO year was read, a month and a day when no day of
    /// the year was, and neither when an ISO week date or a POSIX timestamp
    /// is complete.
    #[must_use]
    pub const fn with_default_date(mut self, year: i64, month: u8, day: u8) -> Self {
        if self.unix_seconds.is_some()
            || (self.iso_year.is_some() && self.iso_week.is_some() && self.iso_weekday.is_some())
        {
            return self;
        }
        if self.year.is_none() && self.year_of_century.is_none() && self.century.is_none() {
            self.year = Some(year);
        }
        if self.day_of_year.is_none() {
            if self.month.is_none() {
                self.month = Some(month);
            }
            if self.day.is_none() {
                self.day = Some(day);
            }
        }
        self
    }

    fn resolve_time(&self) -> ValueResult<hc_calendar::CivilTime> {
        let hour = match (self.hour, self.hour12, self.day_period) {
            (Some(hour), _, _) => hour,
            (None, Some(hour12), period) => {
                let base = hour12 % 12;
                if matches!(period, Some(DayPeriod::Pm)) {
                    base + 12
                } else {
                    base
                }
            }
            (None, None, _) => 0,
        };
        Ok(hc_calendar::CivilTime::new(
            hour,
            self.minute.unwrap_or(0),
            self.second.unwrap_or(0),
            self.subsec_attos.unwrap_or(0),
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::{CivilTime, Rd};

    fn fields(rd: i64, hour: u8) -> Fields {
        Fields::from_civil(CivilDateTime::new(
            Rd(rd),
            CivilTime::hms(hour, 0, 0).unwrap(),
        ))
        .unwrap()
    }

    #[test]
    fn the_derived_fields_agree_with_the_calendars() {
        let value = fields(739_880, 14);
        assert_eq!((value.year, value.month, value.day), (2026, 9, 21));
        assert_eq!(value.day_of_year, 264);
        assert_eq!(value.weekday, Weekday::Monday);
        assert_eq!((value.iso_year, value.iso_week), (2026, 39));
        assert_eq!(value.quarter(), 3);
    }

    #[test]
    fn the_twelve_hour_clock_calls_midnight_and_noon_twelve() {
        assert_eq!(fields(739_880, 0).hour12(), 12);
        assert_eq!(fields(739_880, 12).hour12(), 12);
        assert_eq!(fields(739_880, 13).hour12(), 1);
        assert!(matches!(fields(739_880, 0).day_period(), DayPeriod::Am));
        assert!(matches!(fields(739_880, 12).day_period(), DayPeriod::Pm));
    }

    #[test]
    fn the_sunday_and_monday_week_numbers_differ_where_they_should() {
        // 1 January 2026 is a Thursday, so it is in week 0 of both.
        let new_year = fields(739_617, 0);
        assert_eq!(new_year.week_of_year_sunday(), 0);
        assert_eq!(new_year.week_of_year_monday(), 0);
        // 4 January 2026 is a Sunday: week 1 by %U, still week 0 by %W.
        let sunday = fields(739_620, 0);
        assert_eq!(sunday.week_of_year_sunday(), 1);
        assert_eq!(sunday.week_of_year_monday(), 0);
    }

    #[test]
    fn the_era_boundary_is_at_astronomical_year_one() {
        let bc = Fields::from_civil(CivilDateTime::midnight(
            gregorian::to_fixed(0, 1, 1).unwrap(),
        ))
        .unwrap();
        assert_eq!(bc.era_index(), 0);
        assert_eq!(bc.era_year(), 1);
    }

    #[test]
    fn the_c_locale_names_are_english_where_no_locale_is_given() {
        assert_eq!(
            month_name(None, 9, NameWidth::Wide, NameContext::Format),
            "September"
        );
        assert_eq!(
            weekday_name(
                None,
                Weekday::Monday,
                NameWidth::Abbreviated,
                NameContext::Format
            ),
            "Mon"
        );
        assert_eq!(day_period_name(None, DayPeriod::Pm, NameWidth::Wide), "PM");
    }

    #[test]
    fn a_locale_supplies_its_own_names() {
        let japanese = Locale::parse("ja").unwrap();
        assert_eq!(
            month_name(Some(&japanese), 9, NameWidth::Wide, NameContext::Format),
            "9月"
        );
    }

    #[test]
    fn parsed_fields_prefer_the_week_date_when_one_is_complete() {
        let parsed = ParsedFields {
            iso_year: Some(2026),
            iso_week: Some(38),
            iso_weekday: Some(1),
            year: Some(1999),
            month: Some(1),
            day: Some(1),
            ..ParsedFields::default()
        };
        assert_eq!(parsed.to_offset_date_time().unwrap().local.day, Rd(739_873));
    }

    #[test]
    fn a_two_digit_year_follows_the_posix_pivot() {
        let make = |within: i64| ParsedFields {
            year_of_century: Some(within),
            month: Some(1),
            day: Some(1),
            ..ParsedFields::default()
        };
        assert_eq!(make(68).resolved_year(), Some(2_068));
        assert_eq!(make(69).resolved_year(), Some(1_969));
    }

    #[test]
    fn a_twelve_hour_reading_needs_its_day_period() {
        let parsed = ParsedFields {
            year: Some(2026),
            month: Some(9),
            day: Some(21),
            hour12: Some(2),
            day_period: Some(DayPeriod::Pm),
            ..ParsedFields::default()
        };
        assert_eq!(parsed.to_offset_date_time().unwrap().local.time.hour(), 14);
    }

    #[test]
    fn fields_with_no_date_at_all_name_what_is_missing() {
        assert_eq!(
            ParsedFields::default().to_offset_date_time().unwrap_err(),
            ValueError::MissingField("year")
        );
    }
}
