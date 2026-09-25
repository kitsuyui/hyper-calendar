//! The Berber (Amazigh) agrarian calendar.
//!
//! The Julian calendar as the countryside of the Maghreb still keeps it for
//! the farming year: the same twelve months as the Julian calendar under
//! Latin-derived names — *Yennayer* from *Januarius*, *Furar* from
//! *Februarius* — the same leap day every fourth year without exception,
//! and so, in this century, thirteen days behind the Gregorian calendar.
//! 1 Yennayer, the Berber new year, is Julian 1 January, which is Gregorian
//! 14 January from 1901 to 2100. The Encyclopédie berbère's entry on the
//! calendar says the farmers, "peu lettrés", mostly compute it by shifting
//! the Gregorian date by thirteen days, and this module does the same in
//! the other direction: every date is the Julian date under another name.
//!
//! # The year number
//!
//! The agrarian calendar had no era; years were named, not counted. The
//! "Amazigh era" that counts from 950 BC — the accession of Shoshenq I,
//! the Libyan founder of Egypt's twenty-second dynasty, taken as the first
//! Berber known by name — is a construction of the 1960s in the Paris
//! *Académie berbère*, attributed to Ammar Negadi, and the first calendar
//! bearing the year appeared in 1980, dated 2930. The Encyclopédie berbère
//! calls it "une création très récente" and "un acquis récent, mais
//! désormais largement diffusé": it is now printed on every Yennayer
//! greeting, so this module carries it as the year number — the Julian
//! year plus [`YEAR_OFFSET`], 2976 for the year that began on 14 January
//! 2026 — and says here what it is. [`usage`](Calendar::usage) begins on
//! 1 Yennayer 2930, the first year anyone wrote the number; earlier years
//! are [`hc_calendar::Standing::Proleptic`].
//!
//! # Which January
//!
//! Where the traditional calendar is kept, Yennayer is Julian 1 January,
//! Gregorian 14 January: Genevois notes the thirteen-day lag, and a
//! Tunisian calendar page reproduced on Wikipedia prints 1 Yennayer
//! against 14 January. Algeria's public holiday, and much of the Algerian
//! and diaspora celebration since the cultural associations of the 1960s,
//! is on 12 January — two days early, which Wikipedia attributes to a
//! mistake of those associations and which was noted at Oran as early as
//! 1950. This module carries the calendar's own new year, 14 January; the
//! 12 January observance is a statutory holiday, not a calendar, and is
//! `hc-holiday`'s Algeria table (law 18-12 of 2 July 2018), which does not
//! read this module. Wikipedia also states, without a source, that the
//! leap day is "usually" added at the end of the year rather than of
//! February; nothing read supports it, and the Julian February is kept.
//!
//! The month names are declared with the shape as [`MONTHS`], in the
//! Kabyle forms Wikipedia tabulates; the Riffian, Shilha, Shawiya, Mozabite
//! and Maghrebi Arabic forms differ in spelling and belong to a locale.
//!
//! # Sources
//!
//! * Jean-Pierre Laporte, "Sheshonq Ier et le « calendrier berbère »",
//!   *Encyclopédie berbère* 42 (2019), pp. 7355–7358,
//!   `journals.openedition.org/encyclopedieberbere/4362`, retrieved
//!   2026-09-25: the origin of the era in the Académie berbère, Negadi,
//!   the first calendar of 1980 dated 2930, and that the month names are
//!   the Julian ones.
//! * E. B., M. Gast and J. Delheure, "Calendrier", *Encyclopédie berbère*
//!   11 (1992), pp. 1713–1720,
//!   `journals.openedition.org/encyclopedieberbere/2039`, retrieved
//!   2026-09-25: the Julian calendar as the farmers' calendar, computed by
//!   shifting the Gregorian date thirteen days.
//! * Wikipedia, "Berber calendar", retrieved 2026-09-25: the table of
//!   month names by dialect, the era formula, 14 January and the 12 January
//!   departure; it cites H. Genevois, "Le calendrier agraire", *Le Fichier
//!   périodique* 125 (1975), and J. Servier, *Les Portes de l'année*
//!   (1962), which were not read.
//!
//! The system document is `docs/systems/berber.md`.
//!
//! # Exactness
//!
//! Exact — arithmetic, the Julian calendar under another year number.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};

