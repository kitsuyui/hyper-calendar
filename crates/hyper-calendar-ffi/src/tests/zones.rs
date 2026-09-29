use super::super::*;
use super::read_lines;

fn line(zone: &core::ffi::CStr, locale: &core::ffi::CStr) -> String {
    read_lines(|buffer, capacity, written| unsafe {
        hc_zone_location(zone.as_ptr(), locale.as_ptr(), buffer, capacity, written)
    })
}

/// `zone1970.tab` 2026d: `JP,AU +353916+1394441 Asia/Tokyo Eyre
/// Bird Observatory`; `zone.tab`: `NO +5955+01045 Europe/Oslo`;
/// `backward`: `Link Asia/Kolkata Asia/Calcutta`.
#[test]
fn the_lines_are_the_modules_and_links_answer_with_their_rows() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_zones(core::ptr::null(), buffer, capacity, written)
    });
    assert_eq!(text.lines().count(), 312);
    assert!(text.lines().any(|line| line
        == "Asia/Tokyo\t35.654444\t139.744722\tJP;AU\tJP\tEyre Bird Observatory\tTokyo\ten"));
    assert!(
        line(c"Europe/Oslo", c"en").starts_with("Europe/Oslo\t59.916667\t10.750000\tNO\tNO\t\t")
    );
    assert!(line(c"Asia/Calcutta", c"en").starts_with("Asia/Kolkata\t"));
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_zone_location(
                c"UTC".as_ptr(),
                c"en".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_zone_location(
                core::ptr::null(),
                c"en".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NULL_POINTER
    );
    let tokyo = line(c"Asia/Tokyo", c"ja");
    let expected = if cfg!(any(feature = "calendars", feature = "zone-names")) {
        "\t東京\tja\n"
    } else {
        "\tTokyo\ten\n"
    };
    assert!(tokyo.ends_with(expected), "{tokyo}");
}
