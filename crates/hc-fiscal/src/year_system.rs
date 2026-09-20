//! The core type: a year that begins somewhere other than 1 January.
//!
//! A [`YearSystem`] is three things and no more:
//!
//! 1. a **start** — a month and a day *in a named calendar*, which for Iran
//!    and Ethiopia is not the Gregorian one;
//! 2. a **labelling convention** — whether the year is named after the
//!    calendar year it begins in or the one it ends in;
//! 3. a **validity range**, because countries change their minds.
//!
//! Everything else in this crate is either data expressed in these terms or
//! arithmetic derived from them.
//!
//! # Why the labelling convention is an enum with no default
//!
//! Japan's 2024年度 runs 1 April 2024 to 31 March 2025. The United States'
//! FY 2024 ran 1 October **2023** to 30 September 2024. Both are commonly
//! written "FY2024"; they share only their last six months, and on
//! 1 November 2023 the two systems disagree about what year it is by a whole
//! year. A library that picked a default here would be silently wrong for
//! roughly half the world, and a library that took a boolean would be
//! silently wrong for whoever forgot to set it.
//!
//! So, following `docs/policy.md` §5, the two conventions are named:
//! [`LabelConvention::LabelledByStartYear`] and
//! [`LabelConvention::LabelledByEndYear`]. Neither is the default because
//! [`LabelConvention`] deliberately does not implement `Default`.

use hc_calendar::{CalendarError, CalendarId, Rd, Weekday};
use hc_calendars_solar::{buddhist, ethiopic, gregorian, persian};

use crate::error::{FiscalError, FiscalResult};

/// The calendars a fiscal year's start may be expressed in.
///
/// A closed enum rather than a `dyn Calendar`, for the same reason
/// `hc_holiday::rule::CalendarSystem` is one: a country table has to be a
/// `static` value, and a trait object cannot be one without an allocator.
///
/// Three entries cover every fiscal year this crate has found a source for.
/// A fourth — the Bikram Sambat calendar Nepal's fiscal year is dated in —
/// is a [documented gap](crate::countries::GAPS), not an omission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartCalendar {
    /// The proleptic Gregorian calendar.
    Gregorian,
    /// The Solar Hijri calendar, *arithmetic* (Birashk) variant — CLDR would
    /// call the official one `persian`, and this is explicitly not it.
    ///
    /// Iran's fiscal year begins at Nowruz, which the Iranian civil code
    /// defines by the March equinox at the 52.5°E meridian rather than by
    /// any arithmetic rule. `hc-calendars-solar` ships the 2 820-year cyclic
    /// approximation under the identifier `persian-arithmetic` and leaves
    /// `persian` free for the astronomical implementation `hc-astro` will
    /// eventually supply. This crate uses the approximation and says so:
    /// see [`crate::countries::IRAN`] for what that costs.
    SolarHijriArithmetic,
    /// The Ethiopian calendar (CLDR `ethiopic`).
    Ethiopic,
    /// The Thai solar calendar (CLDR `buddhist`): Gregorian months with the
    /// year number raised by 543.
    ///
    /// Thailand's budget year is legislated in Buddhist Era years, so its
    /// labels are 2505 and 2568 rather than 1962 and 2025. Expressing that
    /// as a start *in the Thai calendar* rather than as a Gregorian start
    /// with 543 added afterwards is the difference between data and a
    /// special case: the same arithmetic then serves Bangkok, Tokyo and
    /// Tehran.
    ThaiBuddhist,
}

impl StartCalendar {
    /// The CLDR-style identifier of the underlying calendar.
    #[must_use]
    pub const fn id(self) -> CalendarId {
        match self {
            Self::Gregorian => CalendarId("gregory"),
            Self::SolarHijriArithmetic => CalendarId("persian-arithmetic"),
            Self::Ethiopic => CalendarId("ethiopic"),
            Self::ThaiBuddhist => CalendarId("buddhist"),
        }
    }

