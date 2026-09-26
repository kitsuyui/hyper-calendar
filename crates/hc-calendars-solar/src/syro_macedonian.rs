//! The Julian year of Roman Syria and Palestine under its city eras: the
//! Seleucid era in its Syrian form, the Caesarean era of Antioch from
//! 1 October and from 1 September, and the era of Gaza.
//!
//! When the Greco-Syrians took up the Julian year they wrote it under the
//! Macedonian month names, Audynaios for January through Apellaios for
//! December — the calendar the *hemerologia* call "of the Hellenes", which
//! is the calendar of Antioch (Bultrighini, "Calendars of the Greek East
//! under Rome", `bultrighini2021`, read 2026-09-26) — and every equation
//! read puts each of its months on the Julian month day for day: 1
//! Hyperberetaios on 1 October (P.Dura 30, in Bultrighini), 21 Dystros on
//! 21 March (Theophilus, in Bultrighini), "the fourteenth day of the month
//! Gorpiaeus, which the Romans call September" (Evagrius, *Ecclesiastical
//! History* 2.12, tr. Walford, 1846, `evagrius-walford1846`, read
//! 2026-09-26). An era here is that year with a year number that changes on
//! a fixed Julian day, [`JulianEra::new_year`], and [`ALL`] is the table of
//! them, read by one [`JulianEraCalendar`] as policy §2 asks.
//!
//! * The Seleucid era "fixed the beginning of the year and consequently that
//!   of the era as October 1" (Grumel, "Eras, Historical", New Catholic
//!   Encyclopedia, `grumel-eras-historical`, read 2026-09-26), so year 1
//!   is from 1 October 312 BC and the Zabad inscription's 24 Gorpiaios 823
//!   is 24 September 512 (Wikipedia, "Seleucid era", "Zabad inscription").
//! * Antioch's Caesarean era began "Dios (later Oct.) 1, 49 (Sept. 1,
//!   following the adjustment to the Byzantine indiction, c. 460)" (Grumel).
//!   "c. 460" is not a day, so the two year starts are two identifiers under
//!   policy §5, and Evagrius's dates of 457 to 588 are in the September one.
//! * Gaza's era began "on Oct. 28, 61 b.c., following the introduction of a
//!   fixed year" (Grumel). Its own months are not carried — where its leap
//!   day fell is in no text read — so its days are the Julian days and months
//!   under the Gaza year number, as [`crate::byzantine`] puts the world era
//!   on them.
//!
//! The system document is `docs/systems/seleucid-eras.md` in the
//! repository: the equations, the worked examples, what is not carried and
//! why.
//!
//! # Exactness
//!
//! Exact — the Julian calendar with its year renumbered and, for three of
//! the four, its months renamed. The days before the Greco-Syrians took up
//! the Julian year, which no source read dates, are the arithmetic continued
//! backwards.

use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};

use crate::julian;

/// The months of the calendar of Antioch, January to December.
///
/// The order is the one Bultrighini gives the calendar, "starting with
/// Aydunaios = January", with Gorpiaios "followed by Hyperberetaios, Dios,
/// and Apellaios"; the spellings are Wikipedia's, "Ancient Macedonian
/// calendar" (`wikipedia-ancient-macedonian-calendar`), as
/// [`crate::bostran::MONTHS`] has them, with Audynaios and Loos as
/// Bultrighini's papyri and Evagrius write them.
pub const MONTHS: [&str; 12] = [
    "Audynaios",
    "Peritios",
    "Dystros",
    "Xanthikos",
    "Artemisios",
    "Daisios",
    "Panemos",
    "Loos",
    "Gorpiaios",
    "Hyperberetaios",
    "Dios",
    "Apellaios",
];

/// The Antiochene months and the seven-day week.
pub const ANTIOCHENE: &[CycleShape] = &[
    CycleShape::named(MONTH, &MONTHS),
    CycleShape::fixed(WEEKDAY, 7),
];

/// The latest year of any era this module converts.
pub const MAX_YEAR: i64 = 9_999;

