//! Significant figures: how many digits of a number are actually claimed.
//!
//! "The universe is 13.8 billion years old" is a statement with three
//! significant figures. Storing it as `13_800_000_000.0` and printing it back
//! silently upgrades it to eleven, which is a claim no cosmologist made. This
//! module keeps the digit count alongside the value, propagates it through
//! arithmetic using the ordinary laboratory rules, and renders the number so
//! that what is printed is exactly what is known.
//!
//! # What the type means
//!
//! A `Significant` is a number reported to `figures` significant figures: the
//! value, rounded to that many digits by the rules of NIST SP 811, B.7.1
//! (a discarded part below one half goes down, above one half goes up, and
//! exactly one half goes to the even digit). The rounding works on the
//! shortest decimal numeral that identifies the `f64` (what a person typed
//! as `2.675`), not on its binary expansion, because "exactly one half" is a
//! statement about decimal digits. [`Significant::rounded`], the printed form
//! and the exponent and last place reported by
//! [`Significant::decimal_exponent`] and
//! [`Significant::last_significant_place`] all read the same rounded
//! numeral, so they cannot disagree: `9.96` to two figures is `10` (printed
//! `10`, exponent 1, last place 0), not `9.96` and not `10.0`.
//!
//! [`MAX_FIGURES`] marks an exact value: nothing is rounded and it prints as
//! the shortest numeral that reads back as the same `f64`, with no padding
//! (`0.1`, not `0.10000000000000001`).
//!
//! # The rules, and what they are worth
//!
//! * Multiplication and division: the result carries the smaller of the two
//!   operands' figure counts.
//! * Addition and subtraction: the result is significant down to the coarser
//!   of the two operands' last significant decimal places, and the figure
//!   count follows from that place and the magnitude of the answer.
//!
//! These are the rules taught for hand computation, and they are a heuristic,
//! not a probability calculus: they systematically over- and under-state the
//! true error in different regimes, and they say nothing about correlated
//! inputs. When the question is "how wrong might this be", use
//! [`crate::quantity::Uncertain`], which propagates an actual variance. This
//! type answers a narrower question: "how many digits am I entitled to show".

use core::fmt;

use hc_core::math;

use crate::error::{UncertaintyError, UncertaintyResult};

/// The most decimal digits an IEEE-754 binary double can distinguish.
///
/// A `f64` has a 53-bit significand, so 2^53 ≈ 9.007·10^15: 15 digits always
/// round-trip and 17 always suffice to identify the value. Claiming more than
/// 17 would be claiming precision the representation does not carry.
pub const MAX_FIGURES: u8 = 17;

/// A real number together with the number of digits that are actually known.
///
/// ```
/// use hc_uncertainty::Significant;
///
/// let age = Significant::new(13.8e9, 3).unwrap();
/// assert_eq!(age.to_string(), "1.38e10");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Significant {
    value: f64,
    figures: u8,
}

impl Significant {
    /// Attach a figure count to a value.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] for NaN or infinity and
    /// [`UncertaintyError::InvalidSignificantFigures`] when `figures` is zero
    /// or above [`MAX_FIGURES`].
    pub fn new(value: f64, figures: u8) -> UncertaintyResult<Self> {
        if !value.is_finite() {
            return Err(UncertaintyError::NotFinite);
        }
        if figures == 0 || figures > MAX_FIGURES {
            return Err(UncertaintyError::InvalidSignificantFigures);
        }
        Ok(Self { value, figures })
    }

    /// A value whose every representable digit is claimed.
    ///
    /// Use this for counts and definitions — the 86 400 seconds of a nominal
    /// day, the defined speed of light — not for measurements.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] for NaN or infinity.
    pub fn exact(value: f64) -> UncertaintyResult<Self> {
        Self::new(value, MAX_FIGURES)
    }

    /// The stored value, unrounded.
    ///
    /// This is the number as it was supplied or as arithmetic produced it. It
    /// may carry digits beyond the significant ones; [`Significant::rounded`]
    /// discards those.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }

