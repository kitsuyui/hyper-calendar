//! A whole-second offset from UTC, and its ISO 8601 spellings.

use core::fmt;
use core::ops::Neg;
use core::str::FromStr;

use hc_calendar::{CivilDateTime, CivilTime, Rd};
use hc_core::{ATTOS_PER_SEC, Duration};

use crate::error::{TzError, TzResult};

/// The largest magnitude an offset may have, in seconds: `25:59:59`.
///
/// RFC 8536 §3.2 says a TZif local time type's `utoff` should lie in
/// `[-89999, 93599]` seconds, more than −25 hours and less than 26; this
/// type accepts the symmetric window `-25:59:59 ..= +25:59:59` that contains
/// it, and POSIX `TZ` strings inherit the same limit in practice. Real
/// civil offsets span only `-12:00 ..= +14:00`, but the wider window lets
/// historical local-mean-time records (Asia/Manila's `-15:56:00` before
/// 1844, say) and hand-written test data through.
pub const MAX_OFFSET_SECONDS: i32 = 25 * 3_600 + 59 * 60 + 59;

/// A fixed displacement from UTC, counted in whole seconds, positive east.
///
/// Seconds are not a curiosity: local mean time offsets in the IANA database
/// carry them (Europe/Amsterdam was `+00:19:32` until 1937), so an offset type
/// that only stores minutes cannot read the historical record back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UtcOffset {
    seconds: i32,
}

impl UtcOffset {
    /// No displacement at all.
    pub const UTC: Self = Self { seconds: 0 };

    /// Build an offset from whole seconds east of Greenwich.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::OffsetOutOfRange`] outside
    /// `-25:59:59 ..= +25:59:59`.
    pub const fn from_seconds(seconds: i32) -> TzResult<Self> {
        if seconds < -MAX_OFFSET_SECONDS || seconds > MAX_OFFSET_SECONDS {
            return Err(TzError::OffsetOutOfRange);
        }
        Ok(Self { seconds })
    }

    /// Build an offset from signed hours, minutes and seconds.
    ///
    /// Every non-zero component must carry the same sign: `-00:30` is
    /// `from_hms(0, -30, 0)`, and mixing signs is a mistake worth reporting
    /// rather than interpreting.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::MalformedOffset`] when the signs disagree or when
    /// `minutes` or `seconds` exceed 59 in magnitude, and
    /// [`TzError::OffsetOutOfRange`] when the total is too large.
    pub const fn from_hms(hours: i32, minutes: i32, seconds: i32) -> TzResult<Self> {
        if minutes <= -60 || minutes >= 60 || seconds <= -60 || seconds >= 60 {
            return Err(TzError::MalformedOffset);
        }
        let negative = hours < 0 || minutes < 0 || seconds < 0;
        let positive = hours > 0 || minutes > 0 || seconds > 0;
        if negative && positive {
            return Err(TzError::MalformedOffset);
        }
        let total = hours as i64 * 3_600 + minutes as i64 * 60 + seconds as i64;
        if total < -(MAX_OFFSET_SECONDS as i64) || total > MAX_OFFSET_SECONDS as i64 {
            return Err(TzError::OffsetOutOfRange);
        }
        Self::from_seconds(total as i32)
    }

    /// The offset in whole seconds, positive east of Greenwich.
    #[must_use]
    pub const fn seconds(self) -> i32 {
        self.seconds
    }

    /// Whether this offset is zero.
    #[must_use]
    pub const fn is_utc(self) -> bool {
        self.seconds == 0
    }

