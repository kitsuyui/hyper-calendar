//! The Unified Hijri Calendar of the Istanbul congress of 2016, as two
//! bodies apply it: Muhammadiyah's *Kalender Hijriah Global Tunggal*
//! (KHGT), `islamic-khgt`, and Türkiye's Diyanet, `islamic-istanbul-2016`.
//!
//! Both treat the whole Earth as one place of sighting (*ittiḥād
//! al-maṭāliʿ*) and begin a month on the same civil day everywhere, the
//! day being counted from the International Date Line. Both take a month to
//! begin on the day after an evening on which, at sunset somewhere, the
//! Moon is at least 8° from the Sun and at least 5° above the horizon. They
//! word the rest differently, and compute the altitude differently, and in
//! Ramaḍān 1447 they began the month a day apart; `docs/policy.md` §5 gives
//! each its own name. `docs/systems/unified-hijri.md` sets out the rules,
//! the sources, a worked example and how the two were measured against the
//! bodies' published calendars.
//!
//! * [`GlobalRule::KHGT`]: the month begins if the parameters are met
//!   anywhere on Earth at a sunset before 24:00 UT, geocentric elongation
//!   and geocentric altitude; or, if they are met only after 24:00 UT, when
//!   they are met on the mainland of the Americas and the conjunction fell
//!   before dawn in New Zealand (`muhammadiyah-ughc-2025`, §C.3;
//!   `muhammadiyah-khgt-site`).
//! * [`GlobalRule::ISTANBUL_2016`]: the month begins if the parameters are
//!   met anywhere on Earth at a sunset before 24:00 UT, or at a sunset on
//!   the mainland of South or North America with the conjunction before
//!   *imsāk* at Wellington (`diyanet-ramazan-1447`), with the altitude
//!   topocentric (`djamaluddin-khgt-turki-2025`). The statement gives the
//!   "anywhere on Earth" condition without an hour; the bound of 24:00 UT
//!   is this library's reading of it, set out in the system document.
//!
//! # What is modelled
//!
//! "Somewhere on Earth before 24:00 UT" is judged on the evening terminator
//! at 24:00 UT, where the Sun is setting at that instant, at every quarter
//! degree of latitude: at a given latitude the Moon at sunset is older, and
//! so farther from the Sun and higher, the later the sunset, so the last
//! sunsets before midnight are the ones to look at. "The mainland of the
//! Americas" is judged the same way along its western edge, a line through
//! 67 coastal places from Point Barrow to Cape Froward whose coordinates
//! are Wikipedia's ([`AMERICAS_WEST_COAST`]): at a given latitude the
//! westernmost point of the mainland sees the latest sunset of the day.
//! Dawn in New Zealand is astronomical dawn at Wellington, the Sun 18°
//! below the horizon; neither body's text read gives the angle. Sunset is
//! the Sun's upper limb on the sea-level horizon, its centre 50′ below.
//!
//! # Range
//!
//! 1900–2100, as the observational prediction of
//! [`crate::islamic_observational`], whose month search and month count
//! these calendars share.

use hc_astro::earth::apparent_sidereal_time_iau1982;
use hc_astro::lunar::{lunar_parallax, lunar_position};
use hc_astro::riseset::sunrise_altitude_degrees;
use hc_astro::solar::solar_position;
use hc_astro::{Location, Twilight, dawn};
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_core::math::{RAD_TO_DEG, acos, asin, cos_deg, floor, sin_deg};

use crate::civil;
use crate::islamic_observational::{
    EARLIEST, Frame, LATEST, NewMonthRule, SunsetCriterion, VisibilityCriterion, compose_with,
    decompose_with,
};
use crate::tabular::{ERA, IslamicDate};

/// The identifier of Muhammadiyah's calendar.
pub const KHGT_ID: CalendarId = CalendarId("islamic-khgt");

/// The identifier of Diyanet's calendar.
pub const ISTANBUL_2016_ID: CalendarId = CalendarId("islamic-istanbul-2016");

/// A place on the western edge of the American mainland, by the name of
/// the Wikipedia article whose coordinates it carries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoastPoint {
    /// The title of the article.
    pub name: &'static str,
    /// Latitude in degrees, positive north.
    pub latitude_degrees: f64,
    /// Longitude in degrees, positive east.
    pub longitude_degrees: f64,
}

impl CoastPoint {
    /// A named point.
    #[must_use]
    pub const fn new(name: &'static str, latitude_degrees: f64, longitude_degrees: f64) -> Self {
        Self {
            name,
            latitude_degrees,
            longitude_degrees,
        }
    }
}

