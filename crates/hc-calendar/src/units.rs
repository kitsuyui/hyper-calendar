//! A stretch of days walked as one calendar's eras, years, months or days.
//!
//! # What this is for
//!
//! A timeline that draws calendars as stacked lanes needs, for each lane,
//! the runs of fixed days that share a unit — this era, this year, this
//! month — and the fields of the first day of each run, so that it can
//! draw a box and label it. [`units`] answers that for any
//! [`DynCalendar`], using nothing but the dynamic interface, so the caller
//! never has to know which calendar it is drawing.
//!
//! # How it walks
//!
//! By the calendar's own lengths, never day by day except where the unit
//! *is* the day. A unit is the run of days on which the fields that
//! identify it — the era; the era and year; the era, year and month — stay
//! the same, and its edges are found by asking the calendar where it thinks
//! they are and checking: the first day of the month is where the calendar
//! puts day 1, its end is that plus
//! [`DynCalendar::days_in_month`], a year's end is its start plus
//! [`DynCalendar::days_in_year`] or, for a calendar that counts years
//! within eras, the first day of the next year of the same era. Each hint
//! is verified with one conversion — of the day it names, which the next
//! span needs anyway — and a hint the calendar gets wrong, or cannot give,
//! as no calendar can for the end of an era, is replaced by an exponential
//! search from the last day known to be inside the unit, then a bisection,
//! which costs a few dozen conversions for an era of any length rather
//! than one per day. A month of an astronomical calendar therefore costs
//! its `days_in_month` and one conversion.
//!
//! # What comes back
//!
//! Spans in order, touching end to start, from the start of the unit that
//! contains `from` to at least `to`: the first and last spans are whole
//! units and may reach outside the range asked for, because a unit's edges
//! are facts about the calendar and not about the question. The exceptions
//! are the calendar's own limits: a unit that begins before
//! [`CalendarMeta::earliest`](crate::CalendarMeta::earliest) or ends after
//! [`CalendarMeta::latest`](crate::CalendarMeta::latest) is clipped there,
//! and the days beyond are a refusal span, as are the days the calendar
//! refuses inside its range — the years of the Japanese schism under the
//! unified stream — and the whole of the range for a unit the calendar does
//! not have: months of a calendar without a month cycle, years of a day
//! count, eras of a calendar that names none. A refusal is a span like any
//! other, with the calendar's own [`CalendarError`] in place of the fields,
//! so that a lane can show "this calendar does not reach here" rather than
//! a gap.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::daystart::Standing;
use crate::error::{CalendarError, CalendarResult};
use crate::fields::DateFields;
#[cfg(feature = "alloc")]
use crate::fields::{Month, YearKind};
use crate::fixed::Rd;
#[cfg(feature = "alloc")]
use crate::shape::MONTH;
#[cfg(feature = "alloc")]
use crate::traits::DynCalendar;

/// A unit of a calendar that a stretch of days can be divided into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    /// The era: the run of days sharing [`DateFields::era`].
    Era,
    /// The year: the run sharing the era and [`DateFields::year`], so that
    /// a calendar counting years within eras starts a new year where the
    /// era changes, as 令和元年 does on 1 May 2019.
    Year,
    /// The month: the run sharing the era, the year and
    /// [`DateFields::month`], leap flag included, so that a leap month is
    /// its own span after the month it repeats.
    Month,
    /// One day.
    Day,
}

impl Unit {
    /// Every unit, largest first.
    pub const ALL: [Self; 4] = [Self::Era, Self::Year, Self::Month, Self::Day];

    /// The unit's name, lower case.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Era => "era",
            Self::Year => "year",
            Self::Month => "month",
            Self::Day => "day",
        }
    }

    /// The unit's position in [`Unit::ALL`]: `0` for the era through `3`
    /// for the day. This is the number an ABI passes.
    #[must_use]
    pub const fn index(self) -> u32 {
        match self {
            Self::Era => 0,
            Self::Year => 1,
            Self::Month => 2,
            Self::Day => 3,
        }
    }

    /// The unit at a position in [`Unit::ALL`], or `None` past the end.
    #[must_use]
    pub const fn from_index(index: u32) -> Option<Self> {
        match index {
            0 => Some(Self::Era),
            1 => Some(Self::Year),
            2 => Some(Self::Month),
            3 => Some(Self::Day),
            _ => None,
        }
    }
}

