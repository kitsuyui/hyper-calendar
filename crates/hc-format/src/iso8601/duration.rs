//! ISO 8601 durations: `P3Y6M4DT12H30M5S`, `PT0.5S`, `P1W`, and the
//! alternative form `P0003-06-04T12:30:05`.
//!
//! # Years and months are not spans
//!
//! A duration in years or months has no length until it is attached to a
//! date: "one month after 31 January" is a question with several defensible
//! answers, and "one month" on its own is 28, 29, 30 or 31 days. This module
//! therefore keeps the components as written and refuses to collapse a
//! nominal duration into a [`hc_core::Duration`]; see
//! [`IsoDuration::to_exact_duration`].

use core::fmt;

use hc_core::{ATTOS_PER_SEC, Duration};

use crate::error::{ErrorKind, ParseResult, ValueError, ValueResult};
use crate::iso8601::{Strictness, scan_fraction};
use crate::scan::Scanner;
use crate::value::{DecimalMark, Fraction, Style, write_fixed};

/// Which of the two spellings a duration was written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DurationForm {
    /// `P3Y6M4DT12H30M5S` — a number and a letter per component.
    #[default]
    Designators,
    /// `P0003-06-04T12:30:05` — the date-time-shaped alternative that ISO
    /// 8601-1:2019 §5.5.2.3 permits by mutual agreement.
    Alternative,
}

/// Which component a duration's fraction belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Slot {
    Years,
    Months,
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds,
}

impl Slot {
    /// The span of one unit in seconds, for the components that have a fixed
    /// one. Years and months do not, so they are absent.
    const fn unit_seconds(self) -> Option<u64> {
        match self {
            Self::Years | Self::Months => None,
            Self::Weeks => Some(7 * 86_400),
            Self::Days => Some(86_400),
            Self::Hours => Some(3_600),
            Self::Minutes => Some(60),
            Self::Seconds => Some(1),
        }
    }

    const fn designator(self) -> char {
        match self {
            Self::Years => 'Y',
            Self::Months | Self::Minutes => 'M',
            Self::Weeks => 'W',
            Self::Days => 'D',
            Self::Hours => 'H',
            Self::Seconds => 'S',
        }
    }
}

/// An ISO 8601 duration, held as the components it was written with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IsoDuration {
    /// Whether the duration was written with a leading minus. ISO 8601-2,
    /// not 8601-1; see [`Strictness::allow_signed_duration`].
    pub negative: bool,
    /// Years, when the text named them.
    pub years: Option<u64>,
    /// Months, when the text named them.
    pub months: Option<u64>,
    /// Weeks, when the text named them.
    pub weeks: Option<u64>,
    /// Days, when the text named them.
    pub days: Option<u64>,
    /// Hours, when the text named them.
    pub hours: Option<u64>,
    /// Minutes, when the text named them.
    pub minutes: Option<u64>,
    /// Seconds, when the text named them.
    pub seconds: Option<u64>,
    /// The fraction of the lowest-order component present. ISO 8601 allows a
    /// fraction on that component only.
    pub fraction: Option<Fraction>,
    /// Which decimal mark the fraction used.
    pub mark: DecimalMark,
    /// Which spelling the duration used.
    pub form: DurationForm,
    /// Whether the alternative form was written with separators.
    pub style: Style,
}

impl IsoDuration {
    /// The zero duration, `PT0S`.
    ///
    /// ISO 8601 has no empty duration — `P` alone is not a value — so zero
    /// has to be spelled with a component.
    pub const ZERO: Self = Self {
        negative: false,
        years: None,
        months: None,
        weeks: None,
        days: None,
        hours: None,
        minutes: None,
        seconds: Some(0),
        fraction: None,
        mark: DecimalMark::Point,
        form: DurationForm::Designators,
        style: Style::Extended,
    };

    /// A duration of whole seconds, written `PTnS`.
    #[must_use]
    pub const fn from_seconds(seconds: u64) -> Self {
        Self {
            seconds: Some(seconds),
            ..Self::ZERO
        }
    }

