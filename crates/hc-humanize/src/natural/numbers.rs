//! The number functions of Python's `humanize` that are not about time:
//! `apnumber`, `fractional`, `scientific`, `metric`, `clamp`, `naturallist`,
//! `naturalsize`, and `intword` for integers of any length.
//!
//! The behaviour is that of `humanize` 4.16.0 as its documentation
//! (<https://humanize.readthedocs.io/en/latest/number/>) and its source
//! (`number.py`, `filesize.py`, `lists.py` on the project's `main`) show it,
//! read 2026-10-03. Python's floats are IEEE doubles and so are these:
//! every rounding is the one `%` formatting does, which is correct rounding
//! of the double's exact value, ties to even.

use core::fmt;
use core::fmt::Write as _;

use super::{DigitBuffer, Natural, NaturalPhrases, Plural, write_python_repr};
use crate::error::{HumanizeError, HumanizeResult};

/// Which suffixes and base `naturalsize` uses: `humanize`'s `binary` and
/// `gnu` flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeStyle {
    /// Powers of 1000: `3.0 MB`.
    Decimal,
    /// Powers of 1024: `2.9 KiB`.
    Binary,
    /// Powers of 1024 with one-letter suffixes and no space: `2.9K`, `300B`.
    Gnu,
}

/// How `clamp` writes the value: `humanize`'s `format`, which in Python is a
/// format string or a function.
#[derive(Clone, Copy)]
pub enum ClampFormat<'a> {
    /// `"{:}"`, Python's default: the value as `str` writes a float.
    Display,
    /// `"{:.Nf}"`.
    Fixed(u8),
    /// `"{:.N%}"`: the value times a hundred, then a percent sign.
    Percent(u8),
    /// A function, Python's `callable` form.
    With(&'a dyn Fn(&mut dyn fmt::Write, f64) -> fmt::Result),
}

impl fmt::Debug for ClampFormat<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Display => f.write_str("Display"),
            Self::Fixed(decimals) => f.debug_tuple("Fixed").field(decimals).finish(),
            Self::Percent(decimals) => f.debug_tuple("Percent").field(decimals).finish(),
            Self::With(_) => f.write_str("With(..)"),
        }
    }
}

/// Powers of ten as Python computes `10**k` and converts it to a float:
/// `float(10**k)` for `k >= 0`, `pow(10.0, k)` below.
fn pow10(exponent: i32) -> f64 {
    const POSITIVE: [f64; 34] = [
        1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16,
        1e17, 1e18, 1e19, 1e20, 1e21, 1e22, 1e23, 1e24, 1e25, 1e26, 1e27, 1e28, 1e29, 1e30, 1e31,
        1e32, 1e33,
    ];
    const NEGATIVE: [f64; 34] = [
        1e-0, 1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12, 1e-13,
        1e-14, 1e-15, 1e-16, 1e-17, 1e-18, 1e-19, 1e-20, 1e-21, 1e-22, 1e-23, 1e-24, 1e-25, 1e-26,
        1e-27, 1e-28, 1e-29, 1e-30, 1e-31, 1e-32, 1e-33,
    ];
    let table = if exponent >= 0 { &POSITIVE } else { &NEGATIVE };
    table
        .get(exponent.unsigned_abs() as usize)
        .copied()
        .unwrap_or(f64::NAN)
}

/// Python's `round(value, digits)` for the comparisons `metric` makes: the
/// double nearest to `value` correctly rounded to `digits` decimals.
fn round_to(value: f64, digits: usize) -> f64 {
    let mut buffer = DigitBuffer::default();
    if write!(buffer, "{value:.digits$}").is_err() {
        return value;
    }
    buffer.as_str().parse().unwrap_or(value)
}

/// `humanize`'s `_format_not_finite`.
fn write_not_finite<W: fmt::Write + ?Sized>(out: &mut W, value: f64) -> fmt::Result {
    if value.is_nan() {
        out.write_str("NaN")
    } else if value < 0.0 {
        out.write_str("-Inf")
    } else {
        out.write_str("+Inf")
    }
}

/// Superscript digits for `scientific`: `str.maketrans("0123456789-", ...)`.
fn write_superscript<W: fmt::Write + ?Sized>(out: &mut W, exponent: i32) -> fmt::Result {
    let mut buffer = DigitBuffer::default();
    write!(buffer, "{exponent}")?;
    for character in buffer.as_str().chars() {
        out.write_char(match character {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            '-' => '⁻',
            other => other,
        })?;
    }
    Ok(())
}

