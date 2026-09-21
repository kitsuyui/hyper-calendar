//! Day counts other than the Julian Day: one origin, one number, no months.
//!
//! Every one of these exists because somebody needed a day number that fit
//! in the storage they had. The Julian Day Number passed two million in the
//! seventeenth century, so astronomy shortened it, then NASA shortened it
//! again, then CNES and CCSDS each picked their own epoch, then IBM picked
//! two more for business software. They are trivially inter-convertible and
//! constantly confused, which is exactly why each deserves a name.
//!
//! [`crate::julian_day`] keeps the Julian Day Number and the Modified
//! Julian Date, which have their own types and their own arithmetic. This
//! module is the rest of the family, all of the same shape: an epoch and a
//! number.
//!
//! # Epochs are dates here, not magic numbers
//!
//! Each count below is defined by the *calendar date* of its epoch and the
//! number that date carries, not by an offset from the Julian Day Number.
//! The offsets are the thing everyone transcribes wrongly; the dates are
//! what the defining documents state. The tests then check the classical
//! offsets — Lilian = JDN − 2 299 160, TJD = JDN − 2 440 001 — so both
//! forms have to agree.
//!
//! # A warning the family invites
//!
//! A day number is not an instant, and these do not all start their day at
//! the same time. The Reduced and Dublin counts inherit the Julian Day's
//! noon; the rest begin at midnight. A conversion between two of them that
//! ignores that is wrong by half a day for half of each day.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, DayBoundary, Rd,
    YearKind,
};

use crate::gregorian;

/// A day number in one of the counts below.
///
/// Deliberately not one newtype per count: they are the same quantity in
/// different origins, and a caller who mixes them up is helped by the
/// calendar's identifier, not by a type they would have to name anyway.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DayNumber(pub i64);

/// How far a day count may stray from the Rata Die epoch before the
/// conversion is refused, matching [`crate::julian_day`].
const MAX_MAGNITUDE: i64 = 1 << 44;

/// A day count defined by an epoch date and the number that date carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayCount {
    id: CalendarId,
    english_name: &'static str,
    epoch: Rd,
    first_number: i64,
    boundary: DayBoundary,
    authority: &'static str,
}

impl DayCount {
    /// A count whose epoch is a Gregorian date.
    ///
    /// # Panics
    ///
    /// If the date does not exist. Every use below is a `const`, so that is
    /// a compile error at the definition rather than a surprise later.
    #[must_use]
    pub const fn from_gregorian(
        id: &'static str,
        english_name: &'static str,
        (year, month, day): (i64, u8, u8),
        first_number: i64,
        boundary: DayBoundary,
        authority: &'static str,
    ) -> Self {
        let epoch = match gregorian::to_fixed(year, month, day) {
            Ok(rd) => rd,
            Err(_) => panic!("a day count's epoch must be a real date"),
        };
        Self {
            id: CalendarId(id),
            english_name,
            epoch,
            first_number,
            boundary,
            authority,
        }
    }

    /// This count's identifier.
    #[must_use]
    pub const fn id(self) -> CalendarId {
        self.id
    }

    /// The fixed day this count's epoch falls on.
    #[must_use]
    pub const fn epoch(self) -> Rd {
        self.epoch
    }

    /// The number this count gives its epoch day.
    #[must_use]
    pub const fn first_number(self) -> i64 {
        self.first_number
    }

    /// Who defines this count.
    #[must_use]
    pub const fn authority(self) -> &'static str {
        self.authority
    }

    /// The day number of a fixed day.
    #[must_use]
    pub const fn number_of(self, rd: Rd) -> DayNumber {
        DayNumber(rd.0 - self.epoch.0 + self.first_number)
    }

    /// The fixed day of a day number.
    #[must_use]
    pub const fn day_of(self, number: DayNumber) -> Rd {
        Rd(number.0 - self.first_number + self.epoch.0)
    }
}