    /// How many digits of the value are claimed.
    #[must_use]
    pub const fn figures(self) -> u8 {
        self.figures
    }

    /// The base-ten exponent of the leading digit of the value as reported.
    ///
    /// `1.38e10` has exponent 10, `0.00123` has exponent −3. This is the
    /// exponent of the rounded numeral, so `9.96` to two figures, which is
    /// `10`, has exponent 1. Zero has no leading digit; it reports 0 by
    /// convention so that callers do not have to special-case it.
    #[must_use]
    pub fn decimal_exponent(self) -> i32 {
        let numeral = self.numeral();
        if numeral.is_zero() {
            0
        } else {
            numeral.exponent
        }
    }

    /// The decimal place of the last significant digit, as a power of ten.
    ///
    /// `13.8` with three figures is significant down to 10^−1; `1.38e10` with
    /// three figures only down to 10^8, which is why it must not be printed
    /// as a plain integer. A value that rounds into a new decade moves with
    /// it: `99.96` to three figures is `100`, significant down to 10^0.
    #[must_use]
    pub fn last_significant_place(self) -> i32 {
        self.decimal_exponent() - i32::from(self.figures) + 1
    }

    /// The value rounded to its significant digits.
    ///
    /// The rule is NIST SP 811, B.7.1: below one half rounds down, above one
    /// half rounds up, and exactly one half goes to the even digit, judged on
    /// the shortest decimal numeral of the value. `2.5` to one figure is `2`,
    /// `3.5` is `4`, and `2.675` to three figures is `2.68`. The printed form
    /// shows the same digits.
    #[must_use]
    pub fn rounded(self) -> f64 {
        round_to_figures(self.value, self.figures)
    }

    /// The same value with a different, explicitly stated figure count.
    ///
    /// # Errors
    ///
    /// See [`Significant::new`].
    pub fn with_figures(self, figures: u8) -> UncertaintyResult<Self> {
        Self::new(self.value, figures)
    }

    /// The value with its sign reversed. Negation loses no digits.
    #[must_use]
    pub const fn negated(self) -> Self {
        Self {
            value: -self.value,
            figures: self.figures,
        }
    }

    /// Sum, significant down to the coarser of the two last places.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the sum overflows `f64`.
    pub fn checked_add(self, other: Self) -> UncertaintyResult<Self> {
        self.additive(other, self.value + other.value)
    }

    /// Difference, significant down to the coarser of the two last places.
    ///
    /// Subtracting two close numbers destroys figures — `1.0000` minus
    /// `0.9999` keeps one — and this is exactly the catastrophic
    /// cancellation the rule is meant to make visible.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the difference overflows
    /// `f64`.
    pub fn checked_sub(self, other: Self) -> UncertaintyResult<Self> {
        self.additive(other, self.value - other.value)
    }

    /// Product, carrying the smaller figure count.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the product overflows
    /// `f64`.
    pub fn checked_mul(self, other: Self) -> UncertaintyResult<Self> {
        Self::new(self.value * other.value, self.figures.min(other.figures))
    }

    /// Quotient, carrying the smaller figure count.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::DivideByZero`] for a zero divisor and
    /// [`UncertaintyError::NotFinite`] when the quotient overflows `f64`.
    pub fn checked_div(self, other: Self) -> UncertaintyResult<Self> {
        if other.value == 0.0 {
            return Err(UncertaintyError::DivideByZero);
        }
        Self::new(self.value / other.value, self.figures.min(other.figures))
    }

    /// Raise to an integer power, keeping this value's figure count.
    ///
    /// Repeated multiplication of a number by itself does not lose figures
    /// under the laboratory rule, because both operands carry the same count.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NotFinite`] when the power overflows
    /// `f64`.
    pub fn checked_powi(self, exponent: i32) -> UncertaintyResult<Self> {
        Self::new(math::powf(self.value, f64::from(exponent)), self.figures)
    }

