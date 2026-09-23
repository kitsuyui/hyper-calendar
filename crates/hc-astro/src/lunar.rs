//! The Moon: apparent position, phase, and the conjunction search that every
//! lunisolar and lunar calendar is built on.
//!
//! Two independent algorithms live here, and that is deliberate.
//!
//! * [`lunar_longitude`] evaluates the abridged ELP-2000/82 series printed in
//!   Meeus, *Astronomical Algorithms*, 2nd ed., tables 47.A and 47.B — sixty
//!   periodic terms in longitude and distance, sixty in latitude. Meeus
//!   states it as good to about 10″ in longitude and 4″ in latitude, and it
//!   answers "where is the Moon now".
//! * [`nth_new_moon`] evaluates the phase series of Meeus chapter 49, which
//!   is fitted directly to the *time* of a conjunction rather than to a
//!   position. It answers "when is the next new moon", to within a few
//!   seconds, and it is what the Chinese and observational Hijri calendars
//!   want.
//!
//! The two are checked against each other in this module's tests: the
//! conjunction found by searching the longitude series agrees with the
//! conjunction predicted by the phase series to well under a minute. Neither
//! is derived from the other, so that agreement is real evidence.

use hc_calendar::fixed::Moment;
use hc_core::math::{RAD_TO_DEG, acos, asin, atan2, cos_deg, normalize_degrees, sin_deg};

use crate::earth::{Equatorial, equatorial_from_ecliptic, true_obliquity_at_centuries};
use crate::search::invert_angular;
use crate::solar::{solar_longitude, solar_radius_vector_at_centuries};
use crate::time::{julian_centuries, universal_time};
use crate::util::{clamp, max_of, modulo, poly};

/// The mean synodic month — the mean interval between new moons — in days.
///
/// Meeus, chapter 49. The *actual* interval swings between about 29.27 and
/// 29.83 days because both orbits are eccentric; this value is only ever
/// used to seed a search.
pub const MEAN_SYNODIC_MONTH: f64 = 29.530_588_861;

/// The mean distance to the Moon in kilometres, the constant term of the
/// distance series (Meeus, chapter 47).
const MEAN_LUNAR_DISTANCE_KM: f64 = 385_000.56;

/// The Earth's equatorial radius in kilometres, used for horizontal parallax.
const EARTH_RADIUS_KM: f64 = 6_378.14;

/// The astronomical unit in kilometres (IAU 2012 definition), used to put the
/// Earth–Sun and Earth–Moon distances into the same units.
const ASTRONOMICAL_UNIT_KM: f64 = 149_597_870.7;

/// Meeus, table 47.A: the arguments `D`, `M`, `M′`, `F`, then the longitude
/// coefficient in units of 10⁻⁶ degrees and the distance coefficient in units
/// of 10⁻³ kilometres.
const LONGITUDE_AND_DISTANCE_TERMS: [(i8, i8, i8, i8, i32, i32); 60] = [
    (0, 0, 1, 0, 6_288_774, -20_905_355),
    (2, 0, -1, 0, 1_274_027, -3_699_111),
    (2, 0, 0, 0, 658_314, -2_955_968),
    (0, 0, 2, 0, 213_618, -569_925),
    (0, 1, 0, 0, -185_116, 48_888),
    (0, 0, 0, 2, -114_332, -3_149),
    (2, 0, -2, 0, 58_793, 246_158),
    (2, -1, -1, 0, 57_066, -152_138),
    (2, 0, 1, 0, 53_322, -170_733),
    (2, -1, 0, 0, 45_758, -204_586),
    (0, 1, -1, 0, -40_923, -129_620),
    (1, 0, 0, 0, -34_720, 108_743),
    (0, 1, 1, 0, -30_383, 104_755),
    (2, 0, 0, -2, 15_327, 10_321),
    (0, 0, 1, 2, -12_528, 0),
    (0, 0, 1, -2, 10_980, 79_661),
    (4, 0, -1, 0, 10_675, -34_782),
    (0, 0, 3, 0, 10_034, -23_210),
    (4, 0, -2, 0, 8_548, -21_636),
    (2, 1, -1, 0, -7_888, 24_208),
    (2, 1, 0, 0, -6_766, 30_824),
    (1, 0, -1, 0, -5_163, -8_379),
    (1, 1, 0, 0, 4_987, -16_675),
    (2, -1, 1, 0, 4_036, -12_831),
    (2, 0, 2, 0, 3_994, -10_445),
    (4, 0, 0, 0, 3_861, -11_650),
    (2, 0, -3, 0, 3_665, 14_403),
    (0, 1, -2, 0, -2_689, -7_003),
    (2, 0, -1, 2, -2_602, 0),
    (2, -1, -2, 0, 2_390, 10_056),
    (1, 0, 1, 0, -2_348, 6_322),
    (2, -2, 0, 0, 2_236, -9_884),
    (0, 1, 2, 0, -2_120, 5_751),
    (0, 2, 0, 0, -2_069, 0),
    (2, -2, -1, 0, 2_048, -4_950),
    (2, 0, 1, -2, -1_773, 4_130),
    (2, 0, 0, 2, -1_595, 0),
    (4, -1, -1, 0, 1_215, -3_958),
    (0, 0, 2, 2, -1_110, 0),
    (3, 0, -1, 0, -892, 3_258),
    (2, 1, 1, 0, -810, 2_616),
    (4, -1, -2, 0, 759, -1_897),
    (0, 2, -1, 0, -713, -2_117),
    (2, 2, -1, 0, -700, 2_354),
    (2, 1, -2, 0, 691, 0),
    (2, -1, 0, -2, 596, 0),
    (4, 0, 1, 0, 549, -1_423),
    (0, 0, 4, 0, 537, -1_117),
    (4, -1, 0, 0, 520, -1_571),
    (1, 0, -2, 0, -487, -1_739),
    (2, 1, 0, -2, -399, 0),
    (0, 0, 2, -2, -381, -4_421),
    (1, 1, 1, 0, 351, 0),
    (3, 0, -2, 0, -340, 0),
    (4, 0, -3, 0, 330, 0),
    (2, -1, 2, 0, 327, 0),
    (0, 2, 1, 0, -323, 1_165),
    (1, 1, -1, 0, 299, 0),
    (2, 0, 3, 0, 294, 0),
    (2, 0, -1, -2, 0, 8_752),
];

