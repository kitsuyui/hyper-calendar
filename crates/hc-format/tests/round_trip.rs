//! Property tests: formatting a value and parsing the text back must give
//! the same value, for every value the crate claims to handle.
//!
//! # Why a hand-rolled generator
//!
//! The workspace takes no external dependencies, so there is no `proptest`
//! here. The generator below is a 64-bit linear congruential generator with
//! the multiplier Knuth gives in *The Art of Computer Programming* vol. 2
//! §3.3.4 — it is not a good source of randomness and does not need to be.
//! What a property test needs is a *reproducible* walk over a large space,
//! and a fixed seed gives that: a failure found in CI reproduces exactly on
//! a developer's machine.
//!
//! The exhaustive loops matter more than the random ones. Every day from
//! 1583 to 2400 goes through every date form, which is 300 000 conversions
//! and catches the off-by-one that a thousand random samples would miss.

use hc_format::hc_calendar::{CivilDateTime, CivilTime, Rd};
use hc_format::hc_calendars_solar::{gregorian, iso_week, ordinal};
use hc_format::hc_tz::{OffsetStyle, UtcOffset};
use hc_format::iso8601::{self, Strictness, duration, interval};
use hc_format::patterns::{FormatContext, cldr, strftime};
use hc_format::value::{
    DateParts, DecimalMark, Fraction, IsoDate, IsoDateTime, IsoTime, OffsetDateTime, Style,
    YearStyle, ZoneInfo,
};
use hc_format::{format_to_string, rfc2822, rfc3339};

/// 1583-01-01, the first full year after the Gregorian reform.
const FIRST_DAY: i64 = 577_814;
/// 2400-12-31, two more 400-year cycles out than anyone needs.
const LAST_DAY: i64 = 876_582;

/// Knuth's LCG. Reproducible, not random.
struct Generator(u64);

impl Generator {
    const fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 11
    }

    fn below(&mut self, limit: u64) -> u64 {
        self.next() % limit
    }
}

#[track_caller]
fn render(value: &IsoDateTime) -> String {
    match format_to_string(value) {
        Ok(text) => text,
        Err(error) => panic!("no ISO 8601 spelling for {value:?}: {error}"),
    }
}

#[test]
fn the_reform_and_the_far_future_are_both_inside_the_tested_range() {
    assert_eq!(gregorian::from_fixed(Rd(FIRST_DAY)).unwrap(), (1583, 1, 1));
    assert_eq!(gregorian::from_fixed(Rd(LAST_DAY)).unwrap(), (2400, 12, 31));
}

#[test]
fn every_calendar_date_for_eight_centuries_round_trips_in_both_formats() {
    for day in FIRST_DAY..=LAST_DAY {
        let rd = Rd(day);
        let (year, month, dom) = gregorian::from_fixed(rd).unwrap();
        for style in [Style::Extended, Style::Basic] {
            let date = IsoDate {
                parts: DateParts::Calendar {
                    year,
                    month: Some(month),
                    day: Some(dom),
                },
                style,
                year_style: YearStyle::Plain,
            };
            let text = format_to_string(&date).unwrap();
            let back = iso8601::parse_date(&text).unwrap();
            assert_eq!(back, date, "{text}");
            assert_eq!(back.to_fixed().unwrap(), rd, "{text}");
        }
    }
}

#[test]
fn every_ordinal_date_for_eight_centuries_round_trips_in_both_formats() {
    for day in FIRST_DAY..=LAST_DAY {
        let rd = Rd(day);
        let (year, day_of_year) = ordinal::from_fixed(rd).unwrap();
        for style in [Style::Extended, Style::Basic] {
            let date = IsoDate {
                parts: DateParts::Ordinal { year, day_of_year },
                style,
                year_style: YearStyle::Plain,
            };
            let text = format_to_string(&date).unwrap();
            let back = iso8601::parse_date(&text).unwrap();
            assert_eq!(back, date, "{text}");
            assert_eq!(back.to_fixed().unwrap(), rd, "{text}");
        }
    }
}

