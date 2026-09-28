//! Hebrew numerals, `hebr`: the letters' values added, the units last, a
//! geresh after a number of one letter and a gershayim before the last
//! letter of a longer one — 17 is י״ז, 20 is כ׳, 5787 is ה׳תשפ״ז.
//!
//! The spellings are CLDR 48's rule-based number format, the `%hebrew`,
//! `%%hebrew-0-99`, `%%hebrew-thousands` and `%hebrew-item` rules of
//! `common/rbnf/root.xml` that `numberingSystems.xml` names for `hebr`
//! (`cldr48-rbnf`), for 1 to 9 999, which is every Hebrew year the library
//! converts and every day of a month:
//!
//! - 15 and 16 are ט״ו and ט״ז, not the letters of ten and five or six, as
//!   Wikipedia's "Hebrew numerals" gives the convention
//!   (`wikipedia-hebrew-numerals`), and the rules keep the same order in
//!   298, 304, 344, 698 and 744: רח״צ, ד״ש, שד״מ, תרח״צ, תשד״מ;
//! - a hundred with nothing after it takes a geresh, ק׳, and a round
//!   number of tens after hundreds takes the gershayim before it, תש״ע,
//!   with the final form of the eighty, תש״ף;
//! - the thousands are the number of thousands, marked, before the rest:
//!   ה׳תשפ״ז; a thousand, two thousand and three thousand are the words
//!   אלף, אלפיים and ג׳ אלפים, and a round thousand from four thousand is
//!   its number of thousands alone, ה׳, which [`parse`] reads as 5: the
//!   rules write both alike.
//!
//! A reader types the geresh as an apostrophe and the gershayim as a
//! quotation mark, which UTS #35's loose matching allows for the first
//! (`uts35-v48`, "Lenient Parsing") and Wikipedia's "Gershayim" and
//! "Geresh" describe for both (`wikipedia-gershayim`,
//! `wikipedia-geresh`): [`parse`] reads י"ז as י״ז. It reads a number only
//! as the rules write it, so י״ה, the older spelling of 15, is refused.
//!
//! Leaving out the thousands of a year, תשפ״ז for 5787, is a convention of
//! dates, not of the numerals: `hc_i18n::names::DateTemplates` states it
//! for the Hebrew calendar's dates.

use core::fmt::{self, Write};

use crate::error::{I18nError, I18nResult};

/// HEBREW PUNCTUATION GERESH, after a number of one letter.
const GERESH: char = '\u{05F3}';
/// HEBREW PUNCTUATION GERSHAYIM, before the last letter of a longer one.
const GERSHAYIM: char = '\u{05F4}';

/// The largest value written.
const MAX: i64 = 9_999;

/// The letters of one to nine.
const UNITS: [char; 9] = ['א', 'ב', 'ג', 'ד', 'ה', 'ו', 'ז', 'ח', 'ט'];
/// The letters of ten to ninety.
const TENS: [char; 9] = ['י', 'כ', 'ל', 'מ', 'נ', 'ס', 'ע', 'פ', 'צ'];
/// The letters of one hundred to nine hundred.
const HUNDREDS: [&str; 9] = ["ק", "ר", "ש", "ת", "תק", "תר", "תש", "תת", "תתק"];

/// The hundreds the rules spell whole: the order changed to avoid a word,
/// and the round hundreds past four hundred, marked between their letters.
const WHOLE: [(i64, &str); 10] = [
    (298, "רח״צ"),
    (304, "ד״ש"),
    (344, "שד״מ"),
    (500, "ת״ק"),
    (600, "ת״ר"),
    (698, "תרח״צ"),
    (700, "ת״ש"),
    (744, "תשד״מ"),
    (800, "ת״ת"),
    (900, "תת״ק"),
];

/// Index into [`UNITS`] or [`TENS`] of a digit from 1 to 9.
fn digit(value: i64) -> usize {
    usize::try_from(value - 1).unwrap_or(0).min(8)
}

/// Write `value`, 1 to 9 999.
pub(super) fn write<W: Write>(value: i64, out: &mut W) -> I18nResult<()> {
    if !(1..=MAX).contains(&value) {
        return Err(I18nError::NumberOutOfRange);
    }
    match value {
        1_000 => out.write_str("אלף")?,
        2_000 => out.write_str("אלפיים")?,
        3_000 => {
            below_thousand(3, out)?;
            out.write_str(" אלפים")?;
        }
        1_001.. => {
            below_thousand(value / 1_000, out)?;
            if value % 1_000 != 0 {
                below_thousand(value % 1_000, out)?;
            }
        }
        _ => below_thousand(value, out)?,
    }
    Ok(())
}

/// `%hebrew` from 1 to 999.
fn below_thousand<W: Write>(value: i64, out: &mut W) -> fmt::Result {
    if value < 100 {
        return below_hundred(value, out);
    }
    if let Some((_, whole)) = WHOLE.iter().find(|(known, _)| *known == value) {
        return out.write_str(whole);
    }
    out.write_str(HUNDREDS[digit(value / 100)])?;
    after_hundreds(value % 100, out)
}

