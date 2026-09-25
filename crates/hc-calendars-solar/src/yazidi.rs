//! The Yazidi year.
//!
//! Yazidis date their festivals by the "Eastern" calendar — the Julian
//! calendar under the Syriac month names, which Kreyenbroek also calls the
//! Seleucid calendar and which in this century runs thirteen days behind
//! the Gregorian — and their year begins with *Serêsal* (*Sersal*,
//! *Çarşema Sor*, "Red Wednesday"): the first Wednesday of Nisan, the
//! Eastern April, which is the first Wednesday on or after 14 April
//! Gregorian from 1900 to 2099. The year number the community prints for
//! the feast is the Gregorian year plus [`YEAR_OFFSET`], 4750 — 6773 on
//! 19 April 2023, 6774 on 17 April 2024 — the same count as the modern
//! Assyrian calendar's ([`crate::assyrian`]), whose epoch of 4750 BC was a
//! 1950s construction. Nothing read says when Yazidis adopted it, so
//! [`usage`](Calendar::usage) is unrecorded.
//!
//! # What is carried, and what is not
//!
//! A new year fixed by a weekday makes the year 364 or 371 days long, and
//! a month-and-day shape cannot hold it: Eastern 1 to 6 April belong to
//! one year in one year and to the next in another, so a year would carry
//! some dates twice and lack others. This module therefore carries the
//! **year and the day of the year** — [`YazidiDate::day_of_year`] counts
//! from Serêsal — and gives the Eastern date beneath as
//! [`YazidiDate::eastern_date`], which is [`crate::julian`]'s. It declares
//! no month cycle. A table of Yazidi month names was not found in the
//! sources read; Kreyenbroek names only Nisan, "the first month of the
//! Yezidi year", so the months are not carried and the Julian months of
//! the Eastern date are the answer for a feast dated by them.
//!
//! The Seleucid *era* — years from 312 BC — is not the Yazidi year number:
//! the sources apply "Seleucid" to the calendar of months, and every year
//! number read is the 4750 count. Nothing here counts Seleucid years.
//!
//! # Sources
//!
//! * Philip G. Kreyenbroek, *Yezidism: Its Background, Observances and
//!   Textual Tradition* (Lewiston: Edwin Mellen Press, 1995), read in the
//!   archive.org text: p. 151, "The Yezidi New Year (Serêsal) is celebrated
//!   on the first Wednesday of Nisan (April)"; p. 150, "Nisan (April), the
//!   first month of the Yezidi year"; p. 164, note 53, "In calculating the
//!   dates of their festivals Yezidis generally use the Seleucid or
//!   'Eastern' calendar, which in this century is thirteen days behind the
//!   Gregorian or 'Western' one".
//! * Artur Rodziewicz, "The Yezidi Wednesday and the Music of the Spheres",
//!   *Iranian Studies* 53.1–2 (2020), pp. 259–293, abstract read: the
//!   festival "on the first Wednesday of the month of Nisan".
//! * Wikipedia, "Yazidi New Year", retrieved 2026-09-25: "the first
//!   Wednesday on or after 14 April according to the Gregorian calendar",
//!   which it cites from *The Cambridge History of the Kurds* (2021), not
//!   read.
//! * Lazghine Ya'qoube, "Red Wednesday: The Yazidi New Year's Ritual",
//!   nlka.net, 26 June 2023, retrieved 2026-09-25: 19 April 2023 as the
//!   year 6773, "thirteen days after the Gregorian calendar"; Rudaw,
//!   17 April 2024, retrieved 2026-09-25: the feast on that day; Kurdistan
//!   Watch, 16 April 2024: the year 6774; Yezidis International, "Ser Sal
//!   or Charshma Sere Nissana", retrieved 2026-09-25: the calendar "6764
//!   years old" in 2014.
//!
//! The system document is `docs/systems/yazidi.md`.
//!
//! # Exactness
//!
//! Exact — arithmetic on the Julian calendar and the week.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::julian;

