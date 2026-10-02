use super::super::*;
use super::read_lines;

/// *Spes non confundit*, 6: the jubilee of 2025 in Rome from
/// 24 December 2024 to 6 January 2026.
#[test]
fn the_jubilee_of_2025_is_the_bulls() {
    let day = hc_gregorian_to_fixed(2025, 6, 1);
    let text = read_lines(|buffer, capacity| unsafe { hc_holy_year_on(day, buffer, capacity) });
    let cells: Vec<&str> = text.trim_end_matches('\n').split('\t').collect();
    assert_eq!(cells.len(), 10);
    assert_eq!(
        cells[..3],
        ["within", "Ordinary Jubilee of the Year 2025", "ordinary"]
    );
    assert_eq!(cells[6], hc_gregorian_to_fixed(2024, 12, 24).to_string());
    assert_eq!(cells[7], hc_gregorian_to_fixed(2026, 1, 6).to_string());
    let after = hc_gregorian_to_fixed(2026, 1, 7);
    let text = read_lines(|buffer, capacity| unsafe { hc_holy_year_on(after, buffer, capacity) });
    assert_eq!(text, "outside\t\t\t\t\t\t\t\t\t\n");
    let null = core::ptr::null_mut();
    assert_eq!(unsafe { hc_holy_year_on(0, null, 0) }, HC_ERR_NO_DATA);
    // The documented edges: from the opening of 1975's jubilee to the
    // day the table's sources were checked.
    assert!(unsafe { hc_holy_year_on(720_981, null, 0) } > 0);
    assert!(unsafe { hc_holy_year_on(739_886, null, 0) } > 0);
    assert_eq!(unsafe { hc_holy_year_on(720_980, null, 0) }, HC_ERR_NO_DATA);
    assert_eq!(unsafe { hc_holy_year_on(739_887, null, 0) }, HC_ERR_NO_DATA);
}

/// St George's Day on Monday 28 April 2025 (Full Fact), a Festival;
/// and in 2011, Easter being 24 April, Philip and James has no day,
/// which `hc_holidays_on` reports as a gap of `common-worship`.
#[test]
fn a_celebration_carries_its_rank_and_an_unsettled_one_is_a_gap() {
    let day = hc_gregorian_to_fixed(2025, 4, 28);
    let text =
        read_lines(|buffer, capacity| unsafe { hc_common_worship_on(day, buffer, capacity) });
    assert_eq!(
        text,
        "George, Martyr, Patron of England\tfestival\tFestival\tgeorge-martyr-patron-of-england\n"
    );
    let null = core::ptr::null_mut();
    let ordinary = hc_gregorian_to_fixed(2025, 4, 23);
    assert_eq!(unsafe { hc_common_worship_on(ordinary, null, 0) }, 0);
    assert_eq!(
        unsafe { hc_common_worship_on(i64::MAX, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    // The documented edges, −3 652 424 999 through 3 652 424 634.
    assert!(unsafe { hc_common_worship_on(-3_652_424_999, null, 0) } >= 0);
    assert!(unsafe { hc_common_worship_on(3_652_424_634, null, 0) } >= 0);
    for day in [-3_652_425_000, 3_652_424_635] {
        assert_eq!(
            unsafe { hc_common_worship_on(day, null, 0) },
            HC_ERR_OUT_OF_RANGE
        );
    }
    let in_2011 = hc_gregorian_to_fixed(2011, 5, 1);
    let text = read_lines(|buffer, capacity| unsafe { hc_holidays_on(in_2011, buffer, capacity) });
    assert!(
        text.lines().any(|line| line.starts_with("common-worship\t")
            && line.contains("\tPhilip and James, Apostles\t\tgap\t")),
        "no gap in {text}"
    );
}

#[test]
fn every_table_is_described_in_the_order_of_the_codes() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_holiday_tables("en".as_ptr(), 2, buffer, capacity)
    });
    let codes = read_lines(|buffer, capacity| unsafe { hc_holiday_codes(buffer, capacity) });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert!(rows.iter().all(|row| row.len() == 13));
    assert_eq!(
        rows.iter().map(|row| row[0]).collect::<Vec<_>>(),
        codes.lines().collect::<Vec<_>>()
    );
    let xhkg = rows
        .iter()
        .find(|row| row[0] == "XHKG")
        .expect("Hong Kong's exchange");
    assert_eq!(xhkg[1], "exchange");
    assert_eq!(xhkg[6], "HK");
    assert_eq!(xhkg[7], "");
    // CLDR 48's short name for Hong Kong, in English and in
    // Japanese.
    let hong_kong = rows.iter().find(|row| row[0] == "HK").expect("HK");
    assert_eq!(
        hong_kong[2..],
        [
            "Hong Kong SAR China",
            "Hong Kong",
            "en",
            hong_kong[5],
            "",
            "Hong Kong",
            "",
            "",
            "",
            "",
            ""
        ]
    );
    let japanese = read_lines(|buffer, capacity| unsafe {
        hc_holiday_tables("ja".as_ptr(), 2, buffer, capacity)
    });
    let hong_kong = japanese
        .lines()
        .find(|line| line.starts_with("HK\t"))
        .expect("HK");
    assert!(hong_kong.ends_with("\t\t香港\t\t\t\t\t"), "{hong_kong}");
    assert_eq!(
        unsafe { hc_holiday_tables(core::ptr::null(), 1, core::ptr::null_mut(), 0) },
        HC_ERR_NULL_POINTER
    );
}

#[test]
fn the_lectionary_and_the_astronomical_easter_cross_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_lectionary(hc_gregorian_to_fixed(2026, 11, 22), buffer, capacity)
    });
    assert_eq!(text, "2026\tA\tII\t29\t34\t34\t34\n");
    assert_eq!(
        unsafe { hc_lectionary(hc_gregorian_to_fixed(4100, 1, 1), core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_lectionary(
                hc_gregorian_to_fixed(1969, 12, 31),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        hc_astronomical_easter(2001),
        hc_gregorian_to_fixed(2001, 4, 15)
    );
    assert_eq!(hc_astronomical_easter(2151), HC_ERR_OUT_OF_RANGE);
}

/// The Aleppo statement's table: the vernal full moon of 2001 on
/// Sunday 8 April, a week before its Easter, and that of 2019 on
/// 21 March.
#[test]
fn the_astronomical_paschal_full_moon_crosses_the_boundary() {
    assert_eq!(
        hc_astronomical_paschal_full_moon(2001),
        hc_gregorian_to_fixed(2001, 4, 8)
    );
    assert_eq!(
        hc_astronomical_easter(2001) - hc_astronomical_paschal_full_moon(2001),
        7
    );
    assert_eq!(
        hc_astronomical_paschal_full_moon(2019),
        hc_gregorian_to_fixed(2019, 3, 21)
    );
    assert_eq!(hc_astronomical_paschal_full_moon(1582), HC_ERR_OUT_OF_RANGE);
    assert_eq!(hc_astronomical_paschal_full_moon(2151), HC_ERR_OUT_OF_RANGE);
}

/// 26 March 1962 kept the Annunciation, transferred from the Third Sunday
/// of Lent (`hc-holiday`'s `roman_calendar_1960`).
#[test]
fn the_1960_office_crosses() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_roman_1960_office_on(716_325, buffer, capacity)
    });
    assert!(text.starts_with(
        "office\tThe Annunciation of the Blessed Virgin Mary\tfirst\tI class\t716324\n"
    ));
    assert_eq!(
        unsafe { hc_roman_1960_office_on(0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}
