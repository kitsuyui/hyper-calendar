//! The Burmese calendar, the lunisolar calendar of Myanmar.
//!
//! The system is written up in `docs/systems/burmese.md` in the
//! repository: the months and the two halves of the month, watat and
//! yat-ngyin, the eras of the Myanmar Era and what each changed, the solar
//! New Year and Hnaung Tagu, Yan Naing Aye's arithmetic with 1374 ME worked
//! by hand, where each exception in the era tables comes from, what is
//! carried and not, and how the dates were checked against the published
//! holidays. This page summarises it and states the code's own facts.
//!
//! Twelve lunar months of 29 and 30 days alternately, Tagu first, kept in
//! step with a year of 365.2587565 days — the *Sūrya Siddhānta*'s sidereal
//! year — by a First Waso inserted before Waso in *watat* years and, in
//! *big watat* years, a thirtieth day of Nayon as well. The year is the
//! Myanmar Era, ME, whose year 0 began in 638 CE; the days are counted
//! waxing 1 to 15 and waning 1 to 14 or 15, waxing 15 being the civil full
//! moon. The New Year is solar: the *atat* moment falls in April, and the
//! days of Tagu (and sometimes Kason) before it are the old year's
//! *Hnaung* — late — Tagu, which [`BurmeseDate::late`] marks.
//!
//! # Whose arithmetic
//!
//! Yan Naing Aye's *Algorithm, Program and Calculation of Myanmar
//! Calendar* (2013), evaluated here in floating point: the year and the
//! lunation stated as ratios, the excess days at the New Year that decide a
//! watat year, the full moon of Second Waso that anchors the year, and
//! five eras that each carry their own watat rule, full-moon offset and
//! exceptions in [`ERAS`], as data. The eras and the exception tables are
//! the source's; the document says which of them the source explains.
//!
//! # What is not carried
//!
//! The Arakanese and Thai variants, which place the intercalary day
//! differently; the Thingyan's astrological moments beyond its days; and
//! any year whose two full moons the source's consistency check rejects,
//! which [`YearInfo::inconsistent`] reports rather than repairs.
//!
//! Sources: Yan Naing Aye, "Algorithm, Program and Calculation of Myanmar
//! Calendar", cool-emerald.blogspot.com, 2013, retrieved 2026-09-22, for
//! every constant, rule, exception and worked example; Wikipedia, "Burmese
//! calendar", retrieved 2026-09-22, for the month names in Burmese script
//! and the structure of the year. Keyed in `docs/references.bib` as
//! `yannaingaye2013` and `wikipedia-burmese-calendar`.

use core::fmt;

use hc_calendar::fields::ExtraFields;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_core::math::{floor, round};

/// The length of the Myanmar solar year, 1 577 917 828 / 4 320 000 days.
pub const SOLAR_YEAR: f64 = 1_577_917_828.0 / 4_320_000.0;

/// The length of the mean lunation, 1 577 917 828 / 53 433 336 days.
pub const LUNAR_MONTH: f64 = 1_577_917_828.0 / 53_433_336.0;

/// The Julian Date at which 0 ME began.
pub const EPOCH_JULIAN_DATE: f64 = 1_954_168.050_623;

/// Years from the Kali Yuga epoch to 0 ME.
pub const KALI_YUGA_OFFSET: i64 = 3_739;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 3_000;

/// The twelve months in Burmese script, Tagu first. Source: the months
/// table of Wikipedia, "Burmese calendar", retrieved 2026-09-22; the Latin
/// forms live in `hc-i18n`.
pub const MONTHS: [&str; 12] = [
    "တန်ခူး",
    "ကဆုန်",
    "နယုန်",
    "ဝါဆို",
    "ဝါခေါင်",
    "တော်သလင်း",
    "သီတင်းကျွတ်",
    "တန်ဆောင်မုန်း",
    "နတ်တော်",
    "ပြာသို",
    "တပို့တွဲ",
    "တပေါင်း",
];