/// Meeus, table 47.B: the arguments `D`, `M`, `M′`, `F` and the latitude
/// coefficient in units of 10⁻⁶ degrees.
const LATITUDE_TERMS: [(i8, i8, i8, i8, i32); 60] = [
    (0, 0, 0, 1, 5_128_122),
    (0, 0, 1, 1, 280_602),
    (0, 0, 1, -1, 277_693),
    (2, 0, 0, -1, 173_237),
    (2, 0, -1, 1, 55_413),
    (2, 0, -1, -1, 46_271),
    (2, 0, 0, 1, 32_573),
    (0, 0, 2, 1, 17_198),
    (2, 0, 1, -1, 9_266),
    (0, 0, 2, -1, 8_822),
    (2, -1, 0, -1, 8_216),
    (2, 0, -2, -1, 4_324),
    (2, 0, 1, 1, 4_200),
    (2, 1, 0, -1, -3_359),
    (2, -1, -1, 1, 2_463),
    (2, -1, 0, 1, 2_211),
    (2, -1, -1, -1, 2_065),
    (0, 1, -1, -1, -1_870),
    (4, 0, -1, -1, 1_828),
    (0, 1, 0, 1, -1_794),
    (0, 0, 0, 3, -1_749),
    (0, 1, -1, 1, -1_565),
    (1, 0, 0, 1, -1_491),
    (0, 1, 1, 1, -1_475),
    (0, 1, 1, -1, -1_410),
    (0, 1, 0, -1, -1_344),
    (1, 0, 0, -1, -1_335),
    (0, 0, 3, 1, 1_107),
    (4, 0, 0, -1, 1_021),
    (4, 0, -1, 1, 833),
    (0, 0, 1, -3, 777),
    (4, 0, -2, 1, 671),
    (2, 0, 0, -3, 607),
    (2, 0, 2, -1, 596),
    (2, -1, 1, -1, 491),
    (2, 0, -2, 1, -451),
    (0, 0, 3, -1, 439),
    (2, 0, 2, 1, 422),
    (2, 0, -3, -1, 421),
    (2, 1, -1, 1, -366),
    (2, 1, 0, 1, -351),
    (4, 0, 0, 1, 331),
    (2, -1, 1, 1, 315),
    (2, -2, 0, -1, 302),
    (0, 0, 1, 3, -283),
    (2, 1, 1, -1, -229),
    (1, 1, 0, -1, 223),
    (1, 1, 0, 1, 223),
    (0, 1, -2, -1, -220),
    (2, 1, -1, -1, -220),
    (1, 0, 1, 1, -185),
    (2, -1, -2, -1, 181),
    (0, 1, 2, 1, -177),
    (4, 0, -2, -1, 176),
    (4, -1, -1, -1, 166),
    (1, 0, 1, -1, -164),
    (4, 0, 1, -1, 132),
    (1, 0, -1, -1, -119),
    (4, -1, 0, -1, 115),
    (2, -2, 0, 1, 107),
];

/// The five fundamental arguments of the lunar theory, in degrees, plus the
/// eccentricity factor `E` that scales every term involving the Sun's mean
/// anomaly.
#[derive(Debug, Clone, Copy)]
struct LunarArguments {
    mean_longitude: f64,
    elongation: f64,
    solar_anomaly: f64,
    lunar_anomaly: f64,
    argument_of_latitude: f64,
    eccentricity_factor: f64,
    venus: f64,
    jupiter: f64,
    flattening: f64,
}

/// Meeus (47.1) to (47.7).
fn lunar_arguments(centuries: f64) -> LunarArguments {
    let t = centuries;
    LunarArguments {
        mean_longitude: normalize_degrees(poly(
            t,
            &[
                218.316_447_7,
                481_267.881_234_21,
                -0.001_578_6,
                1.0 / 538_841.0,
                -1.0 / 65_194_000.0,
            ],
        )),
        elongation: normalize_degrees(poly(
            t,
            &[
                297.850_192_1,
                445_267.111_403_4,
                -0.001_881_9,
                1.0 / 545_868.0,
                -1.0 / 113_065_000.0,
            ],
        )),
        solar_anomaly: normalize_degrees(poly(
            t,
            &[
                357.529_109_2,
                35_999.050_290_9,
                -0.000_153_6,
                1.0 / 24_490_000.0,
            ],
        )),
        lunar_anomaly: normalize_degrees(poly(
            t,
            &[
                134.963_396_4,
                477_198.867_505_5,
                0.008_741_4,
                1.0 / 69_699.0,
                -1.0 / 14_712_000.0,
            ],
        )),
        argument_of_latitude: normalize_degrees(poly(
            t,
            &[
                93.272_095_0,
                483_202.017_523_3,
                -0.003_653_9,
                -1.0 / 3_526_000.0,
                1.0 / 863_310_000.0,
            ],
        )),
        eccentricity_factor: poly(t, &[1.0, -0.002_516, -0.000_007_4]),
        venus: normalize_degrees(poly(t, &[119.75, 131.849])),
        jupiter: normalize_degrees(poly(t, &[53.09, 479_264.290])),
        flattening: normalize_degrees(poly(t, &[313.45, 481_266.484])),
    }
}

