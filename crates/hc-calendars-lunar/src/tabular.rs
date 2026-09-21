//! The tabular (arithmetic) Hijri calendar, parameterised.
//!
//! The Hijri calendar as actually kept is observational: a month begins when
//! the crescent is *seen*. Since the ninth century, astronomers and
//! chancelleries have also used an arithmetic stand-in that averages the
//! synodic month: twelve months alternating 30 and 29 days, giving a year of
//! 354 days, with a thirtieth day added to the last month in eleven years out
//! of every thirty. Thirty years then run 10 631 days, a mean month of
//! 29.530556 days — 2.9 seconds short of the true mean synodic month, so the
//! scheme drifts by a day in roughly 2 500 years.
//!
//! # The data/algorithm split, in miniature
//!
//! The arithmetic above is the *algorithm*, and it is identical for every
//! tabular Hijri calendar ever used. What varies is two pieces of *data*:
//!
//! * **which eleven of the thirty years are long.** Four intercalation
//!   schemes are in circulation, and they are traditionally attributed to
//!   different astronomers ([`LeapYearRule`]).
//! * **which day the epoch falls on.** The Hijra is conventionally placed at
//!   16 July 622 in the Julian calendar (the "civil" or Friday epoch) or at
//!   15 July 622 (the "astronomical" or Thursday epoch).
//!
//! So this module holds one implementation and the callers supply the two
//! constants. [`crate::islamic_civil`] and [`crate::islamic_astronomical`]
//! are the two named combinations; any of the eight is reachable through
//! [`TabularIslamicCalendar::new`].
//!
//! # What this is not
//!
//! It is not the calendar of any state that determines months by sighting or
//! by the Umm al-Qura tables, and it does not claim to reproduce any
//! historical proclamation. Against the Saudi Umm al-Qura calendar
//! ([`crate::islamic_umalqura`]) the civil variant disagrees by a day or two
//! for most months.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

/// The era code of the Hijri era, "anno Hegirae".
pub const ERA: &str = "AH";

/// Years in the intercalation cycle.
pub const CYCLE_YEARS: i64 = 30;

/// Days in the intercalation cycle: 30 years of 354 days plus 11 leap days.
pub const CYCLE_DAYS: i64 = 10_631;

/// The earliest Hijri year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest Hijri year this implementation converts.
///
/// The bound is arbitrary but generous; it keeps the day count well inside
/// `i64` and matches the range the other calendars in `hyper-calendar` offer.
pub const MAX_YEAR: i64 = 9_999;

/// The fixed day of 16 July 622 in the Julian calendar, the Friday on which
/// the "civil" tabular Hijri calendar places 1 Muharram 1 AH.
///
/// Its Julian Day Number is 1 948 440, the value quoted throughout the
/// literature for the civil Hijri epoch.
pub const CIVIL_EPOCH: Rd = Rd(227_015);

/// The fixed day of 15 July 622 in the Julian calendar, the Thursday on which
/// the "astronomical" tabular Hijri calendar places 1 Muharram 1 AH.
///
/// Its Julian Day Number is 1 948 439. CLDR calls this variant
/// `islamic-tbla`, where `tbla` abbreviates "tabular, leap year, astronomical
/// epoch".
pub const ASTRONOMICAL_EPOCH: Rd = Rd(227_014);

