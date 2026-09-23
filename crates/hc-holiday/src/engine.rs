//! The evaluator, and business-day arithmetic.
//!
//! One function walks a [`RuleSet`], and every country and tradition in the
//! crate goes through it. There is no per-country code anywhere below this
//! line, and adding a country adds no branch to anything here.
//!
//! # The order the modifiers apply in
//!
//! 1. **Base rules.** Every rule valid in the year and in the requested
//!    region is evaluated. The neighbouring years are evaluated too, because
//!    a substitution or a bridge can reach across 1 January.
//! 2. **Substitution.** Days are taken in date order so that a substitute
//!    can be pushed past a substitute already assigned — which is exactly
//!    what happens when Christmas Day falls on a Saturday in the United
//!    Kingdom.
//! 3. **Bridges.** Japan's 国民の休日 is evaluated against the *base*
//!    holidays, because the statute says the neighbouring days must be
//!    国民の祝日 and a 振替休日 is not one.
//! 4. **Clipping** to the requested years.

use alloc::vec::Vec;

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::rule::{
    Confidence, HolidayRule, Kind, RuleSet, SubstituteDirection, SubstitutionPolicy,
};

/// One holiday on one day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Holiday {
    /// The day it falls on.
    pub date: Rd,
    /// The English name.
    pub name: &'static str,
    /// The name in the local language, or `""`.
    pub local_name: &'static str,
    /// What sort of day it is.
    pub kind: Kind,
    /// How firm the date is. Anything Hijri-dated is
    /// [`Confidence::Approximate`].
    pub confidence: Confidence,
    /// The subdivisions it applies to; empty means nationwide.
    pub regions: &'static [&'static str],
    /// When this entry is a weekend substitute, the day it substitutes for.
    pub observed_for: Option<Rd>,
    /// Whether this entry was produced by a bridge policy.
    pub bridged: bool,
    /// The instrument that established the entry, when the rule cites one;
    /// see [`HolidayRule::source`](crate::rule::HolidayRule::source).
    pub source: &'static str,
}

impl Holiday {
    /// Whether this entry stops work.
    #[must_use]
    pub const fn is_day_off(&self) -> bool {
        self.kind.is_day_off()
    }

    /// Whether this entry is a weekend substitute rather than the holiday
    /// itself.
    #[must_use]
    pub const fn is_substitute(&self) -> bool {
        self.observed_for.is_some()
    }
}

/// A holiday the calendar could not compute, and why.
///
/// A rule dated in the Chinese, Korean, Vietnamese or Umm al-Qurā calendar
/// has no answer outside that calendar's range, and an evaluated calendar
/// used to express that by simply not listing the holiday — so
/// `holidays_in_year(CHINA, 2151)` returned seven entries instead of
/// thirteen, with 春節, 端午節 and 中秋節 missing and everything that
/// remained marked [`Confidence::Exact`].
///
/// Policy §4 says the library refuses rather than guesses. Omitting a
/// holiday silently is a guess — that it did not happen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gap {
    /// The Gregorian year that could not be answered.
    pub year: i64,
    /// The English name of the holiday.
    pub name: &'static str,
    /// Its name in the local language, or `""`.
    pub local_name: &'static str,
}

/// An evaluated rule set over a span of years.
///
/// Building one is the expensive part — solar terms and lunar conjunctions
/// are astronomy — so queries that touch many days, above all business-day
/// arithmetic, should build a calendar once and ask it repeatedly.
#[derive(Debug, Clone)]
pub struct HolidayCalendar<'a> {
    rules: &'a RuleSet,
    region: Option<&'a str>,
    first_year: i64,
    last_year: i64,
    first_day: Rd,
    last_day: Rd,
    holidays: Vec<Holiday>,
    gaps: Vec<Gap>,
}

impl<'a> HolidayCalendar<'a> {
    /// Evaluate `rules` over `first_year..=last_year`.
    ///
    /// `region` selects a subdivision by ISO 3166-2 code. `None` asks for
    /// the nationwide set, which is the set of rules with no subdivision
    /// scoping — not the union of every subdivision's rules.
    ///
    /// # Panics
    ///
    /// Does not panic. An empty or reversed year range yields an empty
    /// calendar.
    #[must_use]
    pub fn new(
        rules: &'a RuleSet,
        region: Option<&'a str>,
        first_year: i64,
        last_year: i64,
    ) -> Self {
        let first_day = gregorian::to_fixed(first_year, 1, 1).unwrap_or(Rd(0));
        let last_day = gregorian::to_fixed(last_year, 12, 31).unwrap_or(Rd(-1));
        let (holidays, gaps) = if first_year > last_year {
            (Vec::new(), Vec::new())
        } else {
            evaluate_with_includes(rules, region, first_year, last_year, 0)
        };
        Self {
            rules,
            region,
            first_year,
            last_year,
            first_day,
            last_day,
            holidays,
            gaps,
        }
    }