    /// Shared tail of addition and subtraction.
    ///
    /// The figure count of the answer is recovered from the coarser of the
    /// two operands' last significant places rather than from their figure
    /// counts directly, which is what makes `100.0 + 0.001` keep four rather
    /// than seven.
    fn additive(self, other: Self, result: f64) -> UncertaintyResult<Self> {
        let place = self
            .last_significant_place()
            .max(other.last_significant_place());
        Self::at_place(result, place)
    }
}

/// Room for the longest `{:e}` rendering of an `f64`, `1.2345678901234567e-308`.
const BUFFER: usize = 40;

/// A fixed buffer that `core::fmt` can write into, so that this module needs
/// no allocator.
struct Buffer {
    bytes: [u8; BUFFER],
    len: usize,
}

impl Buffer {
    const fn new() -> Self {
        Self {
            bytes: [0; BUFFER],
            len: 0,
        }
    }

    fn as_str(&self) -> &str {
        // Only `fmt::Write::write_str` fills the buffer, with whole `&str`s.
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }
}

impl fmt::Write for Buffer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.len + text.len();
        if end > BUFFER {
            return Err(fmt::Error);
        }
        self.bytes[self.len..end].copy_from_slice(text.as_bytes());
        self.len = end;
        Ok(())
    }
}

/// A decimal numeral: up to 17 digits and the base-ten exponent of the first.
///
/// `1.38e10` is the digits `1, 3, 8` with exponent 10. Zero is the single
/// digit 0 with exponent 0.
#[derive(Clone, Copy)]
struct Decimal {
    negative: bool,
    digits: [u8; MAX_FIGURES as usize],
    len: usize,
    exponent: i32,
}

impl Decimal {
    /// The shortest decimal numeral that reads back as `value`.
    ///
    /// Rust's `{:e}` is that numeral, and it is exact about the exponent where
    /// `log10` is only good to an ulp.
    fn shortest(value: f64) -> Self {
        use fmt::Write as _;
        let mut buffer = Buffer::new();
        let negative = value.is_sign_negative() && value != 0.0;
        // A finite `f64` always fits: at most 17 digits, a point, `e`, a
        // sign and three exponent digits.
        let _ = write!(buffer, "{:e}", math::abs(value));
        let text = buffer.as_str();
        let (mantissa, exponent) = text.split_once('e').unwrap_or((text, "0"));
        let mut digits = [0u8; MAX_FIGURES as usize];
        let mut len = 0;
        for byte in mantissa.bytes().filter(u8::is_ascii_digit) {
            if len < digits.len() {
                digits[len] = byte - b'0';
                len += 1;
            }
        }
        Self {
            negative,
            digits,
            len: len.max(1),
            exponent: exponent.parse().unwrap_or(0),
        }
    }

    /// This numeral rounded to `figures` digits by NIST SP 811, B.7.1.
    ///
    /// Digits already at or below `figures` are left alone: padding is the
    /// renderer's business.
    fn rounded(mut self, figures: usize) -> Self {
        let figures = figures.max(1);
        if self.len <= figures {
            return self;
        }
        let first = self.digits[figures];
        let beyond_first = self.digits[figures + 1..self.len].iter().any(|d| *d != 0);
        let round_up = match first {
            0..=4 => false,
            5 if beyond_first => true,
            // Exactly one half: the preceding digit is left if even and
            // raised if odd (rule 3).
            5 => self.digits[figures - 1] % 2 == 1,
            _ => true,
        };
        self.len = figures;
        if round_up {
            let mut place = figures;
            loop {
                if place == 0 {
                    // 9.99 → 10.0: the numeral gains a decade.
                    self.digits[0] = 1;
                    for digit in &mut self.digits[1..figures] {
                        *digit = 0;
                    }
                    self.exponent += 1;
                    break;
                }
                place -= 1;
                if self.digits[place] == 9 {
                    self.digits[place] = 0;
                } else {
                    self.digits[place] += 1;
                    break;
                }
            }
        }
        self
    }

    fn is_zero(&self) -> bool {
        self.digits[0] == 0
    }

    /// The digit `place` positions after the leading one, zero beyond the end.
    fn digit(&self, place: usize) -> u8 {
        if place < self.len {
            self.digits[place]
        } else {
            0
        }
    }

