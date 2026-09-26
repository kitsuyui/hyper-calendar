//! The Khmer lunisolar calendar, ចន្ទគតិ (*Chhankitek*), as Cambodia keeps it.
//!
//! The system is written up in `docs/systems/khmer-chhankitek.md` in the
//! repository: the months and the three kinds of year, the rules that
//! decide a year's type with the Buddhist year 2568 worked by hand, where
//! the Buddhist Era changes, what is carried and not — Lao, Sinhalese and
//! Tai among it — and how the calendar was checked against Cambodia's
//! published dates. This page summarises it and states the code's facts.
//!
//! Twelve months, មិគសិរ (Migasir) to កត្តិក (Kadeuk), of 29 days when the
//! month's number is odd and 30 when it is even: a normal year of 354 days.
//! A leap-month year doubles អាសាឍ (Asath), month 8, as បឋមាសាឍ and
//! ទុតិយាសាឍ, for 384 days; a leap-day year gives ជេស្ឋ (Jesth), month 7, a
//! 30th day, for 355; a year is never both. The days are counted 1 to 15
//! កើត (waxing) and 1 to 14 or 15 រោច (waning). The layout is the Thai one
//! and lives in [`southeast_asian`](crate::southeast_asian).
//!
//! # The rule
//!
//! The year's type is computed, as Phylypo Tum's "Khmer Chhankitek
//! Calendar" gives the Cambodian *hora*'s rules from Roath Kim Soeun's
//! almanac: a leap month when the solar New Year's lunar day, the
//! *bodithey*, is 25 or more or 5 or less, with a 24-then-6 and a
//! 25-then-5 exception; a leap day when the New Year's avoman is 137 or
//! less, or 126 or less in a 366-day solar year, with a 137-then-0
//! exception; and a day that falls in a leap-month year moved to the next
//! year. The quantities are Gislén and Eade's *suryayatra* ones in the
//! Chulasakarat era, the Buddhist year less [`CHULASAKARAT_OFFSET`]. See
//! [`has_leap_month`], [`has_leap_day_by_rule`] and [`year_type`].
//!
//! # The range
//!
//! The years 2444 to 2744 as the rule numbers them, from 1 keit Migasir,
//! 3 December 1899 — Tum's epoch is 1 keit Bos, 1 January 1900 — to the end
//! of Kadeuk, 6 December 2200. The three readings of the rule compared agree
//! on every year of it and part outside it; every other day is refused.
//!
//! # How the year is numbered
//!
//! A year here is one run of months, Migasir to Kadeuk, numbered as the rule
//! numbers it: by the Buddhist Era Cambodia prints from 1 roaj Pisakh, the
//! day after Visak Bochea, to the end of the year. From 1 keit Migasir to
//! 15 keit Pisakh the printed Buddhist year is one less, which
//! [`KhmerDate::printed_year`] gives and the date's display writes. The
//! animal year, the *sak* and the Jolak Sakaraj, which change at the solar
//! New Year in April, are not carried.
//!
//! # How a date is written in [`DateFields`]
//!
//! The day is counted straight through the month, 1 to 30: days 1 to 15 are
//! 1 to 15 កើត, days 16 to 29 or 30 are 1 to 14 or 15 រោច. The half, the
//! day within it and the printed Buddhist year are the extra fields
//! `waning` (0 or 1), `fortnight-day` and `printed-year`. The first Asath of
//! a leap-month year, the extra one, is `Month::leap(8)`, and ទុតិយាសាឍ is
//! `Month::regular(8)`, as in `thai-lunar`.
//!
//! Sources: Phylypo Tum, "Khmer Chhankitek Calendar", cam-cc.org/calendar,
//! the Internet Archive's copies of 2008–2012, retrieved 2026-09-26, for the
//! rules, the layout and the epoch; L. Gislén and C. J. Eade, "The Calendars
//! of Southeast Asia. 2", *JAHH* 22(3), 2019, for the quantities; Khmer
//! Wikipedia, "ចន្ទគតិ", retrieved 2026-09-26, for the month names; the New
//! Year announcements of 2022–2026 as Fresh News printed them for the
//! change of the Buddhist Era. Keyed in `docs/references.bib` as
//! `tum-chhankitek`, `gisleneade2019`, `wikipedia-km-chankitek` and
//! `freshnews-songkran-2022` to `freshnews-songkran-2026`.

use core::fmt;

use hc_calendar::fields::ExtraFields;
use hc_calendar::shape::{CycleLength, CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd, YearKind,
};

use crate::southeast_asian::{
    Fortnight, YearType, Years, avoman, is_solar_leap_year, new_year_tithi,
};

/// The Buddhist Era year minus the Gregorian year its Visak Bochea falls in.
pub const BUDDHIST_ERA_OFFSET: i64 = 544;

/// The Buddhist Era year minus the Chulasakarat year, in which Gislén and
/// Eade state the quantities of the New Year.
pub const CHULASAKARAT_OFFSET: i64 = 1182;

