//! The Hindu solar calendars: a month is the Sun's stay in a sidereal
//! sign, and the regions differ on which day it begins.
//!
//! The four regional rules, as Sewell and Dikshit state them and as the
//! *Rashtriya Panchang*'s "Regional Calendars" tables show them, the
//! eras, the true and the *Sūrya Siddhānta* Sun and the ayanamsa are
//! written up with their sources in `docs/systems/hindu-calendars.md` in
//! the repository. This page summarises it and states the code's own
//! facts.
//!
//! Every solar reckoning of the subcontinent divides the year at the
//! twelve *saṅkrāntis*, the Sun's entries into the sidereal signs — the
//! same instants for everyone, fixed by the ayanamsa — and then has to say
//! which civil day a month begins on when the saṅkrānti falls in the
//! middle of one. Four answers are in use in India:
//!
//! | Reckoning | The month begins on … | [`SankrantiRule`] |
//! |---|---|---|
//! | Tamil Nadu | the saṅkrānti's day, unless it fell after sunset | [`BeforeSunset`](SankrantiRule::BeforeSunset) |
//! | Kerala | the saṅkrānti's day, unless it fell after three fifths of the daylight (*aparāhṇa*) | [`BeforeAfternoon`](SankrantiRule::BeforeAfternoon) |
//! | Bengal, Assam | the day after the saṅkrānti's | [`DayAfter`](SankrantiRule::DayAfter) |
//! | Punjab, Haryana, Odisha | the sunrise-to-sunrise day the saṅkrānti fell in, named by its date at sunrise | [`SunriseDay`](SankrantiRule::SunriseDay) |
//!
//! Each is a [`HinduSolarCalendar`] value here — [`TAMIL`], [`MALAYALAM`],
//! [`BENGALI`], [`VIKRAMI`] — with the era each counts its years in:
//! the Tiruvaḷḷuvar year, the Kollam era, the Bengali San, the Vikrama
//! Saṃvat. The rules were read off the almanac's tables, twenty-four
//! months of each, and the tests are those tables.
//!
//! # Whose day
//!
//! The saṅkrānti's civil day, its sunrise and its sunset are those of the
//! Central Station of the national calendar, as the almanac computes them;
//! a caller with a local almanac can build the same calendar for another
//! place with [`HinduSolarCalendar::new`].
//!
//! # Whose Sun
//!
//! The saṅkrāntis are those of a [`SolarModel`]: the true Sun of modern
//! astronomy in the sidereal zodiac of an ayanamsa, as the national
//! almanac computes it and all four reckonings above use, or the Sun of
//! the *Sūrya Siddhānta* ([`crate::surya_siddhanta`]), whose saṅkrāntis
//! fall hours away from the modern ones. The Nepali Bikram Sambat is the
//! second with a fifth rule, [`CivilDay`](SankrantiRule::CivilDay), and is
//! [`crate::bikram_sambat`].
//!
//! # What is not here
//!
//! The Odia year counts (*aṅka*) and the sixty-year names of the Tamil year
//! are extras this crate does not yet carry.

use hc_astro::riseset::Location;
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_solar::gregorian;
use hc_seasons::zodiac::rashi::{self, SolarMonthTradition};
use hc_seasons::zodiac::sidereal;
use hc_seasons::zodiac::{Ayanamsa, SiderealSign};

use crate::places::CENTRAL_STATION;
use crate::tithi::{sunrise_of, sunset_of};

/// The number of months in a year.
pub const MONTHS_IN_YEAR: u8 = 12;

/// The earliest Gregorian year whose months this crate computes.
pub const MIN_GREGORIAN_YEAR: i64 = 1_700;

/// The latest Gregorian year whose months this crate computes.
pub const MAX_GREGORIAN_YEAR: i64 = 2_299;

