//! A data table of the major bodies of the solar system, and the three
//! quantities a clock needs derived from it.
//!
//! # The data
//!
//! Every number is measured, not computed, and every row says where it came
//! from. The bulk is the NASA NSSDC *Planetary Fact Sheets*
//! (<https://nssdc.gsfc.nasa.gov/planetary/factsheet/>, `nssdc-factsheets`;
//! the comparison table retrieved 2026-09-26, the per-body sheets it links
//! not re-read that day), supplemented by
//! the IAU Working Group on Cartographic Coordinates and Rotational Elements
//! (Archinal et al., 2018, *Celest. Mech. Dyn. Astr.* **130**:22) for
//! satellite rotation rates and by Konopliv et al. (2018, *Icarus* **299**,
//! 411) for Ceres. The fact sheets disagree with themselves in a few places
//! and with the IAU in one; [`Body::source`] and the crate README name the
//! disagreements rather than averaging them away.
//!
//! # The derivation
//!
//! The one thing this module computes is the **solar day** — the synodic
//! rotation period, how long it takes the Sun to come back to the same place
//! in the sky — from the sidereal rotation period and the orbital period:
//!
//! ```text
//! 1 / P_solar = 1 / P_sidereal − 1 / P_orbit
//! ```
//!
//! Two details make this get the right answer where a careless version gets a
//! wrong one:
//!
//! * **The orbital period is the one around the Sun**, not around the
//!   immediate primary. For a moon that means its planet's year. Get this
//!   wrong for a tidally locked moon and the formula divides by zero and
//!   reports an infinite day; get it right and the Moon's solar day comes out
//!   as the synodic month, 29.53 days, which is exactly what it is.
//! * **The sidereal rotation period carries a sign**, negative for a
//!   retrograde rotator. The two reciprocals then *add* in magnitude instead
//!   of cancelling, so Venus's solar day (116.75 d) is much *shorter* than its
//!   sidereal day (243.02 d) rather than longer. The signed result is
//!   negative, which says the Sun rises in the west; [`Body::solar_day_days`]
//!   returns the magnitude, because a length of day is a duration.
//!
//! Mercury is the test that catches a sign or reciprocal error immediately:
//! its 3:2 spin–orbit resonance makes the solar day exactly two Mercurian
//! years, 175.94 days.
//!
//! # Clock zero points
//!
//! A rate is a physical fact; a zero point is a convention. Only Earth and
//! Mars have a standardised one, and [`ClockEpoch::basis`] says which rows
//! have one and which carry a convention this crate declares. See [`crate::clock`].

use hc_core::math::{abs, floor};

use crate::mars;

/// What kind of object a row describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyKind {
    /// The Sun.
    Star,
    /// One of the eight planets.
    Planet,
    /// A dwarf planet.
    DwarfPlanet,
    /// A natural satellite.
    Moon,
}

/// Whether a clock's zero point is an international standard or a convention
/// this crate states for want of one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpochBasis {
    /// An established standard: Universal Time on Earth, Coordinated Mars Time
    /// on Mars.
    Standard,
    /// A zero point this crate declares. It is reproducible and documented,
    /// but it is not anyone else's.
    Convention,
}

/// Where a body's local-mean-time count starts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClockEpoch {
    /// TT days from J2000.0 to a local mean midnight at the body's prime
    /// meridian.
    pub j2000_offset_days: f64,
    /// The number given to the local day that begins at that midnight.
    pub day_number: i64,
    /// Whether the zero point is standardised or conventional.
    pub basis: EpochBasis,
    /// What the zero point is, in one sentence.
    pub note: &'static str,
}

/// The conventional zero point used for every body without a standardised
/// one: J2000.0 itself is declared to be local mean midnight at the prime
/// meridian, and days are numbered from zero.
const DECLARED: ClockEpoch = ClockEpoch {
    j2000_offset_days: 0.0,
    day_number: 0,
    basis: EpochBasis::Convention,
    note: "J2000.0 is declared to be local mean midnight at the prime meridian. \
           No standard exists for this body; only the rate is physical.",
};

