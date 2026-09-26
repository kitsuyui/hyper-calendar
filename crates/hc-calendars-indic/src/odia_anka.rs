//! The Odia *aṅka*, the regnal years of the Gajapati of Puri, from Suniā —
//! `odia-anka`.
//!
//! The reckoning is written up in `docs/systems/odia-anka.md` in the
//! repository: the numbers it drops, the first Anka of a reign, the year
//! from Bhādrapada śukla 12, the reign carried and the sources, keyed in
//! `docs/references.bib`. This page summarises it and states the code's own
//! facts.
//!
//! # What this is
//!
//! The Anka "changes its numerical designation every 12th day of
//! Bhadrapada-suddha", on the pūrṇimānta months, and in its notation "the
//! years whose numeral is 6, or whose numerals end with 6 or \[0\] (except
//! 10), are dropped" (Sewell and Dikshit, *The Indian Calendar*, 1896,
//! Art. 64). A reign's first full year, opened by the first Suniā after the
//! accession, is its 2nd Anka, so the numbers run 2, 3, 4, 5, 7, … 15, 17,
//! 18, 19, 21, … and 1, 6, 16, 20, 26, 30 and 36 never name a year. The
//! Anka is therefore an integer *mapping* from the count of full years to
//! the number printed — [`anka_of_regnal_year`] and its inverse
//! [`regnal_year_of_anka`] — and nothing here obtains one Anka from another
//! by arithmetic on the number.
//!
//! The reign carried is Dibyasingha Deb's, in office from 7 July 1970, whose
//! 2nd Anka opened at the Suniā of 1970 ([`DIBYASINGHA_DEB`]); the Gajapati
//! declared his 68th on 15 September 2024, his 69th on 4 September 2025 and
//! his 71st in September 2026, and the tests hold the calendar to those.
//!
//! # The day the year turns
//!
//! [`OdiaAnkaCalendar::suniya`] is the first day of nija Bhādrapada's
//! bright fortnight whose sunrise tithi is 12 or later — śukla 12, unless
//! that tithi holds no sunrise — read by the pūrṇimānta calendar this one is
//! built on. A date
//! keeps its month, its tithi and its intercalary or repeated day exactly as
//! [`crate::hindu_purnimanta`] has them; only the year is the Anka. The two
//! cases no source read settles, an intercalary Bhādrapada and a śukla 12
//! without a sunrise, are the system document's.

use hc_calendar::shape::{CycleShape, LUNISOLAR_TWELVE};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_solar::gregorian;

use crate::hindu_lunar::{GREGORIAN_YEAR_OFFSET, HinduLunarDate, MAX_YEAR};
use crate::hindu_purnimanta::HinduPurnimantaCalendar;

/// The calendar's identifier.
pub const ID: CalendarId = CalendarId("odia-anka");

/// Bhādrapada, the month whose śukla 12 opens the year.
pub const BHADRAPADA: u8 = 6;

/// The tithi that opens the year: śukla dvādaśī, the 12th of the bright
/// fortnight.
pub const DVADASHI: u8 = 12;

/// What to subtract from the Gregorian year of a Suniā for the Amli year
/// that opens on it: the era's epoch is A.D. 592–93 (Sewell and Dikshit,
/// Arts. 52 and 71).
pub const AMLI_OFFSET: i64 = 592;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Sewell and Dikshit 1896, Art. 64 [sewell1896], for the Onko reckoning from \
    Bhadrapada sukla 12 at Puri; the Anka of Dibyasingha Deb as declared on Sunia 2011 and \
    2024-2026 [puriwaves-sunia-52, odishatv-sunia-2024, odishatv-sunia-2025, odishatv-sunia-2026], \
    counted from the first Sunia of his reign, 1970, as docs/systems/odia-anka.md states";