/// Which civil day a month begins on, given where its saṅkrānti fell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SankrantiRule {
    /// The saṅkrānti's civil day when it falls before sunset, the next day
    /// otherwise: Tamil Nadu.
    BeforeSunset,
    /// The saṅkrānti's civil day when it falls before *aparāhṇa* — three
    /// fifths of the way from sunrise to sunset — the next day otherwise:
    /// Kerala.
    BeforeAfternoon,
    /// The civil day after the saṅkrānti's, whatever the hour: Bengal and
    /// Assam.
    DayAfter,
    /// The day, sunrise to sunrise, in which the saṅkrānti falls, named by
    /// its civil date at sunrise — so a saṅkrānti between midnight and
    /// sunrise belongs to the day before: Punjab, Haryana and Odisha.
    SunriseDay,
    /// The civil day, midnight to midnight, in which the saṅkrānti falls,
    /// whatever the hour: the Nepali Bikram Sambat, as the months the
    /// Government of Nepal gazettes show it with the *Sūrya Siddhānta*'s
    /// saṅkrāntis ([`crate::bikram_sambat`]).
    CivilDay,
}

impl SankrantiRule {
    /// The civil day a month begins on for a saṅkrānti at `instant`, in
    /// Universal Time, at a location.
    ///
    /// The civil day is the location's local mean time day, which for the
    /// Central Station is Indian Standard Time exactly.
    #[must_use]
    pub fn month_begins(self, instant: Moment, location: Location) -> Rd {
        let civil = Moment(instant.0 + location.longitude_degrees / 360.0).day();
        match self {
            Self::CivilDay => civil,
            Self::DayAfter => Rd(civil.0 + 1),
            Self::SunriseDay => {
                if instant.0 < sunrise_of(civil, location).0 {
                    Rd(civil.0 - 1)
                } else {
                    civil
                }
            }
            Self::BeforeSunset => {
                if instant.0 < sunset_of(civil, location).0 {
                    civil
                } else {
                    Rd(civil.0 + 1)
                }
            }
            Self::BeforeAfternoon => {
                let rise = sunrise_of(civil, location).0;
                let set = sunset_of(civil, location).0;
                if instant.0 < rise + (set - rise) * 0.6 {
                    civil
                } else {
                    Rd(civil.0 + 1)
                }
            }
        }
    }
}

/// Whose Sun a solar calendar follows: which instants its saṅkrāntis are.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SolarModel {
    /// The true Sun of modern astronomy, in the sidereal zodiac whose zero
    /// point an ayanamsa fixes: the *Rashtriya Panchang*'s, with
    /// [`Ayanamsa::LAHIRI`].
    Modern(Ayanamsa),
    /// The Sun of the *Sūrya Siddhānta*, sidereal by construction
    /// ([`crate::surya_siddhanta`]).
    SuryaSiddhanta,
}

impl SolarModel {
    /// The first moment at or after `moment` at which the Sun enters
    /// `sign`.
    #[must_use]
    pub fn ingress_after(self, sign: SiderealSign, moment: Moment) -> Moment {
        match self {
            Self::Modern(ayanamsa) => sidereal::ingress_after(sign, ayanamsa, moment),
            Self::SuryaSiddhanta => crate::surya_siddhanta::ingress_after(sign, moment),
        }
    }

    /// The sign the Sun stands in at a moment.
    #[must_use]
    pub fn sign_at(self, moment: Moment) -> SiderealSign {
        match self {
            Self::Modern(ayanamsa) => sidereal::sign_at_moment(moment, ayanamsa),
            Self::SuryaSiddhanta => crate::surya_siddhanta::sign_at(moment),
        }
    }
}

/// A date in a Hindu solar calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HinduSolarDate {
    /// The year in the calendar's era.
    pub year: i64,
    /// The month, 1 through 12, counted from the month that opens the
    /// year in this reckoning.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

/// One of the Hindu solar reckonings: a tradition's month names and year
/// opening, a rule for the day a month begins, an era, and a place.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HinduSolarCalendar {
    /// The registry identifier.
    pub id: CalendarId,
    /// The English name.
    pub english_name: &'static str,
    /// The languages the calendar's sources are written in; see
    /// [`CalendarMeta::native_locales`].
    pub native_locales: &'static [&'static str],
    /// The month names and the month that opens the year.
    pub tradition: SolarMonthTradition,
    /// Which civil day a month begins on.
    pub rule: SankrantiRule,
    /// The era code.
    pub era: &'static str,
    /// What to add to the Gregorian year the year opens in to get the year
    /// of the era.
    pub era_offset: i64,
    /// Whose civil day, sunrise and sunset decide the rule.
    pub location: Location,
    /// Whose Sun: which instants the saṅkrāntis are.
    pub model: SolarModel,
}