    /// Whether any component is written as years or months.
    ///
    /// Such a duration has no fixed length; see the module documentation.
    #[must_use]
    pub const fn is_nominal(self) -> bool {
        self.years.is_some() || self.months.is_some()
    }

    /// Whether every component present is zero.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.slots()
            .iter()
            .all(|(_, value)| value.unwrap_or(0) == 0)
            && self.fraction.is_none_or(|f| f.attos() == 0)
    }

    /// The components in canonical order.
    const fn slots(self) -> [(Slot, Option<u64>); 7] {
        [
            (Slot::Years, self.years),
            (Slot::Months, self.months),
            (Slot::Weeks, self.weeks),
            (Slot::Days, self.days),
            (Slot::Hours, self.hours),
            (Slot::Minutes, self.minutes),
            (Slot::Seconds, self.seconds),
        ]
    }

    /// The lowest-order component present, which is the one a fraction may
    /// attach to.
    fn last_slot(self) -> Option<Slot> {
        self.slots()
            .into_iter()
            .filter_map(|(slot, value)| value.map(|_| slot))
            .next_back()
    }

    /// The exact span this duration names.
    ///
    /// A week is seven days and a day is 86 400 seconds here: this is the
    /// nominal timeline that civil arithmetic runs on, not elapsed physical
    /// time across a leap second or a daylight-saving transition.
    ///
    /// # Errors
    ///
    /// [`ValueError::NominalDuration`] when years or months are present, and
    /// [`ValueError::Time`] on overflow.
    pub fn to_exact_duration(self) -> ValueResult<Duration> {
        if self.is_nominal() {
            return Err(ValueError::NominalDuration);
        }
        let overflow = || ValueError::Time(hc_core::TimeError::Overflow);
        let mut seconds: i128 = 0;
        for (slot, value) in self.slots() {
            let Some(value) = value else { continue };
            let Some(unit) = slot.unit_seconds() else {
                continue;
            };
            let scaled = i128::from(value)
                .checked_mul(i128::from(unit))
                .ok_or_else(overflow)?;
            seconds = seconds.checked_add(scaled).ok_or_else(overflow)?;
        }
        let mut remainder: u64 = 0;
        if let (Some(fraction), Some(slot)) = (self.fraction, self.last_slot())
            && let Some(unit) = slot.unit_seconds()
        {
            // A fraction of a week is worth more than a second, so the whole
            // part is carried into the seconds rather than kept as a
            // sub-second remainder.
            let scaled = fraction.of_seconds(unit);
            let whole =
                i128::try_from(scaled / u128::from(ATTOS_PER_SEC)).map_err(|_| overflow())?;
            seconds = seconds.checked_add(whole).ok_or_else(overflow)?;
            remainder = (scaled % u128::from(ATTOS_PER_SEC)) as u64;
        }
        let total = Duration::from_secs(seconds)
            .checked_add(Duration::from_attos(i128::from(remainder)))?;
        if self.negative {
            return Ok(total.checked_neg()?);
        }
        Ok(total)
    }

    /// Write the duration.
    ///
    /// # Errors
    ///
    /// [`crate::FormatError::Sink`] when the sink refuses.
    pub fn write<W: fmt::Write>(self, out: &mut W) -> crate::FormatResult<()> {
        if self.negative {
            out.write_char('-')?;
        }
        out.write_char('P')?;
        match self.form {
            DurationForm::Alternative => self.write_alternative(out),
            DurationForm::Designators => self.write_designators(out),
        }
    }

    fn write_designators<W: fmt::Write>(self, out: &mut W) -> crate::FormatResult<()> {
        let last = self.last_slot();
        let mut time_opened = false;
        let mut wrote_anything = false;
        for (slot, value) in self.slots() {
            let Some(value) = value else { continue };
            if matches!(slot, Slot::Hours | Slot::Minutes | Slot::Seconds) && !time_opened {
                out.write_char('T')?;
                time_opened = true;
            }
            write!(out, "{value}")?;
            if Some(slot) == last
                && let Some(fraction) = self.fraction
            {
                out.write_char(self.mark.as_char())?;
                fraction.write_digits(out)?;
            }
            out.write_char(slot.designator())?;
            wrote_anything = true;
        }
        if !wrote_anything {
            // A duration with no components is not writable as ISO 8601; the
            // parser never builds one, but a hand-built value might.
            out.write_str("T0S")?;
        }
        Ok(())
    }

    fn write_alternative<W: fmt::Write>(self, out: &mut W) -> crate::FormatResult<()> {
        let separator = self.style.has_separators();
        write_fixed(out, self.years.unwrap_or(0), 4)?;
        if separator {
            out.write_char('-')?;
        }
        write_fixed(out, self.months.unwrap_or(0), 2)?;
        if separator {
            out.write_char('-')?;
        }
        write_fixed(out, self.days.unwrap_or(0), 2)?;
        let has_time = self.hours.is_some() || self.minutes.is_some() || self.seconds.is_some();
        if has_time {
            out.write_char('T')?;
            write_fixed(out, self.hours.unwrap_or(0), 2)?;
            if separator {
                out.write_char(':')?;
            }
            write_fixed(out, self.minutes.unwrap_or(0), 2)?;
            if separator {
                out.write_char(':')?;
            }
            write_fixed(out, self.seconds.unwrap_or(0), 2)?;
        }
        Ok(())
    }
}

