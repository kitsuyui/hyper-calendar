//! The Mandaean calendar.
//!
//! A year of exactly 365 days, never intercalated: twelve months of thirty
//! days named for the signs of the zodiac, from *Daula* (Aquarius) to
//! *Gadia* (Capricorn), and the five *Parwanaia* — the *Panja*, the
//! intercalary days of the great baptismal feast — which fall not at the
//! end of the year but between the 30th of the eighth month, *Shumbulta*,
//! and the 1st of the ninth, *Qaina*. It is the wandering year of the
//! Sasanian civil calendar, which Taqizadeh and Stern call its only true
//! continuation, and Häberl finds unchanged since 472 CE at the latest.
//! With no leap day the year slips a day against the Gregorian calendar
//! at every Gregorian leap year: Drower found the new year, *Dehwa Rabba*,
//! on 8 August in 1934 and 1935, "in the midst of the summer heat, Qam
//! Daula the First of Winter"; Häberl has it on 18 July from 2016 to 2019
//! and 17 July from 2020 to 2023.
//!
//! # The shape, and the year number
//!
//! The Parwanaia are carried as a thirteenth named position of the month
//! cycle, in their place: positions 1 to 8 are the first eight months,
//! position [`PARWANAIA`] is the five days, and positions 10 to 13 are the
//! calendar's own ninth to twelfth months, so that Qaina, which Mandaeans
//! count as the ninth month, is position 10 here.
//! [`MandaeanDate::traditional_month`] gives the calendar's own number.
//! The months also bear Babylonian-derived names, Shabat to Tabit
//! ([`BABYLONIAN_NAMES`]), which Drower notes "do not correspond in season
//! to their Jewish or Turkish namesakes"; and the year itself is named for
//! the weekday it began on — the year that opened on Wednesday 18 July 2018
//! is the Year of Wednesday — which [`year_weekday`] gives.
//!
//! Years are numbered here from the creation of Adam, the era of the
//! *Ginza Rabba*'s chronology, as Häberl works it out: the new year of
//! 18 July 2019 was 1 Daula 481,343 AA, so [`EPOCH`] is that day less
//! 481,342 years of 365 days, in February of 479,004 BC by the proleptic
//! Gregorian calendar. That is a scholar's reckoning of a scriptural era,
//! not a number Mandaeans write on documents: the manuscript colophons
//! Häberl reads date by the weekday-name of the year and the Hijri year,
//! and Drower's informants named years the same way. The era is carried
//! because a calendar needs a year number and this is the only published
//! one; [`ERA`] marks it.
//!
//! # Sources
//!
//! * E. S. Drower, *The Mandaeans of Iraq and Iran* (Oxford: Clarendon
//!   Press, 1937), read in the archive.org text: pp. 83–85, the year of
//!   twelve thirty-day months with the five Parwanaia "between the 30th day
//!   of Shumbulta and the 1st day of Qaina", the month table with the
//!   zodiacal and Babylonian names, 29 January 1935 as "the 25th of Sartana
//!   ... the sixth month of the year of Arba Habshaba" (Wednesday), and the
//!   new year on 8 August 1935; p. 82, a birth "in Awwal Gita, 1935, on
//!   February 4th"; p. 90, Panja on 5 April in 1932–1935 and 4 April in
//!   1936; p. 92, Petermann's dates of 1854. The month names here are her
//!   forms.
//! * Charles G. Häberl, "Of Calendars—and Kings—and Why the Winter is
//!   Boiling Hot", *Journal of the Royal Asiatic Society* 31.3 (2021),
//!   pp. 535–544, read in the author's proof at
//!   `amesall.rutgers.edu/images/Publications/calendar-proof.pdf`,
//!   retrieved 2026-09-25: the 18 and 17 July new years of 2016–2023, the
//!   Year of Wednesday from 18 July 2018, the era of 480,000 years after
//!   Adam, and 18 July 2019 as 1 Šabat/Dawla AA 481,343.
//!
//! The system document is `docs/systems/mandaean.md`.
//!
//! # Exactness
//!
//! Exact — arithmetic; every year is 365 days.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::{common, gregorian};

/// The calendar identifier.
pub const ID: &str = "mandaean";

/// The era code: years after the creation of Adam.
pub const ERA: &str = "AA";

/// The length of every Mandaean year, without exception.
pub const DAYS_IN_YEAR: u16 = 365;

/// The position of the five Parwanaia in the month cycle, after the
/// eighth month.
pub const PARWANAIA: u8 = 9;

/// The thirteen positions of the month cycle: the twelve zodiacal months
/// in Drower's forms, with the Parwanaia ninth.
pub const MONTHS: [&str; 13] = [
    "Daula",
    "Nuna",
    "Umbara",
    "Taura",
    "Silmia",
    "Sartana",
    "Aria",
    "Shumbulta",
    "Parwanaia",
    "Qaina",
    "Arqba",
    "Hatia",
    "Gadia",
];