#[test]
fn every_week_date_for_eight_centuries_round_trips_in_both_formats() {
    for day in FIRST_DAY..=LAST_DAY {
        let rd = Rd(day);
        let (year, week, weekday) = iso_week::from_fixed(rd).unwrap();
        for style in [Style::Extended, Style::Basic] {
            let date = IsoDate {
                parts: DateParts::Week {
                    year,
                    week,
                    weekday: Some(weekday),
                },
                style,
                year_style: YearStyle::Plain,
            };
            let text = format_to_string(&date).unwrap();
            let back = iso8601::parse_date(&text).unwrap();
            assert_eq!(back, date, "{text}");
            assert_eq!(back.to_fixed().unwrap(), rd, "{text}");
        }
    }
}

#[test]
fn the_three_date_forms_always_name_the_same_day() {
    for day in FIRST_DAY..=LAST_DAY {
        let rd = Rd(day);
        let (year, month, dom) = gregorian::from_fixed(rd).unwrap();
        let (ordinal_year, day_of_year) = ordinal::from_fixed(rd).unwrap();
        let (week_year, week, weekday) = iso_week::from_fixed(rd).unwrap();
        let calendar = format!("{year:04}-{month:02}-{dom:02}");
        let ordinal_text = format!("{ordinal_year:04}-{day_of_year:03}");
        let week_text = format!("{week_year:04}-W{week:02}-{weekday}");
        assert_eq!(
            iso8601::parse_date(&calendar).unwrap().to_fixed().unwrap(),
            rd
        );
        assert_eq!(
            iso8601::parse_date(&ordinal_text)
                .unwrap()
                .to_fixed()
                .unwrap(),
            rd
        );
        assert_eq!(
            iso8601::parse_date(&week_text).unwrap().to_fixed().unwrap(),
            rd
        );
    }
}

#[test]
fn expanded_years_round_trip_at_every_digit_count_they_are_written_with() {
    let mut generator = Generator::new(0x5EED_0001);
    for _ in 0..5_000 {
        let year = generator.below(2_000_000) as i64 - 1_000_000;
        let digits = 6 + (generator.below(4) as u8);
        let date = IsoDate {
            parts: DateParts::Calendar {
                year,
                month: Some(1 + generator.below(12) as u8),
                day: None,
            },
            style: Style::Extended,
            year_style: YearStyle::Expanded(digits),
        };
        let text = format_to_string(&date).unwrap();
        assert_eq!(iso8601::parse_date(&text).unwrap(), date, "{text}");
    }
}

#[test]
fn every_second_of_a_day_round_trips_as_a_time() {
    for seconds in 0..86_400u32 {
        let time = IsoTime {
            hour: (seconds / 3_600) as u8,
            minute: Some(((seconds % 3_600) / 60) as u8),
            second: Some((seconds % 60) as u8),
            fraction: None,
            style: Style::Extended,
            mark: DecimalMark::Point,
        };
        let text = format_to_string(&time).unwrap();
        let (back, zone, _) = iso8601::parse_time(&text).unwrap();
        assert_eq!(back, time, "{text}");
        assert_eq!(zone, ZoneInfo::Unspecified);
    }
}

#[test]
fn times_round_trip_at_every_accuracy_and_in_both_formats_and_both_decimal_marks() {
    let mut generator = Generator::new(0x5EED_0002);
    for _ in 0..20_000 {
        let hour = generator.below(24) as u8;
        let minute = generator.below(60) as u8;
        let second = generator.below(60) as u8;
        let digits = 1 + generator.below(18) as u8;
        let scale = 10u64.pow(u32::from(18 - digits));
        let fraction = Fraction::new(
            generator.below(10u64.pow(u32::from(digits))) * scale,
            digits,
        );
        let (minute, second) = match generator.below(3) {
            0 => (None, None),
            1 => (Some(minute), None),
            _ => (Some(minute), Some(second)),
        };
        let time = IsoTime {
            hour,
            minute,
            second,
            fraction: if generator.below(2) == 0 {
                fraction
            } else {
                None
            },
            style: if generator.below(2) == 0 {
                Style::Extended
            } else {
                Style::Basic
            },
            mark: if generator.below(2) == 0 {
                DecimalMark::Point
            } else {
                DecimalMark::Comma
            },
        };
        // The basic format has no spelling for an hour on its own that
        // differs from the extended one, so both parse back identically.
        let text = format_to_string(&time).unwrap();
        let (back, _, _) = iso8601::parse_time(&text).unwrap();
        // Two shapes leave no trace in the text, so the parse cannot
        // recover them: an hour on its own is spelled the same in both
        // formats, and a decimal mark that no fraction used was never
        // written.
        let expected = IsoTime {
            style: if time.minute.is_none() {
                Style::Extended
            } else {
                time.style
            },
            mark: if time.fraction.is_some() {
                time.mark
            } else {
                DecimalMark::Point
            },
            ..time
        };
        assert_eq!(back, expected, "{text}");
    }
}

