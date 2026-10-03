//! The evaluator, and business-day arithmetic.
//!
//! One function walks a [`RuleSet`], and every country and tradition in the
//! crate goes through it. There is no per-country code anywhere below this
//! line, and adding a country adds no branch to anything here.
//!
//! # The order the modifiers apply in
//!
//! 1. **Base rules.** Every rule valid in the year and in the requested
//!    region is evaluated, over every year within `REACH_DAYS` of the
//!    span asked for, because a substitution or a bridge can reach across
//!    1 January. For a whole year that is the year and both its
//!    neighbours; for one day it is usually the day's year alone.
//! 2. **Substitution.** Days are taken in date order so that a substitute
//!    can be pushed past a substitute already assigned — which is exactly
//!    what happens when Christmas Day falls on a Saturday in the United
//!    Kingdom.
//! 3. **Bridges.** Japan's 国民の休日 is evaluated against the *base*
//!    holidays, because the statute says the neighbouring days must be
//!    国民の祝日 and a 振替休日 is not one.
//! 4. **Clipping** to the requested days.

use alloc::{vec, vec::Vec};

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::group::Group;
use crate::id::HolidayId;
use crate::rule::{
    Confidence, EvaluationContext, HolidayRule, Kind, RuleSet, Scope, SubstituteDirection,
    SubstitutionPolicy, UNREAD_INCLUDED, UNREAD_SUBDIVISION, UNREAD_WEEKEND, Window,
};

/// The identifier of the gap the engine reports for a subdivision whose
/// own days no source read gives, [`UNREAD_SUBDIVISION`]'s.
pub const UNREAD_SUBDIVISION_ID: HolidayId = HolidayId::explicit("unread-subdivision");

/// The identifier of the gap the engine reports for a year whose weekend law
/// in the region was not read, [`UNREAD_WEEKEND`]'s (ADR 0015).
pub const UNREAD_WEEKEND_ID: HolidayId = HolidayId::explicit("unread-weekend");

/// The identifier of the gap the engine reports for a year before an
/// inclusion is read ([`crate::rule::Include::read_from`]).
pub const UNREAD_INCLUDED_ID: HolidayId = HolidayId::explicit("unread-included-holidays");

/// One holiday on one day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Holiday {
    /// The day it falls on.
    pub date: Rd,
    /// The holiday's identifier within its table, [`HolidayRule::id`]:
    /// what a line is joined on, one to a holiday.
    pub id: HolidayId,
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
    /// The groups it is given to alone; empty means everyone.
    pub groups: &'static [Group],
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
/// has no answer outside that calendar's range, and the holiday list alone
/// cannot say so: `holidays_in_year(&CHINA, None, 2151)` gives seven
/// entries instead of thirteen, with 春節, 端午節 and 中秋節 missing and
/// everything that remains marked [`Confidence::Exact`].
///
/// A calendar's range is one cause of a gap. A [`Rule::Listed`] or
/// [`Rule::Tabulated`] rule past the last year its published table covers
/// is another, and a
/// [`Rule::Unsettled`] rule in a year its source leaves open, as *Common
/// Worship*'s Rules do for some Easters, is a third. A year before a rule's
/// [`HolidayRule::read_from`], the first its sources were read for, is a
/// fourth, and a subdivision the table's sources were not read for, with
/// the name [`UNREAD_SUBDIVISION`], a fifth (ADR 0013).
///
/// Policy §4 says the library refuses rather than guesses. Omitting a
/// holiday silently is a guess — that it did not happen — so the calendar
/// records each one it could not compute as a gap.
///
/// [`Rule::Listed`]: crate::rule::Rule::Listed
/// [`Rule::Tabulated`]: crate::rule::Rule::Tabulated
/// [`Rule::Unsettled`]: crate::rule::Rule::Unsettled
/// [`HolidayRule::read_from`]: crate::rule::HolidayRule::read_from
/// [`UNREAD_SUBDIVISION`]: crate::rule::UNREAD_SUBDIVISION
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gap {
    /// The Gregorian year that could not be answered.
    pub year: i64,
    /// The holiday's identifier, [`HolidayRule::id`], as its entry would
    /// carry it; [`UNREAD_SUBDIVISION_ID`] for a subdivision not read.
    pub id: HolidayId,
    /// The English name of the holiday.
    pub name: &'static str,
    /// What sort of day the holiday is, as its entry would be, so that a
    /// caller asking for one kind can keep the gaps of that kind alone;
    /// `Kind::Public` for a subdivision not read, whose days are unknown.
    pub kind: Kind,
    /// Its name in the local language, or `""`.
    pub local_name: &'static str,
    /// The instrument the rule cites, which for a year before its
    /// `read_from` says what was and was not read; `""` where the table's
    /// `sources` speaks for it.
    pub source: &'static str,
    /// The first and the last day the unplaced holiday could fall on, both
    /// included, or `None` where nothing narrows it and any day of the year
    /// could be it ([`Gap::could_fall_on`]).
    ///
    /// A rule whose shape the year can compute gives the days it would
    /// place, widened by the days a substitution can move one; a table of
    /// announced dates that has run out gives the span of dates the day
    /// took in the years it lists. A subdivision not read, a weekend not
    /// read, a rule whose calendar ended and a day no source dates have no
    /// window.
    pub window: Option<(Rd, Rd)>,
}