/// The first year carried, as the rule numbers it: 1900 CE.
pub const FIRST_YEAR: i64 = 2444;

/// The last year carried, as the rule numbers it: 2200 CE.
pub const LAST_YEAR: i64 = 2744;

/// The twelve months in Khmer, មិគសិរ first. Source: Khmer Wikipedia,
/// "ចន្ទគតិ", retrieved 2026-09-26; the romanisations live in `hc-i18n`.
pub const MONTHS: [&str; 12] = [
    "មិគសិរ",
    "បុស្ស",
    "មាឃ",
    "ផល្គុន",
    "ចេត្រ",
    "ពិសាខ",
    "ជេស្ឋ",
    "អាសាឍ",
    "ស្រាពណ៍",
    "ភទ្របទ",
    "អស្សុជ",
    "កត្តិក",
];

/// The first Asath of a leap-month year, the extra month, `Month::leap(8)`.
pub const FIRST_ASATH: &str = "បឋមាសាឍ";

/// The second Asath of a leap-month year, `Month::regular(8)`.
pub const SECOND_ASATH: &str = "ទុតិយាសាឍ";

/// Twelve named months, thirteen in a leap-month year, and the week.
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

/// The Chulasakarat year of the solar New Year the rule reads for `year`.
const fn chulasakarat(year: i64) -> i64 {
    year - CHULASAKARAT_OFFSET
}

/// Whether the rule gives `year` the second Asath: the lunar day of its
/// solar New Year, the *bodithey*, is 25 or more or 5 or less, except that
/// a year of 24 followed by one of 6 has it and a year of 25 followed by
/// one of 5 does not.
#[must_use]
pub const fn has_leap_month(year: i64) -> bool {
    let this = new_year_tithi(chulasakarat(year));
    let next = new_year_tithi(chulasakarat(year) + 1);
    if this == 25 && next == 5 {
        return false;
    }
    if this == 24 && next == 6 {
        return true;
    }
    this >= 25 || this <= 5
}

/// Whether the rule calls for a 30th day of Jesth in `year`, before a
/// collision with the leap month is settled: the avoman of its solar New
/// Year is 126 or less in a 366-day solar year and 137 or less in a
/// 365-day one, except that a year of 137 followed by one of 0 does not.
#[must_use]
pub const fn has_leap_day_by_rule(year: i64) -> bool {
    let solar = chulasakarat(year);
    let excess = avoman(solar);
    if is_solar_leap_year(solar) {
        excess <= 126
    } else if excess == 137 && avoman(solar + 1) == 0 {
        false
    } else {
        excess <= 137
    }
}

/// The type the rule gives `year`, in or out of the range carried: the
/// leap month if it is called for; otherwise the leap day if it is called
/// for, or if the year before was called for both; otherwise normal.
#[must_use]
pub const fn year_type_by_rule(year: i64) -> YearType {
    if has_leap_month(year) {
        YearType::ExtraMonth
    } else if has_leap_day_by_rule(year)
        || (has_leap_month(year - 1) && has_leap_day_by_rule(year - 1))
    {
        YearType::ExtraDay
    } else {
        YearType::Normal
    }
}

/// The type of `year`, or `None` outside [`FIRST_YEAR`] to [`LAST_YEAR`].
#[must_use]
pub fn year_type(year: i64) -> Option<YearType> {
    (FIRST_YEAR..=LAST_YEAR)
        .contains(&year)
        .then(|| year_type_by_rule(year))
}

/// 1 keit Migasir of [`FIRST_YEAR`], 3 December 1899: 29 days before Tum's
/// epoch, 1 keit Bos on 1 January 1900.
const EPOCH: Rd = match hc_calendars_solar::gregorian::to_fixed(1899, 12, 3) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The years carried, laid out from [`EPOCH`].
static YEARS: Years = Years {
    first: FIRST_YEAR,
    last: LAST_YEAR,
    epoch: EPOCH,
    months_after_last: 0,
    year_type,
};

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Cambodia's calendar for religious purposes beside the Gregorian one for civil \
    purposes [tum-chhankitek]; the Royal Government's holiday sub-decrees date Visak Bochea, \
    Pchum Ben and the Water Festival on it and the New Year announcements give each day's \
    lunar date [ibc-kh-holidays-2024-2025, freshnews-songkran-2025], as \
    docs/systems/khmer-chhankitek.md states; no source read dates its beginning";

/// The earliest fixed day converted: 1 keit Migasir of [`FIRST_YEAR`].
#[must_use]
pub const fn earliest() -> Rd {
    EPOCH
}

/// The latest fixed day converted: the last day of Kadeuk of
/// [`LAST_YEAR`].
#[must_use]
pub fn latest() -> Rd {
    YEARS.latest()
}

/// 1 keit Migasir of `year`, or `None` outside the years carried.
#[must_use]
pub fn new_year(year: i64) -> Option<Rd> {
    YEARS.kind(year)?;
    YEARS.new_year(year)
}

