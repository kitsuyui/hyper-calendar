//! The Javanese calendar, *Pananggalan Jawa*: Sultan Agung's lunar year of
//! 1633, the eight-year *windu* and the 120-year *kurup*.
//!
//! The system is written up in `docs/systems/javanese.md` in the
//! repository: the graft of 1633, the months and the day from sunset, the
//! windu of named years, the kurup and why each drops a day, the record of
//! the kurup with the changes of 1748 and 1936, 1 Sura 1959 worked by hand,
//! the three reckonings, what is carried, how it was checked, and every
//! source. This page summarises it and states the code's own facts.
//!
//! In 1633, 1555 of the Śaka era, Sultan Agung of Mataram put the Hijri
//! calendar's lunar months under the Śaka year number, so the year went on
//! as 1555 Jawa, *Anno Javanico*, and the Hijri year is the Javanese one
//! less 512 (`tanaya1971`, `karjanto2020`). The months alternate 30 and 29 days
//! from Sura, as the tabular Hijri months do, and Besar has a thirtieth day
//! in a *wuntu* year. Eight years are a windu, named Alip to Jimakir, of
//! which Ehe, Dal and Jimakir are wuntu, 355 days, and the rest *wastu*,
//! 354: 2835 days, 81 wetonan. Fifteen windu are a kurup, and "one day is
//! taken from the last wuntu year" of each so that the reckoning stays with
//! the Hijri one (`tanaya1971`): 42 524 days, four thirty-year Hijri cycles.
//!
//! The rule is Tanaya's; the record is not quite the rule. Pakubuwana V of
//! Surakarta ended the kurup Kamsiyah two years into a windu, at Ehe 1748,
//! so every kurup since has been counted from 1747 and ended at a Jimakir;
//! the order of 1935 moved 1 Sura 1867 to Tuesday 24 March 1936 and began
//! Asapon (`tanaya1971`). A [`JavaneseCalendar`] is therefore one rule over a
//! list of [`Kurup`], each with its first year and the year whose last day
//! it drops, and the three reckonings are three lists:
//!
//! * [`JAVANESE`], `javanese`: Surakarta's, the standard one, Tanaya's rule
//!   from 1749 and Asapon since 1936.
//! * [`JAVANESE_YOGYAKARTA`], `javanese-yogyakarta`: the Sultanate of
//!   Yogyakarta ran Kamsiyah to Jimakir 1794 and joined Surakarta from
//!   1 Sura 1795, 16 May 1866 (`karjanto2020`).
//! * [`JAVANESE_ABOGE`], `javanese-aboge`: the Aboge communities never took
//!   the change of 1936 and keep Arba'iyah, Alip Rebo Wage, without end
//!   (`idwiki-kalender-jawa`, `tempo-aboge-2017`).
//!
//! Every kurup since 1749 opens on 1 Muḥarram of year AJ − 512 of the civil
//! tabular Hijri calendar ([`crate::islamic_civil`]), which a test holds.
//! The range is 1555 to 2346, the last year of Sabtiyah, the last kurup
//! Tanaya names; Isnainiyah from 1987 onward is the 1935 rule projected.
//! The pasaran, the weton and the *wuku* are the regional crate's
//! `javanese-pasaran` and `balinese-pawukon`, and that crate's tests hold
//! this calendar's days to them.
//!
//! # Sources
//!
//! Keyed as in `docs/references.bib`.
//!
//! * `tanaya1971`: R. Tanaya, *Kabudayan Paugêraning Taun Jawa*, Surakarta,
//!   1971, as the Yayasan Sastra Lestari transcribes it (sastra.org, #616),
//!   read 2026-09-26 through the Wayback Machine's copy of 21 April 2026:
//!   the whole rule, the kurup and their names, the changes of 1748 and
//!   1935, the first-day tables, the day counted from its night.
//! * `karjanto2020`: N. Karjanto and F. Beauducel, "An ethnoarithmetic
//!   excursion into the Javanese calendar", arXiv:2012.10064v1, 2020, read
//!   2026-09-26: the kurup's first days, Yogyakarta's reckoning, 13 Sura
//!   1682.
//! * The others — Wikipedia, the Indonesian Wikipedia, the press for recent
//!   and Aboge dates — are listed in the system document and cited at the
//!   tests that use them.
//!
//! # Exactness
//!
//! Exact: the rule is the definition, in integer arithmetic. It is the
//! rule, not the Surakarta court's adjustments of the Dal months, which
//! Tanaya records and does not follow, and which Karjanto and Beauducel's
//! tables carry; the two part company in those years.

use hc_calendar::shape::{CycleShape, EraName, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, DayBoundary,
    DayNaming, Rd, Usage, YearKind,
};

use crate::tabular::{days_before_month, month_and_day};

/// The era code: *Anno Javanico*, AJ.
pub const ERA: &str = "aj";

/// The first year: 1555, the Śaka year the lunar count took over.
pub const FIRST_YEAR: i64 = 1555;

/// The last year converted: Jimakir 2346, the end of Sabtiyah, the last
/// kurup Tanaya names (`tanaya1971`).
pub const LAST_YEAR: i64 = 2346;

/// 1 Sura 1555, Friday 8 July 1633 (Gregorian), Jumat Legi, which was also
/// 1 Muḥarram 1043 (`tanaya1971`, `karjanto2020`).
pub const EPOCH: Rd = Rd(596_265);

/// What the Javanese year number exceeds the Hijri one by (`tanaya1971`).
pub const HIJRI_OFFSET: i64 = 512;

/// Years in a windu.
pub const WINDU_YEARS: i64 = 8;

/// Days in a windu: five wastu years and three wuntu, 81 wetonan.
pub const WINDU_DAYS: i64 = 2_835;

/// Years in a full kurup: fifteen windu.
pub const KURUP_YEARS: i64 = 120;

/// Days in a full kurup: fifteen windu less the day it drops, which is
/// four thirty-year cycles of the tabular Hijri calendar.
pub const KURUP_DAYS: i64 = 15 * WINDU_DAYS - 1;