/// An era on the Julian year: an identifier, a new-year day and an offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JulianEra {
    /// The calendar identifier.
    pub id: &'static str,
    /// The English name of the calendar.
    pub english_name: &'static str,
    /// The era code a date carries.
    pub era: &'static str,
    /// The Julian month and day the year number changes on.
    pub new_year: (u8, u8),
    /// The era's year less the astronomical Julian year, for the days on
    /// and after the new-year day; the days before it are one less. Year
    /// *N* begins on the new-year day of the Julian year *N* − `offset`.
    pub offset: i64,
    /// The month names the calendar declares: [`ANTIOCHENE`], or the
    /// Julian months where the era's own are not carried.
    pub cycles: &'static [CycleShape],
    /// When the era was used, and on whose authority.
    pub usage: Usage,
    /// The languages the era's own sources write it in.
    pub native_locales: &'static [&'static str],
    /// Where the era's rule comes from.
    pub source: &'static str,
}

impl JulianEra {
    /// Whether a Julian month and day is on or after the new-year day.
    const fn is_on_or_after_new_year(self, month: u8, day: u8) -> bool {
        month > self.new_year.0 || (month == self.new_year.0 && day >= self.new_year.1)
    }

    /// The astronomical Julian year of a month and day of year `year`.
    #[must_use]
    pub const fn julian_year(self, year: i64, month: u8, day: u8) -> i64 {
        if self.is_on_or_after_new_year(month, day) {
            year - self.offset
        } else {
            year - self.offset + 1
        }
    }

    /// The era's year of a Julian date.
    #[must_use]
    pub const fn year_of(self, julian_year: i64, month: u8, day: u8) -> i64 {
        if self.is_on_or_after_new_year(month, day) {
            julian_year + self.offset
        } else {
            julian_year + self.offset - 1
        }
    }

    /// Whether year `year` holds a 29 February.
    #[must_use]
    pub const fn is_leap_year(self, year: i64) -> bool {
        julian::is_leap_year(self.julian_year(year, 2, 29))
    }

    /// The number of days in `month` of `year`, or `None` when `month` is
    /// not in `1..=12`.
    #[must_use]
    pub const fn days_in_month(self, year: i64, month: u8) -> Option<u8> {
        julian::days_in_month(self.julian_year(year, month, 1), month)
    }

    /// The first day of year `year`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside 1 to [`MAX_YEAR`].
    pub const fn new_year_day(self, year: i64) -> CalendarResult<Rd> {
        self.to_fixed(year, self.new_year.0, self.new_year.1)
    }

    /// The first day of year 1.
    #[must_use]
    pub const fn earliest(self) -> Rd {
        match self.new_year_day(1) {
            Ok(rd) => rd,
            Err(_) => Rd(0),
        }
    }

    /// The last day of year [`MAX_YEAR`].
    #[must_use]
    pub const fn latest(self) -> Rd {
        match julian::to_fixed(
            self.julian_year(MAX_YEAR + 1, self.new_year.0, self.new_year.1),
            self.new_year.0,
            self.new_year.1,
        ) {
            Ok(rd) => Rd(rd.0 - 1),
            Err(_) => Rd(0),
        }
    }

    /// The fixed day of a date of this era.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside 1 to [`MAX_YEAR`],
    /// and [`CalendarError::MonthOutOfRange`] or
    /// [`CalendarError::DayOutOfRange`] as [`crate::julian`] does.
    pub const fn to_fixed(self, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        if year < 1 || year > MAX_YEAR {
            return Err(CalendarError::YearOutOfRange);
        }
        julian::to_fixed(self.julian_year(year, month, day), month, day)
    }