    /// The `f64` nearest this numeral.
    ///
    /// Exact where one IEEE operation is: a mantissa of at most 15 digits
    /// (below 2⁵³) scaled by a power of ten up to 10²², the fast path of
    /// Clinger's algorithm, so the result is the correctly rounded one. The
    /// 16- and 17-digit mantissas of a value that claims the most `f64` can
    /// hold, and the powers beyond 10²², can differ from the correctly
    /// rounded value by one unit in the last place; the only caller, a
    /// rounding, accepts that. (The standard library's parser would be exact
    /// everywhere, at some 20 KB of code in a WebAssembly build.)
    fn to_f64(self) -> f64 {
        let mut mantissa = 0.0f64;
        for place in 0..self.len {
            mantissa = mantissa * 10.0 + f64::from(self.digits[place]);
        }
        let scale = self.exponent - (self.len as i32 - 1);
        let value = if scale >= 0 {
            mantissa * power_of_ten(scale)
        } else {
            mantissa / power_of_ten(-scale)
        };
        if self.negative { -value } else { value }
    }
}

/// `10^exponent`, exact up to 10²², and a product of exact powers beyond.
fn power_of_ten(exponent: i32) -> f64 {
    const EXACT: [f64; 23] = [
        1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16,
        1e17, 1e18, 1e19, 1e20, 1e21, 1e22,
    ];
    let mut remaining = exponent;
    let mut power = 1.0;
    while remaining > 22 {
        power *= EXACT[22];
        remaining -= 22;
    }
    power * EXACT[remaining as usize]
}

/// Round `value` so that only `figures` leading digits survive.
fn round_to_figures(value: f64, figures: u8) -> f64 {
    if value == 0.0 {
        return 0.0;
    }
    let rounded = Decimal::shortest(value)
        .rounded(usize::from(figures))
        .to_f64();
    if rounded.is_finite() { rounded } else { value }
}

impl Significant {
    /// The numeral this value is reported as: rounded, not yet padded.
    fn numeral(self) -> Decimal {
        if self.is_exact() {
            Decimal::shortest(self.value)
        } else {
            Decimal::shortest(self.value).rounded(usize::from(self.figures))
        }
    }

    /// Whether every digit that identifies the `f64` is claimed.
    const fn is_exact(self) -> bool {
        self.figures == MAX_FIGURES
    }

    /// A value reported down to the decimal place `place`, a power of ten.
    ///
    /// This is how a sum or an error bar fixes a figure count: from the last
    /// place, not from a count of digits. The place is kept when rounding
    /// carries into a new decade, so `99.96` known to the tenths is `100.0`
    /// (four figures) and not `100` (three). That is the one case where the
    /// stored value is the rounded one, because a bare `(99.96, 4)` would
    /// mean `99.96`.
    pub(crate) fn at_place(value: f64, place: i32) -> UncertaintyResult<Self> {
        if !value.is_finite() {
            return Err(UncertaintyError::NotFinite);
        }
        let numeral = Decimal::shortest(value);
        let exponent = if numeral.is_zero() {
            0
        } else {
            numeral.exponent
        };
        // Clamped to the range `Significant` accepts: a result whose last
        // significant place lies above its own leading digit still gets one
        // figure, because reporting a bare "no digits are meaningful" is
        // less useful to a caller than reporting the order of magnitude.
        let wanted = exponent - place + 1;
        let figures = wanted.clamp(1, i32::from(MAX_FIGURES)) as u8;
        if wanted >= 1 && wanted < i32::from(MAX_FIGURES) && !numeral.is_zero() {
            let rounded = numeral.rounded(wanted as usize);
            if rounded.exponent > numeral.exponent {
                return Self::new(rounded.to_f64(), figures + 1);
            }
        }
        Self::new(value, figures)
    }
}

