//! Two crates count the Chinese lunar mansions of a day, and they count
//! the same cycle.
//!
//! `hc-almanac`'s `mansions::mansion_of` is the Japanese almanac's
//! 二十八宿, a free-running cycle of twenty-eight days from an anchor of
//! 1685; `hc-calendars-regional`'s `tibetan_almanac::chinese_mansion` is
//! the Chinese mansion Henning's computed Tibetan almanacs print beside a
//! day, read off his pages. Neither source states the other's count, so
//! this test is what ties them: the same mansion, from 角 *Jiao*, on every
//! day of 1000–3000.

#![cfg(all(feature = "almanac", feature = "regional"))]

use hyper_calendar::hc_almanac::mansions::mansion_of;
use hyper_calendar::hc_calendar::Rd;
use hyper_calendar::hc_calendars_regional::tibetan_almanac::{CHINESE_MANSIONS, chinese_mansion};

#[test]
fn the_tibetan_almanacs_chinese_mansion_is_the_japanese_almanacs() {
    // JD 2 086 308 is 1 January 1000 (Julian), and JD 2 816 788 is past
    // 3000; each is one addition, so every day is walked in either build.
    for jdn in 2_086_308..2_816_788_i64 {
        let rd = Rd::from_julian_day_number(jdn);
        assert_eq!(chinese_mansion(rd), mansion_of(rd).index(), "JD {jdn}");
    }
    // 11 February 2013 is Henning's "Bi", the Japanese almanac's 畢.
    let day = Rd::from_julian_day_number(2_456_335);
    assert_eq!(CHINESE_MANSIONS[usize::from(chinese_mansion(day))], "Bi");
    assert_eq!(mansion_of(day).japanese_name(), "畢");
}
