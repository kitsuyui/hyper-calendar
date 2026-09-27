//! 恵方 — the year's lucky direction, where 歳徳神 stands.
//!
//! The almanacs place the deity of the year's fortune, 歳徳神, in one of
//! four of the twenty-four compass points, and whatever is done facing it
//! is lucky: the New Year's visit to a shrine in that direction, 恵方参り,
//! and the rolled sushi eaten facing it on 節分, 恵方巻.
//! `docs/systems/east-asian-folk-days.md` describes the system and its
//! tests.
//!
//! # The rule
//!
//! The direction is the year's heavenly stem. Only the five yang stems,
//! 甲丙戊庚壬, have a direction of their own; each yin stem takes a yang
//! one's, so that 己 years face 甲 (世界大百科事典, 「恵（吉）方」, on
//! kotobank.jp, `kotobank-eho`). デジタル大辞泉 tabulates all ten, with the
//! last digit of the Gregorian year (`kotobank-eho`):
//!
//! | Stem | Last digit | 恵方 | Between | About |
//! |---|---|---|---|---|
//! | 甲, 己 | 4, 9 | 甲 | 寅 and 卯 | east-north-east |
//! | 乙, 庚 | 5, 0 | 庚 | 申 and 酉 | west-south-west |
//! | 丙, 辛, 戊, 癸 | 6, 1, 8, 3 | 丙 | 巳 and 午 | south-south-east |
//! | 丁, 壬 | 7, 2 | 壬 | 亥 and 子 | north-north-west |
//!
//! A point of the twenty-four is 15° wide, and the four lie at azimuths of
//! 75°, 255°, 165° and 345° (Wikipedia (ja), 「歳徳神」, `wikipedia-ja-toshitokujin`).
//!
//! # Which year
//!
//! The function takes the year by its Gregorian number, whose stem is the
//! sexagenary year's that mostly overlaps it, and says nothing about the day
//! a year's direction takes over. The two customs read here use the
//! Gregorian year: the 恵方巻 of 節分 on 3 February 2026, the eve of 立春,
//! face 2026's 丙 (All About, `allabout-eho-2026`; JRE Media,
//! `jre-eho-2026`), and 恵方参り is on New Year's Day. The 九星 year,
//! which turns at 立春, is [`crate::nine_stars::nine_star_year`].

/// One of the four directions 歳徳神 can stand in, named for its point of
/// the twenty-four.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LuckyDirection {
    /// 甲, between 寅 and 卯: about east-north-east, 75°.
    Kinoe,
    /// 庚, between 申 and 酉: about west-south-west, 255°.
    Kanoe,
    /// 丙, between 巳 and 午: about south-south-east, 165°.
    Hinoe,
    /// 壬, between 亥 and 子: about north-north-west, 345°.
    Mizunoe,
}

impl LuckyDirection {
    /// The four, in the order the yang stems 甲, 丙, 庚, 壬 would put them
    /// clockwise from the north: 甲, 丙, 庚, 壬.
    pub const ALL: [Self; 4] = [Self::Kinoe, Self::Hinoe, Self::Kanoe, Self::Mizunoe];

    /// The direction of a year whose heavenly stem is `stem`, 甲 being 0;
    /// reduced modulo ten.
    #[must_use]
    pub const fn of_stem(stem: u8) -> Self {
        match stem % 10 {
            0 | 5 => Self::Kinoe,
            1 | 6 => Self::Kanoe,
            3 | 8 => Self::Mizunoe,
            _ => Self::Hinoe,
        }
    }

