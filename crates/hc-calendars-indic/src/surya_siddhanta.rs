//! The Sun and Moon of the *Sūrya Siddhānta*: the classical model whose
//! saṅkrāntis and tithis the traditional almanacs still keep, and the
//! sunrise it reads the day at.
//!
//! The Siddhānta moves a mean Sun uniformly round a sidereal zodiac and
//! corrects it by an epicycle whose size shrinks with the anomaly, and it
//! reads its sines from a table of twenty-four values at steps of 225
//! minutes of arc, interpolating between them. Its zodiac is sidereal by
//! construction, so there is no ayanamsa: the zero point is where the
//! model puts it, and it is not the zero point of any modern ayanāṃśa.
//! Its saṅkrāntis therefore fall hours from the modern ones — the Meṣa
//! saṅkrānti of 2024 two hours and nineteen minutes after the Lahiri one.
//!
//! # Whose arithmetic
//!
//! The functions of Reingold and Dershowitz, *Calendrical Calculations*
//! (4th ed., 2018), in the section on the modern Hindu calendars, as their
//! published code gives them (`reingold2018code`; the book, `reingold2018`,
//! not read here):
//! `hindu-sine-table`, `hindu-sine`, `hindu-arcsin`,
//! `hindu-mean-position`, `hindu-true-position` and
//! `hindu-solar-longitude`, with the sidereal and anomalistic years, the
//! epicycle of 14⁄360 shrinking by 1⁄42, and the table's rounding
//! correction of 0.215 as they give them; `hindu-lunar-longitude`, with
//! the sidereal month, the anomalistic month with the *bīja* and the
//! epicycle of 32⁄360 shrinking by 1⁄96, `hindu-lunar-phase` and
//! `hindu-lunar-day-from-moment`; and `hindu-sunrise`, with
//! `hindu-equation-of-time`, `hindu-ascensional-difference`,
//! `hindu-tropical-longitude`, `hindu-rising-sign`, `hindu-daily-motion`
//! and `hindu-solar-sidereal-difference`, read 2026-09-27. The constants and the order of
//! operations were checked line by line against `modern_hindu.R` of the
//! `calcal` package on CRAN, an implementation of the book's functions,
//! retrieved 2026-09-23.
//!
//! One step differs in how it is computed and not in what it computes.
//! The book counts mean positions from the creation, 1 955 880 000
//! sidereal years before the Kali Yuga epoch; subtracting a date seven
//! hundred billion days away in floating point would keep only four
//! decimal places of a day. The creation is a whole number of sidereal
//! years before the epoch, so the mean Sun is counted from the epoch
//! instead, and the mean anomaly from the epoch plus the fraction of an
//! anomalistic revolution the creation-to-epoch interval leaves over,
//! which is exactly 0.785 75. The Moon is counted the same way: the
//! interval is a whole number of sidereal months, so the mean Moon stands
//! at zero at the epoch, and 25 926 790 776¾ anomalistic months, so its
//! anomaly stands at three quarters.
//!
//! The yoga and the karaṇa follow from the same two bodies as the tithi
//! does ([`yoga_at`], [`karana_at`]): Reingold and Dershowitz's `yoga` and
//! `karana` (`reingold2018code`), the day read at the Siddhānta's own
//! sunrise. The tests hold them to the pañcāṅga extract of Sewell and
//! Dikshit's Art. 30 (`sewell1896`), Poona, September 1894, which was
//! computed by the *Grahalāghava* and so checks the arithmetic, not the
//! model: every legible yoga end within 1.6 ghaṭikās of the printed one.
//!
//! # Whose clock
//!
//! The book evaluates the model at moments counted in the local time of
//! Ujjain, 75°46′6″ E, the prime meridian of the Siddhāntas; this module
//! takes [`Moment`]s in Universal Time and adds Ujjain's longitude before
//! evaluating. The months the Government of Nepal gazettes, which this
//! model reproduces (see [`crate::bikram_sambat`]), fit any reading of
//! the clock from five to six hours ahead of Universal Time, so the
//! choice of Ujjain's meridian over India's or Nepal's standard time does
//! not move any of them.