/// The Tamil solar calendar: Chithirai to Panguni from the Meṣa saṅkrānti,
/// the month beginning on the saṅkrānti's day unless it fell after sunset,
/// years in the Tiruvaḷḷuvar era (Gregorian year plus 31), as the Tamil
/// Nadu government's almanac counts them.
pub const TAMIL: HinduSolarCalendar = HinduSolarCalendar {
    id: CalendarId("hindu-solar-tamil"),
    english_name: "Tamil solar",
    native_locales: &["ta", "sa"],
    tradition: rashi::TAMIL,
    rule: SankrantiRule::BeforeSunset,
    era: "tiruvalluvar",
    era_offset: 31,
    location: CENTRAL_STATION,
    model: SolarModel::Modern(Ayanamsa::LAHIRI),
};

/// The Malayalam calendar of Kerala: Chingam to Karkadakam from the Siṃha
/// saṅkrānti in August, the month beginning on the saṅkrānti's day unless
/// it fell after three fifths of the daylight, years in the Kollam era
/// (Gregorian year of Chingam less 824).
pub const MALAYALAM: HinduSolarCalendar = HinduSolarCalendar {
    id: CalendarId("hindu-solar-malayalam"),
    english_name: "Malayalam (Kollam era)",
    native_locales: &["ml", "sa"],
    tradition: rashi::MALAYALAM,
    rule: SankrantiRule::BeforeAfternoon,
    era: "kollam",
    era_offset: -824,
    location: CENTRAL_STATION,
    model: SolarModel::Modern(Ayanamsa::LAHIRI),
};

/// The Bengali calendar of West Bengal and Assam: Boishakh to Choitro from
/// the Meṣa saṅkrānti, the month beginning the day after the saṅkrānti's,
/// years in the Bengali San (Gregorian year less 593).
pub const BENGALI: HinduSolarCalendar = HinduSolarCalendar {
    id: CalendarId("hindu-solar-bengali"),
    english_name: "Bengali solar (Bangabda)",
    native_locales: &["bn", "sa"],
    tradition: rashi::BENGALI,
    rule: SankrantiRule::DayAfter,
    era: "bangabda",
    era_offset: -593,
    location: CENTRAL_STATION,
    model: SolarModel::Modern(Ayanamsa::LAHIRI),
};

/// The Vikrami solar calendar of Punjab, Haryana and Odisha: Vaiśākha to
/// Chaitra from the Meṣa saṅkrānti, the month
/// beginning on the sunrise-to-sunrise day the saṅkrānti fell in, years in
/// the Vikrama Saṃvat (Gregorian year plus 57). The month names are the
/// Sanskrit ones the almanac's Punjab and Odisha column prints.
pub const VIKRAMI: HinduSolarCalendar = HinduSolarCalendar {
    id: CalendarId("hindu-solar-vikrami"),
    english_name: "Vikrami solar",
    native_locales: &["hi", "sa"],
    tradition: rashi::VIKRAMI,
    rule: SankrantiRule::SunriseDay,
    era: "vs",
    era_offset: 57,
    location: CENTRAL_STATION,
    model: SolarModel::Modern(Ayanamsa::LAHIRI),
};

/// Every solar reckoning this crate registers.
pub const ALL: &[HinduSolarCalendar] = &[TAMIL, MALAYALAM, BENGALI, VIKRAMI];

impl HinduSolarCalendar {
    /// The same reckoning judged at another place, or with another Sun,
    /// under an identifier of the caller's.
    #[must_use]
    pub const fn new(self, id: CalendarId, location: Location, model: SolarModel) -> Self {
        Self {
            id,
            location,
            model,
            ..self
        }
    }

