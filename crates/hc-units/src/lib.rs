//! Exactly defined units of time.
//!
//! A time library is asked two different questions that look like one.
//! "How long is a minute?" has an answer a standards body wrote down.
//! "How long is a year?" does not — it has several, depending on whether
//! you mean the Julian year astronomy counts in, the mean Gregorian year a
//! civil calendar averages to, or the tropical year the Earth actually
//! takes, which is a measurement with an error bar and a slow drift.
//!
//! This crate answers only the first kind. Every unit in it is *defined* as
//! a fixed multiple of the SI second by some authority, so every one of them
//! is an exact rational and conversions between them compose without losing
//! anything. Units that are measured rather than defined — the sidereal day,
//! the tropical year, the synodic month, the galactic year — live with the
//! model that measured them, in `hc-astro`, `hc-planetary` and
//! `hc-deep-time`, where their uncertainties are attached to them.
//!
//! * [`ratio`] — exact rational seconds, and the explicit, refusable step
//!   down to [`hc_core::Duration`].
//! * [`mod@unit`] — the catalogue: SI prefixes, civil units, the Hebrew helek,
//!   the Chinese 刻 in both of its lengths, the Indian ghati, French
//!   Republican decimal time, Nystrom's hexadecimal time, the flick, the
//!   shake and the microfortnight.
//! * [`media`] — frame rates and sample rates, including the NTSC 1000/1001
//!   pull-down, as exact periods.
//! * [`tempo`] — BPM, note values with dots and tuplets, bars and MIDI ticks.
//!
//! # Anchors
//!
//! Each of these is a test in this crate:
//!
//! | Claim | Value |
//! | --- | --- |
//! | A flick divides every frame and sample rate in [`media`] | exactly |
//! | A helek | 3⅓ s |
//! | Swatch `.beat` and the French decimal minute | both 86.4 s |
//! | 120 BPM | a beat of exactly 0.5 s |
//! | MIDI's default 500 000 µs per quarter | 120 BPM, exactly |
//! | A flick as a `Duration` | refused, not rounded |
//!
//! # What this crate is not
//!
//! It is not a general dimensional-analysis library. There is one dimension
//! here, and it is time. Nor does it convert a count of units into a date:
//! "three months from now" is a calendar question, because the answer
//! depends on which month you start in. That is `hc-calendar`'s to answer,
//! and [`unit::MEAN_GREGORIAN_MONTH`] is deliberately named for the average
//! it is, so that nobody reaches for it by accident.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod media;
pub mod ratio;
pub mod tempo;
pub mod unit;

pub use error::{UnitError, UnitResult};
pub use media::Rate;
pub use ratio::Ratio;
pub use tempo::{NoteValue, Ppqn, Tempo, TimeSignature};
pub use unit::{Family, Quantity, Unit, by_id};

pub use hc_core;

#[cfg(test)]
mod tests {
    use super::media::{FRAME_RATES, SAMPLE_RATES};
    use super::tempo::{NoteValue, Ppqn, Tempo, TimeSignature};
    use super::unit::{
        BEAT, DAY, DECIMAL_MINUTE, FLICK, HELEK, HOUR, JULIAN_YEAR, KE_HUNDRED, KE_NINETY_SIX,
        MILLISECOND, Quantity, SECOND,
    };
    use super::{Ratio, UnitError};

    #[test]
    fn a_flick_divides_every_frame_and_sample_rate_exactly() {
        // This is the whole reason the unit exists, so it is the first test.
        for rate in FRAME_RATES.iter().chain(SAMPLE_RATES) {
            let one = rate
                .one_in(FLICK)
                .unwrap_or_else(|_| panic!("{} should convert to flicks", rate.id));
            assert!(
                one.count.is_integer(),
                "one event at {} is {} flicks, which is not whole",
                rate.id,
                one.count
            );
        }
    }

    #[test]
    fn the_ntsc_pulldown_stays_exact() {
        let pulled = super::media::NTSC_30
            .ntsc_pulldown("29.97")
            .expect("the pulldown should not overflow");
        assert_eq!(pulled.hertz, super::media::NTSC_29_97.hertz);
        assert_eq!(pulled.hertz.numerator(), 30_000);
        assert_eq!(pulled.hertz.denominator(), 1_001);
    }

    #[test]
    fn a_flick_has_no_exact_duration_and_says_so() {
        assert_eq!(FLICK.seconds.to_duration(), Err(UnitError::Inexact));
        let rounded = FLICK
            .seconds
            .to_duration_rounded()
            .expect("rounding should succeed");
        // 10^18 / 705_600_000 = 1_417_233_560.0907…, so nearest is …560.
        assert_eq!(rounded.subsec_attos(), 1_417_233_560);
    }

    #[test]
    fn a_helek_is_three_and_a_third_seconds() {
        assert_eq!(HELEK.seconds, Ratio::literal(10, 3));
        let per_hour = HOUR.per(HELEK).expect("the ratio should be exact");
        assert_eq!(per_hour, Ratio::from_secs(1_080));
    }

    #[test]
    fn the_two_ke_are_separate_units_of_different_length() {
        // Policy §5: competing conventions get names, not a parameter.
        assert_eq!(DAY.per(KE_HUNDRED), Ok(Ratio::from_secs(100)));
        assert_eq!(DAY.per(KE_NINETY_SIX), Ok(Ratio::from_secs(96)));
        assert_ne!(KE_HUNDRED.seconds, KE_NINETY_SIX.seconds);
    }

