//! The historical Indian eras over the lunisolar months: the Kārttikādi
//! Vikrama Saṃvat of Gujarat, Śivājī's Rājyābhiṣeka Śaka and the Saptarṣi
//! era of Kashmir over the *Rashtriya Panchang*'s months, registered as
//! `vikram-samvat-kartikadi`, `rajyabhisheka-saka` and `saptarshi`; and the
//! Gupta, Valabhī, Kalachuri and Lakṣmaṇa Sena eras over the *Sūrya
//! Siddhānta*'s months, registered as `gupta`, `valabhi`, `kalachuri` and
//! `lakshmana-sena`.
//!
//! The eras are written up in `docs/systems/indian-eras.md` in the
//! repository: what each is, the offsets and opening days from Sewell and
//! Dikshit's Art. 71 and how they follow from the sources' own equations,
//! current and expired years, what is carried and what is not and why, and
//! the checks. This page states the code's own facts.
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
//! # Which months an era is read over
//!
//! The Gupta and Valabhī inscriptions run from the year 82 to 945 of the era
//! and the Chedi dates Kielhorn examined from 793 to 934 (Sewell and
//! Dikshit, *The Indian Calendar*, 1896, pp. 42–43, `sewell1896`): the
//! fourth century to the thirteenth. The *Rashtriya Panchang*'s calendar
//! converts 1700 to 2299, so an era over it could convert no day anyone
//! dated in these. They are read over the *Sūrya Siddhānta*'s Sun and Moon
//! ([`Months::SIDDHANTA`], the amānta calendar of
//! [`crate::hindu_lunar_siddhanta`] and the pūrṇimānta renaming of it),
//! which converts Kali Yuga 1 to 10 000 and is the reckoning Sewell and
//! Dikshit's tables use. [`GUPTA`], [`VALABHI`] and [`KALACHURI`] are
//! their year counts, [`GUPTA_ERA`], [`VALABHI_ERA`] and [`KALACHURI_ERA`]
//! the calendars, and [`LunarEra::new`] builds an era over any months for
//! a caller who wants another.

use hc_calendar::shape::{CycleShape, LUNISOLAR_TWELVE};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_solar::julian;

use crate::amanta::AmantaMonths;
use crate::hindu_lunar::{HinduLunarCalendar, HinduLunarDate};
use crate::hindu_lunar_siddhanta::SiddhantaLunarCalendar;
use crate::hindu_purnimanta::{HinduPurnimantaCalendar, amanta_of, purnimanta_of};
use crate::year_start::YearStart;

/// Which reckoning of the months an era's dates are written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reckoning {
    /// The amānta months, new moon to new moon.
    Amanta,
    /// The pūrṇimānta months, full moon to full moon.
    Purnimanta,
}

/// The amānta months an era is read over: which sky counts them.
///
/// Both are the same engine ([`AmantaMonths`]) and differ in whose Sun and
/// Moon it reads, so an era's rules do not depend on which it is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Months {
    /// The true Sun and Moon of modern astronomy in an ayanāṃśa's zodiac,
    /// as the *Rashtriya Panchang* computes them
    /// ([`crate::hindu_lunar`]): Śaka 1622 to 2221, March 1700 to March
    /// 2300.
    True(HinduLunarCalendar),
    /// The *Sūrya Siddhānta*'s Sun and Moon at Ujjain's sunrise
    /// ([`crate::hindu_lunar_siddhanta`]): Kali Yuga 1 to 10 000, the
    /// reckoning of the old eras' dates and of Sewell and Dikshit's tables.
    Siddhanta(SiddhantaLunarCalendar),
}

impl Months {
    /// The *Rashtriya Panchang*'s months, as `hindu-lunar` has them.
    pub const RASHTRIYA: Self = Self::True(HinduLunarCalendar::RASHTRIYA);

    /// The *Sūrya Siddhānta*'s months read at Ujjain, as
    /// `hindu-lunar-surya-siddhanta` has them.
    pub const SIDDHANTA: Self = Self::Siddhanta(SiddhantaLunarCalendar::UJJAIN);
}

impl From<HinduLunarCalendar> for Months {
    fn from(lunar: HinduLunarCalendar) -> Self {
        Self::True(lunar)
    }
}

impl From<HinduPurnimantaCalendar> for Months {
    /// The amānta months the pūrṇimānta calendar renames.
    fn from(lunar: HinduPurnimantaCalendar) -> Self {
        Self::True(lunar.amanta)
    }
}

