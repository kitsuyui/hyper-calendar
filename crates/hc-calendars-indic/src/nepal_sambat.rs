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
//! day at Kathmandu's sunrise with the Lahiri ayanāṃśa; the almanac of
//! Nepal's calendar committee, whose notice of 2024 is read for its
//! written form below, was not read, and [`NepalSambatCalendar::new`]
//! takes another place and ayanāṃśa.
//!
//! # The Samiti's form
//!
//! The Nepal Panchang Nirnayak Bikas Samiti, the almanac committee,
//! writes a date by the fortnight, a month's name with *thwa* or *gā*
//! joined to it, and the tithi within it, 1 to 15 and 30 for the new moon
//! (`nepal-panchang-committee-2081`, rules 7 and 8): कछलाथ्व १ is Mha Puja.
//! It is the same day under another written form, a second convention that
//! policy §5 names, [`NepalSambatFortnightCalendar`],
//! `nepal-sambat-fortnight`. The notice requires the weekday beside a
//! doubled tithi (rule 5) and gives no form for it, so none is carried.
//!
//! # What is not here
//!
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

use hc_calendar::shape::{CycleShape, LUNISOLAR_TWELVE, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_seasons::zodiac::Ayanamsa;

use crate::hindu_lunar::{HinduLunarCalendar, HinduLunarDate};
use crate::kartikadi;
use crate::places::KATHMANDU;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Established on 20 October 879 and the official calendar of Nepal until the end of the Malla \
    dynasty in 1769; displaced by the Bikram Sambat in 1903; the Newar calendar of Mha Puja \
    since, in Lalitpur's documents from 2020 and in the government's from 11 November 2023 \
    [wikipedia-nepal-sambat], as docs/systems/nepal-calendars.md states; the epoch is taken \
    as a Julian date";

/// The calendar's identifier.
pub const ID: CalendarId = CalendarId("nepal-sambat");

crate::ayanamsa_id::by_ayanamsa! {
    /// The identifier and English name of the calendar over an ayanāṃśa:
    /// [`ID`] for Lahiri's, and the convention's for any other.
    pub(crate) fn identity, "nepal-sambat", "Nepal Sambat (lunar)"
}

crate::ayanamsa_id::by_ayanamsa! {
    /// The same for the fortnight-first form, [`FORTNIGHT_ID`].
    pub(crate) fn fortnight_identity, "nepal-sambat-fortnight", "Nepal Sambat (fortnight and tithi)"
}
/// The era it counts in.
pub const ERA: &str = "nepal-sambat";

/// Śaka year minus Nepal Sambat year for Kachhalā to Chilā; Chaulā to
/// Kaulā are a Śaka year later.
const SAKA_OFFSET: i64 = 801;

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

/// The two fortnights, the waxing *thwa* and the waning *gā*, in the
/// Latin, Devanagari and Newa letters the Wikipedia article "Nepal Sambat"
/// writes them in (`wikipedia-nepal-sambat`, "Monthly cycle"): thwa, थ्वः,
/// 𑐠𑑂𑐰𑑅, and gā, गाः, 𑐐𑐵𑑅.
pub const FORTNIGHTS: [[&str; 3]; 2] = [["thwa", "थ्वः", "𑐠𑑂𑐰𑑅"], ["gā", "गाः", "𑐐𑐵𑑅"]];

/// The identifier of the fortnight-and-tithi form,
/// [`NepalSambatFortnightCalendar`].
pub const FORTNIGHT_ID: CalendarId = CalendarId("nepal-sambat-fortnight");

/// The fortnights in a common year: two to each of the twelve months.
pub const FORTNIGHTS_IN_YEAR: u8 = 24;

/// The number the Samiti writes for the new moon, *auṃsī*, the last tithi
/// of the dark fortnight: rule 8, "औंसीलाई ३०" (`nepal-panchang-committee-2081`).
pub const NEW_MOON_TITHI: u8 = 30;

/// The twenty-four fortnights as the Nepal Panchang Nirnayak Bikas Samiti
/// names them in rule 7, each a month's name with *thwa* or *gā* joined to
/// it, Kachhalā thwa first: "कार्तिक शुक्ल पक्षलाई कछलाथ्व, मार्ग कृष्ण पक्षलाई
/// कछलागा, …, आश्विन शुक्लपक्षलाई कौलाथ्व, कार्तिक कृष्णपक्षलाई कौलागा"
/// (`nepal-panchang-committee-2081`). The Samiti pairs each *gā* with the
/// dark fortnight of the *following* pūrṇimānta month — Kachhalā gā is
/// Mārga kṛṣṇa — which is the same fortnight the amānta Kārtika ends with,
/// so the fortnights are those of [`MONTHS_DEVANAGARI`]'s months, and
/// rule 2 of the notice says so: "प्रत्येक महिना परेवा तिथिबाट शुरू भई औंसी
/// तिथिमा पूर्ण हुन्छ". The spellings are the notice's, which differ from
/// Wikipedia's table in five months — थिंल्ला, पोहेला, सिल्ला, चिल्ला and
/// दिल्ला with a doubled *l*, वछला with *va*, ञँला with a candrabindu — and
/// the notice writes तछलाथ्व once with a visarga, तछलाथ्वः, which is not
/// kept.
pub const FORTNIGHTS_DEVANAGARI: [&str; 24] = [
    "कछलाथ्व",
    "कछलागा",
    "थिंल्लाथ्व",
    "थिंल्लागा",
    "पोहेलाथ्व",
    "पोहेलागा",
    "सिल्लाथ्व",
    "सिल्लागा",
    "चिल्लाथ्व",
    "चिल्लागा",
    "चौलाथ्व",
    "चौलागा",
    "वछलाथ्व",
    "वछलागा",
    "तछलाथ्व",
    "तछलागा",
    "दिल्लाथ्व",
    "दिल्लागा",
    "गुंलाथ्व",
    "गुंलागा",
    "ञँलाथ्व",
    "ञँलागा",
    "कौलाथ्व",
    "कौलागा",
];

/// The two fortnights of an intercalary month, whichever month it
/// doubles: rule 7 of the notice, the bright fortnight अनलाथ्व and the dark
/// अनलागा (`nepal-panchang-committee-2081`).
pub const INTERCALARY_FORTNIGHTS_DEVANAGARI: [&str; 2] = ["अनलाथ्व", "अनलागा"];

/// What a tithi is called, *milālyā*, in Devanagari and Newa letters, as the
/// same section writes it.
pub const TITHI_WORD: [&str; 3] = ["milālyā", "मिलाल्याः", "𑐩𑐶𑐮𑐵𑐮𑑂𑐫𑐵𑑅"];

/// The tithis for which the same section's table gives a Newar name beside
/// the Sanskrit one, by the tithi's number, 1 to 30, śukla 1 first: Pāru
/// for the first of either fortnight, Dutiya and Chauthi for the second and
/// fourth of *thwa*, Punhi for the full moon, Charhe for the fourteenth of
/// *gā* and Āmai for the new moon. The table names the other tithis by
/// their Sanskrit names only, and it gives the Newar ones in Latin letters
/// alone; it cites the *Journal of Newar Studies*, issue 7, p. 89, which
/// was not read.
pub const NEWAR_TITHI_NAMES: [(u8, &str); 7] = [
    (1, "Pāru"),
    (2, "Dutiya"),
    (4, "Chauthi"),
    (15, "Punhi"),
    (16, "Pāru"),
    (29, "Charhe"),
    (30, "Āmai"),
];

/// The Newar name of the tithi `tithi`, 1 to 30, where
/// [`NEWAR_TITHI_NAMES`] has one.
#[must_use]
pub fn newar_tithi_name(tithi: u8) -> Option<&'static str> {
    NEWAR_TITHI_NAMES
        .iter()
        .find(|(number, _)| *number == tithi)
        .map(|(_, name)| *name)
}

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
        let month = kartikadi::amanta_month(self.month);
        HinduLunarDate {
            year: kartikadi::saka_year(self.year, month, SAKA_OFFSET),
            month,
            leap_month: self.leap_month,
            day: self.day,
            leap_day: self.leap_day,
        }
    }

    /// The Nepal Sambat date of an amānta date.
    const fn from_amanta(date: HinduLunarDate) -> Self {
        Self {
            year: kartikadi::era_year(date.year, date.month, SAKA_OFFSET),
            month: kartikadi::kartikadi_month(date.month),
            leap_month: date.leap_month,
            day: date.day,
            leap_day: date.leap_day,
        }
    }
}

