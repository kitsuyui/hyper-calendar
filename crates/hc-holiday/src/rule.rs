//! The rule vocabulary, and the observance modifiers that wrap it.
//!
//! Everything in this module is *data*. A rule is a value that answers one
//! question — "on which fixed days does this thing fall in Gregorian year
//! *y*?" — and the modifiers around it are values too. No country and no
//! tradition contributes code; they contribute tables of these values.
//!
//! The vocabulary is the one `docs/observances.md` specifies, plus four
//! shapes that fell out of writing the national tables and that are still
//! pure data: [`Rule::WeekdayOnOrAfter`], [`Rule::WeekdayOnOrBefore`],
//! [`Rule::Offset`] and [`Rule::Tabulated`].

#[cfg(feature = "alloc")]
use hc_astro::riseset::Location;
use hc_calendar::Calendar as _;
use hc_calendar::fixed::Moment;
use hc_calendar::{CalendarId, Month, Rd, Weekday};
use hc_calendars_equinox::persian as solar_hijri;
use hc_calendars_indic::nakshatra::nakshatra_span;
use hc_calendars_indic::tithi::{DEGREES_PER_TITHI, TITHIS_PER_MONTH};
#[cfg(feature = "alloc")]
use hc_calendars_indic::tithi::{sunrise_of, sunset_of, tithi_number_at};
use hc_calendars_indic::{
    BikramSambatCalendar, HinduLunarCalendar, HinduSolarDate, Prevalence, hindu_lunar,
};
use hc_calendars_lunar::hebrew;
use hc_calendars_lunar::islamic_umalqura;
use hc_calendars_lunar::tabular::{self, LeapYearRule};
use hc_calendars_lunar::tibetan::{self, LeapNumbering, TibetanCalendar, TibetanDate};
use hc_calendars_lunar::{ChineseCalendar, DangiCalendar, LunisolarDate, VietnameseCalendar};
use hc_calendars_regional::{burmese, thai_lunar};
use hc_calendars_solar::{
    bahai_kept, bangladeshi, coptic, ethiopic, gregorian, julian, nanakshahi, persian,
    revised_julian, zoroastrian,
};
use hc_core::math::{floor, normalize_degrees};
use hc_seasons::solar_terms::term_moment;
use hc_seasons::zodiac::sidereal::ingress_moment;
use hc_seasons::zodiac::{Ayanamsa, SiderealSign};
use hc_seasons::{Meridian, SolarTerm};

use crate::computus::{Computus, easter};

/// The days a single rule yields within one Gregorian year.
///
/// A calendar whose year is shorter than the Gregorian one can put the same
/// anniversary in a Gregorian year twice — 1 Muḥarram fell on both
/// 1 January and 21 December of 2008 — and a [`Rule::Span`] yields every
/// day of a festival that runs a week, so the answer is a small list rather
/// than an `Option`. Sixteen slots hold two week-long spans, one each side
/// of a New Year, and keep the type `Copy` and allocation-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Days {
    days: [Rd; Days::CAPACITY],
    len: u8,
}

impl Days {
    /// How many days a single rule may yield in one Gregorian year.
    pub const CAPACITY: usize = 16;

    /// An empty list.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            days: [Rd(0); Self::CAPACITY],
            len: 0,
        }
    }

    /// A list holding exactly one day.
    #[must_use]
    pub const fn one(day: Rd) -> Self {
        let mut list = Self::new();
        list.days[0] = day;
        list.len = 1;
        list
    }

    /// Append a day, silently ignoring anything past [`Days::CAPACITY`].
    ///
    /// Overflow is dropped rather than reported because no rule in the
    /// vocabulary can produce a seventeenth day: an anniversary lands in a
    /// Gregorian year at most twice, and a span is refused beyond
    /// [`Rule::MAX_SPAN`] days.
    pub const fn push(&mut self, day: Rd) {
        if (self.len as usize) < Self::CAPACITY {
            self.days[self.len as usize] = day;
            self.len += 1;
        }
    }

    /// The days, in the order they were produced.
    #[must_use]
    pub fn as_slice(&self) -> &[Rd] {
        &self.days[..self.len as usize]
    }

    /// How many days the list holds.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// Whether the rule produced nothing this year.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Shift every day by `offset`.
    #[must_use]
    pub fn shifted(self, offset: i32) -> Self {
        let mut out = Self::new();
        for day in self.as_slice() {
            out.push(Rd(day.0 + i64::from(offset)));
        }
        out
    }

    /// Keep only the days that fall inside `[first, last]`.
    #[must_use]
    pub fn clamped(self, first: Rd, last: Rd) -> Self {
        let mut out = Self::new();
        for day in self.as_slice() {
            if *day >= first && *day <= last {
                out.push(*day);
            }
        }
        out
    }
}

impl Default for Days {
    fn default() -> Self {
        Self::new()
    }
}

/// A calendar a [`Rule::FixedInCalendar`] can name a date in.
///
/// # Why this is a struct and not an enum
///
/// A holiday table is a `static` value, and a trait object cannot be one
/// without an allocator. That rules out `dyn Calendar`, but it does not call
/// for a *closed set*.
///
/// A closed set is a hole generator. A holiday dated in a calendar the set
/// does not list — the Ethiopic, the Coptic, the Solar Hijri, the Badíʿ —
/// could not be written down at all: Ethiopian Christmas would have to be
/// approximated in some other calendar or left out, and adding the calendar
/// would mean editing an enum and its match arms in a crate the person who
/// wants it does not own.
///
/// Two function pointers and an identifier are `Copy`, `const`-constructible
/// and `static`-safe, and open: a caller with a calendar this crate has never
/// heard of can build one and date a holiday in it.
///
/// The identifier is a real [`CalendarId`], and
/// `crates/hyper-calendar/tests/holiday_calendars.rs` asserts that every
/// system here names a calendar the registry answers to — the same guard the
/// vocabulary layer has, for the same reason.
#[derive(Debug, Clone, Copy)]
pub struct CalendarSystem {
    /// The calendar this dates in, as the registry names it.
    pub id: CalendarId,
    /// The fixed day of a date, or `None` when it does not exist or falls
    /// outside the calendar's range.
    to_fixed: fn(i64, Month, u8) -> Option<Rd>,
    /// The year containing a fixed day, or `None` outside the range.
    year_containing: fn(Rd) -> Option<i64>,
}

