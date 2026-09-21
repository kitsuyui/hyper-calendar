//! Numbering systems: the shapes a year, month or day number is written in.
//!
//! Two kinds exist, and the difference is not cosmetic:
//!
//! * **Positional** systems are ten digit code points substituted one for
//!   one: Arabic-Indic `٢٠٢٤`, Devanagari `२०२४`, Thai `๒๐๒๔`. Rendering is
//!   a digit-table lookup and parsing is its inverse.
//! * **Algorithmic** systems spell the number out. Han numerals are the case
//!   this crate needs, because a Japanese era year is written `二十三年`,
//!   not `二三年`, and the first year of an era is `元年` rather than
//!   `一年`.
//!
//! The identifiers are the CLDR `numberingSystems.xml` ones, so a
//! `-u-nu-` extension value can be looked up directly.
//!
//! # What is deliberately absent
//!
//! No grouping separators, no decimal separator, no sign other than an
//! ASCII hyphen, no currency and no rule-based spellout of ordinals: a
//! calendar field is an integer, and the surrounding pattern is
//! `hc-format`'s problem. Hebrew and Greek alphabetic numerals, and the
//! Chinese counting-rod and `hanidays` systems, are not implemented.

use core::fmt;

use crate::error::{I18nError, I18nResult};
use crate::locale::Locale;

