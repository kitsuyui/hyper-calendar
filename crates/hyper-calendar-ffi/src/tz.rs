//! Time zones, behind the `tz` feature: the day an instant falls on, and
//! the instant a day begins, by the wall clock of an IANA zone, from the
//! built-in table or the zones a caller loads; and where each zone is: the
//! principal location the IANA database gives a zone, its countries and
//! its CLDR exemplar city. The zones and the lines are
//! `hyper_calendar::zone_lines`', shared with the WebAssembly module.

use core::ffi::c_char;

use hc::zone_lines;

use crate::marshal::{self, status};
use crate::{HC_ERROR_NULL_POINTER, HC_OK, HcStatus};

/// Give the library a zone's TZif data under an IANA name.
///
/// The built-in table carries seventeen zones and only their current
/// rules; a caller that wants another zone, or a zone's history, reads
/// the IANA file and hands its bytes here once, after which the two
/// `_in_zone` entry points, `hc_zone_offset` and `hc_radio_encode`'s
/// `zone:` answer for that name from it — a loaded zone takes
/// precedence over a built-in one of the same name. The bytes are
/// copied. A null or empty `name` is `HC_ERROR_NULL_POINTER`, bytes
/// that are not a TZif file `HC_ERROR_MALFORMED`, and nothing is kept
/// on failure.
///
/// # Safety
///
/// `name` must be null or point to a NUL-terminated string, and `tzif`
/// must be readable for `tzif_len` bytes unless `tzif_len` is zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_zone_load(
    name: *const c_char,
    tzif: *const u8,
    tzif_len: usize,
) -> HcStatus {
    // SAFETY: forwarded to the caller's contract above.
    let name = match unsafe { marshal::name(name) } {
        Ok(name) if !name.is_empty() => name,
        Ok(_) => return HC_ERROR_NULL_POINTER,
        Err(status) => return status,
    };
    // SAFETY: forwarded to the caller's contract above.
    let bytes = unsafe { marshal::bytes(tzif, tzif_len) };
    match zone_lines::load_zone(name, bytes) {
        Ok(()) => HC_OK,
        Err(refusal) => status(refusal),
    }
}

hc::exports!("tz", c_exports);