impl PartialEq for CalendarSystem {
    /// Two systems are the same when they name the same calendar.
    ///
    /// Function pointers do not compare usefully, and the identifier is the
    /// thing that actually distinguishes one system from another.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CalendarSystem {}

impl core::hash::Hash for CalendarSystem {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

hc_core::catalogue! {
    type: CalendarSystem,
    id: |system| system.id.0,
    tests: calendar_system_catalogue_tests,
    associated;

    /// Every system this crate defines.
    ///
    /// Not exhaustive of what is *possible* — that is the point of the type
    /// — but exhaustive of what ships, so a test can check them all.
    pub const ALL;
    /// The system whose identifier this is, as the registry names it.
    pub fn by_id;

    entries: {
        /// The proleptic Gregorian calendar.
        pub const GREGORIAN = Self::new(
            CalendarId("gregory"),
            |year, month, day| gregorian::to_fixed(year, month.ordinal, day).ok(),
            |rd| gregorian::year_from_fixed(rd).ok(),
        );

        /// The Julian calendar, still used for the fixed feasts of most
        /// Orthodox churches.
        pub const JULIAN = Self::new(
            CalendarId("julian"),
            |year, month, day| julian::to_fixed(year, month.ordinal, day).ok(),
            |rd| julian::from_fixed(rd).ok().map(|(year, _, _)| year),
        );

        /// The Revised Julian calendar, in which the Orthodox churches that
        /// adopted it from 1924 date their fixed feasts: the Gregorian dates
        /// from 1 March 1600 to 28 February 2800, and a day apart at times
        /// after.
        pub const REVISED_JULIAN = Self::new(
            CalendarId("revised-julian"),
            |year, month, day| revised_julian::to_fixed(year, month.ordinal, day).ok(),
            |rd| revised_julian::from_fixed(rd).ok().map(|(year, _, _)| year),
        );

        /// The tabular civil Hijri calendar, the arithmetic approximation.
        pub const ISLAMIC_CIVIL = Self::new(
            CalendarId("islamic-civil"),
            |year, month, day| {
                tabular::to_fixed(
                    tabular::CIVIL_EPOCH,
                    LeapYearRule::CIVIL,
                    year,
                    month.ordinal,
                    day,
                )
                .ok()
            },
            |rd| {
                tabular::from_fixed(tabular::CIVIL_EPOCH, LeapYearRule::CIVIL, rd)
                    .ok()
                    .map(|(year, _, _)| year)
            },
        );

        /// The Umm al-Qurā calendar of Saudi Arabia, a published table that
        /// refuses years outside 1300–1600 AH rather than extrapolating.
        pub const ISLAMIC_UMM_AL_QURA = Self::new(
            CalendarId("islamic-umalqura"),
            |year, month, day| islamic_umalqura::to_fixed(year, month.ordinal, day).ok(),
            |rd| islamic_umalqura::from_fixed(rd).ok().map(|(y, _, _)| y),
        );

        /// The Hebrew calendar. Months are Tishrei-first, so Nisan is
        /// `Month::regular(7)` and Adar I in a leap year is `Month::leap(5)`.
        pub const HEBREW = Self::new(
            CalendarId("hebrew"),
            |year, month, day| hebrew::to_fixed(year, month, day).ok(),
            |rd| hebrew::year_from_fixed(rd).ok(),
        );

        /// The Chinese lunisolar calendar, computed at the Beijing meridian.
        pub const CHINESE = Self::new(
            CalendarId("chinese"),
            |year, month, day| {
                ChineseCalendar
                    .to_fixed(LunisolarDate::new(year, month, day))
                    .ok()
            },
            |rd| ChineseCalendar.from_fixed(rd).ok().map(|date| date.year),
        );

        /// The Korean lunisolar calendar, computed at the Seoul meridian, which
        /// puts Seollal a day away from 春節 a few times a century.
        pub const DANGI = Self::new(
            CalendarId("dangi"),
            |year, month, day| {
                DangiCalendar
                    .to_fixed(LunisolarDate::new(year, month, day))
                    .ok()
            },
            |rd| DangiCalendar.from_fixed(rd).ok().map(|date| date.year),
        );

        /// The Vietnamese lunisolar calendar, computed at UTC+7, which puts Tết
        /// a day away from 春節 rather more often.
        pub const VIETNAMESE = Self::new(
            CalendarId("vietnamese"),
            |year, month, day| {
                VietnameseCalendar
                    .to_fixed(LunisolarDate::new(year, month, day))
                    .ok()
            },
            |rd| VietnameseCalendar.from_fixed(rd).ok().map(|date| date.year),
        );

        /// The Mongolian calendar, the New Genden version of the Tibetan
        /// (`hc_calendars_lunar::tibetan::MONGOLIAN`), in which Mongolia's
        /// law dates Tsagaan Sar, Buddha's Birthday and Chinggis Khaan Day.
        /// Month 1 is the first spring month; a leap month is
        /// `Month::leap` and comes before the regular month of its number.
        /// A day number the calendar skips has no day, and one it repeats
        /// gives the second of its two days, the one not marked extra — so
        /// a holiday table wants [`Rule::TibetanDay`], which reports such a
        /// year as a gap instead.
        pub const MONGOLIAN = Self::new(
            CalendarId("mongolian"),
            |year, month, day| tibetan_date(tibetan::MONGOLIAN, year, month, day),
            |rd| tibetan::MONGOLIAN.date_from_fixed(rd).ok().map(|date| date.year),
        );

        /// The Bhutanese version of the Tibetan calendar
        /// (`hc_calendars_lunar::tibetan::TIBETAN_BHUTAN`), which the
        /// Ministry of Home Affairs prints beside the Gregorian days and in
        /// which Bhutan dates Losar and its Buddhist holidays. A leap month
        /// is `Month::leap` and comes *after* the regular month of its
        /// number. Skipped and repeated day numbers are as for
        /// [`Self::MONGOLIAN`].
        pub const TIBETAN_BHUTAN = Self::new(
            CalendarId("tibetan-bhutan"),
            |year, month, day| tibetan_date(tibetan::TIBETAN_BHUTAN, year, month, day),
            |rd| {
                tibetan::TIBETAN_BHUTAN
                    .date_from_fixed(rd)
                    .ok()
                    .map(|date| date.year)
            },
        );

        /// The Ethiopic calendar, in which the Ethiopian Orthodox Tewahedo
        /// Church dates its fixed feasts and Ethiopia its civil year.
        ///
        /// One of the calendars the closed enum could not express. Genna, Timkat
        /// and Enkutatash are fixed dates in it and were not writable before.
        pub const ETHIOPIC = Self::new(
            CalendarId("ethiopic"),
            |year, month, day| ethiopic::to_fixed(year, month.ordinal, day).ok(),
            |rd| ethiopic::from_fixed(rd).ok().map(|(year, _, _)| year),
        );

        /// The Coptic calendar, which the Coptic Orthodox Church of Alexandria
        /// dates its fixed feasts in.
        pub const COPTIC = Self::new(
            CalendarId("coptic"),
            |year, month, day| coptic::to_fixed(year, month.ordinal, day).ok(),
            |rd| coptic::from_fixed(rd).ok().map(|(year, _, _)| year),
        );

        /// The Solar Hijri calendar as Iran keeps it — Nowruz on the day of
        /// the March equinox when the equinox falls before noon, Iran
        /// Standard Time, and the day after otherwise — in which Iran dates
        /// Nowruz and its civil holidays.
        pub const SOLAR_HIJRI = Self::new(
            CalendarId("persian"),
            |year, month, day| solar_hijri::to_fixed(year, month.ordinal, day).ok(),
            |rd| solar_hijri::from_fixed(rd).ok().map(|(year, _, _)| year),
        );

        /// The Solar Hijri calendar as Afghanistan keeps it, in which
        /// Afghanistan dates its solar holidays — 28 Asad, 26 Dalw: the
        /// same days as [`Self::SOLAR_HIJRI`] under the Arabic names of the
        /// signs, *Hamal* … *Hut*, so month 5 is Asad and month 11 Dalw
        /// (`hc_calendars_equinox::persian_afghan`).
        pub const SOLAR_HIJRI_AFGHAN = Self::new(
            hc_calendars_equinox::persian_afghan::ID,
            |year, month, day| solar_hijri::to_fixed(year, month.ordinal, day).ok(),
            |rd| solar_hijri::from_fixed(rd).ok().map(|(year, _, _)| year),
        );

        /// The arithmetic Solar Hijri calendar, Birashk's 2 820-year cycle,
        /// for a table that wants the rule rather than the sky. It puts
        /// Nowruz 1404 a day before Iran did.
        pub const SOLAR_HIJRI_ARITHMETIC = Self::new(
            CalendarId("persian-arithmetic"),
            |year, month, day| persian::to_fixed(year, month.ordinal, day).ok(),
            |rd| persian::from_fixed(rd).ok().map(|(year, _, _)| year),
        );

        /// The Badíʿ calendar as kept, in which the Bahá'í holy days are
        /// dated: the arithmetic Western rule until 171 BE, the Bahá'í World
        /// Centre's published table for 172–221 BE, and nothing after — so a
        /// Badíʿ-dated holiday is exact through 19 March 2065 and a reported
        /// gap beyond.
        pub const BADI = Self::new(
            CalendarId("bahai"),
            |year, month, day| bahai_kept::to_fixed(year, month.ordinal, day).ok(),
            |rd| bahai_kept::from_fixed(rd).ok().map(|(year, _, _)| year),
        );

        /// The Nanakshahi calendar of 2003, in which the Sikh gurpurabs are
        /// dated: a naming of the Gregorian day, so every date is a fixed
        /// Gregorian one.
        pub const NANAKSHAHI = Self::new(
            CalendarId("nanakshahi"),
            |year, month, day| nanakshahi::to_fixed(year, month.ordinal, day).ok(),
            |rd| nanakshahi::from_fixed(rd).ok().map(|(year, _, _)| year),
        );

        /// The Bangladeshi national calendar as revised in 2019, in which
        /// Bangladesh dates Pohela Boishakh, 1 Boishakh: a fixed naming of
        /// the Gregorian day, so every date is a fixed Gregorian one
        /// (`hc_calendars_solar::bangladeshi`).
        pub const BANGLADESHI = Self::new(
            CalendarId("bangladeshi"),
            |year, month, day| bangladeshi::to_fixed(year, month.ordinal, day).ok(),
            |rd| bangladeshi::from_fixed(rd).ok().map(|(year, _, _)| year),
        );

        /// The Bikram Sambat, in which Nepal dates its national days: the
        /// months the Government of Nepal gazettes, and the *Sūrya
        /// Siddhānta* reckoning outside the gazetted years
        /// (`hc_calendars_indic::bikram_sambat`).
        pub const BIKRAM_SAMBAT = Self::new(
            CalendarId("bikram-sambat"),
            |year, month, day| {
                if month.leap {
                    return None;
                }
                BikramSambatCalendar
                    .to_fixed(HinduSolarDate {
                        year,
                        month: month.ordinal,
                        day,
                    })
                    .ok()
            },
            |rd| BikramSambatCalendar.from_fixed(rd).ok().map(|date| date.year),
        );

        /// The Burmese calendar, in which Myanmar dates its full-moon
        /// holidays. Tagu and Kason are split by the solar New Year, so a
        /// day of them is the early half's when the year has it there and
        /// the late half's otherwise — either way the one day of that name
        /// the year holds.
        pub const BURMESE = Self::new(
            CalendarId("burmese"),
            |year, month, day| {
                let early = burmese::BurmeseDate {
                    year,
                    month,
                    late: false,
                    day,
                };
                burmese::to_fixed(early).ok().or_else(|| {
                    burmese::to_fixed(burmese::BurmeseDate {
                        late: true,
                        ..early
                    })
                    .ok()
                })
            },
            |rd| burmese::from_fixed(rd).ok().map(|date| date.year),
        );

        /// The Thai lunar calendar as Thailand publishes it, in which
        /// Thailand dates its Buddhist holidays: the adhikamāsa and
        /// adhikavāra years carried as data for 2535–2570 BE (1992–2027),
        /// with the first six months of 2571 that no year type changes,
        /// and nothing else, so that a Thai lunar holiday beyond the table
        /// is a reported gap. Years are Buddhist Era, changing
        /// at เดือนอ้าย in November or December; the first month 8 of an
        /// adhikamāsa year is `Month::leap(8)` and the second, Asalha
        /// Bucha's, `Month::regular(8)` (`hc_calendars_regional::thai_lunar`).
        pub const THAI_LUNAR = Self::new(
            CalendarId("thai-lunar"),
            |year, month, day| {
                thai_lunar::to_fixed(thai_lunar::ThaiLunarDate::new(year, month, day)).ok()
            },
            |rd| thai_lunar::from_fixed(rd).ok().map(|date| date.year),
        );

        /// The Zoroastrian calendar by the Qadimi reckoning: the 365-day
        /// year from the accession of Yazdegerd III, drifting a day every
        /// four years, in which the Kadmi Parsis and the Zoroastrians of
        /// Yazd date their feasts.
        pub const ZOROASTRIAN_QADIMI = Self::new(
            CalendarId("zoroastrian-qadimi"),
            |year, month, day| {
                zoroastrian::Reckoning::Qadimi
                    .to_fixed(year, month.ordinal, day)
                    .ok()
            },
            |rd| {
                zoroastrian::Reckoning::Qadimi
                    .from_fixed(rd)
                    .ok()
                    .map(|(year, _, _)| year)
            },
        );

        /// The Zoroastrian calendar by the Shahanshahi reckoning, thirty
        /// days behind the Qadimi since the 1120s and stated from 498 Y.Z.,
        /// in which the Parsi majority dates its feasts.
        pub const ZOROASTRIAN_SHAHANSHAHI = Self::new(
            CalendarId("zoroastrian-shahanshahi"),
            |year, month, day| {
                zoroastrian::Reckoning::Shahanshahi
                    .to_fixed(year, month.ordinal, day)
                    .ok()
            },
            |rd| {
                zoroastrian::Reckoning::Shahanshahi
                    .from_fixed(rd)
                    .ok()
                    .map(|(year, _, _)| year)
            },
        );

        /// The Zoroastrian calendar by the Fasli reckoning: 1 Fravardin on
        /// 21 March and a leap day with the Gregorian calendar, in which the
        /// Fasli Parsis date their feasts.
        pub const ZOROASTRIAN_FASLI = Self::new(
            CalendarId("zoroastrian-fasli"),
            |year, month, day| {
                zoroastrian::Reckoning::Fasli
                    .to_fixed(year, month.ordinal, day)
                    .ok()
            },
            |rd| {
                zoroastrian::Reckoning::Fasli
                    .from_fixed(rd)
                    .ok()
                    .map(|(year, _, _)| year)
            },
        );
    }
}

impl CalendarSystem {
    /// A system from its identifier and its two conversions.
    #[must_use]
    pub const fn new(
        id: CalendarId,
        to_fixed: fn(i64, Month, u8) -> Option<Rd>,
        year_containing: fn(Rd) -> Option<i64>,
    ) -> Self {
        Self {
            id,
            to_fixed,
            year_containing,
        }
    }

    /// The fixed day of a date in this calendar, or `None` when that date
    /// does not exist or falls outside the calendar's supported range.
    #[must_use]
    pub fn to_fixed(self, year: i64, month: Month, day: u8) -> Option<Rd> {
        (self.to_fixed)(year, month, day)
    }

    /// Whether this calendar can answer for every day of a Gregorian year.
    ///
    /// The Chinese, Korean and Vietnamese calendars stop at 2150 and start
    /// at 1645; Umm al-Qurā is a published table and refuses years outside
    /// it. A rule dated in one of those calendars has no answer outside
    /// their range — which is different from having no occurrence, and used
    /// to be indistinguishable from it.
    #[must_use]
    pub fn covers_gregorian_year(self, year: i64) -> bool {
        let Ok(first) = gregorian::to_fixed(year, 1, 1) else {
            return false;
        };
        let Ok(last) = gregorian::to_fixed(year, 12, 31) else {
            return false;
        };
        self.year_containing(first).is_some() && self.year_containing(last).is_some()
    }

    /// The year of this calendar that contains `rd`, or `None` outside the
    /// calendar's supported range.
    #[must_use]
    pub fn year_containing(self, rd: Rd) -> Option<i64> {
        (self.year_containing)(rd)
    }
}

/// The day a Tibetan date names, the second of two when the number is
/// repeated, for [`CalendarSystem::MONGOLIAN`] and
/// [`CalendarSystem::TIBETAN_BHUTAN`].
fn tibetan_date(calendar: TibetanCalendar, year: i64, month: Month, day: u8) -> Option<Rd> {
    calendar
        .date_to_fixed(TibetanDate {
            year,
            month,
            day,
            leap_day: false,
        })
        .ok()
}

/// Which month of a Tibetan year a [`Rule::TibetanDay`] names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TibetanMonth {
    /// The first month of the year: month 1, or the leap month 1 in a year
    /// that begins with one. The Phugpa, Tsurphu and Mongolian versions put
    /// a leap month before the regular month of its number, and the New
    /// Year is the first day of the year even when that is a leap month
    /// (Janson, "Tibetan calendar mathematics", Remark 17); the
    /// Bhutanese puts it after, so its first month is always the regular
    /// month 1. Losar and Tsagaan Sar are dated here.
    First,
    /// The regular month of this number, 1 to 12: in a year that repeats
    /// the number, the month that is not the leap one. Holidays "are
    /// usually not celebrated in leap months" (Janson, §11).
    Regular(u8),
    /// The leap month of this number, which a year without it does not
    /// have.
    Leap(u8),
}

/// A lunar phase a [`Rule::LunarPhase`] can key to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phase {
    /// Conjunction.
    New,
    /// Opposition — the full moon.
    Full,
}

impl Phase {
    /// The elongation from the Sun, in degrees, that defines this phase.
    #[must_use]
    pub const fn elongation_degrees(self) -> f64 {
        match self {
            Self::New => 0.0,
            Self::Full => 180.0,
        }
    }
}

