use super::super::*;
use super::{measured, read_lines};

fn line(code: &core::ffi::CStr, locale: &core::ffi::CStr) -> String {
    read_lines(|buffer, capacity, written| unsafe {
        hc_place_name(code.as_ptr(), locale.as_ptr(), buffer, capacity, written)
    })
}

/// CLDR 48 `subdivisions/ja.xml` `jp13` 東京都, provisional;
/// `subdivisions/de.xml` `usca` Kalifornien.
#[test]
fn the_lines_are_the_modules() {
    let japan = read_lines(|buffer, capacity, written| unsafe {
        hc_subdivisions(c"JP".as_ptr(), c"ja".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(japan.lines().count(), 47);
    assert!(
        japan
            .lines()
            .any(|line| line == "JP-13\t東京都\tTokyo\tja\tprovisional\tregular")
    );
    let every = read_lines(|buffer, capacity, written| unsafe {
        hc_subdivisions(core::ptr::null(), c"en".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(every.lines().count(), 5503);
    let territories = read_lines(|buffer, capacity, written| unsafe {
        hc_territories(core::ptr::null(), buffer, capacity, written)
    });
    assert_eq!(territories.lines().count(), 295);
    assert_eq!(
        line(c"US-CA", c"de-AT"),
        "US-CA\tKalifornien\tCalifornia\tde\tprovisional\tregular\n"
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_place_name(c"jp13".as_ptr(), c"ja".as_ptr(), buffer, capacity, written)
        }),
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_place_name(core::ptr::null(), c"ja".as_ptr(), buffer, capacity, written)
        }),
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_subdivisions(c"JPN".as_ptr(), c"ja".as_ptr(), buffer, capacity, written)
        }),
        HC_ERROR_UNKNOWN
    );
}
