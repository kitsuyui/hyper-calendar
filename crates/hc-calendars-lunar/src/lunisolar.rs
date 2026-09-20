//! The East Asian lunisolar engine, parameterised.
//!
//! The Chinese, Korean, Vietnamese and Japanese Tenpō calendars are one
//! calendar with four sets of constants. Rather than four copies of the same
//! two hundred lines, this module holds the rules and
//! [`LunisolarParameters`] holds what differs.
//!
//! # The rules
//!
//! 1. **A month runs from new moon to new moon.** The first day of a month is
//!    the day, *in the calendar's own local time*, containing the
//!    astronomical conjunction. A month is therefore 29 or 30 days, never
//!    anything else, and never by arithmetic alternation.
//! 2. **Month 11 contains the winter solstice.** The stretch from one winter
//!    solstice to the next is a *suì*; it holds 12 or 13 new moons.
//! 3. **A suì with 13 new moons takes a leap month**, and the leap month is
//!    the first one in the suì that contains no *zhōngqì* — no major solar
//!    term, the twelve points where the Sun's apparent longitude is a
//!    multiple of 30°. It takes the number of the month before it and is
//!    written [`Month::leap(n)`](hc_calendar::Month::leap).
//!
//! # What differs between the four
//!
//! * **The meridian.** The day boundary and the solstice's day are read in
//!   local time, so a conjunction just before local midnight in Beijing can
//!   fall on the next day in Seoul. This is not a rounding artefact: it is
//!   the documented reason the Chinese and Korean calendars occasionally
//!   differ by a day, and why Tết 1968 differed between North and South
//!   Vietnam. Each calendar carries its own [`MeridianEra`] history.
//! * **The epoch and the year number.** Each tradition counts years from its
//!   own point, or not at all.
//! * **How a major solar term is defined.** The rule above is *dìngqì*, the
//!   true solar longitude, which China adopted with the Shíxiàn calendar in
//!   1645 and Japan with the Tenpō calendar in 1844. Before that the terms
//!   were *píngqì*, equal divisions of the mean year
//!   ([`SolarTermMode::Mean`]). The parameter exists because the difference
//!   changes which month is the leap month, not as decoration.
//!
//! # Accuracy, and what it refuses to claim
//!
//! The conjunctions come from `hc-astro` and land within about a minute of
//! the truth, which is far inside a day. The solar longitude is the Meeus
//! low-precision series, good to about 0.01°, and its solstice instants run
//! systematically about four and a half minutes early. **When a solstice or a
//! conjunction falls within ten minutes of local midnight, the day this
//! module assigns can be wrong by one**, and a wrong day for the solstice or
//! for a zhōngqì can move a leap month by a whole month.
//!
//! Separately and more importantly: before the twentieth century these
//! calendars were *promulgated*, not computed, by bureaux using their own
//! tables and their own solar theories. A date this module produces for 1700
//! is what the modern rules say, not what the almanac of 1700 said. The
//! supported ranges are set accordingly, and the crate refuses dates outside
//! them rather than guessing.

use hc_astro::solar::Solstice;
use hc_astro::{MEAN_SYNODIC_MONTH, MEAN_TROPICAL_YEAR};
use hc_calendar::cycle::{Sexagenary, sexagenary_day, sexagenary_year};
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_core::math::{floor, round};

use crate::civil;

/// The number of months in the sexagenary month cycle.
const SEXAGENARY_MONTH_ANCHOR: i64 = 2;

/// One segment of a calendar's meridian history.
///
/// The offset is from Universal Time, in hours, exactly as a time-zone table
/// states it; before civil time zones existed it is the local mean solar time
/// of the capital, which is the meridian divided by fifteen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeridianEra {
    /// The first proleptic Gregorian year in which this offset applies.
    ///
    /// The earliest entry of a table should use a year early enough to cover
    /// everything before it.
    pub from_year: i64,
    /// Hours ahead of Universal Time.
    pub offset_hours: f64,
    /// What this offset is, in words, for documentation and error messages.
    pub description: &'static str,
}

