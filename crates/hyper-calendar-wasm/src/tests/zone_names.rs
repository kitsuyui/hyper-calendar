use super::super::*;
use super::read_lines;

/// Tokyo, *Japan Standard Time* (CLDR 48 `en.xml`, metazone `Japan`) and
/// 日本標準時 under `ja`. Another test loads other rules under the name
/// `America/Los_Angeles`, so it is not used here.
#[test]
fn the_names_are_the_facades() {
    let name = |zone: &str, locale: &str, field: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_zone_name(
                zone.as_ptr(),
                zone.len(),
                1_784_000_000,
                locale.as_ptr(),
                locale.len(),
                field.as_ptr(),
                field.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(
        name("Asia/Tokyo", "en", "zzzz"),
        "Japan Standard Time\tzzzz\tAsia/Tokyo\t32400\t0\n"
    );
    assert!(name("Asia/Tokyo", "ja", "zzzz").starts_with("日本標準時\t"));
    let refused = unsafe {
        hc_zone_name(
            "Asia/Tokyo".as_ptr(),
            10,
            0,
            core::ptr::null(),
            0,
            "q".as_ptr(),
            1,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(refused, HC_ERR_UNKNOWN);
}
