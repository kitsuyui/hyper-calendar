//! 七曜 — the seven luminaries the days of the week are named after.
//!
//! The seven-day week itself is [`hc_calendar::Weekday`] and this module does
//! not reimplement it. What a week cycle alone cannot tell you is *why*
//! Wednesday is 水曜日: the East Asian weekday names are not ordinals, they
//! are the Sun, the Moon and the five classical planets in the order of the
//! planetary hours. So this module is a table over the existing `Weekday`,
//! not a second week.
//!
//! # Where the names come from
//!
//! The seven-luminary week reached East Asia with the *Xiuyaojing* (宿曜経),
//! translated into Chinese by Amoghavajra in 759 and carried to Japan by
//! Kūkai in 806. It appears in Japanese 具注暦 from the ninth century as a
//! divinatory cycle rather than a civil one — the 七曜 column sat beside the
//! 十二直 and the 二十八宿, which is why it belongs in an almanac crate at
//! all. See the National Diet Library's 「日本の暦」exhibition, 具注暦
//! section. It became the civil week in April 1876, when 太政官達第27号 of
//! 明治9年3月12日 made Sunday a day off and Saturday afternoon a half-day
//! from that month, replacing the days ending in 1 and 6 (NAOJ 暦計算室, 暦Wiki
//! 「日曜日」 and 「明治以降の休日」).
//!
//! # The names differ by country more than the associations do
//!
//! Japanese and Korean both kept the luminary names. Modern Mandarin did
//! not: it numbers the days (星期一 for Monday through 星期六, with 星期日 or
//! 星期天 for Sunday), and the 七曜 names survive there only in classical and
//! astrological contexts. Both Chinese columns are given below for that
//! reason.

use hc_calendar::Weekday;
use hc_calendar::shape::Naming;

/// One of the seven luminaries (七曜): the Sun, the Moon and the five
/// classical planets.
///
/// Ordering is the weekday order the almanac prints, Sunday first, which is
/// also the order of the planetary hours and not the order of the planets'
/// distance from the Sun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Luminary {
    /// 日 — the Sun. Sunday.
    Sun,
    /// 月 — the Moon. Monday.
    Moon,
    /// 火 — Mars. Tuesday.
    Mars,
    /// 水 — Mercury. Wednesday.
    Mercury,
    /// 木 — Jupiter. Thursday.
    Jupiter,
    /// 金 — Venus. Friday.
    Venus,
    /// 土 — Saturn. Saturday.
    Saturn,
}

/// The ways the 7 positions are named, one entry per language or
/// convention. See [`hc_calendar::shape::Naming`].
pub mod namings {
    use hc_calendar::shape::Naming;

    hc_core::catalogue! {
        type: Naming<7>,
        id: |naming| naming.id,
        provenance: |naming| naming.authority,
        tests: luminary_naming_tests,

        /// Every naming this crate ships.
        pub const ALL;
        /// The naming with this identifier.
        pub fn by_id;

        entries: {
            /// The single character the almanac column uses, e.g. `"水"`.
            pub const CHARACTER = Naming {
                id: "hani-column",
                english_name: "Almanac character",
                names: &[
                "日",
                "月",
                "火",
                "水",
                "木",
                "金",
                "土",
                ],
                authority: "The single character the 七曜 column of a Japanese almanac prints",
            };
            /// The body's own name in Chinese characters, e.g. `"水星"`.
            pub const BODY = Naming {
                id: "zh-body",
                english_name: "Body names in characters",
                names: &[
                "太陽",
                "太陰",
                "火星",
                "水星",
                "木星",
                "金星",
                "土星",
                ],
                authority: "The bodies' own names in Chinese characters",
            };
            /// The Japanese weekday name, e.g. `"水曜日"`.
            pub const JAPANESE = Naming {
                id: "ja",
                english_name: "Japanese",
                names: &[
                "日曜日",
                "月曜日",
                "火曜日",
                "水曜日",
                "木曜日",
                "金曜日",
                "土曜日",
                ],
                authority: "The 曜日 series as Japanese prints it",
            };
            /// The Japanese reading in Hepburn romaji, e.g. `"suiyōbi"`.
            pub const ROMAJI = Naming {
                id: "ja-latn",
                english_name: "Japanese, romanised",
                names: &[
                "nichiyōbi",
                "getsuyōbi",
                "kayōbi",
                "suiyōbi",
                "mokuyōbi",
                "kin'yōbi",
                "doyōbi",
                ],
                authority: "Hepburn romanisation of the Japanese names",
            };
            /// The classical Chinese seven-luminary weekday name, e.g. `"水曜日"`.
            ///
            /// Literary and astrological usage. Not what a mainland Chinese speaker
            /// says today; see `namings::CHINESE_MODERN`.
            pub const CHINESE_CLASSICAL = Naming {
                id: "zh-classical",
                english_name: "Classical Chinese",
                names: &[
                "日曜日",
                "月曜日",
                "火曜日",
                "水曜日",
                "木曜日",
                "金曜日",
                "土曜日",
                ],
                authority: "The seven-luminary weekday names of literary and astrological usage",
            };
            /// The modern Mandarin weekday name, e.g. `"星期三"`.
            pub const CHINESE_MODERN = Naming {
                id: "zh",
                english_name: "Modern Mandarin",
                names: &[
                "星期日",
                "星期一",
                "星期二",
                "星期三",
                "星期四",
                "星期五",
                "星期六",
                ],
                authority: "The 星期 series the People's Republic standardised",
            };
            /// The Korean weekday name in hangul, e.g. `"수요일"`.
            pub const KOREAN = Naming {
                id: "ko",
                english_name: "Korean, Hangul",
                names: &[
                "일요일",
                "월요일",
                "화요일",
                "수요일",
                "목요일",
                "금요일",
                "토요일",
                ],
                authority: "표준국어대사전 (Standard Korean Language Dictionary)",
            };
            /// The Korean weekday name in hanja, e.g. `"水曜日"`.
            pub const KOREAN_HANJA = Naming {
                id: "ko-hani",
                english_name: "Korean, hanja",
                names: &[
                "日曜日",
                "月曜日",
                "火曜日",
                "水曜日",
                "木曜日",
                "金曜日",
                "土曜日",
                ],
                authority: "The same weekday names in hanja",
            };
            /// The English name of the body, e.g. `"Mercury"`.
            pub const ENGLISH = Naming {
                id: "en",
                english_name: "English",
                names: &[
                "Sun",
                "Moon",
                "Mars",
                "Mercury",
                "Jupiter",
                "Venus",
                "Saturn",
                ],
                authority: "The English names of the seven bodies",
            };
        }
    }
}

