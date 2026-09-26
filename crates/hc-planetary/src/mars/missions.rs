//! Mission sol counts for the spacecraft that have landed on Mars.
//!
//! # What a mission sol actually is
//!
//! Every surface mission runs its own clock. The rules are consistent in shape
//! and maddeningly inconsistent in detail, and all four of the following vary
//! between missions:
//!
//! 1. **Where the clock's midnight is.** Not the landing site: the clock is
//!    built before launch from the *planned* landing longitude and is not
//!    re-set afterwards. InSight's clock runs about 85 s ahead of true local
//!    mean solar time at the place it actually came down, and Phoenix's about
//!    3.5 min. Both longitudes are in this table.
//! 2. **Mean or true solar time.** Viking and Pathfinder set their clocks to
//!    local *true* solar midnight — a sundial reading — and then ticked at the
//!    mean rate; everything since Spirit has used local mean solar time.
//! 3. **Whether the landing sol is 0 or 1.** The GISS rule of thumb is that
//!    "timekeeping begins with Sol 0 if the mission landed late in the day and
//!    was unable to perform meaningful mission operations until the next sol".
//!    Viking 1 and 2, Phoenix, Curiosity, InSight and Perseverance start at
//!    sol 0; Pathfinder, Spirit and Opportunity start at sol 1.
//! 4. **Whether a convention was published at all.** Zhurong's was not.
//!
//! # What this module does not model
//!
//! Spirit and Opportunity did not run plain local mean solar time. They ran
//! "hybrid local solar time" under the MER Continuous Time Algorithm (Roncoli
//! et al., 2002), a constant offset from site LMST — more than 41 minutes for
//! Spirit and 37 for Opportunity — chosen so that mission time tracked true
//! solar time to within 30 s around sol 45. This module reproduces the MER
//! **sol numbers**, which the offset does not change, but its [`MissionClock`]
//! time of sol for those two rovers is LMST, not HLST.
//!
//! Sources: NASA GISS, *Mars24 Sunclock — Lander Mission Times* and
//! *Technical Notes* (`mars24-notes`); landing coordinates from the Mars24
//! lander list, *Mars Lander Missions*
//! (<https://www.giss.nasa.gov/tools/mars24/help/landers.html>, updated
//! 2023-12-09, retrieved 2026-09-26, `mars24-landers`). The conventions are
//! written up in `docs/systems/mars-timekeeping.md`.

use hc_core::TimeResult;
use hc_core::math::floor;
use hc_core::{Instant, Tai};

use crate::util::{modulo, utc_unix_seconds};

use super::{MarsMoment, MarsTime};

/// Whether a mission clock's midnight is a mean-solar or a true-solar one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolConvention {
    /// The sol begins at local **mean** solar midnight on the clock meridian.
    LocalMeanSolarTime,
    /// The sol begins at local **true** solar midnight — the sundial's
    /// midnight — on the sol of landing, after which the clock ticks at the
    /// mean rate and so drifts away from the sundial by up to about an hour
    /// and a half over a Mars year.
    LocalTrueSolarTimeAtLanding,
}

/// A surface mission and the rules of its sol count.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mission {
    /// The English name the mission is usually called by.
    pub name: &'static str,
    /// The landing instant in UTC, written out for display.
    pub landing_utc: &'static str,
    /// The landing instant as a POSIX timestamp.
    pub landing_unix_seconds: i64,
    /// The landing site's east longitude in degrees, as located after the
    /// fact.
    pub site_east_longitude_degrees: f64,
    /// The east longitude the mission clock was built on, which is the planned
    /// site rather than the achieved one.
    pub clock_east_longitude_degrees: f64,
    /// Whether the clock's midnight is mean or true solar.
    pub convention: SolConvention,
    /// The sol number of the landing sol.
    pub first_sol: i64,
    /// Whether the mission's operators published a sol convention at all.
    pub convention_is_published: bool,
    /// What is worth knowing about this particular clock.
    pub note: &'static str,
}

impl Mission {
    /// The landing site's west longitude in degrees, in `[0, 360)`.
    #[must_use]
    pub fn site_west_longitude_degrees(&self) -> f64 {
        modulo(-self.site_east_longitude_degrees, 360.0)
    }

