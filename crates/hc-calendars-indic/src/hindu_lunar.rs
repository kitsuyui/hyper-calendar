//! The Hindu lunisolar calendar, *amānta* — `hindu-lunar`.
//!
//! The system is written up in `docs/systems/hindu-calendars.md` in the
//! repository: the tithi, the naming of months by their saṅkrānti, adhika
//! and kṣaya months, the Śaka and Vikrama years, the Central Station and
//! the ayanāṃśa, with the adhika Śrāvaṇa of Śaka 1945 worked through by
//! hand, what is carried and what is not, how the calendar was checked
//! against the *Rashtriya Panchang*'s tables, and the sources, keyed in
//! `docs/references.bib`. This page summarises it and states the code's
//! own facts.
//!
//! The calendar most of India dates its festivals in, computed the way the
//! *Rashtriya Panchang* of the Government of India computes it: from the
//! true positions of the Sun and Moon, with the sidereal zodiac fixed by the
//! Lahiri ayanāṃśa, and the day read at sunrise.
//!
//! # The rules
//!
//! 1. **A month runs from new moon to new moon**, and its days are tithis:
//!    the day's tithi is the one in progress at sunrise ([`crate::tithi`]).
//!    The month begins with the first day whose sunrise follows the
//!    conjunction — śukla pratipadā — and ends with amāvāsyā, the day of the
//!    next conjunction. That is the *amānta* reckoning; the *pūrṇimānta*
//!    reckoning of the north is [`crate::hindu_purnimanta`].
//! 2. **A month is named for the sidereal sign the Sun enters during it**:
//!    the lunar month holding the Meṣa saṅkrānti is Chaitra, and so on
//!    round the twelve.
//! 3. **A month with no saṅkrānti is intercalary** — *adhika māsa* — and
//!    takes the name of the month that follows it, written
//!    [`Month::leap(n)`](hc_calendar::Month::leap).
//! 4. **A month with two saṅkrāntis loses a name** — *kṣaya māsa* — and
//!    keeps the first; the name it loses is reported as not existing. It
//!    is rare: it can only happen near perihelion, when the Sun's stay in a
//!    sign is shortest, and in Sewell and Dikshit's tables of 300 to
//!    1900 CE the expunged months come 19 to 141 years apart
//!    (`sewell1896`, Art. 50). Neither year the tests check has one.
//! 5. **The year is the Śaka era**, counted from Chaitra śukla 1; a
//!    Gregorian year *g* holds the turn of Śaka *g* − 78. The Vikrama year,
//!    135 greater, is carried as an extra field, and so is the year's name
//!    in the southern sixty-year cycle, `samvatsara`: the year that opens
//!    at Chaitra śukla 1 is the Telugu and Kannada Ugādi year, which south
//!    of the Narmada takes the same name as the Tamil solar year that
//!    begins in it (`sewell1896`, Art. 62; [`crate::samvatsara`]).
//!
//! # Whose sunrise
//!
//! Rule 1 needs a place. [`HinduLunarCalendar::RASHTRIYA`], the registered
//! `hindu-lunar`, reads the day at the Central Station's sunrise as the
//! almanac does; [`HinduLunarCalendar::UJJAIN`] at Ujjain, as the classical
//! almanacs do. Both are [`crate::places`] constants, and a caller with a
//! city and a local panchang can build a third with
//! [`HinduLunarCalendar::new`]. Ujjain with the Lahiri ayanāṃśa is not
//! registered: the place is its only difference, a parameter, and it moves
//! 302 of the 11 323 dates of 2000–2030. Reingold and Dershowitz's
//! astronomical Hindu lunisolar calendar, `astro-hindu-lunar-from-fixed`
//! of *Calendrical Calculations* (`reingold2018code`), is the same rules
//! at Ujjain with the book's own ayanāṃśa, zero at the *Sūrya Siddhānta*'s
//! Meṣa saṅkrānti of 285 CE (`Ayanamsa::REINGOLD_DERSHOWITZ`), which is a
//! convention of the book's and so a registered calendar of its own,
//! [`HinduLunarCalendar::REINGOLD_DERSHOWITZ`], `hindu-lunar-reingold-dershowitz`
//! (docs/policy.md §5); the ayanāṃśa moves none of those dates, so it and
//! `UJJAIN` agree on every day of 2000–2030. The same months on the *Sūrya
//! Siddhānta*'s Sun, Moon and sunrise are another convention, and a
//! registered calendar of their own, [`crate::hindu_lunar_siddhanta`];
//! the months' engine the calendars share is the crate's `amanta` module.
//!
//! # What is exact and what is not
//!
//! The astronomy is `hc-astro`'s — the Sun to about 1″, the Moon to about
//! 10″, sunrise to a minute or two — and the ayanāṃśa is `hc-seasons`'s
//! Lahiri anchor. A tithi that ends within a minute or two of sunrise, or a
//! saṅkrānti that falls within seconds of a conjunction, is therefore a
//! decision this calendar makes by a model where a panchang makes it by
//! its own; the *Rashtriya Panchang* itself is the reference, and the tests
//! compare against it. Festival *observance* — a feast kept on the day the
//! tithi holds at midday or in the evening rather than at sunrise — belongs
//! to a holiday rule, not to the date.

use hc_astro::lunar::moon_phase_at_or_after;
use hc_astro::riseset::Location;
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_solar::gregorian;
use hc_seasons::zodiac::sidereal::{ingress_after, sign_at_moment};
use hc_seasons::zodiac::{Ayanamsa, SiderealSign};

use crate::amanta::Sky;
use crate::places::{CENTRAL_STATION, UJJAIN};
use crate::tithi::{sunrise_of, tithi_of_day};

