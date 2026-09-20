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
//! all. Japan adopted it as the civil week only on 1 January 1876, when the
//! government made Sunday a holiday. See the National Diet Library's
//! 「日本の暦」exhibition, 具注暦 section.
//!
//! # The names differ by country more than the associations do
//!
//! Japanese and Korean both kept the luminary names. Modern Mandarin did
//! not: it numbers the days (星期一 for Monday through 星期六, with 星期日 or
//! 星期天 for Sunday), and the 七曜 names survive there only in classical and
//! astrological contexts. Both Chinese columns are given below for that
//! reason.

use hc_calendar::Weekday;

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

/// The names one luminary's day carries across East Asia.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LuminaryNames {
    /// The single character the almanac column uses, e.g. `"水"`.
    pub character: &'static str,
    /// The body's own name in Chinese characters, e.g. `"水星"`.
    pub body: &'static str,
    /// The Japanese weekday name, e.g. `"水曜日"`.
    pub japanese: &'static str,
    /// The Japanese reading in Hepburn romaji, e.g. `"suiyōbi"`.
    pub romaji: &'static str,
    /// The classical Chinese seven-luminary weekday name, e.g. `"水曜日"`.
    ///
    /// Literary and astrological usage. Not what a mainland Chinese speaker
    /// says today; see [`LuminaryNames::chinese_modern`].
    pub chinese_classical: &'static str,
    /// The modern Mandarin weekday name, e.g. `"星期三"`.
    pub chinese_modern: &'static str,
    /// The Korean weekday name in hangul, e.g. `"수요일"`.
    pub korean: &'static str,
    /// The Korean weekday name in hanja, e.g. `"水曜日"`.
    pub korean_hanja: &'static str,
    /// The English name of the body, e.g. `"Mercury"`.
    pub english: &'static str,
}

/// The name table, in [`Luminary`] order.
///
/// Korean hangul from the Standard Korean Language Dictionary
/// (표준국어대사전); the modern Mandarin column is the 星期 series, which is
/// what the People's Republic standardised.
const NAMES: [LuminaryNames; 7] = [
    LuminaryNames {
        character: "日",
        body: "太陽",
        japanese: "日曜日",
        romaji: "nichiyōbi",
        chinese_classical: "日曜日",
        chinese_modern: "星期日",
        korean: "일요일",
        korean_hanja: "日曜日",
        english: "Sun",
    },
    LuminaryNames {
        character: "月",
        body: "太陰",
        japanese: "月曜日",
        romaji: "getsuyōbi",
        chinese_classical: "月曜日",
        chinese_modern: "星期一",
        korean: "월요일",
        korean_hanja: "月曜日",
        english: "Moon",
    },
    LuminaryNames {
        character: "火",
        body: "火星",
        japanese: "火曜日",
        romaji: "kayōbi",
        chinese_classical: "火曜日",
        chinese_modern: "星期二",
        korean: "화요일",
        korean_hanja: "火曜日",
        english: "Mars",
    },
    LuminaryNames {
        character: "水",
        body: "水星",
        japanese: "水曜日",
        romaji: "suiyōbi",
        chinese_classical: "水曜日",
        chinese_modern: "星期三",
        korean: "수요일",
        korean_hanja: "水曜日",
        english: "Mercury",
    },
    LuminaryNames {
        character: "木",
        body: "木星",
        japanese: "木曜日",
        romaji: "mokuyōbi",
        chinese_classical: "木曜日",
        chinese_modern: "星期四",
        korean: "목요일",
        korean_hanja: "木曜日",
        english: "Jupiter",
    },
    LuminaryNames {
        character: "金",
        body: "金星",
        japanese: "金曜日",
        romaji: "kin'yōbi",
        chinese_classical: "金曜日",
        chinese_modern: "星期五",
        korean: "금요일",
        korean_hanja: "金曜日",
        english: "Venus",
    },
    LuminaryNames {
        character: "土",
        body: "土星",
        japanese: "土曜日",
        romaji: "doyōbi",
        chinese_classical: "土曜日",
        chinese_modern: "星期六",
        korean: "토요일",
        korean_hanja: "土曜日",
        english: "Saturn",
    },
];

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

    /// Every name this luminary's day carries.
    #[must_use]
    pub const fn names(self) -> LuminaryNames {
        NAMES[self.index() as usize]
    }

    /// The single character the almanac column prints, e.g. `"水"`.
    #[must_use]
    pub const fn character(self) -> &'static str {
        self.names().character
    }

    /// The Japanese weekday name, e.g. `"水曜日"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        self.names().japanese
    }

    /// The English name of the body, e.g. `"Mercury"`.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        self.names().english
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

    /// 1 January 1876 — RD 684_830 — is the day Japan's civil seven-day week
    /// began, and it was a Saturday, 土曜日.
    #[test]
    fn the_first_day_of_the_japanese_civil_week_was_saturday() {
        let day = Rd(684_830);
        assert_eq!(Weekday::from_rd(day), Weekday::Saturday);
        assert_eq!(luminary_of(day), Luminary::Saturn);
        assert_eq!(luminary_of(day).names().korean, "토요일");
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
            let names = luminary.names();
            assert!(names.japanese.starts_with(names.character));
            assert!(names.korean_hanja.starts_with(names.character));
            assert!(names.chinese_classical.starts_with(names.character));
            assert_eq!(names.japanese.chars().count(), 3);
            assert_eq!(names.korean.chars().count(), 3);
        }
    }

    /// Modern Mandarin numbers the days instead of naming them, and it
    /// numbers from Monday: 星期一 is Monday, not Sunday.
    #[test]
    fn the_modern_mandarin_names_number_from_monday() {
        assert_eq!(Luminary::Moon.names().chinese_modern, "星期一");
        assert_eq!(Luminary::Saturn.names().chinese_modern, "星期六");
        assert_eq!(Luminary::Sun.names().chinese_modern, "星期日");
    }
}