/// One body's rotational and orbital data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Body {
    /// The English name.
    pub name: &'static str,
    /// What kind of object it is.
    pub kind: BodyKind,
    /// The body it orbits, or `None` for the Sun.
    pub primary: Option<&'static str>,
    /// The sidereal rotation period in hours, **negative for a retrograde
    /// rotator**.
    pub sidereal_rotation_hours: f64,
    /// The sidereal orbital period about [`Self::primary`], in days;
    /// negative for a retrograde orbit. Zero for the Sun.
    pub sidereal_orbit_days: f64,
    /// The obliquity of the rotation axis to the orbit, in degrees. For the
    /// Sun this is the tilt of the solar equator to the ecliptic.
    pub axial_tilt_degrees: f64,
    /// The semi-major axis of the orbit about [`Self::primary`], in
    /// kilometres. Zero for the Sun.
    pub semi_major_axis_km: f64,
    /// The solar day where it is itself a published measured constant rather
    /// than something to be derived, in SI seconds.
    ///
    /// Three bodies have one: Earth's day is 86 400 s by definition, Mars's
    /// sol is 88 775.244 s from Allison and McEwen, and the Moon's solar day
    /// is the mean synodic month. Preferring the measured value matters
    /// because the derivation inherits the rounding of a rotation period
    /// quoted to six figures — for Mars that is a third of a second per sol,
    /// which is four hours over the span of the Mars Sol Date. The tests check
    /// that the derived value agrees with the measured one to the table's own
    /// precision.
    pub measured_solar_day_seconds: Option<f64>,
    /// Where this row's numbers come from, and what is contested about them.
    pub source: &'static str,
    /// Where a local mean time count on this body starts.
    pub clock_epoch: ClockEpoch,
}

impl Body {
    /// The sidereal rotation period in seconds, as a magnitude.
    #[must_use]
    pub fn sidereal_rotation_seconds(&self) -> f64 {
        abs(self.sidereal_rotation_hours) * 3_600.0
    }

    /// Whether the body turns backwards with respect to its orbital motion.
    #[must_use]
    pub fn is_retrograde_rotator(&self) -> bool {
        self.sidereal_rotation_hours < 0.0
    }

    /// The period of this body's orbit **around the Sun**, in days.
    ///
    /// For a moon this is its planet's year, because that is what governs the
    /// apparent motion of the Sun in the moon's sky. Returns `None` for the
    /// Sun, which has no heliocentric orbit and therefore no solar day.
    #[must_use]
    pub fn heliocentric_year_days(&self) -> Option<f64> {
        let mut body = self;
        // Four hops is more than enough for Charon -> Pluto or Titan ->
        // Saturn; the bound stops a malformed table from looping forever.
        for _ in 0..4 {
            let name = body.primary?;
            if name == "Sun" {
                return Some(abs(body.sidereal_orbit_days));
            }
            body = by_name(name)?;
        }
        None
    }

    /// The length of the body's solar day in Earth days, derived from the
    /// sidereal rotation period and the heliocentric year.
    ///
    /// `None` for the Sun, and for any body whose rotation is exactly
    /// synchronous with its heliocentric year — a case that does not occur in
    /// this table but which the formula would otherwise report as infinite.
    #[must_use]
    pub fn derived_solar_day_days(&self) -> Option<f64> {
        let sidereal = self.sidereal_rotation_hours / 24.0;
        let year = self.heliocentric_year_days()?;
        let rate = 1.0 / sidereal - 1.0 / year;
        if rate == 0.0 || !rate.is_finite() {
            return None;
        }
        Some(abs(1.0 / rate))
    }

    /// The length of the body's solar day in Earth days: the measured value
    /// where there is one, the derivation otherwise.
    #[must_use]
    pub fn solar_day_days(&self) -> Option<f64> {
        match self.measured_solar_day_seconds {
            Some(seconds) => Some(seconds / 86_400.0),
            None => self.derived_solar_day_days(),
        }
    }

    /// The length of the body's solar day in SI seconds.
    #[must_use]
    pub fn solar_day_seconds(&self) -> Option<f64> {
        self.solar_day_days().map(|days| days * 86_400.0)
    }

    /// The body's year measured in its own solar days.
    ///
    /// Mercury's is 0.5 — half a solar day to the year, the other face of the
    /// 3:2 resonance. Mars's is 668.6 sols, the number every Mars mission
    /// plans against.
    #[must_use]
    pub fn year_in_local_days(&self) -> Option<f64> {
        let year = self.heliocentric_year_days()?;
        self.solar_day_days().map(|day| year / day)
    }

