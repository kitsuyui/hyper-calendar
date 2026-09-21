//! 十二次 — the Chinese twelvefold division of the ecliptic.
//!
//! Twelve stations, thirty degrees each, named for the lunar lodges and the
//! asterisms they contain: 星紀, 玄枵, 娵訾, and so round to 析木. They are
//! older than the Western signs' arrival in China and were originally the
//! twelve stages of Jupiter's journey round the sky, Jupiter taking very
//! nearly twelve years to make one circuit.
//!
//! # Its relationship to the 二十四節気, which is exact
//!
//! In the modern 定気 reckoning a 次 runs from one **節気** to the next but
//! one — that is, from an odd multiple of 15° to the next odd multiple of
//! 30° later — and contains exactly one **中気** in its middle. 星紀 opens at
//! 大雪 (255°) and holds 冬至 (270°); 玄枵 opens at 小寒 (285°) and holds
//! 大寒 (300°).
//!
//! So a 次 is a tropical sign rotated back by fifteen degrees:
//!
//! | | opens at | holds |
//! | --- | --- | --- |
//! | [`super::TropicalSign`] | a 中気 | a 節気 |
//! | [`ChineseStation`] | a 節気 | a 中気 |
//!
//! This also makes a 次 the same interval as a 節月, the "sectional month"
//! that the four-pillar reckoning and several of the 雑節 count from, so
//! [`ChineseStation`] and the solar month of Chinese astrology are one thing
//! under two names.
//!
//! # The twelve animals are not here
//!
//! Each 次 has a matching 十二辰 earthly branch, given by
//! [`ChineseStation::earthly_branch`], and the earthly branches are the
//! twelve animals — rat, ox, tiger, rabbit. **The animal of a year is not a
//! function of this module.** A year's animal is its branch in the
//! sexagenary cycle, a counting cycle with no angle in it at all, and it
//! lives in [`hc_calendar::cycle`]: use
//! [`hc_calendar::cycle::sexagenary_year`] and
//! [`hc_calendar::cycle::Sexagenary::zodiac_animal`]. Looking for your birth
//! animal here will give you the wrong answer, because what this module would
//! tell you is which thirtieth of the ecliptic the Sun was in on your
//! birthday, which is a different question with the same twelve labels.
//!
//! The branches here run *backwards* against the 次 — 星紀 is 丑, 玄枵 is 子,
//! 娵訾 is 亥 — because the 十二辰 were counted clockwise, against the Sun's
//! motion, as a division of the celestial equator for the diurnal rotation.
//! That reversal is itself the clearest evidence the two schemes were built
//! for different purposes.
//!
//! # And the twelve 次 are not the twelve Western signs either
//!
//! Chinese astronomical writing from the Ming dynasty onwards equates each 次
//! with a Western sign — 星紀 with 磨羯 (Capricorn), 降婁 with 白羊 (Aries) —
//! and [`ChineseStation::traditional_zodiac_counterpart`] carries that
//! equation. It is an equation of *names*, not of arcs: under the 定気 rule
//! the two are offset by fifteen degrees, so half of each 次 is in the sign
//! it is equated with and half is in the previous one. The equation was made
//! when the 次 were still reckoned from the lodges rather than from the
//! 節気, and this module does not pretend it is exact.

use hc_astro::solar::{seasonal_event, solar_longitude, solar_longitude_after};
use hc_calendar::Rd;
use hc_calendar::cycle::EARTHLY_BRANCHES;
use hc_calendar::fixed::Moment;
use hc_core::math::floor;

use crate::meridian::Meridian;
use crate::solar_terms::SolarTerm;
use crate::zodiac::tropical::TropicalSign;
use crate::zodiac::{DEGREES_PER_SIGN, SIGNS_PER_ZODIAC, SignPeriod, degrees_into_arc};

/// One of the 十二次.
///
/// Ordering is the traditional one, from 星紀 at 255°, which is also the
/// order the [`hc_calendar::cycle::EARTHLY_BRANCHES`] run backwards through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChineseStation(u8);

