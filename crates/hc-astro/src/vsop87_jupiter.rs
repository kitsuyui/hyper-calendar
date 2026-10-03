//! Jupiter's heliocentric position from VSOP87B, with a truncation you
//! choose. Behind the `jupiter` feature: the series are 47 kB of data.
//!
//! VSOP87 (Bretagnon and Francou, "Planetary theories in rectangular and
//! spherical variables: VSOP87 solutions", *Astronomy and Astrophysics* 202,
//! 309–315, 1988, `bretagnon1988`) is the same theory [`crate::vsop87`] takes
//! the Earth from. Version B gives heliocentric spherical coordinates
//! referred to the dynamical ecliptic and equinox of **J2000.0**: longitude
//! and latitude in radians and distance in astronomical units, each a sum of
//! the powers τ⁰ to τ⁵ of the time τ in Julian millennia from J2000.0 TDB,
//! each power a sum of terms *A* cos(*B* + *C* τ). The file for Jupiter,
//! `VSOP87B.jup` of the IMCCE, has 3 625 terms; every one is here, at the
//! file's own 11 decimals (`scripts/vsop87-jupiter.py` writes them, and its
//! `--check` holds them to the file's sha256, [`SOURCE_SHA256`]).
//!
//! # The truncation
//!
//! [`Truncation::Full`] sums every term. [`Truncation::Amplitude`] drops each
//! term whose largest size over the date asked, *A* |τ|^*k*, falls below the
//! limit, in radians for the longitude and latitude and astronomical units
//! for the radius. The terms are not in the file in order of size, so this
//! is a test per term, not a stop. What the cut costs, measured against the
//! full series at 1 000 dates from −1000 to 3000, and what it saves, are in
//! `docs/systems/jupiter-festivals.md` §Accuracy and in the tests below.
//!
//! The size of the *data* does not shrink with the cut: the tables are
//! whole, so that the choice is the caller's, per call. A build that wants
//! fewer bytes does not enable the `jupiter` feature, which is the one thing
//! that carries them.
//!
//! # What VSOP87 says of itself
//!
//! Its documentation (`VSOP87.doc`, not read at its origin: CDS's server
//! refused the request; read in a copy that quotes it) gives a precision of
//! 1″ for Jupiter and Saturn over 2 000 years either side of J2000.0, and
//! the sum of the neglected terms is smaller than the stated limit. This
//! crate's measured agreement with JPL's DE441 is in the tests.

// The coefficients are the file's, and a frequency that happens to be near a
// round number is the file's.
#![allow(clippy::excessive_precision, clippy::approx_constant)]

use hc_core::math::{abs, cos, powf};

mod data;

use data::{BLOCKS, FREQUENCIES, TERM_BYTES};
pub use data::{SOURCE_SHA256, SOURCE_TERMS};

/// How much of the series to sum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Truncation {
    /// Every term of the file.
    Full,
    /// Only the terms whose size, *A* |τ|^*k* with τ in Julian millennia and
    /// *k* the power, is at least this: radians for longitude and latitude,
    /// astronomical units for the radius.
    Amplitude(f64),
}

impl Truncation {
    /// The amplitude below which a term is dropped; zero keeps all.
    const fn limit(self) -> f64 {
        match self {
            Self::Full => 0.0,
            Self::Amplitude(limit) => limit,
        }
    }
}

/// The units of the stored amplitudes and phases: 10⁻¹¹.
const SCALE: f64 = 1e11;

/// The amplitude of a stored term, in units of 10⁻¹¹: its first six bytes,
/// little-endian.
///
/// The three fields are read by naming the bytes, not by a loop over a
/// slice: the loop is a call and a counter per byte under the size-first
/// profile the WebAssembly module is built with, and a term is read some
/// 5 000 times for each position (the amplitude of every term is tested, and
/// the rest of the 1 400 or so that pass are summed). The values are the
/// same.
#[inline(always)]
fn amplitude_of(term: &[u8; TERM_BYTES]) -> u64 {
    u64::from_le_bytes([term[0], term[1], term[2], term[3], term[4], term[5], 0, 0])
}