use hc_astro::riseset::Location;
use hc_astro::search::bisect_until;
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::{abs, ceil, floor, fract, modulo, round, sin_deg};
use hc_seasons::zodiac::SiderealSign;

use crate::places::UJJAIN;

/// The identifier a caller gives for the Siddhānta's sky where the true sky
/// is named by an ayanāṃśa: the boundary's `hc_hindu_lunar_date` takes
/// `lahiri` or another ayanāṃśa of the true sky, or this.
pub const SKY: &str = "surya-siddhanta";

/// The sidereal year: 1 577 917 828 days in a *mahāyuga* of 4 320 000
/// years, 365.258 756 days.
pub const SIDEREAL_YEAR: f64 = 1_577_917_828.0 / 4_320_000.0;

/// The anomalistic year: the same days in 4 320 000 000 − 387 revolutions
/// of the apsis, 365.258 789 days.
pub const ANOMALISTIC_YEAR: f64 = 1_577_917_828_000.0 / 4_319_999_613.0;

/// The sidereal month: the days of a *mahāyuga* in 57 753 336 revolutions
/// of the Moon, 27.321 674 days (`hindu-sidereal-month`).
pub const SIDEREAL_MONTH: f64 = 1_577_917_828.0 / 57_753_336.0;

/// The anomalistic month, with the *bīja*: the same days in 57 753 336 −
/// 488 199 revolutions of the Moon's apsis, 27.554 598 days
/// (`hindu-anomalistic-month`).
pub const ANOMALISTIC_MONTH: f64 = 1_577_917_828.0 / 57_265_137.0;

/// The mean synodic month, 29 + 7 087 771⁄13 358 334 days, 29.530 588
/// (`hindu-synodic-month`): the sidereal month against the sidereal year.
pub const SYNODIC_MONTH: f64 = 29.0 + 7_087_771.0 / 13_358_334.0;

/// The fraction of an anomalistic month between the creation and the
/// epoch: 1 955 880 000 sidereal years are 25 926 790 776¾ of them.
const MOON_ANOMALY_AT_EPOCH: f64 = 0.75;

/// The Kali Yuga epoch, Friday 18 February 3102 BCE in the Julian
/// calendar, as a fixed day number (`reingold2018code`, `hindu-epoch`).
const EPOCH: f64 = -1_132_959.0;

/// The fraction of an anomalistic revolution between the creation and the
/// epoch: 1 955 880 000 sidereal years are 1 955 880 000 × (1 −
/// 387⁄4 320 000 000) anomalistic years, 1 955 879 824.785 75.
const ANOMALY_AT_EPOCH: f64 = 0.785_75;

/// The step of the sine table, 225 minutes of arc, in degrees.
const SINE_STEP_DEGREES: f64 = 225.0 / 60.0;

/// The radius the table's sines are measured in, 3438 minutes of arc.
const RADIUS: f64 = 3_438.0;

/// An entry of the sine table: the sine of `entry` steps, in the radius's
/// whole minutes, with the book's correction for how the table was
/// rounded (`hindu-sine-table`).
fn sine_table(entry: f64) -> f64 {
    let exact = RADIUS * sin_deg(entry * SINE_STEP_DEGREES);
    let error = 0.215 * signum(exact) * signum(abs(exact) - 1_716.0);
    round(exact + error) / RADIUS
}

