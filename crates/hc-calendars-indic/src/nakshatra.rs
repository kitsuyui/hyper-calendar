//! The nakṣatra: the Moon's station among the twenty-seven, which one is
//! in progress at a moment, and when the Moon enters and leaves it.
//!
//! The sidereal ecliptic is cut into twenty-seven equal arcs of 13°20′
//! from the ayanamsa's zero point, Aśvinī first and Revatī last. The Moon
//! crosses one in about a day — a little under where its orbit is swift, a
//! little over where it is slow — so a nakṣatra, like a tithi, is held at
//! one or two sunrises or, now and then, at none. The almanacs print it
//! beside the tithi, and a few festivals are fixed by it rather than by a
//! tithi: Thaipusam on Puṣya in the month of Thai, Onam on Śravaṇa in
//! Chingam. `hc-holiday` reads those from here.

use hc_astro::lunar::lunar_longitude;
use hc_calendar::fixed::Moment;
use hc_core::math::{floor, normalize_degrees};
use hc_seasons::zodiac::Ayanamsa;

/// The nakṣatras in a sidereal revolution.
pub const NAKSHATRAS_PER_REVOLUTION: u8 = 27;

/// The arc of one nakṣatra: 13°20′.
pub const DEGREES_PER_NAKSHATRA: f64 = 360.0 / 27.0;

/// Aśvinī, the first.
pub const ASHVINI: u8 = 1;
/// Bharaṇī.
pub const BHARANI: u8 = 2;
/// Kṛttikā.
pub const KRITTIKA: u8 = 3;
/// Rohiṇī.
pub const ROHINI: u8 = 4;
/// Mṛgaśīrṣa.
pub const MRIGASHIRSHA: u8 = 5;
/// Ārdrā.
pub const ARDRA: u8 = 6;
/// Punarvasu.
pub const PUNARVASU: u8 = 7;
/// Puṣya — Tamil Pusam, the nakṣatra of Thaipusam.
pub const PUSHYA: u8 = 8;
/// Āśleṣā.
pub const ASHLESHA: u8 = 9;
/// Maghā.
pub const MAGHA: u8 = 10;
/// Pūrva Phalgunī.
pub const PURVA_PHALGUNI: u8 = 11;
/// Uttara Phalgunī.
pub const UTTARA_PHALGUNI: u8 = 12;
/// Hasta.
pub const HASTA: u8 = 13;
/// Citrā.
pub const CHITRA: u8 = 14;
/// Svātī.
pub const SVATI: u8 = 15;
/// Viśākhā.
pub const VISHAKHA: u8 = 16;
/// Anurādhā.
pub const ANURADHA: u8 = 17;
/// Jyeṣṭhā.
pub const JYESHTHA: u8 = 18;
/// Mūla.
pub const MULA: u8 = 19;
/// Pūrvāṣāḍhā.
pub const PURVA_ASHADHA: u8 = 20;
/// Uttarāṣāḍhā.
pub const UTTARA_ASHADHA: u8 = 21;
/// Śravaṇa — Malayalam Thiruvonam, the nakṣatra of Onam.
pub const SHRAVANA: u8 = 22;
/// Dhaniṣṭhā.
pub const DHANISHTHA: u8 = 23;
/// Śatabhiṣā.
pub const SHATABHISHA: u8 = 24;
/// Pūrva Bhādrapadā.
pub const PURVA_BHADRAPADA: u8 = 25;
/// Uttara Bhādrapadā.
pub const UTTARA_BHADRAPADA: u8 = 26;
/// Revatī, the last.
pub const REVATI: u8 = 27;

/// The Moon's sidereal longitude at a moment, in degrees from the
/// ayanamsa's zero point: its apparent longitude less the ayanamsa.
#[must_use]
pub fn sidereal_lunar_longitude(moment: Moment, ayanamsa: Ayanamsa) -> f64 {
    normalize_degrees(lunar_longitude(moment) - ayanamsa.degrees_at(moment))
}

/// The nakṣatra in progress at a moment, 1 for Aśvinī through 27 for
/// Revatī.
#[must_use]
pub fn nakshatra_at(moment: Moment, ayanamsa: Ayanamsa) -> u8 {
    let arc = floor(sidereal_lunar_longitude(moment, ayanamsa) / DEGREES_PER_NAKSHATRA);
    // Rounding can put the longitude a hair past the last arc's end.
    (arc as u8).min(NAKSHATRAS_PER_REVOLUTION - 1) + 1
}

/// The first moment at or after `moment` when the Moon's sidereal
/// longitude reaches `degrees`.
#[must_use]
pub fn sidereal_lunar_longitude_at_or_after(
    degrees: f64,
    moment: Moment,
    ayanamsa: Ayanamsa,
) -> Moment {
    // What is left to travel, in [0, 360). The Moon never turns back and
    // never covers more than about 15.5° in a day, so this falls with time
    // and jumps by a revolution at the crossing.
    let to_go = |at: Moment| normalize_degrees(degrees - sidereal_lunar_longitude(at, ayanamsa));
    const STEP: f64 = 0.5;
    let mut low = moment;
    let mut left = to_go(low);
    let mut high = Moment(low.0 + STEP);
    let mut ahead = to_go(high);
    while ahead < left {
        low = high;
        left = ahead;
        high = Moment(high.0 + STEP);
        ahead = to_go(high);
    }
    // The crossing lies between `low` and `high`: before it less than a
    // step's travel is left, after it nearly a revolution.
    for _ in 0..40 {
        let mid = Moment(f64::midpoint(low.0, high.0));
        if to_go(mid) < 180.0 {
            low = mid;
        } else {
            high = mid;
        }
    }
    high
}