impl Luminary {
    /// All seven, Sunday first.
    pub const ALL: [Self; 7] = [
        Self::Sun,
        Self::Moon,
        Self::Mars,
        Self::Mercury,
        Self::Jupiter,
        Self::Venus,
        Self::Saturn,
    ];

    /// The luminary a weekday is named after.
    ///
    /// This is the mapping that makes the module worth having: it is fixed,
    /// it is the same in Japan, China and Korea, and it survives unchanged
    /// from the ninth-century 具注暦 to a modern wall calendar.
    #[must_use]
    pub const fn of_weekday(weekday: Weekday) -> Self {
        match weekday {
            Weekday::Sunday => Self::Sun,
            Weekday::Monday => Self::Moon,
            Weekday::Tuesday => Self::Mars,
            Weekday::Wednesday => Self::Mercury,
            Weekday::Thursday => Self::Jupiter,
            Weekday::Friday => Self::Venus,
            Weekday::Saturday => Self::Saturn,
        }
    }

    /// The weekday named after this luminary.
    #[must_use]
    pub const fn weekday(self) -> Weekday {
        match self {
            Self::Sun => Weekday::Sunday,
            Self::Moon => Weekday::Monday,
            Self::Mars => Weekday::Tuesday,
            Self::Mercury => Weekday::Wednesday,
            Self::Jupiter => Weekday::Thursday,
            Self::Venus => Weekday::Friday,
            Self::Saturn => Weekday::Saturday,
        }
    }

    /// The zero-based position in the Sunday-first almanac order.
    #[must_use]
    pub const fn index(self) -> u8 {
        match self {
            Self::Sun => 0,
            Self::Moon => 1,
            Self::Mars => 2,
            Self::Mercury => 3,
            Self::Jupiter => 4,
            Self::Venus => 5,
            Self::Saturn => 6,
        }
    }

    /// The name of this luminary's day in one naming — `namings::KOREAN`,
    /// `namings::CHINESE_MODERN` and so on.
    #[must_use]
    pub const fn name(self, naming: &Naming<7>) -> &'static str {
        naming.names[self.index() as usize]
    }

    /// The single character the almanac column prints, e.g. `"水"`.
    #[must_use]
    pub const fn character(self) -> &'static str {
        self.name(&namings::CHARACTER)
    }

    /// The Japanese weekday name, e.g. `"水曜日"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        self.name(&namings::JAPANESE)
    }

    /// The English name of the body, e.g. `"Mercury"`.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        self.name(&namings::ENGLISH)
    }

    /// The five-phase element the body is identified with, if any.
    ///
    /// The five planets *are* the five phases in this scheme — 火星 is
    /// literally "the fire star" — but the Sun and the Moon are 陽 and 陰,
    /// outside the five, so they answer `None`. Do not fill that gap with a
    /// guess; the tradition leaves it open deliberately.
    #[must_use]
    pub const fn five_phase(self) -> Option<&'static str> {
        match self {
            Self::Sun | Self::Moon => None,
            Self::Mars => Some("fire"),
            Self::Mercury => Some("water"),
            Self::Jupiter => Some("wood"),
            Self::Venus => Some("metal"),
            Self::Saturn => Some("earth"),
        }
    }

    /// Whether this luminary is one of the two 陰陽 lights rather than one of
    /// the five planets.
    #[must_use]
    pub const fn is_light(self) -> bool {
        matches!(self, Self::Sun | Self::Moon)
    }
}