/// How one era of the calendar decides its watat years and places its
/// full moons.
#[derive(Debug, Clone, Copy)]
pub struct EraRule {
    /// The first year the rule applies to.
    pub begin: i64,
    /// The last year the rule applies to.
    pub end: i64,
    /// The offset added before rounding the full moon of Second Waso.
    pub full_moon_offset: f64,
    /// The months of excess days that decide a watat year, or −1 for the
    /// Metonic cycle.
    pub months: i64,
    /// Years whose full moon of Second Waso the record puts a day or two
    /// from the rule, with the correction.
    pub full_moon_exceptions: &'static [(i64, i64)],
    /// Years whose watat status the record sets against the rule.
    pub watat_exceptions: &'static [(i64, bool)],
}

/// The five rules, as the source tabulates them.
pub static ERAS: [EraRule; 5] = [
    // Makaranta 1, the kings to 797 ME.
    EraRule {
        begin: -999,
        end: 797,
        full_moon_offset: -1.1,
        months: -1,
        full_moon_exceptions: &[
            (205, 1),
            (246, 1),
            (471, 1),
            (572, -1),
            (651, 1),
            (653, 2),
            (656, 1),
            (672, 1),
            (729, 1),
            (767, -1),
        ],
        watat_exceptions: &[],
    },
    // Makaranta 2, 798–1099 ME.
    EraRule {
        begin: 798,
        end: 1099,
        full_moon_offset: -1.1,
        months: -1,
        full_moon_exceptions: &[
            (813, -1),
            (849, -1),
            (851, -1),
            (854, -1),
            (927, -1),
            (933, -1),
            (936, -1),
            (938, -1),
            (949, -1),
            (952, -1),
            (963, -1),
            (968, -1),
            (1039, -1),
        ],
        watat_exceptions: &[],
    },
    // Thandeikta, 1100–1216 ME.
    EraRule {
        begin: 1100,
        end: 1216,
        full_moon_offset: -0.85,
        months: -1,
        full_moon_exceptions: &[(1120, 1), (1126, -1), (1150, 1), (1172, -1), (1207, 1)],
        watat_exceptions: &[(1201, true), (1202, false)],
    },
    // The British era, 1217–1311 ME.
    EraRule {
        begin: 1217,
        end: 1311,
        full_moon_offset: -1.0,
        months: 4,
        full_moon_exceptions: &[(1234, 1), (1261, -1)],
        watat_exceptions: &[(1263, true), (1264, false)],
    },
    // The Calendar Advisory Board, 1312 ME on.
    EraRule {
        begin: 1312,
        end: 9999,
        full_moon_offset: -0.5,
        months: 8,
        full_moon_exceptions: &[(1377, 1)],
        watat_exceptions: &[(1344, true), (1345, false)],
    },
];

/// The rule in force in `year`.
#[must_use]
pub fn era_of(year: i64) -> &'static EraRule {
    ERAS.iter()
        .rev()
        .find(|era| year >= era.begin)
        .unwrap_or(&ERAS[0])
}

/// The three kinds of year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YearType {
    /// Twelve months, 354 days.
    Common,
    /// A First Waso inserted, 384 days.
    LittleWatat,
    /// A First Waso and a thirtieth day of Nayon inserted, 385 days.
    BigWatat,
}

impl YearType {
    /// The days in a year of this type.
    #[must_use]
    pub const fn days(self) -> i64 {
        match self {
            Self::Common => 354,
            Self::LittleWatat => 384,
            Self::BigWatat => 385,
        }
    }

    /// Whether the year has the intercalary month.
    #[must_use]
    pub const fn has_watat(self) -> bool {
        !matches!(self, Self::Common)
    }
}

