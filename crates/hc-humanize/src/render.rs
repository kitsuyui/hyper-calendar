//! Substituting numbers into patterns, without allocating.
//!
//! Everything a caller ever sees written comes through here, and it all goes
//! into a `core::fmt::Write` sink one piece at a time. Nothing is assembled
//! into an intermediate buffer, which is what lets the whole crate work with
//! no allocator at all.
//!
//! # The pattern language
//!
//! A pattern is literal text with `{0}` and, for the two-place patterns,
//! `{1}` in it — the CLDR message syntax, minus everything a calendar never
//! needs. `{` followed by anything but a digit and a `}` is literal. There
//! is no quoting and no escape: CLDR's own relative-time patterns contain
//! neither, and inventing one would mean a pattern could no longer be read
//! against the published data.

use core::fmt;

use hc_i18n::{Locale, NumberingSystem, PluralCategory, PluralOperands, PluralRules};

use crate::error::{HumanizeError, HumanizeResult};
use crate::lookup;
use crate::pattern::{PluralForms, RelativeStyle};
use crate::unit::UnitAmount;

/// A single argument of a pattern, written on demand.
pub(crate) type Argument<'a, W> = &'a mut dyn FnMut(&mut W) -> HumanizeResult<()>;

/// Write a pattern, calling an argument's writer wherever its placeholder
/// appears.
///
/// A placeholder whose index is past the end of `args` is written through
/// literally, so a malformed data entry degrades to visible nonsense rather
/// than to a silent empty string.
pub(crate) fn write_pattern<W: fmt::Write>(
    pattern: &str,
    args: &mut [Argument<'_, W>],
    out: &mut W,
) -> HumanizeResult<()> {
    let bytes = pattern.as_bytes();
    let mut cursor = 0usize;
    let mut literal_start = 0usize;
    while cursor + 3 <= bytes.len() {
        if bytes[cursor] == b'{' && bytes[cursor + 1].is_ascii_digit() && bytes[cursor + 2] == b'}'
        {
            let index = usize::from(bytes[cursor + 1] - b'0');
            if index < args.len() {
                out.write_str(&pattern[literal_start..cursor])?;
                (args[index])(out)?;
                cursor += 3;
                literal_start = cursor;
                continue;
            }
        }
        cursor += 1;
    }
    out.write_str(&pattern[literal_start..])?;
    Ok(())
}

/// The plural operands of a unit amount's magnitude.
///
/// A half becomes the source string `n.5`, which is why the operands are
/// built from parts rather than from an integer: `v = 1` is what makes
/// English call 1.5 `other` and Czech call it `many`.
pub(crate) fn operands_of(amount: UnitAmount) -> PluralOperands {
    let magnitude = amount.count().unsigned_abs();
    if amount.has_half() {
        PluralOperands::from_parts(magnitude, 1, 5)
    } else {
        PluralOperands::from(magnitude)
    }
}

/// The plural category of a unit amount in a locale.
pub(crate) fn category_of(locale: &Locale, amount: UnitAmount) -> PluralCategory {
    PluralRules::for_locale(locale).select(&operands_of(amount))
}

/// Write the magnitude of an amount in the locale's digits.
///
/// The sign is never written: every pattern that needs one carries it as
/// words, and a minus sign in front of *3 days ago* would say it twice.
pub(crate) fn write_number<W: fmt::Write>(
    locale: &Locale,
    amount: UnitAmount,
    out: &mut W,
) -> HumanizeResult<()> {
    let digits = NumberingSystem::for_locale(locale);
    let magnitude = amount.count().saturating_abs();
    digits.write_integer(magnitude, out)?;
    if amount.has_half() {
        out.write_str(lookup::decimal_separator(locale))?;
        digits.write_integer(5, out)?;
    }
    Ok(())
}

/// Write one of a unit's pattern sets, filled with the amount's magnitude.
///
/// `pick` chooses which of the three sets — past, future or the undirected
/// count — the caller wants.
pub(crate) fn write_unit_pattern<W, F>(
    locale: &Locale,
    style: RelativeStyle,
    amount: UnitAmount,
    pick: F,
    out: &mut W,
) -> HumanizeResult<()>
where
    W: fmt::Write,
    F: Fn(&'static crate::pattern::UnitPatterns) -> &'static PluralForms,
{
    let patterns =
        lookup::unit_patterns(locale, amount.unit(), style).ok_or(HumanizeError::NoPattern)?;
    let forms = pick(patterns);
    let pattern = forms.get(category_of(locale, amount));
    if pattern.is_empty() {
        return Err(HumanizeError::NoPattern);
    }
    let mut number = |sink: &mut W| write_number(locale, amount, sink);
    write_pattern(pattern, &mut [&mut number], out)
}

/// Write the undirected phrase for an amount: *3 days*, *anderthalb
/// Stunden*.
///
/// A half is written through the locale's idiom where the data states one,
/// and as the decimal `1.5` otherwise.
pub(crate) fn write_count<W: fmt::Write>(
    locale: &Locale,
    style: RelativeStyle,
    amount: UnitAmount,
    out: &mut W,
) -> HumanizeResult<()> {
    if amount.has_half()
        && let Some(patterns) = lookup::unit_patterns(locale, amount.unit(), style)
    {
        let idiom = match amount.count().saturating_abs() {
            0 => patterns.half,
            1 => patterns.one_and_a_half,
            _ => "",
        };
        if !idiom.is_empty() {
            out.write_str(idiom)?;
            return Ok(());
        }
    }
    write_unit_pattern(locale, style, amount, |patterns| &patterns.count, out)
}

/// Write a single unit's compact suffix form: the `30m` of `2h30m`.
pub(crate) fn write_compact<W: fmt::Write>(
    locale: &Locale,
    amount: UnitAmount,
    out: &mut W,
) -> HumanizeResult<()> {
    write_number(locale, amount, out)?;
    out.write_str(lookup::compact_units(locale).get(amount.unit()))?;
    Ok(())
}

/// The indefinite singular of a unit, if the language has one and the
/// amount is exactly one whole unit.
pub(crate) fn indefinite_of(locale: &Locale, amount: UnitAmount) -> Option<&'static str> {
    if amount.has_half() || amount.count().saturating_abs() != 1 {
        return None;
    }
    let text = lookup::indefinite_units(locale).get(amount.unit());
    if text.is_empty() { None } else { Some(text) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use core::fmt::Write as _;
    use hc_i18n::PluralCategory;

    use crate::unit::TimeUnit;

    fn locale(tag: &str) -> Locale {
        tag.parse().expect("well-formed tag")
    }

    fn fill(pattern: &str, value: &str) -> String {
        let mut out = String::new();
        let mut argument = |sink: &mut String| -> HumanizeResult<()> {
            sink.write_str(value)?;
            Ok(())
        };
        write_pattern(pattern, &mut [&mut argument], &mut out).expect("writes");
        out
    }

    #[test]
    fn a_pattern_with_no_placeholder_is_written_through_unchanged() {
        assert_eq!(fill("yesterday", "3"), "yesterday");
        assert_eq!(fill("", "3"), "");
    }

    #[test]
    fn a_placeholder_is_replaced_wherever_it_stands() {
        assert_eq!(fill("{0} days ago", "3"), "3 days ago");
        assert_eq!(fill("in {0} days", "3"), "in 3 days");
        assert_eq!(fill("{0}", "3"), "3");
        assert_eq!(fill("{0} and {0}", "3"), "3 and 3");
    }

    #[test]
    fn a_placeholder_this_call_has_no_argument_for_stays_literal() {
        // Visible nonsense beats a silently swallowed field.
        assert_eq!(fill("{0} at {1}", "3"), "3 at {1}");
        assert_eq!(fill("{9}", "3"), "{9}");
    }

    #[test]
    fn a_brace_that_is_not_a_placeholder_is_ordinary_text() {
        assert_eq!(fill("{a} {0}", "3"), "{a} 3");
        assert_eq!(fill("{0", "3"), "{0");
        assert_eq!(fill("{{0}}", "3"), "{3}");
    }

    #[test]
    fn a_half_is_presented_to_the_plural_rules_as_a_written_decimal() {
        let amount = UnitAmount::half_past(1, TimeUnit::Hour);
        let operands = operands_of(amount);
        assert_eq!(operands.i(), 1);
        assert_eq!(operands.v(), 1);
        assert_eq!(operands.f(), 5);
        // English calls a written 1.5 `other`, not `one`, and that is the
        // difference between "1.5 hours" and "1.5 hour".
        assert_eq!(category_of(&locale("en"), amount), PluralCategory::Other);
        // Czech's `many` is its fraction category, so 1.5 lands there.
        assert_eq!(category_of(&locale("cs"), amount), PluralCategory::Many);
    }

    #[test]
    fn a_whole_count_is_presented_as_an_integer() {
        let operands = operands_of(UnitAmount::whole(-3, TimeUnit::Day));
        assert_eq!(operands.i(), 3);
        assert_eq!(operands.v(), 0);
    }

    #[test]
    fn the_magnitude_is_written_without_a_sign() {
        let mut out = String::new();
        write_number(
            &locale("en"),
            UnitAmount::whole(-3, TimeUnit::Day),
            &mut out,
        )
        .expect("writes");
        assert_eq!(out, "3");
    }
}
