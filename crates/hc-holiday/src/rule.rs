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

use hc_calendar::Calendar as _;
use hc_calendar::{CalendarId, Month, Rd, Weekday};
use hc_calendars_equinox::persian as solar_hijri;
use hc_calendars_indic::tithi::tithi_of_day;
use hc_calendars_indic::{HinduLunarCalendar, Prevalence, hindu_lunar};
use hc_calendars_lunar::hebrew;
use hc_calendars_lunar::islamic_umalqura;
use hc_calendars_lunar::tabular::{self, LeapYearRule};
use hc_calendars_lunar::{ChineseCalendar, DangiCalendar, LunisolarDate, VietnameseCalendar};
use hc_calendars_regional::burmese;
use hc_calendars_solar::{
    bahai_kept, coptic, ethiopic, gregorian, julian, nanakshahi, persian, zoroastrian,
};
use hc_seasons::solar_terms::term_day;
use hc_seasons::zodiac::{Ayanamsa, SiderealSign};
use hc_seasons::{Meridian, SolarTerm};

use crate::computus::{Computus, easter};

/// The days a single rule yields within one Gregorian year.
///
/// A calendar whose year is shorter than the Gregorian one can put the same
/// anniversary in a Gregorian year twice — 1 Muḥarram fell on both
/// 1 January and 21 December of 2008 — so the answer is a small list rather
/// than an `Option`. Four slots is one more than any rule in this crate has
/// ever needed, and keeps the type `Copy` and allocation-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Days {
    days: [Rd; Days::CAPACITY],
    len: u8,
}

