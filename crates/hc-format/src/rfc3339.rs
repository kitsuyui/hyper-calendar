//! RFC 3339, the internet profile of ISO 8601.
//!
//! RFC 3339 is ISO 8601 with almost everything taken away: one date form,
//! one time form, a mandatory zone, no basic format, no reduced accuracy, no
//! expanded years, no `24:00`. That narrowness is the point — two
//! implementations that both claim RFC 3339 will agree, which is not true of
//! two that both claim ISO 8601.
//!
//! # `-00:00` is not `+00:00`
//!
//! RFC 3339 §4.3 gives `-00:00` a meaning ISO 8601 does not have: the
//! instant is known and is being stated in UTC, but the offset of the zone
//! it was generated in is *unknown*. `+00:00` asserts the opposite — that the
//! generating zone really is at UTC. Mail systems depend on the difference,
//! so [`ZoneInfo::UnknownLocalOffset`] keeps them apart instead of
//! normalising one into the other.
//!
//! ```
//! use hc_format::{ZoneInfo, rfc3339};
//!
//! assert_eq!(rfc3339::parse("2026-09-21T14:30:05-00:00")?.zone, ZoneInfo::UnknownLocalOffset);
//! assert_eq!(rfc3339::parse("2026-09-21T14:30:05+00:00")?.zone.offset().map(|o| o.seconds()), Some(0));
//! # Ok::<(), hc_format::ParseError>(())
//! ```

use core::fmt;

use hc_calendar::CivilDateTime;
use hc_core::UnixTime;
use hc_tz::{OffsetStyle, UtcOffset};

use crate::error::{ErrorKind, FormatError, FormatResult, ParseError, ParseResult};
use crate::iso8601::{self, Strictness};
use crate::value::{
    DecimalMark, Fraction, IsoDate, IsoDateTime, IsoTime, OffsetDateTime, Style, YearStyle,
    ZoneInfo,
};

/// How many sub-second digits to write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SubsecondPrecision {
    /// As many as the value needs and no more; none at all when the value is
    /// a whole second.
    #[default]
    Auto,
    /// Whole seconds: any remainder is dropped.
    Seconds,
    /// Exactly this many digits, `1..=18`. Values outside that range are
    /// clamped into it.
    Digits(u8),
}

impl SubsecondPrecision {
    /// Three digits: `2026-09-21T14:30:05.123Z`.
    pub const MILLISECONDS: Self = Self::Digits(3);
    /// Six digits.
    pub const MICROSECONDS: Self = Self::Digits(6);
    /// Nine digits.
    pub const NANOSECONDS: Self = Self::Digits(9);

    fn fraction_of(self, attos: u64) -> Option<Fraction> {
        match self {
            Self::Auto => Fraction::minimal(attos),
            Self::Seconds => None,
            Self::Digits(digits) => {
                let digits = digits.clamp(1, 18);
                let scale = 10u64.pow(u32::from(18 - digits));
                Fraction::new(attos / scale * scale, digits)
            }
        }
    }
}

/// Parse an RFC 3339 timestamp.
///
/// The result always carries a zone, because the profile always does.
///
/// # Errors
///
/// See [`ParseError`]. Anything ISO 8601 allows and RFC 3339 does not —
/// `20260921T143005Z`, `2026-09-21`, `2026-09-21T14:30:05` — fails with
/// [`ErrorKind::Forbidden`] naming which relaxation was used.
pub fn parse(text: &str) -> ParseResult<OffsetDateTime> {
    let value = parse_iso(text)?;
    value.to_offset_date_time().map_err(|_| {
        // Under `Strictness::RFC_3339` the value is always a complete
        // date-time, so this arm is unreachable for any input that parsed.
        ParseError::new(ErrorKind::Unrepresentable("the timestamp"), 0)
    })
}

