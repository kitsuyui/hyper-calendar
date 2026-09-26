//! The historical Indian eras over the lunisolar months: the Kārttikādi
//! Vikrama Saṃvat of Gujarat, Śivājī's Rājyābhiṣeka Śaka and the Saptarṣi
//! era of Kashmir, registered as `vikram-samvat-kartikadi`,
//! `rajyabhisheka-saka` and `saptarshi`; and the year arithmetic of the
//! Gupta, Valabhī and Kalachuri eras, which is not registered.
//!
//! The eras are written up in `docs/systems/indian-eras.md` in the
//! repository: what each is, the offsets and opening days from Sewell and
//! Dikshit's Art. 71 and how they follow from the sources' own equations,
//! current and expired years, what is carried and why the three ancient
//! eras are not registered, and the checks. This page states the code's
//! own facts.
//!
//! # What this is
//!
//! An [`EraYear`] is where a year opens ([`YearStart`]) and how far the
//! Śaka year in which it opens is from its number; a [`LunarEra`] is an
//! era year over the amānta months of [`crate::hindu_lunar`] or the
//! pūrṇimānta months of [`crate::hindu_purnimanta`], every date keeping
//! the tithi, fortnight, month and intercalary or repeated day that
//! calendar gives it. A year that opens at the first day of an amānta
//! month numbers its months from that month, as the Vira Nirvana Samvat
//! does; any other numbers them from Chaitra, as the calendar below does.
//!
//! # Why three eras are arithmetic only
//!
//! The Gupta and Valabhī inscriptions run from the year 82 to 945 of the
//! era and the Chedi dates Kielhorn examined from 793 to 934 (Sewell and
//! Dikshit, *The Indian Calendar*, 1896, pp. 42–43, `sewell1896`): the
//! fourth century to the thirteenth. The true lunisolar calendar converts
//! 1700 to 2299, so a registered Gupta calendar would convert no day
//! anyone dated in it. [`GUPTA`], [`VALABHI`] and [`KALACHURI`] give the
//! year of an amānta date and the first day of a year within the range,
//! and [`LunarEra::new`] builds a calendar from any of them for a caller who
//! wants one.

use hc_calendar::shape::{CycleShape, LUNISOLAR_TWELVE};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_solar::julian;

use crate::hindu_lunar::{HinduLunarCalendar, HinduLunarDate};
use crate::hindu_purnimanta::HinduPurnimantaCalendar;
use crate::year_start::YearStart;

/// Which reckoning of the months an era's dates are written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reckoning {
    /// The amānta months, new moon to new moon.
    Amanta,
    /// The pūrṇimānta months, full moon to full moon.
    Purnimanta,
}

/// An era's year count: where its year opens, and the Śaka year in which
/// its year `y` opens, less `y`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EraYear {
    /// The era code, lower-case kebab-case.
    pub era: &'static str,
    /// The tithi of the amānta month at which the year opens.
    pub start: YearStart,
    /// The Śaka year (expired, as [`crate::hindu_lunar`] counts it) in
    /// which the era's year `y` opens, less `y`.
    pub offset: i64,
}

impl EraYear {
    /// The era's year of an amānta date.
    #[must_use]
    pub const fn year_of(self, date: HinduLunarDate) -> i64 {
        self.start.era_year(
            date.year,
            date.month,
            date.leap_month,
            date.day,
            self.offset,
        )
    }

    /// The first day of the era's year `year`, on an amānta calendar.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside the calendar's range.
    pub fn new_year(self, lunar: &HinduLunarCalendar, year: i64) -> CalendarResult<Rd> {
        self.start.new_year(lunar, year, self.offset)
    }
}

/// The Kārttikādi Vikrama year: from Kārttika śukla 1, the Chaitrādi
/// Vikrama year — the Śaka year plus 135 — for Kārttika to Phālguna and one
/// less for Chaitra to Āśvina (Sewell and Dikshit, Art. 74).
pub const KARTTIKADI_VIKRAMA_YEAR: EraYear = EraYear {
    era: "vs",
    start: YearStart::KARTTIKADI,
    offset: -crate::hindu_lunar::VIKRAMA_OFFSET,
};

/// Śivājī's era: year 1 opened at Jyeṣṭha śukla 13 of Śaka 1596 expired,
/// and every year opens at that tithi (Sewell and Dikshit, Art. 71, p. 47).
pub const RAJYABHISHEKA_YEAR: EraYear = EraYear {
    era: "rajyabhisheka-saka",
    start: YearStart::new(3, 13),
    offset: 1_595,
};

