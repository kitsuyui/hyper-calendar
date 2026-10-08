//! The ranges the two boundary READMEs state for the sky and the Hindu
//! exports are the ranges the exports answer for.
//!
//! The "Answers for" tables of `hyper-calendar-wasm/README.md` and
//! `hyper-calendar-ffi/README.md` give each export's accepted input as a
//! range of POSIX seconds. A range written by hand drifts from the code: four
//! Hindu exports stated 0000-01-01 to 4926-09-24 for the years −1000 to 3000.
//! This reads the numbers out of the rows and probes each export at both ends
//! of the range and one second outside it.

#![cfg(all(feature = "astro", feature = "indic", feature = "jupiter"))]

use hyper_calendar::astro_lines::{EARLIEST_YEAR, LATEST_YEAR};
use hyper_calendar::boundary::{Answer, Refusal};
use hyper_calendar::hc_calendar::gregorian::new_year;
use hyper_calendar::jupiter_lines::station_lines;
use hyper_calendar::panchanga_lines::{
    ayanamsa_at_line, ayanamsa_from_anchor_line, tithi_at_lines,
};

const READMES: [&str; 2] = [
    "../hyper-calendar-wasm/README.md",
    "../hyper-calendar-ffi/README.md",
];

/// Seconds in a day.
const DAY: i64 = 86_400;

/// The Unix day on which `year` starts, in the proleptic Gregorian calendar.
fn unix_day(year: i64) -> i64 {
    new_year(year).0 - hyper_calendar::hc_calendar::fixed::RD_OF_UNIX_EPOCH
}

/// The first and the last POSIX second of the years the sky exports answer for:
/// the start of `EARLIEST_YEAR` and the second before the start of the year
/// after `LATEST_YEAR`.
fn era() -> (i64, i64) {
    seconds_of_years(EARLIEST_YEAR, LATEST_YEAR)
}

/// The first and the last POSIX second of the years `first` through `last`.
fn seconds_of_years(first: i64, last: i64) -> (i64, i64) {
    (unix_day(first) * DAY, unix_day(last + 1) * DAY - 1)
}

/// The integers a text writes with a space between groups of three digits
/// and U+2212 for a minus, in order.
fn integers(text: &str) -> Vec<i64> {
    let chars: Vec<char> = text.chars().collect();
    let mut found = Vec::new();
    let mut at = 0;
    while at < chars.len() {
        if !chars[at].is_ascii_digit() {
            at += 1;
            continue;
        }
        let negative = at > 0 && matches!(chars[at - 1], '\u{2212}' | '-');
        let mut value: i64 = 0;
        loop {
            while at < chars.len() && chars[at].is_ascii_digit() {
                value = value * 10 + i64::from(chars[at] as u8 - b'0');
                at += 1;
            }
            let grouped = at + 4 <= chars.len()
                && chars[at].is_whitespace()
                && chars[at + 1..at + 4].iter().all(char::is_ascii_digit)
                && chars.get(at + 4).is_none_or(|next| !next.is_ascii_digit());
            if grouped {
                at += 1;
            } else {
                break;
            }
        }
        found.push(if negative { -value } else { value });
    }
    found
}

/// The text of the row of the range table whose export cell names `export`.
fn row(readme: &str, export: &str) -> String {
    let marker = format!("`{export}`");
    readme
        .lines()
        .find(|line| {
            line.starts_with('|')
                && line
                    .split('|')
                    .nth(2)
                    .is_some_and(|cell| cell.contains(&marker))
                && line
                    .split('|')
                    .nth(3)
                    .is_some_and(|cell| cell.contains("through") || cell.contains("up to"))
        })
        .unwrap_or_else(|| panic!("no range row for {export}"))
        .to_owned()
}

/// The integers of a row's range cell that follow `after`.
fn range_after(row: &str, after: &str) -> Vec<i64> {
    let cell = row
        .split('|')
        .nth(3)
        .unwrap_or_else(|| panic!("no range cell in {row}"));
    let at = cell
        .find(after)
        .unwrap_or_else(|| panic!("no {after} in {cell}"));
    integers(&cell[at + after.len()..])
}

#[test]
fn the_integers_are_read_as_the_readmes_write_them() {
    assert_eq!(
        integers("−93 724 128 000 through 32 535 215 999, the years −1000 to 3000"),
        [-93_724_128_000, 32_535_215_999, -1000, 3000]
    );
}