impl MeridianEra {
    /// An era whose offset is the local mean time of a meridian.
    #[must_use]
    pub const fn from_longitude(
        from_year: i64,
        longitude_degrees_east: f64,
        description: &'static str,
    ) -> Self {
        Self {
            from_year,
            offset_hours: longitude_degrees_east / 15.0,
            description,
        }
    }

    /// An era whose offset is a civil time zone.
    #[must_use]
    pub const fn from_zone(from_year: i64, offset_hours: f64, description: &'static str) -> Self {
        Self {
            from_year,
            offset_hours,
            description,
        }
    }

    /// The meridian this offset corresponds to, in degrees east.
    #[must_use]
    pub fn longitude_degrees_east(&self) -> f64 {
        self.offset_hours * 15.0
    }
}

/// How the twelve major solar terms are placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SolarTermMode {
    /// *Dìngqì*: a term falls when the Sun's **apparent** longitude reaches a
    /// multiple of 30°, so the intervals between terms are unequal.
    ///
    /// In use in China from the Shíxiàn calendar of 1645 and in Japan from
    /// the Tenpō calendar of 1844.
    #[default]
    Apparent,
    /// *Píngqì*: the year is divided into twelve equal stretches from the
    /// winter solstice, so the terms are evenly spaced in time.
    ///
    /// The older rule. It is offered because it is the rule that was actually
    /// in force before the reforms above, not because this crate claims to
    /// reproduce any particular pre-reform almanac.
    Mean,
}

/// Everything that distinguishes one lunisolar calendar from another.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LunisolarParameters {
    /// The machine identifier.
    pub id: CalendarId,
    /// The English name.
    pub english_name: &'static str,
    /// The meridian history, ascending by [`MeridianEra::from_year`].
    pub meridians: &'static [MeridianEra],
    /// The fixed day the continuous year count is measured from.
    pub epoch: Rd,
    /// Added to the count of elapsed years since [`LunisolarParameters::epoch`]
    /// to give the year number this calendar displays.
    pub year_offset: i64,
    /// How major solar terms are placed.
    pub solar_term_mode: SolarTermMode,
    /// The earliest fixed day this calendar will convert.
    pub earliest: Option<Rd>,
    /// The latest fixed day this calendar will convert.
    pub latest: Option<Rd>,
}

/// The Chinese epoch: 15 February 2637 BCE, the traditional first year of the
/// Yellow Emperor's reign, from which the continuous year count runs.
///
/// It is a convention of the reckoning, not a dated event, and all four
/// calendars here measure elapsed years from it even when they display a
/// different number.
pub const CHINESE_EPOCH: Rd = Rd(-963_099);

/// `x` reduced into `1..=n`, the "adjusted modulo" of *Calendrical
/// Calculations*.
const fn adjusted_modulo(x: i64, n: i64) -> i64 {
    (x - 1).rem_euclid(n) + 1
}

impl LunisolarParameters {
    /// The meridian era in force on a fixed day.
    #[must_use]
    pub fn meridian_era(&self, rd: Rd) -> MeridianEra {
        let year = civil::year_from_rd(rd);
        let mut chosen = self.meridians[0];
        for era in self.meridians {
            if era.from_year <= year {
                chosen = *era;
            }
        }
        chosen
    }

    /// The offset from Universal Time in force on a fixed day, in days.
    #[must_use]
    pub fn zone_offset_days(&self, rd: Rd) -> f64 {
        self.meridian_era(rd).offset_hours / 24.0
    }

    /// A local-time moment as Universal Time.
    #[must_use]
    pub fn universal_from_local(&self, local: Moment) -> Moment {
        Moment(local.0 - self.zone_offset_days(local.day()))
    }

    /// A Universal Time moment as local time.
    #[must_use]
    pub fn local_from_universal(&self, universal: Moment) -> Moment {
        // The era is chosen from the universal-time day. The two can differ
        // only for a moment within the offset of midnight, and the eras are
        // centuries apart, so the choice is never the one in question.
        Moment(universal.0 + self.zone_offset_days(universal.day()))
    }

    /// Local midnight beginning `rd`, as a Universal Time moment.
    #[must_use]
    pub fn midnight(&self, rd: Rd) -> Moment {
        self.universal_from_local(Moment(rd.0 as f64))
    }