impl Gap {
    /// Whether the holiday the gap stands for could fall on `day`: the day
    /// lies in its window, or, with no window, in its year.
    #[must_use]
    pub fn could_fall_on(&self, day: Rd) -> bool {
        match self.window {
            Some((first, last)) => first <= day && day <= last,
            None => gregorian::year_from_fixed(day).is_ok_and(|year| year == self.year),
        }
    }

    /// Whether the holiday the gap stands for could fall on a day of
    /// `first..=last`.
    #[must_use]
    pub fn could_fall_between(&self, first: Rd, last: Rd) -> bool {
        if last < first {
            return false;
        }
        let (open_first, open_last) = self.window.unwrap_or_else(|| {
            let start = gregorian::to_fixed(self.year, 1, 1).unwrap_or(Rd(i64::MIN));
            let end = gregorian::to_fixed(self.year, 12, 31).unwrap_or(Rd(i64::MAX));
            (start, end)
        });
        open_first <= last && first <= open_last
    }
}

/// Why a question about a day, or about the days after it, has no answer
/// from a calendar: the three ways it can be open (ADR 0013, ADR 0015).
///
/// A calendar that answered `false` for a day a [`Gap`] leaves open would
/// be guessing that the holiday it could not place did not fall there, and
/// policy §4 says the library refuses rather than guesses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unanswered {
    /// The day lies outside the span the calendar evaluated.
    OutsideSpan,
    /// The weekend law in force on the day in the calendar's region was not
    /// read ([`UNREAD_WEEKEND`]).
    UnreadWeekend,
    /// A gap leaves the answer open: a holiday of a kind the question is
    /// about that the calendar could not place in the day's year, or a
    /// subdivision whose own days were not read ([`Gap`]).
    Gap,
}