/// The number of days in `month` of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`](hc_calendar::CalendarError::YearOutOfRange) outside the years carried and
/// [`CalendarError::MonthOutOfRange`](hc_calendar::CalendarError::MonthOutOfRange) for a month the year does not have.
pub fn month_length(year: i64, month: Month) -> CalendarResult<u8> {
    YEARS.month_length(year, month)
}

/// A Khmer lunar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KhmerDate {
    /// The year as the rule numbers it: the run of months from Migasir to
    /// Kadeuk whose Pisakh falls in the Gregorian year less
    /// [`BUDDHIST_ERA_OFFSET`].
    pub year: i64,
    /// The month, 1 for Migasir through 12 for Kadeuk; the first Asath of a
    /// leap-month year is `Month::leap(8)`.
    pub month: Month,
    /// The day of the month, 1 to 30, counted straight through: 16 is
    /// 1 roaj and 30 is 15 roaj.
    pub day: u8,
}

impl KhmerDate {
    /// A date from its year, month and day.
    #[must_use]
    pub const fn new(year: i64, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }

    /// The half of the month.
    #[must_use]
    pub const fn fortnight(&self) -> Fortnight {
        Fortnight::of(self.day)
    }

    /// The day within its half, the number before កើត or រោច: 1 to 15.
    #[must_use]
    pub const fn fortnight_day(&self) -> u8 {
        Fortnight::day_within(self.day)
    }

    /// The Buddhist Era year Cambodia prints for the date: [`KhmerDate::year`]
    /// from 1 roaj Pisakh on, and one less from 1 keit Migasir to 15 keit
    /// Pisakh.
    #[must_use]
    pub const fn printed_year(&self) -> i64 {
        if self.month.ordinal < 6 || (self.month.ordinal == 6 && self.day <= 15) {
            self.year - 1
        } else {
            self.year
        }
    }

    /// The month's name in Khmer: បឋមាសាឍ and ទុតិយាសាឍ for the two Asath
    /// of a leap-month year.
    #[must_use]
    pub fn month_name(&self) -> &'static str {
        if self.month.leap {
            return FIRST_ASATH;
        }
        if self.month.ordinal == 8 && year_type(self.year).is_some_and(YearType::has_extra_month) {
            return SECOND_ASATH;
        }
        MONTHS[(self.month.ordinal as usize).saturating_sub(1) % 12]
    }
}

/// Writes `number` in Khmer digits, ០ to ៩.
fn khmer_digits(f: &mut fmt::Formatter<'_>, number: i64) -> fmt::Result {
    if number < 0 {
        write!(f, "-")?;
    }
    let mut digits = [0u8; 20];
    let mut count = 0;
    let mut rest = number.unsigned_abs();
    loop {
        digits[count] = (rest % 10) as u8;
        count += 1;
        rest /= 10;
        if rest == 0 {
            break;
        }
    }
    for &digit in digits[..count].iter().rev() {
        let glyph = char::from_u32(0x17E0 + u32::from(digit)).unwrap_or('?');
        write!(f, "{glyph}")?;
    }
    Ok(())
}

impl fmt::Display for KhmerDate {
    /// Writes the date in Khmer digits with the printed Buddhist year after
    /// it, in the form the pages Tum checked print, «២កើត ខែអស្សុជ
    /// ព.ស.២៤៥៧»: `១៥កើត ខែពិសាខ ព.ស.២៥៦៨`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        khmer_digits(f, i64::from(self.fortnight_day()))?;
        let half = match self.fortnight() {
            Fortnight::Waxing => "កើត",
            Fortnight::Waning => "រោច",
        };
        write!(f, "{half} ខែ{} ព.ស.", self.month_name())?;
        khmer_digits(f, self.printed_year())
    }
}

/// The Khmer lunar date of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`](hc_calendar::CalendarError::BeforeEpoch) or
/// [`CalendarError::AfterSupportedRange`](hc_calendar::CalendarError::AfterSupportedRange) outside [`earliest`] to
/// [`latest`].
pub fn from_fixed(rd: Rd) -> CalendarResult<KhmerDate> {
    let (year, month, day) = YEARS.from_fixed(rd)?;
    Ok(KhmerDate { year, month, day })
}

/// The fixed day of a Khmer lunar date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`](hc_calendar::CalendarError::YearOutOfRange) outside the years carried,
/// [`CalendarError::MonthOutOfRange`](hc_calendar::CalendarError::MonthOutOfRange) for a month the year does not have
/// and [`CalendarError::DayOutOfRange`](hc_calendar::CalendarError::DayOutOfRange) for a day the month does not have.
pub fn to_fixed(date: KhmerDate) -> CalendarResult<Rd> {
    YEARS.to_fixed(date.year, date.month, date.day)
}

/// The Khmer lunar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KhmerCalendar;

impl Calendar for KhmerCalendar {
    type Date = KhmerDate;

