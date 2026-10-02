//! A WebAssembly surface for `hyper-calendar`.
//!
//! # Why this is a raw ABI and not `wasm-bindgen`
//!
//! The workspace has no external dependencies ([ADR
//! 0005](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/adr/0005-no-external-dependencies.md)),
//! and that is worth more here than anywhere else. A calendar library is a
//! leaf dependency of a web application; whatever it drags in, the bundle
//! carries. So the exports below are plain `extern "C"` functions over
//! integers and linear memory, which every WebAssembly host can call with no
//! glue at all.
//!
//! The cost is that the JavaScript side does the string marshalling. That is
//! a few lines, shown below, and they are lines the caller can read; written
//! once for every export, they are the binding in `js/hyper-calendar.js`
//! beside this crate, which the README describes.
//!
//! # Memory
//!
//! The module owns its allocator. [`hc_alloc`] hands out a block, the caller
//! writes into it or reads out of it, and [`hc_free`] takes it back. Every
//! block must be freed with the same length it was allocated with, which is
//! the length of the boxed slice the block was made as.
//!
//! Text is UTF-8 and is *not* NUL-terminated: functions that produce it
//! return the byte length, because a length is cheaper and safer than a scan.
//!
//! ```js
//! const { instance } = await WebAssembly.instantiateStreaming(
//!   fetch("hyper_calendar_wasm.wasm"),
//! );
//! const wasm = instance.exports;
//!
//! // 2026-09-21 as a fixed day number. i64 arrives as a BigInt.
//! const rd = wasm.hc_gregorian_to_fixed(2026n, 9, 21);
//!
//! // Read it back as an ISO 8601 date.
//! const cap = 32;
//! const ptr = wasm.hc_alloc(cap);
//! const len = Number(wasm.hc_format_iso_date(rd, ptr, cap));
//! const bytes = new Uint8Array(wasm.memory.buffer, ptr, len);
//! const text = new TextDecoder().decode(bytes);
//! wasm.hc_free(ptr, cap);
//! ```
//!
//! # Errors
//!
//! Functions returning a day number or a count return a negative sentinel on
//! failure rather than trapping, because a trap tears down the instance and
//! takes any other work in it with it. The sentinels are the `HC_ERR_*`
//! constants, all at or below [`HC_ERR_FLOOR`], and the rule is that a
//! value-returning export never returns a number at or below
//! [`HC_ERR_FLOOR`] except as an error. A day number never comes near it:
//! [`HC_ERR_FLOOR`] is more than a thousand times the age of the universe
//! in days. A count of seconds can, about 285 million years back, so the
//! exports that answer in seconds refuse such a day with
//! [`HC_ERR_OUT_OF_RANGE`] rather than return a number every binding would
//! read as a sentinel, and refuse the same way a result that would
//! overflow an `i64`. The README states the range each export answers for.
//!
//! # Lines and cells
//!
//! Every export that answers with more than one value writes UTF-8 lines
//! ending in `\n`, one per entry, with the cells of a line separated by
//! `\t`. The column order of each is stated on the export and in the
//! README; before 1.0 it may change, and the pull request that changes it
//! lists the change. A cell that has nothing to say is
//! empty, never a placeholder, and a cell's text never contains a tab or a
//! line break: the few source strings that do have them replaced by a
//! space. Called with a null `buffer`, such an export returns the byte
//! length the text needs, so the caller can allocate exactly and call
//! again.
//!
//! # Layers
//!
//! The exports come in layers that a page can load as it needs them, each a
//! Cargo feature: `civil` (the default), `timestamps`, `time-codes`, `calendars`, `holiday`, `seasons`,
//! `deep-time`, `tz`, `sky`, `orbital`, `jupiter`, `planetary`, `relativity`, `places`,
//! `humanize` and `zone-names`, with
//! `full` for all of them.
//! Which feature each export needs is in the README's table.
//!
//! Each layer is a module of its own. Most of its exports are rows of
//! `hyper_calendar`'s table of the exports this module shares with the C
//! library, which the module expands with the marshalling in `marshal`;
//! the few whose shape is the module's own are written out beside them.

#![allow(unsafe_code)]
#![warn(missing_docs)]

/// Any return value at or below this is an error sentinel, not a result.
///
/// The age of the universe is about 5 × 10¹² days, so no legitimate fixed day
/// number comes anywhere near this. A count of seconds does, about 285
/// million years back, and an export that answers in seconds returns
/// [`HC_ERR_OUT_OF_RANGE`] for a result that would reach it.
pub const HC_ERR_FLOOR: i64 = -9_000_000_000_000_000;

/// The date does not exist.
pub const HC_ERR_INVALID_DATE: i64 = -9_000_000_000_000_001;
/// A value was outside the supported range.
pub const HC_ERR_OUT_OF_RANGE: i64 = -9_000_000_000_000_002;
/// The supplied buffer was too small.
pub const HC_ERR_BUFFER_TOO_SMALL: i64 = -9_000_000_000_000_003;
/// The requested model has no data for this value.
pub const HC_ERR_NO_DATA: i64 = -9_000_000_000_000_004;
/// A pointer was null with a non-zero length.
pub const HC_ERR_NULL_POINTER: i64 = -9_000_000_000_000_005;
/// The requested table or identifier is not known.
pub const HC_ERR_UNKNOWN: i64 = -9_000_000_000_000_006;
/// Text was not valid UTF-8.
pub const HC_ERR_NOT_UTF8: i64 = -9_000_000_000_000_007;
/// Data was not in the format the call expects.
pub const HC_ERR_MALFORMED: i64 = -9_000_000_000_000_008;

#[macro_use]
mod marshal;

/// Allocate `len` bytes of linear memory and return a pointer to them.
///
/// Returns null when `len` is zero or the allocation fails. Free it with
/// [`hc_free`], passing the same `len`.
#[unsafe(no_mangle)]
pub extern "C" fn hc_alloc(len: usize) -> *mut u8 {
    if len == 0 {
        return core::ptr::null_mut();
    }
    let mut buffer = Vec::<u8>::new();
    if buffer.try_reserve_exact(len).is_err() {
        return core::ptr::null_mut();
    }
    buffer.resize(len, 0);
    let mut boxed = buffer.into_boxed_slice();
    let pointer = boxed.as_mut_ptr();
    core::mem::forget(boxed);
    pointer
}

/// Return a block from [`hc_alloc`] to the allocator.
///
/// # Safety
///
/// `pointer` must have come from [`hc_alloc`] with the same `len`, and must
/// not have been freed already.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_free(pointer: *mut u8, len: usize) {
    if pointer.is_null() || len == 0 {
        return;
    }
    // SAFETY: the caller guarantees the pointer and length came from a
    // matching `hc_alloc`, which built the block as a boxed slice.
    unsafe {
        let slice = core::ptr::slice_from_raw_parts_mut(pointer, len);
        drop(Box::from_raw(slice));
    }
}

/// The library version as UTF-8, returning the byte length written.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_version(buffer: *mut u8, capacity: usize) -> i64 {
    // SAFETY: forwarded to the caller's contract above.
    unsafe { marshal::emit(hc::VERSION, buffer, capacity) }
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