/// `E` raised to the power of `|m|`, which is how Meeus scales the terms
/// containing the Sun's mean anomaly: the Earth's orbital eccentricity is
/// itself changing, and these terms change with it.
fn eccentricity_power(eccentricity_factor: f64, solar_anomaly_multiple: i8) -> f64 {
    match solar_anomaly_multiple.abs() {
        0 => 1.0,
        1 => eccentricity_factor,
        _ => eccentricity_factor * eccentricity_factor,
    }
}

/// The three periodic sums of Meeus chapter 47: `Σl` and `Σb` in units of
/// 10⁻⁶ degrees, `Σr` in units of 10⁻³ kilometres.
fn lunar_sums(arguments: &LunarArguments) -> (f64, f64, f64) {
    let mut longitude = 0.0;
    let mut distance = 0.0;
    for (d, m, m_prime, f, sigma_l, sigma_r) in LONGITUDE_AND_DISTANCE_TERMS {
        let angle = f64::from(d) * arguments.elongation
            + f64::from(m) * arguments.solar_anomaly
            + f64::from(m_prime) * arguments.lunar_anomaly
            + f64::from(f) * arguments.argument_of_latitude;
        let scale = eccentricity_power(arguments.eccentricity_factor, m);
        longitude += f64::from(sigma_l) * scale * sin_deg(angle);
        distance += f64::from(sigma_r) * scale * cos_deg(angle);
    }

    let mut latitude = 0.0;
    for (d, m, m_prime, f, sigma_b) in LATITUDE_TERMS {
        let angle = f64::from(d) * arguments.elongation
            + f64::from(m) * arguments.solar_anomaly
            + f64::from(m_prime) * arguments.lunar_anomaly
            + f64::from(f) * arguments.argument_of_latitude;
        latitude += f64::from(sigma_b)
            * eccentricity_power(arguments.eccentricity_factor, m)
            * sin_deg(angle);
    }

    // The additive terms of Meeus chapter 47: the action of Venus (A1), of
    // Jupiter (A2), and of the Earth's flattening (A3).
    longitude += 3_958.0 * sin_deg(arguments.venus)
        + 1_962.0 * sin_deg(arguments.mean_longitude - arguments.argument_of_latitude)
        + 318.0 * sin_deg(arguments.jupiter);
    latitude += -2_235.0 * sin_deg(arguments.mean_longitude)
        + 382.0 * sin_deg(arguments.flattening)
        + 175.0 * sin_deg(arguments.venus - arguments.argument_of_latitude)
        + 175.0 * sin_deg(arguments.venus + arguments.argument_of_latitude)
        + 127.0 * sin_deg(arguments.mean_longitude - arguments.lunar_anomaly)
        - 115.0 * sin_deg(arguments.mean_longitude + arguments.lunar_anomaly);

    (longitude, latitude, distance)
}

/// The Moon's geometric ecliptic longitude in degrees, referred to the mean
/// equinox of the date — no nutation.
#[must_use]
pub fn geometric_lunar_longitude_at_centuries(centuries: f64) -> f64 {
    let arguments = lunar_arguments(centuries);
    let (longitude, _, _) = lunar_sums(&arguments);
    normalize_degrees(arguments.mean_longitude + longitude / 1_000_000.0)
}

/// The Moon's apparent ecliptic longitude in degrees, referred to the true
/// equinox of the date.
///
/// Unlike the Sun, the Moon is close enough that annual aberration of its
/// light is negligible against the 10″ accuracy of the series, so only
/// nutation is added.
#[must_use]
pub fn lunar_longitude_at_centuries(centuries: f64) -> f64 {
    normalize_degrees(
        geometric_lunar_longitude_at_centuries(centuries)
            + crate::earth::nutation_at_centuries(centuries).longitude_degrees,
    )
}

/// The Moon's apparent ecliptic longitude in degrees at a Universal Time
/// moment.
#[must_use]
pub fn lunar_longitude(moment: Moment) -> f64 {
    lunar_longitude_at_centuries(julian_centuries(moment))
}

/// The Moon's ecliptic latitude in degrees, positive north of the ecliptic.
#[must_use]
pub fn lunar_latitude_at_centuries(centuries: f64) -> f64 {
    let arguments = lunar_arguments(centuries);
    let (_, latitude, _) = lunar_sums(&arguments);
    latitude / 1_000_000.0
}

/// The Moon's ecliptic latitude in degrees at a Universal Time moment.
#[must_use]
pub fn lunar_latitude(moment: Moment) -> f64 {
    lunar_latitude_at_centuries(julian_centuries(moment))
}

/// The distance from the centre of the Earth to the centre of the Moon, in
/// kilometres.
#[must_use]
pub fn lunar_distance_at_centuries(centuries: f64) -> f64 {
    let arguments = lunar_arguments(centuries);
    let (_, _, distance) = lunar_sums(&arguments);
    MEAN_LUNAR_DISTANCE_KM + distance / 1_000.0
}

/// The Earth–Moon distance in kilometres at a Universal Time moment.
#[must_use]
pub fn lunar_distance(moment: Moment) -> f64 {
    lunar_distance_at_centuries(julian_centuries(moment))
}

/// The Moon's equatorial horizontal parallax in degrees — the angle the
/// Earth's radius subtends at the Moon, which is what makes a moonrise
/// happen a little earlier than geometry alone would say.
#[must_use]
pub fn lunar_parallax(moment: Moment) -> f64 {
    asin(clamp(EARTH_RADIUS_KM / lunar_distance(moment), -1.0, 1.0)) * RAD_TO_DEG
}

/// The Moon's apparent right ascension and declination at a Universal Time
/// moment, as seen from the centre of the Earth.
#[must_use]
pub fn lunar_position(moment: Moment) -> Equatorial {
    let centuries = julian_centuries(moment);
    equatorial_from_ecliptic(
        lunar_longitude_at_centuries(centuries),
        lunar_latitude_at_centuries(centuries),
        true_obliquity_at_centuries(centuries),
    )
}

