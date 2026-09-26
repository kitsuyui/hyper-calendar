//! The Darian calendar for Titan, after Thomas Gangale (2003).
//!
//! A circad of one sixteenth of Titan's solar day of 15.969 095 Earth days,
//! 0.998 068 439 days; an eight-circad week, half a solar day; and the
//! Darian calendar's 24 months in the Martian vernal-equinox year of
//! 688.3006 circads. A common year is 688 circads, the third month of each
//! quarter 32 and the rest 28; a leap year adds four circads each to
//! Rishabha and Vrishika, 696 in all, by Gangale's eight-circad system,
//! `8·(Y\25 − Y\400)` circads intercalated through year `Y`. Julian Circad 0
//! begins year 0 in March 1609, and the count is anchored where the page
//! calibrates it, Julian Circad 144 096 beginning at the superior
//! conjunction of Titan of 18 December 2002, at solar noon on the prime
//! meridian.
//!
//! The calendar is [`DARIAN_TITAN`], a [`CircadCalendar`] whose `Rd` is the
//! **Julian Circad**, not an Earth day. The system, the page's misprinted
//! mean years (668.32 and 668.30 for the 688.32 and 688.30 its arithmetic
//! gives, which this module follows), the sixteen-circad alternative and the
//! later formula it does not carry are in `docs/systems/circad-calendars.md`.
//!
//! Source: Gangale, T., "The Darian Calendar for Titan",
//! <https://ops-alaska.com/time/gangale_saturn/Darian_Titan_main.htm>, §§3.2–3.6
//! and Tables 3-1 to 3-4, the perpetual Table 3-2 at
//! <https://ops-alaska.com/time/gangale_saturn/t2003darian_titan.htm>,
//! retrieved 2026-09-26 (`gangale-titan`). The page's calibration cites the
//! *Astronomical Almanac* for 2002, pp. A11 and F45, which was not read.

use crate::circad::{self, CircadCalendar, CircadRule, YearCycle};
use crate::util::SECONDS_PER_DAY;

/// Titan's solar day in Earth days, as the page derives it (§3.2).
pub const SOLAR_DAY_DAYS: f64 = 15.969_095;

/// The circad in Earth days, as the page prints it and calculates with it
/// (§3.3). One sixteenth of [`SOLAR_DAY_DAYS`] is 0.998 068 437 5; the
/// printed value is 0.13 ms longer, and it is the one the calibration uses.
pub const CIRCAD_DAYS: f64 = 0.998_068_439;

/// The Julian Date, read in UTC, of the superior conjunction of Titan of
/// 2002 December 18, 10.7 h UTC (§3.6).
pub const CALIBRATION_JULIAN_DATE_UTC: f64 = 2_452_626.945_83;

/// The Julian Circad that begins at the calibration: the conjunction ended
/// the 30th week of year 209 (§3.6).
pub const CALIBRATION_JULIAN_CIRCAD: i64 = 144_096;

/// `TAI − UTC` at the calibration, in seconds (IERS, from 1999 to 2005).
const CALIBRATION_TAI_MINUS_UTC: f64 = 32.0;

/// The Julian Date of Julian Circad 0 as the page prints it: 1609 March 15,
/// 18:37:32 (§3.6). The count here is anchored at the calibration, from
/// which the page's own rounded chain gives a value 2.3 s earlier; this
/// constant is what the test compares with.
pub const PRINTED_JULIAN_CIRCAD_ZERO_JULIAN_DATE: f64 = 2_308_809.276_07;

/// The position in the solar day, `0..16` from midnight on the prime
/// meridian, of the circad [`CALIBRATION_JULIAN_CIRCAD`]: solar noon.
const CALIBRATION_SOLAR_POSITION: i64 = 8;

/// The 24 month names, the Darian calendar's (Tables 3-1 and 3-2).
pub const MONTH_NAMES: [&str; 24] = crate::mars::darian::MONTH_NAMES;

/// The eight circads of the week: the Galilean calendars' names, which the
/// page says Titan shares, without the "Ti" prefix it says "might be added"
/// (§3.5; Gangale, "The Calendars of Jupiter", Table 2-3).
pub const WEEK_NAMES: [&str; 8] = [
    "Solis", "Lunae", "Terrae", "Martis", "Mercurii", "Jovis", "Veneris", "Saturni",
];

