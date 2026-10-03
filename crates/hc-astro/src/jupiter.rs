//! Where Jupiter is as seen from the Earth, from VSOP87. Behind the
//! `jupiter` feature.
//!
//! The twelve-year festivals of India are set by Jupiter's sign, and a sign
//! is a question about a *longitude*, so this module answers one: the
//! geocentric ecliptic longitude of Jupiter, apparent, in the true equinox of
//! the date, and the sidereal longitude `hc-seasons` makes of it. The rest of
//! what comes out on the way (latitude, distance, the heliocentric position)
//! is returned because it was computed.
//!
//! # How
//!
//! 1. Jupiter's heliocentric position at the moment the light left it, from
//!    VSOP87B ([`crate::vsop87_jupiter`], the ecliptic and equinox of
//!    J2000.0), turned to the mean ecliptic of the date by the rotation of
//!    Meeus (21.5)–(21.7) in IAU 1976's constants, which is the precession
//!    VSOP87D's Earth is stated in.
//! 2. The Earth's heliocentric position at the moment, from the series
//!    [`crate::vsop87`] already carries for the Sun (VSOP87D, the ecliptic
//!    and equinox of date).
//! 3. Their difference, the geocentric vector; the light-time is its length
//!    over the speed of light, and the first step is run again for it until
//!    it stops changing (two passes).
//! 4. Annual aberration, by the classical vector sum with the Earth's
//!    velocity, which is the difference of the Earth's position a minute
//!    either side of the moment, taken in the J2000.0 frame so that the
//!    precession of the frame of date is not mistaken for motion.
//! 5. The step from VSOP87's dynamical ecliptic to the FK5 frame, Meeus
//!    chapter 32: a constant 0.09033″ and a periodic 0.03916″ in latitude,
//!    as [`crate::solar`] applies to the Sun.
//! 6. Nutation in longitude, [`crate::earth::nutation_at_centuries`], to
//!    pass from the mean equinox of the date to the true.
//!
//! This is what an almanac calls the apparent geocentric position, less the
//! gravitational deflection of light (under a milliarcsecond at Jupiter's
//! distance from the Sun except within a few degrees of it) and with the
//! 4-term nutation this crate carries, good to about half an arcsecond.
//!
//! # Accuracy
//!
//! VSOP87 gives Jupiter to 1″ over 2 000 years either side of J2000.0; the
//! tests measure this implementation against JPL's DE441 through Horizons,
//! in the heliocentric J2000 position at twelve dates from 1001 BCE to 3000
//! and in the apparent geocentric longitude on named dates, and
//! `docs/systems/jupiter-ephemeris.md` §Accuracy states what they found.
//! Jupiter moves
//! 0.08° a day on average and stands still at its stations, so an error of
//! 1″ in longitude is a few minutes at the average speed and a few days at
//! a station.

use hc_calendar::fixed::Moment;
use hc_core::math::{
    DEG_TO_RAD, RAD_TO_DEG, asin, atan2, cos, normalize_degrees, signed_degrees, sin, sqrt,
};

use hc_core::duration::SECONDS_PER_DAY_F64;

use crate::earth::nutation_at_centuries;
use crate::hjd::LIGHT_TIME_PER_AU_SECONDS;
use crate::solar::fk5_from_dynamical;
use crate::time::{julian_centuries, julian_centuries_from_dynamical};
use crate::vsop87::earth_heliocentric;
use crate::vsop87_jupiter::{Truncation, jupiter_heliocentric};

pub use crate::vsop87_jupiter::{SOURCE_SHA256, SOURCE_TERMS};

/// The truncation the position functions that take none use: the terms of
/// amplitude 10⁻⁸ and over, which is 985 of the 3 625 at J2000.0 and about
/// 1 400 at the ends of the era, and which differs from the full series by
/// at most 0.093″ in longitude, 0.046″ in latitude and 5.0 × 10⁻⁷ AU in
/// distance, the largest over 60 000 dates from 1000 BCE to 3000 CE. A position
/// takes some 15 microseconds against 37 with every term, in a release build,
/// and a search for an ingress makes a few hundred.
pub const DEFAULT_TRUNCATION: Truncation = Truncation::Amplitude(1e-8);

