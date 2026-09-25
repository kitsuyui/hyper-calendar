//! The Thai lunar calendar, ปฏิทินจันทรคติไทย, as Thailand publishes it.
//!
//! The system is written up in `docs/systems/thai-lunar.md` in the
//! repository: what the calendar is, how a year type fixes every month and
//! how each year's type was read off the published holy days, with a
//! worked example, what is carried and why J. C. Eade's *suriyayatra* rule
//! is not, and how the table was checked against the Bank of Thailand's
//! lists. This page summarises it and states the code's own facts.
//!
//! Twelve months, เดือนอ้าย to เดือนสิบสอง, of 29 days when the month's
//! number is odd and 30 when it is even: a normal year (ปกติมาส) of 354
//! days. In an *adhikamāsa* year (ปีอธิกมาส) month 8 is doubled, the extra
//! month — เดือน 8 หนแรก, the first of the two — coming between month 7 and
//! the regular month 8, both of them 30 days, for 384. In an *adhikavāra*
//! year (ปีอธิกวาร) month 7 has a 30th day, แรม 15 ค่ำ เดือน 7, for 355. A
//! year is never both. The days of a month are counted in two halves:
//! waxing, ขึ้น 1 to 15 ค่ำ, and waning, แรม 1 to 14 ค่ำ, or to 15 in a
//! 30-day month.
//!
//! # Carried as data, not as a rule
//!
//! Which years take the extra month and which the extra day is not
//! computed here: the published calendar is not known to follow the
//! *suriyayatra* rule as Eade states it, and the document says how far
//! that was tested. The type is carried, year by year, in [`YEAR_TYPES`],
//! each read off the Makha, Visakha and Asalha Bucha (to 2006 Khao Phansa)
//! dates Thailand published for it, which fix it, and every year the table
//! does not reach is refused. [`makha_bucha`], [`visakha_bucha`],
//! [`asalha_bucha`] and [`khao_phansa`] give the holy days.
//!
//! # Sources, per span
//!
//! * **2535–2549 BE (1992–2006)**: the Bank of Thailand's lists of
//!   financial-institution holidays for each year, as archived by the
//!   Internet Archive at
//!   `bot.or.th/Thai/FinancialInstitutions/FIholiday/Pages/<year>.aspx`,
//!   retrieved 2026-09-23. These years list Makha Bucha, Visakha Bucha and
//!   Khao Phansa; Asalha Bucha is the day before Khao Phansa. The list for
//!   1996 gives Thursday 29 February, the Thursday before, as the day in
//!   place of a Makha Bucha that fell on Sunday 3 March.
//! * **2550–2565 BE (2007–2022)**: the same pages, which from 2007 list
//!   Asalha Bucha instead of Khao Phansa "on the advice of the Office of
//!   National Buddhism".
//! * **2566–2569 BE (2023–2026)**: the Bank's notifications FPG 3/2565,
//!   FPG 8/2566, FPG 5/2567 and 31/2568, bot.or.th, retrieved 2026-09-23.
//! * **2570 BE (2027)**: Notification of the Bank of Thailand 37/2569,
//!   Royal Gazette vol. 143, special part 202 ง, 25 August 2026, pp. 21–22,
//!   ratchakitcha.soc.go.th, retrieved 2026-09-23.
//!
//! The structure — odd months of 29 days and even of 30, the doubled month
//! 8 and the 30th of month 7 — is as the *Dictionary of Buddhism*
//! (พจนานุกรมพุทธศาสน์ ฉบับประมวลศัพท์) states it under อธิกมาส, and as Thai
//! Wikipedia, "ปฏิทินจันทรคติไทย", retrieved 2026-09-23, describes the
//! official calendar, with the month names. The same sources are keyed in
//! `docs/references.bib`.
//!
//! # The range
//!
//! From ขึ้น 1 ค่ำ เดือนอ้าย of 2535 BE, 7 December 1991, to the end of
//! month 6 of 2571 BE, 23 May 2028. The year after the table's last is
//! carried through its sixth month because nothing a year type changes
//! comes before month 7; its month 7 onward, and whether it has a doubled
//! month 8, are unknown until Thailand publishes the year, and are refused
//! with [`CalendarError::AfterSupportedRange`].
//!
//! # How the year is numbered
//!
//! A year here runs from ขึ้น 1 ค่ำ เดือนอ้าย, which falls between mid
//! November and mid December, to the end of month 12 a year later, and is
//! numbered by the Buddhist Era of the Gregorian year its Makha Bucha falls
//! in: the Gregorian year plus 543. The number therefore changes some weeks
//! before the civil Buddhist Era does on 1 January. That is this module's
//! convention, chosen so that a year is one run of months: a Thai document
//! dates a December day by the civil year it falls in, and the older
//! reckonings changed the year at month 5 or at Songkran.
//!
//! # How a date is written in [`DateFields`]
//!
//! The day is counted straight through the month, 1 to 30: days 1 to 15
//! are ขึ้น 1 to 15 ค่ำ, days 16 to 29 or 30 are แรม 1 to 14 or 15 ค่ำ, so
//! แรม 15 ค่ำ is day 30 and exists only in a 30-day month. The half and
//! the day within it are also given as the extra fields `waning` (0 or 1)
//! and `fortnight-day`. The doubled month follows the Hindu and Burmese
//! calendars here: the first month 8, the extra one, is `Month::leap(8)`,
//! and the second, in which Asalha Bucha falls, is `Month::regular(8)`.

