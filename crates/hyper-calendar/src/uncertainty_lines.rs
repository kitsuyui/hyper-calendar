//! The tab-separated lines the WebAssembly module and the C library write
//! about time that is not exactly known, written once.
//!
//! Everything here reads [`hc_uncertainty`]: ISO 8601-2 Extended Date/Time
//! Format values, which place a date that is doubted, approximate, known to
//! the decade or one of several; significant figures, which say how many
//! digits a number is entitled to and carry the count through arithmetic;
//! Gaussian quantities, `value ± σ`, with first-order propagation and the
//! inverse-variance weighted mean; and guaranteed intervals of time, with
//! their own arithmetic.
//!
//! **Nothing is rounded to a double that the library holds exactly.** An
//! EDTF date is a fixed day, an integer; an interval's bounds are a whole
//! number of seconds and attoseconds, which is how a [`Duration`] holds
//! them; a figure count is an integer. A value that is a measurement — a
//! `value ± σ`, a significant number — is a double because that is what the
//! library holds, and each line says to how many figures it may be read.

use alloc::string::String;
use alloc::vec::Vec;

use hc_calendar::fixed::RD_OF_UNIX_EPOCH;
use hc_core::Duration;
use hc_uncertainty::{
    DurationInterval, EdtfDate, EdtfEndpoint, EdtfPrecision, EdtfQualifier, EdtfSetMember,
    EdtfValue, FuzzyInstant, Significant, Uncertain, UncertaintyError,
};

use crate::boundary::{Answer, Line, Refusal};

/// How many columns a line of [`edtf_lines`] has.
pub const EDTF_COLUMNS: usize = 11;

/// How many columns a line of [`edtf_relations_line`] has.
pub const EDTF_RELATIONS_COLUMNS: usize = 7;

/// How many columns a line of [`significant_line`] has.
pub const SIGNIFICANT_COLUMNS: usize = 6;

/// How many columns a line of [`significant_op_line`] has.
pub const SIGNIFICANT_OP_COLUMNS: usize = 6;

/// How many columns a line of [`uncertain_line`] has.
pub const UNCERTAIN_COLUMNS: usize = 11;

/// How many columns a line of [`uncertain_op_line`] has.
pub const UNCERTAIN_OP_COLUMNS: usize = 5;

/// How many columns a line of [`interval_line`] has.
pub const INTERVAL_COLUMNS: usize = 11;

/// The refusal an error of the library is: text that is not EDTF is
/// [`Refusal::Malformed`], arithmetic that left the range of the type
/// [`Refusal::Overflow`], and every other a value out of range.
fn refusal(error: UncertaintyError) -> Refusal {
    match error {
        UncertaintyError::InvalidSyntax(_) | UncertaintyError::Unsupported(_) => Refusal::Malformed,
        UncertaintyError::Overflow => Refusal::Overflow,
        _ => Refusal::OutOfRange,
    }
}

/// The identifier of an EDTF precision.
const fn precision_id(precision: EdtfPrecision) -> &'static str {
    match precision {
        EdtfPrecision::Unknown => "unknown",
        EdtfPrecision::Millennium => "millennium",
        EdtfPrecision::Century => "century",
        EdtfPrecision::Decade => "decade",
        EdtfPrecision::Year => "year",
        EdtfPrecision::Month => "month",
        EdtfPrecision::Day => "day",
    }
}

/// The identifier of an EDTF qualifier.
const fn qualifier_id(qualifier: EdtfQualifier) -> &'static str {
    match qualifier {
        EdtfQualifier::Certain => "certain",
        EdtfQualifier::Uncertain => "uncertain",
        EdtfQualifier::Approximate => "approximate",
        EdtfQualifier::UncertainAndApproximate => "uncertain-and-approximate",
    }
}

/// The identifier of what a [`FuzzyInstant`] is.
const fn fuzzy_id(instant: &FuzzyInstant) -> &'static str {
    match instant {
        FuzzyInstant::Exact(_) => "exact",
        FuzzyInstant::Resolved { .. } => "resolved",
        FuzzyInstant::Bounded { .. } => "bounded",
        FuzzyInstant::Gaussian { .. } => "gaussian",
        FuzzyInstant::Before(_) => "before",
        FuzzyInstant::After(_) => "after",
        FuzzyInstant::Unknown => "unknown",
    }
}

/// The fixed day an instant counted in 86 400-second days from the 1970
/// epoch falls on.
fn day_of(instant: hc_core::Instant<hc_core::Tai>) -> i128 {
    instant.since_epoch().days_and_seconds().0 + i128::from(RD_OF_UNIX_EPOCH)
}

/// The cells every part of an EDTF value shares, from the third: its text,
/// precision, qualifier and whether it is written in the long `Y` form, and
/// the first and last fixed day it could denote.
fn date_cells(line: &mut Line<'_>, date: EdtfDate) -> Answer<()> {
    let (first, last) = date.day_range().map_err(refusal)?;
    line.value(date)
        .cell(precision_id(date.precision()))
        .cell(qualifier_id(date.qualifier()))
        .flag(date.is_long_form())
        .value(first)
        .value(last);
    Ok(())
}