/// The identifier of the amānta Hindu lunisolar calendar.
pub const ID: CalendarId = CalendarId("hindu-lunar");

/// The English name of the amānta Hindu lunisolar calendar.
pub const ENGLISH_NAME: &str = "Hindu lunisolar (amanta)";

crate::ayanamsa_id::by_ayanamsa! {
    /// The identifier and English name of the amānta calendar over an
    /// ayanāṃśa: [`ID`] and [`ENGLISH_NAME`] for Lahiri's, and the
    /// convention's for any other.
    pub(crate) fn identity, "hindu-lunar", "Hindu lunisolar (amanta)"
}

/// The identifier of Reingold and Dershowitz's astronomical Hindu
/// lunisolar calendar, [`HinduLunarCalendar::REINGOLD_DERSHOWITZ`].
pub const REINGOLD_DERSHOWITZ_ID: CalendarId = CalendarId("hindu-lunar-reingold-dershowitz");

/// The era code of the Śaka era.
pub const ERA: &str = "saka";

/// The number of months in a common year.
pub const MONTHS_IN_YEAR: u8 = 12;

/// Where the period of use of the *Rashtriya Panchang*'s calendars comes
/// from.
pub const USAGE_SOURCE: &str = "The calendars of India's festivals and regional years, older than any source read dates; \
    the reckoning carried is the *Rashtriya Panchang*'s, published from Śaka 1879 (1957–58) \
    [pac-rashtriya-panchang], as docs/systems/hindu-calendars.md states";

/// The Gregorian year in which Śaka year zero would begin, so that Śaka 1
/// begins in 79 CE.
pub const GREGORIAN_YEAR_OFFSET: i64 = 78;

/// The Vikrama (Chaitrādi) year exceeds the Śaka year by this much.
pub const VIKRAMA_OFFSET: i64 = 135;

/// The earliest Śaka year this calendar converts: the one beginning in
/// Gregorian 1700, as far back as the lunar theory is worth asking.
pub const MIN_YEAR: i64 = 1_700 - GREGORIAN_YEAR_OFFSET;

/// The latest Śaka year this calendar converts: the one beginning in
/// Gregorian 2299.
pub const MAX_YEAR: i64 = 2_299 - GREGORIAN_YEAR_OFFSET;

/// The twelve months in Devanagari, Chaitra first: the calendar's own
/// names, which every Indian language writes in its own script and English
/// transliterates.
///
/// Source: the month headings of the *Rashtriya Panchang* (Sanskrit
/// edition) and Wikipedia, "Hindu calendar", retrieved 2026-09-22.
pub const MONTHS: [&str; 12] = [
    "चैत्र",
    "वैशाख",
    "ज्येष्ठ",
    "आषाढ",
    "श्रावण",
    "भाद्रपद",
    "आश्विन",
    "कार्तिक",
    "मार्गशीर्ष",
    "पौष",
    "माघ",
    "फाल्गुन",
];

/// A date in the amānta Hindu lunisolar calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HinduLunarDate {
    /// The Śaka year.
    pub year: i64,
    /// The month, 1 for Chaitra through 12 for Phālguna.
    pub month: u8,
    /// Whether this is the intercalary (*adhika*) month of that name, which
    /// precedes the ordinary one.
    pub leap_month: bool,
    /// The tithi, 1 through 30: 1–15 the bright fortnight, 16–30 the dark.
    pub day: u8,
    /// Whether this is the second day to carry that tithi at sunrise — an
    /// *adhika tithi*.
    pub leap_day: bool,
}

impl HinduLunarDate {
    /// The Vikrama (Chaitrādi) year of this date.
    #[must_use]
    pub const fn vikrama_year(self) -> i64 {
        self.year + VIKRAMA_OFFSET
    }

    /// The fortnight and the day within it.
    #[must_use]
    pub const fn paksha(self) -> (crate::tithi::Paksha, u8) {
        crate::tithi::paksha_of(self.day)
    }
}

/// The amānta Hindu lunisolar calendar, judged at a place with an ayanāṃśa.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HinduLunarCalendar {
    /// The registry identifier: [`ID`] for the calendar at any place with
    /// any ayanāṃśa, and a name of its own for a convention registered
    /// apart, as [`Self::REINGOLD_DERSHOWITZ`] is.
    pub id: CalendarId,
    /// The English name.
    pub english_name: &'static str,
    /// Whose sunrise reads the day.
    pub location: Location,
    /// Which sidereal zero point names the months.
    pub ayanamsa: Ayanamsa,
}

impl Default for HinduLunarCalendar {
    fn default() -> Self {
        Self::RASHTRIYA
    }
}

impl HinduLunarCalendar {
    /// The calendar as the *Rashtriya Panchang* computes it: sunrise at the
    /// Central Station, Lahiri ayanāṃśa. The registered `hindu-lunar`.
    pub const RASHTRIYA: Self = Self::new(CENTRAL_STATION, Ayanamsa::LAHIRI);

    /// The calendar read at Ujjain, as the classical almanacs do, with the
    /// Lahiri ayanāṃśa: `hindu-lunar` at another place, not registered.
    pub const UJJAIN: Self = Self::new(UJJAIN, Ayanamsa::LAHIRI);