    /// The local day on which the December solstice of a Gregorian year
    /// falls.
    #[must_use]
    pub fn winter_solstice_day(&self, gregorian_year: i64) -> Rd {
        self.local_from_universal(hc_astro::solstice(gregorian_year, Solstice::December))
            .day()
    }

    /// The last winter solstice falling on or before `rd`, as a local day.
    #[must_use]
    pub fn winter_solstice_on_or_before(&self, rd: Rd) -> Rd {
        let year = civil::year_from_rd(rd);
        let candidate = self.winter_solstice_day(year);
        if candidate <= rd {
            candidate
        } else {
            self.winter_solstice_day(year - 1)
        }
    }

    /// The December solstice preceding local midnight of `rd`, in Universal
    /// Time.
    fn preceding_december_solstice(&self, rd: Rd) -> Moment {
        let year = civil::year_from_rd(rd);
        let candidate = hc_astro::solstice(year, Solstice::December);
        if self.local_from_universal(candidate).0 <= rd.0 as f64 {
            candidate
        } else {
            hc_astro::solstice(year - 1, Solstice::December)
        }
    }

    /// The first day of the first lunar month beginning on or after `rd`.
    #[must_use]
    pub fn new_moon_on_or_after(&self, rd: Rd) -> Rd {
        self.local_from_universal(hc_astro::new_moon_at_or_after(self.midnight(rd)))
            .day()
    }

    /// The first day of the last lunar month beginning before `rd`.
    #[must_use]
    pub fn new_moon_before(&self, rd: Rd) -> Rd {
        self.local_from_universal(hc_astro::new_moon_before(self.midnight(rd)))
            .day()
    }

    /// Which of the twelve major solar terms, numbered 1 to 12, the Sun had
    /// last passed at local midnight beginning `rd`.
    ///
    /// Term 11 is *dōngzhì*, the winter solstice, which is why month 11 is
    /// the one that must contain it.
    #[must_use]
    pub fn major_solar_term(&self, rd: Rd) -> i64 {
        match self.solar_term_mode {
            SolarTermMode::Apparent => {
                let longitude = hc_astro::solar_longitude(self.midnight(rd));
                adjusted_modulo(2 + floor(longitude / 30.0) as i64, 12)
            }
            SolarTermMode::Mean => {
                let solstice = self.preceding_december_solstice(rd);
                let elapsed = self.midnight(rd).0 - solstice.0;
                let step = MEAN_TROPICAL_YEAR / 12.0;
                adjusted_modulo(11 + floor(elapsed / step) as i64, 12)
            }
        }
    }

    /// Whether the lunar month beginning on `rd` contains no major solar
    /// term, which is what makes a month eligible to be the leap month.
    #[must_use]
    pub fn has_no_major_solar_term(&self, rd: Rd) -> bool {
        let next_month = self.new_moon_on_or_after(Rd(rd.0 + 1));
        self.major_solar_term(rd) == self.major_solar_term(next_month)
    }

    /// Whether a month without a major solar term has already occurred in
    /// this suì at or after `start`, up to and including the month beginning
    /// on `month`.
    ///
    /// Only the *first* such month is the leap month; this is the test that
    /// enforces "first".
    #[must_use]
    pub fn prior_leap_month(&self, start: Rd, month: Rd) -> bool {
        let mut cursor = month;
        while cursor >= start {
            if self.has_no_major_solar_term(cursor) {
                return true;
            }
            cursor = self.new_moon_before(cursor);
        }
        false
    }

    /// The first day of the lunisolar year within the suì containing `rd`.
    #[must_use]
    pub fn new_year_in_sui(&self, rd: Rd) -> Rd {
        let first_solstice = self.winter_solstice_on_or_before(rd);
        let next_solstice = self.winter_solstice_on_or_before(Rd(first_solstice.0 + 370));
        let month_twelve = self.new_moon_on_or_after(Rd(first_solstice.0 + 1));
        let month_thirteen = self.new_moon_on_or_after(Rd(month_twelve.0 + 1));
        let next_month_eleven = self.new_moon_before(Rd(next_solstice.0 + 1));
        let months = round((next_month_eleven.0 - month_twelve.0) as f64 / MEAN_SYNODIC_MONTH);
        // Thirteen new moons in the suì, and the leap month already fallen:
        // the year starts one month later than it otherwise would.
        if (months - 12.0).abs() < 0.5
            && (self.has_no_major_solar_term(month_twelve)
                || self.has_no_major_solar_term(month_thirteen))
        {
            self.new_moon_on_or_after(Rd(month_thirteen.0 + 1))
        } else {
            month_thirteen
        }
    }

