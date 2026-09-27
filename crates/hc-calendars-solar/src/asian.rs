//! The calendar of the Roman province of Asia: the Macedonian months fixed
//! to the Julian year by the decree of 9/8 BC.
//!
//! The koinon of the Greeks of Asia, at the proposal of the proconsul
//! Paullus Fabius Maximus, began the year on Augustus's birthday, the ninth
//! day before the Kalends of October, 23 September, named the first month
//! Kaisar, and began every month on the ninth day before the Kalends of a
//! Roman month. The decree lists the months and their days — Kaisar 31,
//! Apellaios 30, Audnaios 31, Peritios 31, Dystros 28, Xandikos 31,
//! Artemision 30, Daisios 31, Panemos 30, Loos 31, Gorpiaios 31,
//! Hyperberetaios 30, "together 365" — and gives Xandikos 32 days in a leap
//! year (OGIS 458, lines 68–77, in Dittenberger's edition of 1905,
//! `dittenberger-ogis2`, read 2026-09-27; the same table in Bultrighini,
//! "Calendars of the Greek East under Rome", `bultrighini2021`). So every
//! Asian day is a Julian day under another name, and this module is that
//! renaming.
//!
//! A 31-day month opens with an unnumbered day, which the inscriptions call
//! Sebaste, and then counts 1 to 30: the Metropolis *hemerologion* puts 7
//! October on day 14 of the first month and 5 December on day 12 of the
//! third, which only that count gives. In a leap year Xandikos has two
//! unnumbered days before its day 1: Sebaste and the intercalary day. The
//! order of the two is not settled by the sources read, so a date here
//! gives them as the first and second unnumbered day, [`WrittenDay`].
//!
//! The system document is `docs/systems/asian-calendar.md` in the
//! repository: the decree, the worked example, the anchors, and the dates
//! from Acmonia that disagree by a day.
//!
//! # The year number is this library's
//!
//! The province did not number its years by one era: cities kept their own.
//! A date here carries the Julian year, AD, in which its Asian year began,
//! so that year 50 runs from 23 September 50 to 22 September 51, and no era
//! code. That is a label for the arithmetic, not a count anyone wrote.
//!
//! # Range
//!
//! The first year carried begins on 23 September AD 4. The decree's own
//! year is disputed, 9/8 BC or 5 BC, and its leap day falls "two years
//! coming between", Rome's practice of the time; Dittenberger's note 45
//! holds that Asia intercalated in the Roman years. Not until 25 February
//! AD 4 do all the reconstructions of the Roman leap years put a Roman date
//! on its proleptic Julian day: Bennett's, Matzat's, Harriot's and
//! Bünting's do so from 25 February 1 BC, Scaliger's, Ideler's, Kepler's,
//! Christmann's and Soltau's from AD 4 (Wikipedia, "Julian calendar", its
//! table of reconstructions, `wikipedia-julian-calendar`, read 2026-09-27).
//! So the days of the earlier years are refused rather than guessed.
//!
//! That start rests on a secondary source alone. The reconstructions
//! themselves, Bennett's of 2003 (*ZPE* 142 and 147) first, and the
//! ancient accounts of the triennial leap years they read, Macrobius's
//! *Saturnalia* 1.14.13–15, Solinus and Pliny, were not read; they are
//! what would replace the table.

use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};

use crate::julian;

/// The months, from Kaisar, as Bultrighini transliterates the decree's
/// Καῖσαρ, Ἀπελλαῖος, Αὐδναῖος, Περίτιος, Δύστρος, Ξανδικός, Ἀρτεμισιών,
/// Δαίσιος, Πάνημος, Λῶος, Γορπιαῖος and Ὑπερβερεταῖος.
pub const MONTHS: [&str; 12] = [
    "Kaisar",
    "Apellaios",
    "Audnaios",
    "Peritios",
    "Dystros",
    "Xandikos",
    "Artemision",
    "Daisios",
    "Panemos",
    "Loos",
    "Gorpiaios",
    "Hyperberetaios",
];