/// A reign the Anka counts: the era code its dates carry, the ruler's name,
/// and the Śaka year of the first Suniā after the accession, which opens
/// the reign's 2nd Anka.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reign {
    /// The era code, lower-case kebab-case.
    pub era: &'static str,
    /// The ruler's name, as the sources romanise it.
    pub name: &'static str,
    /// The Śaka year whose nija Bhādrapada śukla 12 is the reign's first
    /// Suniā.
    pub first_saka: i64,
}

/// The reign of Dibyasingha Deb, Gajapati from 7 July 1970, whose first
/// Suniā fell in Śaka 1892, September 1970.
pub const DIBYASINGHA_DEB: Reign = Reign {
    era: "dibyasingha-deb",
    name: "Dibyasingha Deb",
    first_saka: 1970 - GREGORIAN_YEAR_OFFSET,
};

/// Whether `number` is one an Anka can be: not 1, not ending in 6, and not
/// ending in 0 unless it is 10.
#[must_use]
pub const fn is_anka(number: i64) -> bool {
    number >= 2 && number % 10 != 6 && (number % 10 != 0 || number == 10)
}

/// The Anka numbers of the first seven full years, which end at 9.
const FIRST_DECADE: [i64; 7] = [2, 3, 4, 5, 7, 8, 9];

/// The Anka numbers from 10 to 19, the one decade that keeps its 0.
const SECOND_DECADE: [i64; 9] = [10, 11, 12, 13, 14, 15, 17, 18, 19];

/// The units every later decade keeps.
const LATER_UNITS: [i64; 8] = [1, 2, 3, 4, 5, 7, 8, 9];

/// The Anka of a reign's `regnal`th full year, the first being the one the
/// first Suniā after the accession opens: 1 is Anka 2, 5 is 7, 14 is 17,
/// 57 is 71. `None` for a year before the first.
#[must_use]
pub const fn anka_of_regnal_year(regnal: i64) -> Option<i64> {
    if regnal < 1 {
        return None;
    }
    let first = FIRST_DECADE.len() as i64;
    let second = SECOND_DECADE.len() as i64;
    if regnal <= first {
        return Some(FIRST_DECADE[(regnal - 1) as usize]);
    }
    if regnal <= first + second {
        return Some(SECOND_DECADE[(regnal - first - 1) as usize]);
    }
    let later = regnal - first - second - 1;
    let per_decade = LATER_UNITS.len() as i64;
    Some((2 + later / per_decade) * 10 + LATER_UNITS[(later % per_decade) as usize])
}

/// The full year of a reign an Anka number names: the inverse of
/// [`anka_of_regnal_year`], and `None` for a number no year has — 1, 6, 16,
/// 20, 26 and the rest.
#[must_use]
pub const fn regnal_year_of_anka(anka: i64) -> Option<i64> {
    if !is_anka(anka) {
        return None;
    }
    if anka < 10 {
        // 2 to 5 are the first four years, 7 to 9 the next three.
        return Some(if anka < 6 { anka - 1 } else { anka - 2 });
    }
    if anka < 20 {
        // Seven years before 10; 10 to 15, then 17 to 19.
        return Some(if anka < 16 { anka - 2 } else { anka - 3 });
    }
    // Sixteen years before 20, then eight in every decade: units 1 to 5 are
    // the first five, 7 to 9 the last three.
    let unit = anka % 10;
    let within = if unit < 6 { unit - 1 } else { unit - 2 };
    Some(16 + (anka / 10 - 2) * 8 + within + 1)
}

/// A date in the Anka reckoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OdiaAnkaDate {
    /// The Anka: never 1, never a number ending in 6, never one ending in 0
    /// but 10.
    pub anka: i64,
    /// The pūrṇimānta month, 1 for Chaitra through 12 for Phālguna, as
    /// [`crate::hindu_purnimanta`] numbers it. The year opens in
    /// Bhādrapada, month 6, and ends in the next Bhādrapada.
    pub month: u8,
    /// Whether this is the intercalary (*adhika*) month of that name.
    pub leap_month: bool,
    /// The tithi, 1 through 30: 1–15 the bright fortnight, 16–30 the dark.
    pub day: u8,
    /// Whether this is the second day to carry that tithi at sunrise.
    pub leap_day: bool,
}