/// The apparent solar longitude at which 星紀, the first 次, opens.
///
/// 255° is 大雪, the 節気 three weeks before the winter solstice. The
/// traditional list starts here because 星紀 is the station that contains the
/// solstice, and the solstice is where the Chinese astronomical year was
/// reckoned from.
pub const FIRST_STATION_START_DEGREES: f64 = 255.0;

/// One row of the station table.
struct StationNames {
    chinese: &'static str,
    japanese: &'static str,
    pinyin: &'static str,
    english: &'static str,
}

/// The station table, in the traditional order from 星紀.
///
/// The Chinese column is in traditional characters and the Japanese one in
/// post-1946 shinjitai; they differ for 實沈/実沈 and 壽星/寿星, which is why
/// they are two columns and not one, exactly as in
/// [`crate::solar_terms`]. Pinyin is the Mandarin reading with tone marks.
///
/// The English column is a gloss of the characters, not a translation of a
/// meaning: 鶉首, 鶉火 and 鶉尾 are the head, fire and tail of the Vermilion
/// Bird — 朱雀, the quail-like southern constellation — and 大火 is the star
/// Antares, which Chinese astronomy called the Great Fire and used as a
/// seasonal marker long before the 次 were formalised.
const STATION_NAMES: [StationNames; SIGNS_PER_ZODIAC] = [
    StationNames {
        chinese: "星紀",
        japanese: "星紀",
        pinyin: "Xīngjì",
        english: "star record",
    },
    StationNames {
        chinese: "玄枵",
        japanese: "玄枵",
        pinyin: "Xuánxiāo",
        english: "dark emptiness",
    },
    StationNames {
        chinese: "娵訾",
        japanese: "娵訾",
        pinyin: "Jūzī",
        english: "the lodge Shi",
    },
    StationNames {
        chinese: "降婁",
        japanese: "降婁",
        pinyin: "Jiànglóu",
        english: "descending Lou",
    },
    StationNames {
        chinese: "大梁",
        japanese: "大梁",
        pinyin: "Dàliáng",
        english: "great beam",
    },
    StationNames {
        chinese: "實沈",
        japanese: "実沈",
        pinyin: "Shíchén",
        english: "deep sinking",
    },
    StationNames {
        chinese: "鶉首",
        japanese: "鶉首",
        pinyin: "Chúnshǒu",
        english: "the bird's head",
    },
    StationNames {
        chinese: "鶉火",
        japanese: "鶉火",
        pinyin: "Chúnhuǒ",
        english: "the bird's fire",
    },
    StationNames {
        chinese: "鶉尾",
        japanese: "鶉尾",
        pinyin: "Chúnwěi",
        english: "the bird's tail",
    },
    StationNames {
        chinese: "壽星",
        japanese: "寿星",
        pinyin: "Shòuxīng",
        english: "star of longevity",
    },
    StationNames {
        chinese: "大火",
        japanese: "大火",
        pinyin: "Dàhuǒ",
        english: "the great fire",
    },
    StationNames {
        chinese: "析木",
        japanese: "析木",
        pinyin: "Xīmù",
        english: "split wood",
    },
];

/// The earthly branch of 星紀, from which the rest count backwards.
///
/// 星紀 is 丑 (`"chou"`), branch index 1 in
/// [`hc_calendar::cycle::EARTHLY_BRANCHES`],
/// and each following 次 takes the *previous* branch: 玄枵 is 子 (0), 娵訾 is
/// 亥 (11), and so on.
const FIRST_STATION_BRANCH_INDEX: u8 = 1;