    /// Evaluate one year.
    #[must_use]
    pub fn for_year(rules: &'a RuleSet, region: Option<&'a str>, year: i64) -> Self {
        Self::new(rules, region, year, year)
    }

    /// The holidays this calendar could not compute, and the years it could
    /// not compute them in.
    ///
    /// Empty for every year inside every referenced calendar's range, which
    /// is every year most callers ask about. When it is not empty, the
    /// holiday list is incomplete and [`HolidayCalendar::is_complete`] says
    /// so.
    #[must_use]
    pub fn gaps(&self) -> &[Gap] {
        &self.gaps
    }

    /// Whether every rule could be answered for every year asked for.
    ///
    /// A `false` here does not mean the answers given are wrong; it means
    /// some are missing. See [`HolidayCalendar::gaps`] for which.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.gaps.is_empty()
    }

    /// The rule set behind this calendar.
    #[must_use]
    pub const fn rule_set(&self) -> &'a RuleSet {
        self.rules
    }

    /// The subdivision this calendar was built for.
    #[must_use]
    pub const fn region(&self) -> Option<&'a str> {
        self.region
    }

    /// The first and last Gregorian year covered.
    #[must_use]
    pub const fn years(&self) -> (i64, i64) {
        (self.first_year, self.last_year)
    }

    /// Whether `day` falls inside the evaluated span.
    #[must_use]
    pub const fn covers(&self, day: Rd) -> bool {
        day.0 >= self.first_day.0 && day.0 <= self.last_day.0
    }

    /// Every holiday in the span, in date order.
    #[must_use]
    pub fn all(&self) -> &[Holiday] {
        &self.holidays
    }

    /// Every holiday in one Gregorian year, in date order.
    #[must_use]
    pub fn in_year(&self, year: i64) -> Vec<Holiday> {
        let Ok(first) = gregorian::to_fixed(year, 1, 1) else {
            return Vec::new();
        };
        let Ok(last) = gregorian::to_fixed(year, 12, 31) else {
            return Vec::new();
        };
        self.holidays
            .iter()
            .filter(|holiday| holiday.date >= first && holiday.date <= last)
            .copied()
            .collect()
    }

    /// Every entry falling on `day`, including observances with no day off.
    #[must_use]
    pub fn on(&self, day: Rd) -> Vec<Holiday> {
        self.holidays
            .iter()
            .filter(|holiday| holiday.date == day)
            .copied()
            .collect()
    }

    /// Whether `day` is a holiday that stops work.
    #[must_use]
    pub fn is_holiday(&self, day: Rd) -> bool {
        self.holidays
            .iter()
            .any(|holiday| holiday.date == day && holiday.is_day_off())
    }

    /// The name of the first day-off holiday on `day`, if any.
    #[must_use]
    pub fn name_on(&self, day: Rd) -> Option<&'static str> {
        self.holidays
            .iter()
            .find(|holiday| holiday.date == day && holiday.is_day_off())
            .map(|holiday| holiday.name)
    }

    /// The next day-off holiday strictly after `day`.
    #[must_use]
    pub fn next_holiday(&self, day: Rd) -> Option<Holiday> {
        self.holidays
            .iter()
            .find(|holiday| holiday.date > day && holiday.is_day_off())
            .copied()
    }

    /// The last day-off holiday strictly before `day`.
    #[must_use]
    pub fn previous_holiday(&self, day: Rd) -> Option<Holiday> {
        self.holidays
            .iter()
            .rev()
            .find(|holiday| holiday.date < day && holiday.is_day_off())
            .copied()
    }

    /// Whether `day` falls on the weekend, under the weekend law in force
    /// that year.
    #[must_use]
    pub fn is_weekend(&self, day: Rd) -> bool {
        let Ok(year) = gregorian::year_from_fixed(day) else {
            return false;
        };
        self.rules.weekend_in(year).contains(&Weekday::from_rd(day))
    }

    /// Whether `day` is a working day: neither the weekend nor a holiday.
    #[must_use]
    pub fn is_business_day(&self, day: Rd) -> bool {
        !self.is_weekend(day) && !self.is_holiday(day)
    }

    /// `day` moved by `count` business days.
    ///
    /// A positive `count` moves forward, a negative one back, and zero
    /// returns `day` unchanged whether or not it is itself a business day.
    /// The starting day is never counted; the result is always a business
    /// day when `count` is non-zero.
    ///
    /// Returns `None` when the walk leaves the evaluated span, because a
    /// calendar cannot honestly answer for a year it has not evaluated.
    #[must_use]
    pub fn add_business_days(&self, day: Rd, count: i64) -> Option<Rd> {
        if !self.covers(day) {
            return None;
        }
        if count == 0 {
            return Some(day);
        }
        let step = if count > 0 { 1 } else { -1 };
        let mut remaining = count.abs();
        let mut cursor = day;
        while remaining > 0 {
            cursor = Rd(cursor.0 + step);
            if !self.covers(cursor) {
                return None;
            }
            if self.is_business_day(cursor) {
                remaining -= 1;
            }
        }
        Some(cursor)
    }

    /// The number of business days in the half-open interval
    /// `[start, end)`.
    ///
    /// Half-open because that is the interval that composes: the count from
    /// Monday to Wednesday plus the count from Wednesday to Friday is the
    /// count from Monday to Friday. `end` before `start` gives a negative
    /// count.
    ///
    /// Returns `None` when either end lies outside the evaluated span.
    #[must_use]
    pub fn business_days_between(&self, start: Rd, end: Rd) -> Option<i64> {
        if !self.covers(start) || !self.covers(end) {
            return None;
        }
        if start == end {
            return Some(0);
        }
        let (from, to, sign) = if start < end {
            (start, end, 1)
        } else {
            (end, start, -1)
        };
        let mut count = 0i64;
        let mut cursor = from;
        while cursor < to {
            if self.is_business_day(cursor) {
                count += 1;
            }
            cursor = Rd(cursor.0 + 1);
        }
        Some(count * sign)
    }
}