/// What a calendar says about a unit it could place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitDate {
    /// The fields of the unit's first day.
    pub fields: DateFields,
    /// Whether the unit is the calendar's intercalary one: a leap year by
    /// [`DynCalendar::is_leap_year_of`], a leap month, a repeated day.
    /// Always false for an era.
    pub leap: bool,
    /// How the unit's first day relates to the period the calendar was in
    /// use.
    pub standing: Standing,
}

/// One run of fixed days that is a unit of a calendar, or that the
/// calendar refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitSpan {
    /// The first day of the run.
    pub start: Rd,
    /// The day after the last day of the run, so that `end - start` is the
    /// length and one span's `end` is the next span's `start`.
    pub end: Rd,
    /// The unit, or the calendar's refusal of every day in the run.
    pub dated: CalendarResult<UnitDate>,
}

impl UnitSpan {
    /// How many days the span covers.
    #[must_use]
    pub const fn days(&self) -> i64 {
        self.end.0 - self.start.0
    }

    /// The refusal, when the span is one.
    #[must_use]
    pub fn error(&self) -> Option<CalendarError> {
        self.dated.err()
    }

    /// The unit, when the span is one.
    #[must_use]
    pub fn date(&self) -> Option<&UnitDate> {
        self.dated.as_ref().ok()
    }
}

/// The fields that identify one unit: as many of era, year and month as
/// the unit needs, so that two days are in the same unit exactly when
/// their keys are equal.
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Key {
    era: Option<&'static str>,
    year: Option<i64>,
    month: Option<Month>,
}

#[cfg(feature = "alloc")]
impl Key {
    fn of(fields: &DateFields, unit: Unit) -> Self {
        Self {
            era: fields.era,
            year: matches!(unit, Unit::Year | Unit::Month).then_some(fields.year),
            month: (unit == Unit::Month).then_some(fields.month).flatten(),
        }
    }
}

/// How far a search goes for a calendar that states no limit: 2⁴⁰ days
/// either side of the epoch, some three billion years, which is beyond
/// anything this library dates and short of anything that overflows.
#[cfg(feature = "alloc")]
const UNBOUNDED: i64 = 1 << 40;

/// The dynamic interface with a one-entry memory, because the day a search
/// last looked at is usually the day the walk asks about next.
#[cfg(feature = "alloc")]
struct Walker<'a> {
    calendar: &'a dyn DynCalendar,
    unit: Unit,
    /// The last conversion, so that verifying a hint and then starting the
    /// next span from the same day costs one conversion rather than two.
    last: Option<(Rd, CalendarResult<DateFields>)>,
    /// The first day the calendar's range does not reach, where it has one,
    /// which no search has to look beyond.
    ceiling: Rd,
    /// The first day the calendar's range does reach, where it has one,
    /// which no search has to look before.
    floor: Rd,
}