    /// Cambodia's religious calendar today, undated at the start; the rule
    /// is carried from 1900, which bounds the range and not the use.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A អធិកមាស year, with its two Asath, or an អធិកវារៈ year, with its
    /// thirtieth day of Jesth.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        YEARS.is_leap_year(year)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("khmer"),
            english_name: "Khmer lunar",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: false,
            earliest: Some(earliest()),
            latest: Some(latest()),
            native_locales: &["km"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        from_fixed(rd)
    }

    /// The year, month and day, with the half of the month, the day within
    /// it and the printed Buddhist year as the extra fields `waning`,
    /// `fortnight-day` and `printed-year`.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        to_fixed(date)?;
        let mut extra = ExtraFields::new();
        extra.set("waning", i64::from(date.fortnight() == Fortnight::Waning))?;
        extra.set("fortnight-day", i64::from(date.fortnight_day()))?;
        extra.set("printed-year", date.printed_year())?;
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
        let date = KhmerDate::new(fields.year, fields.require_month()?, fields.require_day()?);
        to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;

    use hc_calendar::CalendarError;

    use super::*;
    use crate::southeast_asian::{ahargana, kammacabala, months_of};

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    /// Rd 0 is a Sunday: 0 Sunday to 6 Saturday.
    fn weekday(rd: Rd) -> i64 {
        rd.0.rem_euclid(7)
    }

    /// A date by its printed form: the printed Buddhist year, the month,
    /// waning or not, and the day within the half.
    fn printed(printed_year: i64, month: Month, waning: bool, day: u8) -> KhmerDate {
        let counted = if waning { day + 15 } else { day };
        let early = month.ordinal < 6 || (month.ordinal == 6 && !waning);
        let year = if early {
            printed_year + 1
        } else {
            printed_year
        };
        KhmerDate::new(year, month, counted)
    }

    const CHAET: Month = Month::regular(5);
    const PISAKH: Month = Month::regular(6);

    #[test]
    fn the_new_years_of_2022_to_2026_fall_on_the_announced_lunar_days() {
        // Each year's announcement, as Fresh News printed it: the Gregorian
        // day, its weekday, and its lunar date in Chaet. The printed year is
        // the Buddhist year before 1 roaj Pisakh.
        let announced: &[(i64, u8, u8, i64, bool, u8)] = &[
            // 2022: Thursday 13 keit, Friday 14 keit, Saturday 15 keit.
            (2022, 4, 14, 4, false, 13),
            (2022, 4, 15, 5, false, 14),
            (2022, 4, 16, 6, false, 15),
            // 2023: Friday 9 roaj, Saturday 10 roaj, Sunday 11 roaj.
            (2023, 4, 14, 5, true, 9),
            (2023, 4, 15, 6, true, 10),
            (2023, 4, 16, 0, true, 11),
            // 2024: Saturday 5 keit to Tuesday 8 keit, four days.
            (2024, 4, 13, 6, false, 5),
            (2024, 4, 14, 0, false, 6),
            (2024, 4, 15, 1, false, 7),
            (2024, 4, 16, 2, false, 8),
            // 2025: Monday 2 roaj, Tuesday 3 roaj, Wednesday 4 roaj.
            (2025, 4, 14, 1, true, 2),
            (2025, 4, 15, 2, true, 3),
            (2025, 4, 16, 3, true, 4),
            // 2026: Tuesday 12 roaj, Wednesday 13 roaj, Thursday 14 roaj
            // (Kampuchea Thmey too gives Tuesday, 12 roaj of Chaet).
            (2026, 4, 14, 2, true, 12),
            (2026, 4, 15, 3, true, 13),
            (2026, 4, 16, 4, true, 14),
        ];
        assert_eq!(announced.len(), 16);
        for &(year, month, day, week, waning, lunar) in announced {
            let rd = greg(year, month, day);
            assert_eq!(weekday(rd), week, "{year}-{month}-{day}");
            let expected = printed(year + BUDDHIST_ERA_OFFSET - 1, CHAET, waning, lunar);
            assert_eq!(from_fixed(rd), Ok(expected), "{year}-{month}-{day}");
        }
    }

    #[test]
    fn the_buddhist_era_changes_at_the_full_moon_of_pisakh() {
        // The announcements: the Buddhist Era was N − 1 "until" the named
        // weekday, 15 keit of Pisakh, and N from 1 roaj.
        for (year, full_moon, week) in [
            (2022, greg(2022, 5, 15), 0),
            (2023, greg(2023, 5, 4), 4),
            (2024, greg(2024, 5, 22), 3),
            (2025, greg(2025, 5, 11), 0),
            (2026, greg(2026, 5, 1), 5),
        ] {
            let be = year + BUDDHIST_ERA_OFFSET;
            assert_eq!(weekday(full_moon), week, "{year}");
            let last = from_fixed(full_moon).unwrap();
            assert_eq!(last, KhmerDate::new(be, PISAKH, 15), "{year}");
            assert_eq!(last.printed_year(), be - 1);
            let first = from_fixed(Rd(full_moon.0 + 1)).unwrap();
            assert_eq!(first.printed_year(), be);
        }
    }