/// `%hebrew` from 1 to 99: a geresh after one letter, a gershayim before
/// the last of two.
fn below_hundred<W: Write>(value: i64, out: &mut W) -> fmt::Result {
    match value {
        15 => out.write_str("ט״ו"),
        16 => out.write_str("ט״ז"),
        1..=9 => {
            out.write_char(UNITS[digit(value)])?;
            out.write_char(GERESH)
        }
        _ if value % 10 == 0 => {
            out.write_char(TENS[digit(value / 10)])?;
            out.write_char(GERESH)
        }
        _ => {
            out.write_char(TENS[digit(value / 10)])?;
            out.write_char(GERSHAYIM)?;
            out.write_char(UNITS[digit(value % 10)])
        }
    }
}

/// `%%hebrew-0-99`, what follows the hundreds: a geresh where nothing
/// does, and the gershayim before a last letter that stands alone.
fn after_hundreds<W: Write>(value: i64, out: &mut W) -> fmt::Result {
    match value {
        0 => out.write_char(GERESH),
        15 => out.write_str("ט״ו"),
        16 => out.write_str("ט״ז"),
        1..=9 => {
            out.write_char(GERSHAYIM)?;
            out.write_char(UNITS[digit(value)])
        }
        80 => {
            out.write_char(GERSHAYIM)?;
            out.write_char('ף')
        }
        _ if value % 10 == 0 => {
            out.write_char(GERSHAYIM)?;
            out.write_char(TENS[digit(value / 10)])
        }
        _ => {
            out.write_char(TENS[digit(value / 10)])?;
            out.write_char(GERSHAYIM)?;
            out.write_char(UNITS[digit(value % 10)])
        }
    }
}

/// A letter's value, its final form's too.
fn letter_value(letter: char) -> Option<i64> {
    let value = match letter {
        'ך' => 20,
        'ם' => 40,
        'ן' => 50,
        'ף' => 80,
        'ץ' => 90,
        'ק' => 100,
        'ר' => 200,
        'ש' => 300,
        'ת' => 400,
        _ => {
            let unit = UNITS.iter().position(|known| *known == letter);
            let ten = TENS.iter().position(|known| *known == letter);
            return match (unit, ten) {
                (Some(index), _) => i64::try_from(index + 1).ok(),
                (_, Some(index)) => i64::try_from((index + 1) * 10).ok(),
                _ => None,
            };
        }
    };
    Some(value)
}

/// Whether a character of the text is a geresh, typed or not.
const fn is_geresh(character: char) -> bool {
    matches!(character, GERESH | '\'')
}

/// Whether a written character and a typed one are the same: the geresh
/// and an apostrophe, the gershayim and a quotation mark.
pub(super) fn same_mark(written: char, typed: char) -> bool {
    written == typed
        || (written == GERESH && typed == '\'')
        || (written == GERSHAYIM && typed == '"')
}

/// A sink that checks what [`write`] writes against a text.
struct Matches<'t> {
    rest: core::str::Chars<'t>,
}

impl Write for Matches<'_> {
    fn write_str(&mut self, written: &str) -> fmt::Result {
        for character in written.chars() {
            match self.rest.next() {
                Some(typed) if same_mark(character, typed) => {}
                _ => return Err(fmt::Error),
            }
        }
        Ok(())
    }
}

/// Whether `value` is written as `text`.
fn writes_as(value: i64, text: &str) -> bool {
    let mut matches = Matches { rest: text.chars() };
    write(value, &mut matches).is_ok() && matches.rest.next().is_none()
}

/// Read a number written as [`write`] writes it, with a typed geresh or
/// gershayim where the text has one.
pub(super) fn parse(text: &str) -> I18nResult<i64> {
    let mut sum = 0_i64;
    let mut letters = 0_usize;
    for character in text.chars() {
        match letter_value(character) {
            Some(value) => {
                sum += value;
                letters += 1;
            }
            None if is_geresh(character) || matches!(character, GERSHAYIM | '"' | ' ') => {}
            None => return Err(I18nError::InvalidNumber),
        }
        if letters > 16 {
            return Err(I18nError::InvalidNumber);
        }
    }
    // The thousands: a letter and its geresh before the rest.
    let mut characters = text.chars();
    let thousands = match (characters.next(), characters.next()) {
        (Some(first), Some(mark)) if is_geresh(mark) && letters > 1 => letter_value(first)
            .filter(|value| *value < 10)
            .map(|value| value * 1_000 + sum - value),
        _ => None,
    };
    [Some(sum), thousands, Some(1_000), Some(2_000), Some(3_000)]
        .into_iter()
        .flatten()
        .find(|value| writes_as(*value, text))
        .ok_or(I18nError::InvalidNumber)
}
