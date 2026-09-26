//! Nepal Sambat, the lunisolar calendar of the Newar people: the amānta
//! lunar months under their Newar names, the year opening at Kachhalā —
//! `nepal-sambat`.
//!
//! The system is written up in `docs/systems/nepal-calendars.md` in the
//! repository: the calendar's history and standing, the months and their
//! full moons, the year count, whose sunrise the day is read at, Mha Puja
//! of 1134 worked through by hand, what is carried and what is not, the
//! checks against the published New Year's Days, and the sources, keyed
//! in `docs/references.bib`. This page summarises it and states the
//! code's own facts.
//!
//! The months are the amānta months of [`crate::hindu_lunar`], each
//! renamed — Kachhalā is Kārtika, the month whose full moon is Kārtik
//! Purnimā, and so on round the twelve — and the year begins with
//! Kachhalā, on Kārtika śukla pratipadā, the day of Mha Puja during the
//! Swanti festival. A date keeps its tithi, its fortnight and its
//! intercalary or skipped days exactly as the amānta calendar has them.
//!
//! | # | Month | Amānta month |
//! | --- | --- | --- |
//! | 1 | Kachhalā | Kārtika |
//! | 2 | Thinlā | Mārgaśīrṣa |
//! | 3 | Pwanhelā | Pauṣa |
//! | 4 | Silā | Māgha |
//! | 5 | Chilā | Phālguna |
//! | 6 | Chaulā | Chaitra |
//! | 7 | Bachhalā | Vaiśākha |
//! | 8 | Tachhalā | Jyeṣṭha |
//! | 9 | Dilā | Āṣāḍha |
//! | 10 | Gunlā | Śrāvaṇa |
//! | 11 | Yanlā | Bhādrapada |
//! | 12 | Kaulā | Āśvina |
//!
//! Year *N* begins in the autumn of Gregorian year *N* + 879 — New Year's
//! Day of 1134 was 4 November 2013 — and the epoch, 20 October 879, opens
//! year 0. Against the Śaka years of `hindu-lunar`, Kachhalā to Chilā of
//! year *N* are in Śaka *N* + 801 and Chaulā to Kaulā in Śaka *N* + 802.
//! [`NepalSambatCalendar::KATHMANDU`], the registered calendar, reads the
//! day at Kathmandu's sunrise with the Lahiri ayanamsa; the almanac of
//! Nepal's calendar committee was not read, and [`NepalSambatCalendar::new`]
//! takes another place and ayanamsa.
//!
//! # What is not here
//!
//! The solar Nepal Sambat that Lalitpur Metropolitan City devised from year
//! 1141, whose months run fixed Gregorian dates from 20 October. The one
//! source read does place the leap day — its table gives Chaulā 29 days in
//! regular years and 30 in leap years, so the extra day falls in the Chaulā
//! of a Gregorian leap year — but it states the leap rule itself only as "a
//! similar pattern" to the Gregorian one, cites the scheme to a
//! calendar-maker's site and a blog, and names no body that keeps it. A
//! calendar whose rule rests on that is not carried. And the range: the amānta engine answers from
//! Gregorian 1700 to 2299, so the inscriptions dated in Malla Nepal, whose
//! calendar this was, are out of it.
//!
//! Sources: Wikipedia, "Nepal Sambat", retrieved 2026-09-23, for the months,
//! their order and their full moons, the new year, the year count, the
//! epoch, the intercalary and reduced months, and the solar calendar;
//! Wikipedia, "Mha Puja", retrieved 2026-09-23, for New Year's Day in 2013,
//! 2014, 2016 and 2017, which the tests pin.

use hc_calendar::shape::{CycleShape, LUNISOLAR_TWELVE};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_seasons::zodiac::Ayanamsa;

use crate::hindu_lunar::{HinduLunarCalendar, HinduLunarDate};
use crate::places::KATHMANDU;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Established on 20 October 879 and the official calendar of Nepal until the end of the Malla \
    dynasty in 1769; displaced by the Bikram Sambat in 1903; the Newar calendar of Mha Puja \
    since, in Lalitpur's documents from 2020 and in the government's from 11 November 2023 \
    [wikipedia-nepal-sambat], as docs/systems/nepal-calendars.md states; the epoch is taken \
    as a Julian date";

/// The calendar's identifier.
pub const ID: CalendarId = CalendarId("nepal-sambat");
/// The era it counts in.
pub const ERA: &str = "nepal-sambat";