/// The calendar identifier.
pub const ID: &str = "yazidi";

/// What the year count adds to the Gregorian year at Serêsal: year 1
/// began in 4750 BC.
pub const YEAR_OFFSET: i64 = 4_750;

/// The earliest year this implementation converts: the count's first.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 999_999;

/// The Julian year that Serêsal of Yazidi `year` falls in.
#[must_use]
pub const fn julian_year_of(year: i64) -> i64 {
    year - YEAR_OFFSET
}

/// The fixed day of Serêsal of `year`: the first Wednesday on or after
/// 1 April of the Julian year [`julian_year_of`] gives.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    new_year_unchecked(year)
}

/// Serêsal without the range check, so that the last year's end can be
/// asked for.
const fn new_year_unchecked(year: i64) -> CalendarResult<Rd> {
    match julian::to_fixed(julian_year_of(year), 4, 1) {
        Err(error) => Err(error),
        Ok(first_of_nisan) => Ok(Weekday::Wednesday.on_or_after(first_of_nisan)),
    }
}

/// The number of days in `year`: 364, or 371 in the years that take a
/// fifty-third week.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn days_in_year(year: i64) -> CalendarResult<u16> {
    match (new_year(year), new_year_unchecked(year + 1)) {
        (Ok(this), Ok(next)) => Ok((next.0 - this.0) as u16),
        (Err(error), _) | (_, Err(error)) => Err(error),
    }
}

/// Whether `year` is one of the long years of 371 days.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    matches!(days_in_year(year), Ok(371))
}

/// The earliest fixed day this implementation converts: Serêsal of year 1.
pub const EARLIEST: Rd = match new_year_unchecked(MIN_YEAR) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The latest fixed day this implementation converts: the day before
/// Serêsal of the year after the last.
pub const LATEST: Rd = match new_year_unchecked(MAX_YEAR + 1) {
    Ok(rd) => Rd(rd.0 - 1),
    Err(_) => Rd(0),
};

/// The fixed day of a Yazidi year and day of the year.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] or
/// [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, day_of_year: u16) -> CalendarResult<Rd> {
    let length = match days_in_year(year) {
        Ok(length) => length,
        Err(error) => return Err(error),
    };
    if day_of_year == 0 || day_of_year > length {
        return Err(CalendarError::DayOutOfRange);
    }
    match new_year(year) {
        Ok(start) => Ok(Rd(start.0 + day_of_year as i64 - 1)),
        Err(error) => Err(error),
    }
}

/// The Yazidi year and day of the year of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u16)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    let julian_year = match julian::from_fixed(rd) {
        Ok((year, _, _)) => year,
        Err(error) => return Err(error),
    };
    // Serêsal is in April, so the day is in the year whose Serêsal fell in
    // this Julian year, unless it precedes it.
    let candidate = julian_year + YEAR_OFFSET;
    let year = match new_year_unchecked(candidate) {
        Ok(sersal) if rd.0 < sersal.0 => candidate - 1,
        Ok(_) => candidate,
        Err(error) => return Err(error),
    };
    match new_year_unchecked(year) {
        Ok(start) => Ok((year, (rd.0 - start.0 + 1) as u16)),
        Err(error) => Err(error),
    }
}

/// A Yazidi date: a year and the day of the year from Serêsal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YazidiDate {
    /// The year, counting from 1 in 4750 BC.
    pub year: i64,
    /// The day of the year, 1 on Serêsal through 364 or 371.
    pub day_of_year: u16,
}

impl YazidiDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, day_of_year: u16) -> CalendarResult<Self> {
        match to_fixed(year, day_of_year) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self { year, day_of_year }),
        }
    }

    /// Whether this is Serêsal, the new year.
    #[must_use]
    pub const fn is_sersal(self) -> bool {
        self.day_of_year == 1
    }

    /// The Eastern (Julian) year, month and day of this date, which is how
    /// the festivals are dated.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn eastern_date(self) -> CalendarResult<(i64, u8, u8)> {
        match to_fixed(self.year, self.day_of_year) {
            Err(error) => Err(error),
            Ok(rd) => julian::from_fixed(rd),
        }
    }
}