/// The twelve months, Sura first, in the Latin spelling Wikipedia and
/// Tanaya give without Tanaya's *ê* [wikipedia-javanese-calendar,
/// tanaya1971]. Mulud is also Rabingulawal, Bakda Mulud Rabingulakir,
/// Ruwah Arwah, Pasa Siyam, Sela Apit or Dulkangidah, Besar Kaji.
pub const MONTHS: [&str; 12] = [
    "Sura",
    "Sapar",
    "Mulud",
    "Bakda Mulud",
    "Jumadilawal",
    "Jumadilakir",
    "Rejeb",
    "Ruwah",
    "Pasa",
    "Sawal",
    "Sela",
    "Besar",
];

/// The months in Javanese script, as Wikipedia's table prints them beside
/// the Latin forms (`wikipedia-javanese-calendar`).
pub const MONTHS_JAVANESE_SCRIPT: [&str; 12] = [
    "ꦱꦸꦫ",
    "ꦱꦥꦂ",
    "ꦩꦸꦭꦸꦢ꧀",
    "ꦧꦏ꧀ꦢꦩꦸꦭꦸꦢ꧀",
    "ꦗꦸꦩꦢꦶꦭꦮꦭ꧀",
    "ꦗꦸꦩꦢꦶꦭꦏꦶꦂ",
    "ꦉꦗꦼꦧ꧀",
    "ꦫꦸꦮꦃ",
    "ꦥꦱ",
    "ꦱꦮꦭ꧀",
    "ꦱꦼꦭ",
    "ꦧꦼꦱꦂ",
];

/// The eight years of a windu, as Tanaya spells them (`tanaya1971`).
pub const TAUN: [&str; 8] = [
    "Alip", "Ehe", "Jimawal", "Je", "Dal", "Be", "Wawu", "Jimakir",
];

/// Which of the eight years are *wuntu*, 355 days: Ehe, Dal and Jimakir
/// (`tanaya1971`).
pub const WUNTU: [bool; 8] = [false, true, false, false, true, false, false, true];

/// The four windu, as Tanaya spells them (`tanaya1971`); Karjanto and
/// Beauducel write Kuntara and Sengara (`karjanto2020`).
pub const WINDU: [&str; 4] = ["Adi", "Kunthara", "Sangara", "Sancaya"];

/// The seven kurup, named for the weekday their Alip years open on, from
/// Friday back to Saturday (`tanaya1971`).
pub const KURUP_NAMES: [&str; 7] = [
    "Jam'iyah",
    "Kamsiyah",
    "Arba'iyah",
    "Salasiyah",
    "Isnainiyah",
    "Ahadiyah",
    "Sabtiyah",
];

/// Days in a windu before each of its years.
const DAYS_BEFORE_TAUN: [i64; 8] = [0, 354, 709, 1_063, 1_417, 1_772, 2_126, 2_480];

/// A kurup as a reckoning ran it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Kurup {
    /// 1 for Jam'iyah to 7 for Sabtiyah: an index into [`KURUP_NAMES`].
    pub number: u8,
    /// The first year.
    pub first_year: i64,
    /// The last year, whose thirtieth of Besar is dropped; `None` for a
    /// kurup that never ends.
    pub last_year: Option<i64>,
}

impl Kurup {
    /// A kurup from its number, first year and last year.
    #[must_use]
    pub const fn new(number: u8, first_year: i64, last_year: Option<i64>) -> Self {
        Self {
            number,
            first_year,
            last_year,
        }
    }

    /// The kurup's name, or `None` for a number outside 1 to 7.
    #[must_use]
    pub const fn name(self) -> Option<&'static str> {
        if self.number == 0 || self.number as usize > KURUP_NAMES.len() {
            None
        } else {
            Some(KURUP_NAMES[self.number as usize - 1])
        }
    }
}

/// The kurup as Surakarta ran them, and as Tanaya's rule carries them on:
/// Kamsiyah ended at Ehe 1748 by Pakubuwana V's order, Arba'iyah began at
/// Jimawal 1749 and ended at Jimakir 1866 by the order of 1935, and every
/// kurup since is 120 years (`tanaya1971`).
pub const SURAKARTA_KURUPS: &[Kurup] = &[
    Kurup::new(1, 1555, Some(1674)),
    Kurup::new(2, 1675, Some(1748)),
    Kurup::new(3, 1749, Some(1866)),
    Kurup::new(4, 1867, Some(1986)),
    Kurup::new(5, 1987, Some(2106)),
    Kurup::new(6, 2107, Some(2226)),
    Kurup::new(7, 2227, Some(2346)),
];

/// The kurup as Yogyakarta ran them: Kamsiyah its full 120 years, to
/// Jimakir 1794, then Arba'iyah from Alip 1795 to Jimakir 1866, when the two
/// courts agreed again (`karjanto2020`, table 9).
pub const YOGYAKARTA_KURUPS: &[Kurup] = &[
    Kurup::new(1, 1555, Some(1674)),
    Kurup::new(2, 1675, Some(1794)),
    Kurup::new(3, 1795, Some(1866)),
    Kurup::new(4, 1867, Some(1986)),
    Kurup::new(5, 1987, Some(2106)),
    Kurup::new(6, 2107, Some(2226)),
    Kurup::new(7, 2227, Some(2346)),
];

/// The kurup as the Aboge communities keep them: Surakarta's to 1866, and
/// Arba'iyah, Alip Rebo Wage, from then on without end
/// (`idwiki-kalender-jawa`, `tempo-aboge-2017`).
pub const ABOGE_KURUPS: &[Kurup] = &[
    Kurup::new(1, 1555, Some(1674)),
    Kurup::new(2, 1675, Some(1748)),
    Kurup::new(3, 1749, None),
];

/// Where the period of use of `javanese` comes from.
pub const USAGE_SOURCE: &str = "Sultan Agung of Mataram's calendar, in force from 1 Sura 1555, Friday \
    8 July 1633 [tanaya1971, karjanto2020], and kept today beside the Gregorian and Hijri \
    calendars, the Kraton of Yogyakarta printing one [karjanto2020]; Surakarta's reckoning, \
    Asapon since 24 March 1936 [tanaya1971], as docs/systems/javanese.md states";