/// The cells of a part that is not one date: its text, no precision,
/// qualifier or form, and the days it could denote, empty where it does not
/// stop.
fn span_cells(
    line: &mut Line<'_>,
    text: impl core::fmt::Display,
    first: Option<i64>,
    last: Option<i64>,
) {
    line.value(text)
        .empties(3)
        .value_or_empty(first)
        .value_or_empty(last);
}

/// Append a row of an EDTF value that is a part of it, and nothing of the
/// support.
fn part(
    out: &mut String,
    role: &str,
    kind: &str,
    write: impl FnOnce(&mut Line<'_>) -> Answer<()>,
) -> Answer<()> {
    let mut line = Line::new(out);
    line.cell(role).cell(kind);
    write(&mut line)?;
    line.empties(3);
    line.end();
    Ok(())
}

/// Append the rows of an interval's endpoint.
fn endpoint(out: &mut String, role: &str, point: EdtfEndpoint) -> Answer<()> {
    match point {
        EdtfEndpoint::Date(date) => part(out, role, "date", |line| date_cells(line, date)),
        EdtfEndpoint::Open => part(out, role, "open", |line| {
            span_cells(line, "..", None, None);
            Ok(())
        }),
        EdtfEndpoint::Unknown => part(out, role, "unknown", |line| {
            span_cells(line, "", None, None);
            Ok(())
        }),
    }
}

/// Append the row of a member of a set.
fn member(out: &mut String, member: EdtfSetMember) -> Answer<()> {
    match member {
        EdtfSetMember::Date(date) => part(out, "member", "date", |line| date_cells(line, date)),
        EdtfSetMember::Range { start, end } => part(out, "member", "range", |line| {
            let first = start.day_range().map_err(refusal)?.0;
            let last = end.day_range().map_err(refusal)?.1;
            span_cells(
                line,
                format_args!("{start}..{end}"),
                Some(first),
                Some(last),
            );
            Ok(())
        }),
        EdtfSetMember::EarlierThan(date) => part(out, "member", "earlier-than", |line| {
            let last = date.day_range().map_err(refusal)?.1;
            span_cells(line, format_args!("..{date}"), None, Some(last));
            Ok(())
        }),
        EdtfSetMember::LaterThan(date) => part(out, "member", "later-than", |line| {
            let first = date.day_range().map_err(refusal)?.0;
            span_cells(line, format_args!("{date}.."), Some(first), None);
            Ok(())
        }),
    }
}

/// An ISO 8601-2 value placed on the timeline, as lines: first the value
/// itself, then each part it is made of.
///
/// Every line has eleven tab-separated cells. The first is the role: `value`
/// for the whole text, `start` and `end` for the two sides of an interval,
/// `member` for each date or run of a `[...]` or `{...}` set. The second is
/// the kind: for the value `date`, `interval`, `earlier-than`, `later-than`,
/// `one-of` (a `[...]` set: exactly one holds) or `all-of` (a `{...}` list:
/// all apply); for a part `date`, `open` (`..`), `unknown` (empty),
/// `range`, `earlier-than` or `later-than`. The third is the part written
/// in its canonical form, which parses to the same value. The fourth is the
/// precision (`unknown`, `millennium`, `century`, `decade`, `year`, `month`
/// or `day`), the fifth the qualifier (`certain`, `uncertain` for `?`,
/// `approximate` for `~`, `uncertain-and-approximate` for `%`), the sixth
/// `1` for the long `Y` form of a year: all three empty where the part is not
/// one date. The seventh and eighth are the first and last fixed day the part
/// could denote, inclusive, empty where it does not stop, and for a value the
/// days of its support, which a `~` or `%` qualifier widens by one unit of the
/// stated precision on each side. The ninth is the value's support: `resolved`
/// (one unit of a scale), `bounded`, `before`, `after` or `unknown`; the tenth
/// the best estimate as seconds from 1970-01-01 00:00:00 counting 86 400-second
/// days (the midpoint, empty for an open or unknown value); the eleventh the
/// support's width in days. The last three are empty on a part.
///
/// A set is placed by the hull of its members, which is wider than the set:
/// `[1667,1670]` is somewhere from 1667 to the end of 1670, and the gap is
/// lost. Times of day, seasons, sub-year divisions, component-level
/// qualifiers and exponential years are refused as malformed rather than
/// half-read.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not a supported EDTF value, and
/// [`Refusal::Overflow`] for a year past what a fixed day can count.
pub fn edtf_lines(text: &str) -> Answer<String> {
    let value = EdtfValue::parse(text).map_err(refusal)?;
    let support = value.to_fuzzy_instant().map_err(refusal)?;
    let (kind, date) = match &value {
        EdtfValue::Date(date) => ("date", Some(*date)),
        EdtfValue::Interval { .. } => ("interval", None),
        EdtfValue::EarlierThan(date) => ("earlier-than", Some(*date)),
        EdtfValue::LaterThan(date) => ("later-than", Some(*date)),
        EdtfValue::OneOf(_) => ("one-of", None),
        EdtfValue::AllOf(_) => ("all-of", None),
    };
    let reach = support.support().map_err(refusal)?;
    // The support's last day is the day before its exclusive end.
    let first = reach.earliest.map(day_of);
    let last = reach.latest.map(|instant| day_of(instant) - 1);
    let estimate = support.best_estimate().map_err(refusal)?;
    let span = support.span().map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell("value").cell(kind).value(&value);
    match date {
        Some(date) => line
            .cell(precision_id(date.precision()))
            .cell(qualifier_id(date.qualifier()))
            .flag(date.is_long_form()),
        None => line.empties(3),
    };
    line.value_or_empty(first)
        .value_or_empty(last)
        .cell(fuzzy_id(&support))
        .value_or_empty(estimate.map(|instant| instant.since_epoch().whole_seconds()))
        .value_or_empty(span.map(|width| width.days_and_seconds().0));
    line.end();
    match &value {
        EdtfValue::Interval { start, end } => {
            endpoint(&mut out, "start", *start)?;
            endpoint(&mut out, "end", *end)?;
        }
        EdtfValue::OneOf(members) | EdtfValue::AllOf(members) => {
            for each in members {
                member(&mut out, *each)?;
            }
        }
        EdtfValue::Date(_) | EdtfValue::EarlierThan(_) | EdtfValue::LaterThan(_) => {}
    }
    Ok(out)
}

/// What can hold between two EDTF values placed on the timeline, as one
/// line: Allen's relations that are possible between their supports, by
/// name and by symbol, each list separated by `;` in Allen's order, then
/// `1` or `0` for each of: the first is definitely before the second, possibly
/// before, definitely after, possibly after, and possibly concurrent.
///
/// A date known to the year is the year; `1984~` is widened by its own length
/// on each side, 1982-12-31 to 1986-01-02; an open interval has no bound on
/// its open side. Where a bound is unknown, every
/// ordering it could have is considered, so the set is what remains possible,
/// never a guess.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not a supported EDTF value, and
/// [`Refusal::Overflow`] for a year past what a fixed day can count.
pub fn edtf_relations_line(first: &str, second: &str) -> Answer<String> {
    let first = EdtfValue::parse(first)
        .and_then(|value| value.to_fuzzy_instant())
        .map_err(refusal)?;
    let second = EdtfValue::parse(second)
        .and_then(|value| value.to_fuzzy_instant())
        .map_err(refusal)?;
    let relations = first.relations(second).map_err(refusal)?;
    let names: Vec<&str> = relations
        .iter()
        .map(hc_uncertainty::AllenRelation::english_name)
        .collect();
    let symbols: Vec<&str> = relations
        .iter()
        .map(hc_uncertainty::AllenRelation::symbol)
        .collect();
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(&names.join(";"))
        .cell(&symbols.join(";"))
        .flag(first.definitely_before(second).map_err(refusal)?)
        .flag(first.possibly_before(second).map_err(refusal)?)
        .flag(first.definitely_after(second).map_err(refusal)?)
        .flag(first.possibly_after(second).map_err(refusal)?)
        .flag(first.possibly_concurrent(second).map_err(refusal)?);
    line.end();
    Ok(out)
}

/// A number with a count of significant figures, as one line: the number,
/// the figures, the number printed to exactly those figures (`1.38e10`, not
/// `13800000000`), the number rounded to them as a double, the decimal
/// exponent of its leading digit as reported, and the decimal place of its
/// last significant digit.
///
/// 17 figures is the most a double holds and means every digit is claimed,
/// as for a count or a definition; the shortest numeral that reads back as
/// the same double is printed without padding.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a number that is not finite and for a count
/// of figures that is not from 1 to 17.
pub fn significant_line(value: f64, figures: u32) -> Answer<String> {
    let figures = u8::try_from(figures).map_err(|_| Refusal::OutOfRange)?;
    let number = Significant::new(value, figures).map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(value)
        .value(figures)
        .value(number)
        .value(number.rounded())
        .value(number.decimal_exponent())
        .value(number.last_significant_place());
    line.end();
    Ok(out)
}

/// Arithmetic on two numbers with figure counts, as one line: the
/// operation, the result printed to the figures the rule leaves it, the
/// result as a double, its figure count, the decimal exponent of its
/// leading digit and the decimal place of its last significant digit.
///
/// The operations are `add` and `sub`, significant down to the coarser of
/// the two last places, so `100.0 + 0.001` keeps four figures; `mul` and
/// `div`, which carry the smaller figure count; and `pow`, which raises the
/// first number to the integer second one and keeps the first's count, the
/// second's figures being ignored.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an operation that is not one of these,
/// [`Refusal::OutOfRange`] for a number that is not finite, a count of
/// figures that is not from 1 to 17, a division by zero, a power whose
/// exponent is not an integer within an `i32`, and a result that leaves the
/// range of a double.
pub fn significant_op_line(
    operation: &str,
    first: f64,
    first_figures: u32,
    second: f64,
    second_figures: u32,
) -> Answer<String> {
    let first = Significant::new(
        first,
        u8::try_from(first_figures).map_err(|_| Refusal::OutOfRange)?,
    )
    .map_err(refusal)?;
    let name = operation.trim().to_ascii_lowercase();
    let result = if name == "pow" {
        if !second.is_finite()
            || hc_core::math::floor(second) != second
            || hc_core::math::abs(second) > f64::from(i32::MAX)
        {
            return Err(Refusal::OutOfRange);
        }
        first.checked_powi(second as i32)
    } else {
        let second = Significant::new(
            second,
            u8::try_from(second_figures).map_err(|_| Refusal::OutOfRange)?,
        )
        .map_err(refusal)?;
        match name.as_str() {
            "add" => first.checked_add(second),
            "sub" => first.checked_sub(second),
            "mul" => first.checked_mul(second),
            "div" => first.checked_div(second),
            _ => return Err(Refusal::Unknown),
        }
    }
    .map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(&name)
        .value(result)
        .value(result.value())
        .value(result.figures())
        .value(result.decimal_exponent())
        .value(result.last_significant_place());
    line.end();
    Ok(out)
}

/// The text of a quantity printed to the figures its standard deviation
/// supports, empty where it supports none.
fn supported_text(quantity: Uncertain) -> Option<Significant> {
    quantity.to_significant().ok()
}

/// A Gaussian quantity, as one line: the value, its standard deviation, the
/// pair as `value ± σ`, the value printed to the figures its standard
/// deviation supports, the relative uncertainty (empty for a value of 0),
/// and the ranges one, two and three standard deviations either side, as a
/// low and a high each.
///
/// A standard deviation of 0 is an exact value, and its significant text is
/// the shortest numeral that reads back as the same double.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a value or a standard deviation that is not
/// finite, and for a negative standard deviation.
pub fn uncertain_line(value: f64, std_dev: f64) -> Answer<String> {
    let quantity = Uncertain::new(value, std_dev).map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(value)
        .value(std_dev)
        .value(quantity)
        .value_or_empty(supported_text(quantity))
        .value_or_empty(quantity.relative().ok());
    for sigmas in [1.0, 2.0, 3.0] {
        let (low, high) = quantity.within(sigmas).map_err(refusal)?;
        line.value(low).value(high);
    }
    line.end();
    Ok(out)
}

/// Arithmetic on Gaussian quantities, as one line: the operation, the result
/// and its standard deviation (empty for a z-score, which has none), the
/// result as `value ± σ`, and the result printed to the figures its standard
/// deviation supports.
///
/// The operations are `add`, `sub`, `mul` and `div` of independent
/// quantities, with the errors combined in quadrature to first order;
/// `combine`, the inverse-variance weighted mean of two measurements of one
/// quantity (an exact one wins outright); `z-score`, how many combined
/// standard deviations separate the two; `scale`, the first multiplied by
/// the second's value as an exact factor (its standard deviation ignored);
/// `pow`, the first raised to the second's value as an exact exponent; and
/// `ln` and `exp` of the first, the second ignored.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an operation that is not one of these, and
/// [`Refusal::OutOfRange`] for a value or standard deviation that is not
/// finite or is negative, a division by a value of 0, a logarithm of a value
/// that is not positive, a power outside the real numbers, two exact values
/// that disagree under `combine`, a z-score of two exact values, and a result
/// that leaves the range of a double.
pub fn uncertain_op_line(
    operation: &str,
    first: f64,
    first_std_dev: f64,
    second: f64,
    second_std_dev: f64,
) -> Answer<String> {
    let first = Uncertain::new(first, first_std_dev).map_err(refusal)?;
    let second = Uncertain::new(second, second_std_dev).map_err(refusal)?;
    let name = operation.trim().to_ascii_lowercase();
    let result = match name.as_str() {
        "add" => first.checked_add(second),
        "sub" => first.checked_sub(second),
        "mul" => first.checked_mul(second),
        "div" => first.checked_div(second),
        "combine" => first.combine(second),
        "scale" => first.scaled(second.value),
        "pow" => first.powf(second.value),
        "ln" => first.ln(),
        "exp" => first.exp(),
        "z-score" => {
            let z = first.z_score(second).map_err(refusal)?;
            let mut out = String::new();
            let mut line = Line::new(&mut out);
            line.cell(&name).value(z).empty().value(z).value(z);
            line.end();
            return Ok(out);
        }
        _ => return Err(Refusal::Unknown),
    }
    .map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(&name)
        .value(result.value)
        .value(result.std_dev)
        .value(result)
        .value_or_empty(supported_text(result));
    line.end();
    Ok(out)
}

/// A bound as whole seconds and attoseconds, in two cells.
fn bound_cells(line: &mut Line<'_>, bound: Duration) {
    line.value(bound.whole_seconds())
        .value(bound.subsec_attos());
}

/// Arithmetic on intervals of time, as one line: the operation, `1` where the
/// result is the empty interval, the low and high bound and the width and
/// midpoint as whole seconds and attoseconds each, and for the two questions
/// `1` or `0` in the last cell with the cells before it empty.
///
/// The two intervals are `[first_low, first_high]` and `[second_low,
/// second_high]` in whole seconds. The operations are `add`, `sub` (the
/// crossed bounds: `[lo₁ − hi₂, hi₁ − lo₂]`), `intersect` (empty when they do
/// not meet), `hull` (the smallest interval holding both), and the questions
/// `overlaps` and `contains`, whether the first holds the second. The width is
/// the high bound minus the low, and the midpoint is floored to an
/// attosecond. An empty interval has no bounds, width or midpoint.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an operation that is not one of these, and
/// [`Refusal::OutOfRange`] for an interval whose low bound is above its
/// high bound, which is the empty interval written backwards.
pub fn interval_line(
    operation: &str,
    first_low: i64,
    first_high: i64,
    second_low: i64,
    second_high: i64,
) -> Answer<String> {
    let interval = |low: i64, high: i64| {
        DurationInterval::new(
            Duration::from_secs(i128::from(low)),
            Duration::from_secs(i128::from(high)),
        )
        .map_err(refusal)
    };
    let first = interval(first_low, first_high)?;
    let second = interval(second_low, second_high)?;
    let name = operation.trim().to_ascii_lowercase();
    let result = match name.as_str() {
        "add" => first.checked_add(second),
        "sub" => first.checked_sub(second),
        "intersect" => Ok(first.intersect(second)),
        "hull" => Ok(first.hull(second)),
        "overlaps" | "contains" => {
            let holds = if name == "overlaps" {
                first.overlaps(second)
            } else {
                first.contains_interval(second)
            };
            let mut out = String::new();
            let mut line = Line::new(&mut out);
            line.cell(&name).empties(9).flag(holds);
            line.end();
            return Ok(out);
        }
        _ => return Err(Refusal::Unknown),
    }
    .map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(&name).flag(result.is_empty());
    match (result.low(), result.high()) {
        (Some(low), Some(high)) => {
            bound_cells(&mut line, low);
            bound_cells(&mut line, high);
            bound_cells(&mut line, result.width().map_err(refusal)?);
            bound_cells(&mut line, result.midpoint().map_err(refusal)?);
        }
        _ => {
            line.empties(8);
        }
    }
    line.empty();
    line.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::cells;

    fn rows(text: &str) -> Vec<Vec<&str>> {
        text.lines()
            .map(|line| line.split('\t').collect())
            .collect()
    }

    #[test]
    fn every_line_has_its_columns() {
        for text in [
            edtf_lines("1984").unwrap_or_default(),
            edtf_lines("1984/..").unwrap_or_default(),
            edtf_lines("[1667,1668,1670..1672]").unwrap_or_default(),
        ] {
            assert!(
                rows(&text).iter().all(|row| row.len() == EDTF_COLUMNS),
                "{text}"
            );
        }
        let counts = [
            (
                edtf_relations_line("1984", "1985").unwrap_or_default(),
                EDTF_RELATIONS_COLUMNS,
            ),
            (
                significant_line(13.8e9, 3).unwrap_or_default(),
                SIGNIFICANT_COLUMNS,
            ),
            (
                significant_op_line("add", 1.0, 3, 2.0, 3).unwrap_or_default(),
                SIGNIFICANT_OP_COLUMNS,
            ),
            (
                uncertain_line(3200.0, 50.0).unwrap_or_default(),
                UNCERTAIN_COLUMNS,
            ),
            (
                uncertain_op_line("add", 1.0, 1.0, 1.0, 1.0).unwrap_or_default(),
                UNCERTAIN_OP_COLUMNS,
            ),
            (
                interval_line("add", 0, 1, 0, 1).unwrap_or_default(),
                INTERVAL_COLUMNS,
            ),
            (
                interval_line("overlaps", 0, 1, 0, 1).unwrap_or_default(),
                INTERVAL_COLUMNS,
            ),
            (
                interval_line("intersect", 0, 1, 5, 6).unwrap_or_default(),
                INTERVAL_COLUMNS,
            ),
        ];
        for (text, columns) in counts {
            assert_eq!(cells(&text).len(), columns, "{text}");
        }
    }

    /// 1984-01-01 is Rata Die 724 276 and 1984-12-31 is 724 641 (1984 is a
    /// leap year of 366 days), by Python's `date.toordinal`, which counts
    /// Reingold and Dershowitz's fixed days from 0001-01-01 = 1; 1 January
    /// 1970 is 719 163.
    #[test]
    fn a_year_is_one_resolved_unit_of_its_days() {
        let text = edtf_lines("1984").unwrap_or_default();
        let row = cells(&text);
        assert_eq!(
            row[..9],
            [
                "value", "date", "1984", "year", "certain", "0", "724276", "724641", "resolved"
            ]
        );
        // 1984-01-01 00:00 is 441 763 200 s after the epoch and the year is
        // 366 days: its midpoint is 457 574 400 s.
        assert_eq!(row[9], "457574400");
        assert_eq!(row[10], "366");
    }

    #[test]
    fn a_tilde_widens_the_support_by_one_unit_on_each_side() {
        let text = edtf_lines("1984~").unwrap_or_default();
        let row = cells(&text);
        assert_eq!(
            &row[..6],
            ["value", "date", "1984~", "year", "approximate", "0"]
        );
        // The crate widens by the date's own span, 366 days, on each side:
        // 723 910 to 725 007, 1 098 days (Python's `date.toordinal`).
        assert_eq!(row[6], "723910");
        assert_eq!(row[7], "725007");
        assert_eq!(row[8], "bounded");
        assert_eq!(row[10], "1098");
    }

    #[test]
    fn unspecified_digits_and_the_long_form_are_read() {
        let decade = edtf_lines("198X").unwrap_or_default();
        let row = cells(&decade);
        assert_eq!(&row[3..5], ["decade", "certain"]);
        assert_eq!(row[6], "722815");
        assert_eq!(row[7], "726467");
        let month = edtf_lines("1984-XX-XX").unwrap_or_default();
        assert_eq!(cells(&month)[3], "year");
        let long = edtf_lines("Y-170000002").unwrap_or_default();
        let row = cells(&long);
        assert_eq!(&row[2..6], ["Y-170000002", "year", "certain", "1"]);
        assert_eq!(edtf_lines("1985-04-12T23:20:30Z"), Err(Refusal::Malformed));
        assert_eq!(edtf_lines("2001-21"), Err(Refusal::Malformed));
        assert_eq!(edtf_lines("1984-02-30"), Err(Refusal::Malformed));
        assert_eq!(edtf_lines(""), Err(Refusal::Malformed));
        assert_eq!(edtf_lines("Y17E7S3"), Err(Refusal::Malformed));
        assert_eq!(edtf_lines("é"), Err(Refusal::Malformed));
    }

    #[test]
    fn an_interval_has_its_two_sides_and_open_ends() {
        let text = edtf_lines("1984/1985").unwrap_or_default();
        let table = rows(&text);
        assert_eq!(table.len(), 3);
        assert_eq!(&table[0][..3], ["value", "interval", "1984/1985"]);
        assert_eq!(table[0][8], "bounded");
        assert_eq!(&table[1][..2], ["start", "date"]);
        assert_eq!(&table[2][..2], ["end", "date"]);
        assert_eq!(table[2][7], "725006");
        let open = edtf_lines("1984/..").unwrap_or_default();
        let table = rows(&open);
        assert_eq!(table[0][8], "after");
        assert_eq!(table[0][7], "");
        assert_eq!(&table[2][..3], ["end", "open", ".."]);
        let unknown = edtf_lines("1984/").unwrap_or_default();
        assert_eq!(&rows(&unknown)[2][..2], ["end", "unknown"]);
        assert_eq!(rows(&unknown)[0][8], "after");
        let before = edtf_lines("..1760-12-03").unwrap_or_default();
        assert_eq!(&rows(&before)[0][..2], ["value", "earlier-than"]);
        assert_eq!(rows(&before)[0][8], "before");
        assert_eq!(rows(&before)[0][6], "");
        let after = edtf_lines("1760-12..").unwrap_or_default();
        assert_eq!(rows(&after)[0][8], "after");
    }

    #[test]
    fn a_set_is_placed_by_its_hull_and_lists_its_members() {
        let text = edtf_lines("[1667,1668,1670..1672]").unwrap_or_default();
        let table = rows(&text);
        assert_eq!(table.len(), 4);
        assert_eq!(
            &table[0][..3],
            ["value", "one-of", "[1667,1668,1670..1672]"]
        );
        assert_eq!(&table[1][..3], ["member", "date", "1667"]);
        assert_eq!(&table[3][..3], ["member", "range", "1670..1672"]);
        // The first member's first day is the support's first day, and the
        // last member's last day is its last.
        assert_eq!(table[0][6], table[1][6]);
        assert_eq!(table[0][7], table[3][7]);
        let all = edtf_lines("{1667,1668}").unwrap_or_default();
        assert_eq!(&rows(&all)[0][..2], ["value", "all-of"]);
        assert_eq!(edtf_lines("[]"), Err(Refusal::Malformed));
        assert_eq!(edtf_lines("[1667"), Err(Refusal::Malformed));
    }

    #[test]
    fn two_years_are_before_and_a_year_in_a_decade_is_during() {
        let line = edtf_relations_line("1984", "1986").unwrap_or_default();
        let row = cells(&line);
        assert_eq!(row[0], "before");
        assert_eq!(row[1], "<");
        assert_eq!(&row[2..], ["1", "1", "0", "0", "0"]);
        let line = edtf_relations_line("1984", "198X").unwrap_or_default();
        let row = cells(&line);
        // 1984 lies strictly inside the decade 1980-1989.
        assert_eq!(row[0], "during");
        assert_eq!(row[1], "d");
        let line = edtf_relations_line("1984", "1984").unwrap_or_default();
        assert_eq!(cells(&line)[0], "equals");
        // Adjacent years meet: the first ends where the second begins.
        let line = edtf_relations_line("1984", "1985").unwrap_or_default();
        assert_eq!(cells(&line)[0], "meets");
        assert_eq!(
            edtf_relations_line("1984", "nonsense"),
            Err(Refusal::Malformed)
        );
    }

    #[test]
    fn thirteen_point_eight_billion_years_keeps_three_figures() {
        let line = significant_line(13.8e9, 3).unwrap_or_default();
        let row = cells(&line);
        assert_eq!(&row[..3], ["13800000000", "3", "1.38e10"]);
        assert_eq!(row[3], "13800000000");
        assert_eq!(&row[4..], ["10", "8"]);
        // 9.96 to two figures is 10, whose leading digit is the tens.
        let rounded = significant_line(9.96, 2).unwrap_or_default();
        assert_eq!(&cells(&rounded)[2..], ["10", "10", "1", "0"]);
        assert_eq!(
            cells(&significant_line(1.0, 4).unwrap_or_default())[2],
            "1.000"
        );
        assert_eq!(significant_line(1.0, 0), Err(Refusal::OutOfRange));
        assert_eq!(significant_line(1.0, 18), Err(Refusal::OutOfRange));
        assert_eq!(significant_line(1.0, u32::MAX), Err(Refusal::OutOfRange));
        assert_eq!(significant_line(f64::NAN, 3), Err(Refusal::OutOfRange));
    }

    #[test]
    fn the_laboratory_rules_carry_the_figures() {
        let op = |name, a, af, b, bf| {
            let line = significant_op_line(name, a, af, b, bf).unwrap_or_default();
            cells(&line)
                .iter()
                .map(|cell| cell.to_string())
                .collect::<Vec<_>>()
        };
        // A sum is significant to the coarser last place: 100.0 + 0.001 keeps
        // four figures, not seven.
        assert_eq!(op("add", 100.0, 4, 0.001, 1)[1], "100.0");
        // A product carries the smaller count: 2.0 (2) times 3.7 (6).
        assert_eq!(op("mul", 2.0, 2, 3.7, 6)[1], "7.4");
        assert_eq!(op("MUL", 2.0, 2, 3.7, 6)[0], "mul");
        // 1.0000 - 0.9999 destroys four of five figures: one remains.
        assert_eq!(op("sub", 1.0, 5, 0.9999, 4)[3], "1");
        assert_eq!(op("div", 1.0, 3, 8.0, 5)[1], "0.125");
        assert_eq!(op("pow", 2.0, 3, 10.0, 17)[1], "1.02e3");
        assert_eq!(
            significant_op_line("div", 1.0, 3, 0.0, 3),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            significant_op_line("pow", 2.0, 3, 0.5, 3),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            significant_op_line("mod", 2.0, 3, 1.0, 3),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            significant_op_line("add", 1.0, 99, 1.0, 3),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn a_radiocarbon_age_keeps_its_error_bar() {
        // 3200 +/- 50: the value is read to the tens, since two figures of
        // sigma are 50 and the last place of the value is the tens.
        let line = uncertain_line(3200.0, 50.0).unwrap_or_default();
        let row = cells(&line);
        assert_eq!(&row[..3], ["3200", "50", "3200 ± 50"]);
        assert_eq!(row[3], "3200");
        assert_eq!(row[4], "0.015625");
        assert_eq!(&row[5..], ["3150", "3250", "3100", "3300", "3050", "3350"]);
        // Planck 2018's age of the universe, 13.787 +/- 0.020 Gyr.
        let age = uncertain_line(13.787, 0.020).unwrap_or_default();
        let row = cells(&age);
        assert_eq!(row[3], "13.787");
        let exact = uncertain_line(86_400.0, 0.0).unwrap_or_default();
        assert_eq!(cells(&exact)[3], "86400");
        assert_eq!(uncertain_line(1.0, -1.0), Err(Refusal::OutOfRange));
        assert_eq!(uncertain_line(f64::NAN, 1.0), Err(Refusal::OutOfRange));
        assert_eq!(cells(&uncertain_line(0.0, 1.0).unwrap_or_default())[4], "");
    }

    #[test]
    fn independent_errors_add_in_quadrature() {
        let op = |name, a, ad, b, bd| {
            let line = uncertain_op_line(name, a, ad, b, bd).unwrap_or_default();
            cells(&line)
                .iter()
                .map(|cell| cell.to_string())
                .collect::<Vec<_>>()
        };
        // The 3-4-5 triangle: 10 +/- 3 plus 20 +/- 4 is 30 +/- 5.
        let sum = op("add", 10.0, 3.0, 20.0, 4.0);
        assert_eq!(&sum[..3], ["add", "30", "5"]);
        let difference = op("sub", 10.0, 3.0, 10.0, 4.0);
        assert_eq!(&difference[..3], ["sub", "0", "5"]);
        // The inverse-variance mean of 10 +/- 3 and 20 +/- 4: weights 1/9
        // and 1/16, mean (16 * 10 + 9 * 20) / 25 = 13.6, sigma 12/5 = 2.4.
        let mean = op("combine", 10.0, 3.0, 20.0, 4.0);
        assert_eq!(mean[0], "combine");
        assert!(
            (mean[1].parse::<f64>().unwrap_or(0.0) - 13.6).abs() < 1e-12,
            "{mean:?}"
        );
        assert!(
            (mean[2].parse::<f64>().unwrap_or(0.0) - 2.4).abs() < 1e-12,
            "{mean:?}"
        );
        // The z-score of the same pair: 10 / 5 = 2.
        let z = op("z-score", 10.0, 3.0, 20.0, 4.0);
        assert_eq!(&z[..2], ["z-score", "2"]);
        assert_eq!(z[2], "");
        // Scaling by an exact 3 scales the bar; ln of e^1 is 1 with 1/e.
        let scaled = op("scale", 10.0, 3.0, 3.0, 9.0);
        assert_eq!(&scaled[..3], ["scale", "30", "9"]);
        let product = op("mul", 10.0, 1.0, 20.0, 2.0);
        assert_eq!(&product[..2], ["mul", "200"]);
        let squared = op("pow", 3.0, 0.1, 2.0, 0.0);
        assert_eq!(&squared[..2], ["pow", "9"]);
        assert_eq!(op("ln", 1.0, 0.1, 0.0, 0.0)[1], "0");
        assert_eq!(
            uncertain_op_line("div", 1.0, 1.0, 0.0, 1.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            uncertain_op_line("ln", -1.0, 0.1, 0.0, 0.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            uncertain_op_line("z-score", 1.0, 0.0, 2.0, 0.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            uncertain_op_line("combine", 1.0, 0.0, 2.0, 0.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            uncertain_op_line("sqrt", 1.0, 0.0, 2.0, 0.0),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            uncertain_op_line("add", 1.0, -1.0, 2.0, 0.0),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn interval_arithmetic_crosses_its_bounds_and_says_when_it_is_empty() {
        let op = |name, a, b, c, d| {
            let line = interval_line(name, a, b, c, d).unwrap_or_default();
            cells(&line)
                .iter()
                .map(|cell| cell.to_string())
                .collect::<Vec<_>>()
        };
        // [1, 3] + [10, 20] = [11, 23], width 12, midpoint 17.
        let sum = op("add", 1, 3, 10, 20);
        assert_eq!(
            sum,
            ["add", "0", "11", "0", "23", "0", "12", "0", "17", "0", ""]
        );
        // [1, 3] - [10, 20] = [1 - 20, 3 - 10] = [-19, -7], midpoint -13.
        let difference = op("sub", 1, 3, 10, 20);
        assert_eq!(&difference[2..9], ["-19", "0", "-7", "0", "12", "0", "-13"]);
        // [0, 1] and [0, 2] have midpoints 0.5 and 1: the half is 5e17
        // attoseconds.
        let half = op("hull", 0, 1, 5, 6);
        assert_eq!(&half[2..9], ["0", "0", "6", "0", "6", "0", "3"]);
        let shared = op("intersect", 0, 3, 2, 5);
        assert_eq!(&shared[2..9], ["2", "0", "3", "0", "1", "0", "2"]);
        let odd = op("intersect", 0, 1, 0, 3);
        assert_eq!(&odd[8..10], ["0", "500000000000000000"]);
        // Disjoint intervals meet in nothing.
        let none = op("intersect", 0, 1, 5, 6);
        assert_eq!(none[1], "1");
        assert!(none[2..].iter().all(String::is_empty));
        assert_eq!(op("overlaps", 0, 3, 3, 5)[10], "1");
        assert_eq!(op("overlaps", 0, 2, 3, 5)[10], "0");
        assert_eq!(op("contains", 0, 10, 2, 5)[10], "1");
        assert_eq!(op("contains", 2, 5, 0, 10)[10], "0");
        assert!(
            op("contains", 0, 10, 2, 5)[1..10]
                .iter()
                .all(String::is_empty)
        );
        assert_eq!(interval_line("add", 3, 1, 0, 1), Err(Refusal::OutOfRange));
        assert_eq!(interval_line("scale", 0, 1, 0, 1), Err(Refusal::Unknown));
    }
}