/// The Saptarṣi era counted in full: year 1 is Kali 27 current (Sewell and
/// Dikshit, Art. 71, p. 41), the Śaka year expired plus 3154; the year is
/// Chaitrādi.
pub const SAPTARSHI_YEAR: EraYear = EraYear {
    era: "saptarshi",
    start: YearStart::CHAITRADI,
    offset: -3_154,
};

/// The Gupta era: Chaitrādi, year 0 current being Śaka 242 current, so
/// that year 1 opens in Śaka 242 expired (Sewell and Dikshit, Art. 71,
/// p. 43, after Fleet).
pub const GUPTA: EraYear = EraYear {
    era: "gupta",
    start: YearStart::CHAITRADI,
    offset: 241,
};

/// The Valabhī era: the Gupta count with its year thrown back to the
/// previous Kārttika śukla 1, so that year 1 opens at the Kārttika of Śaka
/// 241 expired (Sewell and Dikshit, Art. 71, p. 43).
pub const VALABHI: EraYear = EraYear {
    era: "valabhi",
    start: YearStart::KARTTIKADI,
    offset: 240,
};

/// The Chedi or Kalachuri era: Āśvinādi, year 1 opening at Āśvina śukla 1
/// of Śaka 171 current, 170 expired, 5 September 248 (Sewell and Dikshit,
/// Art. 71, pp. 42–43, after Kielhorn).
pub const KALACHURI: EraYear = EraYear {
    era: "kalachuri",
    start: YearStart::ASVINADI,
    offset: 169,
};

/// The Saptarṣi era's dropped hundreds.
pub mod saptarshi {
    /// The extra field that carries the Laukika year: the Saptarṣi year
    /// with its hundreds dropped.
    pub const LAUKIKA_YEAR_FIELD: &str = "laukika-year";

    /// The Laukika year of a full Saptarṣi year: the year modulo 100. The
    /// hundredth year of a century is 0 here; Sewell and Dikshit's "as soon
    /// as the reckoning reaches 100, a fresh hundred begins from 1" does
    /// not say whether it was written 100 or 0.
    #[must_use]
    pub const fn laukika_year(year: i64) -> i64 {
        year.rem_euclid(100)
    }

    /// The full Saptarṣi year with Laukika year `laukika` nearest to
    /// `near`, within fifty years either way — the caller supplies the
    /// century by supplying a year near it. `None` for a Laukika year
    /// outside 0 to 100; 100 is taken as 0.
    #[must_use]
    pub const fn full_year_near(laukika: i64, near: i64) -> Option<i64> {
        if laukika < 0 || laukika > 100 {
            return None;
        }
        let shift = (laukika - near).rem_euclid(100);
        Some(if shift < 50 {
            near + shift
        } else {
            near + shift - 100
        })
    }
}

/// When an era began to be used, where a source dates it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsageStart {
    /// In use at a beginning no source read dates.
    Undated,
    /// In use from a day, as a year, month and day of the Julian calendar.
    Julian(i64, u8, u8),
}

/// An era over the amānta or pūrṇimānta months.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LunarEra {
    /// The calendar's identifier.
    pub id: CalendarId,
    /// Its English name.
    pub english_name: &'static str,
    /// The locales whose writing of the era is its own.
    pub native_locales: &'static [&'static str],
    /// The year count.
    pub year: EraYear,
    /// Which months its dates are written in.
    pub reckoning: Reckoning,
    /// Whether the date carries the dropped-hundreds year of the Saptarṣi
    /// era as [`saptarshi::LAUKIKA_YEAR_FIELD`].
    pub laukika: bool,
    /// When the era came into use.
    pub usage_start: UsageStart,
    /// Where the period of use comes from.
    pub usage_source: &'static str,
    /// The calendar whose tithis these are.
    pub lunar: HinduPurnimantaCalendar,
}