    /// Reingold and Dershowitz's astronomical Hindu lunisolar calendar,
    /// `astro-hindu-lunar-from-fixed` of *Calendrical Calculations*
    /// (`reingold2018code`) with their errata's correction 15, Universal
    /// Time, applied: the same rules read at Ujjain's sunrise with the
    /// book's own ayanāṃśa, zero at the *Sūrya Siddhānta*'s Meṣa saṅkrānti
    /// of 285 CE. The registered `hindu-lunar-reingold-dershowitz`. It gives
    /// the book's value on every sample date of the book's `dates.l` but
    /// the one its errata explain, and over 2000–2030 it agrees with
    /// [`Self::UJJAIN`] on every day and parts from [`Self::RASHTRIYA`] on
    /// 302 of 11 323 (docs/systems/hindu-calendars.md).
    pub const REINGOLD_DERSHOWITZ: Self = Self::named(
        REINGOLD_DERSHOWITZ_ID,
        "Hindu lunisolar (amanta, Reingold and Dershowitz)",
        UJJAIN,
        Ayanamsa::REINGOLD_DERSHOWITZ,
    );

    /// A calendar judged at any place with any ayanāṃśa. Its identifier is
    /// the convention's, since a sidereal zero point is one (policy §5):
    /// [`ID`] for the Lahiri ayanāṃśa at any place, as for
    /// [`Self::UJJAIN`]; `hindu-lunar-raman` for Raman's and so on for each
    /// named ayanāṃśa of `Ayanamsa::ALL`; `hindu-lunar-other-ayanamsa` for an
    /// anchor this crate does not name. Only the Lahiri one is registered.
    #[must_use]
    pub const fn new(location: Location, ayanamsa: Ayanamsa) -> Self {
        let (id, english_name) = identity(ayanamsa);
        Self::named(id, english_name, location, ayanamsa)
    }

    /// A calendar judged at any place with any ayanāṃśa, registered under
    /// an identifier and an English name of its own.
    #[must_use]
    pub const fn named(
        id: CalendarId,
        english_name: &'static str,
        location: Location,
        ayanamsa: Ayanamsa,
    ) -> Self {
        Self {
            id,
            english_name,
            location,
            ayanamsa,
        }
    }

    /// The position, 1 for Prabhava through 60 for Kṣaya, of a Śaka year's
    /// name in the southern sixty-year cycle: the name of the Telugu and
    /// Kannada year that opens at Ugādi, Chaitra śukla 1, which is the Tamil
    /// solar year's too ([`crate::samvatsara::southern_of_saka`]).
    #[must_use]
    pub const fn samvatsara_of(year: i64) -> u8 {
        crate::samvatsara::southern_of_saka(year)
    }

    crate::amanta::amanta_methods!();
}

crate::amanta::amanta_months!(HinduLunarCalendar);

/// The first conjunction at or after a moment: the instant the Moon's
/// elongation from the Sun, the quantity the tithi is counted in, returns to
/// zero.
///
/// Not the tabulated new moon of [`hc_astro::lunar::nth_new_moon`], which
/// comes from a different series and can differ from the elongation's zero by
/// minutes. A month has to begin where its first tithi does, or a sunrise in
/// those minutes would read the first tithi of a month the month boundaries
/// have not yet begun.
fn conjunction_at_or_after(moment: Moment) -> Moment {
    moon_phase_at_or_after(0.0, moment)
}

/// The true Sun and Moon of modern astronomy, in the zodiac of the
/// calendar's ayanāṃśa, with the day read at its place's sunrise.
impl Sky for HinduLunarCalendar {
    fn conjunction_at_or_after(&self, moment: Moment) -> Moment {
        conjunction_at_or_after(moment)
    }

    fn sign_at(&self, moment: Moment) -> SiderealSign {
        sign_at_moment(moment, self.ayanamsa)
    }

    fn ingress_after(&self, sign: SiderealSign, moment: Moment) -> Moment {
        ingress_after(sign, self.ayanamsa, moment)
    }

    fn sunrise(&self, day: Rd) -> Moment {
        sunrise_of(day, self.location)
    }

    fn tithi_of_day(&self, day: Rd) -> u8 {
        tithi_of_day(day, self.location)
    }

    /// Chaitra, ordinary or intercalary, begins in March or April and opens
    /// the year; every other month begins between mid-April and mid-March
    /// and belongs to the year that opened before it. So a month is in the
    /// Śaka year of its Gregorian year, less 78, unless it is a month other
    /// than Chaitra beginning in January to March, which is the tail of the
    /// year before. That holds while the Meṣa saṅkrānti of the Lahiri
    /// zodiac falls in April, which it does across the range.
    fn year_of(&self, start: Moment, month: u8) -> i64 {
        let (gregorian_year, gregorian_month, _) =
            gregorian::from_fixed(start.day()).unwrap_or((0, 1, 1));
        if month == 1 || gregorian_month >= 4 {
            gregorian_year - GREGORIAN_YEAR_OFFSET
        } else {
            gregorian_year - GREGORIAN_YEAR_OFFSET - 1
        }
    }

    /// The first of January of the Gregorian year the saṅkrānti falls in:
    /// the Śaka year's own for Chaitra through Mārgaśīrṣa and the next for
    /// Pauṣa through Phālguna.
    fn sankranti_search_start(&self, year: i64, month: u8) -> Moment {
        let gregorian_year = year + GREGORIAN_YEAR_OFFSET + i64::from(month >= 10);
        Moment(hc_calendar::gregorian::new_year(gregorian_year).0 as f64)
    }

    fn years(&self) -> (i64, i64) {
        (MIN_YEAR, MAX_YEAR)
    }

    fn named_range(&self) -> Option<(Rd, Rd)> {
        NAMED_RANGES
            .iter()
            .find(|(calendar, _, _)| calendar == self)
            .map(|&(_, earliest, latest)| (earliest, latest))
    }

    fn zodiac_key(&self) -> [u64; 3] {
        self.ayanamsa.key()
    }