use core::fmt;

use hc_calendar::fields::ExtraFields;
use hc_calendar::shape::{CycleLength, CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};

/// The Buddhist Era year minus the Gregorian year it mostly falls in.
pub const BUDDHIST_ERA_OFFSET: i64 = 543;

/// The first year of the table, in the Buddhist Era.
pub const FIRST_YEAR: i64 = 2535;

/// The last year of the table, in the Buddhist Era.
pub const LAST_YEAR: i64 = 2570;

/// How many months of the year after [`LAST_YEAR`] are carried: every
/// month before the first one a year type can lengthen.
pub const MONTHS_KNOWN_AFTER_LAST_YEAR: u8 = 6;

/// The first Gregorian year every day of which the calendar converts.
pub const FIRST_GREGORIAN_YEAR: i64 = FIRST_YEAR - BUDDHIST_ERA_OFFSET;

/// The last Gregorian year every day of which the calendar converts.
pub const LAST_GREGORIAN_YEAR: i64 = LAST_YEAR - BUDDHIST_ERA_OFFSET;

/// The twelve months in Thai, เดือนอ้าย first. Source: Thai Wikipedia,
/// "ปฏิทินจันทรคติไทย", retrieved 2026-09-23.
pub const MONTHS: [&str; 12] = [
    "เดือนอ้าย",
    "เดือนยี่",
    "เดือนสาม",
    "เดือนสี่",
    "เดือนห้า",
    "เดือนหก",
    "เดือนเจ็ด",
    "เดือนแปด",
    "เดือนเก้า",
    "เดือนสิบ",
    "เดือนสิบเอ็ด",
    "เดือนสิบสอง",
];

/// Twelve named months, thirteen in an adhikamāsa year, and the week.
static SHAPE: &[CycleShape] = &[
    CycleShape {
        kind: MONTH,
        length: CycleLength::Intercalary {
            ordinary: 12,
            extended: 13,
        },
        names: &MONTHS,
    },
    CycleShape::fixed(WEEKDAY, 7),
];

/// The three kinds of year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YearType {
    /// ปกติมาส ปกติวาร: twelve months, 354 days.
    Normal,
    /// อธิกวาร: a 30th day in month 7, 355 days.
    ExtraDay,
    /// อธิกมาส: month 8 twice, 384 days.
    ExtraMonth,
}

impl YearType {
    /// The days in a year of this type.
    #[must_use]
    pub const fn days(self) -> i64 {
        match self {
            Self::Normal => 354,
            Self::ExtraDay => 355,
            Self::ExtraMonth => 384,
        }
    }

    /// Whether the year has the doubled month 8.
    #[must_use]
    pub const fn has_extra_month(self) -> bool {
        matches!(self, Self::ExtraMonth)
    }

    /// The Thai name of the year type.
    #[must_use]
    pub const fn thai_name(self) -> &'static str {
        match self {
            Self::Normal => "ปกติมาส",
            Self::ExtraDay => "อธิกวาร",
            Self::ExtraMonth => "อธิกมาส",
        }
    }

    /// The length of `month` in a year of this type, or `None` when the
    /// year has no such month.
    #[must_use]
    pub const fn month_length(self, month: Month) -> Option<u8> {
        if month.ordinal == 0 || month.ordinal > 12 {
            return None;
        }
        if month.leap {
            return if month.ordinal == 8 && self.has_extra_month() {
                Some(30)
            } else {
                None
            };
        }
        if month.ordinal == 7 && matches!(self, Self::ExtraDay) {
            return Some(30);
        }
        Some(30 - month.ordinal % 2)
    }
}

