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
        "George, Martyr, Patron of England\tfestival\tFestival\tgeorge-martyr-patron-of-england\n"
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
    assert_eq!(japan.len(), 14);
    assert!(japan[8].split(';').any(|code| code == "JP-13"), "{japan:?}");
    assert_eq!(
        japan[9..11],
        ["", ""],
        "Japan gives no day to a group alone"
    );
    assert_eq!(
        japan[13], "unread//1992-04-30/;6+7/1992-05-01//",
        "and keeps Saturday and Sunday from 1 May 1992"
    );
    // A region with only a weekend law of its own is listed.
    let cells = |code: &str| -> Vec<&str> {
        english
            .lines()
            .find(|line| line.split('\t').next() == Some(code))
            .expect("a table")
            .split('\t')
            .collect()
    };
    assert_eq!(cells("MY")[8], "MY-01;MY-02;MY-03;MY-09;MY-11");
    assert_eq!(cells("AE")[8], "AE-SH");
    let china: Vec<&str> = english
        .lines()
        .find(|line| line.starts_with("CN\t"))
        .expect("China")
        .split('\t')
        .collect();
    assert_eq!(
        china[9..11],
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
    assert_eq!(line, "2026\tA\tII\t\t\t\t\n");
    // Christ the King, 22 November 2026: Proper 29, the 34th Sunday and
    // week of Ordinary Time on either calendar, as the module writes it.
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 11, 22, &mut day) },
        HC_OK
    );
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_lectionary(day, buffer, capacity, written)
    });
    assert_eq!(line, "2026\tA\tII\t29\t34\t34\t34\n");
    // Before 1 January 1970, the reform's first day, and after 4099.
    let mut written = 0;
    for (year, month, date) in [(1969, 12, 31), (4100, 1, 1)] {
        assert_eq!(
            unsafe { hc_gregorian_to_fixed(year, month, date, &mut day) },
            HC_OK
        );
        assert_eq!(
            unsafe { hc_lectionary(day, core::ptr::null_mut(), 0, &mut written) },
            HC_ERROR_OUT_OF_RANGE
        );
    }
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

/// 25 March 1962 kept the Third Sunday of Lent (`hc-holiday`'s
/// `roman_calendar_1960`).
#[test]
fn the_1960_office_crosses() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_roman_1960_office_on(716_324, buffer, capacity, written)
    });
    assert!(text.starts_with("office\tThird Sunday of Lent\tfirst\t"));
}

/// China's women, 妇女 under `zh-Hans`; Nayrouz of 2025 in Coptic.
#[test]
fn the_groups_and_the_named_days_cross() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_holiday_groups(c"zh-Hans".as_ptr(), buffer, capacity, written)
    });
    assert!(text.starts_with("women\t妇女\tzh-Hans\twomen\n"));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_holidays_on_in(739_505, c"cop".as_ptr(), buffer, capacity, written)
    });
    assert!(
        text.lines()
            .any(|line| line.ends_with("\tⲡⲓⲭⲗⲟⲙ ⲛ̀ⲧⲉ ϯⲣⲟⲙⲡⲓ\tcop\tnayrouz-new-year\t0"))
    );
}

/// Japan's civil service weekend is read from 1 May 1992, and every year to
/// 1948 is a gap; a code that names no table is unknown.
#[test]
fn the_years_a_table_answers_for_cross_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_holiday_coverage(c"JP".as_ptr(), buffer, capacity, written)
    });
    assert!(
        text.starts_with("\t1948\t1949\t\t1\t1992\tHolidays before the Public Holidays Act\n"),
        "{text}"
    );
    let mut written = 0usize;
    assert_eq!(
        unsafe { hc_holiday_coverage(c"ZZ".as_ptr(), core::ptr::null_mut(), 0, &mut written) },
        HC_ERROR_UNKNOWN
    );
}

/// Five business days after Tuesday 28 April 2026 in Japan is Monday
/// 11 May, Golden Week's holidays skipped, as the `JP` table has them.
#[test]
fn business_days_cross() {
    let mut day = 0i64;
    let status = unsafe {
        hc_holiday_add_business_days(
            c"JP".as_ptr(),
            core::ptr::null(),
            core::ptr::null(),
            739_734,
            5,
            &mut day,
        )
    };
    assert_eq!((status, day), (HC_OK, 739_747));
    let mut count = 0i64;
    let status = unsafe {
        hc_holiday_business_days_between(
            c"JP".as_ptr(),
            core::ptr::null(),
            core::ptr::null(),
            739_733,
            739_747,
            &mut count,
        )
    };
    assert_eq!((status, count), (HC_OK, 6));
}