/// The Moon's stay in a nakṣatra: when it enters and when it leaves. The
/// stay in progress at `moment`, or the next one when the Moon is
/// elsewhere then. A `nakshatra` outside 1 through 27 is clamped.
#[must_use]
pub fn nakshatra_span(nakshatra: u8, moment: Moment, ayanamsa: Ayanamsa) -> (Moment, Moment) {
    let nakshatra = nakshatra.clamp(ASHVINI, REVATI);
    let entered_at = f64::from(nakshatra - 1) * DEGREES_PER_NAKSHATRA;
    let left_at = normalize_degrees(entered_at + DEGREES_PER_NAKSHATRA);
    // A stay lasts under a day and a quarter, so one in progress began
    // within that of `moment`.
    let from = if nakshatra_at(moment, ayanamsa) == nakshatra {
        Moment(moment.0 - 1.25)
    } else {
        moment
    };
    let entry = sidereal_lunar_longitude_at_or_after(entered_at, from, ayanamsa);
    let exit = sidereal_lunar_longitude_at_or_after(left_at, entry, ayanamsa);
    (entry, exit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    fn at(year: i64, month: u8, day: u8, hour: f64, minute: f64) -> Moment {
        let day = gregorian::to_fixed(year, month, day).unwrap();
        Moment(day.0 as f64 + (hour + minute / 60.0) / 24.0)
    }

    const THREE_MINUTES: f64 = 3.0 / 1440.0;

    #[test]
    fn pushya_in_january_2024_begins_and_ends_when_the_almanac_says() {
        // Drik Panchang, Thaipusam 2024 for Chennai: Poosam nakṣatra from
        // 08:16 IST on 25 January to 10:28 IST on the 26th, that is 02:46
        // to 04:58 UT.
        let (entry, exit) = nakshatra_span(PUSHYA, at(2024, 1, 25, 12.0, 0.0), Ayanamsa::LAHIRI);
        assert!((entry.0 - at(2024, 1, 25, 2.0, 46.0).0).abs() < THREE_MINUTES);
        assert!((exit.0 - at(2024, 1, 26, 4.0, 58.0).0).abs() < THREE_MINUTES);
        // The same stay from a moment before it, and the one after from a
        // moment after it.
        let (again, _) = nakshatra_span(PUSHYA, at(2024, 1, 24, 12.0, 0.0), Ayanamsa::LAHIRI);
        assert!((again.0 - entry.0).abs() < 1e-6);
        let (next, _) = nakshatra_span(PUSHYA, at(2024, 1, 27, 0.0, 0.0), Ayanamsa::LAHIRI);
        assert!(next.0 - entry.0 > 26.0 && next.0 - entry.0 < 28.5);
    }

    #[test]
    fn pushya_in_february_2025_too() {
        // 18:01 IST on 10 February to 18:34 IST on the 11th: 12:31 to
        // 13:04 UT.
        let (entry, exit) = nakshatra_span(PUSHYA, at(2025, 2, 11, 0.0, 0.0), Ayanamsa::LAHIRI);
        assert!((entry.0 - at(2025, 2, 10, 12.0, 31.0).0).abs() < THREE_MINUTES);
        assert!((exit.0 - at(2025, 2, 11, 13.0, 4.0).0).abs() < THREE_MINUTES);
    }

    #[test]
    fn the_nakshatra_at_a_moment_agrees_with_the_stays() {
        let (entry, exit) = nakshatra_span(PUSHYA, at(2024, 1, 25, 12.0, 0.0), Ayanamsa::LAHIRI);
        assert_eq!(
            nakshatra_at(Moment(entry.0 - 0.01), Ayanamsa::LAHIRI),
            PUNARVASU
        );
        assert_eq!(
            nakshatra_at(Moment(entry.0 + 0.01), Ayanamsa::LAHIRI),
            PUSHYA
        );
        assert_eq!(
            nakshatra_at(Moment(exit.0 - 0.01), Ayanamsa::LAHIRI),
            PUSHYA
        );
        assert_eq!(
            nakshatra_at(Moment(exit.0 + 0.01), Ayanamsa::LAHIRI),
            ASHLESHA
        );
    }

    #[test]
    fn revati_hands_over_to_ashvini_without_a_gap() {
        let start = at(2024, 3, 1, 0.0, 0.0);
        let (_, exit) = nakshatra_span(REVATI, start, Ayanamsa::LAHIRI);
        let (entry, _) = nakshatra_span(ASHVINI, Moment(exit.0 + 0.001), Ayanamsa::LAHIRI);
        assert!((entry.0 - exit.0).abs() < 1e-6, "{} vs {}", entry.0, exit.0);
        assert_eq!(
            nakshatra_at(Moment(exit.0 + 0.001), Ayanamsa::LAHIRI),
            ASHVINI
        );
    }

    #[test]
    fn a_stay_lasts_about_a_day() {
        let mut moment = at(2024, 1, 1, 0.0, 0.0);
        for _ in 0..30 {
            let (entry, exit) = nakshatra_span(
                nakshatra_at(moment, Ayanamsa::LAHIRI),
                moment,
                Ayanamsa::LAHIRI,
            );
            let length = exit.0 - entry.0;
            assert!((0.85..1.2).contains(&length), "{length}");
            assert!(entry.0 <= moment.0 && moment.0 < exit.0);
            moment = Moment(exit.0 + 0.001);
        }
    }
}
