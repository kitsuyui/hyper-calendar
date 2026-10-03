//! The Moon's mean ascending node, Rāhu, and the sign it stands in.
//!
//! Hindu astrology places Rāhu at the Moon's ascending node and Ketu opposite
//! it, 180° on, and reckons their *gocāra*, the transit, by the sign Rāhu is
//! in: it moves backward through the signs, about 19.3° a year, and changes
//! sign every eighteen months or so. Almanacs print two readings: the *mean*
//! node, the smooth line the node's average position follows, and the *true*
//! node, which swings some 1.5° either side of it.
//!
//! This module carries the mean node, from the polynomial that gives its
//! mean longitude of the date, the one the Moon's theory in Meeus's *Astronomical
//! Algorithms* uses (equation 47.7, as `futureboy-sun-frink` prints it: the
//! book not read), and its entries into the sidereal signs. The true node is
//! not carried; see `docs/systems/hindu-calendars.md` for why, and for how
//! the mean node's entries stand against Drik Panchang's.

use hc_astro::julian_centuries;
use hc_calendar::fixed::Moment;
use hc_core::math::{floor, normalize_degrees};

use super::DEGREES_PER_SIGN;
use super::sidereal::{Ayanamsa, SiderealSign};

/// The longitude of the Moon's mean ascending node at a Universal Time moment,
/// in degrees from 0 up to but not including 360, measured from the mean
/// equinox of the date.
///
/// The polynomial in Julian centuries *T* from J2000.0:
/// 125.0445479° − 1934.1362891° *T* + 0.0020754° *T*² + *T*³/467441 −
/// *T*⁴/60616000.
#[must_use]
pub fn mean_ascending_node_longitude(moment: Moment) -> f64 {
    let t = julian_centuries(moment);
    normalize_degrees(
        125.044_547_9 - 1_934.136_289_1 * t + 0.002_075_4 * t * t + t * t * t / 467_441.0
            - t * t * t * t / 60_616_000.0,
    )
}

/// The sidereal longitude of Rāhu, the mean ascending node, in degrees from
/// 0 up to but not including 360: its longitude of the date less the
/// ayanāṃśa.
///
/// The mean node is measured from the mean equinox of the date, so what is
/// subtracted is the *mean* ayanāṃśa, [`Ayanamsa::mean_degrees_at`]: the
/// nutation a true ayanāṃśa adds belongs with a longitude measured from the
/// true equinox, as the Sun's and the Moon's are, and would shift the node
/// by up to 17″ if it were subtracted here.
#[must_use]
pub fn rahu_longitude(moment: Moment, ayanamsa: Ayanamsa) -> f64 {
    normalize_degrees(mean_ascending_node_longitude(moment) - ayanamsa.mean_degrees_at(moment))
}

/// The sidereal longitude of Ketu, the descending node: 180° from Rāhu.
#[must_use]
pub fn ketu_longitude(moment: Moment, ayanamsa: Ayanamsa) -> f64 {
    normalize_degrees(rahu_longitude(moment, ayanamsa) + 180.0)
}

/// The sidereal sign Rāhu stands in.
#[must_use]
pub fn rahu_sign(moment: Moment, ayanamsa: Ayanamsa) -> SiderealSign {
    SiderealSign::at(floor(rahu_longitude(moment, ayanamsa) / DEGREES_PER_SIGN) as u8)
}

/// One transit of the mean node: Rāhu's entry into a sign, with Ketu's into
/// the sign opposite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeIngress {
    /// The moment, in Universal Time.
    pub moment: Moment,
    /// The sign Rāhu leaves.
    pub rahu_from: SiderealSign,
    /// The sign Rāhu enters; Ketu enters the sign seven on from it.
    pub rahu_into: SiderealSign,
}

/// The entries of the mean node into the sidereal signs from a moment until
/// another, in order.
#[derive(Debug, Clone, Copy)]
pub struct NodeIngresses {
    cursor: Moment,
    until: Moment,
    ayanamsa: Ayanamsa,
}

/// The entries of Rāhu into a sign at or after `from` and before `until`,
/// in order, in the zodiac of `ayanamsa`. The mean node only moves backward,
/// so each is a single crossing, found by the Newton step on a rate that
/// varies by under a part in 10 000.
#[must_use]
pub fn ingresses(from: Moment, until: Moment, ayanamsa: Ayanamsa) -> NodeIngresses {
    NodeIngresses {
        cursor: from,
        until,
        ayanamsa,
    }
}

/// The node's mean rate in degrees a day, 1934.136 289 1° a century, and
/// backward.
const RATE_DEGREES_PER_DAY: f64 = 1_934.136_289_1 / 36_525.0;

/// How closely an entry is found, in days: under a tenth of a second.
const PRECISION_DAYS: f64 = 1e-6;

impl Iterator for NodeIngresses {
    type Item = NodeIngress;

