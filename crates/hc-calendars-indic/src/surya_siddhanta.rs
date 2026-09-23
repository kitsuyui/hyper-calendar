//! The Sun of the *Sūrya Siddhānta*: the classical model whose saṅkrāntis
//! the traditional almanacs still keep.
//!
//! The Siddhānta moves a mean Sun uniformly round a sidereal zodiac and
//! corrects it by an epicycle whose size shrinks with the anomaly, and it
//! reads its sines from a table of twenty-four values at steps of 225
//! minutes of arc, interpolating between them. Its zodiac is sidereal by
//! construction, so there is no ayanamsa: the zero point is where the
//! model puts it, and it is not the zero point of any modern ayanamsa.
//! Its saṅkrāntis therefore fall hours from the modern ones — the Meṣa
//! saṅkrānti of 2024 two hours and nineteen minutes after the Lahiri one.
//!
//! # Whose arithmetic
//!
//! The functions of Reingold and Dershowitz, *Calendrical Calculations*
//! (4th ed., 2018), in the section on the modern Hindu calendars:
//! `hindu-sine-table`, `hindu-sine`, `hindu-arcsin`,
//! `hindu-mean-position`, `hindu-true-position` and
//! `hindu-solar-longitude`, with the sidereal and anomalistic years, the
//! epicycle of 14⁄360 shrinking by 1⁄42, and the table's rounding
//! correction of 0.215 as they give them. The constants and the order of
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
//! which is exactly 0.785 75.
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

use hc_calendar::fixed::Moment;
use hc_core::math::{abs, ceil, floor, round, sin_deg};
use hc_seasons::zodiac::SiderealSign;

/// The sidereal year: 1 577 917 828 days in a *mahāyuga* of 4 320 000
/// years, 365.258 756 days.
pub const SIDEREAL_YEAR: f64 = 1_577_917_828.0 / 4_320_000.0;

/// The anomalistic year: the same days in 4 320 000 000 − 387 revolutions
/// of the apsis, 365.258 789 days.
pub const ANOMALISTIC_YEAR: f64 = 1_577_917_828_000.0 / 4_319_999_613.0;

/// The Kali Yuga epoch, Friday 18 February 3102 BCE in the Julian
/// calendar, as a fixed day number (*Calendrical Calculations*,
/// `hindu-epoch`).
const EPOCH: f64 = -1_132_959.0;

/// The fraction of an anomalistic revolution between the creation and the
/// epoch: 1 955 880 000 sidereal years are 1 955 880 000 × (1 −
/// 387⁄4 320 000 000) anomalistic years, 1 955 879 824.785 75.
const ANOMALY_AT_EPOCH: f64 = 0.785_75;

/// Ujjain's longitude as the book gives it (`ujjain`), in degrees east.
pub const UJJAIN_LONGITUDE_DEGREES: f64 = 75.0 + 46.0 / 60.0 + 6.0 / 3_600.0;

/// The step of the sine table, 225 minutes of arc, in degrees.
const SINE_STEP_DEGREES: f64 = 225.0 / 60.0;

/// The radius the table's sines are measured in, 3438 minutes of arc.
const RADIUS: f64 = 3_438.0;

/// `x mod 1`, into `[0, 1)`.
fn fraction(x: f64) -> f64 {
    x - floor(x)
}

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
    let part = fraction(entry);
    part * sine_table(ceil(entry)) + (1.0 - part) * sine_table(floor(entry))
}

/// The angle in degrees whose table sine is `amplitude`, the inverse of
/// [`sine`] (`hindu-arcsin`).
fn arcsin(amplitude: f64) -> f64 {
    if amplitude < 0.0 {
        return -arcsin(-amplitude);
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
    longitude - 360.0 * floor(longitude / 360.0)
}

/// The Sun's sidereal longitude in degrees at a moment in Universal Time
/// (`hindu-solar-longitude`).
#[must_use]
pub fn solar_longitude(moment: Moment) -> f64 {
    let days = moment.0 + UJJAIN_LONGITUDE_DEGREES / 360.0 - EPOCH;
    let mean = fraction(days / SIDEREAL_YEAR);
    let anomaly = fraction(days / ANOMALISTIC_YEAR + ANOMALY_AT_EPOCH);
    true_position(mean, anomaly, 14.0 / 360.0, 1.0 / 42.0)
}

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
        gap - 360.0 * floor(gap / 360.0)
    };
    // Carrying the Sun from where it stands at the mean rate misplaces the
    // entry by however much the equation of centre changes on the way,
    // which is never more than twice its greatest value of 2.18°: under
    // four and a half degrees, and so under five days at the Sun's slowest.
    let estimate = moment.0 + ahead(moment.0) / 360.0 * SIDEREAL_YEAR;
    let mut low = (estimate - 6.0).max(moment.0);
    let mut high = estimate + 6.0;
    // Bisect on whether the Sun has passed the target: `ahead` is small
    // just before the entry and close to 360 just after it.
    for _ in 0..64 {
        let middle = (low + high) / 2.0;
        if ahead(middle) >= 180.0 {
            high = middle;
        } else {
            low = middle;
        }
    }
    Moment(high)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    fn ymd(year: i64, month: u8, day: u8) -> f64 {
        gregorian::to_fixed(year, month, day).unwrap().0 as f64
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
            let days = ymd(2024, 1, 1) + f64::from(day) + UJJAIN_LONGITUDE_DEGREES / 360.0 - EPOCH;
            let mean = 360.0 * fraction(days / SIDEREAL_YEAR);
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
}