impl From<SiddhantaLunarCalendar> for Months {
    fn from(lunar: SiddhantaLunarCalendar) -> Self {
        Self::Siddhanta(lunar)
    }
}

impl AmantaMonths for Months {
    fn date_on(&self, rd: Rd) -> CalendarResult<HinduLunarDate> {
        match self {
            Self::True(lunar) => AmantaMonths::date_on(lunar, rd),
            Self::Siddhanta(lunar) => AmantaMonths::date_on(lunar, rd),
        }
    }

    fn day_of(&self, date: HinduLunarDate) -> CalendarResult<Rd> {
        match self {
            Self::True(lunar) => AmantaMonths::day_of(lunar, date),
            Self::Siddhanta(lunar) => AmantaMonths::day_of(lunar, date),
        }
    }

    fn month_span(&self, year: i64, month: u8, leap: bool) -> CalendarResult<(Rd, Rd)> {
        match self {
            Self::True(lunar) => AmantaMonths::month_span(lunar, year, month, leap),
            Self::Siddhanta(lunar) => AmantaMonths::month_span(lunar, year, month, leap),
        }
    }

    fn leap_month_of(&self, year: i64) -> CalendarResult<Option<(u8, Rd, Rd)>> {
        match self {
            Self::True(lunar) => AmantaMonths::leap_month_of(lunar, year),
            Self::Siddhanta(lunar) => AmantaMonths::leap_month_of(lunar, year),
        }
    }

    fn earliest(&self) -> CalendarResult<Rd> {
        match self {
            Self::True(lunar) => AmantaMonths::earliest(lunar),
            Self::Siddhanta(lunar) => AmantaMonths::earliest(lunar),
        }
    }

    fn latest(&self) -> CalendarResult<Rd> {
        match self {
            Self::True(lunar) => AmantaMonths::latest(lunar),
            Self::Siddhanta(lunar) => AmantaMonths::latest(lunar),
        }
    }

    fn years(&self) -> (i64, i64) {
        match self {
            Self::True(lunar) => AmantaMonths::years(lunar),
            Self::Siddhanta(lunar) => AmantaMonths::years(lunar),
        }
    }