    fn place_key(&self) -> [u64; 3] {
        self.location.key()
    }
}

/// The earliest and latest days of the calendars the crate names, as
/// [`HinduLunarCalendar::earliest`] and [`HinduLunarCalendar::latest`]
/// compute them.
///
/// Each is a search for the conjunctions and the Meṣa saṅkrānti of a year,
/// most of a millisecond for the two, and every calendar built on this one
/// asks for them in its metadata and its range checks, several times for
/// every day a page describes. So they are written down for the
/// calendars that are registered or named here, and computed for any other;
/// `tests::the_named_ranges_are_the_computed_ones` computes these again.
/// Kathmandu's is Nepal Sambat's.
const NAMED_RANGES: [(HinduLunarCalendar, Rd, Rd); 4] = [
    (HinduLunarCalendar::RASHTRIYA, Rd(620_627), Rd(839_773)),
    (HinduLunarCalendar::UJJAIN, Rd(620_627), Rd(839_773)),
    (
        HinduLunarCalendar::REINGOLD_DERSHOWITZ,
        Rd(620_627),
        Rd(839_773),
    ),
    (
        HinduLunarCalendar::new(crate::places::KATHMANDU, Ayanamsa::LAHIRI),
        Rd(620_627),
        Rd(839_773),
    ),
];

impl Calendar for HinduLunarCalendar {
    type Date = HinduLunarDate;