    /// Whether this offset lies west of Greenwich.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.seconds < 0
    }

    /// The whole hours in the offset's magnitude.
    #[must_use]
    pub const fn abs_hours(self) -> u32 {
        self.seconds.unsigned_abs() / 3_600
    }

    /// The whole minutes within the hour, in the offset's magnitude.
    #[must_use]
    pub const fn abs_minutes(self) -> u32 {
        (self.seconds.unsigned_abs() % 3_600) / 60
    }

    /// The seconds within the minute, in the offset's magnitude.
    #[must_use]
    pub const fn abs_seconds(self) -> u32 {
        self.seconds.unsigned_abs() % 60
    }

    /// The offset in the opposite direction.
    ///
    /// # Errors
    ///
    /// Cannot fail for any constructible offset; the signature stays fallible
    /// so that the range invariant is checked in one place only.
    pub const fn negated(self) -> TzResult<Self> {
        Self::from_seconds(-self.seconds)
    }

    /// Add a whole number of seconds to the offset.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::OffsetOutOfRange`] when the sum leaves the
    /// representable window.
    pub const fn checked_add_seconds(self, seconds: i32) -> TzResult<Self> {
        match self.seconds.checked_add(seconds) {
            Some(total) => Self::from_seconds(total),
            None => Err(TzError::OffsetOutOfRange),
        }
    }

    /// Read an offset in any of the ISO 8601 spellings.
    ///
    /// Accepted: `Z`, `z`, `±hh`, `±hh:mm`, `±hhmm`, `±hh:mm:ss`, `±hhmmss`.
    /// `-00:00` is accepted and means UTC: RFC 3339 §4.3 gives it the separate
    /// sense of "offset unknown", which this type has no way to carry, so the
    /// distinction is dropped rather than guessed at.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::MalformedOffset`] for any other shape and
    /// [`TzError::OffsetOutOfRange`] when the value parses but is too large.
    pub fn parse(text: &str) -> TzResult<Self> {
        let bytes = text.as_bytes();
        if bytes == b"Z" || bytes == b"z" {
            return Ok(Self::UTC);
        }
        let (sign, rest) = match bytes.split_first() {
            Some((b'+', rest)) => (1, rest),
            Some((b'-', rest)) => (-1, rest),
            _ => return Err(TzError::MalformedOffset),
        };
        let (hours, minutes, seconds) = match rest {
            [_, _] => (two_digits(rest)?, 0, 0),
            [h @ .., b':', m1, m2] if h.len() == 2 => (two_digits(h)?, two_digits(&[*m1, *m2])?, 0),
            [h1, h2, m1, m2] => (two_digits(&[*h1, *h2])?, two_digits(&[*m1, *m2])?, 0),
            [h1, h2, b':', m1, m2, b':', s1, s2] => (
                two_digits(&[*h1, *h2])?,
                two_digits(&[*m1, *m2])?,
                two_digits(&[*s1, *s2])?,
            ),
            [h1, h2, m1, m2, s1, s2] => (
                two_digits(&[*h1, *h2])?,
                two_digits(&[*m1, *m2])?,
                two_digits(&[*s1, *s2])?,
            ),
            _ => return Err(TzError::MalformedOffset),
        };
        if minutes > 59 || seconds > 59 {
            return Err(TzError::MalformedOffset);
        }
        Self::from_seconds(sign * (hours * 3_600 + minutes * 60 + seconds))
    }

    /// Render the offset in one ISO 8601 form, into a stack buffer.
    ///
    /// The buffer exists so that rendering works in `no_std` builds without
    /// an allocator; [`OffsetText`] derefs to `&str`.
    #[must_use]
    pub fn format(self, style: OffsetStyle) -> OffsetText {
        let mut text = OffsetText::default();
        text.push(if self.is_negative() { b'-' } else { b'+' });
        text.push_two(self.abs_hours());
        match style {
            OffsetStyle::Hours => {}
            OffsetStyle::Extended => {
                text.push(b':');
                text.push_two(self.abs_minutes());
            }
            OffsetStyle::Basic => text.push_two(self.abs_minutes()),
            OffsetStyle::ExtendedSeconds => {
                text.push(b':');
                text.push_two(self.abs_minutes());
                text.push(b':');
                text.push_two(self.abs_seconds());
            }
            OffsetStyle::BasicSeconds => {
                text.push_two(self.abs_minutes());
                text.push_two(self.abs_seconds());
            }
        }
        text
    }

    /// Render the offset, but write the UTC designator `Z` for zero.
    ///
    /// ISO 8601 allows `Z` only for UTC itself, so this is separate from
    /// [`UtcOffset::format`] rather than a flag on it.
    #[must_use]
    pub fn format_zulu(self, style: OffsetStyle) -> OffsetText {
        if self.is_utc() {
            let mut text = OffsetText::default();
            text.push(b'Z');
            return text;
        }
        self.format(style)
    }

    /// Apply the offset to a UTC civil date-time, producing the local one.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::Overflow`] when the shifted day leaves the range of
    /// [`Rd`].
    ///
    /// # Leap seconds
    ///
    /// A UTC `23:59:60` counts as the 86 400th second of its day, so shifting
    /// it lands on the following day's `00:00:00` plus the offset. The leap
    /// flag is lost, exactly as it is in POSIX time: `08:59:60` on 1 January
    /// in Tokyo is not a value [`CivilTime`] can hold, because
    /// [`CivilTime`] only admits `23:59:60`.
    pub fn local_from_utc(self, utc: CivilDateTime) -> TzResult<CivilDateTime> {
        shift(utc, i64::from(self.seconds))
    }

    /// Remove the offset from a local civil date-time, producing the UTC one.
    ///
    /// # Errors
    ///
    /// See [`UtcOffset::local_from_utc`].
    pub fn utc_from_local(self, local: CivilDateTime) -> TzResult<CivilDateTime> {
        shift(local, -i64::from(self.seconds))
    }
}