impl Natural {
    /// `apnumber(value)`: the Associated Press style, which spells out
    /// `zero` to `nine` and leaves the rest as digits.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_apnumber<W: fmt::Write + ?Sized>(
        &self,
        out: &mut W,
        value: i128,
    ) -> HumanizeResult<()> {
        let word = usize::try_from(value)
            .ok()
            .and_then(|index| self.phrases.apnumber.get(index));
        match word {
            Some(word) => out.write_str(word)?,
            None => write!(out, "{value}")?,
        }
        Ok(())
    }

    /// `apnumber(value)` for a float: `int(value)` first, so `3.7` is
    /// `three`; a value that is not finite is `NaN`, `+Inf` or `-Inf`.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_apnumber_f64<W: fmt::Write + ?Sized>(
        &self,
        out: &mut W,
        value: f64,
    ) -> HumanizeResult<()> {
        if !value.is_finite() {
            write_not_finite(out, value)?;
            return Ok(());
        }
        if value.abs() >= 1e30 {
            write!(out, "{value:.0}")?;
            return Ok(());
        }
        self.write_apnumber(out, hc_core::math::trunc(value) as i128)
    }

    /// `fractional(value)`: `0.3` is `3/10`, `1.3` is `1 3/10`, `1` is `1`.
    ///
    /// The fraction part is the nearest fraction with a denominator of at
    /// most 1000 (`Fraction.limit_denominator(1000)`); a negative number
    /// keeps its sign on the whole part, or on the numerator when there is
    /// none. A value that is not finite is `NaN`, `+Inf` or `-Inf`.
    ///
    /// As in `humanize` 4.16.0, a fraction that reduces to a whole number is
    /// folded into the whole part only when that part is not zero and the
    /// fraction is exactly zero: `0.0` is `0/1`, `0.9999` is `1/1` and
    /// `2.9999999` is `2 1/1`. The project's later, unreleased code writes
    /// `0`, `1` and `3`.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_fractional<W: fmt::Write + ?Sized>(
        &self,
        out: &mut W,
        value: f64,
    ) -> HumanizeResult<()> {
        if !value.is_finite() {
            write_not_finite(out, value)?;
            return Ok(());
        }
        // `int(number)`: beyond 2^63 a double has no fraction, and the whole
        // part is written as the float Python converts the integer back to.
        if value.abs() >= 9.0e18 {
            write!(out, "{:.0}", hc_core::math::trunc(value))?;
            return Ok(());
        }
        let whole = hc_core::math::trunc(value) as i128;
        let (numerator, denominator) = limit_denominator(value - whole as f64, 1000);
        if whole != 0 && numerator == 0 && denominator == 1 {
            // An integer, or a number so near one that no fraction is nearer.
            write!(out, "{whole}")?;
        } else if whole == 0 {
            write!(out, "{numerator}/{denominator}")?;
        } else {
            write!(out, "{whole} {}/{denominator}", numerator.abs())?;
        }
        Ok(())
    }

    /// `scientific(value, precision=2)`: `3.00 x 10⁻¹`, `-1.00 x 10³`.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_scientific<W: fmt::Write + ?Sized>(
        &self,
        out: &mut W,
        value: f64,
        precision: u8,
    ) -> HumanizeResult<()> {
        if !value.is_finite() {
            write_not_finite(out, value)?;
            return Ok(());
        }
        let mut buffer = DigitBuffer::default();
        let digits = usize::from(precision);
        write!(buffer, "{value:.digits$e}")?;
        let text = buffer.as_str();
        let (mantissa, exponent) = text.split_once('e').unwrap_or((text, "0"));
        out.write_str(mantissa)?;
        out.write_str(" x 10")?;
        write_superscript(out, exponent.parse().unwrap_or(0))?;
        Ok(())
    }

    /// `metric(value, unit, precision=3)`: `1.50 kV`, `200 MW`, `220 μF`.
    ///
    /// `precision` counts significant digits. A magnitude of 10³³ or more,
    /// or below 10⁻³⁰, has no prefix and is written as [`scientific`] with
    /// one digit fewer, followed by the unit.
    ///
    /// [`scientific`]: Natural::write_scientific
    ///
    /// # Errors
    ///
    /// [`HumanizeError::Unsupported`] for a precision of zero on a magnitude
    /// that falls back to scientific notation — Python's `ValueError` — and
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_metric<W: fmt::Write + ?Sized>(
        &self,
        out: &mut W,
        value: f64,
        unit: &str,
        precision: u8,
    ) -> HumanizeResult<()> {
        if !value.is_finite() {
            write_not_finite(out, value)?;
            return Ok(());
        }
        let mut exponent = if value == 0.0 {
            0
        } else {
            hc_core::math::floor(hc_core::math::log10(value.abs())) as i32
        };
        if !(-30..33).contains(&exponent) {
            let digits = precision
                .checked_sub(1)
                .ok_or(HumanizeError::Unsupported("a precision of zero"))?;
            self.write_scientific(out, value, digits)?;
            out.write_str(unit)?;
            return Ok(());
        }
        let precision = i32::from(precision);
        let old_bucket = exponent.div_euclid(3) * 3;
        let mut scaled = value / pow10(old_bucket);
        let mut digits = (precision - exponent.rem_euclid(3) - 1).max(0);
        // `humanize` 4.16.0 carries a rounded value of 1000 into the next
        // bucket whole (its unreleased code carries 9.999 → 10.0 too).
        if exponent < 30 && round_to(scaled.abs(), digits as usize) >= 1000.0 {
            exponent += 3 - exponent.rem_euclid(3);
            let new_bucket = exponent.div_euclid(3) * 3;
            scaled /= pow10(new_bucket - old_bucket);
            digits = (precision - exponent.rem_euclid(3) - 1).max(0);
        }
        let prefix = if exponent >= 3 {
            "kMGTPEZYRQ"
                .chars()
                .nth((exponent / 3 - 1) as usize)
                .unwrap_or(' ')
        } else if exponent < 0 {
            "mμnpfazyrq"
                .chars()
                .nth(((-exponent - 1) / 3) as usize)
                .unwrap_or(' ')
        } else {
            '\0'
        };
        let has_prefix = prefix != '\0';
        write!(out, "{scaled:.digits$}", digits = digits as usize)?;
        if !(unit.is_empty() && !has_prefix) && !matches!(unit, "°" | "′" | "″") {
            out.write_char(' ')?;
        }
        if has_prefix {
            out.write_char(prefix)?;
        }
        out.write_str(unit)?;
        Ok(())
    }

    /// `clamp(value, format, floor, ceil, floor_token, ceil_token)`: the
    /// value written with `format`, or, outside `[floor, ceil]`, the bound
    /// written the same way after a token: `<0.01`, `>99%`.
    ///
    /// A value that is not finite is `NaN`, `+Inf` or `-Inf`. `floor` is
    /// tested before `ceil`, as in Python.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_clamp<W: fmt::Write>(
        &self,
        out: &mut W,
        value: f64,
        format: ClampFormat<'_>,
        (floor, ceil): (Option<f64>, Option<f64>),
        (floor_token, ceil_token): (&str, &str),
    ) -> HumanizeResult<()> {
        if !value.is_finite() {
            write_not_finite(out, value)?;
            return Ok(());
        }
        let (value, token) = match (floor, ceil) {
            (Some(floor), _) if value < floor => (floor, floor_token),
            (_, Some(ceil)) if value > ceil => (ceil, ceil_token),
            _ => (value, ""),
        };
        out.write_str(token)?;
        match format {
            ClampFormat::Display => write_python_repr(out, value)?,
            ClampFormat::Fixed(decimals) => {
                write!(
                    out,
                    "{value:.precision$}",
                    precision = usize::from(decimals)
                )?;
            }
            ClampFormat::Percent(decimals) => {
                write!(
                    out,
                    "{:.precision$}%",
                    value * 100.0,
                    precision = usize::from(decimals)
                )?;
            }
            ClampFormat::With(function) => function(out, value)?,
        }
        Ok(())
    }

    /// `naturallist(items)`: `one`, `one and two`, `one, two and three`.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_naturallist<W: fmt::Write + ?Sized, T: fmt::Display>(
        &self,
        out: &mut W,
        items: &[T],
    ) -> HumanizeResult<()> {
        let Some((last, rest)) = items.split_last() else {
            return Ok(());
        };
        // `natural_list` is not translated: its `, ` and ` and ` are
        // literals in `lists.py`.
        for (index, item) in rest.iter().enumerate() {
            if index > 0 {
                out.write_str(", ")?;
            }
            write!(out, "{item}")?;
        }
        if !rest.is_empty() {
            out.write_str(" and ")?;
        }
        write!(out, "{last}")?;
        Ok(())
    }

    /// `naturalsize(value, binary, gnu, format="%.{decimals}f")`: `3.0 MB`,
    /// `2.9 KiB`, `300B`.
    ///
    /// A size of one byte is `1 Byte` and below the base is `N Bytes` (`NB`
    /// in the GNU style). The unit is the largest whose value, once rounded
    /// with the format, is below the base.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::Unsupported`] for `NaN`, which Python's `int` refuses,
    /// and [`HumanizeError::WriteFailed`] when the sink refuses.
    pub fn write_naturalsize<W: fmt::Write + ?Sized>(
        &self,
        out: &mut W,
        value: f64,
        style: SizeStyle,
        decimals: u8,
    ) -> HumanizeResult<()> {
        if value.is_nan() {
            return Err(HumanizeError::Unsupported("a size that is not a number"));
        }
        let gnu = style == SizeStyle::Gnu;
        let base: f64 = if style == SizeStyle::Decimal {
            1000.0
        } else {
            1024.0
        };
        let suffixes = match style {
            SizeStyle::Decimal => &self.phrases.size_decimal,
            SizeStyle::Binary => &self.phrases.size_binary,
            SizeStyle::Gnu => &self.phrases.size_gnu,
        };
        let magnitude = value.abs();
        let whole = hc_core::math::trunc(value) as i128;
        let precision = usize::from(decimals);
        if magnitude == 1.0 && !gnu {
            return super::substitute(out, self.phrases.byte, |out| write!(out, "{whole}"));
        }
        if magnitude < base {
            return if gnu {
                write!(out, "{whole}B").map_err(Into::into)
            } else {
                super::substitute(out, self.phrases.bytes, |out| write!(out, "{whole}"))
            };
        }
        // `log(abs_bytes, base)` is `log(abs_bytes) / log(base)`, which for
        // an exact power of the base can fall just below the integer; the
        // check against the rounded value below puts that right.
        let ratio = hc_core::math::ln(magnitude) / hc_core::math::ln(base);
        let mut exponent = ratio.min(suffixes.len() as f64) as usize;
        let power = |exponent: usize| -> f64 {
            if style == SizeStyle::Decimal {
                pow10(3 * exponent as i32)
            } else {
                hc_core::math::powf(1024.0, exponent as f64)
            }
        };
        if exponent < suffixes.len() {
            let mut buffer = DigitBuffer::default();
            write!(buffer, "{:.precision$}", magnitude / power(exponent))?;
            let rounded: f64 = buffer.as_str().parse().unwrap_or(0.0);
            if rounded.abs() >= base {
                exponent += 1;
            }
        }
        let suffix = suffixes.get(exponent.max(1) - 1).copied().unwrap_or("");
        write!(out, "{:.precision$}", value / power(exponent))?;
        if !gnu {
            out.write_char(' ')?;
        }
        out.write_str(suffix)?;
        Ok(())
    }

    /// `intword(value, format="%.{decimals}f")` for an integer written in
    /// decimal digits, of any length: `12.4 thousand`, `1.0 googol`.
    ///
    /// `digits` is an optional `-` or `+` and ASCII digits; leading zeros
    /// are ignored. `humanize` goes up to the googol (10¹⁰⁰), which no
    /// `i128` holds, and divides with Python's integer true division, which
    /// is correctly rounded; so is this.
    ///
    /// # Errors
    ///
    /// [`HumanizeError::Unsupported`] when `digits` is not an integer,
    /// [`HumanizeError::Overflow`] when the integer is beyond the largest
    /// double, about 1.8 × 10³⁰⁸ — Python's `OverflowError` from
    /// `float(value)` — and [`HumanizeError::WriteFailed`] when
    /// the sink refuses.
    pub fn write_intword_digits<W: fmt::Write + ?Sized>(
        &self,
        out: &mut W,
        digits: &str,
        decimals: u8,
    ) -> HumanizeResult<()> {
        let (negative, unsigned) = match digits.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, digits.strip_prefix('+').unwrap_or(digits)),
        };
        if unsigned.is_empty() || !unsigned.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(HumanizeError::Unsupported("an integer written in digits"));
        }
        let unsigned = unsigned.trim_start_matches('0');
        // Python's first step is `float(value)`, which raises `OverflowError`
        // for an integer beyond the largest double (about 1.8 × 10³⁰⁸).
        if unsigned.len() > 309
            || (unsigned.len() == 309 && unsigned.parse::<f64>().is_ok_and(f64::is_infinite))
        {
            return Err(HumanizeError::Overflow);
        }
        if unsigned.len() <= 3 {
            // Below a thousand: the number as it is (`-0` is `0`).
            if unsigned.is_empty() {
                out.write_char('0')?;
            } else {
                if negative {
                    out.write_char('-')?;
                }
                out.write_str(unsigned)?;
            }
            return Ok(());
        }
        // `bisect_right(powers, value)`: the powers are 10^3, 10^6, ... 10^33
        // and 10^100.
        let exponents = NaturalPhrases::POWER_EXPONENTS;
        let passed = exponents
            .iter()
            .filter(|exponent| unsigned.len() > usize::from(**exponent))
            .count();
        let largest = passed == exponents.len();
        let mut index = passed - 1;
        let exponent = |index: usize| usize::from(exponents.get(index).copied().unwrap_or(0));
        let precision = usize::from(decimals);
        let chopped = chop(unsigned, exponent(index))?;
        let mut buffer = DigitBuffer::default();
        write!(buffer, "{chopped:.precision$}")?;
        let mut rounded: f64 = buffer.as_str().parse().unwrap_or(chopped);
        // `rounded_value * power == powers[ordinal + 1]`, as 4.16.0 writes it:
        // a float times a power, compared with an exact integer. Both sides
        // are the same number only while the next power is one a double holds
        // exactly, 10²² or less, so a value of 10²⁴ − 1 stays `1000.0
        // sextillion` there. (The unreleased code compares the integer ratio
        // of the two powers and carries it.)
        if !largest {
            let next = exponent(index + 1);
            if next <= 22 && rounded * pow10(exponent(index) as i32) == pow10(next as i32) {
                index += 1;
                rounded = 1.0;
            }
        }
        let pair = self
            .phrases
            .powers
            .get(index)
            .copied()
            .unwrap_or(Plural::english("", ""));
        // `ngettext` of `math.ceil(rounded_value)`, an integer of any size.
        let mut count = DigitBuffer::default();
        write!(count, "{:.0}", hc_core::math::ceil(rounded))?;
        let name = self.pick_digits(pair, count.as_str());
        if negative {
            out.write_char('-')?;
        }
        // `(format % rounded_value).replace(".", decimal_sep)`.
        let mut number = DigitBuffer::default();
        write!(number, "{rounded:.precision$}")?;
        match number.as_str().split_once('.') {
            Some((whole, fraction)) => {
                write!(out, "{whole}{}{fraction}", self.phrases.grouping.decimal)?;
            }
            None => out.write_str(number.as_str())?,
        }
        write!(out, " {name}")?;
        Ok(())
    }

    /// `intword(value, format)` for a float: `int(value)` first, as Python
    /// does, so `1.5e30` is the integer it exactly is.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_intword_digits`]; a value that is not finite is
    /// `NaN`, `+Inf` or `-Inf`.
    pub fn write_intword_f64<W: fmt::Write + ?Sized>(
        &self,
        out: &mut W,
        value: f64,
        decimals: u8,
    ) -> HumanizeResult<()> {
        if !value.is_finite() {
            write_not_finite(out, value)?;
            return Ok(());
        }
        let mut buffer = DigitBuffer::default();
        write!(buffer, "{:.0}", hc_core::math::trunc(value))?;
        self.write_intword_digits(out, buffer.as_str(), decimals)
    }
}

