//! The Hindu lunisolar calendar, *amānta* — `hindu-lunar`.
//!
//! The system is written up in `docs/systems/hindu-calendars.md` in the
//! repository: the tithi, the naming of months by their saṅkrānti, adhika
//! and kṣaya months, the Śaka and Vikrama years, the Central Station and
//! the ayanamsa, with the adhika Śrāvaṇa of Śaka 1945 worked through by
//! hand, what is carried and what is not, how the calendar was checked
//! against the *Rashtriya Panchang*'s tables, and the sources, keyed in
//! `docs/references.bib`. This page summarises it and states the code's
//! own facts.
//!
//! The calendar most of India dates its festivals in, computed the way the
//! *Rashtriya Panchang* of the Government of India computes it: from the
//! true positions of the Sun and Moon, with the sidereal zodiac fixed by the
//! Lahiri ayanamsa, and the day read at sunrise.
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
//!    keeps the first; the name it loses is reported as not existing.
//! 5. **The year is the Śaka era**, counted from Chaitra śukla 1; a
//!    Gregorian year *g* holds the turn of Śaka *g* − 78. The Vikrama year,
//!    135 greater, is carried as an extra field.
//!
//! # Whose sunrise
//!
//! Rule 1 needs a place. [`HinduLunarCalendar::RASHTRIYA`], the registered
//! calendar, reads the day at the Central Station's sunrise as the almanac
//! does; [`HinduLunarCalendar::UJJAIN`] at Ujjain, as the classical almanacs
//! do. Both are [`crate::places`] constants, and a caller with a city and a
//! local panchang can build a third with [`HinduLunarCalendar::new`].
//!
//! # What is exact and what is not
//!
//! The astronomy is `hc-astro`'s — the Sun to about 1″, the Moon to about
//! 10″, sunrise to a minute or two — and the ayanamsa is `hc-seasons`'s
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

use crate::places::{CENTRAL_STATION, UJJAIN};
use crate::tithi::{TITHIS_PER_MONTH, sunrise_of, tithi_of_day};

/// The identifier of the amānta Hindu lunisolar calendar.
pub const ID: CalendarId = CalendarId("hindu-lunar");

/// The era code of the Śaka era.
pub const ERA: &str = "Saka";

/// The number of months in a common year.
pub const MONTHS_IN_YEAR: u8 = 12;

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

/// One lunar month, as the calendar sees it: the two conjunctions that
/// bound it and the name the saṅkrāntis between them give it.
#[derive(Debug, Clone, Copy, PartialEq)]
struct LunarMonth {
    /// The conjunction the month begins after.
    start: Moment,
    /// The conjunction the month ends with.
    end: Moment,
    /// The month's number, 1 to 12.
    month: u8,
    /// Whether the month is intercalary.
    leap: bool,
    /// Whether the month holds two saṅkrāntis and has lost a name.
    kshaya: bool,
    /// The Śaka year the month belongs to.
    year: i64,
}

/// The amānta Hindu lunisolar calendar, judged at a place with an ayanamsa.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HinduLunarCalendar {
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

/// The last conjunction strictly before a moment, on the same definition.
fn conjunction_before(moment: Moment) -> Moment {
    // A synodic month is under thirty days, so a search from 31 days back
    // finds the conjunction before or the one before that.
    let mut found = conjunction_at_or_after(Moment(moment.0 - 31.0));
    loop {
        let next = conjunction_at_or_after(Moment(found.0 + 1.0));
        if next.0 >= moment.0 {
            return found;
        }
        found = next;
    }
}

impl HinduLunarCalendar {
    /// The calendar as the *Rashtriya Panchang* computes it: sunrise at the
    /// Central Station, Lahiri ayanamsa. The registered `hindu-lunar`.
    pub const RASHTRIYA: Self = Self::new(CENTRAL_STATION, Ayanamsa::LAHIRI);

    /// The calendar read at Ujjain, as the classical almanacs and Reingold
    /// and Dershowitz do, with the Lahiri ayanamsa.
    pub const UJJAIN: Self = Self::new(UJJAIN, Ayanamsa::LAHIRI);

    /// A calendar judged at any place with any ayanamsa.
    #[must_use]
    pub const fn new(location: Location, ayanamsa: Ayanamsa) -> Self {
        Self { location, ayanamsa }
    }

