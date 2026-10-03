//! The shape of a region code (ADR 0014).

use hc_holiday::rule::is_region_code;

#[test]
fn a_region_code_is_a_subdivision_or_a_municipality_of_one() {
    for code in [
        "JP-13",
        "jp-13",
        " US-NH ",
        "GB-ENG",
        "CH-ZH",
        "JP-14-130",
        "JP-01-100",
        "IT-RM-058091",
    ] {
        assert!(is_region_code(code), "{code}");
    }
    for code in [
        "",
        "JP",
        "JP-",
        "JP garbage",
        "Tokyo",
        "JP-14-130-5",
        "JP-1234",
        "JPN-13",
        "J-13",
        "JP-14-",
        "JP-14-1234567",
        "J1-13",
        "JP--13",
        "JP-é",
    ] {
        assert!(!is_region_code(code), "{code:?}");
    }
}