/// The shape of a holiday.
///
/// Each variant answers the same question — which fixed days does this fall
/// on in a given Gregorian year — and the evaluator in [`crate::engine`]
/// knows nothing else about any holiday.
///
/// A variant holds a calendar, a computus or an ayanamsa by `&'static`
/// reference, never by value. Every [`HolidayRule`] of every table holds a
/// `Rule`, so the largest variant sets the size of all of them: a
/// [`TibetanCalendar`] held by value once made each `Rule` 224 bytes on a
/// 64-bit target, where 40 do, and the WebAssembly holiday layer 0.7 MB
/// larger.
///
/// Deliberately not `PartialEq`: [`Rule::Computed`] holds a function
/// pointer, and comparing function pointers is not meaningful — two
/// identical functions may be merged to one address and one function may
/// have several.
#[derive(Debug, Clone, Copy)]
pub enum Rule {
    /// A fixed Gregorian month and day. New Year's Day is `{ 1, 1 }`.
    FixedGregorian {
        /// The Gregorian month, 1–12.
        month: u8,
        /// The day of the month.
        day: u8,
    },
    /// The `n`-th `weekday` of `month`, counting from 1. US Thanksgiving is
    /// the 4th Thursday of November; Japan's 成人の日 is the 2nd Monday of
    /// January. A negative `n` counts back from the end of the month.
    NthWeekday {
        /// The Gregorian month, 1–12.
        month: u8,
        /// Which occurrence, from 1; negative counts back from the end.
        n: i8,
        /// The weekday.
        weekday: Weekday,
    },
    /// The last `weekday` of `month`. The UK Spring Bank Holiday is the last
    /// Monday of May.
    LastWeekday {
        /// The Gregorian month, 1–12.
        month: u8,
        /// The weekday.
        weekday: Weekday,
    },
    /// The first `weekday` on or after a fixed Gregorian date. Sweden's
    /// Midsommardagen is the Saturday on or after 20 June.
    WeekdayOnOrAfter {
        /// The Gregorian month, 1–12.
        month: u8,
        /// The day of the month the window opens on.
        day: u8,
        /// The weekday.
        weekday: Weekday,
    },
    /// The last `weekday` on or before a fixed Gregorian date.
    WeekdayOnOrBefore {
        /// The Gregorian month, 1–12.
        month: u8,
        /// The day of the month the window closes on.
        day: u8,
        /// The weekday.
        weekday: Weekday,
    },
    /// A fixed date in some other calendar. Eid al-Fiṭr is 1 Shawwāl;
    /// Rosh Hashanah is 1 Tishrei; 春節 is 1 正月.
    FixedInCalendar {
        /// The calendar the date is stated in.
        system: CalendarSystem,
        /// The month, with the intercalary flag that lunisolar calendars
        /// need.
        month: Month,
        /// The day of the month.
        day: u8,
    },
    /// The day one of the 24 solar terms begins, at a stated meridian.
    /// Japan's 春分の日 is the spring equinox at UTC+9.
    SolarTerm {
        /// Which term.
        term: SolarTerm,
        /// The meridian whose local midnight cuts the day.
        meridian: Meridian,
    },
    /// A fixed offset in days from Easter. Good Friday is `-2`, Easter
    /// Monday `+1`, Ash Wednesday `-46`, Ascension `+39`, Pentecost `+49`,
    /// Corpus Christi `+60`.
    EasterRelative {
        /// Which computus — Western or Orthodox.
        computus: &'static Computus,
        /// The offset in days.
        offset: i16,
    },
    /// The first occurrence of a lunar phase on or after a fixed Gregorian
    /// date, at a stated meridian.
    ///
    /// This is the crude way to date a lunar observance: a table that has
    /// the calendar the observance is kept by, as
    /// [`crate::traditions::BUDDHIST_THAI`] has `thai-lunar`, uses it.
    LunarPhase {
        /// Which phase.
        phase: Phase,
        /// The Gregorian month the search window opens in.
        month: u8,
        /// The day of the month the search window opens on.
        day: u8,
        /// The meridian whose local midnight cuts the day.
        meridian: Meridian,
    },
    /// Another rule, shifted. Korea's Seollal holiday runs the day before
    /// and the day after 1 정월 as well as the day itself.
    Offset {
        /// The rule being shifted.
        base: &'static Rule,
        /// The shift, in days.
        days: i16,
    },
    /// Every day from the day one rule gives to the next day another
    /// gives, both included: a festival whose first and last days are each
    /// a tithi, so that its length changes from year to year. Nepal's
    /// Dashain holiday runs from Phūlpātī, Āśvina śukla 7, to Āśvina śukla
    /// 12 — six days in one year, seven in another.
    ///
    /// A `from` with no `to` on or after it within [`Rule::MAX_SPAN`] days
    /// yields nothing: the two rules are meant to fall a few days apart,
    /// and a span longer than that is a mistake in the table.
    Span {
        /// The rule for the first day.
        from: &'static Rule,
        /// The rule for the last day.
        to: &'static Rule,
    },
    /// Another rule, moved by the weekday it falls on: the day the base
    /// rule gives, plus the days its weekday is listed with in `moves`, or
    /// the day itself when its weekday is not listed.
    ///
    /// This is the shape of a *Monday holiday* law that keeps the date and
    /// moves the day off: Argentina's *feriados trasladables*, which a
    /// Tuesday or Wednesday pulls to the Monday before and a Thursday or
    /// Friday pushes to the Monday after, and Colombia's Ley Emiliani, which
    /// sends any day but a Monday to the next one. It differs from a
    /// [`SubstitutionPolicy`] in that the move is the holiday's own rule,
    /// not a country's response to a weekend, and the original date is not
    /// a holiday at all.
    MovedByWeekday {
        /// The rule being moved.
        base: &'static Rule,
        /// For each weekday that moves, how many days to add — negative to
        /// move earlier. A weekday not listed stays.
        moves: &'static [(Weekday, i16)],
    },
    /// A tithi of a month of the amānta Hindu lunisolar calendar, kept on
    /// the day the tithi is in progress at a stated part of the day —
    /// sunrise, midday, afternoon, evening or midnight — which is how the
    /// Hindu festivals are dated. See [`crate::hindu`] for the festivals and
    /// their conventions.
    ///
    /// When the year repeats the month, the ordinary month is meant; when
    /// the tithi holds the stated part of two consecutive days, `when_twice`
    /// decides; when it holds neither day's, the day that carries the tithi
    /// at sunrise is taken, or, for a tithi that holds no sunrise, the day
    /// it begins and ends within.
    Tithi {
        /// The amānta month, 1 for Chaitra through 12 for Phālguna.
        month: u8,
        /// The tithi, 1 through 30.
        tithi: u8,
        /// The part of the day the tithi must hold.
        prevails: Prevalence,
        /// Which of two consecutive qualifying days is the festival's.
        when_twice: WhenTwice,
        /// The calendar — its sunrise and its ayanamsa — the tithi is read in.
        calendar: &'static HinduLunarCalendar,
    },
    /// The Sun's entry into a sidereal sign — a saṅkrānti — as a day at a
    /// meridian: Makara Saṅkrānti, the solar new year of Meṣa.
    Sankranti {
        /// The sign entered.
        sign: SiderealSign,
        /// The ayanamsa that fixes the sidereal zero point.
        ayanamsa: &'static Ayanamsa,
        /// The meridian whose local midnight cuts the day.
        meridian: Meridian,
    },
    /// A nakṣatra in a solar month: the civil day at the meridian that holds
    /// the greater part of the Moon's stay in the nakṣatra while the Sun is
    /// in the sign. The Moon returns to a nakṣatra every 27.3 days and the
    /// Sun stays in a sign for 29 to 32, so a month can hold two stays;
    /// then `with_tithi` names the tithi the wanted stay lies nearest — the
    /// full-moon tithi for Thaipusam, so that of two Puṣyas the one at the
    /// full moon is taken — judged by the Moon's elongation from the Sun
    /// at the middle of each stay. With no tithi named, the first stay.
    ///
    /// The almanacs state such a festival as a nakṣatra in a month and no
    /// more; this reading of the day reproduces the dates Malaysia and
    /// Mauritius gazetted for Thaipusam from 2020 to 2026. See
    /// [`crate::hindu::THAIPUSAM`].
    Nakshatra {
        /// The nakṣatra, 1 for Aśvinī through 27 for Revatī, as
        /// [`hc_calendars_indic::nakshatra`] numbers them.
        nakshatra: u8,
        /// The sidereal sign the Sun must be in: the solar month.
        sign: SiderealSign,
        /// The tithi the stay must lie nearest, which picks between two
        /// stays in the month.
        with_tithi: Option<u8>,
        /// The ayanamsa that fixes the sidereal zero point.
        ayanamsa: &'static Ayanamsa,
        /// The meridian whose local midnight cuts the day.
        meridian: Meridian,
    },
    /// A numbered day of a month of a version of the Tibetan calendar, on
    /// the calendar day that bears the number: Mongolia's Tsagaan Sar and
    /// Chinggis Khaan Day, Bhutan's Losar and Buddhist days.
    ///
    /// The calendar names a day by the lunar day current at its dawn, so
    /// now and then a number is skipped and another is repeated, and where
    /// a holiday dated on such a number is kept is not settled: Janson
    /// reports a general rule from Berzin — the day before for a skipped
    /// date, the first of the two for a repeated one — which he did not
    /// check against published calendars, and Mongolia's own practice has
    /// varied. A Gregorian year in which the day is skipped or repeated is
    /// therefore not answered, and reported as a gap. See
    /// `docs/systems/tibetan-calendar-holidays.md`.
    TibetanDay {
        /// The version of the calendar.
        calendar: &'static TibetanCalendar,
        /// The month.
        month: TibetanMonth,
        /// The day number, 1 to 30.
        day: u8,
    },
    /// The genuine handful that resist everything else — a one-off statute,
    /// a rule stated as a sentence and not as a pattern.
    ///
    /// The function takes a Gregorian year and returns the days it falls on
    /// in that year.
    Computed(fn(i64) -> Days),
    /// A computed rule backed by a published table, which therefore has a
    /// last year.
    ///
    /// [`Rule::Computed`] is a formula and answers for any year. A table
    /// runs out, and when it does the function returns nothing — which
    /// looks exactly like a year the holiday does not fall in. New
    /// Zealand's Matariki is the case: the table here reaches 2035 and the
    /// Act schedules dates through 2052, so from 2036 a public holiday
    /// simply stopped appearing.
    Tabulated {
        /// The table, as a function of the Gregorian year.
        function: fn(i64) -> Days,
        /// The first year the table covers.
        first_year: i64,
        /// The last year the table covers.
        last_year: i64,
    },
}

impl Rule {
    /// A fixed Gregorian date.
    #[must_use]
    pub const fn gregorian(month: u8, day: u8) -> Self {
        Self::FixedGregorian { month, day }
    }

    /// The `n`-th `weekday` of a Gregorian month.
    #[must_use]
    pub const fn nth(month: u8, n: i8, weekday: Weekday) -> Self {
        Self::NthWeekday { month, n, weekday }
    }

    /// The last `weekday` of a Gregorian month.
    #[must_use]
    pub const fn last(month: u8, weekday: Weekday) -> Self {
        Self::LastWeekday { month, weekday }
    }

    /// An offset from Western Easter.
    #[must_use]
    pub const fn easter(offset: i16) -> Self {
        Self::EasterRelative {
            computus: &Computus::GREGORIAN,
            offset,
        }
    }

    /// An offset from Orthodox Easter.
    #[must_use]
    pub const fn paschal(offset: i16) -> Self {
        Self::EasterRelative {
            computus: &Computus::JULIAN,
            offset,
        }
    }

    /// A fixed date in an ordinary month of another calendar.
    #[must_use]
    pub const fn in_calendar(system: CalendarSystem, month: u8, day: u8) -> Self {
        Self::FixedInCalendar {
            system,
            month: Month::regular(month),
            day,
        }
    }

    /// A numbered day of a month of the Tibetan calendar; see
    /// [`Rule::TibetanDay`].
    #[must_use]
    pub const fn tibetan(calendar: &'static TibetanCalendar, month: TibetanMonth, day: u8) -> Self {
        Self::TibetanDay {
            calendar,
            month,
            day,
        }
    }

    /// The longest [`Rule::Span`], in days: a week.
    pub const MAX_SPAN: i64 = 7;

    /// Every day from one rule's to another's; see [`Rule::Span`].
    #[must_use]
    pub const fn span(from: &'static Rule, to: &'static Rule) -> Self {
        Self::Span { from, to }
    }

    /// Another rule, moved by the weekday it falls on; see
    /// [`Rule::MovedByWeekday`].
    #[must_use]
    pub const fn moved_by_weekday(base: &'static Rule, moves: &'static [(Weekday, i16)]) -> Self {
        Self::MovedByWeekday { base, moves }
    }

    /// Whether this rule can be answered at all for Gregorian `year`.
    ///
    /// False when the rule is dated in a calendar whose supported range
    /// does not reach that year — a 春節 in 2151, a Hijri feast before the
    /// Umm al-Qurā table begins. [`Rule::days_in_year`] returns an empty
    /// list in that case, which is indistinguishable from "this holiday
    /// does not occur that year", and that silence is what this exists to
    /// break.
    ///
    /// A rule that *can* be answered may still yield nothing — 29 February
    /// in a common year is resolvable and empty.
    #[must_use]
    pub fn is_resolvable_in(&self, year: i64) -> bool {
        self.is_resolvable_with(year, &mut Uncached)
    }