use YearType::{ExtraDay as V, ExtraMonth as M, Normal as N};

/// The year types from [`FIRST_YEAR`] to [`LAST_YEAR`], one a year, each
/// read off the Makha, Visakha and Asalha Bucha (or Khao Phansa) dates
/// Thailand published for it; the module documentation says how.
pub static YEAR_TYPES: [YearType; (LAST_YEAR - FIRST_YEAR + 1) as usize] = [
    // 2535–2549 BE, 1992–2006: the Bank of Thailand's yearly lists of
    // financial-institution holidays (Makha, Visakha, Khao Phansa), as
    // archived by the Internet Archive.
    N, M, N, N, M, V, N, M, V, N, M, N, M, V, N,
    // 2550–2565 BE, 2007–2022: the same lists (Makha, Visakha, Asalha).
    M, N, V, M, N, M, N, N, M, V, N, M, N, V, M, N,
    // 2566–2569 BE, 2023–2026: the Bank's notifications FPG 3/2565,
    // FPG 8/2566, FPG 5/2567 and 31/2568.
    M, N, V, M,
    // 2570 BE, 2027: Bank of Thailand notification 37/2569, Royal Gazette
    // vol. 143, special part 202 ง, 25 August 2026.
    N,
];

/// ขึ้น 1 ค่ำ เดือนอ้าย of [`FIRST_YEAR`]: 7 December 1991, 73 days before
/// the Makha Bucha of 18 February 1992.
const EPOCH: Rd = Rd(727_173);

/// The type of `year`, or `None` outside the table.
#[must_use]
pub fn year_type(year: i64) -> Option<YearType> {
    if !(FIRST_YEAR..=LAST_YEAR).contains(&year) {
        return None;
    }
    usize::try_from(year - FIRST_YEAR)
        .ok()
        .and_then(|index| YEAR_TYPES.get(index).copied())
}

/// The fixed day of ขึ้น 1 ค่ำ เดือนอ้าย of `year`, for every year of the
/// table and the one after it, or `None` otherwise.
#[must_use]
pub fn new_year(year: i64) -> Option<Rd> {
    if !(FIRST_YEAR..=LAST_YEAR + 1).contains(&year) {
        return None;
    }
    let mut start = EPOCH.0;
    for kind in &YEAR_TYPES[..usize::try_from(year - FIRST_YEAR).ok()?] {
        start += kind.days();
    }
    Some(Rd(start))
}

/// The last day the lunar calendar was Thailand's official reckoning,
/// 31 March 1889: the civil reckoning was solar from the next day.
pub const LAST_CIVIL: Rd = match hc_calendars_solar::gregorian::to_fixed(1889, 3, 31) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Official dates lunar, in the Chulasakarat era, until the solar reckoning from 1 April 1889 \
    [proclamation-new-day-rs107]; printed on every Thai wall calendar beside the civil date \
    and the calendar of the holy days since, as docs/systems/thai-lunar.md states; no source \
    read dates its beginning";

/// The earliest fixed day converted: ขึ้น 1 ค่ำ เดือนอ้าย of [`FIRST_YEAR`].
#[must_use]
pub const fn earliest() -> Rd {
    EPOCH
}

/// The latest fixed day converted: the last day of month
/// [`MONTHS_KNOWN_AFTER_LAST_YEAR`] of the year after [`LAST_YEAR`].
#[must_use]
pub fn latest() -> Rd {
    let start = new_year(LAST_YEAR + 1).map_or(EPOCH.0, |rd| rd.0);
    let known: i64 = (1..=MONTHS_KNOWN_AFTER_LAST_YEAR)
        .map(|ordinal| i64::from(30 - ordinal % 2))
        .sum();
    Rd(start + known - 1)
}

/// The months of a year of `kind` in order, with their lengths: month 1 to
/// 7, the extra month 8 if any, and the regular month 8 to 12.
fn months_of(kind: YearType) -> impl Iterator<Item = (Month, u8)> {
    (1..=12u8).flat_map(move |ordinal| {
        let extra = (ordinal == 8 && kind.has_extra_month()).then_some((Month::leap(8), 30));
        let regular = kind
            .month_length(Month::regular(ordinal))
            .map(|length| (Month::regular(ordinal), length));
        extra.into_iter().chain(regular)
    })
}