/// Parse an RFC 3339 timestamp, keeping the shape it was written in.
///
/// # Errors
///
/// See [`parse`].
pub fn parse_iso(text: &str) -> ParseResult<IsoDateTime> {
    iso8601::parse_with(text, Strictness::RFC_3339)
}

/// Write a value as an RFC 3339 timestamp.
///
/// # Errors
///
/// [`FormatError::Unrepresentable`] when the value carries no zone, its
/// year is outside `0000..=9999`, or its offset has seconds, none of which
/// RFC 3339 has a spelling for: its `time-numoffset` is `+hh:mm`.
pub fn write<W: fmt::Write>(
    out: &mut W,
    value: OffsetDateTime,
    precision: SubsecondPrecision,
) -> FormatResult<()> {
    if !value.zone.is_qualified() {
        return Err(FormatError::Unrepresentable(
            "an RFC 3339 timestamp with no offset",
        ));
    }
    let date = IsoDate::from_fixed(value.local.day)
        .map_err(|_| FormatError::Unrepresentable("the date"))?;
    if !matches!(date.year_style, YearStyle::Plain) {
        return Err(FormatError::Unrepresentable(
            "a year outside 0000..=9999 in RFC 3339",
        ));
    }
    date.write(out)?;
    out.write_char('T')?;
    let time = IsoTime {
        hour: value.local.time.hour(),
        minute: Some(value.local.time.minute()),
        second: Some(value.local.time.second()),
        fraction: precision.fraction_of(value.local.time.subsec_attos()),
        style: Style::Extended,
        mark: DecimalMark::Point,
    };
    time.write(out)?;
    value.zone.write(out, OffsetStyle::Extended)
}

/// Write a POSIX timestamp as an RFC 3339 timestamp at a stated offset.
///
/// # Errors
///
/// See [`write`](fn@write), plus [`FormatError::Unrepresentable`] when the
/// instant is outside the Gregorian range.
pub fn write_unix<W: fmt::Write>(
    out: &mut W,
    unix: UnixTime,
    offset: UtcOffset,
    precision: SubsecondPrecision,
) -> FormatResult<()> {
    let zone = if offset.is_utc() {
        ZoneInfo::Zulu
    } else {
        ZoneInfo::Offset(offset)
    };
    let value = OffsetDateTime::from_unix(unix, zone)
        .map_err(|_| FormatError::Unrepresentable("the instant"))?;
    write(out, value, precision)
}