#[test]
fn the_instant_ranges_of_the_hindu_exports_are_the_years_they_name() {
    let (first, last) = era();
    assert_eq!((first, last), (-93_724_128_000, 32_535_215_999));
    for readme in READMES {
        let text =
            std::fs::read_to_string(readme).unwrap_or_else(|error| panic!("{readme}: {error}"));
        let row = row(&text, "hc_tithi_at");
        assert!(row.contains("`hc_ayanamsa_at`") && row.contains("`hc_ayanamsa_from_anchor`"));
        let stated = range_after(&row, "`unix_seconds`");
        assert_eq!(&stated[..2], [first, last], "{readme}: {row}");

        for unix in [first, last] {
            assert!(
                tithi_at_lines(unix, "true").is_ok(),
                "{readme} hc_tithi_at {unix}"
            );
            assert!(
                ayanamsa_at_line(unix, "lahiri").is_ok(),
                "{readme} hc_ayanamsa_at {unix}"
            );
            assert!(
                ayanamsa_from_anchor_line(unix, 2_435_553.5, 23.25).is_ok(),
                "{readme} hc_ayanamsa_from_anchor {unix}"
            );
        }
        for unix in [first - 1, last + 1] {
            assert_eq!(
                tithi_at_lines(unix, "true"),
                Err(Refusal::OutOfRange),
                "{readme} {unix}"
            );
            assert_eq!(
                ayanamsa_at_line(unix, "lahiri"),
                Err(Refusal::OutOfRange),
                "{readme} {unix}"
            );
            assert_eq!(
                ayanamsa_from_anchor_line(unix, 2_435_553.5, 23.25),
                Err(Refusal::OutOfRange),
                "{readme} {unix}"
            );
        }
    }
}

