//! Greek alphabetic numerals, `grek` and `greklow`: the letters' values
//! added, 2026 as ͵ΒΚϜ´ (in capitals) or ͵βκϝ´, as CLDR 48's
//! `%greek-upper` and `%greek-lower` rules of `common/rbnf/root.xml` spell
//! them (`cldr48-rbnf`), the rules `supplemental/numberingSystems.xml`
//! names for the two systems.
//!
//! The rules: one to nine, the tens and the hundreds are a letter each, the
//! archaic ϝ (6), ϟ (90) and ϡ (900) among them; a thousand to 9 999 is the
//! thousands' letter after the lower numeral sign ͵ (U+0375), then the
//! rest; from ten thousand the number is counted in myriads, the count
//! before μ (Μ in capitals) and the rest after a space, 10⁸ as μμ and so on
//! to 10¹⁶; zero is 𐆊 (U+1018A); and the whole number takes the numeral
//! sign after it, which the rules write as the acute accent ´ (U+00B4).
//! A negative number is written after the minus sign −. The rules write
//! 10¹⁸ and more in decimal digits, which this crate does not write.
//!
//! Reading back accepts the numeral sign as the rules write it, as the
//! Greek keraia ʹ (U+0374), as the modifier prime ʹ (U+02B9) it decomposes
//! to, or as an apostrophe, and no sign at all.

use core::fmt::{self, Write};

use crate::error::{I18nError, I18nResult};

/// The letters a case writes.
#[derive(Debug)]
pub(super) struct Letters {
    units: [char; 9],
    tens: [char; 9],
    hundreds: [char; 9],
    myriad: char,
    /// Four myriad letters, for runs of one to four.
    marks: &'static str,
}

/// Capitals: `%%greek-numeral-majuscules`.
pub(super) static UPPER: Letters = Letters {
    units: ['Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ϝ', 'Ζ', 'Η', 'Θ'],
    tens: ['Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο', 'Π', 'Ϟ'],
    hundreds: ['Ρ', 'Σ', 'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω', 'Ϡ'],
    myriad: 'Μ',
    marks: "ΜΜΜΜ",
};