/// The Babylonian-derived names of the twelve months, in Drower's forms,
/// in the calendar's own order; the Parwanaia have none.
pub const BABYLONIAN_NAMES: [&str; 12] = [
    "Shabat", "Adar", "Nisan", "Ayar", "Siwan", "Tammuz", "Ab", "Ellul", "Tishrin", "Mashrwan",
    "Kanun", "Tabit",
];

/// The year Häberl gives for the new year of 18 July 2019.
pub const ANCHOR_YEAR: i64 = 481_343;

/// The fixed day of 1 Daula [`ANCHOR_YEAR`]: 18 July 2019.
pub const ANCHOR: Rd = match gregorian::to_fixed(2019, 7, 18) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of 1 Daula 1 AA, the creation of Adam by the *Ginza*'s
/// chronology: [`ANCHOR`] less 481,342 years of 365 days.
pub const EPOCH: Rd = Rd(ANCHOR.0 - (ANCHOR_YEAR - 1) * DAYS_IN_YEAR as i64);

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 999_999;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Häberl 2021: the structure faithfully maintained since 472 CE at the latest, the \
    continuation of the Sasanian civil calendar; Drower 1937 and Häberl's 2016–2023 New \
    Year's Days for its use today; nothing read dates its beginning";

/// The earliest fixed day this implementation converts: the epoch.
pub const EARLIEST: Rd = EPOCH;

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(EPOCH.0 + MAX_YEAR * DAYS_IN_YEAR as i64 - 1);

/// The number of days in position `month`, or `None` when `month` is not
/// in `1..=13`: thirty, or five for the Parwanaia.
#[must_use]
pub const fn days_in_month(month: u8) -> Option<u8> {
    match month {
        1..=8 | 10..=13 => Some(30),
        PARWANAIA => Some(5),
        _ => None,
    }
}

/// The zero-based day of the year of a month position and day, assuming
/// both are in range.
const fn day_of_year(month: u8, day: u8) -> i64 {
    let day = day as i64 - 1;
    match month {
        1..=8 => (month as i64 - 1) * 30 + day,
        PARWANAIA => 240 + day,
        _ => 245 + (month as i64 - 10) * 30 + day,
    }
}

/// The fixed day of 1 Daula of `year`, Dehwa Rabba.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    to_fixed(year, 1, 1)
}

/// The fixed day of the first of the Parwanaia of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn parwanaia(year: i64) -> CalendarResult<Rd> {
    to_fixed(year, PARWANAIA, 1)
}

/// The weekday `year` begins on, which is what Mandaeans name the year
/// for: the Year of Wednesday began on Wednesday 18 July 2018.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn year_weekday(year: i64) -> CalendarResult<Weekday> {
    match new_year(year) {
        Err(error) => Err(error),
        Ok(rd) => Ok(Weekday::from_rd(rd)),
    }
}

/// The fixed day of a Mandaean date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    if let Err(error) = common::check_day(day, days_in_month(month)) {
        return Err(error);
    }
    Ok(Rd(EPOCH.0
        + (year - 1) * DAYS_IN_YEAR as i64
        + day_of_year(month, day)))
}

/// The Mandaean year, month position and day of a fixed day.
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
    let elapsed = rd.0 - EPOCH.0;
    let year = elapsed.div_euclid(DAYS_IN_YEAR as i64) + 1;
    let within = elapsed.rem_euclid(DAYS_IN_YEAR as i64);
    let (month, day) = if within < 240 {
        ((within / 30 + 1) as u8, (within % 30 + 1) as u8)
    } else if within < 245 {
        (PARWANAIA, (within - 240 + 1) as u8)
    } else {
        let rest = within - 245;
        ((rest / 30 + 10) as u8, (rest % 30 + 1) as u8)
    };
    Ok((year, month, day))
}

/// A Mandaean date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MandaeanDate {
    /// The year after the creation of Adam, counting from 1.
    pub year: i64,
    /// The position in the month cycle, 1 for Daula through 13 for Gadia,
    /// with the Parwanaia at [`PARWANAIA`].
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl MandaeanDate {
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

    /// Whether this is one of the five Parwanaia.
    #[must_use]
    pub const fn is_parwanaia(self) -> bool {
        self.month == PARWANAIA
    }

    /// Whether this is 1 Daula, Dehwa Rabba.
    #[must_use]
    pub const fn is_dehwa_rabba(self) -> bool {
        self.month == 1 && self.day == 1
    }

    /// The month's number as Mandaeans count it, 1 for Daula through 12
    /// for Gadia, or `None` on the Parwanaia.
    #[must_use]
    pub const fn traditional_month(self) -> Option<u8> {
        match self.month {
            1..=8 => Some(self.month),
            PARWANAIA => None,
            _ => Some(self.month - 1),
        }
    }

    /// The zodiacal name of the month, or "Parwanaia".
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }

    /// The Babylonian-derived name of the month, or `None` on the
    /// Parwanaia.
    #[must_use]
    pub const fn babylonian_name(self) -> Option<&'static str> {
        match self.traditional_month() {
            Some(month) => Some(BABYLONIAN_NAMES[month as usize - 1]),
            None => None,
        }
    }
}