    /// The earliest year of the era this calendar converts.
    #[must_use]
    pub const fn min_year(&self) -> i64 {
        MIN_GREGORIAN_YEAR + self.era_offset
    }

    /// The latest year of the era this calendar converts.
    #[must_use]
    pub const fn max_year(&self) -> i64 {
        MAX_GREGORIAN_YEAR + self.era_offset
    }

    /// The sidereal sign whose saṅkrānti opens `month`.
    fn sign_of_month(&self, month: u8) -> SiderealSign {
        let index = (self.tradition.year_opens_at.index() + month - 1) % MONTHS_IN_YEAR;
        SiderealSign::from_index(index).unwrap_or(SiderealSign::MESHA)
    }

    /// The saṅkrānti that opens `year`: the year-opening sign's entry in the
    /// Gregorian year the era says.
    fn year_opening(&self, year: i64) -> Moment {
        let gregorian_year = year - self.era_offset;
        self.model.ingress_after(
            self.tradition.year_opens_at,
            Moment(hc_astro::time::gregorian_new_year(gregorian_year).0 as f64),
        )
    }

    /// The saṅkrānti that opens `month` of `year`, without validation.
    pub(crate) fn sankranti_of(&self, year: i64, month: u8) -> Moment {
        let opening = self.year_opening(year);
        if month == 1 {
            opening
        } else {
            self.model.ingress_after(self.sign_of_month(month), opening)
        }
    }

    /// The first day of `month` of `year`, without validation; `month` may
    /// be 13, for the first day of the next year.
    pub(crate) fn month_start_raw(&self, year: i64, month: u8) -> Rd {
        if month > MONTHS_IN_YEAR {
            return self.month_start_raw(year + 1, 1);
        }
        self.rule
            .month_begins(self.sankranti_of(year, month), self.location)
    }

    /// The first day of a month.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] or
    /// [`CalendarError::MonthOutOfRange`].
    pub fn month_start(&self, year: i64, month: u8) -> CalendarResult<Rd> {
        if !(self.min_year()..=self.max_year()).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        if month == 0 || month > MONTHS_IN_YEAR {
            return Err(CalendarError::MonthOutOfRange);
        }
        Ok(self.month_start_raw(year, month))
    }

    /// The number of days in a month: 29 to 32, since the Sun's stay in a
    /// sign varies with its distance.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] or
    /// [`CalendarError::MonthOutOfRange`].
    pub fn days_in_month(&self, year: i64, month: u8) -> CalendarResult<u8> {
        let start = self.month_start(year, month)?;
        let next = self.month_start_raw(year, month + 1);
        Ok((next.0 - start.0) as u8)
    }

