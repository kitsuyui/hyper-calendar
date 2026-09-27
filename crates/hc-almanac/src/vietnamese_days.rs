//! Ngày Tam Nương and ngày Nguyệt Kỵ — the Vietnamese lunar days avoided
//! for weddings, building, travel and opening a business.
//!
//! Tam Nương, "the three maidens", falls on the 3rd, 7th, 13th, 18th, 22nd
//! and 27th of every lunar month, and Nguyệt Kỵ, "the month's taboo", on
//! the 5th, 14th and 23rd. The two lists are Trần Ngọc Kiệm's, quoted in
//! VTC News, "Quan niệm ngày xấu đại kỵ xuất hành, cưới hỏi hoặc làm việc
//! lớn dưới góc nhìn khoa học", 1 August 2018 (`vtc-ngay-xau-2018`),
//! retrieved 2026-09-28. The same article quotes Vũ Thế Khanh calling the
//! Tam Nương days Nguyệt Kỵ when counted on the solar calendar in the West;
//! that is a remark about naming and not a third list, and is not carried.
//! The lunar date is the Vietnamese calendar's, `vietnamese`.
//! `docs/systems/east-asian-folk-days.md` describes the days and their
//! tests.
//!
//! The rules are by the day's number alone, so the days of a leap month
//! are the same.

use hc_calendar::Rd;
use hc_calendars_lunar::vietnamese;

/// The Vietnamese name, `"Tam Nương"`.
pub const TAM_NUONG_NAME: &str = "Tam Nương";

/// The Vietnamese name, `"Nguyệt Kỵ"`.
pub const NGUYET_KY_NAME: &str = "Nguyệt Kỵ";

/// The lunar days that are Tam Nương.
pub const TAM_NUONG_DAYS: [u8; 6] = [3, 7, 13, 18, 22, 27];

/// The lunar days that are Nguyệt Kỵ.
pub const NGUYET_KY_DAYS: [u8; 3] = [5, 14, 23];

/// Whether a lunar day number is a Tam Nương day.
#[must_use]
pub const fn is_tam_nuong_day_number(day: u8) -> bool {
    matches!(day, 3 | 7 | 13 | 18 | 22 | 27)
}

/// Whether a lunar day number is a Nguyệt Kỵ day.
#[must_use]
pub const fn is_nguyet_ky_day_number(day: u8) -> bool {
    matches!(day, 5 | 14 | 23)
}

/// The day of the lunar month of a day on the Vietnamese calendar, or
/// `None` outside the years `vietnamese` converts.
fn lunar_day(day: Rd) -> Option<u8> {
    let (_, _, lunar_day) = vietnamese::PARAMETERS.from_fixed(day).ok()?;
    Some(lunar_day)
}

/// Whether a day is a Tam Nương day on the Vietnamese lunar calendar, or
/// `None` outside the years `vietnamese` converts.
#[must_use]
pub fn is_tam_nuong(day: Rd) -> Option<bool> {
    lunar_day(day).map(is_tam_nuong_day_number)
}

/// Whether a day is a Nguyệt Kỵ day on the Vietnamese lunar calendar, or
/// `None` outside the years `vietnamese` converts.
#[must_use]
pub fn is_nguyet_ky(day: Rd) -> Option<bool> {
    lunar_day(day).map(is_nguyet_ky_day_number)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::gregorian;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    /// Lịch Ngày Tốt, "Xem lịch ngày Tam Nương 2026", 25 December 2025
    /// (`lichngaytot-tam-nuong-2026`), retrieved 2026-09-28: the Gregorian
    /// days of 2026 month by month, with the lunar date of each. Both
    /// columns have misprints: the lunar 22/23/2026, and 2025 for November;
    /// and 26 October for the 18th of the ninth month, which the page's own
    /// 3/9 on 12 October and 22/9 on 31 October put on 27 October. The
    /// Gregorian column is the one tested, with that day corrected.
    #[test]
    fn the_tam_nuong_days_of_2026_are_the_published_list() {
        let published: &[(u8, &[u8])] = &[
            (1, &[1, 6, 10, 15, 21, 25, 31]),
            (2, &[5, 9, 14, 19, 23]),
            (3, &[1, 6, 10, 15, 21, 25, 31]),
            (4, &[5, 9, 14, 19, 23, 29]),
            (5, &[4, 8, 13, 19, 23, 29]),
            (6, &[3, 7, 12, 17, 21, 27]),
            (7, &[2, 6, 11, 16, 20, 26, 31]),
            (8, &[4, 9, 15, 19, 25, 30]),
            (9, &[3, 8, 13, 17, 23, 28]),
            (10, &[2, 7, 12, 16, 22, 27, 31]),
            (11, &[5, 11, 15, 21, 26, 30]),
            (12, &[5, 11, 15, 21, 26, 30]),
        ];
        let mut expected = Vec::new();
        for (month, days) in published {
            for day in *days {
                expected.push(ymd(2026, *month, *day));
            }
        }
        let first = ymd(2026, 1, 1);
        let found: Vec<Rd> = (0..365)
            .map(|offset| Rd(first.0 + offset))
            .filter(|day| is_tam_nuong(*day) == Some(true))
            .collect();
        assert_eq!(found, expected);
    }

    #[test]
    fn the_twenty_third_of_march_2026_is_nguyet_ky() {
        // Báo Nghệ An, "Lịch Âm Dương ngày 23/3/2026 ... lưu ý ngày Nguyệt
        // Kỵ" (`baonghean-2026-03-23`): Monday 23 March 2026 is the 5th of
        // the second lunar month, a Nguyệt Kỵ day.
        let day = ymd(2026, 3, 23);
        assert_eq!(lunar_day(day), Some(5));
        assert_eq!(is_nguyet_ky(day), Some(true));
        assert_eq!(is_tam_nuong(day), Some(false));
        assert_eq!(is_nguyet_ky(ymd(2026, 3, 22)), Some(false));
    }

    #[test]
    fn the_rules_are_the_day_numbers_and_the_calendar_has_a_range() {
        let tam_nuong: Vec<u8> = (1..=30).filter(|d| is_tam_nuong_day_number(*d)).collect();
        assert_eq!(tam_nuong, TAM_NUONG_DAYS);
        let nguyet_ky: Vec<u8> = (1..=30).filter(|d| is_nguyet_ky_day_number(*d)).collect();
        assert_eq!(nguyet_ky, NGUYET_KY_DAYS);
        assert!(TAM_NUONG_DAYS.iter().all(|d| !is_nguyet_ky_day_number(*d)));
        assert_eq!(is_tam_nuong(Rd(vietnamese::EARLIEST.0 - 1)), None);
        assert_eq!(is_nguyet_ky(Rd(vietnamese::LATEST.0 + 1)), None);
        assert_eq!((TAM_NUONG_NAME, NGUYET_KY_NAME), ("Tam Nương", "Nguyệt Kỵ"));
    }
}
