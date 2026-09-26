//! Calendars counted in *circads*: fixed fractions of a tidally locked moon's
//! solar day, grouped into eight-circad weeks, months and a borrowed year.
//!
//! Gangale's calendars for Titan and the Galilean moons all have this shape,
//! and differ only in their data: the solar day and its divisor, the epoch,
//! the month names and lengths, and the rule that picks each year's month
//! table. A [`CircadRule`] is that data and one [`CircadCalendar`] interprets
//! any rule, so a calendar is a `const` table, not a type. The system is
//! written up in `docs/systems/circad-calendars.md`, and the rules live in
//! [`crate::titan`] and [`crate::galilean`].
//!
//! # The fixed day is a circad
//!
//! As in [`crate::mars::darian`], the [`hc_calendar::Calendar`]
//! implementation pivots on a day count of the body the calendar belongs to:
//! **the [`Rd`] is a circad number, not an Earth day**. It exists so that a
//! circad date can travel through the formatters and the FFI boundary like
//! any other calendar's; it must never be handed to a terrestrial calendar.
//! To cross to Earth time, go through [`CircadCalendar::circad_at`] and
//! [`CircadCalendar::circad_start`], which know how long a circad is.
//!
//! # Every year is whole weeks
//!
//! Every year length in every rule is a multiple of eight, and circad 0 is
//! the first circad of a week, so the week is the circad number modulo
//! eight, running on across months and years without a break. A test holds
//! every rule to that.

use hc_calendar::shape::{CycleShape, MONTH};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_core::math::floor;
use hc_core::{Instant, Tai, TimeResult};

use crate::util::{instant_from_j2000_offset, j2000_offset_days};

/// Circads in a week, in every calendar of the family.
pub const CIRCADS_PER_WEEK: i64 = 8;

/// The earliest year any circad calendar converts, relative to its epoch
/// year.
///
/// Arbitrary and generous, as [`crate::mars::darian::MIN_YEAR`] is: it keeps
/// the arithmetic inside `i64` and rejects a nonsense year instead of
/// wrapping it.
pub const YEAR_SPAN: i64 = 1_000_000;

/// How a rule picks the month table of a year.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearCycle {
    /// A year divisible by `every` and not by `except` takes the second
    /// month table, every other year the first: Titan's eight-circad system,
    /// `every = 25`, `except = 400`. `except` must be a multiple of `every`.
    EveryExcept {
        /// The period of the intercalated years.
        every: i64,
        /// The period of the exceptions to it.
        except: i64,
    },
    /// The month table of a year is chosen by its last digit, through this
    /// ten-entry index into the rule's tables: the ten-year sequences of the
    /// Gregorian-based Galilean calendars.
    ByLastDigit([u8; 10]),
}

/// The data that defines one circad calendar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircadRule {
    /// The machine identifier.
    pub id: &'static str,
    /// The English name of the calendar.
    pub english_name: &'static str,
    /// The body whose solar day the circad divides.
    pub body: &'static str,
    /// The body's solar day in Earth days, as the source gives it.
    pub solar_day_days: f64,
    /// Circads in one solar day.
    pub circads_per_solar_day: u8,
    /// The circad in Earth days, as the source calculates with it.
    pub circad_days: f64,
    /// TT days from J2000.0 at which circad [`Self::anchor_circad`] begins:
    /// the source's calibration.
    pub anchor_j2000_tt_days: f64,
    /// The circad number the calibration names.
    pub anchor_circad: i64,
    /// The year whose first circad is circad 0.
    pub epoch_year: i64,
    /// The month names, in order.
    pub month_names: &'static [&'static str],
    /// The eight names of the circads of the week, in order.
    pub week_names: &'static [&'static str; 8],
    /// The cycles the calendar declares: its months and its week.
    pub cycles: &'static [CycleShape],
    /// The month lengths of each kind of year, shortest year first. Every
    /// table has one entry per month name.
    pub month_tables: &'static [&'static [u8]],
    /// Which table each year uses.
    pub year_cycle: YearCycle,
    /// Where the rule comes from, as the module documentation cites it.
    pub source: &'static str,
}

impl CircadRule {
    /// The index into [`Self::month_tables`] that `year` uses.
    #[must_use]
    pub const fn year_kind(&self, year: i64) -> usize {
        match self.year_cycle {
            YearCycle::EveryExcept { every, except } => {
                if year.rem_euclid(every) == 0 && year.rem_euclid(except) != 0 {
                    1
                } else {
                    0
                }
            }
            YearCycle::ByLastDigit(kinds) => kinds[year.rem_euclid(10) as usize] as usize,
        }
    }