    /// The first day of the lunisolar year on or before `rd`.
    #[must_use]
    pub fn new_year_on_or_before(&self, rd: Rd) -> Rd {
        let candidate = self.new_year_in_sui(rd);
        if rd >= candidate {
            candidate
        } else {
            self.new_year_in_sui(Rd(rd.0 - 180))
        }
    }

    /// The first day of the lunisolar year this calendar numbers `year`,
    /// without the range check.
    ///
    /// A day inside the supported range can belong to a year that began
    /// outside it, so conversion uses this and checks only the day it
    /// produces.
    fn year_start(&self, year: i64) -> Rd {
        let elapsed = year - self.year_offset;
        let mid_year = self.epoch.0 as f64 + (elapsed as f64 - 0.5) * MEAN_TROPICAL_YEAR;
        self.new_year_on_or_before(Rd(floor(mid_year) as i64))
    }

    /// The first day of the lunisolar year this calendar numbers `year`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] when the year's start falls
    /// outside the supported range.
    pub fn new_year(&self, year: i64) -> CalendarResult<Rd> {
        let start = self.year_start(year);
        self.check_range(start)
            .map_err(|_| CalendarError::YearOutOfRange)?;
        Ok(start)
    }

    /// Check a fixed day against the supported range.
    fn check_range(&self, rd: Rd) -> CalendarResult<()> {
        if self.earliest.is_some_and(|first| rd < first) {
            return Err(CalendarError::BeforeEpoch);
        }
        if self.latest.is_some_and(|last| rd > last) {
            return Err(CalendarError::AfterSupportedRange);
        }
        Ok(())
    }

    /// The date of a fixed day, without the range check.
    fn decompose(&self, rd: Rd) -> (i64, Month, u8) {
        let first_solstice = self.winter_solstice_on_or_before(rd);
        let next_solstice = self.winter_solstice_on_or_before(Rd(first_solstice.0 + 370));
        let month_twelve = self.new_moon_on_or_after(Rd(first_solstice.0 + 1));
        let next_month_eleven = self.new_moon_before(Rd(next_solstice.0 + 1));
        let this_month = self.new_moon_before(Rd(rd.0 + 1));
        let months_in_sui =
            round((next_month_eleven.0 - month_twelve.0) as f64 / MEAN_SYNODIC_MONTH);
        let leap_year = (months_in_sui - 12.0).abs() < 0.5;
        let elapsed_months =
            round((this_month.0 - month_twelve.0) as f64 / MEAN_SYNODIC_MONTH) as i64;
        let shift = if leap_year && self.prior_leap_month(month_twelve, this_month) {
            1
        } else {
            0
        };
        let ordinal = adjusted_modulo(elapsed_months - shift, 12);
        let is_leap_month = leap_year
            && self.has_no_major_solar_term(this_month)
            && !self.prior_leap_month(month_twelve, self.new_moon_before(this_month));
        let elapsed_years =
            floor(1.5 - ordinal as f64 / 12.0 + (rd.0 - self.epoch.0) as f64 / MEAN_TROPICAL_YEAR)
                as i64;
        let day = (rd.0 - this_month.0 + 1) as u8;
        (
            elapsed_years + self.year_offset,
            Month {
                ordinal: ordinal as u8,
                leap: is_leap_month,
            },
            day,
        )
    }

    /// The lunisolar year, month and day of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside the supported range.
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<(i64, Month, u8)> {
        self.check_range(rd)?;
        Ok(self.decompose(rd))
    }