    /// The clock meridian's west longitude in degrees, in `[0, 360)`.
    #[must_use]
    pub fn clock_west_longitude_degrees(&self) -> f64 {
        modulo(-self.clock_east_longitude_degrees, 360.0)
    }

    /// The landing instant as a TAI reading.
    ///
    /// # Errors
    ///
    /// Propagates [`hc_core::unix::tai_from_unix`].
    pub fn landing_instant(&self) -> TimeResult<Instant<Tai>> {
        hc_core::unix::tai_from_unix(
            hc_core::UnixTime::from_seconds(self.landing_unix_seconds),
            hc_core::unix::LeapPolicy::Extrapolate,
        )
    }

    /// The landing instant as a Martian moment.
    ///
    /// # Errors
    ///
    /// See [`Self::landing_instant`].
    pub fn landing_moment(&self) -> TimeResult<MarsMoment> {
        Ok(MarsMoment::from_tai(self.landing_instant()?))
    }

    /// The mission's clock, ready to answer sol numbers and times of sol.
    ///
    /// Building it is the fallible step, because it needs the leap-second
    /// table to place the landing; everything afterwards is arithmetic.
    ///
    /// # Errors
    ///
    /// See [`Self::landing_instant`].
    pub fn clock(&self) -> TimeResult<MissionClock> {
        let landing = self.landing_moment()?;
        let west = self.clock_west_longitude_degrees();
        let epoch_mars_sol_date = match self.convention {
            SolConvention::LocalMeanSolarTime => {
                floor(landing.local_mean_sol_index(west)) + west / 360.0
            }
            SolConvention::LocalTrueSolarTimeAtLanding => {
                let index = floor(landing.local_true_sol_index(west));
                // The equation of time is itself a function of the instant, so
                // the true-solar midnight has to be solved for. It moves by at
                // most a few minutes per sol, so the fixed-point iteration
                // converges in a handful of passes; sixteen is generous.
                let mut msd = index + west / 360.0;
                for _ in 0..16 {
                    let correction =
                        MarsMoment::from_mars_sol_date(msd).equation_of_time_degrees() / 360.0;
                    msd = index + west / 360.0 - correction;
                }
                msd
            }
        };
        Ok(MissionClock {
            epoch_mars_sol_date,
            first_sol: self.first_sol,
        })
    }
}

/// A mission's sol counter: a mean-rate clock with its zero at the midnight
/// that began the landing sol.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MissionClock {
    epoch_mars_sol_date: f64,
    first_sol: i64,
}

impl MissionClock {
    /// The Mars Sol Date of the midnight that begins [`Self::first_sol`].
    #[must_use]
    pub fn epoch_mars_sol_date(&self) -> f64 {
        self.epoch_mars_sol_date
    }

    /// The sol number of the landing sol.
    #[must_use]
    pub const fn first_sol(&self) -> i64 {
        self.first_sol
    }

    /// The continuous count of sols since the clock's epoch, including the
    /// fraction of the current sol.
    #[must_use]
    pub fn sol_index(&self, moment: MarsMoment) -> f64 {
        moment.mars_sol_date() - self.epoch_mars_sol_date + self.first_sol as f64
    }

    /// The mission sol number containing a moment.
    #[must_use]
    pub fn sol(&self, moment: MarsMoment) -> i64 {
        floor(self.sol_index(moment)) as i64
    }

    /// The mission clock's time of sol.
    #[must_use]
    pub fn time(&self, moment: MarsMoment) -> MarsTime {
        MarsTime::from_sol_fraction(self.sol_index(moment))
    }

    /// The moment at which a given mission sol begins.
    #[must_use]
    pub fn sol_start(&self, sol: i64) -> MarsMoment {
        MarsMoment::from_mars_sol_date(self.epoch_mars_sol_date + (sol - self.first_sol) as f64)
    }
}

