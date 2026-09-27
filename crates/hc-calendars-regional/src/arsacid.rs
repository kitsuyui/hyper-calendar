//! The Arsacid era in its Babylonian form — `arsacid-era`.
//!
//! The Parthian dynastic era, "beginning with the vernal equinox (in Babylon
//! with 1 Nisan = 14 April) 247 BCE", and written at Babylon on the
//! Babylonian months beside the Seleucid year, "a Babylonian tablet equating
//! the Seleucid year 208 with 144 of the Arsacid era" (*Encyclopaedia
//! Iranica*, "Arsacids v. The Arsacid era", `iranica-arsacid-era`, read
//! 2026-09-26 in the Wayback Machine's copy). So the calendar is
//! [`hc_calendars_lunar::babylonian`]'s, day for day and month for month,
//! with the Seleucid year less [`SELEUCID_OFFSET`]: 1 Nisannu of Seleucid
//! year 65 is 1 Nisannu of Arsacid year 1. It converts what `babylonian`
//! converts from that day, AE 1 to 322, and refuses the rest.
//!
//! Its correctness on every day rests on `babylonian`'s own sweep, which
//! walks every day of `babylonian`'s range, this one included, in a release
//! build (docs/policy.md §7). This module adds no computation to
//! `babylonian`'s: [`to_fixed`] and [`from_fixed`] are `babylonian`'s with
//! the year shifted by the constant [`SELEUCID_OFFSET`], behind range
//! checks that act only at the two ends. So inside the range an Arsacid
//! date is the Babylonian date renamed, and it converts back to its day
//! exactly when the Babylonian date does. Walking every day here as well
//! would repeat `babylonian`'s crescent searches, about 500 CPU-seconds,
//! and prove nothing new. The tests check what the wrapper itself can get
//! wrong: the range checks at both ends; the year, the fields and the era
//! on every 1 Nisannu and the day before it, where the year changes; and
//! the wrapper relation itself on a sample of days. A change that makes this
//! calendar more than `babylonian` renamed fails that last test, and would
//! need a day-by-day sweep of its own.
//!
//! `babylonian` puts 1 Nisannu SE 65 on 15 April 247 BCE, a day after the
//! article's 14 April: a disagreement of the size and direction of the 941
//! of Parker and Dubberstein's 5 664 months that the moonlag criterion
//! begins a day later than their table. The article does not say where its
//! date comes from.
//!
//! The Iranian form of the era, on the Zoroastrian months of a wandering
//! year, is not carried: Grumel's epoch for it and the one dated example
//! read disagree by a year. The system document, `docs/systems/
//! seleucid-eras.md` in the repository, works both through.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    Usage, YearKind,
};
use hc_calendars_lunar::babylonian;

/// The machine identifier of this calendar.
pub const ID: CalendarId = CalendarId("arsacid-era");

/// The era code of the Arsacid era.
pub const ERA: &str = "arsacid";

/// The Seleucid year less the Arsacid year: SE 208 is AE 144.
pub const SELEUCID_OFFSET: i64 = 64;

/// The earliest Arsacid year converted, the first.
pub const MIN_YEAR: i64 = 1;

/// The latest Arsacid year converted, `babylonian`'s last, SE 386.
pub const MAX_YEAR: i64 = babylonian::MAX_YEAR - SELEUCID_OFFSET;

/// The Seleucid year of Arsacid year `year`.
#[must_use]
pub const fn seleucid_year(year: i64) -> i64 {
    year + SELEUCID_OFFSET
}

/// The fixed day of an Arsacid date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside AE 1 to
/// [`MAX_YEAR`], and `babylonian`'s errors for a month or day the year does
/// not have.
pub fn to_fixed(year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    babylonian::to_fixed(seleucid_year(year), month, day)
}

/// The first day of Arsacid year 1, 1 Nisannu SE 65, as `babylonian`
/// converts it.
///
/// Written down rather than converted, since every conversion's range
/// check and every description's metadata ask for it and the conversion
/// costs about a millisecond; `tests::the_written_first_day_is_the_converted_one`
/// converts it again.
pub const EARLIEST: Rd = Rd(-90_114);

/// The first day of Arsacid year 1, [`EARLIEST`].
///
/// # Errors
///
/// None: the day is written down. The `Result` is the one `babylonian`'s
/// conversion of the day would have.
pub const fn earliest() -> CalendarResult<Rd> {
    Ok(EARLIEST)
}

