//! What the WebAssembly module and the C library share about refusing.
//!
//! The line-makers written once for both boundary crates —
//! [`crate::time_lines`], [`crate::astro_lines`] and their siblings —
//! refuse with a [`Refusal`], and each boundary spells it its own way: a
//! sentinel at or below `HC_ERR_FLOOR` in the module, an `HcStatus` in the
//! library. The kinds are the ones both already have; a new kind would be
//! a new sentinel and a new status code, which is a change to both
//! surfaces, not to this enum alone.

use alloc::string::String;

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

/// The result every shared line-maker returns.
pub type Answer<T> = Result<T, Refusal>;

/// Append one cell of a line: `text` with any tab or line break replaced
/// by a space, so the line format survives whatever a source string holds.
pub fn push_cell(out: &mut String, text: &str) {
    for character in text.chars() {
        out.push(match character {
            '\t' | '\n' | '\r' => ' ',
            other => other,
        });
    }
}

/// Whether `given` names `name`: the same letters, in any ASCII case,
/// with the surrounding white space ignored.
#[must_use]
pub fn names(given: &str, name: &str) -> bool {
    given.trim().eq_ignore_ascii_case(name)
}

/// The locale tag that asks for no locale in particular: for a vocabulary
/// of [`hc_i18n::reckonings`], the kind's own language.
#[cfg(feature = "i18n")]
pub const NATIVE: &str = "native";

/// Append a term's name in the locale a tag asks for and the tag of the
/// data that named it, as two cells, by
/// [`hc_i18n::reckonings::name_or_fallback`]: [`NATIVE`] asks for the
/// kind's own language, and a tag that does not parse is the root locale,
/// which names nothing and so falls to English and then to the kind's
/// own. A term no table names leaves both cells empty.
#[cfg(feature = "i18n")]
pub fn push_reckoning_name(out: &mut String, tag: &str, kind: &str, id: &str) {
    use hc_i18n::Locale;
    let requested = (tag != NATIVE).then(|| Locale::parse(tag).unwrap_or(Locale::ROOT));
    match hc_i18n::reckonings::name_or_fallback(requested.as_ref(), kind, id) {
        Some(named) => {
            push_cell(out, named.name);
            out.push('\t');
            push_cell(out, named.tag);
        }
        None => out.push('\t'),
    }
}