impl fmt::Display for IsoDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f).map_err(|_| fmt::Error)
    }
}

/// Parse an ISO 8601 duration.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse(text: &str) -> ParseResult<IsoDuration> {
    parse_with(text, Strictness::ISO)
}

/// Parse an ISO 8601 duration under a stated strictness.
///
/// # Errors
///
/// See [`crate::ParseError`].
pub fn parse_with(text: &str, strictness: Strictness) -> ParseResult<IsoDuration> {
    let mut scanner = Scanner::new(text);
    if scanner.is_empty() {
        return Err(scanner.error(ErrorKind::Empty));
    }
    let value = scan_duration(&mut scanner, &strictness)?;
    scanner.finish()?;
    Ok(value)
}

pub(crate) fn scan_duration(
    scanner: &mut Scanner<'_>,
    strictness: &Strictness,
) -> ParseResult<IsoDuration> {
    let sign_pos = scanner.pos();
    let negative = match scanner.peek() {
        Some(b'-') => {
            scanner.advance(1);
            true
        }
        Some(b'+') => {
            scanner.advance(1);
            false
        }
        _ => false,
    };
    if scanner.pos() != sign_pos && !strictness.allow_signed_duration {
        return Err(Scanner::error_at(
            ErrorKind::Forbidden("a signed duration"),
            sign_pos,
        ));
    }
    scanner.expect(b'P', "P")?;
    let mut value = if is_alternative_form(scanner) {
        scan_alternative(scanner, strictness)?
    } else {
        scan_designators(scanner, strictness)?
    };
    value.negative = negative;
    Ok(value)
}

/// Tell the alternative form from the designator form by the first non-digit
/// after `P`.
///
/// The designator form always puts a letter directly after a number; the
/// alternative form puts a separator or the date/time `T` there, and `PT…`
/// has no digits before its `T` at all.
fn is_alternative_form(scanner: &Scanner<'_>) -> bool {
    let bytes = scanner.rest();
    let digits = bytes.iter().take_while(|b| b.is_ascii_digit()).count();
    if digits == 0 {
        return false;
    }
    matches!(bytes.get(digits), Some(b'-' | b'T'))
}