use crate::julian;

/// The calendar identifier.
pub const ID: &str = "berber";

/// What the Amazigh era adds to the Julian year: year 1 is 950 BC, and
/// 1980 was 2930.
pub const YEAR_OFFSET: i64 = 950;

/// The first year the era was printed: the calendar of 1980.
pub const FIRST_PRINTED_YEAR: i64 = 2930;

/// The earliest year this implementation converts: the era's first.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 999_999;

/// The twelve months, in the Kabyle forms.
pub const MONTHS: [&str; 12] = [
    "Yennayer",
    "Furar",
    "Meɣres",
    "Yebrir",
    "Mayyu",
    "Yunyu",
    "Yulyu",
    "Ɣuct",
    "Ctembeṛ",
    "Tubeṛ",
    "Wambeṛ",
    "Duǧembeṛ",
];

/// Whether `year` has a 29 Furar: the Julian rule on the Julian year.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    julian::is_leap_year(year - YEAR_OFFSET)
}

/// The number of days in `month` of `year`, or `None` when `month` is not
/// in `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    julian::days_in_month(year - YEAR_OFFSET, month)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    julian::days_in_year(year - YEAR_OFFSET)
}

/// The fixed day of 1 Yennayer of `year`, the Berber new year.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    to_fixed(year, 1, 1)
}

/// The earliest fixed day this implementation converts: 1 Yennayer 1.
pub const EARLIEST: Rd = match julian::to_fixed(MIN_YEAR - YEAR_OFFSET, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = match julian::to_fixed(MAX_YEAR - YEAR_OFFSET, 12, 31) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Jean-Pierre Laporte, \"Sheshonq Ier et le « calendrier berbère »\", \
    *Encyclopédie berbère* 42 (2019), retrieved 2026-09-25: the first calendar printed in the \
    era, in 1980, dated 2930";

/// The day the year number was first printed: 1 Yennayer 2930, Gregorian
/// 14 January 1980.
pub const FIRST_PRINTED: Rd = match julian::to_fixed(FIRST_PRINTED_YEAR - YEAR_OFFSET, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of a Berber date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    julian::to_fixed(year - YEAR_OFFSET, month, day)
}

/// The Berber year, month and day of a fixed day.
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
    match julian::from_fixed(rd) {
        Err(error) => Err(error),
        Ok((year, month, day)) => Ok((year + YEAR_OFFSET, month, day)),
    }
}

/// A date in the Berber agrarian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BerberDate {
    /// The year of the Amazigh era, counting from 1 in 950 BC.
    pub year: i64,
    /// The month, 1 for Yennayer through 12 for Duǧembeṛ.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl BerberDate {
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

    /// The Julian year this date falls in, in astronomical numbering.
    #[must_use]
    pub const fn julian_year(self) -> i64 {
        self.year - YEAR_OFFSET
    }

    /// The name of the month.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }

    /// Whether this is 1 Yennayer, the new year.
    #[must_use]
    pub const fn is_yennayer(self) -> bool {
        self.month == 1 && self.day == 1
    }
}

/// The Berber agrarian calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BerberCalendar;

