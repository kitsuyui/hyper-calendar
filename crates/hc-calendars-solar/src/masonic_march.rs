//! The Masonic year of light from 1 March, `masonic-anno-lucis-march`.
//!
//! French Freemasonry wrote its dates in a year of light that begins on
//! 1 March and numbers its months: "L'année maçonnique a la même longueur
//! que l'année grégorienne, mais débute le 1er mars", taking "le millésime
//! de l'année grégorienne en cours, augmenté de 4000", with "les mois ...
//! désignés que par leur numéro ordinal" (French Wikipedia, "Calendrier
//! maçonnique", `frwiki-calendrier-maconnique`, retrieved 2026-09-29): March
//! is the first month, and January and February are the eleventh and
//! twelfth of the year before, as the Rectified Scottish Rite's own page
//! gives it (`rer-calendrier-maconnique`, retrieved 2026-09-29). Bernheim
//! calls it "the March = 1st month code", "frequently used on documents
//! issued in France or by French Brethren", so that "the 12th month of the
//! masonic year 5772 should be decoded as February 1773, not 1772", and the
//! classical code of eighteenth-century French Freemasonry
//! ("The Dating of Masonic Records", *Ars Quatuor Coronatorum* 99, 1986,
//! §1.2.1, `bernheim1986`, read in an archived HTML copy, 2026-09-29).
//!
//! It is a competing convention of `masonic-anno-lucis`, whose year is the
//! Gregorian year plus 4000 from 1 January (policy §5): the two agree from
//! March to December and differ by a year, and by the month's number, in
//! January and February. The year is the Gregorian year its March falls
//! in, plus 4000; the months are the Gregorian months from March, numbered
//! 1 to 12; the days are the Gregorian days. A year is leap when its
//! twelfth month, the February of the next Gregorian year, has 29 days.
//!
//! Not carried: the "June = 1st month code" Bernheim reports from the
//! regulations of one lodge at Saint-Pierre de la Martinique of 1750,
//! "very seldom used" and with no day given for the month's start.
//!
//! The system document is `docs/systems/era-counts.md` in the repository.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};

use crate::gregorian;

/// The machine identifier.
pub const ID: &str = "masonic-anno-lucis-march";

/// The era code, the Anno Lucis's.
pub const ERA: &str = "al";

/// The year of light less the Gregorian year its March falls in.
pub const OFFSET: i64 = 4_000;

/// The earliest year of light converted: 1, from 1 March of the Gregorian
/// year −3999.
pub const MIN_YEAR: i64 = 1;

/// The latest year of light converted, whose twelfth month is still a
/// Gregorian date.
pub const MAX_YEAR: i64 = gregorian::MAX_YEAR - 1 + OFFSET;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Bernheim, \"The Dating of Masonic Records\", AQC 99 (1986), §1.2.1 \
    [bernheim1986]: \"the March = 1st month code\", \"frequently used on documents issued in \
    France or by French Brethren\", the classical code of 18th-century French Freemasonry; \
    French Wikipedia, \"Calendrier maçonnique\" [frwiki-calendrier-maconnique], and the \
    Rectified Scottish Rite's page [rer-calendrier-maconnique], for its use today. None dates \
    a first use";

/// The Gregorian year and month of month `month` of year of light `year`.
const fn gregorian_month(year: i64, month: u8) -> (i64, u8) {
    let base = year - OFFSET;
    if month <= 10 {
        (base, month + 2)
    } else {
        (base + 1, month - 10)
    }
}

/// Whether year of light `year` is leap: whether its twelfth month has 29
/// days.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year - OFFSET + 1)
}

/// The fixed day of a date of this calendar.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`], [`CalendarError::MonthOutOfRange`] outside
/// `1..=12`, and [`CalendarError::DayOutOfRange`] for a day the month does
/// not have.
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    if month == 0 || month > 12 {
        return Err(CalendarError::MonthOutOfRange);
    }
    let (gregorian_year, gregorian_month) = gregorian_month(year, month);
    gregorian::to_fixed(gregorian_year, gregorian_month, day)
}

/// The year of light, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before 1 March of year of light
/// 1, and the Gregorian calendar's errors beyond its range.
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    let (year, month, day) = match gregorian::from_fixed(rd) {
        Ok(date) => date,
        Err(error) => return Err(error),
    };
    let (year, month) = if month >= 3 {
        (year + OFFSET, month - 2)
    } else {
        (year - 1 + OFFSET, month + 10)
    };
    if year < MIN_YEAR {
        return Err(CalendarError::BeforeEpoch);
    }
    if year > MAX_YEAR {
        return Err(CalendarError::AfterSupportedRange);
    }
    Ok((year, month, day))
}

/// A date in the year of light from 1 March.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MasonicMarchDate {
    /// The year of light.
    pub year: i64,
    /// The month, 1 for March to 12 for February.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