    /// [`Rule::is_resolvable_in`] with the calendar look-ups it makes
    /// answered through `lookups`, so that an evaluation of many rules can
    /// memoise them.
    pub(crate) fn is_resolvable_with<L: Lookups>(&self, year: i64, lookups: &mut L) -> bool {
        match self {
            Self::FixedInCalendar { system, .. } => lookups.calendar_years(*system, year).is_some(),
            Self::Tabulated {
                first_year,
                last_year,
                ..
            } => (*first_year..=*last_year).contains(&year),
            // A shifted rule needs its base in the same year. It also
            // *probes* the neighbouring years, because a shift can cross a
            // New Year, but requiring those too would report a gap in every
            // usable edge year: 除夕 is 春節 minus a day, so in 2150 it would
            // be called unanswerable on account of 春節 2151, when 春節 never
            // falls near enough to 1 January to matter.
            //
            // The residue is exact and small: a shifted holiday can still
            // be missed when its base lies in an out-of-range year *and*
            // falls within `days` of the year boundary.
            // The Hindu calendar converts a stated span of years, and a
            // festival needs the month it falls in, so the edge years are
            // left out as well.
            Self::Tithi { .. } => (hindu_lunar::MIN_YEAR + hindu_lunar::GREGORIAN_YEAR_OFFSET + 1
                ..hindu_lunar::MAX_YEAR + hindu_lunar::GREGORIAN_YEAR_OFFSET)
                .contains(&year),
            // A Tibetan day is unanswerable where it is skipped or repeated,
            // as well as outside the calendar's years.
            Self::TibetanDay {
                calendar,
                month,
                day,
            } => tibetan_days(calendar, *month, *day, year).is_some(),
            Self::Offset { base, .. } | Self::MovedByWeekday { base, .. } => {
                base.is_resolvable_with(year, lookups)
            }
            Self::Span { from, to } => {
                from.is_resolvable_with(year, lookups) && to.is_resolvable_with(year, lookups)
            }
            // Everything else is Gregorian arithmetic, astronomy or a
            // closure, none of which has a calendar range to fall outside.
            // The solar terms and the computus do have accuracy limits, but
            // those are a question about confidence, not about whether an
            // answer exists.
            _ => true,
        }
    }

    /// The days this rule falls on within Gregorian `year`.
    ///
    /// Returns an empty list when the rule yields nothing that year — 29
    /// February in a common year, 30 Ḥeshvan in a short Hebrew year, or a
    /// calendar date outside the supported range of its calendar. The last
    /// of those is not really "nothing"; ask [`Rule::is_resolvable_in`] to
    /// tell the two apart.
    #[must_use]
    pub fn days_in_year(&self, year: i64) -> Days {
        self.days_in_year_with(year, Window::ALL, &mut Uncached)
    }

    /// [`Rule::days_in_year`] with the astronomy answered through `lookups`
    /// and the work outside `window` skipped.
    ///
    /// The look-ups are the expensive part of a lunisolar rule — which
    /// Chinese years overlap a Gregorian one, where a Hindu month begins,
    /// the sunrise a tithi is read at — and every rule of a table asks the
    /// same questions, so the engine passes one [`EvaluationContext`]
    /// through all of them.
    ///
    /// The answer holds every day of [`Rule::days_in_year`] inside `window`
    /// and may hold or drop the days outside it: a rule that can tell
    /// cheaply that a month or a term lies wholly outside the window does
    /// not compute it. [`Window::ALL`] asks for everything.
    pub(crate) fn days_in_year_with<L: Lookups>(
        &self,
        year: i64,
        window: Window,
        lookups: &mut L,
    ) -> Days {
        let Ok(first) = gregorian::to_fixed(year, 1, 1) else {
            return Days::new();
        };
        let Ok(last) = gregorian::to_fixed(year, 12, 31) else {
            return Days::new();
        };
        match self {
            Self::FixedGregorian { month, day } => {
                gregorian::to_fixed(year, *month, *day).map_or_else(|_| Days::new(), Days::one)
            }
            Self::NthWeekday { month, n, weekday } => {
                let Some((start, end)) = gregorian_month_span(year, *month) else {
                    return Days::new();
                };
                weekday
                    .nth_within(i32::from(*n), start, end)
                    .map_or_else(Days::new, Days::one)
            }
            Self::LastWeekday { month, weekday } => {
                let Some((start, end)) = gregorian_month_span(year, *month) else {
                    return Days::new();
                };
                weekday
                    .nth_within(-1, start, end)
                    .map_or_else(Days::new, Days::one)
            }
            Self::WeekdayOnOrAfter {
                month,
                day,
                weekday,
            } => gregorian::to_fixed(year, *month, *day).map_or_else(
                |_| Days::new(),
                |anchor| Days::one(weekday.on_or_after(anchor)),
            ),
            Self::WeekdayOnOrBefore {
                month,
                day,
                weekday,
            } => gregorian::to_fixed(year, *month, *day).map_or_else(
                |_| Days::new(),
                |anchor| Days::one(weekday.on_or_before(anchor)),
            ),
            Self::FixedInCalendar { system, month, day } => fixed_in_calendar(
                *system,
                *month,
                *day,
                year,
                Window { first, last },
                window,
                lookups,
            ),
            Self::SolarTerm { term, meridian } => {
                if !window.overlaps(solar_term_bound(year, *term)) {
                    return Days::new();
                }
                Days::one(meridian.day_of(lookups.term_moment(year, *term)))
            }
            Self::Tithi {
                month,
                tithi,
                prevails,
                when_twice,
                calendar,
            } => tithi_days(
                year,
                *month,
                *tithi,
                *prevails,
                *when_twice,
                **calendar,
                window,
                lookups,
            )
            .clamped(first, last),
            Self::Sankranti {
                sign,
                ayanamsa,
                meridian,
            } => Days::one(meridian.day_of(lookups.ingress(year, *sign, **ayanamsa))),
            Self::Nakshatra {
                nakshatra,
                sign,
                with_tithi,
                ayanamsa,
                meridian,
            } => {
                lookups.nakshatra_days(year, *nakshatra, *sign, *with_tithi, **ayanamsa, *meridian)
            }
            Self::EasterRelative { computus, offset } => easter(**computus, year)
                .map_or_else(Days::new, |day| Days::one(Rd(day.0 + i64::from(*offset)))),
            Self::LunarPhase {
                phase,
                month,
                day,
                meridian,
            } => lookups.lunar_phase_day(*phase, year, *month, *day, *meridian),
            Self::Offset { base, days } => {
                // A shifted rule can leave its own Gregorian year, so the
                // neighbouring years are searched too and the result clamped.
                let mut out = Days::new();
                let window = window.shifted(-i64::from(*days));
                for probe in [year - 1, year, year + 1] {
                    for shifted in base
                        .days_in_year_with(probe, window, lookups)
                        .shifted(i32::from(*days))
                        .as_slice()
                    {
                        if *shifted >= first
                            && *shifted <= last
                            && !out.as_slice().contains(shifted)
                        {
                            out.push(*shifted);
                        }
                    }
                }
                out
            }
            Self::Span { from, to } => {
                // A span can straddle a New Year, so a first day in the year
                // before is searched as well, and every last day from this
                // year and the next is a candidate for the end.
                // A first day within a span of the window can begin a run
                // that reaches into it, and the end it takes is the first
                // within a span of that first day.
                let mut ends = Days::new();
                let end_window = window.widened(Self::MAX_SPAN, Self::MAX_SPAN);
                for probe in [year - 1, year, year + 1] {
                    for day in to.days_in_year_with(probe, end_window, lookups).as_slice() {
                        ends.push(*day);
                    }
                }
                let mut out = Days::new();
                let start_window = window.widened(Self::MAX_SPAN, 0);
                for probe in [year - 1, year] {
                    for start in from
                        .days_in_year_with(probe, start_window, lookups)
                        .as_slice()
                    {
                        let Some(end) = ends
                            .as_slice()
                            .iter()
                            .filter(|end| end.0 >= start.0 && end.0 - start.0 < Self::MAX_SPAN)
                            .min()
                        else {
                            continue;
                        };
                        for day in start.0..=end.0 {
                            let day = Rd(day);
                            if day >= first && day <= last && !out.as_slice().contains(&day) {
                                out.push(day);
                            }
                        }
                    }
                }
                out
            }
            Self::MovedByWeekday { base, moves } => {
                // As for a shifted rule: a move can cross the New Year, so
                // the neighbouring years are searched and the result clamped.
                let mut out = Days::new();
                let reach = moves
                    .iter()
                    .map(|(_, days)| i64::from(days.unsigned_abs()))
                    .max()
                    .unwrap_or(0);
                let window = window.widened(reach, reach);
                for probe in [year - 1, year, year + 1] {
                    for day in base.days_in_year_with(probe, window, lookups).as_slice() {
                        let weekday = Weekday::from_rd(*day);
                        let shift = moves
                            .iter()
                            .find(|(trigger, _)| *trigger == weekday)
                            .map_or(0, |(_, days)| i64::from(*days));
                        let moved = Rd(day.0 + shift);
                        if moved >= first && moved <= last && !out.as_slice().contains(&moved) {
                            out.push(moved);
                        }
                    }
                }
                out
            }
            Self::TibetanDay {
                calendar,
                month,
                day,
            } => tibetan_days(calendar, *month, *day, year).unwrap_or_default(),
            Self::Computed(function) => function(year).clamped(first, last),
            Self::Tabulated { function, .. } => function(year).clamped(first, last),
        }
    }
}

/// The moves of a law that sends a holiday to the following Monday whenever
/// it does not fall on one — Colombia's Ley Emiliani.
pub const TO_FOLLOWING_MONDAY: &[(Weekday, i16)] = &[
    (Weekday::Tuesday, 6),
    (Weekday::Wednesday, 5),
    (Weekday::Thursday, 4),
    (Weekday::Friday, 3),
    (Weekday::Saturday, 2),
    (Weekday::Sunday, 1),
];

/// The moves of a law that pulls a Tuesday or Wednesday holiday to the
/// Monday before and pushes a Thursday or Friday one to the Monday after,
/// leaving the weekend where it is — Argentina's Ley 27.399, article 6.
pub const TO_ADJACENT_MONDAY: &[(Weekday, i16)] = &[
    (Weekday::Tuesday, -1),
    (Weekday::Wednesday, -2),
    (Weekday::Thursday, 4),
    (Weekday::Friday, 3),
];

/// The days an evaluation is interested in, both ends included.
///
/// A rule asked for a year with a window narrower than the year may skip
/// the astronomy of anything it can prove falls outside it; see
/// [`Rule::days_in_year_with`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Window {
    /// The first day of interest.
    pub first: Rd,
    /// The last day of interest.
    pub last: Rd,
}

impl Window {
    /// Every day: nothing is skipped.
    pub const ALL: Self = Self {
        first: Rd(i64::MIN / 4),
        last: Rd(i64::MAX / 4),
    };

    /// Whether any day of `low..=high` is in the window.
    #[must_use]
    pub const fn overlaps(self, (low, high): (Rd, Rd)) -> bool {
        low.0 <= self.last.0 && high.0 >= self.first.0
    }

    /// The window opened `before` days earlier and `after` days later.
    #[must_use]
    pub const fn widened(self, before: i64, after: i64) -> Self {
        Self {
            first: Rd(self.first.0.saturating_sub(before)),
            last: Rd(self.last.0.saturating_add(after)),
        }
    }

    /// The window moved by `days`.
    #[must_use]
    pub const fn shifted(self, days: i64) -> Self {
        Self {
            first: Rd(self.first.0.saturating_add(days)),
            last: Rd(self.last.0.saturating_add(days)),
        }
    }
}

/// The days a solar term can fall on in a Gregorian year, at any meridian,
/// from the estimate its search starts at.
///
/// [`hc_astro::solar_longitude_after`] seeds its bisection with the day the
/// Sun would reach the term's longitude at its mean rate from the year's
/// first day, and brackets five days either side of that; the moment it
/// returns lies inside the bracket by construction. A meridian moves the
/// day by less than one more. The estimate is the search's own arithmetic,
/// repeated here so that the bound cannot disagree with it.
fn solar_term_bound(year: i64, term: SolarTerm) -> (Rd, Rd) {
    let new_year = Moment(hc_astro::time::gregorian_new_year(year).0 as f64);
    let rate = hc_astro::MEAN_TROPICAL_YEAR / 360.0;
    let to_go = term.solar_longitude_degrees() - hc_astro::solar_longitude(new_year);
    let to_go = to_go - 360.0 * floor(to_go / 360.0);
    let estimate = floor(new_year.0 + rate * to_go) as i64;
    (Rd(estimate - 7), Rd(estimate + 7))
}

/// The first and last fixed day of a Gregorian month.
fn gregorian_month_span(year: i64, month: u8) -> Option<(Rd, Rd)> {
    let start = gregorian::to_fixed(year, month, 1).ok()?;
    let length = gregorian::days_in_month(year, month)?;
    Some((start, Rd(start.0 + i64::from(length) - 1)))
}

/// The astronomy a rule asks for, answered directly or from a memo.
///
/// A few questions cost nearly everything a lunisolar rule costs: which
/// years of a calendar overlap a Gregorian year and where a date of it
/// falls, where a Hindu month begins and which tithi holds a part of a day,
/// when the Sun enters a sign or reaches a term. Every rule of a table asks
/// them about the same years, and every table asks them about the same
/// sky, so the engine answers them once through an [`EvaluationContext`]
/// and a caller without an allocator answers them each time through
/// [`Uncached`]. The answers are the same either way; only the cost
/// differs.
pub(crate) trait Lookups {
    /// [`CalendarSystem::year_containing`].
    fn year_containing(&mut self, system: CalendarSystem, day: Rd) -> Option<i64>;

