use super::super::*;
use super::{measured, read_lines};

/// Berlin in July 2026 is *Mitteleuropäische Sommerzeit* under `de`
/// (CLDR 48 `de.xml`, metazone `Europe_Central`).
#[test]
fn the_names_are_the_modules() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_zone_name(
            c"Europe/Berlin".as_ptr(),
            1_784_000_000,
            c"de".as_ptr(),
            c"zzzz".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(
        text,
        "Mitteleuropäische Sommerzeit\tzzzz\tEurope/Berlin\t7200\t1\n"
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_zone_name(
                c"Europe/Berlin".as_ptr(),
                1_784_000_000,
                c"de".as_ptr(),
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_NULL_POINTER
    );
}