/// Light takes this many days to cross an astronomical unit: 499.004 783 8 s,
/// from the speed of light, 299 792 458 m/s, and the astronomical unit,
/// 149 597 870 700 m, as [`crate::hjd::LIGHT_TIME_PER_AU_SECONDS`] has them.
const LIGHT_DAYS_PER_AU: f64 = LIGHT_TIME_PER_AU_SECONDS / SECONDS_PER_DAY_F64;

/// One second of arc in degrees.
const ARCSECOND: f64 = 1.0 / 3600.0;

/// How many times the light-time is found again from the last position: the
/// second changes the longitude by under 0.003″, since Jupiter moves under
/// 0.25° a day and the light-time changes by under 3 × 10⁻⁶ day.
const LIGHT_TIME_PASSES: usize = 2;

/// How far either side of the moment the Earth's velocity is taken, in
/// days.
const VELOCITY_STEP_DAYS: f64 = 0.0005;

/// A vector in an ecliptic frame, in astronomical units.
type Vector = [f64; 3];

/// Jupiter as seen from the Earth at one moment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Apparent {
    /// The apparent geocentric ecliptic longitude, in degrees from 0 up to
    /// but not including 360, in the true equinox of the date: the position
    /// light-time, aberration and nutation have moved.
    pub longitude_degrees: f64,
    /// The same longitude in the *mean* equinox of the date, nutation not
    /// applied: the one the ayanāṃśa is measured from.
    pub mean_equinox_longitude_degrees: f64,
    /// The apparent geocentric ecliptic latitude, in degrees.
    pub latitude_degrees: f64,
    /// The geocentric distance in astronomical units, at the moment the
    /// light left Jupiter.
    pub distance_au: f64,
    /// The light-time in days.
    pub light_time_days: f64,
    /// Jupiter's heliocentric longitude in degrees, geometric, in the mean
    /// ecliptic and equinox of the date, at the moment the light left it.
    pub heliocentric_longitude_degrees: f64,
    /// Jupiter's heliocentric latitude in degrees, in the same frame.
    pub heliocentric_latitude_degrees: f64,
    /// Jupiter's distance from the Sun in astronomical units.
    pub heliocentric_distance_au: f64,
}

/// The rotation between the mean ecliptic of J2000.0 and the mean ecliptic
/// of a date: Meeus (21.5), the angles η, Π and *p* in radians.
#[derive(Debug, Clone, Copy)]
struct Precession {
    eta: f64,
    node: f64,
    arc: f64,
}

impl Precession {
    /// The angles for `centuries` Julian centuries from J2000.0.
    fn at(centuries: f64) -> Self {
        let t = centuries;
        let eta = (47.0029 * t - 0.03302 * t * t + 0.000_060 * t * t * t) * ARCSECOND;
        let node = 174.876_384 + (-869.8089 * t + 0.035_36 * t * t) * ARCSECOND;
        let arc = (5029.0966 * t + 1.11113 * t * t - 0.000_006 * t * t * t) * ARCSECOND;
        Self {
            eta: eta * DEG_TO_RAD,
            node: node * DEG_TO_RAD,
            arc: arc * DEG_TO_RAD,
        }
    }

    /// A vector of the J2000.0 frame in the frame of the date.
    fn to_date(self, v: Vector) -> Vector {
        let (x, y, z) = turn(v, -self.node);
        let (x, y, z) = tilt([x, y, z], self.eta);
        let (x, y, z) = turn([x, y, z], self.node + self.arc);
        [x, y, z]
    }

    /// A vector of the frame of the date in the J2000.0 frame.
    fn to_j2000(self, v: Vector) -> Vector {
        let (x, y, z) = turn(v, -(self.node + self.arc));
        let (x, y, z) = tilt([x, y, z], -self.eta);
        let (x, y, z) = turn([x, y, z], self.node);
        [x, y, z]
    }
}