/// The sign of `x`, zero for zero, as the book's `sign`.
fn signum(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// The table's sine of an angle in degrees, interpolated linearly between
/// the entries either side (`hindu-sine`).
fn sine(theta: f64) -> f64 {
    let entry = theta / SINE_STEP_DEGREES;
    let part = fract(entry);
    part * sine_table(ceil(entry)) + (1.0 - part) * sine_table(floor(entry))
}

/// The angle in degrees whose table sine is `amplitude`, the inverse of
/// [`sine`] (`hindu-arcsin`).
///
/// An amplitude above the radius has no angle; the book's search would
/// step past the table's last entry for ever, and this one answers 90°
/// instead, the table's limit. [`sunrise`] meets it only where the Sun
/// does not rise.
fn arcsin(amplitude: f64) -> f64 {
    if amplitude < 0.0 {
        return -arcsin(-amplitude);
    }
    if amplitude >= 1.0 {
        return 90.0;
    }
    let mut position = 0.0;
    while amplitude > sine_table(position) {
        position += 1.0;
    }
    let below = sine_table(position - 1.0);
    SINE_STEP_DEGREES * (position - 1.0 + (amplitude - below) / (sine_table(position) - below))
}

/// The true longitude of a body moving with `period` on an epicycle of
/// relative `size`, shrinking by `change`, whose anomaly turns once in
/// `anomalistic` days (`hindu-true-position`), given the two mean motions
/// as fractions of a revolution.
fn true_position(mean: f64, anomaly: f64, size: f64, change: f64) -> f64 {
    let lambda = 360.0 * mean;
    let offset = sine(360.0 * anomaly);
    let contraction = abs(offset) * change * size;
    let equation = arcsin(offset * (size - contraction));
    let longitude = lambda - equation;
    modulo(longitude, 360.0)
}

/// Ujjain's local time, the book's clock, at a moment in Universal Time.
fn local(moment: Moment) -> f64 {
    moment.0 + UJJAIN.longitude_degrees / 360.0
}

/// The Sun's mean anomaly, as a fraction of a revolution, at a moment of
/// Ujjain's local time.
fn solar_anomaly(local: f64) -> f64 {
    fract((local - EPOCH) / ANOMALISTIC_YEAR + ANOMALY_AT_EPOCH)
}

/// The Sun's sidereal longitude in degrees at a moment of Ujjain's local
/// time.
fn solar_longitude_local(local: f64) -> f64 {
    let mean = fract((local - EPOCH) / SIDEREAL_YEAR);
    true_position(mean, solar_anomaly(local), 14.0 / 360.0, 1.0 / 42.0)
}

/// The Sun's sidereal longitude in degrees at a moment in Universal Time
/// (`hindu-solar-longitude`).
#[must_use]
pub fn solar_longitude(moment: Moment) -> f64 {
    solar_longitude_local(local(moment))
}

/// The Moon's sidereal longitude in degrees at a moment in Universal Time
/// (`hindu-lunar-longitude`).
#[must_use]
pub fn lunar_longitude(moment: Moment) -> f64 {
    let days = local(moment) - EPOCH;
    let mean = fract(days / SIDEREAL_MONTH);
    let anomaly = fract(days / ANOMALISTIC_MONTH + MOON_ANOMALY_AT_EPOCH);
    true_position(mean, anomaly, 32.0 / 360.0, 1.0 / 96.0)
}

/// The Moon's elongation from the Sun in degrees, 0 to 360, at a moment in
/// Universal Time (`hindu-lunar-phase`).
#[must_use]
pub fn lunar_phase(moment: Moment) -> f64 {
    let phase = lunar_longitude(moment) - solar_longitude(moment);
    modulo(phase, 360.0)
}

/// The number, 1 to 30, of the tithi in progress at a moment in Universal
/// Time (`hindu-lunar-day-from-moment`).
#[must_use]
pub fn tithi_at(moment: Moment) -> u8 {
    (floor(lunar_phase(moment) / 12.0) as u8).min(29) + 1
}

/// The first conjunction at or after a moment: the instant the elongation
/// returns to zero.
///
/// The book finds the conjunction before a moment only as closely as the
/// sign it falls in needs (`hindu-new-moon-before`); this finds the
/// instant, since a month here begins with the first sunrise after it.
#[must_use]
pub fn conjunction_at_or_after(moment: Moment) -> Moment {
    let phase = lunar_phase(moment);
    if phase == 0.0 {
        return moment;
    }
    // Carried at the mean rate from where the Moon stands, the estimate is
    // off by no more than the two equations of centre together, under
    // seven and a half degrees of elongation and so under two thirds of a
    // day.
    let estimate = moment.0 + (360.0 - phase) / 360.0 * SYNODIC_MONTH;
    let low = Moment((estimate - 1.5).max(moment.0));
    let high = Moment(estimate + 1.5);
    // Just before the conjunction the elongation is close to 360, just
    // after it close to 0.
    bisect_until(|at| lunar_phase(at) < 180.0, low, high, 64)
}

/// The sum of the Siddhānta's Sun and Moon, modulo 360°: the angle its
/// yoga counts.
#[must_use]
pub fn yoga_longitude(moment: Moment) -> f64 {
    modulo(solar_longitude(moment) + lunar_longitude(moment), 360.0)
}

/// The yoga in progress at a moment by the Siddhānta's Sun and Moon, 1 for
/// Viṣkambha through 27 for Vaidhṛti: one plus the floor of their sum,
/// modulo 360°, over 13°20′ (`reingold2018code`, `yoga`). The names are
/// [`crate::panchanga::YOGA_NAMES`]'s.
#[must_use]
pub fn yoga_at(moment: Moment) -> u8 {
    let arc = floor(yoga_longitude(moment) / crate::panchanga::DEGREES_PER_YOGA);
    (arc as u8).min(crate::panchanga::YOGAS_PER_REVOLUTION - 1) + 1
}

/// The yoga the Siddhānta gives a day: the one in progress at its own
/// sunrise at the place ([`sunrise`]).
#[must_use]
pub fn yoga_of_day(day: Rd, location: Location) -> u8 {
    yoga_at(sunrise(day, location))
}

/// The Siddhānta's yoga in progress at a moment: when it began and when it
/// ends.
#[must_use]
pub fn yoga_span(moment: Moment) -> (Moment, Moment) {
    crate::panchanga::span_of(yoga_longitude, crate::panchanga::DEGREES_PER_YOGA, moment)
}

/// The half-tithi in progress at a moment by the Siddhānta's elongation,
/// 1 for the first half of śukla 1 to 60 for the second half of amāvāsyā;
/// [`crate::panchanga::karana_name`] names it. Two to a tithi of
/// [`tithi_at`], as Reingold and Dershowitz's `karana` numbers them.
#[must_use]
pub fn karana_at(moment: Moment) -> u8 {
    let half = floor(lunar_phase(moment) / crate::panchanga::DEGREES_PER_KARANA);
    (half as u8).min(crate::panchanga::KARANAS_PER_MONTH - 1) + 1
}

/// The half-tithi the Siddhānta gives a day: the one in progress at its own
/// sunrise at the place.
#[must_use]
pub fn karana_of_day(day: Rd, location: Location) -> u8 {
    karana_at(sunrise(day, location))
}

/// The Siddhānta's half-tithi in progress at a moment: when it began and
/// when it ends.
#[must_use]
pub fn karana_span(moment: Moment) -> (Moment, Moment) {
    crate::panchanga::span_of(lunar_phase, crate::panchanga::DEGREES_PER_KARANA, moment)
}

/// `x` into `[low, high)`, as the book's `mod3`.
fn wrap(x: f64, low: f64, high: f64) -> f64 {
    let span = high - low;
    low + (x - low - span * floor((x - low) / span))
}

/// The Sun's daily motion in degrees on a day, `date` its midnight in
/// Ujjain's local time (`hindu-daily-motion`).
fn daily_motion(date: f64) -> f64 {
    let mean_motion = 360.0 / SIDEREAL_YEAR;
    let anomaly = 360.0 * solar_anomaly(date);
    let epicycle = 14.0 / 360.0 - abs(sine(anomaly)) / 1_080.0;
    let entry = floor(anomaly / SINE_STEP_DEGREES);
    let step = sine_table(entry + 1.0) - sine_table(entry);
    let factor = -RADIUS / 225.0 * step * epicycle;
    mean_motion * (1.0 + factor)
}

/// The Sun's tropical longitude in degrees on a day, with the book's
/// precession of at most 27° over a period of 7 200 sidereal years
/// (`hindu-tropical-longitude`).
fn tropical_longitude(date: f64) -> f64 {
    let days = floor(date) - EPOCH;
    let precession = 27.0 - abs(108.0 * wrap(600.0 / 1_577_917_828.0 * days - 0.25, -0.5, 0.5));
    let longitude = solar_longitude_local(date) - precession;
    modulo(longitude, 360.0)
}

/// The tabulated speed of rising of the sign the Sun stands in on a day
/// (`hindu-rising-sign`).
fn rising_sign(date: f64) -> f64 {
    const SPEEDS: [f64; 6] = [1_670.0, 1_795.0, 1_935.0, 1_935.0, 1_795.0, 1_670.0];
    let index = floor(tropical_longitude(date) / 30.0) as usize % 6;
    SPEEDS[index] / 1_800.0
}

/// The difference between the solar and the sidereal day, in degrees
/// (`hindu-solar-sidereal-difference`).
fn solar_sidereal_difference(date: f64) -> f64 {
    daily_motion(date) * rising_sign(date)
}

/// The time from true to mean midnight, in days (`hindu-equation-of-time`,
/// "a gross approximation to the correct value", as the book says).
fn equation_of_time(date: f64) -> f64 {
    let offset = sine(360.0 * solar_anomaly(date));
    let equation_of_sun = offset * (57.0 + 18.0 / 60.0) * (14.0 / 360.0 - abs(offset) / 1_080.0);
    daily_motion(date) / 360.0 * (equation_of_sun / 360.0) * SIDEREAL_YEAR
}

/// The difference between the right and the oblique ascension of the Sun
/// on a day at a latitude, in degrees (`hindu-ascensional-difference`).
fn ascensional_difference(date: f64, latitude_degrees: f64) -> f64 {
    let sin_declination = 1_397.0 / 3_438.0 * sine(tropical_longitude(date));
    let diurnal_radius = sine(90.0 + arcsin(sin_declination));
    let tan_latitude = sine(latitude_degrees) / sine(90.0 + latitude_degrees);
    let earth_sine = sin_declination * tan_latitude;
    arcsin(-earth_sine / diurnal_radius)
}

/// Sunrise on a day at a place, by the Siddhānta, in Universal Time
/// (`hindu-sunrise`): six in the morning at the place's meridian, less the
/// equation of time, plus the ascensional difference and a quarter of the
/// solar-sidereal difference turned from degrees of the sidereal day into
/// time.
///
/// The book computes it at Ujjain; its formula carries the place's
/// latitude and its longitude from Ujjain's, and so does this. The day is
/// the civil day of Ujjain's clock, which for any place of the
/// subcontinent is the local one. The ascensional difference is defined
/// only where the Sun rises every day, below [`MAX_SUNRISE_LATITUDE`];
/// beyond it the answer is the table's limit of 90° and names no sunrise.
#[must_use]
pub fn sunrise(day: Rd, location: Location) -> Moment {
    let date = day.0 as f64;
    let offset = (UJJAIN.longitude_degrees - location.longitude_degrees) / 360.0;
    let sidereal = 1_577_917_828.0 / 1_582_237_828.0 / 360.0;
    let local = date + 0.25 + offset - equation_of_time(date)
        + sidereal
            * (ascensional_difference(date, location.latitude_degrees)
                + 0.25 * solar_sidereal_difference(date));
    Moment(local - UJJAIN.longitude_degrees / 360.0)
}

/// The greatest latitude, north or south, at which the Siddhānta's Sun
/// rises every day: its greatest declination is arcsin(1397⁄3438),
/// 23.97°, so the ascensional difference is defined while tan φ is at most
/// cot 23.97°, 2.249, below 66.03° in exact trigonometry. The bound is
/// taken a degree inside that, since the model reads its sines off the
/// table; `tests::a_polar_place_does_not_hang_the_sunrise` holds a year of
/// sunrises at it.
pub const MAX_SUNRISE_LATITUDE: f64 = 65.0;

/// The sign the Sun stands in at a moment.
#[must_use]
pub fn sign_at(moment: Moment) -> SiderealSign {
    let index = floor(solar_longitude(moment) / 30.0) as u8 % 12;
    SiderealSign::from_index(index).unwrap_or(SiderealSign::MESHA)
}

/// The first moment at or after `moment` at which the Sun enters `sign`:
/// its saṅkrānti.
#[must_use]
pub fn ingress_after(sign: SiderealSign, moment: Moment) -> Moment {
    let target = sign.start_longitude_degrees();
    // How far the Sun still has to go, in degrees, from where it stands.
    let ahead = |at: f64| {
        let gap = target - solar_longitude(Moment(at));
        modulo(gap, 360.0)
    };
    // Carrying the Sun from where it stands at the mean rate misplaces the
    // entry by however much the equation of centre changes on the way,
    // which is never more than twice its greatest value of 2.18°: under
    // four and a half degrees, and so under five days at the Sun's slowest.
    let estimate = moment.0 + ahead(moment.0) / 360.0 * SIDEREAL_YEAR;
    let low = Moment((estimate - 6.0).max(moment.0));
    let high = Moment(estimate + 6.0);
    // Bisect on whether the Sun has passed the target: `ahead` is small
    // just before the entry and close to 360 just after it.
    bisect_until(|at| ahead(at.0) >= 180.0, low, high, 64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    fn ymd(year: i64, month: u8, day: u8) -> f64 {
        gregorian::to_fixed(year, month, day).unwrap().0 as f64
    }

    /// Beyond the latitude where the Sun rises every day the book's arcsin
    /// would search for ever; here it stops at the table's limit, and a
    /// sunrise at 80° N at midwinter comes back at once.
    #[test]
    fn a_polar_place_does_not_hang_the_sunrise() {
        let north = Location::new(80.0, 20.0, 0.0);
        let midwinter = Rd(739_241);
        assert!(sunrise(midwinter, north).0.is_finite());
        let edge = Location::new(MAX_SUNRISE_LATITUDE, 20.0, 0.0);
        for day in 739_000..739_366 {
            let rise = sunrise(Rd(day), edge).0 - day as f64;
            assert!((-0.5..1.0).contains(&rise), "R.D. {day}: {rise}");
        }
    }

    #[test]
    fn the_table_holds_the_classical_jyas() {
        // The twenty-four sines of the classical table in minutes of a
        // radius of 3438, as Wikipedia's "Āryabhaṭa's sine table"
        // (retrieved 2026-09-23) lists them: rounding 3438 sin A alone
        // gets several wrong — 1105.1089 is 1105 but 3083.4485 is not
        // 3084 — and the book's correction exists to land on these.
        let table: [f64; 24] = [
            225.0, 449.0, 671.0, 890.0, 1105.0, 1315.0, 1520.0, 1719.0, 1910.0, 2093.0, 2267.0,
            2431.0, 2585.0, 2728.0, 2859.0, 2978.0, 3084.0, 3177.0, 3256.0, 3321.0, 3372.0, 3409.0,
            3431.0, 3438.0,
        ];
        for (index, minutes) in table.iter().enumerate() {
            let entry = index as f64 + 1.0;
            assert_eq!(round(sine_table(entry) * RADIUS), *minutes, "entry {entry}");
        }
        assert_eq!(sine_table(0.0), 0.0);
    }

    #[test]
    fn arcsin_inverts_sine() {
        for tenth in -900..=900 {
            let theta = f64::from(tenth) / 10.0;
            assert!((arcsin(sine(theta)) - theta).abs() < 1e-9, "{theta}");
        }
    }

    #[test]
    fn the_longitude_comes_round_in_a_sidereal_year() {
        // The mean Sun exactly; the true Sun all but, because the anomaly
        // is 33 millionths of a day short of a revolution.
        let start = Moment(ymd(2000, 1, 1));
        let after = Moment(start.0 + SIDEREAL_YEAR);
        let gap = solar_longitude(after) - solar_longitude(start);
        assert!(gap.abs() < 1e-5, "{gap}");
    }

    #[test]
    fn the_equation_of_centre_is_at_most_two_and_a_quarter_degrees() {
        // The epicycle of 14⁄360 of the deferent, shrunk by a forty-second
        // at quadrature: arcsin(14⁄360 × (1 − 1⁄42)) is 2.18°.
        let mut largest: f64 = 0.0;
        for day in 0..366 {
            let days = ymd(2024, 1, 1) + f64::from(day) + UJJAIN.longitude_degrees / 360.0 - EPOCH;
            let mean = 360.0 * fract(days / SIDEREAL_YEAR);
            let lon = solar_longitude(Moment(ymd(2024, 1, 1) + f64::from(day)));
            let gap = (lon - mean + 540.0) % 360.0 - 180.0;
            largest = largest.max(gap.abs());
        }
        assert!(largest > 2.1 && largest < 2.2, "{largest}");
    }

    #[test]
    fn an_ingress_is_where_the_longitude_crosses_the_sign() {
        let from = Moment(ymd(2024, 1, 1));
        for index in 0..12 {
            let sign = SiderealSign::from_index(index).unwrap();
            let entry = ingress_after(sign, from);
            assert!(entry.0 >= from.0 && entry.0 < from.0 + 366.0);
            assert_eq!(sign_at(Moment(entry.0 + 1e-6)), sign);
            assert_eq!(sign_at(Moment(entry.0 - 1e-6)), sign.previous());
        }
    }

    #[test]
    fn the_mesha_sankranti_of_2024_is_later_than_the_lahiri_one() {
        use hc_seasons::zodiac::Ayanamsa;
        use hc_seasons::zodiac::sidereal;
        let from = Moment(ymd(2024, 3, 1));
        let siddhanta = ingress_after(SiderealSign::MESHA, from);
        let lahiri = sidereal::ingress_after(SiderealSign::MESHA, Ayanamsa::LAHIRI, from);
        let minutes = (siddhanta.0 - lahiri.0) * 24.0 * 60.0;
        assert!((minutes - 139.0).abs() < 1.0, "{minutes}");
    }

    /// Poona, 18°31′ N, 73°52′ E, where the pañcāṅga of Sewell and
    /// Dikshit's Art. 30 was published.
    const POONA: Location = Location::new(18.0 + 31.0 / 60.0, 73.0 + 52.0 / 60.0, 0.0);

    /// Sewell and Dikshit's pañcāṅga extract of Art. 30 (`sewell1896`),
    /// Poona, Bhādrapada of Śaka 1816 expired, 31 August to 29 September
    /// 1894: the day as days after 31 August, the yoga, 1 to 27, it prints
    /// for the day and the ghaṭikās and palas after sunrise at which that
    /// yoga ends. Read off the Internet Archive's OCR text; the days whose
    /// figures the OCR garbles are left out. The extract is computed by
    /// Gaṇeśa Daivajña's *Grahalāghava*, not by the *Sūrya Siddhānta*.
    const EXTRACT_YOGAS: [(i64, u8, u8, u8); 24] = [
        (0, 21, 31, 22),
        (1, 22, 25, 23),
        (2, 23, 19, 31),
        (3, 24, 14, 50),
        (4, 25, 11, 7),
        (5, 26, 8, 24),
        (6, 27, 6, 30),
        (7, 1, 5, 19),
        (8, 2, 6, 2),
        (9, 3, 6, 53),
        (10, 4, 8, 1),
        (11, 5, 9, 29),
        (13, 7, 11, 51),
        (14, 8, 12, 20),
        (15, 9, 12, 7),
        (16, 10, 10, 43),
        (17, 11, 8, 30),
        (20, 15, 49, 13),
        (21, 16, 43, 1),
        (22, 17, 35, 58),
        (23, 18, 28, 28),
        (24, 19, 20, 15),
        (27, 23, 51, 4),
        (29, 25, 38, 10),
    ];

    /// The same extract's karaṇas: the day, the name, 0 to 10 as
    /// [`crate::panchanga::KARANA_NAMES`] numbers them, and the ghaṭikās
    /// and palas after sunrise at which it ends.
    const EXTRACT_KARANAS: [(i64, u8, u8, u8); 15] = [
        (0, 0, 16, 30),
        (1, 2, 11, 53),
        (2, 4, 8, 9),
        (3, 6, 5, 27),
        (4, 1, 3, 54),
        (5, 3, 3, 42),
        (6, 5, 4, 44),
        (7, 7, 6, 53),
        (9, 4, 14, 28),
        (10, 6, 19, 16),
        (11, 1, 24, 14),
        (15, 1, 8, 11),
        (16, 3, 9, 59),
        (18, 7, 9, 35),
        (19, 2, 7, 20),
    ];

    /// The ghaṭikās from the Siddhānta's sunrise at Poona to a moment.
    fn ghatikas(day: Rd, moment: Moment) -> f64 {
        (moment.0 - sunrise(day, POONA).0) * 60.0
    }

    #[test]
    fn the_siddhantas_yoga_is_the_extracts_on_every_legible_day() {
        let first = gregorian::to_fixed(1894, 8, 31).unwrap();
        let mut worst: f64 = 0.0;
        for (offset, yoga, ghatika, pala) in EXTRACT_YOGAS {
            let day = Rd(first.0 + offset);
            assert_eq!(yoga_of_day(day, POONA), yoga, "day {offset}");
            let (_, ends) = yoga_span(sunrise(day, POONA));
            let printed = f64::from(ghatika) + f64::from(pala) / 60.0;
            worst = worst.max((ghatikas(day, ends) - printed).abs());
        }
        // Every end within a ghaṭikā and three quarters, 42 minutes, of
        // the printed one: the *Grahalāghava*'s Sun and Moon are not the
        // Siddhānta's.
        assert!(worst < 1.75, "{worst} ghatikas");
    }

    #[test]
    fn the_siddhantas_karana_is_the_extracts_on_every_legible_day() {
        let first = gregorian::to_fixed(1894, 8, 31).unwrap();
        let mut worst: f64 = 0.0;
        for (offset, name, ghatika, pala) in EXTRACT_KARANAS {
            let day = Rd(first.0 + offset);
            let half = karana_of_day(day, POONA);
            assert_eq!(crate::panchanga::karana_name(half), name, "day {offset}");
            let (_, ends) = karana_span(sunrise(day, POONA));
            let printed = f64::from(ghatika) + f64::from(pala) / 60.0;
            worst = worst.max((ghatikas(day, ends) - printed).abs());
        }
        assert!(worst < 2.0, "{worst} ghatikas");
    }

    #[test]
    fn the_yoga_is_the_books_arithmetic_and_the_karana_half_the_tithi() {
        // `yoga`: one plus the floor of the two longitudes' sum, modulo
        // 360°, over 13°20′; `karana` numbers two halves to a tithi.
        let mut moment = Moment(ymd(2024, 1, 1));
        for _ in 0..200 {
            let sum = (solar_longitude(moment) + lunar_longitude(moment)) % 360.0;
            assert_eq!(yoga_at(moment), (sum / (360.0 / 27.0)) as u8 + 1);
            assert_eq!(karana_at(moment).div_ceil(2), tithi_at(moment));
            let (entry, exit) = yoga_span(moment);
            assert!(entry.0 <= moment.0 && moment.0 < exit.0);
            assert_eq!(yoga_at(Moment(exit.0 + 1e-5)), yoga_at(moment) % 27 + 1);
            let (entry, exit) = karana_span(moment);
            assert!(entry.0 <= moment.0 && moment.0 < exit.0);
            assert_eq!(karana_at(Moment(exit.0 + 1e-5)), karana_at(moment) % 60 + 1);
            moment = Moment(moment.0 + 0.37);
        }
    }

    /// Reingold and Dershowitz's ayanāṃśa is zero at this Sun's Meṣa
    /// saṅkrānti of 285 CE (`sidereal-start`), which `hc-seasons` anchors
    /// as a Julian date because it cannot compute this Sun itself.
    #[test]
    fn the_books_ayanamsa_is_zero_at_the_siddhantas_mesha_of_285() {
        use hc_seasons::zodiac::Ayanamsa;
        let from = Moment(gregorian::to_fixed(285, 1, 1).unwrap().0 as f64);
        let mesha = ingress_after(SiderealSign::MESHA, from);
        let anchor = Ayanamsa::REINGOLD_DERSHOWITZ.anchor_julian_date() - 1_721_424.5;
        assert!((mesha.0 - anchor).abs() * 86_400.0 < 1.0, "{}", mesha.0);
        assert_eq!(Ayanamsa::REINGOLD_DERSHOWITZ.degrees_at_anchor(), 0.0);
        // The book's own `ayanamsha`, Meeus's precession since then,
        // evaluated from its code: 23.863 933° on 1 January 2000 and
        // 24.213 224° on 1 January 2025. The IAU 2006 precession carries
        // the same zero to within 1.3″ of both.
        for (rd, book) in [(730_120.0, 23.863_933), (739_252.0, 24.213_224)] {
            let ours = Ayanamsa::REINGOLD_DERSHOWITZ.degrees_at(Moment(rd));
            assert!(((ours - book) * 3_600.0).abs() < 1.3, "{rd}: {ours}");
        }
    }
}