    /// The fixed day of a lunisolar date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] when the month — typically
    /// a leap month — does not exist in that year, and
    /// [`CalendarError::DayOutOfRange`] when the month is shorter than `day`.
    pub fn to_fixed(&self, year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
        if month.ordinal == 0 || month.ordinal > 12 {
            return Err(CalendarError::MonthOutOfRange);
        }
        if day == 0 || day > 30 {
            return Err(CalendarError::DayOutOfRange);
        }
        let month_start = self
            .month_start(year, month)
            .ok_or(CalendarError::MonthOutOfRange)?;
        let rd = Rd(month_start.0 + day as i64 - 1);
        self.check_range(rd)?;
        // A lunisolar month's existence and length are not knowable without
        // running the rules, so the date is validated by reading it back.
        let (round_year, round_month, round_day) = self.decompose(rd);
        if round_month != month || round_year != year {
            return Err(CalendarError::MonthOutOfRange);
        }
        if round_day != day {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(rd)
    }

    /// The first day of a month, or `None` when it does not exist in that
    /// year.
    ///
    /// Twenty-nine days per month never overshoots, so the search starts at
    /// or before the month wanted and steps forward at most once.
    fn month_start(&self, year: i64, month: Month) -> Option<Rd> {
        if month.ordinal == 0 || month.ordinal > 12 {
            return None;
        }
        let start = self.year_start(year);
        let approximate = self.new_moon_on_or_after(Rd(start.0 + 29 * (month.ordinal as i64 - 1)));
        let (found_year, found, _) = self.decompose(approximate);
        if found == month && found_year == year {
            return Some(approximate);
        }
        let next = self.new_moon_on_or_after(Rd(approximate.0 + 1));
        let (next_year, next_month, _) = self.decompose(next);
        if next_month == month && next_year == year {
            Some(next)
        } else {
            None
        }
    }

    /// The number of days in a month, or `None` when that month does not
    /// exist in that year.
    #[must_use]
    pub fn days_in_month(&self, year: i64, month: Month) -> Option<u8> {
        let first = self.month_start(year, month)?;
        let next = self.new_moon_on_or_after(Rd(first.0 + 1));
        u8::try_from(next.0 - first.0).ok()
    }

    /// The number of months in a lunisolar year: 12, or 13 when it carries a
    /// leap month.
    ///
    /// # Errors
    ///
    /// Propagates the range check of [`LunisolarParameters::new_year`].
    pub fn months_in_year(&self, year: i64) -> CalendarResult<u8> {
        let start = self.new_year(year)?;
        let next = self.year_start(year + 1);
        let months = round((next.0 - start.0) as f64 / MEAN_SYNODIC_MONTH) as u8;
        Ok(months)
    }

    /// Whether the year carries a leap month.
    ///
    /// # Errors
    ///
    /// Propagates the range check of
    /// [`LunisolarParameters::months_in_year`].
    pub fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(self.months_in_year(year)? == 13)
    }

    /// Which month of `year` is the leap month, if any.
    ///
    /// # Errors
    ///
    /// Propagates the range check of [`LunisolarParameters::new_year`].
    pub fn leap_month(&self, year: i64) -> CalendarResult<Option<u8>> {
        let start = self.new_year(year)?;
        let next = self.year_start(year + 1);
        let mut cursor = start;
        while cursor < next {
            let (_, month, _) = self.decompose(cursor);
            if month.leap {
                return Ok(Some(month.ordinal));
            }
            cursor = self.new_moon_on_or_after(Rd(cursor.0 + 1));
        }
        Ok(None)
    }

    /// The sexagenary (干支) term of a year.
    ///
    /// The cycle is the shared East Asian one, so it is taken from the
    /// elapsed-year count rather than from the number this calendar
    /// displays: Dangi 4357 and Chinese 4661 are the same year and are both
    /// *jiǎ-chén*.
    #[must_use]
    pub fn sexagenary_year(&self, year: i64) -> Sexagenary {
        sexagenary_year(year - self.year_offset)
    }

    /// The sexagenary term of a month.
    ///
    /// A leap month has no term of its own in the tradition; it is given the
    /// term of the month it repeats, which is what this returns.
    #[must_use]
    pub fn sexagenary_month(&self, year: i64, month: Month) -> Sexagenary {
        let elapsed = year - self.year_offset;
        Sexagenary::from_index(
            12 * (elapsed - 1) + month.ordinal as i64 - 1 + SEXAGENARY_MONTH_ANCHOR,
        )
    }