impl ChineseStation {
    /// 星紀, 255°–285°, opening at 大雪 and holding 冬至. Branch 丑.
    pub const XINGJI: Self = Self(0);
    /// 玄枵, 285°–315°, opening at 小寒 and holding 大寒. Branch 子.
    pub const XUANXIAO: Self = Self(1);
    /// 娵訾, 315°–345°, opening at 立春 and holding 雨水. Branch 亥.
    pub const JUZI: Self = Self(2);
    /// 降婁, 345°–15°, opening at 啓蟄 and holding 春分. Branch 戌.
    pub const JIANGLOU: Self = Self(3);
    /// 大梁, 15°–45°, opening at 清明 and holding 穀雨. Branch 酉.
    pub const DALIANG: Self = Self(4);
    /// 實沈, 45°–75°, opening at 立夏 and holding 小満. Branch 申.
    pub const SHICHEN: Self = Self(5);
    /// 鶉首, 75°–105°, opening at 芒種 and holding 夏至. Branch 未.
    pub const CHUNSHOU: Self = Self(6);
    /// 鶉火, 105°–135°, opening at 小暑 and holding 大暑. Branch 午.
    pub const CHUNHUO: Self = Self(7);
    /// 鶉尾, 135°–165°, opening at 立秋 and holding 処暑. Branch 巳.
    pub const CHUNWEI: Self = Self(8);
    /// 壽星, 165°–195°, opening at 白露 and holding 秋分. Branch 辰.
    pub const SHOUXING: Self = Self(9);
    /// 大火, 195°–225°, opening at 寒露 and holding 霜降. Branch 卯.
    pub const DAHUO: Self = Self(10);
    /// 析木, 225°–255°, opening at 立冬 and holding 小雪. Branch 寅.
    pub const XIMU: Self = Self(11);

    /// All twelve, in the traditional order from 星紀.
    pub const ALL: [Self; SIGNS_PER_ZODIAC] = [
        Self::XINGJI,
        Self::XUANXIAO,
        Self::JUZI,
        Self::JIANGLOU,
        Self::DALIANG,
        Self::SHICHEN,
        Self::CHUNSHOU,
        Self::CHUNHUO,
        Self::CHUNWEI,
        Self::SHOUXING,
        Self::DAHUO,
        Self::XIMU,
    ];

    /// The station at an index reduced modulo twelve.
    const fn at(index: u8) -> Self {
        Self(index % 12)
    }

    /// The station at an index counted from 星紀.
    ///
    /// Returns `None` for an index of 12 or more.
    #[must_use]
    pub const fn from_index(index: u8) -> Option<Self> {
        if index as usize >= SIGNS_PER_ZODIAC {
            return None;
        }
        Some(Self(index))
    }

    /// This station's index from 星紀, 0 to 11.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// The station an arbitrary apparent solar longitude falls in.
    #[must_use]
    pub fn containing_degrees(longitude_degrees: f64) -> Self {
        let from_first =
            hc_core::math::normalize_degrees(longitude_degrees - FIRST_STATION_START_DEGREES);
        Self::at(floor(from_first / DEGREES_PER_SIGN) as u8)
    }

    /// The apparent solar longitude at which the station opens, 0° to 360°.
    #[must_use]
    pub fn start_longitude_degrees(self) -> f64 {
        hc_core::math::normalize_degrees(
            FIRST_STATION_START_DEGREES + f64::from(self.0) * DEGREES_PER_SIGN,
        )
    }

    /// The 節気 at which the station opens.
    ///
    /// Always a sectional term, because the stations are offset from the
    /// multiples of 30° by 15°.
    #[must_use]
    pub fn opening_term(self) -> SolarTerm {
        let degrees = (255 + i32::from(self.0) * 30).rem_euclid(360);
        match SolarTerm::from_degrees(degrees) {
            Some(term) => term,
            // Unreachable: 255 + 30n is an odd multiple of 15.
            None => SolarTerm::BEGINNING_OF_SPRING,
        }
    }