    /// The year, month and day of this era of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] before year 1 and
    /// [`CalendarError::AfterSupportedRange`] after [`MAX_YEAR`].
    pub const fn from_fixed(self, rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        if rd.0 < self.earliest().0 {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd.0 > self.latest().0 {
            return Err(CalendarError::AfterSupportedRange);
        }
        match julian::from_fixed(rd) {
            Ok((year, month, day)) => Ok((self.year_of(year, month, day), month, day)),
            Err(error) => Err(error),
        }
    }
}

/// Where the calendar of Antioch comes from.
pub const ANTIOCH_MONTHS_SOURCE: &str = "Bultrighini, \"Calendars of the Greek East under Rome\", \
    2021, read 2026-09-26 [bultrighini2021]: the calendar of the Hellenes in the hemerologia as \
    the calendar of Antioch, its months from Audynaios = January, and P.Dura 30, P.Euphr. 6, \
    P.Dura 29 and Theophilus's 21 Dystros = 21 March; Evagrius, Ecclesiastical History, tr. \
    Walford, 1846 [evagrius-walford1846], for Gorpiaios as September, Panemos as July, Xanthikos \
    as April, Loos as August, Artemisios as May, Apellaios as December and the last day of \
    Hyperberetaios";

/// The Seleucid era in its Syrian form, from 1 October 312 BC.
pub const SELEUCID_SYRIAN: JulianEra = JulianEra {
    id: "seleucid-syrian",
    english_name: "Seleucid era (Syrian, from 1 October)",
    era: "se",
    new_year: (10, 1),
    offset: 312,
    cycles: ANTIOCHENE,
    // Grumel and Wikipedia date the era's Julian use by the century — from
    // the Greco-Syrians' taking up of the Julian year, which neither dates,
    // to the Syriac chroniclers of the twelfth century and the tombstones
    // of the fourteenth — so no period of days is claimed.
    usage: Usage::UNRECORDED,
    native_locales: &["grc", "syr"],
    source: "Grumel, \"Eras, Historical\", New Catholic Encyclopedia, read 2026-09-26 \
        [grumel-eras-historical]: the era from Dios 1, 312 BC, the Julian year's beginning \
        fixed \"as October 1, and later, c. a.d. 460, as September 1 … but this did not affect \
        the Oriental Syrians\"; Wikipedia, \"Seleucid era\" [wikipedia-seleucid-era] and \
        \"Zabad inscription\" [wikipedia-zabad-inscription], for 24 Gorpiaios 823 = 24 September \
        512",
};

/// The Caesarean era of Antioch with the year from 1 October, from
/// 1 October 49 BC.
pub const ANTIOCH_OCTOBER: JulianEra = JulianEra {
    id: "antioch-caesarean-era",
    english_name: "Caesarean era of Antioch (from 1 October)",
    era: "antioch",
    new_year: (10, 1),
    offset: 49,
    cycles: ANTIOCHENE,
    usage: Usage::UNRECORDED,
    native_locales: &["grc"],
    source: ANTIOCH_SOURCE,
};

/// The Caesarean era of Antioch with the year from 1 September, from
/// 1 September 49 BC.
pub const ANTIOCH_SEPTEMBER: JulianEra = JulianEra {
    id: "antioch-caesarean-era-september",
    english_name: "Caesarean era of Antioch (from 1 September)",
    era: "antioch",
    new_year: (9, 1),
    offset: 49,
    cycles: ANTIOCHENE,
    usage: Usage::UNRECORDED,
    native_locales: &["grc"],
    source: ANTIOCH_SOURCE,
};

/// Where the era of Antioch comes from.
pub const ANTIOCH_SOURCE: &str = "Grumel, \"Eras, Historical\", New Catholic Encyclopedia, read \
    2026-09-26 [grumel-eras-historical]: \"Antioch (the most important of all), Dios (later \
    Oct.) 1, 49 (Sept. 1, following the adjustment to the Byzantine indiction, c. 460)\"; \
    Evagrius, Ecclesiastical History, tr. Walford, 1846 [evagrius-walford1846], for the dates of \
    457 to 588 in the September year";

/// The era of Gaza, from 28 October 61 BC.
pub const GAZA: JulianEra = JulianEra {
    id: "gaza-era",
    english_name: "Era of Gaza",
    era: "gaza",
    new_year: (10, 28),
    offset: 61,
    cycles: hc_calendar::shape::SOLAR_TWELVE,
    // "In use until the 7th century" is a century, not a day.
    usage: Usage::UNRECORDED,
    native_locales: &["grc"],
    source: "Grumel, \"Eras, Historical\", New Catholic Encyclopedia, read 2026-09-26 \
        [grumel-eras-historical]: \"Gaza on Oct. 28, 61 b.c., following the introduction of a \
        fixed year — an era in use until the 7th century\"; the Kissufim mosaic, Loos 636, \
        indiction 9, 4 August 576 [csla-e03137]",
};

/// Every era in this module, in registry order.
pub const ALL: &[JulianEra] = &[SELEUCID_SYRIAN, ANTIOCH_OCTOBER, ANTIOCH_SEPTEMBER, GAZA];

/// A date of an era on the Julian year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JulianEraDate {
    /// The year of the era, from 1.
    pub year: i64,
    /// The month, 1 for January (Audynaios) through 12 for December
    /// (Apellaios), whichever month the year begins in.
    pub month: u8,
    /// The day of the month, from 1.
    pub day: u8,
}