    /// The sexagenary term of a day.
    ///
    /// The day cycle has run without interruption for longer than any of
    /// these calendars, so it does not depend on the calendar at all.
    #[must_use]
    pub fn sexagenary_day(&self, rd: Rd) -> Sexagenary {
        sexagenary_day(rd)
    }

    /// This calendar's metadata.
    #[must_use]
    pub fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id,
            english_name: self.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: self.earliest,
            latest: self.latest,
        }
    }
}

/// A date in a lunisolar calendar.
///
/// The type carries no calendar: the same year, month and day means different
/// days at different meridians, and the calendar it came from is what says
/// which.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LunisolarDate {
    /// The year, in this calendar's own numbering.
    pub year: i64,
    /// The month, 1 to 12, with the intercalary month flagged.
    pub month: Month,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl LunisolarDate {
    /// A date, without validation; the calendar validates it.
    #[must_use]
    pub const fn new(year: i64, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Whether this date falls in an intercalary month.
    #[must_use]
    pub const fn is_in_leap_month(self) -> bool {
        self.month.leap
    }
}

/// A lunisolar calendar built from a parameter set.
///
/// The four named calendars in this crate are thin wrappers over this; the
/// type is public so that a caller who wants a different meridian, a
/// different supported range or the *píngqì* rule can have one without
/// forking the engine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LunisolarCalendar {
    parameters: &'static LunisolarParameters,
}

impl LunisolarCalendar {
    /// A calendar from its parameters.
    #[must_use]
    pub const fn new(parameters: &'static LunisolarParameters) -> Self {
        Self { parameters }
    }

    /// The parameters behind this calendar.
    #[must_use]
    pub const fn parameters(&self) -> &'static LunisolarParameters {
        self.parameters
    }
}

impl Calendar for LunisolarCalendar {
    type Date = LunisolarDate;