impl fmt::Display for Significant {
    /// Render the value at its true precision, and no better.
    ///
    /// The digits printed are those of [`Significant::rounded`], padded with
    /// zeros up to the claimed count, so `1.0` known to four figures is
    /// `1.000` and `100` known to three is `100`. Plain decimal notation is
    /// used when every digit it shows is either significant or a placeholder
    /// zero after the point. When the value is large enough that plain
    /// notation would need non-significant zeros before the point — 13.8
    /// billion, whose `13800000000` claims eleven digits — or small enough
    /// that it would need more than four leading zeros, scientific notation
    /// is used instead. This is the convention of the SI Brochure and of
    /// *Physical Review*'s style guide, and it is the only notation in which
    /// "three significant figures" is unambiguous.
    ///
    /// An exact value ([`MAX_FIGURES`]) prints the shortest numeral that
    /// reads back as the same `f64`, without padding.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let claimed = usize::from(self.figures);
        let numeral = self.numeral();
        // How many digits are written: the claim, or all of an exact value's.
        let shown = if self.is_exact() {
            numeral.len
        } else {
            claimed
        };
        if numeral.is_zero() {
            f.write_str("0")?;
            if shown > 1 {
                f.write_str(".")?;
                for _ in 1..shown {
                    f.write_str("0")?;
                }
            }
            return Ok(());
        }
        if numeral.negative {
            f.write_str("-")?;
        }
        let exponent = numeral.exponent;
        let digit = |place: usize| char::from(b'0' + numeral.digit(place));
        if exponent >= i32::from(self.figures) || exponent < -4 {
            write!(f, "{}", digit(0))?;
            if shown > 1 {
                f.write_str(".")?;
                for place in 1..shown {
                    write!(f, "{}", digit(place))?;
                }
            }
            write!(f, "e{exponent}")
        } else if exponent >= 0 {
            let integer = exponent as usize + 1;
            for place in 0..integer {
                write!(f, "{}", digit(place))?;
            }
            if shown > integer {
                f.write_str(".")?;
                for place in integer..shown {
                    write!(f, "{}", digit(place))?;
                }
            }
            Ok(())
        } else {
            f.write_str("0.")?;
            for _ in 0..(-exponent - 1) {
                f.write_str("0")?;
            }
            for place in 0..shown {
                write!(f, "{}", digit(place))?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::string::ToString as _;

    #[test]
    fn thirteen_point_eight_billion_years_keeps_three_figures() {
        let age = Significant::new(13.8e9, 3).unwrap();
        assert_eq!(age.figures(), 3);
        #[cfg(feature = "alloc")]
        assert_eq!(age.to_string(), "1.38e10");
    }

    #[test]
    fn a_three_figure_value_never_renders_as_eleven_digits() {
        let age = Significant::new(13.8e9, 3).unwrap();
        #[cfg(feature = "alloc")]
        assert_ne!(age.to_string(), "13800000000");
        assert_eq!(age.last_significant_place(), 8);
    }

    #[test]
    fn small_magnitudes_render_in_plain_decimal_until_they_get_silly() {
        let a = Significant::new(0.00123, 3).unwrap();
        let b = Significant::new(0.000_012_3, 3).unwrap();
        #[cfg(feature = "alloc")]
        {
            assert_eq!(a.to_string(), "0.00123");
            assert_eq!(b.to_string(), "1.23e-5");
        }
        let _ = (a, b);
    }

    #[test]
    fn modest_magnitudes_render_in_plain_decimal() {
        #[cfg(feature = "alloc")]
        {
            assert_eq!(Significant::new(13.8, 3).unwrap().to_string(), "13.8");
            assert_eq!(Significant::new(1.0, 4).unwrap().to_string(), "1.000");
            assert_eq!(Significant::new(-2.5, 2).unwrap().to_string(), "-2.5");
            assert_eq!(Significant::new(100.0, 3).unwrap().to_string(), "100");
        }
    }

    #[test]
    fn zero_renders_with_its_claimed_decimals() {
        #[cfg(feature = "alloc")]
        assert_eq!(Significant::new(0.0, 3).unwrap().to_string(), "0.00");
        assert_eq!(Significant::new(0.0, 3).unwrap().decimal_exponent(), 0);
    }

    #[test]
    fn the_decimal_exponent_is_exact_at_powers_of_ten() {
        for exponent in -12..=12 {
            // The literal `1e<exponent>`, which is exact where libm's
            // `powf(10, -5)` is one ulp low.
            let mut buffer = Buffer::new();
            fmt::Write::write_fmt(&mut buffer, format_args!("1e{exponent}")).unwrap();
            let value: f64 = buffer.as_str().parse().unwrap();
            assert_eq!(
                Decimal::shortest(value).exponent,
                exponent,
                "at 10^{exponent}"
            );
            assert_eq!(
                Decimal::shortest(-value).exponent,
                exponent,
                "at -10^{exponent}"
            );
        }
    }

    #[test]
    fn the_decimal_exponent_brackets_every_magnitude() {
        let samples = [1.0, 9.999, 10.0, 99.999, 1e-7, 6.02e23, 3.5e-18];
        for value in samples {
            let exponent = Decimal::shortest(value).exponent;
            let low = math::powf(10.0, f64::from(exponent));
            let high = math::powf(10.0, f64::from(exponent + 1));
            assert!(low <= value && value < high, "{value} not in decade");
        }
    }

    #[test]
    fn rounding_discards_digits_beyond_the_claim() {
        assert!((Significant::new(1.23456, 3).unwrap().rounded() - 1.23).abs() < 1e-12);
        assert!((Significant::new(9.87654, 2).unwrap().rounded() - 9.9).abs() < 1e-12);
        assert!((Significant::new(123_456.0, 2).unwrap().rounded() - 120_000.0).abs() < 1e-6);
    }

    #[test]
    fn nist_sp_811_b71_rounds_below_one_half_down_and_above_up() {
        // NIST SP 811, B.7.1, rules 1 and 2, with its own number 6.974 951 5:
        // 3 digits 6.97 (the discard begins with 4); 5 digits 6.9750 (it
        // begins with 5 and a nonzero digit follows).
        let nist = 6.974_951_5;
        assert_eq!(round_to_figures(nist, 3), 6.97);
        assert_eq!(round_to_figures(nist, 5), 6.975);
        #[cfg(feature = "alloc")]
        {
            assert_eq!(Significant::new(nist, 3).unwrap().to_string(), "6.97");
            assert_eq!(Significant::new(nist, 5).unwrap().to_string(), "6.9750");
        }
    }

    #[test]
    fn nist_sp_811_b71_sends_exactly_one_half_to_the_even_digit() {
        // Rule 3 with NIST's own examples: 6.974 951 5 to 7 digits is
        // 6.974 952 (the preceding 1 is odd), 6.974 950 5 to 7 is 6.974 950
        // (the preceding 0 is even).
        assert_eq!(round_to_figures(6.974_951_5, 7), 6.974_952);
        assert_eq!(round_to_figures(6.974_950_5, 7), 6.974_950);
        #[cfg(feature = "alloc")]
        {
            assert_eq!(
                Significant::new(6.974_951_5, 7).unwrap().to_string(),
                "6.974952"
            );
            assert_eq!(
                Significant::new(6.974_950_5, 7).unwrap().to_string(),
                "6.974950"
            );
        }
        // The same rule at one figure, either sign.
        for (value, expected) in [
            (0.25, 0.2),
            (0.35, 0.4),
            (1.5, 2.0),
            (2.5, 2.0),
            (3.5, 4.0),
            (-2.5, -2.0),
        ] {
            assert_eq!(round_to_figures(value, 1), expected, "{value}");
        }
    }

    #[test]
    fn a_tie_is_a_property_of_the_decimal_numeral_not_of_the_binary_value() {
        // 2.675 is stored just below the tie, 1.15 just below, 0.285 just
        // below; read as the numerals they are, each is exactly one half.
        assert_eq!(round_to_figures(2.675, 3), 2.68);
        assert_eq!(round_to_figures(1.15, 2), 1.2);
        assert_eq!(round_to_figures(0.285, 2), 0.28);
    }

    #[test]
    fn display_and_rounded_agree_at_a_tie() {
        // Before: 2.5 printed `2` (Rust's ties-to-even) but rounded() gave 3.
        #[cfg(feature = "alloc")]
        for (value, figures) in [(2.5, 1u8), (3.5, 1), (0.125, 2), (2.675, 3), (-1.5, 1)] {
            let significant = Significant::new(value, figures).unwrap();
            let printed: f64 = significant.to_string().parse().unwrap();
            assert_eq!(printed, significant.rounded(), "{value} to {figures}");
        }
    }

    #[test]
    fn a_decade_crossing_keeps_the_claimed_figures() {
        // Before: 9.96 to two figures printed `10.0` (three figures), 99.96
        // to three printed `100.0`, 0.0999 to two printed `0.100`.
        #[cfg(feature = "alloc")]
        {
            assert_eq!(Significant::new(9.96, 2).unwrap().to_string(), "10");
            assert_eq!(Significant::new(99.96, 3).unwrap().to_string(), "100");
            assert_eq!(Significant::new(0.0999, 2).unwrap().to_string(), "0.10");
            assert_eq!(Significant::new(9.996, 3).unwrap().to_string(), "10.0");
            assert_eq!(Significant::new(-9.96, 2).unwrap().to_string(), "-10");
            assert_eq!(Significant::new(9.5e9, 1).unwrap().to_string(), "1e10");
        }
        let crossed = Significant::new(9.96, 2).unwrap();
        assert_eq!(crossed.rounded(), 10.0);
        // The exponent and last place are those of the numeral reported.
        assert_eq!(crossed.decimal_exponent(), 1);
        assert_eq!(crossed.last_significant_place(), 0);
        let crossed = Significant::new(0.0999, 2).unwrap();
        assert_eq!(crossed.decimal_exponent(), -1);
        assert_eq!(crossed.last_significant_place(), -2);
    }

    #[test]
    fn an_exact_value_prints_the_shortest_numeral_that_reads_back() {
        // Before: exact(0.1) printed 0.10000000000000001 (17 figures).
        #[cfg(feature = "alloc")]
        for (value, text) in [
            (0.1, "0.1"),
            (13.8e9, "13800000000"),
            (86_400.0, "86400"),
            (299_792_458.0, "299792458"),
            (-0.3, "-0.3"),
            (1e20, "1e20"),
            (1.5e-7, "1.5e-7"),
            (0.0, "0"),
        ] {
            let exact = Significant::exact(value).unwrap();
            assert_eq!(exact.to_string(), text);
            assert_eq!(text.parse::<f64>().unwrap(), value);
            assert_eq!(exact.rounded(), value);
        }
    }

    #[test]
    fn a_sum_that_crosses_a_decade_keeps_its_last_place() {
        // 99.9 (to the tenths) + 0.06 = 99.96, known to the tenths: 100.0.
        let a = Significant::new(99.9, 3).unwrap();
        let b = Significant::new(0.06, 1).unwrap();
        let sum = a.checked_add(b).unwrap();
        assert_eq!(sum.last_significant_place(), -1);
        assert_eq!(sum.figures(), 4);
        #[cfg(feature = "alloc")]
        assert_eq!(sum.to_string(), "100.0");
    }

    #[test]
    fn rounding_a_value_already_at_precision_changes_nothing() {
        for figures in 1..=9u8 {
            let value = 1.234_567_89_f64;
            let once = round_to_figures(value, figures);
            let twice = round_to_figures(once, figures);
            assert!((once - twice).abs() < 1e-15, "unstable at {figures}");
        }
    }

    #[test]
    fn multiplication_keeps_the_weaker_operand() {
        let a = Significant::new(2.0, 1).unwrap();
        let b = Significant::new(3.000, 4).unwrap();
        let product = a.checked_mul(b).unwrap();
        assert_eq!(product.figures(), 1);
        assert!((product.value() - 6.0).abs() < 1e-12);
    }

    #[test]
    fn division_keeps_the_weaker_operand() {
        let a = Significant::new(1.0, 2).unwrap();
        let b = Significant::new(3.0, 5).unwrap();
        let quotient = a.checked_div(b).unwrap();
        assert_eq!(quotient.figures(), 2);
        #[cfg(feature = "alloc")]
        assert_eq!(quotient.to_string(), "0.33");
    }

    #[test]
    fn division_by_zero_is_reported_rather_than_returning_infinity() {
        let a = Significant::new(1.0, 3).unwrap();
        let zero = Significant::new(0.0, 3).unwrap();
        assert_eq!(a.checked_div(zero), Err(UncertaintyError::DivideByZero));
    }

    #[test]
    fn addition_is_limited_by_the_coarser_decimal_place() {
        // 100.0 knows nothing past the tenths; 0.001 cannot rescue it.
        let coarse = Significant::new(100.0, 4).unwrap();
        let fine = Significant::new(0.001, 1).unwrap();
        let sum = coarse.checked_add(fine).unwrap();
        assert_eq!(sum.figures(), 4);
        #[cfg(feature = "alloc")]
        assert_eq!(sum.to_string(), "100.0");
    }

    #[test]
    fn subtraction_of_near_equals_destroys_figures() {
        let a = Significant::new(1.0000, 5).unwrap();
        let b = Significant::new(0.9999, 4).unwrap();
        let difference = a.checked_sub(b).unwrap();
        assert_eq!(difference.figures(), 1);
    }

    #[test]
    fn negation_preserves_the_figure_count() {
        let value = Significant::new(6.674e-11, 4).unwrap();
        let negated = value.negated();
        assert_eq!(negated.figures(), 4);
        assert!((negated.value() + 6.674e-11).abs() < 1e-25);
    }

    #[test]
    fn integer_powers_keep_the_figure_count() {
        let value = Significant::new(2.50, 3).unwrap();
        let squared = value.checked_powi(2).unwrap();
        assert_eq!(squared.figures(), 3);
        assert!((squared.value() - 6.25).abs() < 1e-12);
    }

    #[test]
    fn a_figure_count_of_zero_or_eighteen_is_rejected() {
        assert_eq!(
            Significant::new(1.0, 0),
            Err(UncertaintyError::InvalidSignificantFigures)
        );
        assert_eq!(
            Significant::new(1.0, 18),
            Err(UncertaintyError::InvalidSignificantFigures)
        );
    }

    #[test]
    fn non_finite_values_are_rejected() {
        assert_eq!(
            Significant::new(f64::NAN, 3),
            Err(UncertaintyError::NotFinite)
        );
        assert_eq!(
            Significant::new(f64::INFINITY, 3),
            Err(UncertaintyError::NotFinite)
        );
    }

    #[test]
    fn overflow_in_a_product_is_reported_as_non_finite() {
        let big = Significant::new(1e300, 3).unwrap();
        assert_eq!(big.checked_mul(big), Err(UncertaintyError::NotFinite));
    }

    #[test]
    fn an_exact_value_claims_every_representable_digit() {
        let day = Significant::exact(86_400.0).unwrap();
        assert_eq!(day.figures(), MAX_FIGURES);
        assert!((day.rounded() - 86_400.0).abs() < 1e-9);
    }

    #[test]
    fn with_figures_restates_the_claim_without_moving_the_value() {
        let value = Significant::new(1.23456, 6).unwrap();
        let coarser = value.with_figures(3).unwrap();
        assert!((coarser.value() - 1.23456).abs() < 1e-15);
        assert!((coarser.rounded() - 1.23).abs() < 1e-12);
    }

    #[test]
    fn rendering_round_trips_through_a_reparsed_magnitude() {
        // Whatever notation is chosen, the rendered digits must round-trip to
        // the rounded value; that is the whole point of the type.
        #[cfg(feature = "alloc")]
        for (value, figures) in [(13.8e9, 3u8), (0.00123, 3), (1.0, 1), (-45.67, 4)] {
            let rendered = Significant::new(value, figures).unwrap().to_string();
            let parsed: f64 = rendered.parse().unwrap();
            let expected = round_to_figures(value, figures);
            assert!(
                (parsed - expected).abs() <= math::abs(expected) * 1e-12,
                "{rendered} != {expected}"
            );
        }
    }
}