/// Move a civil date-time by a whole number of seconds, treating every day as
/// 86 400 seconds long.
fn shift(value: CivilDateTime, seconds: i64) -> TzResult<CivilDateTime> {
    let within_day = value.time.since_midnight().whole_seconds();
    let attos = value.time.subsec_attos();
    let total = i128::from(value.day.get()) * 86_400 + within_day + i128::from(seconds);
    let day = i64::try_from(total.div_euclid(86_400)).map_err(|_| TzError::Overflow)?;
    let remainder = total.rem_euclid(86_400);
    let time = CivilTime::from_midnight_offset(Duration::from_attos(
        remainder * i128::from(ATTOS_PER_SEC) + i128::from(attos),
    ))?;
    Ok(CivilDateTime::new(Rd(day), time))
}

/// Read exactly two ASCII digits.
fn two_digits(bytes: &[u8]) -> TzResult<i32> {
    match bytes {
        [a, b] if a.is_ascii_digit() && b.is_ascii_digit() => {
            Ok(i32::from(a - b'0') * 10 + i32::from(b - b'0'))
        }
        _ => Err(TzError::MalformedOffset),
    }
}

impl Neg for UtcOffset {
    type Output = Self;

    /// Infallible because the permitted range is symmetric about zero.
    fn neg(self) -> Self {
        Self {
            seconds: -self.seconds,
        }
    }
}

impl fmt::Display for UtcOffset {
    /// Renders the extended form, `+09:00`, and never `Z`.
    ///
    /// A `Display` that sometimes produced `Z` would make `+00:00` and UTC
    /// indistinguishable in logs; callers who want `Z` ask for it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = if self.abs_seconds() == 0 {
            OffsetStyle::Extended
        } else {
            OffsetStyle::ExtendedSeconds
        };
        f.write_str(self.format(style).as_str())
    }
}

impl FromStr for UtcOffset {
    type Err = TzError;

    fn from_str(text: &str) -> TzResult<Self> {
        Self::parse(text)
    }
}

/// Which ISO 8601 spelling [`UtcOffset::format`] should produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OffsetStyle {
    /// `+09` — hours only. Loses any minutes; use it only when the offset is
    /// known to be a whole hour.
    Hours,
    /// `+09:00` — the extended form, and the default everywhere in ISO 8601
    /// date-times.
    #[default]
    Extended,
    /// `+0900` — the basic form.
    Basic,
    /// `+09:00:00` — extended, with seconds. Needed for historical local mean
    /// time offsets.
    ExtendedSeconds,
    /// `+090000` — basic, with seconds.
    BasicSeconds,
}

/// A rendered offset held in a stack buffer.
///
/// The longest form, `+25:59:59`, is nine bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OffsetText {
    bytes: [u8; 9],
    len: u8,
}

impl OffsetText {
    /// The rendered text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        // Every byte written here is ASCII, so the conversion cannot fail;
        // the fallback keeps the function total without an `unwrap`.
        core::str::from_utf8(&self.bytes[..self.len as usize]).unwrap_or("")
    }

    fn push(&mut self, byte: u8) {
        if let Some(slot) = self.bytes.get_mut(self.len as usize) {
            *slot = byte;
            self.len += 1;
        }
    }

    fn push_two(&mut self, value: u32) {
        let value = value % 100;
        self.push(b'0' + (value / 10) as u8);
        self.push(b'0' + (value % 10) as u8);
    }
}

impl core::ops::Deref for OffsetText {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for OffsetText {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OffsetText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::rd_from_ymd;

