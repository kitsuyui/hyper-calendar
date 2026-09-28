//! Greenwich Mean Astronomical Time: Greenwich mean time counted from
//! noon, the reckoning of the *Nautical Almanac* before 1925.
//!
//! The almanac calls both reckonings "G.M.T."; the name *Greenwich Mean
//! Astronomical Time*, GMAT, "was introduced to unambiguously refer to the
//! previous noon-based astronomical convention" (Wikipedia, "Greenwich Mean
//! Time", retrieved 2026-09-28, `wikipedia-greenwich-mean-time`, citing
//! the *Astronomical Supplement to the Astronomical Almanac*, University
//! Science Books, 1992, p. 76, not read).
//!
//! The astronomical day begins at mean noon at Greenwich and is named by
//! the civil day it begins on, so GMAT = GMT − 12 h, and the astronomical
//! date is the civil date for the hours from noon and the day before for
//! the hours before it. The *Nautical Almanac* for 1924 states both
//! reckonings: "the times styled G.M.T. are at present reckoned from noon,
//! corresponding to 12 hours (Civil Time); but from the year 1925
//! inclusive and thenceforward the times styled G.M.T. in these
//! publications will be given commencing at midnight" (H.M. Nautical
//! Almanac Office, *The Nautical Almanac and Astronomical Ephemeris for
//! the Year 1924*, 1921, the notice before the title page; read
//! 2026-09-28 in the Digital Library of India's scan, `nautical-almanac-1924`).
//! So a G.M.T. printed in the *Nautical Almanac* for 1924 or earlier is
//! GMAT, and one printed for 1925 or later is civil GMT. The notice speaks
//! of the Office's own publications only; another almanac's reckoning is
//! that almanac's to state.
//!
//! GMT here is mean solar time at Greenwich counted from midnight, what is
//! now Universal Time. Both functions take and give a [`CivilDateTime`]:
//! the day and the time of day on the clock named. They are exact, and
//! they apply to any date; which reckoning a given document used is the
//! document's to say. `docs/time-scales.md` places this beside UT1.

use hc_calendar::daystart::{DayBoundary, DayNaming};
use hc_calendar::{CalendarError, CalendarResult, CivilDateTime, CivilTime, Rd};
use hc_core::Duration;

/// Where the astronomical day begins, and which civil day names it.
pub const ASTRONOMICAL_DAY: DayBoundary = DayBoundary::Noon(DayNaming::ByStart);

/// Half a day.
const TWELVE_HOURS: Duration = Duration::from_secs(43_200);

/// The time of day twelve hours on from `time`.
fn shift(time: CivilTime) -> CalendarResult<CivilTime> {
    if time.is_leap_second() {
        // 23:59:60 would be 11:59:60, which no clock reads.
        return Err(CalendarError::DayOutOfRange);
    }
    let since_midnight = time.since_midnight();
    let shifted = if since_midnight >= TWELVE_HOURS {
        since_midnight - TWELVE_HOURS
    } else {
        since_midnight + TWELVE_HOURS
    };
    CivilTime::from_midnight_offset(shifted)
}

/// The astronomical date and the GMAT of a GMT reading.
///
/// # Errors
///
/// [`CalendarError::DayOutOfRange`] for 23:59:60, which has no reading
/// twelve hours earlier.
pub fn gmat_from_gmt(gmt: CivilDateTime) -> CalendarResult<CivilDateTime> {
    let offset = ASTRONOMICAL_DAY
        .civil_day_offset(gmt.time)
        .ok_or(CalendarError::DayOutOfRange)?;
    Ok(CivilDateTime::new(Rd(gmt.day.0 + offset), shift(gmt.time)?))
}

