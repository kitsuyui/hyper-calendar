//! Musical time: tempo, note values, bars and MIDI ticks.
//!
//! # Why a calendar library has a tempo module
//!
//! Because BPM is a unit of time and nothing else. "120 BPM" says a beat is
//! exactly half a second, the same way "25 fps" says a frame is exactly a
//! fortieth of one. A sequencer that stores a beat as a float and a
//! calendar that stores a month as a float make the same mistake, and this
//! crate already has the machinery not to.
//!
//! The arithmetic is exact throughout. A dotted quarter triplet at 138 BPM
//! is 10/23 of a second — a number no decimal writes down, and one that a
//! sequencer must get right a thousand times a minute without drifting.

use crate::error::{UnitError, UnitResult};
use crate::ratio::Ratio;
use crate::unit::{Quantity, SECOND, Unit};

/// A tempo, in beats per minute.
///
/// Held as an exact [`Ratio`] so that a tempo read from a MIDI file —
/// which stores microseconds per quarter note, an integer that rarely
/// inverts to a round BPM — survives the round trip unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tempo {
    bpm: Ratio,
}

impl Tempo {
    /// A tempo from a whole number of beats per minute.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] if the tempo is zero.
    pub const fn from_bpm(bpm: i128) -> UnitResult<Self> {
        Self::from_bpm_ratio(Ratio::from_secs(bpm))
    }

    /// A tempo from an exact fractional BPM.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] if the tempo is zero or negative.
    pub const fn from_bpm_ratio(bpm: Ratio) -> UnitResult<Self> {
        if bpm.is_zero() || bpm.is_negative() {
            return Err(UnitError::DivideByZero);
        }
        Ok(Self { bpm })
    }

    /// The tempo in beats per minute.
    #[must_use]
    pub const fn bpm(self) -> Ratio {
        self.bpm
    }

    /// The exact length of one beat, in seconds.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the division leaves `i128`.
    pub const fn beat(self) -> UnitResult<Ratio> {
        Ratio::from_secs(60).checked_div(self.bpm)
    }

    /// One beat, counted in some unit.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the conversion leaves `i128`.
    pub const fn beat_in(self, unit: Unit) -> UnitResult<Quantity> {
        let Ok(beat) = self.beat() else {
            return Err(UnitError::Overflow);
        };
        Quantity::new(beat, SECOND).to(unit)
    }

    /// The tempo a MIDI `Set Tempo` meta event encodes.
    ///
    /// The event carries a 24-bit count of microseconds per quarter note;
    /// the default, 500 000, is 120 BPM.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] if the count is zero.
    pub const fn from_midi_micros_per_beat(micros: u32) -> UnitResult<Self> {
        let Ok(bpm) = Ratio::new(60_000_000, micros as i128) else {
            return Err(UnitError::DivideByZero);
        };
        Self::from_bpm_ratio(bpm)
    }

    /// The microseconds-per-quarter-note a MIDI file would store, and
    /// whether it is exact.
    ///
    /// MIDI's count is an integer, so most tempos do not survive: 140 BPM is
    /// 428 571.428… µs. The `bool` says whether this one did, rather than
    /// leaving the caller to discover the drift in bar 400.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the count does not fit in 24 bits, which
    /// happens below about 3.6 BPM.
    pub const fn to_midi_micros_per_beat(self) -> UnitResult<(u32, bool)> {
        let Ok(micros) = Ratio::from_secs(60_000_000).checked_div(self.bpm) else {
            return Err(UnitError::Overflow);
        };
        let truncated = micros.trunc();
        if truncated <= 0 || truncated > 0x00FF_FFFF {
            return Err(UnitError::Overflow);
        }
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "bounded to 1..=0xFFFFFF on the line above"
        )]
        let value = truncated as u32;
        Ok((value, micros.is_integer()))
    }
}

/// A note value: a power-of-two division of a whole note, with dots and an
/// optional tuplet.
///
/// Every part of that is exact. A dot multiplies by 3/2, two dots by 7/4,
/// *n* dots by (2^(n+1) − 1)/2^n; a triplet multiplies by 2/3; and a
/// quintuplet in the space of four by 4/5. None of those are representable
/// in binary floating point, and all of them are representable here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoteValue {
    /// How many times the whole note is halved: 0 whole, 1 half, 2 quarter,
    /// 3 eighth, and so on.
    pub halvings: u8,
    /// Augmentation dots.
    pub dots: u8,
    /// A tuplet as (in the time of, this many): a triplet is `(2, 3)`.
    pub tuplet: Option<(u32, u32)>,
}

impl NoteValue {
    /// A plain note value with no dots and no tuplet.
    #[must_use]
    pub const fn new(halvings: u8) -> Self {
        Self {
            halvings,
            dots: 0,
            tuplet: None,
        }
    }