    /// The years of `system` containing 1 January and 31 December of the
    /// Gregorian `year`, or `None` when either falls outside the calendar.
    fn calendar_years(&mut self, system: CalendarSystem, year: i64) -> Option<(i64, i64)> {
        let first = gregorian::to_fixed(year, 1, 1).ok()?;
        let last = gregorian::to_fixed(year, 12, 31).ok()?;
        Some((
            self.year_containing(system, first)?,
            self.year_containing(system, last)?,
        ))
    }

    /// [`CalendarSystem::to_fixed`].
    fn fixed_day(&mut self, system: CalendarSystem, year: i64, month: Month, day: u8)
    -> Option<Rd>;

    /// [`HinduLunarCalendar::month_span`] of the ordinary month `month` in
    /// Śaka year `saka`, or `None` where the calendar has no such month.
    fn hindu_month(
        &mut self,
        calendar: HinduLunarCalendar,
        saka: i64,
        month: u8,
    ) -> Option<(Rd, Rd)>;

    /// [`Prevalence::tithi_on`] at the calendar's place.
    fn tithi_at(&mut self, calendar: HinduLunarCalendar, prevails: Prevalence, day: Rd) -> u8;

    /// [`ingress_moment`]: the saṅkrānti into `sign` in a Gregorian year.
    fn ingress(&mut self, year: i64, sign: SiderealSign, ayanamsa: Ayanamsa) -> Moment;

    /// [`term_moment`]: the instant a solar term begins in a Gregorian year.
    fn term_moment(&mut self, year: i64, term: SolarTerm) -> Moment;

    /// [`lunar_phase_day`].
    fn lunar_phase_day(
        &mut self,
        phase: Phase,
        year: i64,
        month: u8,
        day: u8,
        meridian: Meridian,
    ) -> Days;

    /// [`nakshatra_days`].
    fn nakshatra_days(
        &mut self,
        year: i64,
        nakshatra: u8,
        sign: SiderealSign,
        with_tithi: Option<u8>,
        ayanamsa: Ayanamsa,
        meridian: Meridian,
    ) -> Days;
}

/// [`Lookups`] that computes every answer afresh.
pub(crate) struct Uncached;

impl Lookups for Uncached {
    fn year_containing(&mut self, system: CalendarSystem, day: Rd) -> Option<i64> {
        system.year_containing(day)
    }

    fn fixed_day(
        &mut self,
        system: CalendarSystem,
        year: i64,
        month: Month,
        day: u8,
    ) -> Option<Rd> {
        system.to_fixed(year, month, day)
    }

    fn hindu_month(
        &mut self,
        calendar: HinduLunarCalendar,
        saka: i64,
        month: u8,
    ) -> Option<(Rd, Rd)> {
        calendar.month_span(saka, month, false).ok()
    }

    fn tithi_at(&mut self, calendar: HinduLunarCalendar, prevails: Prevalence, day: Rd) -> u8 {
        prevails.tithi_on(day, calendar.location)
    }

    fn ingress(&mut self, year: i64, sign: SiderealSign, ayanamsa: Ayanamsa) -> Moment {
        ingress_moment(year, sign, ayanamsa)
    }

    fn term_moment(&mut self, year: i64, term: SolarTerm) -> Moment {
        term_moment(year, term)
    }

    fn lunar_phase_day(
        &mut self,
        phase: Phase,
        year: i64,
        month: u8,
        day: u8,
        meridian: Meridian,
    ) -> Days {
        lunar_phase_day(phase, year, month, day, meridian)
    }

    fn nakshatra_days(
        &mut self,
        year: i64,
        nakshatra: u8,
        sign: SiderealSign,
        with_tithi: Option<u8>,
        ayanamsa: Ayanamsa,
        meridian: Meridian,
    ) -> Days {
        nakshatra_days(year, nakshatra, sign, with_tithi, ayanamsa, meridian)
    }
}

/// A memo: keys in order, found by binary search.
///
/// An evaluation asks a few hundred distinct questions, for which a sorted
/// list is as quick as a tree and a great deal less code in a WebAssembly
/// module.
#[cfg(feature = "alloc")]
#[derive(Debug)]
struct Memo<K, V>(alloc::vec::Vec<(K, V)>);

#[cfg(feature = "alloc")]
impl<K, V> Default for Memo<K, V> {
    fn default() -> Self {
        Self(alloc::vec::Vec::new())
    }
}

#[cfg(feature = "alloc")]
impl<K: Ord, V: Copy> Memo<K, V> {
    /// The value under `key`, or `compute`'s, remembered.
    fn get_or_insert_with(&mut self, key: K, compute: impl FnOnce() -> V) -> V {
        let index = match self.0.binary_search_by(|(known, _)| known.cmp(&key)) {
            Ok(index) => return self.0.get(index).map_or_else(compute, |(_, value)| *value),
            Err(index) => index,
        };
        let value = compute();
        self.0.insert(index, (key, value));
        value
    }
}

/// A place as a memo key: the bits of its coordinates.
#[cfg(feature = "alloc")]
type LocationKey = [u64; 3];

/// An ayanamsa as a memo key: its name and the bits of its anchor.
#[cfg(feature = "alloc")]
type AyanamsaKey = (&'static str, u64, u64);

/// A Hindu month as a memo key: the calendar's place and ayanamsa, the
/// Śaka year and the month.
#[cfg(feature = "alloc")]
type HinduMonthKey = (LocationKey, AyanamsaKey, i64, u8);

/// A [`Rule::Nakshatra`] evaluation as a memo key: the year and the rule's
/// fields.
#[cfg(feature = "alloc")]
type NakshatraKey = (i64, u8, SiderealSign, Option<u8>, AyanamsaKey, Meridian);

#[cfg(feature = "alloc")]
fn location_key(location: Location) -> LocationKey {
    [
        location.latitude_degrees.to_bits(),
        location.longitude_degrees.to_bits(),
        location.elevation_metres.to_bits(),
    ]
}

#[cfg(feature = "alloc")]
fn ayanamsa_key(ayanamsa: Ayanamsa) -> AyanamsaKey {
    (
        ayanamsa.name(),
        ayanamsa.anchor_julian_date().to_bits(),
        ayanamsa.degrees_at_anchor().to_bits(),
    )
}

/// The memo of one evaluation: every answer the astronomy gave, kept for
/// the rest of the evaluation.
///
/// Building a [`HolidayCalendar`](crate::engine::HolidayCalendar) is
/// mostly astronomy — the conjunctions and solar terms of a lunisolar
/// calendar, the sunrises a tithi is read at — and the same questions come
/// back from every rule of a table and from every table that dates by the
/// same sky. A context answers each once. A page that asks "what is
/// today, in every table" builds one context and passes it to
/// [`HolidayCalendar::for_day_with`](crate::engine::HolidayCalendar::for_day_with)
/// for every table; the answers are the ones
/// [`HolidayCalendar::for_day`](crate::engine::HolidayCalendar::for_day)
/// gives, at a fraction of the cost.
///
/// Every key is integers: a day, a year, a month, the bits of a place. The
/// memo holds nothing that could go stale, because nothing it holds
/// depends on anything but its key, so a context can live as long as the
/// caller likes.
#[cfg(feature = "alloc")]
#[derive(Debug, Default)]
pub struct EvaluationContext {
    calendar_years: Memo<(CalendarId, Rd), Option<i64>>,
    fixed_days: Memo<(CalendarId, i64, Month, u8), Option<Rd>>,
    hindu_months: Memo<HinduMonthKey, Option<(Rd, Rd)>>,
    sunrises: Memo<(LocationKey, Rd), Moment>,
    sunsets: Memo<(LocationKey, Rd), Moment>,
    tithis: Memo<(LocationKey, Prevalence, Rd), u8>,
    ingresses: Memo<(i64, SiderealSign, AyanamsaKey), Moment>,
    terms: Memo<(i64, SolarTerm), Moment>,
    lunar_phases: Memo<(Phase, i64, u8, u8, Meridian), Days>,
    nakshatras: Memo<NakshatraKey, Days>,
}

#[cfg(feature = "alloc")]
impl EvaluationContext {
    /// An empty memo.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// [`sunrise_of`] a day at a place, once.
    fn sunrise(&mut self, location: Location, day: Rd) -> Moment {
        self.sunrises
            .get_or_insert_with((location_key(location), day), || sunrise_of(day, location))
    }

    /// [`sunset_of`] a day at a place, once.
    fn sunset(&mut self, location: Location, day: Rd) -> Moment {
        self.sunsets
            .get_or_insert_with((location_key(location), day), || sunset_of(day, location))
    }
}

#[cfg(feature = "alloc")]
impl Lookups for EvaluationContext {
    fn year_containing(&mut self, system: CalendarSystem, day: Rd) -> Option<i64> {
        self.calendar_years
            .get_or_insert_with((system.id, day), || system.year_containing(day))
    }

    fn fixed_day(
        &mut self,
        system: CalendarSystem,
        year: i64,
        month: Month,
        day: u8,
    ) -> Option<Rd> {
        self.fixed_days
            .get_or_insert_with((system.id, year, month, day), || {
                Uncached.fixed_day(system, year, month, day)
            })
    }

    fn hindu_month(
        &mut self,
        calendar: HinduLunarCalendar,
        saka: i64,
        month: u8,
    ) -> Option<(Rd, Rd)> {
        let key = (
            location_key(calendar.location),
            ayanamsa_key(calendar.ayanamsa),
            saka,
            month,
        );
        self.hindu_months
            .get_or_insert_with(key, || Uncached.hindu_month(calendar, saka, month))
    }

    fn tithi_at(&mut self, calendar: HinduLunarCalendar, prevails: Prevalence, day: Rd) -> u8 {
        let location = calendar.location;
        let key = (location_key(location), prevails, day);
        if let Ok(index) = self.tithis.0.binary_search_by(|(known, _)| known.cmp(&key))
            && let Some((_, tithi)) = self.tithis.0.get(index)
        {
            return *tithi;
        }
        // The same arithmetic as `Prevalence::tithi_on`, over sunrises
        // found once per day rather than once per rule.
        let rise = self.sunrise(location, day);
        let set = self.sunset(location, day);
        let moment = prevails.moment_between(rise, set, || self.sunrise(location, Rd(day.0 + 1)));
        let tithi = tithi_number_at(moment);
        self.tithis.get_or_insert_with(key, || tithi)
    }

    fn ingress(&mut self, year: i64, sign: SiderealSign, ayanamsa: Ayanamsa) -> Moment {
        self.ingresses
            .get_or_insert_with((year, sign, ayanamsa_key(ayanamsa)), || {
                ingress_moment(year, sign, ayanamsa)
            })
    }

    fn term_moment(&mut self, year: i64, term: SolarTerm) -> Moment {
        self.terms
            .get_or_insert_with((year, term), || term_moment(year, term))
    }

    fn lunar_phase_day(
        &mut self,
        phase: Phase,
        year: i64,
        month: u8,
        day: u8,
        meridian: Meridian,
    ) -> Days {
        self.lunar_phases
            .get_or_insert_with((phase, year, month, day, meridian), || {
                lunar_phase_day(phase, year, month, day, meridian)
            })
    }

    fn nakshatra_days(
        &mut self,
        year: i64,
        nakshatra: u8,
        sign: SiderealSign,
        with_tithi: Option<u8>,
        ayanamsa: Ayanamsa,
        meridian: Meridian,
    ) -> Days {
        let key = (
            year,
            nakshatra,
            sign,
            with_tithi,
            ayanamsa_key(ayanamsa),
            meridian,
        );
        self.nakshatras.get_or_insert_with(key, || {
            nakshatra_days(year, nakshatra, sign, with_tithi, ayanamsa, meridian)
        })
    }
}

/// Every occurrence of `month`/`day` in `system` that lands inside
/// `span`, the Gregorian `year`, and inside `window`.
///
/// The calendar years that can possibly overlap a Gregorian year are the one
/// containing 1 January through the one containing 31 December, so the search
/// is bounded without knowing anything about the calendar's year length; and
/// the ones that can reach the window are the one containing its first day
/// through the one containing its last, because a calendar's years are
/// contiguous, so a window narrower than the year — one lunation, for a
/// day — converts one date where a year converts two.
fn fixed_in_calendar<L: Lookups>(
    system: CalendarSystem,
    month: Month,
    day: u8,
    year: i64,
    span: Window,
    window: Window,
    lookups: &mut L,
) -> Days {
    let mut out = Days::new();
    let Some((from, to)) = lookups.calendar_years(system, year) else {
        return out;
    };
    let (low, high) = (span.first.max(window.first), span.last.min(window.last));
    if low > high {
        return out;
    }
    let from = lookups
        .year_containing(system, low)
        .map_or(from, |reached| reached.max(from));
    let to = lookups
        .year_containing(system, high)
        .map_or(to, |reached| reached.min(to));
    let mut candidate = from;
    while candidate <= to {
        if let Some(rd) = lookups.fixed_day(system, candidate, month, day)
            && rd >= span.first
            && rd <= span.last
        {
            out.push(rd);
        }
        candidate += 1;
    }
    out
}

/// The days a [`Rule::TibetanDay`] falls on in Gregorian `year`, or `None`
/// when the year cannot be answered: the day is skipped or repeated in a
/// Tibetan year whose occurrence lies within it, or the Tibetan years it
/// needs lie outside the calendar's range.
///
/// A Tibetan year begins between late January and March, so the two that
/// can reach Gregorian `year` are those numbered `year - 1` and `year`. A
/// skipped day lies, for this purpose, on the calendar day its lunar day
/// ends in: the day before the next number's first day, or, for a skipped
/// 30, the day numbered 29.
fn tibetan_days(
    calendar: &TibetanCalendar,
    month: TibetanMonth,
    day: u8,
    year: i64,
) -> Option<Days> {
    let first = gregorian::to_fixed(year, 1, 1).ok()?;
    let last = gregorian::to_fixed(year, 12, 31).ok()?;
    let within = |rd: Rd| first <= rd && rd <= last;
    let mut out = Days::new();
    for tibetan_year in [year - 1, year] {
        if !(tibetan::MIN_YEAR..=tibetan::MAX_YEAR).contains(&tibetan_year) {
            return None;
        }
        let leap_month = calendar.leap_month_of(tibetan_year);
        let month = match month {
            TibetanMonth::First
                if leap_month == Some(1)
                    && calendar.leap_numbering() == LeapNumbering::Following =>
            {
                Month::leap(1)
            }
            TibetanMonth::First => Month::regular(1),
            TibetanMonth::Regular(number) => Month::regular(number),
            TibetanMonth::Leap(number) if leap_month == Some(number) => Month::leap(number),
            TibetanMonth::Leap(_) => continue,
        };
        let date = |day: u8, leap_day: bool| {
            calendar
                .date_to_fixed(TibetanDate {
                    year: tibetan_year,
                    month,
                    day,
                    leap_day,
                })
                .ok()
        };
        match (date(day, false), date(day, true)) {
            (Some(regular), None) => {
                if within(regular) {
                    out.push(regular);
                }
            }
            (Some(regular), Some(extra)) => {
                if within(regular) || within(extra) {
                    return None;
                }
            }
            (None, _) => {
                let ends = if day < 30 {
                    date(day + 1, true)
                        .or_else(|| date(day + 1, false))
                        .map(|next| Rd(next.0 - 1))
                } else {
                    date(29, false)
                };
                if ends.is_none_or(within) {
                    return None;
                }
            }
        }
    }
    Some(out)
}

/// The first `phase` at or after a fixed Gregorian date, as a local day.
/// Which of two consecutive days a tithi rule takes when the tithi holds the
/// stated part of both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WhenTwice {
    /// The earlier day — the *pūrva-viddha* convention.
    Earlier,
    /// The later day — the *para-viddha* convention, as for Dīpāvalī and
    /// Janmāṣṭamī.
    Later,
}