/// Convenience: every holiday of one year for one rule set.
#[must_use]
pub fn holidays_in_year(rules: &RuleSet, region: Option<&str>, year: i64) -> Vec<Holiday> {
    HolidayCalendar::for_year(rules, region, year)
        .all()
        .to_vec()
}

/// Convenience: whether one day is a day-off holiday.
#[must_use]
pub fn is_holiday(rules: &RuleSet, region: Option<&str>, day: Rd) -> bool {
    let Ok(year) = gregorian::year_from_fixed(day) else {
        return false;
    };
    HolidayCalendar::for_year(rules, region, year).is_holiday(day)
}

/// An occurrence before the modifiers have been applied.
#[derive(Debug, Clone, Copy)]
struct Occurrence {
    date: Rd,
    rule: &'static HolidayRule,
}

/// Evaluate a rule set, apply every modifier, and clip to the requested
/// years.
/// How many levels of [`RuleSet::includes`] the engine follows.
const INCLUDE_DEPTH: u8 = 8;

/// A set's own holidays and gaps, with those of every set it includes,
/// each evaluated under its own policies, merged in date order.
fn evaluate_with_includes(
    rules: &RuleSet,
    region: Option<&str>,
    first_year: i64,
    last_year: i64,
    depth: u8,
) -> (Vec<Holiday>, Vec<Gap>) {
    let (mut holidays, mut gaps) = evaluate(rules, region, first_year, last_year);
    if depth >= INCLUDE_DEPTH {
        return (holidays, gaps);
    }
    for included in rules.includes {
        let (more, more_gaps) = evaluate_with_includes(
            included.set,
            included.region,
            first_year,
            last_year,
            depth + 1,
        );
        // Only the days off: an included set's commemorations are its own.
        // Hong Kong keeps the Winter Solstice as an observance, and the
        // exchange that closes on Hong Kong's holidays trades through it.
        holidays.extend(more.into_iter().filter(Holiday::is_day_off));
        gaps.extend(more_gaps);
    }
    if !rules.includes.is_empty() {
        holidays.sort_by_key(|holiday| holiday.date);
    }
    (holidays, gaps)
}