/// How a numbering system turns an integer into text.
#[derive(Debug, Clone, Copy)]
enum Kind {
    /// Ten digits substituted positionally.
    Positional(&'static [char; 10]),
    /// Han numerals, spelled out with unit words.
    Han(&'static HanStyle),
}

/// The pieces a Han numeral style is built from.
///
/// Splitting the style out is what lets Japanese, Japanese financial,
/// simplified Chinese and traditional Chinese share one algorithm: they
/// differ only in glyphs and in two conventions about the digit one.
#[derive(Debug)]
struct HanStyle {
    /// Digits zero through nine.
    digits: [&'static str; 10],
    /// 10, 100, 1000.
    ten: &'static str,
    hundred: &'static str,
    thousand: &'static str,
    /// The myriad units: 10^4, 10^8, 10^12.
    myriad: &'static str,
    hundred_million: &'static str,
    trillion: &'static str,
    /// Japanese drops the one before every unit: 110 is 百十.
    omit_one_before_units: bool,
    /// Chinese drops it only when ten leads the whole number: 十五 but
    /// 一百一十五.
    omit_leading_ten: bool,
    /// Chinese marks an interior gap: 一百〇五. Japanese does not: 百五.
    zero_filler: Option<&'static str>,
}

/// Latin digits.
const LATN_DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
/// Arabic-Indic digits, used for Arabic.
const ARAB_DIGITS: [char; 10] = ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'];
/// Extended Arabic-Indic digits, used for Persian and Urdu.
const ARABEXT_DIGITS: [char; 10] = ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'];
/// Devanagari digits.
const DEVA_DIGITS: [char; 10] = ['०', '१', '२', '३', '४', '५', '६', '७', '८', '९'];
/// Bengali digits.
const BENG_DIGITS: [char; 10] = ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'];
/// Thai digits.
const THAI_DIGITS: [char; 10] = ['๐', '๑', '๒', '๓', '๔', '๕', '๖', '๗', '๘', '๙'];
/// Myanmar digits.
const MYMR_DIGITS: [char; 10] = ['၀', '၁', '၂', '၃', '၄', '၅', '၆', '၇', '၈', '၉'];
/// Positional Han digits: 2024 is 二〇二四, digit by digit.
const HANIDEC_DIGITS: [char; 10] = ['〇', '一', '二', '三', '四', '五', '六', '七', '八', '九'];
/// Fullwidth Latin digits.
const FULLWIDE_DIGITS: [char; 10] = ['０', '１', '２', '３', '４', '５', '６', '７', '８', '９'];

/// Japanese Han numerals.
static JPAN_STYLE: HanStyle = HanStyle {
    digits: ["〇", "一", "二", "三", "四", "五", "六", "七", "八", "九"],
    ten: "十",
    hundred: "百",
    thousand: "千",
    myriad: "万",
    hundred_million: "億",
    trillion: "兆",
    omit_one_before_units: true,
    omit_leading_ten: false,
    zero_filler: None,
};

/// Japanese financial (大字) numerals.
///
/// Banks and deeds use these because 一 cannot be turned into 二 or 三 with
/// one stroke of a pen, which is also why this style never omits the one:
/// 拾 alone could be padded, 壱拾 cannot.
static JPANFIN_STYLE: HanStyle = HanStyle {
    digits: ["零", "壱", "弐", "参", "四", "伍", "六", "七", "八", "九"],
    ten: "拾",
    hundred: "百",
    thousand: "阡",
    myriad: "萬",
    hundred_million: "億",
    trillion: "兆",
    omit_one_before_units: false,
    omit_leading_ten: false,
    zero_filler: None,
};

/// Simplified Chinese Han numerals.
static HANS_STYLE: HanStyle = HanStyle {
    digits: ["〇", "一", "二", "三", "四", "五", "六", "七", "八", "九"],
    ten: "十",
    hundred: "百",
    thousand: "千",
    myriad: "万",
    hundred_million: "亿",
    trillion: "兆",
    omit_one_before_units: false,
    omit_leading_ten: true,
    zero_filler: Some("〇"),
};

/// Traditional Chinese Han numerals.
static HANT_STYLE: HanStyle = HanStyle {
    digits: ["〇", "一", "二", "三", "四", "五", "六", "七", "八", "九"],
    ten: "十",
    hundred: "百",
    thousand: "千",
    myriad: "萬",
    hundred_million: "億",
    trillion: "兆",
    omit_one_before_units: false,
    omit_leading_ten: true,
    zero_filler: Some("〇"),
};

/// A numbering system: an identifier plus the way it writes integers.
#[derive(Debug, Clone, Copy)]
pub struct NumberingSystem {
    id: &'static str,
    kind: Kind,
}

/// The one system every locale can fall back to.
pub static LATN: NumberingSystem = NumberingSystem {
    id: "latn",
    kind: Kind::Positional(&LATN_DIGITS),
};

/// Every numbering system this crate knows, in identifier order.
///
/// Adding a positional system is one entry here plus one digit table; no
/// function changes.
pub static ALL: &[NumberingSystem] = &[
    NumberingSystem {
        id: "arab",
        kind: Kind::Positional(&ARAB_DIGITS),
    },
    NumberingSystem {
        id: "arabext",
        kind: Kind::Positional(&ARABEXT_DIGITS),
    },
    NumberingSystem {
        id: "beng",
        kind: Kind::Positional(&BENG_DIGITS),
    },
    NumberingSystem {
        id: "deva",
        kind: Kind::Positional(&DEVA_DIGITS),
    },
    NumberingSystem {
        id: "fullwide",
        kind: Kind::Positional(&FULLWIDE_DIGITS),
    },
    NumberingSystem {
        id: "hanidec",
        kind: Kind::Positional(&HANIDEC_DIGITS),
    },
    NumberingSystem {
        id: "hans",
        kind: Kind::Han(&HANS_STYLE),
    },
    NumberingSystem {
        id: "hant",
        kind: Kind::Han(&HANT_STYLE),
    },
    NumberingSystem {
        id: "jpan",
        kind: Kind::Han(&JPAN_STYLE),
    },
    NumberingSystem {
        id: "jpanfin",
        kind: Kind::Han(&JPANFIN_STYLE),
    },
    NumberingSystem {
        id: "latn",
        kind: Kind::Positional(&LATN_DIGITS),
    },
    NumberingSystem {
        id: "mymr",
        kind: Kind::Positional(&MYMR_DIGITS),
    },
    NumberingSystem {
        id: "thai",
        kind: Kind::Positional(&THAI_DIGITS),
    },
];

/// The marker written instead of `一` for the first year of a Japanese era.
///
/// 令和元年 is the year Reiwa began; 令和一年 is not written, although it is
/// understood. The same convention applies to Chinese era years.
pub const FIRST_YEAR_MARKER: &str = "元";

/// The largest magnitude the Han styles can write, exclusive.
///
/// The unit table stops at 兆 (10^12), so the largest writable group ends at
/// 9999兆 = 10^16 − 1. Bigger values need 京 and are refused rather than
/// rendered wrongly.
const HAN_LIMIT: u128 = 10_000_000_000_000_000;

impl NumberingSystem {
    /// Look a system up by its CLDR identifier.
    #[must_use]
    pub fn from_id(id: &str) -> Option<&'static Self> {
        ALL.iter().find(|system| system.id == id)
    }

    /// The system a locale asks for.
    ///
    /// `-u-nu-` wins; then the locale's own default from [`crate::data`];
    /// then `latn`.
    #[must_use]
    pub fn for_locale(locale: &Locale) -> &'static Self {
        locale
            .numbering_system()
            .and_then(Self::from_id)
            .or_else(|| Self::from_id(crate::names::locale_data(locale).numbering))
            .unwrap_or(&LATN)
    }

    /// The CLDR identifier.
    #[must_use]
    pub const fn id(&self) -> &'static str {
        self.id
    }