/// The days of each month in a common year, as the decree lists them.
pub const MONTH_LENGTHS: [u8; 12] = [31, 30, 31, 31, 28, 31, 30, 31, 30, 31, 31, 30];

/// The month that takes the leap day: Xandikos.
pub const LEAP_MONTH: u8 = 6;

/// The Julian month and day each Asian month begins on: the ninth day
/// before the Kalends of the Roman month after it.
pub const MONTH_STARTS: [(u8, u8); 12] = [
    (9, 23),
    (10, 24),
    (11, 23),
    (12, 24),
    (1, 24),
    (2, 21),
    (3, 24),
    (4, 23),
    (5, 24),
    (6, 23),
    (7, 24),
    (8, 24),
];

/// The months and the seven-day week.
pub const CYCLES: &[CycleShape] = &[
    CycleShape::named(MONTH, &MONTHS),
    CycleShape::fixed(WEEKDAY, 7),
];

/// The first year carried: the year that begins on 23 September AD 4.
pub const MIN_YEAR: i64 = 4;

/// The last year carried.
pub const MAX_YEAR: i64 = 9_999;

/// Where the calendar comes from.
pub const SOURCE: &str = "OGIS 458, lines 50-77, in Dittenberger, Orientis Graeci Inscriptiones \
    Selectae II, 1905, read 2026-09-27 [dittenberger-ogis2]: the year from a.d. IX Kal. Oct., \
    Kaisar the first month, each month from a.d. IX Kal., the twelve months and their days, \
    together 365, and Xandikos of 32 days in a leap year; Bultrighini, \"Calendars of the Greek \
    East under Rome\", 2021, read 2026-09-27 [bultrighini2021], for the same table and the \
    dated equations of Sardis, Metropolis, Acmonia and Pergamon";

/// Whether the Asian year `year` has a 32-day Xandikos: whether the Julian
/// year its Xandikos falls in, `year + 1`, is leap.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    julian::is_leap_year(year + 1)
}

/// The number of days in `month` of `year`, or `None` when `month` is not
/// in `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month < 1 || month > 12 {
        return None;
    }
    let length = MONTH_LENGTHS[(month - 1) as usize];
    if month == LEAP_MONTH && is_leap_year(year) {
        Some(length + 1)
    } else {
        Some(length)
    }
}

/// The first day of `year`, 23 September of the Julian year `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside [`MIN_YEAR`] to
/// [`MAX_YEAR`].
pub const fn new_year_day(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    julian::to_fixed(year, 9, 23)
}

/// The earliest day carried, 23 September AD 4.
pub const EARLIEST: Rd = match new_year_day(MIN_YEAR) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The latest day carried, the last day of [`MAX_YEAR`].
pub const LATEST: Rd = match julian::to_fixed(MAX_YEAR + 1, 9, 22) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of an Asian date. `day` is the day's place in the month,
/// counting the unnumbered days: 1 is Sebaste in a 31-day month.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    let first = match new_year_day(year) {
        Ok(rd) => rd,
        Err(error) => return Err(error),
    };
    let length = match days_in_month(year, month) {
        Some(length) => length,
        None => return Err(CalendarError::MonthOutOfRange),
    };
    if day < 1 || day > length {
        return Err(CalendarError::DayOutOfRange);
    }
    let mut offset = 0i64;
    let mut earlier = 1u8;
    while earlier < month {
        offset += match days_in_month(year, earlier) {
            Some(days) => days as i64,
            None => 0,
        };
        earlier += 1;
    }
    Ok(Rd(first.0 + offset + day as i64 - 1))
}

