//! Deep time, behind the `deep-time` feature: the cosmic, geologic and
//! archaeological chronologies of `hc-deep-time`, and a moment placed in
//! all of them at once.

use hc::deep_time_lines;

use crate::HC_ERR_UNKNOWN;
use crate::marshal::{emit_or_measure, text};

/// Every interval of one rank of the geologic time scale, as UTF-8
/// lines, returning the byte length written.
///
/// `rank` is 0 for the eons, 1 for the eras, 2 for the periods, 3 for
/// the epochs and 4 for the ages; anything else is `HC_ERR_UNKNOWN`.
/// The intervals come youngest first, each a line of the columns
/// `hc_place_years_ago` writes, in `megayears-before-present` with the
/// chart's own figures and uncertainties, the chart as the source and
/// the chart's name in `locale` last. A null `buffer` returns the
/// length the text needs.
///
/// # Safety
///
/// `locale` must be readable for `locale_len` bytes unless null with a
/// zero length; `buffer` must be writable for `capacity` bytes unless it
/// is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_geologic_intervals(
    rank: u32,
    locale: *const u8,
    locale_len: usize,
    buffer: *mut u8,
    capacity: usize,
) -> i64 {
    let Some(rank) = deep_time_lines::rank(rank) else {
        return HC_ERR_UNKNOWN;
    };
    // SAFETY: forwarded to the caller's contract above.
    let tag = match unsafe { text(locale, locale_len) } {
        Ok(tag) => tag,
        Err(sentinel) => return sentinel,
    };
    let text = deep_time_lines::intervals(rank, tag);
    // SAFETY: forwarded to the caller's contract above.
    unsafe { emit_or_measure(&text, buffer, capacity) }
}

hc::exports!("deep-time", w_exports);