    /// The month lengths of `year`.
    #[must_use]
    pub const fn month_lengths(&self, year: i64) -> &'static [u8] {
        self.month_tables[self.year_kind(year)]
    }

    /// The circads in a year of kind `kind`.
    const fn table_length(&self, kind: usize) -> i64 {
        let table = self.month_tables[kind];
        let mut total = 0i64;
        let mut index = 0;
        while index < table.len() {
            total += table[index] as i64;
            index += 1;
        }
        total
    }

    /// The circads in `year`.
    #[must_use]
    pub const fn circads_in_year(&self, year: i64) -> i64 {
        self.table_length(self.year_kind(year))
    }

    /// Whether `year` carries the intercalary week: the longer month table
    /// of an [`YearCycle::EveryExcept`] rule, or of a
    /// [`YearCycle::ByLastDigit`] rule that has two.
    #[must_use]
    pub const fn is_leap_year(&self, year: i64) -> bool {
        self.year_kind(year) == 1
    }

    /// Circads from the start of year 0 of the rule's own count to the start
    /// of `year`, on an arbitrary origin; only differences of it mean
    /// anything.
    const fn cumulative(&self, year: i64) -> i64 {
        match self.year_cycle {
            YearCycle::EveryExcept { every, except } => {
                let common = self.table_length(0);
                let extra = self.table_length(1) - common;
                // Multiples of `every` in 0..year, less the multiples of
                // `except`; each count is `(year − 1) \ m + 1` with floor
                // division, so the `+ 1`s cancel and negative years count
                // backwards.
                common * year
                    + extra * ((year - 1).div_euclid(every) - (year - 1).div_euclid(except))
            }
            YearCycle::ByLastDigit(kinds) => {
                let mut decade = 0i64;
                let mut before = 0i64;
                let digit = year.rem_euclid(10) as usize;
                let mut index = 0;
                while index < 10 {
                    let length = self.table_length(kinds[index] as usize);
                    decade += length;
                    if index < digit {
                        before += length;
                    }
                    index += 1;
                }
                year.div_euclid(10) * decade + before
            }
        }
    }

    /// The circad number of the first circad of `year`.
    #[must_use]
    pub const fn year_start(&self, year: i64) -> i64 {
        self.cumulative(year) - self.cumulative(self.epoch_year)
    }

    /// The earliest year converted.
    #[must_use]
    pub const fn min_year(&self) -> i64 {
        self.epoch_year - YEAR_SPAN
    }

    /// The latest year converted.
    #[must_use]
    pub const fn max_year(&self) -> i64 {
        self.epoch_year + YEAR_SPAN
    }
}

/// A date in a circad calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CircadDate {
    /// The year, in the calendar's own count. Year 0 is a real year.
    pub year: i64,
    /// The month, from 1.
    pub month: u8,
    /// The circad of the month, from 1.
    pub day: u8,
}

impl CircadDate {
    /// A date, unchecked; [`CircadCalendar::circad_from_date`] validates it.
    #[must_use]
    pub const fn new(year: i64, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

/// A circad calendar: a [`CircadRule`] and the arithmetic every rule shares.
///
/// See the module documentation before using the [`Calendar`]
/// implementation: the `Rd` values it produces are **circad** numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircadCalendar {
    rule: &'static CircadRule,
}

impl CircadCalendar {
    /// The calendar a rule defines.
    #[must_use]
    pub const fn new(rule: &'static CircadRule) -> Self {
        Self { rule }
    }

    /// The rule.
    #[must_use]
    pub const fn rule(&self) -> &'static CircadRule {
        self.rule
    }