/// The days a tithi of an amānta month can fall on, either side of the day
/// of the saṅkrānti that names the month plus the tithi's number.
///
/// The ordinary month holds the saṅkrānti and begins at the conjunction
/// before it, under thirty days earlier, so its first day is at least
/// thirty-one days before the saṅkrānti's and, since the first day is the
/// first whose sunrise follows that conjunction, at most two days after
/// it. [`tithi_days`] then takes a day from three before to three after
/// the day numbered `tithi` from the first, and one before the first. So
/// the day lies from thirty-five before to five after the saṅkrānti's day
/// plus `tithi`; the bounds here leave two days over that.
const TITHI_REACH: (i64, i64) = (37, 7);

/// The days in Gregorian `year` on which `tithi` of amānta `month` holds
/// the stated part of the day, in the ordinary month of that name.
///
/// A month whose tithi cannot reach `window` is not found: the saṅkrānti
/// that names the month bounds the day to [`TITHI_REACH`], and the
/// saṅkrānti is one search where the month is several and a dozen
/// sunrises.
// The five fields of `Rule::Tithi`, the year, the window and the memo.
#[allow(clippy::too_many_arguments)]
fn tithi_days<L: Lookups>(
    year: i64,
    month: u8,
    tithi: u8,
    prevails: Prevalence,
    when_twice: WhenTwice,
    calendar: HinduLunarCalendar,
    window: Window,
    lookups: &mut L,
) -> Days {
    let mut out = Days::new();
    // The month falls in one of the two Śaka years that overlap the
    // Gregorian one.
    for saka in [
        year - hindu_lunar::GREGORIAN_YEAR_OFFSET - 1,
        year - hindu_lunar::GREGORIAN_YEAR_OFFSET,
    ] {
        // The month is the one holding the saṅkrānti into its sign, which
        // falls in the Gregorian year the Śaka year begins in for Chaitra
        // through Mārgaśīrṣa and in the next for Pauṣa through Phālguna —
        // as `HinduLunarCalendar` places it.
        let Some(sign) = SiderealSign::from_index(month.wrapping_sub(1)) else {
            continue;
        };
        let sankranti_year = saka + hindu_lunar::GREGORIAN_YEAR_OFFSET + i64::from(month >= 10);
        let sankranti = lookups
            .ingress(sankranti_year, sign, calendar.ayanamsa)
            .day();
        let numbered = sankranti.0 + i64::from(tithi);
        if !window.overlaps((Rd(numbered - TITHI_REACH.0), Rd(numbered + TITHI_REACH.1))) {
            continue;
        }
        let Some((first, end)) = lookups.hindu_month(calendar, saka, month) else {
            continue;
        };
        // Tithis run from 0.9 to 1.1 days, so the `tithi`-th cannot drift
        // more than three days from the day numbered `tithi`; only that
        // window is read, which is what keeps a year's festivals cheap. The
        // first tithi begins at the new moon, which can fall after the
        // sunrise of the day before the month's first: the window opens
        // there, so that a pratipadā that holds only that afternoon, or
        // that no sunrise carries at all, is still found.
        let centre = first.0 + i64::from(tithi) - 1;
        let low = Rd(centre.saturating_sub(3).max(first.0 - 1));
        let high = Rd((centre + 4).min(end.0));
        let mut qualifying: [Option<Rd>; 2] = [None, None];
        let mut found = 0;
        let mut day = low;
        while day < high {
            if lookups.tithi_at(calendar, prevails, day) == tithi {
                if found < 2 {
                    qualifying[found] = Some(day);
                }
                found += 1;
            }
            day = Rd(day.0 + 1);
        }
        let chosen = match (qualifying, when_twice) {
            ([Some(_), Some(later)], WhenTwice::Later) => Some(later),
            ([Some(earlier), _], _) => Some(earlier),
            ([None, _], _) => {
                // The tithi holds the stated part of no day: take the day it
                // holds at sunrise, or the day a skipped tithi begins in.
                let mut day = low;
                let mut fallback = None;
                while day < high {
                    let at_sunrise = lookups.tithi_at(calendar, Prevalence::Sunrise, day);
                    if at_sunrise == tithi {
                        fallback = Some(day);
                        break;
                    }
                    // The thirtieth tithi is followed by the first.
                    let following = at_sunrise % TITHIS_PER_MONTH + 1;
                    let after_it = tithi % TITHIS_PER_MONTH + 1;
                    if following == tithi
                        && lookups.tithi_at(calendar, Prevalence::Sunrise, Rd(day.0 + 1))
                            == after_it
                    {
                        fallback = Some(day);
                        break;
                    }
                    day = Rd(day.0 + 1);
                }
                fallback
            }
        };
        if let Some(day) = chosen {
            out.push(day);
        }
    }
    out
}

/// The day of a nakṣatra in a solar month: see [`Rule::Nakshatra`].
fn nakshatra_days(
    year: i64,
    nakshatra: u8,
    sign: SiderealSign,
    with_tithi: Option<u8>,
    ayanamsa: Ayanamsa,
    meridian: Meridian,
) -> Days {
    use hc_seasons::zodiac::sidereal::{ingress_after, ingress_moment};
    let start = ingress_moment(year, sign, ayanamsa);
    let following = SiderealSign::from_index((sign.index() + 1) % 12).unwrap_or(sign);
    let end = ingress_after(following, ayanamsa, Moment(start.0 + 1.0));
    // The Moon's stays in the nakṣatra that fall in the month, clipped to
    // it: one, or two.
    let mut stays: [Option<(Moment, Moment)>; 2] = [None, None];
    let mut cursor = start;
    for slot in &mut stays {
        let (entry, exit) = nakshatra_span(nakshatra, cursor, ayanamsa);
        if entry.0 >= end.0 {
            break;
        }
        *slot = Some((Moment(entry.0.max(start.0)), Moment(exit.0.min(end.0))));
        cursor = Moment(exit.0 + 0.5);
    }
    let chosen = match (stays, with_tithi) {
        ([Some(first), Some(second)], Some(tithi))
            if degrees_from_tithi(second, tithi) < degrees_from_tithi(first, tithi) =>
        {
            second
        }
        ([Some(first), _], _) => first,
        ([None, _], _) => return Days::new(),
    };
    Days::one(day_holding_most_of(meridian, chosen))
}

/// How far, in degrees of elongation, the Moon stands from the middle of
/// a tithi's arc at the middle of a stay.
fn degrees_from_tithi((entry, exit): (Moment, Moment), tithi: u8) -> f64 {
    let middle = Moment(f64::midpoint(entry.0, exit.0));
    let centre = (f64::from(tithi.clamp(1, TITHIS_PER_MONTH)) - 0.5) * DEGREES_PER_TITHI;
    let away = normalize_degrees(hc_astro::lunar_phase(middle) - centre);
    away.min(360.0 - away)
}

/// The civil day at a meridian that holds the greater part of a span, the
/// earlier of two that hold the same.
fn day_holding_most_of(meridian: Meridian, (from, to): (Moment, Moment)) -> Rd {
    let mut day = meridian.day_of(from);
    let last = meridian.day_of(to);
    let mut best = (day, f64::NEG_INFINITY);
    while day.0 <= last.0 {
        let begins = meridian.midnight(day).0.max(from.0);
        let ends = meridian.midnight(Rd(day.0 + 1)).0.min(to.0);
        if ends - begins > best.1 {
            best = (day, ends - begins);
        }
        day = Rd(day.0 + 1);
    }
    best.0
}

fn lunar_phase_day(phase: Phase, year: i64, month: u8, day: u8, meridian: Meridian) -> Days {
    let Ok(anchor) = gregorian::to_fixed(year, month, day) else {
        return Days::new();
    };
    let start = meridian.midnight(anchor);
    let moment = hc_astro::moon_phase_at_or_after(phase.elongation_degrees(), start);
    Days::one(meridian.day_of(moment))
}

/// What sort of day an entry is.
///
/// The distinction matters because business-day arithmetic must count only
/// the days on which the country actually stops, and a commemoration without
/// a day off is not one of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    /// A statutory public holiday: work stops.
    Public,
    /// A bank holiday, where the banking day and the working day differ.
    Bank,
    /// A religious day of obligation that is not a statutory day off.
    Religious,
    /// A commemoration or flag day with no day off attached.
    Observance,
    /// A school holiday.
    School,
    /// A day made a working day by the authority that sets the calendar:
    /// in China, a weekend day the State Council's annual arrangement puts
    /// to work in exchange for a weekday off beside a holiday (调休上班).
    /// It is the opposite of a day off — business-day arithmetic counts it
    /// even when it falls on the weekend.
    Workday,
}

impl Kind {
    /// Whether this kind stops work, and therefore counts against
    /// business-day arithmetic.
    #[must_use]
    pub const fn is_day_off(self) -> bool {
        matches!(self, Self::Public | Self::Bank)
    }
}

/// How firm a computed date is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Confidence {
    /// The date follows from a rule that is fixed in law or in an
    /// arithmetic calendar, and this crate computes it exactly.
    Exact,
    /// The date is a prediction. Every Hijri-dated holiday is one, because
    /// the real date depends on a crescent sighting decided per country,
    /// sometimes on the night before; so is anything announced by annual
    /// decree rather than fixed in law, and anything whose calendar this
    /// crate only approximates.
    Approximate,
}