/// Small letters: `%%greek-numeral-minuscules`.
pub(super) static LOWER: Letters = Letters {
    units: ['α', 'β', 'γ', 'δ', 'ε', 'ϝ', 'ζ', 'η', 'θ'],
    tens: ['ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ϟ'],
    hundreds: ['ρ', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω', 'ϡ'],
    myriad: 'μ',
    marks: "μμμμ",
};

/// GREEK LOWER NUMERAL SIGN, before the thousands.
const THOUSANDS: char = '\u{0375}';
/// The numeral sign after the number, as CLDR's rules write it: ACUTE ACCENT.
const SIGN: char = '\u{00B4}';
/// The other spellings of the numeral sign a reader accepts.
const SIGNS: [char; 4] = [SIGN, '\u{0374}', '\u{02B9}', '\''];
/// GREEK ZERO SIGN.
const ZERO: char = '\u{1018A}';
/// The minus sign of the rules' `-x` rule.
const MINUS: char = '\u{2212}';
/// The first number the rules write in decimal digits.
const LIMIT: u64 = 1_000_000_000_000_000_000;

/// Write `value` in Greek numerals of one case.
pub(super) fn write<W: Write>(letters: &Letters, value: i64, out: &mut W) -> I18nResult<()> {
    let magnitude = value.unsigned_abs();
    if magnitude >= LIMIT {
        return Err(I18nError::NumberOutOfRange);
    }
    if value < 0 {
        out.write_char(MINUS)?;
    }
    numeral(letters, magnitude, out)?;
    out.write_char(SIGN)?;
    Ok(())
}

/// The private rule set: the number without its sign.
fn numeral<W: Write>(letters: &Letters, value: u64, out: &mut W) -> fmt::Result {
    let letter = |set: &[char; 9], index: u64| set[(index - 1) as usize];
    match value {
        0 => out.write_char(ZERO),
        1..=9 => out.write_char(letter(&letters.units, value)),
        10..=99 => {
            out.write_char(letter(&letters.tens, value / 10))?;
            rest(letters, value % 10, "", out)
        }
        100..=999 => {
            out.write_char(letter(&letters.hundreds, value / 100))?;
            rest(letters, value % 100, "", out)
        }
        1_000..=9_999 => {
            out.write_char(THOUSANDS)?;
            out.write_char(letter(&letters.units, value / 1_000))?;
            rest(letters, value % 1_000, "", out)
        }
        _ => {
            let mut level = 1u32;
            while value >= 10_000u64.pow(level + 1) && level < 4 {
                level += 1;
            }
            let base = 10_000u64.pow(level);
            numeral(letters, value / base, out)?;
            for _ in 0..level {
                out.write_char(letters.myriad)?;
            }
            rest(letters, value % base, " ", out)
        }
    }
}

/// An optional remainder, `[>>]`: nothing where it is zero.
fn rest<W: Write>(letters: &Letters, value: u64, before: &str, out: &mut W) -> fmt::Result {
    if value == 0 {
        return Ok(());
    }
    out.write_str(before)?;
    numeral(letters, value, out)
}

/// Whether `character` is one the system writes numbers with.
pub(super) fn writes_char(letters: &Letters, character: char) -> bool {
    letters.units.contains(&character)
        || letters.tens.contains(&character)
        || letters.hundreds.contains(&character)
        || [THOUSANDS, ZERO, MINUS, ' '].contains(&character)
        || SIGNS.contains(&character)
}

/// Read a number written in Greek numerals of one case.
pub(super) fn parse(letters: &Letters, text: &str) -> I18nResult<i64> {
    let (negative, body) = match text.strip_prefix(MINUS) {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let body = body.strip_suffix(SIGNS).unwrap_or(body);
    let value = read(letters, body, 4).ok_or(I18nError::InvalidNumber)?;
    let value = i64::try_from(value).map_err(|_| I18nError::NumberOutOfRange)?;
    Ok(if negative { -value } else { value })
}

/// The value of a numeral with at most `level` myriad marks in a group,
/// found by trying each place the marks could end its count.
fn read(letters: &Letters, text: &str, level: u32) -> Option<u64> {
    if text.chars().eq([ZERO]) {
        return Some(0);
    }
    if let Some(value) = small(letters, text) {
        return Some(value);
    }
    for count in (1..=level).rev() {
        let mark = &letters.marks[..letters.myriad.len_utf8() * count as usize];
        let mut from = 0;
        while let Some(at) = text[from..].find(mark) {
            let at = from + at;
            let (count_text, after) = (&text[..at], &text[at + mark.len()..]);
            let base = 10_000u64.pow(count);
            let quotient = read(letters, count_text, count - 1).filter(|q| (1..10_000).contains(q));
            let remainder = match after {
                "" => Some(0),
                _ => after
                    .strip_prefix(' ')
                    .and_then(|rest| read(letters, rest, count - 1))
                    .filter(|value| (1..base).contains(value)),
            };
            if let (Some(quotient), Some(remainder)) = (quotient, remainder) {
                return Some(quotient * base + remainder);
            }
            from = at + letters.myriad.len_utf8();
        }
    }
    None
}

/// A numeral below ten thousand, its letters in the one order the rules
/// write them.
fn small(letters: &Letters, text: &str) -> Option<u64> {
    let mut chars = text.chars().peekable();
    let mut value = 0u64;
    let position = |set: &[char; 9], character: char| {
        set.iter()
            .position(|letter| *letter == character)
            .map(|index| index as u64 + 1)
    };
    if chars.peek() == Some(&THOUSANDS) {
        chars.next();
        value += position(&letters.units, chars.next()?)? * 1_000;
    }
    if let Some(found) = chars.peek().and_then(|c| position(&letters.hundreds, *c)) {
        value += found * 100;
        chars.next();
    }
    if let Some(found) = chars.peek().and_then(|c| position(&letters.tens, *c)) {
        value += found * 10;
        chars.next();
    }
    if let Some(found) = chars.peek().and_then(|c| position(&letters.units, *c)) {
        value += found;
        chars.next();
    }
    (chars.next().is_none() && value > 0).then_some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;

    fn upper(value: i64) -> String {
        let mut out = String::new();
        write(&UPPER, value, &mut out).unwrap();
        out
    }

    fn lower(value: i64) -> String {
        let mut out = String::new();
        write(&LOWER, value, &mut out).unwrap();
        out
    }

    /// Values worked by hand from the rules of `common/rbnf/root.xml`.
    #[test]
    fn the_rules_spell_these_numbers() {
        assert_eq!(upper(0), "𐆊´");
        assert_eq!(upper(6), "Ϝ´");
        assert_eq!(upper(99), "ϞΘ´");
        assert_eq!(upper(666), "ΧΞϜ´");
        assert_eq!(upper(2026), "͵ΒΚϜ´");
        assert_eq!(lower(2026), "͵βκϝ´");
        assert_eq!(lower(1000), "͵α´");
        assert_eq!(lower(10_000), "αμ´");
        assert_eq!(lower(12_345), "αμ ͵βτμε´");
        assert_eq!(lower(40_000), "δμ´");
        assert_eq!(lower(400_000), "μμ´");
        assert_eq!(lower(100_000_000), "αμμ´");
        assert_eq!(lower(-7), "−ζ´");
    }

    #[test]
    fn every_number_to_a_million_reads_back_and_some_beyond() {
        for value in (0..1_000_000).chain([99_999_999, 100_000_000, 123_456_789_012]) {
            for letters in [&UPPER, &LOWER] {
                let mut out = String::new();
                write(letters, value, &mut out).unwrap();
                assert_eq!(parse(letters, &out), Ok(value), "{out}");
            }
        }
        assert_eq!(parse(&LOWER, "͵βκϝʹ"), Ok(2026));
        assert_eq!(parse(&LOWER, "͵βκϝ"), Ok(2026));
        assert_eq!(parse(&LOWER, "κ͵β"), Err(I18nError::InvalidNumber));
    }
}