/// The months of a common year of 688 circads: the third month of each
/// quarter has 32 (Table 3-2, "Common Years").
pub const COMMON_MONTHS: [u8; 24] = [
    28, 28, 32, 28, 28, 28, 28, 28, 32, 28, 28, 28, 28, 28, 32, 28, 28, 28, 28, 28, 32, 28, 28, 28,
];

/// The months of a leap year of 696 circads: Rishabha and Vrishika gain
/// half a week each (§3.5; Table 3-2, "Leap Years").
pub const LEAP_MONTHS: [u8; 24] = [
    28, 28, 32, 28, 28, 28, 28, 28, 32, 28, 28, 32, 28, 28, 32, 28, 28, 28, 28, 28, 32, 28, 28, 32,
];

const CYCLES: [hc_calendar::shape::CycleShape; 2] = circad::cycles(&MONTH_NAMES, &WEEK_NAMES);

/// The rule of the Darian calendar for Titan.
pub const RULE: CircadRule = CircadRule {
    id: "darian-titan",
    english_name: "Darian (Titan)",
    body: "Titan",
    solar_day_days: SOLAR_DAY_DAYS,
    circads_per_solar_day: 16,
    circad_days: CIRCAD_DAYS,
    anchor_j2000_tt_days: (CALIBRATION_JULIAN_DATE_UTC - 2_451_545.0)
        + (CALIBRATION_TAI_MINUS_UTC + 32.184) / SECONDS_PER_DAY,
    anchor_circad: CALIBRATION_JULIAN_CIRCAD,
    epoch_year: 0,
    month_names: &MONTH_NAMES,
    week_names: &WEEK_NAMES,
    cycles: &CYCLES,
    month_tables: &[&COMMON_MONTHS, &LEAP_MONTHS],
    year_cycle: YearCycle::EveryExcept {
        every: 25,
        except: 400,
    },
    source: "Gangale, \"The Darian Calendar for Titan\", \
             https://ops-alaska.com/time/gangale_saturn/Darian_Titan_main.htm, \
             sections 3.2-3.6, retrieved 2026-09-26",
};

/// The Darian calendar for Titan, `darian-titan`. Its `Rd` is the Julian
/// Circad.
pub const DARIAN_TITAN: CircadCalendar = CircadCalendar::new(&RULE);

/// Whether `year` is a leap year of 696 circads: divisible by 25 and not by
/// 400. Year 0 is not.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    RULE.is_leap_year(year)
}