/// Which eleven years of the thirty-year cycle carry the extra day.
///
/// The four schemes below are the ones the standard surveys of the medieval
/// *zīj*es tabulate. The attributions are the conventional ones and are given
/// here to name the variants, not as a claim about who first wrote each table
/// down; the schemes are much better attested than their authorship.
///
/// All four agree on years 2, 5, 13, 21 and 24, so they never drift more than
/// a day or two from one another inside a cycle and they realign exactly
/// every thirty years.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LeapYearRule {
    /// 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29 — the common scheme,
    /// associated with al-Fazārī, al-Khwārizmī and al-Battānī, and the one
    /// every "tabular Hijri" implementation means unless it says otherwise.
    ///
    /// Equivalent to the test `(14 + 11 · year) mod 30 < 11` used by
    /// Reingold and Dershowitz, *Calendrical Calculations*.
    #[default]
    Civil,
    /// 2, 5, 7, 10, 13, 15, 18, 21, 24, 26, 29 — associated with Kūshyār
    /// ibn Labbān and with the Fatimid/Ismaili (Ṭayyibī Bohra) reckoning.
    ///
    /// It differs from [`LeapYearRule::Civil`] in a single year: the long
    /// year at 16 moves to 15, so the two calendars run one day apart for
    /// years 15 to 15 of the cycle and agree again from year 16.
    KushyarIbnLabban,
    /// 2, 5, 8, 10, 13, 16, 19, 21, 24, 27, 29.
    Fatimid,
    /// 2, 5, 8, 11, 13, 16, 19, 21, 24, 27, 30 — associated with Ḥabash
    /// al-Ḥāsib, al-Bīrūnī and Elias of Nisibis.
    ///
    /// The only scheme whose thirtieth year is long, which means the extra
    /// day falls at the very end of the cycle rather than one year earlier.
    HabashAlHasib,
}

impl LeapYearRule {
    /// The eleven long years of the cycle, in ascending order.
    #[must_use]
    pub const fn leap_years(self) -> [u8; 11] {
        match self {
            Self::Civil => [2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29],
            Self::KushyarIbnLabban => [2, 5, 7, 10, 13, 15, 18, 21, 24, 26, 29],
            Self::Fatimid => [2, 5, 8, 10, 13, 16, 19, 21, 24, 27, 29],
            Self::HabashAlHasib => [2, 5, 8, 11, 13, 16, 19, 21, 24, 27, 30],
        }
    }

    /// Whether `position`, a year numbered 1 to 30 within the cycle, is long.
    ///
    /// Positions outside `1..=30` are never long.
    #[must_use]
    pub const fn is_long_position(self, position: u8) -> bool {
        let table = self.leap_years();
        let mut index = 0;
        while index < table.len() {
            if table[index] == position {
                return true;
            }
            index += 1;
        }
        false
    }

    /// How many long years precede `position` within the cycle.
    ///
    /// `position` is a year numbered 1 to 30; the count covers positions
    /// `1..position`.
    #[must_use]
    pub const fn long_years_before(self, position: u8) -> i64 {
        let table = self.leap_years();
        let mut index = 0;
        let mut count = 0;
        while index < table.len() {
            if (table[index] as i64) < position as i64 {
                count += 1;
            }
            index += 1;
        }
        count
    }
}

/// A year's position within the thirty-year cycle, numbered 1 to 30, and how
/// many whole cycles precede it.
const fn cycle_position(year: i64) -> (i64, u8) {
    let elapsed = year - 1;
    (
        elapsed.div_euclid(CYCLE_YEARS),
        (elapsed.rem_euclid(CYCLE_YEARS) + 1) as u8,
    )
}

/// Whether `year` carries the extra day in Dhū al-Ḥijja.
#[must_use]
pub const fn is_leap_year(rule: LeapYearRule, year: i64) -> bool {
    let (_, position) = cycle_position(year);
    rule.is_long_position(position)
}

/// The number of days in `month` of `year`, or `None` outside `1..=12`.
///
/// Odd months have 30 days and even months 29, except that the twelfth month
/// gains a day in a long year.
#[must_use]
pub const fn days_in_month(rule: LeapYearRule, year: i64, month: u8) -> Option<u8> {
    match month {
        1..=11 => Some(if month % 2 == 1 { 30 } else { 29 }),
        12 => Some(if is_leap_year(rule, year) { 30 } else { 29 }),
        _ => None,
    }
}

/// The number of days in `year`: 354, or 355 in a long year.
#[must_use]
pub const fn days_in_year(rule: LeapYearRule, year: i64) -> u16 {
    if is_leap_year(rule, year) { 355 } else { 354 }
}