fn scan_designators(
    scanner: &mut Scanner<'_>,
    strictness: &Strictness,
) -> ParseResult<IsoDuration> {
    let mut value = IsoDuration {
        form: DurationForm::Designators,
        ..IsoDuration::default()
    };
    let mut in_time = false;
    let mut previous: Option<Slot> = None;
    let start = scanner.pos();
    loop {
        if scanner.peek() == Some(b'T') {
            if in_time {
                return Err(scanner.error(ErrorKind::Invalid("a second time designator")));
            }
            scanner.advance(1);
            in_time = true;
            continue;
        }
        let field_start = scanner.pos();
        let run = scanner.digit_run();
        if run == 0 {
            break;
        }
        if run > 18 {
            return Err(Scanner::error_at(
                ErrorKind::OutOfRange("duration component"),
                field_start,
            ));
        }
        let magnitude = scanner.take_digits(run)?;
        let fraction = scan_fraction(scanner, strictness)?;
        let Some(letter) = scanner.peek() else {
            return Err(scanner.error(ErrorKind::UnexpectedEnd));
        };
        let slot = match (in_time, letter) {
            (false, b'Y') => Slot::Years,
            (false, b'M') => Slot::Months,
            (false, b'W') => Slot::Weeks,
            (false, b'D') => Slot::Days,
            (true, b'H') => Slot::Hours,
            (true, b'M') => Slot::Minutes,
            (true, b'S') => Slot::Seconds,
            (false, _) => return Err(scanner.error(ErrorKind::OneOf("YMWD"))),
            (true, _) => return Err(scanner.error(ErrorKind::OneOf("HMS"))),
        };
        if previous.is_some_and(|earlier| earlier >= slot) {
            return Err(Scanner::error_at(
                ErrorKind::Invalid("duration components out of order"),
                field_start,
            ));
        }
        scanner.advance(1);
        previous = Some(slot);
        match slot {
            Slot::Years => value.years = Some(magnitude),
            Slot::Months => value.months = Some(magnitude),
            Slot::Weeks => value.weeks = Some(magnitude),
            Slot::Days => value.days = Some(magnitude),
            Slot::Hours => value.hours = Some(magnitude),
            Slot::Minutes => value.minutes = Some(magnitude),
            Slot::Seconds => value.seconds = Some(magnitude),
        }
        if let Some((fraction, mark)) = fraction {
            value.fraction = Some(fraction);
            value.mark = mark;
            // ISO 8601-1:2019 §5.5.2.2: the fraction may only be on the
            // lowest-order component present, so nothing may follow it.
            if !scanner.is_empty() {
                return Err(scanner.error(ErrorKind::Invalid(
                    "a fraction on a component that is not the last",
                )));
            }
            break;
        }
    }
    if previous.is_none() {
        return Err(Scanner::error_at(
            ErrorKind::Invalid("a duration with no components"),
            start,
        ));
    }
    Ok(value)
}

