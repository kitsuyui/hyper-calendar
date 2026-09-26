//! Weekdays, and the week cycles that are not seven days long.

use core::fmt;

use crate::fixed::Rd;

/// A day of the seven-day week.
///
/// The seven-day week is one of the very few cycles that has run unbroken
/// across calendar reforms — the Gregorian reform of 1582 skipped ten dates
/// but not a single weekday — so it can be derived from [`Rd`] alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Weekday {
    /// Monday. ISO 8601 day 1.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday. ISO 8601 day 7.
    Sunday,
}

impl Weekday {
    /// All seven days, Monday first.
    pub const ALL: [Self; 7] = [
        Self::Monday,
        Self::Tuesday,
        Self::Wednesday,
        Self::Thursday,
        Self::Friday,
        Self::Saturday,
        Self::Sunday,
    ];

    /// The weekday of a fixed day.
    ///
    /// `Rd(1)` — `0001-01-01` proleptic Gregorian — was a Monday.
    #[must_use]
    pub const fn from_rd(rd: Rd) -> Self {
        match (rd.0 - 1).rem_euclid(7) {
            0 => Self::Monday,
            1 => Self::Tuesday,
            2 => Self::Wednesday,
            3 => Self::Thursday,
            4 => Self::Friday,
            5 => Self::Saturday,
            _ => Self::Sunday,
        }
    }

    /// The ISO 8601 number, Monday = 1 through Sunday = 7.
    #[must_use]
    pub const fn iso_number(self) -> u8 {
        match self {
            Self::Monday => 1,
            Self::Tuesday => 2,
            Self::Wednesday => 3,
            Self::Thursday => 4,
            Self::Friday => 5,
            Self::Saturday => 6,
            Self::Sunday => 7,
        }
    }

    /// The C `tm_wday` number, Sunday = 0 through Saturday = 6.
    #[must_use]
    pub const fn sunday_first_number(self) -> u8 {
        self.iso_number() % 7
    }

    /// The zero-based number from Monday, Monday = 0 through Sunday = 6.
    ///
    /// This is Python's `date.weekday()` and `struct_time.tm_wday`, which
    /// differ from C's `tm_wday` (see [`Weekday::sunday_first_number`]) in
    /// where the week starts.
    #[must_use]
    pub const fn monday_first_number(self) -> u8 {
        self.iso_number() - 1
    }

    /// Build a weekday from an ISO number.
    #[must_use]
    pub const fn from_iso_number(number: u8) -> Option<Self> {
        match number {
            1 => Some(Self::Monday),
            2 => Some(Self::Tuesday),
            3 => Some(Self::Wednesday),
            4 => Some(Self::Thursday),
            5 => Some(Self::Friday),
            6 => Some(Self::Saturday),
            7 => Some(Self::Sunday),
            _ => None,
        }
    }

    /// The English name, for diagnostics. Localised names come from
    /// `hc-i18n`.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Sunday => "Sunday",
        }
    }

    /// The last occurrence of this weekday on or before `rd`.
    #[must_use]
    pub const fn on_or_before(self, rd: Rd) -> Rd {
        let target = self.iso_number() as i64;
        let offset = (rd.0 - target).rem_euclid(7);
        Rd(rd.0 - offset)
    }

    /// The next occurrence of this weekday on or after `rd`.
    #[must_use]
    pub const fn on_or_after(self, rd: Rd) -> Rd {
        Rd(self.on_or_before(Rd(rd.0 + 6)).0)
    }

    /// The next occurrence strictly after `rd`.
    #[must_use]
    pub const fn after(self, rd: Rd) -> Rd {
        Rd(self.on_or_before(Rd(rd.0 + 7)).0)
    }

    /// The previous occurrence strictly before `rd`.
    #[must_use]
    pub const fn before(self, rd: Rd) -> Rd {
        Rd(self.on_or_before(Rd(rd.0 - 1)).0)
    }

    /// The `n`-th occurrence of this weekday within `[start, end]`.
    ///
    /// `n` counts from 1. A negative `n` counts back from the end, so `-1` is
    /// "the last Monday of the month". Returns `None` when there is no such
    /// occurrence.
    #[must_use]
    pub const fn nth_within(self, n: i32, start: Rd, end: Rd) -> Option<Rd> {
        if n == 0 {
            return None;
        }
        let candidate = if n > 0 {
            Rd(self.on_or_after(start).0 + 7 * (n as i64 - 1))
        } else {
            Rd(self.on_or_before(end).0 + 7 * (n as i64 + 1))
        };
        if candidate.0 < start.0 || candidate.0 > end.0 {
            None
        } else {
            Some(candidate)
        }
    }

    /// Whether this day falls on the weekend under the most common
    /// Saturday-Sunday convention.
    ///
    /// Weekends are a *civil* convention, not a property of the week: Friday
    /// and Saturday in much of the Middle East, Sunday alone in Nepal and
    /// historically in Israel. Use `hc-holiday`'s per-region weekend rules
    /// when correctness matters.
    #[must_use]
    pub const fn is_saturday_sunday_weekend(self) -> bool {
        matches!(self, Self::Saturday | Self::Sunday)
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.english_name())
    }
}