    /// The day of `year` given by month, day 1 to 30.
    fn day_of(year: i64, month: Month, day: u8) -> Rd {
        to_fixed(KhmerDate::new(year + BUDDHIST_ERA_OFFSET, month, day)).expect("in range")
    }

    #[test]
    fn the_sub_decreed_days_of_2024_to_2027_are_reproduced() {
        // Visak Bochea, 15 keit Pisakh; the Royal Ploughing Ceremony,
        // 4 roaj Pisakh; the middle day of Pchum Ben, 15 roaj Photrobot; the
        // first of the Water Festival, 14 keit Kadeuk. 2024 and 2025 from
        // the International Business Chamber's lists of sub-decrees
        // No. 230 and No. 204, 2026 from Andersen's of No. 167, 2027 from
        // hc-holiday's reading of No. 198.
        for (year, visak, plough, pchum_ben, water) in [
            (2024, (5, 22), (5, 26), (10, 2), (11, 14)),
            (2025, (5, 11), (5, 15), (9, 22), (11, 4)),
            (2026, (5, 1), (5, 5), (10, 11), (11, 23)),
            (2027, (5, 20), (5, 24), (9, 30), (11, 12)),
        ] {
            let on = |(month, day): (u8, u8)| greg(year, month, day);
            assert_eq!(day_of(year, PISAKH, 15), on(visak), "{year}");
            assert_eq!(day_of(year, PISAKH, 19), on(plough), "{year}");
            assert_eq!(
                day_of(year, Month::regular(10), 30),
                on(pchum_ben),
                "{year}"
            );
            assert_eq!(day_of(year, Month::regular(12), 14), on(water), "{year}");
        }
    }

    #[test]
    fn the_holidays_of_2015_and_2019_are_reproduced() {
        // timeanddate.com's lists, as the Internet Archive kept them on
        // 20 September 2015 and 15 August 2020: Meak Bochea, Visak Bochea,
        // the middle day of Pchum Ben (the second of three in 2015 and of
        // four in 2019) and the first of the Water Festival.
        for (year, meak, visak, pchum_ben, water) in [
            (2015, (2, 3), (5, 2), (10, 12), (11, 24)),
            (2019, (2, 19), (5, 18), (9, 28), (11, 10)),
        ] {
            let on = |(month, day): (u8, u8)| greg(year, month, day);
            assert_eq!(day_of(year, Month::regular(3), 15), on(meak), "{year}");
            assert_eq!(day_of(year, PISAKH, 15), on(visak), "{year}");
            assert_eq!(
                day_of(year, Month::regular(10), 30),
                on(pchum_ben),
                "{year}"
            );
            assert_eq!(day_of(year, Month::regular(12), 14), on(water), "{year}");
        }
        // 2015 is the year the rarer rules show in: 2014 avoman 137 and
        // 2015 avoman 0; 2015 is called for the month and the day, keeps the
        // month, and the day goes to 2016.
        let cs = |year: i64| year + BUDDHIST_ERA_OFFSET - CHULASAKARAT_OFFSET;
        assert_eq!((avoman(cs(2014)), avoman(cs(2015))), (137, 0));
        assert_eq!(year_type(2558), Some(YearType::Normal));
        assert!(has_leap_month(2559) && has_leap_day_by_rule(2559));
        assert_eq!(year_type(2559), Some(YearType::ExtraMonth));
        assert_eq!(year_type(2560), Some(YearType::ExtraDay));
        assert_eq!(month_length(2560, Month::regular(7)), Ok(30));
    }

    /// A Gregorian year, month and day.
    type Ymd = (i64, u8, u8);