/// The Gujarati Vikrama year from Kārttika śukla 1 on the amānta months,
/// the months numbered from Kārttika — `vikram-samvat-kartikadi`.
pub const VIKRAM_SAMVAT_KARTIKADI: LunarEra = LunarEra {
    id: CalendarId("vikram-samvat-kartikadi"),
    english_name: "Vikram Samvat (Karttikadi, Gujarat)",
    native_locales: &["gu", "hi"],
    year: KARTTIKADI_VIKRAMA_YEAR,
    reckoning: Reckoning::Amanta,
    laukika: false,
    usage_start: UsageStart::Undated,
    usage_source: "Sewell and Dikshit 1896, Art. 71, p. 41 [sewell1896]: the Vikrama year of Gujarat, \
        Karttikadi and amanta, and Kielhorn's finding, as they report it, that the era was \
        Karttikadi from the beginning; printed as the Gujarati Samvat today \
        [drik-day-panchang-2025], as docs/systems/indian-eras.md states",
    lunar: HinduPurnimantaCalendar::RASHTRIYA,
};

/// Śivājī's Rājyābhiṣeka Śaka on the amānta months, the year opening at
/// Jyeṣṭha śukla 13 — `rajyabhisheka-saka`.
pub const RAJYABHISHEKA_SAKA: LunarEra = LunarEra {
    id: CalendarId("rajyabhisheka-saka"),
    english_name: "Rajyabhisheka Saka (Maratha)",
    native_locales: &["mr"],
    year: RAJYABHISHEKA_YEAR,
    reckoning: Reckoning::Amanta,
    laukika: false,
    usage_start: UsageStart::Julian(1674, 6, 6),
    usage_source: "Sewell and Dikshit 1896, Art. 71, p. 47 [sewell1896]: established by Sivaji on \
        Jyeshtha sukla 13 of Saka 1596 expired, and not in use in 1896, on a last day they do not \
        date; the coronation on 6 June 1674 [wikipedia-shivaji], a Julian date, as \
        docs/systems/indian-eras.md states",
    lunar: HinduPurnimantaCalendar::RASHTRIYA,
};

/// The Saptarṣi era of Kashmir, counted in full from Kali 27 current on the
/// pūrṇimānta months, with the Laukika year beside it — `saptarshi`.
pub const SAPTARSHI: LunarEra = LunarEra {
    id: CalendarId("saptarshi"),
    english_name: "Saptarshi (Laukika, Kashmir)",
    native_locales: &["ks", "sa"],
    year: SAPTARSHI_YEAR,
    reckoning: Reckoning::Purnimanta,
    laukika: true,
    usage_start: UsageStart::Undated,
    usage_source: "Sewell and Dikshit 1896, Art. 71, p. 41 [sewell1896]: in use in Kashmir, and in \
        Multan in Alberuni's time, the only reckoning of the Raja-Tarangini; older than any source \
        read dates, as docs/systems/indian-eras.md states",
    lunar: HinduPurnimantaCalendar::RASHTRIYA,
};

/// Every lunisolar era this crate registers.
pub const ALL: &[LunarEra] = &[VIKRAM_SAMVAT_KARTIKADI, RAJYABHISHEKA_SAKA, SAPTARSHI];

/// A date in an era over the lunisolar months.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LunarEraDate {
    /// The era's year.
    pub year: i64,
    /// The month: 1 for the opening month where the year opens at the
    /// first day of an amānta month, and otherwise 1 for Chaitra.
    pub month: u8,
    /// Whether this is the intercalary (*adhika*) month of that name.
    pub leap_month: bool,
    /// The tithi, 1 through 30: 1–15 the bright fortnight, 16–30 the dark.
    pub day: u8,
    /// Whether this is the second day to carry that tithi at sunrise.
    pub leap_day: bool,
}

impl LunarEra {
    /// An era over another calendar: another place's sunrise, another
    /// ayanamsa — or one of the unregistered year counts, [`GUPTA`] say,
    /// under an identifier of the caller's.
    #[must_use]
    pub const fn new(
        id: CalendarId,
        english_name: &'static str,
        year: EraYear,
        reckoning: Reckoning,
        lunar: HinduPurnimantaCalendar,
    ) -> Self {
        Self {
            id,
            english_name,
            native_locales: &[],
            year,
            reckoning,
            laukika: false,
            usage_start: UsageStart::Undated,
            usage_source: "",
            lunar,
        }
    }

    /// Whether the months are numbered from the opening month: an amānta
    /// year that opens at a month's first day.
    const fn numbers_from_opening(&self) -> bool {
        matches!(self.reckoning, Reckoning::Amanta) && self.year.start.opens_a_month()
    }

