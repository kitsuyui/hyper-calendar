//! The horizon a rising or a setting is measured against, as a named
//! convention.
//!
//! "Sunrise" is the moment the Sun's upper limb meets the visible horizon,
//! and every authority agrees on that much. They disagree about where the
//! visible horizon is: how much the air lifts the Sun at the horizon, how
//! large the Sun and the Moon are taken to be, and whether, and by how much,
//! a height above the sea lowers the horizon. Each answer is a convention,
//! and each is carried here under its own name, as [policy.md §5] asks:
//!
//! | Horizon | Refraction | Height above the sea | The Moon | Source |
//! |---|---|---|---|---|
//! | [`GEOMETRIC_DIP`], the default | 34′ | the geometric dip, arccos(R/(R+h)) | semidiameter 0.2725π, centre at 0.7275π − 34′ − dip | Meeus ch. 15 and the USNO for the sea-level horizon (`meeus1998`, `usno-rst-definitions`); the dip as `calendar-code2` computes it (`reingold2018code`) |
//! | [`USNO`] | 34′ | not used: every place at sea level | as the default, with no dip | the USNO's definitions (`usno-rst-definitions`) |
//! | [`CALENDRICAL_CALCULATIONS`] | 34′ | the geometric dip and 19″·√h more | semidiameter 16′, parallax arcsin(sin π cos h) | `refraction`, `sunrise`, `sunset` and `observed-lunar-altitude` in `calendar-code2` (`reingold2018code`) |
//!
//! Every convention takes the Sun's semidiameter as 16′. The twilights are
//! not affected: a twilight is a depression of the Sun's *centre* below the
//! *geometric* horizon, with no refraction, in all three.
//!
//! The rules, a worked example, the conventions that are not carried
//! (the NAOJ's 35′8″ and the refracted dip of Newcomb and Sôma) and how each
//! was measured are in
//! [`docs/systems/rise-and-set.md`](../../../docs/systems/rise-and-set.md).
//!
//! The list is data, not an enum (ADR 0007): another authority's horizon is
//! a new entry under its own identifier.
//!
//! [policy.md §5]: ../../../docs/policy.md

use hc_calendar::fixed::Moment;
use hc_core::math::{RAD_TO_DEG, asin, cos_deg, sin_deg, sqrt};

use crate::lunar::lunar_parallax;
use crate::riseset::{
    HORIZONTAL_REFRACTION_DEGREES, Location, SOLAR_SEMIDIAMETER_DEGREES, horizon_dip_degrees,
    lunar_altitude,
};
use crate::util::clamp;

/// How a horizon takes the observer's height above the sea.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dip {
    /// Not at all: the observer is on the sea-level surface wherever they
    /// are, as the USNO computes its rise and set times.
    Ignored,
    /// The geometric dip of the horizon, arccos(R/(R+h)), with no
    /// allowance for the bending of the grazing ray between the horizon and
    /// the observer; see [`horizon_dip_degrees`].
    Geometric,
    /// The geometric dip and a further `arcseconds_per_root_metre`·√h of
    /// refraction, h in metres, as `calendar-code2`'s `refraction` adds 19″
    /// for each root metre.
    GeometricAndRefraction {
        /// The further depression, in seconds of arc per root metre.
        arcseconds_per_root_metre: f64,
    },
}

/// How a horizon takes the Moon's upper limb.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoonLimb {
    /// The semidiameter as 0.2725 of the horizontal parallax π, so that the
    /// Moon's geocentric centre is at (1 − 0.2725)π less the refraction and
    /// the dip when its upper limb is on the horizon: Meeus's
    /// `h₀ = 0.7275π − 34′`, and the USNO's zenith distance of
    /// 90°34′ + s − π with s the Moon's apparent radius.
    ScaledByParallax,
    /// A fixed semidiameter, and the parallax at the Moon's geocentric
    /// altitude h, arcsin(sin π · cos h), subtracted from the altitude, as
    /// `calendar-code2`'s `topocentric-lunar-altitude` and
    /// `observed-lunar-altitude` have it.
    Fixed {
        /// The Moon's semidiameter, in degrees.
        semidiameter_degrees: f64,
    },
}