#[test]
fn a_fractional_component_always_names_the_span_it_should() {
    // A fraction of an hour, a minute and a second must all resolve to the
    // same elapsed span when they name the same point.
    let half_hour = iso8601::parse("2026-09-21T14.5").unwrap();
    let half_minute = iso8601::parse("2026-09-21T14:30.5").unwrap();
    let half_second = iso8601::parse("2026-09-21T14:30:30.5").unwrap();
    assert_eq!(
        half_hour.time.unwrap().since_midnight(),
        hc_format::hc_core::Duration::from_secs(14 * 3_600 + 30 * 60)
    );
    assert_eq!(
        half_minute.time.unwrap().since_midnight(),
        hc_format::hc_core::Duration::from_secs(14 * 3_600 + 30 * 60 + 30)
    );
    assert_eq!(
        half_second.time.unwrap().since_midnight(),
        hc_format::hc_core::Duration::from_millis((14 * 3_600 + 30 * 60 + 30) * 1_000 + 500)
    );
}

#[test]
fn every_whole_minute_offset_round_trips_in_every_style() {
    for minutes in -(26 * 60)..=(26 * 60) {
        let Ok(offset) = UtcOffset::from_seconds(minutes * 60) else {
            continue;
        };
        for style in [
            OffsetStyle::Extended,
            OffsetStyle::Basic,
            OffsetStyle::ExtendedSeconds,
            OffsetStyle::BasicSeconds,
        ] {
            let text = offset.format(style).as_str().to_string();
            let (zone, back_style) = iso8601::parse_offset(&text).unwrap();
            assert_eq!(zone.offset().unwrap(), offset, "{text}");
            assert_eq!(back_style, style, "{text}");
        }
        if offset.abs_minutes() == 0 && offset.abs_seconds() == 0 && !offset.is_negative() {
            let text = offset.format(OffsetStyle::Hours).as_str().to_string();
            let (zone, style) = iso8601::parse_offset(&text).unwrap();
            assert_eq!(zone.offset().unwrap(), offset, "{text}");
            assert_eq!(style, OffsetStyle::Hours);
        }
    }
}

#[test]
fn offsets_with_seconds_round_trip_too() {
    let mut generator = Generator::new(0x5EED_0003);
    for _ in 0..5_000 {
        let magnitude = generator.below(25 * 3_600) as i32;
        let signed = if generator.below(2) == 0 {
            magnitude
        } else {
            -magnitude
        };
        if signed == 0 {
            continue;
        }
        let offset = UtcOffset::from_seconds(signed).unwrap();
        for style in [OffsetStyle::ExtendedSeconds, OffsetStyle::BasicSeconds] {
            let text = offset.format(style).as_str().to_string();
            let (zone, _) = iso8601::parse_offset(&text).unwrap();
            assert_eq!(zone.offset().unwrap(), offset, "{text}");
        }
    }
}