/// The western coast of mainland Alaska, north to south, from Point Barrow
/// to Cold Bay near the end of the Alaska Peninsula. Coordinates from the
/// Wikipedia articles of these titles, read through its GeoData interface
/// on 2026-09-27 (`wikipedia-coordinates-americas`).
const ALASKA_WEST_COAST: [CoastPoint; 21] = [
    CoastPoint::new("Point Barrow", 71.388_888_89, -156.479_166_67),
    CoastPoint::new("Wainwright, Alaska", 70.647_222_22, -160.016_111_11),
    CoastPoint::new("Point Lay, Alaska", 69.741_111_11, -162.865_555_56),
    CoastPoint::new("Cape Lisburne", 68.881_111_11, -166.21),
    CoastPoint::new("Point Hope, Alaska", 68.346_944_44, -166.763_055_56),
    CoastPoint::new("Kivalina, Alaska", 67.727_222_22, -164.539_166_67),
    CoastPoint::new("Shishmaref, Alaska", 66.255_555_56, -166.072_222_22),
    CoastPoint::new("Cape Prince of Wales", 65.64, -168.12),
    CoastPoint::new("Nome, Alaska", 64.503_888_89, -165.399_444_44),
    CoastPoint::new("Emmonak, Alaska", 62.777_222_22, -164.545),
    CoastPoint::new("Scammon Bay, Alaska", 61.842_489, -165.581_694),
    CoastPoint::new("Hooper Bay, Alaska", 61.528_888_89, -166.096_111_11),
    CoastPoint::new("Newtok, Alaska", 60.944_444_44, -164.644_166_67),
    CoastPoint::new("Quinhagak, Alaska", 59.753_333_33, -161.902_777_78),
    CoastPoint::new("Platinum, Alaska", 59.013_055_56, -161.816_388_89),
    CoastPoint::new("Cape Constantine", 58.393_055_56, -158.893_888_89),
    CoastPoint::new("Egegik, Alaska", 58.219_166_67, -157.358_055_56),
    CoastPoint::new("Pilot Point, Alaska", 57.560_277_78, -157.582_222_22),
    CoastPoint::new("Port Heiden, Alaska", 56.948_888_89, -158.655_833_33),
    CoastPoint::new("Nelson Lagoon, Alaska", 56.000_522, -161.203_561),
    CoastPoint::new("Cold Bay, Alaska", 55.209_038, -162.714_298),
];

/// The Pacific coast of the mainland from British Columbia to the Strait
/// of Magellan, north to south; the sources as [`ALASKA_WEST_COAST`].
const PACIFIC_COAST: [CoastPoint; 46] = [
    CoastPoint::new(
        "Prince Rupert, British Columbia",
        54.312_777_78,
        -130.325_277_78,
    ),
    CoastPoint::new("Cape Flattery", 48.385_961, -124.726_912),
    CoastPoint::new("Cape Blanco (Oregon)", 42.837_608_9, -124.563_999_7),
    CoastPoint::new("Cape Mendocino", 40.4401, -124.4095),
    CoastPoint::new("Point Arguello", 34.616_666_67, -120.6),
    CoastPoint::new("Punta Eugenia", 27.846_944_44, -115.081_666_67),
    CoastPoint::new("Cabo San Lucas", 22.8947, -109.9153),
    CoastPoint::new("Mazatlán", 23.216_666_67, -106.416_666_67),
    CoastPoint::new("Puerto Vallarta", 20.645_833_33, -105.222_222_22),
    CoastPoint::new("Lázaro Cárdenas, Michoacán", 17.956_111_11, -102.192_222_22),
    CoastPoint::new("Acapulco", 16.863_611_11, -99.8825),
    CoastPoint::new("Puerto Ángel", 15.666_666_67, -96.490_555_56),
    CoastPoint::new("Acajutla", 13.59, -89.833_611_11),
    CoastPoint::new("Corinto, Nicaragua", 12.483_333_33, -87.183_333_33),
    CoastPoint::new("Tamarindo, Costa Rica", 10.292_341_7, -85.798_175),
    CoastPoint::new("Puerto Armuelles", 8.283_333_33, -82.866_666_67),
    CoastPoint::new("Bahía Solano", 6.216_666_67, -77.4),
    CoastPoint::new(
        "Buenaventura, Valle del Cauca",
        3.877_222_22,
        -77.026_666_67,
    ),
    CoastPoint::new("Tumaco", 1.806_666_67, -78.764_722_22),
    CoastPoint::new("Esmeraldas, Ecuador", 0.966_666_67, -79.652_777_78),
    CoastPoint::new("Manta, Ecuador", -0.950_022_22, -80.7162),
    CoastPoint::new("Salinas, Ecuador", -2.2167, -80.950_066_67),
    CoastPoint::new("Talara", -4.579_902_78, -81.271_877_78),
    CoastPoint::new("Punta Pariñas", -4.679_166_67, -81.326_388_89),
    CoastPoint::new("Paita", -5.091_111_11, -81.106_388_89),
    CoastPoint::new("Chimbote", -9.074_544_44, -78.593_572_22),
    CoastPoint::new("Callao", -12.052_222_22, -77.139_166_67),
    CoastPoint::new("Paracas Peninsula", -13.858_888_89, -76.328_888_89),
    CoastPoint::new("Marcona District", -15.3617, -75.1666),
    CoastPoint::new("Mollendo", -17.023_055_56, -72.014_722_22),
    CoastPoint::new("Arica", -18.478_39, -70.321_22),
    CoastPoint::new("Iquique", -20.216_666_67, -70.15),
    CoastPoint::new("Antofagasta", -23.65, -70.4),
    CoastPoint::new("Taltal", -25.4, -70.483_333_33),
    CoastPoint::new("Caldera, Chile", -27.066_666_67, -70.833_333_33),
    CoastPoint::new("Point Lengua de Vaca", -30.238_055_56, -71.627_222_22),
    CoastPoint::new("Valparaíso", -33.046_111_11, -71.619_722_22),
    CoastPoint::new("Constitución, Chile", -35.333_333_33, -72.416_666_67),
    CoastPoint::new("Lebu, Chile", -37.6, -73.666_666_67),
    CoastPoint::new("Valdivia", -39.813_888_89, -73.245_833_33),
    CoastPoint::new("Puerto Montt", -41.466_666_67, -72.933_333_33),
    CoastPoint::new("Chaitén", -42.916_666_67, -72.7),
    CoastPoint::new("Taitao Peninsula", -46.5, -74.416_666_67),
    CoastPoint::new("Caleta Tortel", -47.783_333_33, -73.533_333_33),
    CoastPoint::new("Puerto Natales", -51.733_333_33, -72.516_666_67),
    CoastPoint::new("Cape Froward", -53.892_944_44, -71.306_416_67),
];