    /// The fraction of a local solar day elapsed at the prime meridian, in
    /// `[0, 1)`, together with the local day number.
    ///
    /// This is the raw mechanism behind [`crate::clock::BodyClock`]; see there
    /// for the longitude handling and for what the zero point means.
    #[must_use]
    pub fn local_day_index(&self, j2000_offset_days: f64) -> Option<f64> {
        let day = self.solar_day_days()?;
        let elapsed = (j2000_offset_days - self.clock_epoch.j2000_offset_days) / day;
        Some(elapsed + self.clock_epoch.day_number as f64)
    }
}

/// Look a body up by name. The comparison is case-sensitive.
#[must_use]
pub fn by_name(name: &str) -> Option<&'static Body> {
    ALL.iter().find(|body| body.name == name)
}

/// The number of bodies in the table.
#[must_use]
pub fn count() -> usize {
    ALL.len()
}

/// Whether a local day index falls on a whole day boundary, used by the tests
/// and by [`crate::clock`].
pub(crate) fn whole_and_fraction(index: f64) -> (i64, f64) {
    let whole = floor(index);
    (whole as i64, index - whole)
}

const NSSDC: &str = "NASA NSSDC Planetary Fact Sheet, nssdc.gsfc.nasa.gov/planetary/factsheet";