/// An evaluated rule set over a span of years.
///
/// Building one is the expensive part — solar terms and lunar conjunctions
/// are astronomy — so queries that touch many days, above all business-day
/// arithmetic, should build a calendar once and ask it repeatedly, and a
/// caller building many calendars should pass one [`EvaluationContext`]
/// through the `_with` constructors so that the astronomy is done once.
#[derive(Debug, Clone)]
pub struct HolidayCalendar<'a> {
    rules: &'a RuleSet,
    scope: Scope<'a>,
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
        Self::new_with(
            rules,
            region,
            first_year,
            last_year,
            &mut EvaluationContext::new(),
        )
    }

    /// [`HolidayCalendar::new`] with the astronomy memoised in `context`,
    /// which a caller building several calendars shares between them.
    #[must_use]
    pub fn new_with(
        rules: &'a RuleSet,
        region: Option<&'a str>,
        first_year: i64,
        last_year: i64,
        context: &mut EvaluationContext,
    ) -> Self {
        Self::scoped_with(rules, Scope::from(region), first_year, last_year, context)
    }

    /// Evaluate `rules` over `first_year..=last_year` in a [`Scope`]: a
    /// subdivision, a group of people, both or neither.
    ///
    /// [`HolidayCalendar::new`] is this with no group. A group adds the
    /// rules given to it alone — China's 妇女节 half day for `women` —
    /// and no group asks for everyone's days, not the union of every
    /// group's.
    #[must_use]
    pub fn scoped(rules: &'a RuleSet, scope: Scope<'a>, first_year: i64, last_year: i64) -> Self {
        Self::scoped_with(
            rules,
            scope,
            first_year,
            last_year,
            &mut EvaluationContext::new(),
        )
    }

    /// [`HolidayCalendar::scoped`] with the astronomy memoised in
    /// `context`.
    #[must_use]
    pub fn scoped_with(
        rules: &'a RuleSet,
        scope: Scope<'a>,
        first_year: i64,
        last_year: i64,
        context: &mut EvaluationContext,
    ) -> Self {
        let first_day = gregorian::to_fixed(first_year, 1, 1).unwrap_or(Rd(0));
        let last_day = gregorian::to_fixed(last_year, 12, 31).unwrap_or(Rd(-1));
        let (holidays, gaps) = if first_year > last_year {
            (Vec::new(), Vec::new())
        } else {
            evaluate_with_includes(rules, scope, first_day, last_day, 0, context)
        };
        Self {
            rules,
            scope,
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

    /// Evaluate one year in a [`Scope`].
    #[must_use]
    pub fn for_year_scoped(rules: &'a RuleSet, scope: Scope<'a>, year: i64) -> Self {
        Self::scoped(rules, scope, year, year)
    }

    /// [`HolidayCalendar::for_year`] with the astronomy memoised in
    /// `context`.
    #[must_use]
    pub fn for_year_with(
        rules: &'a RuleSet,
        region: Option<&'a str>,
        year: i64,
        context: &mut EvaluationContext,
    ) -> Self {
        Self::new_with(rules, region, year, year, context)
    }

    /// Evaluate one day.
    ///
    /// The result is what [`HolidayCalendar::for_year`] for the day's year
    /// would say about the day — the same holidays from
    /// [`HolidayCalendar::on`], the same [`HolidayCalendar::gaps`] — at a
    /// fraction of the cost, because only the calendar years that can reach
    /// the day are evaluated rather than the Gregorian year and both its
    /// neighbours. A page that asks "what is today, in every table" asks
    /// this once for every table, which is why it exists — and why
    /// [`HolidayCalendar::for_day_with`] exists, so that the tables share
    /// one [`EvaluationContext`].
    ///
    /// [`HolidayCalendar::covers`] is true for the day alone, so
    /// business-day arithmetic on the result answers `None` for every other
    /// day.
    #[must_use]
    pub fn for_day(rules: &'a RuleSet, region: Option<&'a str>, day: Rd) -> Self {
        Self::for_day_with(rules, region, day, &mut EvaluationContext::new())
    }

    /// [`HolidayCalendar::for_day`] with the astronomy memoised in
    /// `context`.
    ///
    /// The tables that date by the same sky ask the same questions of it —
    /// every Hindu table the same tithis, every Chinese-dated table the
    /// same new moons — so a caller evaluating many tables for one day
    /// builds one context and passes it to each.
    ///
    /// The evaluation runs inside an [`hc_core::memo::scope`], which
    /// memoises what the context does not see: the solstices and new moons
    /// a lunisolar calendar's conversions search for, asked again for every
    /// date the rules convert, and the conversions of rules computed by a
    /// function, such as the Japanese 旧暦 days. A caller evaluating many
    /// tables opens one scope around them all, so that every table shares
    /// it, as `hc_holidays_on` does; the scope changes how long the call
    /// takes and nothing else.
    #[must_use]
    pub fn for_day_with(
        rules: &'a RuleSet,
        region: Option<&'a str>,
        day: Rd,
        context: &mut EvaluationContext,
    ) -> Self {
        Self::for_day_scoped_with(rules, Scope::from(region), day, context)
    }

    /// Evaluate one day in a [`Scope`], as [`HolidayCalendar::for_day`]
    /// does in a region.
    #[must_use]
    pub fn for_day_scoped(rules: &'a RuleSet, scope: Scope<'a>, day: Rd) -> Self {
        Self::for_day_scoped_with(rules, scope, day, &mut EvaluationContext::new())
    }

    /// [`HolidayCalendar::for_day_scoped`] with the astronomy memoised in
    /// `context`.
    #[must_use]
    pub fn for_day_scoped_with(
        rules: &'a RuleSet,
        scope: Scope<'a>,
        day: Rd,
        context: &mut EvaluationContext,
    ) -> Self {
        let year = gregorian::year_from_fixed(day).unwrap_or(0);
        let (holidays, gaps) =
            hc_core::memo::scope(|| evaluate_with_includes(rules, scope, day, day, 0, context));
        Self {
            rules,
            scope,
            first_year: year,
            last_year: year,
            first_day: day,
            last_day: day,
            holidays,
            gaps,
        }
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

    /// The gaps of the holiday list alone: [`HolidayCalendar::gaps`] without
    /// the year's unread weekend law ([`UNREAD_WEEKEND`]).
    pub fn holiday_gaps(&self) -> impl Iterator<Item = &Gap> {
        self.gaps.iter().filter(|gap| gap.id != UNREAD_WEEKEND_ID)
    }

    /// Whether every rule could be answered for every year asked for: no gap
    /// in the holiday list. A year whose weekend law was not read
    /// ([`UNREAD_WEEKEND`]) is a gap of [`HolidayCalendar::gaps`] that does
    /// not make the list of holidays incomplete;
    /// [`HolidayCalendar::weekend_is_read`] says whether a day's weekend law
    /// was read.
    ///
    /// A `false` here does not mean the answers given are wrong; it means
    /// some are missing. See [`HolidayCalendar::gaps`] for which.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.gaps.iter().all(|gap| gap.id == UNREAD_WEEKEND_ID)
    }

    /// The rule set behind this calendar.
    #[must_use]
    pub const fn rule_set(&self) -> &'a RuleSet {
        self.rules
    }

    /// The subdivision this calendar was built for.
    #[must_use]
    pub const fn region(&self) -> Option<&'a str> {
        self.scope.region
    }

    /// The group this calendar was built for.
    #[must_use]
    pub const fn group(&self) -> Option<&'a str> {
        self.scope.group
    }

    /// The region and the group this calendar was built for.
    #[must_use]
    pub const fn scope(&self) -> Scope<'a> {
        self.scope
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
    /// that day in the calendar's region.
    ///
    /// A region may keep a weekend of its own: Kedah's is Friday and
    /// Saturday, the rest of Malaysia's Saturday and Sunday (ADR 0015). A
    /// day on which the region's weekend law was not read is not a weekend
    /// day here, and [`HolidayCalendar::weekend_is_read`] says so.
    #[must_use]
    pub fn is_weekend(&self, day: Rd) -> bool {
        self.rules
            .weekend_in(self.scope.region, day)
            .is_some_and(|days| days.contains(&Weekday::from_rd(day)))
    }

    /// Whether the weekend law in force on `day` in the calendar's region
    /// was read. Where it was not, [`HolidayCalendar::is_weekend`] says no
    /// and the business-day arithmetic refuses.
    #[must_use]
    pub fn weekend_is_read(&self, day: Rd) -> bool {
        self.rules.weekend_in(self.scope.region, day).is_some()
    }

    /// Whether `day` is a weekend day the calendar makes a working day — a
    /// [`Kind::Workday`] entry, China's 调休上班.
    #[must_use]
    pub fn is_designated_workday(&self, day: Rd) -> bool {
        self.holidays
            .iter()
            .any(|holiday| holiday.date == day && holiday.kind == Kind::Workday)
    }

    /// Whether `day` is a working day: not a holiday, and either not the
    /// weekend or a weekend day the calendar makes a working day.
    #[must_use]
    pub fn is_business_day(&self, day: Rd) -> bool {
        !self.is_holiday(day) && (!self.is_weekend(day) || self.is_designated_workday(day))
    }

    /// Whether a gap leaves `day` open: a holiday of a kind `wanted` says
    /// that the calendar could not place and that could fall on `day`
    /// ([`Gap::could_fall_on`]), or, whatever the kind, a subdivision
    /// whose own days were not read, as the year's lines write it. The gap
    /// for an unread weekend law is not counted here, because
    /// [`HolidayCalendar::weekend_is_read`] says whether the law of the day
    /// itself was read.
    fn gap_on(&self, day: Rd, wanted: impl Fn(Kind) -> bool) -> bool {
        self.gaps.iter().any(|gap| {
            gap.id != UNREAD_WEEKEND_ID
                && gap.could_fall_on(day)
                && (gap.id == UNREAD_SUBDIVISION_ID || wanted(gap.kind))
        })
    }

    /// Whether `day` is a day off, or why that is not known.
    ///
    /// A day with a day-off entry is a day off whatever else is open. A day
    /// without one is a day that is not, unless the weekend law in force on
    /// it was not read ([`Unanswered::UnreadWeekend`]), or a gap of a kind
    /// that stops work lies in its year ([`Unanswered::Gap`]): the holiday
    /// the table could not place may be this day.
    /// [`HolidayCalendar::is_holiday`] is the entries alone.
    ///
    /// # Errors
    ///
    /// [`Unanswered::OutsideSpan`] for a day the calendar did not evaluate;
    /// otherwise as above.
    pub fn day_off(&self, day: Rd) -> Result<bool, Unanswered> {
        if !self.covers(day) {
            return Err(Unanswered::OutsideSpan);
        }
        if self.is_holiday(day) {
            return Ok(true);
        }
        if !self.weekend_is_read(day) {
            return Err(Unanswered::UnreadWeekend);
        }
        if self.gap_on(day, Kind::is_day_off) {
            return Err(Unanswered::Gap);
        }
        Ok(false)
    }

    /// Whether `day` is a business day, or why that is not known.
    ///
    /// As [`HolidayCalendar::is_business_day`] where the entries and the
    /// weekend decide. A weekday that is not a holiday is open when a gap
    /// of a kind that stops work lies in its year, and a weekend day the
    /// calendar does not make a working day is open when a gap of the kind
    /// that makes one ([`Kind::Workday`]) does; a day whose weekend law was
    /// not read is open whatever else is.
    ///
    /// # Errors
    ///
    /// As [`HolidayCalendar::day_off`].
    pub fn business_day(&self, day: Rd) -> Result<bool, Unanswered> {
        if !self.covers(day) {
            return Err(Unanswered::OutsideSpan);
        }
        if self.is_holiday(day) {
            return Ok(false);
        }
        if !self.weekend_is_read(day) {
            return Err(Unanswered::UnreadWeekend);
        }
        if self.is_weekend(day) {
            if self.is_designated_workday(day) {
                return Ok(true);
            }
            if self.gap_on(day, |kind| kind == Kind::Workday) {
                return Err(Unanswered::Gap);
            }
            return Ok(false);
        }
        if self.gap_on(day, Kind::is_day_off) {
            return Err(Unanswered::Gap);
        }
        Ok(true)
    }

    /// `day` moved by `count` business days.
    ///
    /// A positive `count` moves forward, a negative one back, and zero
    /// returns `day` unchanged whether or not it is itself a business day.
    /// The starting day is never counted; the result is always a business
    /// day when `count` is non-zero.
    ///
    /// Returns `None` when the walk leaves the evaluated span, because a
    /// calendar cannot honestly answer for a year it has not evaluated,
    /// reaches a day whose weekend law in the region was not read, or
    /// reaches a day a gap leaves open: a day the walk would count, or skip,
    /// on a guess. [`HolidayCalendar::try_add_business_days`] says which.
    #[must_use]
    pub fn add_business_days(&self, day: Rd, count: i64) -> Option<Rd> {
        self.try_add_business_days(day, count).ok()
    }

    /// [`HolidayCalendar::add_business_days`], saying why there is no
    /// answer.
    ///
    /// # Errors
    ///
    /// [`Unanswered::OutsideSpan`] for a starting day or a walk that leaves
    /// the span, [`Unanswered::UnreadWeekend`] for a walk that reaches a
    /// day whose weekend law was not read, and [`Unanswered::Gap`] for one
    /// that reaches a day a gap leaves open
    /// ([`HolidayCalendar::business_day`]).
    pub fn try_add_business_days(&self, day: Rd, count: i64) -> Result<Rd, Unanswered> {
        if !self.covers(day) {
            return Err(Unanswered::OutsideSpan);
        }
        if count == 0 {
            return Ok(day);
        }
        let step = if count > 0 { 1 } else { -1 };
        let mut remaining = count.abs();
        let mut cursor = day;
        while remaining > 0 {
            cursor = Rd(cursor.0 + step);
            if self.business_day(cursor)? {
                remaining -= 1;
            }
        }
        Ok(cursor)
    }

    /// The number of business days in the half-open interval
    /// `[start, end)`.
    ///
    /// Half-open because that is the interval that composes: the count from
    /// Monday to Wednesday plus the count from Wednesday to Friday is the
    /// count from Monday to Friday. `end` before `start` gives a negative
    /// count.
    ///
    /// Returns `None` when either end lies outside the evaluated span, or
    /// a day of the interval has a weekend law the region's sources did not
    /// read or is one a gap leaves open.
    /// [`HolidayCalendar::try_business_days_between`] says which.
    #[must_use]
    pub fn business_days_between(&self, start: Rd, end: Rd) -> Option<i64> {
        self.try_business_days_between(start, end).ok()
    }

    /// [`HolidayCalendar::business_days_between`], saying why there is no
    /// answer.
    ///
    /// # Errors
    ///
    /// As [`HolidayCalendar::try_add_business_days`], for the days of the
    /// interval.
    pub fn try_business_days_between(&self, start: Rd, end: Rd) -> Result<i64, Unanswered> {
        if !self.covers(start) || !self.covers(end) {
            return Err(Unanswered::OutsideSpan);
        }
        if start == end {
            return Ok(0);
        }
        let (from, to, sign) = if start < end {
            (start, end, 1)
        } else {
            (end, start, -1)
        };
        let mut count = 0i64;
        let mut cursor = from;
        while cursor < to {
            if self.business_day(cursor)? {
                count += 1;
            }
            cursor = Rd(cursor.0 + 1);
        }
        Ok(count * sign)
    }

    /// Whether a holiday of one of `kinds` could be missing from the days
    /// `first..=last`: a gap of such a kind could fall on one of them, or,
    /// whatever the kinds, a subdivision's own days or a weekend law were
    /// not read in a year of them. `kinds` empty means the kinds that stop
    /// work.
    fn gap_between(&self, first: Rd, last: Rd, kinds: &[Kind]) -> bool {
        self.gaps.iter().any(|gap| {
            gap.could_fall_between(first, last)
                && (gap.id == UNREAD_SUBDIVISION_ID
                    || gap.id == UNREAD_WEEKEND_ID
                    || Self::kind_wanted(kinds, gap.kind))
        })
    }

    /// Whether `kind` is one of `kinds`, or a kind that stops work when
    /// `kinds` is empty.
    fn kind_wanted(kinds: &[Kind], kind: Kind) -> bool {
        if kinds.is_empty() {
            kind.is_day_off()
        } else {
            kinds.contains(&kind)
        }
    }

    /// The next entry strictly after `day` of one of `kinds`, or of a kind
    /// that stops work when `kinds` is empty, as
    /// [`HolidayCalendar::next_holiday`] finds it, and refused when a gap
    /// could hide an earlier one.
    ///
    /// The span ends the search: `Ok(None)` says there is none by the end
    /// of it, so a caller who wants a longer reach builds a longer
    /// calendar. A gap of a wanted kind in any year from `day`'s to the
    /// found entry's, or to the last year of the span when there is none, is
    /// [`Unanswered::Gap`]: the holiday the table could not place may be the
    /// nearer one.
    ///
    /// # Errors
    ///
    /// [`Unanswered::OutsideSpan`] for a `day` the calendar did not
    /// evaluate, and [`Unanswered::Gap`] as above.
    pub fn try_next_of(&self, day: Rd, kinds: &[Kind]) -> Result<Option<Holiday>, Unanswered> {
        if !self.covers(day) {
            return Err(Unanswered::OutsideSpan);
        }
        let found = self
            .holidays
            .iter()
            .find(|holiday| holiday.date > day && Self::kind_wanted(kinds, holiday.kind))
            .copied();
        // The days that could hold a nearer holiday: those after `day` up
        // to the one found, or to the end of the span when there is none.
        let end = found.map_or(self.last_day, |holiday| Rd(holiday.date.0 - 1));
        if self.gap_between(Rd(day.0 + 1), end, kinds) {
            return Err(Unanswered::Gap);
        }
        Ok(found)
    }

    /// The last entry strictly before `day` of one of `kinds`, as
    /// [`HolidayCalendar::try_next_of`] finds the next.
    ///
    /// # Errors
    ///
    /// As [`HolidayCalendar::try_next_of`], for the years back from `day`'s.
    pub fn try_previous_of(&self, day: Rd, kinds: &[Kind]) -> Result<Option<Holiday>, Unanswered> {
        if !self.covers(day) {
            return Err(Unanswered::OutsideSpan);
        }
        let found = self
            .holidays
            .iter()
            .rev()
            .find(|holiday| holiday.date < day && Self::kind_wanted(kinds, holiday.kind))
            .copied();
        let start = found.map_or(self.first_day, |holiday| Rd(holiday.date.0 + 1));
        if self.gap_between(start, Rd(day.0 - 1), kinds) {
            return Err(Unanswered::Gap);
        }
        Ok(found)
    }
}