/// The Moon's elongation from the Sun in degrees: 0° at new moon, 90° at
/// first quarter, 180° at full, 270° at last quarter.
///
/// This is the difference of apparent ecliptic longitudes, which is the
/// definition the calendars use — not the true angular separation on the
/// sky, which also involves the Moon's latitude.
#[must_use]
pub fn lunar_phase(moment: Moment) -> f64 {
    modulo(lunar_longitude(moment) - solar_longitude(moment), 360.0)
}

/// The fraction of the Moon's disc that is illuminated, from 0 at new moon
/// to 1 at full.
///
/// Meeus (48.1) to (48.3): the true elongation `ψ` from the Sun, then the
/// phase angle `i` of the Sun–Moon–Earth triangle, then
/// `k = (1 + cos i)/2`. Meeus gives this as accurate to about 0.0005.
#[must_use]
pub fn lunar_illuminated_fraction(moment: Moment) -> f64 {
    let centuries = julian_centuries(moment);
    let lunar = lunar_longitude_at_centuries(centuries);
    let latitude = lunar_latitude_at_centuries(centuries);
    let solar = crate::solar::solar_longitude_at_centuries(centuries);
    let solar_distance = solar_radius_vector_at_centuries(centuries) * ASTRONOMICAL_UNIT_KM;
    let lunar_distance = lunar_distance_at_centuries(centuries);

    let cos_elongation = clamp(cos_deg(latitude) * cos_deg(lunar - solar), -1.0, 1.0);
    let elongation = acos(cos_elongation) * RAD_TO_DEG;
    let phase_angle = atan2(
        solar_distance * sin_deg(elongation),
        lunar_distance - solar_distance * cos_elongation,
    ) * RAD_TO_DEG;
    (1.0 + cos_deg(phase_angle)) / 2.0
}

/// Which quarter of the lunation a phase instant belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoonPhase {
    /// Conjunction: the Moon and Sun share an ecliptic longitude.
    New,
    /// The Moon is 90° east of the Sun.
    FirstQuarter,
    /// Opposition: the Moon is 180° from the Sun.
    Full,
    /// The Moon is 270° east of the Sun.
    LastQuarter,
}

impl MoonPhase {
    /// The elongation from the Sun, in degrees, that defines this phase.
    #[must_use]
    pub const fn elongation_degrees(self) -> f64 {
        match self {
            Self::New => 0.0,
            Self::FirstQuarter => 90.0,
            Self::Full => 180.0,
            Self::LastQuarter => 270.0,
        }
    }

    /// The fraction Meeus adds to the lunation index `k` for this phase.
    #[must_use]
    pub const fn lunation_offset(self) -> f64 {
        match self {
            Self::New => 0.0,
            Self::FirstQuarter => 0.25,
            Self::Full => 0.5,
            Self::LastQuarter => 0.75,
        }
    }
}

/// One periodic term of the phase series: the multiples of `M`, `M′` and `F`,
/// the coefficient in days, and the power of `E` that scales it.
type PhaseTerm = (i8, i8, i8, f64, i8);

/// Meeus, chapter 49: corrections to the time of a new moon, in days.
const NEW_MOON_TERMS: [PhaseTerm; 24] = [
    (0, 1, 0, -0.407_20, 0),
    (1, 0, 0, 0.172_41, 1),
    (0, 2, 0, 0.016_08, 0),
    (0, 0, 2, 0.010_39, 0),
    (-1, 1, 0, 0.007_39, 1),
    (1, 1, 0, -0.005_14, 1),
    (2, 0, 0, 0.002_08, 2),
    (0, 1, -2, -0.001_11, 0),
    (0, 1, 2, -0.000_57, 0),
    (1, 2, 0, 0.000_56, 1),
    (0, 3, 0, -0.000_42, 0),
    (1, 0, 2, 0.000_42, 1),
    (1, 0, -2, 0.000_38, 1),
    (-1, 2, 0, -0.000_24, 1),
    (2, 1, 0, -0.000_07, 0),
    (0, 2, -2, 0.000_04, 0),
    (3, 0, 0, 0.000_04, 0),
    (1, 1, -2, 0.000_03, 0),
    (0, 2, 2, 0.000_03, 0),
    (1, 1, 2, -0.000_03, 0),
    (-1, 1, 2, 0.000_03, 0),
    (-1, 1, -2, -0.000_02, 0),
    (1, 3, 0, -0.000_02, 0),
    (0, 4, 0, 0.000_02, 0),
];

/// Meeus, chapter 49: corrections to the time of a full moon, in days.
const FULL_MOON_TERMS: [PhaseTerm; 24] = [
    (0, 1, 0, -0.406_14, 0),
    (1, 0, 0, 0.173_02, 1),
    (0, 2, 0, 0.016_14, 0),
    (0, 0, 2, 0.010_43, 0),
    (-1, 1, 0, 0.007_34, 1),
    (1, 1, 0, -0.005_15, 1),
    (2, 0, 0, 0.002_09, 2),
    (0, 1, -2, -0.001_11, 0),
    (0, 1, 2, -0.000_57, 0),
    (1, 2, 0, 0.000_56, 1),
    (0, 3, 0, -0.000_42, 0),
    (1, 0, 2, 0.000_42, 1),
    (1, 0, -2, 0.000_38, 1),
    (-1, 2, 0, -0.000_24, 1),
    (2, 1, 0, -0.000_07, 0),
    (0, 2, -2, 0.000_04, 0),
    (3, 0, 0, 0.000_04, 0),
    (1, 1, -2, 0.000_03, 0),
    (0, 2, 2, 0.000_03, 0),
    (1, 1, 2, -0.000_03, 0),
    (-1, 1, 2, 0.000_03, 0),
    (-1, 1, -2, -0.000_02, 0),
    (1, 3, 0, -0.000_02, 0),
    (0, 4, 0, 0.000_02, 0),
];