    #[test]
    fn tums_checked_dates_are_reproduced() {
        // Tum's "Data Verification": the Khmer and Gregorian dates of a life
        // or an event from Khmer Wikipedia and other pages, as printed Khmer
        // dates, with the weekday where the page gives one.
        let assoch = Month::regular(11);
        let photrobot = Month::regular(10);
        let checked: &[(Ymd, KhmerDate, Option<i64>)] = &[
            // Thursday 2 keit Assoch, BE 2457: 2 October 1913.
            ((1913, 10, 2), printed(2457, assoch, false, 2), Some(4)),
            // Thursday 13 roaj Bos, BE 2488: 11 January 1945, where the
            // page's year 1944 is a misprint by Tum's reading.
            (
                (1945, 1, 11),
                printed(2488, Month::regular(2), true, 13),
                Some(4),
            ),
            // 8 keit Kadeuk, BE 2491: 20 November 1947.
            (
                (1947, 11, 20),
                printed(2491, Month::regular(12), false, 8),
                None,
            ),
            // Monday 12 keit Meak, BE 2493: 18 February 1951. The lunar
            // date is what Tum matched; the day was a Sunday, and the
            // printed year by the change at Pisakh is 2494.
            (
                (1951, 2, 18),
                printed(2494, Month::regular(3), false, 12),
                None,
            ),
            // 12, 13 and 14 keit Photrobot, BE 2513: 23–25 September 1969.
            ((1969, 9, 23), printed(2513, photrobot, false, 12), None),
            ((1969, 9, 24), printed(2513, photrobot, false, 13), None),
            ((1969, 9, 25), printed(2513, photrobot, false, 14), None),
            // Monday 6 roaj Assoch, BE 2532: 31 October 1988, where the
            // page's 1987 is a misprint by Tum's reading.
            ((1988, 10, 31), printed(2532, assoch, true, 6), Some(1)),
            // 4 roaj Pisakh, the Royal Ploughing Ceremony: 26 May 2005.
            ((2005, 5, 26), printed(2549, PISAKH, true, 4), None),
            // Pchum Ben, 15 roaj Photrobot, BE 2552: 29 September 2008.
            ((2008, 9, 29), printed(2552, photrobot, true, 15), None),
        ];
        for &((year, month, day), expected, week) in checked {
            let rd = greg(year, month, day);
            assert_eq!(from_fixed(rd), Ok(expected), "{year}-{month}-{day}");
            if let Some(week) = week {
                assert_eq!(weekday(rd), week, "{year}-{month}-{day}");
            }
        }
        assert_eq!(from_fixed(greg(1951, 2, 18)).unwrap().printed_year(), 2494);
        assert_eq!(weekday(greg(1951, 2, 18)), 0);
    }

    #[test]
    fn tums_table_of_2000_to_2020_is_reproduced() {
        // Tum, "Khmer Calendar Algorithm": AD year, ahakun, avoman,
        // bodithey and the calendar's type; 2000's type is not given.
        use YearType::{ExtraDay as D, ExtraMonth as M, Normal as N};
        let table: &[(i64, i64, i64, i64, Option<YearType>)] = &[
            (2000, 929_222, 627, 11, None),
            (2001, 929_588, 501, 23, Some(N)),
            (2002, 929_953, 364, 4, Some(M)),
            (2003, 930_318, 227, 15, Some(N)),
            (2004, 930_683, 90, 26, Some(M)),
            (2005, 931_049, 656, 7, Some(D)),
            (2006, 931_414, 519, 18, Some(N)),
            (2007, 931_779, 382, 29, Some(M)),
            (2008, 932_144, 245, 10, Some(N)),
            (2009, 932_510, 119, 22, Some(D)),
            (2010, 932_875, 674, 2, Some(M)),
            (2011, 933_240, 537, 13, Some(N)),
            (2012, 933_605, 400, 24, Some(M)),
            (2013, 933_971, 274, 6, Some(N)),
            (2014, 934_336, 137, 17, Some(N)),
            (2015, 934_701, 0, 28, Some(M)),
            (2016, 935_067, 566, 9, Some(D)),
            (2017, 935_432, 429, 20, Some(N)),
            (2018, 935_797, 292, 1, Some(M)),
            (2019, 936_162, 155, 12, Some(N)),
            (2020, 936_528, 29, 24, Some(D)),
        ];
        for &(ad, ahakun, excess, bodithey, kind) in table {
            let be = ad + BUDDHIST_ERA_OFFSET;
            let cs = be - CHULASAKARAT_OFFSET;
            assert_eq!(ahargana(cs) + 431_739, ahakun, "{ad}");
            assert_eq!(avoman(cs), excess, "{ad}");
            assert_eq!(new_year_tithi(cs), bodithey, "{ad}");
            if let Some(kind) = kind {
                assert_eq!(year_type(be), Some(kind), "{ad}");
            }
        }
    }

    /// Tum's formulas in the Buddhist Era, as his page states them.
    fn tum_ahakun(be: i64) -> i64 {
        (be * 292_207 + 499).div_euclid(800) + 4
    }

    fn tum_avoman(be: i64) -> i64 {
        (tum_ahakun(be) * 11 + 25).rem_euclid(692)
    }

    fn tum_bodithey(be: i64) -> i64 {
        let ahakun = tum_ahakun(be);
        ((ahakun * 11 + 25).div_euclid(692) + ahakun + 29).rem_euclid(30)
    }

    /// momentkh's kromthupul, which Tum's page does not state, in the same
    /// Buddhist-Era form.
    fn momentkh_kromthupul(be: i64) -> i64 {
        800 - (be * 292_207 + 499).rem_euclid(800)
    }

    #[test]
    fn the_buddhist_era_form_is_the_chulasakarat_form() {
        for be in FIRST_YEAR - 1..=LAST_YEAR + 1 {
            let cs = be - CHULASAKARAT_OFFSET;
            assert_eq!(tum_ahakun(be), ahargana(cs) + 431_739, "{be}");
            assert_eq!(tum_avoman(be), avoman(cs), "{be}");
            assert_eq!(tum_bodithey(be), new_year_tithi(cs), "{be}");
            assert_eq!(momentkh_kromthupul(be), kammacabala(cs), "{be}");
        }
    }

