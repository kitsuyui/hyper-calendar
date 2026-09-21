//! A byte scanner shared by every parser in the crate.
//!
//! Every grammar here is ASCII, so scanning bytes rather than characters is
//! both correct and what the error offsets need to be measured in. The
//! scanner never slices a `&str` at a non-boundary, because it only ever
//! advances past bytes it has already checked are ASCII.

use crate::error::{ErrorKind, ParseError, ParseResult};

/// A cursor over the bytes of the input.
#[derive(Debug, Clone)]
pub(crate) struct Scanner<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Scanner<'a> {
    /// Start at the beginning of `text`.
    pub(crate) const fn new(text: &'a str) -> Self {
        Self {
            bytes: text.as_bytes(),
            pos: 0,
        }
    }

    /// The current byte offset.
    pub(crate) const fn pos(&self) -> usize {
        self.pos
    }

    /// Whether every byte has been consumed.
    pub(crate) const fn is_empty(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    /// The next byte without consuming it.
    pub(crate) fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    /// The byte `ahead` positions further on, without consuming anything.
    pub(crate) fn peek_ahead(&self, ahead: usize) -> Option<u8> {
        self.bytes.get(self.pos + ahead).copied()
    }

    /// The unconsumed bytes.
    pub(crate) fn rest(&self) -> &'a [u8] {
        self.bytes.get(self.pos..).unwrap_or(&[])
    }

    /// Move the cursor forward, saturating at the end of the input.
    pub(crate) fn advance(&mut self, by: usize) {
        self.pos = (self.pos + by).min(self.bytes.len());
    }

    /// An error at the current offset.
    pub(crate) const fn error(&self, kind: ErrorKind) -> ParseError {
        ParseError::new(kind, self.pos)
    }

    /// An error at an offset the caller remembered earlier.
    pub(crate) const fn error_at(kind: ErrorKind, offset: usize) -> ParseError {
        ParseError::new(kind, offset)
    }

    /// Consume `byte` if it is next.
    pub(crate) fn eat(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    /// Consume the next byte if it is any of `set`, returning which.
    pub(crate) fn eat_any(&mut self, set: &[u8]) -> Option<u8> {
        let next = self.peek()?;
        if set.contains(&next) {
            self.pos += 1;
            Some(next)
        } else {
            None
        }
    }

    /// Consume `byte`, or fail naming `literal`.
    ///
    /// # Errors
    ///
    /// [`ErrorKind::UnexpectedEnd`] at the end of input, otherwise
    /// [`ErrorKind::Literal`].
    pub(crate) fn expect(&mut self, byte: u8, literal: &'static str) -> ParseResult<()> {
        match self.peek() {
            Some(found) if found == byte => {
                self.pos += 1;
                Ok(())
            }
            Some(_) => Err(self.error(ErrorKind::Literal(literal))),
            None => Err(self.error(ErrorKind::UnexpectedEnd)),
        }
    }

    /// How many consecutive ASCII digits start at the cursor.
    pub(crate) fn digit_run(&self) -> usize {
        self.rest()
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count()
    }

    /// Consume exactly `count` digits and return their value.
    ///
    /// # Errors
    ///
    /// [`ErrorKind::UnexpectedEnd`] when the input runs out, otherwise
    /// [`ErrorKind::DigitCount`] at the offset of the first offending byte.
    pub(crate) fn take_digits(&mut self, count: usize) -> ParseResult<u64> {
        let start = self.pos;
        let mut value: u64 = 0;
        for index in 0..count {
            match self.peek_ahead(index) {
                Some(byte) if byte.is_ascii_digit() => {
                    // `count` never exceeds 18 anywhere in this crate, so the
                    // accumulator cannot overflow.
                    value = value * 10 + u64::from(byte - b'0');
                }
                Some(_) => {
                    return Err(Self::error_at(
                        ErrorKind::DigitCount(count as u8),
                        start + index,
                    ));
                }
                None => {
                    return Err(Self::error_at(ErrorKind::UnexpectedEnd, start + index));
                }
            }
        }
        self.pos += count;
        Ok(value)
    }

    /// Consume ASCII whitespace.
    pub(crate) fn skip_ascii_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.pos += 1;
        }
    }

    /// Require that nothing but the consumed value was in the input.
    ///
    /// # Errors
    ///
    /// [`ErrorKind::TrailingText`] at the first unconsumed byte.
    pub(crate) fn finish(&self) -> ParseResult<()> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self.error(ErrorKind::TrailingText))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_are_counted_before_they_are_consumed() {
        let scanner = Scanner::new("2026-09");
        assert_eq!(scanner.digit_run(), 4);
    }

    #[test]
    fn a_short_digit_field_reports_the_offset_of_the_offending_byte() {
        let mut scanner = Scanner::new("20a6");
        let error = scanner.take_digits(4).unwrap_err();
        assert_eq!(error.offset(), 2);
        assert_eq!(error.kind(), ErrorKind::DigitCount(4));
    }

    #[test]
    fn running_out_of_input_is_distinct_from_meeting_a_non_digit() {
        let mut scanner = Scanner::new("20");
        let error = scanner.take_digits(4).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnexpectedEnd);
        assert_eq!(error.offset(), 2);
    }

    #[test]
    fn trailing_text_is_reported_where_it_starts() {
        let mut scanner = Scanner::new("2026xyz");
        assert_eq!(scanner.take_digits(4).unwrap(), 2026);
        assert_eq!(scanner.finish().unwrap_err().offset(), 4);
    }
}
