//! What the WebAssembly module and the C library share about refusing and
//! about writing lines.
//!
//! The line-makers written once for both boundary crates —
//! [`crate::time_lines`], [`crate::astro_lines`] and their siblings —
//! refuse with a [`Refusal`], and each boundary spells it its own way: a
//! sentinel at or below `HC_ERR_FLOOR` in the module, an `HcStatus` in the
//! library. The kinds are the ones both already have; a new kind would be
//! a new sentinel and a new status code, which is a change to both
//! surfaces, not to this enum alone.
//!
//! # From an error to a refusal to a code
//!
//! An error of the library becomes a refusal here, once, and nowhere
//! else: [`From<TimeError>`](Refusal#impl-From<TimeError>-for-Refusal) and
//! [`From<CalendarError>`](Refusal#impl-From<CalendarError>-for-Refusal).
//! Each boundary then has one table from a refusal to its code, so the same
//! input refuses with the same refusal in both. The codes are:
//!
//! | Refusal | C library | WebAssembly module |
//! | --- | --- | --- |
//! | [`Refusal::OutOfRange`] | `HC_ERROR_OUT_OF_RANGE` | `HC_ERR_OUT_OF_RANGE` |
//! | [`Refusal::Overflow`] | `HC_ERROR_OVERFLOW` | `HC_ERR_OUT_OF_RANGE` |
//! | [`Refusal::NoData`] | `HC_ERROR_NO_DATA` | `HC_ERR_NO_DATA` |
//! | [`Refusal::Unknown`] | `HC_ERROR_UNKNOWN` | `HC_ERR_UNKNOWN` |
//! | [`Refusal::Malformed`] | `HC_ERROR_MALFORMED` | `HC_ERR_MALFORMED` |
//! | [`Refusal::InvalidDate`] | `HC_ERROR_INVALID_DATE` | `HC_ERR_INVALID_DATE` |
//!
//! The module has no sentinel of its own for an overflow, so it answers
//! `HC_ERR_OUT_OF_RANGE`; that is the one place the two differ. The codes
//! a boundary has for its own marshalling — a null pointer, text that is
//! not UTF-8, a buffer too small — are not refusals and stay in each
//! boundary.
//!
//! # Lines
//!
//! Every line a boundary writes is written with a [`Line`], which escapes
//! each cell, so that no text a table holds can add or move a column.

use alloc::string::String;
use core::fmt;

#[cfg(feature = "civil")]
use hc_calendar::CalendarError;
use hc_core::TimeError;

/// Why a shared line-maker did not answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Refusal {
    /// A value was outside the range the answer is defined for:
    /// `HC_ERR_OUT_OF_RANGE`, `HC_ERROR_OUT_OF_RANGE`.
    OutOfRange,
    /// Arithmetic left the representable range. The module has no
    /// sentinel of its own for this and answers `HC_ERR_OUT_OF_RANGE`; the
    /// library answers `HC_ERROR_OVERFLOW`.
    Overflow,
    /// The model has no data for the value: `HC_ERR_NO_DATA`,
    /// `HC_ERROR_NO_DATA`.
    NoData,
    /// A name the call does not know: `HC_ERR_UNKNOWN`, `HC_ERROR_UNKNOWN`.
    Unknown,
    /// Text that is not in the format the call reads: `HC_ERR_MALFORMED`,
    /// `HC_ERROR_MALFORMED`.
    Malformed,
    /// A date that does not exist: `HC_ERR_INVALID_DATE`,
    /// `HC_ERROR_INVALID_DATE`.
    InvalidDate,
}

impl From<TimeError> for Refusal {
    /// The refusal a time-scale error is: the ends of a model are
    /// [`Refusal::NoData`], as the C library's `hc_tai_minus_utc` has it.
    fn from(error: TimeError) -> Self {
        match error {
            TimeError::Overflow => Self::Overflow,
            TimeError::BeforeModelStart | TimeError::AfterModelEnd => Self::NoData,
            _ => Self::OutOfRange,
        }
    }
}

#[cfg(feature = "civil")]
impl From<CalendarError> for Refusal {
    /// The refusal a calendar error is: a year, month or day that does not
    /// exist is [`Refusal::InvalidDate`]; arithmetic that overflowed,
    /// [`Refusal::Overflow`]; a calendar or era not known,
    /// [`Refusal::Unknown`]; an astronomical model that could not answer,
    /// [`Refusal::NoData`]; and every other — a date before the calendar's
    /// epoch or past its supported range, a field missing or not the
    /// calendar's — [`Refusal::OutOfRange`].
    fn from(error: CalendarError) -> Self {
        match error {
            CalendarError::YearOutOfRange
            | CalendarError::MonthOutOfRange
            | CalendarError::DayOutOfRange => Self::InvalidDate,
            CalendarError::Overflow => Self::Overflow,
            CalendarError::UnknownCalendar | CalendarError::UnknownEra => Self::Unknown,
            CalendarError::AstronomicalModelFailure => Self::NoData,
            _ => Self::OutOfRange,
        }
    }
}

/// The result every shared line-maker returns.
pub type Answer<T> = Result<T, Refusal>;

/// One line of tab-separated cells, being written: the one writer of every
/// line both boundaries write.
///
/// Every cell is escaped, whatever it holds — a tab or a line break in its
/// text becomes a space — so a line always has the cells it was written
/// with, however a source string reads. The first cell is written with no
/// separator before it, every later one with a tab, and [`Line::end`]
/// writes the line break.
///
/// ```
/// use hyper_calendar::boundary::Line;
///
/// let mut out = String::new();
/// let mut line = Line::new(&mut out);
/// line.value(2026).cell("a name\twith a tab").empty().flag(true);
/// line.end();
/// assert_eq!(out, "2026\ta name with a tab\t\t1\n");
/// ```
#[derive(Debug)]
pub struct Line<'a> {
    out: &'a mut String,
    first: bool,
}