/// Meeus, chapter 49: corrections to the time of a quarter, in days. The
/// quarters additionally take the `W` term, with its sign depending on which
/// quarter is wanted.
const QUARTER_TERMS: [PhaseTerm; 24] = [
    (0, 1, 0, -0.628_01, 0),
    (1, 0, 0, 0.171_72, 1),
    (1, 1, 0, -0.011_83, 1),
    (0, 2, 0, 0.008_62, 0),
    (0, 0, 2, 0.008_04, 0),
    (-1, 1, 0, 0.004_54, 1),
    (2, 0, 0, 0.002_04, 2),
    (0, 1, -2, -0.001_80, 0),
    (0, 1, 2, -0.000_70, 0),
    (0, 3, 0, -0.000_40, 0),
    (-1, 2, 0, -0.000_34, 1),
    (1, 0, 2, 0.000_32, 1),
    (1, 0, -2, 0.000_32, 1),
    (2, 1, 0, -0.000_28, 2),
    (1, 2, 0, 0.000_27, 1),
    (-1, 1, -2, -0.000_05, 0),
    (0, 2, 2, 0.000_04, 0),
    (1, 1, 2, -0.000_04, 0),
    (-2, 1, 0, 0.000_04, 0),
    (1, 1, -2, 0.000_03, 0),
    (3, 0, 0, 0.000_03, 0),
    (0, 2, -2, 0.000_02, 0),
    (-1, 1, 2, 0.000_02, 0),
    (1, 3, 0, -0.000_02, 0),
];

/// Meeus, chapter 49: the fourteen planetary arguments, as
/// (constant, coefficient of `k`, coefficient of `T²`, amplitude in days).
const PLANETARY_ARGUMENTS: [(f64, f64, f64, f64); 14] = [
    (299.77, 0.107_408, -0.009_173, 0.000_325),
    (251.88, 0.016_321, 0.0, 0.000_165),
    (251.83, 26.651_886, 0.0, 0.000_164),
    (349.42, 36.412_478, 0.0, 0.000_126),
    (84.66, 18.206_239, 0.0, 0.000_110),
    (141.74, 53.303_771, 0.0, 0.000_062),
    (207.14, 2.453_732, 0.0, 0.000_060),
    (154.84, 7.306_860, 0.0, 0.000_056),
    (34.52, 27.261_239, 0.0, 0.000_047),
    (207.19, 0.121_824, 0.0, 0.000_042),
    (291.34, 1.844_379, 0.0, 0.000_040),
    (161.72, 24.198_154, 0.0, 0.000_037),
    (239.56, 25.513_099, 0.0, 0.000_035),
    (331.55, 3.592_518, 0.0, 0.000_023),
];

/// The moment of the `lunation`-th occurrence of a given phase, in Universal
/// Time.
///
/// Lunation 0 is the new moon of 2000 January 6; lunation −283 is the new
/// moon of 1977 February, which is Meeus's own worked example. The index is
/// a count of mean lunations, so the same integer names the new moon, the
/// first quarter, the full moon and the last quarter of one cycle.
///
/// Meeus states the series as good to a few seconds over the four centuries
/// around the present and to under a minute over four millennia; the
/// uncertainty in ΔT dominates well before the series does.
#[must_use]
pub fn nth_moon_phase(lunation: i64, phase: MoonPhase) -> Moment {
    let k = lunation as f64 + phase.lunation_offset();
    let t = k / 1_236.85;

    let mean_phase = poly(
        t,
        &[
            2_451_550.097_66,
            0.0,
            0.000_154_37,
            -0.000_000_150,
            0.000_000_000_73,
        ],
    ) + 29.530_588_861 * k;

    let eccentricity_factor = poly(t, &[1.0, -0.002_516, -0.000_007_4]);
    let solar_anomaly = poly(t, &[2.5534, 0.0, -0.000_001_4, -0.000_000_11]) + 29.105_356_70 * k;
    let lunar_anomaly = poly(
        t,
        &[201.5643, 0.0, 0.010_758_2, 0.000_012_38, -0.000_000_058],
    ) + 385.816_935_28 * k;
    let argument_of_latitude = poly(
        t,
        &[160.7108, 0.0, -0.001_611_8, -0.000_002_27, 0.000_000_011],
    ) + 390.670_502_84 * k;
    let ascending_node = poly(t, &[124.7746, 0.0, 0.002_067_2, 0.000_002_15]) - 1.563_755_88 * k;

    let terms: &[PhaseTerm] = match phase {
        MoonPhase::New => &NEW_MOON_TERMS,
        MoonPhase::Full => &FULL_MOON_TERMS,
        MoonPhase::FirstQuarter | MoonPhase::LastQuarter => &QUARTER_TERMS,
    };

    let mut correction = 0.0;
    for (m, m_prime, f, coefficient, power) in terms {
        let angle = f64::from(*m) * solar_anomaly
            + f64::from(*m_prime) * lunar_anomaly
            + f64::from(*f) * argument_of_latitude;
        let scale = match power {
            0 => 1.0,
            1 => eccentricity_factor,
            _ => eccentricity_factor * eccentricity_factor,
        };
        correction += coefficient * scale * sin_deg(angle);
    }
    // The node term is common to all four phases and is the one argument that
    // is not a combination of M, M' and F.
    correction += -0.000_17 * sin_deg(ascending_node);

    for (constant, per_lunation, quadratic, amplitude) in PLANETARY_ARGUMENTS {
        let angle = constant + per_lunation * k + quadratic * t * t;
        correction += amplitude * sin_deg(angle);
    }

    if matches!(phase, MoonPhase::FirstQuarter | MoonPhase::LastQuarter) {
        let w = 0.003_06 - 0.000_38 * eccentricity_factor * cos_deg(solar_anomaly)
            + 0.000_26 * cos_deg(lunar_anomaly)
            - 0.000_02 * cos_deg(lunar_anomaly - solar_anomaly)
            + 0.000_02 * cos_deg(lunar_anomaly + solar_anomaly)
            + 0.000_02 * cos_deg(2.0 * argument_of_latitude);
        correction += if matches!(phase, MoonPhase::FirstQuarter) {
            w
        } else {
            -w
        };
    }

    universal_time(Moment::from_julian_date(mean_phase + correction))
}