#[test]
fn whole_date_times_round_trip_across_the_whole_space() {
    let mut generator = Generator::new(0x5EED_0004);
    for _ in 0..30_000 {
        let day = FIRST_DAY + generator.below((LAST_DAY - FIRST_DAY) as u64) as i64;
        let (year, month, dom) = gregorian::from_fixed(Rd(day)).unwrap();
        let style = if generator.below(2) == 0 {
            Style::Extended
        } else {
            Style::Basic
        };
        let zone = match generator.below(4) {
            0 => ZoneInfo::Unspecified,
            1 => ZoneInfo::Zulu,
            _ => ZoneInfo::Offset(
                UtcOffset::from_seconds((generator.below(50 * 4 + 1) as i32 - 25 * 4) * 15 * 60)
                    .unwrap_or(UtcOffset::UTC),
            ),
        };
        let zone_style = match generator.below(3) {
            0 => OffsetStyle::Extended,
            1 => OffsetStyle::Basic,
            _ => OffsetStyle::ExtendedSeconds,
        };
        let value = IsoDateTime {
            date: IsoDate {
                parts: DateParts::Calendar {
                    year,
                    month: Some(month),
                    day: Some(dom),
                },
                style,
                year_style: YearStyle::Plain,
            },
            time: Some(IsoTime {
                hour: generator.below(24) as u8,
                minute: Some(generator.below(60) as u8),
                second: Some(generator.below(60) as u8),
                fraction: None,
                style,
                mark: DecimalMark::Point,
            }),
            zone,
            zone_style,
        };
        let text = render(&value);
        let back = iso8601::parse(&text).unwrap();
        // `Z` and an absent designator carry no style, so a round trip
        // cannot report one.
        let expected = IsoDateTime {
            zone_style: if matches!(zone, ZoneInfo::Offset(_)) {
                value.zone_style
            } else {
                OffsetStyle::Extended
            },
            ..value
        };
        assert_eq!(back, expected, "{text}");
    }
}

#[test]
fn a_parsed_date_time_and_its_instant_agree_with_the_offset_arithmetic() {
    let mut generator = Generator::new(0x5EED_0005);
    for _ in 0..20_000 {
        let day = FIRST_DAY + generator.below((LAST_DAY - FIRST_DAY) as u64) as i64;
        let seconds = generator.below(86_400) as u32;
        let offset_quarters = generator.below(50 * 4 + 1) as i32 - 25 * 4;
        let offset = UtcOffset::from_seconds(offset_quarters * 15 * 60).unwrap();
        let local = CivilDateTime::new(
            Rd(day),
            CivilTime::hms(
                (seconds / 3_600) as u8,
                ((seconds % 3_600) / 60) as u8,
                (seconds % 60) as u8,
            )
            .unwrap(),
        );
        let value = OffsetDateTime::with_offset(local, offset);
        let unix = value.to_unix().unwrap();
        let expected = (day - 719_163) * 86_400 + i64::from(seconds) - i64::from(offset.seconds());
        assert_eq!(unix.seconds(), expected);
        let back = OffsetDateTime::from_unix(unix, ZoneInfo::Offset(offset)).unwrap();
        assert_eq!(back.local, local);
    }
}

#[test]
fn twenty_four_hundred_and_the_leap_second_both_survive_the_round_trip() {
    for text in [
        "2026-09-21T24:00",
        "2026-09-21T24:00:00",
        "2026-09-21T24:00:00Z",
        "20260921T240000+0900",
        "1972-06-30T23:59:60Z",
        "1972-12-31T23:59:60Z",
        "2016-12-31T23:59:60+00:00",
        "1972-06-30T23:59:60.5Z",
    ] {
        let value = iso8601::parse(text).unwrap();
        assert_eq!(render(&value), text);
    }
}

#[test]
fn the_first_leap_second_ever_inserted_lands_where_the_iers_says_it_does() {
    // IERS Bulletin C: the first leap second was inserted at the end of
    // 30 June 1972, so 1972-06-30T23:59:60Z is followed by
    // 1972-07-01T00:00:00Z.
    let leap = rfc3339::parse("1972-06-30T23:59:60Z").unwrap();
    let after = rfc3339::parse("1972-07-01T00:00:00Z").unwrap();
    assert!(leap.local.time.is_leap_second());
    assert_eq!(leap.to_unix().unwrap(), after.to_unix().unwrap());
    let instant = leap.to_utc_instant().unwrap();
    assert!(instant.leap_second);
    assert!(!after.to_utc_instant().unwrap().leap_second);
    assert_eq!(instant.unix_seconds, 78_796_800);
    let mut out = String::new();
    rfc3339::write(&mut out, leap, rfc3339::SubsecondPrecision::Auto).unwrap();
    assert_eq!(out, "1972-06-30T23:59:60Z");
}