/// The amānta month, 1 for Chaitra, of Kachhalā: Kārtika.
const KACHHALA: u8 = 8;
/// Śaka year minus Nepal Sambat year for Kachhalā to Chilā.
const SAKA_OFFSET_AUTUMN: i64 = 801;
/// Śaka year minus Nepal Sambat year for Chaulā to Kaulā.
const SAKA_OFFSET_SPRING: i64 = 802;

/// The twelve months in Devanagari, Kachhalā first (Wikipedia, "Nepal
/// Sambat", the table of months).
pub const MONTHS_DEVANAGARI: [&str; 12] = [
    "कछला",
    "थिंला",
    "प्वँहेला",
    "सिला",
    "चिला",
    "चौला",
    "बछला",
    "तछला",
    "दिला",
    "गुंला",
    "ञंला",
    "कौला",
];

/// The twelve months in the Newa script (Prachalit), Kachhalā first, from
/// the same table.
pub const MONTHS_NEWA: [&str; 12] = [
    "𑐎𑐕𑐮𑐵",
    "𑐠𑐶𑑄𑐮𑐵",
    "𑐥𑑂𑐰𑑃𑐴𑐾𑐮𑐵",
    "𑐳𑐶𑐮𑐵",
    "𑐔𑐶𑐮𑐵",
    "𑐔𑑁𑐮𑐵",
    "𑐧𑐕𑐮𑐵",
    "𑐟𑐕𑐮𑐵",
    "𑐡𑐶𑐮𑐵",
    "𑐐𑐸𑑄𑐮𑐵",
    "𑐫𑑄𑐮𑐵",
    "𑐎𑑁𑐮𑐵",
];

/// A date in Nepal Sambat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NepalSambatDate {
    /// The Nepal Sambat year.
    pub year: i64,
    /// The month, 1 for Kachhalā through 12 for Kaulā.
    pub month: u8,
    /// Whether this is the intercalary month of that name — Analā, as the
    /// tradition calls every intercalary month — which precedes the
    /// ordinary one.
    pub leap_month: bool,
    /// The tithi, 1 through 30: 1–15 the bright fortnight (*thwa*), 16–30
    /// the dark (*gā*).
    pub day: u8,
    /// Whether this is the second day to carry that tithi at sunrise.
    pub leap_day: bool,
}

impl NepalSambatDate {
    /// The amānta date of the same day.
    const fn to_amanta(self) -> HinduLunarDate {
        let month = (self.month + 6) % 12 + 1;
        let offset = if month >= KACHHALA {
            SAKA_OFFSET_AUTUMN
        } else {
            SAKA_OFFSET_SPRING
        };
        HinduLunarDate {
            year: self.year + offset,
            month,
            leap_month: self.leap_month,
            day: self.day,
            leap_day: self.leap_day,
        }
    }

    /// The Nepal Sambat date of an amānta date.
    const fn from_amanta(date: HinduLunarDate) -> Self {
        let offset = if date.month >= KACHHALA {
            SAKA_OFFSET_AUTUMN
        } else {
            SAKA_OFFSET_SPRING
        };
        Self {
            year: date.year - offset,
            month: (date.month + 4) % 12 + 1,
            leap_month: date.leap_month,
            day: date.day,
            leap_day: date.leap_day,
        }
    }
}

/// The lunar Nepal Sambat, judged at a place with an ayanamsa.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NepalSambatCalendar {
    lunar: HinduLunarCalendar,
}

impl NepalSambatCalendar {
    /// Kathmandu's sunrise, the Lahiri ayanamsa: the registered
    /// `nepal-sambat`.
    pub const KATHMANDU: Self = Self::new(HinduLunarCalendar::new(KATHMANDU, Ayanamsa::LAHIRI));

    /// Nepal Sambat over any amānta calendar — another place's sunrise,
    /// another ayanamsa.
    #[must_use]
    pub const fn new(lunar: HinduLunarCalendar) -> Self {
        Self { lunar }
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::to_fixed`]: a year outside the engine's
    /// range, a month number outside 1–12, an intercalary month the year
    /// does not have, or a tithi the month skips.
    pub fn to_fixed(&self, date: NepalSambatDate) -> CalendarResult<Rd> {
        if !(1..=12).contains(&date.month) {
            return Err(CalendarError::MonthOutOfRange);
        }
        self.lunar.to_fixed(date.to_amanta())
    }

    /// The date on a fixed day.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::from_fixed`], outside the engine's range.
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<NepalSambatDate> {
        self.lunar.from_fixed(rd).map(NepalSambatDate::from_amanta)
    }