    /// Whether the system spells numbers out rather than substituting digits.
    #[must_use]
    pub const fn is_algorithmic(&self) -> bool {
        matches!(self.kind, Kind::Han(_))
    }

    /// The ten digits, for a positional system.
    #[must_use]
    pub const fn digits(&self) -> Option<&'static [char; 10]> {
        match self.kind {
            Kind::Positional(digits) => Some(digits),
            Kind::Han(_) => None,
        }
    }

    /// Write an integer in this system.
    ///
    /// # Errors
    ///
    /// Returns [`I18nError::NumberOutOfRange`] if an algorithmic system
    /// cannot express the magnitude, and [`I18nError::WriteFailed`] if the
    /// sink refuses a write.
    pub fn write_integer<W: fmt::Write>(&self, value: i64, out: &mut W) -> I18nResult<()> {
        match self.kind {
            Kind::Positional(digits) => write_positional(digits, value, out),
            Kind::Han(style) => write_han(style, value, out),
        }
    }

    /// Render an integer in this system.
    ///
    /// # Errors
    ///
    /// As [`NumberingSystem::write_integer`].
    #[cfg(feature = "alloc")]
    pub fn format_integer(&self, value: i64) -> I18nResult<alloc::string::String> {
        let mut text = alloc::string::String::new();
        self.write_integer(value, &mut text)?;
        Ok(text)
    }

    /// Read an integer back out of this system's notation.
    ///
    /// # Errors
    ///
    /// Returns [`I18nError::InvalidNumber`] if the text is not a number in
    /// this system, and [`I18nError::NumberOutOfRange`] if it is one but does
    /// not fit in an `i64`.
    pub fn parse_integer(&self, text: &str) -> I18nResult<i64> {
        match self.kind {
            Kind::Positional(digits) => parse_positional(digits, text),
            Kind::Han(style) => parse_han(style, text),
        }
    }
}

fn write_positional<W: fmt::Write>(digits: &[char; 10], value: i64, out: &mut W) -> I18nResult<()> {
    // i64::MIN has no positive counterpart, so the magnitude is taken in i128.
    let mut magnitude = i128::from(value).unsigned_abs();
    if value < 0 {
        out.write_char('-')?;
    }
    if magnitude == 0 {
        out.write_char(digits[0])?;
        return Ok(());
    }
    let mut buffer = ['\0'; 20];
    let mut length = 0usize;
    while magnitude > 0 {
        buffer[length] = digits[(magnitude % 10) as usize];
        magnitude /= 10;
        length += 1;
    }
    for index in (0..length).rev() {
        out.write_char(buffer[index])?;
    }
    Ok(())
}

fn parse_positional(digits: &[char; 10], text: &str) -> I18nResult<i64> {
    let (negative, body) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    if body.is_empty() {
        return Err(I18nError::InvalidNumber);
    }
    let mut accumulated: i128 = 0;
    for character in body.chars() {
        let digit = digits
            .iter()
            .position(|candidate| *candidate == character)
            .ok_or(I18nError::InvalidNumber)?;
        accumulated = accumulated
            .checked_mul(10)
            .and_then(|value| value.checked_add(digit as i128))
            .ok_or(I18nError::NumberOutOfRange)?;
        if accumulated > i128::from(i64::MAX) + 1 {
            return Err(I18nError::NumberOutOfRange);
        }
    }
    let signed = if negative { -accumulated } else { accumulated };
    i64::try_from(signed).map_err(|_| I18nError::NumberOutOfRange)
}

fn write_han<W: fmt::Write>(style: &HanStyle, value: i64, out: &mut W) -> I18nResult<()> {
    if value == 0 {
        out.write_str(style.digits[0])?;
        return Ok(());
    }
    let magnitude = i128::from(value).unsigned_abs();
    if magnitude >= HAN_LIMIT {
        return Err(I18nError::NumberOutOfRange);
    }
    if value < 0 {
        out.write_char('-')?;
    }
    let scales: [(u128, &str); 4] = [
        (1_000_000_000_000, style.trillion),
        (100_000_000, style.hundred_million),
        (10_000, style.myriad),
        (1, ""),
    ];
    let mut written = false;
    for (scale, unit) in scales {
        let group = ((magnitude / scale) % 10_000) as u16;
        if group == 0 {
            continue;
        }
        if written && group < 1_000 {
            // A lower group that does not start at its thousands place has a
            // gap in front of it: 一万零五, never 一万五 in Chinese.
            if let Some(filler) = style.zero_filler {
                out.write_str(filler)?;
            }
        }
        write_han_group(style, group, !written, out)?;
        out.write_str(unit)?;
        written = true;
    }
    Ok(())
}