/// An era on the Julian year as a calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JulianEraCalendar(pub JulianEra);

impl Calendar for JulianEraCalendar {
    type Date = JulianEraDate;

    /// The sources date the eras' use by the century, never the day.
    fn usage(&self) -> Usage {
        self.0.usage
    }

    fn cycles(&self) -> &'static [CycleShape] {
        self.0.cycles
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(1..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self.0.is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(self.0.id),
            english_name: self.0.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(self.0.earliest()),
            latest: Some(self.0.latest()),
            native_locales: self.0.native_locales,
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.0.to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = self.0.from_fixed(rd)?;
        Ok(JulianEraDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(self.0.era))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != self.0.era) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        self.0.to_fixed(fields.year, month.ordinal, day)?;
        Ok(JulianEraDate {
            year: fields.year,
            month: month.ordinal,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::byzantine;

    fn julian(year: i64, month: u8, day: u8) -> Rd {
        julian::to_fixed(year, month, day).unwrap()
    }

    /// The index of a month name in [`MONTHS`], as a month ordinal.
    fn month(name: &str) -> u8 {
        let index = MONTHS.iter().position(|month| *month == name).unwrap();
        u8::try_from(index + 1).unwrap()
    }

    #[test]
    fn the_antiochene_months_are_the_julian_months_the_sources_equate() {
        // (Macedonian month, day, Julian month, day), each as the source
        // equates them; the year does not enter.
        for (name, day, julian_month, julian_day) in [
            ("Hyperberetaios", 1, 10, 1),   // P.Dura 30, "the calends of October"
            ("Dios", 6, 11, 6), // P.Euphr. 6, "the eighth day before the ides of November"
            ("Hyperberetaios", 2, 10, 2), // P.Dura 29, "the 6th day before the nones of October"
            ("Dystros", 21, 3, 21), // Theophilus, the equinox
            ("Gorpiaios", 14, 9, 14), // Evagrius 2.12, "which the Romans call September"
            ("Panemos", 9, 7, 9), // Evagrius 4.1, "which the Romans call July"
            ("Xanthikos", 1, 4, 1), // Evagrius 4.9, "Xanthicus, or April"
            ("Loos", 1, 8, 1),  // Evagrius 4.9, "Lous, or August"
            ("Artemisios", 29, 5, 29), // Evagrius 4.5, "Artemisius or May"
            ("Apellaios", 9, 12, 9), // Evagrius 4.19, "called by the Latins December"
            ("Hyperberetaios", 31, 10, 31), // Evagrius 6.8, "the last day"
            ("Audynaios", 1, 1, 1), // Bultrighini, "Aydunaios = January"
        ] {
            assert_eq!((month(name), day), (julian_month, julian_day), "{name}");
        }
        // And every month is the Julian month's length, so Peritios holds
        // the leap day: 21 Dystros is 21 March in every year.
        for year in 1..=40 {
            let dystros = SELEUCID_SYRIAN
                .to_fixed(year, month("Dystros"), 21)
                .unwrap();
            let (_, julian_month, julian_day) = julian::from_fixed(dystros).unwrap();
            assert_eq!((julian_month, julian_day), (3, 21), "SE {year}");
        }
    }

    #[test]
    fn the_zabad_inscription_is_twenty_four_gorpiaios_823() {
        // "In the year 823 on the 24th day of month Gorpiaios"; "24
        // September, 512 AD" (Wikipedia, "Zabad inscription", "Seleucid
        // era"). September is before the October new year, so AD + 311.
        assert_eq!(
            SELEUCID_SYRIAN.to_fixed(823, month("Gorpiaios"), 24),
            Ok(julian(512, 9, 24))
        );
        // Year 1 from 1 October 312 BC, astronomical -311 (Grumel); 823
        // from 1 October 511.
        assert_eq!(SELEUCID_SYRIAN.earliest(), julian(-311, 10, 1));
        assert_eq!(SELEUCID_SYRIAN.new_year_day(823), Ok(julian(511, 10, 1)));
        assert_eq!(
            SELEUCID_SYRIAN.from_fixed(julian(511, 9, 30)),
            Ok((822, 9, 30))
        );
    }

    #[test]
    fn evagrius_dates_the_earthquake_under_leo_by_the_september_year() {
        // Evagrius 2.12: "the five hundred and sixth year …, on the
        // fourteenth day of the month Gorpiaeus, which the Romans call
        // September, on the eve of the Lord's day, in the eleventh cycle of
        // the indiction".
        let day = ANTIOCH_SEPTEMBER
            .to_fixed(506, month("Gorpiaios"), 14)
            .unwrap();
        assert_eq!(day, julian(457, 9, 14));
        // A Saturday: Rd 1 is a Monday.
        assert_eq!(day.0.rem_euclid(7), 6);
        let (world_year, _, _) = byzantine::from_fixed(day).unwrap();
        assert_eq!(byzantine::indiction(world_year), 11);
        // The October year puts the same number a year later, on a Sunday
        // in the twelfth indiction, which is not the date.
        let october = ANTIOCH_OCTOBER
            .to_fixed(506, month("Gorpiaios"), 14)
            .unwrap();
        assert_eq!(october, julian(458, 9, 14));
        assert_eq!(october.0.rem_euclid(7), 0);
    }

    #[test]
    fn evagrius_dates_the_sixth_century_by_the_september_year() {
        // (year, month, day, Julian date, whether the October year agrees).
        for (year, name, day, expected, october_agrees) in [
            // 4.1: Justin "in the five hundred and sixty-sixth year of the
            // Era of Antioch, on the ninth day of the month Panemus".
            (566, "Panemos", 9, (518, 7, 9), true),
            // 4.4: Severus flees "in the month Gorpiaeus … in the five
            // hundred and sixty-seventh year".
            (567, "Gorpiaios", 1, (518, 9, 1), false),
            // 4.9: Justinian "proclaimed on the first of the month
            // Xanthicus, or April, in the five hundred and seventy-fifth
            // year".
            (575, "Xanthikos", 1, (527, 4, 1), true),
            // 4.9: Justin dies "on the first of the month Lous", four
            // months later, so in the same year.
            (575, "Loos", 1, (527, 8, 1), true),
            // 6.8: "the six hundred and thirty-seventh year of the era of
            // Theopolis … on the last day of the month Hyperberetaeus".
            (637, "Hyperberetaios", 31, (588, 10, 31), true),
        ] {
            let (julian_year, julian_month, julian_day) = expected;
            let rd = julian(julian_year, julian_month, julian_day);
            assert_eq!(
                ANTIOCH_SEPTEMBER.to_fixed(year, month(name), day),
                Ok(rd),
                "{year} {name}"
            );
            assert_eq!(
                ANTIOCH_OCTOBER.to_fixed(year, month(name), day) == Ok(rd),
                october_agrees,
                "{year} {name}"
            );
        }
        // 3.33: Severus's accession "in the five hundred and sixty-first
        // year …, in the month Dius, the sixth year of the Indiction":
        // November 512 in both years, in the sixth indiction.
        for era in [ANTIOCH_SEPTEMBER, ANTIOCH_OCTOBER] {
            let first = era.to_fixed(561, month("Dios"), 1).unwrap();
            assert_eq!(first, julian(512, 11, 1), "{}", era.id);
            let (world_year, _, _) = byzantine::from_fixed(first).unwrap();
            assert_eq!(byzantine::indiction(world_year), 6);
        }
    }

    #[test]
    fn the_october_year_of_antioch_begins_in_49_bc() {
        // Grumel: "Dios (later Oct.) 1, 49"; 49 BC is astronomical -48.
        // In the Julian year 1 October is 1 Hyperberetaios (P.Dura 30).
        assert_eq!(ANTIOCH_OCTOBER.earliest(), julian(-48, 10, 1));
        assert_eq!(ANTIOCH_SEPTEMBER.earliest(), julian(-48, 9, 1));
        assert_eq!(
            ANTIOCH_OCTOBER.from_fixed(julian(-48, 10, 1)),
            Ok((1, month("Hyperberetaios"), 1))
        );
        // The two agree from October to August and differ in September.
        assert_eq!(
            ANTIOCH_OCTOBER.from_fixed(julian(457, 8, 31)),
            ANTIOCH_SEPTEMBER.from_fixed(julian(457, 8, 31))
        );
        assert_eq!(
            ANTIOCH_OCTOBER.from_fixed(julian(457, 9, 1)),
            Ok((505, 9, 1))
        );
        assert_eq!(
            ANTIOCH_SEPTEMBER.from_fixed(julian(457, 9, 1)),
            Ok((506, 9, 1))
        );
    }

    #[test]
    fn the_kissufim_mosaic_is_in_gaza_636() {
        // "In the month of Loos, on the 11th (day), year 636, indiction 9",
        // 4 August 576 (csla-e03137). The Gaza day of the month is not
        // carried, so the test is of the year.
        let day = julian(576, 8, 4);
        assert_eq!(GAZA.from_fixed(day), Ok((636, 8, 4)));
        let (world_year, _, _) = byzantine::from_fixed(day).unwrap();
        assert_eq!(byzantine::indiction(world_year), 9);
        // Year 1 from 28 October 61 BC, astronomical -60 (Grumel); 636
        // from 28 October 575.
        assert_eq!(GAZA.earliest(), julian(-60, 10, 28));
        assert_eq!(GAZA.new_year_day(636), Ok(julian(575, 10, 28)));
        assert_eq!(GAZA.from_fixed(julian(575, 10, 27)), Ok((635, 10, 27)));
    }

    #[test]
    fn every_era_round_trips_and_starts_at_year_one() {
        for era in ALL {
            let calendar = JulianEraCalendar(*era);
            let meta = calendar.meta();
            let (first, last) = (meta.earliest.unwrap(), meta.latest.unwrap());
            assert_eq!(
                era.from_fixed(first).map(|date| date.0),
                Ok(1),
                "{}",
                era.id
            );
            assert_eq!(
                era.from_fixed(Rd(first.0 - 1)),
                Err(CalendarError::BeforeEpoch)
            );
            assert_eq!(
                era.from_fixed(last).map(|date| date.0),
                Ok(MAX_YEAR),
                "{}",
                era.id
            );
            assert_eq!(
                era.from_fixed(Rd(last.0 + 1)),
                Err(CalendarError::AfterSupportedRange)
            );
            assert_eq!(era.to_fixed(0, 12, 31), Err(CalendarError::YearOutOfRange));
            assert_eq!(
                era.to_fixed(MAX_YEAR + 1, 1, 1),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(era.to_fixed(1, 13, 1), Err(CalendarError::MonthOutOfRange));
            for rd in (first.0..=last.0).step_by(97) {
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{} {rd}", era.id);
                let fields = calendar.to_fields(date).unwrap();
                assert_eq!(fields.era, Some(era.era));
                assert_eq!(calendar.from_fields(&fields), Ok(date), "{} {rd}", era.id);
                assert_eq!(
                    era.days_in_month(date.year, date.month),
                    julian::days_in_month(julian::from_fixed(Rd(rd)).unwrap().0, date.month)
                );
            }
            // A year is 365 or 366 days, and 366 exactly when it holds a
            // 29 February.
            for year in 1..=200 {
                let length =
                    era.new_year_day(year + 1).unwrap().0 - era.new_year_day(year).unwrap().0;
                assert_eq!(length == 366, era.is_leap_year(year), "{} {year}", era.id);
                assert!(length == 365 || length == 366);
            }
            assert_eq!(calendar.is_leap_year(0), Err(CalendarError::YearOutOfRange));
            assert_eq!(
                calendar.from_fields(&DateFields::ymd(100, 1, 1).with_era("ad")),
                Err(CalendarError::UnknownEra)
            );
        }
    }

    #[test]
    fn the_identifiers_are_distinct_and_the_era_codes_shared_only_by_antioch() {
        for (index, era) in ALL.iter().enumerate() {
            for other in &ALL[index + 1..] {
                assert_ne!(era.id, other.id);
                assert_eq!(
                    era.era == other.era,
                    era.id.starts_with("antioch") && other.id.starts_with("antioch")
                );
            }
        }
    }
}