    fn next_month_label(&self, day: Rd) -> (u8, bool) {
        match self {
            Self::True(lunar) => AmantaMonths::next_month_label(lunar, day),
            Self::Siddhanta(lunar) => AmantaMonths::next_month_label(lunar, day),
        }
    }
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
    pub fn new_year<L: AmantaMonths>(self, lunar: &L, year: i64) -> CalendarResult<Rd> {
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

/// The Lakṣmaṇa Sena era of Mithila and Tirhut, as Kielhorn reads it from
/// six inscriptions of 1194 to 1551: Kārttikādi, its first year AD 1119–20
/// and its epoch, the beginning of year 0 current, AD 1118–19, Śaka 1041–42
/// current (Sewell and Dikshit, Art. 71, p. 46, after Kielhorn). Counted
/// here in *current* years, as their tables give every era's and as the
/// *Mithila Panchang* prints it, 907 in October 2026; "documents and
/// inscriptions are generally dated in the expired year", one less, and
/// Kielhorn's equation "Laksh. sam. 505 = Saka sam. 1546" is of those.
pub const LAKSHMANA_SENA: EraYear = EraYear {
    era: "lakshmana-sena",
    start: YearStart::KARTTIKADI,
    offset: 1_040,
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
    /// In use from the opening of the era's own year 1, the epoch a source
    /// states, with no end. The range of the dated inscriptions a source
    /// reports is a lower bound on the use and not the whole of it, so it is
    /// named in the source and does not bound the period.
    Epoch,
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
    /// The months these dates are read over, whose tithis they keep.
    pub lunar: Months,
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
    lunar: Months::RASHTRIYA,
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
    lunar: Months::RASHTRIYA,
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
    lunar: Months::RASHTRIYA,
};

/// The Gupta era, Chaitrādi over the pūrṇimānta months, current years, read
/// over the *Sūrya Siddhānta* — `gupta`.
pub const GUPTA_ERA: LunarEra = LunarEra {
    id: CalendarId("gupta"),
    english_name: "Gupta (Chaitradi, purnimanta)",
    native_locales: &["sa"],
    year: GUPTA,
    reckoning: Reckoning::Purnimanta,
    laukika: false,
    usage_start: UsageStart::Epoch,
    usage_source: "Sewell and Dikshit 1896, Art. 71, p. 43 [sewell1896]: \"The inscriptions as yet \
        discovered which are dated in the Gupta and Valabhi era range from the years 82 to 945\", \
        a range of the two eras together and a lower bound on their use; Fleet's examination of \
        163 to 386 concludes the years are current and Chaitradi and the months purnimanta; the \
        period runs from the opening of year 1, the epoch the source states, and no source read \
        dates the last use of the era, so none is carried, as docs/systems/indian-eras.md states",
    lunar: Months::SIDDHANTA,
};

/// The Valabhī era: the Gupta count with its year thrown back to the
/// Kārttika before, over the amānta months, read over the *Sūrya
/// Siddhānta* — `valabhi`.
pub const VALABHI_ERA: LunarEra = LunarEra {
    id: CalendarId("valabhi"),
    english_name: "Valabhi (Karttikadi, amanta)",
    native_locales: &["sa"],
    year: VALABHI,
    reckoning: Reckoning::Amanta,
    laukika: false,
    usage_start: UsageStart::Epoch,
    usage_source: "Sewell and Dikshit 1896, Art. 71, p. 43 [sewell1896]: the Gupta era \"with its \
        name changed\", used in Kathiawar from about the fourth Gupta century, its year thrown back \
        to the previous Karttika sukla 1, \"its months seem to be both amanta and purnimanta\"; \
        Wikipedia's \"Gupta era\" [wikipedia-gupta-era], after Salomon, gives them as amanta; the \
        inscriptions of the two eras together run from the year 82 to 945, a lower bound on its \
        use; the period runs from the opening of year 1, the epoch the sources state, and no \
        source read dates the last use of the era, so none is carried, as \
        docs/systems/indian-eras.md states",
    lunar: Months::SIDDHANTA,
};

/// The Chedi or Kalachuri era, Āśvinādi over the pūrṇimānta months, current
/// years, read over the *Sūrya Siddhānta* — `kalachuri`.
pub const KALACHURI_ERA: LunarEra = LunarEra {
    id: CalendarId("kalachuri"),
    english_name: "Kalachuri or Chedi (Asvinadi, purnimanta)",
    native_locales: &["sa"],
    year: KALACHURI,
    reckoning: Reckoning::Purnimanta,
    laukika: false,
    usage_start: UsageStart::Epoch,
    usage_source: "Sewell and Dikshit 1896, Art. 71, pp. 42-43 [sewell1896]: Kielhorn's ten \
        inscriptions of the years 793 to 934, from which the first current year began at Asvina \
        sukla pratipada, 5 September A.D. 248, its years Asvinadi and current and its months \
        purnimanta; the era was used by the Kalachuri kings and \"appears to have been in use in \
        that part of India in still earlier times\", so the range is a lower bound on its use; the \
        period runs from the opening of year 1, the epoch the source states, and no source read \
        dates the last use of the era, so none is carried, as docs/systems/indian-eras.md states",
    lunar: Months::SIDDHANTA,
};

/// The Lakṣmaṇa Sena era of Mithila, Kielhorn's reading: Kārttikādi over the
/// amānta months, read over the *Sūrya Siddhānta* — `lakshmana-sena`.
pub const LAKSHMANA_SENA_ERA: LunarEra = LunarEra {
    id: CalendarId("lakshmana-sena"),
    english_name: "Lakshmana Sena (Karttikadi, Mithila)",
    native_locales: &["hi"],
    year: LAKSHMANA_SENA,
    reckoning: Reckoning::Amanta,
    laukika: false,
    usage_start: UsageStart::Epoch,
    usage_source: "Sewell and Dikshit 1896, Art. 71, p. 46 [sewell1896]: in use in Tirhut and \
        Mithila, and dated in six inscriptions of A.D. 1194 to 1551 that Kielhorn reads as \
        Karttikadi and amanta, its first year A.D. 1119-20; the Mithila Panchang prints its year, \
        907 in October 2026 [hinducalculator-mithila-panchang]; the period runs from the opening \
        of year 1 on Kielhorn's epoch, which earlier writers placed in 1105 to 1109, and is still \
        in use, as docs/systems/indian-eras.md states",
    lunar: Months::SIDDHANTA,
};

/// Every lunisolar era this crate registers: the three over the *Rashtriya
/// Panchang*'s months, and the four over the *Sūrya Siddhānta*'s.
pub const ALL: &[LunarEra] = &[
    VIKRAM_SAMVAT_KARTIKADI,
    RAJYABHISHEKA_SAKA,
    SAPTARSHI,
    GUPTA_ERA,
    VALABHI_ERA,
    KALACHURI_ERA,
    LAKSHMANA_SENA_ERA,
];

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
    /// An era over other months: another place's sunrise, another
    /// ayanāṃśa, or the *Sūrya Siddhānta*'s, under an identifier of the
    /// caller's.
    #[must_use]
    pub const fn new(
        id: CalendarId,
        english_name: &'static str,
        year: EraYear,
        reckoning: Reckoning,
        lunar: Months,
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
            Reckoning::Purnimanta => purnimanta_of(&self.lunar, amanta)?,
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
        self.date_of_amanta(self.lunar.date_on(rd)?)
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
                Reckoning::Amanta => self.lunar.day_of(lunar),
                Reckoning::Purnimanta => {
                    amanta_of(&self.lunar, lunar).and_then(|amanta| self.lunar.day_of(amanta))
                }
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
        self.year.new_year(&self.lunar, year)
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
            UsageStart::Epoch => match self.new_year(1) {
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
            .is_leap_year(&self.lunar, year, self.year.offset)
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
            earliest: self.lunar.earliest().ok(),
            latest: self.lunar.latest().ok(),
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
        let (first, last) = (ymd(2023, 1, 1).0, ymd(2026, 1, 1).0 - 1);
        // Every day in a release build; in a debug one every seventh and
        // both boundaries the rule turns on, Kārttika 1 and Chaitra 1, with
        // the day before each.
        for day in crate::sweep_days(first, last, 7, &boundaries(&era, first, last)) {
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
    fn the_raja_saka_opens_on_the_days_raigad_keeps() {
        // The coronation is kept at Raigad on its tithi as well as on
        // 6 June. Pudhari, 1 June 2023: the 350th Shivrajyabhishek "on
        // Friday (2 June)", "by the tithi of the Marathi panchang"
        // (`pudhari-shivrajyabhishek-2023`). ETV Bharat, 27 June 2026: the
        // 353rd held at Raigad "on Saturday (the 27th) by the tithi"
        // (`etvbharat-shivrajyabhishek-2026`), a nija Jyeṣṭha, the adhika
        // one having run from May. The ceremony's ordinal is the era's
        // year, year 1 having opened at the coronation.
        let era = RAJYABHISHEKA_SAKA;
        for (year, (y, m, d)) in [(350, (2023, 6, 2)), (353, (2026, 6, 27))] {
            let opening = era.new_year(year).expect("in range");
            assert_eq!(opening, ymd(y, m, d), "{year}");
            let lunar = AMANTA.from_fixed(opening).expect("in range");
            assert_eq!(
                (lunar.year, lunar.month, lunar.leap_month, lunar.day),
                (year + 1_595, 3, false, 13)
            );
            assert_eq!(era.from_fixed(opening).map(|d| d.year), Ok(year));
            assert_eq!(
                era.from_fixed(Rd(opening.0 - 1)).map(|d| d.year),
                Ok(year - 1)
            );
        }
        let intercalary = AMANTA.from_fixed(ymd(2026, 6, 1)).expect("in range");
        assert_eq!((intercalary.month, intercalary.leap_month), (3, true));
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
        let (first, last) = (ymd(2024, 1, 1).0, ymd(2026, 1, 1).0 - 1);
        // Every day in a release build; in a debug one every fifth and
        // both boundaries the rule turns on, the era's opening and
        // Chaitra 1, with the day before each.
        for day in crate::sweep_days(first, last, 5, &boundaries(&era, first, last)) {
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
        // Saptarṣi 5126 − 26 = 5100. Rising Kashmir, 9 April 2024: Navreh
        // "falls on April 9, 2024", "the beginning of a new century of
        // Saptrishi Samvat 5100" (`risingkashmir-navreh-2024`); the Daily
        // Excelsior of 10 February 2024 gives the same Navreh and day
        // (`dailyexcelsior-saptrishi-5100`).
        let navreh = AMANTA.new_year(1_946).expect("in range");
        assert_eq!(navreh, ymd(2024, 4, 9));
        let date = era.from_fixed(navreh).expect("in range");
        assert_eq!((date.year, date.month, date.day), (5_100, 1, 1));
        assert_eq!(era.from_fixed(Rd(navreh.0 - 1)).map(|d| d.year), Ok(5_099));
        // "Add 47 to the Saptarshi year to find the corresponding current
        // Saka year, and 24–25 for the corresponding Christian year", the
        // hundreds disregarded: 24 for a day from Chaitra to December, 25
        // for one from January to the next Chaitra, the Christian year
        // having turned while the Saptarṣi year had not.
        for (rd, current_saka, christian_of_chaitra) in [
            (navreh, 1_947, 2_024),
            (ymd(2025, 3, 1), 1_947, 2_024),
            (ymd(2025, 4, 1), 1_948, 2_025),
            (ymd(2027, 1, 10), 1_949, 2_026),
        ] {
            let year = era.from_fixed(rd).expect("in range").year;
            let laukika = saptarshi::laukika_year(year);
            assert_eq!((laukika + 47) % 100, current_saka % 100);
            assert_eq!((laukika + 24) % 100, christian_of_chaitra % 100);
            let (christian, _, _) = gregorian::from_fixed(rd).expect("a date");
            let add = if christian == christian_of_chaitra {
                24
            } else {
                25
            };
            assert_eq!((laukika + add) % 100, christian % 100, "{rd:?}");
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

    /// The first days of `era`'s years and of the Śaka years of
    /// [`AMANTA`] from a year before `first` to a year after `last`: the
    /// days on which a rule relating the two counts can change.
    fn boundaries(era: &LunarEra, first: i64, last: i64) -> alloc::vec::Vec<i64> {
        let (from, to) = (
            AMANTA.from_fixed(Rd(first)).expect("in range").year - 1,
            AMANTA.from_fixed(Rd(last)).expect("in range").year + 1,
        );
        let mut days: alloc::vec::Vec<i64> = (from..=to)
            .filter_map(|year| AMANTA.new_year(year).ok().map(|day| day.0))
            .collect();
        let years = |day: i64| era.from_fixed(Rd(day)).expect("in range").year;
        days.extend(openings(era, years(first) - 1..=years(last) + 1));
        days
    }

    /// Every opening of `era`'s years in `range`, as fixed days.
    fn openings(era: &LunarEra, range: core::ops::RangeInclusive<i64>) -> alloc::vec::Vec<i64> {
        range
            .filter_map(|year| era.new_year(year).ok().map(|day| day.0))
            .collect()
    }

    /// The date on `rd` converts back, directly and through its fields.
    fn round_trips(era: &LunarEra, rd: Rd) -> LunarEraDate {
        let date = era.from_fixed(rd).expect("in range");
        assert_eq!(era.to_fixed(date), Ok(rd), "{}: {date:?}", era.id);
        let fields = Calendar::to_fields(era, date).expect("fields");
        assert_eq!(Calendar::from_fields(era, &fields), Ok(date), "{}", era.id);
        date
    }

    #[test]
    fn every_day_of_three_years_converts_and_converts_back() {
        use core::sync::atomic::{AtomicI64, Ordering};
        for era in ALL {
            // The first and last year a day in an intercalary month falls
            // in: one year when they are equal.
            let (earliest, latest) = (AtomicI64::new(i64::MAX), AtomicI64::new(i64::MIN));
            let (first, last) = (ymd(2022, 11, 1).0, ymd(2025, 11, 1).0 - 1);
            let year = |day: i64| era.from_fixed(Rd(day)).expect("in range").year;
            // Every day in a release build; in a debug one every third and
            // each year's first day and the day before it, spread over the
            // machine's threads.
            let openings = openings(era, year(first)..=year(last));
            crate::check_days(&crate::sweep_days(first, last, 3, &openings), |day| {
                let date = round_trips(era, Rd(day));
                if date.leap_month {
                    earliest.fetch_min(date.year, Ordering::Relaxed);
                    latest.fetch_max(date.year, Ordering::Relaxed);
                }
            });
            // The adhika Śrāvaṇa of Śaka 1945, July–August 2023, is one
            // year of each era's, which says it is leap.
            let intercalary = earliest.into_inner();
            assert_eq!(latest.into_inner(), intercalary, "{}", era.id);
            assert_eq!(Calendar::is_leap_year(era, intercalary), Ok(true));
            assert_eq!(Calendar::is_leap_year(era, intercalary + 1), Ok(false));
        }
    }

    /// Every year of `era` in the engine's range opens where `new_year`
    /// says: its first day is in it and the day before in the year before;
    /// in a release build both also convert back, directly and through
    /// their fields, which a debug build leaves to the sweep above for the
    /// time it takes. Over the *Rashtriya Panchang*'s 600 years every year
    /// is checked in both builds; over the *Sūrya Siddhānta*'s ten
    /// thousand, every year in a release build and every seventh in a
    /// debug one.
    fn every_year_boundary_of(era: &LunarEra) {
        let meta = Calendar::meta(era);
        let (first, last) = (
            meta.earliest.expect("bounded"),
            meta.latest.expect("bounded"),
        );
        let year = |day: Rd| era.from_fixed(day).expect("in range").year;
        let count = core::sync::atomic::AtomicUsize::new(0);
        let date = |day: i64| {
            if cfg!(debug_assertions) {
                era.from_fixed(Rd(day)).expect("in range")
            } else {
                round_trips(era, Rd(day))
            }
        };
        // The years spread over the machine's threads, each opened where
        // `new_year` says as `openings` does.
        let (low, high) = (year(first) + 1, year(last));
        let years: alloc::vec::Vec<i64> = if matches!(era.lunar, Months::True(_)) {
            (low..=high).collect()
        } else {
            crate::sweep_years(low, high, 7).collect()
        };
        crate::check_days(&years, |year| {
            let Ok(Rd(day)) = era.new_year(year) else {
                return;
            };
            let opening = date(day);
            let eve = date(day - 1);
            assert_eq!(eve.year + 1, opening.year, "{}: {opening:?}", era.id);
            assert_eq!(era.new_year(opening.year), Ok(Rd(day)));
            count.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        });
        // Chaitra 1700 to March 2300: some six hundred years; at most the
        // year a range opens in and the one it closes in are not whole.
        let count = count.into_inner();
        assert!(
            count + 3 >= years.len(),
            "{}: {count} of {}",
            era.id,
            years.len()
        );
        assert!(count >= 598 / crate::sweep_stride(7), "{}: {count}", era.id);
    }

    #[test]
    fn every_year_of_the_siddhanta_eras_opens_on_its_day() {
        for era in [GUPTA_ERA, VALABHI_ERA, KALACHURI_ERA, LAKSHMANA_SENA_ERA] {
            every_year_boundary_of(&era);
        }
    }

    #[test]
    fn every_gujarati_year_opens_on_its_day() {
        every_year_boundary_of(&VIKRAM_SAMVAT_KARTIKADI);
    }

    #[test]
    fn every_raja_saka_year_opens_on_its_day() {
        every_year_boundary_of(&RAJYABHISHEKA_SAKA);
    }

    #[test]
    fn every_saptarshi_year_opens_on_its_day() {
        every_year_boundary_of(&SAPTARSHI);
    }

    const SIDDHANTA: SiddhantaLunarCalendar = SiddhantaLunarCalendar::UJJAIN;

    fn julian_day(year: i64, month: u8, day: u8) -> Rd {
        julian::to_fixed(year, month, day).expect("a date")
    }

    fn julian_year(rd: Rd) -> i64 {
        julian::from_fixed(rd).expect("a date").0
    }

    #[test]
    fn the_gupta_era_opens_on_26_february_320() {
        // Wikipedia's "Chandragupta I": coronation 26 February 320, the
        // first day of the Gupta era, which its "Gupta era" page says "began
        // on the first day of the shukla paksha of the Chaitra month"
        // (`wikipedia-chandragupta-i`, `wikipedia-gupta-era`); the dates are
        // Julian. By the Siddhānta Chaitra śukla 1 of Śaka 242 expired is
        // that day.
        let era = GUPTA_ERA;
        let opening = era.new_year(1).expect("in range");
        assert_eq!(opening, julian_day(320, 2, 26));
        let date = era.from_fixed(opening).expect("in range");
        assert_eq!((date.year, date.month, date.day), (1, 1, 1));
        assert_eq!(era.from_fixed(Rd(opening.0 - 1)).map(|d| d.year), Ok(0));
        assert_eq!(era.to_fixed(date), Ok(opening));
        // Sewell and Dikshit, Fleet's conclusion: Gupta 0 current is Śaka
        // 242 current, "A.D. 319–20", and 163 to 386 are inscriptions' years.
        let before = era.new_year(0).expect("in range");
        assert_eq!(julian_year(before), 319);
        // The period of use opens with the epoch's year 1, 26 February 320,
        // and has no last day: the inscriptions of the years 82 to 945 the
        // source reports are a lower bound that the source names.
        let usage = Calendar::usage(&era);
        assert_eq!(usage.from, Some(opening));
        assert_eq!(usage.until, None);
        assert!(usage.source.contains("82 to 945"));
        assert_eq!(usage.standing(opening), hc_calendar::Standing::InUse);
        assert_eq!(
            usage.standing(Rd(opening.0 - 1)),
            hc_calendar::Standing::Proleptic
        );
    }

    #[test]
    fn the_chedi_era_opens_on_kielhorns_5_september_248() {
        // Kielhorn: "the 1st day of the 1st current Chedi year corresponds
        // to Asvina sukla pratipada ... 5th Sept., A.D. 248" (Sewell and
        // Dikshit, Art. 71, pp. 42–43). A Julian date; by the Siddhānta it
        // is Āśvina śukla 1 of Śaka 170 expired, and the day before is the
        // dark fortnight of Bhādrapada, which a pūrṇimānta calendar names
        // Āśvina's: the year opens in the middle of its month.
        let era = KALACHURI_ERA;
        let opening = era.new_year(1).expect("in range");
        assert_eq!(opening, julian_day(248, 9, 5));
        let date = era.from_fixed(opening).expect("in range");
        assert_eq!((date.year, date.month, date.day), (1, 7, 1));
        let eve = era.from_fixed(Rd(opening.0 - 1)).expect("in range");
        assert_eq!((eve.year, eve.month, eve.day), (0, 7, 29));
        assert_eq!(era.to_fixed(date), Ok(opening));
        assert_eq!(era.to_fixed(eve), Ok(Rd(opening.0 - 1)));
        // The epoch, the beginning of year 0 current: A.D. 247–48.
        assert_eq!(julian_year(era.new_year(0).expect("in range")), 247);
        let usage = Calendar::usage(&era);
        // The period opens with Chedi year 1, 5 September 248; the
        // inscriptions of 793 to 934 (A.D. 1040 to 1182) are a lower bound.
        assert_eq!(usage.from, Some(opening));
        assert_eq!(usage.until, None);
        assert!(usage.source.contains("793 to 934"));
    }

    #[test]
    fn the_valabhi_year_is_the_gupta_count_thrown_back_five_months() {
        // "The beginning of the year was thrown back from Chaitra sukla 1st
        // to the previous Karttika sukla 1st, and therefore its epoch went
        // back five months" (Art. 71, p. 43): Valabhī 1 opens at Kārttika
        // śukla 1 of Śaka 241 expired, five amānta months, 147 days, before
        // Gupta 1, and Wikipedia's "Gupta era" gives the Valabhī years as
        // Kārttikādi and amānta.
        let valabhi = VALABHI_ERA.new_year(1).expect("in range");
        let gupta = GUPTA_ERA.new_year(1).expect("in range");
        assert_eq!(julian_year(valabhi), 319);
        let days = gupta.0 - valabhi.0;
        assert!((145..=150).contains(&days), "{days} days");
        // Its months are numbered from Kārttika, and the year before is
        // Āśvina's last day.
        let date = VALABHI_ERA.from_fixed(valabhi).expect("in range");
        assert_eq!((date.year, date.month, date.day), (1, 1, 1));
        let eve = VALABHI_ERA.from_fixed(Rd(valabhi.0 - 1)).expect("in range");
        assert_eq!((eve.year, eve.month), (0, 12));
        // And a day of Chaitra to Āśvina is one year behind the Gupta's
        // opening five months on: both turn within the Gupta year.
        let in_gupta_1 = Rd(gupta.0 + 10);
        assert_eq!(GUPTA_ERA.from_fixed(in_gupta_1).map(|d| d.year), Ok(1));
        assert_eq!(VALABHI_ERA.from_fixed(in_gupta_1).map(|d| d.year), Ok(1));
    }

    #[test]
    fn the_years_of_saka_1000_are_table_ii_s_through_the_calendars() {
        // Table II, part ii: amānta Āṣāḍha of Śaka 1000 current, 999
        // expired, is Gupta 758, Kārttikādi Vikrama 1134 and Chedi 829, all
        // current. The Siddhānta is the reckoning those tables are
        // computed by; here it is converted through the registered
        // calendars rather than the year arithmetic.
        let day = SIDDHANTA.to_fixed(date(999, 4, 10)).expect("in range");
        assert_eq!(GUPTA_ERA.from_fixed(day).map(|d| d.year), Ok(758));
        assert_eq!(VALABHI_ERA.from_fixed(day).map(|d| d.year), Ok(758));
        assert_eq!(KALACHURI_ERA.from_fixed(day).map(|d| d.year), Ok(829));
        // Āṣāḍha is month 4 in the Gupta's and 10 in the Valabhī's, which
        // counts from Kārttika.
        assert_eq!(GUPTA_ERA.from_fixed(day).map(|d| d.month), Ok(4));
        assert_eq!(VALABHI_ERA.from_fixed(day).map(|d| d.month), Ok(9));
    }

    #[test]
    fn the_lakshmana_sena_year_is_kielhorns_and_the_mithila_panchangs() {
        let era = LAKSHMANA_SENA_ERA;
        // Kielhorn: the epoch, the beginning of year 0 current, is A.D.
        // 1118–19, Śaka 1041–42 current; the first year A.D. 1119–20, and
        // the year is Kārttikādi (Art. 71, p. 46).
        assert_eq!(julian_year(era.new_year(0).expect("in range")), 1_118);
        let first = era.new_year(1).expect("in range");
        assert_eq!(julian_year(first), 1_119);
        let opening = SIDDHANTA.from_fixed(first).expect("in range");
        assert_eq!((opening.year, opening.month, opening.day), (1_041, 8, 1));
        // His equation from a manuscript of the Smṛtitattvāmṛta, "Laksh.
        // sam. 505 = Saka sam. 1546", is of expired years: Kārttika to
        // Phālguna of Śaka 1546 expired is Lakṣmaṇa Sena 506 current, 505
        // expired, and Chaitra to Āśvina of the same Śaka year, before the
        // Lakṣmaṇa Sena year turns, is 504 expired.
        for (month, expired) in [(8, 505), (12, 505), (1, 504), (7, 504)] {
            let day = SIDDHANTA
                .to_fixed(date(1_546, month, 10))
                .expect("in range");
            let year = era.from_fixed(day).expect("in range").year;
            assert_eq!(year - 1, expired, "month {month}");
        }
        // The Mithila Panchang prints "La. Sam. 907" for October 2026
        // (`hinducalculator-mithila-panchang`): current years, 907 from
        // Kārttika of 2025 to Āśvina of 2026.
        let at = |y, m, d| era.from_fixed(ymd(y, m, d)).map(|d| d.year);
        assert_eq!(at(2025, 11, 15), Ok(907));
        assert_eq!(at(2026, 10, 15), Ok(907));
        assert_eq!(at(2026, 11, 20), Ok(908));
        assert_eq!(at(2025, 10, 15), Ok(906));
        assert!(Calendar::usage(&era).source.contains("907"));
        // Its period of use opens with year 1, in A.D. 1119, and is not over.
        assert_eq!(Calendar::usage(&era).from, Some(first));
        assert_eq!(Calendar::usage(&era).until, None);
    }

    #[test]
    fn a_stride_through_the_siddhanta_eras_round_trips() {
        // Policy §7. The Siddhānta's months are swept every day on their
        // own; what an era adds, the year and, for the pūrṇimānta ones, the
        // names of the dark fortnights, is checked every 211th day of Kali
        // Yuga 1 to 10 000 and each year's first day with its eve.
        for era in [GUPTA_ERA, VALABHI_ERA, KALACHURI_ERA, LAKSHMANA_SENA_ERA] {
            let (first, last) = (
                era.lunar.earliest().expect("bounded").0,
                era.lunar.latest().expect("bounded").0,
            );
            let years = era.from_fixed(Rd(first)).expect("in range").year + 1
                ..=era.from_fixed(Rd(last)).expect("in range").year;
            let openings: alloc::vec::Vec<i64> = years
                .step_by(97)
                .filter_map(|year| era.new_year(year).ok().map(|day| day.0))
                .collect();
            let days = crate::strided_days(first, last, 211, 5, &openings);
            crate::check_days(&days, |day| {
                let date = era.from_fixed(Rd(day)).expect("in range");
                assert_eq!(era.to_fixed(date), Ok(Rd(day)), "{}: {date:?}", era.id);
            });
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
                CalendarId("x-gupta-over-the-true-sky"),
                "Gupta over the true sky",
                GUPTA,
                Reckoning::Purnimanta,
                Months::RASHTRIYA,
            )),
            hc_calendar::Usage::UNRECORDED
        );
    }
}