    /// The English name of the underlying calendar.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Gregorian => "Gregorian",
            Self::SolarHijriArithmetic => "Solar Hijri (arithmetic)",
            Self::Ethiopic => "Ethiopic",
            Self::ThaiBuddhist => "Thai solar (Buddhist Era)",
        }
    }

    /// Whether the calendar's date depends on an astronomical model that
    /// this crate only approximates.
    ///
    /// True for [`StartCalendar::SolarHijriArithmetic`] alone. A fiscal year
    /// anchored to it is a very good prediction of Nowruz, not a
    /// proclamation of it.
    #[must_use]
    pub const fn is_approximate(self) -> bool {
        matches!(self, Self::SolarHijriArithmetic)
    }

    /// How many months of equal standing the year divides into, when it
    /// divides into any.
    ///
    /// `Some(12)` for the Gregorian and Solar Hijri calendars. `None` for
    /// the Ethiopic calendar, whose year is twelve months of thirty days
    /// followed by Pagumen — five days, six in a leap year. Pagumen is
    /// numbered as month 13 by the calendar's implementation but it is not a
    /// thirteenth month in any sense that would let this crate say which
    /// quarter it belongs to, so [`quarters`](crate::quarters) refuses the
    /// question instead of answering it.
    #[must_use]
    pub const fn months_of_equal_standing(self) -> Option<u8> {
        match self {
            Self::Gregorian | Self::SolarHijriArithmetic | Self::ThaiBuddhist => Some(12),
            Self::Ethiopic => None,
        }
    }

    /// The fixed day of a date in this calendar.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::Calendar`] when the date does not exist or
    /// falls outside the calendar's supported range.
    pub fn to_fixed(self, year: i64, month: u8, day: u8) -> FiscalResult<Rd> {
        let rd = match self {
            Self::Gregorian => gregorian::to_fixed(year, month, day),
            Self::SolarHijriArithmetic => persian::to_fixed(year, month, day),
            Self::Ethiopic => ethiopic::to_fixed(year, month, day),
            Self::ThaiBuddhist => buddhist::to_fixed(year, month, day),
        }?;
        Ok(rd)
    }

    /// The year, month and day this calendar gives to `rd`.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::Calendar`] outside the calendar's supported
    /// range.
    pub fn from_fixed(self, rd: Rd) -> FiscalResult<(i64, u8, u8)> {
        let parts = match self {
            Self::Gregorian => gregorian::from_fixed(rd),
            Self::SolarHijriArithmetic => persian::from_fixed(rd),
            Self::Ethiopic => ethiopic::from_fixed(rd),
            Self::ThaiBuddhist => buddhist::from_fixed(rd),
        }?;
        Ok(parts)
    }

    /// The year of this calendar that contains `rd`.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::Calendar`] outside the calendar's supported
    /// range.
    pub fn year_containing(self, rd: Rd) -> FiscalResult<i64> {
        match self {
            Self::Gregorian => Ok(gregorian::year_from_fixed(rd)?),
            Self::SolarHijriArithmetic => Ok(persian::year_from_fixed(rd)?),
            Self::Ethiopic | Self::ThaiBuddhist => Ok(self.from_fixed(rd)?.0),
        }
    }
}

/// Where a year begins: a month and a day, in a named calendar.
///
/// "1 April" is not a complete answer to "when does the year start" until
/// the calendar is named, which is the whole reason this type exists rather
/// than a bare `(u8, u8)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YearStart {
    /// The calendar the month and day are expressed in.
    pub calendar: StartCalendar,
    /// The month within that calendar's year, counting from 1.
    pub month: u8,
    /// The day within that month, counting from 1.
    pub day: u8,
}

impl YearStart {
    /// A start date in a named calendar.
    #[must_use]
    pub const fn new(calendar: StartCalendar, month: u8, day: u8) -> Self {
        Self {
            calendar,
            month,
            day,
        }
    }

    /// A Gregorian start date — the common case.
    #[must_use]
    pub const fn gregorian(month: u8, day: u8) -> Self {
        Self::new(StartCalendar::Gregorian, month, day)
    }

    /// Whether this start coincides with the start of its calendar's own
    /// year, which is what "this country has no fiscal offset" means.
    #[must_use]
    pub const fn is_calendar_new_year(self) -> bool {
        self.month == 1 && self.day == 1
    }