#[cfg(feature = "alloc")]
impl Walker<'_> {
    fn fields_at(&mut self, rd: Rd) -> CalendarResult<DateFields> {
        if let Some((day, result)) = self.last
            && day == rd
        {
            return result;
        }
        let result = if rd < self.floor {
            Err(CalendarError::BeforeEpoch)
        } else if rd >= self.ceiling {
            Err(CalendarError::AfterSupportedRange)
        } else {
            self.calendar.fixed_to_fields(rd)
        };
        self.last = Some((rd, result));
        result
    }

    /// Whether `rd` converts and is in the unit `key` names.
    fn same(&mut self, rd: Rd, key: Key) -> bool {
        self.fields_at(rd)
            .is_ok_and(|fields| Key::of(&fields, self.unit) == key)
    }

    /// Where the calendar says the unit containing `fields` begins, if it
    /// can say: day 1 of the month, or day 1 of month 1 of the year. Not
    /// verified here.
    fn start_hint(&self, fields: &DateFields) -> Option<Rd> {
        let mut probe = *fields;
        match self.unit {
            Unit::Month => {
                probe.day = Some(1);
                probe.leap_day = false;
            }
            Unit::Year => {
                if probe.month.is_some() {
                    probe.month = Some(Month::regular(1));
                    probe.day = Some(1);
                    probe.leap_day = false;
                } else {
                    // A calendar with a year and no months — an ISO week
                    // date, an ordinal date — reads a bare year as its first
                    // day, which is how `days_in_year` measures it too.
                    probe = DateFields::new(probe.year);
                    probe.era = fields.era;
                }
            }
            Unit::Era | Unit::Day => return None,
        }
        self.calendar.fields_to_fixed(&probe).ok()
    }

    /// Where the calendar says the unit that begins at `start` with
    /// `fields` ends, if it can say. Not verified here.
    fn end_hint(&self, start: Rd, fields: &DateFields) -> Option<Rd> {
        match self.unit {
            Unit::Month => {
                let length = self.calendar.days_in_month(fields).ok()?;
                start.0.checked_add(i64::from(length)).map(Rd)
            }
            Unit::Year => {
                if self.calendar.meta().year_kind == YearKind::EraRelative {
                    // `days_in_year` takes the continuous count and the
                    // fields carry the count within the era, so ask for the
                    // next year of the same era instead; where the era ends
                    // first, the check below rejects the answer and the
                    // search finds the real edge.
                    let mut probe = *fields;
                    probe.year = probe.year.checked_add(1)?;
                    if probe.month.is_some() {
                        probe.month = Some(Month::regular(1));
                        probe.day = Some(1);
                        probe.leap_day = false;
                    }
                    return self.calendar.fields_to_fixed(&probe).ok();
                }
                let length = self.calendar.days_in_year(fields.year).ok()?;
                start.0.checked_add(i64::from(length)).map(Rd)
            }
            Unit::Era | Unit::Day => None,
        }
    }

    /// The first day of the unit containing `cursor`, which is known to be
    /// in it: the calendar's hint when it checks out, else a search back
    /// from `cursor`. Never before the floor.
    fn find_start(&mut self, cursor: Rd, fields: &DateFields, key: Key) -> Rd {
        if let Some(hint) = self.start_hint(fields)
            && hint <= cursor
            && hint >= self.floor
            && (hint == cursor || self.same(hint, key))
            && (hint == self.floor || !self.same(Rd(hint.0 - 1), key))
        {
            return hint;
        }
        // Exponential search back to a day outside the unit, then bisect.
        let mut inside = cursor;
        let mut outside;
        let mut step = 1i64;
        loop {
            let candidate = Rd(inside.0.saturating_sub(step));
            if candidate < self.floor {
                outside = Rd(self.floor.0 - 1);
                break;
            }
            if self.same(candidate, key) {
                inside = candidate;
                step = step.saturating_mul(2);
            } else {
                outside = candidate;
                break;
            }
        }
        while inside.0 - outside.0 > 1 {
            let middle = Rd(outside.0 + (inside.0 - outside.0) / 2);
            if self.same(middle, key) {
                inside = middle;
            } else {
                outside = middle;
            }
        }
        inside
    }

    /// The day after the last day of the unit that begins at `start` and
    /// contains `cursor`: the calendar's hint when it checks out, else a
    /// search forward from `cursor`. Never past the ceiling.
    ///
    /// A hint is checked with one conversion, of the day it names, which
    /// must lie outside the unit — and which the walk converts anyway,
    /// since it is where the next span begins. A hint that lands inside
    /// the unit is rejected; one that overshoots it is the calendar's own
    /// statement of its length and is taken at its word, because the
    /// conversion that would catch it costs as much as the one that finds
    /// it, and for an astronomical calendar that is the whole cost.
    fn find_end(&mut self, start: Rd, cursor: Rd, fields: &DateFields, key: Key) -> Rd {
        if let Some(hint) = self.end_hint(start, fields)
            && hint > cursor
            && hint <= self.ceiling
            && (hint == self.ceiling || !self.same(hint, key))
        {
            return hint;
        }
        let mut inside = cursor;
        let mut outside;
        let mut step = 1i64;
        loop {
            let candidate = Rd(inside.0.saturating_add(step));
            if candidate >= self.ceiling {
                outside = self.ceiling;
                break;
            }
            if self.same(candidate, key) {
                inside = candidate;
                step = step.saturating_mul(2);
            } else {
                outside = candidate;
                break;
            }
        }
        while outside.0 - inside.0 > 1 {
            let middle = Rd(inside.0 + (outside.0 - inside.0) / 2);
            if self.same(middle, key) {
                inside = middle;
            } else {
                outside = middle;
            }
        }
        outside
    }

    /// The first day at or after `cursor`, which the calendar refuses, that
    /// it converts — or `limit`, when nothing before `limit` does. A
    /// refusal is assumed to be one run of days, which is what a schism, an
    /// epoch and a table's end all are.
    fn next_convertible(&mut self, cursor: Rd, limit: Rd) -> Rd {
        let limit = limit.min(self.ceiling);
        let mut refused = cursor;
        let mut converts = limit;
        let mut step = 1i64;
        loop {
            let candidate = Rd(refused.0.saturating_add(step));
            if candidate >= limit {
                break;
            }
            if self.fields_at(candidate).is_ok() {
                converts = candidate;
                break;
            }
            refused = candidate;
            step = step.saturating_mul(2);
        }
        while converts.0 - refused.0 > 1 {
            let middle = Rd(refused.0 + (converts.0 - refused.0) / 2);
            if self.fields_at(middle).is_ok() {
                converts = middle;
            } else {
                refused = middle;
            }
        }
        converts
    }
}