    /// The circad number of a date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`],
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`]
    /// when the date does not exist — including a circad a short month lacks.
    pub fn circad_from_date(&self, date: CircadDate) -> CalendarResult<i64> {
        let rule = self.rule;
        if date.year < rule.min_year() || date.year > rule.max_year() {
            return Err(CalendarError::YearOutOfRange);
        }
        let lengths = rule.month_lengths(date.year);
        let length = *lengths
            .get(usize::from(date.month).wrapping_sub(1))
            .ok_or(CalendarError::MonthOutOfRange)?;
        if date.day == 0 || date.day > length {
            return Err(CalendarError::DayOutOfRange);
        }
        let before: i64 = lengths[..usize::from(date.month - 1)]
            .iter()
            .map(|&days| i64::from(days))
            .sum();
        Ok(rule.year_start(date.year) + before + i64::from(date.day) - 1)
    }

    /// The date of a circad number.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside the supported years.
    pub fn date_from_circad(&self, circad: i64) -> CalendarResult<CircadDate> {
        let rule = self.rule;
        if circad < rule.year_start(rule.min_year())
            || circad >= rule.year_start(rule.max_year() + 1)
        {
            return Err(CalendarError::YearOutOfRange);
        }
        // The mean year over one full cycle gives an estimate within a year
        // or two; the loops below make it exact.
        let (cycle_years, cycle_length) = match rule.year_cycle {
            YearCycle::EveryExcept { except, .. } => (
                except,
                rule.year_start(rule.epoch_year + except) - rule.year_start(rule.epoch_year),
            ),
            YearCycle::ByLastDigit(_) => (
                10,
                rule.year_start(rule.epoch_year + 10) - rule.year_start(rule.epoch_year),
            ),
        };
        let mut year = rule.epoch_year + (circad * cycle_years).div_euclid(cycle_length);
        while rule.year_start(year + 1) <= circad {
            year += 1;
        }
        while rule.year_start(year) > circad {
            year -= 1;
        }
        let mut remaining = circad - rule.year_start(year);
        for (index, &length) in rule.month_lengths(year).iter().enumerate() {
            let length = i64::from(length);
            if remaining < length {
                // Both fit: a month index is below 24 and a day below 41.
                return Ok(CircadDate::new(year, index as u8 + 1, remaining as u8 + 1));
            }
            remaining -= length;
        }
        Err(CalendarError::YearOutOfRange)
    }

    /// The name of a date's month.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] for a month the calendar
    /// does not have.
    pub fn month_name(&self, date: CircadDate) -> CalendarResult<&'static str> {
        self.rule
            .month_names
            .get(usize::from(date.month).wrapping_sub(1))
            .copied()
            .ok_or(CalendarError::MonthOutOfRange)
    }

    /// The position of a circad in its week, `0..8`, 0 being the first.
    #[must_use]
    pub const fn week_position(circad: i64) -> u8 {
        circad.rem_euclid(CIRCADS_PER_WEEK) as u8
    }

    /// The name of a circad's place in the week.
    #[must_use]
    pub fn week_name(&self, circad: i64) -> &'static str {
        self.rule.week_names[usize::from(Self::week_position(circad))]
    }

    /// The circad number an instant falls in, and how far through it, in
    /// `[0, 1)`.
    #[must_use]
    pub fn circad_and_fraction_at(&self, instant: Instant<Tai>) -> (i64, f64) {
        let rule = self.rule;
        let elapsed = (j2000_offset_days(instant) - rule.anchor_j2000_tt_days) / rule.circad_days;
        let whole = floor(elapsed);
        (rule.anchor_circad + whole as i64, elapsed - whole)
    }

    /// The circad number an instant falls in.
    #[must_use]
    pub fn circad_at(&self, instant: Instant<Tai>) -> i64 {
        self.circad_and_fraction_at(instant).0
    }

    /// How far through its circad an instant is, in `[0, 1)`: the circad's
    /// clock, which Gangale divides into 24 hours of 60 minutes.
    #[must_use]
    pub fn circad_fraction(&self, instant: Instant<Tai>) -> f64 {
        self.circad_and_fraction_at(instant).1
    }

    /// The date at an instant.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside the supported years.
    pub fn date_at(&self, instant: Instant<Tai>) -> CalendarResult<CircadDate> {
        self.date_from_circad(self.circad_at(instant))
    }

    /// The instant a circad begins.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::Overflow`] when the instant leaves the
    /// representable range.
    pub fn circad_start(&self, circad: i64) -> TimeResult<Instant<Tai>> {
        let rule = self.rule;
        instant_from_j2000_offset(
            rule.anchor_j2000_tt_days + (circad - rule.anchor_circad) as f64 * rule.circad_days,
        )
    }
}

impl Calendar for CircadCalendar {
    type Date = CircadDate;