    fn meta(&self) -> CalendarMeta {
        self.parameters.meta()
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.parameters.to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = self.parameters.from_fixed(rd)?;
        Ok(LunisolarDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let elapsed = date.year - self.parameters.year_offset;
        let mut fields = DateFields::new(date.year);
        fields.month = Some(date.month);
        fields.day = Some(date.day);
        // The cycle-and-position pair is how these dates are traditionally
        // written, and it is not recoverable from the year number alone
        // without knowing the calendar's offset.
        fields
            .extra
            .set("cycle", (elapsed - 1).div_euclid(60) + 1)?;
        fields
            .extra
            .set("year_of_cycle", adjusted_modulo(elapsed, 60))?;
        fields.extra.set(
            "sexagenary_year",
            i64::from(self.parameters.sexagenary_year(date.year).index()),
        )?;
        fields.extra.set(
            "sexagenary_month",
            i64::from(
                self.parameters
                    .sexagenary_month(date.year, date.month)
                    .index(),
            ),
        )?;
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let date = LunisolarDate {
            year: fields.year,
            month: fields.require_month()?,
            day: fields.require_day()?,
        };
        self.parameters
            .to_fixed(date.year, date.month, date.day)
            .map(|_| date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A bare Chinese parameter set, used to exercise the engine itself; the
    /// named calendars have their own tests.
    static ENGINE_TEST: LunisolarParameters = LunisolarParameters {
        id: CalendarId("test-lunisolar"),
        english_name: "Lunisolar engine test",
        meridians: &[
            MeridianEra::from_longitude(-5_000, 116.416_666_7, "Beijing local mean time"),
            MeridianEra::from_zone(1929, 8.0, "UTC+8"),
        ],
        epoch: CHINESE_EPOCH,
        year_offset: 0,
        solar_term_mode: SolarTermMode::Apparent,
        earliest: None,
        latest: None,
    };

    #[test]
    fn adjusted_modulo_lands_in_one_through_n() {
        assert_eq!(adjusted_modulo(1, 12), 1);
        assert_eq!(adjusted_modulo(12, 12), 12);
        assert_eq!(adjusted_modulo(13, 12), 1);
        assert_eq!(adjusted_modulo(0, 12), 12);
        assert_eq!(adjusted_modulo(-1, 12), 11);
        assert_eq!(adjusted_modulo(60, 60), 60);
    }

    #[test]
    fn the_meridian_table_is_read_in_order() {
        let before = ENGINE_TEST.meridian_era(civil::to_rd(1900, 1, 1));
        let after = ENGINE_TEST.meridian_era(civil::to_rd(1930, 1, 1));
        assert!((before.offset_hours - 116.416_666_7 / 15.0).abs() < 1e-9);
        assert!((after.offset_hours - 8.0).abs() < 1e-12);
        // The boundary is the Gregorian new year of the era's first year.
        assert_eq!(
            ENGINE_TEST
                .meridian_era(civil::to_rd(1929, 1, 1))
                .offset_hours,
            8.0
        );
        assert!(
            (ENGINE_TEST
                .meridian_era(civil::to_rd(1928, 12, 31))
                .offset_hours
                - 8.0)
                .abs()
                > 0.1
        );
    }

    #[test]
    fn a_meridian_era_reports_the_longitude_it_came_from() {
        let era = MeridianEra::from_longitude(1, 120.0, "UTC+8 as a meridian");
        assert!((era.offset_hours - 8.0).abs() < 1e-12);
        assert!((era.longitude_degrees_east() - 120.0).abs() < 1e-9);
    }

    #[test]
    fn the_winter_solstice_lands_in_the_second_half_of_december() {
        for year in 1900..2100i64 {
            let day = ENGINE_TEST.winter_solstice_day(year);
            let (gregorian_year, month, date) = civil::from_rd(day);
            assert_eq!(gregorian_year, year);
            assert_eq!(month, 12);
            assert!((20..=23).contains(&date), "{year}-12-{date}");
        }
    }

    #[test]
    fn month_eleven_contains_the_winter_solstice() {
        for year in 1950..2050i64 {
            let solstice = ENGINE_TEST.winter_solstice_day(year);
            let (_, month, _) = ENGINE_TEST.decompose(solstice);
            assert_eq!(month.ordinal, 11, "solstice of {year}");
            assert!(!month.leap);
        }
    }

    #[test]
    fn consecutive_new_moons_are_twenty_nine_or_thirty_days_apart() {
        let mut cursor = ENGINE_TEST.new_moon_on_or_after(civil::to_rd(1980, 1, 1));
        let end = civil::to_rd(2030, 1, 1);
        let mut months = 0;
        while cursor < end {
            let next = ENGINE_TEST.new_moon_on_or_after(Rd(cursor.0 + 1));
            let length = next.0 - cursor.0;
            assert!(
                (29..=30).contains(&length),
                "month at {cursor} was {length}"
            );
            months += 1;
            cursor = next;
        }
        assert!(months > 600, "{months} months examined");
    }

    #[test]
    fn the_mean_and_apparent_solar_term_rules_disagree_somewhere() {
        static MEAN: LunisolarParameters = LunisolarParameters {
            solar_term_mode: SolarTermMode::Mean,
            ..ENGINE_TEST_COPY
        };
        const ENGINE_TEST_COPY: LunisolarParameters = LunisolarParameters {
            id: CalendarId("test-lunisolar-pingqi"),
            english_name: "Lunisolar engine test, mean terms",
            meridians: &[
                MeridianEra::from_longitude(-5_000, 116.416_666_7, "Beijing local mean time"),
                MeridianEra::from_zone(1929, 8.0, "UTC+8"),
            ],
            epoch: CHINESE_EPOCH,
            year_offset: 0,
            solar_term_mode: SolarTermMode::Apparent,
            earliest: None,
            latest: None,
        };
        // Equal divisions of the mean year and true 30° steps agree at the
        // solstices and part company in between, so the term index differs on
        // a good fraction of days.
        let mut differing = 0;
        for offset in 0..365i64 {
            let rd = Rd(civil::to_rd(2000, 1, 1).0 + offset);
            if MEAN.major_solar_term(rd) != ENGINE_TEST.major_solar_term(rd) {
                differing += 1;
            }
        }
        assert!(differing > 10, "only {differing} days differed");
    }

    #[test]
    fn the_engine_round_trips_over_four_thousand_days() {
        let start = civil::to_rd(2000, 1, 1);
        for offset in 0..4_000i64 {
            let rd = Rd(start.0 + offset);
            let (year, month, day) = ENGINE_TEST.from_fixed(rd).expect("unbounded");
            assert_eq!(
                ENGINE_TEST.to_fixed(year, month, day),
                Ok(rd),
                "RD {rd} decoded to {year}-{month}-{day}"
            );
        }
    }

    #[test]
    fn a_year_holds_twelve_or_thirteen_months() {
        for year in 4_650..4_680i64 {
            let months = ENGINE_TEST.months_in_year(year).expect("unbounded");
            assert!(months == 12 || months == 13, "year {year} had {months}");
            let leap = ENGINE_TEST.leap_month(year).expect("unbounded");
            assert_eq!(leap.is_some(), months == 13, "year {year}");
            assert_eq!(
                ENGINE_TEST.is_leap_year(year).expect("unbounded"),
                months == 13
            );
        }
    }

    #[test]
    fn a_leap_month_repeats_the_ordinal_of_the_month_before_it() {
        for year in 4_650..4_690i64 {
            if let Some(ordinal) = ENGINE_TEST.leap_month(year).expect("unbounded") {
                let regular = ENGINE_TEST
                    .to_fixed(year, Month::regular(ordinal), 1)
                    .expect("the regular month exists");
                let leap = ENGINE_TEST
                    .to_fixed(year, Month::leap(ordinal), 1)
                    .expect("the leap month exists");
                let gap = leap.0 - regular.0;
                assert!((29..=30).contains(&gap), "year {year} gap {gap}");
            }
        }
    }

    #[test]
    fn a_leap_month_is_refused_in_a_year_that_has_none() {
        for year in 4_650..4_670i64 {
            let leap = ENGINE_TEST.leap_month(year).expect("unbounded");
            for ordinal in 1..=12u8 {
                if leap != Some(ordinal) {
                    assert_eq!(
                        ENGINE_TEST.to_fixed(year, Month::leap(ordinal), 1),
                        Err(CalendarError::MonthOutOfRange),
                        "year {year} leap month {ordinal}"
                    );
                }
            }
        }
    }

    #[test]
    fn out_of_range_months_and_days_are_refused() {
        assert_eq!(
            ENGINE_TEST.to_fixed(4_661, Month::regular(0), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            ENGINE_TEST.to_fixed(4_661, Month::regular(13), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            ENGINE_TEST.to_fixed(4_661, Month::regular(1), 0),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            ENGINE_TEST.to_fixed(4_661, Month::regular(1), 31),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn the_sexagenary_month_anchor_reproduces_the_traditional_rule() {
        // "甲己之年丙作首": in a year whose stem is jiǎ or jǐ, the first
        // month is bǐng-yín. Chinese year 4661 is jiǎ-chén.
        assert_eq!(ENGINE_TEST.sexagenary_year(4_661).stem_name(), "jia");
        assert_eq!(ENGINE_TEST.sexagenary_year(4_661).branch_name(), "chen");
        let first_month = ENGINE_TEST.sexagenary_month(4_661, Month::regular(1));
        assert_eq!(first_month.stem_name(), "bing");
        assert_eq!(first_month.branch_name(), "yin");
        // "乙庚之岁戊为头": in a yǐ or gēng year it is wù-yín.
        let next = ENGINE_TEST.sexagenary_month(4_662, Month::regular(1));
        assert_eq!(next.stem_name(), "wu");
        assert_eq!(next.branch_name(), "yin");
    }

    #[test]
    fn month_eleven_always_carries_the_rat_branch() {
        for year in 4_600..4_700i64 {
            assert_eq!(
                ENGINE_TEST
                    .sexagenary_month(year, Month::regular(11))
                    .branch_name(),
                "zi"
            );
        }
    }

    #[test]
    fn the_day_cycle_advances_by_one_a_day() {
        let rd = civil::to_rd(2024, 2, 10);
        assert_eq!(
            ENGINE_TEST.sexagenary_day(Rd(rd.0 + 1)),
            ENGINE_TEST.sexagenary_day(rd).next()
        );
        assert_eq!(
            ENGINE_TEST.sexagenary_day(Rd(rd.0 + 60)),
            ENGINE_TEST.sexagenary_day(rd)
        );
    }
}