    /// The 中気 that falls in the middle of the station, 15° in.
    ///
    /// This is the term the station is *named by* in the four-pillar
    /// reckoning, and the one the lunisolar leap-month rule counts.
    #[must_use]
    pub fn midpoint_term(self) -> SolarTerm {
        let degrees = (270 + i32::from(self.0) * 30).rem_euclid(360);
        match SolarTerm::from_degrees(degrees) {
            Some(term) => term,
            // Unreachable: 270 + 30n is a multiple of 30.
            None => SolarTerm::WINTER_SOLSTICE,
        }
    }

    /// The name in traditional Chinese characters, e.g. `"實沈"`.
    #[must_use]
    pub const fn chinese_name(self) -> &'static str {
        STATION_NAMES[self.0 as usize].chinese
    }

    /// The name in Japanese shinjitai, e.g. `"実沈"`.
    ///
    /// Identical to [`Self::chinese_name`] for ten of the twelve.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        STATION_NAMES[self.0 as usize].japanese
    }

    /// The Mandarin reading in pinyin with tone marks, e.g. `"Shíchén"`.
    #[must_use]
    pub const fn pinyin(self) -> &'static str {
        STATION_NAMES[self.0 as usize].pinyin
    }

    /// A short English gloss of the characters, e.g. `"deep sinking"`.
    ///
    /// A gloss, not a translation: several of these names are asterism names
    /// whose literal sense was already obscure in antiquity.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        STATION_NAMES[self.0 as usize].english
    }

    /// The index of the matching 十二辰 earthly branch, 0 (子) to 11 (亥).
    ///
    /// The branches run backwards against the stations from 丑 at 星紀.
    #[must_use]
    pub const fn earthly_branch_index(self) -> u8 {
        (FIRST_STATION_BRANCH_INDEX + 12 - self.0 % 12) % 12
    }

    /// The matching earthly branch, romanised, e.g. `"chou"` for 丑.
    ///
    /// The strings come from [`hc_calendar::cycle::EARTHLY_BRANCHES`] rather
    /// than from a copy here, so the two can never disagree about what the
    /// twelve branches are or what order they run in.
    ///
    /// This is a spatial correspondence between two divisions of the sky, not
    /// a year's animal. See the module documentation.
    #[must_use]
    pub const fn earthly_branch(self) -> &'static str {
        EARTHLY_BRANCHES[self.earthly_branch_index() as usize]
    }

    /// The Western sign the station is traditionally equated with.
    ///
    /// 星紀 with Capricorn, 玄枵 with Aquarius, and so on. An equation of
    /// names made before the 定気 rule fixed the stations to the 節気; under
    /// that rule the two arcs are 15° apart, so this is a correspondence and
    /// not an identity. See the module documentation.
    #[must_use]
    pub const fn traditional_zodiac_counterpart(self) -> TropicalSign {
        // 星紀 is Capricorn, index 9, and both run forwards from there.
        TropicalSign::at(self.0 + 9)
    }

    /// The next station, 30° further along the ecliptic, wrapping at 360°.
    #[must_use]
    pub const fn next(self) -> Self {
        Self((self.0 + 1) % 12)
    }

    /// The previous station, wrapping at 0°.
    #[must_use]
    pub const fn previous(self) -> Self {
        Self((self.0 + 11) % 12)
    }
}

/// The Universal Time instant the Sun enters a station in a Gregorian year.
///
/// The stations open at the 節気, so this is
/// [`crate::solar_terms::term_moment`] of [`ChineseStation::opening_term`],
/// and the two agree to the noise of the search.
#[must_use]
pub fn station_moment(year: i64, station: ChineseStation) -> Moment {
    seasonal_event(year, station.start_longitude_degrees())
}

/// The day the Sun enters a station in a Gregorian year, at a meridian.
#[must_use]
pub fn station_day(year: i64, station: ChineseStation, meridian: Meridian) -> Rd {
    meridian.day_of(station_moment(year, station))
}

/// The station the Sun is in at an instant.
#[must_use]
pub fn station_at_moment(moment: Moment) -> ChineseStation {
    ChineseStation::containing_degrees(solar_longitude(moment))
}