hc_core::catalogue! {
    type: DayCount,
    id: |count| count.id.0,
    provenance: |count| count.authority,
    tests: day_count_catalogue,

    /// Every count in this module.
    ///
    /// Generated together with the entries above, so a count cannot be
    /// defined and left off the list — which is the mistake that used to
    /// need a separate edit to avoid.
    pub const ALL;

    /// The count with this identifier.
    pub fn by_id;

    entries: {
    /// The Lilian date: day 1 is 15 October 1582, the first day of the
    /// Gregorian calendar.
    ///
    /// Named for Aloysius Lilius, who designed the reform. IBM defined the
    /// count in 1986 and uses it in `INTDATE(LILIAN)`.
    pub const LILIAN = DayCount::from_gregorian(
        "lilian",
        "Lilian date",
        (1582, 10, 15),
        1,
        DayBoundary::Midnight,
        "Bruce G. Ohms, IBM Systems Journal 25(2) (1986)",
    );

    /// The ANSI date: day 1 is 1 January 1601.
    ///
    /// The other IBM count, `INTDATE(ANSI)`. The epoch is the start of the
    /// Gregorian 400-year cycle containing the reform, which is also why
    /// Windows `FILETIME` counts from the same day.
    pub const ANSI = DayCount::from_gregorian(
        "ansi-date",
        "ANSI date",
        (1601, 1, 1),
        1,
        DayBoundary::Midnight,
        "IBM, INTDATE(ANSI)",
    );

    /// The Dublin Julian Date: day 0 begins at noon on 31 December 1899.
    ///
    /// Adopted by the IAU in 1955, with its epoch written "1900 January 0.5" —
    /// which is the trap. January 0 is 31 December of the year before, and the
    /// .5 is the Julian Day's noon. Writing the epoch as 1 January 1900 puts
    /// every Dublin date a day out, which is what the offset test below caught.
    pub const DUBLIN = DayCount::from_gregorian(
        "dublin-julian-day",
        "Dublin Julian Date",
        (1899, 12, 31),
        0,
        DayBoundary::Noon,
        "IAU General Assembly, Dublin (1955)",
    );

    /// The Reduced Julian Date: day 0 begins at noon on 16 November 1858.
    ///
    /// JD − 2 400 000, so it keeps the Julian Day's noon and differs from the
    /// Modified Julian Date by exactly half a day — which is the whole reason
    /// the two are confused.
    pub const REDUCED = DayCount::from_gregorian(
        "reduced-julian-day",
        "Reduced Julian Date",
        (1858, 11, 16),
        0,
        DayBoundary::Noon,
        "Astronomical usage; JD − 2 400 000",
    );

    /// The Truncated Julian Date: day 0 is 24 May 1968.
    ///
    /// NASA defined it in 1979 for spacecraft telemetry, where four digits were
    /// all that fit.
    pub const TRUNCATED = DayCount::from_gregorian(
        "truncated-julian-day",
        "Truncated Julian Date",
        (1968, 5, 24),
        0,
        DayBoundary::Midnight,
        "NASA (1979)",
    );

    /// The CNES Julian Date: day 0 is 1 January 1950.
    ///
    /// The French space agency's count, used throughout its mission products.
    pub const CNES = DayCount::from_gregorian(
        "cnes-julian-day",
        "CNES Julian Date",
        (1950, 1, 1),
        0,
        DayBoundary::Midnight,
        "Centre national d'études spatiales",
    );

    /// The CCSDS day count: day 0 is 1 January 1958.
    ///
    /// The epoch of CCSDS Day Segmented time codes, and also the epoch TAI was
    /// aligned to UT2 at.
    pub const CCSDS = DayCount::from_gregorian(
        "ccsds-day",
        "CCSDS day count",
        (1958, 1, 1),
        0,
        DayBoundary::Midnight,
        "CCSDS 301.0-B, Time Code Formats",
    );
    }
}

/// A calendar over one of the counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayCountCalendar(pub DayCount);

impl Calendar for DayCountCalendar {
    type Date = DayNumber;