    fn civil(year: i64, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> CivilDateTime {
        CivilDateTime::new(
            Rd(rd_from_ymd(year, month, day)),
            CivilTime::hms(hour, minute, second).unwrap(),
        )
    }

    #[test]
    fn utc_is_zero_and_prints_as_z_only_when_asked() {
        assert_eq!(UtcOffset::UTC.seconds(), 0);
        assert!(UtcOffset::UTC.is_utc());
        assert_eq!(UtcOffset::UTC.to_string(), "+00:00");
        assert_eq!(
            UtcOffset::UTC.format_zulu(OffsetStyle::Extended).as_str(),
            "Z"
        );
    }

    #[test]
    fn offsets_parse_in_every_iso_8601_form() {
        let nine_hours = UtcOffset::from_hms(9, 0, 0).unwrap();
        for text in ["+09", "+0900", "+09:00", "+090000", "+09:00:00"] {
            assert_eq!(UtcOffset::parse(text).unwrap(), nine_hours, "{text}");
        }
        assert_eq!(UtcOffset::parse("Z").unwrap(), UtcOffset::UTC);
        assert_eq!(UtcOffset::parse("z").unwrap(), UtcOffset::UTC);
        assert_eq!(UtcOffset::parse("-00:00").unwrap(), UtcOffset::UTC);
    }

    #[test]
    fn kathmandu_is_five_hours_and_forty_five_minutes_east() {
        let offset = UtcOffset::parse("+05:45").unwrap();
        assert_eq!(offset.seconds(), 5 * 3_600 + 45 * 60);
        assert_eq!(offset.abs_hours(), 5);
        assert_eq!(offset.abs_minutes(), 45);
        assert_eq!(offset.to_string(), "+05:45");
    }

    #[test]
    fn historical_local_mean_time_offsets_keep_their_seconds() {
        // Europe/Amsterdam kept Amsterdam mean time, +00:19:32, until 1937.
        let offset = UtcOffset::parse("+00:19:32").unwrap();
        assert_eq!(offset.seconds(), 19 * 60 + 32);
        assert_eq!(offset.to_string(), "+00:19:32");
        assert_eq!(offset.format(OffsetStyle::BasicSeconds).as_str(), "+001932");
    }

    #[test]
    fn the_second_bearing_styles_round_trip_exactly() {
        for seconds in [-93_599, -45_000, -3_600, 0, 1_800, 20_700, 50_400, 93_599] {
            let offset = UtcOffset::from_seconds(seconds).unwrap();
            for style in [OffsetStyle::ExtendedSeconds, OffsetStyle::BasicSeconds] {
                let text = offset.format(style);
                assert_eq!(
                    UtcOffset::parse(text.as_str()).unwrap(),
                    offset,
                    "{seconds} {style:?} {text}"
                );
            }
        }
    }

    #[test]
    fn the_minute_styles_round_trip_whenever_the_seconds_field_is_zero() {
        for seconds in (-93_540..=93_540).step_by(60) {
            let offset = UtcOffset::from_seconds(seconds).unwrap();
            for style in [OffsetStyle::Extended, OffsetStyle::Basic] {
                let text = offset.format(style);
                assert_eq!(UtcOffset::parse(text.as_str()).unwrap(), offset, "{text}");
            }
        }
    }

    #[test]
    fn the_hours_only_style_truncates_and_says_so() {
        let offset = UtcOffset::from_hms(5, 45, 0).unwrap();
        assert_eq!(offset.format(OffsetStyle::Hours).as_str(), "+05");
        assert_eq!(
            UtcOffset::parse(offset.format(OffsetStyle::Hours).as_str())
                .unwrap()
                .seconds(),
            5 * 3_600
        );
    }

    #[test]
    fn offsets_beyond_twenty_six_hours_are_refused() {
        assert_eq!(
            UtcOffset::from_seconds(MAX_OFFSET_SECONDS + 1),
            Err(TzError::OffsetOutOfRange)
        );
        assert_eq!(
            UtcOffset::from_seconds(-MAX_OFFSET_SECONDS - 1),
            Err(TzError::OffsetOutOfRange)
        );
        assert!(UtcOffset::from_seconds(MAX_OFFSET_SECONDS).is_ok());
        assert!(UtcOffset::from_seconds(-MAX_OFFSET_SECONDS).is_ok());
        assert_eq!(
            UtcOffset::parse("+26:00:00"),
            Err(TzError::OffsetOutOfRange)
        );
    }

    #[test]
    fn malformed_offsets_are_refused() {
        for text in [
            "",
            "+",
            "09:00",
            "+9:00",
            "+09:0",
            "+09:60",
            "+09:00:60",
            "++9",
            "+09:00:0",
        ] {
            assert!(UtcOffset::parse(text).is_err(), "{text} should not parse");
        }
    }

    #[test]
    fn from_hms_refuses_mixed_signs_and_overlarge_parts() {
        assert_eq!(
            UtcOffset::from_hms(9, -30, 0),
            Err(TzError::MalformedOffset)
        );
        assert_eq!(UtcOffset::from_hms(0, 60, 0), Err(TzError::MalformedOffset));
        assert_eq!(
            UtcOffset::from_hms(0, 0, -60),
            Err(TzError::MalformedOffset)
        );
        assert_eq!(UtcOffset::from_hms(0, -30, 0).unwrap().seconds(), -1_800);
    }

    #[test]
    fn negation_is_symmetric_across_the_whole_range() {
        for seconds in (-MAX_OFFSET_SECONDS..=MAX_OFFSET_SECONDS).step_by(997) {
            let offset = UtcOffset::from_seconds(seconds).unwrap();
            assert_eq!((-offset).seconds(), -seconds);
            assert_eq!(offset.negated().unwrap(), -offset);
        }
    }

    #[test]
    fn applying_an_offset_can_move_the_civil_day() {
        // New Year in Tokyo: 2023-12-31T15:00:00Z is 2024-01-01T00:00:00+09:00.
        let utc = civil(2023, 12, 31, 15, 0, 0);
        let tokyo = UtcOffset::from_hms(9, 0, 0).unwrap();
        assert_eq!(
            tokyo.local_from_utc(utc).unwrap(),
            civil(2024, 1, 1, 0, 0, 0)
        );
        assert_eq!(
            tokyo.utc_from_local(civil(2024, 1, 1, 0, 0, 0)).unwrap(),
            utc
        );
    }

    #[test]
    fn western_offsets_move_the_day_backwards() {
        // Honolulu, -10:00, sees the new year ten hours after UTC does.
        let utc = civil(2024, 1, 1, 5, 0, 0);
        let honolulu = UtcOffset::from_hms(-10, 0, 0).unwrap();
        assert_eq!(
            honolulu.local_from_utc(utc).unwrap(),
            civil(2023, 12, 31, 19, 0, 0)
        );
    }

    #[test]
    fn offsets_round_trip_over_ten_thousand_days() {
        let offsets = [
            UtcOffset::from_seconds(-43_200).unwrap(),
            UtcOffset::from_seconds(-12_600).unwrap(),
            UtcOffset::UTC,
            UtcOffset::from_seconds(20_700).unwrap(),
            UtcOffset::from_seconds(50_400).unwrap(),
        ];
        for day in 700_000..710_000 {
            let utc = CivilDateTime::new(Rd(day), CivilTime::hms(13, 7, 5).unwrap());
            for offset in offsets {
                let local = offset.local_from_utc(utc).unwrap();
                assert_eq!(offset.utc_from_local(local).unwrap(), utc, "{day}");
            }
        }
    }

    #[test]
    fn a_utc_leap_second_shifts_as_the_day_s_last_second() {
        // 2016-12-31T23:59:60Z is the 86 400th second of that day, so in
        // Tokyo it collapses onto 1 January 09:00:00 rather than 08:59:60.
        let leap = CivilDateTime::new(
            Rd(rd_from_ymd(2016, 12, 31)),
            CivilTime::hms(23, 59, 60).unwrap(),
        );
        let tokyo = UtcOffset::from_hms(9, 0, 0).unwrap();
        assert_eq!(
            tokyo.local_from_utc(leap).unwrap(),
            civil(2017, 1, 1, 9, 0, 0)
        );
    }

    #[test]
    fn rendered_offsets_behave_like_strings() {
        let text = UtcOffset::from_hms(-3, -30, 0)
            .unwrap()
            .format(OffsetStyle::Extended);
        assert_eq!(text.as_str(), "-03:30");
        assert_eq!(&*text, "-03:30");
        assert_eq!(AsRef::<str>::as_ref(&text), "-03:30");
        assert_eq!(text.to_string(), "-03:30");
        assert!(text.starts_with('-'));
    }

    #[test]
    fn offsets_can_be_read_with_from_str() {
        let offset: UtcOffset = "+05:30".parse().unwrap();
        assert_eq!(offset.seconds(), 19_800);
        assert!("noon".parse::<UtcOffset>().is_err());
    }

    #[test]
    fn checked_addition_reports_leaving_the_window() {
        let offset = UtcOffset::from_hms(25, 0, 0).unwrap();
        assert!(offset.checked_add_seconds(3_600).is_err());
        assert_eq!(
            offset.checked_add_seconds(-3_600).unwrap().seconds(),
            24 * 3_600
        );
        assert!(
            UtcOffset::from_seconds(MAX_OFFSET_SECONDS)
                .unwrap()
                .checked_add_seconds(i32::MAX)
                .is_err()
        );
    }
}