    /// The era's date of an amānta date.
    fn date_of_amanta(&self, amanta: HinduLunarDate) -> CalendarResult<LunarEraDate> {
        let year = self.year.year_of(amanta);
        let shown = match self.reckoning {
            Reckoning::Amanta => amanta,
            Reckoning::Purnimanta => self.lunar.from_amanta(amanta)?,
        };
        let month = if self.numbers_from_opening() {
            self.year.start.era_month(shown.month)
        } else {
            shown.month
        };
        Ok(LunarEraDate {
            year,
            month,
            leap_month: shown.leap_month,
            day: shown.day,
            leap_day: shown.leap_day,
        })
    }

    /// The date on a fixed day.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::from_fixed`], outside the engine's range.
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<LunarEraDate> {
        self.date_of_amanta(self.lunar.amanta.from_fixed(rd)?)
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// [`CalendarError::MonthOutOfRange`] for a month outside 1–12, and
    /// otherwise as [`HinduLunarCalendar::to_fixed`]. A year that opens in
    /// the middle of a month holds the month twice, from the opening tithi
    /// at its start and up to it at its end, so every tithi of it names one
    /// day.
    pub fn to_fixed(&self, date: LunarEraDate) -> CalendarResult<Rd> {
        if !(1..=12).contains(&date.month) {
            return Err(CalendarError::MonthOutOfRange);
        }
        let month = if self.numbers_from_opening() {
            self.year.start.amanta_month(date.month)
        } else {
            date.month
        };
        // The Śaka year is the opening one or the next, or, for a
        // pūrṇimānta dark fortnight that closes a Śaka year, the one
        // before; whichever gives a day with this date is the one.
        let opening = date.year + self.year.offset;
        let mut last_error = CalendarError::DayOutOfRange;
        for saka in [opening, opening + 1, opening - 1] {
            let lunar = HinduLunarDate {
                year: saka,
                month,
                leap_month: date.leap_month,
                day: date.day,
                leap_day: date.leap_day,
            };
            let found = match self.reckoning {
                Reckoning::Amanta => self.lunar.amanta.to_fixed(lunar),
                Reckoning::Purnimanta => self.lunar.to_fixed(lunar),
            };
            match found {
                Ok(rd) => {
                    if self.from_fixed(rd) == Ok(date) {
                        return Ok(rd);
                    }
                }
                Err(error) => last_error = error,
            }
        }
        Err(match last_error {
            CalendarError::YearOutOfRange => CalendarError::YearOutOfRange,
            CalendarError::MonthOutOfRange => CalendarError::MonthOutOfRange,
            _ => CalendarError::DayOutOfRange,
        })
    }

    /// New Year's Day of a year.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside the engine's range.
    pub fn new_year(&self, year: i64) -> CalendarResult<Rd> {
        self.year.new_year(&self.lunar.amanta, year)
    }
}

impl Calendar for LunarEra {
    type Date = LunarEraDate;

    fn usage(&self) -> hc_calendar::Usage {
        match self.usage_start {
            UsageStart::Undated if self.usage_source.is_empty() => hc_calendar::Usage::UNRECORDED,
            UsageStart::Undated => hc_calendar::Usage::undated(self.usage_source),
            UsageStart::Julian(year, month, day) => match julian::to_fixed(year, month, day) {
                Ok(from) => hc_calendar::Usage::since(from, self.usage_source),
                Err(_) => hc_calendar::Usage::undated(self.usage_source),
            },
        }
    }

    /// Twelve months with a thirteenth in an intercalary year, and the
    /// seven-day week.
    fn cycles(&self) -> &'static [CycleShape] {
        LUNISOLAR_TWELVE
    }