/// The Anka reckoning of a reign over a pūrṇimānta calendar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OdiaAnkaCalendar {
    lunar: HinduPurnimantaCalendar,
    reign: Reign,
}

impl Default for OdiaAnkaCalendar {
    fn default() -> Self {
        Self::PURI
    }
}

impl OdiaAnkaCalendar {
    /// Dibyasingha Deb's Anka over the *Rashtriya Panchang*'s pūrṇimānta
    /// reckoning — the Central Station's sunrise, the Lahiri ayanamsa: the
    /// registered `odia-anka`.
    pub const PURI: Self = Self::new(HinduPurnimantaCalendar::RASHTRIYA, DIBYASINGHA_DEB);

    /// Another reign's Anka, or the same one over another pūrṇimānta
    /// calendar — Puri's sunrise, for a local almanac.
    #[must_use]
    pub const fn new(lunar: HinduPurnimantaCalendar, reign: Reign) -> Self {
        Self { lunar, reign }
    }

    /// The reign this calendar counts.
    #[must_use]
    pub const fn reign(&self) -> Reign {
        self.reign
    }

    /// The Suniā of a Śaka year, where the Anka turns: the first day of
    /// nija Bhādrapada's bright fortnight whose sunrise tithi is śukla 12
    /// or later.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside the lunar calendar's
    /// range.
    pub fn suniya(&self, saka: i64) -> CalendarResult<Rd> {
        suniya_of(&self.lunar, saka)
    }

    /// The Śaka year of the Suniā that opened the Anka year a day is in.
    fn opening_saka(&self, rd: Rd) -> CalendarResult<i64> {
        let (gregorian_year, _, _) = gregorian::from_fixed(rd)?;
        let saka = gregorian_year - GREGORIAN_YEAR_OFFSET;
        if saka > MAX_YEAR {
            // The lunar calendar ends in March, before any Suniā.
            return Ok(saka - 1);
        }
        Ok(if rd >= self.suniya(saka)? {
            saka
        } else {
            saka - 1
        })
    }

    /// The first day of the reign's 2nd Anka, the earliest day converted.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] for a reign outside the lunar
    /// calendar's range.
    pub fn earliest(&self) -> CalendarResult<Rd> {
        self.suniya(self.reign.first_saka)
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] for a number no Anka has or a
    /// year outside the range, [`CalendarError::MonthOutOfRange`] for a
    /// month outside 1–12, and otherwise as
    /// [`HinduPurnimantaCalendar::to_fixed`]; a tithi of Bhādrapada the year
    /// does not hold — śukla 5 at its start, say, which is the year
    /// before's — is [`CalendarError::DayOutOfRange`].
    pub fn to_fixed(&self, date: OdiaAnkaDate) -> CalendarResult<Rd> {
        let regnal = regnal_year_of_anka(date.anka).ok_or(CalendarError::YearOutOfRange)?;
        if !(1..=12).contains(&date.month) {
            return Err(CalendarError::MonthOutOfRange);
        }
        let opening = self.reign.first_saka + regnal - 1;
        // The year spans the end of one Śaka year and the start of the
        // next, and the pūrṇimānta dark fortnight of Chaitra closes the
        // Śaka year before; whichever Śaka year gives a day in this Anka
        // is the one.
        let mut last_error = CalendarError::DayOutOfRange;
        for saka in [opening, opening + 1, opening - 1] {
            let lunar = HinduLunarDate {
                year: saka,
                month: date.month,
                leap_month: date.leap_month,
                day: date.day,
                leap_day: date.leap_day,
            };
            match self.lunar.to_fixed(lunar) {
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
            _ => CalendarError::DayOutOfRange,
        })
    }