/// Whether `year` is a watat year, and if so the Julian Day Number of the
/// full moon of its Second Waso.
#[must_use]
pub fn watat(year: i64) -> (bool, i64) {
    let era = era_of(year);
    let months = era.months as f64;
    let threshold_adjust = (SOLAR_YEAR / 12.0 - LUNAR_MONTH) * (12.0 - months);
    let mut excess = (SOLAR_YEAR * (year + KALI_YUGA_OFFSET) as f64) % LUNAR_MONTH;
    if excess < threshold_adjust {
        excess += LUNAR_MONTH;
    }
    let mut is_watat = if era.months >= 0 {
        let threshold_watat = LUNAR_MONTH - (SOLAR_YEAR / 12.0 - LUNAR_MONTH) * months;
        excess >= threshold_watat
    } else {
        // The Metonic cycle: remainders 2, 5, 7, 10, 13, 15 and 18.
        (year * 7 + 2).rem_euclid(19) / 12 == 1
    };
    if let Some((_, forced)) = era
        .watat_exceptions
        .iter()
        .find(|(exception, _)| *exception == year)
    {
        is_watat = *forced;
    }
    if !is_watat {
        return (false, 0);
    }
    let mut full_moon = round(
        SOLAR_YEAR * year as f64 + EPOCH_JULIAN_DATE - excess
            + 4.5 * LUNAR_MONTH
            + era.full_moon_offset,
    ) as i64;
    if let Some((_, shift)) = era
        .full_moon_exceptions
        .iter()
        .find(|(exception, _)| *exception == year)
    {
        full_moon += shift;
    }
    (true, full_moon)
}

/// What a year is, and where it starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearInfo {
    /// The year's type.
    pub year_type: YearType,
    /// The Julian Day Number of the first day of Oo Tagu, the early Tagu.
    pub first_tagu: i64,
    /// The Julian Day Number of the full moon of Waso — of Second Waso in a
    /// watat year.
    pub waso_full_moon: i64,
    /// Whether the interval from the previous watat year's full moon was
    /// neither 30 nor 31 days over the common years between, a case the
    /// source says would be disputed; reported rather than repaired.
    pub inconsistent: bool,
}

/// The year's type, first day of Tagu and full moon of Waso.
#[must_use]
pub fn year_info(year: i64) -> YearInfo {
    let (this_watat, this_full_moon) = watat(year);
    let mut years_back = 0;
    let mut previous = (false, 0);
    while years_back < 3 {
        years_back += 1;
        previous = watat(year - years_back);
        if previous.0 {
            break;
        }
    }
    let mut inconsistent = false;
    let (year_type, waso_full_moon) = if this_watat {
        let gap = (this_full_moon - previous.1).rem_euclid(354);
        let year_type = if gap / 31 + 1 == 2 {
            YearType::BigWatat
        } else {
            YearType::LittleWatat
        };
        if gap != 30 && gap != 31 {
            inconsistent = true;
        }
        (year_type, this_full_moon)
    } else {
        (YearType::Common, previous.1 + 354 * years_back)
    };
    YearInfo {
        year_type,
        first_tagu: previous.1 + 354 * years_back - 102,
        waso_full_moon,
        inconsistent,
    }
}

/// The Julian Day Number of the New Year's day of `year` — the day after
/// the *atat* day.
#[must_use]
pub fn new_year_day(year: i64) -> i64 {
    round(SOLAR_YEAR * year as f64 + EPOCH_JULIAN_DATE) as i64 + 1
}

/// The Myanmar year a Julian Day Number falls in, by the solar New Year.
#[must_use]
pub fn year_of_jdn(jdn: i64) -> i64 {
    floor((jdn as f64 - 0.5 - EPOCH_JULIAN_DATE) / SOLAR_YEAR) as i64
}

/// The days of Thingyan, the New Year festival, for a Myanmar year: the
/// *akyo* eve, the *akya* day on which the festival begins, the one or two
/// *akyat* days between, the *atat* day on which the old year ends, and
/// the New Year's day after it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Thingyan {
    /// The eve, the day before *akya*.
    pub akyo: Rd,
    /// The first day of the festival.
    pub akya: Rd,
    /// The last day between *akya* and *atat*, one or two days after *akya*.
    pub last_akyat: Rd,
    /// The day the old year ends.
    pub atat: Rd,
    /// The New Year's day.
    pub new_year: Rd,
}