fn write_han_group<W: fmt::Write>(
    style: &HanStyle,
    group: u16,
    leading_group: bool,
    out: &mut W,
) -> I18nResult<()> {
    let digits = [
        (group / 1000) as usize,
        (group / 100 % 10) as usize,
        (group / 10 % 10) as usize,
        (group % 10) as usize,
    ];
    let units = [style.thousand, style.hundred, style.ten, ""];
    let mut started = false;
    let mut pending_zero = false;
    for position in 0..4 {
        let digit = digits[position];
        if digit == 0 {
            if started {
                pending_zero = true;
            }
            continue;
        }
        if pending_zero {
            if let Some(filler) = style.zero_filler {
                out.write_str(filler)?;
            }
            pending_zero = false;
        }
        let omit_one = digit == 1
            && position < 3
            && (style.omit_one_before_units
                || (style.omit_leading_ten && leading_group && !started && position == 2));
        if !omit_one {
            out.write_str(style.digits[digit])?;
        }
        out.write_str(units[position])?;
        started = true;
    }
    Ok(())
}

fn parse_han(style: &HanStyle, text: &str) -> I18nResult<i64> {
    let (negative, mut rest) = match text.strip_prefix('-') {
        Some(body) => (true, body),
        None => (false, text),
    };
    if rest.is_empty() {
        return Err(I18nError::InvalidNumber);
    }

    let small_units: [(&str, i128); 3] = [
        (style.thousand, 1_000),
        (style.hundred, 100),
        (style.ten, 10),
    ];
    let big_units: [(&str, i128); 3] = [
        (style.trillion, 1_000_000_000_000),
        (style.hundred_million, 100_000_000),
        (style.myriad, 10_000),
    ];

    let mut total: i128 = 0;
    let mut section: i128 = 0;
    let mut pending: Option<i128> = None;

    'outer: while !rest.is_empty() {
        // Both spellings of zero are accepted whatever the style renders,
        // because 一百〇五 and 一百零五 are the same number.
        for zero in ["〇", "零"] {
            if let Some(remainder) = rest.strip_prefix(zero) {
                pending = Some(0);
                rest = remainder;
                continue 'outer;
            }
        }
        for (digit, glyph) in style.digits.iter().enumerate() {
            if let Some(remainder) = rest.strip_prefix(glyph) {
                pending = Some(digit as i128);
                rest = remainder;
                continue 'outer;
            }
        }
        for (glyph, scale) in small_units {
            if let Some(remainder) = rest.strip_prefix(glyph) {
                section += pending.take().unwrap_or(1) * scale;
                rest = remainder;
                continue 'outer;
            }
        }
        for (glyph, scale) in big_units {
            if let Some(remainder) = rest.strip_prefix(glyph) {
                section += pending.take().unwrap_or(0);
                total += section * scale;
                section = 0;
                rest = remainder;
                continue 'outer;
            }
        }
        return Err(I18nError::InvalidNumber);
    }

    total += section + pending.unwrap_or(0);
    let signed = if negative { -total } else { total };
    i64::try_from(signed).map_err(|_| I18nError::NumberOutOfRange)
}

/// Write a Japanese era year, using 元 for the first year of the era.
///
/// # Errors
///
/// Returns [`I18nError::NumberOutOfRange`] for a year below 1, and passes on
/// the errors of [`NumberingSystem::write_integer`].
pub fn write_japanese_era_year<W: fmt::Write>(year: i64, out: &mut W) -> I18nResult<()> {
    if year < 1 {
        return Err(I18nError::NumberOutOfRange);
    }
    if year == 1 {
        out.write_str(FIRST_YEAR_MARKER)?;
        return Ok(());
    }
    write_han(&JPAN_STYLE, year, out)
}