/// The Asian year, month and place in the month of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before [`EARLIEST`] and
/// [`CalendarError::AfterSupportedRange`] after [`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    let (julian_year, _, _) = match julian::from_fixed(rd) {
        Ok(date) => date,
        Err(error) => return Err(error),
    };
    // The year began on 23 September of this Julian year or of the one
    // before.
    let mut year = julian_year;
    let mut first = match julian::to_fixed(year, 9, 23) {
        Ok(first) => first,
        Err(error) => return Err(error),
    };
    if rd.0 < first.0 {
        year -= 1;
        first = match julian::to_fixed(year, 9, 23) {
            Ok(first) => first,
            Err(error) => return Err(error),
        };
    }
    let mut remaining = rd.0 - first.0;
    let mut month = 1u8;
    while month <= 12 {
        let length = match days_in_month(year, month) {
            Some(length) => length as i64,
            None => 0,
        };
        if remaining < length {
            return Ok((year, month, (remaining + 1) as u8));
        }
        remaining -= length;
        month += 1;
    }
    Err(CalendarError::AfterSupportedRange)
}

/// A day of an Asian month as the calendar writes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrittenDay {
    /// A day before day 1: Sebaste in a 31-day month, and in a leap
    /// Xandikos the first or second of Sebaste and the intercalary day,
    /// whose order the sources read do not settle.
    Unnumbered(u8),
    /// A numbered day, 1 to 30, or to 28 in Dystros.
    Numbered(u8),
}

/// A date of the Asian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsianDate {
    /// The Julian year, AD, in which the Asian year began.
    pub year: i64,
    /// The month, 1 for Kaisar through 12 for Hyperberetaios.
    pub month: u8,
    /// The day's place in the month, counting the unnumbered days first.
    pub day: u8,
}

impl AsianDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        match to_fixed(year, month, day) {
            Ok(_) => Ok(Self { year, month, day }),
            Err(error) => Err(error),
        }
    }

    /// The date whose day is written as `written`.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the month has no such day.
    pub const fn from_written(year: i64, month: u8, written: WrittenDay) -> CalendarResult<Self> {
        let length = match days_in_month(year, month) {
            Some(length) => length,
            None => return Err(CalendarError::MonthOutOfRange),
        };
        let unnumbered = length.saturating_sub(30);
        let day = match written {
            WrittenDay::Unnumbered(place) if place >= 1 && place <= unnumbered => place,
            WrittenDay::Numbered(number) if number >= 1 && number <= length - unnumbered => {
                unnumbered + number
            }
            _ => return Err(CalendarError::DayOutOfRange),
        };
        Self::new(year, month, day)
    }

    /// How the day is written: unnumbered or by its number.
    #[must_use]
    pub const fn written_day(self) -> WrittenDay {
        let unnumbered = match days_in_month(self.year, self.month) {
            Some(length) => length.saturating_sub(30),
            None => 0,
        };
        if self.day <= unnumbered {
            WrittenDay::Unnumbered(self.day)
        } else {
            WrittenDay::Numbered(self.day - unnumbered)
        }
    }

    /// The English transliteration of the month's name.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[(self.month - 1) as usize]
    }
}

/// The calendar of the province of Asia.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AsianCalendar;

impl Calendar for AsianCalendar {
    type Date = AsianDate;

