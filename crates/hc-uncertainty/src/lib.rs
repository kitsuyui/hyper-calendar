//! Time that is not exactly known.
//!
//! Almost nothing outside a laboratory has an exact timestamp. A charter is
//! dated "in the third year of the king"; a radiocarbon sample comes back as
//! `3200 ± 50 BP`; a catalogue says `1667 or 1668`; the universe is 13.8
//! billion years old, which is three digits and not eleven. A date library
//! that can only hold exact instants has to either refuse these or invent the
//! missing precision, and inventing it is worse.
//!
//! This crate provides the four vocabularies needed to hold them honestly:
//!
//! * [`Significant`] — how many digits a number actually claims, propagated
//!   through arithmetic and, crucially, through rendering.
//! * [`Uncertain`] — a Gaussian `value ± σ`, with first-order propagation and
//!   the inverse-variance weighted mean used to merge published values.
//! * [`DurationInterval`] — a guaranteed enclosure, with the interval
//!   arithmetic that comes with it and the dependency problem that does too.
//! * [`FuzzyInstant`] — an instant known exactly, to a resolution, within
//!   bounds, as a Gaussian, one-sidedly, or not at all, with Allen's interval
//!   algebra computed over pairs of them.
//!
//! and one interchange format:
//!
//! * [`edtf`] — ISO 8601-2 Extended Date/Time Format levels 0 to 2, the
//!   standard vocabulary libraries and museums use for exactly this.
//!
//! # What this crate deliberately does not do
//!
//! It does not know about calendars. Everything here is expressed against
//! [`hc_core::Instant`] and [`hc_core::Duration`]; turning "the third century
//! BC" into a pair of instants is the job of a calendar crate, and the one
//! small exception — EDTF needs proleptic Gregorian day arithmetic to parse
//! `1984-01-01` at all — is documented in [`edtf`] and kept private.
//!
//! It also does not do Monte Carlo. [`Uncertain`] is a linear approximation
//! and says so; when the relative uncertainty is large enough for that to
//! matter, the honest answer is a distribution, not a wider `±`.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod edtf;
pub mod error;
pub mod fuzzy;
pub mod interval;
pub mod quantity;
pub mod sig_figs;

pub use error::{UncertaintyError, UncertaintyResult};
pub use fuzzy::{AllenRelation, DEFAULT_SIGMA_ENVELOPE, FuzzyInstant, RelationSet, Support};
pub use interval::DurationInterval;
pub use quantity::Uncertain;
pub use sig_figs::{MAX_FIGURES, Significant};

pub use edtf::{
    EdtfComponent, EdtfDate, EdtfEndpoint, EdtfPrecision, EdtfQualifier, EdtfSetMember, EdtfValue,
};

pub use hc_core;