fn scan_alternative(
    scanner: &mut Scanner<'_>,
    strictness: &Strictness,
) -> ParseResult<IsoDuration> {
    let years = scanner.take_digits(4)?;
    let extended = scanner.eat(b'-');
    if !extended {
        crate::iso8601::require_basic(scanner, strictness, "a basic-format duration")?;
    }
    let months = scanner.take_digits(2)?;
    if extended {
        scanner.expect(b'-', "-")?;
    }
    let days = scanner.take_digits(2)?;
    let mut value = IsoDuration {
        years: Some(years),
        months: Some(months),
        days: Some(days),
        form: DurationForm::Alternative,
        style: if extended {
            Style::Extended
        } else {
            Style::Basic
        },
        ..IsoDuration::default()
    };
    if scanner.eat(b'T') {
        value.hours = Some(scanner.take_digits(2)?);
        if extended {
            scanner.expect(b':', ":")?;
        }
        value.minutes = Some(scanner.take_digits(2)?);
        if extended {
            scanner.expect(b':', ":")?;
        }
        value.seconds = Some(scanner.take_digits(2)?);
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::{String, ToString as _};

    fn round_trip(text: &str) -> String {
        parse(text).unwrap().to_string()
    }

    #[test]
    fn the_worked_example_from_the_standard_round_trips() {
        assert_eq!(round_trip("P3Y6M4DT12H30M5S"), "P3Y6M4DT12H30M5S");
    }

    #[test]
    fn a_week_duration_stands_alone() {
        assert_eq!(round_trip("P1W"), "P1W");
        assert_eq!(
            parse("P1W").unwrap().to_exact_duration().unwrap(),
            Duration::from_days(7)
        );
    }

    #[test]
    fn a_fractional_second_round_trips() {
        assert_eq!(round_trip("PT0.5S"), "PT0.5S");
        assert_eq!(
            parse("PT0.5S").unwrap().to_exact_duration().unwrap(),
            Duration::from_millis(500)
        );
    }

    #[test]
    fn a_fraction_may_sit_on_a_larger_component_too() {
        assert_eq!(round_trip("PT1.5H"), "PT1.5H");
        assert_eq!(
            parse("PT1.5H").unwrap().to_exact_duration().unwrap(),
            Duration::from_secs(5_400)
        );
    }

    #[test]
    fn the_alternative_form_round_trips_in_both_styles() {
        assert_eq!(round_trip("P0003-06-04T12:30:05"), "P0003-06-04T12:30:05");
        assert_eq!(round_trip("P00030604T123005"), "P00030604T123005");
    }

    #[test]
    fn the_alternative_form_carries_the_same_components() {
        let value = parse("P0003-06-04T12:30:05").unwrap();
        assert_eq!(value.years, Some(3));
        assert_eq!(value.months, Some(6));
        assert_eq!(value.days, Some(4));
        assert_eq!(value.seconds, Some(5));
    }

    #[test]
    fn a_nominal_duration_refuses_to_become_a_span() {
        let value = parse("P1M").unwrap();
        assert!(value.is_nominal());
        assert_eq!(
            value.to_exact_duration().unwrap_err(),
            ValueError::NominalDuration
        );
    }

    #[test]
    fn a_negative_duration_negates_the_whole_span() {
        let value = parse("-PT1H").unwrap();
        assert!(value.negative);
        assert_eq!(value.to_exact_duration().unwrap(), Duration::from_hours(-1));
        assert_eq!(value.to_string(), "-PT1H");
    }

    #[test]
    fn a_signed_duration_is_not_iso_8601_part_one() {
        let error = parse_with("-PT1H", Strictness::FULL).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Forbidden("a signed duration"));
        assert_eq!(error.offset(), 0);
    }

    #[test]
    fn components_must_come_in_order() {
        let error = parse("P1D2M").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Invalid("duration components out of order")
        );
        assert_eq!(error.offset(), 3);
    }

    #[test]
    fn a_fraction_may_not_sit_on_a_component_that_is_not_the_last() {
        let error = parse("PT1.5H30M").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Invalid("a fraction on a component that is not the last")
        );
    }

    #[test]
    fn a_duration_with_no_components_is_not_a_duration() {
        let error = parse("P").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Invalid("a duration with no components")
        );
        assert_eq!(error.offset(), 1);
    }

    #[test]
    fn a_month_designator_means_minutes_after_the_time_designator() {
        let value = parse("PT30M").unwrap();
        assert_eq!(value.minutes, Some(30));
        assert_eq!(value.months, None);
    }

    #[test]
    fn zero_has_to_be_spelled_with_a_component() {
        assert_eq!(IsoDuration::ZERO.to_string(), "PT0S");
        assert!(IsoDuration::ZERO.is_zero());
        assert_eq!(round_trip("PT0S"), "PT0S");
    }

    #[test]
    fn a_stray_letter_names_the_designators_that_would_have_worked() {
        assert_eq!(parse("P1X").unwrap_err().kind(), ErrorKind::OneOf("YMWD"));
        assert_eq!(parse("PT1X").unwrap_err().kind(), ErrorKind::OneOf("HMS"));
    }

    #[test]
    fn a_duration_that_ends_mid_component_says_so() {
        let error = parse("P3Y6").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnexpectedEnd);
        assert_eq!(error.offset(), 4);
    }

    #[test]
    fn whole_seconds_build_the_simplest_duration() {
        assert_eq!(IsoDuration::from_seconds(90).to_string(), "PT90S");
    }
}
