//! The meridian a seasonal *day* is measured at.
//!
//! Every event in this crate is an astronomical instant, and `hc-astro`
//! returns those in Universal Time. A calendar, though, wants a day, and a
//! day starts at midnight *somewhere*. That "somewhere" is not a detail: the
//! Chinese and Japanese calendars disagree about the date of a solar term
//! several times a century purely because Beijing is an hour behind Tokyo,
//! and the same instant therefore falls on two different dates.
//!
//! So nothing in this crate silently assumes UT. Every function that returns
//! an [`Rd`] takes a `Meridian`.
//!
//! This is *not* a time-zone database. A `Meridian` is a fixed offset, with
//! no daylight saving and no history of political changes; `hc-tz` is where
//! that lives. Japan has had no summer time since 1951 and China none since
//! 1991, so for the almanacs this crate reproduces a fixed offset is the
//! right model — but for a general civil date it is not.

use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::{floor, round};

/// Seconds in a day, as the astronomical series count them.
const SECONDS_PER_DAY: f64 = 86_400.0;

/// A fixed offset from Universal Time, used to decide which day an instant
/// falls on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Meridian {
    /// East of Greenwich is positive, in whole seconds.
    offset_seconds: i32,
}

impl Meridian {
    /// Greenwich: the day boundary of Universal Time itself.
    pub const UNIVERSAL: Self = Self::from_seconds(0);

    /// Japan Standard Time, UTC+9, the 135°E meridian.
    ///
    /// This is the meridian the National Astronomical Observatory of Japan
    /// publishes the 暦要項 (*Calendar Essentials*) at, so it is the one to
    /// use for 春分の日, the 雑節 and the Japanese solar-term dates.
    pub const JAPAN: Self = Self::from_seconds(9 * 3_600);

    /// China Standard Time, UTC+8, the 120°E meridian.
    ///
    /// The modern Chinese calendar has been computed at this meridian since
    /// 1929; before that see [`Meridian::CHINA_BEFORE_1929`].
    pub const CHINA: Self = Self::from_seconds(8 * 3_600);

    /// Korea Standard Time, UTC+9, the meridian the Dangi calendar uses.
    pub const KOREA: Self = Self::from_seconds(9 * 3_600);

    /// India Standard Time, UTC+5:30, the 82°30′E meridian.
    ///
    /// Unusually, India's civil offset *is* its reference meridian's local
    /// mean time exactly: 82°30′ is 5½ hours east of Greenwich to the second.
    /// The Calendar Reform Committee of 1955 fixed this meridian for the
    /// national calendar, and the *Indian Astronomical Ephemeris* computes
    /// the saṅkrānti — the Sun's entries into the sidereal signs — at it, so
    /// it is the meridian for [`crate::zodiac::rashi`].
    pub const INDIA: Self = Self::from_seconds(5 * 3_600 + 1_800);

    /// Beijing local mean time, 116°25′E, i.e. UTC+7:45:40.
    ///
    /// Chinese calendar dates before the 1929 switch to the 120° standard
    /// were computed at the capital's own meridian; Reingold & Dershowitz,
    /// *Calendrical Calculations*, 4th ed., §19.1 uses 1397/180 hours for it.
    pub const CHINA_BEFORE_1929: Self = Self::from_seconds(27_940);

    /// A meridian from a whole number of seconds east of Greenwich.
    #[must_use]
    pub const fn from_seconds(offset_seconds: i32) -> Self {
        Self { offset_seconds }
    }

    /// A meridian from an offset in hours, rounded to the nearest second.
    #[must_use]
    pub fn from_hours(offset_hours: f64) -> Self {
        Self::from_seconds(round(offset_hours * 3_600.0) as i32)
    }

    /// A meridian from a terrestrial longitude in degrees east, converted at
    /// the nominal 15° per hour.
    ///
    /// This gives *local mean solar time*, which is what a pre-standard-time
    /// almanac was computed at. It is not any country's civil clock.
    #[must_use]
    pub fn from_longitude_degrees(longitude_east: f64) -> Self {
        Self::from_hours(longitude_east / 15.0)
    }

    /// The offset east of Greenwich in whole seconds.
    #[must_use]
    pub const fn offset_seconds(self) -> i32 {
        self.offset_seconds
    }

    /// The offset east of Greenwich in hours.
    #[must_use]
    pub fn offset_hours(self) -> f64 {
        f64::from(self.offset_seconds) / 3_600.0
    }

    /// The offset east of Greenwich as a fraction of a day.
    #[must_use]
    pub fn offset_days(self) -> f64 {
        f64::from(self.offset_seconds) / SECONDS_PER_DAY
    }

    /// A Universal Time moment read as a local one.
    #[must_use]
    pub fn local(self, universal: Moment) -> Moment {
        Moment(universal.0 + self.offset_days())
    }

    /// A local moment read as a Universal Time one.
    #[must_use]
    pub fn universal(self, local: Moment) -> Moment {
        Moment(local.0 - self.offset_days())
    }