/// Every spacecraft that has landed on Mars and operated on the surface, in
/// landing order.
///
/// The landing instants are spacecraft event times (SCET) where the
/// distinction is documented. Several of the famous "landing times" quoted by
/// NASA are Earth-received times (ERT), later by the one-way light time, which
/// was eight to thirteen minutes for these events; Curiosity is the clearest
/// case, at 05:17:57 SCET against 05:32:00 ERT. Mixing the two shifts a
/// mission clock by minutes, though not — for any of these afternoon landings
/// — by a whole sol.
pub const MISSIONS: &[Mission] = &[
    Mission {
        name: "Viking 1",
        landing_utc: "1976-07-20T11:53:06Z",
        landing_unix_seconds: utc_unix_seconds(1976, 7, 20, 11, 53, 6),
        site_east_longitude_degrees: 312.05,
        clock_east_longitude_degrees: 312.5,
        convention: SolConvention::LocalTrueSolarTimeAtLanding,
        first_sol: 0,
        convention_is_published: true,
        note: "Local Lander Time ran from true solar midnight before touchdown. \
               The modern site longitude, 312.05 degrees east, is the one that \
               defines the Martian prime meridian through Kuchynka et al. \
               (2014); the 1976 mission clock used 312.5.",
    },
    Mission {
        name: "Viking 2",
        landing_utc: "1976-09-03T22:37:50Z",
        landing_unix_seconds: utc_unix_seconds(1976, 9, 3, 22, 37, 50),
        site_east_longitude_degrees: 134.28,
        clock_east_longitude_degrees: 134.14,
        convention: SolConvention::LocalTrueSolarTimeAtLanding,
        first_sol: 0,
        convention_is_published: true,
        note: "Utopia Planitia; same clock design as Viking 1.",
    },
    Mission {
        name: "Mars Pathfinder",
        landing_utc: "1997-07-04T16:56:55Z",
        landing_unix_seconds: utc_unix_seconds(1997, 7, 4, 16, 56, 55),
        site_east_longitude_degrees: 326.75,
        clock_east_longitude_degrees: 326.75,
        convention: SolConvention::LocalTrueSolarTimeAtLanding,
        first_sol: 1,
        convention_is_published: true,
        note: "Carl Sagan Memorial Station. Mission time tags are local true \
               solar time, and the landing sol is sol 1.",
    },
    Mission {
        name: "Spirit",
        landing_utc: "2004-01-04T04:26:53Z",
        landing_unix_seconds: utc_unix_seconds(2004, 1, 4, 4, 26, 53),
        site_east_longitude_degrees: 175.4785,
        clock_east_longitude_degrees: 175.298,
        convention: SolConvention::LocalMeanSolarTime,
        first_sol: 1,
        convention_is_published: true,
        note: "Gusev Crater. Operations ran on hybrid local solar time, more \
               than 41 minutes offset from site LMST; the sol numbers here are \
               the mission's, the times of sol are LMST.",
    },
    Mission {
        name: "Opportunity",
        landing_utc: "2004-01-25T04:54:00Z",
        landing_unix_seconds: utc_unix_seconds(2004, 1, 25, 4, 54, 0),
        site_east_longitude_degrees: 354.4734,
        clock_east_longitude_degrees: 354.4734,
        convention: SolConvention::LocalMeanSolarTime,
        first_sol: 1,
        convention_is_published: true,
        note: "Meridiani Planum, a few degrees from the prime meridian. \
               Hybrid local solar time offset was more than 37 minutes.",
    },
    Mission {
        name: "Phoenix",
        landing_utc: "2008-05-25T23:38:24Z",
        landing_unix_seconds: utc_unix_seconds(2008, 5, 25, 23, 38, 24),
        site_east_longitude_degrees: 234.248,
        clock_east_longitude_degrees: 233.35,
        convention: SolConvention::LocalMeanSolarTime,
        first_sol: 0,
        convention_is_published: true,
        note: "Green Valley, Vastitas Borealis. The clock was built on the \
               planned 233.35 degrees east, so mission time runs about three \
               and a half minutes from LMST at the achieved site.",
    },
    Mission {
        name: "Curiosity",
        landing_utc: "2012-08-06T05:17:57Z",
        landing_unix_seconds: utc_unix_seconds(2012, 8, 6, 5, 17, 57),
        site_east_longitude_degrees: 137.4417,
        clock_east_longitude_degrees: 137.42,
        convention: SolConvention::LocalMeanSolarTime,
        first_sol: 0,
        convention_is_published: true,
        note: "Bradbury Landing, Gale Crater. Touchdown was at about 15:03 \
               local mean solar time, late enough in the sol that the landing \
               sol is sol 0. The quoted 05:32 UTC is Earth-received time.",
    },
    Mission {
        name: "InSight",
        landing_utc: "2018-11-26T19:44:52Z",
        landing_unix_seconds: utc_unix_seconds(2018, 11, 26, 19, 44, 52),
        site_east_longitude_degrees: 135.62,
        clock_east_longitude_degrees: 135.97,
        convention: SolConvention::LocalMeanSolarTime,
        first_sol: 0,
        convention_is_published: true,
        note: "Elysium Planitia. The clock meridian is 0.35 degrees east of \
               the achieved site, so mission time runs about 85 seconds ahead \
               of site LMST. The quoted 19:52:59 UTC is Earth-received time.",
    },
    Mission {
        name: "Perseverance",
        landing_utc: "2021-02-18T20:43:48Z",
        landing_unix_seconds: utc_unix_seconds(2021, 2, 18, 20, 43, 48),
        site_east_longitude_degrees: 77.45,
        clock_east_longitude_degrees: 77.43,
        convention: SolConvention::LocalMeanSolarTime,
        first_sol: 0,
        convention_is_published: true,
        note: "Octavia E. Butler Landing, Jezero Crater. Touchdown was at \
               about 15:53 local mean solar time. The quoted 20:55 UTC is \
               Earth-received time.",
    },
    Mission {
        name: "Zhurong",
        landing_utc: "2021-05-14T23:18:00Z",
        landing_unix_seconds: utc_unix_seconds(2021, 5, 14, 23, 18, 0),
        site_east_longitude_degrees: 109.925,
        clock_east_longitude_degrees: 109.925,
        convention: SolConvention::LocalMeanSolarTime,
        first_sol: 1,
        convention_is_published: false,
        note: "Utopia Planitia. CNSA has published no mission clock or sol \
               numbering; the only sol-like figure in the public record counts \
               from rover deployment on 2021-05-22, not from landing. The \
               convention here — sol 1 at the landing sol, site LMST — is this \
               crate's own stated choice and must not be quoted as the \
               mission's. The landing time itself is given to the minute.",
    },
];