/// The number of days in `month` of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside the years carried,
/// [`CalendarError::MonthOutOfRange`] for a month the year does not have,
/// and [`CalendarError::AfterSupportedRange`] for a month of the year after
/// [`LAST_YEAR`] that its unpublished type could change.
pub fn month_length(year: i64, month: Month) -> CalendarResult<u8> {
    if let Some(kind) = year_type(year) {
        return kind
            .month_length(month)
            .ok_or(CalendarError::MonthOutOfRange);
    }
    if year != LAST_YEAR + 1 {
        return Err(CalendarError::YearOutOfRange);
    }
    if month.ordinal == 0 || month.ordinal > 12 {
        return Err(CalendarError::MonthOutOfRange);
    }
    if month.leap || month.ordinal > MONTHS_KNOWN_AFTER_LAST_YEAR {
        return Err(CalendarError::AfterSupportedRange);
    }
    YearType::Normal
        .month_length(month)
        .ok_or(CalendarError::MonthOutOfRange)
}

/// The half of the month a day falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fortnight {
    /// ข้างขึ้น, the waxing half: ขึ้น 1 to 15 ค่ำ.
    Waxing,
    /// ข้างแรม, the waning half: แรม 1 to 14 or 15 ค่ำ.
    Waning,
}

/// A Thai lunar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThaiLunarDate {
    /// The year, in the Buddhist Era, numbered as the module documentation
    /// says.
    pub year: i64,
    /// The month, 1 for เดือนอ้าย through 12; the first month 8 of an
    /// adhikamāsa year is `Month::leap(8)`.
    pub month: Month,
    /// The day of the month, 1 to 30, counted straight through: 16 is
    /// แรม 1 ค่ำ and 30 is แรม 15 ค่ำ.
    pub day: u8,
}

impl ThaiLunarDate {
    /// A date from its year, month and day.
    #[must_use]
    pub const fn new(year: i64, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }

    /// The half of the month.
    #[must_use]
    pub const fn fortnight(&self) -> Fortnight {
        if self.day <= 15 {
            Fortnight::Waxing
        } else {
            Fortnight::Waning
        }
    }

    /// The day within its half, the number before ค่ำ: 1 to 15.
    #[must_use]
    pub const fn fortnight_day(&self) -> u8 {
        if self.day <= 15 {
            self.day
        } else {
            self.day - 15
        }
    }

    /// The month's name in Thai.
    #[must_use]
    pub const fn month_name(&self) -> &'static str {
        MONTHS[(self.month.ordinal as usize).saturating_sub(1) % 12]
    }
}

impl fmt::Display for ThaiLunarDate {
    /// Writes the date as a Thai calendar does, with the year after it:
    /// `ขึ้น 15 ค่ำ เดือน 8 หลัง, 2569 BE`, the second month 8 of an
    /// adhikamāsa year being เดือน 8 หลัง and the first เดือน 8 แรก.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let half = match self.fortnight() {
            Fortnight::Waxing => "ขึ้น",
            Fortnight::Waning => "แรม",
        };
        write!(
            f,
            "{half} {} ค่ำ เดือน {}",
            self.fortnight_day(),
            self.month.ordinal
        )?;
        if self.month.leap {
            write!(f, " แรก")?;
        } else if self.month.ordinal == 8
            && year_type(self.year).is_some_and(YearType::has_extra_month)
        {
            write!(f, " หลัง")?;
        }
        write!(f, ", {} BE", self.year)
    }
}