/// Whether the calendar has `unit` at all, judged from one converted day.
#[cfg(feature = "alloc")]
fn supports(calendar: &dyn DynCalendar, unit: Unit, fields: &DateFields) -> CalendarResult<()> {
    match unit {
        Unit::Era => fields
            .era
            .map(|_| ())
            .ok_or(CalendarError::UnsupportedField("era")),
        Unit::Year => match calendar.is_leap_year_of(fields) {
            Err(error @ CalendarError::UnsupportedField(_)) => Err(error),
            _ => Ok(()),
        },
        Unit::Month => {
            if calendar.cycles().iter().any(|cycle| cycle.kind == MONTH) {
                Ok(())
            } else {
                Err(CalendarError::UnsupportedField("month"))
            }
        }
        Unit::Day => Ok(()),
    }
}

/// The days from `from` up to but not including `to`, as the units of
/// `calendar` that cover them, in order.
///
/// See the module documentation for what the spans are and how their
/// edges are found. An empty range gives no spans.
#[cfg(feature = "alloc")]
#[must_use]
pub fn units(calendar: &dyn DynCalendar, unit: Unit, from: Rd, to: Rd) -> Vec<UnitSpan> {
    let mut out = Vec::new();
    if from >= to {
        return out;
    }
    let meta = calendar.meta();
    let mut walker = Walker {
        calendar,
        unit,
        last: None,
        ceiling: meta.latest.map_or(Rd(UNBOUNDED), |last| Rd(last.0 + 1)),
        floor: meta.earliest.unwrap_or(Rd(-UNBOUNDED)),
    };
    let refusal = |start: Rd, end: Rd, error: CalendarError| UnitSpan {
        start,
        end,
        dated: Err(error),
    };
    let mut cursor = from;
    let mut support: Option<CalendarResult<()>> = None;
    let mut first = true;
    while cursor < to {
        if cursor < walker.floor {
            let end = walker.floor.min(to);
            out.push(refusal(cursor, end, CalendarError::BeforeEpoch));
            cursor = end;
            first = false;
            continue;
        }
        if cursor >= walker.ceiling {
            out.push(refusal(cursor, to, CalendarError::AfterSupportedRange));
            break;
        }
        let fields = match walker.fields_at(cursor) {
            Ok(fields) => fields,
            Err(error) => {
                let end = walker.next_convertible(cursor, to);
                out.push(refusal(cursor, end, error));
                cursor = end;
                first = false;
                continue;
            }
        };
        let support = *support.get_or_insert_with(|| supports(calendar, unit, &fields));
        if let Err(error) = support {
            out.push(refusal(cursor, to, error));
            break;
        }
        let (start, end) = if unit == Unit::Day {
            (cursor, Rd(cursor.0 + 1))
        } else {
            let key = Key::of(&fields, unit);
            let start = if first {
                walker.find_start(cursor, &fields, key)
            } else {
                cursor
            };
            let end = walker.find_end(start, cursor, &fields, key);
            (start, end)
        };
        let fields = if start == cursor {
            fields
        } else {
            walker.fields_at(start).unwrap_or(fields)
        };
        let leap = match unit {
            Unit::Era => false,
            Unit::Year => calendar.is_leap_year_of(&fields).unwrap_or(false),
            Unit::Month => fields.month.is_some_and(|month| month.leap),
            Unit::Day => fields.leap_day,
        };
        out.push(UnitSpan {
            start,
            end,
            dated: Ok(UnitDate {
                fields,
                leap,
                standing: calendar.standing(start),
            }),
        });
        cursor = end;
        first = false;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daystart::Usage;
    use crate::shape::CycleShape;
    use crate::traits::{Calendar, CalendarId, CalendarMeta, DynAdapter};

    /// A toy calendar: years of a hundred days in ten months of ten, every
    /// fourth year with an eleventh month of five days, in two eras that
    /// change on day 1000, and defined only from day -500 to day 2999.
    #[derive(Debug, Clone, Copy)]
    struct Toy;

    const FLOOR: i64 = -500;
    const CEILING: i64 = 2_999;

    impl Toy {
        fn year_start(year: i64) -> i64 {
            // Years 0.. begin at 0, 100, 200, ...; each leap year adds five.
            year * 100 + (year.max(0) + 3) / 4 * 5
        }
    }

    impl Calendar for Toy {
        type Date = (i64, u8, u8);

        fn cycles(&self) -> &'static [CycleShape] {
            const SHAPE: &[CycleShape] = &[CycleShape::intercalary(MONTH, 10, 11)];
            SHAPE
        }

        fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
            Ok(year >= 0 && year % 4 == 0)
        }

        fn meta(&self) -> CalendarMeta {
            CalendarMeta {
                id: CalendarId("toy"),
                english_name: "Toy",
                year_kind: YearKind::Astronomical,
                has_leap_months: true,
                is_astronomical: false,
                earliest: Some(Rd(FLOOR)),
                latest: Some(Rd(CEILING)),
                native_locales: &[],
            }
        }

        fn usage(&self) -> Usage {
            Usage::since(Rd(0), "a test calendar in use from its epoch")
        }

        fn to_fixed(&self, (year, month, day): Self::Date) -> CalendarResult<Rd> {
            let months = if self.is_leap_year(year)? { 11 } else { 10 };
            if month < 1 || month > months {
                return Err(CalendarError::MonthOutOfRange);
            }
            let length = if month == 11 { 5 } else { 10 };
            if day < 1 || day > length {
                return Err(CalendarError::DayOutOfRange);
            }
            let rd = Rd(Self::year_start(year) + (i64::from(month) - 1) * 10 + i64::from(day) - 1);
            self.meta().check_range(rd)?;
            Ok(rd)
        }

        fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
            self.meta().check_range(rd)?;
            let mut year = rd.0.div_euclid(100);
            while Self::year_start(year + 1) <= rd.0 {
                year += 1;
            }
            while Self::year_start(year) > rd.0 {
                year -= 1;
            }
            let within = rd.0 - Self::year_start(year);
            Ok((year, (within / 10) as u8 + 1, (within % 10) as u8 + 1))
        }

        fn to_fields(&self, (year, month, day): Self::Date) -> CalendarResult<DateFields> {
            let era = if Self::year_start(year) >= 1_000 {
                "late"
            } else {
                "early"
            };
            Ok(DateFields::ymd(year, month, day).with_era(era))
        }

        fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
            Ok((
                fields.year,
                fields.require_month()?.ordinal,
                fields.require_day()?,
            ))
        }
    }

    fn walk(unit: Unit, from: i64, to: i64) -> Vec<UnitSpan> {
        units(&DynAdapter::new(Toy), unit, Rd(from), Rd(to))
    }

    fn contiguous(spans: &[UnitSpan]) {
        for pair in spans.windows(2) {
            assert_eq!(pair[0].end, pair[1].start, "{pair:?}");
        }
        for span in spans {
            assert!(span.start < span.end, "{span:?}");
        }
    }

    #[test]
    fn months_come_back_whole_with_their_first_days_fields() {
        let spans = walk(Unit::Month, 5, 25);
        contiguous(&spans);
        assert_eq!(spans.len(), 3);
        assert_eq!((spans[0].start, spans[0].end), (Rd(0), Rd(10)));
        assert_eq!((spans[2].start, spans[2].end), (Rd(20), Rd(30)));
        let first = spans[0].date().unwrap();
        assert_eq!(first.fields.month, Some(Month::regular(1)));
        assert_eq!(first.fields.day, Some(1));
        assert!(!first.leap);
        assert_eq!(first.standing, Standing::InUse);
    }

    #[test]
    fn a_leap_year_is_longer_and_flagged() {
        let spans = walk(Unit::Year, 0, 400);
        contiguous(&spans);
        let lengths: Vec<i64> = spans.iter().map(UnitSpan::days).collect();
        assert_eq!(lengths, [105, 100, 100, 100]);
        assert!(spans[0].date().unwrap().leap);
        assert!(!spans[1].date().unwrap().leap);
        // The eleventh month of a leap year is the intercalary one.
        let months = walk(Unit::Month, 100, 105);
        assert_eq!(months.len(), 1);
        assert_eq!((months[0].start, months[0].end), (Rd(100), Rd(105)));
    }

    #[test]
    fn eras_are_found_without_a_hint() {
        let spans = walk(Unit::Era, 900, 1_100);
        contiguous(&spans);
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].date().unwrap().fields.era, Some("early"));
        assert_eq!(spans[1].date().unwrap().fields.era, Some("late"));
        // The era boundary is a year boundary: the first year starting at
        // or after day 1000.
        assert_eq!(spans[0].end, spans[1].start);
        assert!(spans[1].start.0 >= 1_000 && spans[1].start.0 < 1_110);
        assert_eq!(spans[0].start, Rd(FLOOR), "clipped at the floor");
        assert_eq!(spans[1].end, Rd(CEILING + 1), "clipped at the ceiling");
    }

    #[test]
    fn the_calendars_limits_become_refusals() {
        let spans = walk(Unit::Year, -700, -450);
        contiguous(&spans);
        assert_eq!(spans[0].error(), Some(CalendarError::BeforeEpoch));
        assert_eq!((spans[0].start, spans[0].end), (Rd(-700), Rd(FLOOR)));
        assert_eq!(spans[1].start, Rd(FLOOR), "the first year is clipped");
        let spans = walk(Unit::Month, 2_990, 3_020);
        contiguous(&spans);
        let last = spans.last().unwrap();
        assert_eq!(last.error(), Some(CalendarError::AfterSupportedRange));
        assert_eq!((last.start, last.end), (Rd(CEILING + 1), Rd(3_020)));
        assert_eq!(spans[spans.len() - 2].end, Rd(CEILING + 1));
    }

    #[test]
    fn days_are_one_span_each_and_an_empty_range_is_nothing() {
        let spans = walk(Unit::Day, 10, 13);
        assert_eq!(spans.len(), 3);
        assert!(spans.iter().all(|span| span.days() == 1));
        assert!(walk(Unit::Day, 13, 13).is_empty());
        assert!(walk(Unit::Year, 20, 10).is_empty());
    }

    /// A day count: no era, no year, no month.
    #[derive(Debug, Clone, Copy)]
    struct Count;

    impl Calendar for Count {
        type Date = i64;

        fn cycles(&self) -> &'static [CycleShape] {
            &[]
        }

        fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
            Err(CalendarError::UnsupportedField("year"))
        }

        fn meta(&self) -> CalendarMeta {
            CalendarMeta {
                id: CalendarId("count"),
                english_name: "Count",
                year_kind: YearKind::Astronomical,
                has_leap_months: false,
                is_astronomical: false,
                earliest: None,
                latest: None,
                native_locales: &[],
            }
        }

        fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
            Ok(Rd(date))
        }

        fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
            Ok(rd.0)
        }

        fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
            Ok(DateFields::new(date))
        }

        fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
            Ok(fields.year)
        }
    }

    #[test]
    fn a_unit_the_calendar_does_not_have_is_one_refusal() {
        let calendar = DynAdapter::new(Count);
        for (unit, field) in [
            (Unit::Era, "era"),
            (Unit::Year, "year"),
            (Unit::Month, "month"),
        ] {
            let spans = units(&calendar, unit, Rd(0), Rd(1_000));
            assert_eq!(spans.len(), 1, "{unit:?}");
            assert_eq!(
                spans[0].error(),
                Some(CalendarError::UnsupportedField(field))
            );
            assert_eq!((spans[0].start, spans[0].end), (Rd(0), Rd(1_000)));
        }
        assert_eq!(units(&calendar, Unit::Day, Rd(0), Rd(5)).len(), 5);
    }

    #[test]
    fn units_are_named_and_numbered_stably() {
        for (index, unit) in Unit::ALL.iter().enumerate() {
            assert_eq!(unit.index() as usize, index);
            assert_eq!(Unit::from_index(unit.index()), Some(*unit));
        }
        assert_eq!(Unit::from_index(4), None);
        assert_eq!(Unit::Month.name(), "month");
    }
}