#[test]
fn durations_round_trip_across_every_combination_of_components() {
    let mut generator = Generator::new(0x5EED_0006);
    for _ in 0..30_000 {
        let present = generator.below(128);
        let pick = |bit: u64, generator: &mut Generator| {
            if present & (1 << bit) == 0 {
                None
            } else {
                Some(generator.below(1_000))
            }
        };
        let mut value = hc_format::IsoDuration {
            negative: generator.below(2) == 0,
            years: pick(0, &mut generator),
            months: pick(1, &mut generator),
            weeks: pick(2, &mut generator),
            days: pick(3, &mut generator),
            hours: pick(4, &mut generator),
            minutes: pick(5, &mut generator),
            seconds: pick(6, &mut generator),
            ..hc_format::IsoDuration::default()
        };
        if present == 0 {
            value.seconds = Some(0);
        }
        if generator.below(2) == 0 {
            let digits = 1 + generator.below(9) as u8;
            let scale = 10u64.pow(u32::from(18 - digits));
            value.fraction = Fraction::new(
                generator.below(10u64.pow(u32::from(digits))) * scale,
                digits,
            );
            value.mark = if generator.below(2) == 0 {
                DecimalMark::Point
            } else {
                DecimalMark::Comma
            };
        }
        let text = format_to_string(&value).unwrap();
        let back = duration::parse(&text).unwrap();
        assert_eq!(back, value, "{text}");
    }
}

#[test]
fn every_exact_duration_agrees_with_the_span_it_names() {
    let mut generator = Generator::new(0x5EED_0007);
    for _ in 0..10_000 {
        let weeks = generator.below(100);
        let days = generator.below(100);
        let hours = generator.below(100);
        let minutes = generator.below(100);
        let seconds = generator.below(100);
        let value = hc_format::IsoDuration {
            weeks: Some(weeks),
            days: Some(days),
            hours: Some(hours),
            minutes: Some(minutes),
            seconds: Some(seconds),
            ..hc_format::IsoDuration::default()
        };
        let expected =
            (weeks * 604_800 + days * 86_400 + hours * 3_600 + minutes * 60 + seconds) as i128;
        assert_eq!(
            value.to_exact_duration().unwrap().whole_seconds(),
            expected,
            "{value}"
        );
        let text = format_to_string(&value).unwrap();
        assert_eq!(duration::parse(&text).unwrap(), value);
    }
}

#[test]
fn alternative_form_durations_round_trip_in_both_styles() {
    let mut generator = Generator::new(0x5EED_0008);
    for _ in 0..5_000 {
        let value = hc_format::IsoDuration {
            years: Some(generator.below(10_000)),
            months: Some(generator.below(100)),
            days: Some(generator.below(100)),
            hours: Some(generator.below(100)),
            minutes: Some(generator.below(100)),
            seconds: Some(generator.below(100)),
            form: hc_format::iso8601::DurationForm::Alternative,
            style: if generator.below(2) == 0 {
                Style::Extended
            } else {
                Style::Basic
            },
            ..hc_format::IsoDuration::default()
        };
        let text = format_to_string(&value).unwrap();
        assert_eq!(duration::parse(&text).unwrap(), value, "{text}");
    }
}

#[test]
fn every_interval_shape_round_trips() {
    let mut generator = Generator::new(0x5EED_0009);
    for _ in 0..5_000 {
        let day = FIRST_DAY + generator.below((LAST_DAY - FIRST_DAY) as u64) as i64;
        let (year, month, dom) = gregorian::from_fixed(Rd(day)).unwrap();
        let start = format!("{year:04}-{month:02}-{dom:02}T00:00:00Z");
        let end = format!("{year:04}-{month:02}-{dom:02}T23:59:59Z");
        let span = format!("PT{}H", generator.below(100));
        for text in [
            format!("{start}/{end}"),
            format!("{start}/{span}"),
            format!("{span}/{end}"),
            span.clone(),
            format!("R{}/{start}/{span}", generator.below(1_000)),
            format!("R/{span}"),
        ] {
            if text.starts_with('R') {
                let value = interval::parse_repeating(&text).unwrap();
                assert_eq!(format_to_string(&value).unwrap(), text);
            } else {
                let value = interval::parse(&text).unwrap();
                assert_eq!(format_to_string(&value).unwrap(), text);
            }
        }
    }
}