/// The western edge of the American mainland as two lines of named places:
/// Alaska, and the Pacific coast from British Columbia south. The Gulf of
/// Alaska between them is sea, so the two are not joined. Each line is
/// followed between its places in steps of at most a quarter of a degree.
pub const AMERICAS_WEST_COAST: [&[CoastPoint]; 2] = [&ALASKA_WEST_COAST, &PACIFIC_COAST];

/// The longest step, in degrees of latitude or longitude, between the
/// points at which the western edge of the Americas is judged.
const COAST_STEP_DEGREES: f64 = 0.25;

/// Wellington, which Diyanet names as the easternmost place whose dawn the
/// conjunction must precede, at the coordinates of the Wikipedia article
/// (`wikipedia-coordinates-americas`). KHGT's text says New Zealand
/// without a place, and the same point stands for it.
pub const WELLINGTON: Location = Location::new(-41.288_888_89, 174.777_222_22, 0.0);

/// The step in latitude, in degrees, at which the terminator at 24:00 UT
/// is judged.
const TERMINATOR_STEP_DEGREES: f64 = 0.25;

/// The Sun's and the Moon's apparent places over the day and a half in
/// which the sunsets of one local date fall anywhere on Earth, sampled at
/// seven instants and interpolated: the searches below judge hundreds of
/// sunsets an evening, and the lunar series is the cost of each.
///
/// Over a node spacing of 0.3 day a sixth-degree polynomial follows the
/// Moon to well under a second of arc; the crate's tests bound it.
#[derive(Debug, Clone, Copy)]
struct Sky {
    first: f64,
    sun_ra: [f64; NODES],
    sun_dec: [f64; NODES],
    moon_ra: [f64; NODES],
    moon_dec: [f64; NODES],
    parallax: [f64; NODES],
    sidereal: f64,
}

/// How many instants [`Sky`] samples.
const NODES: usize = 7;

/// The spacing of [`Sky`]'s nodes, in days: 1.8 days from the first to
/// the last.
const SKY_STEP: f64 = 1.8 / (NODES - 1) as f64;

/// How much the apparent sidereal time advances in a day of Universal
/// Time, in degrees.
const SIDEREAL_DEGREES_PER_DAY: f64 = 360.985_647_366_29;

/// Right ascensions unwrapped so that they increase without a jump.
fn unwrapped(values: [f64; NODES]) -> [f64; NODES] {
    let mut out = values;
    for index in 1..NODES {
        while out[index] < out[index - 1] - 180.0 {
            out[index] += 360.0;
        }
        while out[index] > out[index - 1] + 180.0 {
            out[index] -= 360.0;
        }
    }
    out
}

/// The Lagrange polynomial through the equally spaced nodes, at a
/// fractional node index.
fn interpolate(values: &[f64; NODES], at: f64) -> f64 {
    let mut total = 0.0;
    for (i, value) in values.iter().enumerate() {
        let mut weight = 1.0;
        for j in 0..NODES {
            if i != j {
                weight *= (at - j as f64) / (i as f64 - j as f64);
            }
        }
        total += weight * value;
    }
    total
}

/// A direction in right ascension and declination, in degrees.
#[derive(Debug, Clone, Copy)]
struct Place {
    ra: f64,
    dec: f64,
}