/// Look a mission up by name. The comparison is case-sensitive, because these
/// are proper nouns.
#[must_use]
pub fn mission(name: &str) -> Option<&'static Mission> {
    MISSIONS.iter().find(|entry| entry.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tai_from_utc_fields;

    fn moment(year: i64, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> MarsMoment {
        MarsMoment::from_tai(tai_from_utc_fields(year, month, day, hour, minute, second).unwrap())
    }

    #[test]
    fn every_mission_lands_on_its_own_first_sol() {
        for entry in MISSIONS {
            let clock = entry.clock().unwrap();
            assert_eq!(
                clock.sol(entry.landing_moment().unwrap()),
                entry.first_sol,
                "{}",
                entry.name
            );
        }
    }

    #[test]
    fn the_sol_numbering_conventions_are_the_ones_the_missions_used() {
        let zero: [&str; 6] = [
            "Viking 1",
            "Viking 2",
            "Phoenix",
            "Curiosity",
            "InSight",
            "Perseverance",
        ];
        let one: [&str; 3] = ["Mars Pathfinder", "Spirit", "Opportunity"];
        for name in zero {
            assert_eq!(mission(name).unwrap().first_sol, 0, "{name}");
        }
        for name in one {
            assert_eq!(mission(name).unwrap().first_sol, 1, "{name}");
        }
    }

    /// Every anchor here is a published sol-to-Earth-date correspondence. The
    /// four from the GISS lander table are the strongest, because they come
    /// from the same people who wrote the algorithm.
    #[test]
    fn the_published_sol_anchors_all_land_in_the_right_sol() {
        let anchors: [(&str, i64, (i64, u8, u8)); 10] = [
            ("Viking 1", 2243, (1982, 11, 11)),
            ("Viking 2", 1280, (1980, 4, 11)),
            ("Mars Pathfinder", 93, (1997, 10, 7)),
            ("Spirit", 2210, (2010, 3, 22)),
            ("Opportunity", 5111, (2018, 6, 10)),
            ("Phoenix", 156, (2008, 11, 2)),
            ("Curiosity", 4032, (2023, 12, 9)),
            ("Curiosity", 1000, (2015, 5, 30)),
            ("InSight", 1440, (2022, 12, 15)),
            ("Perseverance", 997, (2023, 12, 9)),
        ];
        for (name, sol, (year, month, day)) in anchors {
            let clock = mission(name).unwrap().clock().unwrap();
            let first = clock.sol(moment(year, month, day, 0, 0, 0));
            let last = clock.sol(moment(year, month, day, 23, 59, 59));
            assert!(
                (first..=last).contains(&sol),
                "{name} sol {sol}: {year}-{month}-{day} spans sols {first}..={last}"
            );
        }
    }

    #[test]
    fn a_sol_lasts_one_sol_and_the_count_advances_by_one() {
        let clock = mission("Curiosity").unwrap().clock().unwrap();
        for sol in [0, 1, 1_000, 4_032] {
            let start = clock.sol_start(sol);
            let next = clock.sol_start(sol + 1);
            assert_eq!(clock.sol(start), sol);
            assert_eq!(clock.sol(next), sol + 1);
            let elapsed = (next.j2000_offset() - start.j2000_offset()) * 86_400.0;
            assert!(
                (elapsed - super::super::MARS_SOL_SECONDS).abs() < 1e-3,
                "{elapsed}"
            );
        }
    }

    #[test]
    fn a_sol_begins_at_midnight_on_the_mission_clock() {
        for entry in MISSIONS {
            let clock = entry.clock().unwrap();
            let start = clock.sol_start(entry.first_sol + 7);
            // A tolerance rather than a string compare: the sol start is a
            // floating-point round trip and can land a nanosecond short of
            // midnight, which would print as 23:59:59.
            let fraction = clock.time(start).sol_fraction();
            assert!(
                !(1e-9..=1.0 - 1e-9).contains(&fraction),
                "{} {fraction}",
                entry.name
            );
        }
    }

    #[test]
    fn the_mean_solar_missions_keep_their_clock_on_local_mean_time() {
        for entry in MISSIONS {
            if entry.convention != SolConvention::LocalMeanSolarTime {
                continue;
            }
            let clock = entry.clock().unwrap();
            let when = clock.sol_start(entry.first_sol + 300);
            let lmst = when
                .local_mean_solar_time(entry.clock_west_longitude_degrees())
                .sol_fraction();
            assert!(
                !(1e-6..=1.0 - 1e-6).contains(&lmst),
                "{} {lmst}",
                entry.name
            );
        }
    }

    #[test]
    fn the_viking_clocks_start_at_a_sundial_midnight_and_then_drift() {
        for name in ["Viking 1", "Viking 2", "Mars Pathfinder"] {
            let entry = mission(name).unwrap();
            let clock = entry.clock().unwrap();
            let start = clock.sol_start(entry.first_sol);
            let ltst = start
                .local_true_solar_time(entry.clock_west_longitude_degrees())
                .sol_fraction();
            assert!(!(1e-6..=1.0 - 1e-6).contains(&ltst), "{name} {ltst}");
            // Half a Mars year later the sundial and the clock have parted
            // company by the change in the equation of time.
            let later = clock.sol_start(entry.first_sol + 334);
            let drift = later
                .local_true_solar_time(entry.clock_west_longitude_degrees())
                .sol_fraction();
            assert!(
                drift > 1e-3 && drift < 1.0 - 1e-3,
                "{name} drifted only {drift}"
            );
        }
    }

    #[test]
    fn the_landing_local_times_match_what_was_reported_at_the_time() {
        // Curiosity touched down at about 15:03 local mean solar time at Gale
        // Crater and Perseverance at about 15:53 at Jezero; both figures were
        // published at the time and neither is used to derive anything here,
        // so they are an independent check on the landing instants and the
        // longitudes at once.
        let expected: [(&str, u8, u8); 2] = [("Curiosity", 15, 3), ("Perseverance", 15, 53)];
        for (name, hour, minute) in expected {
            let entry = mission(name).unwrap();
            let time = entry
                .landing_moment()
                .unwrap()
                .local_mean_solar_time(entry.site_west_longitude_degrees());
            assert_eq!(time.hour(), hour, "{name}");
            assert!(
                (i32::from(time.minute()) - i32::from(minute)).abs() <= 1,
                "{name}: {time}"
            );
        }
    }

    #[test]
    fn the_insight_clock_runs_the_published_eighty_five_seconds_fast() {
        // The clock meridian is east of the achieved site, so mission time is
        // ahead of site local mean solar time by a fixed amount.
        let entry = mission("InSight").unwrap();
        let clock = entry.clock().unwrap();
        let moment = clock.sol_start(500);
        let site = moment
            .local_mean_solar_time(entry.site_west_longitude_degrees())
            .sol_fraction();
        // The mission clock reads midnight; the site clock has not got there.
        let lag = (1.0 - site) * super::super::MARS_SOL_SECONDS;
        assert!((lag - 86.3).abs() < 1.0, "{lag} s");
    }

    #[test]
    fn the_clock_meridian_and_the_site_are_recorded_separately() {
        let insight = mission("InSight").unwrap();
        assert!(
            (insight.clock_east_longitude_degrees - insight.site_east_longitude_degrees - 0.35)
                .abs()
                < 1e-9
        );
        // 0.35 degrees is 0.35/360 of a sol, about 86 seconds.
        let offset = 0.35 / 360.0 * super::super::MARS_SOL_SECONDS;
        assert!((offset - 86.3).abs() < 0.5, "{offset}");
    }

    #[test]
    fn longitudes_convert_between_the_two_conventions() {
        let curiosity = mission("Curiosity").unwrap();
        assert!((curiosity.site_west_longitude_degrees() - 222.5583).abs() < 1e-6);
        let opportunity = mission("Opportunity").unwrap();
        assert!((opportunity.site_west_longitude_degrees() - 5.5266).abs() < 1e-6);
    }

    #[test]
    fn zhurong_is_present_but_flagged_as_having_no_published_convention() {
        let zhurong = mission("Zhurong").unwrap();
        assert!(!zhurong.convention_is_published);
        for entry in MISSIONS {
            if entry.name != "Zhurong" {
                assert!(entry.convention_is_published, "{}", entry.name);
            }
        }
    }

    #[test]
    fn the_landing_strings_agree_with_the_timestamps() {
        // A cheap guard against a typo in one of the two representations.
        let expected: [(&str, &str); 3] = [
            ("Viking 1", "1976-07-20T11:53:06Z"),
            ("Curiosity", "2012-08-06T05:17:57Z"),
            ("Perseverance", "2021-02-18T20:43:48Z"),
        ];
        for (name, text) in expected {
            assert_eq!(mission(name).unwrap().landing_utc, text);
        }
        assert_eq!(
            mission("Curiosity").unwrap().landing_unix_seconds,
            1_344_230_277
        );
    }

    #[test]
    fn the_missions_are_listed_in_landing_order() {
        let mut previous = i64::MIN;
        for entry in MISSIONS {
            assert!(
                entry.landing_unix_seconds > previous,
                "{} out of order",
                entry.name
            );
            previous = entry.landing_unix_seconds;
        }
        assert_eq!(MISSIONS.len(), 10);
    }

    #[test]
    fn an_unknown_mission_is_not_invented() {
        assert!(mission("Beagle 2").is_none());
        assert!(mission("curiosity").is_none());
    }

    #[test]
    fn the_afternoon_landings_are_all_in_the_afternoon() {
        // Landings are timed for daylight at the site; Pathfinder's was early
        // morning and the rest were afternoon. This is a sanity check on the
        // landing instants as much as on the clocks.
        for entry in MISSIONS {
            let hour = entry
                .landing_moment()
                .unwrap()
                .local_mean_solar_time(entry.site_west_longitude_degrees())
                .hour();
            assert!((2..=17).contains(&hour), "{} landed at {hour}", entry.name);
        }
    }
}