#[test]
fn rfc_3339_round_trips_over_the_whole_posix_range_it_can_express() {
    let mut generator = Generator::new(0x5EED_000A);
    for _ in 0..20_000 {
        let day = 693_596 + generator.below(120_000) as i64; // 1900-01-01 onwards
        let (year, month, dom) = gregorian::from_fixed(Rd(day)).unwrap();
        let seconds = generator.below(86_400) as u32;
        let offset_quarters = generator.below(50 * 4 + 1) as i32 - 25 * 4;
        let zone = match generator.below(3) {
            0 => ZoneInfo::Zulu,
            1 => ZoneInfo::UnknownLocalOffset,
            _ => ZoneInfo::Offset(
                UtcOffset::from_seconds(offset_quarters * 15 * 60).unwrap_or(UtcOffset::UTC),
            ),
        };
        let local = CivilDateTime::new(
            Rd(day),
            CivilTime::hms(
                (seconds / 3_600) as u8,
                ((seconds % 3_600) / 60) as u8,
                (seconds % 60) as u8,
            )
            .unwrap(),
        );
        let value = OffsetDateTime {
            local,
            zone,
            written_as_end_of_day: false,
        };
        let mut text = String::new();
        rfc3339::write(&mut text, value, rfc3339::SubsecondPrecision::Auto).unwrap();
        let back = rfc3339::parse(&text).unwrap();
        assert_eq!(back.local, value.local, "{text}");
        assert_eq!(back.zone, value.zone, "{text}");
        assert_eq!(
            (year, month, dom),
            gregorian::from_fixed(back.local.day).unwrap()
        );
    }
}

#[test]
fn rfc_3339_sub_second_precision_round_trips_at_every_digit_count() {
    let mut generator = Generator::new(0x5EED_000B);
    for digits in 1..=18u8 {
        for _ in 0..200 {
            let scale = 10u64.pow(u32::from(18 - digits));
            let attos = generator.below(10u64.pow(u32::from(digits))) * scale;
            let local = CivilDateTime::new(Rd(739_880), CivilTime::new(1, 2, 3, attos).unwrap());
            let value = OffsetDateTime {
                local,
                zone: ZoneInfo::Zulu,
                written_as_end_of_day: false,
            };
            let mut text = String::new();
            rfc3339::write(
                &mut text,
                value,
                rfc3339::SubsecondPrecision::Digits(digits),
            )
            .unwrap();
            let back = rfc3339::parse(&text).unwrap();
            assert_eq!(back.local.time.subsec_attos(), attos, "{text}");
        }
    }
}

#[test]
fn rfc_5322_round_trips_over_a_century() {
    let mut generator = Generator::new(0x5EED_000C);
    for _ in 0..20_000 {
        let day = 693_596 + generator.below(80_000) as i64;
        let seconds = generator.below(86_400) as u32;
        let offset_quarters = generator.below(50 * 4 + 1) as i32 - 25 * 4;
        let offset = UtcOffset::from_seconds(offset_quarters * 15 * 60).unwrap();
        let local = CivilDateTime::new(
            Rd(day),
            CivilTime::hms(
                (seconds / 3_600) as u8,
                ((seconds % 3_600) / 60) as u8,
                (seconds % 60) as u8,
            )
            .unwrap(),
        );
        let zone = if offset.is_utc() {
            ZoneInfo::Zulu
        } else {
            ZoneInfo::Offset(offset)
        };
        let value = OffsetDateTime {
            local,
            zone,
            written_as_end_of_day: false,
        };
        let mut text = String::new();
        rfc2822::write(&mut text, value).unwrap();
        let back = rfc2822::parse(&text).unwrap();
        assert_eq!(back.local, value.local, "{text}");
        assert_eq!(back.zone.offset(), value.zone.offset(), "{text}");
    }
}

#[test]
fn the_email_and_iso_spellings_of_the_same_instant_agree() {
    let mut generator = Generator::new(0x5EED_000D);
    for _ in 0..10_000 {
        let day = 693_596 + generator.below(80_000) as i64;
        let seconds = generator.below(86_400) as u32;
        let local = CivilDateTime::new(
            Rd(day),
            CivilTime::hms(
                (seconds / 3_600) as u8,
                ((seconds % 3_600) / 60) as u8,
                (seconds % 60) as u8,
            )
            .unwrap(),
        );
        let value = OffsetDateTime {
            local,
            zone: ZoneInfo::Zulu,
            written_as_end_of_day: false,
        };
        let mut email = String::new();
        rfc2822::write(&mut email, value).unwrap();
        let mut iso = String::new();
        rfc3339::write(&mut iso, value, rfc3339::SubsecondPrecision::Auto).unwrap();
        assert_eq!(
            rfc2822::parse(&email).unwrap().to_unix().unwrap(),
            rfc3339::parse(&iso).unwrap().to_unix().unwrap()
        );
    }
}