    /// Unrecorded: the decree's year is disputed and the sources read
    /// attest the calendar by single documents, from Sardis in the first
    /// century to a sermon of 387, not by a period of days.
    fn usage(&self) -> Usage {
        Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [CycleShape] {
        CYCLES
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("asian"),
            english_name: "Asian (Roman province of Asia)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["grc"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(AsianDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some() {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        AsianDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn julian(year: i64, month: u8, day: u8) -> Rd {
        julian::to_fixed(year, month, day).unwrap()
    }

    /// The date of a month and a written day in `year`.
    fn written(year: i64, month: u8, written: WrittenDay) -> Rd {
        AsianDate::from_written(year, month, written)
            .and_then(|date| AsianCalendar.to_fixed(date))
            .unwrap()
    }

    #[test]
    fn the_decree_lists_365_days_and_32_for_xandikos_in_a_leap_year() {
        // OGIS 458, 70-72: "together 365"; "Xandikos shall have 32 days".
        let total: u32 = MONTH_LENGTHS.iter().map(|&days| u32::from(days)).sum();
        assert_eq!(total, 365);
        assert_eq!(days_in_month(7, LEAP_MONTH), Some(32)); // Xandikos of AD 8
        assert_eq!(days_in_month(8, LEAP_MONTH), Some(31));
        for year in MIN_YEAR..2_100 {
            let length = new_year_day(year + 1).unwrap().0 - new_year_day(year).unwrap().0;
            assert_eq!(length == 366, is_leap_year(year), "{year}");
            assert!(length == 365 || length == 366);
        }
    }

    #[test]
    fn every_month_begins_on_the_ninth_day_before_the_kalends() {
        // OGIS 458, 76: "the beginning of each month shall be the ninth day
        // before the Kalends". The ninth day before the Kalends of a month
        // is its first day less eight, by the Roman inclusive count, and in
        // February of a leap year still the 21st, because the Roman leap
        // day is a doubled sixth day before the Kalends of March.
        for year in [4, 7, 8, 99, 100, 387, 1_999] {
            for (index, &(month, day)) in MONTH_STARTS.iter().enumerate() {
                let month_number = u8::try_from(index + 1).unwrap();
                let julian_year = if month >= 9 { year } else { year + 1 };
                let first = to_fixed(year, month_number, 1).unwrap();
                assert_eq!(first, julian(julian_year, month, day), "{year} {index}");
                let (next_year, next_month) = if month == 12 {
                    (julian_year + 1, 1)
                } else {
                    (julian_year, month + 1)
                };
                let kalends = julian(next_year, next_month, 1);
                let expected = if next_month == 3 {
                    8 + i64::from(julian::is_leap_year(julian_year))
                } else {
                    8
                };
                assert_eq!(kalends.0 - first.0, expected, "{year} {index}");
            }
        }
    }

    #[test]
    fn the_year_begins_on_augustus_birthday() {
        // OGIS 458, 50-52 and 54-56: the new month from the ninth day
        // before the Kalends of October, "the birthday of Augustus", and
        // Kaisar the first month. Pergamon's hymnodoi (Bultrighini no. 77):
        // the first of Kaisar is the birthday; the penultimate day of
        // Hyperberetaios is the birthday of the Augusta, 21 September.
        assert_eq!(new_year_day(129), Ok(julian(129, 9, 23)));
        assert_eq!(to_fixed(128, 12, 29), Ok(julian(129, 9, 21)));
        assert_eq!(from_fixed(julian(129, 9, 22)), Ok((128, 12, 30)));
        assert_eq!(from_fixed(julian(129, 9, 23)), Ok((129, 1, 1)));
    }

    #[test]
    fn the_dated_equations_of_the_inscriptions_hold() {
        // (Julian month and day, Asian month, written day), each as
        // Bultrighini lists it; the years are not given or do not matter,
        // so a common year and a leap year are both checked.
        for (julian_month, julian_day, month, day, source) in [
            // Metropolis hemerologion, no. 72.
            (10, 7, 1, 14, "Metropolis"),
            (11, 5, 2, 13, "Metropolis"),
            (12, 5, 3, 12, "Metropolis"),
            (1, 5, 4, 12, "Metropolis"),
            // Sardis, no. 71: "the 8th of the month Xandikos, on the
            // Kalends of March".
            (3, 1, 6, 8, "Sardis"),
            // The Hemerologia, nos. 77 and 80 notes: the Kalends of January
            // on 8 Peritios; 31 October on 8 Apellaios.
            (1, 1, 4, 8, "Hemerologia"),
            (10, 31, 2, 8, "Hemerologia"),
            // In sanctum pascha, 387, no. 81: the Epiphany, 6 January, "the
            // thirteenth day of the fourth month according to the Asians".
            (1, 6, 4, 13, "In sanctum pascha"),
        ] {
            for year in [98, 99] {
                let julian_year = if julian_month >= 9 { year } else { year + 1 };
                assert_eq!(
                    written(year, month, WrittenDay::Numbered(day)),
                    julian(julian_year, julian_month, julian_day),
                    "{source} {year}"
                );
            }
        }
        // Year 99's Xandikos is in AD 100, a Julian leap year, and year
        // 98's in 99, a common one; the Kalends of March are 8 Xandikos in
        // both.
        assert!(is_leap_year(99) && !is_leap_year(98));
    }

    #[test]
    fn a_thirty_one_day_month_opens_with_an_unnumbered_day() {
        let kaisar = AsianDate::new(50, 1, 1).unwrap();
        assert_eq!(kaisar.written_day(), WrittenDay::Unnumbered(1));
        assert_eq!(kaisar.month_name(), "Kaisar");
        assert_eq!(
            AsianDate::new(50, 1, 2).unwrap().written_day(),
            WrittenDay::Numbered(1)
        );
        assert_eq!(
            AsianDate::new(50, 1, 31).unwrap().written_day(),
            WrittenDay::Numbered(30)
        );
        // A 30-day month has none.
        assert_eq!(
            AsianDate::new(50, 2, 1).unwrap().written_day(),
            WrittenDay::Numbered(1)
        );
        assert_eq!(
            AsianDate::from_written(50, 2, WrittenDay::Unnumbered(1)),
            Err(CalendarError::DayOutOfRange)
        );
        // Dystros has 28 days, all numbered.
        assert_eq!(written(50, 5, WrittenDay::Numbered(28)), julian(51, 2, 20));
        assert_eq!(
            AsianDate::from_written(50, 5, WrittenDay::Numbered(29)),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn a_leap_xandikos_has_two_unnumbered_days_and_keeps_its_numbers() {
        // Year 99: Xandikos from 21 February 100, a Julian leap year.
        assert_eq!(days_in_month(99, 6), Some(32));
        assert_eq!(
            written(99, 6, WrittenDay::Unnumbered(1)),
            julian(100, 2, 21)
        );
        assert_eq!(
            written(99, 6, WrittenDay::Unnumbered(2)),
            julian(100, 2, 22)
        );
        assert_eq!(written(99, 6, WrittenDay::Numbered(1)), julian(100, 2, 23));
        assert_eq!(written(99, 6, WrittenDay::Numbered(30)), julian(100, 3, 23));
        // In a common year the one unnumbered day is 21 February and day 1
        // is the 22nd; from the Kalends of March on, the numbers agree.
        assert_eq!(written(98, 6, WrittenDay::Numbered(1)), julian(99, 2, 22));
        assert_eq!(
            AsianDate::from_written(98, 6, WrittenDay::Unnumbered(2)),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn acmonia_disagrees_by_a_day_as_bultrighini_notes() {
        // No. 75, AD 85: "the third day before the Nones of March, the
        // thirteenth of Xandikos" — 5 March, which this calendar writes as
        // the twelfth. Laffi and Thonemann, as Bultrighini reports them,
        // read Acmonia as counting a 31-day month "Sebaste, Day 2, Day 3".
        assert_eq!(written(84, 6, WrittenDay::Numbered(12)), julian(85, 3, 5));
        // No. 74, AD 68: 8 April as 17 Artemision, which is 9 April here.
        assert_eq!(written(67, 7, WrittenDay::Numbered(17)), julian(68, 4, 9));
    }

    #[test]
    fn every_day_round_trips_and_the_range_is_refused_outside() {
        let calendar = AsianCalendar;
        let year_starts = (MIN_YEAR..=MAX_YEAR).map(|year| new_year_day(year).unwrap().0);
        for rd in crate::sweep_days(EARLIEST.0, LATEST.0, 97, year_starts) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, None);
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            let again = AsianDate::from_written(date.year, date.month, date.written_day());
            assert_eq!(again, Ok(date), "rd {rd}");
        }
        assert_eq!(from_fixed(EARLIEST), Ok((MIN_YEAR, 1, 1)));
        assert_eq!(EARLIEST, julian(4, 9, 23));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(from_fixed(LATEST), Ok((MAX_YEAR, 12, 30)));
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(3, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(50, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(50, 5, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(50, 1, 0), Err(CalendarError::DayOutOfRange));
        assert_eq!(calendar.is_leap_year(3), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(50, 1, 1).with_era("ad")),
            Err(CalendarError::UnknownEra)
        );
    }
}