impl Sky {
    /// The sky over the sunsets of local date `eve`.
    fn for_evening(eve: Rd) -> Self {
        let first = eve.0 as f64 - 0.2;
        let at = |index: usize| Moment(first + index as f64 * SKY_STEP);
        let sun: [_; NODES] = core::array::from_fn(|index| solar_position(at(index)));
        let moon: [_; NODES] = core::array::from_fn(|index| lunar_position(at(index)));
        Self {
            first,
            sun_ra: unwrapped(sun.map(|place| place.right_ascension_degrees)),
            sun_dec: sun.map(|place| place.declination_degrees),
            moon_ra: unwrapped(moon.map(|place| place.right_ascension_degrees)),
            moon_dec: moon.map(|place| place.declination_degrees),
            parallax: core::array::from_fn(|index| lunar_parallax(at(index))),
            sidereal: apparent_sidereal_time_iau1982(Moment(first)),
        }
    }

    fn index(&self, moment: f64) -> f64 {
        (moment - self.first) / SKY_STEP
    }

    fn sun(&self, moment: f64) -> Place {
        let at = self.index(moment);
        Place {
            ra: interpolate(&self.sun_ra, at),
            dec: interpolate(&self.sun_dec, at),
        }
    }

    fn moon(&self, moment: f64) -> Place {
        let at = self.index(moment);
        Place {
            ra: interpolate(&self.moon_ra, at),
            dec: interpolate(&self.moon_dec, at),
        }
    }

    fn parallax(&self, moment: f64) -> f64 {
        interpolate(&self.parallax, self.index(moment))
    }

    /// The apparent sidereal time at Greenwich, in degrees.
    fn sidereal_time(&self, moment: f64) -> f64 {
        self.sidereal + SIDEREAL_DEGREES_PER_DAY * (moment - self.first)
    }
}

/// The altitude of a direction at an hour angle and a latitude, in degrees.
fn altitude(place: Place, hour_angle: f64, latitude: f64) -> f64 {
    let sine = sin_deg(latitude) * sin_deg(place.dec)
        + cos_deg(latitude) * cos_deg(place.dec) * cos_deg(hour_angle);
    asin(sine.clamp(-1.0, 1.0)) * RAD_TO_DEG
}

/// The angle between two directions, in degrees.
fn separation(a: Place, b: Place) -> f64 {
    let cosine =
        sin_deg(a.dec) * sin_deg(b.dec) + cos_deg(a.dec) * cos_deg(b.dec) * cos_deg(a.ra - b.ra);
    acos(cosine.clamp(-1.0, 1.0)) * RAD_TO_DEG
}

/// An angle reduced to `(-180, 180]`.
fn signed(degrees: f64) -> f64 {
    let reduced = degrees - 360.0 * floor(degrees / 360.0);
    if reduced > 180.0 {
        reduced - 360.0
    } else {
        reduced
    }
}

/// The hour angle of the setting Sun's centre at a latitude, or `None`
/// where it does not set: the centre 50′ below the geometric horizon.
fn setting_hour_angle(sun: Place, latitude: f64) -> Option<f64> {
    let cosine = (sin_deg(sunrise_altitude_degrees(0.0)) - sin_deg(latitude) * sin_deg(sun.dec))
        / (cos_deg(latitude) * cos_deg(sun.dec));
    (-1.0..=1.0)
        .contains(&cosine)
        .then(|| acos(cosine) * RAD_TO_DEG)
}

impl SunsetCriterion {
    /// Whether the thresholds hold at a moment, at a place, in `sky`.
    fn holds_in(&self, sky: &Sky, moment: f64, latitude: f64, longitude: f64) -> bool {
        let sun = sky.sun(moment);
        let moon = sky.moon(moment);
        let sidereal = sky.sidereal_time(moment);
        let sun_altitude = altitude(sun, sidereal + longitude - sun.ra, latitude);
        let geocentric = altitude(moon, sidereal + longitude - moon.ra, latitude);
        let parallax = sky.parallax(moment);
        let topocentric = geocentric
            - asin((sin_deg(parallax) * cos_deg(geocentric)).clamp(-1.0, 1.0)) * RAD_TO_DEG;
        let arc = separation(sun, moon);
        let elongation = match self.elongation {
            Frame::Geocentric => arc,
            Frame::Topocentric => {
                // As `topocentric_arc_of_light`: the azimuth difference from
                // the geocentric arc, the arc again from the lowered Moon.
                let cos_azimuth = ((cos_deg(arc) - sin_deg(sun_altitude) * sin_deg(geocentric))
                    / (cos_deg(sun_altitude) * cos_deg(geocentric)))
                .clamp(-1.0, 1.0);
                let cosine = sin_deg(sun_altitude) * sin_deg(topocentric)
                    + cos_deg(sun_altitude) * cos_deg(topocentric) * cos_azimuth;
                acos(cosine.clamp(-1.0, 1.0)) * RAD_TO_DEG
            }
        };
        let height = match self.altitude {
            Frame::Geocentric => geocentric,
            Frame::Topocentric => topocentric,
        };
        elongation >= self.minimum_elongation_degrees && height >= self.minimum_altitude_degrees
    }
}