    fn next(&mut self) -> Option<NodeIngress> {
        let longitude = rahu_longitude(self.cursor, self.ayanamsa);
        let from = SiderealSign::at(floor(longitude / DEGREES_PER_SIGN) as u8);
        // The sign's lower edge is the boundary the node, moving back,
        // reaches next.
        let edge = from.start_longitude_degrees();
        let mut moment = self.cursor.0 + (longitude - edge) / RATE_DEGREES_PER_DAY;
        for _ in 0..8 {
            let error =
                hc_core::math::signed_degrees(rahu_longitude(Moment(moment), self.ayanamsa) - edge);
            let step = error / RATE_DEGREES_PER_DAY;
            moment += step;
            if step.abs() < PRECISION_DAYS {
                break;
            }
        }
        if moment >= self.until.0 {
            self.cursor = self.until;
            return None;
        }
        self.cursor = Moment(moment + 1.0);
        Some(NodeIngress {
            moment: Moment(moment),
            rahu_from: from,
            rahu_into: SiderealSign::at(from.index().wrapping_add(11) % 12),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    /// Drik Panchang's mean Rāhu transits for New Delhi, in Indian Standard
    /// Time (`drik-rahu-transit`, read 2026-10-03): the date and time, and
    /// the sign Rāhu enters, from Meṣa as 0.
    const DRIK_MEAN: [(i64, u8, u8, u8, u8, u8); 5] = [
        (2020, 9, 23, 7, 38, 1),
        (2022, 4, 12, 10, 35, 0),
        (2023, 10, 30, 13, 33, 11),
        (2025, 5, 18, 16, 30, 10),
        (2026, 12, 5, 19, 28, 9),
    ];

    fn ist(year: i64, month: u8, day: u8, hour: u8, minute: u8) -> Moment {
        let day = gregorian::to_fixed(year, month, day).expect("a date");
        Moment(day.0 as f64 + (f64::from(hour) + f64::from(minute) / 60.0 - 5.5) / 24.0)
    }

    /// J2000.0 is the polynomial's constant, 125.0445°, and the node moves
    /// back 19.34° a year, which is its rate (1934.136° a century).
    #[test]
    fn the_mean_node_is_where_its_polynomial_puts_it() {
        let j2000 = Moment(730_120.5);
        assert!((mean_ascending_node_longitude(j2000) - 125.044_5).abs() < 0.001);
        let a_year_on = Moment(j2000.0 + 365.25);
        let moved = normalize_degrees(
            mean_ascending_node_longitude(j2000) - mean_ascending_node_longitude(a_year_on),
        );
        assert!((moved - 19.3414).abs() < 0.001, "{moved}");
        // Ketu is opposite Rāhu.
        let ayanamsa = Ayanamsa::LAHIRI_DRIK;
        let gap =
            normalize_degrees(ketu_longitude(j2000, ayanamsa) - rahu_longitude(j2000, ayanamsa));
        assert!((gap - 180.0).abs() < 1e-9);
    }

    /// The mean transits Drik Panchang prints for New Delhi, 2020 to 2026,
    /// with the ayanāṃśa Drik prints: each within 2.4 minutes of its time
    /// and each in the sign Drik names, Rāhu entering the sign before. With
    /// this crate's Lahiri, 25″ below Drik's, the node moving back 0.053° a
    /// day arrives 194 minutes later (3.2 hours), which is the same 25″.
    #[test]
    fn the_mean_transits_are_drik_panchangs_with_its_ayanamsa() {
        for row in DRIK_MEAN {
            let drik = ist(row.0, row.1, row.2, row.3, row.4);
            for (ayanamsa, lower, upper) in [
                (Ayanamsa::LAHIRI_DRIK, -3.0, 3.0),
                (Ayanamsa::LAHIRI, 190.0, 198.0),
            ] {
                let found = ingresses(Moment(drik.0 - 100.0), Moment(drik.0 + 100.0), ayanamsa)
                    .next()
                    .expect("an entry within a hundred days");
                assert_eq!(found.rahu_into.index(), row.5, "{row:?}");
                let minutes = (found.moment.0 - drik.0) * 1_440.0;
                assert!(
                    (lower..upper).contains(&minutes),
                    "{} {row:?}: {minutes} minutes",
                    ayanamsa.id()
                );
                // Rāhu is in the sign before just before, and the sign after
                // just after; Ketu keeps the opposite one.
                let before = Moment(found.moment.0 - 0.01);
                let after = Moment(found.moment.0 + 0.01);
                assert_eq!(rahu_sign(before, ayanamsa), found.rahu_from);
                assert_eq!(rahu_sign(after, ayanamsa), found.rahu_into);
            }
        }
    }

    /// A true ayanāṃśa's nutation is left out of the node's longitude, which
    /// is a mean one: the Committee's true Lahiri gives the node where its
    /// mean value does.
    #[test]
    fn the_mean_node_is_read_against_the_mean_ayanamsa() {
        let at = Moment(738_967.0);
        let by_true = rahu_longitude(at, Ayanamsa::LAHIRI_CRC_1955);
        let by_mean = normalize_degrees(
            mean_ascending_node_longitude(at) - Ayanamsa::LAHIRI_CRC_1955.mean_degrees_at(at),
        );
        assert!((by_true - by_mean).abs() < 1e-12);
        let nutation = Ayanamsa::LAHIRI_CRC_1955.degrees_at(at)
            - Ayanamsa::LAHIRI_CRC_1955.mean_degrees_at(at);
        assert!(nutation.abs() * 3_600.0 > 1.0, "{nutation}");
    }

    /// Rāhu passes through the twelve signs in 18.6 years, a sign in about 566
    /// days, always backward and in order.
    #[test]
    fn the_node_goes_back_through_the_twelve_signs_in_eighteen_years() {
        let from = ist(2019, 1, 1, 0, 0);
        let found: Vec<NodeIngress> =
            ingresses(from, Moment(from.0 + 18.6 * 365.25), Ayanamsa::LAHIRI_DRIK).collect();
        assert!((11..=12).contains(&found.len()), "{}", found.len());
        for pair in found.windows(2) {
            assert_eq!(pair[0].rahu_into, pair[1].rahu_from);
            assert_eq!(
                pair[1].rahu_into.index(),
                (pair[1].rahu_from.index() + 11) % 12
            );
            let days = pair[1].moment.0 - pair[0].moment.0;
            assert!((565.0..567.5).contains(&days), "{days}");
        }
        assert_eq!(ingresses(from, from, Ayanamsa::LAHIRI).count(), 0);
    }
}