/// Days elapsed in the year before the first of `month`.
///
/// `29 · (m − 1) + ⌊m / 2⌋` is the closed form of "alternating 30 and 29
/// starting at 30".
const fn days_before_month(month: u8) -> i64 {
    29 * (month as i64 - 1) + (month as i64) / 2
}

/// The fixed day of a tabular Hijri date, without validation.
const fn to_fixed_raw(epoch: Rd, rule: LeapYearRule, year: i64, month: u8, day: u8) -> i64 {
    let (cycles, position) = cycle_position(year);
    epoch.0
        + cycles * CYCLE_DAYS
        + 354 * (position as i64 - 1)
        + rule.long_years_before(position)
        + days_before_month(month)
        + day as i64
        - 1
}

/// The fixed day of a tabular Hijri date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(
    epoch: Rd,
    rule: LeapYearRule,
    year: i64,
    month: u8,
    day: u8,
) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match days_in_month(rule, year, month) {
        None => Err(CalendarError::MonthOutOfRange),
        Some(length) => {
            if day == 0 || day > length {
                Err(CalendarError::DayOutOfRange)
            } else {
                Ok(Rd(to_fixed_raw(epoch, rule, year, month, day)))
            }
        }
    }
}

/// The earliest fixed day a given parameter set converts.
#[must_use]
pub const fn earliest(epoch: Rd, rule: LeapYearRule) -> Rd {
    Rd(to_fixed_raw(epoch, rule, MIN_YEAR, 1, 1))
}

/// The latest fixed day a given parameter set converts.
#[must_use]
pub const fn latest(epoch: Rd, rule: LeapYearRule) -> Rd {
    Rd(to_fixed_raw(epoch, rule, MAX_YEAR + 1, 1, 1) - 1)
}

/// The tabular Hijri year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside the supported range.
pub const fn from_fixed(epoch: Rd, rule: LeapYearRule, rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < earliest(epoch, rule).0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > latest(epoch, rule).0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    let elapsed = rd.0 - epoch.0;
    let cycles = elapsed.div_euclid(CYCLE_DAYS);
    let mut within = elapsed.rem_euclid(CYCLE_DAYS);
    // Thirty steps at worst: the cycle is short enough that walking it is
    // cheaper to read than the closed form, and `from_fixed` is not on a hot
    // path.
    let mut position: u8 = 1;
    while position < 30 {
        let length = if rule.is_long_position(position) {
            355
        } else {
            354
        };
        if within < length {
            break;
        }
        within -= length;
        position += 1;
    }
    let year = 1 + cycles * CYCLE_YEARS + position as i64 - 1;
    // Months alternate 30 and 29, so two days per 59 gives the month index
    // directly; only the intercalary 30th of the twelfth month overflows it.
    let month_index = (2 * within) / 59 + 1;
    let month = if month_index > 12 {
        12
    } else {
        month_index as u8
    };
    let day = (within - days_before_month(month) + 1) as u8;
    Ok((year, month, day))
}

/// A tabular Hijri date.
///
/// The type carries no parameters: the same year, month and day means
/// different days in different variants, and the calendar it came from is
/// what says which.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IslamicDate {
    /// The year of the Hijri era, counting from 1.
    pub year: i64,
    /// The month, 1 for Muḥarram through 12 for Dhū al-Ḥijja.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

/// A tabular Hijri calendar with a chosen epoch and intercalation scheme.
///
/// The named combinations are [`crate::islamic_civil::IslamicCivilCalendar`]
/// and [`crate::islamic_astronomical::IslamicAstronomicalCalendar`]; this
/// type is how the other six are reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabularIslamicCalendar {
    id: CalendarId,
    english_name: &'static str,
    epoch: Rd,
    rule: LeapYearRule,
}

impl TabularIslamicCalendar {
    /// A tabular Hijri calendar from its two parameters.
    ///
    /// `id` and `english_name` are carried rather than derived because a
    /// variant that CLDR has no identifier for still has to be able to name
    /// itself in a registry.
    #[must_use]
    pub const fn new(
        id: CalendarId,
        english_name: &'static str,
        epoch: Rd,
        rule: LeapYearRule,
    ) -> Self {
        Self {
            id,
            english_name,
            epoch,
            rule,
        }
    }