    /// The date on a fixed day.
    ///
    /// # Errors
    ///
    /// [`CalendarError::BeforeEpoch`] before the reign's 2nd Anka, and as
    /// [`HinduPurnimantaCalendar::from_fixed`] after the lunar calendar's
    /// range.
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<OdiaAnkaDate> {
        let lunar = self.lunar.from_fixed(rd)?;
        let regnal = self.opening_saka(rd)? - self.reign.first_saka + 1;
        let anka = anka_of_regnal_year(regnal).ok_or(CalendarError::BeforeEpoch)?;
        Ok(OdiaAnkaDate {
            anka,
            month: lunar.month,
            leap_month: lunar.leap_month,
            day: lunar.day,
            leap_day: lunar.leap_day,
        })
    }

    /// The Śaka year whose Suniā opens an Anka.
    fn opening_of(&self, anka: i64) -> CalendarResult<i64> {
        regnal_year_of_anka(anka)
            .map(|regnal| self.reign.first_saka + regnal - 1)
            .ok_or(CalendarError::YearOutOfRange)
    }

    /// New Year's Day of an Anka: its Suniā.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] for a number no Anka has, or
    /// outside the lunar calendar's range.
    pub fn new_year(&self, anka: i64) -> CalendarResult<Rd> {
        self.suniya(self.opening_of(anka)?)
    }
}

/// The Suniā of a Śaka year over a pūrṇimānta calendar; see
/// [`OdiaAnkaCalendar::suniya`]. The bright fortnight has the same name
/// and year in the amānta reckoning, which is where it is looked up.
///
/// # Errors
///
/// [`CalendarError::YearOutOfRange`] outside the lunar calendar's range.
pub fn suniya_of(lunar: &HinduPurnimantaCalendar, saka: i64) -> CalendarResult<Rd> {
    let mut last_error = CalendarError::DayOutOfRange;
    for day in DVADASHI..=15 {
        let date = HinduLunarDate {
            year: saka,
            month: BHADRAPADA,
            leap_month: false,
            day,
            leap_day: false,
        };
        match lunar.amanta.to_fixed(date) {
            Ok(rd) => return Ok(rd),
            Err(error @ CalendarError::YearOutOfRange) => return Err(error),
            Err(error) => last_error = error,
        }
    }
    Err(last_error)
}

impl Calendar for OdiaAnkaCalendar {
    type Date = OdiaAnkaDate;

    /// From the reign's first Suniā, and declared every year since.
    fn usage(&self) -> hc_calendar::Usage {
        match self.earliest() {
            Ok(first) => hc_calendar::Usage::since(first, USAGE_SOURCE),
            Err(_) => hc_calendar::Usage::UNRECORDED,
        }
    }