    /// A day count names nothing: it has no months and no week, only a number.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        &[]
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.0.id,
            english_name: self.0.english_name,
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(Rd(-MAX_MAGNITUDE)),
            latest: Some(Rd(MAX_MAGNITUDE)),
        }
    }

    fn day_boundary(&self) -> DayBoundary {
        self.0.boundary
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        let rd = self.0.day_of(date);
        self.meta().check_range(rd)?;
        Ok(rd)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        Ok(self.0.number_of(rd))
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        // The count goes in `year` for the same reason as in
        // `crate::julian_day`: it is the only unbounded signed field a
        // generic consumer is guaranteed to have.
        DateFields::new(date.0).with_extra(
            "julian-day-number",
            self.0.day_of(date).to_julian_day_number(),
        )
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.day.is_some() {
            return Err(CalendarError::UnsupportedField("day"));
        }
        Ok(DayNumber(fields.year))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::julian_day::JulianDayCalendar;

    /// Each count's epoch carries the number its defining document says it
    /// does. This is the definition, so it is the first test.
    ///
    /// Identifier uniqueness and the presence of an authority used to be
    /// asserted here too; both are generated by `hc_core::catalogue!` now,
    /// along with findability, which nothing checked before.
    #[test]
    fn every_epoch_carries_its_stated_number() {
        for count in ALL {
            assert_eq!(
                count.number_of(count.epoch()),
                DayNumber(count.first_number()),
                "{}",
                count.english_name
            );
            assert_eq!(count.day_of(DayNumber(count.first_number())), count.epoch());
        }
    }

    /// The classical offsets from the Julian Day Number, which is how these
    /// are usually quoted. The epochs above and these offsets are two
    /// independent statements of the same fact, so if either were
    /// mistranscribed this would fail. It did, four times over.
    ///
    /// The midnight counts are quoted against a *fractional* Julian Date
    /// ending in .5 — TJD = JD − 2 440 000.5 — and the whole-day offset from
    /// the Julian Day *Number* is therefore one larger, because JDN is
    /// assigned at the following noon. Three of the offsets here were
    /// transcribed from the fractional form and were a day short. And the
    /// Dublin epoch is written "1900 January 0.5", which is 31 December
    /// 1899, not 1 January 1900.
    #[test]
    fn the_offsets_from_the_julian_day_number_are_the_published_ones() {
        let day = gregorian::to_fixed(2024, 1, 1).expect("exists");
        let jdn = JulianDayCalendar.from_fixed(day).expect("in range").0;
        for (count, offset) in [
            (LILIAN, 2_299_160),
            (ANSI, 2_305_813),
            (DUBLIN, 2_415_020),
            (REDUCED, 2_400_000),
            (TRUNCATED, 2_440_001),
            (CNES, 2_433_283),
            (CCSDS, 2_436_205),
        ] {
            assert_eq!(
                count.number_of(day),
                DayNumber(jdn - offset),
                "{} should be JDN − {offset}",
                count.english_name
            );
        }
    }

    /// The noon counts and the midnight counts are both here, and mixing
    /// them is the mistake the module header warns about.
    #[test]
    fn the_counts_disagree_about_when_a_day_starts() {
        assert_eq!(DayCountCalendar(REDUCED).day_boundary(), DayBoundary::Noon);
        assert_eq!(DayCountCalendar(DUBLIN).day_boundary(), DayBoundary::Noon);
        for count in [LILIAN, ANSI, TRUNCATED, CNES, CCSDS] {
            assert_eq!(
                DayCountCalendar(count).day_boundary(),
                DayBoundary::Midnight,
                "{}",
                count.english_name
            );
        }
    }

    /// The Reduced Julian Date and the Modified Julian Date are half a day
    /// apart, which as whole day numbers means the reduced count is one
    /// ahead on the same fixed day.
    #[test]
    fn the_reduced_and_modified_counts_differ_by_one_whole_day() {
        for offset in [-10_000, -1, 0, 1, 10_000, 100_000] {
            let rd = Rd(gregorian::to_fixed(2000, 1, 1).expect("exists").0 + offset);
            let reduced = REDUCED.number_of(rd).0;
            let modified = rd.to_modified_julian_day();
            assert_eq!(reduced - modified, 1, "{rd}");
        }
    }

    #[test]
    fn every_count_round_trips_over_a_wide_span() {
        for count in ALL {
            let calendar = DayCountCalendar(*count);
            for offset in (-400_000..400_000).step_by(9_973) {
                let rd = Rd(offset);
                let number = calendar.from_fixed(rd).expect("in range");
                assert_eq!(
                    calendar.to_fixed(number),
                    Ok(rd),
                    "{} {rd}",
                    count.english_name
                );
            }
        }
    }

    #[test]
    fn a_day_count_has_no_day_of_the_month() {
        let calendar = DayCountCalendar(LILIAN);
        let fields = DateFields {
            day: Some(1),
            ..DateFields::new(1)
        };
        assert_eq!(
            calendar.from_fields(&fields),
            Err(CalendarError::UnsupportedField("day"))
        );
    }
}
