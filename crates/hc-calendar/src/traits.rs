//! The two calendar interfaces, and the bridge between them.

use core::fmt;
use core::marker::PhantomData;

use crate::daystart::{DayBoundary, Standing, Usage};
use crate::error::{CalendarError, CalendarResult};
use crate::fields::{DateFields, YearKind};
use crate::fixed::Rd;
use crate::shape::CycleShape;

/// A stable machine identifier for a calendar.
///
/// Where the Unicode CLDR already has a name — `gregory`, `islamic-civil`,
/// `japanese`, `chinese` — this crate uses it, so that `hyper-calendar`
/// values interoperate with `Intl.DateTimeFormat` and ICU without a
/// translation table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CalendarId(pub &'static str);

impl CalendarId {
    /// The identifier as a string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for CalendarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Descriptive facts about a calendar that do not depend on any date.
///
/// This is the "data" half of the data/algorithm split at the level of the
/// calendar itself: a caller can decide whether a calendar is usable for a
/// task without running a single conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarMeta {
    /// The machine identifier.
    pub id: CalendarId,
    /// The English name of the calendar.
    pub english_name: &'static str,
    /// How the calendar counts years.
    pub year_kind: YearKind,
    /// Whether some years contain an intercalary month.
    pub has_leap_months: bool,
    /// Whether the calendar's rules depend on astronomical observation or
    /// computation rather than arithmetic alone.
    ///
    /// Astronomical calendars are exact only to the precision of the model
    /// behind them, and historical dates may disagree with what was actually
    /// proclaimed at the time.
    pub is_astronomical: bool,
    /// The earliest fixed day this implementation will convert, if bounded.
    pub earliest: Option<Rd>,
    /// The latest fixed day this implementation will convert, if bounded.
    pub latest: Option<Rd>,
}

impl CalendarMeta {
    /// Whether `rd` is inside the supported range.
    #[must_use]
    pub fn supports(&self, rd: Rd) -> bool {
        self.earliest.is_none_or(|first| rd >= first) && self.latest.is_none_or(|last| rd <= last)
    }

    /// Check `rd` against the supported range.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`].
    pub fn check_range(&self, rd: Rd) -> CalendarResult<()> {
        if self.earliest.is_some_and(|first| rd < first) {
            return Err(CalendarError::BeforeEpoch);
        }
        if self.latest.is_some_and(|last| rd > last) {
            return Err(CalendarError::AfterSupportedRange);
        }
        Ok(())
    }
}

/// A calendar with its own date type.
///
/// Implementing this is all a calendar author has to do: the dynamic
/// interface, the registry entry and the cross-calendar conversions all
/// follow from `to_fixed` and `from_fixed`.
///
/// # Contract
///
/// For every `date` the calendar considers valid,
/// `from_fixed(to_fixed(date)) == date`, and for every `rd` within
/// [`CalendarMeta::earliest`]..=[`CalendarMeta::latest`],
/// `to_fixed(from_fixed(rd)) == rd`. Implementations are expected to test
/// both directions over their whole supported range.
// `from_fixed` and `from_fields` take `&self` because a calendar instance can
// carry configuration — an observation meridian, a reform date, an era table.
// The names are the ones *Calendrical Calculations* and every port of it use,
// so renaming them to satisfy the lint would cost more than it buys.
#[allow(clippy::wrong_self_convention)]
pub trait Calendar {
    /// The calendar's own date representation.
    type Date: Clone + Copy + fmt::Debug + PartialEq;

    /// Facts about this calendar.
    fn meta(&self) -> CalendarMeta;

    /// Convert a date in this calendar to a fixed day.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd>;

    /// Convert a fixed day to a date in this calendar.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the day is outside the supported
    /// range.
    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date>;

    /// Express one of this calendar's dates as generic fields.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date cannot be described.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields>;

    /// Interpret generic fields as one of this calendar's dates.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the fields are missing, extraneous or
    /// out of range.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date>;

    /// The positional cycles this calendar runs: months, weekdays, and
    /// whatever else its dates are built out of.
    ///
    /// This is what a vocabulary is keyed to. A calendar that declares
    /// nineteen months can be given nineteen month names; one that
    /// declares a ten-day week can be given ten. See [`crate::shape`] for
    /// why the shape is the calendar's to state rather than an assumption
    /// baked into the name tables.
    ///
    /// There is no default. A calendar that has no named cycles — a day
    /// count — says so with an empty slice; a calendar that says nothing
    /// does not compile. A defaulted method would let a calendar stay
    /// silent, and a gap that a test can only report is still a gap.
    fn cycles(&self) -> &'static [CycleShape];

    /// Where this calendar's day begins.
    ///
    /// Defaults to midnight, which is right for most calendars and for every
    /// purely arithmetic one. Override it where it is not: the Julian Day
    /// begins at noon, the Hebrew and Islamic days at sunset, the traditional
    /// Chinese day at 23:00.
    ///
    /// This names the convention. Resolving a solar boundary to an instant
    /// needs a location and an ephemeris, which is `hc-astro`'s job — see
    /// [`DayBoundary::needs_observation`].
    fn day_boundary(&self) -> DayBoundary {
        DayBoundary::Midnight
    }