/// The moment of the `lunation`-th new moon, in Universal Time. Lunation 0 is
/// the new moon of 2000 January 6.
#[must_use]
pub fn nth_new_moon(lunation: i64) -> Moment {
    nth_moon_phase(lunation, MoonPhase::New)
}

/// The lunation index whose new moon is nearest to a moment, used only to
/// seed a search.
fn estimated_lunation(moment: Moment) -> i64 {
    let zeroth = 730_125.76; // The new moon of 2000-01-06, to within a minute.
    hc_core::math::floor((moment.0 - zeroth) / MEAN_SYNODIC_MONTH) as i64
}

/// The last new moon strictly before a moment.
#[must_use]
pub fn new_moon_before(moment: Moment) -> Moment {
    let mut lunation = estimated_lunation(moment) + 2;
    let mut candidate = nth_new_moon(lunation);
    // The seed is never more than a lunation out, so this walks at most a
    // handful of steps; the bound is there so that a nonsensical argument
    // cannot spin.
    for _ in 0..8 {
        if candidate.0 < moment.0 {
            break;
        }
        lunation -= 1;
        candidate = nth_new_moon(lunation);
    }
    candidate
}

/// The first new moon at or after a moment.
///
/// This is the conjunction search the Chinese and observational Hijri
/// calendars need: a lunar month begins on the day containing (or following) a new
/// moon, depending on the calendar's own rule and its own meridian.
#[must_use]
pub fn new_moon_at_or_after(moment: Moment) -> Moment {
    let mut lunation = estimated_lunation(moment) - 2;
    let mut candidate = nth_new_moon(lunation);
    for _ in 0..8 {
        if candidate.0 >= moment.0 {
            break;
        }
        lunation += 1;
        candidate = nth_new_moon(lunation);
    }
    candidate
}