    /// The month number a saṅkrānti into `sign` gives: Meṣa's is Chaitra.
    const fn month_of_sign(sign: SiderealSign) -> u8 {
        sign.index() + 1
    }

    /// The lunar month bounded by the conjunction before `moment` and the
    /// one after, named and placed in its year.
    fn month_containing(&self, moment: Moment) -> LunarMonth {
        let start = conjunction_before(moment);
        self.month_from(start)
    }

    /// The lunar month beginning after the conjunction `start`.
    fn month_from(&self, start: Moment) -> LunarMonth {
        let end = conjunction_at_or_after(Moment(start.0 + 1.0));
        let sign_at_start = sign_at_moment(start, self.ayanamsa);
        let sign_at_end = sign_at_moment(end, self.ayanamsa);
        let crossings =
            (i16::from(sign_at_end.index()) - i16::from(sign_at_start.index())).rem_euclid(12);
        let (month, leap, kshaya) = match crossings {
            0 => (Self::month_of_sign(sign_at_start.next()), true, false),
            1 => (Self::month_of_sign(sign_at_end), false, false),
            _ => (Self::month_of_sign(sign_at_start.next()), false, true),
        };
        let year = self.year_of(start, month);
        LunarMonth {
            start,
            end,
            month,
            leap,
            kshaya,
            year,
        }
    }

    /// The Śaka year of a month that begins after the conjunction `start`.
    ///
    /// Chaitra, ordinary or intercalary, begins in March or April and opens
    /// the year; every other month begins between mid-April and mid-March
    /// and belongs to the year that opened before it. So a month is in the
    /// Śaka year of its Gregorian year, less 78, unless it is a month other
    /// than Chaitra beginning in January to March, which is the tail of the
    /// year before.
    fn year_of(&self, start: Moment, month: u8) -> i64 {
        let (gregorian_year, gregorian_month, _) =
            gregorian::from_fixed(start.day()).unwrap_or((0, 1, 1));
        if month == 1 || gregorian_month >= 4 {
            gregorian_year - GREGORIAN_YEAR_OFFSET
        } else {
            gregorian_year - GREGORIAN_YEAR_OFFSET - 1
        }
    }

    /// The label — month number and intercalary flag — of the month after
    /// the one containing `day`, for the pūrṇimānta renaming.
    pub(crate) fn next_month_label(&self, day: Rd) -> (u8, bool) {
        let current = self.month_containing(sunrise_of(day, self.location));
        let next = self.month_from(current.end);
        (next.month, next.leap)
    }

    /// The first day of a month: the first day whose sunrise follows the
    /// conjunction the month begins after.
    ///
    /// The search starts the day before the conjunction's date in Universal
    /// Time and walks forward. It cannot stop at the day after: east of
    /// Greenwich the local date runs ahead, and a conjunction late in the
    /// Universal day can fall after the next local sunrise too. At the
    /// Central Station on 11 June 2002 the new moon came at 05:17 Indian
    /// Standard Time and the sun had risen at 05:13, so the month began on
    /// the 12th.
    fn first_day_of(&self, month: LunarMonth) -> Rd {
        let mut day = Rd(month.start.day().0 - 1);
        while sunrise_of(day, self.location).0 <= month.start.0 {
            day = Rd(day.0 + 1);
        }
        day
    }

    /// The months of a Śaka year, in order: twelve or thirteen of them.
    ///
    /// The year opens with its first month named Chaitra — the intercalary
    /// one if there is one — and runs to the month before the next.
    fn months_of_year(&self, year: i64) -> MonthsOfYear {
        // The ordinary Chaitra is the month holding the Meṣa saṅkrānti of
        // the Gregorian year the Śaka year begins in; an intercalary Chaitra
        // would be the month before it.
        let gregorian_year = year + GREGORIAN_YEAR_OFFSET;
        let mesha = ingress_after(
            SiderealSign::MESHA,
            self.ayanamsa,
            Moment(hc_astro::time::gregorian_new_year(gregorian_year).0 as f64),
        );
        let chaitra = self.month_containing(mesha);
        let before = self.month_from(conjunction_before(Moment(chaitra.start.0 - 1.0)));
        let first = if before.month == 1 && before.leap {
            before
        } else {
            chaitra
        };
        MonthsOfYear {
            calendar: *self,
            current: Some(first),
            year,
        }
    }