    /// The local day on which a Universal Time instant falls.
    ///
    /// This is the single place where a seasonal instant becomes a calendar
    /// date, and therefore the single place where `hc-astro`'s roughly
    /// −4.5-minute bias in solar longitude can change an answer. See the
    /// crate README.
    #[must_use]
    pub fn day_of(self, universal: Moment) -> Rd {
        self.local(universal).day()
    }

    /// The Universal Time moment of local midnight beginning a day.
    #[must_use]
    pub fn midnight(self, day: Rd) -> Moment {
        self.universal(Moment(day.0 as f64))
    }

    /// The Universal Time moment of local noon on a day.
    ///
    /// Moon age and illuminated fraction are conventionally quoted for local
    /// noon, so that a single number describes the whole day.
    #[must_use]
    pub fn noon(self, day: Rd) -> Moment {
        self.universal(Moment(day.0 as f64 + 0.5))
    }

    /// The local time of day of a Universal Time instant, in hours.
    #[must_use]
    pub fn local_hours(self, universal: Moment) -> f64 {
        let local = self.local(universal).0;
        (local - floor(local)) * 24.0
    }
}

impl Default for Meridian {
    /// Greenwich, because a library that guessed a country would be worse
    /// than one that made the caller say.
    fn default() -> Self {
        Self::UNIVERSAL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_named_meridians_have_the_offsets_they_claim() {
        assert_eq!(Meridian::UNIVERSAL.offset_seconds(), 0);
        assert_eq!(Meridian::JAPAN.offset_seconds(), 32_400);
        assert_eq!(Meridian::CHINA.offset_seconds(), 28_800);
        assert_eq!(Meridian::KOREA, Meridian::JAPAN);
        assert_eq!(Meridian::INDIA.offset_seconds(), 19_800);
        assert!((Meridian::JAPAN.offset_hours() - 9.0).abs() < 1e-12);
        assert!((Meridian::CHINA.offset_days() - 1.0 / 3.0).abs() < 1e-12);
    }

    /// India's civil offset is its reference meridian's mean solar time to
    /// the second, which is not true of most countries.
    #[test]
    fn the_indian_meridian_is_exactly_five_and_a_half_hours_east() {
        assert_eq!(Meridian::from_longitude_degrees(82.5), Meridian::INDIA);
        assert!((Meridian::INDIA.offset_hours() - 5.5).abs() < 1e-12);
    }

    /// 1397/180 hours is the Beijing local mean time offset used for Chinese
    /// dates before 1929.
    #[test]
    fn the_pre_1929_chinese_meridian_is_beijing_local_mean_time() {
        let expected = 1397.0 / 180.0 * 3_600.0;
        assert!(
            (f64::from(Meridian::CHINA_BEFORE_1929.offset_seconds()) - expected).abs() < 1.0,
            "offset was {}",
            Meridian::CHINA_BEFORE_1929.offset_seconds()
        );
    }

    #[test]
    fn a_meridian_from_longitude_is_local_mean_solar_time() {
        assert_eq!(Meridian::from_longitude_degrees(135.0), Meridian::JAPAN);
        assert_eq!(Meridian::from_longitude_degrees(120.0), Meridian::CHINA);
        assert_eq!(Meridian::from_hours(-5.0).offset_seconds(), -18_000);
    }

    #[test]
    fn converting_to_local_and_back_is_the_identity() {
        for meridian in [
            Meridian::UNIVERSAL,
            Meridian::JAPAN,
            Meridian::CHINA,
            Meridian::CHINA_BEFORE_1929,
            Meridian::from_hours(-11.5),
        ] {
            let moment = Moment(739_000.123_456);
            let round_tripped = meridian.universal(meridian.local(moment));
            assert!((round_tripped.0 - moment.0).abs() < 1e-9);
        }
    }

    /// An instant at 16:00 UT is already the next day in Tokyo and not yet in
    /// Beijing — the whole reason this type exists.
    #[test]
    fn one_instant_falls_on_two_days_at_two_meridians() {
        let moment = Moment(739_000.0 + 16.0 / 24.0);
        assert_eq!(Meridian::UNIVERSAL.day_of(moment), Rd(739_000));
        assert_eq!(Meridian::CHINA.day_of(moment), Rd(739_001));
        assert_eq!(Meridian::JAPAN.day_of(moment), Rd(739_001));

        let earlier = Moment(739_000.0 + 15.5 / 24.0);
        assert_eq!(Meridian::CHINA.day_of(earlier), Rd(739_000));
        assert_eq!(Meridian::JAPAN.day_of(earlier), Rd(739_001));
    }

    #[test]
    fn midnight_and_noon_bracket_the_local_day() {
        let day = Rd(739_000);
        let meridian = Meridian::JAPAN;
        assert_eq!(meridian.day_of(meridian.midnight(day)), day);
        assert_eq!(meridian.day_of(meridian.noon(day)), day);
        assert!((meridian.local_hours(meridian.noon(day)) - 12.0).abs() < 1e-6);
        assert!(meridian.local_hours(meridian.midnight(day)) < 1e-6);
    }

    #[test]
    fn the_default_meridian_is_greenwich() {
        assert_eq!(Meridian::default(), Meridian::UNIVERSAL);
    }
}
