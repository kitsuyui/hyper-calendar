//! 손 없는 날 — the Korean days without *son*.
//!
//! *Son* (손) is a spirit that goes about hindering people's work and
//! harming them. On the lunar days ending in 9 and 0 — the 9th, 10th, 19th,
//! 20th, 29th and 30th of any month — it is abroad in none of the eight
//! directions, and those days are chosen for moving house, weddings,
//! building work and opening a business (Wikipedia (ko), 「손 없는 날」,
//! `wikipedia-ko-son-eomneun-nal`). The lunar date is the Korean calendar's,
//! `dangi`. `docs/systems/east-asian-folk-days.md` describes the system and
//! its tests.
//!
//! The rule is by the day's number alone, so the days of a leap month are
//! the same. Where *son* is on the other days is not carried: the source
//! gives no table of it.

use hc_calendar::Rd;
use hc_calendars_lunar::dangi;

/// The Korean name, `"손 없는 날"`.
pub const KOREAN_NAME: &str = "손 없는 날";

/// Whether a lunar day number is one without *son*: 9, 10, 19, 20, 29 or
/// 30.
#[must_use]
pub const fn is_day_number_without_son(day: u8) -> bool {
    matches!(day, 9 | 10 | 19 | 20 | 29 | 30)
}

/// Whether a day is 손 없는 날 on the Korean lunar calendar, or `None`
/// outside the years `dangi` converts, 1645 to 2150.
#[must_use]
pub fn is_day_without_son(day: Rd) -> Option<bool> {
    let (_, _, lunar_day) = dangi::PARAMETERS.from_fixed(day).ok()?;
    Some(is_day_number_without_son(lunar_day))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::gregorian;

    /// superkts.com, 「2026년 손없는 날」 (`superkts-son-2026`), retrieved
    /// 2026-09-27: the 68 days of 2026, from 7 January (음력 2025-11-19) to
    /// 28 December (음력 11-20).
    #[test]
    fn the_sixty_eight_days_of_2026_are_the_published_list() {
        let published: &[(u8, &[u8])] = &[
            (1, &[7, 8, 17, 18, 27, 28]),
            (2, &[6, 7, 16, 25, 26]),
            (3, &[7, 8, 17, 18, 27, 28]),
            (4, &[6, 7, 16, 25, 26]),
            (5, &[5, 6, 15, 16, 25, 26]),
            (6, &[4, 5, 14, 23, 24]),
            (7, &[3, 4, 13, 22, 23]),
            (8, &[1, 2, 11, 12, 21, 22, 31]),
            (9, &[1, 10, 19, 20, 29, 30]),
            (10, &[9, 10, 19, 20, 29, 30]),
            (11, &[8, 17, 18, 27, 28]),
            (12, &[7, 8, 17, 18, 27, 28]),
        ];
        let mut expected = Vec::new();
        for (month, days) in published {
            for day in *days {
                expected.push(gregorian::to_fixed(2026, *month, *day).expect("a date"));
            }
        }
        assert_eq!(expected.len(), 68);
        let first = gregorian::to_fixed(2026, 1, 1).expect("a date");
        let found: Vec<Rd> = (0..365)
            .map(|offset| Rd(first.0 + offset))
            .filter(|day| is_day_without_son(*day) == Some(true))
            .collect();
        assert_eq!(found, expected);
    }

    #[test]
    fn the_rule_is_the_day_number_and_the_calendar_has_a_range() {
        let numbers: Vec<u8> = (1..=30)
            .filter(|day| is_day_number_without_son(*day))
            .collect();
        assert_eq!(numbers, [9, 10, 19, 20, 29, 30]);
        assert_eq!(is_day_without_son(Rd(dangi::EARLIEST.0 - 1)), None);
        assert_eq!(is_day_without_son(Rd(dangi::LATEST.0 + 1)), None);
        assert_eq!(KOREAN_NAME, "손 없는 날");
    }
}
