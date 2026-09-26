//! 六曜 — re-exported, because the implementation belongs somewhere else.
//!
//! 六曜 is the 暦注 everyone in Japan can name: 大安 for weddings, and 友引
//! the day funerals are avoided, when some crematoria close. The National
//! Diet Library gives 大安 as 「特に婚礼に良い」 and 友引 as a day that
//! 「葬式などの凶事を忌む」 (「日本の暦」, 吉凶を表す言葉①六曜,
//! <https://www.ndl.go.jp/koyomi/chapter3/s3.html>, read 2026-09-26);
//! Japanese Wikipedia 六曜 adds that funeral businesses and crematoria are
//! sometimes closed on 友引 (「友引の日は葬祭関連業や火葬場が休業となって
//! いることがある」, <https://ja.wikipedia.org/wiki/六曜>, read 2026-09-26).
//! It plainly belongs in an almanac crate, and a caller looking for 暦注
//! should find it here.
//!
//! It is nonetheless implemented in [`hc_seasons::rokuyo`], and this module
//! is a re-export rather than a second implementation. The reason is that
//! 六曜 is not a rule over a cycle the way everything else in this crate is:
//! it is `(lunisolar month + lunisolar day) mod 6` and nothing else. It has
//! no sexagenary component, no solar term, no 節月 — it is a pure function of
//! the lunisolar date, exactly like 十五夜 and the phase names that sit
//! beside it in `hc-seasons`. Splitting it away from the lunisolar
//! derivation it is made of, to put it next to rules it shares no machinery
//! with, would be filing by subject matter instead of by dependency.
//!
//! So: one implementation, in the crate that owns the lunisolar day; one
//! place to look, which is here.
//!
//! ```
//! use hc_almanac::{Meridian, Rd, rokuyo::rokuyo};
//!
//! // The 六曜 of a day comes from its lunisolar date, so it needs a
//! // meridian like everything else that touches the Moon.
//! let _ = rokuyo(Rd(738_886), Meridian::JAPAN);
//! ```

pub use hc_seasons::rokuyo::{Rokuyo, rokuyo, rokuyo_of};

#[cfg(test)]
mod tests {
    use hc_calendar::Rd;
    use hc_seasons::Meridian;

    use super::*;

    /// The rule in one line: the first day of the first lunisolar month is
    /// always 先勝, the first of the second month always 友引, and the cycle
    /// restarts at every new moon.
    #[test]
    fn the_re_export_is_the_same_cycle_hc_seasons_computes() {
        assert_eq!(Rokuyo::from_lunisolar(1, 1), Rokuyo::Sensho);
        assert_eq!(Rokuyo::from_lunisolar(2, 1), Rokuyo::Tomobiki);
        assert_eq!(
            rokuyo(Rd(738_886), Meridian::JAPAN),
            hc_seasons::rokuyo::rokuyo(Rd(738_886), Meridian::JAPAN)
        );
    }

    #[test]
    fn the_six_advance_one_a_day_inside_a_lunisolar_month() {
        for day in 1..=28u8 {
            assert_eq!(
                Rokuyo::from_lunisolar(3, day + 1),
                Rokuyo::from_lunisolar(3, day).next()
            );
        }
    }
}
