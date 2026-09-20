//! The scrap of Gregorian arithmetic this crate needs, kept private.
//!
//! Only [`crate::moon_names::harvest_moon_falls_in`] needs a month number
//! from an [`Rd`]: the Harvest Moon rule produces a fixed day and the caller
//! wants to know whether it landed in September or October. Everything else
//! in the crate is keyed by a [`hc_calendar::Month`] the caller already
//! holds.
//!
//! `hc-calendars-solar` owns the real proleptic Gregorian calendar, and this
//! crate deliberately does not depend on it — an attribution table has no
//! business pulling in a calendar implementation. `hc-seasons` keeps the
//! same scrap private for the same reason, and the formulae are from the
//! same place: Reingold & Dershowitz, *Calendrical Calculations*, 4th ed.,
//! §2.2. A test checks this copy against `hc-seasons`' observable behaviour
//! so the duplication cannot drift.

use hc_calendar::Rd;

/// Whether a proleptic Gregorian year has 366 days.
const fn is_leap_year(year: i64) -> bool {
    year.rem_euclid(4) == 0 && year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0
}

/// The fixed day on which a proleptic Gregorian year begins.
const fn new_year(year: i64) -> Rd {
    let y = year - 1;
    Rd(365 * y + y.div_euclid(4) - y.div_euclid(100) + y.div_euclid(400) + 1)
}

/// The proleptic Gregorian year containing a fixed day.
const fn year_from_rd(rd: Rd) -> i64 {
    let d0 = rd.0 - 1;
    let n400 = d0.div_euclid(146_097);
    let d1 = d0.rem_euclid(146_097);
    let n100 = d1.div_euclid(36_524);
    let d2 = d1.rem_euclid(36_524);
    let n4 = d2.div_euclid(1_461);
    let d3 = d2.rem_euclid(1_461);
    let n1 = d3.div_euclid(365);
    let year = 400 * n400 + 100 * n100 + 4 * n4 + n1;
    if n100 == 4 || n1 == 4 { year } else { year + 1 }
}

/// The fixed day of a proleptic Gregorian date.
pub(crate) const fn from_year_month_day(year: i64, month: u8, day: u8) -> Rd {
    let correction = if month <= 2 {
        0
    } else if is_leap_year(year) {
        -1
    } else {
        -2
    };
    Rd(new_year(year).0 - 1 + (367 * month as i64 - 362).div_euclid(12) + correction + day as i64)
}

/// The proleptic Gregorian year, month and day of a fixed day.
pub(crate) const fn year_month_day_from_rd(rd: Rd) -> (i64, u8, u8) {
    let year = year_from_rd(rd);
    let prior_days = rd.0 - new_year(year).0;
    let correction = if rd.0 < from_year_month_day(year, 3, 1).0 {
        0
    } else if is_leap_year(year) {
        1
    } else {
        2
    };
    let month = ((12 * (prior_days + correction) + 373).div_euclid(367)) as u8;
    let day = (rd.0 - from_year_month_day(year, month, 1).0 + 1) as u8;
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RD 1 is 1 January of year 1, by definition of the Rata Die epoch.
    #[test]
    fn the_rata_die_epoch_is_the_first_of_january_of_year_one() {
        assert_eq!(year_month_day_from_rd(Rd(1)), (1, 1, 1));
        assert_eq!(from_year_month_day(1, 1, 1), Rd(1));
    }

    /// RD 719 163 is 1 January 1970, the Unix epoch, which
    /// `hc_calendar::Rd::UNIX_EPOCH` also states.
    #[test]
    fn the_unix_epoch_lands_where_the_standard_says() {
        assert_eq!(from_year_month_day(1970, 1, 1), Rd(719_163));
        assert_eq!(year_month_day_from_rd(Rd::UNIX_EPOCH), (1970, 1, 1));
    }

    #[test]
    fn year_month_day_round_trips_over_four_centuries() {
        let start = from_year_month_day(1800, 1, 1).0;
        let end = from_year_month_day(2200, 1, 1).0;
        for day in start..end {
            let (year, month, date) = year_month_day_from_rd(Rd(day));
            assert_eq!(from_year_month_day(year, month, date), Rd(day));
            assert!((1..=12).contains(&month), "{day}");
            assert!((1..=31).contains(&date), "{day}");
        }
    }

    #[test]
    fn february_has_the_right_length_either_way() {
        // 2024 is a leap year, 2023 and 1900 are not, 2000 is.
        assert_eq!(
            year_month_day_from_rd(Rd(from_year_month_day(2024, 3, 1).0 - 1)).2,
            29
        );
        assert_eq!(
            year_month_day_from_rd(Rd(from_year_month_day(2023, 3, 1).0 - 1)).2,
            28
        );
        assert_eq!(
            year_month_day_from_rd(Rd(from_year_month_day(1900, 3, 1).0 - 1)).2,
            28
        );
        assert_eq!(
            year_month_day_from_rd(Rd(from_year_month_day(2000, 3, 1).0 - 1)).2,
            29
        );
    }

    /// The reason this copy exists is a month number for the harvest moon,
    /// so pin the two months it ever answers with.
    #[test]
    fn september_and_october_boundaries_are_where_they_should_be() {
        assert_eq!(
            year_month_day_from_rd(from_year_month_day(2025, 9, 30)),
            (2025, 9, 30)
        );
        assert_eq!(
            year_month_day_from_rd(from_year_month_day(2025, 10, 1)),
            (2025, 10, 1)
        );
        assert_eq!(
            from_year_month_day(2025, 10, 1).0 - from_year_month_day(2025, 9, 1).0,
            30
        );
    }

    /// `hc-seasons` computes the meteorological seasons from the same
    /// arithmetic, so a day this module calls September must be a day that
    /// crate calls northern autumn.
    #[test]
    fn this_copy_agrees_with_the_seasons_crates_month_arithmetic() {
        use hc_seasons::seasons::{Hemisphere, Season, SeasonDefinition, season_of};
        for year in [1900i64, 1999, 2000, 2024, 2100] {
            for (month, season) in [
                (1u8, Season::Winter),
                (4, Season::Spring),
                (7, Season::Summer),
                (10, Season::Autumn),
            ] {
                let day = from_year_month_day(year, month, 15);
                assert_eq!(
                    season_of(
                        day,
                        SeasonDefinition::Meteorological,
                        Hemisphere::Northern,
                        hc_seasons::Meridian::UNIVERSAL,
                    ),
                    season,
                    "{year}-{month}"
                );
                assert_eq!(year_month_day_from_rd(day), (year, month, 15));
            }
        }
    }
}
