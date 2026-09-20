//! Fixed-capacity ASCII storage.
//!
//! A locale is a handful of very short ASCII strings. Storing them inline
//! keeps [`crate::locale::Locale`] `Copy` and keeps the whole crate usable
//! on a target with no allocator, which is the only reason this module
//! exists rather than a `String` field.

use core::fmt;

/// How a subtag is normalised when it is stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Case {
    /// Language subtags, extension keys and extension values.
    Lower,
    /// Region subtags.
    Upper,
    /// Script subtags: first letter upper, rest lower.
    Title,
}

/// An inline ASCII string of at most `N` bytes.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Subtag<const N: usize> {
    bytes: [u8; N],
    len: u8,
}

impl<const N: usize> Subtag<N> {
    /// Store a literal that is already known to be well-formed and
    /// normalised. Truncates silently, so it is only for crate constants.
    pub(crate) const fn literal(text: &str) -> Self {
        let source = text.as_bytes();
        let mut bytes = [0u8; N];
        let mut index = 0;
        while index < source.len() && index < N {
            bytes[index] = source[index];
            index += 1;
        }
        Self {
            bytes,
            len: index as u8,
        }
    }

    /// Store `text`, normalising its case and rejecting anything that is not
    /// ASCII alphanumeric or a hyphen.
    ///
    /// The hyphen is allowed because a `-u-ca-` value can span several
    /// subtags: `islamic-umalqura` is one calendar, not two.
    pub(crate) fn normalised(text: &str, case: Case) -> Option<Self> {
        if text.is_empty() || text.len() > N {
            return None;
        }
        let mut bytes = [0u8; N];
        let mut previous_was_boundary = true;
        for (index, byte) in text.as_bytes().iter().enumerate() {
            if !byte.is_ascii_alphanumeric() && *byte != b'-' {
                return None;
            }
            bytes[index] = match case {
                Case::Lower => byte.to_ascii_lowercase(),
                Case::Upper => byte.to_ascii_uppercase(),
                Case::Title => {
                    if previous_was_boundary {
                        byte.to_ascii_uppercase()
                    } else {
                        byte.to_ascii_lowercase()
                    }
                }
            };
            previous_was_boundary = *byte == b'-';
        }
        Some(Self {
            bytes,
            len: text.len() as u8,
        })
    }

    /// The stored text.
    pub(crate) fn as_str(&self) -> &str {
        // The constructors only ever store ASCII, so this cannot fail; the
        // fallback keeps the crate free of `unwrap`.
        core::str::from_utf8(&self.bytes[..self.len as usize]).unwrap_or("")
    }
}

impl<const N: usize> fmt::Debug for Subtag<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<const N: usize> fmt::Display for Subtag<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A `core::fmt::Write` sink backed by an inline buffer.
///
/// Used to render a locale into a comparable tag without allocating, which
/// is how data lookup works when the crate is built without `alloc`.
#[derive(Clone, Copy)]
pub(crate) struct StackString<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> StackString<N> {
    pub(crate) const fn new() -> Self {
        Self {
            bytes: [0u8; N],
            len: 0,
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }
}

impl<const N: usize> fmt::Write for StackString<N> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let source = text.as_bytes();
        if self.len + source.len() > N {
            return Err(fmt::Error);
        }
        self.bytes[self.len..self.len + source.len()].copy_from_slice(source);
        self.len += source.len();
        Ok(())
    }
}

impl<const N: usize> Default for StackString<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Write as _;

    #[test]
    fn subtags_normalise_their_case_on_the_way_in() {
        let language = Subtag::<8>::normalised("JA", Case::Lower).unwrap();
        assert_eq!(language.as_str(), "ja");
        let script = Subtag::<4>::normalised("hANS", Case::Title).unwrap();
        assert_eq!(script.as_str(), "Hans");
        let region = Subtag::<3>::normalised("jp", Case::Upper).unwrap();
        assert_eq!(region.as_str(), "JP");
    }

    #[test]
    fn a_hyphenated_extension_value_titlecases_each_piece() {
        let value = Subtag::<20>::normalised("ISLAMIC-UMALQURA", Case::Lower).unwrap();
        assert_eq!(value.as_str(), "islamic-umalqura");
        let titled = Subtag::<20>::normalised("islamic-umalqura", Case::Title).unwrap();
        assert_eq!(titled.as_str(), "Islamic-Umalqura");
    }

    #[test]
    fn subtags_reject_overlong_and_non_ascii_input() {
        assert!(Subtag::<4>::normalised("toolong", Case::Lower).is_none());
        assert!(Subtag::<8>::normalised("", Case::Lower).is_none());
        assert!(Subtag::<8>::normalised("j a", Case::Lower).is_none());
    }

    #[test]
    fn the_stack_sink_refuses_to_overflow() {
        let mut sink = StackString::<4>::new();
        assert!(sink.write_str("ab").is_ok());
        assert!(sink.write_str("cd").is_ok());
        assert!(sink.write_str("e").is_err());
        assert_eq!(sink.as_str(), "abcd");
    }
}
