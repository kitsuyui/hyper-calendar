//! Deep time, behind the `deep-time` feature: the cosmic, geologic and
//! archaeological chronologies of `hc-deep-time`, and a moment placed in
//! all of them at once.

use core::ffi::c_char;

use hc::deep_time_lines;

use crate::marshal::{text, write_text};
use crate::{HC_ERROR_UNKNOWN, HcStatus};

/// Every interval of one rank of the geologic time scale, as
/// NUL-terminated UTF-8 lines in a caller-owned buffer.
///
/// `rank` is 0 for the eons, 1 for the eras, 2 for the periods, 3 for
/// the epochs and 4 for the ages; anything else is `HC_ERROR_UNKNOWN`.
/// The intervals come youngest first, each a line of the columns
/// `hc_place_years_ago` writes, in `megayears-before-present` with the
/// chart's own figures and uncertainties, the chart as the source and
/// the chart's name in `locale` last. Writes the required length,
/// including the terminator, into `written`.
///
/// # Safety
///
/// `locale` must be null or point to a NUL-terminated string; `buffer`
/// must be writable for `capacity` bytes and `written` must be null or
/// writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_geologic_intervals(
    rank: u32,
    locale: *const c_char,
    buffer: *mut c_char,
    capacity: usize,
    written: *mut usize,
) -> HcStatus {
    let Some(rank) = deep_time_lines::rank(rank) else {
        return HC_ERROR_UNKNOWN;
    };
    // SAFETY: forwarded to the caller's contract above.
    let tag = match unsafe { text(locale) } {
        Ok(tag) => tag.unwrap_or(""),
        Err(status) => return status,
    };
    let text = deep_time_lines::intervals(rank, tag);
    // SAFETY: forwarded to the caller's contract above.
    unsafe { write_text(&text, buffer, capacity, written) }
}

hc::exports!("deep-time", c_exports);