    /// In use today and older than any source read dates, so undated at the
    /// start; the almanac whose reckoning is carried has been published since
    /// 1957.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(USAGE_SOURCE)
    }

    /// Twelve months with a thirteenth in an intercalary year, the
    /// seven-day week, and the sixty year names of the southern cycle.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// A year with an adhika māsa.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(self.leap_month_of(year)?.is_some())
    }

    /// The Hindu day begins at sunrise and is named by the civil day on
    /// whose sunrise it begins: Reingold and Dershowitz read a fixed day's
    /// date at "Sunrise that day" (`reingold2018code`, `hindu-lunar-from-fixed`).
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
            earliest: self.earliest().ok(),
            latest: self.latest().ok(),
            native_locales: &["sa", "hi"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        HinduLunarCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        HinduLunarCalendar::from_fixed(self, rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        fields_of(date)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let date = date_of(fields)?;
        HinduLunarCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

/// The shape of an amānta calendar: twelve months with a thirteenth in an
/// intercalary year, the seven-day week, and the sixty year names of the
/// southern cycle.
pub(crate) const SHAPE: &[hc_calendar::shape::CycleShape] = {
    use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
    &[
        CycleShape::intercalary(MONTH, 12, 13),
        CycleShape::fixed(WEEKDAY, 7),
        CycleShape::named(crate::samvatsara::CYCLE, &crate::samvatsara::NAMES),
    ]
};

/// An amānta date's fields: the Śaka year, the month with its intercalary
/// flag, the tithi and its repetition, and the Vikrama year and the
/// southern year name as extra fields.
pub(crate) fn fields_of(date: HinduLunarDate) -> CalendarResult<DateFields> {
    let month = if date.leap_month {
        Month::leap(date.month)
    } else {
        Month::regular(date.month)
    };
    let mut fields = DateFields::ymd(date.year, date.month, date.day).with_era(ERA);
    fields.month = Some(month);
    fields.leap_day = date.leap_day;
    fields
        .with_extra("vikrama-year", date.vikrama_year())?
        .with_extra(
            crate::samvatsara::CYCLE,
            i64::from(HinduLunarCalendar::samvatsara_of(date.year)),
        )
}

/// The amānta date fields name, not yet checked against a calendar.
///
/// # Errors
///
/// [`CalendarError::UnknownEra`] for an era other than the Śaka, and the
/// errors of a missing month or day.
pub(crate) fn date_of(fields: &DateFields) -> CalendarResult<HinduLunarDate> {
    if fields.era.is_some_and(|era| era != ERA) {
        return Err(CalendarError::UnknownEra);
    }
    let month = fields.require_month()?;
    Ok(HinduLunarDate {
        year: fields.year,
        month: month.ordinal,
        leap_month: month.leap,
        day: fields.require_day()?,
        leap_day: fields.leap_day,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_named_ranges_are_the_computed_ones() {
        for (calendar, earliest, latest) in NAMED_RANGES {
            let engine = crate::amanta::Amanta(calendar);
            assert_eq!(engine.computed_earliest(), Ok(earliest), "{calendar:?}");
            assert_eq!(engine.computed_latest(), Ok(latest), "{calendar:?}");
        }
    }

    #[test]
    fn a_month_begins_where_its_first_tithi_does() {
        // 1 October 2016 at Kathmandu (27.71° N, 85.32° E): the conjunction
        // fell within minutes of sunrise, and the tabulated new moon and the
        // elongation's zero fell on either side of it. Read at sunrise the
        // day carries the first tithi, so it is the first day of Āśvina.
        let kathmandu = HinduLunarCalendar::new(
            hc_astro::riseset::Location::new(27.71, 85.32, 0.0),
            Ayanamsa::LAHIRI,
        );
        let day = |m, d| gregorian::to_fixed(2016, m, d).expect("a date");
        let first = kathmandu.from_fixed(day(10, 1)).expect("in range");
        assert_eq!((first.month, first.day), (7, 1), "{first:?}");
        for d in 25..=30 {
            let date = kathmandu.from_fixed(day(9, d)).expect("in range");
            assert_eq!(kathmandu.to_fixed(date), Ok(day(9, d)), "{date:?}");
        }
        for d in 1..=5 {
            let date = kathmandu.from_fixed(day(10, d)).expect("in range");
            assert_eq!(kathmandu.to_fixed(date), Ok(day(10, d)), "{date:?}");
        }
    }

    #[test]
    fn a_new_moon_after_the_next_local_sunrise_starts_the_month_a_day_later() {
        // 11 June 2002: the new moon at 05:17 IST, after that day's sunrise
        // at the Central Station, so the sunrise of the 11th still carries
        // the thirtieth tithi and Jyeṣṭha begins on the 12th.
        let rashtriya = HinduLunarCalendar::RASHTRIYA;
        let day = |d| gregorian::to_fixed(2002, 6, d).expect("a date");
        assert_eq!(
            rashtriya.month_span(1924, 2, false),
            Ok((gregorian::to_fixed(2002, 5, 13).expect("a date"), day(12)))
        );
        let eleventh = rashtriya.from_fixed(day(11)).expect("in range");
        assert_eq!((eleventh.month, eleventh.day), (2, 30));
        assert_eq!(rashtriya.from_fixed(day(12)).map(|date| date.month), Ok(3));
        for d in 8..=16 {
            let date = rashtriya.from_fixed(day(d)).expect("in range");
            assert_eq!(rashtriya.to_fixed(date), Ok(day(d)), "{date:?}");
        }
    }

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    const RASHTRIYA: HinduLunarCalendar = HinduLunarCalendar::RASHTRIYA;

    /// The first day of each fortnight of Śaka 1945 and 1946, from the
    /// "Regional Calendars" table of the *Rashtriya Panchang* for those
    /// years (Positional Astronomy Centre, India Meteorological Department;
    /// English editions, pages x–xiii): month, intercalary, then the
    /// Gregorian dates of śukla 1 and kṛṣṇa 1 (tithi 16).
    /// A month's Śaka year, number and intercalary flag, then the Gregorian
    /// dates its two fortnights begin on.
    type Fortnight = (i64, u8, bool, (i64, u8, u8), (i64, u8, u8));

    const FORTNIGHTS: &[Fortnight] = &[
        (1945, 1, false, (2023, 3, 22), (2023, 4, 7)),
        (1945, 2, false, (2023, 4, 21), (2023, 5, 6)),
        (1945, 3, false, (2023, 5, 20), (2023, 6, 5)),
        (1945, 4, false, (2023, 6, 19), (2023, 7, 4)),
        (1945, 5, true, (2023, 7, 18), (2023, 8, 2)),
        (1945, 5, false, (2023, 8, 17), (2023, 8, 31)),
        (1945, 6, false, (2023, 9, 16), (2023, 9, 30)),
        (1945, 7, false, (2023, 10, 15), (2023, 10, 29)),
        (1945, 8, false, (2023, 11, 14), (2023, 11, 28)),
        (1945, 9, false, (2023, 12, 13), (2023, 12, 27)),
        (1945, 10, false, (2024, 1, 12), (2024, 1, 26)),
        (1945, 11, false, (2024, 2, 10), (2024, 2, 25)),
        (1945, 12, false, (2024, 3, 11), (2024, 3, 26)),
        (1946, 1, false, (2024, 4, 9), (2024, 4, 24)),
        (1946, 2, false, (2024, 5, 9), (2024, 5, 24)),
        (1946, 3, false, (2024, 6, 7), (2024, 6, 22)),
        (1946, 4, false, (2024, 7, 6), (2024, 7, 22)),
        (1946, 5, false, (2024, 8, 5), (2024, 8, 20)),
        (1946, 6, false, (2024, 9, 4), (2024, 9, 18)),
        (1946, 7, false, (2024, 10, 3), (2024, 10, 18)),
        (1946, 8, false, (2024, 11, 2), (2024, 11, 16)),
        (1946, 9, false, (2024, 12, 2), (2024, 12, 16)),
        (1946, 10, false, (2024, 12, 31), (2025, 1, 14)),
        (1946, 11, false, (2025, 1, 30), (2025, 2, 13)),
        (1946, 12, false, (2025, 2, 28), (2025, 3, 15)),
    ];

    #[test]
    fn every_fortnight_of_two_years_begins_where_the_rashtriya_panchang_says() {
        // The table's date for a fortnight is the day its first tithi holds
        // at sunrise — or, when that tithi holds no sunrise at all (a kṣaya
        // tithi), the day it begins and ends within. Three of the
        // fifty-two are the second kind, and the test says which.
        let mut mismatches = alloc::vec::Vec::new();
        let mut skipped = alloc::vec::Vec::new();
        for &(year, month, leap, sukla, krishna) in FORTNIGHTS {
            for (day, (y, m, d)) in [(1, sukla), (16, krishna)] {
                let date = HinduLunarDate {
                    year,
                    month,
                    leap_month: leap,
                    day,
                    leap_day: false,
                };
                let expected = ymd(y, m, d);
                let forward = RASHTRIYA.to_fixed(date);
                if forward == Err(CalendarError::DayOutOfRange) {
                    // The tithi is skipped: it begins after that day's
                    // sunrise and ends before the next.
                    let at_sunrise = tithi_of_day(expected, CENTRAL_STATION);
                    let next = tithi_of_day(Rd(expected.0 + 1), CENTRAL_STATION);
                    if at_sunrise == day - 1 && next == day + 1 {
                        skipped.push((y, m, d));
                        continue;
                    }
                }
                let back = RASHTRIYA.from_fixed(expected);
                if forward != Ok(expected) || back != Ok(date) {
                    mismatches.push(alloc::format!(
                        "{year} month {month} leap {leap} tithi {day}: expected {y}-{m:02}-{d:02}, got {forward:?}; that day reads {back:?}"
                    ));
                }
            }
        }
        assert!(mismatches.is_empty(), "{mismatches:#?}");
        assert_eq!(
            skipped,
            [(2023, 8, 31), (2024, 6, 22), (2024, 9, 18)],
            "the fortnights that begin with a skipped tithi changed"
        );
    }

    #[test]
    fn the_year_opens_at_chaitra_sukla_pratipada() {
        // Chaitra Sukladi — Gudi Padava, Ugadi — in the panchang's festival
        // list: 22 March 2023, 9 April 2024, 30 March 2025.
        assert_eq!(RASHTRIYA.new_year(1945), Ok(ymd(2023, 3, 22)));
        assert_eq!(RASHTRIYA.new_year(1946), Ok(ymd(2024, 4, 9)));
        assert_eq!(RASHTRIYA.new_year(1947), Ok(ymd(2025, 3, 30)));
        assert_eq!(RASHTRIYA.days_in_year(1945), Ok(384));
        assert_eq!(RASHTRIYA.days_in_year(1946), Ok(355));
    }

    #[test]
    fn a_month_span_runs_from_sukla_1_to_the_next_sukla_1() {
        // Ordinary Śrāvaṇa 1945: 17 August to 15 September 2023, the day
        // before Bhādrapada śukla 1 on the 16th.
        assert_eq!(
            RASHTRIYA.month_span(1945, 5, false),
            Ok((ymd(2023, 8, 17), ymd(2023, 9, 16)))
        );
        assert_eq!(
            RASHTRIYA.month_span(1945, 5, true),
            Ok((ymd(2023, 7, 18), ymd(2023, 8, 17)))
        );
    }

    #[test]
    fn saka_1945_has_the_intercalary_sravana_and_1946_none() {
        // "Sravana (Mala)" from 18 July 2023, "Sravana (Suddha)" from
        // 17 August 2023: the intercalary month runs to the 16th.
        assert_eq!(
            RASHTRIYA.leap_month_of(1945),
            Ok(Some((5, ymd(2023, 7, 18), ymd(2023, 8, 16))))
        );
        assert_eq!(RASHTRIYA.leap_month_of(1946), Ok(None));
        assert_eq!(RASHTRIYA.has_kshaya_month(1945), Ok(false));
        assert_eq!(RASHTRIYA.has_kshaya_month(1946), Ok(false));
        // Asking for the intercalary month in a year without one is an error,
        // not a guess.
        let date = HinduLunarDate {
            year: 1946,
            month: 5,
            leap_month: true,
            day: 1,
            leap_day: false,
        };
        assert_eq!(
            RASHTRIYA.to_fixed(date),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn festivals_the_panchang_dates_by_the_sunrise_tithi_fall_on_their_days() {
        // From the "Principal Festivals and Anniversaries" lists of the 1945
        // and 1946 editions: the festival, and the tithi it is kept on.
        for (year, month, day, tithi, name) in [
            (2023, 3, 30, (1945, 1, 9), "Rama Navami"),
            (2023, 9, 19, (1945, 6, 4), "Ganesha Chaturthi"),
            (2023, 10, 24, (1945, 7, 10), "Vijaya Dasami"),
            (2024, 3, 25, (1945, 12, 15), "Holi"),
            (2024, 4, 17, (1946, 1, 9), "Rama Navami"),
            (2024, 9, 7, (1946, 6, 4), "Ganesha Chaturthi"),
            (2025, 3, 14, (1946, 12, 15), "Holi"),
        ] {
            let (saka, m, t) = tithi;
            let date = RASHTRIYA.from_fixed(ymd(year, month, day)).unwrap();
            assert_eq!(
                (date.year, date.month, date.leap_month, date.day),
                (saka, m, false, t),
                "{name} {year}-{month:02}-{day:02}: {date:?}"
            );
        }
    }

    #[test]
    fn a_festival_kept_on_an_afternoon_tithi_is_a_holiday_rule_not_a_date() {
        // The panchang lists Raksha Bandhan 2023 on 30 August, the day the
        // full-moon tithi held the afternoon; at sunrise that day the tithi
        // was still the fourteenth, and the fifteenth held the sunrise of
        // the 31st. The date is right; the observance is another rule's.
        let eve = RASHTRIYA.from_fixed(ymd(2023, 8, 30)).unwrap();
        assert_eq!((eve.month, eve.leap_month, eve.day), (5, false, 14));
        let purnima = RASHTRIYA.from_fixed(ymd(2023, 8, 31)).unwrap();
        assert_eq!(
            (purnima.month, purnima.leap_month, purnima.day),
            (5, false, 15)
        );
        // Likewise Vijaya Daśamī 2024, kept on 12 October when the tenth
        // tithi held the afternoon; at sunrise it was still the ninth.
        let navami = RASHTRIYA.from_fixed(ymd(2024, 10, 12)).unwrap();
        assert_eq!((navami.month, navami.leap_month, navami.day), (7, false, 9));
    }

    #[test]
    fn a_sample_of_days_round_trips_including_repeated_tithis() {
        let start = ymd(2023, 3, 22).0;
        let end = ymd(2025, 3, 30).0;
        let mut repeated = 0;
        for rd in start..end {
            let date = RASHTRIYA.from_fixed(Rd(rd)).unwrap();
            assert_eq!(RASHTRIYA.to_fixed(date), Ok(Rd(rd)), "rd {rd}: {date:?}");
            if date.leap_day {
                repeated += 1;
            }
        }
        // A tithi holds two sunrises a dozen or so times a year.
        assert!((10..=40).contains(&repeated), "{repeated} repeated tithis");
    }

    #[test]
    fn every_day_of_the_range_round_trips() {
        // The amānta engine the Siddhānta calendar and the pūrṇimānta
        // renaming share, on the true sky. Every day of the 600 years in a
        // release build, spread over the machine's threads: about 0.9 ms a
        // day, three minutes of one core. A debug build, which the coverage
        // job runs instrumented, takes every 211th day and every Chaitra
        // śukla 1 with the day before it (docs/policy.md §7); a build
        // instrumented for coverage takes a third as many of the days and the
        // opening of every seventh year.
        let (first, last) = (
            RASHTRIYA.earliest().unwrap().0,
            RASHTRIYA.latest().unwrap().0,
        );
        let openings: alloc::vec::Vec<i64> = if cfg!(debug_assertions) {
            (MIN_YEAR..=MAX_YEAR)
                .step_by(hc_core::sweep::year_step())
                .map(|year| RASHTRIYA.new_year(year).unwrap().0)
                .chain([last + 1])
                .collect()
        } else {
            alloc::vec::Vec::new()
        };
        let days = crate::sweep_days(first, last, 211, &openings);
        assert!(days.contains(&first) && days.contains(&last));
        crate::check_days(&days, |rd| {
            let date = RASHTRIYA.from_fixed(Rd(rd)).unwrap();
            assert_eq!(RASHTRIYA.to_fixed(date), Ok(Rd(rd)), "rd {rd}: {date:?}");
        });
    }

    #[test]
    fn the_books_calendar_round_trips_over_its_whole_range() {
        // `hindu-lunar-reingold-dershowitz` reads Ujjain's sunrise with the
        // book's ayanāṃśa: its own computation, not `hindu-lunar` renamed,
        // so the round trip of the true engine above does not stand for it.
        // Every eleventh day of the 600 years, and each Chaitra śukla 1 with
        // the day before it; every fifty-fifth in a debug build
        // (docs/policy.md §7).
        let book = HinduLunarCalendar::REINGOLD_DERSHOWITZ;
        let (first, last) = (book.earliest().unwrap().0, book.latest().unwrap().0);
        let openings: alloc::vec::Vec<i64> = (MIN_YEAR + 1..=MAX_YEAR)
            .map(|year| book.new_year(year).unwrap().0)
            .collect();
        let days = crate::strided_days(first, last, 11, 5, &openings);
        crate::check_days(&days, |rd| {
            let date = book.from_fixed(Rd(rd)).unwrap();
            assert_eq!(book.to_fixed(date), Ok(Rd(rd)), "rd {rd}: {date:?}");
        });
        assert_eq!(
            book.from_fixed(Rd(first - 1)),
            Err(CalendarError::BeforeEpoch)
        );
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        for rd in (ymd(2023, 3, 22).0..ymd(2024, 4, 9).0).step_by(7) {
            let date = RASHTRIYA.from_fixed(Rd(rd)).unwrap();
            let fields = Calendar::to_fields(&RASHTRIYA, date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(
                Calendar::from_fields(&RASHTRIYA, &fields),
                Ok(date),
                "rd {rd}"
            );
        }
        let date = RASHTRIYA.from_fixed(ymd(2023, 7, 18)).unwrap();
        let fields = Calendar::to_fields(&RASHTRIYA, date).unwrap();
        assert!(
            fields
                .month
                .is_some_and(|month| month.leap && month.ordinal == 5)
        );
        assert_eq!(date.vikrama_year(), 2080);
    }

    #[test]
    fn the_ayanamsa_the_panchang_prints_is_the_one_used() {
        // "Ayanamsa on 1st Chaitra: 24° 11′ 39″" for 1946 (22 March 2024) and
        // "24° 12′ 35″" for 1947 (22 March 2025).
        for (year, expected) in [
            (2024, 24.0 + 11.0 / 60.0 + 39.0 / 3_600.0),
            (2025, 24.0 + 12.0 / 60.0 + 35.0 / 3_600.0),
        ] {
            let at = sunrise_of(ymd(year, 3, 22), CENTRAL_STATION);
            let degrees = Ayanamsa::LAHIRI.degrees_at(at);
            assert!(
                (degrees - expected).abs() * 3_600.0 < 10.0,
                "{year}: {degrees}° against {expected}°"
            );
        }
    }

    #[test]
    fn the_range_is_stated_and_refused_outside() {
        assert_eq!(
            RASHTRIYA.new_year(MIN_YEAR - 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            RASHTRIYA.new_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
        let date = HinduLunarDate {
            year: 1945,
            month: 13,
            leap_month: false,
            day: 1,
            leap_day: false,
        };
        assert_eq!(
            RASHTRIYA.to_fixed(date),
            Err(CalendarError::MonthOutOfRange)
        );
        let date = HinduLunarDate {
            month: 1,
            day: 31,
            ..date
        };
        assert_eq!(RASHTRIYA.to_fixed(date), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn the_ugadi_years_carry_their_printed_names() {
        // Prokerala's Telugu calendar: "Chaitra Masam 2024 Telugu Calendar |
        // Krodhi Nama Samvatsaram", Shalivahana Śaka 1946, from 9 April;
        // Phalguna of the same year, to 29 March 2025, still Krodhi;
        // Chaitra 2025, Viswavasu, 1947, from 30 March; Chaitra 2026,
        // Parabhava, 1948, from 20 March, which is the day this calendar's
        // year 1948 opens too (`prokerala-telugu-calendar`, retrieved
        // 2026-09-26). The names are Sewell and Dikshit's southern cycle on
        // the Śaka year (Art. 62), the Tamil year's.
        let named = |rd: Rd| {
            let date = RASHTRIYA.from_fixed(rd).unwrap();
            let fields = Calendar::to_fields(&RASHTRIYA, date).unwrap();
            let position = fields.extra.get(crate::samvatsara::CYCLE).unwrap();
            crate::samvatsara::name(u8::try_from(position).unwrap())
        };
        assert_eq!(named(ymd(2024, 4, 8)), Some("Sobhana"));
        assert_eq!(named(ymd(2024, 4, 9)), Some("Krodhin"));
        assert_eq!(named(ymd(2025, 3, 29)), Some("Krodhin"));
        assert_eq!(named(ymd(2025, 3, 30)), Some("Visvavasu"));
        assert_eq!(RASHTRIYA.new_year(1_948), Ok(ymd(2026, 3, 20)));
        assert_eq!(named(ymd(2026, 3, 19)), Some("Visvavasu"));
        assert_eq!(named(ymd(2026, 3, 20)), Some("Parabhava"));
        assert_eq!(HinduLunarCalendar::samvatsara_of(1_946), 38);
        assert!(Calendar::cycles(&RASHTRIYA).iter().any(|cycle| {
            cycle.kind == crate::samvatsara::CYCLE && cycle.name(38) == Some("Visvavasu")
        }));
    }

    #[test]
    fn ujjain_and_the_central_station_part_on_one_day_in_forty() {
        // The book's astronomical lunisolar calendar is this one read at
        // Ujjain; `hindu-lunar` is read at the Central Station, 6.7° east,
        // whose sunrise comes about 27 minutes earlier. A tithi that ends
        // between the two sunrises gives the two places different dates:
        // 10 of the 366 days of 2024, and in a release build 302 of the
        // 11 323 days of 2000–2030, 2.7%. That is the whole difference the
        // place makes (docs/systems/hindu-calendars.md), which is why
        // `UJJAIN`, the Lahiri ayanāṃśa at Ujjain, is a constant and not a
        // registered calendar: a place is a parameter. The book's own
        // ayanāṃśa is a convention, and `REINGOLD_DERSHOWITZ` is
        // registered; it stands 24.9″ above Lahiri's and moves none of the
        // same days, so the registered calendar parts from `hindu-lunar` on
        // exactly the days the place does.
        let count = |calendar: HinduLunarCalendar, first: Rd, last: Rd| {
            (first.0..=last.0)
                .filter(|&day| {
                    calendar.from_fixed(Rd(day))
                        != HinduLunarCalendar::RASHTRIYA.from_fixed(Rd(day))
                })
                .count()
        };
        let same = |first: Rd, last: Rd| {
            (first.0..=last.0).all(|day| {
                HinduLunarCalendar::REINGOLD_DERSHOWITZ.from_fixed(Rd(day))
                    == HinduLunarCalendar::UJJAIN.from_fixed(Rd(day))
            })
        };
        assert_eq!(
            count(
                HinduLunarCalendar::UJJAIN,
                ymd(2024, 1, 1),
                ymd(2024, 12, 31)
            ),
            10
        );
        assert_eq!(
            count(
                HinduLunarCalendar::REINGOLD_DERSHOWITZ,
                ymd(2024, 1, 1),
                ymd(2024, 12, 31)
            ),
            10
        );
        assert!(same(ymd(2024, 1, 1), ymd(2024, 12, 31)));
        if !cfg!(debug_assertions) {
            assert_eq!(
                count(
                    HinduLunarCalendar::UJJAIN,
                    ymd(2000, 1, 1),
                    ymd(2030, 12, 31)
                ),
                302
            );
            assert!(same(ymd(2000, 1, 1), ymd(2030, 12, 31)));
        }
    }

    #[test]
    fn the_books_calendar_is_registered_under_its_own_name_and_the_place_is_not() {
        // Policy §5: the book's ayanāṃśa is a convention and gets a name;
        // Ujjain with Lahiri's is a place, a parameter, and keeps
        // `hindu-lunar`'s.
        let book = HinduLunarCalendar::REINGOLD_DERSHOWITZ;
        assert_eq!(book.meta().id, REINGOLD_DERSHOWITZ_ID);
        assert_eq!(book.ayanamsa, Ayanamsa::REINGOLD_DERSHOWITZ);
        assert_eq!(book.location, UJJAIN);
        assert_eq!(HinduLunarCalendar::UJJAIN.meta().id, ID);
        // Raman's is another convention, and another name (§5): the id
        // `hindu-lunar` is the Lahiri calendar's alone.
        assert_eq!(
            HinduLunarCalendar::new(UJJAIN, Ayanamsa::RAMAN).meta().id,
            CalendarId("hindu-lunar-raman")
        );
        assert_eq!(
            HinduLunarCalendar::new(UJJAIN, Ayanamsa::REINGOLD_DERSHOWITZ)
                .meta()
                .id,
            REINGOLD_DERSHOWITZ_ID
        );
        assert_eq!(RASHTRIYA.meta().english_name, ENGLISH_NAME);
        // The book's `dates.l`: RD 764 652, 18 July 2094, is Vikrama 2151,
        // month 4, day 6 by the book's code in standard time and day 5, a
        // leap day, at the sunrise itself (`reingold2018errata`, correction
        // 15; `crates/hyper-calendar/tests/rd_sample_dates.rs` holds every
        // sample date). Śaka 2151 − 135 = 2016.
        let date = book.from_fixed(Rd(764_652)).unwrap();
        assert_eq!(
            (date.year, date.month, date.day, date.leap_day),
            (2_016, 4, 5, true)
        );
    }
}