    /// When this calendar was actually in use.
    ///
    /// Distinct from [`CalendarMeta::earliest`] and
    /// [`CalendarMeta::latest`], which bound where the *arithmetic* is
    /// defined. The two are rarely the same: the Gregorian calendar computes
    /// happily for the year 3000 BC and nobody used it before 1582.
    ///
    /// Defaults to [`Usage::UNRECORDED`], which is honest for a proposed
    /// calendar or a pure day count and wrong for a historical one — so a
    /// historical calendar should override it.
    fn usage(&self) -> Usage {
        Usage::UNRECORDED
    }

    /// Whether a day falls inside the period this calendar was used in.
    ///
    /// The arithmetic will answer for any day in range; this says whether the
    /// answer is a historical reading or a projection. Today in the Shōwa era
    /// is [`Standing::Extended`]: perfectly computable, and not what anyone
    /// writes.
    fn standing(&self, rd: Rd) -> Standing {
        self.usage().standing(rd)
    }

    /// Convert a date straight into another calendar.
    ///
    /// # Errors
    ///
    /// Propagates errors from either side of the conversion.
    fn convert_to<C: Calendar>(&self, date: Self::Date, other: &C) -> CalendarResult<C::Date> {
        other.from_fixed(self.to_fixed(date)?)
    }
}

/// The object-safe calendar interface.
///
/// Everything here speaks [`DateFields`] and [`Rd`], so a `dyn DynCalendar`
/// can be stored in a registry, selected by string at run time, or reached
/// across an FFI boundary.
pub trait DynCalendar {
    /// Facts about this calendar.
    fn meta(&self) -> CalendarMeta;

    /// Convert generic fields to a fixed day.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    fn fields_to_fixed(&self, fields: &DateFields) -> CalendarResult<Rd>;

    /// Convert a fixed day to generic fields.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the day is outside the supported
    /// range.
    fn fixed_to_fields(&self, rd: Rd) -> CalendarResult<DateFields>;

    /// The number of days in the given month of the given year.
    ///
    /// The default walks the month boundaries through fixed days, which works
    /// for every calendar but is slower than a closed form. Calendars with a
    /// cheap rule should override it.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the month does not exist.
    fn days_in_month(&self, fields: &DateFields) -> CalendarResult<u16> {
        let mut probe = *fields;
        probe.day = Some(1);
        let first = self.fields_to_fixed(&probe)?;
        let mut length = 1u16;
        loop {
            let next = Rd(first.0 + length as i64);
            let next_fields = self.fixed_to_fields(next)?;
            if next_fields.month != probe.month || next_fields.year != probe.year {
                return Ok(length);
            }
            length += 1;
            if length > 400 {
                return Err(CalendarError::MonthOutOfRange);
            }
        }
    }

    /// The number of days in the given year.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the year does not exist.
    fn days_in_year(&self, year: i64) -> CalendarResult<u16>;

    /// The cycles this calendar runs. See [`Calendar::cycles`].
    fn cycles(&self) -> &'static [CycleShape];

    /// Where this calendar's day begins. See [`Calendar::day_boundary`].
    fn day_boundary(&self) -> DayBoundary {
        DayBoundary::Midnight
    }

    /// When this calendar was actually in use. See [`Calendar::usage`].
    fn usage(&self) -> Usage {
        Usage::UNRECORDED
    }

    /// How a day relates to that period. See [`Calendar::standing`].
    fn standing(&self, rd: Rd) -> Standing {
        self.usage().standing(rd)
    }

    /// Whether the given year is longer than an ordinary one.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the year does not exist.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool>;
}

/// Turns any [`Calendar`] into a [`DynCalendar`].
///
/// Calendar authors never write this bridge; they wrap their type in
/// `DynAdapter` when they need dynamic dispatch.
///
/// ```
/// # use hc_calendar::{Calendar, DynAdapter, DynCalendar};
/// # fn demo<C: Calendar>(calendar: C) -> DynAdapter<C> {
/// DynAdapter::new(calendar)
/// # }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct DynAdapter<C> {
    inner: C,
}

impl<C> DynAdapter<C> {
    /// Wrap a calendar.
    pub const fn new(inner: C) -> Self {
        Self { inner }
    }

    /// The wrapped calendar.
    pub const fn inner(&self) -> &C {
        &self.inner
    }
}

impl<C: Calendar> DynCalendar for DynAdapter<C> {
    fn meta(&self) -> CalendarMeta {
        self.inner.meta()
    }

    fn fields_to_fixed(&self, fields: &DateFields) -> CalendarResult<Rd> {
        let date = self.inner.from_fields(fields)?;
        self.inner.to_fixed(date)
    }