    /// The point of the twenty-four, a stem: `"甲"`, `"庚"`, `"丙"` or `"壬"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::Kinoe => "甲",
            Self::Kanoe => "庚",
            Self::Hinoe => "丙",
            Self::Mizunoe => "壬",
        }
    }

    /// The reading in Hepburn romaji, e.g. `"kinoe"`.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        match self {
            Self::Kinoe => "kinoe",
            Self::Kanoe => "kanoe",
            Self::Hinoe => "hinoe",
            Self::Mizunoe => "mizunoe",
        }
    }

    /// The two earthly branches the point lies between, e.g. `("寅", "卯")`.
    #[must_use]
    pub const fn between_branches(self) -> (&'static str, &'static str) {
        match self {
            Self::Kinoe => ("寅", "卯"),
            Self::Kanoe => ("申", "酉"),
            Self::Hinoe => ("巳", "午"),
            Self::Mizunoe => ("亥", "子"),
        }
    }

    /// The azimuth of the point's centre, in degrees clockwise from north.
    #[must_use]
    pub const fn azimuth_degrees(self) -> u16 {
        match self {
            Self::Kinoe => 75,
            Self::Kanoe => 255,
            Self::Hinoe => 165,
            Self::Mizunoe => 345,
        }
    }

    /// The nearest of the sixteen compass points in Japanese, as the
    /// almanac-derived tables give it: `"東北東"`, `"西南西"`, `"南南東"` or
    /// `"北北西"`. Each is 7.5° from the azimuth, a little towards the
    /// cardinal direction beyond it.
    #[must_use]
    pub const fn sixteen_point_name(self) -> &'static str {
        match self {
            Self::Kinoe => "東北東",
            Self::Kanoe => "西南西",
            Self::Hinoe => "南南東",
            Self::Mizunoe => "北北西",
        }
    }

    /// The same point in English, e.g. `"east-north-east"`.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Kinoe => "east-north-east",
            Self::Kanoe => "west-south-west",
            Self::Hinoe => "south-south-east",
            Self::Mizunoe => "north-north-west",
        }
    }
}

/// 恵方 of a year numbered as the Gregorian year it mostly overlaps: the
/// direction of the year's stem, `(year − 4) mod 10`, 1984 being 甲子.
#[must_use]
pub const fn lucky_direction_of_year(year: i64) -> LuckyDirection {
    LuckyDirection::of_stem((year - 4).rem_euclid(10) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// デジタル大辞泉's table: the last digit of the year 4 甲, 5 庚, 6 丙,
    /// 7 壬, 8 丙, 9 甲, 0 庚, 1 丙, 2 壬, 3 丙; and the stems it pairs them
    /// with, 甲 for 4 through 癸 for 3.
    #[test]
    fn the_direction_follows_the_last_digit_of_the_year() {
        let by_digit = ["庚", "丙", "壬", "丙", "甲", "庚", "丙", "壬", "丙", "甲"];
        for year in 1900i64..2100 {
            let digit = year.rem_euclid(10) as usize;
            assert_eq!(
                lucky_direction_of_year(year).japanese_name(),
                by_digit[digit],
                "{year}"
            );
        }
        // 己 years face 甲: the yin stem borrows its yang partner's.
        assert_eq!(LuckyDirection::of_stem(5), LuckyDirection::Kinoe);
        assert_eq!(LuckyDirection::of_stem(15), LuckyDirection::Kinoe);
    }

    /// All About (三浦康子, updated 2026-01-27): the 恵方 of 節分 2026 is
    /// 南南東, 165°. JRE Media: 2026 南南東, 2027 北北西, 2028 南南東, 2029
    /// 東北東, 2030 西南西, and 甲 75°, 丙 165°, 庚 255°, 壬 345°.
    #[test]
    fn the_published_directions_of_2026_to_2030() {
        for (year, point, azimuth) in [
            (2026, "南南東", 165),
            (2027, "北北西", 345),
            (2028, "南南東", 165),
            (2029, "東北東", 75),
            (2030, "西南西", 255),
        ] {
            let direction = lucky_direction_of_year(year);
            assert_eq!(direction.sixteen_point_name(), point, "{year}");
            assert_eq!(direction.azimuth_degrees(), azimuth, "{year}");
        }
        assert_eq!(
            lucky_direction_of_year(2026).between_branches(),
            ("巳", "午")
        );
        assert_eq!(
            lucky_direction_of_year(2026).english_name(),
            "south-south-east"
        );
    }

    /// Each point is 7.5° from the nearest of the sixteen, the half-width of
    /// a point of the twenty-four.
    #[test]
    fn the_four_points_sit_between_two_of_the_sixteen() {
        for direction in LuckyDirection::ALL {
            let azimuth = f64::from(direction.azimuth_degrees());
            let sixteenth = 22.5;
            let offset = azimuth % sixteenth;
            assert!((offset - 7.5).abs() < 1e-9 || (offset - 15.0).abs() < 1e-9);
            assert!(!direction.romaji().is_empty());
        }
    }
}