    /// The number of days in a year, 365 or 366.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`].
    pub fn days_in_year(&self, year: i64) -> CalendarResult<u16> {
        let start = self.month_start(year, 1)?;
        let next = self.month_start_raw(year + 1, 1);
        Ok((next.0 - start.0) as u16)
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`],
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
    pub fn to_fixed(&self, date: HinduSolarDate) -> CalendarResult<Rd> {
        let start = self.month_start(date.year, date.month)?;
        let length = self.days_in_month(date.year, date.month)?;
        if date.day == 0 || date.day > length {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(Rd(start.0 + i64::from(date.day) - 1))
    }

    /// The earliest fixed day this calendar converts.
    #[must_use]
    pub fn earliest(&self) -> Rd {
        self.month_start_raw(self.min_year(), 1)
    }

    /// The latest fixed day this calendar converts.
    #[must_use]
    pub fn latest(&self) -> Rd {
        Rd(self.month_start_raw(self.max_year() + 1, 1).0 - 1)
    }

    /// The date of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside
    /// [`earliest`](Self::earliest)..=[`latest`](Self::latest).
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<HinduSolarDate> {
        if rd < self.earliest() {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd > self.latest() {
            return Err(CalendarError::AfterSupportedRange);
        }
        // The sign the Sun stands in at local noon names the month whose
        // saṅkrānti came last; the rule may have moved that month's first
        // day to after `rd`, in which case the day is still the month
        // before's.
        let noon = Moment(rd.0 as f64 + 0.5 - self.location.longitude_degrees / 360.0);
        let sign = self.model.sign_at(noon);
        // The Sun stays in a sign for at most 32 days, so its entry into the
        // current sign lies within the last 33.
        let entry = self.model.ingress_after(sign, Moment(noon.0 - 33.0));
        // The Sun may enter the next sign later this very day, and every
        // rule but the day-after one may then begin the next month today.
        let next_sign = sign.next();
        let next_entry = self.model.ingress_after(next_sign, Moment(entry.0 + 1.0));
        let next_start = self.rule.month_begins(next_entry, self.location);
        if rd >= next_start {
            let (year, month) = self.year_and_month_of_ingress(next_sign, next_entry);
            return Ok(HinduSolarDate {
                year,
                month,
                day: (rd.0 - next_start.0 + 1) as u8,
            });
        }
        let (year, month) = self.year_and_month_of_ingress(sign, entry);
        let start = self.rule.month_begins(entry, self.location);
        if rd >= start {
            return Ok(HinduSolarDate {
                year,
                month,
                day: (rd.0 - start.0 + 1) as u8,
            });
        }
        // The month before: its saṅkrānti is the entry into the previous
        // sign, within the last 65 days.
        let previous_sign = sign.previous();
        let previous_entry = self
            .model
            .ingress_after(previous_sign, Moment(noon.0 - 65.0));
        let (year, month) = self.year_and_month_of_ingress(previous_sign, previous_entry);
        let start = self.rule.month_begins(previous_entry, self.location);
        Ok(HinduSolarDate {
            year,
            month,
            day: (rd.0 - start.0 + 1) as u8,
        })
    }

    /// The era year and month number of the month whose saṅkrānti is the
    /// entry into `sign` at `entry`.
    fn year_and_month_of_ingress(&self, sign: SiderealSign, entry: Moment) -> (i64, u8) {
        let month = (sign.index() + MONTHS_IN_YEAR - self.tradition.year_opens_at.index())
            % MONTHS_IN_YEAR
            + 1;
        let (gregorian_year, _, _) = gregorian::from_fixed(entry.day()).unwrap_or((0, 1, 1));
        // The year opened at the most recent entry into the opening sign:
        // in this Gregorian year if that entry has already happened, else
        // the year before.
        let mut year = gregorian_year + self.era_offset;
        // The opening entry found from two different starting points can
        // differ by floating-point noise; a year's opening is either this
        // entry or a year away, so half a day of tolerance is safe.
        if entry.0 + 0.5 < self.year_opening(year).0 {
            year -= 1;
        }
        (year, month)
    }
}

impl Calendar for HinduSolarCalendar {
    type Date = HinduSolarDate;

    /// In use today in each tradition's region and older than any source read
    /// dates, so undated at the start; see
    /// [`crate::hindu_lunar::USAGE_SOURCE`].
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(crate::hindu_lunar::USAGE_SOURCE)
    }

    /// Twelve named months, in the tradition's own names, and the seven-day
    /// week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
        // A `const` per tradition, so that the names are the tradition's
        // and the slice is `'static`.
        const TAMIL_SHAPE: &[CycleShape] = &[
            CycleShape::named(MONTH, rashi::TAMIL.months),
            CycleShape::fixed(WEEKDAY, 7),
        ];
        const MALAYALAM_SHAPE: &[CycleShape] = &[
            CycleShape::named(MONTH, MALAYALAM_MONTHS),
            CycleShape::fixed(WEEKDAY, 7),
        ];
        const BENGALI_SHAPE: &[CycleShape] = &[
            CycleShape::named(MONTH, rashi::BENGALI.months),
            CycleShape::fixed(WEEKDAY, 7),
        ];
        const VIKRAMI_SHAPE: &[CycleShape] = &[
            CycleShape::named(MONTH, rashi::VIKRAMI.months),
            CycleShape::fixed(WEEKDAY, 7),
        ];
        match self.tradition.id {
            "malayalam" => MALAYALAM_SHAPE,
            "bengali" => BENGALI_SHAPE,
            "vikrami" => VIKRAMI_SHAPE,
            _ => TAMIL_SHAPE,
        }
    }