/// The luminary of a fixed day.
///
/// The seven-day week has run unbroken across every calendar reform, so this
/// needs nothing but the fixed day — no meridian, no astronomy.
#[must_use]
pub const fn luminary_of(day: hc_calendar::Rd) -> Luminary {
    Luminary::of_weekday(Weekday::from_rd(day))
}

#[cfg(test)]
mod tests {
    use hc_calendar::Rd;

    use super::*;

    #[test]
    fn every_weekday_maps_to_exactly_one_luminary_and_back() {
        for weekday in Weekday::ALL {
            assert_eq!(Luminary::of_weekday(weekday).weekday(), weekday);
        }
        for luminary in Luminary::ALL {
            assert_eq!(Luminary::of_weekday(luminary.weekday()), luminary);
        }
    }

    #[test]
    fn the_luminaries_are_indexed_sunday_first() {
        for (position, luminary) in Luminary::ALL.iter().enumerate() {
            assert_eq!(luminary.index() as usize, position);
        }
        assert_eq!(Luminary::Sun.weekday(), Weekday::Sunday);
        assert_eq!(Luminary::Saturn.weekday(), Weekday::Saturday);
    }

    /// The Rata Die epoch, 0001-01-01 proleptic Gregorian, was a Monday, so
    /// it is 月曜日 — the Moon's day.
    #[test]
    fn the_rata_die_epoch_is_the_moons_day() {
        assert_eq!(luminary_of(Rd(1)), Luminary::Moon);
        assert_eq!(luminary_of(Rd(1)).japanese_name(), "月曜日");
    }

    /// 1 April 1876 — RD 684_921 — is the day 太政官達第27号 put the
    /// Sunday rest into force, and it was a Saturday, 土曜日, the first of the
    /// half-days; the first Sunday off was the next day.
    #[test]
    fn the_sunday_rest_took_force_on_a_saturday() {
        let day = Rd(684_921);
        assert_eq!(Weekday::from_rd(Rd(day.0 + 1)), Weekday::Sunday);
        assert_eq!(Weekday::from_rd(day), Weekday::Saturday);
        assert_eq!(luminary_of(day), Luminary::Saturn);
        assert_eq!(luminary_of(day).name(&namings::KOREAN), "토요일");
    }

    #[test]
    fn the_five_planets_carry_the_five_phases_and_the_two_lights_do_not() {
        assert_eq!(Luminary::Mars.five_phase(), Some("fire"));
        assert_eq!(Luminary::Mercury.five_phase(), Some("water"));
        assert_eq!(Luminary::Jupiter.five_phase(), Some("wood"));
        assert_eq!(Luminary::Venus.five_phase(), Some("metal"));
        assert_eq!(Luminary::Saturn.five_phase(), Some("earth"));
        assert_eq!(Luminary::Sun.five_phase(), None);
        assert_eq!(Luminary::Moon.five_phase(), None);
        assert!(Luminary::Sun.is_light() && Luminary::Moon.is_light());
        assert!(!Luminary::Mars.is_light());
    }

    #[test]
    fn the_almanac_character_opens_the_japanese_and_korean_names() {
        for luminary in Luminary::ALL {
            let character = luminary.name(&namings::CHARACTER);
            assert!(luminary.name(&namings::JAPANESE).starts_with(character));
            assert!(luminary.name(&namings::KOREAN_HANJA).starts_with(character));
            assert!(
                luminary
                    .name(&namings::CHINESE_CLASSICAL)
                    .starts_with(character)
            );
            assert_eq!(luminary.name(&namings::JAPANESE).chars().count(), 3);
            assert_eq!(luminary.name(&namings::KOREAN).chars().count(), 3);
        }
    }

    /// Modern Mandarin numbers the days instead of naming them, and it
    /// numbers from Monday: 星期一 is Monday, not Sunday.
    #[test]
    fn the_modern_mandarin_names_number_from_monday() {
        assert_eq!(Luminary::Moon.name(&namings::CHINESE_MODERN), "星期一");
        assert_eq!(Luminary::Saturn.name(&namings::CHINESE_MODERN), "星期六");
        assert_eq!(Luminary::Sun.name(&namings::CHINESE_MODERN), "星期日");
    }
}