/// Which direction a weekend substitution moves in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubstituteDirection {
    /// Forwards only: the next day that is neither a weekend nor already a
    /// holiday. Japan's 振替休日 and the UK's bank-holiday shift.
    Forward,
    /// Backwards only: the last such day before the holiday.
    Backward,
    /// Outwards in the nearer direction: Saturday moves back to Friday,
    /// Sunday forward to Monday. The US federal "observed" rule.
    Nearest,
    /// Outwards in the nearer direction, to a working day: a Saturday
    /// holiday to the last working day before it, a Sunday holiday to the
    /// first working day after it, going on past a day already taken when
    /// the policy skips occupied days. Taiwan's rule, whose "前一個上班日"
    /// is the Thursday when the Friday is itself a holiday.
    NearestWorkingDay,
}

/// A country's weekend-substitution law.
///
/// The crate will not invent one of these. If a country's statute is not in
/// front of the author, its table carries no policy and its holidays simply
/// fall on the weekend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubstitutionPolicy {
    /// The weekdays that trigger a substitution.
    pub trigger: &'static [Weekday],
    /// Which way the substitute moves.
    pub direction: SubstituteDirection,
    /// Whether the search keeps going past a day that is already a holiday.
    ///
    /// Japan's original 1973 rule said simply "the following day"; the 2005
    /// amendment, in force from 2007, changed it to "the nearest following
    /// day that is not a holiday", which is this flag.
    pub skip_occupied: bool,
    /// Whether two holidays landing on the same day also trigger a
    /// substitution.
    ///
    /// South Korea's 대체공휴일 is triggered by a holiday coinciding with
    /// "a Sunday **or another public holiday**", which is how 5 May 2025 —
    /// Children's Day and Buddha's Birthday at once — produced a day off on
    /// 6 May. Most countries simply let the two coincide.
    pub on_collision: bool,
    /// The first Gregorian year the policy applies to.
    pub valid_from: Option<i32>,
    /// The last Gregorian year the policy applies to.
    pub valid_until: Option<i32>,
}

impl SubstitutionPolicy {
    /// Whether this policy is in force in `year`.
    #[must_use]
    pub const fn applies_in(&self, year: i64) -> bool {
        year_in_range(year, self.valid_from, self.valid_until)
    }
}

/// A "bridge" rule: a working day trapped between two holidays becomes one.
///
/// Japan's 国民の休日, introduced in 1985, is the only instance this crate
/// ships, and its exclusions — a Sunday, and a day that is already a
/// substitute holiday — are parameters rather than special cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgePolicy {
    /// The English name of the bridged day.
    pub name: &'static str,
    /// Its name in the local language, or `""` when there is none.
    pub local_name: &'static str,
    /// How many consecutive working days may be bridged. Japan's rule
    /// bridges exactly one.
    pub max_gap: u8,
    /// Weekdays that are never bridged.
    pub exclude_weekdays: &'static [Weekday],
    /// The first Gregorian year the policy applies to.
    pub valid_from: Option<i32>,
    /// The last Gregorian year the policy applies to.
    pub valid_until: Option<i32>,
}

impl BridgePolicy {
    /// Whether this policy is in force in `year`.
    #[must_use]
    pub const fn applies_in(&self, year: i64) -> bool {
        year_in_range(year, self.valid_from, self.valid_until)
    }
}

/// Which days of the week are the weekend, and when.
///
/// Friday–Saturday is the weekend in much of the Middle East and Friday
/// alone in Iran, and Saudi Arabia (2013), the United Arab Emirates (2022)
/// and Nepal (2026) all changed theirs within living memory, so a policy
/// carries the years it is in force like everything else — and, because
/// most of those changes took effect in the middle of a year, the day in
/// its first and last year where the source gives one. A bound with a
/// year and no day covers that year from 1 January or to 31 December, and
/// the table that carries it says what its source did not date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeekendPolicy {
    /// The weekend days.
    pub days: &'static [Weekday],
    /// The first Gregorian year the policy applies to.
    pub valid_from: Option<i32>,
    /// The month and day in `valid_from`'s year on which the policy takes
    /// effect, or `None` for 1 January.
    pub valid_from_day: Option<(u8, u8)>,
    /// The last Gregorian year the policy applies to.
    pub valid_until: Option<i32>,
    /// The month and day in `valid_until`'s year that is the policy's last,
    /// or `None` for 31 December.
    pub valid_until_day: Option<(u8, u8)>,
}

impl WeekendPolicy {
    /// Whether this policy is in force on `day`.
    #[must_use]
    pub fn applies_on(&self, day: Rd) -> bool {
        let Ok((year, month, day_of_month)) = gregorian::from_fixed(day) else {
            return false;
        };
        let here = (month, day_of_month);
        if let Some(first) = self.valid_from {
            let first = i64::from(first);
            if year < first
                || (year == first && self.valid_from_day.is_some_and(|start| here < start))
            {
                return false;
            }
        }
        if let Some(last) = self.valid_until {
            let last = i64::from(last);
            if year > last || (year == last && self.valid_until_day.is_some_and(|end| here > end)) {
                return false;
            }
        }
        true
    }
}

/// The Saturday–Sunday weekend, with no start or end date.
pub const SATURDAY_SUNDAY: &[WeekendPolicy] = &[WeekendPolicy {
    days: &[Weekday::Saturday, Weekday::Sunday],
    valid_from: None,
    valid_from_day: None,
    valid_until: None,
    valid_until_day: None,
}];

/// One holiday, as a value.
///
/// A `HolidayRule` is inert. It says what the holiday is called, what shape
/// it has, which years it existed in, which subdivisions it applies to and
/// whether the country's substitution law reaches it — and nothing about how
/// to evaluate any of that.
#[derive(Debug, Clone, Copy)]
pub struct HolidayRule {
    /// The English name.
    pub name: &'static str,
    /// The name in the local language and script, or `""` when the English
    /// name is the local one.
    pub local_name: &'static str,
    /// The shape.
    pub rule: Rule,
    /// What sort of day it is.
    pub kind: Kind,
    /// How firm the computed date is.
    pub confidence: Confidence,
    /// The first Gregorian year the holiday existed in, if it was created.
    pub valid_from: Option<i32>,
    /// The last Gregorian year the holiday existed in, if it was abolished.
    pub valid_until: Option<i32>,
    /// The subdivisions it applies to, as ISO 3166-2 codes. Empty means
    /// nationwide.
    pub regions: &'static [&'static str],
    /// The first year the country's substitution policy reaches this
    /// holiday, or `None` when it never does.
    ///
    /// South Korea is why this is per holiday rather than per country: its
    /// 대체공휴일 covered only Seollal, Chuseok and Children's Day until
    /// 2021, four more national days from August 2021, and Buddha's
    /// Birthday and Christmas from 2023.
    pub substitute_from: Option<i32>,
    /// Weekdays that trigger a substitution for this holiday alone,
    /// overriding the country policy's own `trigger`.
    ///
    /// Also South Korea: until 2021 a Seollal or Chuseok day was moved only
    /// when it fell on a Sunday, while Children's Day moved from a Saturday
    /// too.
    pub substitute_trigger: Option<&'static [Weekday]>,
    /// The direction a substitute moves for this holiday alone, overriding
    /// the country policy's own `direction`.
    ///
    /// Taiwan: a Saturday holiday is made up the working day before, except
    /// the Lunar New Year days, which are always made up after.
    pub substitute_direction: Option<SubstituteDirection>,
    /// The instrument that established this entry — a statute, a decree, a
    /// General Assembly resolution — or `""` when the table's `sources`
    /// speaks for it. The United Nations days cite their resolutions here,
    /// one by one, because "each entry cites its resolution" is a promise
    /// the docs make.
    pub source: &'static str,
}

impl HolidayRule {
    /// A nationwide, exact, substitutable public holiday — the common case.
    #[must_use]
    pub const fn public(name: &'static str, local_name: &'static str, rule: Rule) -> Self {
        Self {
            name,
            local_name,
            rule,
            kind: Kind::Public,
            confidence: Confidence::Exact,
            valid_from: None,
            valid_until: None,
            regions: &[],
            substitute_from: Some(i32::MIN),
            substitute_trigger: None,
            substitute_direction: None,
            source: "",
        }
    }

    /// The same rule, citing the instrument that established it.
    #[must_use]
    pub const fn cited(self, source: &'static str) -> Self {
        Self { source, ..self }
    }

    /// The same, but never substituted when it falls on a weekend.
    #[must_use]
    pub const fn fixed_public(name: &'static str, local_name: &'static str, rule: Rule) -> Self {
        Self {
            substitute_from: None,
            ..Self::public(name, local_name, rule)
        }
    }

    /// A commemoration with no day off.
    #[must_use]
    pub const fn observance(name: &'static str, local_name: &'static str, rule: Rule) -> Self {
        Self {
            kind: Kind::Observance,
            substitute_from: None,
            ..Self::public(name, local_name, rule)
        }
    }

    /// A weekend day made a working day. See [`Kind::Workday`].
    #[must_use]
    pub const fn workday(name: &'static str, local_name: &'static str, rule: Rule) -> Self {
        Self {
            kind: Kind::Workday,
            substitute_from: None,
            ..Self::public(name, local_name, rule)
        }
    }

    /// The same rule, restricted to a span of Gregorian years.
    #[must_use]
    pub const fn years(self, from: Option<i32>, until: Option<i32>) -> Self {
        Self {
            valid_from: from,
            valid_until: until,
            ..self
        }
    }

    /// The same rule, restricted to a set of subdivisions.
    #[must_use]
    pub const fn in_regions(self, regions: &'static [&'static str]) -> Self {
        Self { regions, ..self }
    }

    /// The same rule, marked as a prediction rather than a fact.
    #[must_use]
    pub const fn approximate(self) -> Self {
        Self {
            confidence: Confidence::Approximate,
            ..self
        }
    }

    /// The same rule, with a different [`Kind`].
    #[must_use]
    pub const fn of_kind(self, kind: Kind) -> Self {
        Self { kind, ..self }
    }

    /// The same rule, substituted only from `year` onwards.
    #[must_use]
    pub const fn substituted_from(self, year: i32) -> Self {
        Self {
            substitute_from: Some(year),
            ..self
        }
    }

    /// The same rule, with its own set of weekdays that trigger a
    /// substitution.
    #[must_use]
    pub const fn substitute_on(self, trigger: &'static [Weekday]) -> Self {
        Self {
            substitute_trigger: Some(trigger),
            ..self
        }
    }

    /// The same rule, with its own direction for a substitute.
    #[must_use]
    pub const fn substitute_towards(self, direction: SubstituteDirection) -> Self {
        Self {
            substitute_direction: Some(direction),
            ..self
        }
    }

    /// Whether the holiday existed in `year`.
    #[must_use]
    pub const fn applies_in(&self, year: i64) -> bool {
        year_in_range(year, self.valid_from, self.valid_until)
    }

    /// Whether the holiday applies in `region`.
    ///
    /// `None` asks for the nationwide set: only rules with no subdivision
    /// scoping qualify. `Some(code)` adds the rules scoped to that code.
    #[must_use]
    pub fn applies_in_region(&self, region: Option<&str>) -> bool {
        if self.regions.is_empty() {
            return true;
        }
        region.is_some_and(|code| self.regions.contains(&code))
    }

    /// Whether the country's substitution policy reaches this holiday in
    /// `year`.
    #[must_use]
    pub const fn substitutes_in(&self, year: i64) -> bool {
        match self.substitute_from {
            None => false,
            Some(from) => year >= from as i64,
        }
    }
}

/// The day a rule set's sources were last checked against their statute.
///
/// `docs/observances.md` is blunt about why this exists: a holiday without a
/// source is a rumour, and a holiday list without a date is a rumour about
/// when it was true.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceDate {
    /// The Gregorian year.
    pub year: i32,
    /// The Gregorian month, 1–12.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl SourceDate {
    /// A source-checked date.
    #[must_use]
    pub const fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

/// A rule set another keeps the days off of, and which of its regions:
/// see [`RuleSet::includes`].
///
/// The region belongs to the inclusion, not to whoever asks. The London
/// Stock Exchange closes on the bank holidays of England and Wales, and
/// asking for its calendar in no particular region still means those.
#[derive(Debug, Clone, Copy)]
pub struct Include {
    /// The set whose days off are kept.
    pub set: &'static RuleSet,
    /// The region of it to evaluate, or `None` for its nationwide days.
    pub region: Option<&'static str>,
}

impl Include {
    /// A set's nationwide days off.
    #[must_use]
    pub const fn nationwide(set: &'static RuleSet) -> Self {
        Self { set, region: None }
    }

    /// A set's days off in one of its regions, the nationwide ones among
    /// them.
    #[must_use]
    pub const fn in_region(set: &'static RuleSet, region: &'static str) -> Self {
        Self {
            set,
            region: Some(region),
        }
    }
}

