//! The Gregorian-based calendars of the Galilean moons, after Thomas Gangale
//! (1998).
//!
//! Each moon's solar day divided into circads of about 21 hours — two on Io,
//! four on Europa, eight on Ganymede, nineteen on Callisto — an eight-circad
//! week, and the Earth year in thirteen months of 32 circads, with the
//! Roman *Mercedonius* between Februarius and Martius. A year is 408 or 416
//! circads by a ten-year sequence on the last digit of the year number,
//! Table 2-7 of the page; on Io, Europa and Callisto a 408-circad year's
//! December has 24 circads, and on Ganymede Junius always has 24. The year
//! number is the Gregorian one, and each calendar begins 2002 at the moon's
//! inferior conjunction nearest the opposition of Jupiter of 1 January 2002,
//! Table 2-9: midnight on its prime meridian.
//!
//! The calendars are [`GREGORIAN_IO`], [`GREGORIAN_EUROPA`],
//! [`GREGORIAN_GANYMEDE`] and [`GREGORIAN_CALLISTO`], [`CircadCalendar`]s
//! whose `Rd` counts **circads** from the first circad of 2002, not Earth
//! days. The page's second family, on the Darian calendar, is not carried,
//! nor is its extended intercalation scheme, Table 2-8; why, and how far
//! the ten-year sequences drift, is in `docs/systems/circad-calendars.md`.
//!
//! Source: Gangale, T., "The Calendars of Jupiter",
//! <https://ops-alaska.com/time/gangale_jupiter/jupiter.htm>, §§2.2–2.7 and
//! Tables 2-1 to 2-9, the perpetual Table 2-6 at
//! <https://ops-alaska.com/time/gangale_jupiter/t1998jup_gregorian.htm>,
//! retrieved 2026-09-26 (`gangale-jupiter`). Table 2-9 was computed with
//! NASA Ames' Jupiter Ephemeris Generator 1.2, which was not read.

use crate::circad::{self, CircadCalendar, CircadRule, YearCycle};
use crate::util::{j2000_tt_days_from_utc, utc_unix_seconds};

/// `TAI − UTC` at the conjunctions of Table 2-9, in seconds (IERS, from
/// 1999 to 2005).
const TAI_MINUS_UTC_2002: i64 = 32;

/// Thirteen months of 32 circads: a year of 416.
pub const FULL_YEAR: [u8; 13] = [32; 13];

/// A 408-circad year on Io, Europa or Callisto: December shortened by a
/// week (§2.5).
pub const SHORT_DECEMBER_YEAR: [u8; 13] = [32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 24];

/// Ganymede's year of 408 circads: Junius always has 24 (§2.5; Table 2-6).
pub const GANYMEDE_YEAR: [u8; 13] = [32, 32, 32, 32, 32, 32, 24, 32, 32, 32, 32, 32, 32];

macro_rules! week_names {
    ($prefix:literal) => {
        [
            concat!($prefix, " Solis"),
            concat!($prefix, " Lunae"),
            concat!($prefix, " Terrae"),
            concat!($prefix, " Martis"),
            concat!($prefix, " Mercurii"),
            concat!($prefix, " Jovis"),
            concat!($prefix, " Veneris"),
            concat!($prefix, " Saturni"),
        ]
    };
}

macro_rules! month_names {
    ($prefix:literal, $tenth:literal) => {
        [
            concat!($prefix, " Januarius"),
            concat!($prefix, " Februarius"),
            concat!($prefix, " Mercedonius"),
            concat!($prefix, " Martius"),
            concat!($prefix, " Aprilis"),
            concat!($prefix, " Maius"),
            concat!($prefix, " Junius"),
            concat!($prefix, " Julius"),
            concat!($prefix, " Augustus"),
            concat!($prefix, " ", $tenth),
            concat!($prefix, " October"),
            concat!($prefix, " November"),
            concat!($prefix, " December"),
        ]
    };
}