/// The civil date and the GMT of a GMAT reading.
///
/// # Errors
///
/// [`CalendarError::DayOutOfRange`] for a second 60, which GMAT, whose
/// day ends at noon, does not read.
pub fn gmt_from_gmat(gmat: CivilDateTime) -> CalendarResult<CivilDateTime> {
    let next_day = gmat.time.since_midnight() >= TWELVE_HOURS;
    Ok(CivilDateTime::new(
        Rd(gmat.day.0 + i64::from(next_day)),
        shift(gmat.time)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::gregorian;

    fn reading(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        attos: (u8, u64),
    ) -> CivilDateTime {
        CivilDateTime::new(
            gregorian::to_fixed(year, month, day).expect("exists"),
            CivilTime::new(hour, minute, attos.0, attos.1).expect("valid"),
        )
    }

    /// The *Nautical Almanac* for 1924, "Eclipses, 1924", pp. 461–468: the opposition in right ascension of the total lunar eclipse
    /// of February 20 at "February 20ᵈ 4ʰ 12ᵐ 25ˢ·7" G.M.T., and of August
    /// 14 at "August 14ᵈ 8ʰ 22ᵐ 59ˢ·1", both reckoned from noon. Their civil
    /// times are the afternoon and evening, where NASA's catalogue puts the
    /// greatest eclipses, at 16:08:55 and 20:20:30 TD (Espenak,
    /// "Catalog of Lunar Eclipses: 1901 to 2000", `espenak-lunar-eclipses-1901`).
    #[test]
    fn the_1924_almanacs_eclipses() {
        for (month, day, gmat, gmt) in [
            (2, 20, (4, 12, 25), (16, 12, 25)),
            (8, 14, (8, 22, 59), (20, 22, 59)),
        ] {
            let tenths = if month == 2 { 7 } else { 1 };
            let attos = tenths * 100_000_000_000_000_000;
            let astronomical = reading(1924, month, day, gmat.0, gmat.1, (gmat.2, attos));
            let civil = reading(1924, month, day, gmt.0, gmt.1, (gmt.2, attos));
            assert_eq!(gmt_from_gmat(astronomical), Ok(civil));
            assert_eq!(gmat_from_gmt(civil), Ok(astronomical));
        }
    }

    /// The change of 1925, from the notice's two reckonings: the first
    /// instant the almanac for 1925 calls 1925 January 1, 0ʰ is 1924
    /// December 31, 12ʰ counted from noon; the morning of a civil day is
    /// the astronomical day before.
    #[test]
    fn the_change_of_1925() {
        let civil = reading(1925, 1, 1, 0, 0, (0, 0));
        let astronomical = reading(1924, 12, 31, 12, 0, (0, 0));
        assert_eq!(gmat_from_gmt(civil), Ok(astronomical));
        assert_eq!(gmt_from_gmat(astronomical), Ok(civil));
        let morning = reading(1924, 6, 1, 11, 59, (59, 999_999_999_999_999_999));
        assert_eq!(
            gmat_from_gmt(morning),
            Ok(reading(1924, 5, 31, 23, 59, (59, 999_999_999_999_999_999)))
        );
        let noon = reading(1924, 6, 1, 12, 0, (0, 0));
        assert_eq!(gmat_from_gmt(noon), Ok(reading(1924, 6, 1, 0, 0, (0, 0))));
        assert_eq!(ASTRONOMICAL_DAY.fixed_offset(), Some(CivilTime::NOON));
        let leap = reading(2016, 12, 31, 23, 59, (60, 0));
        assert_eq!(gmat_from_gmt(leap), Err(CalendarError::DayOutOfRange));
    }

    /// Every hour of a sample of days, both ways.
    #[test]
    fn round_trips() {
        for day in (600_000..800_000).step_by(997) {
            for hour in 0..24 {
                let gmt = CivilDateTime::new(Rd(day), CivilTime::hms(hour, 17, 3).expect("valid"));
                let gmat = gmat_from_gmt(gmt).expect("valid");
                assert_eq!(gmat.day.0, day - i64::from(hour < 12));
                assert_eq!(gmt_from_gmat(gmat), Ok(gmt));
            }
        }
    }
}