    /// Never: a solar year is the Sun's passage through the twelve signs,
    /// 365 or 366 days as the saṅkrāntis fall, with nothing inserted.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(self.min_year()..=self.max_year()).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(false)
    }

    /// The Hindu day begins at sunrise.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id,
            english_name: self.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: true,
            earliest: Some(self.earliest()),
            latest: Some(self.latest()),
            native_locales: self.native_locales,
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        HinduSolarCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        HinduSolarCalendar::from_fixed(self, rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(self.era))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != self.era) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = HinduSolarDate {
            year: fields.year,
            month: month.ordinal,
            day: fields.require_day()?,
        };
        HinduSolarCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

/// The Malayalam months in year order, Chingam first — the tradition's
/// list is in rāśi order, Medam first, and the calendar counts from the
/// month that opens the Kollam year.
const MALAYALAM_MONTHS: &[&str] = &[
    "Chingam",
    "Kanni",
    "Thulam",
    "Vrischikam",
    "Dhanu",
    "Makaram",
    "Kumbham",
    "Meenam",
    "Medam",
    "Edavam",
    "Mithunam",
    "Karkadakam",
];

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    /// The "Regional Calendars" tables of the *Rashtriya Panchang* for
    /// Śaka 1945 and 1946 (2023–2025), Positional Astronomy Centre, India
    /// Meteorological Department: the first day of each solar month in
    /// each reckoning, Meṣa's month first, twelve per year.
    type Year = [(i64, u8, u8); 12];

    const TAMIL_1945: Year = [
        (2023, 4, 14),
        (2023, 5, 15),
        (2023, 6, 15),
        (2023, 7, 17),
        (2023, 8, 17),
        (2023, 9, 17),
        (2023, 10, 18),
        (2023, 11, 17),
        (2023, 12, 16),
        (2024, 1, 15),
        (2024, 2, 13),
        (2024, 3, 14),
    ];
    const TAMIL_1946: Year = [
        (2024, 4, 14),
        (2024, 5, 14),
        (2024, 6, 15),
        (2024, 7, 16),
        (2024, 8, 17),
        (2024, 9, 17),
        (2024, 10, 17),
        (2024, 11, 16),
        (2024, 12, 16),
        (2025, 1, 14),
        (2025, 2, 13),
        (2025, 3, 15),
    ];
    const BENGALI_1945: Year = [
        (2023, 4, 15),
        (2023, 5, 16),
        (2023, 6, 16),
        (2023, 7, 18),
        (2023, 8, 18),
        (2023, 9, 18),
        (2023, 10, 19),
        (2023, 11, 18),
        (2023, 12, 17),
        (2024, 1, 16),
        (2024, 2, 14),
        (2024, 3, 15),
    ];
    const BENGALI_1946: Year = [
        (2024, 4, 14),
        (2024, 5, 15),
        (2024, 6, 16),
        (2024, 7, 17),
        (2024, 8, 17),
        (2024, 9, 17),
        (2024, 10, 18),
        (2024, 11, 17),
        (2024, 12, 16),
        (2025, 1, 15),
        (2025, 2, 13),
        (2025, 3, 15),
    ];
    const VIKRAMI_1945: Year = [
        (2023, 4, 14),
        (2023, 5, 15),
        (2023, 6, 15),
        (2023, 7, 16),
        (2023, 8, 17),
        (2023, 9, 17),
        (2023, 10, 17),
        (2023, 11, 16),
        (2023, 12, 16),
        (2024, 1, 14),
        (2024, 2, 13),
        (2024, 3, 14),
    ];
    const VIKRAMI_1946: Year = [
        (2024, 4, 13),
        (2024, 5, 14),
        (2024, 6, 14),
        (2024, 7, 16),
        (2024, 8, 16),
        (2024, 9, 16),
        (2024, 10, 17),
        (2024, 11, 16),
        (2024, 12, 15),
        (2025, 1, 14),
        (2025, 2, 12),
        (2025, 3, 14),
    ];
    /// Kerala's column, Meṣa first as printed. Its Meṣa row gives the day
    /// of the saṅkrānti itself (14 April 2023, 13 April 2024), which under
    /// the rule the other eleven rows follow is the day *before* the month
    /// — the month begins on 15 April 2023 and 14 April 2024, the days
    /// Kerala keeps Vishu. The test says so.
    const KERALA_1945: Year = [
        (2023, 4, 14),
        (2023, 5, 15),
        (2023, 6, 16),
        (2023, 7, 17),
        (2023, 8, 18),
        (2023, 9, 18),
        (2023, 10, 18),
        (2023, 11, 17),
        (2023, 12, 17),
        (2024, 1, 15),
        (2024, 2, 14),
        (2024, 3, 14),
    ];
    const KERALA_1946: Year = [
        (2024, 4, 13),
        (2024, 5, 15),
        (2024, 6, 15),
        (2024, 7, 16),
        (2024, 8, 17),
        (2024, 9, 17),
        (2024, 10, 17),
        (2024, 11, 16),
        (2024, 12, 16),
        (2025, 1, 14),
        (2025, 2, 13),
        (2025, 3, 15),
    ];

    /// Check a Meṣa-first year of a table against a calendar, returning
    /// the rows that disagree.
    fn mismatches(
        calendar: &HinduSolarCalendar,
        year: i64,
        table: &Year,
    ) -> alloc::vec::Vec<alloc::string::String> {
        let mut out = alloc::vec::Vec::new();
        for (row, (y, m, d)) in table.iter().enumerate() {
            let expected = ymd(*y, *m, *d);
            // Row 0 is Meṣa's month; the calendar's month 1 is its year
            // opener, so the row's month number is offset by the opener.
            let opener = calendar.tradition.year_opens_at.index();
            let month = (row as u8 + MONTHS_IN_YEAR - opener) % MONTHS_IN_YEAR + 1;
            // Rows before the opener belong to the era year that began the
            // Gregorian year before.
            let era_year = year + calendar.era_offset - i64::from((row as u8) < opener);
            let got = calendar.month_start(era_year, month);
            if got != Ok(expected) {
                out.push(alloc::format!(
                    "{} year {era_year} month {month} (row {row}): expected {y}-{m:02}-{d:02}, got {got:?}",
                    calendar.id
                ));
            } else {
                let back = calendar.from_fixed(expected);
                let want = HinduSolarDate {
                    year: era_year,
                    month,
                    day: 1,
                };
                if back != Ok(want) {
                    out.push(alloc::format!(
                        "{} {y}-{m:02}-{d:02} reads {back:?}, not {want:?}",
                        calendar.id
                    ));
                }
            }
        }
        out
    }

    #[test]
    fn the_tamil_months_begin_where_the_rashtriya_panchang_says() {
        let mut bad = mismatches(&TAMIL, 2023, &TAMIL_1945);
        bad.extend(mismatches(&TAMIL, 2024, &TAMIL_1946));
        assert!(bad.is_empty(), "{bad:#?}");
    }

    #[test]
    fn the_bengali_months_begin_where_the_rashtriya_panchang_says() {
        let mut bad = mismatches(&BENGALI, 2023, &BENGALI_1945);
        bad.extend(mismatches(&BENGALI, 2024, &BENGALI_1946));
        assert!(bad.is_empty(), "{bad:#?}");
    }

    #[test]
    fn the_vikrami_months_begin_where_the_rashtriya_panchang_says() {
        let mut bad = mismatches(&VIKRAMI, 2023, &VIKRAMI_1945);
        bad.extend(mismatches(&VIKRAMI, 2024, &VIKRAMI_1946));
        assert!(bad.is_empty(), "{bad:#?}");
    }

    #[test]
    fn the_malayalam_months_begin_where_the_rashtriya_panchang_says_save_medam() {
        let mut bad = mismatches(&MALAYALAM, 2023, &KERALA_1945);
        bad.extend(mismatches(&MALAYALAM, 2024, &KERALA_1946));
        // The two Meṣa rows: the table prints the saṅkrānti's day, the rule
        // begins Medam the day after, on Vishu.
        assert_eq!(bad.len(), 2, "{bad:#?}");
        assert!(
            bad[0].contains("month 9") && bad[0].contains("2023-04-14"),
            "{}",
            bad[0]
        );
        assert!(
            bad[1].contains("month 9") && bad[1].contains("2024-04-13"),
            "{}",
            bad[1]
        );
        assert_eq!(MALAYALAM.month_start(1198, 9), Ok(ymd(2023, 4, 15)));
        assert_eq!(MALAYALAM.month_start(1199, 9), Ok(ymd(2024, 4, 14)));
    }

    #[test]
    fn the_eras_begin_where_the_almanac_says() {
        // Bengali San 1430 from 15 April 2023, Kollam 1199 from 18 August
        // 2023, Vikrama 2080 from 14 April 2023, and the Tamil year of 2023
        // is Tiruvaḷḷuvar 2054.
        assert_eq!(BENGALI.month_start(1430, 1), Ok(ymd(2023, 4, 15)));
        assert_eq!(BENGALI.month_start(1431, 1), Ok(ymd(2024, 4, 14)));
        assert_eq!(MALAYALAM.month_start(1199, 1), Ok(ymd(2023, 8, 18)));
        assert_eq!(MALAYALAM.month_start(1200, 1), Ok(ymd(2024, 8, 17)));
        assert_eq!(VIKRAMI.month_start(2080, 1), Ok(ymd(2023, 4, 14)));
        assert_eq!(VIKRAMI.month_start(2081, 1), Ok(ymd(2024, 4, 13)));
        assert_eq!(TAMIL.month_start(2054, 1), Ok(ymd(2023, 4, 14)));
        assert_eq!(TAMIL.month_start(2055, 1), Ok(ymd(2024, 4, 14)));
    }

    #[test]
    fn months_run_twenty_nine_to_thirty_two_days_and_years_365_or_366() {
        for calendar in ALL {
            for year in [calendar.min_year() + 300, 2023 + calendar.era_offset] {
                let mut total = 0u16;
                for month in 1..=MONTHS_IN_YEAR {
                    let length = calendar.days_in_month(year, month).unwrap();
                    assert!(
                        (29..=32).contains(&length),
                        "{} {year}/{month}: {length}",
                        calendar.id
                    );
                    total += u16::from(length);
                }
                assert_eq!(
                    total,
                    calendar.days_in_year(year).unwrap(),
                    "{} {year}",
                    calendar.id
                );
                assert!(
                    matches!(total, 365 | 366),
                    "{} {year}: {total} days",
                    calendar.id
                );
            }
        }
    }

    #[test]
    fn a_sample_of_days_round_trips_in_every_reckoning() {
        for calendar in ALL {
            let start = ymd(2022, 1, 1).0;
            let end = ymd(2026, 1, 1).0;
            for rd in (start..end).step_by(11) {
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                assert_eq!(
                    calendar.to_fixed(date),
                    Ok(Rd(rd)),
                    "{} rd {rd}: {date:?}",
                    calendar.id
                );
                let fields = Calendar::to_fields(calendar, date).unwrap();
                assert_eq!(Calendar::from_fields(calendar, &fields), Ok(date));
            }
        }
    }

    #[test]
    fn the_range_is_stated_and_refused_outside() {
        for calendar in ALL {
            assert_eq!(
                calendar.month_start(calendar.min_year() - 1, 1),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                calendar.month_start(calendar.max_year() + 1, 1),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                calendar.month_start(calendar.min_year() + 300, 13),
                Err(CalendarError::MonthOutOfRange)
            );
            assert_eq!(
                calendar.from_fixed(Rd(calendar.earliest().0 - 1)),
                Err(CalendarError::BeforeEpoch)
            );
            assert_eq!(
                calendar.from_fixed(Rd(calendar.latest().0 + 1)),
                Err(CalendarError::AfterSupportedRange)
            );
            assert!(calendar.from_fixed(calendar.earliest()).is_ok());
            assert!(calendar.from_fixed(calendar.latest()).is_ok());
        }
    }
}