    /// The months and the eight-circad week, whose positions are circads
    /// and not Earth weekdays, so its kind is `circad-of-week`.
    fn cycles(&self) -> &'static [CycleShape] {
        self.rule.cycles
    }

    /// A year that carries the intercalary week.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if year < self.rule.min_year() || year > self.rule.max_year() {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self.rule.is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        let rule = self.rule;
        CalendarMeta {
            id: CalendarId(rule.id),
            english_name: rule.english_name,
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(Rd(rule.year_start(rule.min_year()))),
            latest: Some(Rd(rule.year_start(rule.max_year() + 1) - 1)),
            native_locales: &[],
        }
    }

    /// The **circad** number of a date, carried in an [`Rd`].
    ///
    /// # Errors
    ///
    /// See [`CircadCalendar::circad_from_date`].
    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.circad_from_date(date).map(Rd)
    }

    /// The date of a **circad** number carried in an [`Rd`].
    ///
    /// # Errors
    ///
    /// See [`CircadCalendar::date_from_circad`].
    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.date_from_circad(rd.get())
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        self.circad_from_date(date)?;
        Ok(DateFields::ymd(date.year, date.month, date.day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = CircadDate::new(fields.year, month.ordinal, fields.require_day()?);
        self.circad_from_date(date)?;
        Ok(date)
    }
}

/// The cycles of a circad calendar with these month and week names.
///
/// A `const` helper so that each rule's `cycles` slice is one line.
#[must_use]
pub const fn cycles(
    month_names: &'static [&'static str],
    week_names: &'static [&'static str; 8],
) -> [CycleShape; 2] {
    [
        CycleShape::named(MONTH, month_names),
        CycleShape::named("circad-of-week", week_names),
    ]
}

/// Every circad calendar this crate defines.
#[must_use]
pub const fn all() -> [CircadCalendar; 5] {
    [
        crate::titan::DARIAN_TITAN,
        crate::galilean::GREGORIAN_IO,
        crate::galilean::GREGORIAN_EUROPA,
        crate::galilean::GREGORIAN_GANYMEDE,
        crate::galilean::GREGORIAN_CALLISTO,
    ]
}