/// A cyclic day-count that is not seven days long.
///
/// Balinese Pawukon runs ten concurrent week cycles from one to ten days;
/// the Igbo calendar uses a four-day week; the French Republican calendar
/// used ten. This helper turns any of them into a position from a known
/// anchor day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayCycle {
    /// The cycle length in days.
    pub length: u16,
    /// A fixed day whose position within the cycle is zero.
    pub anchor: Rd,
}

impl DayCycle {
    /// Build a cycle from its length and anchor.
    #[must_use]
    pub const fn new(length: u16, anchor: Rd) -> Self {
        Self { length, anchor }
    }

    /// The zero-based position of `rd` within the cycle.
    ///
    /// Returns `None` for a zero-length cycle.
    #[must_use]
    pub const fn position(self, rd: Rd) -> Option<u16> {
        if self.length == 0 {
            return None;
        }
        Some((rd.0 - self.anchor.0).rem_euclid(self.length as i64) as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Python's documentation: `date(2002, 12, 4).weekday()` is `2` and
    /// `.isoweekday()` is `3` — a Wednesday.
    #[test]
    fn the_monday_first_number_is_python_s_weekday() {
        let wednesday = Weekday::from_rd(crate::gregorian::to_fixed(2002, 12, 4).unwrap());
        assert_eq!(wednesday.monday_first_number(), 2);
        assert_eq!(wednesday.iso_number(), 3);
        assert_eq!(Weekday::Monday.monday_first_number(), 0);
        assert_eq!(Weekday::Sunday.monday_first_number(), 6);
    }

    #[test]
    fn the_rata_die_epoch_was_a_monday() {
        assert_eq!(Weekday::from_rd(Rd(1)), Weekday::Monday);
    }

    #[test]
    fn the_unix_epoch_was_a_thursday() {
        assert_eq!(Weekday::from_rd(Rd::UNIX_EPOCH), Weekday::Thursday);
    }

    #[test]
    fn weekdays_cycle_forwards_and_backwards() {
        for offset in -14i64..=14 {
            let rd = Rd(Rd::UNIX_EPOCH.0 + offset);
            let expected = Weekday::ALL[((offset + 3).rem_euclid(7)) as usize];
            assert_eq!(Weekday::from_rd(rd), expected, "offset {offset}");
        }
    }

    #[test]
    fn iso_numbers_round_trip() {
        for day in Weekday::ALL {
            assert_eq!(Weekday::from_iso_number(day.iso_number()), Some(day));
        }
        assert_eq!(Weekday::from_iso_number(0), None);
        assert_eq!(Weekday::from_iso_number(8), None);
    }

    #[test]
    fn sunday_first_numbering_matches_c() {
        assert_eq!(Weekday::Sunday.sunday_first_number(), 0);
        assert_eq!(Weekday::Monday.sunday_first_number(), 1);
        assert_eq!(Weekday::Saturday.sunday_first_number(), 6);
    }

    #[test]
    fn on_or_before_is_idempotent_on_a_matching_day() {
        let thursday = Rd::UNIX_EPOCH;
        assert_eq!(Weekday::Thursday.on_or_before(thursday), thursday);
        assert_eq!(Weekday::Thursday.on_or_after(thursday), thursday);
        assert_eq!(Weekday::Thursday.after(thursday), thursday + 7);
        assert_eq!(Weekday::Thursday.before(thursday), thursday - 7);
    }

    #[test]
    fn nth_within_finds_positional_days() {
        // January 1970: the 1st was a Thursday, so the month runs RD 719163
        // through RD 719193.
        let start = Rd::UNIX_EPOCH;
        let end = Rd(Rd::UNIX_EPOCH.0 + 30);
        assert_eq!(Weekday::Monday.nth_within(1, start, end), Some(start + 4));
        assert_eq!(Weekday::Monday.nth_within(-1, start, end), Some(start + 25));
        assert_eq!(Weekday::Monday.nth_within(5, start, end), None);
        assert_eq!(Weekday::Monday.nth_within(0, start, end), None);
    }

    #[test]
    fn day_cycles_of_other_lengths_work() {
        let four_day = DayCycle::new(4, Rd::UNIX_EPOCH);
        assert_eq!(four_day.position(Rd::UNIX_EPOCH), Some(0));
        assert_eq!(four_day.position(Rd::UNIX_EPOCH + 5), Some(1));
        assert_eq!(four_day.position(Rd::UNIX_EPOCH - 1), Some(3));
        assert_eq!(DayCycle::new(0, Rd(0)).position(Rd(5)), None);
    }
}