/// Convenience: every holiday of one year for one rule set.
#[must_use]
pub fn holidays_in_year(rules: &RuleSet, region: Option<&str>, year: i64) -> Vec<Holiday> {
    HolidayCalendar::for_year(rules, region, year)
        .all()
        .to_vec()
}

/// Convenience: every entry falling on one day, including observances
/// with no day off.
#[must_use]
pub fn holidays_on(rules: &RuleSet, region: Option<&str>, day: Rd) -> Vec<Holiday> {
    HolidayCalendar::for_day(rules, region, day).on(day)
}

/// Convenience: whether one day is a day-off holiday.
#[must_use]
pub fn is_holiday(rules: &RuleSet, region: Option<&str>, day: Rd) -> bool {
    HolidayCalendar::for_day(rules, region, day).is_holiday(day)
}

/// An occurrence before the modifiers have been applied.
#[derive(Debug, Clone, Copy)]
struct Occurrence {
    date: Rd,
    rule: &'static HolidayRule,
}

/// How many levels of [`RuleSet::includes`] the engine follows.
const INCLUDE_DEPTH: u8 = 8;

/// How far a modifier can reach from the occurrence it modifies, in days.
///
/// A substitute is found within thirty steps of its holiday
/// ([`substitute_day`]), a bridge sits one day from each neighbour, and a
/// substitute already assigned can push a later one along. So the base
/// rules are evaluated this far outside the span asked for, and the
/// modifiers inside the span see every occurrence that can touch them.
/// Sixty-two days is twice the longest reach any real chain has needed;
/// `tests/on_day.rs` holds every table to it. For a whole year it evaluates
/// the year and both its neighbours, as the engine always did.
const REACH_DAYS: i64 = 62;