/// The Masonic year of light from 1 March, with numbered months.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MasonicMarchCalendar;

/// The months as French Freemasonry writes them, by number: "le 28e jour
/// du 12e mois", "le 1er jour du 1er mois" (`frwiki-calendrier-maconnique`).
pub const MONTHS: [&str; 12] = [
    "1er mois", "2e mois", "3e mois", "4e mois", "5e mois", "6e mois", "7e mois", "8e mois",
    "9e mois", "10e mois", "11e mois", "12e mois",
];

/// Twelve numbered months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for MasonicMarchCalendar {
    type Date = MasonicMarchDate;

    /// French Freemasonry's code, of the eighteenth century and today, from
    /// no dated first use.
    fn usage(&self) -> Usage {
        Usage::undated(USAGE_SOURCE)
    }

    /// Twelve months known by their numbers, and the week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// "A.L.", the abbreviation `masonic-anno-lucis` writes, which the
    /// locales key by that calendar's identifier.
    fn era_name(&self, code: &str) -> Option<hc_calendar::shape::EraName> {
        (code == ERA).then_some(hc_calendar::shape::EraName::new("A.L.", ""))
    }

    fn era_code(&self, index: usize) -> Option<&'static str> {
        (index == 0).then_some(ERA)
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Masonic Anno Lucis (from 1 March)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: to_fixed(MIN_YEAR, 1, 1).ok(),
            latest: to_fixed(MAX_YEAR, 12, 28).ok(),
            native_locales: &["fr"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(MasonicMarchDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(ERA))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        to_fixed(fields.year, month.ordinal, day)?;
        Ok(MasonicMarchDate {
            year: fields.year,
            month: month.ordinal,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    /// French Wikipedia's pair of 2019 and Bernheim's decodings.
    #[test]
    fn the_sources_dates_decode_as_they_say() {
        // "le 28 février 2019 a été : le 28e jour du 12e mois de l'an 6018",
        // "le 1er mars 2019 a été : le 1er jour du 1er mois de l'an 6019".
        assert_eq!(from_fixed(day(2019, 2, 28)), Ok((6_018, 12, 28)));
        assert_eq!(from_fixed(day(2019, 3, 1)), Ok((6_019, 1, 1)));
        // Bernheim: "le 21e Jour du 12e mois de l'an Maçonnique 5801 ...
        // means 21 February 1802"; the 12th month of 5772 is February 1773;
        // Mackey's 1 January 1872 is correctly "the 1st day of the 11th
        // Masonic month" of 5871.
        assert_eq!(to_fixed(5_801, 12, 21), Ok(day(1802, 2, 21)));
        assert_eq!(to_fixed(5_772, 12, 1), Ok(day(1773, 2, 1)));
        assert_eq!(from_fixed(day(1872, 1, 1)), Ok((5_871, 11, 1)));
        // The Rectified Scottish Rite's page: 3 January 2017 is the third
        // day of the eleventh month of the year before.
        assert_eq!(
            from_fixed(day(2017, 1, 3)).map(|(_, m, d)| (m, d)),
            Ok((11, 3))
        );
    }

    #[test]
    fn the_year_agrees_with_anno_lucis_from_march_to_december() {
        let from_january = crate::year_counts::ANNO_LUCIS;
        for rd in (day(1700, 1, 1).0..day(2100, 1, 1).0).step_by(13) {
            let (year, month, _) = from_fixed(Rd(rd)).unwrap();
            let (january_year, gregorian_month, _) = from_january.from_fixed(Rd(rd)).unwrap();
            if gregorian_month >= 3 {
                assert_eq!((year, month), (january_year, gregorian_month - 2));
            } else {
                assert_eq!((year, month), (january_year - 1, gregorian_month + 10));
            }
        }
    }

    #[test]
    fn every_day_round_trips_and_the_errors_are_the_right_ones() {
        let calendar = MasonicMarchCalendar;
        for rd in day(1996, 1, 1).0..day(2005, 1, 1).0 {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date));
        }
        // 6019's twelfth month is February 2020, of 29 days.
        assert!(is_leap_year(6_019));
        assert!(!is_leap_year(6_020));
        assert_eq!(to_fixed(6_019, 12, 29), Ok(day(2020, 2, 29)));
        assert_eq!(to_fixed(6_020, 12, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(6_020, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            from_fixed(Rd(day(-3_999, 3, 1).0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(from_fixed(day(-3_999, 3, 1)), Ok((1, 1, 1)));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(6_019, 1, 1).with_era("ad")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(calendar.is_leap_year(0), Err(CalendarError::YearOutOfRange));
        assert_eq!(calendar.era_name(ERA).map(|name| name.native), Some("A.L."));
        assert_eq!(calendar.era_code(0), Some(ERA));
        assert_eq!(calendar.era_code(1), None);
    }
}