    /// The fixed day on which this variant places 1 Muḥarram 1 AH.
    #[must_use]
    pub const fn epoch(&self) -> Rd {
        self.epoch
    }

    /// This variant's intercalation scheme.
    #[must_use]
    pub const fn rule(&self) -> LeapYearRule {
        self.rule
    }

    /// Whether `year` carries the extra day in Dhū al-Ḥijja.
    #[must_use]
    pub const fn is_leap_year(&self, year: i64) -> bool {
        is_leap_year(self.rule, year)
    }

    /// The number of days in `month` of `year`, or `None` outside `1..=12`.
    #[must_use]
    pub const fn days_in_month(&self, year: i64, month: u8) -> Option<u8> {
        days_in_month(self.rule, year, month)
    }

    /// The number of days in `year`.
    #[must_use]
    pub const fn days_in_year(&self, year: i64) -> u16 {
        days_in_year(self.rule, year)
    }
}

/// The Fatimid or Ṭayyibī tabular Hijri calendar — the Bohra *Misri*.
///
/// The official calendar of the Ṭayyibī Ismāʿīlī communities, of whom the
/// Dawoodi Bohras are the largest, and the operative calendar for every
/// religious date they keep. It is purely calculated and never sighted,
/// which is the point: the community's own account of it gives the rule as
/// "divide the Hijri year by 30; if the remainder is 2, 5, 8, 10, 13, 16,
/// 19, 21, 24, 27 or 29 it is a *kabisa* year", with odd months of 30 days
/// and even months of 29.
///
/// That is [`LeapYearRule::Fatimid`] — the common scheme with three of its
/// long years delayed by one, the third to 8, the seventh to 19 and the
/// tenth to 27 — at the Thursday epoch, not the Friday one. The epoch is
/// fixed here by the community's own published anchor rather than by
/// assumption: 12 Rabīʿ al-Awwal 1439 fell on 30 November 2017, which the
/// Thursday epoch gives and the Friday epoch misses by a day. The test says
/// so.
///
/// **Source:** the Dawoodi Bohra community's published description of the
/// Misri-Hijri calculation (thedawoodibohras.com), which states the
/// thirty-year *qarn saghir* and its eleven *kabisa* remainders.
pub const FATIMID: TabularIslamicCalendar = TabularIslamicCalendar::new(
    CalendarId("islamic-fatimid"),
    "Hijri (Fatimid, Ṭayyibī Bohra \"Misri\")",
    ASTRONOMICAL_EPOCH,
    LeapYearRule::Fatimid,
);

impl Default for TabularIslamicCalendar {
    fn default() -> Self {
        Self::new(
            CalendarId("islamic-civil"),
            "Hijri (tabular, civil epoch)",
            CIVIL_EPOCH,
            LeapYearRule::Civil,
        )
    }
}

impl Calendar for TabularIslamicCalendar {
    type Date = IslamicDate;