impl Days {
    /// How many days a single rule may yield in one Gregorian year.
    pub const CAPACITY: usize = 4;

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
    /// vocabulary can produce a fifth day: the shortest calendar year in the
    /// crate is the Hijri one at about 354 days, which fits an anniversary
    /// into a Gregorian year at most twice.
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
/// It was an enum, with this justification: "the whole point of the crate
/// is that a holiday table is a `static` value, and a trait object cannot
/// be one without an allocator". The first half is right and the second is
/// true of `dyn Calendar` — but neither requires a *closed set*.
///
/// A closed set is a hole generator. A holiday dated in the Ethiopic,
/// Coptic, Solar Hijri or Badíʿ calendar could not be written down at all,
/// however much anyone wanted to: the rule vocabulary had no way to say it,
/// so Ethiopian Christmas had to be approximated in some other calendar or
/// left out. Adding one meant editing an enum and two match arms in a crate
/// the person who wanted it does not own.
///
/// Two function pointers and an identifier are `Copy`, `const`-constructible
/// and `static`-safe, exactly as the enum was, and open: a caller with a
/// calendar this crate has never heard of can build one and date a holiday
/// in it.
///
/// The identifier is a real [`CalendarId`], and
/// `hyper-calendar/tests/holiday_calendars.rs` asserts that every system
/// here names a calendar the registry answers to — the same guard the
/// vocabulary layer gained, for the same reason.
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

/// A lunar phase a [`Rule::LunarPhase`] can key to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        computus: Computus,
        /// The offset in days.
        offset: i16,
    },
    /// The first occurrence of a lunar phase on or after a fixed Gregorian
    /// date, at a stated meridian.
    ///
    /// This is the crude way to date a Buddhist observance, and the crate
    /// says so: see [`crate::traditions::BUDDHIST`].
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
        calendar: HinduLunarCalendar,
    },
    /// The Sun's entry into a sidereal sign — a saṅkrānti — as a day at a
    /// meridian: Makara Saṅkrānti, the solar new year of Meṣa.
    Sankranti {
        /// The sign entered.
        sign: SiderealSign,
        /// The ayanamsa that fixes the sidereal zero point.
        ayanamsa: Ayanamsa,
        /// The meridian whose local midnight cuts the day.
        meridian: Meridian,
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
            computus: Computus::GREGORIAN,
            offset,
        }
    }

    /// An offset from Orthodox Easter.
    #[must_use]
    pub const fn paschal(offset: i16) -> Self {
        Self::EasterRelative {
            computus: Computus::JULIAN,
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
        match self {
            Self::FixedInCalendar { system, .. } => system.covers_gregorian_year(year),
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
            Self::Offset { base, .. } | Self::MovedByWeekday { base, .. } => {
                base.is_resolvable_in(year)
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
            Self::FixedInCalendar { system, month, day } => {
                fixed_in_calendar(*system, *month, *day, first, last)
            }
            Self::SolarTerm { term, meridian } => Days::one(term_day(year, *term, *meridian)),
            Self::Tithi {
                month,
                tithi,
                prevails,
                when_twice,
                calendar,
            } => tithi_days(year, *month, *tithi, *prevails, *when_twice, *calendar)
                .clamped(first, last),
            Self::Sankranti {
                sign,
                ayanamsa,
                meridian,
            } => Days::one(hc_seasons::zodiac::sidereal::ingress_day(
                year, *sign, *ayanamsa, *meridian,
            )),
            Self::EasterRelative { computus, offset } => easter(*computus, year)
                .map_or_else(Days::new, |day| Days::one(Rd(day.0 + i64::from(*offset)))),
            Self::LunarPhase {
                phase,
                month,
                day,
                meridian,
            } => lunar_phase_day(*phase, year, *month, *day, *meridian),
            Self::Offset { base, days } => {
                // A shifted rule can leave its own Gregorian year, so the
                // neighbouring years are searched too and the result clamped.
                let mut out = Days::new();
                for probe in [year - 1, year, year + 1] {
                    for shifted in base
                        .days_in_year(probe)
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
            Self::MovedByWeekday { base, moves } => {
                // As for a shifted rule: a move can cross the New Year, so
                // the neighbouring years are searched and the result clamped.
                let mut out = Days::new();
                for probe in [year - 1, year, year + 1] {
                    for day in base.days_in_year(probe).as_slice() {
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

/// The first and last fixed day of a Gregorian month.
fn gregorian_month_span(year: i64, month: u8) -> Option<(Rd, Rd)> {
    let start = gregorian::to_fixed(year, month, 1).ok()?;
    let length = gregorian::days_in_month(year, month)?;
    Some((start, Rd(start.0 + i64::from(length) - 1)))
}

/// Every occurrence of `month`/`day` in `system` that lands inside
/// `[first, last]`.
///
/// The calendar years that can possibly overlap a Gregorian year are the one
/// containing 1 January through the one containing 31 December, so the search
/// is bounded without knowing anything about the calendar's year length.
fn fixed_in_calendar(system: CalendarSystem, month: Month, day: u8, first: Rd, last: Rd) -> Days {
    let mut out = Days::new();
    let Some(from) = system.year_containing(first) else {
        return out;
    };
    let Some(to) = system.year_containing(last) else {
        return out;
    };
    let mut candidate = from;
    while candidate <= to {
        if let Some(rd) = system.to_fixed(candidate, month, day)
            && rd >= first
            && rd <= last
        {
            out.push(rd);
        }
        candidate += 1;
    }
    out
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

/// The days in Gregorian `year` on which `tithi` of amānta `month` holds
/// the stated part of the day, in the ordinary month of that name.
fn tithi_days(
    year: i64,
    month: u8,
    tithi: u8,
    prevails: Prevalence,
    when_twice: WhenTwice,
    calendar: HinduLunarCalendar,
) -> Days {
    let mut out = Days::new();
    // The month falls in one of the two Śaka years that overlap the
    // Gregorian one.
    for saka in [
        year - hindu_lunar::GREGORIAN_YEAR_OFFSET - 1,
        year - hindu_lunar::GREGORIAN_YEAR_OFFSET,
    ] {
        let Ok((first, end)) = calendar.month_span(saka, month, false) else {
            continue;
        };
        let location = calendar.location;
        // Tithis run from 0.9 to 1.1 days, so the `tithi`-th cannot drift
        // more than three days from the day numbered `tithi`; only that
        // window is read, which is what keeps a year's festivals cheap.
        let centre = first.0 + i64::from(tithi) - 1;
        let low = Rd(centre.saturating_sub(3).max(first.0));
        let high = Rd((centre + 4).min(end.0));
        let mut qualifying: [Option<Rd>; 2] = [None, None];
        let mut found = 0;
        let mut day = low;
        while day < high {
            if prevails.tithi_on(day, location) == tithi {
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
                    let at_sunrise = tithi_of_day(day, location);
                    if at_sunrise == tithi {
                        fallback = Some(day);
                        break;
                    }
                    if at_sunrise + 1 == tithi && tithi_of_day(Rd(day.0 + 1), location) == tithi + 1
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
/// Friday–Saturday is the weekend in much of the Middle East, Sunday alone in
/// Nepal, and both Saudi Arabia (2013) and the United Arab Emirates (2022)
/// changed theirs within living memory, so this carries years like everything
/// else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeekendPolicy {
    /// The weekend days.
    pub days: &'static [Weekday],
    /// The first Gregorian year the policy applies to.
    pub valid_from: Option<i32>,
    /// The last Gregorian year the policy applies to.
    pub valid_until: Option<i32>,
}

impl WeekendPolicy {
    /// Whether this policy is in force in `year`.
    #[must_use]
    pub const fn applies_in(&self, year: i64) -> bool {
        year_in_range(year, self.valid_from, self.valid_until)
    }
}

/// The Saturday–Sunday weekend, with no start or end date.
pub const SATURDAY_SUNDAY: &[WeekendPolicy] = &[WeekendPolicy {
    days: &[Weekday::Saturday, Weekday::Sunday],
    valid_from: None,
    valid_until: None,
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
    /// 2021, four more national days from July 2021, and Buddha's Birthday
    /// and Christmas from 2023.
    pub substitute_from: Option<i32>,
    /// Weekdays that trigger a substitution for this holiday alone,
    /// overriding the country policy's own `trigger`.
    ///
    /// Also South Korea: until 2021 a Seollal or Chuseok day was moved only
    /// when it fell on a Sunday, while Children's Day moved from a Saturday
    /// too.
    pub substitute_trigger: Option<&'static [Weekday]>,
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

    /// The weekend days in force in `year`.
    ///
    /// Falls back to Saturday–Sunday when the table states nothing, because
    /// a table that states nothing has not made a claim.
    #[must_use]
    pub fn weekend_in(&self, year: i64) -> &'static [Weekday] {
        self.weekend
            .iter()
            .find(|policy| policy.applies_in(year))
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
        for offset in 0..10 {
            days.push(Rd(offset));
        }
        assert_eq!(days.len(), Days::CAPACITY);
    }
}