/// Read a Japanese era year back, accepting 元 for year 1.
///
/// # Errors
///
/// Returns [`I18nError::InvalidNumber`] if the text is neither 元 nor a
/// Japanese Han numeral.
pub fn parse_japanese_era_year(text: &str) -> I18nResult<i64> {
    if text == FIRST_YEAR_MARKER {
        return Ok(1);
    }
    parse_han(&JPAN_STYLE, text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;

    fn render(id: &str, value: i64) -> String {
        NumberingSystem::from_id(id)
            .unwrap()
            .format_integer(value)
            .unwrap()
    }

    fn read(id: &str, text: &str) -> i64 {
        NumberingSystem::from_id(id)
            .unwrap()
            .parse_integer(text)
            .unwrap()
    }

    #[test]
    fn every_registered_system_is_reachable_by_its_identifier() {
        for system in ALL {
            assert_eq!(
                NumberingSystem::from_id(system.id()).unwrap().id(),
                system.id()
            );
        }
        assert!(NumberingSystem::from_id("rodrigues").is_none());
    }

    #[test]
    fn the_identifier_table_is_sorted_and_unique() {
        for pair in ALL.windows(2) {
            assert!(
                pair[0].id() < pair[1].id(),
                "{} !< {}",
                pair[0].id(),
                pair[1].id()
            );
        }
    }

    #[test]
    fn positional_systems_substitute_digit_for_digit() {
        assert_eq!(render("latn", 2024), "2024");
        assert_eq!(render("arab", 2024), "٢٠٢٤");
        assert_eq!(render("arabext", 2024), "۲۰۲۴");
        assert_eq!(render("deva", 2024), "२०२४");
        assert_eq!(render("beng", 2024), "২০২৪");
        assert_eq!(render("thai", 2024), "๒๐๒๔");
        assert_eq!(render("mymr", 2024), "၂၀၂၄");
        assert_eq!(render("hanidec", 2024), "二〇二四");
        assert_eq!(render("fullwide", 2024), "２０２４");
    }

    #[test]
    fn positional_systems_round_trip_across_four_digits() {
        for system in ALL.iter().filter(|system| !system.is_algorithmic()) {
            for value in -100..=2100 {
                let text = system.format_integer(value).unwrap();
                assert_eq!(
                    system.parse_integer(&text).unwrap(),
                    value,
                    "{} {value}",
                    system.id()
                );
            }
        }
    }

    #[test]
    fn positional_systems_round_trip_the_extremes_of_i64() {
        for system in ALL.iter().filter(|system| !system.is_algorithmic()) {
            for value in [i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX] {
                let text = system.format_integer(value).unwrap();
                assert_eq!(system.parse_integer(&text).unwrap(), value);
            }
        }
    }

    #[test]
    fn a_positional_parse_rejects_foreign_digits_and_overflow() {
        let latn = NumberingSystem::from_id("latn").unwrap();
        assert_eq!(latn.parse_integer("٢٠٢٤"), Err(I18nError::InvalidNumber));
        assert_eq!(latn.parse_integer(""), Err(I18nError::InvalidNumber));
        assert_eq!(latn.parse_integer("-"), Err(I18nError::InvalidNumber));
        assert_eq!(
            latn.parse_integer("99999999999999999999"),
            Err(I18nError::NumberOutOfRange)
        );
    }

    #[test]
    fn japanese_han_numerals_omit_the_one_before_a_unit() {
        assert_eq!(render("jpan", 1), "一");
        assert_eq!(render("jpan", 10), "十");
        assert_eq!(render("jpan", 11), "十一");
        assert_eq!(render("jpan", 23), "二十三");
        assert_eq!(render("jpan", 100), "百");
        assert_eq!(render("jpan", 110), "百十");
        assert_eq!(render("jpan", 1000), "千");
        assert_eq!(render("jpan", 1989), "千九百八十九");
        assert_eq!(render("jpan", 10_000), "一万");
        assert_eq!(render("jpan", 10_005), "一万五");
        assert_eq!(render("jpan", 100_000_000), "一億");
    }

    #[test]
    fn chinese_han_numerals_keep_the_one_and_mark_interior_gaps() {
        assert_eq!(render("hans", 10), "十");
        assert_eq!(render("hans", 15), "十五");
        assert_eq!(render("hans", 105), "一百〇五");
        assert_eq!(render("hans", 110), "一百一十");
        assert_eq!(render("hans", 10_005), "一万〇五");
        assert_eq!(render("hans", 100_000_000), "一亿");
        assert_eq!(render("hant", 100_000_000), "一億");
        assert_eq!(render("hant", 12_000), "一萬二千");
    }

    #[test]
    fn financial_han_numerals_never_omit_the_one() {
        assert_eq!(render("jpanfin", 10), "壱拾");
        assert_eq!(render("jpanfin", 2024), "弐阡弐拾四");
        assert_eq!(render("jpanfin", 0), "零");
    }

    #[test]
    fn han_numerals_round_trip_over_a_wide_range() {
        for id in ["jpan", "jpanfin", "hans", "hant"] {
            let system = NumberingSystem::from_id(id).unwrap();
            for value in 0..=3000 {
                let text = system.format_integer(value).unwrap();
                assert_eq!(system.parse_integer(&text).unwrap(), value, "{id} {value}");
            }
            for value in [
                9_999i64,
                10_000,
                10_005,
                12_345,
                99_999,
                1_234_567,
                100_000_000,
                123_456_789,
                1_000_000_000_000,
                1_234_567_890_123,
                -2024,
            ] {
                let text = system.format_integer(value).unwrap();
                assert_eq!(system.parse_integer(&text).unwrap(), value, "{id} {value}");
            }
        }
    }

    #[test]
    fn han_numerals_refuse_magnitudes_they_have_no_unit_for() {
        let jpan = NumberingSystem::from_id("jpan").unwrap();
        assert_eq!(
            jpan.format_integer(9_999_999_999_999_999),
            Ok("九千九百九十九兆九千九百九十九億九千九百九十九万九千九百九十九".into())
        );
        assert_eq!(
            jpan.format_integer(10_000_000_000_000_000),
            Err(I18nError::NumberOutOfRange)
        );
    }

    #[test]
    fn a_han_parse_accepts_either_spelling_of_zero_and_rejects_junk() {
        assert_eq!(read("hans", "一百零五"), 105);
        assert_eq!(read("hans", "一百〇五"), 105);
        assert_eq!(read("jpan", "千九百八十九"), 1989);
        assert_eq!(read("jpan", "一千九百八十九"), 1989);
        let jpan = NumberingSystem::from_id("jpan").unwrap();
        assert_eq!(
            jpan.parse_integer("nineteen"),
            Err(I18nError::InvalidNumber)
        );
        assert_eq!(jpan.parse_integer(""), Err(I18nError::InvalidNumber));
    }

    #[test]
    fn the_first_year_of_a_japanese_era_is_written_gannen() {
        let mut text = String::new();
        write_japanese_era_year(1, &mut text).unwrap();
        assert_eq!(text, "元");
        text.clear();
        write_japanese_era_year(23, &mut text).unwrap();
        assert_eq!(text, "二十三");
        assert_eq!(parse_japanese_era_year("元").unwrap(), 1);
        assert_eq!(parse_japanese_era_year("二十三").unwrap(), 23);
        assert_eq!(
            write_japanese_era_year(0, &mut text),
            Err(I18nError::NumberOutOfRange)
        );
    }

    #[test]
    fn a_locale_picks_its_numbering_system_from_the_extension_first() {
        let locale = Locale::parse("ja-JP-u-nu-jpan").unwrap();
        assert_eq!(NumberingSystem::for_locale(&locale).id(), "jpan");
        let arabic = Locale::parse("ar-EG").unwrap();
        assert_eq!(NumberingSystem::for_locale(&arabic).id(), "arab");
        let persian = Locale::parse("fa-IR").unwrap();
        assert_eq!(NumberingSystem::for_locale(&persian).id(), "arabext");
        let english = Locale::parse("en-GB").unwrap();
        assert_eq!(NumberingSystem::for_locale(&english).id(), "latn");
        let nonsense = Locale::parse("en-u-nu-madeup").unwrap();
        assert_eq!(NumberingSystem::for_locale(&nonsense).id(), "latn");
    }

    #[test]
    fn digit_tables_are_exposed_only_for_positional_systems() {
        assert_eq!(
            NumberingSystem::from_id("thai").unwrap().digits(),
            Some(&THAI_DIGITS)
        );
        assert!(NumberingSystem::from_id("jpan").unwrap().digits().is_none());
        assert!(NumberingSystem::from_id("jpan").unwrap().is_algorithmic());
        assert!(!NumberingSystem::from_id("latn").unwrap().is_algorithmic());
    }
}

hc_core::catalogue_tests! {
    type: NumberingSystem,
    id: |system| system.id,
    sorted_by: |system| system.id,
    tests: system_table_tests,
    all: ALL,
    lookup: NumberingSystem::from_id,
}