/// One moon's calendar: its names, its cycles, its rule and the calendar.
macro_rules! galilean {
    (
        $(#[$doc:meta])*
        $calendar:ident, $rule:ident, $months:ident, $week:ident, $cycles:ident,
        id: $id:literal, name: $name:literal, body: $body:literal,
        prefix: $prefix:literal, tenth: $tenth:literal,
        solar_day: $solar:literal, circads: $circads:literal,
        conjunction: ($y:literal, $mo:literal, $d:literal, $h:literal, $mi:literal, $s:literal),
        tables: $tables:expr, digits: $digits:expr $(,)?
    ) => {
        #[doc = concat!("The month names of ", $body, "'s calendar, as Table 2-5 prints them.")]
        pub const $months: [&str; 13] = month_names!($prefix, $tenth);

        #[doc = concat!("The circads of ", $body, "'s week (Table 2-3).")]
        pub const $week: [&str; 8] = week_names!($prefix);

        const $cycles: [hc_calendar::shape::CycleShape; 2] = circad::cycles(&$months, &$week);

        #[doc = concat!("The rule of ", $body, "'s Gregorian-based calendar.")]
        pub const $rule: CircadRule = CircadRule {
            id: $id,
            english_name: $name,
            body: $body,
            solar_day_days: $solar,
            circads_per_solar_day: $circads,
            circad_days: $solar / $circads as f64,
            anchor_j2000_tt_days: j2000_tt_days_from_utc(
                utc_unix_seconds($y, $mo, $d, $h, $mi, $s),
                TAI_MINUS_UTC_2002,
            ),
            anchor_circad: 0,
            epoch_year: 2002,
            month_names: &$months,
            week_names: &$week,
            cycles: &$cycles,
            month_tables: $tables,
            year_cycle: YearCycle::ByLastDigit($digits),
            source: concat!(
                "Gangale, \"The Calendars of Jupiter\", ",
                "https://ops-alaska.com/time/gangale_jupiter/jupiter.htm, ",
                "Tables 2-1, 2-2, 2-3, 2-5, 2-7 and 2-9, retrieved 2026-09-26"
            ),
        };

        $(#[$doc])*
        pub const $calendar: CircadCalendar = CircadCalendar::new(&$rule);
    };
}

galilean! {
    /// Io's Gregorian-based calendar, `gregorian-io`: two circads to the
    /// solar day, the first circad of 2002 beginning 2001 Dec 31 16:07:45 UTC.
    GREGORIAN_IO, IO_RULE, IO_MONTH_NAMES, IO_WEEK_NAMES, IO_CYCLES,
    id: "gregorian-io", name: "Gregorian-based (Io)", body: "Io",
    prefix: "Io", tenth: "September",
    solar_day: 1.769_860, circads: 2,
    conjunction: (2001, 12, 31, 16, 7, 45),
    tables: &[&SHORT_DECEMBER_YEAR, &FULL_YEAR],
    digits: [1, 1, 0, 1, 0, 1, 1, 0, 1, 0],
}

galilean! {
    /// Europa's Gregorian-based calendar, `gregorian-europa`: four circads to
    /// the solar day, the first circad of 2002 beginning 2002 Jan 02
    /// 17:12:57 UTC. Its tenth month is *Eu Septembris*, as Table 2-5 prints
    /// it.
    GREGORIAN_EUROPA, EUROPA_RULE, EUROPA_MONTH_NAMES, EUROPA_WEEK_NAMES, EUROPA_CYCLES,
    id: "gregorian-europa", name: "Gregorian-based (Europa)", body: "Europa",
    prefix: "Eu", tenth: "Septembris",
    solar_day: 3.554_094, circads: 4,
    conjunction: (2002, 1, 2, 17, 12, 57),
    tables: &[&SHORT_DECEMBER_YEAR, &FULL_YEAR],
    digits: [1, 0, 1, 0, 0, 1, 0, 1, 0, 0],
}

galilean! {
    /// Ganymede's Gregorian-based calendar, `gregorian-ganymede`: eight
    /// circads to the solar day, so its week is one solar day; the first
    /// circad of 2002 begins 2002 Jan 01 11:08:29 UTC. Every year is 408
    /// circads under Table 2-7.
    GREGORIAN_GANYMEDE, GANYMEDE_RULE, GANYMEDE_MONTH_NAMES, GANYMEDE_WEEK_NAMES, GANYMEDE_CYCLES,
    id: "gregorian-ganymede", name: "Gregorian-based (Ganymede)", body: "Ganymede",
    prefix: "Gan", tenth: "September",
    solar_day: 7.166_386, circads: 8,
    conjunction: (2002, 1, 1, 11, 8, 29),
    tables: &[&GANYMEDE_YEAR],
    digits: [0; 10],
}

galilean! {
    /// Callisto's Gregorian-based calendar, `gregorian-callisto`: nineteen
    /// circads to the solar day, the first circad of 2002 beginning 2001
    /// Dec 28 12:27:23 UTC.
    GREGORIAN_CALLISTO, CALLISTO_RULE, CALLISTO_MONTH_NAMES, CALLISTO_WEEK_NAMES, CALLISTO_CYCLES,
    id: "gregorian-callisto", name: "Gregorian-based (Callisto)", body: "Callisto",
    prefix: "Cal", tenth: "September",
    solar_day: 16.753_548, circads: 19,
    conjunction: (2001, 12, 28, 12, 27, 23),
    tables: &[&SHORT_DECEMBER_YEAR, &FULL_YEAR],
    digits: [1, 1, 0, 1, 1, 1, 1, 0, 1, 1],
}

/// The four Gregorian-based calendars, Io outwards.
pub const ALL: [CircadCalendar; 4] = [
    GREGORIAN_IO,
    GREGORIAN_EUROPA,
    GREGORIAN_GANYMEDE,
    GREGORIAN_CALLISTO,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circad::CircadDate;
    use crate::util::{SECONDS_PER_DAY, j2000_offset_days, tai_from_utc_fields};
    use crate::util::{j2000_tt_days_from_utc, utc_unix_seconds};

    /// A UTC time as its fields: year, month, day, hour, minute, second.
    type Utc = (i64, u8, u8, u8, u8, u8);

    /// Table 2-9, the conjunctions that are 2002 January 01 00:00:00.
    const CONJUNCTIONS: [(CircadCalendar, Utc); 4] = [
        (GREGORIAN_IO, (2001, 12, 31, 16, 7, 45)),
        (GREGORIAN_EUROPA, (2002, 1, 2, 17, 12, 57)),
        (GREGORIAN_GANYMEDE, (2002, 1, 1, 11, 8, 29)),
        (GREGORIAN_CALLISTO, (2001, 12, 28, 12, 27, 23)),
    ];

    #[test]
    fn each_calendar_begins_2002_at_its_table_2_9_conjunction() {
        for (calendar, (y, mo, d, h, mi, s)) in CONJUNCTIONS {
            let conjunction = tai_from_utc_fields(y, mo, d, h, mi, s).unwrap();
            let start = calendar.circad_start(0).unwrap();
            let gap = (j2000_offset_days(start) - j2000_offset_days(conjunction)) * SECONDS_PER_DAY;
            assert!(gap.abs() < 1e-3, "{}: {gap} s", calendar.rule().id);
            // A second after the conjunction is 1 Januarius 2002, a Solis; a
            // second before is the last circad of 2001.
            let after = tai_from_utc_fields(y, mo, d, h, mi, s + 1).unwrap();
            let date = calendar.date_at(after).unwrap();
            assert_eq!(date, CircadDate::new(2002, 1, 1));
            assert_eq!(
                calendar.week_name(calendar.circad_at(after)),
                calendar.rule().week_names[0]
            );
            assert!(calendar.month_name(date).unwrap().ends_with(" Januarius"));
            let before = tai_from_utc_fields(y, mo, d, h, mi, s - 1).unwrap();
            let date = calendar.date_at(before).unwrap();
            assert_eq!(date.year, 2001);
            assert_eq!(date.month, 13);
        }
    }

    #[test]
    fn the_ten_year_sequences_are_table_2_7() {
        const TABLE: [(CircadCalendar, [i64; 10], i64); 4] = [
            (
                GREGORIAN_IO,
                [416, 416, 408, 416, 408, 416, 416, 408, 416, 408],
                4128,
            ),
            (
                GREGORIAN_EUROPA,
                [416, 408, 416, 408, 408, 416, 408, 416, 408, 408],
                4112,
            ),
            (GREGORIAN_GANYMEDE, [408; 10], 4080),
            (
                GREGORIAN_CALLISTO,
                [416, 416, 408, 416, 416, 416, 416, 408, 416, 416],
                4144,
            ),
        ];
        for (calendar, lengths, decade) in TABLE {
            let rule = calendar.rule();
            for year in 1_990..2_050 {
                assert_eq!(
                    rule.circads_in_year(year),
                    lengths[(year % 10) as usize],
                    "{} {year}",
                    rule.id
                );
            }
            // Negative years keep the sequence by the last digit of the
            // astronomical year count.
            assert_eq!(rule.circads_in_year(-8), lengths[2]);
            assert_eq!(rule.year_start(2_010) - rule.year_start(2_000), decade);
            // The leap year is the one whose December keeps its fourth week.
            for year in 2_000..2_010 {
                assert_eq!(
                    rule.is_leap_year(year),
                    lengths[(year % 10) as usize] == 416
                );
            }
        }
    }

    #[test]
    fn the_months_are_table_2_5() {
        assert_eq!(IO_MONTH_NAMES[0], "Io Januarius");
        assert_eq!(IO_MONTH_NAMES[2], "Io Mercedonius");
        assert_eq!(IO_MONTH_NAMES[3], "Io Martius");
        assert_eq!(IO_MONTH_NAMES[12], "Io December");
        assert_eq!(EUROPA_MONTH_NAMES[9], "Eu Septembris");
        assert_eq!(EUROPA_MONTH_NAMES[10], "Eu October");
        assert_eq!(GANYMEDE_MONTH_NAMES[9], "Gan September");
        assert_eq!(CALLISTO_MONTH_NAMES[12], "Cal December");
        assert_eq!(IO_WEEK_NAMES[2], "Io Terrae");
        assert_eq!(EUROPA_WEEK_NAMES[0], "Eu Solis");
        assert_eq!(GANYMEDE_WEEK_NAMES[7], "Gan Saturni");
        assert_eq!(CALLISTO_WEEK_NAMES[4], "Cal Mercurii");
        // Ganymede's Junius is 24 circads every year; the others' December
        // is 24 in a 408-circad year and 32 in a 416.
        for year in 2_000..2_020 {
            assert_eq!(GANYMEDE_RULE.month_lengths(year)[6], 24);
            assert_eq!(GANYMEDE_RULE.month_lengths(year)[12], 32);
            for rule in [IO_RULE, EUROPA_RULE, CALLISTO_RULE] {
                let december = rule.month_lengths(year)[12];
                assert_eq!(i64::from(december) + 384, rule.circads_in_year(year));
            }
        }
    }

    #[test]
    fn io_2003_begins_on_the_worked_example_day() {
        // 2002 ends in 2, a 408-circad year on Io, so Io December 2002 has 24
        // circads, and 408 circads of 0.884930 days from the epoch reach
        // 2002 December 27 17:21:49 UTC.
        assert_eq!(IO_RULE.year_start(2_003), 408);
        assert_eq!(IO_RULE.month_lengths(2_002)[12], 24);
        let start = GREGORIAN_IO.circad_start(408).unwrap();
        let expected = tai_from_utc_fields(2002, 12, 27, 17, 21, 49).unwrap();
        let gap = (j2000_offset_days(start) - j2000_offset_days(expected)) * SECONDS_PER_DAY;
        assert!(gap.abs() < 1.0, "{gap} s");
        let date = GREGORIAN_IO
            .date_at(tai_from_utc_fields(2002, 12, 27, 18, 0, 0).unwrap())
            .unwrap();
        assert_eq!(date, CircadDate::new(2003, 1, 1));
    }

    #[test]
    fn the_new_year_stays_within_days_of_the_gregorian_one_for_decades() {
        // The page sets the epoch "such that New Year's Circad will always
        // occur within a few terrestrial days of the beginning of the
        // calendar year on Earth". Under the ten-year sequences alone that
        // holds for these years, and drifts beyond them.
        for calendar in ALL {
            for year in 1_990..=2_030 {
                let start = calendar
                    .circad_start(calendar.rule().year_start(year))
                    .unwrap();
                let january = tai_from_utc_fields(year, 1, 1, 0, 0, 0).unwrap();
                let days = j2000_offset_days(start) - j2000_offset_days(january);
                assert!(days.abs() < 12.0, "{} {year}: {days}", calendar.rule().id);
            }
        }
    }

    #[test]
    fn the_ten_year_sequences_drift_as_their_residuals_say() {
        // Ganymede's sequence runs 2.7162 circads long a decade (Table 2-7),
        // so its new year walks through January and on: 1 Gan Januarius
        // 2100 falls on 25 January 2100 and 2500 on 2 May 2500 (UTC, with
        // TAI - UTC held at its 2017 value, which moves nothing by a day).
        for (year, month, day) in [(2_100, 1, 25), (2_500, 5, 2)] {
            let start = GREGORIAN_GANYMEDE
                .circad_start(GANYMEDE_RULE.year_start(year))
                .unwrap();
            let midnight = j2000_tt_days_from_utc(utc_unix_seconds(year, month, day, 0, 0, 0), 37);
            let into_day = j2000_offset_days(start) - midnight;
            assert!((0.0..1.0).contains(&into_day), "{year}: {into_day}");
        }
    }

    #[test]
    fn the_circads_agree_with_the_bodies_table() {
        // Table 2-4: circads per 365.24238-day year, 412.7358 (Io), 411.0667,
        // 407.7284, 414.2170. Table 2-1's days give Io's as 412.7359, one in
        // the last digit.
        for (calendar, printed) in [
            (GREGORIAN_IO, 412.7358),
            (GREGORIAN_EUROPA, 411.0667),
            (GREGORIAN_GANYMEDE, 407.7284),
            (GREGORIAN_CALLISTO, 414.2170),
        ] {
            let rule = calendar.rule();
            let per_year = 365.242_38 / rule.circad_days;
            assert!(
                (per_year - printed).abs() < 1.5e-4,
                "{}: {per_year}",
                rule.id
            );
            let derived = crate::bodies::by_name(rule.body)
                .unwrap()
                .solar_day_days()
                .unwrap();
            let relative = (derived - rule.solar_day_days).abs() / rule.solar_day_days;
            assert!(relative < 1e-6, "{}: {derived} {relative}", rule.id);
        }
    }

    #[test]
    fn the_circads_are_about_twenty_one_hours() {
        // Table 2-2: 21.23833, 21.32456, 21.49916 and 21.16238 hours.
        for (calendar, hours) in [
            (GREGORIAN_IO, 21.238_33),
            (GREGORIAN_EUROPA, 21.324_56),
            (GREGORIAN_GANYMEDE, 21.499_16),
            (GREGORIAN_CALLISTO, 21.162_38),
        ] {
            let circad = calendar.rule().circad_days * 24.0;
            assert!(
                (circad - hours).abs() < 2e-5,
                "{}: {circad}",
                calendar.rule().id
            );
        }
    }
}