    /// New Year's Day of a Nepal Sambat year: the first day of its first
    /// Kachhalā — the intercalary one, in a year that has one.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside the engine's range.
    pub fn new_year(&self, year: i64) -> CalendarResult<Rd> {
        let saka = year + SAKA_OFFSET_AUTUMN;
        self.lunar
            .month_span(saka, KACHHALA, true)
            .or_else(|_| self.lunar.month_span(saka, KACHHALA, false))
            .map(|(first, _)| first)
    }

    /// The first day converted, the engine's.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::earliest`].
    pub fn earliest(&self) -> CalendarResult<Rd> {
        self.lunar.earliest()
    }

    /// The last day converted, the engine's.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::latest`].
    pub fn latest(&self) -> CalendarResult<Rd> {
        self.lunar.latest()
    }
}

impl Calendar for NepalSambatCalendar {
    type Date = NepalSambatDate;

    /// From its establishment on 20 October 879, official until 1769, and the
    /// calendar of the Newar new year since; the end of civil use is given by
    /// the year only and is not carried as a day.
    fn usage(&self) -> hc_calendar::Usage {
        match hc_calendars_solar::julian::to_fixed(879, 10, 20) {
            Ok(rd) => hc_calendar::Usage::since(rd, USAGE_SOURCE),
            Err(_) => hc_calendar::Usage::UNRECORDED,
        }
    }