/// Every body this crate carries data for, ordered outward from the Sun with
/// each planet's moons following it.
pub const ALL: &[Body] = &[
    Body {
        name: "Sun",
        kind: BodyKind::Star,
        primary: None,
        // The Carrington rate, which NSSDC's own footnote says is the rate at
        // 16 degrees of latitude, not at the equator; the Sun rotates
        // differentially and has no single period. At the equator the same
        // sheet's law gives about 25.05 d.
        sidereal_rotation_hours: 609.12,
        sidereal_orbit_days: 0.0,
        axial_tilt_degrees: 7.25,
        semi_major_axis_km: 0.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Sun fact sheet; 609.12 h is the Carrington rate at 16 deg \
                 latitude, not the equatorial one",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Mercury",
        kind: BodyKind::Planet,
        primary: Some("Sun"),
        // The IAU 2015 rate, 6.1385108 deg/day; NSSDC rounds it to 1407.6 h,
        // which would stretch the derived solar day by half an hour and spoil
        // the 3:2 resonance that the same sheet's 4222.6 h assumes.
        sidereal_rotation_hours: 1_407.508_8,
        sidereal_orbit_days: 87.969,
        axial_tilt_degrees: 0.034,
        semi_major_axis_km: 57.909e6,
        measured_solar_day_seconds: None,
        source: "IAU 2015 rotation rate (Archinal et al. 2018); NSSDC quotes \
                 1407.6 h but derives its 4222.6 h solar day from this value",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Venus",
        kind: BodyKind::Planet,
        primary: Some("Sun"),
        sidereal_rotation_hours: -5_832.5,
        sidereal_orbit_days: 224.701,
        axial_tilt_degrees: 177.36,
        semi_major_axis_km: 108.209e6,
        measured_solar_day_seconds: None,
        source: "NSSDC comparison table; the Venus sheet itself says -5832.6 h \
                 and the IAU rate gives -5832.444 h",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Earth",
        kind: BodyKind::Planet,
        primary: Some("Sun"),
        sidereal_rotation_hours: 23.9345,
        sidereal_orbit_days: 365.256,
        axial_tilt_degrees: 23.44,
        semi_major_axis_km: 149.598e6,
        // The mean solar day is 86400 s by the definition of the unit.
        measured_solar_day_seconds: Some(86_400.0),
        source: NSSDC,
        clock_epoch: ClockEpoch {
            // 2000-01-01T00:00 TT, twelve hours before the J2000 epoch.
            j2000_offset_days: -0.5,
            // The Rata Die of 2000-01-01, so that the day number of this
            // clock is the fixed day number the rest of the workspace uses.
            day_number: 730_120,
            basis: EpochBasis::Standard,
            note: "Mean solar time at the Greenwich meridian, numbered by Rata \
                   Die. The day is a fixed 86400 s, so this clock is UT-like \
                   but uniform: it drifts from UT1 by delta-T, which is minutes \
                   over a century and is hc-astro's business, not this crate's.",
        },
    },
    Body {
        name: "Moon",
        kind: BodyKind::Moon,
        primary: Some("Earth"),
        sidereal_rotation_hours: 655.720,
        sidereal_orbit_days: 27.3217,
        axial_tilt_degrees: 6.68,
        semi_major_axis_km: 384_400.0,
        // The Moon's solar day is the mean synodic month.
        measured_solar_day_seconds: Some(hc_astro::MEAN_SYNODIC_MONTH * 86_400.0),
        source: "NSSDC Moon fact sheet; tilt is to its own orbit, 1.54 deg to \
                 the ecliptic",
        clock_epoch: ClockEpoch {
            // Meeus (49.1) puts the mean new moon of lunation 0 at
            // JDE 2451550.09766.
            j2000_offset_days: 5.097_66,
            day_number: 0,
            basis: EpochBasis::Convention,
            note: "Mean lunar midnight at the prime meridian is mean new moon, \
                   because the near side faces the Sun at full and away from it \
                   at new; the epoch is Meeus's mean new moon for lunation 0, so \
                   the day number is the Meeus lunation number. This is NOT \
                   Coordinated Lunar Time, which does not yet exist.",
        },
    },
    Body {
        name: "Mars",
        kind: BodyKind::Planet,
        primary: Some("Sun"),
        sidereal_rotation_hours: 24.6229,
        sidereal_orbit_days: 686.980,
        axial_tilt_degrees: 25.19,
        semi_major_axis_km: 227.956e6,
        // The sol implied by the Mars24 ratio. It differs from the quoted
        // 88775.244 s in the tenth significant figure, and using it is what
        // makes the generic clock and Coordinated Mars Time agree exactly
        // rather than drifting a second apart over the span of the Mars Sol
        // Date.
        measured_solar_day_seconds: Some(mars::SOL_IN_DAYS * 86_400.0),
        source: NSSDC,
        clock_epoch: ClockEpoch {
            j2000_offset_days: mars::MSD_EPOCH_J2000_OFFSET
                + mars::MSD_MIDNIGHT_ADJUSTMENT * mars::SOL_IN_DAYS,
            day_number: 44_796,
            basis: EpochBasis::Standard,
            note: "Coordinated Mars Time at the Airy-0 meridian, numbered by \
                   Mars Sol Date. Agrees with the mars module to the resolution \
                   of the table's rotation period.",
        },
    },
    Body {
        name: "Phobos",
        kind: BodyKind::Moon,
        primary: Some("Mars"),
        sidereal_rotation_hours: 7.653_84,
        sidereal_orbit_days: 0.318_91,
        axial_tilt_degrees: 0.0,
        semi_major_axis_km: 9_378.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Mars fact sheet; synchronous rotation",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Deimos",
        kind: BodyKind::Moon,
        primary: Some("Mars"),
        sidereal_rotation_hours: 30.298_56,
        sidereal_orbit_days: 1.262_44,
        axial_tilt_degrees: 0.0,
        semi_major_axis_km: 23_459.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Mars fact sheet; synchronous rotation",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Ceres",
        kind: BodyKind::DwarfPlanet,
        primary: Some("Sun"),
        sidereal_rotation_hours: 9.074_170,
        sidereal_orbit_days: 1_681.63,
        axial_tilt_degrees: 4.0,
        semi_major_axis_km: 414.0e6,
        measured_solar_day_seconds: None,
        source: "Konopliv et al. (2018) for the rotation; JPL Small-Body \
                 Database for the osculating orbit, which drifts with epoch",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Jupiter",
        kind: BodyKind::Planet,
        primary: Some("Sun"),
        // System III, the rotation of the magnetic field; Jupiter has no
        // surface and its cloud decks rotate at their own rates.
        sidereal_rotation_hours: 9.9250,
        sidereal_orbit_days: 4_332.589,
        axial_tilt_degrees: 3.13,
        semi_major_axis_km: 778.479e6,
        measured_solar_day_seconds: None,
        source: "NSSDC Jupiter fact sheet; System III rotation",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Io",
        kind: BodyKind::Moon,
        primary: Some("Jupiter"),
        sidereal_rotation_hours: 42.459_31,
        sidereal_orbit_days: 1.769_138,
        axial_tilt_degrees: 0.0,
        semi_major_axis_km: 421_800.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Jovian satellite fact sheet; synchronous rotation",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Europa",
        kind: BodyKind::Moon,
        primary: Some("Jupiter"),
        sidereal_rotation_hours: 85.228_35,
        sidereal_orbit_days: 3.551_181,
        axial_tilt_degrees: 0.0,
        semi_major_axis_km: 671_100.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Jovian satellite fact sheet; synchronous rotation",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Ganymede",
        kind: BodyKind::Moon,
        primary: Some("Jupiter"),
        sidereal_rotation_hours: 171.709_27,
        sidereal_orbit_days: 7.154_553,
        axial_tilt_degrees: 0.0,
        semi_major_axis_km: 1_070_400.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Jovian satellite fact sheet; synchronous rotation",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Callisto",
        kind: BodyKind::Moon,
        primary: Some("Jupiter"),
        sidereal_rotation_hours: 400.536_41,
        sidereal_orbit_days: 16.689_017,
        axial_tilt_degrees: 0.0,
        semi_major_axis_km: 1_882_700.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Jovian satellite fact sheet; synchronous rotation",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Saturn",
        kind: BodyKind::Planet,
        primary: Some("Sun"),
        sidereal_rotation_hours: 10.656,
        sidereal_orbit_days: 10_755.699,
        axial_tilt_degrees: 26.73,
        semi_major_axis_km: 1_432.041e6,
        measured_solar_day_seconds: None,
        source: "NSSDC Saturn fact sheet; System III rotation, which Cassini \
                 showed is not the interior's",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Enceladus",
        kind: BodyKind::Moon,
        primary: Some("Saturn"),
        sidereal_rotation_hours: 32.885_23,
        sidereal_orbit_days: 1.370_218,
        axial_tilt_degrees: 0.0,
        semi_major_axis_km: 238_020.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Saturnian satellite fact sheet; synchronous rotation",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Titan",
        kind: BodyKind::Moon,
        primary: Some("Saturn"),
        sidereal_rotation_hours: 382.690_10,
        sidereal_orbit_days: 15.945_421,
        axial_tilt_degrees: 0.3,
        semi_major_axis_km: 1_221_870.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Saturnian satellite fact sheet; synchronous rotation",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Uranus",
        kind: BodyKind::Planet,
        primary: Some("Sun"),
        sidereal_rotation_hours: -17.24,
        sidereal_orbit_days: 30_685.4,
        axial_tilt_degrees: 97.77,
        semi_major_axis_km: 2_867.043e6,
        measured_solar_day_seconds: None,
        source: "NSSDC Uranus fact sheet; the 97.77 deg tilt makes the rotation \
                 retrograde in the ecliptic sense",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Neptune",
        kind: BodyKind::Planet,
        primary: Some("Sun"),
        sidereal_rotation_hours: 16.11,
        sidereal_orbit_days: 60_189.018,
        axial_tilt_degrees: 28.32,
        semi_major_axis_km: 4_514.953e6,
        measured_solar_day_seconds: None,
        source: "NSSDC Neptune fact sheet, the IAU 2009 System III value; the \
                 IAU 2015 report adopts 15.9663 h after Karkoschka (2011), and \
                 the two are genuinely in conflict",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Triton",
        kind: BodyKind::Moon,
        primary: Some("Neptune"),
        sidereal_rotation_hours: -141.044_50,
        sidereal_orbit_days: -5.876_854,
        axial_tilt_degrees: 0.0,
        semi_major_axis_km: 354_760.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Neptunian satellite fact sheet; retrograde orbit at \
                 157.345 deg inclination, synchronously locked to it",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Pluto",
        kind: BodyKind::DwarfPlanet,
        primary: Some("Sun"),
        sidereal_rotation_hours: -153.2928,
        sidereal_orbit_days: 90_560.0,
        axial_tilt_degrees: 119.51,
        semi_major_axis_km: 5_906.4e6,
        measured_solar_day_seconds: None,
        source: "NSSDC Pluto fact sheet; the IAU convention calls the same \
                 rotation prograde about its own defined pole",
        clock_epoch: DECLARED,
    },
    Body {
        name: "Charon",
        kind: BodyKind::Moon,
        primary: Some("Pluto"),
        sidereal_rotation_hours: -153.2928,
        sidereal_orbit_days: 6.3872,
        axial_tilt_degrees: 0.0,
        semi_major_axis_km: 19_596.0,
        measured_solar_day_seconds: None,
        source: "NSSDC Pluto fact sheet; Pluto and Charon are mutually locked, \
                 so they share a rotation rate exactly",
        clock_epoch: DECLARED,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    fn body(name: &str) -> &'static Body {
        by_name(name).unwrap()
    }

    #[test]
    fn the_table_covers_everything_the_crate_promises() {
        let expected = [
            "Sun",
            "Mercury",
            "Venus",
            "Earth",
            "Moon",
            "Mars",
            "Phobos",
            "Deimos",
            "Jupiter",
            "Io",
            "Europa",
            "Ganymede",
            "Callisto",
            "Saturn",
            "Titan",
            "Enceladus",
            "Uranus",
            "Neptune",
            "Triton",
            "Pluto",
            "Charon",
            "Ceres",
        ];
        for name in expected {
            assert!(by_name(name).is_some(), "{name} missing");
        }
        assert_eq!(count(), expected.len());
        assert!(by_name("Planet Nine").is_none());
    }

    #[test]
    fn every_row_has_a_source_and_a_plausible_primary() {
        for entry in ALL {
            assert!(!entry.source.is_empty(), "{}", entry.name);
            match entry.kind {
                BodyKind::Star => assert!(entry.primary.is_none()),
                BodyKind::Planet | BodyKind::DwarfPlanet => {
                    assert_eq!(entry.primary, Some("Sun"), "{}", entry.name);
                }
                BodyKind::Moon => {
                    let primary = by_name(entry.primary.unwrap()).unwrap();
                    assert_ne!(primary.kind, BodyKind::Moon, "{}", entry.name);
                }
            }
        }
    }

    #[test]
    fn a_solar_day_on_mercury_is_two_mercurian_years() {
        let mercury = body("Mercury");
        let day = mercury.solar_day_days().unwrap();
        assert!((day - 175.94).abs() < 0.01, "{day}");
        // The 3:2 resonance, stated the other way round.
        let ratio = day / mercury.sidereal_orbit_days;
        assert!((ratio - 2.0).abs() < 1e-3, "{ratio}");
        assert!((mercury.year_in_local_days().unwrap() - 0.5).abs() < 1e-3);
        // NSSDC publishes 4222.6 h.
        let hours = day * 24.0;
        assert!((hours - 4_222.6).abs() < 0.2, "{hours}");
    }

    #[test]
    fn venus_rotates_backwards_and_still_gets_a_sensible_solar_day() {
        let venus = body("Venus");
        assert!(venus.is_retrograde_rotator());
        let day = venus.solar_day_days().unwrap();
        // NSSDC publishes 2802.0 h = 116.75 d.
        assert!(day > 0.0, "a length of day must be positive: {day}");
        assert!((day - 116.75).abs() < 0.01, "{day}");
        // The solar day is far SHORTER than the sidereal day, which is the
        // signature of retrograde rotation and the thing a sign error breaks.
        assert!(day < abs(venus.sidereal_rotation_hours) / 24.0);
        assert!((day * 24.0 - 2_802.0).abs() < 0.2);
    }

    #[test]
    fn the_earths_solar_day_comes_out_at_exactly_twenty_four_hours() {
        let day = body("Earth").solar_day_days().unwrap();
        assert!((day - 1.0).abs() < 2e-6, "{day}");
        assert!((day * 24.0 - 24.0).abs() < 1e-4);
    }

    #[test]
    fn the_moons_solar_day_is_the_synodic_month() {
        // The check that the heliocentric-year rule is right: a tidally locked
        // moon has a solar day, and for the Moon it is the month everybody
        // knows.
        let day = body("Moon").solar_day_days().unwrap();
        assert!((day - 29.530_59).abs() < 1e-4, "{day}");
        assert!((day - hc_astro::MEAN_SYNODIC_MONTH).abs() < 1e-3);
    }

    #[test]
    fn the_martian_solar_day_derived_from_the_table_is_the_sol() {
        let seconds = body("Mars").solar_day_seconds().unwrap();
        assert!((seconds - mars::MARS_SOL_SECONDS).abs() < 1e-3, "{seconds}");
        // The table's rotation period is quoted to six figures, so the value
        // derived from it agrees with the measured sol only to a third of a
        // second -- which is why the measured one is carried.
        let derived = body("Mars").derived_solar_day_days().unwrap() * 86_400.0;
        assert!((derived - mars::MARS_SOL_SECONDS).abs() < 0.4, "{derived}");
        let sols = body("Mars").year_in_local_days().unwrap();
        assert!((sols - 668.6).abs() < 0.05, "{sols}");
    }

    #[test]
    fn a_measured_solar_day_agrees_with_the_one_derived_from_the_table() {
        // Where both exist they must not disagree by more than the rounding of
        // the rotation period that the derivation is built on.
        for (name, tolerance) in [("Earth", 2e-1), ("Mars", 0.4), ("Moon", 1.0)] {
            let entry = body(name);
            let measured = entry.measured_solar_day_seconds.unwrap();
            let derived = entry.derived_solar_day_days().unwrap() * 86_400.0;
            assert!(
                (measured - derived).abs() < tolerance,
                "{name}: {measured} vs {derived}"
            );
        }
        // And every other row has no measured value to disagree with.
        for entry in ALL {
            if !matches!(entry.name, "Earth" | "Mars" | "Moon") {
                assert!(entry.measured_solar_day_seconds.is_none(), "{}", entry.name);
            }
        }
    }

    #[test]
    fn the_sun_has_no_solar_day() {
        let sun = body("Sun");
        assert!(sun.heliocentric_year_days().is_none());
        assert!(sun.solar_day_days().is_none());
        assert!(sun.solar_day_seconds().is_none());
        assert!(sun.year_in_local_days().is_none());
        assert!(sun.local_day_index(0.0).is_none());
    }

    #[test]
    fn moons_inherit_their_planets_year() {
        for (moon, planet) in [
            ("Moon", "Earth"),
            ("Phobos", "Mars"),
            ("Io", "Jupiter"),
            ("Titan", "Saturn"),
            ("Triton", "Neptune"),
            ("Charon", "Pluto"),
        ] {
            assert_eq!(
                by_name(moon).unwrap().heliocentric_year_days(),
                by_name(planet).unwrap().heliocentric_year_days(),
                "{moon}"
            );
        }
    }

    #[test]
    fn a_tidally_locked_moons_solar_day_is_a_little_longer_than_its_orbit() {
        for name in ["Io", "Europa", "Ganymede", "Callisto", "Titan", "Enceladus"] {
            let entry = body(name);
            let day = entry.solar_day_days().unwrap();
            let orbit = abs(entry.sidereal_orbit_days);
            assert!(day > orbit, "{name}: {day} vs {orbit}");
            assert!((day - orbit) / orbit < 0.01, "{name}");
        }
        // Titan's is about 15.97 days against a 15.945-day orbit.
        let titan = body("Titan").solar_day_days().unwrap();
        assert!((titan - 15.969).abs() < 0.01, "{titan}");
    }

    #[test]
    fn a_retrograde_rotators_solar_day_is_shorter_than_its_sidereal_day() {
        for name in ["Venus", "Uranus", "Pluto", "Triton", "Charon"] {
            let entry = body(name);
            assert!(entry.is_retrograde_rotator(), "{name}");
            let solar = entry.solar_day_days().unwrap();
            let sidereal = abs(entry.sidereal_rotation_hours) / 24.0;
            assert!(solar < sidereal, "{name}: {solar} vs {sidereal}");
        }
        // Pluto's is only twenty-odd seconds shorter, because its year is so
        // long; Venus's is halved.
        let pluto = body("Pluto");
        let gap =
            (abs(pluto.sidereal_rotation_hours) - pluto.solar_day_days().unwrap() * 24.0) * 3_600.0;
        assert!((gap - 38.9).abs() < 2.0, "{gap} s");
    }

    #[test]
    fn a_prograde_rotators_solar_day_is_longer_than_its_sidereal_day() {
        for name in ["Mercury", "Earth", "Mars", "Jupiter", "Saturn", "Neptune"] {
            let entry = body(name);
            assert!(!entry.is_retrograde_rotator(), "{name}");
            let solar = entry.solar_day_days().unwrap();
            let sidereal = entry.sidereal_rotation_hours / 24.0;
            assert!(solar > sidereal, "{name}: {solar} vs {sidereal}");
        }
    }

    #[test]
    fn the_year_in_local_days_is_the_year_divided_by_the_day() {
        for entry in ALL {
            let Some(year) = entry.heliocentric_year_days() else {
                continue;
            };
            let day = entry.solar_day_days().unwrap();
            let local = entry.year_in_local_days().unwrap();
            assert!((local * day - year).abs() < 1e-6, "{}", entry.name);
            assert!(local > 0.0, "{}", entry.name);
        }
        // A Jovian year is about 10 476 of Jupiter's ten-hour days.
        let jupiter = body("Jupiter").year_in_local_days().unwrap();
        assert!((jupiter - 10_476.0).abs() < 2.0, "{jupiter}");
    }

    #[test]
    fn the_synodic_relation_holds_in_reverse_for_every_body() {
        // 1/P_solar = 1/P_sidereal - 1/P_orbit, recomputed from the answer.
        for entry in ALL {
            let Some(day) = entry.derived_solar_day_days() else {
                continue;
            };
            let year = entry.heliocentric_year_days().unwrap();
            let sidereal = entry.sidereal_rotation_hours / 24.0;
            let expected = 1.0 / sidereal - 1.0 / year;
            let recovered = if expected < 0.0 {
                -1.0 / day
            } else {
                1.0 / day
            };
            // The derivation, not the measured override, is what this checks.
            assert!(
                (recovered - expected).abs() < 1e-12,
                "{}: {recovered} vs {expected}",
                entry.name
            );
        }
    }

    #[test]
    fn the_axial_tilts_are_in_range_and_name_the_retrograde_planets() {
        for entry in ALL {
            assert!(
                (0.0..=180.0).contains(&entry.axial_tilt_degrees),
                "{}",
                entry.name
            );
        }
        // A tilt past 90 degrees is retrograde rotation stated as geometry.
        for name in ["Venus", "Uranus", "Pluto"] {
            assert!(body(name).axial_tilt_degrees > 90.0, "{name}");
        }
        assert!((body("Earth").axial_tilt_degrees - 23.44).abs() < 1e-9);
    }

    #[test]
    fn only_earth_and_mars_have_a_standardised_clock_zero() {
        for entry in ALL {
            let standard = entry.clock_epoch.basis == EpochBasis::Standard;
            assert_eq!(
                standard,
                entry.name == "Earth" || entry.name == "Mars",
                "{}",
                entry.name
            );
            assert!(!entry.clock_epoch.note.is_empty(), "{}", entry.name);
        }
    }

    #[test]
    fn the_martian_clock_epoch_agrees_with_the_mars_module() {
        let epoch = body("Mars").clock_epoch;
        let moment = mars::MarsMoment::from_j2000_offset(epoch.j2000_offset_days);
        assert!(
            (moment.mars_sol_date() - epoch.day_number as f64).abs() < 1e-9,
            "{}",
            moment.mars_sol_date()
        );
    }

    #[test]
    fn the_local_day_index_advances_by_one_per_solar_day() {
        let mars = body("Mars");
        let day = mars.solar_day_days().unwrap();
        let first = mars.local_day_index(0.0).unwrap();
        let second = mars.local_day_index(day).unwrap();
        assert!((second - first - 1.0).abs() < 1e-9);
        let (whole, fraction) = whole_and_fraction(first);
        assert!((0.0..1.0).contains(&fraction));
        assert_eq!(whole as f64 + fraction, first);
    }

    #[test]
    fn the_semi_major_axes_increase_outward_for_the_planets() {
        let order = [
            "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto",
        ];
        let mut previous = 0.0;
        for name in order {
            let axis = body(name).semi_major_axis_km;
            assert!(axis > previous, "{name}");
            previous = axis;
        }
        // Ceres sits between Mars and Jupiter, as an asteroid-belt body must.
        let ceres = body("Ceres").semi_major_axis_km;
        assert!(ceres > body("Mars").semi_major_axis_km);
        assert!(ceres < body("Jupiter").semi_major_axis_km);
    }

    #[test]
    fn the_only_retrograde_orbit_in_the_table_is_tritons() {
        for entry in ALL {
            let retrograde = entry.sidereal_orbit_days < 0.0;
            assert_eq!(retrograde, entry.name == "Triton", "{}", entry.name);
        }
        // It is still a positive length of day.
        assert!(body("Triton").solar_day_days().unwrap() > 0.0);
    }
}

hc_core::catalogue_tests! {
    type: Body,
    id: |body| body.name,
    provenance: |body| body.source,
    tests: body_table_tests,
    all: ALL,
    lookup: by_name,
}