/// A vector turned about the pole by `angle`: its longitude grows by it.
fn turn(v: Vector, angle: f64) -> (f64, f64, f64) {
    let (s, c) = (sin(angle), cos(angle));
    (v[0] * c - v[1] * s, v[0] * s + v[1] * c, v[2])
}

/// A vector turned about the first axis, so that the pole moves by `eta`.
fn tilt(v: Vector, eta: f64) -> (f64, f64, f64) {
    let (s, c) = (sin(eta), cos(eta));
    (v[0], c * v[1] + s * v[2], -s * v[1] + c * v[2])
}

/// A vector from longitude, latitude and distance.
fn rectangular(longitude: f64, latitude: f64, radius: f64) -> Vector {
    let flat = radius * cos(latitude);
    [
        flat * cos(longitude),
        flat * sin(longitude),
        radius * sin(latitude),
    ]
}

/// Longitude and latitude in radians, and the length, of a vector.
fn spherical(v: Vector) -> (f64, f64, f64) {
    let radius = sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    let latitude = if radius > 0.0 {
        asin((v[2] / radius).clamp(-1.0, 1.0))
    } else {
        0.0
    };
    (atan2(v[1], v[0]), latitude, radius)
}

/// The Earth's heliocentric position in the mean ecliptic of the date, at
/// `centuries`, as a vector.
fn earth_of_date(centuries: f64) -> Vector {
    let (longitude, latitude, radius) = earth_heliocentric(centuries / 10.0);
    rectangular(longitude, latitude, radius)
}

/// Jupiter's heliocentric position at `centuries`, turned to the frame of
/// the date `frame`.
fn jupiter_in(centuries: f64, frame: Precession, truncation: Truncation) -> Vector {
    let (longitude, latitude, radius) = jupiter_heliocentric(centuries / 10.0, truncation);
    frame.to_date(rectangular(longitude, latitude, radius))
}

/// The Earth's velocity at `centuries`, in astronomical units a day, in the
/// frame of the date.
fn earth_velocity(centuries: f64, frame: Precession) -> Vector {
    let step = VELOCITY_STEP_DAYS / 36_525.0;
    let before = Precession::at(centuries - step).to_j2000(earth_of_date(centuries - step));
    let after = Precession::at(centuries + step).to_j2000(earth_of_date(centuries + step));
    frame.to_date([
        (after[0] - before[0]) / (2.0 * VELOCITY_STEP_DAYS),
        (after[1] - before[1]) / (2.0 * VELOCITY_STEP_DAYS),
        (after[2] - before[2]) / (2.0 * VELOCITY_STEP_DAYS),
    ])
}

/// Jupiter as seen from the Earth at `centuries` Julian centuries of
/// dynamical time from J2000.0, with the given truncation of the series.
///
/// The argument is TT. [`apparent`] is the same for a Universal Time moment.
#[must_use]
pub fn apparent_at_centuries(centuries: f64, truncation: Truncation) -> Apparent {
    let frame = Precession::at(centuries);
    let earth = earth_of_date(centuries);
    let mut light_time = 0.0;
    let mut jupiter = jupiter_in(centuries, frame, truncation);
    let mut geocentric = sub(jupiter, earth);
    for _ in 0..LIGHT_TIME_PASSES {
        light_time = spherical(geocentric).2 * LIGHT_DAYS_PER_AU;
        jupiter = jupiter_in(centuries - light_time / 36_525.0, frame, truncation);
        geocentric = sub(jupiter, earth);
    }
    let distance = spherical(geocentric).2;
    // Annual aberration: the apparent direction leans toward where the Earth
    // is going by v/c, to first order.
    let velocity = earth_velocity(centuries, frame);
    let speed_of_light = 1.0 / LIGHT_DAYS_PER_AU;
    let direction = [
        geocentric[0] / distance + velocity[0] / speed_of_light,
        geocentric[1] / distance + velocity[1] / speed_of_light,
        geocentric[2] / distance + velocity[2] / speed_of_light,
    ];
    let (longitude, latitude, _) = spherical(direction);
    let mut longitude = longitude * RAD_TO_DEG;
    let mut latitude = latitude * RAD_TO_DEG;
    // From the dynamical ecliptic of VSOP87 to the FK5 frame (Meeus 32.3),
    // the function the Sun's position passes through in `solar`.
    (longitude, latitude) = fk5_from_dynamical(longitude, latitude, centuries);
    let mean = normalize_degrees(longitude);
    let nutation = nutation_at_centuries(centuries).longitude_degrees;
    let (heliocentric_longitude, heliocentric_latitude, heliocentric_distance) = spherical(jupiter);
    Apparent {
        longitude_degrees: normalize_degrees(mean + nutation),
        mean_equinox_longitude_degrees: mean,
        latitude_degrees: latitude,
        distance_au: distance,
        light_time_days: light_time,
        heliocentric_longitude_degrees: normalize_degrees(heliocentric_longitude * RAD_TO_DEG),
        heliocentric_latitude_degrees: heliocentric_latitude * RAD_TO_DEG,
        heliocentric_distance_au: heliocentric_distance,
    }
}