/// Where the period of use of `javanese-yogyakarta` comes from.
pub const YOGYAKARTA_USAGE_SOURCE: &str = "The Sultanate of Yogyakarta, divided from Surakarta by \
    the Treaty of Giyanti in 1755, which no source read dates to a day; it kept the kurup \
    Kamsiyah to Jimakir 1794 and followed Surakarta from 1 Sura 1795, 16 May 1866 \
    [karjanto2020, table 9], so its last day of its own is 15 May 1866, as \
    docs/systems/javanese.md states";

/// Where the period of use of `javanese-aboge` comes from.
pub const ABOGE_USAGE_SOURCE: &str = "Sultan Agung's calendar from 8 July 1633 [tanaya1971], \
    Surakarta's to 1866; the Aboge communities of Banyumas, Purbalingga, Cilacap and \
    Probolinggo, who did not take the change of 1936 [idwiki-kalender-jawa], keep it today \
    [tempo-aboge-2017, kompas-aboge-2025], as docs/systems/javanese.md states";

/// The day 1 Sura 1795 in Yogyakarta, 16 May 1866, less one: the last day
/// Yogyakarta's reckoning differed from Surakarta's.
const YOGYAKARTA_LAST_OWN_DAY: Rd = Rd(681_312);

/// Surakarta's reckoning, the standard one.
pub const JAVANESE: JavaneseCalendar = JavaneseCalendar::new(
    CalendarId("javanese"),
    "Javanese (Pananggalan Jawa)",
    SURAKARTA_KURUPS,
    Usage::since(EPOCH, USAGE_SOURCE),
);

/// Yogyakarta's reckoning of 1749 to 1866.
pub const JAVANESE_YOGYAKARTA: JavaneseCalendar = JavaneseCalendar::new(
    CalendarId("javanese-yogyakarta"),
    "Javanese (Yogyakarta, 1749–1866)",
    YOGYAKARTA_KURUPS,
    Usage::until(YOGYAKARTA_LAST_OWN_DAY, YOGYAKARTA_USAGE_SOURCE),
);

/// The Aboge reckoning.
pub const JAVANESE_ABOGE: JavaneseCalendar = JavaneseCalendar::new(
    CalendarId("javanese-aboge"),
    "Javanese (Aboge)",
    ABOGE_KURUPS,
    Usage::since(EPOCH, ABOGE_USAGE_SOURCE),
);

/// The months with their own names, the week, and the named years and
/// windu.
const SHAPE: &[CycleShape] = &[
    CycleShape::named(MONTH, &MONTHS),
    CycleShape::fixed(WEEKDAY, 7),
    CycleShape::named("taun", &TAUN),
    CycleShape::named("windu", &WINDU),
];

/// The year's place in the windu, 1 for Alip to 8 for Jimakir: Tanaya's
/// "the year plus 6, divided by 8", with a remainder of 0 as Jimakir
/// (`tanaya1971`).
#[must_use]
pub const fn taun(year: i64) -> u8 {
    match (year + 6).rem_euclid(WINDU_YEARS) {
        0 => 8,
        rest => rest as u8,
    }
}

/// The windu of a year, 1 for Adi to 4 for Sancaya: Tanaya's "the year plus
/// 6, divided by 32", a remainder of 1 to 8 being Sangara, 9 to 16
/// Sancaya, 17 to 24 Adi and 25 to 32 Kunthara (`tanaya1971`).
#[must_use]
pub const fn windu(year: i64) -> u8 {
    match (year + 6).rem_euclid(32) {
        1..=8 => 3,
        9..=16 => 4,
        17..=24 => 1,
        _ => 2,
    }
}

/// Whether a year is wuntu by its place in the windu, before any kurup
/// drops its day.
#[must_use]
pub const fn is_wuntu_position(year: i64) -> bool {
    WUNTU[taun(year) as usize - 1]
}

/// A date in the Javanese calendar.
///
/// The type carries no reckoning: the same year, month and day is a
/// different day in `javanese` and `javanese-aboge` after 1866, and the
/// calendar it came from is what says which.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JavaneseDate {
    /// The year AJ.
    pub year: i64,
    /// The month, 1 for Sura to 12 for Besar.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl JavaneseDate {
    /// A date, without validation; the calendar validates it.
    #[must_use]
    pub const fn new(year: i64, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// The year's place in the windu, 1 for Alip to 8 for Jimakir.
    #[must_use]
    pub const fn taun(self) -> u8 {
        taun(self.year)
    }

    /// The year's name in the windu.
    #[must_use]
    pub const fn taun_name(self) -> &'static str {
        TAUN[taun(self.year) as usize - 1]
    }

    /// The windu, 1 for Adi to 4 for Sancaya.
    #[must_use]
    pub const fn windu(self) -> u8 {
        windu(self.year)
    }

    /// The windu's name.
    #[must_use]
    pub const fn windu_name(self) -> &'static str {
        WINDU[windu(self.year) as usize - 1]
    }

    /// The month's name.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] outside 1 to 12.
    pub const fn month_name(self) -> CalendarResult<&'static str> {
        if self.month == 0 || self.month > 12 {
            return Err(CalendarError::MonthOutOfRange);
        }
        Ok(MONTHS[self.month as usize - 1])
    }
}

/// A Javanese reckoning: Tanaya's rule over a list of kurup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JavaneseCalendar {
    id: CalendarId,
    english_name: &'static str,
    kurups: &'static [Kurup],
    usage: Usage,
}

impl Default for JavaneseCalendar {
    fn default() -> Self {
        JAVANESE
    }
}

impl JavaneseCalendar {
    /// A reckoning from its kurup.
    ///
    /// The kurup are expected in order, each beginning the year after the
    /// last one ends, the first at [`FIRST_YEAR`], and only the last
    /// without an end; the three shipped are held to that by a test.
    #[must_use]
    pub const fn new(
        id: CalendarId,
        english_name: &'static str,
        kurups: &'static [Kurup],
        usage: Usage,
    ) -> Self {
        Self {
            id,
            english_name,
            kurups,
            usage,
        }
    }