/// Sunset on local date `eve` at a latitude and longitude, from `sky`: the
/// moment the Sun's centre is 50′ below the geometric horizon, by four
/// passes of the hour-angle relation from 18:00 local mean time.
fn sunset_in(sky: &Sky, eve: Rd, latitude: f64, longitude: f64) -> Option<f64> {
    let mut moment = eve.0 as f64 + 0.75 - longitude / 360.0;
    for _ in 0..4 {
        let sun = sky.sun(moment);
        let setting = setting_hour_angle(sun, latitude)?;
        let hour_angle = sky.sidereal_time(moment) + longitude - sun.ra;
        moment -= signed(hour_angle - setting) / SIDEREAL_DEGREES_PER_DAY;
    }
    Some(moment)
}

/// The points at which the western edge of the Americas is judged: every
/// named place, and points between neighbours at most
/// [`COAST_STEP_DEGREES`] apart, as `(latitude, longitude)`.
fn coast_points() -> impl Iterator<Item = (f64, f64)> {
    AMERICAS_WEST_COAST.into_iter().flat_map(|line| {
        line.windows(2)
            .flat_map(|pair| {
                let (a, b) = (pair[0], pair[1]);
                let span = hc_core::math::abs(b.latitude_degrees - a.latitude_degrees).max(
                    hc_core::math::abs(b.longitude_degrees - a.longitude_degrees),
                );
                let steps = hc_core::math::ceil(span / COAST_STEP_DEGREES).max(1.0) as usize;
                (0..steps).map(move |step| {
                    let fraction = step as f64 / steps as f64;
                    (
                        a.latitude_degrees + (b.latitude_degrees - a.latitude_degrees) * fraction,
                        a.longitude_degrees
                            + (b.longitude_degrees - a.longitude_degrees) * fraction,
                    )
                })
            })
            .chain(
                line.last()
                    .map(|last| (last.latitude_degrees, last.longitude_degrees)),
            )
    })
}

/// A month rule over the whole Earth: the parameters at sunset, and where
/// and before when they must be met.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalRule {
    /// The elongation and altitude that must hold at a sunset.
    pub criterion: SunsetCriterion,
    /// Whether the parameters met anywhere on Earth at a sunset before
    /// 24:00 UT begin the month on the next day, as KHGT's first parameter
    /// has it.
    pub anywhere_before_midnight: bool,
}

impl GlobalRule {
    /// Muhammadiyah's KHGT: [`VisibilityCriterion::KHGT`] anywhere before
    /// 24:00 UT, or on the mainland of the Americas with the conjunction
    /// before dawn in New Zealand.
    pub const KHGT: Self = Self {
        criterion: sunset_criterion(VisibilityCriterion::KHGT),
        anywhere_before_midnight: true,
    };

    /// Diyanet's rule: [`VisibilityCriterion::ISTANBUL_2016`] anywhere
    /// before 24:00 UT, or on the mainland of the Americas with the
    /// conjunction before *imsāk* at Wellington.
    pub const ISTANBUL_2016: Self = Self {
        criterion: sunset_criterion(VisibilityCriterion::ISTANBUL_2016),
        anywhere_before_midnight: true,
    };

    /// Whether the parameters hold at some sunset, anywhere on Earth,
    /// before 24:00 UT on the civil day `eve`: on the evening terminator at
    /// that instant, at every quarter degree of latitude where the Sun sets.
    #[must_use]
    pub fn met_anywhere_before_midnight(&self, eve: Rd) -> bool {
        let sky = Sky::for_evening(eve);
        let midnight = (eve.0 + 1) as f64;
        if !after_conjunction(eve, midnight) {
            return false;
        }
        let sun = sky.sun(midnight);
        let sidereal = sky.sidereal_time(midnight);
        let steps = (180.0 / TERMINATOR_STEP_DEGREES) as i32;
        (1..steps).any(|step| {
            let latitude = -90.0 + f64::from(step) * TERMINATOR_STEP_DEGREES;
            let Some(setting) = setting_hour_angle(sun, latitude) else {
                return false;
            };
            let longitude = signed(sun.ra + setting - sidereal);
            // A sunset at 24:00 UT belongs to the local date `eve` west of
            // Greenwich; east of it the Sun set there on the next day.
            longitude < 0.0 && self.criterion.holds_in(&sky, midnight, latitude, longitude)
        })
    }

    /// Whether the parameters hold at a sunset of the local date `eve`
    /// somewhere on the western edge of the American mainland.
    #[must_use]
    pub fn met_on_the_americas(&self, eve: Rd) -> bool {
        let sky = Sky::for_evening(eve);
        coast_points().any(|(latitude, longitude)| {
            sunset_in(&sky, eve, latitude, longitude).is_some_and(|moment| {
                after_conjunction(eve, moment)
                    && self.criterion.holds_in(&sky, moment, latitude, longitude)
            })
        })
    }