/// A named horizon: what the visible horizon is taken to be for a rising or
/// a setting, with its source.
#[derive(Debug, Clone, Copy)]
pub struct Horizon {
    /// A stable identifier, lowercase and hyphenated.
    pub id: &'static str,
    /// The English name.
    pub english_name: &'static str,
    /// What the convention takes the visible horizon to be, in a sentence.
    pub description: &'static str,
    /// The refraction at the horizon for an observer at sea level, in
    /// degrees.
    pub refraction_degrees: f64,
    /// The Sun's semidiameter, in degrees.
    pub solar_semidiameter_degrees: f64,
    /// How the observer's height lowers the horizon.
    pub dip: Dip,
    /// How the Moon's upper limb is found.
    pub moon: MoonLimb,
    /// Where the convention comes from.
    pub source: &'static str,
}

impl Horizon {
    /// How far the visible horizon, lifted by refraction, lies below the
    /// geometric horizon for an observer at this height, in degrees: the
    /// refraction at the horizon and whatever [`Self::dip`] adds.
    ///
    /// For [`CALENDRICAL_CALCULATIONS`] this is `calendar-code2`'s
    /// `refraction`: 34′ + arccos(R/(R+h)) + 19″·√h, with a height below
    /// the sea taken as zero.
    #[must_use]
    pub fn depression_degrees(&self, elevation_metres: f64) -> f64 {
        let height = if elevation_metres > 0.0 {
            elevation_metres
        } else {
            0.0
        };
        self.refraction_degrees
            + match self.dip {
                Dip::Ignored => 0.0,
                Dip::Geometric => horizon_dip_degrees(height),
                Dip::GeometricAndRefraction {
                    arcseconds_per_root_metre,
                } => {
                    horizon_dip_degrees(height) + arcseconds_per_root_metre * sqrt(height) / 3_600.0
                }
            }
    }

    /// The geometric altitude of the Sun's centre when its upper limb is on
    /// this horizon, in degrees: always negative.
    #[must_use]
    pub fn sunrise_altitude_degrees(&self, elevation_metres: f64) -> f64 {
        -(self.depression_degrees(elevation_metres) + self.solar_semidiameter_degrees)
    }

    /// How far the Moon's upper limb stands above this horizon at a
    /// moment, in degrees: zero at moonrise and moonset, positive while the
    /// Moon is up.
    #[must_use]
    pub fn lunar_limb_altitude_degrees(&self, moment: Moment, location: Location) -> f64 {
        let geocentric = lunar_altitude(moment, location);
        let parallax = lunar_parallax(moment);
        let depression = self.depression_degrees(location.elevation_metres);
        match self.moon {
            MoonLimb::ScaledByParallax => {
                geocentric - (MOON_CENTRE_PARALLAX_FRACTION * parallax - depression)
            }
            MoonLimb::Fixed {
                semidiameter_degrees,
            } => {
                let parallax_at_altitude =
                    asin(clamp(sin_deg(parallax) * cos_deg(geocentric), -1.0, 1.0)) * RAD_TO_DEG;
                geocentric - parallax_at_altitude + depression + semidiameter_degrees
            }
        }
    }
}

/// 1 − 0.2725: the part of the horizontal parallax left once the Moon's
/// semidiameter, 0.2725 of it, is taken off (Meeus, ch. 15, `meeus1998`).
pub(crate) const MOON_CENTRE_PARALLAX_FRACTION: f64 = 0.7275;