/// How many days a substitution or a bridge moves a holiday at most, as a
/// gap's window counts it ([`Gap::window`]).
const GAP_MARGIN_DAYS: i64 = 7;

/// A set's own holidays and gaps, with those of every set it includes,
/// each evaluated under its own policies, merged in date order.
fn evaluate_with_includes(
    rules: &RuleSet,
    scope: Scope<'_>,
    first_day: Rd,
    last_day: Rd,
    depth: u8,
    context: &mut EvaluationContext,
) -> (Vec<Holiday>, Vec<Gap>) {
    let (mut holidays, mut gaps) = evaluate(rules, scope, first_day, last_day, context);
    if depth >= INCLUDE_DEPTH {
        return (holidays, gaps);
    }
    for included in rules.includes {
        let (more, more_gaps) = evaluate_with_includes(
            included.set,
            // The region is the inclusion's, and the group no one's: an
            // exchange closes on its country's days for everyone.
            Scope::from(included.region),
            first_day,
            last_day,
            depth + 1,
            context,
        );
        // Only the days off: an included set's commemorations are its own.
        // Hong Kong keeps the Winter Solstice as an observance, and the
        // exchange that closes on Hong Kong's holidays trades through it.
        let (more, more_gaps) = match included.read_from {
            Some(first) => {
                let first = i64::from(first);
                let year_of = |day: Rd| gregorian::year_from_fixed(day).unwrap_or(i64::MIN);
                let mut kept_gaps: Vec<Gap> = more_gaps
                    .into_iter()
                    .filter(|gap| gap.year >= first)
                    .collect();
                // A year before the inclusion was read is a gap of its own,
                // for each year of the span asked for.
                for year in year_of(first_day).max(i64::MIN + 1)..=year_of(last_day) {
                    if year < first {
                        kept_gaps.push(Gap {
                            year,
                            id: UNREAD_INCLUDED_ID,
                            name: UNREAD_INCLUDED,
                            kind: Kind::Public,
                            local_name: "",
                            source: "",
                            window: None,
                        });
                    }
                }
                let kept: Vec<Holiday> = more
                    .into_iter()
                    .filter(|holiday| year_of(holiday.date) >= first)
                    .collect();
                (kept, kept_gaps)
            }
            None => (more, more_gaps),
        };
        holidays.extend(more.into_iter().filter(Holiday::is_day_off));
        gaps.extend(more_gaps);
    }
    if !rules.includes.is_empty() {
        holidays.sort_by_key(|holiday| holiday.date);
        dedup_gaps(&mut gaps);
    }
    (holidays, gaps)
}