    /// Whether the new month begins on `rd`, judged on the evening before.
    fn computed_begins_month_on(&self, rd: Rd) -> bool {
        let eve = Rd(rd.0 - 1);
        (self.anywhere_before_midnight && self.met_anywhere_before_midnight(eve))
            || (conjunction_before_new_zealand_dawn(eve) && self.met_on_the_americas(eve))
    }

    /// The rule as words of a [`hc_core::memo`] key.
    fn key(&self) -> [u64; 5] {
        let criterion = self.criterion;
        [
            criterion.minimum_elongation_degrees.to_bits(),
            criterion.minimum_altitude_degrees.to_bits(),
            u64::from(criterion.elongation == Frame::Topocentric),
            u64::from(criterion.altitude == Frame::Topocentric),
            u64::from(self.anywhere_before_midnight),
        ]
    }
}

/// The thresholds of a criterion of the at-sunset shape; any other shape is
/// not one a global rule is stated in, and yields thresholds nothing
/// passes.
const fn sunset_criterion(criterion: VisibilityCriterion) -> SunsetCriterion {
    match criterion {
        VisibilityCriterion::AtSunset(thresholds) => thresholds,
        _ => SunsetCriterion {
            minimum_elongation_degrees: 360.0,
            elongation: Frame::Geocentric,
            minimum_altitude_degrees: 90.0,
            altitude: Frame::Geocentric,
        },
    }
}

/// The geocentric conjunction that opens the lunation whose first evening
/// `eve` might be.
fn conjunction(eve: Rd) -> Moment {
    hc_astro::new_moon_before(Moment(eve.0 as f64 + 1.75))
}

/// Whether `moment` falls after the conjunction and within the week after
/// it, when the Moon is past new and short of first quarter.
fn after_conjunction(eve: Rd, moment: f64) -> bool {
    let new_moon = conjunction(eve).0;
    moment > new_moon && moment - new_moon < 7.0
}

/// The depression of the Sun at the dawn in New Zealand that the
/// conjunction must precede, in degrees: astronomical dawn. Neither body's
/// text read gives the angle; `docs/systems/unified-hijri.md` says how
/// much it matters.
pub const NEW_ZEALAND_DAWN: Twilight = Twilight::Astronomical;

/// Whether the conjunction fell before dawn at Wellington on the civil day
/// after `eve`, the day the new month would begin there first.
#[must_use]
pub fn conjunction_before_new_zealand_dawn(eve: Rd) -> bool {
    dawn(Rd(eve.0 + 1), WELLINGTON, NEW_ZEALAND_DAWN).is_some_and(|at| conjunction(eve).0 < at.0)
}

impl NewMonthRule for GlobalRule {
    /// Inside a [`hc_core::memo::scope`] each day is judged once.
    fn begins_month_on(&self, rd: Rd) -> bool {
        enum BeginsMonth {}
        let mut key = [0; 6];
        key[..5].copy_from_slice(&self.key());
        key[5] = rd.0 as u64;
        hc_core::memo::cached::<BeginsMonth, _, 6>(key, || self.computed_begins_month_on(rd))
    }
}

/// A Hijri calendar under a [`GlobalRule`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IslamicGlobalCalendar {
    rule: GlobalRule,
    id: CalendarId,
    english_name: &'static str,
    usage: hc_calendar::Usage,
    native_locales: &'static [&'static str],
}

/// Where KHGT's period of use comes from.
pub const KHGT_USAGE_SOURCE: &str = "Muhammadiyah, The Unified Global Hijri Calendar (2025), \
    preface: implementation from 1 Muharram 1447 AH, 26 June 2025 [muhammadiyah-ughc-2025]";

impl IslamicGlobalCalendar {
    /// Muhammadiyah's KHGT: `islamic-khgt`.
    pub const KHGT: Self = Self {
        rule: GlobalRule::KHGT,
        id: KHGT_ID,
        english_name: "Hijri (Unified Global Hijri Calendar, Muhammadiyah)",
        usage: hc_calendar::Usage::since(civil::to_rd(2025, 6, 26), KHGT_USAGE_SOURCE),
        native_locales: &["id"],
    };

    /// Diyanet's Unified Hijri Calendar: `islamic-istanbul-2016`.
    pub const ISTANBUL_2016: Self = Self {
        rule: GlobalRule::ISTANBUL_2016,
        id: ISTANBUL_2016_ID,
        english_name: "Hijri (Unified Hijri Calendar, Diyanet)",
        usage: hc_calendar::Usage::UNRECORDED,
        native_locales: &["tr"],
    };

    /// The rule this calendar begins its months by.
    #[must_use]
    pub const fn rule(&self) -> GlobalRule {
        self.rule
    }

    /// The Hijri year, month and day of a fixed day.
    ///
    /// # Errors
    ///
    /// The range errors outside 1900–2100, and
    /// [`CalendarError::AstronomicalModelFailure`] when the month search
    /// does not converge.
    pub fn decompose(&self, rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        decompose_with(&self.rule, rd)
    }