/// The lunar Nepal Sambat, judged at a place with an ayanāṃśa.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NepalSambatCalendar {
    lunar: HinduLunarCalendar,
}

impl NepalSambatCalendar {
    /// Kathmandu's sunrise, the Lahiri ayanamsa: the registered
    /// `nepal-sambat`.
    pub const KATHMANDU: Self = Self::new(HinduLunarCalendar::new(KATHMANDU, Ayanamsa::LAHIRI));

    /// Nepal Sambat over any amānta calendar — another place's sunrise,
    /// another ayanāṃśa.
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
        kartikadi::new_year(&self.lunar, year, SAKA_OFFSET)
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
        kartikadi::is_leap_year(&self.lunar, year, SAKA_OFFSET)
    }

    /// The day begins at sunrise and is named by the civil day on
    /// whose sunrise it begins: Reingold and Dershowitz read a fixed day's
    /// date at "Sunrise that day" (`reingold2018code`, `hindu-lunar-from-fixed`).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise(hc_calendar::DayNaming::ByStart)
    }

    fn meta(&self) -> CalendarMeta {
        let (id, english_name) = identity(self.lunar.ayanamsa);
        CalendarMeta {
            id,
            english_name,
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

/// Nepal Sambat written as the Nepal Panchang Nirnayak Bikas Samiti
/// writes it: the fortnight, a month's name with *thwa* or *gā* joined to
/// it, and the tithi counted within the fortnight — 1 to 15 in *thwa*, 1 to
/// 14 in *gā* and 30 for the new moon — `nepal-sambat-fortnight`.
///
/// The same days, tithis and intercalary months as [`NepalSambatCalendar`],
/// every conversion going through it; only the written form differs, as
/// the Samiti's notice of 16 April 2024 gives it
/// (`nepal-panchang-committee-2081`), rule 8: "परेवा लाई १, द्वितीयालाई २, …,
/// चतुर्दशीलाई १४ पूर्णिमालाई १५ र औंसीलाई ३०", the fortnights
/// [`FORTNIGHTS_DEVANAGARI`] and [`INTERCALARY_FORTNIGHTS_DEVANAGARI`].
/// The date is [`NepalSambatDate`], the day its tithi 1–30; the fields
/// are the fortnight, 1 for Kachhalā thwa through 24 for Kaulā gā, marked
/// intercalary for Analā, and the tithi as written. Rule 5 of the notice
/// says a doubled tithi is written on both days and the weekday must be
/// written to tell them apart, "बार पनि अनिवार्य लेख्नुपर्छ", but gives no
/// form for it, so the fields carry the repetition as `leap_day` and no
/// weekday; `docs/systems/nepal-calendars.md` says so.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NepalSambatFortnightCalendar {
    /// The calendar whose days these are.
    pub sambat: NepalSambatCalendar,
}

impl NepalSambatFortnightCalendar {
    /// Kathmandu's sunrise, the Lahiri ayanāṃśa: the registered
    /// `nepal-sambat-fortnight`.
    pub const KATHMANDU: Self = Self::new(NepalSambatCalendar::KATHMANDU);

    /// The fortnight form over any Nepal Sambat.
    #[must_use]
    pub const fn new(sambat: NepalSambatCalendar) -> Self {
        Self { sambat }
    }

    /// The fortnight, 1 for Kachhalā thwa through 24 for Kaulā gā, and the
    /// tithi as the Samiti writes it: the tithi's own number in the bright
    /// fortnight, fifteen less in the dark, and [`NEW_MOON_TITHI`] for the
    /// new moon.
    #[must_use]
    pub const fn written(date: NepalSambatDate) -> (u8, u8) {
        let (fortnight, tithi) = if date.day <= FULL_MOON {
            (2 * date.month - 1, date.day)
        } else if date.day == NEW_MOON_TITHI {
            (2 * date.month, NEW_MOON_TITHI)
        } else {
            (2 * date.month, date.day - FULL_MOON)
        };
        (fortnight, tithi)
    }

    /// The date of a written fortnight and tithi.
    ///
    /// # Errors
    ///
    /// [`CalendarError::MonthOutOfRange`] for a fortnight outside 1–24;
    /// [`CalendarError::DayOutOfRange`] for a tithi outside 1–15 in a
    /// bright fortnight, or outside 1–14 and 30 in a dark one.
    pub const fn read(
        year: i64,
        fortnight: u8,
        leap_month: bool,
        tithi: u8,
        leap_day: bool,
    ) -> CalendarResult<NepalSambatDate> {
        if fortnight == 0 || fortnight > FORTNIGHTS_IN_YEAR {
            return Err(CalendarError::MonthOutOfRange);
        }
        let month = fortnight.div_ceil(2);
        let day = if fortnight % 2 == 1 {
            if tithi == 0 || tithi > FULL_MOON {
                return Err(CalendarError::DayOutOfRange);
            }
            tithi
        } else if tithi == NEW_MOON_TITHI {
            NEW_MOON_TITHI
        } else if tithi == 0 || tithi >= FULL_MOON {
            return Err(CalendarError::DayOutOfRange);
        } else {
            tithi + FULL_MOON
        };
        Ok(NepalSambatDate {
            year,
            month,
            leap_month,
            day,
            leap_day,
        })
    }
}

/// The last tithi of the bright fortnight.
const FULL_MOON: u8 = 15;

impl Calendar for NepalSambatFortnightCalendar {
    type Date = NepalSambatDate;

    /// As [`NepalSambatCalendar`]'s; the Samiti's notice of 2024 is the
    /// earliest statement of this form read, and it does not date the form.
    fn usage(&self) -> hc_calendar::Usage {
        self.sambat.usage()
    }

    /// Twenty-four fortnights with two more in an intercalary year, and the
    /// seven-day week.
    fn cycles(&self) -> &'static [CycleShape] {
        const SHAPE: &[CycleShape] = &[
            CycleShape::intercalary(
                MONTH,
                FORTNIGHTS_IN_YEAR as u16,
                FORTNIGHTS_IN_YEAR as u16 + 2,
            ),
            CycleShape::fixed(WEEKDAY, 7),
        ];
        SHAPE
    }

    /// A year with an Analā: the same years as [`NepalSambatCalendar`]'s.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        self.sambat.is_leap_year(year)
    }

    /// As [`NepalSambatCalendar`]'s: sunrise, named by the civil day on
    /// whose sunrise the day begins.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        self.sambat.day_boundary()
    }

    fn meta(&self) -> CalendarMeta {
        let (id, english_name) = fortnight_identity(self.sambat.lunar.ayanamsa);
        CalendarMeta {
            id,
            english_name,
            ..self.sambat.meta()
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.sambat.to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.sambat.from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let (fortnight, tithi) = Self::written(date);
        let month = if date.leap_month {
            Month::leap(fortnight)
        } else {
            Month::regular(fortnight)
        };
        let mut fields = DateFields::ymd(date.year, fortnight, tithi).with_era(ERA);
        fields.month = Some(month);
        fields.leap_day = date.leap_day;
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        let date = Self::read(
            fields.year,
            month.ordinal,
            month.leap,
            fields.require_day()?,
            fields.leap_day,
        )?;
        self.sambat.to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    const NS: NepalSambatCalendar = NepalSambatCalendar::KATHMANDU;
    const FORTNIGHT: NepalSambatFortnightCalendar = NepalSambatFortnightCalendar::KATHMANDU;

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
    fn a_stride_through_the_whole_range_converts_and_converts_back() {
        // Policy §7. Every eleventh day from Chaitra 1700 to March 2300 and
        // each Nepal Sambat year's first day with the day before it; a
        // debug build takes every fifty-fifth.
        let (first, last) = (
            NS.earliest().expect("in range").0,
            NS.latest().expect("in range").0,
        );
        let years = NS.from_fixed(Rd(first)).expect("in range").year + 1
            ..=NS.from_fixed(Rd(last)).expect("in range").year;
        let openings: alloc::vec::Vec<i64> = years
            .filter_map(|year| NS.new_year(year).ok().map(|day| day.0))
            .collect();
        let days = crate::strided_days(first, last, 11, 5, &openings);
        crate::check_days(&days, |day| {
            let date = NS.from_fixed(Rd(day)).expect("in range");
            assert_eq!(NS.to_fixed(date), Ok(Rd(day)), "{date:?}");
        });
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

    #[test]
    fn the_full_moons_are_punhi_and_the_new_moon_amai() {
        // The article's table of months names each full moon "… Punhi",
        // Saki Milā Punhi to Analā Punhi, and its table of tithis gives
        // Punhi for the fifteenth of thwa and Āmai for the new moon.
        assert_eq!(newar_tithi_name(15), Some("Punhi"));
        assert_eq!(newar_tithi_name(30), Some("Āmai"));
        assert_eq!(newar_tithi_name(1), newar_tithi_name(16));
        assert_eq!(newar_tithi_name(3), None);
        // Mha Puja, the first day of the year, is Kachhalā thwa Pāru: 4
        // November 2013, as Wikipedia's "Mha Puja" dates it.
        let mha_puja = gregorian::to_fixed(2013, 11, 4).unwrap();
        let date = NepalSambatCalendar::KATHMANDU.from_fixed(mha_puja).unwrap();
        assert_eq!((date.month, date.day), (1, 1));
        assert_eq!(newar_tithi_name(date.day), Some("Pāru"));
        assert_eq!(FORTNIGHTS[0][0], "thwa");
    }

    #[test]
    fn the_samitis_fortnights_count_the_tithis_as_its_notice_does() {
        // Rule 8 of the notice (`nepal-panchang-committee-2081`): परेवा 1 to
        // पूर्णिमा 15, and औंसी 30; rule 4: the numbers repeat every fifteen
        // days. So the bright fortnight is 1–15, the dark 1–14 and 30.
        let date = |month, day| NepalSambatDate {
            year: 1134,
            month,
            leap_month: false,
            day,
            leap_day: false,
        };
        for (month, day, fortnight, tithi) in [
            (1, 1, 1, 1),
            (1, 15, 1, 15),
            (1, 16, 2, 1),
            (1, 29, 2, 14),
            (1, 30, 2, 30),
            (12, 1, 23, 1),
            (12, 30, 24, 30),
        ] {
            let d = date(month, day);
            assert_eq!(NepalSambatFortnightCalendar::written(d), (fortnight, tithi));
            assert_eq!(
                NepalSambatFortnightCalendar::read(1134, fortnight, false, tithi, false),
                Ok(d)
            );
        }
        // No dark fortnight has a 15th, and no bright one a 30th.
        assert_eq!(
            NepalSambatFortnightCalendar::read(1134, 2, false, 15, false),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            NepalSambatFortnightCalendar::read(1134, 1, false, 30, false),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            NepalSambatFortnightCalendar::read(1134, 25, false, 1, false),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(FORTNIGHTS_DEVANAGARI.len(), usize::from(FORTNIGHTS_IN_YEAR));
        assert_eq!(FORTNIGHTS_DEVANAGARI[0], "कछलाथ्व");
        assert_eq!(FORTNIGHTS_DEVANAGARI[1], "कछलागा");
        assert_eq!(FORTNIGHTS_DEVANAGARI[22], "कौलाथ्व");
        assert_eq!(FORTNIGHTS_DEVANAGARI[23], "कौलागा");
    }

    #[test]
    fn mha_puja_of_2013_is_kachhala_thwa_1_in_the_fortnight_form() {
        // Mha Puja, 4 November 2013, Wikipedia's "Mha Puja": Nepal Sambat
        // 1134, the first tithi of Kachhalā thwa, कछलाथ्व १.
        let rd = ymd(2013, 11, 4);
        let date = FORTNIGHT.from_fixed(rd).expect("in range");
        let fields = Calendar::to_fields(&FORTNIGHT, date).expect("fields");
        assert_eq!(
            (fields.year, fields.month, fields.day),
            (1134, Some(Month::regular(1)), Some(1))
        );
        assert_eq!(FORTNIGHT.meta().id, FORTNIGHT_ID);
        assert_eq!(Calendar::from_fields(&FORTNIGHT, &fields), Ok(date));
        // The day before is the new moon of Kaulā of 1133, कौलागा ३०.
        let eve = FORTNIGHT.from_fixed(Rd(rd.0 - 1)).expect("in range");
        let eve_fields = Calendar::to_fields(&FORTNIGHT, eve).expect("fields");
        assert_eq!(
            (eve_fields.year, eve_fields.month, eve_fields.day),
            (1133, Some(Month::regular(24)), Some(30))
        );
    }

    #[test]
    fn the_fortnight_form_is_the_same_days_as_nepal_sambat() {
        // Every day in a release build, every eleventh in a debug one: the
        // days, the intercalary flag and the repeated tithi are
        // `nepal-sambat`'s, and the fields convert back. Nepal Sambat 1138
        // to 1140 hold two intercalary months.
        for day in (NS.new_year(1138).expect("in range").0..NS.new_year(1141).expect("in range").0)
            .step_by(crate::sweep_stride(11))
        {
            let rd = Rd(day);
            let date = NS.from_fixed(rd).expect("in range");
            assert_eq!(FORTNIGHT.from_fixed(rd), Ok(date));
            let fields = Calendar::to_fields(&FORTNIGHT, date).expect("fields");
            assert_eq!(fields.leap_day, date.leap_day);
            assert_eq!(fields.month.map(|month| month.leap), Some(date.leap_month));
            assert_eq!(
                Calendar::from_fields(&FORTNIGHT, &fields),
                Ok(date),
                "{rd:?}"
            );
        }
    }

    #[test]
    fn an_intercalary_month_has_two_fortnights_of_its_own() {
        // Tachhalā of 1138 is intercalary (above): its fortnights are the
        // 15th and 16th, Analā thwa and Analā gā, with the leap flag set.
        let first = NS
            .to_fixed(NepalSambatDate {
                year: 1138,
                month: 8,
                leap_month: true,
                day: 1,
                leap_day: false,
            })
            .expect("an intercalary Tachhalā");
        let date = FORTNIGHT.from_fixed(first).expect("in range");
        let fields = Calendar::to_fields(&FORTNIGHT, date).expect("fields");
        assert_eq!(fields.month, Some(Month::leap(15)));
        assert_eq!(fields.day, Some(1));
        assert_eq!(INTERCALARY_FORTNIGHTS_DEVANAGARI, ["अनलाथ्व", "अनलागा"]);
        assert_eq!(Calendar::is_leap_year(&FORTNIGHT, 1138), Ok(true));
        assert_eq!(
            Calendar::from_fields(&FORTNIGHT, &{
                let mut wrong = fields;
                wrong.era = Some("Saka");
                wrong
            }),
            Err(CalendarError::UnknownEra)
        );
    }
}