/// The Arsacid year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before 1 Nisannu AE 1 and
/// `babylonian`'s [`CalendarError::AfterSupportedRange`] after its range.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, Month, u8)> {
    if rd < earliest()? {
        return Err(CalendarError::BeforeEpoch);
    }
    let (year, month, day) = babylonian::from_fixed(rd)?;
    Ok((year - SELEUCID_OFFSET, month, day))
}

/// A date of the Arsacid era.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArsacidDate {
    /// The Arsacid year, from 1.
    pub year: i64,
    /// The Babylonian month, as `babylonian` numbers it.
    pub month: Month,
    /// The day of the month, from 1.
    pub day: u8,
}

/// The Arsacid era on the Babylonian months.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ArsacidCalendar;

impl Calendar for ArsacidCalendar {
    type Date = ArsacidDate;

    /// Unrecorded as a period of days: the source dates the era's use by
    /// its documents — a tablet of SE 208, ostraca of about 100 BCE to CE
    /// 13, letters and contracts of CE 21 and later — never by a first or
    /// last day.
    fn usage(&self) -> Usage {
        Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// A year with a second Addaru or a second Ulūlu, `babylonian`'s rule
    /// on the Seleucid year.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(babylonian::is_leap_year(seleucid_year(year)))
    }

    /// `babylonian`'s day, from sunset, named by the day it ends on.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset(hc_calendar::DayNaming::ByEnd)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Arsacid era (Babylonian)",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: earliest().ok(),
            latest: Some(babylonian::LATEST),
            native_locales: &["akk"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(ArsacidDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut fields = DateFields::new(date.year).with_era(ERA);
        fields.month = Some(date.month);
        fields.day = Some(date.day);
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let (month, day) = (fields.require_month()?, fields.require_day()?);
        to_fixed(fields.year, month, day)?;
        Ok(ArsacidDate {
            year: fields.year,
            month,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::julian;

    #[test]
    fn the_written_first_day_is_the_converted_one() {
        assert_eq!(to_fixed(MIN_YEAR, Month::regular(1), 1), Ok(EARLIEST));
    }

    #[test]
    fn the_arsacid_year_is_the_seleucid_less_sixty_four() {
        // "A Babylonian tablet equating the Seleucid year 208 with 144 of
        // the Arsacid era". The tablet's month is not given, so every month
        // of SE 208 is tried.
        let nisannu = babylonian::to_fixed(208, Month::regular(1), 1).unwrap();
        let next = babylonian::to_fixed(209, Month::regular(1), 1).unwrap();
        for rd in [nisannu.0, (nisannu.0 + next.0) / 2, next.0 - 1] {
            let (year, _, _) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(year, 144);
            assert_eq!(babylonian::from_fixed(Rd(rd)).unwrap().0, 208);
        }
        assert_eq!(MAX_YEAR, 322);
    }

    #[test]
    fn the_arsacid_era_begins_with_nisannu_of_se_65() {
        let first = earliest().unwrap();
        assert_eq!(
            first,
            babylonian::to_fixed(65, Month::regular(1), 1).unwrap()
        );
        assert_eq!(from_fixed(first), Ok((1, Month::regular(1), 1)));
        // The article's "1 Nisan = 14 April" 247 BCE is a day earlier: the
        // moonlag criterion begins 941 of Parker and Dubberstein's 5 664
        // months a day after their table, and this may be one of them.
        assert_eq!(julian::from_fixed(first), Ok((-246, 4, 15)));
        assert_eq!(from_fixed(Rd(first.0 - 1)), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            from_fixed(Rd(babylonian::LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            to_fixed(0, Month::regular(1), 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(MAX_YEAR + 1, Month::regular(1), 1),
            Err(CalendarError::YearOutOfRange)
        );
        // The last day is `babylonian`'s last, in AE 322.
        assert_eq!(
            from_fixed(babylonian::LATEST).map(|(year, _, _)| year),
            Ok(MAX_YEAR)
        );
        let calendar = ArsacidCalendar;
        for year in [MIN_YEAR - 1, MAX_YEAR + 1] {
            assert_eq!(
                calendar.is_leap_year(year),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                calendar.from_fields(&DateFields::ymd(year, 1, 1).with_era(ERA)),
                Err(CalendarError::YearOutOfRange)
            );
        }
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(100, 1, 1).with_era("se")),
            Err(CalendarError::UnknownEra)
        );
    }

    /// Checks one day inside the range: the Arsacid date is `babylonian`'s
    /// with the year less [`SELEUCID_OFFSET`], it converts back to the day,
    /// directly and through its fields, and its year has `babylonian`'s
    /// leap rule. Returns the date.
    fn is_babylonian_renamed(rd: i64) -> ArsacidDate {
        let calendar = ArsacidCalendar;
        let date = calendar.from_fixed(Rd(rd)).unwrap();
        let (se, month, day) = babylonian::from_fixed(Rd(rd)).unwrap();
        assert_eq!(
            (date.year + SELEUCID_OFFSET, date.month, date.day),
            (se, month, day),
            "RD {rd}"
        );
        assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "RD {rd}");
        let fields = calendar.to_fields(date).unwrap();
        assert_eq!(fields.era, Some(ERA));
        assert_eq!(calendar.from_fields(&fields), Ok(date), "RD {rd}");
        assert_eq!(
            calendar.is_leap_year(date.year),
            Ok(babylonian::is_leap_year(se))
        );
        date
    }

    #[test]
    fn every_new_year_and_its_eve_is_babylonians_renamed() {
        // Every 1 Nisannu, AE 1 to 322, and the day before it, the last of
        // the year before: the days where the wrapper's year arithmetic
        // and range checks act. The day before AE 1 is refused, and the
        // range ends on the eve of 1 Nisannu SE 387, `babylonian`'s last day.
        let ended = |year: i64| {
            let se = year - 1 + SELEUCID_OFFSET;
            (
                babylonian::is_leap_year(se),
                babylonian::has_second_ululu(se),
            )
        };
        // Years that end common, with a second Addaru and with a second
        // Ulūlu are all among the eves.
        for kind in [(false, false), (true, false), (true, true)] {
            assert!((2..=MAX_YEAR).any(|year| ended(year) == kind));
        }
        let years: alloc::vec::Vec<i64> = (MIN_YEAR..=MAX_YEAR).collect();
        crate::check_days(&years, |year| {
            let opening = to_fixed(year, Month::regular(1), 1).unwrap();
            assert_eq!(
                Ok(opening),
                babylonian::to_fixed(seleucid_year(year), Month::regular(1), 1)
            );
            let date = is_babylonian_renamed(opening.0);
            assert_eq!(
                (date.year, date.month, date.day),
                (year, Month::regular(1), 1)
            );
            if year == MIN_YEAR {
                assert_eq!(
                    from_fixed(Rd(opening.0 - 1)),
                    Err(CalendarError::BeforeEpoch)
                );
            } else {
                assert_eq!(is_babylonian_renamed(opening.0 - 1).year, year - 1);
            }
        });
        let last = is_babylonian_renamed(babylonian::LATEST.0);
        assert_eq!(last.year, MAX_YEAR);
        assert_eq!(
            babylonian::from_fixed(Rd(babylonian::LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn the_calendar_is_babylonian_renamed_on_a_sample_of_days() {
        // Pins the wrapper property the module documentation rests on: on
        // every 97th day of the range in a release build, every 776th in a
        // debug one, and the first and last, the Arsacid fields are
        // `babylonian`'s with the year less 64 and the era renamed, and
        // both calendars write the date back to the same day. A change that
        // makes this calendar more than `babylonian` renamed fails here, and
        // needs a day-by-day sweep of its own (docs/policy.md §7).
        let (first, last) = (earliest().unwrap().0, babylonian::LATEST.0);
        let mut days: alloc::vec::Vec<i64> = (first..=last)
            .step_by(97 * crate::sweep_stride(8))
            .chain([last])
            .collect();
        days.dedup();
        let (arsacid, seleucid) = (ArsacidCalendar, babylonian::BabylonianCalendar);
        crate::check_days(&days, |rd| {
            let date = arsacid.from_fixed(Rd(rd)).unwrap();
            let base = seleucid.from_fixed(Rd(rd)).unwrap();
            let mut expected = seleucid.to_fields(base).unwrap();
            assert_eq!(expected.era, Some(babylonian::ERA));
            expected.year -= SELEUCID_OFFSET;
            expected.era = Some(ERA);
            assert_eq!(arsacid.to_fields(date), Ok(expected), "RD {rd}");
            assert_eq!(arsacid.to_fixed(date), Ok(Rd(rd)), "RD {rd}");
            assert_eq!(seleucid.to_fixed(base), Ok(Rd(rd)), "RD {rd}");
        });
    }
}