/// The circad calendar with this identifier.
#[must_use]
pub fn by_id(id: &str) -> Option<CircadCalendar> {
    all().into_iter().find(|calendar| calendar.rule.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_rule_is_well_formed() {
        for calendar in all() {
            let rule = calendar.rule();
            assert_eq!(rule.week_names.len(), 8, "{}", rule.id);
            assert!(!rule.month_tables.is_empty(), "{}", rule.id);
            for table in rule.month_tables {
                assert_eq!(table.len(), rule.month_names.len(), "{}", rule.id);
            }
            assert_eq!(rule.cycles.len(), 2, "{}", rule.id);
            assert_eq!(rule.cycles[0].names, rule.month_names, "{}", rule.id);
            // The circad is the stated fraction of the solar day, to the
            // digits the sources print.
            let circad = rule.solar_day_days / f64::from(rule.circads_per_solar_day);
            assert!((circad - rule.circad_days).abs() < 2e-9, "{}", rule.id);
        }
    }

    #[test]
    fn every_year_is_whole_weeks_so_the_week_never_breaks() {
        for calendar in all() {
            let rule = calendar.rule();
            for year in rule.epoch_year - 1_000..rule.epoch_year + 1_000 {
                assert_eq!(
                    rule.circads_in_year(year) % CIRCADS_PER_WEEK,
                    0,
                    "{} {year}",
                    rule.id
                );
            }
            assert_eq!(rule.year_start(rule.epoch_year), 0, "{}", rule.id);
        }
    }

    #[test]
    fn the_month_lengths_add_up_to_the_year_lengths() {
        for calendar in all() {
            let rule = calendar.rule();
            for year in rule.epoch_year - 700..rule.epoch_year + 700 {
                let total: i64 = rule.month_lengths(year).iter().map(|&m| i64::from(m)).sum();
                assert_eq!(total, rule.circads_in_year(year), "{} {year}", rule.id);
                assert_eq!(
                    rule.year_start(year + 1) - rule.year_start(year),
                    total,
                    "{} {year}",
                    rule.id
                );
            }
        }
    }

    #[test]
    fn every_rule_round_trips_through_circad_numbers() {
        for calendar in all() {
            let rule = calendar.rule();
            let mut circad = rule.year_start(rule.epoch_year - 120);
            let end = rule.year_start(rule.epoch_year + 120);
            while circad < end {
                let date = calendar.date_from_circad(circad).unwrap();
                assert_eq!(
                    calendar.circad_from_date(date).unwrap(),
                    circad,
                    "{} {date:?}",
                    rule.id
                );
                circad += 1;
            }
            // And at the far ends of the range, where the estimate of the year
            // is furthest from exact.
            for circad in [
                rule.year_start(rule.min_year()),
                rule.year_start(rule.max_year() + 1) - 1,
            ] {
                let date = calendar.date_from_circad(circad).unwrap();
                assert_eq!(
                    calendar.circad_from_date(date).unwrap(),
                    circad,
                    "{}",
                    rule.id
                );
            }
        }
    }

    #[test]
    fn the_calendar_trait_round_trips_and_rejects_what_does_not_exist() {
        for calendar in all() {
            let rule = calendar.rule();
            let months = rule.month_names.len() as u8;
            let date = CircadDate::new(rule.epoch_year, 1, 1);
            assert_eq!(calendar.to_fixed(date), Ok(Rd(0)), "{}", rule.id);
            assert_eq!(calendar.from_fixed(Rd(0)), Ok(date), "{}", rule.id);
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date));
            assert_eq!(
                calendar.to_fixed(CircadDate::new(rule.epoch_year, 0, 1)),
                Err(CalendarError::MonthOutOfRange)
            );
            assert_eq!(
                calendar.to_fixed(CircadDate::new(rule.epoch_year, months + 1, 1)),
                Err(CalendarError::MonthOutOfRange)
            );
            assert_eq!(
                calendar.to_fixed(CircadDate::new(rule.epoch_year, 1, 0)),
                Err(CalendarError::DayOutOfRange)
            );
            assert_eq!(
                calendar.to_fixed(CircadDate::new(rule.epoch_year, 1, 41)),
                Err(CalendarError::DayOutOfRange)
            );
            assert_eq!(
                calendar.to_fixed(CircadDate::new(rule.max_year() + 1, 1, 1)),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                calendar.from_fixed(Rd(rule.year_start(rule.min_year()) - 1)),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                calendar.is_leap_year(rule.max_year() + 1),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                calendar.from_fields(&DateFields::ymd_leap_month(rule.epoch_year, 1, 1)),
                Err(CalendarError::MonthOutOfRange)
            );
            assert!(
                calendar
                    .from_fields(&DateFields::new(rule.epoch_year))
                    .is_err()
            );
            assert_eq!(
                calendar.month_name(CircadDate::new(0, months + 1, 1)),
                Err(CalendarError::MonthOutOfRange)
            );
            let meta = calendar.meta();
            assert_eq!(meta.id, CalendarId(rule.id));
            assert!(meta.supports(Rd(0)));
            assert!(!meta.has_leap_months && !meta.is_astronomical);
            assert_eq!(by_id(rule.id), Some(calendar));
        }
        assert_eq!(by_id("darian"), None);
    }

    #[test]
    fn the_week_runs_on_across_every_month_and_year_boundary() {
        for calendar in all() {
            let rule = calendar.rule();
            let first = rule.year_start(rule.epoch_year - 3);
            let last = rule.year_start(rule.epoch_year + 3);
            for circad in first..last {
                let here = CircadCalendar::week_position(circad);
                let next = CircadCalendar::week_position(circad + 1);
                assert_eq!((here + 1) % 8, next);
            }
            // Every year begins on the first circad of the week.
            for year in rule.epoch_year - 50..rule.epoch_year + 50 {
                assert_eq!(
                    calendar.week_name(rule.year_start(year)),
                    rule.week_names[0]
                );
            }
        }
    }

    #[test]
    fn an_instant_falls_in_the_circad_that_begins_before_it() {
        for calendar in all() {
            for circad in [-100_000, -1, 0, 1, 12_345, 150_000] {
                let start = calendar.circad_start(circad).unwrap();
                let (found, fraction) = calendar.circad_and_fraction_at(start);
                // The start instant is at worst a hair either side of the
                // boundary after the round trip through f64.
                assert!(
                    (found == circad && fraction < 1e-6)
                        || (found == circad - 1 && fraction > 1.0 - 1e-6),
                    "{} {circad}: {found} {fraction}",
                    calendar.rule().id
                );
                let middle = calendar.circad_start(circad).unwrap().checked_add(
                    hc_core::Duration::from_secs_f64(calendar.rule().circad_days * 43_200.0)
                        .unwrap(),
                );
                let middle = middle.unwrap();
                assert_eq!(calendar.circad_at(middle), circad);
                assert!((calendar.circad_fraction(middle) - 0.5).abs() < 1e-6);
            }
        }
    }
}