/// How far into its station the Sun is at an instant, in degrees from 0 up to
/// but not including 30.
#[must_use]
pub fn degrees_into_station(moment: Moment) -> f64 {
    let longitude = solar_longitude(moment);
    degrees_into_arc(
        longitude,
        ChineseStation::containing_degrees(longitude).start_longitude_degrees(),
    )
}

/// A period built from a known opening instant.
fn period_from_start(
    station: ChineseStation,
    start: Moment,
    meridian: Meridian,
) -> SignPeriod<ChineseStation> {
    let end = solar_longitude_after(
        station.next().start_longitude_degrees(),
        Moment(start.0 + 1.0),
    );
    SignPeriod {
        sign: station,
        start,
        end,
        start_day: meridian.day_of(start),
        end_day: Rd(meridian.day_of(end).0 - 1),
    }
}

/// The period a station occupies in a Gregorian year.
#[must_use]
pub fn station_period(
    year: i64,
    station: ChineseStation,
    meridian: Meridian,
) -> SignPeriod<ChineseStation> {
    period_from_start(station, station_moment(year, station), meridian)
}

/// The period of the station in effect on a day.
#[must_use]
pub fn station_in_effect(day: Rd, meridian: Meridian) -> SignPeriod<ChineseStation> {
    let end_of_day = meridian.midnight(Rd(day.0 + 1));
    let station = ChineseStation::containing_degrees(solar_longitude(end_of_day));
    let start = solar_longitude_after(
        station.start_longitude_degrees(),
        Moment(end_of_day.0 - 35.0),
    );
    period_from_start(station, start, meridian)
}

/// The station in effect on a day.
#[must_use]
pub fn station_on_day(day: Rd, meridian: Meridian) -> ChineseStation {
    station_in_effect(day, meridian).sign
}

/// The twelve station periods of a Gregorian year, in date order.
///
/// The first is 玄枵, which opens at 小寒 in the first week of January — the
/// same term the crate's [`crate::solar_terms::terms_in_year`] opens with,
/// for the same reason. The last is 星紀, which opens at 大雪 in December and
/// runs into the following January.
#[must_use]
pub fn stations_in_year(year: i64, meridian: Meridian) -> StationsInYear {
    let station = ChineseStation::XUANXIAO;
    StationsInYear {
        meridian,
        station,
        start: station_moment(year, station),
        remaining: SIGNS_PER_ZODIAC,
    }
}

/// The iterator returned by [`stations_in_year`].
#[derive(Debug, Clone, Copy)]
pub struct StationsInYear {
    meridian: Meridian,
    station: ChineseStation,
    start: Moment,
    remaining: usize,
}