impl<'a> Line<'a> {
    /// Begin a line at the end of `out`.
    pub fn new(out: &'a mut String) -> Self {
        Self { out, first: true }
    }

    /// The separator before a cell, unless it is the line's first.
    fn next(&mut self) {
        if !self.first {
            self.out.push('\t');
        }
        self.first = false;
    }

    /// A cell of text.
    pub fn cell(&mut self, text: &str) -> &mut Self {
        self.next();
        escape_into(self.out, text);
        self
    }

    /// A cell of text, or an empty cell for `None`.
    pub fn cell_or_empty(&mut self, text: Option<&str>) -> &mut Self {
        self.cell(text.unwrap_or(""))
    }

    /// A cell of what a value displays: a number, or text formatted with
    /// `format_args!`.
    pub fn value(&mut self, value: impl fmt::Display) -> &mut Self {
        self.formatted(format_args!("{value}"))
    }

    /// A cell of what a value displays, or an empty cell for `None`.
    pub fn value_or_empty(&mut self, value: Option<impl fmt::Display>) -> &mut Self {
        match value {
            Some(value) => self.value(value),
            None => self.empty(),
        }
    }

    /// The cell of [`Line::value`], written once for every type.
    fn formatted(&mut self, arguments: fmt::Arguments<'_>) -> &mut Self {
        self.next();
        let _ = fmt::Write::write_fmt(&mut Escaping(self.out), arguments);
        self
    }

    /// A cell written in pieces: `write` writes it through a writer that
    /// escapes each piece as a cell's text is escaped.
    pub fn cell_with(&mut self, write: impl FnOnce(&mut dyn fmt::Write)) -> &mut Self {
        self.next();
        write(&mut Escaping(self.out));
        self
    }

    /// A cell of `1` for true and `0` for false.
    pub fn flag(&mut self, value: bool) -> &mut Self {
        self.value(u8::from(value))
    }

    /// An empty cell.
    pub fn empty(&mut self) -> &mut Self {
        self.next();
        self
    }

    /// `count` empty cells.
    pub fn empties(&mut self, count: usize) -> &mut Self {
        for _ in 0..count {
            self.empty();
        }
        self
    }

    /// End the line with its line break.
    pub fn end(self) {
        self.out.push('\n');
    }
}

/// A line of its own, written by `write`: what a line-maker returns for
/// an export that answers with one line.
///
/// ```
/// let text = hyper_calendar::boundary::line(|line| {
///     line.value(1).value(2);
/// });
/// assert_eq!(text, "1\t2\n");
/// ```
pub fn line(write: impl FnOnce(&mut Line<'_>)) -> String {
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    write(&mut line);
    line.end();
    out
}

/// Text written into a cell, with each tab and line break a space.
fn escape_into(out: &mut String, text: &str) {
    for character in text.chars() {
        out.push(match character {
            '\t' | '\n' | '\r' => ' ',
            other => other,
        });
    }
}

/// A writer that escapes what it is given, as a cell must.
struct Escaping<'a>(&'a mut String);

impl fmt::Write for Escaping<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        escape_into(self.0, text);
        Ok(())
    }
}

/// The cells of a line, its line break dropped: what a test reads back.
#[cfg(test)]
pub(crate) fn cells(line: &str) -> alloc::vec::Vec<&str> {
    line.trim_end_matches('\n').split('\t').collect()
}

/// The locale tag that asks for no locale in particular: for a vocabulary
/// of [`hc_i18n::reckonings`], the kind's own language.
#[cfg(feature = "i18n")]
pub const NATIVE: &str = "native";

/// A term's name in the locale a tag asks for and the tag of the data
/// that named it, as two cells, by
/// [`hc_i18n::reckonings::name_or_fallback`]: [`NATIVE`] asks for the
/// kind's own language, and a tag that does not parse is the root locale,
/// which names nothing and so falls to English and then to the kind's
/// own. A term no table names leaves both cells empty.
#[cfg(feature = "i18n")]
pub fn reckoning_name(line: &mut Line<'_>, tag: &str, kind: &str, id: &str) {
    use hc_i18n::Locale;
    let requested = (tag != NATIVE).then(|| Locale::parse(tag).unwrap_or(Locale::ROOT));
    match hc_i18n::reckonings::name_or_fallback(requested.as_ref(), kind, id) {
        Some(named) => line.cell(named.name).cell(named.tag),
        None => line.empties(2),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tab, a newline and a carriage return in a cell's text are each a
    /// space, so the line keeps its columns.
    #[test]
    fn a_cell_never_carries_a_separator() {
        let mut out = String::new();
        let mut line = Line::new(&mut out);
        line.cell("a\tb\nc\r\nd")
            .value(format_args!("x\ty"))
            .empty();
        line.end();
        assert_eq!(out, "a b c  d\tx y\t\n");
        assert_eq!(cells(&out), ["a b c  d", "x y", ""]);
    }

    /// A line of no cells is an empty line, and a line of one empty cell
    /// is too; the second cell is the first with a separator.
    #[test]
    fn the_separators_are_between_the_cells() {
        let mut out = String::new();
        Line::new(&mut out).end();
        let mut line = Line::new(&mut out);
        line.empty();
        line.end();
        let mut line = Line::new(&mut out);
        line.value_or_empty(None::<i64>).value_or_empty(Some(-3));
        line.cell_or_empty(None)
            .cell_or_empty(Some("x"))
            .flag(false);
        line.end();
        assert_eq!(out, "\n\n\t-3\t\tx\t0\n");
    }
}