fn evaluate(
    rules: &RuleSet,
    region: Option<&str>,
    first_year: i64,
    last_year: i64,
) -> (Vec<Holiday>, Vec<Gap>) {
    // One year of slack each side: a substitution can push 31 December into
    // January, and a bridge can sit either side of New Year's Day.
    let mut base: Vec<Occurrence> = Vec::new();
    let mut gaps: Vec<Gap> = Vec::new();
    for year in (first_year - 1)..=(last_year + 1) {
        for rule in rules.rules {
            if !rule.applies_in(year) || !rule.applies_in_region(region) {
                continue;
            }
            // Only the years actually asked for are reported as gaps; the
            // year of slack on each side exists to catch a substitution
            // crossing New Year, and its absence is not a hole in the
            // answer.
            if !rule.rule.is_resolvable_in(year) {
                if (first_year..=last_year).contains(&year) {
                    gaps.push(Gap {
                        year,
                        name: rule.name,
                        local_name: rule.local_name,
                    });
                }
                continue;
            }
            for date in rule.rule.days_in_year(year).as_slice() {
                base.push(Occurrence { date: *date, rule });
            }
        }
    }
    base.sort_by_key(|occurrence| (occurrence.date.0, occurrence.rule.name));
    base.dedup_by_key(|occurrence| (occurrence.date.0, occurrence.rule.name));

    let mut out: Vec<Holiday> = base
        .iter()
        .map(|occurrence| Holiday {
            date: occurrence.date,
            name: occurrence.rule.name,
            local_name: occurrence.rule.local_name,
            kind: occurrence.rule.kind,
            confidence: occurrence.rule.confidence,
            regions: occurrence.rule.regions,
            observed_for: None,
            bridged: false,
            source: occurrence.rule.source,
        })
        .collect();

    // A day-off day, for the purposes of "is this slot taken". Substitutes
    // are added to this set as they are assigned, which is what makes the
    // British Christmas/Boxing Day pair land two days apart.
    let mut occupied: Vec<Rd> = out
        .iter()
        .filter(|holiday| holiday.is_day_off())
        .map(|holiday| holiday.date)
        .collect();
    occupied.sort_unstable();

    let collided = collisions(&base);
    let mut substitutes: Vec<Holiday> = Vec::new();
    for (occurrence, &collided) in base.iter().zip(&collided) {
        let Ok(year) = gregorian::year_from_fixed(occurrence.date) else {
            continue;
        };
        if !occurrence.rule.kind.is_day_off() {
            continue;
        }
        if !occurrence.rule.substitutes_in(year) {
            continue;
        }
        let Some(policy) = rules.substitution_in(year) else {
            continue;
        };
        let trigger = occurrence.rule.substitute_trigger.unwrap_or(policy.trigger);
        let triggered = trigger.contains(&Weekday::from_rd(occurrence.date))
            || (policy.on_collision && collided);
        if !triggered {
            continue;
        }
        let Some(target) = substitute_day(occurrence.date, policy, trigger, &occupied) else {
            continue;
        };
        if let Err(index) = occupied.binary_search(&target) {
            occupied.insert(index, target);
        }
        substitutes.push(Holiday {
            date: target,
            name: occurrence.rule.name,
            local_name: occurrence.rule.local_name,
            kind: occurrence.rule.kind,
            confidence: occurrence.rule.confidence,
            regions: occurrence.rule.regions,
            observed_for: Some(occurrence.date),
            bridged: false,
            source: occurrence.rule.source,
        });
    }

    // Bridges see the base holidays as neighbours, and the base holidays
    // plus the substitutes as "already taken".
    let base_days: Vec<Rd> = out
        .iter()
        .filter(|holiday| holiday.is_day_off())
        .map(|holiday| holiday.date)
        .collect();
    let mut bridges: Vec<Holiday> = Vec::new();
    for policy in rules.bridges {
        for offset in 0..=(last_year - first_year + 2) {
            let year = first_year - 1 + offset;
            if !policy.applies_in(year) {
                continue;
            }
            let (Ok(start), Ok(end)) = (
                gregorian::to_fixed(year, 1, 1),
                gregorian::to_fixed(year, 12, 31),
            ) else {
                continue;
            };
            let mut day = start;
            while day <= end {
                let candidate = day;
                day = Rd(day.0 + 1);
                if policy.max_gap == 0 {
                    continue;
                }
                if policy
                    .exclude_weekdays
                    .contains(&Weekday::from_rd(candidate))
                {
                    continue;
                }
                if occupied.binary_search(&candidate).is_ok() {
                    continue;
                }
                let before = Rd(candidate.0 - 1);
                let after = Rd(candidate.0 + 1);
                if base_days.binary_search(&before).is_ok()
                    && base_days.binary_search(&after).is_ok()
                {
                    bridges.push(Holiday {
                        date: candidate,
                        name: policy.name,
                        local_name: policy.local_name,
                        kind: Kind::Public,
                        confidence: Confidence::Exact,
                        regions: &[],
                        observed_for: None,
                        bridged: true,
                        source: "",
                    });
                }
            }
        }
    }

    out.extend(substitutes);
    out.extend(bridges);

    let (Ok(clip_start), Ok(clip_end)) = (
        gregorian::to_fixed(first_year, 1, 1),
        gregorian::to_fixed(last_year, 12, 31),
    ) else {
        return (Vec::new(), gaps);
    };
    out.retain(|holiday| holiday.date >= clip_start && holiday.date <= clip_end);
    out.sort_by_key(|holiday| {
        (
            holiday.date.0,
            holiday.observed_for.is_some(),
            holiday.name,
            holiday.bridged,
        )
    });
    out.dedup_by_key(|holiday| (holiday.date.0, holiday.name, holiday.observed_for.is_some()));
    (out, gaps)
}

