//! 黄道十二宮 — the ecliptic cut into twelve, in the three traditions that
//! cut it differently.
//!
//! This crate already computes [`crate::solar_terms`] as the instants the
//! Sun's apparent longitude reaches a multiple of 15°. A zodiac sign is the
//! same computation at a multiple of 30°, so nothing new is needed but the
//! naming and the question of *where you measure from*. Three answers are
//! shipped here, and they are 15° and 24° apart from one another:
//!
//! | Division | Zero point | Boundaries in this crate's terms |
//! | --- | --- | --- |
//! | [`tropical`] | the March equinox | the twelve 中気, exactly |
//! | [`sidereal`] | the fixed stars, an *ayanamsa* behind the equinox | ~24° later than the tropical ones |
//! | [`chinese_twelve`] | 大雪 at 255° | the twelve 節気, exactly |
//!
//! The first row is the reason this module belongs in `hc-seasons` rather
//! than anywhere else: the tropical sign boundaries **are** the 中気, the
//! principal solar terms, the same twelve instants the lunisolar leap-month
//! rule counts. Aries opens at 春分, Cancer at 夏至, Libra at 秋分, Capricorn
//! at 冬至. A test asserts it rather than a comment claiming it.
//!
//! The third row is the same statement shifted by half a sign: 十二次 runs
//! 節気 to 節気, so each 次 is a tropical sign rotated 15° backwards and
//! contains exactly one 中気 in its middle.
//!
//! # Why the sidereal one is here too
//!
//! A library that shipped only the tropical zodiac would be taking a side
//! without saying so. Indian (Vedic) astrology measures the same twelve
//! divisions from the fixed stars, not from the equinox, and precession has
//! pulled the two about 24° apart since they last coincided. The practical
//! consequence is blunt: **for four days in five the same moment is in
//! different signs under the two systems** — 24° of 30° — and the Indian
//! solar calendars of Tamil Nadu, Bengal and Kerala take their months from
//! the sidereal answer. See [`rashi`].
//!
//! # Where the Chinese zodiac *animal* is, and it is not here
//!
//! [`chinese_twelve`] is 十二次, a division of the ecliptic. The twelve
//! animals — rat, ox, tiger — are the twelve 地支 (earthly branches) of the
//! sexagenary cycle applied to a *year*, which is a counting cycle and not an
//! angle. That lives in [`hc_calendar::cycle`]: see
//! [`hc_calendar::cycle::sexagenary_year`] and
//! [`hc_calendar::cycle::ZODIAC_ANIMALS`]. Nothing here duplicates it.
//!
//! # Accuracy: a boundary near midnight can land on the wrong day
//!
//! Every instant here comes from `hc-astro`'s Meeus low-precision solar
//! longitude, good to about 0.01° — roughly a quarter of an hour of the
//! Sun's motion — with a measured systematic bias of about **−4.5 minutes**.
//! A sign boundary falling within about ten minutes of local midnight can
//! therefore be given the wrong *day*. That is a property of the series, not
//! a bug, and it is the same caveat the solar terms carry.
//!
//! The sidereal boundaries carry one more term of uncertainty on top: the
//! published Lahiri ayanamsa values disagree among themselves by a few tens
//! of arcseconds, and 20″ of solar longitude is about eight minutes of time.
//! [`sidereal::Ayanamsa`] documents which parameterisation is used.
//!
//! # The conventional dates are not these dates
//!
//! Newspaper astrology columns print fixed dates — "Aries: March 21 –
//! April 19" — that have not been recomputed since roughly the 1920s. The
//! equinox has drifted since. [`TropicalSign::conventional_period`] ships
//! those fixed dates as data so that the disagreement can be measured;
//! `tests/zodiac_conventional_dates.rs` measures it and prints the table.
//! The drift is real, and it is currently about a day.

pub mod chinese_twelve;
pub mod rashi;
pub mod sidereal;
pub mod tropical;

pub use chinese_twelve::ChineseStation;
pub use rashi::{
    BENGALI, MALAYALAM, Rashi, SANSKRIT, SOLAR_MONTH_TRADITIONS, SolarMonthTradition, TAMIL,
};
pub use sidereal::{Ayanamsa, SiderealSign};
pub use tropical::{ConventionalPeriod, Element, Modality, RulingPlanet, TropicalSign};

use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::normalize_degrees;

/// How many degrees of ecliptic longitude one sign spans.
///
/// Twice [`crate::solar_terms::DEGREES_PER_TERM`], which is the whole
/// relationship between this module and the solar terms.
pub const DEGREES_PER_SIGN: f64 = 30.0;

/// How many signs make up a zodiac.
pub const SIGNS_PER_ZODIAC: usize = 12;

/// The stretch of time one sign occupies: two instants and two days.
///
/// Both are carried because they answer different questions. The instants
/// are what the astronomy computes and what a comparison against a published
/// ephemeris wants; the days are what a calendar prints, and they depend on
/// the [`crate::Meridian`] the period was asked for.
///
/// `end` is the instant the *next* sign begins, so consecutive periods share
/// it exactly and cannot leave a gap. `end_day` is the day *before* the next
/// sign's `start_day`, so consecutive periods' days do not overlap either.
/// A sign therefore covers `start_day ..= end_day` inclusive.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SignPeriod<S> {
    /// Which sign.
    pub sign: S,
    /// The Universal Time instant the Sun entered the sign.
    pub start: Moment,
    /// The Universal Time instant the Sun left it, i.e. entered the next.
    pub end: Moment,
    /// The first day of the sign, at the meridian it was asked for.
    pub start_day: Rd,
    /// The last day of the sign, at the same meridian.
    pub end_day: Rd,
}