    /// The reckoning's kurup, in order.
    #[must_use]
    pub const fn kurups(&self) -> &'static [Kurup] {
        self.kurups
    }

    /// The kurup a year belongs to, or `None` outside the range.
    #[must_use]
    pub const fn kurup(&self, year: i64) -> Option<Kurup> {
        if year < FIRST_YEAR || year > LAST_YEAR {
            return None;
        }
        let mut found = None;
        let mut index = 0;
        while index < self.kurups.len() {
            if self.kurups[index].first_year <= year {
                found = Some(self.kurups[index]);
            }
            index += 1;
        }
        found
    }

    /// Whether a kurup ends with `year`, which then loses its thirtieth of
    /// Besar.
    const fn drops_day(&self, year: i64) -> bool {
        let mut index = 0;
        while index < self.kurups.len() {
            if let Some(last) = self.kurups[index].last_year
                && last == year
            {
                return true;
            }
            index += 1;
        }
        false
    }

    /// How many kurup have ended before `year`.
    const fn drops_before(&self, year: i64) -> i64 {
        let mut count = 0;
        let mut index = 0;
        while index < self.kurups.len() {
            if let Some(last) = self.kurups[index].last_year
                && last < year
            {
                count += 1;
            }
            index += 1;
        }
        count
    }

    /// Whether `year` is wuntu, 355 days: a wuntu place in the windu that no
    /// kurup ends on. `None` outside the range.
    #[must_use]
    pub const fn is_wuntu(&self, year: i64) -> Option<bool> {
        if year < FIRST_YEAR || year > LAST_YEAR {
            return None;
        }
        Some(is_wuntu_position(year) && !self.drops_day(year))
    }

    /// The number of days in `year`, or `None` outside the range.
    #[must_use]
    pub const fn days_in_year(&self, year: i64) -> Option<u16> {
        let base = if is_wuntu_position(year) { 355 } else { 354 };
        match self.is_wuntu(year) {
            None => None,
            Some(_) if self.drops_day(year) => Some(base - 1),
            Some(_) => Some(base),
        }
    }

    /// The number of days in a month, or `None` for a month or year that
    /// does not exist.
    #[must_use]
    pub const fn days_in_month(&self, year: i64, month: u8) -> Option<u8> {
        match self.is_wuntu(year) {
            None => None,
            Some(wuntu) => match month {
                1..=11 => Some(if month % 2 == 1 { 30 } else { 29 }),
                12 => Some(if wuntu { 30 } else { 29 }),
                _ => None,
            },
        }
    }

    /// Days from the epoch to the first of `year`, for any year from
    /// [`FIRST_YEAR`] to the year after [`LAST_YEAR`].
    const fn days_before_year(&self, year: i64) -> i64 {
        let elapsed = year - FIRST_YEAR;
        elapsed.div_euclid(WINDU_YEARS) * WINDU_DAYS
            + DAYS_BEFORE_TAUN[elapsed.rem_euclid(WINDU_YEARS) as usize]
            - self.drops_before(year)
    }

    /// The last fixed day this reckoning converts: the end of Jimakir 2346.
    #[must_use]
    pub const fn latest(&self) -> Rd {
        Rd(EPOCH.0 + self.days_before_year(LAST_YEAR + 1) - 1)
    }

    /// The fixed day of the first of `year`, or `None` outside the range.
    #[must_use]
    pub const fn new_year(&self, year: i64) -> Option<Rd> {
        if year < FIRST_YEAR || year > LAST_YEAR {
            None
        } else {
            Some(Rd(EPOCH.0 + self.days_before_year(year)))
        }
    }
}

impl Calendar for JavaneseCalendar {
    type Date = JavaneseDate;

    /// From 1 Sura 1555, and kept today, except Yogyakarta's reckoning,
    /// whose own dates end on 15 May 1866; see [`USAGE_SOURCE`],
    /// [`YOGYAKARTA_USAGE_SOURCE`] and [`ABOGE_USAGE_SOURCE`].
    fn usage(&self) -> Usage {
        self.usage
    }