/// The Thingyan of `year`: the *atat* moment is the start of the solar
/// year, and the *akya* moment is 2.169918982 days before it since 1312 ME
/// and 2.1675 days before it in the years of the kings.
#[must_use]
pub fn thingyan(year: i64) -> Thingyan {
    let atat_time = SOLAR_YEAR * year as f64 + EPOCH_JULIAN_DATE;
    let akya_time = if year >= 1312 {
        atat_time - 2.169_918_982
    } else {
        atat_time - 2.1675
    };
    let atat = round(atat_time) as i64;
    let akya = round(akya_time) as i64;
    Thingyan {
        akyo: jdn_to_rd(akya - 1),
        akya: jdn_to_rd(akya),
        last_akyat: jdn_to_rd(atat - 1),
        atat: jdn_to_rd(atat),
        new_year: jdn_to_rd(atat + 1),
    }
}

const fn jdn_to_rd(jdn: i64) -> Rd {
    Rd(jdn - 1_721_425)
}

const fn rd_to_jdn(rd: Rd) -> i64 {
    rd.0 + 1_721_425
}

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Year 0 of the Myanmar Era begun on 22 March 638 by the reckoning of King Popa Sawrahan \
    [wikipedia-burmese-calendar]; the eras of the calendar's rules to the Calendar Advisory \
    Board of 1950 on [yannaingaye2013], which sits today in the Ministry of Religious Affairs \
    and Culture, as docs/systems/burmese.md states";

/// The earliest fixed day this implementation converts: the New Year's
/// day of 1 ME.
pub fn earliest() -> Rd {
    jdn_to_rd(new_year_day(MIN_YEAR))
}

/// The latest fixed day this implementation converts: the day before the
/// New Year's day of the year after [`MAX_YEAR`].
pub fn latest() -> Rd {
    jdn_to_rd(new_year_day(MAX_YEAR + 1) - 1)
}

/// The phase of the month a day falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoonPhase {
    /// Waxing, 1 to 14.
    Waxing,
    /// The full moon, waxing 15.
    FullMoon,
    /// Waning, 1 to 13 or 14.
    Waning,
    /// The new moon, the last day of the month.
    NewMoon,
}

/// A Burmese date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BurmeseDate {
    /// The year of the Myanmar Era.
    pub year: i64,
    /// The month, 1 for Tagu through 12 for Tabaung; the intercalary
    /// First Waso is the leap month 4, and precedes the regular Waso.
    pub month: Month,
    /// Whether this is the late — *Hnaung* — Tagu or Kason, the stretch of
    /// the month that falls after the year's other months and before the
    /// next solar New Year.
    pub late: bool,
    /// The day of the month, 1 to 30, counted straight through.
    pub day: u8,
}

impl BurmeseDate {
    /// The number of days in this date's month.
    #[must_use]
    pub fn month_length(&self) -> u8 {
        month_length(self.year, self.month)
    }

    /// The day within the fortnight, 1 to 15.
    #[must_use]
    pub const fn fortnight_day(&self) -> u8 {
        self.day - 15 * (self.day / 16)
    }

    /// The phase of the month this day falls in.
    #[must_use]
    pub fn phase(&self) -> MoonPhase {
        let length = self.month_length();
        match (self.day + 1) / 16 + self.day / 16 + self.day / length {
            0 => MoonPhase::Waxing,
            1 => MoonPhase::FullMoon,
            2 => MoonPhase::Waning,
            _ => MoonPhase::NewMoon,
        }
    }

    /// The name of the month in Burmese script.
    #[must_use]
    pub const fn month_name(&self) -> &'static str {
        MONTHS[self.month.ordinal as usize - 1]
    }
}