    /// The fixed day on which the cycle beginning in calendar year `year`
    /// starts.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::Calendar`] when the date does not exist in
    /// that year — a start of 30 Esfand would not exist in a common Solar
    /// Hijri year — or when the year is out of range.
    pub fn in_calendar_year(self, year: i64) -> FiscalResult<Rd> {
        self.calendar.to_fixed(year, self.month, self.day)
    }
}

/// Which calendar year a fiscal year is named after.
///
/// This type deliberately has **no `Default`**. See the module documentation
/// for why: the two conventions produce spans that differ by twelve months,
/// both are written "FY2024", and a default would make the library wrong on
/// behalf of a caller who did not know the question existed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LabelConvention {
    /// The year is named after the calendar year it **begins** in.
    ///
    /// Japan's 2024年度 begins 1 April 2024. The United Kingdom's 2024/25
    /// tax year begins 6 April 2024. Every calendar-year country is
    /// trivially of this kind too.
    LabelledByStartYear,
    /// The year is named after the calendar year it **ends** in.
    ///
    /// The United States' FY 2024 began 1 October 2023 and ended
    /// 30 September 2024. Ethiopia's fiscal year is the same shape in the
    /// Ethiopic calendar.
    LabelledByEndYear,
}

impl LabelConvention {
    /// The calendar year the cycle labelled `label` begins in.
    #[must_use]
    pub const fn start_calendar_year(self, label: i64) -> i64 {
        match self {
            Self::LabelledByStartYear => label,
            Self::LabelledByEndYear => label - 1,
        }
    }

    /// The label of the cycle that begins in calendar year `year`.
    #[must_use]
    pub const fn label_of_cycle_starting_in(self, year: i64) -> i64 {
        match self {
            Self::LabelledByStartYear => year,
            Self::LabelledByEndYear => year + 1,
        }
    }

    /// A short English description, for rendering a system's definition.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::LabelledByStartYear => "labelled by the year it starts in",
            Self::LabelledByEndYear => "labelled by the year it ends in",
        }
    }
}

/// What sort of year a [`YearSystem`] describes.
///
/// The United Kingdom is why this exists: its government financial year and
/// its personal tax year start five days apart and are both "the UK fiscal
/// year" in ordinary speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKind {
    /// The year the state budgets and accounts in.
    Government,
    /// The year individuals are assessed for income tax in.
    PersonalTax,
    /// The default accounting year for companies, where law supplies one.
    CorporateDefault,
    /// The year schools or universities run on.
    Academic,
}

impl SystemKind {
    /// A short English name.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Government => "government financial year",
            Self::PersonalTax => "personal tax year",
            Self::CorporateDefault => "default corporate accounting year",
            Self::Academic => "academic year",
        }
    }
}

/// How firmly a year system is fixed.
///
/// `docs/observances.md` says a holiday without a source is a rumour. The
/// same applies here, and rather more sharply for academic years: most
/// countries do not legislate when school starts, and a table that asserted
/// a national answer for the United States would be inventing one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Authority {
    /// Fixed by primary legislation.
    Statute,
    /// Fixed by secondary legislation — a regulation, an ordinance, a
    /// ministerial order.
    Regulation,
    /// Not fixed in law, but uniform enough in practice to state.
    Convention,
    /// Set by a subdivision — a state, a province, a *Land* — so the
    /// national entry is the common case and not the rule.
    PerRegion,
    /// Set by each school, university or company individually. The entry is
    /// the modal choice, nothing more.
    PerInstitution,
}

impl Authority {
    /// Whether this system can honestly be asserted as a national answer.
    ///
    /// False for [`Authority::PerRegion`] and [`Authority::PerInstitution`],
    /// where the entry records what is usual rather than what is required.
    #[must_use]
    pub const fn is_national_rule(self) -> bool {
        matches!(self, Self::Statute | Self::Regulation)
    }

    /// A short English name.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Statute => "statute",
            Self::Regulation => "regulation",
            Self::Convention => "convention",
            Self::PerRegion => "set per region",
            Self::PerInstitution => "set per institution",
        }
    }
}

