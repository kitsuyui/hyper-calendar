//! The planetary hours: the twelve temporal hours of the daylight and the
//! twelve of the night, each ruled by one of the seven planets in the
//! Chaldean order.
//!
//! The day runs from sunrise to the next sunrise. Its first hour is ruled
//! by the planet of the weekday, and each hour after it by the next planet
//! of [`RulingPlanet::CHALDEAN_ORDER`], Saturn, Jupiter, Mars, the Sun,
//! Venus, Mercury, the Moon, and round again. Twenty-four hours move the
//! order on by three, so the first hour of the next day is ruled by the
//! next weekday's planet.
//!
//! al-Bīrūnī states the rule: "the first hour of the first day Sunday
//! should be given to the … sun. The second hour is allotted to the next
//! lower planet Venus, the third to Mercury, the fourth to the moon, the
//! fifth to Saturn and so on till the second day Monday whose first hour
//! falls to the moon, second to Saturn"; and the first hour of the night is
//! "the thirteenth planet counting downwards from the lord of the preceding
//! day" (*The Book of Instruction in the Elements of the Art of Astrology*,
//! tr. R. Ramsay Wright, 1934, §§390–391, `biruni-wright1934`, read
//! 2026-09-28). William Lilly's table for London divides the daylight and
//! the night into twelve equal hours each and works three hours of Monday
//! 15 March 1646, Old Style, through it (*Christian Astrology*, 1647, "A
//! Table whereby to find the Planetary hour" and its use,
//! `lilly-christian-astrology-1647`, read 2026-09-28). The hours here are
//! `hc-astro`'s temporal hours, [`hc_astro::solar_time::temporal_time`],
//! from sunrise to sunset and sunset to sunrise.
//!
//! Agrippa records a second way to divide the day, by fifteen degrees of
//! the ecliptic's oblique ascension to the hour (*Three Books of Occult
//! Philosophy*, 1651, Book II, chapter 34, `agrippa-1651`); it is not
//! carried. The Key of Solomon's table, which counts from midnight or from
//! sunset, is not carried either. Nothing here is an astrological claim.

use hc_astro::Location;
use hc_astro::solar_time::{MissingSolarEvent, temporal_time, universal_from_temporal_time};
use hc_calendar::fixed::Moment;
use hc_calendar::{Rd, Weekday};
use hc_core::math::floor;

use crate::zodiac::RulingPlanet;

/// The hours of a planetary day, twelve of daylight and twelve of night.
pub const HOURS_PER_DAY: u8 = 24;

/// Where in the Chaldean order the ruler of a weekday's first hour
/// stands: the Sun, fourth of Saturn, Jupiter, Mars, the Sun, for Sunday,
/// and three places on for each day after it.
const fn first_hour_index(weekday: Weekday) -> usize {
    (3 + 3 * weekday.sunday_first_number() as usize) % 7
}

/// The ruler of the `hour`th hour, 1 to 24 from sunrise, of a day that
/// begins on `weekday`: hours 1 to 12 are the daylight's, 13 to 24 the
/// night's.
#[must_use]
pub const fn ruler_of_hour(weekday: Weekday, hour: u8) -> Option<RulingPlanet> {
    if hour < 1 || hour > HOURS_PER_DAY {
        return None;
    }
    Some(RulingPlanet::CHALDEAN_ORDER[(first_hour_index(weekday) + hour as usize - 1) % 7])
}

/// The planet of a weekday, which rules its first hour.
#[must_use]
pub const fn ruler_of_day(weekday: Weekday) -> RulingPlanet {
    RulingPlanet::CHALDEAN_ORDER[first_hour_index(weekday)]
}

/// A planetary hour at a place.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlanetaryHour {
    /// The local date of the sunrise the planetary day begins at; the
    /// hours after midnight and before sunrise belong to the day before.
    pub day: Rd,
    /// The hour, 1 to 24 from sunrise: 1 to 12 of the daylight, 13 to 24
    /// of the night.
    pub hour: u8,
    /// The planet that rules it.
    pub ruler: RulingPlanet,
}