    /// The fixed day of a Hijri date.
    ///
    /// # Errors
    ///
    /// [`CalendarError::MonthOutOfRange`], [`CalendarError::DayOutOfRange`],
    /// the range errors, or [`CalendarError::AstronomicalModelFailure`].
    pub fn compose(&self, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        compose_with(&self.rule, year, month, day)
    }
}

impl Calendar for IslamicGlobalCalendar {
    type Date = IslamicDate;

    /// KHGT from 1 Muḥarram 1447, when Muhammadiyah put it into effect.
    /// Diyanet's calendar is unrecorded here: its statement dates the
    /// criteria to the Istanbul conference of 1978 and their confirmation
    /// to the congress of 2016, and no source read says from when its
    /// lists of religious days follow the 2016 rule.
    fn usage(&self) -> hc_calendar::Usage {
        self.usage
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// A year of 355 days.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        let start = self.compose(year, 1, 1)?;
        let next = self.compose(year + 1, 1, 1)?;
        Ok(next.0 - start.0 == 355)
    }

    /// The Islamic day begins at sunset and is named by the civil day it
    /// ends on; both bodies date the month by the civil day that follows
    /// the evening the parameters are met on.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset(hc_calendar::DayNaming::ByEnd)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id,
            english_name: self.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: true,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: self.native_locales,
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.compose(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = self.decompose(rd)?;
        Ok(IslamicDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(ERA))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = IslamicDate {
            year: fields.year,
            month: month.ordinal,
            day: fields.require_day()?,
        };
        self.compose(date.year, date.month, date.day)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_astro::riseset::{lunar_altitude, sunset};

    fn day(year: i64, month: u8, day: u8) -> Rd {
        civil::to_rd(year, month, day)
    }

    /// The interpolated sky follows the series it samples: the Sun and the
    /// Moon to a ten-thousandth of a degree, the sidereal time to a
    /// thousandth, over the whole window.
    #[test]
    fn the_interpolated_sky_follows_the_ephemeris() {
        for eve in [day(1900, 1, 3), day(2026, 2, 17), day(2100, 12, 30)] {
            let sky = Sky::for_evening(eve);
            for step in 0..=36 {
                let moment = sky.first + f64::from(step) * 0.05;
                let sun = solar_position(Moment(moment));
                let moon = lunar_position(Moment(moment));
                let ours_sun = sky.sun(moment);
                let ours_moon = sky.moon(moment);
                assert!(
                    hc_core::math::abs(signed(ours_sun.ra - sun.right_ascension_degrees)) < 1e-4
                );
                assert!(hc_core::math::abs(ours_sun.dec - sun.declination_degrees) < 1e-4);
                assert!(
                    hc_core::math::abs(signed(ours_moon.ra - moon.right_ascension_degrees)) < 1e-4,
                    "{step}"
                );
                assert!(hc_core::math::abs(ours_moon.dec - moon.declination_degrees) < 1e-4);
                let sidereal = apparent_sidereal_time_iau1982(Moment(moment));
                assert!(hc_core::math::abs(signed(sky.sidereal_time(moment) - sidereal)) < 1e-3);
                assert!(
                    hc_core::math::abs(sky.parallax(moment) - lunar_parallax(Moment(moment)))
                        < 1e-6
                );
            }
        }
    }

    /// The sunsets the searches use are `hc-astro`'s to a few seconds.
    #[test]
    fn the_searched_sunsets_are_the_crates_sunsets() {
        let eve = day(2026, 2, 17);
        let sky = Sky::for_evening(eve);
        for (latitude, longitude) in coast_points().step_by(17) {
            let ours = sunset_in(&sky, eve, latitude, longitude).expect("a sunset");
            let theirs = sunset(eve, Location::new(latitude, longitude, 0.0)).expect("a sunset");
            assert!(
                hc_core::math::abs(ours - theirs.0) * 86_400.0 < 3.0,
                "{latitude}, {longitude}"
            );
        }
    }

    /// Muhammadiyah's case for 1 Ramaḍān 1447 = 18 February 2026: the
    /// conjunction at 12:01 UT on the 17th, nothing met before 24:00 UT,
    /// the conjunction before dawn at Wellington, and the parameters met in
    /// Alaska (`muhammadiyah-ramadan-1447`). Diyanet's for 19 February: the
    /// crescent visible that evening over the Pacific and not on the
    /// American mainland (`diyanet-ramazan-1447`). With a geocentric
    /// altitude the Alaska Peninsula has it; with a topocentric one no
    /// point of the mainland does.
    #[test]
    fn ramadan_1447_begins_a_day_apart_and_for_the_stated_reasons() {
        let eve = day(2026, 2, 17);
        let khgt = GlobalRule::KHGT;
        let diyanet = GlobalRule::ISTANBUL_2016;
        assert!(!khgt.met_anywhere_before_midnight(eve));
        assert!(!diyanet.met_anywhere_before_midnight(eve));
        assert!(conjunction_before_new_zealand_dawn(eve));
        assert!(khgt.met_on_the_americas(eve));
        assert!(!diyanet.met_on_the_americas(eve));
        assert!(diyanet.met_on_the_americas(Rd(eve.0 + 1)));
        assert_eq!(
            IslamicGlobalCalendar::KHGT.compose(1_447, 9, 1),
            Ok(day(2026, 2, 18))
        );
        assert_eq!(
            IslamicGlobalCalendar::ISTANBUL_2016.compose(1_447, 9, 1),
            Ok(day(2026, 2, 19))
        );
        // The conjunction at 12:01 UT, as both bodies give it.
        let new_moon = conjunction(eve).0 - eve.0 as f64;
        assert!(hc_core::math::abs(new_moon * 1_440.0 - (12.0 * 60.0 + 1.0)) < 1.0);
    }

    /// Muhammadiyah began the calendar on 1 Muḥarram 1447 = 26 June 2025
    /// (`muhammadiyah-ughc-2025`, preface).
    #[test]
    fn the_calendar_began_on_the_first_of_muharram_1447() {
        assert_eq!(
            IslamicGlobalCalendar::KHGT.compose(1_447, 1, 1),
            Ok(day(2025, 6, 26))
        );
        let usage = IslamicGlobalCalendar::KHGT.usage();
        assert_eq!(usage.from, Some(day(2025, 6, 26)));
    }

    /// At a place the named criteria are the same thresholds: at Bethel,
    /// Alaska, on 17 February 2026 the geocentric figures are a hair under
    /// 8° and 5°, which is why the peninsula to the south decides.
    #[test]
    fn bethel_on_the_evening_of_17_february_2026_just_misses() {
        let bethel = Location::new(60.7922, -161.7558, 0.0);
        let set = sunset(day(2026, 2, 17), bethel).expect("a sunset");
        let arc = crate::islamic_observational::arc_of_light(set);
        let height = lunar_altitude(set, bethel);
        assert!((7.99..8.0).contains(&arc), "{arc}");
        assert!((4.99..5.0).contains(&height), "{height}");
    }

    /// Every day of 1900–2100 round-trips under both rules: every day in a
    /// release build, spread over the machine's threads; in a debug build
    /// every 101st day and every 1 Muḥarram with the day before it
    /// (docs/policy.md §7).
    #[test]
    fn every_day_of_1900_to_2100_round_trips_under_both_rules() {
        use crate::islamic_observational::{FIRST_YEAR, LAST_YEAR};
        for calendar in [
            IslamicGlobalCalendar::KHGT,
            IslamicGlobalCalendar::ISTANBUL_2016,
        ] {
            let openings: alloc::vec::Vec<i64> = if cfg!(debug_assertions) {
                // One memo for the openings: a year's composition reads
                // the month back, asking the same evenings again.
                hc_core::memo::scope(|| {
                    (FIRST_YEAR..=LAST_YEAR)
                        .filter_map(|year| calendar.compose(year, 1, 1).ok())
                        .map(|day| day.0)
                        .collect()
                })
            } else {
                alloc::vec::Vec::new()
            };
            // Every year's 1 Muḥarram but the first's, which falls before
            // 1900, is in the sample.
            if cfg!(debug_assertions) {
                assert_eq!(openings.len(), (LAST_YEAR - FIRST_YEAR) as usize);
            }
            let mut days: alloc::vec::Vec<i64> =
                crate::sweep_days(EARLIEST.0, LATEST.0, 101, openings.iter().copied())
                    .chain([LATEST.0])
                    .collect();
            days.sort_unstable();
            days.dedup();
            crate::check_days(&days, |day| {
                let date = calendar.from_fixed(Rd(day)).expect("in range");
                assert_eq!(calendar.to_fixed(date), Ok(Rd(day)), "RD {day}: {date:?}");
            });
        }
    }

    /// Every month both rules begin in 1900–2100 has 29 or 30 days, as
    /// Muhammadiyah requires of its calendar (`muhammadiyah-ughc-2025`,
    /// §C.2 b): every month in a release build, the months of every
    /// seventh year in a debug one.
    #[test]
    fn every_month_has_twenty_nine_or_thirty_days() {
        use crate::islamic_observational::{FIRST_YEAR, LAST_YEAR};
        for calendar in [
            IslamicGlobalCalendar::KHGT,
            IslamicGlobalCalendar::ISTANBUL_2016,
        ] {
            hc_core::memo::scope(|| {
                for year in crate::sweep_years(FIRST_YEAR + 1, LAST_YEAR - 1, 7) {
                    let starts: alloc::vec::Vec<i64> = (1..=12u8)
                        .map(|month| calendar.compose(year, month, 1).expect("in range").0)
                        .chain([calendar.compose(year + 1, 1, 1).expect("in range").0])
                        .collect();
                    for (month, pair) in starts.windows(2).enumerate() {
                        let length = pair[1] - pair[0];
                        assert!(matches!(length, 29 | 30), "{year}-{}: {length}", month + 1);
                    }
                }
            });
        }
    }
}