/// `a` minus `b`.
fn sub(a: Vector, b: Vector) -> Vector {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Jupiter as seen from the Earth at a Universal Time moment.
#[must_use]
pub fn apparent(moment: Moment, truncation: Truncation) -> Apparent {
    apparent_at_centuries(julian_centuries(moment), truncation)
}

/// Jupiter's apparent geocentric longitude in degrees at a Universal Time
/// moment, in the true equinox of the date, at the default truncation.
#[must_use]
pub fn longitude(moment: Moment) -> f64 {
    apparent(moment, DEFAULT_TRUNCATION).longitude_degrees
}

/// Jupiter's apparent geocentric longitude's rate of change in degrees a day
/// at a Universal Time moment, by the difference of the longitude half a day
/// either side: positive when Jupiter moves forward through the signs,
/// negative when it is in retrograde.
#[must_use]
pub fn daily_motion_degrees(moment: Moment) -> f64 {
    let before = apparent(Moment(moment.0 - 0.5), DEFAULT_TRUNCATION).longitude_degrees;
    let after = apparent(Moment(moment.0 + 0.5), DEFAULT_TRUNCATION).longitude_degrees;
    signed_degrees(after - before)
}

/// The same for a moment given in dynamical time.
#[must_use]
pub fn longitude_at_dynamical(moment: Moment) -> f64 {
    apparent_at_centuries(julian_centuries_from_dynamical(moment), DEFAULT_TRUNCATION)
        .longitude_degrees
}

// The constants are Horizons' digits as it prints them.
#[cfg(test)]
#[allow(clippy::excessive_precision)]
mod tests {
    use super::*;

    /// One astronomical unit in kilometres, as Horizons prints vectors in km.
    const AU_KM: f64 = 149_597_870.7;

    /// JPL Horizons (`jpl-horizons`), DE441, Jupiter system barycenter (5)
    /// relative to the Sun's centre (500@10), geometric Cartesian position
    /// in the ecliptic and equinox of J2000.0 (ICRF frame), at twelve
    /// dates in TDB, as Julian Dates, with the position in kilometres.
    /// Retrieved 2026-10-03.
    const HORIZONS_VECTORS: [(f64, [f64; 3]); 12] = [
        (
            1_355_818.5,
            [
                2.116_975_575_167_689E8,
                7.329_546_150_514_607E8,
                -6.990_201_538_383_961E6,
            ],
        ),
        (
            1_721_423.5,
            [
                -7.270_361_946_627_020E8,
                -3.618_573_478_284_532E8,
                1.815_594_840_827_723E7,
            ],
        ),
        (
            2_086_302.5,
            [
                1.454_416_906_105_162E8,
                -7.638_792_298_106_785E8,
                -6.345_151_850_483_418E5,
            ],
        ),
        (
            2_268_932.5,
            [
                6.918_577_010_104_604E8,
                -2.835_989_629_805_431E8,
                -1.455_057_536_852_300E7,
            ],
        ),
        (
            2_415_020.5,
            [
                -4.511_932_233_923_504E8,
                -6.672_354_805_364_699E8,
                1.283_615_917_706_537E7,
            ],
        ),
        (
            2_433_282.5,
            [
                5.096_210_888_498_111E8,
                -5.625_672_918_791_175E8,
                -9.109_228_403_226_405E6,
            ],
        ),
        (
            2_451_545.0,
            [
                5.985_675_835_979_289E8,
                4.396_047_284_920_244E8,
                -1.522_686_065_301_856E7,
            ],
        ),
        (
            2_460_676.5,
            [
                1.579_803_695_514_827E8,
                7.437_186_838_480_072E8,
                -6.623_903_345_367_759E6,
            ],
        ),
        (
            2_469_807.5,
            [
                -3.576_954_419_468_434E8,
                6.977_355_468_360_499E8,
                5.081_187_752_887_189E6,
            ],
        ),
        (
            2_488_069.5,
            [
                -8.039_031_471_764_840E8,
                -1.359_068_506_911_921E8,
                1.853_333_936_700_456E7,
            ],
        ),
        (
            2_634_166.5,
            [
                -5.623_403_106_616_534E7,
                7.671_251_456_317_476E8,
                -2.164_914_333_667_576E6,
            ],
        ),
        (
            2_816_787.5,
            [
                -6.739_560_709_710_881E8,
                4.355_733_304_556_929E8,
                1.270_406_516_307_074E7,
            ],
        ),
    ];

    /// The largest disagreement of the heliocentric position, in arcseconds
    /// of longitude and latitude, from the series against Horizons, by the
    /// year: the series' own error, which grows away from J2000.0.
    /// 1001 BCE: 8.3″; year 1: 2.1″; 999: 0.9″; 1500: 0.3″; 1900 to 2100:
    /// under 0.5″; 2500: 0.5″; 3000: 0.7″.
    #[test]
    fn the_heliocentric_position_agrees_with_horizons_as_vsop87_says() {
        let limits = [8.5, 2.2, 1.0, 0.4, 0.2, 0.2, 0.3, 0.3, 0.4, 0.5, 0.6, 0.8];
        for ((julian_date, km), limit) in HORIZONS_VECTORS.iter().zip(limits) {
            let tau = (julian_date - crate::time::J2000_JULIAN_DATE) / 365_250.0;
            let (l, b, r) = jupiter_heliocentric(tau, Truncation::Full);
            let [x, y, z] = km.map(|k| k / AU_KM);
            let (lh, bh, rh) = spherical([x, y, z]);
            let dl = signed_degrees((l - lh) * RAD_TO_DEG) * 3600.0;
            let db = (b - bh) * RAD_TO_DEG * 3600.0;
            assert!(
                dl.abs() < limit && db.abs() < limit,
                "JD {julian_date}: dL {dl}″ dB {db}″"
            );
            // 3 000 km in 1001 BCE, under 400 km from 1500 to 3000.
            let limit_km = if tau < -2.0 { 3_500.0 } else { 400.0 };
            assert!(
                (r - rh).abs() * AU_KM < limit_km,
                "JD {julian_date}: dR {} km",
                (r - rh) * AU_KM
            );
        }
    }
    /// JPL Horizons, DE441, Jupiter barycenter (5) from the Earth's centre
    /// (500@399), quantities 20 and 31 with `EXTRA_PREC`: the apparent range
    /// "delta" in astronomical units and the "IAU76/80 ecliptic-of-date
    /// longitude and latitude of the target centers' apparent position, with
    /// light-time, gravitational deflection of light, and stellar
    /// aberrations", in degrees. At twelve dates in TT, as Julian Dates.
    /// Retrieved 2026-10-03.
    const HORIZONS_TT: [(f64, f64, f64, f64); 12] = [
        (
            1_355_818.5,
            4.853_667_698_057_51,
            21.208_641_2,
            -0.949_908_2,
        ),
        (
            1_721_423.5,
            5.337_137_461_161_47,
            189.107_171_7,
            1.423_909_6,
        ),
        (
            2_086_302.5,
            6.157_241_494_574_05,
            269.022_355_0,
            0.068_513_2,
        ),
        (
            2_268_932.5,
            5.774_821_757_302_01,
            324.266_538_2,
            -0.947_239_9,
        ),
        (
            2_415_020.5,
            6.113_063_472_505_69,
            241.135_882_5,
            0.814_290_2,
        ),
        (
            2_433_282.5,
            5.935_365_588_760_70,
            306.505_259_3,
            -0.583_997_1,
        ),
        (
            2_451_545.0,
            4.621_163_601_788_51,
            25.253_043_3,
            -1.262_190_6,
        ),
        (
            2_460_676.5,
            4.190_743_700_288_54,
            73.215_532_6,
            -0.601_569_8,
        ),
        (
            2_469_807.5,
            4.311_138_569_118_12,
            121.691_644_1,
            0.458_068_7,
        ),
        (
            2_488_069.5,
            5.545_856_129_794_35,
            201.206_243_0,
            1.276_602_4,
        ),
        (
            2_634_166.5,
            4.157_925_545_763_57,
            101.346_612_6,
            -0.120_377_1,
        ),
        (
            2_816_787.5,
            4.961_981_041_933_63,
            171.123_492_0,
            1.040_493_2,
        ),
    ];

    /// The same at eleven dates at 0 h UT (UTC after 1972): the dates of
    /// some of Drik Panchang's entries, the opposition of 2024 and J2000's
    /// neighbours.
    const HORIZONS_UT: [(f64, f64, f64, f64); 11] = [
        (
            2_448_774.5,
            5.408_291_137_816_23,
            156.063_966_0,
            1.198_649_5,
        ),
        (
            2_451_726.5,
            5.742_400_163_515_72,
            60.142_742_3,
            -0.840_552_1,
        ),
        (
            2_455_536.5,
            4.680_218_276_633_21,
            354.005_306_7,
            -1.373_208_9,
        ),
        (
            2_458_595.5,
            4.644_526_290_681_16,
            264.151_638_3,
            0.634_064_4,
        ),
        (
            2_459_173.5,
            5.597_155_881_743_01,
            294.085_108_1,
            -0.457_648_7,
        ),
        (
            2_460_651.5,
            4.089_415_184_615_56,
            76.375_153_3,
            -0.671_804_1,
        ),
        (
            2_460_809.5,
            5.971_927_444_008_44,
            84.059_637_7,
            -0.211_816_4,
        ),
        (
            2_460_966.5,
            5.075_709_852_714_84,
            114.172_531_3,
            0.054_703_3,
        ),
        (
            2_461_582.5,
            5.979_989_363_366_71,
            144.245_736_5,
            0.921_999_6,
        ),
        (
            2_461_829.5,
            4.471_002_374_889_77,
            174.324_774_7,
            1.494_513_3,
        ),
        (
            2_462_526.5,
            5.661_816_066_452_00,
            234.301_896_3,
            1.037_577_5,
        ),
    ];

    /// The apparent position against Horizons, the full series and the default
    /// cut. The residual is the series': 11.5″ in 1001 BCE, 2.0″ in the year 1,
    /// 0.9″ in 999 and 0.6″ in 1500, and 0.5″ or less from 1900 to 2500; the
    /// latitude 0.15″ from 1500; the distance a few hundred kilometres.
    #[test]
    fn the_apparent_position_agrees_with_horizons() {
        let longitude_limits = [12.0, 2.2, 1.0, 0.7, 0.1, 0.1, 0.2, 0.4, 0.5, 0.5, 0.7, 0.9];
        for truncation in [Truncation::Full, DEFAULT_TRUNCATION] {
            for (row, limit) in HORIZONS_TT.iter().zip(longitude_limits) {
                let (julian_date, distance, longitude, latitude) = *row;
                let at = apparent_at_centuries(
                    (julian_date - crate::time::J2000_JULIAN_DATE) / 36_525.0,
                    truncation,
                );
                let dl = signed_degrees(at.longitude_degrees - longitude) * 3600.0;
                let db = (at.latitude_degrees - latitude) * 3600.0;
                let dr = (at.distance_au - distance) * AU_KM;
                let ok = dl.abs() < limit && db.abs() < limit.min(7.0) && dr.abs() < 11_000.0;
                assert!(ok, "JD {julian_date} TT: {dl}″ {db}″ {dr} km");
                if julian_date > 2_268_000.0 {
                    assert!(
                        db.abs() < 0.15 && dr.abs() < 400.0,
                        "JD {julian_date} TT: {db}″ {dr} km"
                    );
                }
            }
            for (julian_date, distance, longitude, latitude) in HORIZONS_UT {
                let at = apparent(Moment::from_julian_date(julian_date), truncation);
                let dl = signed_degrees(at.longitude_degrees - longitude) * 3600.0;
                let db = (at.latitude_degrees - latitude) * 3600.0;
                let dr = (at.distance_au - distance) * AU_KM;
                assert!(
                    dl.abs() < 0.45 && db.abs() < 0.1 && dr.abs() < 300.0,
                    "JD {julian_date} UT: {dl}″ {db}″ {dr} km"
                );
            }
        }
    }

    /// The default cut stays within 0.1″ of the full series in longitude,
    /// 0.05″ in latitude and 6 × 10⁻⁷ AU in distance (the largest over
    /// 60 000 dates from 1000 BCE to 3000 CE is 0.093″, 0.046″ and
    /// 5.0 × 10⁻⁷ AU), as the docs say, and keeps about a quarter to two
    /// fifths of the terms.
    #[test]
    fn the_default_cut_is_close_to_the_full_series() {
        for i in 0..400 {
            let centuries = -10.0 + 40.0 * (f64::from(i) + 0.37) / 400.0;
            let full = apparent_at_centuries(centuries, Truncation::Full);
            let cut = apparent_at_centuries(centuries, DEFAULT_TRUNCATION);
            let dl = signed_degrees(full.longitude_degrees - cut.longitude_degrees) * 3600.0;
            assert!(dl.abs() < 0.1, "{centuries}: {dl}″");
            assert!((full.latitude_degrees - cut.latitude_degrees).abs() * 3600.0 < 0.05);
            assert!((full.distance_au - cut.distance_au).abs() < 6e-7);
        }
        use crate::vsop87_jupiter::terms_kept;
        assert_eq!(terms_kept(0.0, DEFAULT_TRUNCATION), 985);
        assert!((1_300..1_500).contains(&terms_kept(0.3, DEFAULT_TRUNCATION)));
    }

    /// The rotation between the two ecliptics is its own inverse.
    #[test]
    fn the_precession_rotation_inverts() {
        for centuries in [-20.0, -1.0, 0.0, 0.25, 3.0] {
            let frame = Precession::at(centuries);
            let v = [1.2, -4.0, 0.3];
            let back = frame.to_j2000(frame.to_date(v));
            assert!(
                back.iter().zip(v).all(|(a, b)| (a - b).abs() < 1e-14),
                "{back:?}"
            );
        }
    }

    /// Jupiter is in retrograde near opposition and moving forward at
    /// conjunction, and always within a quarter of a degree a day.
    #[test]
    fn jupiter_is_retrograde_at_opposition() {
        // The opposition of 2024-12-07 (Horizons' longitude 76.375° that day,
        // the Sun at 255°).
        let opposition = Moment::from_julian_date(2_460_651.5);
        assert!(daily_motion_degrees(opposition) < -0.1);
        let conjunction = Moment::from_julian_date(2_460_651.5 + 199.0);
        assert!(daily_motion_degrees(conjunction) > 0.1);
    }
}