impl<S> SignPeriod<S> {
    /// How many whole days the sign covers, both endpoints included.
    ///
    /// Between 29 and 32. The spread is real: the Sun crosses 30° of ecliptic
    /// in about 29.4 days near perihelion in January and about 31.5 days near
    /// aphelion in July, because the Earth's orbit is an ellipse. Sagittarius
    /// and Capricorn are the short signs; Gemini and Cancer the long ones.
    #[must_use]
    pub const fn length_days(&self) -> i64 {
        self.end_day.0 - self.start_day.0 + 1
    }

    /// How long the sign lasted as an exact interval, in days.
    #[must_use]
    pub fn duration_days(&self) -> f64 {
        self.end.0 - self.start.0
    }

    /// Whether a day falls inside the period.
    #[must_use]
    pub const fn contains(&self, day: Rd) -> bool {
        day.0 >= self.start_day.0 && day.0 <= self.end_day.0
    }

    /// Whether an instant falls inside the period.
    ///
    /// Half-open at the top: the instant a sign ends is the instant the next
    /// one begins, and it belongs to the next.
    #[must_use]
    pub fn contains_moment(&self, moment: Moment) -> bool {
        moment.0 >= self.start.0 && moment.0 < self.end.0
    }
}

/// How far a longitude has travelled into an arc that begins at
/// `arc_start_degrees`, reduced to 0°–360°.
///
/// Shared by all three divisions because "degrees into the sign" is the same
/// subtraction whichever zero point the sign was measured from.
pub(crate) fn degrees_into_arc(longitude_degrees: f64, arc_start_degrees: f64) -> f64 {
    normalize_degrees(longitude_degrees - arc_start_degrees)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Meridian;

    #[test]
    fn a_sign_spans_twice_a_solar_term() {
        assert!((DEGREES_PER_SIGN - 2.0 * crate::solar_terms::DEGREES_PER_TERM).abs() < 1e-12);
        assert_eq!(SIGNS_PER_ZODIAC * 2, crate::solar_terms::TERMS_PER_YEAR);
    }

    #[test]
    fn degrees_into_an_arc_wrap_the_short_way_round() {
        assert!((degrees_into_arc(10.0, 0.0) - 10.0).abs() < 1e-12);
        assert!((degrees_into_arc(5.0, 350.0) - 15.0).abs() < 1e-12);
        assert!((degrees_into_arc(350.0, 5.0) - 345.0).abs() < 1e-12);
        assert!(degrees_into_arc(0.0, 0.0).abs() < 1e-12);
    }

    /// The period type is shared by all three divisions, so its day and
    /// instant contracts are tested once, here, on a hand-built value.
    #[test]
    fn a_period_contains_its_own_days_and_nothing_beyond_them() {
        let period = SignPeriod {
            sign: TropicalSign::ARIES,
            start: Moment(739_000.25),
            end: Moment(739_030.75),
            start_day: Rd(739_000),
            end_day: Rd(739_030),
        };
        assert_eq!(period.length_days(), 31);
        assert!((period.duration_days() - 30.5).abs() < 1e-9);
        assert!(period.contains(Rd(739_000)));
        assert!(period.contains(Rd(739_030)));
        assert!(!period.contains(Rd(738_999)));
        assert!(!period.contains(Rd(739_031)));
        assert!(period.contains_moment(Moment(739_000.25)));
        assert!(period.contains_moment(Moment(739_030.0)));
        // Half-open at the top: the end instant belongs to the next sign.
        assert!(!period.contains_moment(Moment(739_030.75)));
        assert!(!period.contains_moment(Moment(739_000.0)));
    }

    /// The three divisions are three answers to one question, and on an
    /// ordinary day in the modern era all three disagree: the Sun is in the
    /// tropical sign one place ahead of the sidereal one, and in a 次 that
    /// straddles them.
    #[test]
    fn the_three_divisions_give_three_different_answers_for_one_day() {
        // 2024-05-10, chosen because it is well away from every boundary.
        let day = Rd(739_016);
        let japan = Meridian::JAPAN;
        assert_eq!(tropical::sign_on_day(day, japan), TropicalSign::TAURUS);
        assert_eq!(
            sidereal::sign_on_day(day, Ayanamsa::LAHIRI, japan),
            SiderealSign::MESHA
        );
        assert_eq!(
            chinese_twelve::station_on_day(day, japan),
            ChineseStation::SHICHEN
        );
        // The sidereal sign is the tropical one's predecessor here, which is
        // what a 24° ayanamsa does for most of a 30° sign.
        assert_eq!(
            sidereal::sign_on_day(day, Ayanamsa::LAHIRI, japan).tropical_counterpart(),
            TropicalSign::TAURUS.previous()
        );
    }
}