/// The first moment at or after `moment` when the Moon's elongation from the
/// Sun reaches `phase_degrees`.
///
/// Unlike [`nth_new_moon`], this searches the position series directly, so it
/// answers for any phase angle — a first quarter, a full moon, or the 14°
/// elongation some crescent-visibility rules use.
#[must_use]
pub fn moon_phase_at_or_after(phase_degrees: f64, moment: Moment) -> Moment {
    let rate = MEAN_SYNODIC_MONTH / 360.0;
    let to_go = modulo(phase_degrees - lunar_phase(moment), 360.0);
    let estimate = moment.0 + rate * to_go;
    invert_angular(
        lunar_phase,
        normalize_degrees(phase_degrees),
        Moment(max_of(moment.0, estimate - 2.0)),
        Moment(estimate + 2.0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::{centuries_from_dynamical_julian_date, dynamical_time, gregorian_new_year};

    /// Meeus, example 47.a: 1992 April 12.0 TD.
    const EXAMPLE_47A_JDE: f64 = 2_448_724.5;

    fn example_47a_centuries() -> f64 {
        centuries_from_dynamical_julian_date(EXAMPLE_47A_JDE)
    }

    #[test]
    fn meeus_example_47a_has_the_stated_fundamental_arguments() {
        let arguments = lunar_arguments(example_47a_centuries());
        assert!((arguments.mean_longitude - 134.290_182).abs() < 1e-5);
        assert!((arguments.elongation - 113.842_304).abs() < 1e-5);
        assert!((arguments.solar_anomaly - 97.643_514).abs() < 1e-5);
        assert!((arguments.lunar_anomaly - 5.150_833).abs() < 1e-5);
        assert!((arguments.argument_of_latitude - 219.889_721).abs() < 1e-5);
        assert!((arguments.eccentricity_factor - 1.000_194).abs() < 1e-6);
        assert!((arguments.venus - 109.57).abs() < 0.01);
        assert!((arguments.jupiter - 123.78).abs() < 0.01);
        assert!((arguments.flattening - 229.53).abs() < 0.01);
    }

    /// Meeus prints the three periodic sums for example 47.a, which makes
    /// them a checksum on all 120 rows of tables 47.A and 47.B.
    #[test]
    fn the_periodic_sums_match_meeus_example_47a() {
        let arguments = lunar_arguments(example_47a_centuries());
        let (longitude, latitude, distance) = lunar_sums(&arguments);
        assert!(
            (longitude - -1_127_527.0).abs() < 2.0,
            "sigma l was {longitude}"
        );
        assert!(
            (latitude - -3_229_126.0).abs() < 2.0,
            "sigma b was {latitude}"
        );
        assert!(
            (distance - -16_590_875.0).abs() < 2.0,
            "sigma r was {distance}"
        );
    }

    /// Meeus, example 47.a: λ = 133.162655°, β = −3.229126°, Δ = 368409.7 km,
    /// π = 0.991990°, and an apparent longitude of 133.167265°.
    #[test]
    fn the_lunar_position_matches_meeus_example_47a() {
        let centuries = example_47a_centuries();
        let geometric = geometric_lunar_longitude_at_centuries(centuries);
        assert!((geometric - 133.162_655).abs() < 1e-5, "lambda {geometric}");
        let apparent = lunar_longitude_at_centuries(centuries);
        assert!((apparent - 133.167_265).abs() < 1e-3, "apparent {apparent}");
        let latitude = lunar_latitude_at_centuries(centuries);
        assert!((latitude + 3.229_126).abs() < 1e-5, "beta {latitude}");
        let distance = lunar_distance_at_centuries(centuries);
        assert!((distance - 368_409.7).abs() < 0.2, "distance {distance}");
    }

    #[test]
    fn the_lunar_parallax_matches_meeus_example_47a() {
        let moment = crate::time::universal_time(Moment::from_julian_date(EXAMPLE_47A_JDE));
        let parallax = lunar_parallax(moment);
        assert!((parallax - 0.991_990).abs() < 1e-5, "parallax {parallax}");
    }

    /// Meeus, example 48.a, same instant: the illuminated fraction is 0.6786.
    #[test]
    fn the_illuminated_fraction_matches_meeus_example_48a() {
        let moment = crate::time::universal_time(Moment::from_julian_date(EXAMPLE_47A_JDE));
        let fraction = lunar_illuminated_fraction(moment);
        assert!((fraction - 0.6786).abs() < 0.001, "fraction {fraction}");
    }

    /// Meeus, example 49.a: the new moon of 1977 February is lunation −283.
    /// The *mean* new moon is JDE 2443192.94102; the periodic terms take
    /// 0.28984 days off it, giving JDE 2443192.65118, which Meeus states as
    /// 1977 February 18 at 3h37m42s TD.
    #[test]
    fn the_new_moon_of_february_1977_matches_meeus_example_49a() {
        let moment = nth_new_moon(-283);
        let dynamical = dynamical_time(moment);
        let error_seconds = (dynamical.to_julian_date() - 2_443_192.651_18) * 86_400.0;
        assert!(
            error_seconds.abs() < 2.0,
            "off by {error_seconds} seconds (got {})",
            dynamical.to_julian_date()
        );
        // The stated clock time, 3h37m42s TD on 1977 February 18.
        let day_fraction = dynamical.0 - 721_768.0;
        let seconds_of_day = day_fraction * 86_400.0;
        assert!(
            (seconds_of_day - (3.0 * 3600.0 + 37.0 * 60.0 + 42.0)).abs() < 2.0,
            "clock time was {seconds_of_day} s into 1977-02-18"
        );
    }

    /// Meeus, example 49.b: the last quarter of 2044 January is lunation 544,
    /// at JDE 2467636.49186.
    #[test]
    fn the_last_quarter_of_january_2044_matches_meeus_example_49b() {
        let moment = nth_moon_phase(544, MoonPhase::LastQuarter);
        let dynamical = dynamical_time(moment);
        let error_seconds = (dynamical.to_julian_date() - 2_467_636.491_86) * 86_400.0;
        assert!(
            error_seconds.abs() < 2.0,
            "off by {error_seconds} seconds (got {})",
            dynamical.to_julian_date()
        );
    }

    /// The new moon of 2000 January 6 was at about 18:14 UT; it is lunation 0
    /// by construction of Meeus's index.
    #[test]
    fn lunation_zero_is_the_new_moon_of_january_2000() {
        let moment = nth_new_moon(0);
        assert_eq!(moment.day(), hc_calendar::Rd(730_125));
        let expected = 730_125.0 + (18.0 + 14.0 / 60.0) / 24.0;
        let error_minutes = (moment.0 - expected) * 24.0 * 60.0;
        assert!(
            error_minutes.abs() < 2.0,
            "off by {error_minutes} minutes (got {})",
            moment.0
        );
    }

    #[test]
    fn consecutive_new_moons_are_between_twenty_nine_and_thirty_days_apart() {
        // Over two millennia the synodic month swings between about 29.27 and
        // 29.83 days; anything outside that is a bug, not astronomy.
        let mut previous = nth_new_moon(-13_000);
        let mut shortest = f64::INFINITY;
        let mut longest: f64 = 0.0;
        for lunation in -12_999..12_000 {
            let current = nth_new_moon(lunation);
            let gap = current.0 - previous.0;
            assert!(
                (29.2..29.9).contains(&gap),
                "gap of {gap} days at lunation {lunation}"
            );
            if gap < shortest {
                shortest = gap;
            }
            if gap > longest {
                longest = gap;
            }
            previous = current;
        }
        assert!(shortest < 29.30, "shortest month was {shortest}");
        assert!(longest > 29.80, "longest month was {longest}");
    }

    #[test]
    fn the_mean_of_many_lunations_is_the_mean_synodic_month() {
        let span = nth_new_moon(12_000).0 - nth_new_moon(-12_000).0;
        let mean = span / 24_000.0;
        assert!(
            (mean - MEAN_SYNODIC_MONTH).abs() < 0.001,
            "mean synodic month came out {mean}"
        );
    }

    #[test]
    fn new_moons_bracket_any_moment_they_are_asked_about() {
        for step in 0..500 {
            let moment = Moment(700_000.0 + f64::from(step) * 13.7);
            let before = new_moon_before(moment);
            let after = new_moon_at_or_after(moment);
            assert!(before.0 < moment.0, "before was not before at {}", moment.0);
            assert!(after.0 >= moment.0, "after was not after at {}", moment.0);
            let gap = after.0 - before.0;
            assert!((29.2..29.9).contains(&gap), "bracket span {gap}");
        }
    }

    /// The phase series of chapter 49 and the position series of chapter 47
    /// are fitted independently. Their agreement is the strongest evidence
    /// this module has that both tables were transcribed correctly.
    #[test]
    fn the_two_independent_conjunction_algorithms_agree() {
        for lunation in [-2_000i64, -500, -37, 0, 1, 42, 300, 1_200] {
            let from_phase_series = nth_new_moon(lunation);
            let from_position_series =
                moon_phase_at_or_after(0.0, Moment(from_phase_series.0 - 5.0));
            let difference_seconds = (from_phase_series.0 - from_position_series.0) * 86_400.0;
            assert!(
                difference_seconds.abs() < 120.0,
                "lunation {lunation}: the two algorithms differ by {difference_seconds} seconds"
            );
        }
    }

    #[test]
    fn the_lunar_phase_is_zero_at_a_new_moon_and_half_a_turn_at_a_full_one() {
        for lunation in [-300i64, -1, 0, 17, 500] {
            let new = lunar_phase(nth_new_moon(lunation));
            assert!(
                !(0.05..=359.95).contains(&new),
                "phase at new moon was {new}"
            );
            let full = lunar_phase(nth_moon_phase(lunation, MoonPhase::Full));
            assert!((full - 180.0).abs() < 0.05, "phase at full moon was {full}");
            let first = lunar_phase(nth_moon_phase(lunation, MoonPhase::FirstQuarter));
            assert!(
                (first - 90.0).abs() < 0.05,
                "phase at first quarter {first}"
            );
            let last = lunar_phase(nth_moon_phase(lunation, MoonPhase::LastQuarter));
            assert!((last - 270.0).abs() < 0.05, "phase at last quarter {last}");
        }
    }

    #[test]
    fn the_moon_is_dark_at_conjunction_and_full_at_opposition() {
        for lunation in [-100i64, 0, 60] {
            let new = lunar_illuminated_fraction(nth_new_moon(lunation));
            assert!(new < 0.01, "illumination at new moon was {new}");
            let full = lunar_illuminated_fraction(nth_moon_phase(lunation, MoonPhase::Full));
            assert!(full > 0.99, "illumination at full moon was {full}");
            let quarter =
                lunar_illuminated_fraction(nth_moon_phase(lunation, MoonPhase::FirstQuarter));
            assert!(
                (quarter - 0.5).abs() < 0.01,
                "illumination at first quarter was {quarter}"
            );
        }
    }

    #[test]
    fn the_illuminated_fraction_never_leaves_the_unit_interval() {
        for step in 0..3_000 {
            let moment = Moment(730_000.0 + f64::from(step) * 0.37);
            let fraction = lunar_illuminated_fraction(moment);
            assert!(
                (0.0..=1.0).contains(&fraction),
                "fraction {fraction} at {}",
                moment.0
            );
        }
    }

    #[test]
    fn the_moon_moves_about_thirteen_degrees_a_day() {
        let start = gregorian_new_year(2024).0 as f64;
        for step in 0..120 {
            let a = lunar_longitude(Moment(start + f64::from(step)));
            let b = lunar_longitude(Moment(start + f64::from(step) + 1.0));
            let advance = modulo(b - a, 360.0);
            assert!(
                (11.5..15.5).contains(&advance),
                "the Moon advanced {advance} degrees on day {step}"
            );
        }
    }

    #[test]
    fn the_moon_stays_within_six_degrees_of_the_ecliptic() {
        // The inclination of the lunar orbit is 5.145 degrees, and the
        // series' own wobble adds a fraction of a degree.
        for step in 0..4_000 {
            let latitude = lunar_latitude(Moment(720_000.0 + f64::from(step) * 2.3));
            assert!(latitude.abs() < 5.4, "latitude {latitude}");
        }
    }

    #[test]
    fn the_lunar_distance_stays_between_perigee_and_apogee() {
        for step in 0..4_000 {
            let distance = lunar_distance(Moment(720_000.0 + f64::from(step) * 2.3));
            assert!(
                (355_000.0..407_000.0).contains(&distance),
                "distance {distance} km"
            );
        }
    }

    #[test]
    fn moon_phase_at_or_after_lands_on_its_target() {
        let start = Moment(738_000.0);
        for target_step in 0..24 {
            let target = f64::from(target_step) * 15.0;
            let found = moon_phase_at_or_after(target, start);
            let error = crate::util::signed_degrees(lunar_phase(found) - target);
            assert!(
                error.abs() < 1e-4,
                "target {target}: off by {error} degrees"
            );
            assert!(found.0 >= start.0);
            assert!(found.0 <= start.0 + MEAN_SYNODIC_MONTH + 1.0);
        }
    }

    #[test]
    fn the_four_phases_of_a_lunation_come_in_order() {
        for lunation in [-500i64, 0, 250] {
            let new = nth_new_moon(lunation).0;
            let first = nth_moon_phase(lunation, MoonPhase::FirstQuarter).0;
            let full = nth_moon_phase(lunation, MoonPhase::Full).0;
            let last = nth_moon_phase(lunation, MoonPhase::LastQuarter).0;
            let next = nth_new_moon(lunation + 1).0;
            assert!(new < first && first < full && full < last && last < next);
            assert!((first - new - 7.4).abs() < 1.0);
            assert!((full - new - 14.8).abs() < 1.0);
        }
    }

    #[test]
    fn the_phase_enum_names_the_right_elongations_and_offsets() {
        assert!((MoonPhase::New.elongation_degrees() - 0.0).abs() < 1e-12);
        assert!((MoonPhase::FirstQuarter.elongation_degrees() - 90.0).abs() < 1e-12);
        assert!((MoonPhase::Full.elongation_degrees() - 180.0).abs() < 1e-12);
        assert!((MoonPhase::LastQuarter.elongation_degrees() - 270.0).abs() < 1e-12);
        assert!((MoonPhase::LastQuarter.lunation_offset() - 0.75).abs() < 1e-12);
    }

    #[test]
    fn the_lunar_position_is_near_the_solar_one_at_a_new_moon() {
        let moment = nth_new_moon(300);
        let moon = lunar_position(moment);
        let sun = crate::solar::solar_position(moment);
        let separation =
            crate::util::signed_degrees(moon.right_ascension_degrees - sun.right_ascension_degrees);
        assert!(separation.abs() < 1.0, "separation {separation} degrees");
    }
}