/// For each occurrence, in `base`'s date order, whether it is owed a
/// substitute for sharing its day with another day off.
///
/// Holidays that share a day lose one day each but one, so a day with `k`
/// of them owes `k - 1` substitutes. They go to the occurrences that can be
/// substituted, the later ones in `base`'s order first: on 3 October 2017
/// the eve of Chuseok, substituted since 2014, shared its day with National
/// Foundation Day, not substituted until 2021, and the substitute was
/// Chuseok's.
fn collisions(base: &[Occurrence]) -> Vec<bool> {
    let mut collided = vec![false; base.len()];
    let mut end = base.len();
    while end > 0 {
        let date = base[end - 1].date;
        let mut start = end - 1;
        while start > 0 && base[start - 1].date == date {
            start -= 1;
        }
        let day_off = |occurrence: &Occurrence| occurrence.rule.kind.is_day_off();
        let sharing = base[start..end].iter().filter(|o| day_off(o)).count();
        let mut owed = sharing.saturating_sub(1);
        if let Ok(year) = gregorian::year_from_fixed(date) {
            for index in (start..end).rev() {
                let occurrence = &base[index];
                if owed > 0 && day_off(occurrence) && occurrence.rule.substitutes_in(year) {
                    collided[index] = true;
                    owed -= 1;
                }
            }
        }
        end = start;
    }
    collided
}

