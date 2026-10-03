//! Just enough 256-bit unsigned arithmetic for [`crate::Duration`] to scale
//! a span by an exact ratio without rounding twice.
//!
//! A span is up to 2¹⁸⁷ attoseconds and a double's mantissa is 53 bits, so
//! the exact product is up to 2²⁴⁰: it does not fit an `i128`.

/// An unsigned 256-bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct U256 {
    hi: u128,
    lo: u128,
}

impl U256 {
    pub(crate) const ZERO: Self = Self { hi: 0, lo: 0 };

    pub(crate) const fn from_u128(value: u128) -> Self {
        Self { hi: 0, lo: value }
    }

    pub(crate) const fn is_zero(self) -> bool {
        self.hi == 0 && self.lo == 0
    }

    /// The low 128 bits, when the value has no more.
    pub(crate) const fn to_u128(self) -> Option<u128> {
        if self.hi == 0 { Some(self.lo) } else { None }
    }

    pub(crate) const fn is_odd(self) -> bool {
        self.lo & 1 == 1
    }

    /// `self * factor`, or `None` when it needs more than 256 bits.
    pub(crate) fn checked_mul_u64(self, factor: u64) -> Option<Self> {
        let factor = u128::from(factor);
        let (lo_low, lo_high) = mul_u128_u64_parts(self.lo, factor);
        // `self.hi * factor` must fit 128 bits, plus the carry from the low half.
        let (hi_low, hi_high) = mul_u128_u64_parts(self.hi, factor);
        if hi_high != 0 {
            return None;
        }
        let hi = hi_low.checked_add(lo_high)?;
        Some(Self { hi, lo: lo_low })
    }

    /// `self + addend`, or `None` on overflow.
    pub(crate) fn checked_add_u64(self, addend: u64) -> Option<Self> {
        let (lo, carry) = self.lo.overflowing_add(u128::from(addend));
        Some(Self {
            hi: if carry {
                self.hi.checked_add(1)?
            } else {
                self.hi
            },
            lo,
        })
    }

    /// `self - subtrahend`, for a value known to be at least `subtrahend`.
    pub(crate) fn sub_u64(self, subtrahend: u64) -> Self {
        let (lo, borrow) = self.lo.overflowing_sub(u128::from(subtrahend));
        Self {
            hi: if borrow { self.hi - 1 } else { self.hi },
            lo,
        }
    }

    pub(crate) fn checked_add_one(self) -> Option<Self> {
        let (lo, carry) = self.lo.overflowing_add(1);
        if carry {
            Some(Self {
                hi: self.hi.checked_add(1)?,
                lo,
            })
        } else {
            Some(Self { hi: self.hi, lo })
        }
    }

    /// `self << shift`, or `None` when a set bit would be lost.
    pub(crate) fn checked_shl(self, shift: u32) -> Option<Self> {
        if self.is_zero() || shift == 0 {
            return Some(self);
        }
        if shift >= 256 || self.leading_zeros() < shift {
            return None;
        }
        Some(if shift >= 128 {
            Self {
                hi: self.lo << (shift - 128),
                lo: 0,
            }
        } else {
            Self {
                hi: (self.hi << shift) | (self.lo >> (128 - shift)),
                lo: self.lo << shift,
            }
        })
    }

    /// `self >> shift`; 0 when `shift` is 256 or more.
    pub(crate) fn shr(self, shift: u32) -> Self {
        if shift == 0 {
            self
        } else if shift >= 256 {
            Self::ZERO
        } else if shift >= 128 {
            Self {
                hi: 0,
                lo: self.hi >> (shift - 128),
            }
        } else {
            Self {
                hi: self.hi >> shift,
                lo: (self.lo >> shift) | (self.hi << (128 - shift)),
            }
        }
    }

