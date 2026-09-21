//! Frame rates and sample rates, as exact rational periods.
//!
//! # The 1000/1001 problem
//!
//! When NTSC added colour in 1953 the subcarrier had to avoid beating
//! against the audio carrier, so the field rate was pulled down by a factor
//! of exactly 1000/1001 — 60 Hz became 59.94005994… Hz. Seventy years later
//! every film and broadcast pipeline still carries that factor, and every
//! one that stores a frame period as a float accumulates drift from it.
//!
//! So a rate here is a [`Ratio`] of frames per second, never a float, and
//! its period is the exact reciprocal. 24 fps is 1/24 s; 23.976 fps is
//! 1001/24000 s, which is not 0.0417083333… but exactly itself.

use crate::error::{UnitError, UnitResult};
use crate::ratio::Ratio;
use crate::unit::{Quantity, Unit};

/// A rate in events per second, held exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rate {
    /// A stable identifier.
    pub id: &'static str,
    /// Events per second, exactly.
    pub hertz: Ratio,
}

impl Rate {
    /// A rate from an exact fraction of events per second.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] for a zero denominator.
    pub const fn new(id: &'static str, num: i128, den: i128) -> UnitResult<Self> {
        let Ok(hertz) = Ratio::new(num, den) else {
            return Err(UnitError::DivideByZero);
        };
        Ok(Self { id, hertz })
    }

    /// The length of one event, in seconds.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] if the rate is zero.
    pub const fn period(self) -> UnitResult<Ratio> {
        self.hertz.checked_recip()
    }

    /// One event, as a quantity in some unit.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] for a zero rate, [`UnitError::Overflow`]
    /// if the conversion leaves `i128`.
    pub const fn one_in(self, unit: Unit) -> UnitResult<Quantity> {
        let Ok(period) = self.period() else {
            return Err(UnitError::DivideByZero);
        };
        Quantity::new(period, crate::unit::SECOND).to(unit)
    }

    /// The same rate pulled down by 1000/1001, the NTSC colour factor.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the numerator leaves `i128`.
    pub const fn ntsc_pulldown(self, id: &'static str) -> UnitResult<Self> {
        let Ok(factor) = Ratio::new(1_000, 1_001) else {
            return Err(UnitError::DivideByZero);
        };
        let Ok(hertz) = self.hertz.checked_mul(factor) else {
            return Err(UnitError::Overflow);
        };
        Ok(Self { id, hertz })
    }
}

/// Shorthand for the tables below.
const fn rate(id: &'static str, num: i128, den: i128) -> Rate {
    Rate {
        id,
        hertz: Ratio::literal(num, den),
    }
}

/// 24 fps: the sound-film rate since 1929.
pub const FILM_24: Rate = rate("24", 24, 1);
/// 25 fps: PAL and SECAM television, and EBU film transfer.
pub const PAL_25: Rate = rate("25", 25, 1);
/// 30 fps: monochrome NTSC, and modern video that has escaped the pulldown.
pub const NTSC_30: Rate = rate("30", 30, 1);
/// 48 fps.
pub const HFR_48: Rate = rate("48", 48, 1);
/// 50 fps: PAL-derived high frame rate.
pub const PAL_50: Rate = rate("50", 50, 1);
/// 60 fps.
pub const HFR_60: Rate = rate("60", 60, 1);
/// 90 fps: the rate VR headsets target.
pub const VR_90: Rate = rate("90", 90, 1);
/// 100 fps.
pub const HFR_100: Rate = rate("100", 100, 1);
/// 120 fps.
pub const HFR_120: Rate = rate("120", 120, 1);
/// 24000/1001 fps, written 23.976: film transferred to NTSC video.
pub const NTSC_23_976: Rate = rate("23.976", 24_000, 1_001);
/// 30000/1001 fps, written 29.97: colour NTSC.
pub const NTSC_29_97: Rate = rate("29.97", 30_000, 1_001);
/// 60000/1001 fps, written 59.94.
pub const NTSC_59_94: Rate = rate("59.94", 60_000, 1_001);

/// Every frame rate above.
pub const FRAME_RATES: &[Rate] = &[
    NTSC_23_976,
    FILM_24,
    PAL_25,
    NTSC_29_97,
    NTSC_30,
    HFR_48,
    PAL_50,
    NTSC_59_94,
    HFR_60,
    VR_90,
    HFR_100,
    HFR_120,
];

/// 8 kHz: telephony.
pub const AUDIO_8K: Rate = rate("8000", 8_000, 1);
/// 16 kHz: wideband speech.
pub const AUDIO_16K: Rate = rate("16000", 16_000, 1);
/// 22.05 kHz: half of the CD rate.
pub const AUDIO_22_05K: Rate = rate("22050", 22_050, 1);
/// 24 kHz: half of 48 kHz.
pub const AUDIO_24K: Rate = rate("24000", 24_000, 1);
/// 32 kHz: broadcast.
pub const AUDIO_32K: Rate = rate("32000", 32_000, 1);
/// 44.1 kHz: the Compact Disc rate.
pub const AUDIO_44_1K: Rate = rate("44100", 44_100, 1);
/// 48 kHz: the professional and video rate.
pub const AUDIO_48K: Rate = rate("48000", 48_000, 1);
/// 88.2 kHz.
pub const AUDIO_88_2K: Rate = rate("88200", 88_200, 1);
/// 96 kHz.
pub const AUDIO_96K: Rate = rate("96000", 96_000, 1);
/// 192 kHz.
pub const AUDIO_192K: Rate = rate("192000", 192_000, 1);

/// Every sample rate above.
pub const SAMPLE_RATES: &[Rate] = &[
    AUDIO_8K,
    AUDIO_16K,
    AUDIO_22_05K,
    AUDIO_24K,
    AUDIO_32K,
    AUDIO_44_1K,
    AUDIO_48K,
    AUDIO_88_2K,
    AUDIO_96K,
    AUDIO_192K,
];