#[test]
fn the_span_of_the_jupiter_stations_is_the_years_the_readme_names() {
    let (first, last) = era();
    for readme in READMES {
        let text =
            std::fs::read_to_string(readme).unwrap_or_else(|error| panic!("{readme}: {error}"));
        let row = row(&text, "hc_jupiter_stations");
        let from = range_after(&row, "`from_unix_seconds`");
        // `from` through `last`, and `to` up to the second after it.
        assert_eq!(&from[..3], [first, last, last + 1], "{readme}: {row}");
        assert!(station_lines(first, first + 1, "lahiri").is_ok());
        assert!(station_lines(last, last + 1, "lahiri").is_ok());
        assert_eq!(
            station_lines(first - 1, first, "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            station_lines(last + 1, last + 2, "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            station_lines(last, last + 2, "lahiri"),
            Err(Refusal::OutOfRange)
        );
    }
}

/// Holds the first two integers that the range cell of `export`'s row states
/// after `after`, in each README, to `first` and `last`. The seconds and the
/// years of a range are both stated this way.
fn assert_states(export: &str, after: &str, first: i64, last: i64) {
    for readme in READMES {
        let text =
            std::fs::read_to_string(readme).unwrap_or_else(|error| panic!("{readme}: {error}"));
        let row = row(&text, export);
        let stated = range_after(&row, after);
        assert_eq!(&stated[..2], [first, last], "{readme}: {row}");
    }
}

/// The civil and zone exports whose READMEs state a range of years, and the
/// zone exports whose range is POSIX seconds, probed at both ends of the range
/// and one second outside it.
#[cfg(all(feature = "civil", feature = "tz", feature = "std"))]
mod civil_and_zones {
    use super::*;
    use hyper_calendar::python_lines::{
        fixed_from_week, gmtime_line, local_resolution_line, localtime_line, mktime, timegm,
    };

    #[test]
    fn gmtime_answers_the_seconds_of_the_years_its_row_names() {
        let (first, last) = seconds_of_years(-9_999_999, 9_999_999);
        assert_states("hc_gmtime", "of the years", -9_999_999, 9_999_999);
        assert!(gmtime_line(first).is_ok());
        assert!(gmtime_line(last).is_ok());
        assert_eq!(gmtime_line(first - 1), Err(Refusal::OutOfRange));
        assert_eq!(gmtime_line(last + 1), Err(Refusal::OutOfRange));
    }

    #[test]
    fn timegm_answers_the_years_its_row_names() {
        assert_states("hc_timegm", "`year`", -9_999_999, 9_999_999);
        assert_eq!(timegm([2000, 1, 1, 0, 0, 0]), Ok(946_684_800));
        // Day, hour, minute and second of any size, as Python's.
        assert_eq!(
            timegm([2000, 1, 1, 0, 0, 1_000_000_000]),
            Ok(946_684_800 + 1_000_000_000)
        );
        assert!(timegm([-9_999_999, 1, 1, 0, 0, 0]).is_ok());
        assert!(timegm([9_999_999, 12, 31, 23, 59, 59]).is_ok());
        assert_eq!(
            timegm([-10_000_000, 1, 1, 0, 0, 0]),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            timegm([10_000_000, 1, 1, 0, 0, 0]),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(timegm([2000, 0, 1, 0, 0, 0]), Err(Refusal::InvalidDate));
        assert_eq!(timegm([2000, 13, 1, 0, 0, 0]), Err(Refusal::InvalidDate));
        assert_eq!(
            timegm([9_999_999, 12, 31, i64::MAX, 0, 0]),
            Err(Refusal::Overflow)
        );
    }

    #[test]
    fn fixed_from_week_answers_the_years_its_row_names() {
        assert_states("hc_fixed_from_week", "`week_year`", -9_999_998, 9_999_998);
        assert!(fixed_from_week(-9_999_998, 1, 1, 1, 4).is_ok());
        assert!(fixed_from_week(9_999_998, 1, 1, 1, 4).is_ok());
        assert_eq!(
            fixed_from_week(-9_999_999, 1, 1, 1, 4),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            fixed_from_week(9_999_999, 1, 1, 1, 4),
            Err(Refusal::OutOfRange)
        );
        // 2026 has 53 weeks under the ISO rule, 2025 has 52.
        assert!(fixed_from_week(2026, 53, 1, 1, 4).is_ok());
        assert_eq!(
            fixed_from_week(2025, 53, 1, 1, 4),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(fixed_from_week(2025, 0, 1, 1, 4), Err(Refusal::InvalidDate));
        assert_eq!(fixed_from_week(2025, 1, 0, 1, 4), Err(Refusal::InvalidDate));
        assert_eq!(fixed_from_week(2025, 1, 8, 1, 4), Err(Refusal::InvalidDate));
        assert_eq!(fixed_from_week(2025, 1, 1, 8, 4), Err(Refusal::OutOfRange));
        assert_eq!(fixed_from_week(2025, 1, 1, 1, 0), Err(Refusal::OutOfRange));
    }

    #[test]
    fn localtime_answers_the_instants_its_row_names() {
        // The instants of `hc_zone_offset`, a wider range than the sky's.
        let (first, last) = (-315_631_497_830_400, 315_507_195_014_399);
        assert_states("hc_localtime", "`unix_seconds`", first, last);
        assert!(localtime_line(first, "UTC").is_ok());
        assert!(localtime_line(last, "UTC").is_ok());
        assert_eq!(localtime_line(first - 1, "UTC"), Err(Refusal::OutOfRange));
        assert_eq!(localtime_line(last + 1, "UTC"), Err(Refusal::OutOfRange));
    }

    #[test]
    fn local_resolution_and_mktime_answer_the_readings_of_the_years_their_rows_name() {
        for readme in READMES {
            let text =
                std::fs::read_to_string(readme).unwrap_or_else(|error| panic!("{readme}: {error}"));
            for export in ["hc_local_resolution", "hc_mktime"] {
                let row = row(&text, export);
                let stated = range_after(&row, "of the years");
                assert_eq!(&stated[..2], [-9_999_994, 9_999_994], "{readme}: {row}");
            }
        }
        let first = [-9_999_994, 1, 1, 0, 0, 0];
        let last = [9_999_994, 12, 31, 23, 59, 59];
        let before = [-9_999_995, 12, 31, 23, 59, 59];
        let after = [9_999_995, 1, 1, 0, 0, 0];
        for reading in [first, last] {
            assert!(local_resolution_line(reading, "UTC").is_ok(), "{reading:?}");
            assert!(mktime(reading, "UTC", "reject").is_ok(), "{reading:?}");
        }
        for reading in [before, after] {
            assert_eq!(
                local_resolution_line(reading, "UTC"),
                Err(Refusal::OutOfRange),
                "{reading:?}"
            );
            assert_eq!(
                mktime(reading, "UTC", "reject"),
                Err(Refusal::OutOfRange),
                "{reading:?}"
            );
        }
        // A second of 60 is refused: UTC inserted none on this day.
        assert_eq!(
            local_resolution_line([2000, 1, 1, 0, 0, 60], "UTC"),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(
            mktime([2000, 1, 1, 0, 0, 60], "UTC", "reject"),
            Err(Refusal::InvalidDate)
        );
    }
}

/// The sky exports of the wave that followed the Hindu ones: their rows state
/// the POSIX seconds of the years −1000 to 3000, and their spans are at most a
/// hundred Julian years.
mod wave_e_sky {
    use super::*;
    use hyper_calendar::astro_lines::equation_of_time_line;
    use hyper_calendar::jupiter_lines::{ingress_lines, jupiter_line, rising_lines};
    use hyper_calendar::panchanga_lines::{
        nakshatra_at_lines, panchanga_at_lines, rahu_at_line, rahu_ingresses_lines,
        solar_nakshatra_ingresses_lines,
    };

    /// `3 155 760 000` seconds, a hundred Julian years.
    const HUNDRED_YEARS: i64 = 3_155_760_000;

    #[test]
    fn the_instants_of_the_wave_e_sky_exports_are_the_years_their_rows_name() {
        let (first, last) = era();
        for export in [
            "hc_rahu_at",
            "hc_panchanga_at",
            "hc_nakshatra_at",
            "hc_equation_of_time",
            "hc_jupiter_at",
        ] {
            assert_states(export, "`unix_seconds`", first, last);
        }
        for unix in [first, last] {
            assert!(rahu_at_line(unix, "lahiri").is_ok(), "rahu {unix}");
            assert!(
                panchanga_at_lines(unix, "lahiri").is_ok(),
                "panchanga {unix}"
            );
            assert!(
                nakshatra_at_lines(unix, "lahiri").is_ok(),
                "nakshatra {unix}"
            );
            assert!(equation_of_time_line(unix).is_ok(), "equation {unix}");
            assert!(jupiter_line(unix, "lahiri").is_ok(), "jupiter {unix}");
        }
        for unix in [first - 1, last + 1] {
            assert_eq!(rahu_at_line(unix, "lahiri"), Err(Refusal::OutOfRange));
            assert_eq!(panchanga_at_lines(unix, "lahiri"), Err(Refusal::OutOfRange));
            assert_eq!(nakshatra_at_lines(unix, "lahiri"), Err(Refusal::OutOfRange));
            assert_eq!(equation_of_time_line(unix), Err(Refusal::OutOfRange));
            assert_eq!(jupiter_line(unix, "lahiri"), Err(Refusal::OutOfRange));
        }
    }

    #[test]
    fn the_ingress_spans_of_the_wave_e_sky_exports_are_the_spans_their_rows_name() {
        let (first, last) = era();
        for export in [
            "hc_rahu_ingresses",
            "hc_solar_nakshatra_ingresses",
            "hc_jupiter_ingresses",
            "hc_jupiter_risings",
        ] {
            assert_states(export, "`from_unix_seconds`", first, last + 1);
        }
        type Span = fn(i64, i64, &str) -> Answer<String>;
        let spans: [(&str, Span); 4] = [
            ("rahu ingresses", rahu_ingresses_lines),
            ("solar nakshatra ingresses", solar_nakshatra_ingresses_lines),
            ("jupiter ingresses", ingress_lines),
            ("jupiter risings", rising_lines),
        ];
        for (name, span) in spans {
            assert!(
                span(first, first + HUNDRED_YEARS, "lahiri").is_ok(),
                "{name}"
            );
            assert!(
                span(last + 1 - HUNDRED_YEARS, last + 1, "lahiri").is_ok(),
                "{name}"
            );
            // A `to` not after the `from` writes nothing, and is no error.
            assert!(span(first + 5, first + 5, "lahiri").is_ok(), "{name}");
            assert_eq!(
                span(first, first + HUNDRED_YEARS + 1, "lahiri"),
                Err(Refusal::OutOfRange),
                "{name}"
            );
            assert_eq!(
                span(first - 1, first, "lahiri"),
                Err(Refusal::OutOfRange),
                "{name}"
            );
            assert_eq!(
                span(last + 1, last + 2, "lahiri"),
                Err(Refusal::OutOfRange),
                "{name}"
            );
        }
    }
}
