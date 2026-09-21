//! Errors raised while parsing locales or rendering localised numbers.

use core::fmt;

/// Result alias for internationalisation operations.
pub type I18nResult<T> = Result<T, I18nError>;

/// What can go wrong in this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum I18nError {
    /// The language tag was empty.
    EmptyTag,
    /// The language tag is not well-formed BCP 47.
    InvalidTag,
    /// A subtag was longer than the grammar or this crate's storage allows.
    SubtagTooLong,
    /// The tag carried more subtags than this crate stores.
    TooManySubtags,
    /// A `-u-` extension key this crate does not model was present.
    ///
    /// The crate refuses rather than silently dropping the key, because a
    /// dropped key would make a parse/render round-trip lossy.
    UnknownExtensionKey,
    /// A singleton extension other than `-u-` (`-t-`, `-x-`, …) was present.
    UnsupportedExtension,
    /// A `-u-` extension key was present with a value it does not accept.
    InvalidExtensionValue,
    /// No numbering system is registered under that identifier.
    UnknownNumberingSystem,
    /// The value cannot be written in the requested numbering system.
    NumberOutOfRange,
    /// The text is not a number in the requested numbering system.
    InvalidNumber,
    /// The output sink refused the write.
    WriteFailed,
}

impl fmt::Display for I18nError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTag => f.write_str("empty language tag"),
            Self::InvalidTag => f.write_str("not a well-formed BCP 47 language tag"),
            Self::SubtagTooLong => f.write_str("subtag longer than the grammar allows"),
            Self::TooManySubtags => f.write_str("language tag has too many subtags"),
            Self::UnknownExtensionKey => f.write_str("unmodelled -u- extension key"),
            Self::UnsupportedExtension => f.write_str("only the -u- extension is supported"),
            Self::InvalidExtensionValue => f.write_str("invalid value for a -u- extension key"),
            Self::UnknownNumberingSystem => f.write_str("unknown numbering system"),
            Self::NumberOutOfRange => f.write_str("value out of range for this numbering system"),
            Self::InvalidNumber => f.write_str("not a number in this numbering system"),
            Self::WriteFailed => f.write_str("the output sink refused the write"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for I18nError {}

impl From<fmt::Error> for I18nError {
    fn from(_: fmt::Error) -> Self {
        Self::WriteFailed
    }
}