/// The Yazidi year.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct YazidiCalendar;

impl Calendar for YazidiCalendar {
    type Date = YazidiDate;

    /// A long year of 371 days, fifty-three weeks from one Serêsal to the
    /// next.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        days_in_year(year).map(|days| days == 371)
    }

    /// The 364 or 371 numbered days of the year, and the seven-day week
    /// the new year is set by; no months, since none are carried.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        const SHAPE: &[hc_calendar::shape::CycleShape] = &[
            hc_calendar::shape::CycleShape::intercalary("day-of-year", 364, 371),
            hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
        ];
        SHAPE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Yazidi",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["ku"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.day_of_year)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, day_of_year) = from_fixed(rd)?;
        Ok(YazidiDate { year, day_of_year })
    }

    /// The year, the day of the year, and the Eastern month and day the
    /// festivals are dated by.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let (_, month, day) = date.eastern_date()?;
        DateFields::new(date.year)
            .with_extra("day-of-year", i64::from(date.day_of_year))?
            .with_extra("eastern-month", i64::from(month))?
            .with_extra("eastern-day", i64::from(day))
    }

    /// The day number lives in an extra field because it does not fit the
    /// `day`-within-`month` shape; it defaults to 1 when absent, so the
    /// object-safe layer can still measure a year. The Eastern fields are
    /// informative and not read back.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let day_of_year = fields.extra.get("day-of-year").unwrap_or(1);
        let day_of_year = u16::try_from(day_of_year).map_err(|_| CalendarError::DayOutOfRange)?;
        YazidiDate::new(fields.year, day_of_year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn sersal_is_the_first_wednesday_of_eastern_nisan() {
        // 19 April 2023 was 6773 (nlka.net); 17 April 2024 was 6774
        // (Rudaw, Kurdistan Watch); 15 April 2026 (timeanddate).
        assert_eq!(new_year(6773), Ok(gregorian(2023, 4, 19)));
        assert_eq!(new_year(6774), Ok(gregorian(2024, 4, 17)));
        assert_eq!(new_year(6776), Ok(gregorian(2026, 4, 15)));
        assert_eq!(from_fixed(gregorian(2023, 4, 19)), Ok((6773, 1)));
        assert_eq!(from_fixed(gregorian(2024, 4, 17)), Ok((6774, 1)));
        // 2014 was 6764 (Yezidis International).
        assert_eq!(from_fixed(gregorian(2014, 4, 16)), Ok((6764, 1)));
        for year in 6773..=6776 {
            let sersal = new_year(year).unwrap();
            assert_eq!(Weekday::from_rd(sersal), Weekday::Wednesday, "{year}");
            let (_, month, day) = julian::from_fixed(sersal).unwrap();
            assert_eq!(month, 4, "{year}");
            assert!(day <= 7, "{year}");
        }
    }

    #[test]
    fn this_century_it_is_the_first_wednesday_on_or_after_14_april_gregorian() {
        for year in 1900..=2099 {
            let expected = Weekday::Wednesday.on_or_after(gregorian(year, 4, 14));
            assert_eq!(new_year(year + YEAR_OFFSET), Ok(expected), "{year}");
        }
        // And on or after 13 April in the nineteenth century, when the
        // Eastern calendar was twelve days behind.
        assert_eq!(
            new_year(1850 + YEAR_OFFSET),
            Ok(Weekday::Wednesday.on_or_after(gregorian(1850, 4, 13)))
        );
    }

    #[test]
    fn the_year_is_364_or_371_days_and_the_day_before_sersal_ends_it() {
        // 6773 ran from 19 April 2023 to 16 April 2024, 364 days, and so
        // did 6774, 6775 and 6776, which ends the day before Wednesday
        // 14 April 2027.
        assert_eq!(days_in_year(6773), Ok(364));
        assert_eq!(days_in_year(6774), Ok(364));
        assert_eq!(new_year(6777), Ok(gregorian(2027, 4, 14)));
        assert_eq!(days_in_year(6776), Ok(364));
        // 14 April 2021 was itself a Wednesday, so 6771 began on it and
        // ran to 20 April 2022, the next Serêsal: 371 days.
        assert_eq!(new_year(6771), Ok(gregorian(2021, 4, 14)));
        assert_eq!(new_year(6772), Ok(gregorian(2022, 4, 20)));
        assert_eq!(days_in_year(6771), Ok(371));
        assert!(is_leap_year(6771));
        assert!(!is_leap_year(6773));
        let mut long = 0;
        for year in 6700..=6800 {
            let length = days_in_year(year).unwrap();
            assert!(length == 364 || length == 371, "{year}: {length}");
            if length == 371 {
                long += 1;
            }
        }
        // 101 years of 365.25 days need 101 × 1.25 ≈ 126 extra days, so
        // eighteen long years, give or take.
        assert!((17..=19).contains(&long), "{long}");
        assert_eq!(
            from_fixed(Rd(new_year(6774).unwrap().0 - 1)),
            Ok((6773, 364))
        );
        assert_eq!(to_fixed(6773, 365), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            to_fixed(6771, 371).unwrap().0 + 1,
            new_year(6772).unwrap().0
        );
    }

    #[test]
    fn the_eastern_date_beneath_is_the_julian_one() {
        let sersal = YazidiDate::new(6774, 1).unwrap();
        assert!(sersal.is_sersal());
        // 17 April 2024 Gregorian is 4 April 2024 Julian.
        assert_eq!(sersal.eastern_date(), Ok((2024, 4, 4)));
        // The Feast of the Assembly, 23 September Eastern, is 6 October
        // Gregorian.
        let assembly = YazidiCalendar.from_fixed(gregorian(2024, 10, 6)).unwrap();
        assert_eq!(assembly.eastern_date(), Ok((2024, 9, 23)));
        assert!(!assembly.is_sersal());
        assert_eq!(assembly.year, 6774);
        let fields = YazidiCalendar.to_fields(assembly).unwrap();
        assert_eq!(fields.extra.get("eastern-month"), Some(9));
        assert_eq!(fields.extra.get("eastern-day"), Some(23));
        assert_eq!(fields.month, None);
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (-1_500_000..=2_000_000).step_by(97) {
            let (year, day_of_year) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, day_of_year), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_decade_round_trips_in_order() {
        let start = new_year(6766).unwrap().0;
        let end = new_year(6777).unwrap().0;
        let mut previous = None;
        for rd in start..end {
            let date = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(date.0, date.1), Ok(Rd(rd)), "rd {rd}");
            if let Some(previous) = previous {
                assert!(previous < date, "rd {rd}");
            }
            previous = Some(date);
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = YazidiCalendar;
        for rd in (-500_000..=1_000_000).step_by(1_009) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId(ID));
        assert_eq!(calendar.meta().year_kind, YearKind::EpochForward);
        assert_eq!(
            calendar.from_fields(&DateFields::new(6774)),
            Ok(YazidiDate::new(6774, 1).unwrap())
        );
    }

    #[test]
    fn dates_outside_the_supported_range_are_refused() {
        assert_eq!(to_fixed(0, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(new_year(MAX_YEAR + 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(days_in_year(0), Err(CalendarError::YearOutOfRange));
        assert!(!is_leap_year(0));
        assert_eq!(to_fixed(6774, 0), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(6774, 372), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(from_fixed(EARLIEST), Ok((MIN_YEAR, 1)));
        assert_eq!(julian::from_fixed(EARLIEST).unwrap().0, -4749);
        assert_eq!(from_fixed(LATEST).unwrap().0, MAX_YEAR);
        assert_eq!(
            YazidiCalendar
                .from_fields(&DateFields::new(6774).with_extra("day-of-year", -1).unwrap()),
            Err(CalendarError::DayOutOfRange)
        );
    }
}
