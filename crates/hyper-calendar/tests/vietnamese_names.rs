//! The Vietnamese lunisolar calendar written in Vietnamese: the date as
//! CLDR 48's `vi.xml` writes the Chinese calendar's long date, 'Ngày' dd
//! 'tháng' M 'năm' U, with the year by its stem and branch as `vi.xml`
//! states them (Giáp Tý to Quý Hợi), the leap month by its pattern `{0}
//! Nhuận`, and the months by the traditional names of the Vietnamese
//! Wikipedia's "Nông lịch" (`wikipedia-vi-nong-lich`): tháng giêng, hai, ba,
//! tư, năm, sáu, bảy, tám, chín, mười, mười một and chạp. Before this the
//! locale had no entry for the calendar and the date was written in
//! English, "Eighth Month 18, 2026".

#![cfg(all(
    feature = "alloc",
    feature = "lunar",
    feature = "i18n",
    feature = "format"
))]
#![expect(
    clippy::expect_used,
    reason = "a fixture that does not convert is a failed test, and the \
              message says which"
)]

use hyper_calendar::hc_calendars_solar::gregorian;
use hyper_calendar::hc_format::label;
use hyper_calendar::hc_i18n::Locale;

fn written(calendar: &str, year: i64, month: u8, day: u8, tag: &str) -> String {
    let registry = hyper_calendar::registry();
    let calendar = registry.get_by_name(calendar).expect("registered");
    let rd = gregorian::to_fixed(year, month, day).expect("a Gregorian date");
    let fields = calendar.fixed_to_fields(rd).expect("converts");
    let locale = Locale::parse(tag).expect("a tag");
    let locale = label::locale_for(calendar, Some(&locale));
    label::date(calendar, &fields, &locale)
}

/// Tết Giáp Thìn, 10 February 2024, the first of the first month: the
/// Vietnamese write it "mùng 1 tháng Giêng", and the year is Giáp Thìn.
/// 2026-09-28 is the 18th of the eighth month of Bính Ngọ, and 25 March
/// 2023 the 4th of the leap second month of Quý Mão, which CLDR's pattern
/// writes after the month.
#[test]
fn the_vietnamese_calendar_is_written_in_vietnamese() {
    assert_eq!(
        written("vietnamese", 2024, 2, 10, "vi"),
        "Ngày 1 tháng Giêng năm Giáp Thìn 2024"
    );
    assert_eq!(
        written("vietnamese", 2026, 9, 28, "vi"),
        "Ngày 18 tháng Tám năm Bính Ngọ 2026"
    );
    assert_eq!(
        written("vietnamese", 2023, 3, 25, "vi"),
        "Ngày 4 tháng Hai Nhuận năm Quý Mão 2023"
    );
}

/// Every month of a year has its name, the first and the last as the
/// Vietnamese write them in the festivals of Tết, tháng Giêng and tháng
/// Chạp, and the eleventh as "tháng Mười Một".
#[test]
fn the_twelve_months_have_their_traditional_names() {
    // 2024-02-10 begins the year of Giáp Thìn; its months begin on these
    // days of the Gregorian calendar, which the calendar's own rules give
    // and the Purple Mountain Observatory's table, checked month by month
    // in `docs/systems/east-asian-lunisolar.md`, agrees with.
    let starts = [
        (2024, 2, 10, "Giêng"),
        (2024, 3, 10, "Hai"),
        (2024, 4, 9, "Ba"),
        (2024, 5, 8, "Tư"),
        (2024, 6, 6, "Năm"),
        (2024, 7, 6, "Sáu"),
        (2024, 8, 4, "Bảy"),
        (2024, 9, 3, "Tám"),
        (2024, 10, 3, "Chín"),
        (2024, 11, 1, "Mười"),
        (2024, 12, 1, "Mười Một"),
        (2024, 12, 31, "Chạp"),
    ];
    for (year, month, day, name) in starts {
        let text = written("vietnamese", year, month, day, "vi");
        assert!(
            text.starts_with(&format!("Ngày 1 tháng {name} năm ")),
            "{year}-{month}-{day}: {text}"
        );
    }
}

/// The Chinese and Dangi calendars, which share the month structure,
/// are written the same way in Vietnamese; the other locales are as
/// before.
#[test]
fn the_other_calendars_of_the_family_follow_and_english_is_unchanged() {
    assert_eq!(
        written("chinese", 2026, 9, 28, "vi"),
        "Ngày 18 tháng Tám năm Bính Ngọ 2026"
    );
    assert_eq!(
        written("vietnamese", 2026, 9, 28, "en"),
        "Eighth Month 18, 2026"
    );
}