/// Write a local civil reading and a zone as an RFC 3339 timestamp.
///
/// # Errors
///
/// See [`write`](fn@write).
pub fn write_civil<W: fmt::Write>(
    out: &mut W,
    local: CivilDateTime,
    zone: ZoneInfo,
    precision: SubsecondPrecision,
) -> FormatResult<()> {
    write(
        out,
        OffsetDateTime {
            local,
            zone,
            written_as_end_of_day: false,
        },
        precision,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use hc_calendar::{CivilTime, Rd};

    fn render(text: &str, precision: SubsecondPrecision) -> String {
        let value = parse(text).unwrap();
        let mut out = String::new();
        write(&mut out, value, precision).unwrap();
        out
    }

    #[test]
    fn the_canonical_timestamp_round_trips() {
        assert_eq!(
            render("2026-09-21T14:30:05Z", SubsecondPrecision::Auto),
            "2026-09-21T14:30:05Z"
        );
        assert_eq!(
            render("2026-09-21T14:30:05+09:00", SubsecondPrecision::Auto),
            "2026-09-21T14:30:05+09:00"
        );
    }

    #[test]
    fn the_unknown_local_offset_round_trips_as_itself() {
        assert_eq!(
            render("2026-09-21T14:30:05-00:00", SubsecondPrecision::Auto),
            "2026-09-21T14:30:05-00:00"
        );
        assert_eq!(
            parse("2026-09-21T14:30:05-00:00").unwrap().zone,
            ZoneInfo::UnknownLocalOffset
        );
    }

    #[test]
    fn plus_zero_and_minus_zero_name_the_same_instant_and_different_facts() {
        let plus = parse("2026-09-21T14:30:05+00:00").unwrap();
        let minus = parse("2026-09-21T14:30:05-00:00").unwrap();
        assert_eq!(plus.to_unix().unwrap(), minus.to_unix().unwrap());
        assert_ne!(plus.zone, minus.zone);
    }

    #[test]
    fn the_1972_leap_second_parses_and_round_trips() {
        let text = "1972-06-30T23:59:60Z";
        assert_eq!(render(text, SubsecondPrecision::Auto), text);
        let value = parse(text).unwrap();
        assert!(value.local.time.is_leap_second());
        let instant = value.to_utc_instant().unwrap();
        assert!(instant.leap_second);
        // 1972-07-01T00:00:00Z is 78 796 800 seconds after the POSIX epoch;
        // the leap second before it collapses to the same timestamp.
        assert_eq!(instant.unix_seconds, 78_796_800);
    }

    /// RFC 3339 §5.7 allows a second 60 at the end of June or December of
    /// a year that had a leap second; the table `hc_core::leap` is built
    /// from, the IANA `leap-seconds.list` (`iana-leap-seconds-list`), has
    /// them at the ends of 27 UTC days, the last of them 2016-12-31.
    #[test]
    fn a_second_60_is_read_only_on_a_day_that_ended_in_a_leap_second() {
        for text in [
            "1972-06-30T23:59:60Z",
            "2015-06-30T23:59:60Z",
            "2016-12-31T23:59:60Z",
            "2016-12-31T23:59:60.5+00:00",
        ] {
            assert!(parse(text).is_ok(), "{text}");
        }
        // An ordinary day, the last day of a half year that had none, the
        // first second of the day after one, and the days after the table.
        for text in [
            "2026-09-21T23:59:60Z",
            "2016-06-30T23:59:60Z",
            "2015-12-31T23:59:60Z",
            "2017-01-01T23:59:60Z",
        ] {
            let error = parse(text).unwrap_err();
            assert_eq!(
                error.kind(),
                ErrorKind::Invalid("a leap second that UTC did not insert"),
                "{text}"
            );
        }
        let error = parse("2030-12-31T23:59:60Z").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Unrepresentable("a leap second after the end of the leap-second table")
        );
    }

    /// The second 60 falls at 23:59:60 UTC, which a zone east or west of UTC
    /// reads at another hour; a reading `23:59:60` at an offset is therefore
    /// no leap second, whatever the day.
    #[test]
    fn a_second_60_at_a_non_zero_offset_is_refused() {
        for text in [
            "2016-12-31T23:59:60+01:00",
            "1972-06-30T23:59:60+09:00",
            "2016-12-31T23:59:60-00:30",
        ] {
            assert_eq!(
                parse(text).unwrap_err().kind(),
                ErrorKind::Invalid("a leap second that UTC did not insert"),
                "{text}"
            );
        }
    }

    /// RFC 3339 §5.7: "in time zones other than 'Z', the leap second point is
    /// shifted by the zone offset (so it happens at the same instant around
    /// the globe)"; §5.8 gives `1990-12-31T15:59:60-08:00` as "the same leap
    /// second in Pacific Standard Time" as `1990-12-31T23:59:60Z`. Japan read
    /// the end of 2016 as `2017-01-01T08:59:60+09:00`.
    #[test]
    fn the_leap_second_is_read_at_the_zones_own_clock() {
        for (local, utc, unix) in [
            (
                "1990-12-31T15:59:60-08:00",
                "1990-12-31T23:59:60Z",
                662_688_000,
            ),
            (
                "2017-01-01T08:59:60+09:00",
                "2016-12-31T23:59:60Z",
                1_483_228_800,
            ),
            (
                "1990-12-31T20:29:60-03:30",
                "1990-12-31T23:59:60Z",
                662_688_000,
            ),
            ("1990-12-31T23:59:60Z", "1990-12-31T23:59:60Z", 662_688_000),
        ] {
            assert_eq!(render(local, SubsecondPrecision::Auto), local);
            let value = parse(local).unwrap();
            assert!(value.local.time.is_leap_second(), "{local}");
            let instant = value.to_utc_instant().unwrap();
            assert!(instant.leap_second, "{local}");
            assert_eq!(instant.unix_seconds, unix, "{local}");
            assert_eq!(parse(utc).unwrap().to_utc_instant().unwrap(), instant);
        }
        // The same wall clock a minute off, a day off, or in the wrong
        // zone is no leap second.
        for text in [
            "1990-12-31T15:58:60-08:00",
            "1990-12-31T16:00:60-08:00",
            "1990-12-30T15:59:60-08:00",
            "1991-01-01T15:59:60-08:00",
            "1990-12-31T15:59:60+08:00",
            "1990-12-31T15:59:60Z",
        ] {
            assert_eq!(
                parse(text).unwrap_err().kind(),
                ErrorKind::Invalid("a leap second that UTC did not insert"),
                "{text}"
            );
        }
    }

    /// RFC 3339 §5.6: `full-date` is `date-fullyear "-" date-month "-"
    /// date-mday`, and `time-numoffset` is `("+" / "-") time-hour ":"
    /// time-minute`, an hour of 00 through 23 and no seconds.
    #[test]
    fn the_profile_refuses_what_its_grammar_has_no_production_for() {
        for (text, kind) in [
            (
                "2026-W39-1T14:30:05Z",
                ErrorKind::Forbidden("a week date or an ordinal date"),
            ),
            (
                "2026-264T14:30:05Z",
                ErrorKind::Forbidden("a week date or an ordinal date"),
            ),
            (
                "2026-09-21T14:30:05+24:00",
                ErrorKind::OutOfRange("offset hours"),
            ),
            (
                "2026-09-21T14:30:05+25:59",
                ErrorKind::OutOfRange("offset hours"),
            ),
            (
                "2026-09-21T14:30:05+05:30:15",
                ErrorKind::Forbidden("seconds in a UTC offset"),
            ),
        ] {
            assert_eq!(parse(text).unwrap_err().kind(), kind, "{text}");
        }
        assert!(parse("2026-09-21T14:30:05+23:59").is_ok());
        assert!(parse("2026-09-21T14:30:05-23:59").is_ok());
    }

    #[test]
    fn rfc_3339_refuses_the_basic_format() {
        let error = parse("20260921T143005Z").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Forbidden("a basic-format date"));
        assert_eq!(error.offset(), 4);
    }

    #[test]
    fn rfc_3339_refuses_a_date_with_no_time() {
        let error = parse("2026-09-21").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Forbidden("a date with no time of day")
        );
    }

    #[test]
    fn rfc_3339_refuses_an_unqualified_time() {
        let error = parse("2026-09-21T14:30:05").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Forbidden("an unqualified local time")
        );
        assert_eq!(error.offset(), 19);
    }

    #[test]
    fn rfc_3339_refuses_the_end_of_day_reading() {
        let error = parse("2026-09-21T24:00:00Z").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Forbidden("the 24:00 end-of-day reading")
        );
        assert_eq!(error.offset(), 11);
    }

    #[test]
    fn rfc_3339_tolerates_the_lowercase_designators_and_the_space() {
        assert!(parse("2026-09-21t14:30:05z").is_ok());
        assert!(parse("2026-09-21 14:30:05Z").is_ok());
    }

    #[test]
    fn rfc_3339_refuses_the_comma_decimal_mark() {
        let error = parse("2026-09-21T14:30:05,5Z").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Forbidden("the comma decimal mark"));
        assert_eq!(error.offset(), 19);
    }

    #[test]
    fn sub_second_precision_is_the_writers_choice() {
        let text = "2026-09-21T14:30:05.123456789Z";
        assert_eq!(render(text, SubsecondPrecision::Auto), text);
        assert_eq!(
            render(text, SubsecondPrecision::MILLISECONDS),
            "2026-09-21T14:30:05.123Z"
        );
        assert_eq!(
            render(text, SubsecondPrecision::Seconds),
            "2026-09-21T14:30:05Z"
        );
    }

    #[test]
    fn a_reading_with_no_zone_has_no_rfc_3339_spelling() {
        let value = OffsetDateTime::local(CivilDateTime::new(
            Rd(739_880),
            CivilTime::hms(14, 30, 5).unwrap(),
        ));
        let mut out = String::new();
        let error = write(&mut out, value, SubsecondPrecision::Auto).unwrap_err();
        assert_eq!(
            error,
            FormatError::Unrepresentable("an RFC 3339 timestamp with no offset")
        );
    }

    /// RFC 3339 §5.8 gives Amsterdam in 1937, whose local time was 00:19:32
    /// ahead of UTC, as `+00:20`: the grammar's `time-numoffset` is `+hh:mm`
    /// and has no seconds. Writing the wall clock of the whole offset with
    /// `+00:20` would read back 28 seconds off, so a writer refuses an offset
    /// it cannot spell, and the caller rounds it, as the RFC does, and
    /// computes the wall clock from the rounded one.
    #[test]
    fn an_offset_with_seconds_has_no_rfc_3339_spelling() {
        let amsterdam = UtcOffset::from_seconds(19 * 60 + 32).unwrap();
        let mut out = String::new();
        let error = write_unix(
            &mut out,
            UnixTime::from_seconds(-1_041_379_200 + 43_200),
            amsterdam,
            SubsecondPrecision::Auto,
        )
        .unwrap_err();
        assert_eq!(
            error,
            FormatError::Unrepresentable("an offset with more digits than the form has")
        );
        let rounded = UtcOffset::from_seconds(20 * 60).unwrap();
        let mut out = String::new();
        write_unix(
            &mut out,
            UnixTime::from_seconds(-1_041_379_200 + 43_200),
            rounded,
            SubsecondPrecision::Auto,
        )
        .unwrap();
        assert_eq!(out, "1937-01-01T12:20:00+00:20");
        // The RFC's own example reads back to the same instant.
        let example = parse("1937-01-01T12:00:27.87+00:20").unwrap();
        assert_eq!(
            example.to_unix().unwrap().seconds(),
            -1_041_379_200 + 43_200 - 1_200 + 27
        );
    }

    #[test]
    fn a_posix_timestamp_writes_as_zulu_when_the_offset_is_zero() {
        let mut out = String::new();
        write_unix(
            &mut out,
            UnixTime::from_seconds(0),
            UtcOffset::UTC,
            SubsecondPrecision::Auto,
        )
        .unwrap();
        assert_eq!(out, "1970-01-01T00:00:00Z");
    }

    #[test]
    fn a_posix_timestamp_writes_in_the_offset_it_is_given() {
        let mut out = String::new();
        write_unix(
            &mut out,
            UnixTime::from_seconds(0),
            UtcOffset::from_hms(9, 0, 0).unwrap(),
            SubsecondPrecision::Auto,
        )
        .unwrap();
        assert_eq!(out, "1970-01-01T09:00:00+09:00");
    }

    #[test]
    fn a_civil_reading_writes_against_whatever_zone_it_is_paired_with() {
        let mut out = String::new();
        write_civil(
            &mut out,
            CivilDateTime::new(Rd(739_880), CivilTime::hms(14, 30, 5).unwrap()),
            ZoneInfo::Zulu,
            SubsecondPrecision::Auto,
        )
        .unwrap();
        assert_eq!(out, "2026-09-21T14:30:05Z");
    }
}