/// `value / 10**exponent` for an integer written in digits, correctly
/// rounded to a double: Python's `int / int`.
fn chop(digits: &str, exponent: usize) -> HumanizeResult<f64> {
    let mut buffer = DigitBuffer::default();
    write!(buffer, "{digits}e-{exponent}").map_err(|_| HumanizeError::Overflow)?;
    let quotient: f64 = buffer
        .as_str()
        .parse()
        .map_err(|_| HumanizeError::Unsupported("an integer written in digits"))?;
    if quotient.is_finite() {
        Ok(quotient)
    } else {
        Err(HumanizeError::Overflow)
    }
}

/// `Fraction(x).limit_denominator(max_denominator)` for a double `x` with
/// `|x| < 1`, as a numerator and a denominator, by the algorithm of Python's
/// `fractions` module (continued fractions with floor division).
///
/// `x` is `m / 2^k` exactly; a value below 2⁻⁶⁰ is nearer 0 than to the
/// nearest fraction a denominator of 1000 allows, so it is 0 without the
/// big integers its denominator would need.
fn limit_denominator(x: f64, max_denominator: i128) -> (i128, i128) {
    if x == 0.0 || x.abs() < 1e-18 {
        return (0, 1);
    }
    let bits = x.abs().to_bits();
    let exponent = i32::try_from((bits >> 52) & 0x7ff).unwrap_or(0);
    let fraction = i128::from(bits & ((1 << 52) - 1));
    // `|x| = mantissa / 2^shift`.
    let (mut mantissa, mut shift) = if exponent == 0 {
        (fraction, 1074)
    } else {
        (fraction | (1 << 52), 1075 - exponent)
    };
    while shift > 0 && mantissa % 2 == 0 {
        mantissa /= 2;
        shift -= 1;
    }
    let numerator = if x < 0.0 { -mantissa } else { mantissa };
    let denominator = 1i128 << shift;
    if denominator <= max_denominator {
        return (numerator, denominator);
    }
    let (mut p0, mut q0, mut p1, mut q1) = (0i128, 1i128, 1i128, 0i128);
    let (mut n, mut d) = (numerator, denominator);
    loop {
        let a = n.div_euclid(d);
        let q2 = q0 + a * q1;
        if q2 > max_denominator {
            break;
        }
        (p0, q0, p1, q1) = (p1, q1, p0 + a * p1, q2);
        (n, d) = (d, n - a * d);
    }
    let k = (max_denominator - q0) / q1;
    // The candidate (p0 + k p1) / (q0 + k q1) against p1 / q1: whichever is
    // nearer to x, the second on a tie.
    if 2 * d * (q0 + k * q1) <= denominator {
        (p1, q1)
    } else {
        (p0 + k * p1, q0 + k * q1)
    }
}