/// Twelve named months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for BerberCalendar {
    type Date = BerberDate;

    /// The Julian rule, on the Julian year the Berber year names.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    /// The Julian months under their Berber names, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// The year number has been written since the calendar of 1980; the
    /// months are far older, but a year before 2930 is one nobody dated.
    fn usage(&self) -> Usage {
        Usage::since(FIRST_PRINTED, USAGE_SOURCE)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Berber (Amazigh agrarian)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(BerberDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("julian-year", date.julian_year())
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        BerberDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;
    use hc_calendar::Standing;

    #[test]
    fn yennayer_is_julian_new_year_and_gregorian_14_january_this_century() {
        // Wikipedia's Tunisian calendar page: 1 Yennayer against 14 January;
        // Genevois 1975: a thirteen-day lag at present.
        assert_eq!(new_year(2976), gregorian::to_fixed(2026, 1, 14));
        assert_eq!(new_year(2976), julian::to_fixed(2026, 1, 1));
        for year in 1901..=2100 {
            assert_eq!(
                new_year(year + YEAR_OFFSET),
                gregorian::to_fixed(year, 1, 14),
                "{year}"
            );
        }
        // Twelve days until the Gregorian calendar skipped 29 February
        // 1900, fourteen once it skips 29 February 2100: the lag is the
        // Julian calendar's, not a fixture.
        assert_eq!(
            new_year(1900 + YEAR_OFFSET),
            gregorian::to_fixed(1900, 1, 13)
        );
        assert_eq!(
            new_year(2101 + YEAR_OFFSET),
            gregorian::to_fixed(2101, 1, 15)
        );
    }

    #[test]
    fn the_era_counts_from_950_bc_and_was_first_printed_as_2930_in_1980() {
        // Laporte 2019: the first Berber calendar, of 1980, bore 2930.
        assert_eq!(FIRST_PRINTED_YEAR - YEAR_OFFSET, 1980);
        assert_eq!(Ok(FIRST_PRINTED), gregorian::to_fixed(1980, 1, 14));
        // The National, 12 January 2018: "Happy 2968".
        assert_eq!(
            from_fixed(gregorian::to_fixed(2018, 1, 14).unwrap()),
            Ok((2968, 1, 1))
        );
        // Year 1 is 950 BC, astronomical -949.
        assert_eq!(BerberDate::new(1, 1, 1).unwrap().julian_year(), -949);
        assert_eq!(julian::from_fixed(EARLIEST), Ok((-949, 1, 1)));
        assert_eq!(
            BerberCalendar.standing(Rd(FIRST_PRINTED.0 - 1)),
            Standing::Proleptic
        );
        assert_eq!(BerberCalendar.standing(FIRST_PRINTED), Standing::InUse);
    }

    #[test]
    fn every_date_is_the_julian_date_under_another_year_number() {
        for rd in (-300_000..=900_000).step_by(211) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(
                julian::from_fixed(Rd(rd)),
                Ok((year - YEAR_OFFSET, month, day)),
                "rd {rd}"
            );
        }
    }

    #[test]
    fn the_months_are_the_julian_ones_under_latin_derived_names() {
        assert_eq!(MONTHS[0], "Yennayer");
        assert_eq!(MONTHS[1], "Furar");
        assert_eq!(MONTHS[7], "Ɣuct");
        assert_eq!(days_in_month(2976, 2), Some(28));
        assert_eq!(days_in_month(2974, 2), Some(29)); // Julian 2024
        assert!(is_leap_year(2974));
        assert!(is_leap_year(2850)); // Julian 1900: no century exception
        assert!(!is_leap_year(2976));
        assert_eq!(days_in_year(2974), 366);
        assert_eq!(days_in_month(2976, 13), None);
        let date = BerberDate::new(2976, 1, 1).unwrap();
        assert!(date.is_yennayer());
        assert_eq!(date.month_name(), "Yennayer");
        assert!(!BerberDate::new(2976, 1, 2).unwrap().is_yennayer());
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = BerberCalendar;
        for rd in (-300_000..=1_000_000).step_by(1_009) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
            assert_eq!(fields.extra.get("julian-year"), Some(date.julian_year()));
        }
        assert_eq!(calendar.meta().id, CalendarId(ID));
        assert_eq!(calendar.meta().year_kind, YearKind::EpochForward);
        assert_eq!(calendar.usage(), Usage::since(FIRST_PRINTED, USAGE_SOURCE));
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = to_fixed(2972, 1, 1).unwrap().0;
        let end = to_fixed(2980, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn dates_outside_the_supported_range_are_refused() {
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(to_fixed(2976, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(2976, 2, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            BerberCalendar.from_fields(&DateFields::ymd_leap_month(2976, 4, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
    }
}
