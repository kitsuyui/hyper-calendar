//! A C ABI for `hyper-calendar`.
//!
//! # The shape of this interface, and why
//!
//! A C boundary has three ways to go wrong, and each is closed deliberately
//! here.
//!
//! **Panics cannot cross it.** Unwinding across an `extern "C"` frame is
//! undefined behaviour. The workspace already denies `unwrap` and `expect`
//! outside tests, so the Rust side does not panic; every entry point here
//! additionally returns a status code rather than a value, so a failure is a
//! value the caller can see rather than a trap.
//!
//! **Nothing is allocated that the caller cannot free.** There are no
//! returned pointers and no opaque handles. Every function writes into
//! storage the caller owns: integers through out-parameters, text into a
//! caller-supplied buffer. That removes the whole class of "who frees this"
//! bugs, and it means the library has no global state to tear down.
//!
//! **Truncation is reported, not silent.** A text function that does not fit
//! returns [`HC_ERROR_BUFFER_TOO_SMALL`] and writes the required length, so
//! the caller can retry with a bigger buffer. It never writes a partial
//! answer and calls it success.
//!
//! # Status codes
//!
//! Every function returns [`HcStatus`]: zero for success, negative for
//! failure. The codes are stable and are the only error channel.
//!
//! ```c
//! int64_t rd;
//! if (hc_gregorian_to_fixed(2026, 9, 21, &rd) == HC_OK) {
//!     // rd == 739880
//! }
//! ```
//!
//! # Lines and cells
//!
//! Every entry point that answers with more than one value writes UTF-8
//! lines ending in `\n`, one per entry, with the cells of a line separated
//! by `\t`, NUL-terminated as a whole. The column order of each is stated
//! on the entry point and in the README; before 1.0 it may change, and the
//! pull request that changes it lists the change. A cell with nothing to
//! say is empty, and no cell contains a tab or a line break. The lines are the same lines the WebAssembly module
//! writes.
//!
//! # Layers
//!
//! The entry points come in layers, each a Cargo feature: `civil` (the
//! default), `timestamps`, `time-codes`, `calendars`, `holiday`, `seasons`,
//! `deep-time`, `tz`, `sky`, `orbital`, `jupiter`, `planetary`, `relativity`, `places`,
//! `humanize` and `zone-names`, with
//! `full` for all of them. Which feature each needs is in the README's table.
//!
//! Each layer is a module of its own. Most of its entry points are rows of
//! `hyper_calendar`'s table of the entry points this library shares with
//! the WebAssembly module, which the module expands with the marshalling
//! in `marshal`; the few whose shape is the library's own are written out
//! beside them.

#![allow(unsafe_code)]
#![warn(missing_docs)]

use core::ffi::{c_char, c_int};

/// The status returned by every entry point. Zero is success.
pub type HcStatus = c_int;

/// The call succeeded.
pub const HC_OK: HcStatus = 0;
/// A required pointer was null.
pub const HC_ERROR_NULL_POINTER: HcStatus = -1;
/// A field was outside its valid range.
pub const HC_ERROR_OUT_OF_RANGE: HcStatus = -2;
/// The date does not exist in the requested calendar.
pub const HC_ERROR_INVALID_DATE: HcStatus = -3;
/// Arithmetic left the representable range.
pub const HC_ERROR_OVERFLOW: HcStatus = -4;
/// The supplied buffer was too small; the required length, including the
/// terminating NUL, was written out.
pub const HC_ERROR_BUFFER_TOO_SMALL: HcStatus = -5;
/// The value lies outside the range where the requested model has data.
pub const HC_ERROR_NO_DATA: HcStatus = -6;
/// The requested calendar, table or identifier is not known.
pub const HC_ERROR_UNKNOWN: HcStatus = -7;
/// A string argument was not valid UTF-8.
pub const HC_ERROR_NOT_UTF8: HcStatus = -8;
/// Data was not in the format the call expects.
pub const HC_ERROR_MALFORMED: HcStatus = -9;

#[macro_use]
mod marshal;

/// The library version, as a NUL-terminated string.
///
/// Writes the required length, including the terminator, into `written`.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes and `written` must be null
/// or writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_version(
    buffer: *mut c_char,
    capacity: usize,
    written: *mut usize,
) -> HcStatus {
    // SAFETY: forwarded to the caller's contract above.
    unsafe { marshal::write_text(hc::VERSION, buffer, capacity, written) }
}

#[cfg(feature = "civil")]
mod civil;
#[cfg(feature = "civil")]
pub use civil::*;

#[cfg(feature = "timestamps")]
mod timestamps;
#[cfg(feature = "timestamps")]
pub use timestamps::*;

#[cfg(feature = "time-codes")]
mod time_codes;
#[cfg(feature = "time-codes")]
pub use time_codes::*;

#[cfg(feature = "calendars")]
mod calendars;
#[cfg(feature = "calendars")]
pub use calendars::*;

#[cfg(feature = "seasons")]
mod seasons;
#[cfg(feature = "seasons")]
pub use seasons::*;

#[cfg(feature = "holiday")]
mod holiday;
#[cfg(feature = "holiday")]
pub use holiday::*;

#[cfg(feature = "deep-time")]
mod deep_time;
#[cfg(feature = "deep-time")]
pub use deep_time::*;

#[cfg(feature = "tz")]
mod tz;
#[cfg(feature = "tz")]
pub use tz::*;

#[cfg(feature = "sky")]
mod sky;
#[cfg(feature = "sky")]
pub use sky::*;

#[cfg(feature = "orbital")]
mod orbital;
#[cfg(feature = "orbital")]
pub use orbital::*;

#[cfg(feature = "jupiter")]
mod jupiter;
#[cfg(feature = "jupiter")]
pub use jupiter::*;

#[cfg(feature = "planetary")]
mod planetary;
#[cfg(feature = "planetary")]
pub use planetary::*;

#[cfg(feature = "relativity")]
mod relativity;
#[cfg(feature = "relativity")]
pub use relativity::*;

#[cfg(feature = "places")]
mod places;
#[cfg(feature = "places")]
pub use places::*;

#[cfg(feature = "humanize")]
mod humanize;
#[cfg(feature = "humanize")]
pub use humanize::*;

#[cfg(feature = "zone-names")]
mod zone_names;
#[cfg(feature = "zone-names")]
pub use zone_names::*;

#[cfg(test)]
mod tests;