    fn fixed_to_fields(&self, rd: Rd) -> CalendarResult<DateFields> {
        let date = self.inner.from_fixed(rd)?;
        self.inner.to_fields(date)
    }

    fn cycles(&self) -> &'static [CycleShape] {
        self.inner.cycles()
    }

    fn day_boundary(&self) -> DayBoundary {
        self.inner.day_boundary()
    }

    fn usage(&self) -> Usage {
        self.inner.usage()
    }

    fn days_in_year(&self, year: i64) -> CalendarResult<u16> {
        let start = self.fields_to_fixed(&DateFields::ymd(year, 1, 1))?;
        let next = self.fields_to_fixed(&DateFields::ymd(year + 1, 1, 1))?;
        u16::try_from(next.0 - start.0).map_err(|_| CalendarError::YearOutOfRange)
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        let this = self.days_in_year(year)?;
        let previous = self.days_in_year(year - 1)?;
        Ok(this > previous)
    }
}

/// A phantom-typed handle to a calendar, useful where a calendar must be
/// named in a type but never instantiated.
#[derive(Debug, Clone, Copy, Default)]
pub struct CalendarHandle<C>(PhantomData<C>);

impl<C> CalendarHandle<C> {
    /// A new handle.
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A one-day-per-year toy calendar, enough to exercise the contracts
    /// without depending on a real calendar crate.
    #[derive(Debug, Clone, Copy)]
    struct Decimal;

    impl Calendar for Decimal {
        type Date = (i64, u8, u8);

        fn cycles(&self) -> &'static [CycleShape] {
            const SHAPE: &[CycleShape] = &[
                CycleShape::fixed(crate::shape::MONTH, 10),
                CycleShape::fixed("decimal-day", 10),
            ];
            SHAPE
        }

        fn meta(&self) -> CalendarMeta {
            CalendarMeta {
                id: CalendarId("test-decimal"),
                english_name: "Decimal test calendar",
                year_kind: YearKind::Astronomical,
                has_leap_months: false,
                is_astronomical: false,
                earliest: None,
                latest: None,
            }
        }

        fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
            let (year, month, day) = date;
            if !(1..=10).contains(&month) {
                return Err(CalendarError::MonthOutOfRange);
            }
            if !(1..=10).contains(&day) {
                return Err(CalendarError::DayOutOfRange);
            }
            Ok(Rd(year * 100 + (month as i64 - 1) * 10 + day as i64 - 1))
        }

        fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
            let year = rd.0.div_euclid(100);
            let within = rd.0.rem_euclid(100);
            Ok((year, (within / 10) as u8 + 1, (within % 10) as u8 + 1))
        }

        fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
            Ok(DateFields::ymd(date.0, date.1, date.2))
        }

        fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
            Ok((
                fields.year,
                fields.require_month()?.ordinal,
                fields.require_day()?,
            ))
        }
    }

    #[test]
    fn round_trips_hold_in_both_directions() {
        let calendar = Decimal;
        for rd in 0..1_000 {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date).unwrap(), Rd(rd));
        }
    }

    #[test]
    fn the_adapter_derives_the_dynamic_interface() {
        let dynamic = DynAdapter::new(Decimal);
        let fields = dynamic.fixed_to_fields(Rd(123)).unwrap();
        assert_eq!(fields.year, 1);
        assert_eq!(fields.month.unwrap().ordinal, 3);
        assert_eq!(fields.day.unwrap(), 4);
        assert_eq!(dynamic.fields_to_fixed(&fields).unwrap(), Rd(123));
    }

    #[test]
    fn the_default_days_in_month_walks_the_boundary() {
        let dynamic = DynAdapter::new(Decimal);
        let fields = DateFields::ymd(3, 5, 1);
        assert_eq!(dynamic.days_in_month(&fields).unwrap(), 10);
        assert_eq!(dynamic.days_in_year(3).unwrap(), 100);
        assert!(!dynamic.is_leap_year(3).unwrap());
    }

    #[test]
    fn calendars_convert_straight_into_one_another() {
        let calendar = Decimal;
        let other = Decimal;
        let date = calendar.from_fixed(Rd(456)).unwrap();
        assert_eq!(calendar.convert_to(date, &other).unwrap(), date);
    }

    #[test]
    fn range_checks_use_the_metadata() {
        let meta = CalendarMeta {
            id: CalendarId("bounded"),
            english_name: "Bounded",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(Rd(10)),
            latest: Some(Rd(20)),
        };
        assert!(meta.supports(Rd(15)));
        assert_eq!(meta.check_range(Rd(9)), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            meta.check_range(Rd(21)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(meta.check_range(Rd(10)), Ok(()));
    }

    #[test]
    fn invalid_fields_are_rejected() {
        let calendar = Decimal;
        assert_eq!(
            calendar.to_fixed((1, 11, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            calendar.to_fixed((1, 1, 0)),
            Err(CalendarError::DayOutOfRange)
        );
    }
}