    /// The month of `year` numbered `month`, intercalary or not.
    ///
    /// The ordinary month is the one holding the saṅkrānti that names it —
    /// Meṣa's for Chaitra — which falls in the Gregorian year the Śaka year
    /// begins in for Chaitra through Mārgaśīrṣa and in the next for Pauṣa
    /// through Phālguna; the intercalary month of the same name is the one
    /// before it, when that one has no saṅkrānti. A kṣaya month is the one
    /// case the saṅkrānti does not land in a month of its own name, and it
    /// is reported as the name not existing.
    fn find_month(&self, year: i64, month: u8, leap: bool) -> CalendarResult<LunarMonth> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        if month == 0 || month > MONTHS_IN_YEAR {
            return Err(CalendarError::MonthOutOfRange);
        }
        let Some(sign) = SiderealSign::from_index(month - 1) else {
            return Err(CalendarError::MonthOutOfRange);
        };
        let gregorian_year = year + GREGORIAN_YEAR_OFFSET + i64::from(month >= 10);
        let sankranti = ingress_after(
            sign,
            self.ayanamsa,
            Moment(hc_astro::time::gregorian_new_year(gregorian_year).0 as f64),
        );
        let ordinary = self.month_containing(sankranti);
        if ordinary.month != month || ordinary.year != year {
            return Err(CalendarError::MonthOutOfRange);
        }
        if !leap {
            return Ok(ordinary);
        }
        let before = self.month_from(conjunction_before(Moment(ordinary.start.0 - 1.0)));
        if before.month == month && before.leap && before.year == year {
            Ok(before)
        } else {
            Err(CalendarError::MonthOutOfRange)
        }
    }

    /// The day of a month carrying tithi `day` at sunrise — the second such
    /// day when `leap_day` — or `None` when the tithi holds no sunrise.
    fn day_in_month(&self, month: LunarMonth, day: u8, leap_day: bool) -> Option<Rd> {
        let first = self.first_day_of(month);
        let next_first = self.first_day_of(self.month_from(month.end));
        // Tithis average a little under a day, so tithi `day` falls on or a
        // day or two before the day numbered `day`; start three days early
        // and walk forward until the tithi is passed.
        let mut candidate = Rd(first.0 + i64::from(day) - 4);
        if candidate < first {
            candidate = first;
        }
        let mut found = None;
        while candidate < next_first {
            let tithi = tithi_of_day(candidate, self.location);
            if tithi == day {
                if !leap_day {
                    return Some(candidate);
                }
                if found.is_some() {
                    return Some(candidate);
                }
                found = Some(candidate);
            } else if tithi > day {
                // Passed it: a skipped tithi, or no second day for a
                // repeated one.
                return None;
            }
            candidate = Rd(candidate.0 + 1);
        }
        None
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`]; [`CalendarError::MonthOutOfRange`] for
    /// a month number outside 1–12, for an intercalary month the year does
    /// not have, or for a name a kṣaya month lost; and
    /// [`CalendarError::DayOutOfRange`] for a tithi outside 1–30, a tithi
    /// the month skips, or a repeated tithi the month does not repeat.
    pub fn to_fixed(&self, date: HinduLunarDate) -> CalendarResult<Rd> {
        if date.day == 0 || date.day > TITHIS_PER_MONTH {
            return Err(CalendarError::DayOutOfRange);
        }
        let month = self.find_month(date.year, date.month, date.leap_month)?;
        self.day_in_month(month, date.day, date.leap_day)
            .ok_or(CalendarError::DayOutOfRange)
    }

    /// The date of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside the years this
    /// calendar converts.
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<HinduLunarDate> {
        let sunrise = sunrise_of(rd, self.location);
        let month = self.month_containing(sunrise);
        if month.year < MIN_YEAR {
            return Err(CalendarError::BeforeEpoch);
        }
        if month.year > MAX_YEAR {
            return Err(CalendarError::AfterSupportedRange);
        }
        let day = tithi_of_day(rd, self.location);
        let first = self.first_day_of(month);
        let leap_day = rd > first && tithi_of_day(Rd(rd.0 - 1), self.location) == day;
        Ok(HinduLunarDate {
            year: month.year,
            month: month.month,
            leap_month: month.leap,
            day,
            leap_day,
        })
    }

    /// The days of a month: its first day, and the day after its last.
    ///
    /// # Errors
    ///
    /// As [`HinduLunarCalendar::to_fixed`]: the year out of range, the month
    /// number outside 1–12, an intercalary month the year does not have, or
    /// a name a kṣaya month lost.
    pub fn month_span(&self, year: i64, month: u8, leap: bool) -> CalendarResult<(Rd, Rd)> {
        let found = self.find_month(year, month, leap)?;
        let first = self.first_day_of(found);
        let next = self.first_day_of(self.month_from(found.end));
        Ok((first, next))
    }

    /// The first day of Chaitra — Chaitra śukla pratipadā, the new year —
    /// of a Śaka year. When the year opens with an intercalary Chaitra, this
    /// is that month's first day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`].
    pub fn new_year(&self, year: i64) -> CalendarResult<Rd> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        let first = self
            .months_of_year(year)
            .next()
            .ok_or(CalendarError::YearOutOfRange)?;
        Ok(self.first_day_of(first))
    }

    /// The intercalary month of a Śaka year, if it has one, as its number
    /// and its first and last days.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`].
    pub fn leap_month_of(&self, year: i64) -> CalendarResult<Option<(u8, Rd, Rd)>> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self
            .months_of_year(year)
            .find(|month| month.leap)
            .map(|month| {
                let first = self.first_day_of(month);
                let last = Rd(self.first_day_of(self.month_from(month.end)).0 - 1);
                (month.month, first, last)
            }))
    }

    /// Whether a Śaka year has a kṣaya month — one that lost its name to a
    /// second saṅkrānti.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`].
    pub fn has_kshaya_month(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self.months_of_year(year).any(|month| month.kshaya))
    }

    /// The number of days in a Śaka year.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`].
    pub fn days_in_year(&self, year: i64) -> CalendarResult<u16> {
        let this = self.new_year(year)?;
        let next = self.new_year(year + 1)?;
        Ok((next.0 - this.0) as u16)
    }

    /// The earliest fixed day this calendar converts: Chaitra śukla 1 of
    /// [`MIN_YEAR`].
    ///
    /// # Errors
    ///
    /// Returns an error only if the astronomy cannot place the year, which
    /// it can.
    pub fn earliest(&self) -> CalendarResult<Rd> {
        self.new_year(MIN_YEAR)
    }

    /// The latest fixed day this calendar converts: the day before Chaitra
    /// śukla 1 of the year after [`MAX_YEAR`].
    ///
    /// # Errors
    ///
    /// Returns an error only if the astronomy cannot place the year, which
    /// it can.
    pub fn latest(&self) -> CalendarResult<Rd> {
        let first = self
            .months_of_year(MAX_YEAR + 1)
            .next()
            .ok_or(CalendarError::YearOutOfRange)?;
        Ok(Rd(self.first_day_of(first).0 - 1))
    }
}

/// The months of one Śaka year, in order.
struct MonthsOfYear {
    calendar: HinduLunarCalendar,
    current: Option<LunarMonth>,
    year: i64,
}

impl Iterator for MonthsOfYear {
    type Item = LunarMonth;

    fn next(&mut self) -> Option<LunarMonth> {
        let month = self.current?;
        let following = self.calendar.month_from(month.end);
        // The year ends where the next Chaitra begins.
        self.current = if following.month == 1 {
            None
        } else {
            Some(following)
        };
        if month.year == self.year {
            Some(month)
        } else {
            None
        }
    }
}

impl Calendar for HinduLunarCalendar {
    type Date = HinduLunarDate;

    /// Twelve months with a thirteenth in an intercalary year, and the
    /// seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// The Hindu day begins at sunrise.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Hindu lunisolar (amanta)",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: self.earliest().ok(),
            latest: self.latest().ok(),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        HinduLunarCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        HinduLunarCalendar::from_fixed(self, rd)
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
        fields.with_extra("vikrama-year", date.vikrama_year())
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        let date = HinduLunarDate {
            year: fields.year,
            month: month.ordinal,
            leap_month: month.leap,
            day: fields.require_day()?,
            leap_day: fields.leap_day,
        };
        HinduLunarCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