/// A named table of holiday rules, with the policies that modify them.
///
/// One country is one of these; so is one religious tradition. The evaluator
/// in [`crate::engine`] takes nothing else, which is what lets a caller pass
/// a company calendar, a school year or a fictional setting and get the same
/// engine.
#[derive(Debug, Clone, Copy)]
pub struct RuleSet {
    /// A stable identifier — an ISO 3166-1 alpha-2 code for a country, a
    /// lower-case slug for a tradition.
    pub code: &'static str,
    /// The English name of the country or tradition.
    pub english_name: &'static str,
    /// The rules themselves.
    pub rules: &'static [HolidayRule],
    /// The weekend-substitution law, in force over stated years.
    pub substitution: &'static [SubstitutionPolicy],
    /// Bridge policies, such as Japan's 国民の休日.
    pub bridges: &'static [BridgePolicy],
    /// Other rule sets whose days off this one keeps as well, each
    /// evaluated under its own policies, in the region the [`Include`]
    /// names, and merged in: an exchange that closes on its country's
    /// holidays includes the country's table and lists only its own days.
    /// Only the days off come along — public and bank holidays, with their
    /// substitutes and bridges — and not the included set's religious days
    /// or observances, which are its own: Hong Kong's Winter Solstice is
    /// not a day its exchange notes. A set may include a set that includes
    /// another; the engine follows eight levels and no further.
    pub includes: &'static [Include],
    /// Which days are the weekend, over stated years.
    pub weekend: &'static [WeekendPolicy],
    /// When the sources behind this table were last checked.
    pub sources_checked: SourceDate,
    /// The statute, gazette or official calendar the table came from.
    pub sources: &'static str,
}

impl RuleSet {
    /// The substitution policy in force in `year`, if any.
    #[must_use]
    pub fn substitution_in(&self, year: i64) -> Option<&'static SubstitutionPolicy> {
        self.substitution
            .iter()
            .find(|policy| policy.applies_in(year))
    }

    /// The weekend days in force on `day`.
    ///
    /// Asked of a day rather than a year, because a weekend law can change
    /// in the middle of one. Falls back to Saturday–Sunday when the table
    /// states nothing, because a table that states nothing has not made a
    /// claim.
    #[must_use]
    pub fn weekend_on(&self, day: Rd) -> &'static [Weekday] {
        self.weekend
            .iter()
            .find(|policy| policy.applies_on(day))
            .map_or(&[Weekday::Saturday, Weekday::Sunday], |policy| policy.days)
    }

    /// Every subdivision code mentioned anywhere in the table.
    ///
    /// Returned as an iterator of the raw codes, which may repeat; callers
    /// that want a set should build one.
    pub fn region_codes(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.rules
            .iter()
            .flat_map(|rule| rule.regions.iter().copied())
    }
}

/// Whether `year` falls inside an inclusive, optionally open-ended range.
const fn year_in_range(year: i64, from: Option<i32>, until: Option<i32>) -> bool {
    if let Some(first) = from
        && year < first as i64
    {
        return false;
    }
    if let Some(last) = until
        && year > last as i64
    {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    #[test]
    fn a_fixed_gregorian_rule_yields_one_day_a_year() {
        let rule = Rule::FixedGregorian { month: 1, day: 1 };
        assert_eq!(rule.days_in_year(2024).as_slice(), &[ymd(2024, 1, 1)]);
    }

    #[test]
    fn the_twenty_ninth_of_february_is_absent_from_a_common_year() {
        let rule = Rule::FixedGregorian { month: 2, day: 29 };
        assert!(rule.days_in_year(2023).is_empty());
        assert_eq!(rule.days_in_year(2024).as_slice(), &[ymd(2024, 2, 29)]);
    }

    #[test]
    fn nth_weekday_finds_us_thanksgiving() {
        let rule = Rule::NthWeekday {
            month: 11,
            n: 4,
            weekday: Weekday::Thursday,
        };
        assert_eq!(rule.days_in_year(2024).as_slice(), &[ymd(2024, 11, 28)]);
        assert_eq!(rule.days_in_year(2025).as_slice(), &[ymd(2025, 11, 27)]);
    }

    #[test]
    fn last_weekday_finds_the_uk_spring_bank_holiday() {
        let rule = Rule::LastWeekday {
            month: 5,
            weekday: Weekday::Monday,
        };
        assert_eq!(rule.days_in_year(2024).as_slice(), &[ymd(2024, 5, 27)]);
        assert_eq!(rule.days_in_year(2023).as_slice(), &[ymd(2023, 5, 29)]);
    }

    #[test]
    fn weekday_on_or_after_finds_swedish_midsummer_day() {
        // Midsommardagen is the Saturday falling 20–26 June.
        let rule = Rule::WeekdayOnOrAfter {
            month: 6,
            day: 20,
            weekday: Weekday::Saturday,
        };
        assert_eq!(rule.days_in_year(2024).as_slice(), &[ymd(2024, 6, 22)]);
        assert_eq!(rule.days_in_year(2025).as_slice(), &[ymd(2025, 6, 21)]);
    }

    #[test]
    fn a_rule_moved_by_weekday_lands_on_the_monday_its_table_says() {
        // San Martín, 17 August, under Argentina's article 6: a Monday
        // (2026) stays, a Tuesday (2027) pulls back, a Thursday (2028)
        // pushes on, a Sunday (2025) is not listed and stays.
        static SAN_MARTIN: Rule = Rule::gregorian(8, 17);
        let argentina = Rule::moved_by_weekday(&SAN_MARTIN, TO_ADJACENT_MONDAY);
        assert_eq!(argentina.days_in_year(2026).as_slice(), &[ymd(2026, 8, 17)]);
        assert_eq!(argentina.days_in_year(2027).as_slice(), &[ymd(2027, 8, 16)]);
        assert_eq!(argentina.days_in_year(2028).as_slice(), &[ymd(2028, 8, 21)]);
        assert_eq!(argentina.days_in_year(2025).as_slice(), &[ymd(2025, 8, 17)]);
        // Epiphany under Colombia's Ley Emiliani: a Monday (2025) stays, a
        // Tuesday (2026) and a Saturday (2024) go to the Monday after.
        static EPIPHANY: Rule = Rule::gregorian(1, 6);
        let colombia = Rule::moved_by_weekday(&EPIPHANY, TO_FOLLOWING_MONDAY);
        assert_eq!(colombia.days_in_year(2025).as_slice(), &[ymd(2025, 1, 6)]);
        assert_eq!(colombia.days_in_year(2026).as_slice(), &[ymd(2026, 1, 12)]);
        assert_eq!(colombia.days_in_year(2024).as_slice(), &[ymd(2024, 1, 8)]);
        // A move never leaves the year it is asked about with a duplicate,
        // and a base in the neighbouring year is found: 31 December on a
        // Tuesday (2024) becomes Monday 30 December.
        static YEARS_END: Rule = Rule::gregorian(12, 31);
        let moved = Rule::moved_by_weekday(&YEARS_END, TO_ADJACENT_MONDAY);
        assert_eq!(moved.days_in_year(2024).as_slice(), &[ymd(2024, 12, 30)]);
        assert!(moved.is_resolvable_in(2024));
    }

    #[test]
    fn a_hijri_anniversary_can_fall_twice_in_one_gregorian_year() {
        // 1 Muḥarram fell on 1 January and on 21 December 2008 under the
        // tabular civil reckoning.
        let rule = Rule::FixedInCalendar {
            system: CalendarSystem::ISLAMIC_CIVIL,
            month: Month::regular(1),
            day: 1,
        };
        assert_eq!(rule.days_in_year(2008).len(), 2);
    }

    #[test]
    fn the_solar_term_rule_gives_japans_equinox_days() {
        // 官報: 2024-03-20 春分の日, 2024-09-22 秋分の日.
        let spring = Rule::SolarTerm {
            term: SolarTerm::SPRING_EQUINOX,
            meridian: Meridian::JAPAN,
        };
        let autumn = Rule::SolarTerm {
            term: SolarTerm::AUTUMN_EQUINOX,
            meridian: Meridian::JAPAN,
        };
        assert_eq!(spring.days_in_year(2024).as_slice(), &[ymd(2024, 3, 20)]);
        assert_eq!(autumn.days_in_year(2024).as_slice(), &[ymd(2024, 9, 22)]);
    }

    #[test]
    fn an_offset_rule_can_reach_back_into_the_previous_year() {
        // New Year's Eve, expressed as the day before New Year's Day.
        static BASE: Rule = Rule::FixedGregorian { month: 1, day: 1 };
        let rule = Rule::Offset {
            base: &BASE,
            days: -1,
        };
        assert_eq!(rule.days_in_year(2024).as_slice(), &[ymd(2024, 12, 31)]);
    }

    #[test]
    fn region_scoping_hides_a_subdivision_rule_from_the_national_set() {
        let rule = HolidayRule::public("Test", "", Rule::FixedGregorian { month: 1, day: 2 })
            .in_regions(&["DE-BY"]);
        assert!(!rule.applies_in_region(None));
        assert!(rule.applies_in_region(Some("DE-BY")));
        assert!(!rule.applies_in_region(Some("DE-BE")));
    }

    #[test]
    fn validity_years_are_inclusive_on_both_sides() {
        let rule = HolidayRule::public("Test", "", Rule::FixedGregorian { month: 1, day: 2 })
            .years(Some(1996), Some(2019));
        assert!(!rule.applies_in(1995));
        assert!(rule.applies_in(1996));
        assert!(rule.applies_in(2019));
        assert!(!rule.applies_in(2020));
    }

    #[test]
    fn days_drops_anything_past_its_capacity() {
        let mut days = Days::new();
        for offset in 0..(Days::CAPACITY as i64 + 5) {
            days.push(Rd(offset));
        }
        assert_eq!(days.len(), Days::CAPACITY);
    }

    #[test]
    fn a_rule_is_no_larger_than_five_words() {
        // Every table entry holds a `Rule`; a variant that holds a large
        // value inline widens every entry of every table. Five words is
        // `FixedInCalendar`, a calendar system and a month.
        assert!(
            core::mem::size_of::<Rule>() <= 5 * core::mem::size_of::<u64>(),
            "Rule is {} bytes",
            core::mem::size_of::<Rule>()
        );
    }

    #[test]
    fn a_tibetan_day_is_a_gap_where_its_number_is_skipped_or_repeated() {
        use hc_calendars_lunar::tibetan::{MONGOLIAN, TIBETAN_BHUTAN};
        // Tsagaan Sar 2025: the first number is skipped, and Saturday
        // 1 March is the second (Resolution No. 109).
        let first = Rule::tibetan(&MONGOLIAN, TibetanMonth::First, 1);
        assert!(!first.is_resolvable_in(2025));
        assert!(first.days_in_year(2025).is_empty());
        let second = Rule::tibetan(&MONGOLIAN, TibetanMonth::First, 2);
        assert_eq!(second.days_in_year(2025).as_slice(), &[ymd(2025, 3, 1)]);
        // The calendar system has no day for the skipped number.
        let system = CalendarSystem::MONGOLIAN;
        assert_eq!(system.to_fixed(2025, Month::regular(1), 1), None);
        assert_eq!(
            system.to_fixed(2025, Month::regular(1), 2),
            Some(ymd(2025, 3, 1))
        );
        // The fifteenth of the fourth month of 2034 is repeated, on 1 and
        // 2 June; the system gives the second.
        let fifteenth = Rule::tibetan(&MONGOLIAN, TibetanMonth::Regular(4), 15);
        assert!(!fifteenth.is_resolvable_in(2034));
        assert_eq!(
            system.to_fixed(2034, Month::regular(4), 15),
            Some(ymd(2034, 6, 2))
        );
        // Mongolian 2006 opens with a leap month 1, which holds the New
        // Year; the regular month 1 follows it.
        assert_eq!(first.days_in_year(2006).as_slice(), &[ymd(2006, 1, 30)]);
        let leap_first = Rule::tibetan(&MONGOLIAN, TibetanMonth::Leap(1), 1);
        assert_eq!(
            leap_first.days_in_year(2006).as_slice(),
            &[ymd(2006, 1, 30)]
        );
        assert!(leap_first.is_resolvable_in(2007));
        assert!(leap_first.days_in_year(2007).is_empty());
        // The Bhutanese leap month 12 of 2021 follows the regular one.
        let regular = Rule::tibetan(&TIBETAN_BHUTAN, TibetanMonth::Regular(12), 1);
        let leap = Rule::tibetan(&TIBETAN_BHUTAN, TibetanMonth::Leap(12), 1);
        assert_eq!(regular.days_in_year(2022).as_slice(), &[ymd(2022, 1, 3)]);
        let after = leap.days_in_year(2022);
        assert_eq!(after.len(), 1);
        assert!(after.as_slice()[0] > ymd(2022, 1, 3) && after.as_slice()[0] < ymd(2022, 3, 3));
        // The calendar converts from 1000; Gregorian 1000 needs 999.
        assert!(!first.is_resolvable_in(1000));
        assert!(first.is_resolvable_in(1001));
    }
}