#[cfg(feature = "alloc")]
impl Natural {
    /// [`Natural::write_apnumber`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_apnumber`].
    pub fn apnumber(&self, value: i128) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_apnumber(out, value))
    }

    /// [`Natural::write_apnumber_f64`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_apnumber_f64`].
    pub fn apnumber_f64(&self, value: f64) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_apnumber_f64(out, value))
    }

    /// [`Natural::write_fractional`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_fractional`].
    pub fn fractional(&self, value: f64) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_fractional(out, value))
    }

    /// [`Natural::write_scientific`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_scientific`].
    pub fn scientific(&self, value: f64, precision: u8) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_scientific(out, value, precision))
    }

    /// [`Natural::write_metric`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_metric`].
    pub fn metric(
        &self,
        value: f64,
        unit: &str,
        precision: u8,
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_metric(out, value, unit, precision))
    }

    /// [`Natural::write_clamp`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_clamp`].
    pub fn clamp(
        &self,
        value: f64,
        format: ClampFormat<'_>,
        bounds: (Option<f64>, Option<f64>),
        tokens: (&str, &str),
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_clamp(out, value, format, bounds, tokens))
    }

    /// [`Natural::write_naturallist`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_naturallist`].
    pub fn naturallist<T: fmt::Display>(
        &self,
        items: &[T],
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_naturallist(out, items))
    }

    /// [`Natural::write_naturalsize`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_naturalsize`].
    pub fn naturalsize(
        &self,
        value: f64,
        style: SizeStyle,
        decimals: u8,
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_naturalsize(out, value, style, decimals))
    }

    /// [`Natural::write_intword_digits`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_intword_digits`].
    pub fn intword_digits(
        &self,
        digits: &str,
        decimals: u8,
    ) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_intword_digits(out, digits, decimals))
    }

    /// [`Natural::write_intword_f64`] into a new string.
    ///
    /// # Errors
    ///
    /// As [`Natural::write_intword_f64`].
    pub fn intword_f64(&self, value: f64, decimals: u8) -> HumanizeResult<alloc::string::String> {
        Self::collect(|out| self.write_intword_f64(out, value, decimals))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::natural::Grouping;
    use alloc::string::String;

    fn natural() -> Natural {
        Natural::english()
    }

    /// The examples of `humanize` 4.16's documentation of `apnumber`,
    /// `fractional`, `scientific`, `metric`, `clamp`, `intword`
    /// (<https://humanize.readthedocs.io/en/latest/number/>, read
    /// 2026-10-03), `naturalsize` and `natural_list` (their docstrings in
    /// `filesize.py` and `lists.py`).
    #[test]
    fn the_documented_examples_hold() {
        let n = natural();
        assert_eq!(n.apnumber(0).unwrap(), "zero");
        assert_eq!(n.apnumber(5).unwrap(), "five");
        assert_eq!(n.apnumber(10).unwrap(), "10");
        assert_eq!(n.fractional(0.3).unwrap(), "3/10");
        assert_eq!(n.fractional(1.3).unwrap(), "1 3/10");
        assert_eq!(n.fractional(1.0 / 3.0).unwrap(), "1/3");
        assert_eq!(n.fractional(1.0).unwrap(), "1");
        assert_eq!(n.scientific(0.3, 2).unwrap(), "3.00 x 10⁻¹");
        assert_eq!(n.scientific(500.0, 2).unwrap(), "5.00 x 10²");
        assert_eq!(n.scientific(-1000.0, 2).unwrap(), "-1.00 x 10³");
        assert_eq!(n.scientific(1000.0, 1).unwrap(), "1.0 x 10³");
        assert_eq!(n.scientific(1000.0, 3).unwrap(), "1.000 x 10³");
        assert_eq!(n.scientific(99.0, 2).unwrap(), "9.90 x 10¹");
        assert_eq!(n.metric(1500.0, "V", 3).unwrap(), "1.50 kV");
        assert_eq!(n.metric(2e8, "W", 3).unwrap(), "200 MW");
        assert_eq!(n.metric(220e-6, "F", 3).unwrap(), "220 μF");
        assert_eq!(n.metric(1e-14, "", 4).unwrap(), "10.00 f");
        assert_eq!(n.metric(1e40, "", 3).unwrap(), "1.00 x 10⁴⁰");
        let none = (None, None);
        let tokens = ("<", ">");
        assert_eq!(
            n.clamp(123.456, ClampFormat::Display, none, tokens)
                .unwrap(),
            "123.456"
        );
        assert_eq!(
            n.clamp(0.0001, ClampFormat::Display, (Some(0.01), None), tokens)
                .unwrap(),
            "<0.01"
        );
        assert_eq!(
            n.clamp(0.99, ClampFormat::Percent(0), (None, Some(0.99)), tokens)
                .unwrap(),
            "99%"
        );
        assert_eq!(
            n.clamp(0.999, ClampFormat::Percent(0), (None, Some(0.99)), tokens)
                .unwrap(),
            ">99%"
        );
        let intword = |out: &mut dyn fmt::Write, value: f64| {
            Natural::english()
                .write_intword_f64(out, value, 1)
                .map_err(|_| fmt::Error)
        };
        assert_eq!(
            n.clamp(
                1.0,
                ClampFormat::With(&intword),
                (Some(1e6), None),
                ("under ", ">")
            )
            .unwrap(),
            "under 1.0 million"
        );
        assert_eq!(n.intword(100, 1).unwrap(), "100");
        assert_eq!(n.intword(12_400, 1).unwrap(), "12.4 thousand");
        assert_eq!(n.intword(1_000_000, 1).unwrap(), "1.0 million");
        assert_eq!(n.intword(1_200_000_000, 1).unwrap(), "1.2 billion");
        assert_eq!(
            n.intword(8_100_000_000_000_000_000_000_000_000_000_000, 1)
                .unwrap(),
            "8.1 decillion"
        );
        assert_eq!(n.intword(1_234_000, 3).unwrap(), "1.234 million");
        assert_eq!(
            n.naturallist(&["one", "two", "three"]).unwrap(),
            "one, two and three"
        );
        assert_eq!(n.naturallist(&["one", "two"]).unwrap(), "one and two");
        assert_eq!(n.naturallist(&["one"]).unwrap(), "one");
        assert_eq!(n.naturallist::<&str>(&[]).unwrap(), "");
        assert_eq!(
            n.naturalsize(3_000_000.0, SizeStyle::Decimal, 1).unwrap(),
            "3.0 MB"
        );
        assert_eq!(n.naturalsize(300.0, SizeStyle::Gnu, 1).unwrap(), "300B");
        assert_eq!(n.naturalsize(3000.0, SizeStyle::Gnu, 1).unwrap(), "2.9K");
        assert_eq!(
            n.naturalsize(3000.0, SizeStyle::Binary, 1).unwrap(),
            "2.9 KiB"
        );
    }

    /// Values from `humanize` 4.16's source (`number.py`, `filesize.py`),
    /// worked through by a transcription of it: the cases where its rounding
    /// moves a value into the next unit.
    #[test]
    fn rounding_carries_into_the_next_unit_as_humanize_does() {
        let n = natural();
        // `log(1e6, 1000)` is 1.9999999999999998, so the exponent starts one
        // too low and the rounded value 1000.0 is moved up.
        assert_eq!(n.naturalsize(1e6, SizeStyle::Decimal, 1).unwrap(), "1.0 MB");
        assert_eq!(
            n.naturalsize(999_950.0, SizeStyle::Decimal, 1).unwrap(),
            "1.0 MB"
        );
        assert_eq!(n.naturalsize(1.0, SizeStyle::Decimal, 1).unwrap(), "1 Byte");
        assert_eq!(
            n.naturalsize(-1.0, SizeStyle::Decimal, 1).unwrap(),
            "-1 Byte"
        );
        assert_eq!(
            n.naturalsize(0.0, SizeStyle::Decimal, 1).unwrap(),
            "0 Bytes"
        );
        assert_eq!(
            n.naturalsize(999.0, SizeStyle::Decimal, 1).unwrap(),
            "999 Bytes"
        );
        assert_eq!(n.naturalsize(1023.0, SizeStyle::Gnu, 1).unwrap(), "1023B");
        assert_eq!(n.naturalsize(1024.0, SizeStyle::Gnu, 1).unwrap(), "1.0K");
        assert_eq!(
            n.naturalsize(1536.0, SizeStyle::Binary, 1).unwrap(),
            "1.5 KiB"
        );
        assert_eq!(
            n.naturalsize(-3000.0, SizeStyle::Decimal, 1).unwrap(),
            "-3.0 kB"
        );
        // Beyond the last suffix the value just grows.
        assert_eq!(
            n.naturalsize(1e33, SizeStyle::Decimal, 1).unwrap(),
            "1000.0 QB"
        );
        assert_eq!(
            n.naturalsize(f64::INFINITY, SizeStyle::Decimal, 1).unwrap(),
            "inf QB"
        );
        assert!(n.naturalsize(f64::NAN, SizeStyle::Decimal, 1).is_err());
        assert_eq!(n.metric(999.9999, "V", 3).unwrap(), "1.00 kV");
        // 4.16.0 carries a rounded 1000 only, so 9.9999 reads 10.00 here
        // where the unreleased code writes 10.0.
        assert_eq!(n.metric(9.9999, "V", 3).unwrap(), "10.00 V");
        assert_eq!(n.metric(99.999, "V", 3).unwrap(), "100.0 V");
        assert_eq!(n.metric(0.000_999_9, "V", 3).unwrap(), "1.00 mV");
        assert_eq!(n.metric(-1500.0, "V", 3).unwrap(), "-1.50 kV");
        assert_eq!(n.metric(0.0, "V", 3).unwrap(), "0.00 V");
        assert_eq!(n.metric(12_345.678, "V", 3).unwrap(), "12.3 kV");
        assert_eq!(n.metric(1e32, "V", 3).unwrap(), "100 QV");
        assert_eq!(n.metric(9.9999e32, "V", 3).unwrap(), "1000 QV");
        assert_eq!(n.metric(1e33, "V", 3).unwrap(), "1.00 x 10³³V");
        assert_eq!(n.metric(1e-30, "V", 3).unwrap(), "1.00 qV");
        assert_eq!(n.metric(1e-31, "V", 3).unwrap(), "1.00 x 10⁻³¹V");
        // No space before a degree, minute or second sign, nor with neither
        // prefix nor unit.
        assert_eq!(n.metric(45.0, "°", 3).unwrap(), "45.0°");
        assert_eq!(n.metric(1.0, "", 3).unwrap(), "1.00");
        assert_eq!(n.metric(1500.0, "", 3).unwrap(), "1.50 k");
        assert!(n.metric(1e40, "", 0).is_err());
        assert_eq!(n.scientific(0.0, 2).unwrap(), "0.00 x 10⁰");
        assert_eq!(n.scientific(1e-100, 2).unwrap(), "1.00 x 10⁻¹⁰⁰");
        assert_eq!(n.scientific(f64::NAN, 2).unwrap(), "NaN");
        assert_eq!(n.scientific(f64::NEG_INFINITY, 2).unwrap(), "-Inf");
    }

    /// `fractional` as `humanize` 4.16.0 (`number.py`, tag `4.16.0`) writes
    /// it: the whole part is `int(number)`, the fraction `Fraction(number -
    /// whole).limit_denominator(1000)`; the whole part is written alone only
    /// when it is not zero and the fraction is exactly 0/1, so the cases a
    /// later, unreleased version folds (`0.9999`, `2.9999999`, `0.0`) read
    /// `1/1`, `2 1/1` and `0/1`. The values are the transcription's.
    #[test]
    fn fractional_keeps_the_sign_and_the_nearest_small_fraction() {
        let n = natural();
        assert_eq!(n.fractional(-0.5).unwrap(), "-1/2");
        assert_eq!(n.fractional(-1.3).unwrap(), "-1 3/10");
        assert_eq!(n.fractional(2.0001).unwrap(), "2");
        assert_eq!(n.fractional(1.0).unwrap(), "1");
        assert_eq!(n.fractional(-3.0).unwrap(), "-3");
        assert_eq!(n.fractional(5.0 + 1.0 / 3.0).unwrap(), "5 1/3");
        assert_eq!(n.fractional(0.001).unwrap(), "1/1000");
        assert_eq!(n.fractional(123_456.5).unwrap(), "123456 1/2");
        assert_eq!(n.fractional(0.9999).unwrap(), "1/1");
        assert_eq!(n.fractional(-0.9999).unwrap(), "-1/1");
        assert_eq!(n.fractional(2.999_999_9).unwrap(), "2 1/1");
        assert_eq!(n.fractional(-2.999_999_9).unwrap(), "-2 1/1");
        assert_eq!(n.fractional(0.0).unwrap(), "0/1");
        assert_eq!(n.fractional(1e-30).unwrap(), "0/1");
        assert_eq!(n.fractional(f64::INFINITY).unwrap(), "+Inf");
        assert_eq!(n.fractional(f64::NAN).unwrap(), "NaN");
    }

    #[test]
    fn apnumber_truncates_a_float_and_spells_only_zero_to_nine() {
        let n = natural();
        assert_eq!(n.apnumber_f64(3.7).unwrap(), "three");
        assert_eq!(n.apnumber_f64(9.99).unwrap(), "nine");
        assert_eq!(n.apnumber(-1).unwrap(), "-1");
        assert_eq!(n.apnumber_f64(f64::INFINITY).unwrap(), "+Inf");
    }

    /// `humanize` 4.16 lists the googol as its last power; `int(value)` of a
    /// float or a Python integer of any length reaches it.
    #[test]
    fn intword_reaches_the_googol_and_refuses_what_a_double_cannot_hold() {
        let n = natural();
        let googol = |digits: &str| n.intword_digits(digits, 1).unwrap();
        let mut ten_to_100 = String::from("1");
        ten_to_100.push_str(&"0".repeat(100));
        assert_eq!(googol(&ten_to_100), "1.0 googol");
        // One below the googol stays in decillions: the promotion test
        // compares with `10**67`, which no double equals.
        let nines = "9".repeat(100);
        assert_eq!(
            googol(&nines),
            "9999999999999999827367757839185598317239782875580932278577147150336.0 decillion"
        );
        assert_eq!(
            googol(&format!("{}000000000000000000000000000000", 999_949)),
            "999.9 decillion"
        );
        assert_eq!(
            googol(&format!("{}000000000000000000000000000000", 999_950)),
            "1000.0 decillion"
        );
        // `rounded_value * power == powers[ordinal + 1]` compares a float
        // with an exact integer, which are equal only while the next power is
        // one a double holds: the carry is skipped from 10²⁴ on, where the
        // unreleased code writes `1.0 septillion`.
        assert_eq!(googol(&"9".repeat(24)), "1000.0 sextillion");
        assert_eq!(googol(&"9".repeat(21)), "1.0 sextillion");
        assert_eq!(googol("-999999"), "-1.0 million");
        assert_eq!(googol("999999"), "1.0 million");
        assert_eq!(googol("-1000"), "-1.0 thousand");
        assert_eq!(googol("-0"), "0");
        assert_eq!(googol("0012"), "12");
        assert_eq!(n.intword_f64(1.5e30, 2).unwrap(), "1.50 nonillion");
        assert!(matches!(
            n.intword_digits(&"9".repeat(400), 1),
            Err(HumanizeError::Overflow)
        ));
        assert!(matches!(
            n.intword_digits("12x", 1),
            Err(HumanizeError::Unsupported(_))
        ));
    }

    /// `repr(float)` chooses the digit nearest the double, and on an exact
    /// tie the even one: `1000000000000000.25` is `.2`, not `.3`.
    #[test]
    fn the_float_repr_takes_the_even_digit_on_a_tie() {
        let n = natural();
        let g = Grouping {
            thousands: "",
            decimal: ".",
        };
        assert_eq!(
            n.intcomma_f64(1e15 + 0.25, None, g).unwrap(),
            "1000000000000000.2"
        );
        assert_eq!(n.intcomma_f64(1e16, None, g).unwrap(), "1e+16");
        assert_eq!(n.intcomma_f64(0.0001, None, g).unwrap(), "0.0001");
        assert_eq!(n.intcomma_f64(0.00001, None, g).unwrap(), "1e-05");
    }
}