    fn cycles(&self) -> Option<&'static [hc_calendar::shape::CycleShape]> {
        Some(hc_calendar::shape::SOLAR_TWELVE)
    }

    /// The Islamic day begins at sunset, which is also why the month begins
    /// with a crescent seen after one.
    ///
    /// Declared here rather than only on the wrappers in `islamic_civil`
    /// and `islamic_astronomical`. It used to be only there, so those two
    /// were right and every other tabular variant a caller built — or that
    /// this crate registered — silently said midnight.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id,
            english_name: self.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(earliest(self.epoch, self.rule)),
            latest: Some(latest(self.epoch, self.rule)),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(self.epoch, self.rule, date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(self.epoch, self.rule, rd)?;
        Ok(IslamicDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(ERA))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = IslamicDate {
            year: fields.year,
            month: month.ordinal,
            day: fields.require_day()?,
        };
        to_fixed(self.epoch, self.rule, date.year, date.month, date.day)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {

    /// The anchor the community publishes itself: 12 Rabīʿ al-Awwal 1439,
    /// the Mawlid of that year, fell on 30 November 2017.
    ///
    /// This is what fixes [`FATIMID`]'s epoch. The Fatimid leap rule at the
    /// Friday epoch puts that day at 11 Rabīʿ al-Awwal; at the Thursday
    /// epoch it lands exactly. Without the anchor the choice would have
    /// been a guess, and a guess of one day is invisible until somebody
    /// misses a fast.
    #[test]
    fn the_bohra_misri_calendar_matches_its_own_published_mawlid() {
        let day = hc_calendar::gregorian::to_fixed(2017, 11, 30).expect("2017-11-30 exists");
        let date = FATIMID.from_fixed(day).expect("the date is in range");
        assert_eq!(date.year, 1439);
        assert_eq!(date.month, 3);
        assert_eq!(date.day, 12);
        assert_eq!(FATIMID.to_fixed(date), Ok(day));

        // The Friday epoch is the near miss that makes the test worth having.
        let friday = TabularIslamicCalendar::new(
            CalendarId("test-only"),
            "test",
            CIVIL_EPOCH,
            LeapYearRule::Fatimid,
        );
        assert_eq!(friday.from_fixed(day).map(|other| other.day), Ok(11));
    }

    /// Every tabular Hijri calendar starts its day at sunset, whatever
    /// epoch and leap rule it carries. This used to be true only of the two
    /// that had their own wrapper type.
    #[test]
    fn a_tabular_hijri_day_begins_at_sunset() {
        use hc_calendar::DayBoundary;
        assert_eq!(FATIMID.day_boundary(), DayBoundary::Sunset);
        assert_eq!(
            TabularIslamicCalendar::default().day_boundary(),
            DayBoundary::Sunset
        );
        let custom = TabularIslamicCalendar::new(
            CalendarId("test-only"),
            "test",
            CIVIL_EPOCH,
            LeapYearRule::HabashAlHasib,
        );
        assert_eq!(custom.day_boundary(), DayBoundary::Sunset);
    }

    /// The rule the community states: divide by 30, and these eleven
    /// remainders are long years.
    #[test]
    fn the_kabisa_remainders_are_the_ones_the_community_publishes() {
        assert_eq!(
            LeapYearRule::Fatimid.leap_years(),
            [2, 5, 8, 10, 13, 16, 19, 21, 24, 27, 29]
        );
        // 1431 has remainder 21 and is kabisa; 1432 has remainder 22 and is
        // not — the worked example on the community's own page.
        assert_eq!(FATIMID.days_in_year(1431), 355);
        assert_eq!(FATIMID.days_in_year(1432), 354);
    }
    use super::*;
    use crate::civil;

    const ALL_RULES: [LeapYearRule; 4] = [
        LeapYearRule::Civil,
        LeapYearRule::KushyarIbnLabban,
        LeapYearRule::Fatimid,
        LeapYearRule::HabashAlHasib,
    ];

    /// The closed form given by Reingold and Dershowitz, *Calendrical
    /// Calculations*, for `fixed-from-islamic`. It is written out here so
    /// that the table-driven implementation above is checked against a
    /// published formula rather than against itself.
    fn reingold_dershowitz_to_fixed(year: i64, month: u8, day: u8) -> i64 {
        CIVIL_EPOCH.0 - 1
            + 354 * (year - 1)
            + (3 + 11 * year).div_euclid(30)
            + 29 * (month as i64 - 1)
            + (month as i64) / 2
            + day as i64
    }

    #[test]
    fn the_civil_epoch_is_the_sixteenth_of_july_622() {
        // 16 July 622 Julian is 19 July 622 in the proleptic Gregorian
        // calendar; the Julian Day Number of the civil Hijri epoch is
        // 1 948 440.
        assert_eq!(CIVIL_EPOCH.to_julian_day_number(), 1_948_440);
        assert_eq!(civil::from_rd(CIVIL_EPOCH), (622, 7, 19));
        assert_eq!(
            to_fixed(CIVIL_EPOCH, LeapYearRule::Civil, 1, 1, 1),
            Ok(CIVIL_EPOCH)
        );
    }

    #[test]
    fn the_astronomical_epoch_is_one_day_earlier() {
        assert_eq!(ASTRONOMICAL_EPOCH.to_julian_day_number(), 1_948_439);
        assert_eq!(ASTRONOMICAL_EPOCH.0, CIVIL_EPOCH.0 - 1);
        assert_eq!(civil::from_rd(ASTRONOMICAL_EPOCH), (622, 7, 18));
    }

    #[test]
    fn the_civil_epoch_is_a_friday_and_the_astronomical_one_a_thursday() {
        use hc_calendar::Weekday;
        assert_eq!(Weekday::from_rd(CIVIL_EPOCH), Weekday::Friday);
        assert_eq!(Weekday::from_rd(ASTRONOMICAL_EPOCH), Weekday::Thursday);
    }

    #[test]
    fn the_table_driven_civil_rule_agrees_with_the_published_closed_form() {
        for year in 1..=3_000i64 {
            assert_eq!(
                is_leap_year(LeapYearRule::Civil, year),
                (14 + 11 * year).rem_euclid(30) < 11,
                "year {year}"
            );
            for month in 1..=12u8 {
                let day = 1;
                assert_eq!(
                    to_fixed(CIVIL_EPOCH, LeapYearRule::Civil, year, month, day),
                    Ok(Rd(reingold_dershowitz_to_fixed(year, month, day))),
                    "{year}-{month}-{day}"
                );
            }
        }
    }

    #[test]
    fn every_rule_has_eleven_long_years_in_thirty() {
        for rule in ALL_RULES {
            let table = rule.leap_years();
            assert_eq!(table.len(), 11);
            let mut previous = 0u8;
            for position in table {
                assert!(position > previous, "{rule:?} is not ascending");
                assert!((1..=30).contains(&position));
                previous = position;
            }
            let long = (1..=30u8).filter(|p| rule.is_long_position(*p)).count();
            assert_eq!(long, 11, "{rule:?}");
        }
    }

    #[test]
    fn every_rule_gives_a_cycle_of_ten_thousand_six_hundred_and_thirty_one_days() {
        for rule in ALL_RULES {
            let total: i64 = (1..=30i64)
                .map(|year| days_in_year(rule, year) as i64)
                .sum();
            assert_eq!(total, CYCLE_DAYS, "{rule:?}");
        }
    }

    #[test]
    fn all_four_rules_agree_on_the_years_the_sources_agree_on() {
        for position in [2u8, 5, 13, 21, 24] {
            for rule in ALL_RULES {
                assert!(rule.is_long_position(position), "{rule:?} at {position}");
            }
        }
    }

    #[test]
    fn the_variants_realign_at_every_cycle_boundary() {
        // The eleven long years are distributed differently but there are
        // always eleven, so each cycle boundary falls on the same day in
        // every variant.
        for rule in ALL_RULES {
            for cycle in 0..40i64 {
                let year = cycle * 30 + 1;
                assert_eq!(
                    to_fixed(CIVIL_EPOCH, rule, year, 1, 1),
                    Ok(Rd(CIVIL_EPOCH.0 + cycle * CYCLE_DAYS)),
                    "{rule:?} cycle {cycle}"
                );
            }
        }
    }

    #[test]
    fn kushyars_rule_differs_from_the_civil_one_in_exactly_one_year() {
        let differing = (1..=30u8)
            .filter(|position| {
                LeapYearRule::Civil.is_long_position(*position)
                    != LeapYearRule::KushyarIbnLabban.is_long_position(*position)
            })
            .count();
        assert_eq!(differing, 2, "15 becomes long and 16 becomes short");
        assert!(LeapYearRule::KushyarIbnLabban.is_long_position(15));
        assert!(!LeapYearRule::KushyarIbnLabban.is_long_position(16));
    }

    #[test]
    fn every_variant_round_trips_over_thirty_thousand_days() {
        for rule in ALL_RULES {
            for epoch in [CIVIL_EPOCH, ASTRONOMICAL_EPOCH] {
                let start = earliest(epoch, rule).0;
                for offset in 0..30_000i64 {
                    let rd = Rd(start + offset);
                    let (year, month, day) =
                        from_fixed(epoch, rule, rd).expect("inside the supported range");
                    assert_eq!(
                        to_fixed(epoch, rule, year, month, day),
                        Ok(rd),
                        "{rule:?} {year}-{month}-{day}"
                    );
                }
            }
        }
    }

    #[test]
    fn the_last_supported_day_round_trips() {
        for rule in ALL_RULES {
            let last = latest(CIVIL_EPOCH, rule);
            let (year, month, day) = from_fixed(CIVIL_EPOCH, rule, last).expect("in range");
            assert_eq!(year, MAX_YEAR);
            assert_eq!(month, 12);
            assert_eq!(to_fixed(CIVIL_EPOCH, rule, year, month, day), Ok(last));
            assert_eq!(
                from_fixed(CIVIL_EPOCH, rule, Rd(last.0 + 1)),
                Err(CalendarError::AfterSupportedRange)
            );
            assert_eq!(
                from_fixed(CIVIL_EPOCH, rule, Rd(earliest(CIVIL_EPOCH, rule).0 - 1)),
                Err(CalendarError::BeforeEpoch)
            );
        }
    }

    #[test]
    fn months_alternate_thirty_and_twenty_nine_days() {
        for month in 1..=11u8 {
            let expected = if month % 2 == 1 { 30 } else { 29 };
            assert_eq!(
                days_in_month(LeapYearRule::Civil, 1_445, month),
                Some(expected)
            );
        }
        assert_eq!(days_in_month(LeapYearRule::Civil, 1_445, 13), None);
        assert_eq!(days_in_month(LeapYearRule::Civil, 1_445, 0), None);
    }

    #[test]
    fn the_intercalary_day_falls_at_the_end_of_the_twelfth_month() {
        // 1453 AH sits at position 13 of the cycle, which is long in every
        // one of the four schemes.
        for rule in ALL_RULES {
            assert!(is_leap_year(rule, 1_453), "{rule:?}");
            assert_eq!(days_in_month(rule, 1_453, 12), Some(30));
            assert_eq!(days_in_year(rule, 1_453), 355);
        }
        // 1454 sits at position 14, which is short in every scheme.
        for rule in ALL_RULES {
            assert!(!is_leap_year(rule, 1_454), "{rule:?}");
            assert_eq!(days_in_month(rule, 1_454, 12), Some(29));
            assert_eq!(days_in_year(rule, 1_454), 354);
        }
    }

    #[test]
    fn out_of_range_fields_name_the_field_that_is_wrong() {
        let rule = LeapYearRule::Civil;
        assert_eq!(
            to_fixed(CIVIL_EPOCH, rule, 0, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(CIVIL_EPOCH, rule, MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(CIVIL_EPOCH, rule, 1_445, 13, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(CIVIL_EPOCH, rule, 1_445, 2, 30),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(CIVIL_EPOCH, rule, 1_445, 1, 0),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn the_calendar_trait_rejects_a_leap_month_and_an_unknown_era() {
        let calendar = TabularIslamicCalendar::default();
        assert_eq!(
            calendar.from_fields(&DateFields::ymd_leap_month(1_445, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1_445, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::new(1_445)),
            Err(CalendarError::MissingField("month"))
        );
    }

    #[test]
    fn the_astronomical_variant_runs_exactly_one_day_ahead_of_the_civil_one() {
        for year in [1i64, 100, 1_000, 1_445, 5_000] {
            for month in 1..=12u8 {
                let civil_day = to_fixed(CIVIL_EPOCH, LeapYearRule::Civil, year, month, 1);
                let astronomical =
                    to_fixed(ASTRONOMICAL_EPOCH, LeapYearRule::Civil, year, month, 1);
                assert_eq!(
                    astronomical.map(|rd| rd.0 + 1),
                    civil_day.map(|rd| rd.0),
                    "{year}-{month}"
                );
            }
        }
    }
}