/// The day a table's sources were last checked against their statute.
///
/// The same type, and the same reasoning, as `hc_holiday::rule::SourceDate`:
/// a fiscal year without a source is a rumour, and a fiscal year without a
/// check date is a rumour about when it was true. Duplicated rather than
/// shared because `hc-fiscal` does not depend on `hc-holiday` and should not
/// start to for three integers.
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

/// One fiscal, tax or academic year, as a value.
///
/// A `YearSystem` is inert. It says where the year starts, in which
/// calendar, how the year is named, over which labels it was in force, and
/// where the claim came from — and nothing about how to evaluate any of
/// that. The evaluation is the handful of methods below, shared by every
/// entry in [`countries`](crate::countries) and
/// [`academic`](crate::academic).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearSystem {
    /// The English name.
    pub name: &'static str,
    /// The name in the local language and script, or `""` when the English
    /// name is the local one.
    pub local_name: &'static str,
    /// What sort of year this is.
    pub kind: SystemKind,
    /// How firmly it is fixed, and by whom.
    pub authority: Authority,
    /// Where the year begins.
    pub start: YearStart,
    /// Which calendar year the year is named after.
    pub label: LabelConvention,
    /// The first label this system covers, or `None` when it reaches back
    /// indefinitely.
    ///
    /// **These are year labels of this system, not Gregorian years.** For
    /// Iran that means Solar Hijri years; for Ethiopia, Ethiopic ones. It is
    /// the number a caller passes to [`YearSystem::span`], which is the only
    /// number a caller ever has in hand.
    pub valid_from: Option<i64>,
    /// The last label this system covers, or `None` when it is still in
    /// force.
    pub valid_until: Option<i64>,
    /// What the entry deliberately does not claim, or the historical detail
    /// that explains it. Empty when there is nothing to add.
    pub note: &'static str,
}

impl YearSystem {
    /// Whether this system covers the year labelled `label`.
    #[must_use]
    pub const fn covers(&self, label: i64) -> bool {
        if let Some(first) = self.valid_from
            && label < first
        {
            return false;
        }
        if let Some(last) = self.valid_until
            && label > last
        {
            return false;
        }
        true
    }

    /// Whether the year is the plain calendar year of its own calendar.
    ///
    /// Included because "does this country have a fiscal offset at all" is
    /// itself a question people ask, and `false` is a real answer rather
    /// than a missing entry.
    #[must_use]
    pub const fn is_calendar_year(&self) -> bool {
        self.start.is_calendar_new_year()
            && matches!(self.label, LabelConvention::LabelledByStartYear)
    }

    /// Whether the year coincides with the plain Gregorian calendar year.
    ///
    /// Distinct from [`YearSystem::is_calendar_year`], and the difference is
    /// Iran: the Iranian fiscal year *is* its own calendar's year, so
    /// `is_calendar_year` is true for it, while its Gregorian start moves
    /// between 20 and 21 March and nothing about it aligns with 1 January.
    /// "Does this country have a fiscal offset" is a question about the
    /// Gregorian year, so it is this method that answers it.
    #[must_use]
    pub const fn is_gregorian_calendar_year(&self) -> bool {
        self.is_calendar_year() && matches!(self.start.calendar, StartCalendar::Gregorian)
    }

    /// The first day of the year labelled `label`, ignoring the validity
    /// range.
    ///
    /// "Projected" in the sense `hc-calendars-solar` uses "proleptic": the
    /// rule is run outside the period it was actually in force, which is
    /// useful for comparing two systems on the same date and wrong for
    /// answering a historical question. Use [`YearSystem::span`] for the
    /// latter.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::Calendar`] when the start date does not exist
    /// in the relevant calendar year.
    pub fn projected_start(&self, label: i64) -> FiscalResult<Rd> {
        self.start
            .in_calendar_year(self.label.start_calendar_year(label))
    }