    #[test]
    fn the_readings_of_the_rule_agree_over_the_range() {
        let gregorian_leap = |year: i64| year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        // Tum's page, which tests the Gregorian year the New Year falls in.
        let tum_leap_day = |be: i64| {
            let excess = tum_avoman(be);
            if gregorian_leap(be - BUDDHIST_ERA_OFFSET) {
                excess <= 126
            } else {
                excess <= 137 && !(excess == 137 && tum_avoman(be + 1) == 0)
            }
        };
        let tum = |be: i64| {
            let both = |year: i64| has_leap_month(year) && tum_leap_day(year);
            if has_leap_month(be) {
                YearType::ExtraMonth
            } else if tum_leap_day(be) || both(be - 1) {
                YearType::ExtraDay
            } else {
                YearType::Normal
            }
        };
        // momentkh, which carries a moved day on through a run of
        // leap-month years.
        let chained = |be: i64| {
            if has_leap_month(be) {
                return YearType::ExtraMonth;
            }
            if has_leap_day_by_rule(be) {
                return YearType::ExtraDay;
            }
            let mut earlier = be - 1;
            while has_leap_month(earlier) {
                if has_leap_day_by_rule(earlier) {
                    return YearType::ExtraDay;
                }
                earlier -= 1;
            }
            YearType::Normal
        };
        for be in FIRST_YEAR..=LAST_YEAR {
            let kind = year_type(be).unwrap();
            assert_eq!(tum(be), kind, "{be}");
            assert_eq!(chained(be), kind, "{be}");
            // The moved day always lands: a year called for both is
            // followed by one called for neither.
            if has_leap_month(be) && has_leap_day_by_rule(be) {
                assert!(
                    !has_leap_month(be + 1) && !has_leap_day_by_rule(be + 1),
                    "{be}"
                );
            }
        }
        // They part just outside: in 1818 and in 2272.
        assert_ne!(tum(2362), year_type_by_rule(2362));
        assert_ne!(tum(2816), year_type_by_rule(2816));
    }

    #[test]
    fn the_worked_example_of_2568_is_reproduced() {
        let cs = 2568 - CHULASAKARAT_OFFSET;
        assert_eq!(tum_ahakun(2568), 937_989);
        assert_eq!(kammacabala(cs), 725);
        assert_eq!(avoman(cs), 184);
        assert_eq!(new_year_tithi(cs), 8);
        assert_eq!(year_type(2567), Some(YearType::ExtraMonth));
        assert_eq!(year_type(2568), Some(YearType::Normal));
        assert_eq!(new_year(2567), Some(greg(2022, 11, 24)));
        assert_eq!(new_year(2568), Some(greg(2023, 12, 13)));
        assert_eq!(
            to_fixed(KhmerDate::new(2568, CHAET, 1)),
            Ok(greg(2024, 4, 9))
        );
        assert_eq!(
            to_fixed(KhmerDate::new(2568, CHAET, 5)),
            Ok(greg(2024, 4, 13))
        );
        assert_eq!(
            to_fixed(KhmerDate::new(2568, PISAKH, 15)),
            Ok(greg(2024, 5, 22))
        );
        assert_eq!(
            to_fixed(KhmerDate::new(2568, Month::regular(10), 30)),
            Ok(greg(2024, 10, 2))
        );
        // 2569 takes its own leap day.
        assert_eq!(avoman(cs + 1), 47);
        assert!(!is_solar_leap_year(cs + 1));
        assert_eq!(year_type(2569), Some(YearType::ExtraDay));
    }

    #[test]
    fn the_rule_gives_thailands_published_types_but_for_1994_and_1997() {
        // The same layout, the same years: the Thai year numbered 2535 BE
        // is the run of months whose Makha Bucha falls in 1992, as the
        // Khmer year 2536 is.
        use crate::thai_lunar;
        assert_eq!(thai_lunar::YEAR_TYPES.len(), 36);
        for (index, published) in thai_lunar::YEAR_TYPES.iter().enumerate() {
            let ce = thai_lunar::FIRST_GREGORIAN_YEAR + index as i64;
            let rule = year_type_by_rule(ce + BUDDHIST_ERA_OFFSET);
            assert_eq!(rule != *published, matches!(ce, 1994 | 1997), "{ce}");
        }
        assert_eq!(
            year_type(1994 + BUDDHIST_ERA_OFFSET),
            Some(YearType::ExtraDay)
        );
        assert_eq!(thai_lunar::year_type(1997 + 543), Some(YearType::ExtraDay));
    }

    #[test]
    fn month_lengths_follow_the_year_type() {
        let mut counts = [0usize; 3];
        for year in FIRST_YEAR..=LAST_YEAR {
            let kind = year_type(year).unwrap();
            let total: i64 = months_of(kind).map(|(_, length)| i64::from(length)).sum();
            assert_eq!(total, kind.days(), "{year}");
            if year < LAST_YEAR {
                assert_eq!(
                    new_year(year + 1).unwrap().0 - new_year(year).unwrap().0,
                    kind.days(),
                    "{year}"
                );
            }
            assert_eq!(
                month_length(year, Month::regular(7)),
                Ok(if kind == YearType::ExtraDay { 30 } else { 29 })
            );
            assert_eq!(
                month_length(year, Month::leap(8)).is_ok(),
                kind.has_extra_month()
            );
            counts[match kind {
                YearType::Normal => 0,
                YearType::ExtraDay => 1,
                YearType::ExtraMonth => 2,
            }] += 1;
        }
        assert_eq!(counts, [132, 58, 111]);
    }