#[test]
fn strftime_format_then_parse_is_the_identity_over_the_whole_range() {
    let mut generator = Generator::new(0x5EED_000E);
    for _ in 0..20_000 {
        let day = 577_813 + generator.below(298_000) as i64;
        let seconds = generator.below(86_400) as u32;
        let local = CivilDateTime::new(
            Rd(day),
            CivilTime::hms(
                (seconds / 3_600) as u8,
                ((seconds % 3_600) / 60) as u8,
                (seconds % 60) as u8,
            )
            .unwrap(),
        );
        for pattern in ["%Y-%m-%d %H:%M:%S", "%Y-%j %T", "%G-W%V-%u %T", "%F %T"] {
            let mut text = String::new();
            strftime::format(&mut text, pattern, &FormatContext::new(local)).unwrap();
            let parsed = strftime::parse(pattern, &text).unwrap();
            let back = parsed.to_offset_date_time().unwrap();
            assert_eq!(back.local, local, "{pattern} -> {text}");
        }
    }
}

#[test]
fn cldr_format_then_parse_is_the_identity_over_the_whole_range() {
    let mut generator = Generator::new(0x5EED_000F);
    for _ in 0..20_000 {
        let day = 577_813 + generator.below(298_000) as i64;
        let seconds = generator.below(86_400) as u32;
        let local = CivilDateTime::new(
            Rd(day),
            CivilTime::hms(
                (seconds / 3_600) as u8,
                ((seconds % 3_600) / 60) as u8,
                (seconds % 60) as u8,
            )
            .unwrap(),
        );
        for pattern in [
            "yyyy-MM-dd HH:mm:ss",
            "yyyy-MM-dd'T'HH:mm:ss",
            "d MMMM yyyy HH:mm:ss",
            "yyyy-DDD HH:mm:ss",
        ] {
            let mut text = String::new();
            cldr::format(&mut text, pattern, &FormatContext::new(local)).unwrap();
            let parsed = cldr::parse(pattern, &text).unwrap();
            let back = parsed.to_offset_date_time().unwrap();
            assert_eq!(back.local, local, "{pattern} -> {text}");
        }
    }
}

#[test]
fn strftime_and_iso_8601_never_disagree_about_a_date() {
    for day in (FIRST_DAY..=LAST_DAY).step_by(7) {
        let local = CivilDateTime::midnight(Rd(day));
        let mut text = String::new();
        strftime::format(&mut text, "%Y-%m-%d", &FormatContext::new(local)).unwrap();
        assert_eq!(
            iso8601::parse_date(&text).unwrap().to_fixed().unwrap(),
            Rd(day)
        );
        let mut week = String::new();
        strftime::format(&mut week, "%G-W%V-%u", &FormatContext::new(local)).unwrap();
        assert_eq!(
            iso8601::parse_date(&week).unwrap().to_fixed().unwrap(),
            Rd(day)
        );
        let mut ordinal_text = String::new();
        strftime::format(&mut ordinal_text, "%Y-%j", &FormatContext::new(local)).unwrap();
        assert_eq!(
            iso8601::parse_date(&ordinal_text)
                .unwrap()
                .to_fixed()
                .unwrap(),
            Rd(day)
        );
    }
}

#[test]
fn the_sniffing_front_door_agrees_with_the_grammar_it_picked() {
    let mut generator = Generator::new(0x5EED_0010);
    for _ in 0..5_000 {
        let day = 693_596 + generator.below(120_000) as i64;
        let (year, month, dom) = gregorian::from_fixed(Rd(day)).unwrap();
        let iso = format!("{year:04}-{month:02}-{dom:02}T12:00:00Z");
        let value = OffsetDateTime {
            local: CivilDateTime::new(Rd(day), CivilTime::hms(12, 0, 0).unwrap()),
            zone: ZoneInfo::Zulu,
            written_as_end_of_day: false,
        };
        let mut email = String::new();
        rfc2822::write(&mut email, value).unwrap();
        assert_eq!(hc_format::parse::day(&iso).unwrap(), Rd(day));
        assert_eq!(hc_format::parse::day(&email).unwrap(), Rd(day));
        assert_eq!(
            hc_format::parse::date_time(&iso)
                .unwrap()
                .to_unix()
                .unwrap(),
            hc_format::parse::date_time(&email)
                .unwrap()
                .to_unix()
                .unwrap()
        );
    }
}