/// Where a substitution lands.
fn substitute_day(
    date: Rd,
    policy: &SubstitutionPolicy,
    trigger: &[Weekday],
    occupied: &[Rd],
) -> Option<Rd> {
    match policy.direction {
        SubstituteDirection::Nearest => {
            // Saturday moves back, Sunday moves forward; anything else in the
            // trigger set has no "nearer" side and is left alone.
            match Weekday::from_rd(date) {
                Weekday::Saturday => Some(Rd(date.0 - 1)),
                Weekday::Sunday => Some(Rd(date.0 + 1)),
                _ => None,
            }
        }
        SubstituteDirection::Forward | SubstituteDirection::Backward => {
            let step = if policy.direction == SubstituteDirection::Forward {
                1
            } else {
                -1
            };
            let mut cursor = date;
            // Thirty steps is far more than any real law needs and bounds
            // the loop against a table that triggers on every weekday.
            for _ in 0..30 {
                cursor = Rd(cursor.0 + step);
                if trigger.contains(&Weekday::from_rd(cursor)) {
                    continue;
                }
                if occupied.binary_search(&cursor).is_ok() {
                    if policy.skip_occupied {
                        continue;
                    }
                    // Japan's pre-2007 rule named one day and stopped: when
                    // that day was already a 祝日, no extra day was created.
                    return None;
                }
                return Some(cursor);
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::{Rule, SourceDate, WeekendPolicy};

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    static SIMPLE_RULES: [HolidayRule; 2] = [
        HolidayRule::public(
            "New Year's Day",
            "",
            Rule::FixedGregorian { month: 1, day: 1 },
        ),
        HolidayRule::fixed_public("Anniversary", "", Rule::FixedGregorian { month: 7, day: 4 }),
    ];

    static FORWARD: [SubstitutionPolicy; 1] = [SubstitutionPolicy {
        trigger: &[Weekday::Saturday, Weekday::Sunday],
        direction: SubstituteDirection::Forward,
        skip_occupied: true,
        on_collision: false,
        valid_from: None,
        valid_until: None,
    }];

    static SIMPLE: RuleSet = RuleSet {
        code: "XX",
        english_name: "Test",
        rules: &SIMPLE_RULES,
        substitution: &FORWARD,
        bridges: &[],
        includes: &[],
        weekend: crate::rule::SATURDAY_SUNDAY,
        sources_checked: SourceDate::new(2026, 9, 21),
        sources: "invented for the test suite",
    };

    static FRIDAY_SATURDAY: [WeekendPolicy; 1] = [WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: None,
        valid_until: None,
    }];

    static GULF: RuleSet = RuleSet {
        weekend: &FRIDAY_SATURDAY,
        substitution: &[],
        ..SIMPLE
    };

    #[test]
    fn a_holiday_on_a_sunday_moves_to_the_monday() {
        // 1 January 2023 was a Sunday.
        let calendar = HolidayCalendar::for_year(&SIMPLE, None, 2023);
        assert!(calendar.is_holiday(ymd(2023, 1, 1)));
        assert!(calendar.is_holiday(ymd(2023, 1, 2)));
        let substitute = calendar
            .on(ymd(2023, 1, 2))
            .into_iter()
            .find(Holiday::is_substitute)
            .expect("a substitute");
        assert_eq!(substitute.observed_for, Some(ymd(2023, 1, 1)));
    }

    #[test]
    fn a_rule_that_opts_out_of_substitution_stays_on_the_weekend() {
        // 4 July 2026 is a Saturday and the test rule never substitutes.
        let calendar = HolidayCalendar::for_year(&SIMPLE, None, 2026);
        assert!(calendar.is_holiday(ymd(2026, 7, 4)));
        assert!(!calendar.is_holiday(ymd(2026, 7, 6)));
    }

    #[test]
    fn business_day_arithmetic_steps_over_a_holiday() {
        // Friday 30 December 2022, plus one business day, is Tuesday
        // 3 January 2023: Monday is New Year's Day.
        let calendar = HolidayCalendar::new(&SIMPLE, None, 2022, 2023);
        assert_eq!(
            calendar.add_business_days(ymd(2022, 12, 30), 1),
            Some(ymd(2023, 1, 3))
        );
    }

    #[test]
    fn business_day_arithmetic_honours_a_friday_saturday_weekend() {
        // 2026-03-04 is a Wednesday. Under a Friday–Saturday weekend the
        // next three business days are Thursday, Sunday and Monday.
        let calendar = HolidayCalendar::new(&GULF, None, 2026, 2026);
        assert_eq!(
            calendar.add_business_days(ymd(2026, 3, 4), 3),
            Some(ymd(2026, 3, 9))
        );
        assert!(!calendar.is_business_day(ymd(2026, 3, 6)));
        assert!(calendar.is_business_day(ymd(2026, 3, 8)));
    }

    #[test]
    fn business_days_between_is_additive_and_signed() {
        let calendar = HolidayCalendar::new(&SIMPLE, None, 2024, 2024);
        let monday = ymd(2024, 3, 4);
        let wednesday = ymd(2024, 3, 6);
        let friday = ymd(2024, 3, 8);
        let first = calendar
            .business_days_between(monday, wednesday)
            .expect("in range");
        let second = calendar
            .business_days_between(wednesday, friday)
            .expect("in range");
        let whole = calendar
            .business_days_between(monday, friday)
            .expect("in range");
        assert_eq!(first + second, whole);
        assert_eq!(whole, 4);
        assert_eq!(calendar.business_days_between(friday, monday), Some(-4));
    }

    #[test]
    fn a_query_outside_the_evaluated_span_refuses_rather_than_guessing() {
        let calendar = HolidayCalendar::for_year(&SIMPLE, None, 2024);
        assert_eq!(calendar.add_business_days(ymd(2030, 1, 2), 1), None);
        assert_eq!(
            calendar.business_days_between(ymd(2024, 1, 2), ymd(2030, 1, 2)),
            None
        );
        assert!(!calendar.covers(ymd(2025, 1, 1)));
    }

    #[test]
    fn next_and_previous_holiday_skip_non_holidays() {
        let calendar = HolidayCalendar::new(&SIMPLE, None, 2024, 2025);
        let next = calendar.next_holiday(ymd(2024, 3, 1)).expect("a holiday");
        assert_eq!(next.date, ymd(2024, 7, 4));
        let previous = calendar
            .previous_holiday(ymd(2024, 3, 1))
            .expect("a holiday");
        assert_eq!(previous.date, ymd(2024, 1, 1));
    }
}