impl PlanetaryHour {
    /// Whether the hour is one of the daylight's.
    #[must_use]
    pub const fn is_daytime(&self) -> bool {
        self.hour <= 12
    }
}

/// The planetary hour at a Universal Time moment and a place.
///
/// # Errors
///
/// A [`MissingSolarEvent`] where a sunrise or sunset around the moment
/// does not happen, above the polar circles.
pub fn planetary_hour(
    universal: Moment,
    location: Location,
) -> Result<PlanetaryHour, MissingSolarEvent> {
    let temporal = temporal_time(universal, location)?;
    let local_day = temporal.day();
    // A temporal reading's hour is 6 at sunrise and 18 at sunset; one
    // before 6 is a night hour of the day before.
    let reading = temporal.day_fraction() * 24.0;
    let (day, since_sunrise) = if reading >= 6.0 {
        (local_day, reading - 6.0)
    } else {
        (local_day - 1, reading + 18.0)
    };
    let hour = (floor(since_sunrise) as i64).clamp(0, 23) as u8 + 1;
    let weekday = Weekday::from_rd(day);
    Ok(PlanetaryHour {
        day,
        hour,
        ruler: RulingPlanet::CHALDEAN_ORDER[(first_hour_index(weekday) + hour as usize - 1) % 7],
    })
}