#[test]
fn every_error_path_reports_the_offset_the_documentation_promises() {
    use hc_format::ErrorKind;
    let cases: &[(&str, ErrorKind, usize)] = &[
        ("", ErrorKind::Empty, 0),
        ("20xx-01-01", ErrorKind::DigitCount(4), 2),
        ("2026-", ErrorKind::DigitCount(2), 5),
        ("2026-1", ErrorKind::DigitCount(2), 5),
        ("2026-13-01", ErrorKind::OutOfRange("month"), 5),
        ("2026-00-01", ErrorKind::OutOfRange("month"), 5),
        ("2026-02-30", ErrorKind::OutOfRange("day"), 8),
        ("2026-09-00", ErrorKind::OutOfRange("day"), 8),
        ("2026-000", ErrorKind::OutOfRange("day of year"), 5),
        ("2026-366", ErrorKind::OutOfRange("day of year"), 5),
        ("2025-W53-1", ErrorKind::OutOfRange("week"), 6),
        ("2026-W00-1", ErrorKind::OutOfRange("week"), 6),
        ("2026-W38-0", ErrorKind::OutOfRange("weekday"), 9),
        ("2026-09-21T25:00:00", ErrorKind::OutOfRange("hour"), 11),
        ("2026-09-21T14:60:00", ErrorKind::OutOfRange("minute"), 14),
        ("2026-09-21T14:30:61", ErrorKind::OutOfRange("second"), 17),
        (
            "2026-09-21T12:59:60",
            ErrorKind::Invalid("a leap second outside 23:59"),
            17,
        ),
        (
            "2026-09-21T24:00:01",
            ErrorKind::Invalid("24:00 with a non-zero component"),
            11,
        ),
        ("2026-09-21T14:30:05.", ErrorKind::Digit, 20),
        (
            "2026-09-21T14:30:05+99:00",
            ErrorKind::OutOfRange("UTC offset"),
            19,
        ),
        (
            "2026-09-21T14:30:05+09:99",
            ErrorKind::OutOfRange("offset minutes"),
            23,
        ),
        (
            "2026-09-21T14:30:05-00:00",
            ErrorKind::Forbidden("the negative zero UTC offset"),
            19,
        ),
        ("2026-09-21T14:30:05Z!", ErrorKind::TrailingText, 20),
        ("2026-0921", ErrorKind::MixedFormat, 5),
        (
            "202609",
            ErrorKind::Forbidden("a month-accuracy date in the basic format"),
            4,
        ),
    ];
    for (text, kind, offset) in cases {
        let error = iso8601::parse(text).unwrap_err();
        assert_eq!(error.kind(), *kind, "{text}");
        assert_eq!(error.offset(), *offset, "{text}");
    }
}

#[test]
fn full_strictness_refuses_everything_it_documents_and_nothing_else() {
    let accepted = [
        "2026-09-21T14:30:05Z",
        "2026-09-21T14:30:05+09:00",
        "2026-09-21T14:30:05.5+09:00",
        "+002026-09-21T14:30:05Z",
    ];
    let refused = [
        "2026",
        "2026-09",
        "2026-09-21",
        "20260921T143005Z",
        "2026-09-21T14:30Z",
        "2026-09-21T14:30:05",
        "2026-09-21T24:00:00Z",
        "2026-09-21T14:30:05,5Z",
        "2026-09-21T14:30:05+09",
    ];
    for text in accepted {
        assert!(
            iso8601::parse_with(text, Strictness::FULL).is_ok(),
            "should accept {text}"
        );
    }
    for text in refused {
        assert!(
            iso8601::parse_with(text, Strictness::FULL).is_err(),
            "should refuse {text}"
        );
        assert!(
            iso8601::parse_with(text, Strictness::ISO).is_ok(),
            "ISO should accept {text}"
        );
    }
}