    /// Twelve months with a thirteenth in an intercalary year, and the
    /// seven-day week.
    fn cycles(&self) -> &'static [CycleShape] {
        LUNISOLAR_TWELVE
    }

    /// A year with an adhika māsa. The year runs from Kachhalā of one Śaka
    /// year into the next, so the month may fall in either.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        let saka = year + SAKA_OFFSET_AUTUMN;
        let autumn = self
            .lunar
            .leap_month_of(saka)?
            .is_some_and(|(month, _, _)| month >= KACHHALA);
        let spring = self
            .lunar
            .leap_month_of(saka + 1)?
            .is_some_and(|(month, _, _)| month < KACHHALA);
        Ok(autumn || spring)
    }

    /// The day begins at sunrise.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Nepal Sambat (lunar)",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: self.earliest().ok(),
            latest: self.latest().ok(),
            native_locales: &["new", "ne"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        NepalSambatCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        NepalSambatCalendar::from_fixed(self, rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let month = if date.leap_month {
            Month::leap(date.month)
        } else {
            Month::regular(date.month)
        };
        let mut fields = DateFields::ymd(date.year, date.month, date.day).with_era(ERA);
        fields.month = Some(month);
        fields.leap_day = date.leap_day;
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        let date = NepalSambatDate {
            year: fields.year,
            month: month.ordinal,
            leap_month: month.leap,
            day: fields.require_day()?,
            leap_day: fields.leap_day,
        };
        NepalSambatCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    const NS: NepalSambatCalendar = NepalSambatCalendar::KATHMANDU;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    #[test]
    fn new_years_day_is_mha_puja() {
        // Wikipedia, "Mha Puja": 4 November 2013 (Nepal Sambat 1134),
        // 24 October 2014 (1135), 31 October 2016, 20 October 2017.
        for (year, month, day, ns) in [
            (2013, 11, 4, 1134),
            (2014, 10, 24, 1135),
            (2016, 10, 31, 1137),
            (2017, 10, 20, 1138),
        ] {
            let rd = ymd(year, month, day);
            assert_eq!(NS.new_year(ns), Ok(rd), "{ns}");
            let date = NS.from_fixed(rd).expect("in range");
            assert_eq!((date.year, date.month, date.day), (ns, 1, 1), "{ns}");
            // The day before is the last of Kaulā of the year before.
            let eve = NS.from_fixed(Rd(rd.0 - 1)).expect("in range");
            assert_eq!((eve.year, eve.month), (ns - 1, 12), "{ns}");
        }
    }

    #[test]
    fn the_year_lalitpur_began_dating_in_is_1140() {
        // "1140, i.e. mid 2020."
        assert_eq!(
            NS.from_fixed(ymd(2020, 7, 1)).map(|date| date.year),
            Ok(1140)
        );
    }

    #[test]
    fn each_full_moon_falls_in_the_gregorian_months_the_table_gives() {
        // The table's "corresponding Gregorian month" for each Newar month:
        // its full moon, tithi 15, falls in one of the two.
        const WHEN: [(u8, u8); 12] = [
            (10, 11),
            (11, 12),
            (12, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 8),
            (8, 9),
            (9, 10),
        ];
        for year in 1130..1160 {
            for (index, (first, second)) in WHEN.iter().enumerate() {
                let month = index as u8 + 1;
                let date = NepalSambatDate {
                    year,
                    month,
                    leap_month: false,
                    day: 15,
                    leap_day: false,
                };
                // A skipped fifteenth tithi takes the fourteenth's next day.
                let rd = NS.to_fixed(date).or_else(|_| {
                    NS.to_fixed(NepalSambatDate { day: 14, ..date })
                        .map(|rd| Rd(rd.0 + 1))
                });
                let rd = rd.expect("a full moon in range");
                let (_, gregorian_month, _) = gregorian::from_fixed(rd).expect("a day");
                assert!(
                    gregorian_month == *first || gregorian_month == *second,
                    "{year} month {month}: full moon in Gregorian month {gregorian_month}"
                );
            }
        }
    }

    #[test]
    fn years_run_353_to_355_days_or_383_to_385() {
        // The article: "spans from 353 to 355 (in leap 383 to 385) days".
        let mut seen = alloc::vec::Vec::new();
        for year in 1100..1200 {
            let length =
                NS.new_year(year + 1).expect("in range").0 - NS.new_year(year).expect("in range").0;
            assert!(
                (353..=355).contains(&length) || (383..=385).contains(&length),
                "Nepal Sambat {year} is {length} days"
            );
            seen.push(length);
        }
        assert!(seen.iter().any(|length| *length > 380));
    }

    #[test]
    fn every_day_of_three_years_converts_and_converts_back() {
        // Nepal Sambat 1138 to 1140, 2017 to 2020: three year boundaries and
        // two intercalary months, Tachhalā in 1138 and Kaulā in 1140. The
        // renaming is all this module adds to the amānta engine, which is
        // round-tripped over its own range. Every day in a release build,
        // every eleventh in a debug one, which still lands in every month.
        let mut intercalary = alloc::vec::Vec::new();
        for day in (NS.new_year(1138).expect("in range").0..NS.new_year(1141).expect("in range").0)
            .step_by(crate::sweep_stride(11))
        {
            let rd = Rd(day);
            let date = NS.from_fixed(rd).expect("in range");
            assert_eq!(NS.to_fixed(date), Ok(rd), "{date:?}");
            if date.leap_month && !intercalary.contains(&(date.year, date.month)) {
                intercalary.push((date.year, date.month));
            }
        }
        assert_eq!(intercalary, [(1138, 8), (1140, 12)]);
    }

    #[test]
    fn a_date_keeps_its_tithi_from_the_amanta_calendar_at_the_same_sunrise() {
        let amanta = HinduLunarCalendar::new(KATHMANDU, Ayanamsa::LAHIRI);
        // Every day in a release build, every eleventh in a debug one.
        for day in (ymd(2024, 1, 1).0..ymd(2026, 12, 31).0).step_by(crate::sweep_stride(11)) {
            let rd = Rd(day);
            let lunar = amanta.from_fixed(rd).expect("in range");
            let ns = NS.from_fixed(rd).expect("in range");
            assert_eq!(
                (ns.day, ns.leap_day, ns.leap_month),
                (lunar.day, lunar.leap_day, lunar.leap_month)
            );
            assert_eq!(ns.to_amanta(), lunar);
        }
    }

    #[test]
    fn fields_carry_the_era_and_refuse_another() {
        let date = NS.from_fixed(ymd(2025, 10, 22)).expect("in range");
        let fields = Calendar::to_fields(&NS, date).expect("fields");
        assert_eq!(fields.era, Some(ERA));
        assert_eq!(Calendar::from_fields(&NS, &fields), Ok(date));
        let mut wrong = fields;
        wrong.era = Some("Saka");
        assert_eq!(
            Calendar::from_fields(&NS, &wrong),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            NS.to_fixed(NepalSambatDate { month: 13, ..date }),
            Err(CalendarError::MonthOutOfRange)
        );
    }
}