/// The Universal Time at which the `hour`th planetary hour, 1 to 24, of
/// the day beginning at sunrise on `day` begins; hour 25 is the next
/// sunrise.
///
/// # Errors
///
/// A [`MissingSolarEvent`] where the sunrise or sunset it is counted from
/// does not happen. `None` for an hour outside 1 to 25.
pub fn planetary_hour_start(
    day: Rd,
    hour: u8,
    location: Location,
) -> Option<Result<Moment, MissingSolarEvent>> {
    if !(1..=HOURS_PER_DAY + 1).contains(&hour) {
        return None;
    }
    let reading = Moment(day.0 as f64 + (5.0 + f64::from(hour)) / 24.0);
    Some(universal_from_temporal_time(reading, location))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_astro::solar_time::universal_from_local_apparent_time;
    use hc_calendar::gregorian;

    /// Whether the rulers of `hours` of a day are `expected`, by name.
    fn rulers_are(
        weekday: Weekday,
        hours: core::ops::RangeInclusive<u8>,
        expected: &[&str],
    ) -> bool {
        hours
            .map(|hour| ruler_of_hour(weekday, hour).map(RulingPlanet::english_name))
            .eq(expected.iter().map(|&name| Some(name)))
    }

    /// al-Bīrūnī §390: Sunday's hours from the Sun, Monday's from the
    /// Moon; §391: the first hour of the night is the thirteenth planet
    /// from the lord of the day.
    #[test]
    fn al_birunis_rule() {
        assert!(rulers_are(
            Weekday::Sunday,
            1..=5,
            &["Sun", "Venus", "Mercury", "Moon", "Saturn"]
        ));
        assert!(rulers_are(Weekday::Monday, 1..=2, &["Moon", "Saturn"]));
        let week = [
            Weekday::Sunday,
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
        ];
        let lords = week.map(|day| ruler_of_day(day).english_name());
        assert_eq!(
            lords,
            [
                "Sun", "Moon", "Mars", "Mercury", "Jupiter", "Venus", "Saturn"
            ]
        );
        for (index, &day) in week.iter().enumerate() {
            let lord = ruler_of_day(day);
            let at = RulingPlanet::CHALDEAN_ORDER
                .iter()
                .position(|&planet| planet == lord)
                .expect("a planet of the order");
            // The thirteenth counting downwards, the lord of the day the first.
            assert_eq!(
                ruler_of_hour(day, 13),
                Some(RulingPlanet::CHALDEAN_ORDER[(at + 12) % 7])
            );
            // The hour after the twenty-fourth is the next day's first.
            assert_eq!(
                RulingPlanet::CHALDEAN_ORDER[(at + 24) % 7],
                ruler_of_day(week[(index + 1) % 7])
            );
        }
        assert_eq!(ruler_of_hour(Weekday::Monday, 0), None);
        assert_eq!(ruler_of_hour(Weekday::Monday, 25), None);
    }

    /// Lilly's three hours of Monday 15 March 1646, Old Style — 15 March
    /// 1647 in the Julian calendar with the year from 1 January, 25 March
    /// 1647 Gregorian — at London, in local apparent time: at 9:30 the
    /// fourth hour, Mars's, from 8:54 to 9:56; at 5:20 in the afternoon the
    /// twelfth, the Sun's, from 5:11 to 6:13; at 11:10 at night the
    /// eighteenth, Mars's again, from 11:02 to 12:00. His table has the Sun
    /// rise at 5:47 and set at 6:13; the hours here begin within three
    /// minutes of his.
    #[test]
    fn lillys_example() {
        let london = Location::new(51.5, -0.1, 0.0);
        let day = gregorian::to_fixed(1647, 3, 25).expect("exists");
        assert_eq!(Weekday::from_rd(day), Weekday::Monday);
        let apparent = |hour: u8, minute: u8| {
            universal_from_local_apparent_time(
                Moment(day.0 as f64 + (f64::from(hour) + f64::from(minute) / 60.0) / 24.0),
                london,
            )
        };
        for ((hour, minute), number, ruler) in [
            ((9, 30), 4, "Mars"),
            ((17, 20), 12, "Sun"),
            ((23, 10), 18, "Mars"),
        ] {
            let found = planetary_hour(apparent(hour, minute), london).expect("the Sun rises");
            assert_eq!((found.day, found.hour), (day, number), "{hour}:{minute}");
            assert_eq!(found.ruler.english_name(), ruler);
            assert_eq!(found.is_daytime(), number <= 12);
        }
        for (hour, (h, m)) in [
            (1, (5, 47)),
            (4, (8, 54)),
            (12, (17, 11)),
            (13, (18, 13)),
            (18, (23, 2)),
        ] {
            let start = planetary_hour_start(day, hour, london)
                .expect("an hour")
                .expect("the Sun rises");
            let minutes = (start.0 - apparent(h, m).0) * 1_440.0;
            assert!(minutes.abs() < 3.0, "hour {hour}: {minutes:.1} min");
        }
    }

    /// The small hours belong to the day before, whose night they end.
    #[test]
    fn the_small_hours_are_the_night_before() {
        let london = Location::new(51.5, -0.1, 0.0);
        let day = gregorian::to_fixed(1647, 3, 26).expect("exists");
        let found = planetary_hour(Moment(day.0 as f64 + 2.5 / 24.0), london).expect("valid");
        assert_eq!(found.day, day - 1);
        assert_eq!(found.hour, 21);
        // Monday's 21st hour: the Moon's order from 1, three past Mars's 18th.
        assert_eq!(
            found.ruler,
            ruler_of_hour(Weekday::Monday, 21).expect("an hour")
        );
        assert!(planetary_hour_start(day, 0, london).is_none());
        assert!(planetary_hour_start(day, 26, london).is_none());
        let next = planetary_hour_start(day - 1, 25, london)
            .expect("an hour")
            .expect("valid");
        let first = planetary_hour_start(day, 1, london)
            .expect("an hour")
            .expect("valid");
        assert!((next.0 - first.0).abs() < 1e-9);
    }

    /// Above the polar circle in summer there is no sunset.
    #[test]
    fn no_hours_without_a_sunset() {
        let tromso = Location::new(69.65, 18.96, 0.0);
        let day = gregorian::to_fixed(2024, 6, 21).expect("exists");
        assert!(planetary_hour(Moment(day.0 as f64 + 0.5), tromso).is_err());
    }
}