/// The Mandaean calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MandaeanCalendar;

/// Thirteen named positions — the twelve months with the Parwanaia after
/// the eighth — and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for MandaeanCalendar {
    type Date = MandaeanDate;

    /// In use today and unchanged since 472 at the latest; no source read
    /// dates where it began, so the period is undated at that end. The years
    /// after Adam it is numbered in are Häberl's reckoning, and say nothing
    /// about use.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(USAGE_SOURCE)
    }

    /// Never: every year is 365 days and nothing is intercalated.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(false)
    }

    /// Twelve months and the Parwanaia, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Mandaean",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["mid"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(MandaeanDate { year, month, day })
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
        MandaeanDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn haberls_new_years_of_2016_to_2023_and_his_year_number() {
        // "on 18 July from 2016 to 2019 CE and 17 July from 2020 to 2023".
        for year in 2016..=2019 {
            assert_eq!(from_fixed(gregorian(year, 7, 18)).unwrap().1, 1, "{year}");
            assert_eq!(from_fixed(gregorian(year, 7, 18)).unwrap().2, 1, "{year}");
        }
        for year in 2020..=2023 {
            assert_eq!(from_fixed(gregorian(year, 7, 17)).unwrap().1, 1, "{year}");
            assert_eq!(from_fixed(gregorian(year, 7, 17)).unwrap().2, 1, "{year}");
        }
        // 18 July 2019 was 1 Šabat / Dawla AA 481,343.
        assert_eq!(from_fixed(gregorian(2019, 7, 18)), Ok((481_343, 1, 1)));
        assert_eq!(new_year(ANCHOR_YEAR), Ok(ANCHOR));
        // The Year of Wednesday began on Wednesday 18 July 2018.
        assert_eq!(new_year(481_342), Ok(gregorian(2018, 7, 18)));
        assert_eq!(year_weekday(481_342), Ok(Weekday::Wednesday));
        assert_eq!(year_weekday(481_343), Ok(Weekday::Thursday));
        // Wikipedia's 2024 dates, which follow: 16 July, Parwanaya 13–17 March.
        assert_eq!(new_year(481_348), Ok(gregorian(2024, 7, 16)));
        assert_eq!(parwanaia(481_347), Ok(gregorian(2024, 3, 13)));
    }

    #[test]
    fn drowers_dates_of_the_1930s() {
        // The new year on 8 August 1935, and she wrote on 29 January 1935
        // "the 25th of Sartana ... the sixth month of the year of Arba
        // Habshaba", the Year of Wednesday, so that year began on 8 August
        // 1934 as well.
        let year_1934 = from_fixed(gregorian(1934, 8, 8)).unwrap();
        assert_eq!((year_1934.1, year_1934.2), (1, 1));
        assert_eq!(from_fixed(gregorian(1935, 8, 8)).unwrap().1, 1);
        assert_eq!(from_fixed(gregorian(1935, 8, 8)).unwrap().2, 1);
        assert_eq!(from_fixed(gregorian(1935, 1, 29)), Ok((year_1934.0, 6, 25)));
        assert_eq!(year_weekday(year_1934.0), Ok(Weekday::Wednesday));
        assert_eq!(MONTHS[5], "Sartana");
        // Awwal Gita, the first of Aria, on 4 February 1935.
        assert_eq!(from_fixed(gregorian(1935, 2, 4)), Ok((year_1934.0, 7, 1)));
        // Panja on 5 April in 1932 to 1935 and on 4 April in 1936.
        for year in 1932..=1935 {
            let (_, month, day) = from_fixed(gregorian(year, 4, 5)).unwrap();
            assert_eq!((month, day), (PARWANAIA, 1), "{year}");
        }
        let (_, month, day) = from_fixed(gregorian(1936, 4, 4)).unwrap();
        assert_eq!((month, day), (PARWANAIA, 1));
        // Eighty-four years of 365 days from 8 August 1935 to 18 July 2019.
        assert_eq!(year_1934.0 + 1 + 84, ANCHOR_YEAR);
    }

    #[test]
    fn petermanns_1854_dates_differ_by_a_day_from_the_present_reckoning() {
        // Drower quotes Petermann's 1854 record: Awwal Gita on 23 February,
        // Awwal Paiz on 28 May, Awwal Sitwa on 26 August, Awwal Abhar on
        // 24 November. Run back from 1934, the first agrees and the other
        // three are a day later here.
        assert_eq!(from_fixed(gregorian(1854, 2, 23)).unwrap().1, 7);
        assert_eq!(from_fixed(gregorian(1854, 2, 23)).unwrap().2, 1);
        assert_eq!(from_fixed(gregorian(1854, 5, 29)).unwrap().1, 11);
        assert_eq!(from_fixed(gregorian(1854, 5, 29)).unwrap().2, 1);
        assert_eq!(from_fixed(gregorian(1854, 8, 27)).unwrap().1, 1);
        assert_eq!(from_fixed(gregorian(1854, 8, 27)).unwrap().2, 1);
        assert_eq!(from_fixed(gregorian(1854, 11, 25)).unwrap().1, 4);
        assert_eq!(from_fixed(gregorian(1854, 11, 25)).unwrap().2, 1);
    }

    #[test]
    fn the_epoch_is_the_creation_of_adam_in_february_479004_bc() {
        // Häberl: 1 Šabat / Dawla AA 1 is in February 479,004 BCE by the
        // Gregorian calendar, astronomical -479,003.
        let (year, month, _) = gregorian::from_fixed(EPOCH).unwrap();
        assert_eq!((year, month), (-479_003, 2));
        assert_eq!(new_year(1), Ok(EPOCH));
        assert_eq!(EPOCH, EARLIEST);
    }

    #[test]
    fn the_parwanaia_follow_shumbulta_and_the_year_is_always_365_days() {
        assert_eq!(MONTHS[7], "Shumbulta");
        assert_eq!(MONTHS[8], "Parwanaia");
        assert_eq!(MONTHS[9], "Qaina");
        assert_eq!(days_in_month(8), Some(30));
        assert_eq!(days_in_month(PARWANAIA), Some(5));
        assert_eq!(days_in_month(13), Some(30));
        assert_eq!(days_in_month(0), None);
        assert_eq!(days_in_month(14), None);
        let last_of_shumbulta = to_fixed(481_343, 8, 30).unwrap();
        assert_eq!(
            to_fixed(481_343, PARWANAIA, 1),
            Ok(Rd(last_of_shumbulta.0 + 1))
        );
        assert_eq!(to_fixed(481_343, 10, 1), Ok(Rd(last_of_shumbulta.0 + 6)));
        assert_eq!(
            new_year(481_344).unwrap().0 - new_year(481_343).unwrap().0,
            i64::from(DAYS_IN_YEAR)
        );
        let panja = MandaeanDate::new(481_343, PARWANAIA, 3).unwrap();
        assert!(panja.is_parwanaia());
        assert_eq!(panja.traditional_month(), None);
        assert_eq!(panja.babylonian_name(), None);
        assert_eq!(panja.month_name(), "Parwanaia");
        let qaina = MandaeanDate::new(481_343, 10, 1).unwrap();
        assert_eq!(qaina.traditional_month(), Some(9));
        assert_eq!(qaina.babylonian_name(), Some("Tishrin"));
        let daula = MandaeanDate::new(481_343, 1, 1).unwrap();
        assert!(daula.is_dehwa_rabba());
        assert_eq!(daula.babylonian_name(), Some("Shabat"));
        assert_eq!(daula.traditional_month(), Some(1));
        assert!(!qaina.is_dehwa_rabba());
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (-2_000_000..=2_000_000).step_by(89) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_two_years_round_trips_in_order() {
        let start = new_year(481_342).unwrap().0;
        let end = new_year(481_344).unwrap().0;
        let mut previous = None;
        for rd in start..end {
            let date = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(date.0, date.1, date.2), Ok(Rd(rd)), "rd {rd}");
            if let Some(previous) = previous {
                assert!(previous < date, "rd {rd}");
            }
            previous = Some(date);
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = MandaeanCalendar;
        for rd in (-500_000..=1_000_000).step_by(1_009) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId(ID));
        assert_eq!(calendar.meta().year_kind, YearKind::EpochForward);
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(481_343, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn dates_outside_the_supported_range_are_refused() {
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(481_343, 14, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(to_fixed(481_343, 0, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(481_343, 1, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            to_fixed(481_343, PARWANAIA, 6),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(from_fixed(LATEST), Ok((MAX_YEAR, 13, 30)));
        assert_eq!(year_weekday(0), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            MandaeanCalendar.from_fields(&DateFields::ymd_leap_month(481_343, 4, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
    }
}