/// The phase of a stored term, in units of 10⁻¹¹: the next five bytes.
#[inline(always)]
fn phase_of(term: &[u8; TERM_BYTES]) -> u64 {
    u64::from_le_bytes([term[6], term[7], term[8], term[9], term[10], 0, 0, 0])
}

/// The index of a stored term's frequency in [`FREQUENCIES`]: the last two
/// bytes.
#[inline(always)]
fn frequency_index_of(term: &[u8; TERM_BYTES]) -> usize {
    usize::from(u16::from_le_bytes([term[11], term[12]]))
}

/// Jupiter's heliocentric longitude and latitude in radians and distance in
/// astronomical units, in the dynamical ecliptic and equinox of J2000.0, at
/// `millennia` Julian millennia from J2000.0 in dynamical time.
///
/// The longitude is not reduced to a turn: it is the sum the series gives,
/// about 0.6 rad plus 529.7 rad per millennium.
#[must_use]
pub fn jupiter_heliocentric(millennia: f64, truncation: Truncation) -> (f64, f64, f64) {
    let mut sums = [0.0; 3];
    let limit = truncation.limit() * SCALE;
    let mut reaches = [1.0; 6];
    let mut factors = [1.0; 6];
    for k in 1..6 {
        reaches[k] = reaches[k - 1] * abs(millennia);
        factors[k] = factors[k - 1] * millennia;
    }
    for (variable, power, packed) in BLOCKS {
        let reach = reaches[power as usize];
        let mut sum = 0.0;
        for term in packed.as_chunks::<TERM_BYTES>().0 {
            let amplitude = amplitude_of(term) as f64;
            if amplitude * reach < limit {
                continue;
            }
            let phase = phase_of(term) as f64 / SCALE;
            let frequency = FREQUENCIES[frequency_index_of(term)];
            sum += amplitude * cos(phase + frequency * millennia);
        }
        sums[usize::from(variable) - 1] += sum / SCALE * factors[power as usize];
    }
    (sums[0], sums[1], sums[2])
}

/// How many terms of the 3 625 a truncation keeps at `millennia`.
#[must_use]
pub fn terms_kept(millennia: f64, truncation: Truncation) -> usize {
    let limit = truncation.limit() * SCALE;
    let mut kept = 0;
    for (_, power, packed) in BLOCKS {
        let reach = powf(abs(millennia), f64::from(power));
        for term in packed.as_chunks::<TERM_BYTES>().0 {
            if amplitude_of(term) as f64 * reach >= limit {
                kept += 1;
            }
        }
    }
    kept
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_data_has_the_files_terms() {
        let total: usize = BLOCKS
            .iter()
            .map(|(_, _, packed)| packed.len() / TERM_BYTES)
            .sum();
        assert_eq!(total, SOURCE_TERMS);
        assert_eq!(terms_kept(0.0, Truncation::Full), SOURCE_TERMS);
        assert_eq!(SOURCE_TERMS, 3625);
        for (_, _, packed) in BLOCKS {
            assert_eq!(packed.len() % TERM_BYTES, 0);
        }
    }

    /// The file's first and last term, read by eye from the lines printed
    /// in `VSOP87B.jup`: L0 term 1 (0.59954691494, phase 0, frequency 0) and
    /// the last term of R5, 0.00000001033 cos(4.50671820436 + 529.69096509460 τ).
    #[test]
    fn the_first_and_the_last_term_are_the_files() {
        let (_, _, packed) = BLOCKS[0];
        let first = &packed.as_chunks::<TERM_BYTES>().0[0];
        assert_eq!(amplitude_of(first), 59_954_691_494);
        assert_eq!(phase_of(first), 0);
        assert_eq!(FREQUENCIES[frequency_index_of(first)], 0.0);
        let (variable, power, packed) = BLOCKS[17];
        assert_eq!((variable, power), (3, 5));
        let last = packed.as_chunks::<TERM_BYTES>().0.last().expect("terms");
        assert_eq!(amplitude_of(last), 1_033);
        assert_eq!(phase_of(last), 450_671_820_436);
        assert_eq!(FREQUENCIES[frequency_index_of(last)], 529.690_965_094_60);
    }
}