    /// A whole note (semibreve).
    pub const WHOLE: Self = Self::new(0);
    /// A half note (minim).
    pub const HALF: Self = Self::new(1);
    /// A quarter note (crotchet), the usual beat.
    pub const QUARTER: Self = Self::new(2);
    /// An eighth note (quaver).
    pub const EIGHTH: Self = Self::new(3);
    /// A sixteenth note (semiquaver).
    pub const SIXTEENTH: Self = Self::new(4);
    /// A thirty-second note (demisemiquaver).
    pub const THIRTY_SECOND: Self = Self::new(5);

    /// The same value with `dots` augmentation dots.
    #[must_use]
    pub const fn dotted(self, dots: u8) -> Self {
        Self { dots, ..self }
    }

    /// The same value inside a tuplet of `count` in the time of `space`.
    #[must_use]
    pub const fn tuplet(self, space: u32, count: u32) -> Self {
        Self {
            tuplet: Some((space, count)),
            ..self
        }
    }

    /// This note's length as a fraction of a whole note.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] beyond about 120 halvings, and
    /// [`UnitError::DivideByZero`] for a tuplet with a zero on either side.
    pub const fn fraction_of_whole(self) -> UnitResult<Ratio> {
        if self.halvings >= 127 || self.dots >= 127 {
            return Err(UnitError::Overflow);
        }
        let Some(denominator) = 1_i128.checked_shl(self.halvings as u32) else {
            return Err(UnitError::Overflow);
        };
        let Ok(base) = Ratio::new(1, denominator) else {
            return Err(UnitError::DivideByZero);
        };
        // n dots multiply by (2^(n+1) - 1) / 2^n.
        let Some(dot_den) = 1_i128.checked_shl(self.dots as u32) else {
            return Err(UnitError::Overflow);
        };
        let Some(doubled) = dot_den.checked_mul(2) else {
            return Err(UnitError::Overflow);
        };
        let Ok(dot_factor) = Ratio::new(doubled - 1, dot_den) else {
            return Err(UnitError::DivideByZero);
        };
        let Ok(dotted) = base.checked_mul(dot_factor) else {
            return Err(UnitError::Overflow);
        };
        match self.tuplet {
            None => Ok(dotted),
            Some((space, count)) => {
                let Ok(factor) = Ratio::new(space as i128, count as i128) else {
                    return Err(UnitError::DivideByZero);
                };
                dotted.checked_mul(factor)
            }
        }
    }

    /// How long this note lasts at `tempo`, where the beat is `beat_unit`.
    ///
    /// The beat unit matters: 120 BPM in 6/8 conventionally counts dotted
    /// quarters, not quarters, and the same note is then a different length.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the arithmetic leaves `i128`.
    pub const fn duration_at(self, tempo: Tempo, beat_unit: Self) -> UnitResult<Ratio> {
        let Ok(mine) = self.fraction_of_whole() else {
            return Err(UnitError::Overflow);
        };
        let Ok(theirs) = beat_unit.fraction_of_whole() else {
            return Err(UnitError::Overflow);
        };
        let Ok(beats) = mine.checked_div(theirs) else {
            return Err(UnitError::Overflow);
        };
        let Ok(beat) = tempo.beat() else {
            return Err(UnitError::Overflow);
        };
        beats.checked_mul(beat)
    }
}

/// A time signature, and the bar length it implies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSignature {
    /// The upper number: beats to a bar.
    pub beats: u32,
    /// The lower number, as a note value: 4 is [`NoteValue::QUARTER`].
    pub beat_value: NoteValue,
}

impl TimeSignature {
    /// A time signature from its two numbers, the lower being a power of two.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] if either number is zero or the lower is
    /// not a power of two.
    pub const fn new(beats: u32, lower: u32) -> UnitResult<Self> {
        if beats == 0 || lower == 0 || !lower.is_power_of_two() {
            return Err(UnitError::DivideByZero);
        }
        Ok(Self {
            beats,
            beat_value: NoteValue::new(lower.trailing_zeros() as u8),
        })
    }

    /// How long one bar lasts at `tempo`, counting the tempo's beat as
    /// [`TimeSignature::beat_value`].
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the arithmetic leaves `i128`.
    pub const fn bar(self, tempo: Tempo) -> UnitResult<Ratio> {
        let Ok(beat) = tempo.beat() else {
            return Err(UnitError::Overflow);
        };
        beat.checked_mul_int(self.beats as i128)
    }
}

/// A MIDI or sequencer tick grid, in pulses per quarter note.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ppqn(pub u32);

impl Ppqn {
    /// 96 PPQN: the original MIDI File Format 0 default.
    pub const CLASSIC: Self = Self(96);
    /// 480 PPQN: the common modern DAW grid.
    pub const MODERN: Self = Self(480);
    /// 960 PPQN.
    pub const FINE: Self = Self(960);

    /// The exact length of one tick at `tempo`.
    ///
    /// # Errors
    ///
    /// [`UnitError::DivideByZero`] for a zero grid.
    pub const fn tick(self, tempo: Tempo) -> UnitResult<Ratio> {
        let Ok(beat) = tempo.beat() else {
            return Err(UnitError::Overflow);
        };
        let Ok(divisor) = Ratio::new(self.0 as i128, 1) else {
            return Err(UnitError::DivideByZero);
        };
        beat.checked_div(divisor)
    }
}