impl fmt::Display for BurmeseDate {
    /// Writes the date as it is spoken: the month, its late half if so, and
    /// the fortnight day with its phase, as in `Nayon waxing 3, 1374 ME`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.late {
            write!(f, "Hnaung ")?;
        }
        if self.month.leap {
            write!(f, "First ")?;
        }
        write!(f, "{} ", self.month_name())?;
        match self.phase() {
            MoonPhase::Waxing => write!(f, "waxing {}", self.fortnight_day())?,
            MoonPhase::FullMoon => write!(f, "full moon")?,
            MoonPhase::Waning => write!(f, "waning {}", self.fortnight_day())?,
            MoonPhase::NewMoon => write!(f, "new moon")?,
        }
        write!(f, ", {} ME", self.year)
    }
}

/// The number of days in `month` of `year`: 29 and 30 alternately, First
/// Waso 30, and Nayon 30 in a big watat year.
#[must_use]
pub fn month_length(year: i64, month: Month) -> u8 {
    if month.leap {
        return 30;
    }
    let mut length = 30 - month.ordinal % 2;
    if month.ordinal == 3 && year_info(year).year_type == YearType::BigWatat {
        length += 1;
    }
    length
}

/// The Burmese date of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside the years converted.
pub fn from_fixed(rd: Rd) -> CalendarResult<BurmeseDate> {
    if rd < earliest() {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > latest() {
        return Err(CalendarError::AfterSupportedRange);
    }
    let jdn = rd_to_jdn(rd);
    let year = year_of_jdn(jdn);
    let info = year_info(year);
    let mut day_count = jdn - info.first_tagu + 1;
    let big = i64::from(info.year_type == YearType::BigWatat);
    let common = i64::from(info.year_type == YearType::Common);
    let year_length = info.year_type.days();
    let late = (day_count - 1).div_euclid(year_length);
    day_count -= late * year_length;
    let a = (day_count + 423).div_euclid(512);
    let month_index =
        floor(((day_count - big * a + common * a * 30) as f64 + 29.26) / 29.544) as i64;
    let e = (month_index + 12).div_euclid(16);
    let f = (month_index + 11).div_euclid(16);
    let day =
        day_count - floor(29.544 * month_index as f64 - 29.26) as i64 - big * e + common * f * 30;
    let month_number = month_index + f * 3 - e * 4;
    let month = if month_number == 0 {
        Month::leap(4)
    } else {
        Month::regular(month_number as u8)
    };
    Ok(BurmeseDate {
        year,
        month,
        late: late == 1,
        day: day as u8,
    })
}

/// The fixed day of a Burmese date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside the years converted,
/// [`CalendarError::MonthOutOfRange`] for a First Waso in a common year or
/// a month outside `1..=12`, and [`CalendarError::DayOutOfRange`] for a day
/// the month does not have, including a late Tagu the year's days do not
/// reach.
pub fn to_fixed(date: BurmeseDate) -> CalendarResult<Rd> {
    if date.year < MIN_YEAR || date.year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    if date.month.ordinal == 0 || date.month.ordinal > 12 {
        return Err(CalendarError::MonthOutOfRange);
    }
    if date.month.leap && date.month.ordinal != 4 {
        return Err(CalendarError::MonthOutOfRange);
    }
    let info = year_info(date.year);
    if date.month.leap && !info.year_type.has_watat() {
        return Err(CalendarError::MonthOutOfRange);
    }
    if date.day == 0 || date.day > month_length(date.year, date.month) {
        return Err(CalendarError::DayOutOfRange);
    }
    let big = i64::from(info.year_type == YearType::BigWatat);
    let common = i64::from(info.year_type == YearType::Common);
    let month_number = if date.month.leap {
        0
    } else {
        i64::from(date.month.ordinal)
    };
    let month_index = month_number + 4 - 4 * (month_number + 15).div_euclid(16)
        + (month_number + 12).div_euclid(16);
    let mut day_count = i64::from(date.day) + floor(29.544 * month_index as f64 - 29.26) as i64
        - common * (month_index + 11).div_euclid(16) * 30
        + big * (month_index + 12).div_euclid(16);
    if date.late {
        day_count += info.year_type.days();
    }
    let rd = jdn_to_rd(day_count + info.first_tagu - 1);
    // A late Tagu or Kason that runs into the next year's early months, or
    // a date the arithmetic cannot place, is refused rather than returned.
    if from_fixed(rd)? != date {
        return Err(CalendarError::DayOutOfRange);
    }
    Ok(rd)
}

/// The Burmese calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BurmeseCalendar;

impl Calendar for BurmeseCalendar {
    type Date = BurmeseDate;

    /// From the New Year's Day of 1 ME, the first day this calendar converts,
    /// and kept today under the Calendar Advisory Board.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(earliest(), USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// A watat year, with its Second Waso.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(year_info(year).year_type.has_watat())
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("burmese"),
            english_name: "Burmese",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: false,
            earliest: Some(earliest()),
            latest: Some(latest()),
            native_locales: &["my"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        from_fixed(rd)
    }

    /// The year, month and day, with the late half, the fortnight day and
    /// the phase as extra fields.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("late", i64::from(date.late))?;
        extra.set("fortnight-day", i64::from(date.fortnight_day()))?;
        extra.set(
            "phase",
            match date.phase() {
                MoonPhase::Waxing => 0,
                MoonPhase::FullMoon => 1,
                MoonPhase::Waning => 2,
                MoonPhase::NewMoon => 3,
            },
        )?;
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
        let month = fields.require_month()?;
        let day = fields.require_day()?;
        let late = fields.extra.get("late").unwrap_or(0) == 1;
        let date = BurmeseDate {
            year: fields.year,
            month,
            late,
            day,
        };
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

    #[test]
    fn the_sources_worked_example_of_1374_me_is_reproduced() {
        // 1374 ME is a watat year whose Second Waso full moon is JDN
        // 2456142, 2 August 2012; the previous watat year is 1372, full moon
        // 2455404; 1374 is a little watat year and its Tagu begins at JDN
        // 2456010, 23 March 2012.
        assert_eq!(watat(1374), (true, 2_456_142));
        assert_eq!(watat(1372), (true, 2_455_404));
        assert!(!watat(1373).0);
        let info = year_info(1374);
        assert_eq!(info.year_type, YearType::LittleWatat);
        assert_eq!(info.first_tagu, 2_456_010);
        assert_eq!(info.waso_full_moon, 2_456_142);
        assert!(!info.inconsistent);
        assert_eq!(jdn_to_rd(2_456_010), greg(2012, 3, 23));
        assert_eq!(jdn_to_rd(2_456_142), greg(2012, 8, 2));
        // 23 May 2012 is Nayon waxing 3, 1374 ME.
        let date = from_fixed(greg(2012, 5, 23)).expect("in range");
        assert_eq!(
            (date.year, date.month, date.late, date.day),
            (1374, Month::regular(3), false, 3)
        );
        assert_eq!(date.to_string(), "နယုန် waxing 3, 1374 ME");
        assert_eq!(date.phase(), MoonPhase::Waxing);
        // The start of 1375 ME is 16 April 2013, still Hnaung Tagu of 1374.
        let atat = from_fixed(greg(2013, 4, 16)).expect("in range");
        assert_eq!(
            (atat.year, atat.month.ordinal, atat.late, atat.day),
            (1374, 1, true, 6)
        );
        assert_eq!(jdn_to_rd(new_year_day(1375)), greg(2013, 4, 17));
    }

    #[test]
    fn the_exceptions_of_the_record_are_applied() {
        // 1344 ME had the intercalary month instead of 1345.
        assert!(watat(1344).0);
        assert!(!watat(1345).0);
        // 1263 instead of 1264, 1201 instead of 1202.
        assert!(watat(1263).0);
        assert!(!watat(1264).0);
        assert!(watat(1201).0);
        assert!(!watat(1202).0);
        // 1377's full moon a day later than the rule.
        let era = era_of(1377);
        assert_eq!(era.begin, 1312);
        assert_eq!(era.full_moon_exceptions, &[(1377, 1)]);
        assert_eq!(era_of(1216).end, 1216);
        assert_eq!(era_of(0).full_moon_offset, -1.1);
    }

    #[test]
    fn the_full_moons_of_1386_me_fall_where_the_published_calendar_puts_them() {
        // Kason, Waso, Thadingyut and Tazaungmon full moons of 2024.
        for (y, m, d, month) in [
            (2024, 5, 22, 2),
            (2024, 7, 20, 4),
            (2024, 10, 17, 7),
            (2024, 11, 15, 8),
        ] {
            let date = from_fixed(greg(y, m, d)).expect("in range");
            assert_eq!(
                (date.year, date.month, date.day),
                (1386, Month::regular(month), 15),
                "{y}-{m}-{d}"
            );
            assert_eq!(date.phase(), MoonPhase::FullMoon);
            assert_eq!(date.fortnight_day(), 15);
        }
        assert_eq!(year_info(1386).year_type, YearType::Common);
        assert_eq!(year_info(1385).year_type, YearType::BigWatat);
        assert_eq!(year_info(1388).year_type, YearType::BigWatat);
        assert_eq!(year_info(1382).year_type, YearType::LittleWatat);
        assert_eq!(month_length(1385, Month::regular(3)), 30);
        assert_eq!(month_length(1386, Month::regular(3)), 29);
        assert_eq!(month_length(1385, Month::leap(4)), 30);
    }

    #[test]
    fn every_day_of_three_decades_round_trips() {
        let calendar = BurmeseCalendar;
        let start = greg(2000, 1, 1).0;
        let end = greg(2031, 1, 1).0;
        let mut previous: Option<BurmeseDate> = None;
        for rd in start..end {
            let date = calendar.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd} {date}");
            let fields = calendar.to_fields(date).expect("describable");
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            if let Some(before) = previous
                && before.year == date.year
                && before.month == date.month
                && before.late == date.late
            {
                assert_eq!(date.day, before.day + 1, "rd {rd}");
            }
            previous = Some(date);
        }
        for rd in (earliest().0..=latest().0).step_by(997) {
            let date = calendar.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn impossible_dates_are_refused() {
        // 1386 ME has no First Waso; 1385 has one.
        let first_waso = BurmeseDate {
            year: 1386,
            month: Month::leap(4),
            late: false,
            day: 1,
        };
        assert_eq!(to_fixed(first_waso), Err(CalendarError::MonthOutOfRange));
        assert!(
            to_fixed(BurmeseDate {
                year: 1385,
                month: Month::leap(4),
                late: false,
                day: 30
            })
            .is_ok()
        );
        assert_eq!(
            to_fixed(BurmeseDate {
                year: 1386,
                month: Month::regular(1),
                late: false,
                day: 30
            }),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(BurmeseDate {
                year: 1386,
                month: Month::regular(13),
                late: false,
                day: 1
            }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(BurmeseDate {
                year: 0,
                month: Month::regular(1),
                late: false,
                day: 1
            }),
            Err(CalendarError::YearOutOfRange)
        );
        // A late Kason the year does not reach: 1386's New Year fell in Tagu.
        assert_eq!(
            to_fixed(BurmeseDate {
                year: 1386,
                month: Month::regular(2),
                late: true,
                day: 1
            }),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            from_fixed(Rd(earliest().0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(latest().0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn the_thingyan_of_1386_me_ran_from_13_to_17_april_2024() {
        let festival = thingyan(1386);
        assert_eq!(festival.akyo, greg(2024, 4, 13));
        assert_eq!(festival.akya, greg(2024, 4, 14));
        assert_eq!(festival.last_akyat, greg(2024, 4, 15));
        assert_eq!(festival.atat, greg(2024, 4, 16));
        assert_eq!(festival.new_year, greg(2024, 4, 17));
        assert_eq!(festival.new_year, jdn_to_rd(new_year_day(1386)));
    }
}
