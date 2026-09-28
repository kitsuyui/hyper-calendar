use super::super::*;
use super::{measured, read_lines};

/// The jubilee of 2025 and St George's Day of 2025, a Festival kept
/// on Monday 28 April, as the module writes them.
#[test]
fn the_holy_year_and_common_worship_lines_are_the_modules() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2025, 4, 28, &mut day) },
        HC_OK
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_holy_year_on(day, buffer, capacity, written)
    });
    assert_eq!(Ok(text), hc::holiday_lines::holy_year_line(day));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_common_worship_on(day, buffer, capacity, written)
    });
    assert_eq!(
        text,
        "George, Martyr, Patron of England\tfestival\tFestival\n"
    );
    let mut written = 0usize;
    assert_eq!(
        unsafe { hc_holy_year_on(0, core::ptr::null_mut(), 0, &mut written) },
        HC_ERROR_NO_DATA
    );
    assert_eq!(
        unsafe { hc_common_worship_on(-3_652_425_000, core::ptr::null_mut(), 0, &mut written) },
        HC_ERROR_OUT_OF_RANGE
    );
    // The documented edges: the holy-year table from the opening of
    // 1975's jubilee to the day its sources were checked, and the
    // lectionary's supported days.
    let holy = |day: i64| {
        measured(|buffer, capacity, written| unsafe {
            hc_holy_year_on(day, buffer, capacity, written)
        })
    };
    assert_eq!(holy(720_981), HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(holy(739_886), HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(holy(720_980), HC_ERROR_NO_DATA);
    assert_eq!(holy(739_887), HC_ERROR_NO_DATA);
    let worship = |day: i64| unsafe {
        hc_common_worship_on(day, core::ptr::null_mut(), 0, core::ptr::null_mut())
    };
    assert_ne!(worship(-3_652_424_999), HC_ERROR_OUT_OF_RANGE);
    assert_ne!(worship(3_652_424_634), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(worship(3_652_424_635), HC_ERROR_OUT_OF_RANGE);
    // A day that keeps nothing writes the empty string.
    let mut buffer = [7 as c_char; 1];
    assert_eq!(
        unsafe { hc_common_worship_on(day - 5, buffer.as_mut_ptr(), 1, &mut written) },
        HC_OK
    );
    assert_eq!((buffer[0], written), (0, 1));
}

#[test]
fn the_tables_the_lectionary_and_easter_cross_the_boundary() {
    let english = read_lines(|buffer, capacity, written| unsafe {
        hc_holiday_tables(c"en-US".as_ptr(), buffer, capacity, written)
    });
    let none = read_lines(|buffer, capacity, written| unsafe {
        hc_holiday_tables(core::ptr::null(), buffer, capacity, written)
    });
    assert_eq!(english.lines().count(), none.lines().count());
    let japan: Vec<&str> = english
        .lines()
        .find(|line| line.starts_with("JP\t"))
        .expect("Japan")
        .split('\t')
        .collect();
    assert_eq!(japan[..5], ["JP", "country", "Japan", "Japan", "en"]);
    assert_eq!(japan.len(), 11);
    assert!(japan[8].split(';').any(|code| code == "JP-13"), "{japan:?}");
    assert_eq!(japan[9..], ["", ""], "Japan gives no day to a group alone");
    let china: Vec<&str> = english
        .lines()
        .find(|line| line.starts_with("CN\t"))
        .expect("China")
        .split('\t')
        .collect();
    assert_eq!(
        china[9..],
        [
            "children;military;women;youth",
            "children;military personnel;women;youth"
        ]
    );
    assert_eq!(japan[7], "", "CLDR has no short name for Japan");
    let japanese = read_lines(|buffer, capacity, written| unsafe {
        hc_holiday_tables(c"ja".as_ptr(), buffer, capacity, written)
    });
    let hong_kong: Vec<&str> = japanese
        .lines()
        .find(|line| line.starts_with("HK\t"))
        .expect("Hong Kong")
        .split('\t')
        .collect();
    assert_eq!(hong_kong[2], "中華人民共和国香港特別行政区");
    assert_eq!(hong_kong[7], "香港");
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2025, 11, 30, &mut day) },
        HC_OK
    );
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_lectionary(day, buffer, capacity, written)
    });
    assert_eq!(line, "2026\tA\tII\t\n");
    let mut easter = 0i64;
    assert_eq!(unsafe { hc_astronomical_easter(2001, &mut easter) }, HC_OK);
    let mut expected = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2001, 4, 15, &mut expected) },
        HC_OK
    );
    assert_eq!(easter, expected);
    assert_eq!(
        unsafe { hc_astronomical_easter(1582, &mut easter) },
        HC_ERROR_OUT_OF_RANGE
    );
}

/// The Aleppo statement's table: the vernal full moon of 2001 on
/// Sunday 8 April, a week before its Easter.
#[test]
fn the_astronomical_paschal_full_moon_crosses_through_an_out_parameter() {
    let (mut full_moon, mut expected, mut easter) = (0i64, 0i64, 0i64);
    assert_eq!(
        unsafe { hc_astronomical_paschal_full_moon(2001, &mut full_moon) },
        HC_OK
    );
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2001, 4, 8, &mut expected) },
        HC_OK
    );
    assert_eq!(full_moon, expected);
    assert_eq!(unsafe { hc_astronomical_easter(2001, &mut easter) }, HC_OK);
    assert_eq!(easter - full_moon, 7);
    assert_eq!(
        unsafe { hc_astronomical_paschal_full_moon(2151, &mut full_moon) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_astronomical_paschal_full_moon(2001, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
}
