//! 十五夜 and 十三夜, the two moon-viewing nights of the Japanese almanac.
//!
//! Both are lunisolar dates, not astronomical events: 十五夜 (中秋の名月) is
//! the fifteenth day of the eighth month and 十三夜 (後の月) the thirteenth
//! of the ninth, in the lunisolar calendar [`crate::lunisolar`] reads at the
//! meridian passed — the Japanese 旧暦 at [`Meridian::JAPAN`], where these
//! are Japanese observances; at [`Meridian::CHINA`] the fifteenth of the
//! eighth month is the Chinese calendar's 中秋. Neither is reliably
//! the night of the actual full moon — a lunation is 29.53 days, so the full
//! moon falls on the fifteenth day only about half the time, and 中秋の名月
//! can be a day or two off. That is not an error; the observance is dated by
//! the calendar, not by the sky. `hc_seasons::moon_calendar` has the phases.
//!
//! In 2033 the eighth month is the one no rule settles, and the 旧暦 here
//! gives the Observatory's first resolution, 案1, whose 中秋の名月 is 8
//! September; under 案2 it would be 7 October (`nao-topics-2014-2033`).

use hc_calendar::Rd;
use hc_seasons::Meridian;

use crate::lunisolar::Reckoning;

/// 十五夜, the 中秋の名月: the fifteenth day of the eighth lunisolar month,
/// which always falls in the Gregorian year it is asked for.
///
/// `None` only for a year outside the Gregorian conversion range, since
/// the eighth month always begins between August and October.
///
/// ```
/// use hc_almanac::{Meridian, Rd, moon_viewing::mid_autumn_moon};
///
/// // 17 September 2024.
/// assert_eq!(mid_autumn_moon(2024, Meridian::JAPAN), Some(Rd(739_146)));
/// ```
#[must_use]
pub fn mid_autumn_moon(year: i64, meridian: Meridian) -> Option<Rd> {
    Reckoning::at(meridian).day_in_gregorian_year(year, 8, 15)
}

/// 十三夜, the 後の月: the thirteenth day of the ninth lunisolar month.
///
/// The companion to 十五夜, about a month later. Viewing one and not the
/// other was 片見月 and held to be unlucky.
#[must_use]
pub fn thirteenth_night(year: i64, meridian: Meridian) -> Option<Rd> {
    Reckoning::at(meridian).day_in_gregorian_year(year, 9, 13)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::gregorian;
    use hc_seasons::moon_calendar::{illuminated_fraction, principal_phases_in_month};

    const JAPAN: Meridian = Meridian::JAPAN;

    /// Published 中秋の名月 dates, JST: these are printed in every Japanese
    /// calendar and reported in the newspapers each year.
    #[test]
    fn the_mid_autumn_moon_falls_where_the_almanacs_put_it() {
        for (year, month, day) in [
            (2020, 10, 1),
            (2021, 9, 21),
            (2022, 9, 10),
            (2023, 9, 29),
            (2024, 9, 17),
            (2025, 10, 6),
        ] {
            assert_eq!(
                mid_autumn_moon(year, JAPAN),
                Some(gregorian::to_fixed_saturating(year, month, day)),
                "中秋の名月 of {year}"
            );
        }
    }

    /// Published 十三夜 dates, JST.
    #[test]
    fn the_thirteenth_night_falls_where_the_almanacs_put_it() {
        for (year, month, day) in [
            (2021, 10, 18),
            (2022, 10, 8),
            (2023, 10, 27),
            (2024, 10, 15),
            (2025, 11, 2),
        ] {
            assert_eq!(
                thirteenth_night(year, JAPAN),
                Some(gregorian::to_fixed_saturating(year, month, day)),
                "十三夜 of {year}"
            );
        }
    }

    /// 2033: 案1's 8 September, and not 案2's 7 October, which the
    /// simplified derivation `hc-seasons` kept gave (`nao-topics-2014-2033`).
    #[test]
    fn the_mid_autumn_moon_of_2033_is_the_first_resolutions() {
        assert_eq!(
            mid_autumn_moon(2033, JAPAN),
            Some(gregorian::to_fixed_saturating(2033, 9, 8))
        );
    }

    /// 十三夜 comes about a month after 十五夜 — unless a leap eighth month
    /// falls between them, in which case it comes about two. 1995 had a
    /// 閏八月 and its two viewing nights were 57 days apart, which is not a
    /// bug but the calendar working.
    #[test]
    fn the_thirteenth_night_follows_the_mid_autumn_moon_by_a_month_or_by_two() {
        hc_core::memo::scope(|| {
            let mut intercalated = 0;
            for year in 1980..2060 {
                let (Some(fifteenth), Some(thirteenth)) =
                    (mid_autumn_moon(year, JAPAN), thirteenth_night(year, JAPAN))
                else {
                    panic!("{year} was missing one of the two moon-viewing nights");
                };
                let gap = thirteenth.0 - fifteenth.0;
                if gap > 40 {
                    intercalated += 1;
                    assert!(
                        (55..=61).contains(&gap),
                        "{year}: {gap} days, which is neither one month nor two"
                    );
                    // A leap eighth month is the only thing that can do this.
                    let between = Reckoning::JapaneseTenpo.date(Rd(fifteenth.0 + 30));
                    assert!(
                        between.month.leap,
                        "{year}: the long gap was not a leap month"
                    );
                } else {
                    assert!(
                        (26..=31).contains(&gap),
                        "{year}: {gap} days between the two viewings"
                    );
                }
            }
            assert!(
                (1..=6).contains(&intercalated),
                "{intercalated} leap eighth months in eighty years"
            );
        });
    }

    /// The tradition dates 十五夜 by the calendar, not by the sky, so it is
    /// the actual full moon rather less than half the time. Stating that as a
    /// test stops anyone "fixing" it later.
    #[test]
    fn the_mid_autumn_moon_is_often_not_the_full_moon() {
        hc_core::memo::scope(|| {
            let mut exact = 0;
            let mut total = 0;
            for year in 1980..2060 {
                let Some(night) = mid_autumn_moon(year, JAPAN) else {
                    continue;
                };
                total += 1;
                let (y, m, _) = gregorian::ymd(night);
                let phases = principal_phases_in_month(y, m, Meridian::JAPAN);
                if phases.full_moon().map(|event| event.day) == Some(night) {
                    exact += 1;
                }
                // Whether or not it is exact, the Moon is near enough full
                // to be worth looking at.
                assert!(
                    illuminated_fraction(night, Meridian::JAPAN) > 0.93,
                    "{year}: the harvest moon was only {} lit",
                    illuminated_fraction(night, Meridian::JAPAN)
                );
            }
            assert!(total > 70);
            assert!(
                exact * 2 < total,
                "{exact} of {total} were the exact full moon, which is suspiciously many"
            );
        });
    }
}