    /// The span of the year labelled `label`, ignoring the validity range.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::Calendar`] when either boundary cannot be
    /// computed.
    pub fn projected_span(&self, label: i64) -> FiscalResult<FiscalSpan> {
        let first = self.projected_start(label)?;
        let next = self.projected_start(label.checked_add(1).ok_or(FiscalError::Overflow)?)?;
        if next <= first {
            // Cannot arise from any table in this crate; guarded so that a
            // caller-supplied system with a nonsensical start cannot produce
            // a negative-length span.
            return Err(FiscalError::Calendar(CalendarError::YearOutOfRange));
        }
        Ok(FiscalSpan {
            label,
            first,
            last: Rd(next.0 - 1),
        })
    }

    /// The span of the year labelled `label`.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::OutsideValidity`] when `label` falls outside
    /// the range this system was in force, and [`FiscalError::Calendar`]
    /// when a boundary cannot be computed.
    pub fn span(&self, label: i64) -> FiscalResult<FiscalSpan> {
        if !self.covers(label) {
            return Err(FiscalError::OutsideValidity);
        }
        self.projected_span(label)
    }

    /// The label of the year containing `rd`, ignoring the validity range.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::Calendar`] when `rd` lies outside the start
    /// calendar's supported range.
    pub fn projected_label_at(&self, rd: Rd) -> FiscalResult<i64> {
        let calendar_year = self.start.calendar.year_containing(rd)?;
        let start_this_year = self.start.in_calendar_year(calendar_year)?;
        let cycle_year = if rd >= start_this_year {
            calendar_year
        } else {
            calendar_year - 1
        };
        Ok(self.label.label_of_cycle_starting_in(cycle_year))
    }

    /// The label of the year containing `rd`.
    ///
    /// # Errors
    ///
    /// Returns [`FiscalError::OutsideValidity`] when the answer falls
    /// outside the range this system was in force, and
    /// [`FiscalError::Calendar`] when `rd` is outside the start calendar's
    /// range.
    pub fn label_at(&self, rd: Rd) -> FiscalResult<i64> {
        let label = self.projected_label_at(rd)?;
        if self.covers(label) {
            Ok(label)
        } else {
            Err(FiscalError::OutsideValidity)
        }
    }

    /// Where `rd` sits inside its fiscal year.
    ///
    /// # Errors
    ///
    /// As [`YearSystem::label_at`], plus [`FiscalError::Calendar`] when the
    /// surrounding span cannot be computed.
    pub fn locate(&self, rd: Rd) -> FiscalResult<FiscalPosition> {
        let label = self.label_at(rd)?;
        let span = self.projected_span(label)?;
        span.position_of(rd).ok_or(FiscalError::Overflow)
    }
}

/// One fiscal year, as a closed interval of fixed days.
///
/// Both ends are inclusive, because "the year ends on 31 March" is how every
/// source in this crate phrases it and a half-open interval would invite the
/// reader to check which convention was meant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FiscalSpan {
    /// The year's label.
    pub label: i64,
    /// The first day of the year.
    pub first: Rd,
    /// The last day of the year.
    pub last: Rd,
}

impl FiscalSpan {
    /// How many days the year contains.
    #[must_use]
    pub const fn days(&self) -> i64 {
        self.last.0 - self.first.0 + 1
    }

    /// Whether `rd` falls inside the year.
    #[must_use]
    pub const fn contains(&self, rd: Rd) -> bool {
        rd.0 >= self.first.0 && rd.0 <= self.last.0
    }

    /// Where `rd` sits inside the year, or `None` when it falls outside.
    #[must_use]
    pub fn position_of(&self, rd: Rd) -> Option<FiscalPosition> {
        if !self.contains(rd) {
            return None;
        }
        let elapsed = rd.0 - self.first.0;
        let day_of_year = u16::try_from(elapsed + 1).ok()?;
        let days_in_year = u16::try_from(self.days()).ok()?;
        Some(FiscalPosition {
            label: self.label,
            day_of_year,
            days_in_year,
            weekday: Weekday::from_rd(rd),
        })
    }
}