/// The Thai lunar date of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`earliest`] to
/// [`latest`].
pub fn from_fixed(rd: Rd) -> CalendarResult<ThaiLunarDate> {
    if rd < earliest() {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > latest() {
        return Err(CalendarError::AfterSupportedRange);
    }
    let mut year = FIRST_YEAR;
    let mut start = EPOCH.0;
    while let Some(kind) = year_type(year) {
        if rd.0 < start + kind.days() {
            break;
        }
        start += kind.days();
        year += 1;
    }
    // Past the table only the first months of the next year are in range,
    // and they are the same in every type of year.
    let kind = year_type(year).unwrap_or(YearType::Normal);
    let mut offset = rd.0 - start;
    for (month, length) in months_of(kind) {
        if offset < i64::from(length) {
            let day = u8::try_from(offset + 1).map_err(|_| CalendarError::DayOutOfRange)?;
            return Ok(ThaiLunarDate { year, month, day });
        }
        offset -= i64::from(length);
    }
    Err(CalendarError::AfterSupportedRange)
}

/// The fixed day of a Thai lunar date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside the years carried,
/// [`CalendarError::MonthOutOfRange`] for a month the year does not have,
/// [`CalendarError::DayOutOfRange`] for a day the month does not have, and
/// [`CalendarError::AfterSupportedRange`] for a day of the year after
/// [`LAST_YEAR`] beyond its sixth month.
pub fn to_fixed(date: ThaiLunarDate) -> CalendarResult<Rd> {
    let length = month_length(date.year, date.month)?;
    if date.day == 0 || date.day > length {
        return Err(CalendarError::DayOutOfRange);
    }
    let start = new_year(date.year).ok_or(CalendarError::YearOutOfRange)?;
    let kind = year_type(date.year).unwrap_or(YearType::Normal);
    let before: i64 = months_of(kind)
        .take_while(|(month, _)| *month != date.month)
        .map(|(_, length)| i64::from(length))
        .sum();
    Ok(Rd(start.0 + before + i64::from(date.day) - 1))
}

/// The full moon, ขึ้น 15 ค่ำ, of an ordinary month of `year`.
fn full_moon(year: i64, ordinal: u8) -> CalendarResult<Rd> {
    to_fixed(ThaiLunarDate::new(year, Month::regular(ordinal), 15))
}

/// The type of `year`, for a day that depends on it.
fn known_type(year: i64) -> CalendarResult<YearType> {
    year_type(year).ok_or(if year == LAST_YEAR + 1 {
        CalendarError::AfterSupportedRange
    } else {
        CalendarError::YearOutOfRange
    })
}

/// Makha Bucha, วันมาฆบูชา: the full moon of month 3, or of month 4 in an
/// adhikamāsa year.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside the table and
/// [`CalendarError::AfterSupportedRange`] for the year after it, whose type
/// decides the month and is not yet published.
pub fn makha_bucha(year: i64) -> CalendarResult<Rd> {
    let kind = known_type(year)?;
    full_moon(year, if kind.has_extra_month() { 4 } else { 3 })
}

/// Visakha Bucha, วันวิสาขบูชา: the full moon of month 6, or of month 7 in
/// an adhikamāsa year.
///
/// # Errors
///
/// As [`makha_bucha`].
pub fn visakha_bucha(year: i64) -> CalendarResult<Rd> {
    let kind = known_type(year)?;
    full_moon(year, if kind.has_extra_month() { 7 } else { 6 })
}

/// Asalha Bucha, วันอาสาฬหบูชา: the full moon of month 8, the regular one
/// in an adhikamāsa year.
///
/// # Errors
///
/// As [`makha_bucha`].
pub fn asalha_bucha(year: i64) -> CalendarResult<Rd> {
    known_type(year)?;
    full_moon(year, 8)
}

/// Khao Phansa, วันเข้าพรรษา: แรม 1 ค่ำ of month 8, the day after Asalha
/// Bucha.
///
/// # Errors
///
/// As [`makha_bucha`].
pub fn khao_phansa(year: i64) -> CalendarResult<Rd> {
    known_type(year)?;
    to_fixed(ThaiLunarDate::new(year, Month::regular(8), 16))
}

/// The Thai lunar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ThaiLunarCalendar;

impl Calendar for ThaiLunarCalendar {
    type Date = ThaiLunarDate;

    /// Thailand's official reckoning until 31 March 1889 and the calendar of
    /// its holy days since, undated at the start; the year types carried run
    /// from 2535 BE, which bounds the range and not the use.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(USAGE_SOURCE).civil_until(LAST_CIVIL)
    }

    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// An อธิกมาส year, with its doubled month 8, or an อธิกวาร year,
    /// with its thirtieth day of month 7: Thailand names both as
    /// intercalations, and the table records which a year was.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        match year_type(year) {
            Some(kind) => Ok(!matches!(kind, YearType::Normal)),
            None if year == LAST_YEAR + 1 => Err(CalendarError::AfterSupportedRange),
            None => Err(CalendarError::YearOutOfRange),
        }
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("thai-lunar"),
            english_name: "Thai lunar",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: false,
            earliest: Some(earliest()),
            latest: Some(latest()),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        from_fixed(rd)
    }

    /// The year, month and day, with the half of the month and the day
    /// within it as the extra fields `waning` and `fortnight-day`.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        to_fixed(date)?;
        let mut extra = ExtraFields::new();
        extra.set("waning", i64::from(date.fortnight() == Fortnight::Waning))?;
        extra.set("fortnight-day", i64::from(date.fortnight_day()))?;
        Ok(DateFields {
            era: None,
            year: date.year,
            month: Some(date.month),
            day: Some(date.day),
            leap_day: false,
            extra,
        })
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let date = ThaiLunarDate::new(fields.year, fields.require_month()?, fields.require_day()?);
        to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;

    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    /// How a published date was given.
    #[derive(Clone, Copy)]
    enum Given {
        /// The day itself.
        On(u8, u8),
        /// Only the Monday — once the Thursday before — that stood in for
        /// a holiday on a weekend day, which the table must put on that
        /// weekend.
        InPlaceOf(u8, u8),
    }
    use Given::{InPlaceOf, On};

    /// The published days, Gregorian year first: Makha Bucha, Visakha
    /// Bucha, and Khao Phansa to 2006 or Asalha Bucha from 2007, as the
    /// module documentation's sources give them.
    const PUBLISHED: &[(i64, Given, Given, Given)] = &[
        (1992, On(2, 18), InPlaceOf(5, 18), On(7, 15)),
        (1993, InPlaceOf(3, 8), On(6, 4), On(8, 3)),
        (1994, On(2, 25), On(5, 24), InPlaceOf(7, 25)),
        (1995, On(2, 14), InPlaceOf(5, 15), On(7, 12)),
        (1996, InPlaceOf(2, 29), On(5, 31), On(7, 30)),
        (1997, On(2, 21), On(5, 20), InPlaceOf(7, 21)),
        (1998, On(2, 11), InPlaceOf(5, 11), On(7, 9)),
        (1999, On(3, 1), InPlaceOf(5, 31), On(7, 28)),
        (2000, InPlaceOf(2, 21), On(5, 17), On(7, 17)),
        (2001, On(2, 8), On(5, 7), On(7, 6)),
        (2002, On(2, 26), InPlaceOf(5, 27), On(7, 25)),
        (2003, InPlaceOf(2, 17), On(5, 15), On(7, 14)),
        (2004, On(3, 5), On(6, 2), On(8, 1)),
        (2005, On(2, 23), On(5, 22), On(7, 22)),
        (2006, On(2, 13), On(5, 12), On(7, 11)),
        (2007, On(3, 3), On(5, 31), On(7, 29)),
        (2008, On(2, 21), On(5, 19), On(7, 17)),
        (2009, On(2, 9), On(5, 8), On(7, 7)),
        (2010, On(2, 28), On(5, 28), On(7, 26)),
        (2011, On(2, 18), On(5, 17), On(7, 15)),
        (2012, On(3, 7), On(6, 4), On(8, 2)),
        (2013, On(2, 25), On(5, 24), On(7, 22)),
        (2014, On(2, 14), On(5, 13), On(7, 11)),
        (2015, On(3, 4), On(6, 1), On(7, 30)),
        (2016, On(2, 22), On(5, 20), On(7, 19)),
        (2017, On(2, 11), On(5, 10), On(7, 8)),
        (2018, On(3, 1), On(5, 29), On(7, 27)),
        (2019, On(2, 19), On(5, 18), On(7, 16)),
        (2020, On(2, 8), On(5, 6), On(7, 5)),
        (2021, On(2, 26), On(5, 26), On(7, 24)),
        (2022, On(2, 16), On(5, 15), On(7, 13)),
        (2023, On(3, 6), On(6, 3), On(8, 1)),
        (2024, On(2, 24), On(5, 22), On(7, 20)),
        (2025, On(2, 12), On(5, 11), On(7, 10)),
        (2026, On(3, 3), On(5, 31), On(7, 29)),
        (2027, On(2, 21), On(5, 20), On(7, 18)),
    ];

    fn check(year: i64, given: Given, actual: Rd, what: &str) {
        match given {
            On(month, day) => assert_eq!(actual, greg(year, month, day), "{year} {what}"),
            InPlaceOf(month, day) => {
                let stand_in = greg(year, month, day);
                let weekday = actual.0.rem_euclid(7);
                // Rd 0 is a Sunday: 0 Sunday, 6 Saturday.
                assert!(weekday == 0 || weekday == 6, "{year} {what} on a weekday");
                assert!(
                    (actual.0 - stand_in.0).abs() <= 4,
                    "{year} {what}: {actual} is not the weekend {stand_in} stood in for"
                );
            }
        }
    }

    #[test]
    fn every_published_holiday_is_reproduced() {
        assert_eq!(PUBLISHED.len(), YEAR_TYPES.len());
        for &(year, makha, visakha, third) in PUBLISHED {
            let be = year + BUDDHIST_ERA_OFFSET;
            check(
                year,
                makha,
                makha_bucha(be).expect("in range"),
                "Makha Bucha",
            );
            check(
                year,
                visakha,
                visakha_bucha(be).expect("in range"),
                "Visakha Bucha",
            );
            if year <= 2006 {
                check(
                    year,
                    third,
                    khao_phansa(be).expect("in range"),
                    "Khao Phansa",
                );
            } else {
                check(
                    year,
                    third,
                    asalha_bucha(be).expect("in range"),
                    "Asalha Bucha",
                );
            }
            assert_eq!(
                khao_phansa(be).expect("in range").0,
                asalha_bucha(be).expect("in range").0 + 1
            );
        }
    }

    #[test]
    fn the_weekend_days_resolve_as_the_neighbouring_years_require() {
        // 1996's Makha Bucha was a Sunday, whichever weekday stood in.
        assert_eq!(makha_bucha(2539), Ok(greg(1996, 3, 3)));
        assert_eq!(visakha_bucha(2535), Ok(greg(1992, 5, 16)));
        assert_eq!(khao_phansa(2537), Ok(greg(1994, 7, 23)));
        assert_eq!(khao_phansa(2540), Ok(greg(1997, 7, 20)));
        assert_eq!(makha_bucha(2543), Ok(greg(2000, 2, 19)));
        assert_eq!(visakha_bucha(2545), Ok(greg(2002, 5, 26)));
        assert_eq!(makha_bucha(2546), Ok(greg(2003, 2, 16)));
    }

    #[test]
    fn the_adhikamasa_year_2569_doubles_month_8() {
        assert_eq!(year_type(2569), Some(YearType::ExtraMonth));
        assert_eq!(new_year(2569), Some(greg(2025, 11, 21)));
        assert_eq!(new_year(2570), Some(greg(2026, 12, 10)));
        assert_eq!(new_year(2570).unwrap().0 - new_year(2569).unwrap().0, 384);
        // The first month 8 is the extra one, and Asalha Bucha falls in
        // the second.
        let first_eighth = to_fixed(ThaiLunarDate::new(2569, Month::leap(8), 1)).unwrap();
        let second_eighth = to_fixed(ThaiLunarDate::new(2569, Month::regular(8), 1)).unwrap();
        assert_eq!(second_eighth.0 - first_eighth.0, 30);
        let asalha = from_fixed(greg(2026, 7, 29)).unwrap();
        assert_eq!(asalha, ThaiLunarDate::new(2569, Month::regular(8), 15));
        assert_eq!(asalha.to_string(), "ขึ้น 15 ค่ำ เดือน 8 หลัง, 2569 BE");
        let first = from_fixed(greg(2026, 6, 30)).unwrap();
        assert_eq!(first.month, Month::leap(8));
        assert_eq!(first.to_string(), "แรม 1 ค่ำ เดือน 8 แรก, 2569 BE");
        assert_eq!(month_length(2569, Month::leap(8)), Ok(30));
        assert_eq!(month_length(2569, Month::regular(8)), Ok(30));
        assert_eq!(month_length(2569, Month::regular(7)), Ok(29));
    }

    #[test]
    fn month_lengths_follow_the_year_type() {
        for year in FIRST_YEAR..=LAST_YEAR {
            let kind = year_type(year).unwrap();
            let total: i64 = months_of(kind).map(|(_, length)| i64::from(length)).sum();
            assert_eq!(total, kind.days(), "{year}");
            assert_eq!(
                new_year(year + 1).unwrap().0 - new_year(year).unwrap().0,
                kind.days(),
                "{year}"
            );
            for ordinal in 1..=12u8 {
                let expected = match (kind, ordinal) {
                    (YearType::ExtraDay, 7) => 30,
                    _ => 30 - ordinal % 2,
                };
                assert_eq!(
                    month_length(year, Month::regular(ordinal)),
                    Ok(expected),
                    "{year} month {ordinal}"
                );
            }
            assert_eq!(
                month_length(year, Month::leap(8)).is_ok(),
                kind.has_extra_month()
            );
        }
        // แรม 15 ค่ำ เดือน 7 exists only in an adhikavāra year: 2568 BE is
        // one, 2567 is not.
        assert_eq!(year_type(2568), Some(YearType::ExtraDay));
        assert!(to_fixed(ThaiLunarDate::new(2568, Month::regular(7), 30)).is_ok());
        assert_eq!(
            to_fixed(ThaiLunarDate::new(2567, Month::regular(7), 30)),
            Err(CalendarError::DayOutOfRange)
        );
        // Thirteen adhikamāsa and seven adhikavāra years in thirty-six.
        let count = |wanted| YEAR_TYPES.iter().filter(|kind| **kind == wanted).count();
        assert_eq!(count(YearType::ExtraMonth), 13);
        assert_eq!(count(YearType::ExtraDay), 7);
    }

    #[test]
    fn every_day_of_the_range_round_trips() {
        let calendar = ThaiLunarCalendar;
        let mut previous: Option<ThaiLunarDate> = None;
        for rd in earliest().0..=latest().0 {
            let date = calendar.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{date}");
            let fields = calendar.to_fields(date).expect("describable");
            assert_eq!(calendar.from_fields(&fields), Ok(date), "{date}");
            assert_eq!(
                fields.extra.get("fortnight-day"),
                Some(i64::from(date.fortnight_day()))
            );
            match previous {
                Some(before) if before.month == date.month && before.year == date.year => {
                    assert_eq!(date.day, before.day + 1, "{date}");
                }
                Some(_) => assert_eq!(date.day, 1, "{date}"),
                None => assert_eq!(date, ThaiLunarDate::new(FIRST_YEAR, Month::regular(1), 1)),
            }
            previous = Some(date);
        }
    }

    #[test]
    fn the_range_ends_where_the_table_does() {
        assert_eq!(earliest(), greg(1991, 12, 7));
        assert_eq!(latest(), greg(2028, 5, 23));
        assert_eq!(new_year(LAST_YEAR + 1), Some(greg(2027, 11, 29)));
        assert_eq!(
            from_fixed(Rd(earliest().0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(latest().0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        let last = from_fixed(latest()).unwrap();
        assert_eq!(last, ThaiLunarDate::new(2571, Month::regular(6), 30));
        // Month 7 of 2571 BE, and its possible doubled month 8, wait on a
        // year type Thailand has not published.
        for month in [Month::regular(7), Month::leap(8), Month::regular(8)] {
            assert_eq!(
                to_fixed(ThaiLunarDate::new(2571, month, 1)),
                Err(CalendarError::AfterSupportedRange)
            );
        }
        assert_eq!(makha_bucha(2571), Err(CalendarError::AfterSupportedRange));
        assert_eq!(makha_bucha(2534), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            to_fixed(ThaiLunarDate::new(2572, Month::regular(1), 1)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(ThaiLunarDate::new(2534, Month::regular(12), 30)),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn impossible_dates_are_refused() {
        // 2567 BE has one month 8.
        assert_eq!(
            to_fixed(ThaiLunarDate::new(2567, Month::leap(8), 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(ThaiLunarDate::new(2569, Month::leap(7), 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(ThaiLunarDate::new(2567, Month::regular(13), 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(ThaiLunarDate::new(2567, Month::regular(3), 30)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(ThaiLunarDate::new(2567, Month::regular(4), 0)),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn a_date_reads_as_a_thai_calendar_writes_it() {
        // Makha Bucha 2567 BE, 24 February 2024.
        let makha = from_fixed(greg(2024, 2, 24)).unwrap();
        assert_eq!(makha, ThaiLunarDate::new(2567, Month::regular(3), 15));
        assert_eq!(makha.to_string(), "ขึ้น 15 ค่ำ เดือน 3, 2567 BE");
        assert_eq!(makha.month_name(), "เดือนสาม");
        // Khao Phansa 2567 BE, the day after Asalha Bucha on 20 July 2024.
        let phansa = from_fixed(greg(2024, 7, 21)).unwrap();
        assert_eq!(phansa.fortnight(), Fortnight::Waning);
        assert_eq!(phansa.to_string(), "แรม 1 ค่ำ เดือน 8, 2567 BE");
        // The year number changes at เดือนอ้าย, before 1 January.
        let december = from_fixed(greg(2023, 12, 13)).unwrap();
        assert_eq!(december, ThaiLunarDate::new(2567, Month::regular(1), 1));
        assert_eq!(from_fixed(greg(2023, 12, 12)).unwrap().year, 2566);
    }
}