    /// The twelve named months, the week, and the named years and windu.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A wuntu year, whose Besar has thirty days. A wuntu year that closes
    /// a kurup "is made wastu" (`tanaya1971`) and is not leap.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        self.is_wuntu(year).ok_or(CalendarError::YearOutOfRange)
    }

    /// "AJ", *Anno Javanico* (`wikipedia-javanese-calendar`).
    fn era_name(&self, code: &str) -> Option<EraName> {
        (code == ERA).then_some(EraName::new("AJ", ""))
    }

    /// The date begins at nightfall and is named by the civil day it ends
    /// on: the *tanggal* is counted "from its night and then its day",
    /// where the weekday and pasaran are counted from sunrise
    /// (`tanaya1971`); "days in the Javanese calendar, like the Islamic
    /// calendar, begin at sunset" (`wikipedia-javanese-calendar`); *malam 1
    /// Suro* is the evening before 1 Sura (`kompas-suro-1959`).
    fn day_boundary(&self) -> DayBoundary {
        DayBoundary::Sunset(DayNaming::ByEnd)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id,
            english_name: self.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EPOCH),
            latest: Some(self.latest()),
            native_locales: &["jv"],
        }
    }

    /// The month's length from the rule, without walking the days.
    fn days_in_month(&self, fields: &DateFields) -> CalendarResult<u16> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        if !(FIRST_YEAR..=LAST_YEAR).contains(&fields.year) {
            return Err(CalendarError::YearOutOfRange);
        }
        self.days_in_month(fields.year, month.ordinal)
            .map(u16::from)
            .ok_or(CalendarError::MonthOutOfRange)
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        if !(FIRST_YEAR..=LAST_YEAR).contains(&date.year) {
            return Err(CalendarError::YearOutOfRange);
        }
        let length = self
            .days_in_month(date.year, date.month)
            .ok_or(CalendarError::MonthOutOfRange)?;
        if date.day == 0 || date.day > length {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(Rd(EPOCH.0
            + self.days_before_year(date.year)
            + days_before_month(date.month)
            + i64::from(date.day)
            - 1))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        if rd < EPOCH {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd > self.latest() {
            return Err(CalendarError::AfterSupportedRange);
        }
        let elapsed = rd.0 - EPOCH.0;
        // The windu's own days, less the days the kurup have dropped, are
        // never more than the elapsed days, so the year is at or after the
        // first year of this windu, and at most a windu on.
        let mut year = FIRST_YEAR + elapsed.div_euclid(WINDU_DAYS) * WINDU_YEARS;
        while year < LAST_YEAR && self.days_before_year(year + 1) <= elapsed {
            year += 1;
        }
        let (month, day) = month_and_day(elapsed - self.days_before_year(year));
        Ok(JavaneseDate { year, month, day })
    }

    /// The year AJ, the month and the day, with the year's place in the
    /// windu as `taun`, the windu as `windu` and the kurup as `kurup`,
    /// 1 Jam'iyah to 7 Sabtiyah.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let kurup = self.kurup(date.year).ok_or(CalendarError::YearOutOfRange)?;
        DateFields::ymd(date.year, date.month, date.day)
            .with_era(ERA)
            .with_extra("taun", i64::from(date.taun()))?
            .with_extra("windu", i64::from(date.windu()))?
            .with_extra("kurup", i64::from(kurup.number))
    }

    /// Reads the year, month and day; `taun`, `windu` and `kurup` follow
    /// from the year and are not read.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = JavaneseDate::new(fields.year, month.ordinal, fields.require_day()?);
        self.to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::islamic_civil::IslamicCivilCalendar;
    use crate::tabular::IslamicDate;
    use hc_calendar::{Weekday, gregorian};

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    fn day(calendar: &JavaneseCalendar, year: i64, month: u8, day: u8) -> Rd {
        calendar
            .to_fixed(JavaneseDate::new(year, month, day))
            .unwrap()
    }

    const RECKONINGS: [JavaneseCalendar; 3] = [JAVANESE, JAVANESE_YOGYAKARTA, JAVANESE_ABOGE];

    /// "Friday Legi, 1 Muharram of the first year, Alip, windu Kunthara ...
    /// 1 Muharram 1043 of the Hijra, 8 July 1633" (`tanaya1971`); Karjanto and
    /// Beauducel's "Jemuwah Legi 1 Sura Alip 1555 AJ", 8 July 1633
    /// (`karjanto2020`).
    #[test]
    fn the_first_day_is_friday_8_july_1633_and_1_muharram_1043() {
        assert_eq!(EPOCH, greg(1633, 7, 8));
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Friday);
        for calendar in RECKONINGS {
            let date = calendar.from_fixed(EPOCH).unwrap();
            assert_eq!(date, JavaneseDate::new(1555, 1, 1));
            assert_eq!((date.taun_name(), date.windu_name()), ("Alip", "Kunthara"));
            assert_eq!(calendar.meta().earliest, Some(EPOCH));
        }
        assert_eq!(
            IslamicCivilCalendar.to_fixed(IslamicDate {
                year: 1043,
                month: 1,
                day: 1
            }),
            Ok(EPOCH)
        );
        assert_eq!(1555 - HIJRI_OFFSET, 1043);
    }

    /// Karjanto and Beauducel's Table 8, the first day of each kurup, and
    /// their Table 9 for Yogyakarta (`karjanto2020`); Wikipedia's 24 March
    /// 1936 and 26 August 2052 (`wikipedia-javanese-calendar`). Tanaya gives
    /// the weekdays: Jimawal 1749 on Friday, Kamsiyah's Jimawal, which
    /// Yogyakarta kept in 1749, on Saturday, and the Alip years on the
    /// kurup's own day (`tanaya1971`).
    #[test]
    fn every_kurup_opens_where_the_sources_put_it() {
        for (calendar, year, date, weekday) in [
            (JAVANESE, 1555, (1633, 7, 8), Weekday::Friday),
            (JAVANESE, 1675, (1749, 12, 11), Weekday::Thursday),
            (JAVANESE, 1749, (1821, 9, 28), Weekday::Friday),
            (JAVANESE, 1867, (1936, 3, 24), Weekday::Tuesday),
            (JAVANESE, 1987, (2052, 8, 26), Weekday::Monday),
            (JAVANESE_YOGYAKARTA, 1675, (1749, 12, 11), Weekday::Thursday),
            (JAVANESE_YOGYAKARTA, 1749, (1821, 9, 29), Weekday::Saturday),
            (JAVANESE_YOGYAKARTA, 1795, (1866, 5, 16), Weekday::Wednesday),
            (JAVANESE_YOGYAKARTA, 1867, (1936, 3, 24), Weekday::Tuesday),
            (JAVANESE_ABOGE, 1749, (1821, 9, 28), Weekday::Friday),
            (JAVANESE_ABOGE, 1867, (1936, 3, 25), Weekday::Wednesday),
            (JAVANESE_ABOGE, 1987, (2052, 8, 28), Weekday::Wednesday),
        ] {
            let first = greg(date.0, date.1, date.2);
            assert_eq!(
                calendar.new_year(year),
                Some(first),
                "{} {year}",
                calendar.id
            );
            assert_eq!(Weekday::from_rd(first), weekday, "{} {year}", calendar.id);
            assert_eq!(
                calendar.from_fixed(first),
                Ok(JavaneseDate::new(year, 1, 1))
            );
        }
        let named = |calendar: JavaneseCalendar, year| calendar.kurup(year).unwrap().name();
        assert_eq!(named(JAVANESE, 1555), Some("Jam'iyah"));
        assert_eq!(named(JAVANESE, 1748), Some("Kamsiyah"));
        assert_eq!(named(JAVANESE, 1749), Some("Arba'iyah"));
        assert_eq!(named(JAVANESE_YOGYAKARTA, 1749), Some("Kamsiyah"));
        assert_eq!(named(JAVANESE, 1959), Some("Salasiyah"));
        assert_eq!(named(JAVANESE_ABOGE, 1959), Some("Arba'iyah"));
        assert_eq!(named(JAVANESE, 2346), Some("Sabtiyah"));
        assert_eq!(named(JAVANESE_ABOGE, 2346), Some("Arba'iyah"));
    }

    /// "After Thursday 29 Besar of the year Ehe 1748, the count went on
    /// with Friday 1 Sura of the year Jimawal 1749" (`tanaya1971`): Ehe, a
    /// wuntu year, has no 30 Besar that year in Surakarta, and has one in
    /// Yogyakarta.
    #[test]
    fn pakubuwana_v_stepped_over_a_day_after_29_besar_1748() {
        let last = day(&JAVANESE, 1748, 12, 29);
        assert_eq!(last, greg(1821, 9, 27));
        assert_eq!(Weekday::from_rd(last), Weekday::Thursday);
        assert_eq!(
            JAVANESE.from_fixed(Rd(last.0 + 1)),
            Ok(JavaneseDate::new(1749, 1, 1))
        );
        assert_eq!(
            JAVANESE.to_fixed(JavaneseDate::new(1748, 12, 30)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(JAVANESE.is_leap_year(1748), Ok(false));
        assert!(is_wuntu_position(1748));
        assert_eq!(JAVANESE_YOGYAKARTA.is_leap_year(1748), Ok(true));
        assert_eq!(day(&JAVANESE_YOGYAKARTA, 1748, 12, 30), greg(1821, 9, 28));
        // Yogyakarta dropped its day at Jimakir 1794 instead, and the two
        // agree from 1 Sura 1795 [karjanto2020].
        assert_eq!(JAVANESE_YOGYAKARTA.is_leap_year(1794), Ok(false));
        assert_eq!(JAVANESE.is_leap_year(1794), Ok(true));
        assert_eq!(JAVANESE.new_year(1795), JAVANESE_YOGYAKARTA.new_year(1795));
        assert_eq!(
            JAVANESE_YOGYAKARTA.usage().until,
            Some(Rd(greg(1866, 5, 16).0 - 1))
        );
        for year in 1749..=1794 {
            assert_eq!(
                JAVANESE_YOGYAKARTA.new_year(year).unwrap().0,
                JAVANESE.new_year(year).unwrap().0 + 1,
                "{year}"
            );
        }
    }

    /// The order of 15 December 1935: Jimakir 1866 "is not used as a wuntu
    /// year", and 1 Sura Alip 1867 "falls on 24 March 1936, a Selasa Pon"
    /// (`tanaya1971`). The Aboge did not follow it, and Asapon's last day is
    /// Sunday 25 August 2052 (`wikipedia-javanese-calendar`).
    #[test]
    fn the_1935_order_moved_1_sura_1867_to_24_march_1936() {
        assert_eq!(JAVANESE.is_leap_year(1866), Ok(false));
        assert_eq!(JAVANESE_ABOGE.is_leap_year(1866), Ok(true));
        assert_eq!(day(&JAVANESE, 1866, 12, 29), greg(1936, 3, 23));
        assert_eq!(day(&JAVANESE_ABOGE, 1866, 12, 30), greg(1936, 3, 24));
        let end = day(&JAVANESE, 1986, 12, 29);
        assert_eq!(end, greg(2052, 8, 25));
        assert_eq!(Weekday::from_rd(end), Weekday::Sunday);
        assert_eq!(JAVANESE.is_leap_year(1986), Ok(false));
        assert_eq!(JAVANESE_ABOGE.is_leap_year(1986), Ok(true));
    }

    /// A full kurup is 42 524 days, four thirty-year Hijri cycles, and each
    /// since Jimawal 1749 opens on 1 Muḥarram of the year AJ − 512 of the
    /// civil tabular calendar; Wikipedia says so of 1355 and 1475
    /// (`wikipedia-javanese-calendar`). Within a kurup the two drift a day
    /// apart: 1 Sura 1747 is a day after 1 Muḥarram 1235, the day behind
    /// the experts noticed (`karjanto2020`).
    #[test]
    fn every_kurup_opens_on_the_tabular_first_of_muharram() {
        assert_eq!(KURUP_DAYS, 42_524);
        assert_eq!(KURUP_DAYS, 4 * crate::tabular::CYCLE_DAYS);
        let muharram = |year| {
            IslamicCivilCalendar
                .to_fixed(IslamicDate {
                    year,
                    month: 1,
                    day: 1,
                })
                .unwrap()
        };
        for year in [1555, 1675, 1749, 1867, 1987, 2107, 2227] {
            assert_eq!(
                JAVANESE.new_year(year),
                Some(muharram(year - HIJRI_OFFSET)),
                "{year}"
            );
        }
        for kurup in SURAKARTA_KURUPS {
            let last = kurup.last_year.unwrap();
            if last - kurup.first_year + 1 == KURUP_YEARS {
                let length = JAVANESE.new_year(last).unwrap().0
                    + i64::from(JAVANESE.days_in_year(last).unwrap())
                    - JAVANESE.new_year(kurup.first_year).unwrap().0;
                assert_eq!(length, KURUP_DAYS, "{kurup:?}");
            }
        }
        assert_eq!(JAVANESE.new_year(1747).unwrap().0, muharram(1235).0 + 1);
    }

    /// Dated days in Tanaya and in Karjanto and Beauducel, with their
    /// weekdays; the pasaran are checked in the regional crate.
    #[test]
    fn the_dated_days_in_tanaya_and_karjanto() {
        for (date, weekday, civil) in [
            // Sultan Agung's first Garebeg Mulud, 12 Mulud of the Dal year
            // 1559, on Senen Pon [tanaya1971].
            (JavaneseDate::new(1559, 3, 12), Weekday::Monday, None),
            // Garebeg Mulud of Je 1902, Senen Legi [tanaya1971].
            (JavaneseDate::new(1902, 3, 12), Weekday::Monday, None),
            // Bagus Ngarfah's note, "Rebo Wage, 4 Zu'lkaedah of the year
            // Dal 1831", written in the plain dates [tanaya1971].
            (JavaneseDate::new(1831, 11, 4), Weekday::Wednesday, None),
            // Hamengkubuwana I moved into the Kraton of Yogyakarta on
            // "13 Sura 1682 AJ (7 Oktober 1756 CE), a Kemis Pahing"
            // [karjanto2020].
            (
                JavaneseDate::new(1682, 1, 13),
                Weekday::Thursday,
                Some((1756, 10, 7)),
            ),
        ] {
            let rd = JAVANESE.to_fixed(date).unwrap();
            assert_eq!(Weekday::from_rd(rd), weekday, "{date:?}");
            if let Some((year, month, day)) = civil {
                assert_eq!(rd, greg(year, month, day), "{date:?}");
            }
        }
    }

    /// The calendars the press printed for recent years: "Jumat Kliwon,
    /// 1 Suro 1959 Dal: 27 Juni 2025" and "Sabtu Wage, 30 Suro 1959 Dal:
    /// 26 Juli 2025" (`kompas-suro-1959`); 19 July 2023, Rebo Legi, "30 Besar
    /// 1956, in the year Ehe", from the Radyapustaka Museum's calendar
    /// expert (`detik-besar-1956`).
    #[test]
    fn the_printed_calendars_of_recent_years() {
        let first = greg(2025, 6, 27);
        let date = JAVANESE.from_fixed(first).unwrap();
        assert_eq!(date, JavaneseDate::new(1959, 1, 1));
        assert_eq!(
            (date.taun_name(), date.windu_name(), date.month_name()),
            ("Dal", "Sancaya", Ok("Sura"))
        );
        assert_eq!(Weekday::from_rd(first), Weekday::Friday);
        assert_eq!(day(&JAVANESE, 1959, 1, 30), greg(2025, 7, 26));
        let besar = JAVANESE.from_fixed(greg(2023, 7, 19)).unwrap();
        assert_eq!(besar, JavaneseDate::new(1956, 12, 30));
        assert_eq!(besar.taun_name(), "Ehe");
        // The worked example of docs/systems/javanese.md.
        assert_eq!(first.0 - greg(1936, 3, 24).0, 32_602);
    }

    /// The Aboge communities: Idul Fitri on Tuesday 27 June 2017 in a Je
    /// year (`tempo-aboge-2017`); the fast from Rebo Wage 13 March 2024 and
    /// Idul Fitri on Jumat Wage 12 April (`detik-aboge-2024`); the fast from
    /// Sunday 2 March 2025 and Idul Fitri on Selasa Pon 1 April
    /// (`kompas-aboge-2025`). Each is a day after `javanese`.
    #[test]
    fn the_aboge_communities_dates() {
        for (year, month, civil) in [
            (1950, 10, (2017, 6, 27)),
            (1957, 9, (2024, 3, 13)),
            (1957, 10, (2024, 4, 12)),
            (1958, 9, (2025, 3, 2)),
            (1958, 10, (2025, 4, 1)),
        ] {
            let rd = greg(civil.0, civil.1, civil.2);
            assert_eq!(day(&JAVANESE_ABOGE, year, month, 1), rd, "{year}-{month}");
            assert_eq!(day(&JAVANESE, year, month, 1).0 + 1, rd.0, "{year}-{month}");
        }
        assert_eq!(JavaneseDate::new(1950, 1, 1).taun_name(), "Je");
        assert_eq!(1950 - HIJRI_OFFSET, 1438);
    }

    /// Two almanac sites put 1 Sura 1957 on 19 July 2023 [kidemang-almanak,
    /// kalenderku-2024] and 30 Besar 1957 on 7 July 2024
    /// (`kalenderku-2024`), a long Jimawal after a short Ehe, as Wikipedia's
    /// list of year lengths has it (`wikipedia-javanese-calendar`). Tanaya's
    /// Ehe is long and his Jimawal short (`tanaya1971`); the two agree again
    /// from 1 Sura 1958. The disagreement is recorded, not followed.
    #[test]
    fn the_web_almanacs_long_jimawal_is_not_followed() {
        assert_eq!(
            JAVANESE.from_fixed(greg(2023, 7, 19)),
            Ok(JavaneseDate::new(1956, 12, 30))
        );
        assert_eq!(
            JAVANESE.from_fixed(greg(2023, 7, 20)),
            Ok(JavaneseDate::new(1957, 1, 1))
        );
        assert_eq!(
            JAVANESE.from_fixed(greg(2024, 7, 7)),
            Ok(JavaneseDate::new(1957, 12, 29))
        );
        assert_eq!(
            JAVANESE.from_fixed(greg(2024, 7, 8)),
            Ok(JavaneseDate::new(1958, 1, 1))
        );
    }

    /// "The year plus 6, divided by 32" and "by 8", and Tanaya's example:
    /// 1708 is windu Adi and the year Ehe (`tanaya1971`); 1555 is windu
    /// Kunthara; 1867 to 1874 are Adi and 1875 to 1882 Kunthara, as his
    /// tables head them.
    #[test]
    fn the_windu_and_year_names_follow_tanayas_rules() {
        let names = |year| {
            let date = JavaneseDate::new(year, 1, 1);
            (date.windu_name(), date.taun_name())
        };
        assert_eq!(names(1708), ("Adi", "Ehe"));
        assert_eq!(names(1555), ("Kunthara", "Alip"));
        assert_eq!(names(1867), ("Adi", "Alip"));
        assert_eq!(names(1874), ("Adi", "Jimakir"));
        assert_eq!(names(1875), ("Kunthara", "Alip"));
        assert_eq!(names(1883), ("Sangara", "Alip"));
        assert_eq!(names(1891), ("Sancaya", "Alip"));
        for year in FIRST_YEAR..=LAST_YEAR {
            assert_eq!(i64::from(taun(year)), (year - FIRST_YEAR) % 8 + 1);
            assert_eq!(
                i64::from(windu(year)),
                ((year - FIRST_YEAR) / 8 + 1) % 4 + 1,
                "{year}"
            );
        }
    }

    /// The three kurup lists are well formed: in order from 1555, each
    /// beginning the year after the last ends, each ending on a wuntu
    /// place, and only the last open. Every year is 354 or 355 days and
    /// every windu 2835, less the day of a kurup ending in it.
    #[test]
    fn the_kurup_lists_are_well_formed() {
        for calendar in RECKONINGS {
            let kurups = calendar.kurups();
            assert_eq!(kurups[0].first_year, FIRST_YEAR);
            for pair in kurups.windows(2) {
                assert_eq!(pair[0].last_year, Some(pair[1].first_year - 1));
                assert_eq!(pair[0].number + 1, pair[1].number);
            }
            for kurup in kurups {
                assert!(kurup.name().is_some());
                if let Some(last) = kurup.last_year {
                    assert!(is_wuntu_position(last), "{kurup:?}");
                }
            }
            for year in FIRST_YEAR..=LAST_YEAR {
                let length = calendar.days_in_year(year).unwrap();
                let months: u16 = (1..=12)
                    .map(|month| u16::from(calendar.days_in_month(year, month).unwrap()))
                    .sum();
                assert_eq!(length, months);
                assert!(length == 354 || length == 355);
                assert_eq!(calendar.is_leap_year(year), Ok(length == 355));
            }
            let mut windu = FIRST_YEAR;
            while windu + 8 <= LAST_YEAR + 1 {
                let length = calendar
                    .new_year(windu + 8)
                    .map_or_else(|| calendar.latest().0 + 1, |rd| rd.0)
                    - calendar.new_year(windu).unwrap().0;
                let dropped = (windu..windu + 8)
                    .filter(|y| calendar.drops_day(*y))
                    .count();
                assert_eq!(
                    length,
                    WINDU_DAYS - dropped as i64,
                    "{} {windu}",
                    calendar.id
                );
                windu += 8;
            }
        }
        assert_eq!(Kurup::new(0, 1, None).name(), None);
        assert_eq!(Kurup::new(8, 1, None).name(), None);
    }

    #[test]
    fn the_range_ends_with_jimakir_2346() {
        assert_eq!(JAVANESE.latest(), greg(2401, 12, 6));
        assert_eq!(JAVANESE_YOGYAKARTA.latest(), greg(2401, 12, 6));
        assert_eq!(JAVANESE_ABOGE.latest(), greg(2401, 12, 11));
        for calendar in RECKONINGS {
            let last = calendar.latest();
            let date = calendar.from_fixed(last).unwrap();
            assert_eq!((date.year, date.month), (LAST_YEAR, 12));
            assert_eq!(
                calendar.from_fixed(Rd(last.0 + 1)),
                Err(CalendarError::AfterSupportedRange)
            );
            assert_eq!(
                calendar.from_fixed(Rd(EPOCH.0 - 1)),
                Err(CalendarError::BeforeEpoch)
            );
            assert_eq!(
                calendar.to_fixed(JavaneseDate::new(FIRST_YEAR - 1, 12, 29)),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                calendar.to_fixed(JavaneseDate::new(LAST_YEAR + 1, 1, 1)),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                calendar.is_leap_year(LAST_YEAR + 1),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(calendar.meta().latest, Some(last));
        }
        assert_eq!(JAVANESE.is_leap_year(LAST_YEAR), Ok(false));
        assert_eq!(JAVANESE_ABOGE.is_leap_year(LAST_YEAR), Ok(true));
    }

    #[test]
    fn every_day_of_the_range_round_trips() {
        for calendar in RECKONINGS {
            let (first, last) = (EPOCH.0, calendar.latest().0);
            let mut previous: Option<JavaneseDate> = None;
            for rd in (first..=last).step_by(crate::sweep_stride(7)) {
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{}", calendar.id);
                if let Some(before) = previous {
                    assert!(before < date);
                }
                previous = Some(date);
            }
        }
    }

    #[test]
    fn fields_round_trip_and_carry_the_windu_and_the_kurup() {
        for calendar in RECKONINGS {
            for rd in (EPOCH.0..=calendar.latest().0).step_by(997) {
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                let fields = calendar.to_fields(date).unwrap();
                assert_eq!(fields.era, Some(ERA));
                assert_eq!(calendar.from_fields(&fields), Ok(date));
                assert_eq!(
                    Calendar::days_in_month(&calendar, &fields),
                    Ok(u16::from(
                        calendar.days_in_month(date.year, date.month).unwrap()
                    ))
                );
            }
        }
        let fields = JAVANESE.to_fields(JavaneseDate::new(1959, 1, 1)).unwrap();
        assert_eq!(fields.extra.get("taun"), Some(5));
        assert_eq!(fields.extra.get("windu"), Some(4));
        assert_eq!(fields.extra.get("kurup"), Some(4));
        let aboge = JAVANESE_ABOGE
            .to_fields(JavaneseDate::new(1959, 1, 1))
            .unwrap();
        assert_eq!(aboge.extra.get("kurup"), Some(3));
        assert_eq!(
            JAVANESE.from_fields(&DateFields::ymd(1959, 1, 1).with_era("ah")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            JAVANESE.from_fields(&DateFields::ymd_leap_month(1959, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            JAVANESE.from_fields(&DateFields::ymd(1957, 12, 30)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            JAVANESE.from_fields(&DateFields::ymd(1957, 13, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            Calendar::days_in_month(&JAVANESE, &DateFields::ymd(1553, 1, 1)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            Calendar::days_in_month(&JAVANESE, &DateFields::ymd_leap_month(1959, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            Calendar::days_in_month(&JAVANESE, &DateFields::ymd(1959, 13, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn the_names_and_the_metadata() {
        assert_eq!(MONTHS.len(), MONTHS_JAVANESE_SCRIPT.len());
        assert_eq!(JavaneseDate::new(1959, 12, 1).month_name(), Ok("Besar"));
        assert_eq!(
            JavaneseDate::new(1959, 13, 1).month_name(),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(WUNTU.iter().filter(|long| **long).count(), 3);
        assert_eq!(
            DAYS_BEFORE_TAUN[7] + 355,
            WINDU_DAYS,
            "five wastu and three wuntu years"
        );
        assert_eq!(WINDU_DAYS % 35, 0, "a windu is 81 wetonan");
        for calendar in RECKONINGS {
            let meta = calendar.meta();
            assert_eq!(meta.native_locales, &["jv"]);
            assert!(!meta.is_astronomical && !meta.has_leap_months);
            assert_eq!(
                calendar.day_boundary(),
                DayBoundary::Sunset(DayNaming::ByEnd)
            );
            assert_eq!(calendar.era_name(ERA).map(|name| name.latin()), Some("AJ"));
            assert_eq!(calendar.era_name("ah"), None);
            assert!(calendar.usage().is_recorded());
        }
        assert_eq!(JavaneseCalendar::default(), JAVANESE);
        assert_eq!(JAVANESE.meta().id, CalendarId("javanese"));
        assert_eq!(JAVANESE_ABOGE.meta().id, CalendarId("javanese-aboge"));
        assert_eq!(
            JAVANESE_YOGYAKARTA.meta().id,
            CalendarId("javanese-yogyakarta")
        );
        assert_eq!(JAVANESE.kurup(FIRST_YEAR - 1), None);
        assert_eq!(JAVANESE.days_in_year(LAST_YEAR + 1), None);
        assert_eq!(JAVANESE.days_in_month(1959, 0), None);
    }
}