    /// The pūrṇimānta months, with a thirteenth in an intercalary year, and
    /// the seven-day week.
    fn cycles(&self) -> &'static [CycleShape] {
        LUNISOLAR_TWELVE
    }

    /// An Anka with an adhika month: after its Suniā in the Śaka year it
    /// opens in — Āśvina to Phālguna — or before the next Suniā in the
    /// next — Chaitra to an adhika Bhādrapada, which precedes the nija
    /// month the year turns in.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] for a number no Anka has.
    fn is_leap_year(&self, anka: i64) -> CalendarResult<bool> {
        let saka = self.opening_of(anka)?;
        let autumn = self
            .lunar
            .amanta
            .leap_month_of(saka)?
            .is_some_and(|(month, _, _)| month > BHADRAPADA);
        let spring = match self.lunar.amanta.leap_month_of(saka + 1) {
            Ok(leap) => leap.is_some_and(|(month, _, _)| month <= BHADRAPADA),
            // The lunar calendar's last year: the rest is not converted.
            Err(CalendarError::YearOutOfRange) if saka == MAX_YEAR => false,
            Err(error) => return Err(error),
        };
        Ok(autumn || spring)
    }

    /// The day boundary of the pūrṇimānta calendar the dates are read on.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        Calendar::day_boundary(&self.lunar)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Odia Anka (Gajapati of Puri)",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: self.earliest().ok(),
            latest: Calendar::meta(&self.lunar).latest,
            native_locales: &["or"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        OdiaAnkaCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        OdiaAnkaCalendar::from_fixed(self, rd)
    }

    /// The Anka as the year under the reign's era code, the pūrṇimānta
    /// month and tithi, and two derived extras, ignored on input: the
    /// `regnal-year`, the full year of the reign, and the `amli-year`
    /// that opened on the same Suniā.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let month = if date.leap_month {
            Month::leap(date.month)
        } else {
            Month::regular(date.month)
        };
        let regnal = regnal_year_of_anka(date.anka).ok_or(CalendarError::YearOutOfRange)?;
        let opening = self.reign.first_saka + regnal - 1;
        let mut fields = DateFields::ymd(date.anka, date.month, date.day)
            .with_era(self.reign.era)
            .with_extra("regnal-year", regnal)?
            .with_extra("amli-year", opening + GREGORIAN_YEAR_OFFSET - AMLI_OFFSET)?;
        fields.month = Some(month);
        fields.leap_day = date.leap_day;
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != self.reign.era) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        let date = OdiaAnkaDate {
            anka: fields.year,
            month: month.ordinal,
            leap_month: month.leap,
            day: fields.require_day()?,
            leap_day: fields.leap_day,
        };
        OdiaAnkaCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANKA: OdiaAnkaCalendar = OdiaAnkaCalendar::PURI;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    #[test]
    fn the_dropped_numbers_are_those_sewell_and_dikshit_list() {
        // Sewell and Dikshit, Art. 64: "the 1st — possibly, the 6th, 16th,
        // 20th, 26th, 30th, 36th, 40th, 46th, 50th, 56th"; the 7th follows
        // the 5th and the 21st the 19th, and 10 is kept.
        let dropped: alloc::vec::Vec<i64> = (1..=60).filter(|n| !is_anka(*n)).collect();
        assert_eq!(dropped, [1, 6, 16, 20, 26, 30, 36, 40, 46, 50, 56, 60]);
        assert!(is_anka(10));
        for number in [
            1, 6, 16, 20, 26, 30, 36, 40, 46, 50, 56, 60, 66, 70, 100, 106,
        ] {
            assert_eq!(regnal_year_of_anka(number), None, "{number}");
        }
        let after = |anka| anka_of_regnal_year(regnal_year_of_anka(anka).expect("an Anka") + 1);
        assert_eq!(after(5), Some(7));
        assert_eq!(after(19), Some(21));
        assert_eq!(after(9), Some(10));
        assert_eq!(after(69), Some(71));
    }

    #[test]
    fn the_first_thirty_regnal_years_are_wikipedias_table() {
        // Wikipedia, "Anka year", the table of regnal years 1–30.
        let table = [
            2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 21, 22, 23, 24, 25, 27, 28,
            29, 31, 32, 33, 34, 35, 37,
        ];
        for (regnal, anka) in (1..).zip(table) {
            assert_eq!(
                anka_of_regnal_year(regnal),
                Some(anka),
                "regnal year {regnal}"
            );
        }
        assert_eq!(anka_of_regnal_year(0), None);
    }

    #[test]
    fn the_mapping_and_its_inverse_agree() {
        let mut previous = 1;
        for regnal in 1..=1_000 {
            let anka = anka_of_regnal_year(regnal).expect("a year of the reign");
            assert!(is_anka(anka), "{anka}");
            assert_eq!(regnal_year_of_anka(anka), Some(regnal));
            // Increasing, and every number skipped on the way is dropped.
            assert!((previous + 1..anka).all(|between| !is_anka(between)));
            previous = anka;
        }
    }

    #[test]
    fn suniya_falls_on_sewell_and_dikshits_days() {
        // Sewell and Dikshit, Art. 64: "Onko 2 of Mukundadeva …
        // September 2, 1797", of Ramachandradeva September 22, 1817, of
        // Virakesvaradeva September 4, 1854, of Divyasimhadeva September 8,
        // 1859, each "Bhadrapada sukla 12th".
        for (year, day) in [(1797, 2), (1817, 22), (1854, 4), (1859, 8)] {
            assert_eq!(
                ANKA.suniya(year - GREGORIAN_YEAR_OFFSET),
                Ok(ymd(year, 9, day)),
                "{year}"
            );
        }
    }

    #[test]
    fn the_printed_ankas_of_2011_and_2024_to_2026() {
        // "52 anka 1419" (PuriWaves); "68 'Anka'", "1432 'Sal'" on
        // 15 September 2024 and the 69th Anka with 1433 Sal on 4 September
        // 2025 (OdishaTV); the 71st with 1434 Sal in September 2026
        // (OdishaTV, 24 September; Kalinga TV, 23 September).
        for ((year, month, day), anka, amli) in [
            ((2011, 9, 9), 52, 1419),
            ((2024, 9, 15), 68, 1432),
            ((2025, 9, 4), 69, 1433),
            ((2026, 9, 23), 71, 1434),
        ] {
            let rd = ymd(year, month, day);
            assert_eq!(ANKA.new_year(anka), Ok(rd), "{anka}");
            let date = ANKA.from_fixed(rd).expect("in range");
            assert_eq!(
                (date.anka, date.month, date.leap_month, date.day),
                (anka, BHADRAPADA, false, DVADASHI),
                "{year}"
            );
            let fields = Calendar::to_fields(&ANKA, date).expect("fields");
            assert_eq!(fields.era, Some("dibyasingha-deb"));
            assert_eq!(fields.extra.get("amli-year"), Some(amli), "{year}");
            let eve = ANKA.from_fixed(Rd(rd.0 - 1)).expect("in range");
            let before = anka_of_regnal_year(regnal_year_of_anka(anka).expect("an Anka") - 1);
            assert_eq!(Some(eve.anka), before, "{year}");
            assert_eq!((eve.month, eve.day), (BHADRAPADA, DVADASHI - 1), "{year}");
        }
        // 67 in 2023: the Anka before 68 is 67, and before 71 is 69.
        assert_eq!(ANKA.from_fixed(ymd(2024, 9, 14)).map(|d| d.anka), Ok(67));
        assert_eq!(ANKA.from_fixed(ymd(2026, 9, 22)).map(|d| d.anka), Ok(69));
    }

    #[test]
    fn the_reign_begins_at_the_suniya_of_1970() {
        let first = ANKA.earliest().expect("in range");
        // The 12th held no sunrise at the Central Station in 1970; the
        // year opens on the day of the 13th.
        assert_eq!(first, ymd(1970, 9, 13));
        let date = ANKA.from_fixed(first).expect("in range");
        assert_eq!((date.anka, date.month, date.day), (2, BHADRAPADA, 13));
        assert_eq!(
            ANKA.from_fixed(Rd(first.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(Calendar::meta(&ANKA).earliest, Some(first));
        assert_eq!(Calendar::usage(&ANKA).from, Some(first));
    }

    #[test]
    fn every_day_of_three_years_converts_and_converts_back() {
        let start = ANKA.new_year(67).expect("in range");
        let end = ANKA.new_year(71).expect("in range");
        let mut intercalary = alloc::vec::Vec::new();
        for day in (start.0..end.0).step_by(crate::sweep_stride(3)) {
            let rd = Rd(day);
            let date = ANKA.from_fixed(rd).expect("in range");
            assert_eq!(ANKA.to_fixed(date), Ok(rd), "{date:?}");
            let fields = Calendar::to_fields(&ANKA, date).expect("fields");
            assert_eq!(Calendar::from_fields(&ANKA, &fields), Ok(date));
            if date.leap_month && !intercalary.contains(&date.anka) {
                intercalary.push(date.anka);
            }
        }
        // The adhika Jyeṣṭha of Śaka 1948, May–June 2026, is in the 69th.
        assert_eq!(intercalary, [69]);
        for (anka, leap) in [(67, false), (68, false), (69, true), (71, false)] {
            assert_eq!(Calendar::is_leap_year(&ANKA, anka), Ok(leap), "{anka}");
        }
    }

    #[test]
    fn an_intercalary_bhadrapada_is_passed_to_the_nija_month() {
        // 2012 has an adhika Bhādrapada: Suniā is the nija month's śukla
        // 12, 27 September, and the adhika month is the end of the 52nd.
        let suniya = ANKA.new_year(53).expect("in range");
        assert_eq!(suniya, ymd(2012, 9, 27));
        let adhika = ANKA.from_fixed(ymd(2012, 8, 29)).expect("in range");
        assert_eq!(
            (adhika.anka, adhika.month, adhika.leap_month),
            (52, 6, true)
        );
        assert_eq!(Calendar::is_leap_year(&ANKA, 52), Ok(true));
    }

    #[test]
    fn dropped_numbers_and_foreign_eras_are_refused() {
        let date = ANKA.from_fixed(ymd(2025, 10, 1)).expect("in range");
        assert_eq!(date.anka, 69);
        for anka in [1, 6, 16, 20, 60, 70] {
            assert_eq!(
                ANKA.to_fixed(OdiaAnkaDate { anka, ..date }),
                Err(CalendarError::YearOutOfRange),
                "{anka}"
            );
            assert_eq!(
                Calendar::is_leap_year(&ANKA, anka),
                Err(CalendarError::YearOutOfRange)
            );
        }
        assert_eq!(
            ANKA.to_fixed(OdiaAnkaDate { month: 13, ..date }),
            Err(CalendarError::MonthOutOfRange)
        );
        // Bhādrapada appears at both ends of a year and no tithi twice:
        // śukla 5 of the 69th is in September 2026, at its end, and of the
        // 71st in September 2027. The 2nd Anka opened on śukla 13 in 1970,
        // the 12th holding no sunrise, so it has no Bhādrapada śukla 12.
        let end = OdiaAnkaDate {
            anka: 69,
            month: BHADRAPADA,
            leap_month: false,
            day: 5,
            leap_day: false,
        };
        let day = ANKA.to_fixed(end).expect("in the 69th");
        assert!(ymd(2026, 9, 1) < day && day < ymd(2026, 9, 23), "{day:?}");
        let later = ANKA
            .to_fixed(OdiaAnkaDate { anka: 71, ..end })
            .expect("in the 71st");
        assert!(
            ymd(2027, 9, 1) < later && later < ymd(2027, 9, 30),
            "{later:?}"
        );
        assert_eq!(
            ANKA.to_fixed(OdiaAnkaDate {
                anka: 2,
                day: DVADASHI,
                ..end
            }),
            Err(CalendarError::DayOutOfRange)
        );
        let mut fields = Calendar::to_fields(&ANKA, date).expect("fields");
        fields.era = Some("saka");
        assert_eq!(
            Calendar::from_fields(&ANKA, &fields),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn the_day_is_the_purnimanta_calendars() {
        let lunar = HinduPurnimantaCalendar::RASHTRIYA;
        for day in (ymd(2024, 1, 1).0..ymd(2027, 1, 1).0).step_by(7) {
            let rd = Rd(day);
            let anka = ANKA.from_fixed(rd).expect("in range");
            let purnimanta = lunar.from_fixed(rd).expect("in range");
            assert_eq!(
                (anka.month, anka.leap_month, anka.day, anka.leap_day),
                (
                    purnimanta.month,
                    purnimanta.leap_month,
                    purnimanta.day,
                    purnimanta.leap_day
                )
            );
        }
        assert_eq!(
            Calendar::day_boundary(&ANKA),
            Calendar::day_boundary(&lunar)
        );
    }
}