    #[test]
    fn every_day_of_the_range_round_trips() {
        let calendar = KhmerCalendar;
        let (first, last) = (earliest().0, latest().0);
        assert_eq!(last - first + 1, 109_942);
        for rd in (first..=last).step_by(crate::sweep_stride(37)) {
            let date = calendar.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{date}");
            let fields = calendar.to_fields(date).expect("describable");
            assert_eq!(calendar.from_fields(&fields), Ok(date), "{date}");
            assert_eq!(fields.extra.get("printed-year"), Some(date.printed_year()));
            if rd == last {
                continue;
            }
            // The next day follows on, in the same month or on day 1 of the
            // next.
            let next = calendar.from_fixed(Rd(rd + 1)).expect("in range");
            if next.month == date.month && next.year == date.year {
                assert_eq!(next.day, date.day + 1, "{date}");
            } else {
                assert_eq!(next.day, 1, "{date}");
                assert_eq!(
                    Ok(date.day),
                    month_length(date.year, date.month),
                    "{date} ends its month"
                );
            }
        }
    }

    #[test]
    fn the_range_ends_where_the_rule_is_carried() {
        assert_eq!(earliest(), greg(1899, 12, 3));
        assert_eq!(latest(), greg(2200, 12, 6));
        // Tum's epoch: 1 January 1900 is 1 keit Bos.
        assert_eq!(
            from_fixed(greg(1900, 1, 1)),
            Ok(KhmerDate::new(FIRST_YEAR, Month::regular(2), 1))
        );
        assert_eq!(
            from_fixed(Rd(earliest().0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(latest().0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            from_fixed(latest()),
            Ok(KhmerDate::new(LAST_YEAR, Month::regular(12), 30))
        );
        assert_eq!(new_year(LAST_YEAR + 1), None);
        assert_eq!(
            to_fixed(KhmerDate::new(LAST_YEAR + 1, Month::regular(1), 1)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(KhmerDate::new(FIRST_YEAR - 1, Month::regular(12), 30)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            KhmerCalendar.is_leap_year(LAST_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn impossible_dates_are_refused() {
        // 2568 has one Asath and a 29-day Jesth.
        assert_eq!(
            to_fixed(KhmerDate::new(2568, Month::leap(8), 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(KhmerDate::new(2568, Month::regular(7), 30)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(KhmerDate::new(2570, Month::leap(7), 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(KhmerDate::new(2568, Month::regular(13), 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(KhmerDate::new(2568, Month::regular(1), 30)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(KhmerDate::new(2568, Month::regular(2), 0)),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn a_date_reads_as_a_khmer_calendar_writes_it() {
        // Khmer Wikipedia's example: Sunday 3 December 2017, 15 keit
        // Migasir, BE 2561.
        let rd = greg(2017, 12, 3);
        assert_eq!(weekday(rd), 0);
        let date = from_fixed(rd).unwrap();
        assert_eq!(date, KhmerDate::new(2562, Month::regular(1), 15));
        assert_eq!(date.printed_year(), 2561);
        assert_eq!(date.to_string(), "១៥កើត ខែមិគសិរ ព.ស.២៥៦១");
        // Visak Bochea 2025 and the day after, when the era changes.
        assert_eq!(
            from_fixed(greg(2025, 5, 11)).unwrap().to_string(),
            "១៥កើត ខែពិសាខ ព.ស.២៥៦៨"
        );
        assert_eq!(
            from_fixed(greg(2025, 5, 12)).unwrap().to_string(),
            "១រោច ខែពិសាខ ព.ស.២៥៦៩"
        );
        // 2026 doubles Asath: the first is the extra one.
        assert_eq!(year_type(2570), Some(YearType::ExtraMonth));
        let first = to_fixed(KhmerDate::new(2570, Month::leap(8), 1)).unwrap();
        let second = to_fixed(KhmerDate::new(2570, Month::regular(8), 1)).unwrap();
        assert_eq!(second.0 - first.0, 30);
        assert_eq!(from_fixed(first).unwrap().month_name(), FIRST_ASATH);
        assert_eq!(from_fixed(second).unwrap().month_name(), SECOND_ASATH);
        assert_eq!(
            from_fixed(Rd(second.0 + 15)).unwrap().to_string(),
            "១រោច ខែទុតិយាសាឍ ព.ស.២៥៧០"
        );
        assert_eq!(YearType::ExtraMonth.khmer_name(), "អធិកមាស បកតិវារៈ");
    }
}