    /// A year with an adhika māsa, which may fall in either of the Śaka
    /// years the era's year spans.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        self.year
            .start
            .is_leap_year(&self.lunar.amanta, year, self.year.offset)
    }

    /// The day begins at sunrise and is named by the civil day on whose
    /// sunrise it begins, the amānta calendar's reading.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise(hc_calendar::DayNaming::ByStart)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id,
            english_name: self.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: self.lunar.amanta.earliest().ok(),
            latest: self.lunar.amanta.latest().ok(),
            native_locales: self.native_locales,
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        LunarEra::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        LunarEra::from_fixed(self, rd)
    }

    /// The year under the era's code, the month and the tithi; for the
    /// Saptarṣi era the Laukika year as the extra
    /// [`saptarshi::LAUKIKA_YEAR_FIELD`], derived and ignored on input.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let month = if date.leap_month {
            Month::leap(date.month)
        } else {
            Month::regular(date.month)
        };
        let mut fields = DateFields::ymd(date.year, date.month, date.day).with_era(self.year.era);
        fields.month = Some(month);
        fields.leap_day = date.leap_day;
        if self.laukika {
            fields = fields.with_extra(
                saptarshi::LAUKIKA_YEAR_FIELD,
                saptarshi::laukika_year(date.year),
            )?;
        }
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != self.year.era) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        let date = LunarEraDate {
            year: fields.year,
            month: month.ordinal,
            leap_month: month.leap,
            day: fields.require_day()?,
            leap_day: fields.leap_day,
        };
        LunarEra::to_fixed(self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::places::UJJAIN;
    use crate::tithi::tithi_of_day;
    use hc_calendars_solar::gregorian;

    const AMANTA: HinduLunarCalendar = HinduLunarCalendar::RASHTRIYA;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    fn date(year: i64, month: u8, day: u8) -> HinduLunarDate {
        HinduLunarDate {
            year,
            month,
            leap_month: false,
            day,
            leap_day: false,
        }
    }

    #[test]
    fn the_gujarati_year_is_the_one_drik_panchang_prints() {
        // Drik Panchang's New Delhi pages (`drik-day-panchang-2025`):
        // "Gujarati Samvat 2080" on 15 August 2024, "2081 Nala" on
        // 1 January, 10 April and 21 October 2025, "2082 Pingala" on
        // 22 October 2025, Kārttika śukla pratipadā.
        let era = VIKRAM_SAMVAT_KARTIKADI;
        for ((y, m, d), year) in [
            ((2024, 8, 15), 2_080),
            ((2025, 1, 1), 2_081),
            ((2025, 4, 10), 2_081),
            ((2025, 10, 21), 2_081),
            ((2025, 10, 22), 2_082),
        ] {
            assert_eq!(era.from_fixed(ymd(y, m, d)).map(|d| d.year), Ok(year));
        }
        let new_year = era.from_fixed(ymd(2025, 10, 22)).expect("in range");
        assert_eq!((new_year.month, new_year.day), (1, 1));
        assert_eq!(era.new_year(2_082), Ok(ymd(2025, 10, 22)));
        let eve = era.from_fixed(ymd(2025, 10, 21)).expect("in range");
        assert_eq!((eve.month, eve.day), (12, 30));
    }

    #[test]
    fn the_gujarati_year_is_the_chaitradi_year_from_karttika_and_one_less_before() {
        let era = VIKRAM_SAMVAT_KARTIKADI;
        for day in (ymd(2023, 1, 1).0..ymd(2026, 1, 1).0).step_by(7) {
            let rd = Rd(day);
            let lunar = AMANTA.from_fixed(rd).expect("in range");
            let gujarati = era.from_fixed(rd).expect("in range");
            let expected = if lunar.month >= 8 {
                lunar.vikrama_year()
            } else {
                lunar.vikrama_year() - 1
            };
            assert_eq!(gujarati.year, expected, "{lunar:?}");
            assert_eq!(crate::kartikadi::amanta_month(gujarati.month), lunar.month);
        }
    }

    #[test]
    fn the_years_of_saka_1000_are_table_ii_s() {
        // Sewell and Dikshit, Art. 103 and Table II, part ii: amānta
        // Āṣāḍha of the Śaka year 1000 current, 999 expired, is Gupta 758,
        // Kārttikādi Vikrama 1134 and Chedi 829, all current. Table II's
        // heading prints Chedi 829; Art. 103's text prints 828, which
        // Kielhorn's epoch does not give.
        let ashadha = date(999, 4, 10);
        assert_eq!(GUPTA.year_of(ashadha), 758);
        assert_eq!(KALACHURI.year_of(ashadha), 829);
        // The Kārttikādi Vikrama year is counted expired, one less than
        // the table's current year.
        assert_eq!(KARTTIKADI_VIKRAMA_YEAR.year_of(ashadha) + 1, 1_134);
        // Valabhī is the Gupta count for Chaitra to Āśvina.
        assert_eq!(VALABHI.year_of(ashadha), 758);
        assert_eq!(VALABHI.year_of(date(999, 8, 1)), 759);
    }

    #[test]
    fn the_ancient_epochs_are_the_sources() {
        // Kielhorn: Chedi 1 opens at Āśvina śukla 1 of Śaka 171 current;
        // the Āśvina before is still year 0.
        assert_eq!(KALACHURI.year_of(date(170, 7, 1)), 1);
        assert_eq!(KALACHURI.year_of(date(170, 6, 30)), 0);
        assert_eq!(KALACHURI.year_of(date(171, 6, 30)), 1);
        // Fleet: Gupta 0 current is Śaka 242 current, 241 expired.
        assert_eq!(GUPTA.year_of(date(241, 1, 1)), 0);
        assert_eq!(GUPTA.year_of(date(242, 1, 1)), 1);
        // Valabhī: the epoch is the Kārttikādi Vikrama year 376 current,
        // Śaka 241–42 current, so year 1 opens at the Kārttika of Śaka
        // 241 expired, five months before Gupta 1.
        assert_eq!(VALABHI.year_of(date(241, 8, 1)), 1);
        assert_eq!(VALABHI.year_of(date(241, 7, 30)), 0);
        assert_eq!(KARTTIKADI_VIKRAMA_YEAR.year_of(date(241, 8, 1)) + 1, 377);
        // Rājyābhiṣeka 1 at Jyeṣṭha śukla 13 of Śaka 1596 expired.
        assert_eq!(RAJYABHISHEKA_YEAR.year_of(date(1_596, 3, 13)), 1);
        assert_eq!(RAJYABHISHEKA_YEAR.year_of(date(1_596, 3, 12)), 0);
        // The engine's range begins in 1700, so the epochs themselves are
        // refused.
        assert_eq!(
            GUPTA.new_year(&AMANTA, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_epoch_days_carry_their_tithis() {
        // Kielhorn's 5 September 248 and the coronation's 6 June 1674 are
        // Julian dates: by the library's astronomy, outside its calendars'
        // range, the first begins with tithi 1 at Ujjain's sunrise and the
        // second with tithi 13, and the Gregorian 6 June 1674 does not.
        let kielhorn = julian::to_fixed(248, 9, 5).expect("a date");
        assert_eq!(tithi_of_day(kielhorn, UJJAIN), 1);
        assert_eq!(tithi_of_day(Rd(kielhorn.0 - 1), UJJAIN), 29);
        let coronation = julian::to_fixed(1674, 6, 6).expect("a date");
        assert_eq!(tithi_of_day(coronation, UJJAIN), 13);
        assert_ne!(tithi_of_day(ymd(1674, 6, 6), UJJAIN), 13);
        assert_eq!(Calendar::usage(&RAJYABHISHEKA_SAKA).from, Some(coronation));
    }

    #[test]
    fn the_raja_saka_turns_at_jyeshtha_shukla_13() {
        let era = RAJYABHISHEKA_SAKA;
        let opening = era.new_year(351).expect("in range");
        // The first day of nija Jyeṣṭha of Śaka 1946 whose tithi is 13 or
        // later.
        let lunar = AMANTA.from_fixed(opening).expect("in range");
        assert_eq!(
            (lunar.year, lunar.month, lunar.leap_month),
            (1_946, 3, false)
        );
        assert!(lunar.day >= 13);
        let first = era.from_fixed(opening).expect("in range");
        assert_eq!((first.year, first.month, first.day), (351, 3, lunar.day));
        let eve = era.from_fixed(Rd(opening.0 - 1)).expect("in range");
        assert_eq!((eve.year, eve.month), (350, 3));
        // The year is the Śaka year less 1595 from the opening, and 1596
        // less before it.
        for day in (ymd(2024, 1, 1).0..ymd(2026, 1, 1).0).step_by(5) {
            let rd = Rd(day);
            let lunar = AMANTA.from_fixed(rd).expect("in range");
            let raja = era.from_fixed(rd).expect("in range");
            let offset = if rd >= era.new_year(lunar.year - 1_595).expect("in range") {
                1_595
            } else {
                1_596
            };
            assert_eq!(raja.year, lunar.year - offset, "{lunar:?}");
            assert_eq!(
                (raja.month, raja.day),
                (lunar.month, lunar.day),
                "{lunar:?}"
            );
        }
        // A year that opens in the middle of Jyeṣṭha holds two pieces of
        // it: śukla 13 on of Śaka 1946 at its start, and the days before
        // śukla 13 of Śaka 1947 at its end.
        let late = LunarEraDate {
            year: 351,
            month: 3,
            leap_month: false,
            day: 5,
            leap_day: false,
        };
        let rd = era.to_fixed(late).expect("in the year");
        assert_eq!(
            AMANTA.from_fixed(rd).map(|d| (d.year, d.month, d.day)),
            Ok((1_947, 3, 5))
        );
        assert!(rd < era.new_year(352).expect("in range"));
    }

    #[test]
    fn the_saptarshi_year_keeps_sewell_and_dikshits_equations() {
        let era = SAPTARSHI;
        // Chaitra śukla 1 of Śaka 1946, 9 April 2024: Kali 5126 current,
        // Saptarṣi 5126 − 26 = 5100.
        let navreh = AMANTA.new_year(1_946).expect("in range");
        assert_eq!(navreh, ymd(2024, 4, 9));
        let date = era.from_fixed(navreh).expect("in range");
        assert_eq!((date.year, date.month, date.day), (5_100, 1, 1));
        assert_eq!(era.from_fixed(Rd(navreh.0 - 1)).map(|d| d.year), Ok(5_099));
        for (rd, current_saka, christian) in [
            (navreh, 1_947, 2_024),
            (ymd(2025, 4, 1), 1_948, 2_025),
            (ymd(2027, 1, 10), 1_949, 2_026),
        ] {
            let year = era.from_fixed(rd).expect("in range").year;
            let laukika = saptarshi::laukika_year(year);
            // "Add 47 to the Saptarshi year to find the corresponding
            // current Saka year, and 24–25 for the corresponding Christian
            // year", the hundreds disregarded.
            assert_eq!((laukika + 47) % 100, current_saka % 100);
            assert_eq!((laukika + 24) % 100, christian % 100);
        }
        let fields = Calendar::to_fields(&era, date).expect("fields");
        assert_eq!(
            fields.extra.get(saptarshi::LAUKIKA_YEAR_FIELD),
            Some(0),
            "5100 is Laukika 0"
        );
        assert_eq!(saptarshi::full_year_near(0, 5_090), Some(5_100));
        assert_eq!(saptarshi::full_year_near(100, 5_090), Some(5_100));
        assert_eq!(saptarshi::full_year_near(47, 5_100), Some(5_147));
        assert_eq!(saptarshi::full_year_near(60, 5_100), Some(5_060));
        assert_eq!(saptarshi::full_year_near(101, 5_100), None);
        assert_eq!(saptarshi::full_year_near(-1, 5_100), None);
    }

    #[test]
    fn every_day_of_three_years_converts_and_converts_back() {
        for era in ALL {
            let mut intercalary = alloc::vec::Vec::new();
            for day in (ymd(2022, 11, 1).0..ymd(2025, 11, 1).0).step_by(crate::sweep_stride(3)) {
                let rd = Rd(day);
                let date = era.from_fixed(rd).expect("in range");
                assert_eq!(era.to_fixed(date), Ok(rd), "{}: {date:?}", era.id);
                let fields = Calendar::to_fields(era, date).expect("fields");
                assert_eq!(Calendar::from_fields(era, &fields), Ok(date));
                if date.leap_month && !intercalary.contains(&date.year) {
                    intercalary.push(date.year);
                }
            }
            // The adhika Śrāvaṇa of Śaka 1945, July–August 2023, is one
            // year of each era's, which says it is leap.
            assert_eq!(intercalary.len(), 1, "{}", era.id);
            assert_eq!(Calendar::is_leap_year(era, intercalary[0]), Ok(true));
            assert_eq!(Calendar::is_leap_year(era, intercalary[0] + 1), Ok(false));
        }
    }

    #[test]
    fn fields_carry_the_era_and_refuse_another() {
        let era = VIKRAM_SAMVAT_KARTIKADI;
        let date = era.from_fixed(ymd(2025, 10, 22)).expect("in range");
        let fields = Calendar::to_fields(&era, date).expect("fields");
        assert_eq!(fields.era, Some("vs"));
        let mut wrong = fields;
        wrong.era = Some("saka");
        assert_eq!(
            Calendar::from_fields(&era, &wrong),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            era.to_fixed(LunarEraDate { month: 13, ..date }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(era.new_year(1_000), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            Calendar::usage(&LunarEra::new(
                CalendarId("gupta"),
                "Gupta",
                GUPTA,
                Reckoning::Purnimanta,
                HinduPurnimantaCalendar::RASHTRIYA,
            )),
            hc_calendar::Usage::UNRECORDED
        );
    }
}