    #[test]
    fn a_swatch_beat_is_a_french_decimal_minute() {
        assert_eq!(BEAT.seconds, DECIMAL_MINUTE.seconds);
        assert_eq!(DAY.per(BEAT), Ok(Ratio::from_secs(1_000)));
    }

    #[test]
    fn a_julian_year_is_three_hundred_sixty_five_and_a_quarter_days() {
        assert_eq!(JULIAN_YEAR.per(DAY), Ok(Ratio::literal(1_461, 4)));
    }

    #[test]
    fn converting_a_quantity_keeps_the_length() {
        let two_hours = Quantity::whole(2, HOUR);
        let in_millis = two_hours.to(MILLISECOND).expect("should convert");
        assert_eq!(in_millis.count, Ratio::from_secs(7_200_000));
        assert_eq!(in_millis.seconds(), two_hours.seconds());
    }

    #[test]
    fn one_hundred_twenty_bpm_is_half_a_second_a_beat() {
        let tempo = Tempo::from_bpm(120).expect("120 BPM is a tempo");
        assert_eq!(tempo.beat(), Ok(Ratio::literal(1, 2)));
    }

    #[test]
    fn the_midi_default_tempo_round_trips() {
        let tempo = Tempo::from_midi_micros_per_beat(500_000).expect("the default is a tempo");
        assert_eq!(tempo.bpm(), Ratio::from_secs(120));
        assert_eq!(tempo.to_midi_micros_per_beat(), Ok((500_000, true)));
    }

    #[test]
    fn a_tempo_midi_cannot_store_exactly_admits_it() {
        // 60_000_000 / 140 = 428_571.428…, so the stored value is short.
        let tempo = Tempo::from_bpm(140).expect("140 BPM is a tempo");
        let (micros, exact) = tempo
            .to_midi_micros_per_beat()
            .expect("140 BPM fits in 24 bits");
        assert_eq!(micros, 428_571);
        assert!(!exact, "140 BPM should not be storable exactly");
    }

    #[test]
    fn dots_and_tuplets_are_exact() {
        // A dotted quarter is 3/8 of a whole note.
        assert_eq!(
            NoteValue::QUARTER.dotted(1).fraction_of_whole(),
            Ok(Ratio::literal(3, 8))
        );
        // Double-dotted is 7/16.
        assert_eq!(
            NoteValue::QUARTER.dotted(2).fraction_of_whole(),
            Ok(Ratio::literal(7, 16))
        );
        // An eighth-note triplet is 1/12.
        assert_eq!(
            NoteValue::EIGHTH.tuplet(2, 3).fraction_of_whole(),
            Ok(Ratio::literal(1, 12))
        );
    }

    #[test]
    fn a_dotted_quarter_triplet_at_one_hundred_thirty_eight_bpm() {
        let tempo = Tempo::from_bpm(138).expect("138 BPM is a tempo");
        let note = NoteValue::QUARTER.dotted(1).tuplet(2, 3);
        // 3/8 × 2/3 = 1/4 of a whole = one beat; one beat at 138 is 10/23 s.
        assert_eq!(
            note.duration_at(tempo, NoteValue::QUARTER),
            Ok(Ratio::literal(10, 23))
        );
    }

    #[test]
    fn a_bar_of_four_four_is_four_beats() {
        let tempo = Tempo::from_bpm(90).expect("90 BPM is a tempo");
        let signature = TimeSignature::new(4, 4).expect("4/4 is a signature");
        assert_eq!(signature.bar(tempo), Ok(Ratio::literal(8, 3)));
    }

    #[test]
    fn a_time_signature_needs_a_power_of_two_below_the_line() {
        assert_eq!(TimeSignature::new(4, 5), Err(UnitError::DivideByZero));
        assert!(TimeSignature::new(7, 8).is_ok());
    }

    #[test]
    fn a_midi_tick_is_exact_at_the_default_grid() {
        let tempo = Tempo::from_bpm(120).expect("120 BPM is a tempo");
        assert_eq!(Ppqn::MODERN.tick(tempo), Ok(Ratio::literal(1, 960)));
    }

    #[test]
    fn a_frame_does_not_always_land_on_a_sample() {
        let frame = super::media::NTSC_29_97
            .one_in(SECOND)
            .expect("a frame is a length");
        assert!(
            !frame.divides_into(super::unit::MILLISECOND),
            "a 29.97 fps frame is not a whole number of milliseconds"
        );
        assert!(
            frame.divides_into(FLICK),
            "but it is a whole number of flicks"
        );
    }

    #[test]
    fn a_ratio_comparison_that_overflows_cross_multiplication_still_orders() {
        let tiny = super::unit::QUECTOSECOND.seconds;
        let huge = super::unit::JULIAN_MILLENNIUM.seconds;
        // 10^30 × 31_557_600_000 is 3.2×10^40, past i128, so this exercises
        // the f64 fallback in `Ord`.
        assert!(tiny < huge);
        assert!(huge > tiny);
    }

    #[test]
    fn a_duration_round_trips_through_a_ratio() {
        let duration = hc_core::Duration::new(1_234, 567_000_000_000_000_000)
            .expect("the duration is in range");
        let ratio = Ratio::from_duration(duration).expect("it fits");
        assert_eq!(ratio.to_duration(), Ok(duration));
    }
}