/// Where a day sits inside its fiscal year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FiscalPosition {
    /// The label of the fiscal year the day belongs to.
    pub label: i64,
    /// The day's ordinal within that year, counting from 1.
    pub day_of_year: u16,
    /// How many days that year contains.
    pub days_in_year: u16,
    /// The day of the week, which a fiscal calendar is asked for often
    /// enough — "the last working day of the year" — to be worth carrying.
    pub weekday: Weekday,
}

impl FiscalPosition {
    /// How many days remain in the year, counting the day itself as gone.
    #[must_use]
    pub const fn days_remaining(&self) -> u16 {
        self.days_in_year - self.day_of_year
    }

    /// How far through the year the day is, from 0.0 on the first day to
    /// just under 1.0 on the last.
    ///
    /// The value is the elapsed-day count divided by the year length, so it
    /// is exact to the resolution of a day and claims nothing finer.
    #[must_use]
    pub fn fraction_elapsed(&self) -> f64 {
        f64::from(self.day_of_year - 1) / f64::from(self.days_in_year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Japan's 会計年度: 1 April, named after the year it starts in.
    const JAPAN: YearSystem = YearSystem {
        name: "Japanese fiscal year",
        local_name: "年度",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(4, 1),
        label: LabelConvention::LabelledByStartYear,
        valid_from: None,
        valid_until: None,
        note: "",
    };

    /// The United States federal fiscal year: 1 October, named after the
    /// year it ends in.
    const US: YearSystem = YearSystem {
        name: "United States federal fiscal year",
        local_name: "",
        kind: SystemKind::Government,
        authority: Authority::Statute,
        start: YearStart::gregorian(10, 1),
        label: LabelConvention::LabelledByEndYear,
        valid_from: Some(1977),
        valid_until: None,
        note: "",
    };

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_japanese_fiscal_year_runs_april_to_march() {
        let span = JAPAN.span(2024).unwrap();
        assert_eq!(span.first, greg(2024, 4, 1));
        assert_eq!(span.last, greg(2025, 3, 31));
        assert_eq!(span.days(), 365);
    }

    #[test]
    fn the_japanese_fiscal_year_turns_between_march_31_and_april_1() {
        assert_eq!(JAPAN.label_at(greg(2024, 3, 31)).unwrap(), 2023);
        assert_eq!(JAPAN.label_at(greg(2024, 4, 1)).unwrap(), 2024);
        assert_eq!(JAPAN.label_at(greg(2025, 3, 31)).unwrap(), 2024);
        assert_eq!(JAPAN.label_at(greg(2025, 4, 1)).unwrap(), 2025);
    }

    #[test]
    fn the_us_federal_year_labelled_2024_began_in_2023() {
        let span = US.span(2024).unwrap();
        assert_eq!(span.first, greg(2023, 10, 1));
        assert_eq!(span.last, greg(2024, 9, 30));
    }

    #[test]
    fn japan_and_the_united_states_disagree_by_a_year_on_the_same_day() {
        // 1 November 2023. Tokyo says FY2023; Washington says FY2024. Both
        // write it "FY". This is the trap the enum exists to make visible.
        let day = greg(2023, 11, 1);
        assert_eq!(JAPAN.label_at(day).unwrap(), 2023);
        assert_eq!(US.label_at(day).unwrap(), 2024);
        assert_eq!(US.label_at(day).unwrap() - JAPAN.label_at(day).unwrap(), 1);
    }

    #[test]
    fn the_two_years_written_fy2024_share_only_six_months() {
        let japan = JAPAN.span(2024).unwrap();
        let us = US.span(2024).unwrap();
        assert!(us.first < japan.first);
        // Overlap is 1 April 2024 to 30 September 2024 inclusive: 183 days.
        let overlap = us.last.0 - japan.first.0 + 1;
        assert_eq!(overlap, 183);
    }

    #[test]
    fn the_label_conventions_are_exact_inverses_of_each_other() {
        for label in -50..2500 {
            for convention in [
                LabelConvention::LabelledByStartYear,
                LabelConvention::LabelledByEndYear,
            ] {
                let start = convention.start_calendar_year(label);
                assert_eq!(convention.label_of_cycle_starting_in(start), label);
            }
        }
    }

    #[test]
    fn a_label_and_a_day_round_trip_through_each_other() {
        for label in 1900..2100 {
            for system in [JAPAN, US] {
                let span = system.projected_span(label).unwrap();
                assert_eq!(system.projected_label_at(span.first).unwrap(), label);
                assert_eq!(system.projected_label_at(span.last).unwrap(), label);
                assert_eq!(
                    system.projected_label_at(Rd(span.first.0 - 1)).unwrap(),
                    label - 1
                );
                assert_eq!(
                    system.projected_label_at(Rd(span.last.0 + 1)).unwrap(),
                    label + 1
                );
            }
        }
    }

    #[test]
    fn consecutive_years_abut_without_a_gap_or_an_overlap() {
        for label in 1800..2200 {
            let this = JAPAN.projected_span(label).unwrap();
            let next = JAPAN.projected_span(label + 1).unwrap();
            assert_eq!(next.first.0, this.last.0 + 1);
        }
    }

    #[test]
    fn every_day_of_a_long_run_lands_in_exactly_one_year() {
        let first = greg(1990, 1, 1);
        let last = greg(2040, 12, 31);
        for day in first.0..=last.0 {
            let rd = Rd(day);
            let label = US.projected_label_at(rd).unwrap();
            let span = US.projected_span(label).unwrap();
            assert!(span.contains(rd));
            assert!(!US.projected_span(label - 1).unwrap().contains(rd));
            assert!(!US.projected_span(label + 1).unwrap().contains(rd));
        }
    }

    #[test]
    fn a_leap_day_makes_the_fiscal_year_366_days_long() {
        // 29 February 2024 falls inside Japan's 2023年度, not 2024年度.
        let leap_day = greg(2024, 2, 29);
        assert_eq!(JAPAN.label_at(leap_day).unwrap(), 2023);
        assert_eq!(JAPAN.span(2023).unwrap().days(), 366);
        assert_eq!(JAPAN.span(2024).unwrap().days(), 365);
    }

    #[test]
    fn the_validity_range_refuses_a_year_the_system_did_not_cover() {
        assert_eq!(US.span(1976), Err(FiscalError::OutsideValidity));
        assert_eq!(US.span(1977).unwrap().first, greg(1976, 10, 1));
        // The rule can still be projected backwards when a caller asks for
        // that explicitly.
        assert_eq!(US.projected_span(1976).unwrap().first, greg(1975, 10, 1));
    }

    #[test]
    fn a_date_before_the_system_existed_has_no_label() {
        assert_eq!(
            US.label_at(greg(1960, 3, 1)),
            Err(FiscalError::OutsideValidity)
        );
    }

    #[test]
    fn the_position_within_the_year_counts_from_one() {
        let first_day = JAPAN.locate(greg(2024, 4, 1)).unwrap();
        assert_eq!(first_day.day_of_year, 1);
        assert_eq!(first_day.days_in_year, 365);
        assert_eq!(first_day.days_remaining(), 364);
        assert!(first_day.fraction_elapsed().abs() < 1e-12);

        let last_day = JAPAN.locate(greg(2025, 3, 31)).unwrap();
        assert_eq!(last_day.day_of_year, 365);
        assert_eq!(last_day.days_remaining(), 0);
        assert!(last_day.fraction_elapsed() < 1.0);
    }

    #[test]
    fn the_position_carries_the_weekday() {
        // 1 April 2024 was a Monday.
        assert_eq!(
            JAPAN.locate(greg(2024, 4, 1)).unwrap().weekday,
            Weekday::Monday
        );
    }

    #[test]
    fn a_calendar_year_system_reports_itself_as_one() {
        let germany = YearSystem {
            name: "German federal budget year",
            local_name: "Haushaltsjahr",
            kind: SystemKind::Government,
            authority: Authority::Statute,
            start: YearStart::gregorian(1, 1),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            note: "",
        };
        assert!(germany.is_calendar_year());
        assert!(!JAPAN.is_calendar_year());
        assert!(!US.is_calendar_year());
        let span = germany.span(2024).unwrap();
        assert_eq!(span.first, greg(2024, 1, 1));
        assert_eq!(span.last, greg(2024, 12, 31));
    }

    #[test]
    fn every_start_calendar_round_trips_a_long_run_of_days() {
        // The four calendars a start date may be written in must all answer
        // both directions consistently over the range this crate uses them
        // in, or a fiscal-year boundary could land on a day the calendar
        // does not agree exists.
        for calendar in [
            StartCalendar::Gregorian,
            StartCalendar::SolarHijriArithmetic,
            StartCalendar::Ethiopic,
            StartCalendar::ThaiBuddhist,
        ] {
            for rd in (greg(1800, 1, 1).0..=greg(2200, 1, 1).0).step_by(37) {
                let (year, month, day) = calendar.from_fixed(Rd(rd)).unwrap();
                assert_eq!(calendar.to_fixed(year, month, day).unwrap(), Rd(rd));
                assert_eq!(calendar.year_containing(Rd(rd)).unwrap(), year);
            }
        }
    }

    #[test]
    fn a_calendar_new_year_start_is_recognised_in_every_calendar() {
        assert!(YearStart::gregorian(1, 1).is_calendar_new_year());
        assert!(YearStart::new(StartCalendar::SolarHijriArithmetic, 1, 1).is_calendar_new_year());
        assert!(!YearStart::gregorian(4, 1).is_calendar_new_year());
    }

    #[test]
    fn only_the_solar_hijri_calendar_is_flagged_approximate() {
        assert!(StartCalendar::SolarHijriArithmetic.is_approximate());
        assert!(!StartCalendar::Gregorian.is_approximate());
        assert!(!StartCalendar::Ethiopic.is_approximate());
        assert!(!StartCalendar::ThaiBuddhist.is_approximate());
    }

    #[test]
    fn the_ethiopic_year_has_no_twelve_months_of_equal_standing() {
        assert_eq!(
            StartCalendar::Gregorian.months_of_equal_standing(),
            Some(12)
        );
        assert_eq!(
            StartCalendar::SolarHijriArithmetic.months_of_equal_standing(),
            Some(12)
        );
        assert_eq!(
            StartCalendar::ThaiBuddhist.months_of_equal_standing(),
            Some(12)
        );
        assert_eq!(StartCalendar::Ethiopic.months_of_equal_standing(), None);
    }

    #[test]
    fn a_nonexistent_start_date_is_reported_rather_than_clamped() {
        let impossible = YearSystem {
            name: "test",
            local_name: "",
            kind: SystemKind::Government,
            authority: Authority::Convention,
            start: YearStart::gregorian(2, 30),
            label: LabelConvention::LabelledByStartYear,
            valid_from: None,
            valid_until: None,
            note: "",
        };
        assert_eq!(
            impossible.span(2024),
            Err(FiscalError::Calendar(CalendarError::DayOutOfRange))
        );
    }

    #[test]
    fn the_calendar_identifiers_follow_cldr_where_one_exists() {
        assert_eq!(StartCalendar::Gregorian.id().as_str(), "gregory");
        assert_eq!(StartCalendar::Ethiopic.id().as_str(), "ethiopic");
        assert_eq!(StartCalendar::ThaiBuddhist.id().as_str(), "buddhist");
        // Deliberately not "persian": that identifier belongs to the
        // astronomical calendar this crate does not have.
        assert_eq!(
            StartCalendar::SolarHijriArithmetic.id().as_str(),
            "persian-arithmetic"
        );
    }

    #[test]
    fn an_open_ended_validity_range_covers_everything() {
        assert!(JAPAN.covers(1600));
        assert!(JAPAN.covers(2600));
        assert!(!US.covers(1976));
        assert!(US.covers(1977));
        assert!(US.covers(2600));
    }

    #[test]
    fn authority_distinguishes_a_national_rule_from_a_local_habit() {
        assert!(Authority::Statute.is_national_rule());
        assert!(Authority::Regulation.is_national_rule());
        assert!(!Authority::Convention.is_national_rule());
        assert!(!Authority::PerRegion.is_national_rule());
        assert!(!Authority::PerInstitution.is_national_rule());
    }
}