/// Keep the first gap of each year and identifier.
fn dedup_gaps(gaps: &mut Vec<Gap>) {
    let mut seen: Vec<(i64, HolidayId)> = Vec::with_capacity(gaps.len());
    gaps.retain(|gap| {
        if seen
            .iter()
            .any(|&(year, id)| year == gap.year && id == gap.id)
        {
            return false;
        }
        seen.push((gap.year, gap.id));
        true
    });
}

/// Evaluate a rule set over `first_day..=last_day`, apply every modifier,
/// and clip to those days.
fn evaluate(
    rules: &RuleSet,
    scope: Scope<'_>,
    first_day: Rd,
    last_day: Rd,
    context: &mut EvaluationContext,
) -> (Vec<Holiday>, Vec<Gap>) {
    // The years asked about, and the years the base rules are evaluated
    // over: [`REACH_DAYS`] of slack each side, because a substitution can
    // push 31 December into January and a bridge can sit either side of
    // New Year's Day.
    let (Ok(first_year), Ok(last_year)) = (
        gregorian::year_from_fixed(first_day),
        gregorian::year_from_fixed(last_day),
    ) else {
        return (Vec::new(), Vec::new());
    };
    let (Ok(first_reached), Ok(last_reached)) = (
        gregorian::year_from_fixed(Rd(first_day.0 - REACH_DAYS)),
        gregorian::year_from_fixed(Rd(last_day.0 + REACH_DAYS)),
    ) else {
        return (Vec::new(), Vec::new());
    };
    // The base rules are asked for whole years, but only the days within
    // reach of the span can touch the answer, and a rule may skip the
    // astronomy of anything it can prove lies outside them.
    let window = Window {
        first: Rd(first_day.0 - REACH_DAYS),
        last: Rd(last_day.0 + REACH_DAYS),
    };
    let mut base: Vec<Occurrence> = Vec::new();
    let mut gaps: Vec<Gap> = Vec::new();
    // A subdivision the table was not read for keeps the nationwide days,
    // and its own are a gap in every year asked for.
    if let Some(region) = scope.region {
        for year in first_year..=last_year {
            if rules.reads_region_in(region, year) {
                continue;
            }
            gaps.push(Gap {
                year,
                id: UNREAD_SUBDIVISION_ID,
                name: UNREAD_SUBDIVISION,
                kind: Kind::Public,
                local_name: "",
                source: "",
                window: None,
            });
        }
    }
    // A year whose weekend law the region's sources did not read is a gap
    // as well, and a day moved off a weekend in it is only as sure.
    if !rules.weekend.is_empty() {
        for year in first_year..=last_year {
            if rules.weekend_unread_in(scope.region, year) {
                gaps.push(Gap {
                    year,
                    id: UNREAD_WEEKEND_ID,
                    name: UNREAD_WEEKEND,
                    kind: Kind::Public,
                    local_name: "",
                    source: "",
                    window: None,
                });
            }
        }
    }
    // A day a substitution or a bridge may move lies a few days from the
    // rule's own, and a gap's window is widened by as many.
    let margin = if rules.substitution.is_empty() && rules.bridges.is_empty() {
        0
    } else {
        GAP_MARGIN_DAYS
    };
    for year in first_reached..=last_reached {
        for rule in rules.rules {
            // Outside its establishment and abolition the day did not
            // exist, which is an answer; every other year is either
            // answered or reported.
            if !rule.applies_in(year) || !rule.applies_in_scope(scope) {
                continue;
            }
            // Only the years actually asked for are reported as gaps; the
            // slack on each side exists to catch a substitution crossing
            // New Year, and its absence is not a hole in the answer. A year
            // the rule's sources were not read for is a gap whatever its
            // shape could compute.
            if rule.is_unread_in(year) || !rule.rule.is_resolvable_with(year, context) {
                // Two rules that share an identifier are one unknown day:
                // Quebec's table has two rules for "Good Friday or Easter
                // Monday, at the employer's choice", and a year it could
                // not answer is one gap, not two. The gaps are written year
                // by year, so only the tail can share the year.
                if (first_year..=last_year).contains(&year) {
                    let window = rule.gap_window(year, margin, context);
                    let written = gaps
                        .iter()
                        .rposition(|gap| gap.year == year && gap.id == rule.id());
                    if let Some(index) = written {
                        // One unknown day with two windows is open on both.
                        gaps[index].window = match (gaps[index].window, window) {
                            (Some((a, b)), Some((c, d))) => Some((a.min(c), b.max(d))),
                            _ => None,
                        };
                    } else {
                        gaps.push(Gap {
                            year,
                            id: rule.id(),
                            name: rule.name,
                            kind: rule.kind,
                            local_name: rule.local_name,
                            source: rule.source,
                            window,
                        });
                    }
                }
                continue;
            }
            for date in rule
                .rule
                .days_in_year_with(year, window, context)
                .as_slice()
            {
                base.push(Occurrence { date: *date, rule });
            }
        }
    }
    base.sort_by_key(|occurrence| (occurrence.date.0, occurrence.rule.name, occurrence.rule.id));
    base.dedup_by_key(|occurrence| (occurrence.date.0, occurrence.rule.name, occurrence.rule.id));

    let mut out: Vec<Holiday> = base
        .iter()
        .map(|occurrence| Holiday {
            date: occurrence.date,
            id: occurrence.rule.id(),
            name: occurrence.rule.name,
            local_name: occurrence.rule.local_name,
            kind: occurrence.rule.kind,
            confidence: occurrence.rule.confidence,
            regions: occurrence.rule.regions,
            groups: occurrence.rule.groups,
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
        let Some(policy) = rules.substitution_in_region(year, scope.region) else {
            continue;
        };
        let trigger = occurrence.rule.substitute_trigger.unwrap_or(policy.trigger);
        let triggered = trigger.contains(&Weekday::from_rd(occurrence.date))
            || (policy.on_collision && collided);
        if !triggered {
            continue;
        }
        let direction = occurrence
            .rule
            .substitute_direction
            .unwrap_or(policy.direction);
        let Some(target) = substitute_day(occurrence.date, direction, policy, trigger, &occupied)
        else {
            continue;
        };
        if let Err(index) = occupied.binary_search(&target) {
            occupied.insert(index, target);
        }
        substitutes.push(Holiday {
            date: target,
            id: occurrence.rule.id(),
            name: occurrence.rule.name,
            local_name: occurrence.rule.local_name,
            kind: occurrence.rule.kind,
            confidence: occurrence.rule.confidence,
            regions: occurrence.rule.regions,
            groups: occurrence.rule.groups,
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
        for year in first_reached..=last_reached {
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
                        id: HolidayId::of_name(policy.name),
                        name: policy.name,
                        local_name: policy.local_name,
                        kind: Kind::Public,
                        confidence: Confidence::Exact,
                        regions: &[],
                        groups: &[],
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

    out.retain(|holiday| holiday.date >= first_day && holiday.date <= last_day);
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
/// Chuseok's. The rule this models, South Korea's "다른 공휴일과 겹칠
/// 경우", and the cases it was checked against are in
/// `docs/systems/korea-holidays.md` in the repository.
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
    direction: SubstituteDirection,
    policy: &SubstitutionPolicy,
    trigger: &[Weekday],
    occupied: &[Rd],
) -> Option<Rd> {
    let step = match direction {
        SubstituteDirection::Nearest => {
            // Saturday moves back, Sunday moves forward; anything else in the
            // trigger set has no "nearer" side and is left alone.
            return match Weekday::from_rd(date) {
                Weekday::Saturday => Some(Rd(date.0 - 1)),
                Weekday::Sunday => Some(Rd(date.0 + 1)),
                _ => None,
            };
        }
        SubstituteDirection::NearestWorkingDay => match Weekday::from_rd(date) {
            Weekday::Saturday => -1,
            Weekday::Sunday => 1,
            _ => return None,
        },
        SubstituteDirection::Forward => 1,
        SubstituteDirection::Backward => -1,
    };
    let mut cursor = date;
    // Thirty steps is far more than any real law needs and bounds the loop
    // against a table that triggers on every weekday.
    for _ in 0..30 {
        cursor = Rd(cursor.0 + step);
        if trigger.contains(&Weekday::from_rd(cursor))
            || policy.avoid.contains(&Weekday::from_rd(cursor))
        {
            continue;
        }
        if occupied.binary_search(&cursor).is_ok() {
            if policy.skip_occupied {
                continue;
            }
            // Japan's pre-2007 rule named one day and stopped: when that day
            // was already a 祝日, no extra day was created.
            return None;
        }
        return Some(cursor);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::{Rule, SourceDate, Subdivisions, WeekendPolicy};

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
        regions: &[],
        avoid: &[],
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
        subdivisions: Subdivisions::Undivided,
    };

    static FRIDAY_SATURDAY: [WeekendPolicy; 1] = [WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        regions: &[],
        valid_from: None,
        valid_from_day: None,
        valid_until: None,
        valid_until_day: None,
    }];

    static GULF: RuleSet = RuleSet {
        weekend: &FRIDAY_SATURDAY,
        substitution: &[],
        ..SIMPLE
    };

    static INCLUDES_CHINA: RuleSet = RuleSet {
        rules: &[],
        substitution: &[],
        includes: &[crate::rule::Include::nationwide(&crate::countries::CHINA)],
        ..SIMPLE
    };

    #[test]
    fn a_working_weekend_day_counts_and_is_not_lent() {
        // Sunday 4 February 2024 was worked in China.
        let sunday = gregorian::to_fixed(2024, 2, 4).unwrap();
        let china = HolidayCalendar::for_year(&crate::countries::CHINA, None, 2024);
        assert!(china.is_weekend(sunday));
        assert!(china.is_designated_workday(sunday));
        assert!(china.is_business_day(sunday));
        // A table that includes China takes its days off and not the day
        // worked: the Sunday stays the weekend.
        let including = HolidayCalendar::for_year(&INCLUDES_CHINA, None, 2024);
        assert!(!including.is_business_day(sunday));
        assert!(including.is_holiday(gregorian::to_fixed(2024, 2, 13).unwrap()));
    }

    // 3 and 4 July 2026 are a Friday and a Saturday: the Saturday holiday
    // is made up the working day before the Friday one, the Thursday. A
    // Saturday 1 January 2028 with its own direction goes forward instead.
    static NEAREST_WORKING_RULES: [HolidayRule; 3] = [
        HolidayRule::fixed_public("Eve", "", Rule::FixedGregorian { month: 7, day: 3 }),
        HolidayRule::public("Day", "", Rule::FixedGregorian { month: 7, day: 4 }),
        HolidayRule::public("New Year", "", Rule::FixedGregorian { month: 1, day: 1 })
            .substitute_towards(SubstituteDirection::Forward),
    ];

    static NEAREST_WORKING: [SubstitutionPolicy; 1] = [SubstitutionPolicy {
        trigger: &[Weekday::Saturday, Weekday::Sunday],
        direction: SubstituteDirection::NearestWorkingDay,
        skip_occupied: true,
        on_collision: false,
        regions: &[],
        avoid: &[],
        valid_from: None,
        valid_until: None,
    }];

    static NEAREST_WORKING_SET: RuleSet = RuleSet {
        rules: &NEAREST_WORKING_RULES,
        substitution: &NEAREST_WORKING,
        ..SIMPLE
    };

    #[test]
    fn a_saturday_holiday_goes_back_past_a_day_already_taken() {
        let calendar = HolidayCalendar::for_year(&NEAREST_WORKING_SET, None, 2026);
        let thursday = gregorian::to_fixed(2026, 7, 2).unwrap();
        assert!(calendar.is_holiday(thursday));
        let calendar = HolidayCalendar::for_year(&NEAREST_WORKING_SET, None, 2028);
        assert!(calendar.is_holiday(gregorian::to_fixed(2028, 1, 3).unwrap()));
        assert!(!calendar.is_holiday(gregorian::to_fixed(2027, 12, 31).unwrap()));
    }

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
