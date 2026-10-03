//! Deep time: the spans an ordinary calendar cannot reach, in both
//! directions.
//!
//! [`hc_core::Duration`] is exact — `i128` seconds plus attoseconds — and it
//! covers everything a clock can measure. This crate covers what lies outside
//! that: below an attosecond, where the Planck time sits twenty-six decades
//! further down, and above the span where "a count of seconds" is a sensible
//! answer at all. Values out there are not exact. They are *published
//! magnitudes with error bars*, so that is what this crate stores.
//!
//! | Module | What it holds |
//! | --- | --- |
//! | [`magnitude`] | [`DeepTime`], a span in seconds as an uncertain magnitude, with logarithmic comparison and rendering |
//! | [`constants`] | The Planck units from CODATA 2022, each with its stated relative uncertainty |
//! | [`universe`] | The chronology of the universe as data, on Planck 2018 parameters |
//! | [`future`] | The far future as data, out to 10¹⁰⁰ years and past it |
//! | [`geologic`] | The ICS International Chronostratigraphic Chart as a queryable tree |
//! | [`names`] | The chart's interval names in fourteen other languages, from the ICS's own vocabulary, and the other tables' names in Japanese where an established term was read |
//! | [`archaeology`] | The `BP` convention, and the calibrated/uncalibrated distinction |
//! | [`evidence`] | The published claims to the earliest evidence of life, of *Homo sapiens* and of writing, each with the shape of its date |
//! | [`periods`] | Long astronomical recurrences: the precession of the equinoxes and the galactic year |
//! | [`timeline`] | All of the above, queried together |
//!
//! # The one rule
//!
//! Every numeric entry in every table carries an uncertainty and names a
//! source. A timeline that says the Hadean began 4.567 Ga with no error bar is
//! not a timeline, it is a decoration.
//!
//! ```
//! use hc_deep_time::{DeepTime, universe};
//!
//! let now = universe::AGE_OF_UNIVERSE.deep_time().unwrap();
//! let planck = DeepTime::from_planck_times(1.0, 0.0).unwrap();
//! // Sixty-one decades from the Planck time to the present.
//! assert!((now.orders_of_magnitude_between(planck).unwrap() - 60.9).abs() < 0.1);
//! ```
//!
//! # What this crate does not carry
//!
//! **Not carried: a cosmology solver.** The crate carries published values
//! and does arithmetic on them with the error bars intact. The integration
//! that produced the early cosmic ages was run once, offline, and
//! [`universe`] documents its inputs and its result; computing a value from
//! a cosmological model is not yet done.
//!
//! **Not carried: radiocarbon calibration.** Turning an uncalibrated
//! radiocarbon age into a calendar year needs the IntCal20 curve and its
//! marine and southern-hemisphere companions, none of which is carried.
//! [`archaeology`] models the *distinction* and refuses the conversion
//! rather than pretending the two are the same thing (policy §4).
//!
//! **Not carried: the archaeological periods of any region but Southwest
//! Asia and Europe.** The period table names its region in every entry, and
//! no other region's sequence has been added yet.
//!
//! Calendars are not here either. Everything here is a span in SI seconds,
//! or a count of years before a stated epoch, and a calendar year becomes a
//! fixed day in `hc-calendar`.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod archaeology;
pub mod constants;
pub mod error;
pub mod evidence;
pub mod future;
pub mod geologic;
pub mod magnitude;
pub mod names;
pub mod periods;
pub mod timeline;
pub mod universe;

pub use error::{DeepTimeError, DeepTimeResult};
pub use magnitude::{DeepTime, DeepUnit, LogMagnitude};

pub use archaeology::{ArchaeologicalPeriod, Bp, Calibration};
pub use constants::PhysicalConstant;
pub use evidence::{Dating, EarliestEvidence, EvidenceAge};
pub use future::{FutureEra, FutureEvent, Prediction};
pub use geologic::{GeologicInterval, GeologicRank};
pub use periods::{AstronomicalPeriod, GALACTIC_YEAR, Stability};
pub use timeline::{
    PRESENT_HORIZON_YEARS, Placement, place, place_megayears_ago, place_years_ago, span_between,
};
pub use universe::{CosmicEpoch, CosmicEvent};

#[cfg(test)]
mod identifier_tests {
    use crate::geologic::{self, GeologicRank};
    use crate::{archaeology, evidence, future, universe};

    /// Every identifier of every table, in table order.
    fn every_id() -> impl Iterator<Item = &'static str> {
        universe::EPOCHS
            .iter()
            .map(|entry| entry.id)
            .chain(universe::EVENTS.iter().map(|entry| entry.id))
            .chain(future::EVENTS.iter().map(|entry| entry.id))
            .chain(future::ERAS.iter().map(|entry| entry.id))
            .chain(
                GeologicRank::ALL
                    .iter()
                    .flat_map(|rank| geologic::intervals(*rank))
                    .map(|entry| entry.id),
            )
            .chain(archaeology::PERIODS.iter().map(|entry| entry.id))
            .chain(evidence::EVIDENCE.iter().map(|entry| entry.id))
    }

    /// Lower-case kebab: ASCII letters and digits in hyphen-separated runs.
    fn is_kebab(id: &str) -> bool {
        !id.is_empty()
            && id.split('-').all(|run| {
                !run.is_empty()
                    && run
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
            })
    }

    #[test]
    fn every_identifier_is_lower_case_kebab() {
        for id in every_id() {
            assert!(is_kebab(id), "{id}");
        }
        assert!(!is_kebab("Upper-Cretaceous") && !is_kebab("stage--10") && !is_kebab(""));
    }

    #[test]
    fn no_two_entries_of_any_table_share_an_identifier() {
        for (index, id) in every_id().enumerate() {
            assert!(
                !every_id().skip(index + 1).any(|other| other == id),
                "{id} is used twice"
            );
        }
    }

    #[test]
    fn a_geologic_identifier_is_its_chart_name_in_kebab_case() {
        for rank in GeologicRank::ALL {
            for interval in geologic::intervals(*rank) {
                let mut words = interval.name.split(' ');
                let mut parts = interval.id.split('-');
                assert!(
                    words
                        .by_ref()
                        .zip(parts.by_ref())
                        .all(|(word, part)| word.eq_ignore_ascii_case(part)),
                    "{}",
                    interval.id
                );
                assert!(
                    words.next().is_none() && parts.next().is_none(),
                    "{}",
                    interval.id
                );
            }
        }
        assert_eq!(
            geologic::by_id("cambrian-stage-10").map(|i| i.name),
            Some("Cambrian Stage 10")
        );
    }
}