impl PartialEq for Horizon {
    /// Two horizons are the same when they carry the same identifier.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Horizon {}

hc_core::catalogue! {
    type: Horizon,
    id: |horizon| horizon.id,
    provenance: |horizon| horizon.source,
    tests: horizon_catalogue,

    /// Every horizon this crate carries.
    pub const HORIZONS;

    /// The horizon with this identifier.
    pub fn by_id;

    entries: {
        /// The default of [`crate::riseset::sunrise`] and the rest: the
        /// sea-level horizon of Meeus and the USNO, lowered by the geometric
        /// dip for the observer's height.
        ///
        /// The geometric dip is the least a height can lower the horizon;
        /// Sôma (`soma2001`) shows that the ray's bending between the
        /// horizon and the observer makes the effect on a rising about 8%
        /// larger, 2.09′·√h against 1.93′·√h. This convention leaves that
        /// out.
        pub const GEOMETRIC_DIP = Horizon {
            id: "geometric-dip",
            english_name: "sea-level horizon lowered by the geometric dip",
            description: "34′ of refraction and a Sun of 16′; the horizon lowered by the \
                          geometric dip arccos(R/(R+h)) for the observer's height; the Moon's \
                          semidiameter 0.2725 of its horizontal parallax",
            refraction_degrees: HORIZONTAL_REFRACTION_DEGREES,
            solar_semidiameter_degrees: SOLAR_SEMIDIAMETER_DEGREES,
            dip: Dip::Geometric,
            moon: MoonLimb::ScaledByParallax,
            source: "J. Meeus, Astronomical Algorithms, 2nd ed. (1998), ch. 15, h0 = -0°50' and \
                     0.7275π - 34'; USNO, Rise, Set, and Twilight Definitions; the dip \
                     arccos(R/(R+h)), R = 6 372 km, as refraction in E. M. Reingold and N. \
                     Dershowitz, calendar-code2, calendar.l",
        };

        /// The USNO's definitions: the Sun's centre at a zenith distance of
        /// 90°50′, the Moon's at 90°34′ + s − π, for an observer on the
        /// surface of the Earth, whatever their height.
        pub const USNO = Horizon {
            id: "usno",
            english_name: "US Naval Observatory, sea level",
            description: "34′ of refraction and a Sun of 16′, for an observer at sea level \
                          whatever their height; the Moon's semidiameter 0.2725 of its \
                          horizontal parallax",
            refraction_degrees: HORIZONTAL_REFRACTION_DEGREES,
            solar_semidiameter_degrees: SOLAR_SEMIDIAMETER_DEGREES,
            dip: Dip::Ignored,
            moon: MoonLimb::ScaledByParallax,
            source: "U.S. Naval Observatory, Astronomical Applications Department, Rise, Set, \
                     and Twilight Definitions, https://aa.usno.navy.mil/faq/RST_defs, \
                     retrieved 2026-09-27",
        };

        /// Reingold and Dershowitz's: 34′ of refraction, the geometric dip
        /// and 19″·√h more, and a Sun and a Moon of 16′.
        pub const CALENDRICAL_CALCULATIONS = Horizon {
            id: "calendrical-calculations",
            english_name: "Reingold and Dershowitz, Calendrical Calculations",
            description: "34′ of refraction and a Sun of 16′; the geometric dip and 19″·√h \
                          more for a height of h metres; a Moon of 16′ with the parallax at \
                          its altitude",
            refraction_degrees: HORIZONTAL_REFRACTION_DEGREES,
            solar_semidiameter_degrees: SOLAR_SEMIDIAMETER_DEGREES,
            dip: Dip::GeometricAndRefraction {
                arcseconds_per_root_metre: 19.0,
            },
            moon: MoonLimb::Fixed {
                semidiameter_degrees: SOLAR_SEMIDIAMETER_DEGREES,
            },
            source: "E. M. Reingold and N. Dershowitz, calendar-code2, calendar.l (Calendrica \
                     4.0), refraction, sunrise, sunset, topocentric-lunar-altitude and \
                     observed-lunar-altitude, read 2026-09-27",
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_sea_level_the_three_horizons_put_the_sun_at_fifty_minutes() {
        for horizon in HORIZONS {
            let altitude = horizon.sunrise_altitude_degrees(0.0);
            assert!(
                (altitude + 50.0 / 60.0).abs() < 1e-12,
                "{}: {altitude}",
                horizon.id
            );
        }
    }

    #[test]
    fn every_horizon_describes_itself_in_one_line() {
        for horizon in HORIZONS {
            assert!(!horizon.description.is_empty(), "{}", horizon.id);
            assert!(
                !horizon.description.contains(['\t', '\n']),
                "{}",
                horizon.id
            );
        }
    }

    #[test]
    fn the_usno_takes_no_account_of_height() {
        assert!((USNO.depression_degrees(3_000.0) - 34.0 / 60.0).abs() < 1e-12);
    }

    #[test]
    fn the_default_lowers_the_horizon_by_the_geometric_dip() {
        let height = 740.0;
        let expected = 34.0 / 60.0 + horizon_dip_degrees(height);
        assert!((GEOMETRIC_DIP.depression_degrees(height) - expected).abs() < 1e-12);
    }

    /// `refraction` in `calendar-code2`: at Jerusalem's 740 m the dip is
    /// 52.4′ and the 19″·√h another 8.6′, so the Sun's centre is 111.0′
    /// below the geometric horizon at sunset.
    #[test]
    fn calendrical_calculations_adds_nineteen_seconds_of_arc_a_root_metre() {
        let height = 740.0;
        let extra = CALENDRICAL_CALCULATIONS.depression_degrees(height)
            - GEOMETRIC_DIP.depression_degrees(height);
        assert!((extra * 60.0 - 19.0 * sqrt(740.0) / 60.0).abs() < 1e-9);
        assert!(
            (extra * 60.0 - 8.62).abs() < 0.01,
            "{} arcminutes",
            extra * 60.0
        );
        let depression = -CALENDRICAL_CALCULATIONS.sunrise_altitude_degrees(height) * 60.0;
        assert!((depression - 111.0).abs() < 0.05, "{depression} arcminutes");
    }

    #[test]
    fn a_height_below_the_sea_is_taken_as_the_sea() {
        for horizon in HORIZONS {
            assert!(
                (horizon.depression_degrees(-400.0) - horizon.depression_degrees(0.0)).abs()
                    < 1e-12,
                "{}",
                horizon.id
            );
        }
    }

    #[test]
    fn the_default_moon_is_the_one_moonrise_altitude_degrees_gives() {
        let tokyo = Location::new(35.6581, 139.7414, 40.0);
        for step in 0..50 {
            let moment = Moment(738_886.0 + f64::from(step) * 0.37);
            let limb = GEOMETRIC_DIP.lunar_limb_altitude_degrees(moment, tokyo);
            let direct = lunar_altitude(moment, tokyo)
                - crate::riseset::moonrise_altitude_degrees(moment, tokyo.elevation_metres);
            assert!((limb - direct).abs() < 1e-12);
        }
    }

    /// The two ways of taking the Moon's limb differ by the semidiameter,
    /// 16′ against 0.2725π, which runs from 14.7′ at apogee to 16.7′ at
    /// perigee, and by the parallax's cos h, under 0.6″ within a degree of
    /// the horizon.
    #[test]
    fn the_two_moons_differ_by_the_semidiameter_and_the_parallax_at_altitude() {
        let place = Location::new(21.4233, 39.8233, 0.0);
        for step in 0..100 {
            let moment = Moment(738_886.0 + f64::from(step) * 0.29);
            let difference = CALENDRICAL_CALCULATIONS.lunar_limb_altitude_degrees(moment, place)
                - USNO.lunar_limb_altitude_degrees(moment, place);
            let parallax = lunar_parallax(moment);
            let geocentric = lunar_altitude(moment, place);
            let at_altitude = asin(sin_deg(parallax) * cos_deg(geocentric)) * RAD_TO_DEG;
            let expected = 16.0 / 60.0 - 0.2725 * parallax + (parallax - at_altitude);
            assert!((difference - expected).abs() < 1e-9);
            let near_horizon = parallax - asin(sin_deg(parallax) * cos_deg(1.0)) * RAD_TO_DEG;
            assert!(
                near_horizon * 3_600.0 < 0.6,
                "{} arcseconds",
                near_horizon * 3_600.0
            );
        }
    }
}