    /// The low `bits` bits of the value (`bits` below 256).
    pub(crate) fn low_bits(self, bits: u32) -> Self {
        if bits >= 256 {
            self
        } else if bits >= 128 {
            Self {
                hi: self.hi & ((1u128 << (bits - 128)) - 1),
                lo: self.lo,
            }
        } else {
            Self {
                hi: 0,
                lo: self.lo & ((1u128 << bits) - 1),
            }
        }
    }

    /// Whether `bit` (counting from 0) is set.
    pub(crate) fn bit(self, bit: u32) -> bool {
        if bit >= 128 {
            (self.hi >> (bit - 128)) & 1 == 1
        } else {
            (self.lo >> bit) & 1 == 1
        }
    }

    pub(crate) fn leading_zeros(self) -> u32 {
        if self.hi != 0 {
            self.hi.leading_zeros()
        } else {
            128 + self.lo.leading_zeros()
        }
    }

    pub(crate) fn cmp_with(self, other: Self) -> core::cmp::Ordering {
        self.hi.cmp(&other.hi).then(self.lo.cmp(&other.lo))
    }

    /// Quotient and remainder by a divisor of at most 2¹²⁷.
    ///
    /// Binary long division: 256 shifts, which is plenty for a conversion
    /// that happens once per scaling.
    pub(crate) fn divmod_u128(self, divisor: u128) -> (Self, u128) {
        debug_assert!(divisor != 0 && divisor < (1 << 127));
        let mut quotient = Self::ZERO;
        let mut remainder = 0u128;
        for index in (0..256u32).rev() {
            remainder = (remainder << 1) | u128::from(self.bit(index));
            if remainder >= divisor {
                remainder -= divisor;
                if index >= 128 {
                    quotient.hi |= 1u128 << (index - 128);
                } else {
                    quotient.lo |= 1u128 << index;
                }
            }
        }
        (quotient, remainder)
    }
}

/// `value * factor` as a (low, high) pair of 128-bit halves; `factor` is
/// below 2⁶⁴, so each 64-bit limb of `value` times it fits.
fn mul_u128_u64_parts(value: u128, factor: u128) -> (u128, u128) {
    let low_limb = value & u128::from(u64::MAX);
    let high_limb = value >> 64;
    let low_product = low_limb * factor; // < 2^128
    let high_product = high_limb * factor; // < 2^128
    // value * factor = low_product + high_product * 2^64
    let (lo, carry) = low_product.overflowing_add(high_product << 64);
    let hi = (high_product >> 64) + u128::from(carry);
    (lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication_and_division_round_trip() {
        let value = U256 {
            hi: 0x1234_5678,
            lo: 0xfedc_ba98_7654_3210_0123_4567_89ab_cdef,
        };
        let product = value.checked_mul_u64(1_000_003).unwrap();
        let (quotient, remainder) = product.divmod_u128(1_000_003);
        assert_eq!((quotient, remainder), (value, 0));
        let (quotient, remainder) = product.divmod_u128(7);
        let back = quotient.checked_mul_u64(7).unwrap();
        assert_eq!(back.cmp_with(product), core::cmp::Ordering::Less);
        assert!(remainder < 7);
    }

    #[test]
    fn shifts_are_inverse_while_nothing_is_lost() {
        let value = U256::from_u128(0xabcdef);
        for shift in [0, 1, 63, 64, 127, 128, 129, 200] {
            let shifted = value.checked_shl(shift).unwrap();
            assert_eq!(shifted.shr(shift), value, "{shift}");
        }
        assert!(value.checked_shl(240).is_none());
        assert_eq!(value.shr(256), U256::ZERO);
    }

    #[test]
    fn low_bits_and_bits_agree() {
        let value = U256 {
            hi: 1,
            lo: u128::MAX,
        };
        assert!(value.bit(128));
        assert!(value.bit(127));
        assert_eq!(value.low_bits(128), U256::from_u128(u128::MAX));
        assert_eq!(value.low_bits(129), value);
    }
}