impl Iterator for StationsInYear {
    type Item = SignPeriod<ChineseStation>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let period = period_from_start(self.station, self.start, self.meridian);
        self.remaining -= 1;
        self.station = self.station.next();
        self.start = period.end;
        Some(period)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for StationsInYear {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::{from_year_month_day, year_month_day_from_rd};
    use crate::solar_terms::{TermKind, term_moment};

    const JAPAN: Meridian = Meridian::JAPAN;

    #[test]
    fn the_twelve_stations_are_thirty_degree_arcs_from_two_hundred_and_fifty_five() {
        for (index, station) in ChineseStation::ALL.into_iter().enumerate() {
            assert_eq!(station.index() as usize, index);
            assert_eq!(ChineseStation::from_index(index as u8), Some(station));
            let expected = (255.0 + index as f64 * 30.0) % 360.0;
            assert!(
                (station.start_longitude_degrees() - expected).abs() < 1e-12,
                "{} opened at {}",
                station.japanese_name(),
                station.start_longitude_degrees()
            );
        }
        assert_eq!(ChineseStation::from_index(12), None);
        assert!((ChineseStation::XINGJI.start_longitude_degrees() - 255.0).abs() < 1e-12);
        assert!((ChineseStation::JIANGLOU.start_longitude_degrees() - 345.0).abs() < 1e-12);
        assert!((ChineseStation::DALIANG.start_longitude_degrees() - 15.0).abs() < 1e-12);
    }

    /// The relationship the module exists to state: a 次 opens at a 節気 and
    /// holds a 中気, which is the tropical zodiac's arrangement turned inside
    /// out.
    #[test]
    fn a_station_opens_at_a_sectional_term_and_holds_a_principal_one() {
        for station in ChineseStation::ALL {
            assert_eq!(station.opening_term().kind(), TermKind::Sectional);
            assert_eq!(station.midpoint_term().kind(), TermKind::Principal);
            assert!(
                (station.opening_term().solar_longitude_degrees()
                    - station.start_longitude_degrees())
                .abs()
                    < 1e-12
            );
            let into = (station.midpoint_term().solar_longitude_degrees()
                - station.start_longitude_degrees()
                + 360.0)
                % 360.0;
            assert!((into - 15.0).abs() < 1e-12, "{into} degrees in");
        }
        assert_eq!(
            ChineseStation::XINGJI.midpoint_term(),
            SolarTerm::WINTER_SOLSTICE
        );
        assert_eq!(
            ChineseStation::JIANGLOU.midpoint_term(),
            SolarTerm::SPRING_EQUINOX
        );
        assert_eq!(
            ChineseStation::JUZI.opening_term(),
            SolarTerm::BEGINNING_OF_SPRING
        );
        assert_eq!(
            ChineseStation::CHUNSHOU.opening_term(),
            SolarTerm::GRAIN_IN_EAR
        );
    }

    /// A 次 is a tropical sign rotated back by half a sign, which is what
    /// "the two divisions are 15° apart" means concretely.
    #[test]
    fn a_station_is_a_tropical_sign_rotated_back_fifteen_degrees() {
        for station in ChineseStation::ALL {
            let counterpart = station.traditional_zodiac_counterpart();
            let gap = (counterpart.start_longitude_degrees() - station.start_longitude_degrees()
                + 360.0)
                % 360.0;
            assert!(
                (gap - 15.0).abs() < 1e-12,
                "{} and {} are {gap} degrees apart",
                station.japanese_name(),
                counterpart.english_name()
            );
        }
        assert_eq!(
            ChineseStation::XINGJI.traditional_zodiac_counterpart(),
            TropicalSign::CAPRICORN
        );
        assert_eq!(
            ChineseStation::JIANGLOU.traditional_zodiac_counterpart(),
            TropicalSign::ARIES
        );
        assert_eq!(
            ChineseStation::XIMU.traditional_zodiac_counterpart(),
            TropicalSign::SAGITTARIUS
        );
    }

    /// The branches run backwards from 丑 at 星紀, so that 十二次 and 十二辰
    /// turn in opposite directions.
    #[test]
    fn the_earthly_branches_run_backwards_against_the_stations() {
        // Romanised, because that is the form
        // `hc_calendar::cycle::EARTHLY_BRANCHES` carries; the characters are
        // given beside each for the reader.
        let expected = [
            (ChineseStation::XINGJI, "chou"),   // 丑
            (ChineseStation::XUANXIAO, "zi"),   // 子
            (ChineseStation::JUZI, "hai"),      // 亥
            (ChineseStation::JIANGLOU, "xu"),   // 戌
            (ChineseStation::DALIANG, "you"),   // 酉
            (ChineseStation::SHICHEN, "shen"),  // 申
            (ChineseStation::CHUNSHOU, "wei"),  // 未
            (ChineseStation::CHUNHUO, "wu"),    // 午
            (ChineseStation::CHUNWEI, "si"),    // 巳
            (ChineseStation::SHOUXING, "chen"), // 辰
            (ChineseStation::DAHUO, "mao"),     // 卯
            (ChineseStation::XIMU, "yin"),      // 寅
        ];
        for (station, branch) in expected {
            assert_eq!(
                station.earthly_branch(),
                branch,
                "{} took the wrong branch",
                station.japanese_name()
            );
        }
        // All twelve branches are used exactly once.
        let mut seen = [false; 12];
        for station in ChineseStation::ALL {
            let index = station.earthly_branch_index() as usize;
            assert!(!seen[index]);
            seen[index] = true;
        }
        assert!(seen.iter().all(|flag| *flag));
    }

    /// The station cycle and the branch cycle turn in opposite directions:
    /// stepping one station forward steps the branch one back.
    #[test]
    fn stepping_a_station_forward_steps_its_branch_backward() {
        for station in ChineseStation::ALL {
            let here = i16::from(station.earthly_branch_index());
            let there = i16::from(station.next().earthly_branch_index());
            assert_eq!((here - there).rem_euclid(12), 1);
        }
    }

    #[test]
    fn every_station_has_a_full_set_of_names() {
        for station in ChineseStation::ALL {
            assert!(!station.chinese_name().is_empty());
            assert!(!station.japanese_name().is_empty());
            assert!(!station.pinyin().is_empty());
            assert!(!station.english_name().is_empty());
            assert!(station.english_name().is_ascii());
        }
    }

    /// Two of the twelve are written differently in the two scripts, which is
    /// the same reason the solar-term table carries two columns.
    #[test]
    fn two_stations_are_written_differently_in_the_two_scripts() {
        let differing: usize = ChineseStation::ALL
            .iter()
            .filter(|station| station.chinese_name() != station.japanese_name())
            .count();
        assert_eq!(differing, 2);
        assert_eq!(ChineseStation::SHICHEN.chinese_name(), "實沈");
        assert_eq!(ChineseStation::SHICHEN.japanese_name(), "実沈");
        assert_eq!(ChineseStation::SHOUXING.chinese_name(), "壽星");
        assert_eq!(ChineseStation::SHOUXING.japanese_name(), "寿星");
    }

    #[test]
    fn a_station_opens_at_the_same_instant_as_its_opening_term() {
        for year in [1900i64, 1950, 2000, 2024, 2099] {
            for station in ChineseStation::ALL {
                let opening = station_moment(year, station);
                let term = term_moment(year, station.opening_term());
                assert!(
                    (opening.0 - term.0).abs() < 1e-6,
                    "{} of {year}",
                    station.japanese_name()
                );
            }
        }
    }

    #[test]
    fn the_twelve_stations_partition_the_year() {
        for year in 1990..2040 {
            let mut previous: Option<SignPeriod<ChineseStation>> = None;
            let mut count = 0;
            for period in stations_in_year(year, JAPAN) {
                if let Some(earlier) = previous {
                    assert!((earlier.end.0 - period.start.0).abs() < 1e-9);
                    assert_eq!(earlier.end_day.0 + 1, period.start_day.0);
                    assert_eq!(earlier.sign.next(), period.sign);
                }
                assert!(
                    (29..=32).contains(&period.length_days()),
                    "{} of {year} ran {} days",
                    period.sign.japanese_name(),
                    period.length_days()
                );
                previous = Some(period);
                count += 1;
            }
            assert_eq!(count, SIGNS_PER_ZODIAC);
        }
    }

    #[test]
    fn a_years_stations_open_with_xuanxiao_in_january_and_close_with_xingji() {
        let mut iterator = stations_in_year(2024, JAPAN);
        assert_eq!(iterator.len(), 12);
        let first = iterator.next().unwrap();
        assert_eq!(first.sign, ChineseStation::XUANXIAO);
        let (_, month, day) = year_month_day_from_rd(first.start_day);
        assert_eq!(month, 1);
        assert!(day <= 7, "小寒 was on January {day}");
        let last = iterator.last().unwrap();
        assert_eq!(last.sign, ChineseStation::XINGJI);
        assert_eq!(year_month_day_from_rd(last.start_day).1, 12);
    }

    #[test]
    fn the_station_in_effect_agrees_with_the_years_own_table() {
        for period in stations_in_year(2024, JAPAN) {
            assert_eq!(station_on_day(period.start_day, JAPAN), period.sign);
            assert_eq!(station_on_day(period.end_day, JAPAN), period.sign);
            assert_eq!(
                station_on_day(Rd(period.start_day.0 - 1), JAPAN),
                period.sign.previous()
            );
            assert!(period.contains(period.start_day));
        }
    }

    /// Every day of the year sits in exactly one station, and that station is
    /// the one holding the term in effect on that day — the four-pillar
    /// "solar month" of the day. Since a station spans one 節気 and the 中気
    /// after it, the term in effect is always one of those two.
    #[test]
    fn every_day_of_a_year_falls_in_exactly_one_station() {
        let start = from_year_month_day(2024, 1, 1);
        let mut counts = [0i64; SIGNS_PER_ZODIAC];
        for offset in 0..366 {
            let day = Rd(start.0 + offset);
            let period = station_in_effect(day, JAPAN);
            assert!(period.contains(day));
            let term = crate::solar_terms::term_on_day(day, JAPAN);
            assert!(
                term == period.sign.opening_term() || term == period.sign.midpoint_term(),
                "{day} was in {} but its term was {}",
                period.sign.japanese_name(),
                term.japanese_name()
            );
            counts[period.sign.index() as usize] += 1;
        }
        assert_eq!(counts.iter().sum::<i64>(), 366);
        for count in counts {
            assert!((29..=32).contains(&count));
        }
    }

    #[test]
    fn stepping_round_the_stations_returns_to_the_same_one() {
        for station in ChineseStation::ALL {
            assert_eq!(station.next().previous(), station);
            assert_eq!(station.previous().next(), station);
        }
        assert_eq!(ChineseStation::XIMU.next(), ChineseStation::XINGJI);
    }

    #[test]
    fn an_arbitrary_longitude_lands_in_the_station_that_covers_it() {
        assert_eq!(
            ChineseStation::containing_degrees(255.0),
            ChineseStation::XINGJI
        );
        assert_eq!(
            ChineseStation::containing_degrees(284.999),
            ChineseStation::XINGJI
        );
        assert_eq!(
            ChineseStation::containing_degrees(285.0),
            ChineseStation::XUANXIAO
        );
        assert_eq!(
            ChineseStation::containing_degrees(0.0),
            ChineseStation::JIANGLOU
        );
        assert_eq!(
            ChineseStation::containing_degrees(254.999),
            ChineseStation::XIMU
        );
    }

    #[test]
    fn degrees_into_a_station_run_from_zero_to_thirty() {
        for period in stations_in_year(2024, JAPAN) {
            let early = degrees_into_station(Moment(period.start.0 + 0.001));
            let late = degrees_into_station(Moment(period.end.0 - 0.001));
            assert!(early < 0.01, "{early} degrees in just after the opening");
            assert!(late > 29.99, "{late} degrees in just before the next");
            assert_eq!(
                station_at_moment(Moment(period.start.0 + 10.0)),
                period.sign
            );
        }
    }

    /// A station and the tropical sign it is equated with overlap for half
    /// their length and no more, which is the 15° offset made visible in
    /// days.
    #[test]
    fn a_station_and_its_western_counterpart_overlap_for_about_half_a_month() {
        let start = from_year_month_day(2024, 1, 1);
        let mut agreements = 0;
        for offset in 0..366 {
            let day = Rd(start.0 + offset);
            let station = station_on_day(day, JAPAN);
            let sign = crate::zodiac::tropical::sign_on_day(day, JAPAN);
            if station.traditional_zodiac_counterpart() == sign {
                agreements += 1;
            }
        }
        assert!(
            (160..=200).contains(&agreements),
            "{agreements} days of agreement out of 366 is not half"
        );
    }
}