/// The position of a circad in Titan's solar day on the prime meridian,
/// `0..16`: 0 begins at midnight, 4 at sunrise, 8 at noon, 12 at sunset,
/// as the page's Table 3-3 counts them.
#[must_use]
pub const fn solar_day_position(julian_circad: i64) -> u8 {
    (julian_circad - CALIBRATION_JULIAN_CIRCAD + CALIBRATION_SOLAR_POSITION).rem_euclid(16) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circad::CircadDate;
    use crate::util::tai_from_utc_fields;
    use hc_calendar::Calendar;

    #[test]
    fn the_published_calibration_reproduces() {
        // §3.6: the superior conjunction of 2002 Dec 18 at 10.7 h UTC was
        // 209 Ari 13, Julian Circad 144096, at solar noon.
        let conjunction = tai_from_utc_fields(2002, 12, 18, 10, 42, 0).unwrap();
        let circad = DARIAN_TITAN.circad_at(conjunction);
        assert_eq!(circad, 144_096);
        let date = DARIAN_TITAN.date_at(conjunction).unwrap();
        assert_eq!(date, CircadDate::new(209, 9, 13));
        assert_eq!(DARIAN_TITAN.month_name(date).unwrap(), "Aries");
        assert_eq!(DARIAN_TITAN.week_name(circad), "Solis");
        assert_eq!(solar_day_position(circad), 8);

        // Year 209 began at Julian Circad 143856, 2002 Apr 22 21:49:32 UTC,
        // at solar noon.
        assert_eq!(RULE.year_start(209), 143_856);
        assert_eq!(solar_day_position(143_856), 8);
        let start = DARIAN_TITAN.circad_start(143_856).unwrap();
        let printed = tai_from_utc_fields(2002, 4, 22, 21, 49, 32).unwrap();
        let gap = crate::util::j2000_offset_days(start) - crate::util::j2000_offset_days(printed);
        assert!(
            (gap * SECONDS_PER_DAY).abs() < 1.0,
            "{} s",
            gap * SECONDS_PER_DAY
        );
        assert_eq!(
            DARIAN_TITAN.date_at(tai_from_utc_fields(2002, 4, 22, 21, 50, 0).unwrap()),
            Ok(CircadDate::new(209, 1, 1))
        );
        assert_eq!(
            DARIAN_TITAN.date_at(tai_from_utc_fields(2002, 4, 22, 21, 49, 0).unwrap()),
            Ok(CircadDate::new(208, 24, 28))
        );
    }

    #[test]
    fn julian_circad_zero_is_within_seconds_of_the_printed_julian_date() {
        // The page prints JD 2308809.27607 (1609 Mar 15 18:37:32). The count
        // anchored at the conjunction, read back on the page's own scale —
        // its Julian Days are UTC in 2002, extended without a ΔT — gives
        // 2308809.27604.
        let zero = DARIAN_TITAN.circad_start(0).unwrap();
        let tt_days = crate::util::j2000_offset_days(zero);
        let page_scale = tt_days - (CALIBRATION_TAI_MINUS_UTC + 32.184) / SECONDS_PER_DAY;
        let julian_date = page_scale + 2_451_545.0;
        let gap = (julian_date - PRINTED_JULIAN_CIRCAD_ZERO_JULIAN_DATE) * SECONDS_PER_DAY;
        assert!(gap.abs() < 3.0, "{gap} s");
        assert_eq!(
            DARIAN_TITAN.from_fixed(hc_calendar::Rd(0)),
            Ok(CircadDate::new(0, 1, 1))
        );
    }

    #[test]
    fn the_leap_rule_puts_eight_leap_years_before_year_209() {
        // §3.6: "there should have been eight leap years from the year 0
        // (which was not leap year, being counted as divisible by 400) to
        // the year 209", and 201 * 688 + 8 * 696 = 143856.
        assert!(!is_leap_year(0));
        let leap: Vec<i64> = (0..209).filter(|&year| is_leap_year(year)).collect();
        assert_eq!(leap, [25, 50, 75, 100, 125, 150, 175, 200]);
        assert_eq!(201 * 688 + 8 * 696, RULE.year_start(209));
        assert!(!is_leap_year(400) && !is_leap_year(800));
        assert!(is_leap_year(425) && is_leap_year(-25));
        assert_eq!(DARIAN_TITAN.is_leap_year(225), Ok(true));
        assert_eq!(RULE.circads_in_year(225), 696);
        assert_eq!(RULE.circads_in_year(226), 688);
    }

    #[test]
    fn the_intercalation_is_eight_times_y25_less_y400() {
        // The page's formula counts the circads intercalated through year Y.
        for year in 0..2_000i64 {
            let intercalated = RULE.year_start(year + 1) - 688 * (year + 1);
            assert_eq!(intercalated, 8 * (year / 25 - year / 400), "{year}");
        }
    }

    #[test]
    fn the_mean_year_is_688_3_circads_as_the_arithmetic_gives() {
        // The page prints "(688 * 25 + 8) / 25 circads = 668.3200 circads"
        // and "668.3000"; the arithmetic is 688.32 and 688.30, against the
        // 688.3006 circads of the Martian vernal-equinox year.
        let over_25: f64 = (688.0 * 25.0 + 8.0) / 25.0;
        assert!((over_25 - 688.32).abs() < 1e-12);
        let mean = (RULE.year_start(400) - RULE.year_start(0)) as f64 / 400.0;
        assert!((mean - 688.3).abs() < 1e-12, "{mean}");
        let year = 686.971_1 * 16.0 / SOLAR_DAY_DAYS;
        assert!((year - 688.3006).abs() < 5e-5, "{year}");
    }

    #[test]
    fn the_month_tables_are_table_3_2() {
        assert_eq!(
            COMMON_MONTHS.iter().map(|&m| u32::from(m)).sum::<u32>(),
            688
        );
        assert_eq!(LEAP_MONTHS.iter().map(|&m| u32::from(m)).sum::<u32>(), 696);
        // The 32-circad months of a common year are the third of each
        // quarter (Table 3-1).
        for (index, &length) in COMMON_MONTHS.iter().enumerate() {
            assert_eq!(length == 32, index % 6 == 2, "{}", MONTH_NAMES[index]);
        }
        // A leap year lengthens Rishabha and Vrishika and nothing else.
        for index in 0..24 {
            let gained = LEAP_MONTHS[index] - COMMON_MONTHS[index];
            assert_eq!(gained, if index == 11 || index == 23 { 4 } else { 0 });
        }
        assert_eq!(RULE.month_lengths(209), &COMMON_MONTHS);
        assert_eq!(RULE.month_lengths(225), &LEAP_MONTHS);
    }

    /// Table 3-3: the position in the solar day at which each month begins,
    /// for the four kinds of year of the eight-circad system.
    #[test]
    fn the_solar_phasing_is_table_3_3() {
        const TYPE_1_COMMON: [u8; 24] = [
            0, 12, 8, 8, 4, 0, 12, 8, 4, 4, 0, 12, 8, 4, 0, 0, 12, 8, 4, 0, 12, 12, 8, 4,
        ];
        const TYPE_1_LEAP: [u8; 24] = [
            0, 12, 8, 8, 4, 0, 12, 8, 4, 4, 0, 12, 12, 8, 4, 4, 0, 12, 8, 4, 0, 0, 12, 8,
        ];
        const TYPE_2_COMMON: [u8; 24] = [
            8, 4, 0, 0, 12, 8, 4, 0, 12, 12, 8, 4, 0, 12, 8, 8, 4, 0, 12, 8, 4, 4, 0, 12,
        ];
        const TYPE_2_LEAP: [u8; 24] = [
            8, 4, 0, 0, 12, 8, 4, 0, 12, 12, 8, 4, 4, 0, 12, 12, 8, 4, 0, 12, 8, 8, 4, 0,
        ];
        let mut seen = [false; 4];
        for year in 0..1_000 {
            let start = RULE.year_start(year);
            let leap = is_leap_year(year);
            let expected = match (solar_day_position(start), leap) {
                (0, false) => (0, &TYPE_1_COMMON),
                (0, true) => (1, &TYPE_1_LEAP),
                (8, false) => (2, &TYPE_2_COMMON),
                (8, true) => (3, &TYPE_2_LEAP),
                (other, _) => panic!("year {year} begins at solar position {other}"),
            };
            seen[expected.0] = true;
            let mut circad = start;
            for (month, &length) in RULE.month_lengths(year).iter().enumerate() {
                assert_eq!(
                    solar_day_position(circad),
                    expected.1[month],
                    "{year} {month}"
                );
                circad += i64::from(length);
            }
        }
        assert_eq!(seen, [true; 4]);
    }

    #[test]
    fn the_circad_agrees_with_the_bodies_table() {
        let derived = crate::bodies::by_name("Titan")
            .unwrap()
            .solar_day_days()
            .unwrap();
        let relative = (derived - SOLAR_DAY_DAYS).abs() / SOLAR_DAY_DAYS;
        assert!(relative < 1e-7, "{derived} vs {SOLAR_DAY_DAYS}: {relative}");
        // "23 hours, 57 minutes, 13.11 seconds".
        let seconds = CIRCAD_DAYS * SECONDS_PER_DAY;
        assert!((seconds - (23.0 * 3_600.0 + 57.0 * 60.0 + 13.11)).abs() < 0.005);
    }

    #[test]
    fn the_calendar_describes_itself() {
        let meta = DARIAN_TITAN.meta();
        assert_eq!(meta.id.to_string(), "darian-titan");
        assert_eq!(meta.english_name, "Darian (Titan)");
        assert_eq!(DARIAN_TITAN.cycles()[1].kind, "circad-of-week");
        assert_eq!(DARIAN_TITAN.cycles()[0].names.len(), 24);
    }
}
