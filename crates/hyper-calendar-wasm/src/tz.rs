//! Time zones, behind the `tz` feature: the day an instant falls on, and
//! the instant a day begins, by the wall clock of an IANA zone, from the
//! built-in table or the zones a page loads; and where each zone is: the
//! principal location the IANA database gives a zone, its countries and
//! its CLDR exemplar city. The zones and the lines are
//! `hyper_calendar::zone_lines`', shared with the C library.

use hc::zone_lines;

use crate::HC_ERR_UNKNOWN;
use crate::marshal::{self, sentinel};

/// Give the module a zone's TZif data under an IANA name, returning 0.
///
/// The built-in table carries seventeen zones and only their current
/// rules; a page that wants another zone, or a zone's history, fetches
/// the IANA file (`/usr/share/zoneinfo/Europe/Rome` on most systems)
/// and hands its bytes here once, after which the two `_in_zone`
/// exports, `hc_zone_offset` and `hc_radio_encode`'s `zone:` answer for
/// that name from it — a loaded zone takes precedence over a built-in
/// one of the same name. The bytes are copied, so the caller
/// may free them. Bytes that are not a TZif file are `HC_ERR_MALFORMED`
/// and nothing is kept; the name's pointer and bytes fail as for
/// `hc_parse_iso_date`.
///
/// # Safety
///
/// `name` must be readable for `name_len` bytes and `tzif` for
/// `tzif_len`, unless null with a zero length.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hc_zone_load(
    name: *const u8,
    name_len: usize,
    tzif: *const u8,
    tzif_len: usize,
) -> i64 {
    // SAFETY: forwarded to the caller's contract above.
    let name = match unsafe { marshal::text(name, name_len) } {
        Ok(name) => name,
        Err(sentinel) => return sentinel,
    };
    if name.is_empty() {
        return HC_ERR_UNKNOWN;
    }
    // SAFETY: forwarded to the caller's contract above.
    let bytes = unsafe { marshal::bytes(tzif, tzif_len) };
    match zone_lines::load_zone(name, bytes) {
        Ok(()) => 0,
        Err(refusal) => sentinel(refusal),
    }
}

hc::exports!("tz", w_exports);
