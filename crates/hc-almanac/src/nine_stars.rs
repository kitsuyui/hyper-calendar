//! 九星 — the nine stars, for the year, the 節月 and the day.
//!
//! 一白水星 through 九紫火星: a nine-term cycle mapped onto the eight
//! trigrams plus the centre, and through them onto the eight compass
//! directions. It is the backbone of 気学 and 方位 divination, and it is
//! printed in the margin of every Japanese almanac.
//!
//! Three cycles, three rules, in increasing order of difficulty.
//!
//! # 年家九星 — the year star
//!
//! Counts **backward** one star per year: `11 − (year mod 9)`, folded into
//! 1..=9. 2024 is 三碧木星, 2025 二黒土星, 2026 一白水星.
//!
//! The year turns at **立春**, not at New Year. This is not a stylistic
//! choice — こよみのページ computes the 九星 year by 「定気による節切り」, and
//! a published almanac for 2 January 2025 still prints 三碧木星, 2024's star,
//! because 立春 2025 had not yet come. Popular fortune-telling sites that
//! compute from the plain calendar year mis-assign everyone born between 1
//! January and 3 February; [`year_star`] takes a meridian precisely so that
//! it can find 立春.
//!
//! # 月家九星 — the month star
//!
//! Counts backward one star per 節月, from a starting star that depends on
//! which of three groups the year's earthly branch falls in. 子午卯酉 years
//! open at 八白, 辰戌丑未 years at 五黄, 寅申巳亥 years at 二黒.
//!
//! # 日家九星 — the day star, and where this crate stops
//!
//! The day star reverses twice a year.
//!
//! * **陽遁** — forward counting — begins on the 甲子 day *nearest* the
//!   December solstice, and that day is 一白.
//! * **陰遁** — backward counting — begins on the 甲子 day nearest the June
//!   solstice, and that day is 九紫.
//!
//! "Nearest" is doing real work and is the part most sources get wrong. The
//! December solstice of 2023 fell on 22 December; the last 甲子 on or before
//! it was 2 November, fifty days earlier, and the nearest was 1 January
//! 2024, ten days later. Published almanacs print 一白水星 against 1 January
//! 2024, so the rule is *nearest*, not *last*. こよみのページ states it
//! operationally: take the solstice day's sexagenary position; if it is in
//! the first half of the cycle (甲子 0 through 癸巳 29) the switch is the
//! preceding 甲子, otherwise the following one. That is implemented in
//! [`switch_day_near`], and the tests check it against five published switch
//! dates from 2024 to 2026.
//!
//! ## The doubled star at each reversal
//!
//! A normal period is exactly 180 days, and 180 is twenty nines. So a 陽遁
//! period's last day is 九紫 and the 陰遁 that starts the next day is also
//! 九紫; a 陰遁 period ends on 一白 and the 陽遁 begins on 一白. The star is
//! printed twice running. The tests check both directions against published
//! almanac pages (2025-12-20/21 and 2026-06-18/19).
//!
//! ## The 閏, which this crate does not implement
//!
//! Solstice to solstice is about 182.6 days but a period is 180, so the
//! switch day drifts roughly 2.6 days earlier each half-year. Every eleven
//! or twelve years the drift passes thirty days, the nearest 甲子 jumps to
//! the next one, and that period runs **240 days instead of 180**. 240 is
//! not a multiple of nine, so the star sequence cannot close, and almanacs
//! insert a 閏.
//!
//! **There is no agreed rule for that 閏.** こよみのページ says so in as many
//! words — 「日家九星の閏にはいくつもの計算方式が乱立している状態」 — and
//! labels its own scheme 「この計算は独自に考案した方式です」, a method it
//! devised itself. Other published descriptions of the 閏 are not equivalent
//! to it.
//!
//! So this crate **counts continuously from the switch day and applies no
//! 閏 correction**. In a 180-day period — which is every period except
//! roughly one in twenty-three — the answers match published almanacs, and
//! the tests demonstrate that. In a 240-day period the last sixty days will
//! differ from any given publisher, and this crate does not pretend
//! otherwise: [`DayStarPeriod::is_leap_period`] reports when a caller is in
//! one. Under the nearest-甲子 rule, 240-day periods open on 23 November
//! 2019, 24 May 2031 and 26 May 2042.
//!
//! # Sources
//!
//! Japanese Wikipedia 九星 for the 五行, colour and 後天定位 attributions;
//! こよみのページ 「九星の陰遁と陽遁」and its 九星表作成 tool for the three
//! rules and the 節切り year; published almanac pages (こよみる) and 開運道's
//! switch-day tool, which states it follows 天象学会『萬年暦』, for the
//! checks in the tests.

use hc_calendar::Rd;
use hc_calendar::cycle::sexagenary_day;
use hc_calendar::gregorian;
use hc_seasons::Meridian;
use hc_seasons::solar_terms::{SolarTerm, term_day};

use crate::context::DayContext;

/// How many stars there are.
pub const STAR_COUNT: u8 = 9;

/// How many days a normal 陽遁 or 陰遁 period runs for.
///
/// Twenty nines, which is why the star repeats across a reversal.
pub const NORMAL_PERIOD_DAYS: i64 = 180;

/// The sexagenary position above which the *following* 甲子 is nearer than
/// the preceding one.
const NEAREST_CYCLE_MIDPOINT: u8 = 30;

/// One of the nine stars.
///
/// Ordering is 一白 first, which is the order the cycle counts in under
/// 陽遁.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NineStar {
    /// 一白水星 — white, water, north, the trigram 坎.
    OneWhite,
    /// 二黒土星 — black, earth, south-west, the trigram 坤.
    TwoBlack,
    /// 三碧木星 — jade green, wood, east, the trigram 震.
    ThreeJade,
    /// 四緑木星 — green, wood, south-east, the trigram 巽.
    FourGreen,
    /// 五黄土星 — yellow, earth, the centre. No trigram: it is the axis the
    /// other eight turn about.
    FiveYellow,
    /// 六白金星 — white, metal, north-west, the trigram 乾.
    SixWhite,
    /// 七赤金星 — red, metal, west, the trigram 兌.
    SevenRed,
    /// 八白土星 — white, earth, north-east, the trigram 艮.
    EightWhite,
    /// 九紫火星 — purple, fire, south, the trigram 離.
    NinePurple,
}

/// The nine, in counting order.
const ALL: [NineStar; 9] = [
    NineStar::OneWhite,
    NineStar::TwoBlack,
    NineStar::ThreeJade,
    NineStar::FourGreen,
    NineStar::FiveYellow,
    NineStar::SixWhite,
    NineStar::SevenRed,
    NineStar::EightWhite,
    NineStar::NinePurple,
];

impl NineStar {
    /// All nine, 一白 first.
    pub const ALL: [Self; 9] = ALL;

    /// The star at a one-based number, wrapping.
    ///
    /// `1` is 一白 and `9` is 九紫; `10` wraps to 一白 and `0` to 九紫.
    #[must_use]
    pub const fn from_number(number: i64) -> Self {
        ALL[(number - 1).rem_euclid(STAR_COUNT as i64) as usize]
    }

    /// The one-based number, 1 for 一白 through 9 for 九紫.
    #[must_use]
    pub const fn number(self) -> u8 {
        match self {
            Self::OneWhite => 1,
            Self::TwoBlack => 2,
            Self::ThreeJade => 3,
            Self::FourGreen => 4,
            Self::FiveYellow => 5,
            Self::SixWhite => 6,
            Self::SevenRed => 7,
            Self::EightWhite => 8,
            Self::NinePurple => 9,
        }
    }

    /// The name in Japanese characters, e.g. `"一白水星"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::OneWhite => "一白水星",
            Self::TwoBlack => "二黒土星",
            Self::ThreeJade => "三碧木星",
            Self::FourGreen => "四緑木星",
            Self::FiveYellow => "五黄土星",
            Self::SixWhite => "六白金星",
            Self::SevenRed => "七赤金星",
            Self::EightWhite => "八白土星",
            Self::NinePurple => "九紫火星",
        }
    }

    /// The reading in Hepburn romaji, e.g. `"ippaku suisei"`.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        match self {
            Self::OneWhite => "ippaku suisei",
            Self::TwoBlack => "jikoku dosei",
            Self::ThreeJade => "sanpeki mokusei",
            Self::FourGreen => "shiroku mokusei",
            Self::FiveYellow => "goō dosei",
            Self::SixWhite => "roppaku kinsei",
            Self::SevenRed => "shichiseki kinsei",
            Self::EightWhite => "happaku dosei",
            Self::NinePurple => "kyūshi kasei",
        }
    }

    /// The five-phase element, e.g. `"water"`.
    ///
    /// The phase is in the name: 一白**水**星 is the water star.
    #[must_use]
    pub const fn five_phase(self) -> &'static str {
        match self {
            Self::OneWhite => "water",
            Self::TwoBlack | Self::FiveYellow | Self::EightWhite => "earth",
            Self::ThreeJade | Self::FourGreen => "wood",
            Self::SixWhite | Self::SevenRed => "metal",
            Self::NinePurple => "fire",
        }
    }

    /// The colour, e.g. `"white"`.
    ///
    /// Also in the name: 一**白**水星 is the white one. 碧 is the blue-green
    /// of jade, which English has no single word for.
    #[must_use]
    pub const fn colour(self) -> &'static str {
        match self {
            Self::OneWhite | Self::SixWhite | Self::EightWhite => "white",
            Self::TwoBlack => "black",
            Self::ThreeJade => "jade green",
            Self::FourGreen => "green",
            Self::FiveYellow => "yellow",
            Self::SevenRed => "red",
            Self::NinePurple => "purple",
        }
    }

    /// The compass direction of the star's 定位 in the 後天定位盤.
    ///
    /// 五黄 answers `"centre"`, which is not a direction; it is the axis.
    #[must_use]
    pub const fn direction(self) -> &'static str {
        match self {
            Self::OneWhite => "north",
            Self::TwoBlack => "south-west",
            Self::ThreeJade => "east",
            Self::FourGreen => "south-east",
            Self::FiveYellow => "centre",
            Self::SixWhite => "north-west",
            Self::SevenRed => "west",
            Self::EightWhite => "north-east",
            Self::NinePurple => "south",
        }
    }

    /// The trigram of the 後天定位盤, e.g. `"坎"`.
    ///
    /// `None` for 五黄土星, which sits at the centre and has none. The
    /// tradition leaves that empty; this crate does not fill it in.
    #[must_use]
    pub const fn trigram(self) -> Option<&'static str> {
        match self {
            Self::OneWhite => Some("坎"),
            Self::TwoBlack => Some("坤"),
            Self::ThreeJade => Some("震"),
            Self::FourGreen => Some("巽"),
            Self::FiveYellow => None,
            Self::SixWhite => Some("乾"),
            Self::SevenRed => Some("兌"),
            Self::EightWhite => Some("艮"),
            Self::NinePurple => Some("離"),
        }
    }
}

/// Which way the day star is counting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dun {
    /// 陽遁 — counting forward from 一白, from the December solstice.
    Yang,
    /// 陰遁 — counting backward from 九紫, from the June solstice.
    Yin,
}

impl Dun {
    /// The name in Japanese characters, `"陽遁"` or `"陰遁"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::Yang => "陽遁",
            Self::Yin => "陰遁",
        }
    }

    /// The star the period's opening 甲子 day carries.
    #[must_use]
    pub const fn opening_star(self) -> NineStar {
        match self {
            Self::Yang => NineStar::OneWhite,
            Self::Yin => NineStar::NinePurple,
        }
    }
}

/// One 陽遁 or 陰遁 period: where it starts, which way it runs, how long it
/// is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayStarPeriod {
    /// The 甲子 day the period opens on.
    pub start: Rd,
    /// The 甲子 day the next period opens on.
    pub end: Rd,
    /// Which way the count runs.
    pub dun: Dun,
}

impl DayStarPeriod {
    /// How many days the period covers.
    #[must_use]
    pub const fn length_days(self) -> i64 {
        self.end.0 - self.start.0
    }

    /// Whether this is one of the roughly one-in-twenty-three periods that
    /// runs 240 days instead of 180 and therefore needs a 閏.
    ///
    /// This crate applies no 閏 correction, so a `true` here means its day
    /// stars for the last sixty days of the period will differ from a
    /// published almanac's. See the module documentation for why no
    /// correction is applied.
    #[must_use]
    pub const fn is_leap_period(self) -> bool {
        self.length_days() != NORMAL_PERIOD_DAYS
    }

    /// The star on a day inside this period.
    #[must_use]
    pub const fn star_on(self, day: Rd) -> NineStar {
        let elapsed = day.0 - self.start.0;
        match self.dun {
            Dun::Yang => NineStar::from_number(1 + elapsed),
            Dun::Yin => NineStar::from_number(9 - elapsed),
        }
    }
}

/// The 甲子 day nearest a given day, ties resolved toward the later.
///
/// The sexagenary cycle is sixty days, so a 甲子 always lies within thirty
/// days either way. こよみのページ states the rule operationally: if the
/// day's own position is in the first half of the cycle the preceding 甲子
/// is nearer, otherwise the following one. Position 30 — 癸巳 is 29 — is
/// exactly equidistant and goes forward under this reading.
#[must_use]
pub const fn switch_day_near(day: Rd) -> Rd {
    let position = sexagenary_day(day).index();
    if position < NEAREST_CYCLE_MIDPOINT {
        Rd(day.0 - position as i64)
    } else {
        Rd(day.0 + (60 - position as i64))
    }
}

/// The 陽遁 or 陰遁 period a day falls in, at a meridian.
///
/// The meridian decides which day each solstice falls on, and so which 甲子
/// is nearest it.
#[must_use]
pub fn day_star_period(day: Rd, meridian: Meridian) -> DayStarPeriod {
    let year = gregorian::year_from_fixed(day);
    // Four candidate switches bracket any day: the December solstice of the
    // previous year can reach into late January, and the December solstice
    // of the year after cannot start before late November, so this window is
    // wider than it needs to be and cannot miss one.
    let mut switches = [(Rd(i64::MIN / 4), Dun::Yang); 6];
    let mut count = 0;
    for offset in -1..=1 {
        switches[count] = (
            switch_day_near(term_day(
                year + offset,
                SolarTerm::WINTER_SOLSTICE,
                meridian,
            )),
            Dun::Yang,
        );
        count += 1;
        switches[count] = (
            switch_day_near(term_day(
                year + offset,
                SolarTerm::SUMMER_SOLSTICE,
                meridian,
            )),
            Dun::Yin,
        );
        count += 1;
    }
    switches.sort_unstable_by_key(|(start, _)| start.0);

    let mut current = switches[0];
    let mut next = switches[1];
    for window in 0..switches.len() - 1 {
        if switches[window].0.0 <= day.0 && day.0 < switches[window + 1].0.0 {
            current = switches[window];
            next = switches[window + 1];
            break;
        }
    }
    DayStarPeriod {
        start: current.0,
        end: next.0,
        dun: current.1,
    }
}

/// 日家九星 — the day star, at a meridian.
///
/// See the module documentation for the 陽遁 / 陰遁 reversal and for the 閏
/// this crate deliberately does not implement.
#[must_use]
pub fn day_star(day: Rd, meridian: Meridian) -> NineStar {
    day_star_period(day, meridian).star_on(day)
}

/// The 九星 year a day belongs to: the Gregorian year whose 立春 last
/// preceded it.
///
/// A day between 1 January and 立春 belongs to the previous 九星 year, which
/// is why this cannot be read off the Gregorian year alone.
#[must_use]
pub fn nine_star_year(day: Rd, meridian: Meridian) -> i64 {
    let year = gregorian::year_from_fixed(day);
    if day.0 < term_day(year, SolarTerm::BEGINNING_OF_SPRING, meridian).0 {
        year - 1
    } else {
        year
    }
}

/// 年家九星 — the year star of a 九星 year number.
///
/// The count runs backward: `11 − (year mod 9)`, folded into 1..=9.
#[must_use]
pub const fn year_star_of_year(nine_star_year: i64) -> NineStar {
    NineStar::from_number(11 - nine_star_year.rem_euclid(STAR_COUNT as i64))
}

/// 年家九星 — the year star in force on a day, at a meridian.
#[must_use]
pub fn year_star(day: Rd, meridian: Meridian) -> NineStar {
    year_star_of_year(nine_star_year(day, meridian))
}

/// The star the first 節月 of a year opens on, by the year's earthly branch.
///
/// 子午卯酉 years open at 八白, 辰戌丑未 at 五黄, 寅申巳亥 at 二黒. Those
/// three groups are exactly the branch index modulo three, which is why this
/// is arithmetic rather than a twelve-entry table.
const FIRST_MONTH_STAR: [u8; 3] = [8, 5, 2];

/// The earthly branch of a 九星 year.
///
/// The sexagenary year cycle is anchored so that 1984 was 甲子; the branch
/// is therefore `(year − 4) mod 12`.
#[must_use]
pub const fn year_branch_index(nine_star_year: i64) -> u8 {
    (nine_star_year - 4).rem_euclid(12) as u8
}

/// 月家九星 — the month star of a 九星 year and a 節月 number.
///
/// `solar_month` is one-based, 1 for 寅月 which opens at 立春. The count runs
/// backward one star per month.
#[must_use]
pub const fn month_star_of(nine_star_year: i64, solar_month: u8) -> NineStar {
    let group = (year_branch_index(nine_star_year) % 3) as usize;
    let first = FIRST_MONTH_STAR[group] as i64;
    NineStar::from_number(first - (solar_month as i64 - 1))
}

/// 月家九星 — the month star in force on a day, at a meridian.
#[must_use]
pub fn month_star(day: Rd, meridian: Meridian) -> NineStar {
    let context = DayContext::new(day, meridian);
    month_star_of(
        nine_star_year(day, meridian),
        context.solar_month().number(),
    )
}

/// The three stars a printed almanac gives for a day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NineStars {
    /// 年家九星.
    pub year: NineStar,
    /// 月家九星.
    pub month: NineStar,
    /// 日家九星.
    pub day: NineStar,
}

/// All three stars of a day, from a context.
///
/// Cheaper than calling the three functions separately, because the 節月 is
/// already computed.
#[must_use]
pub fn nine_stars_of_context(context: &DayContext) -> NineStars {
    let meridian = context.meridian();
    let year = nine_star_year(context.day(), meridian);
    NineStars {
        year: year_star_of_year(year),
        month: month_star_of(year, context.solar_month().number()),
        day: day_star(context.day(), meridian),
    }
}

/// All three stars of a day at a meridian.
#[must_use]
pub fn nine_stars(day: Rd, meridian: Meridian) -> NineStars {
    nine_stars_of_context(&DayContext::new(day, meridian))
}

#[cfg(test)]
mod tests {
    use super::*;

    const JAPAN: Meridian = Meridian::JAPAN;

    /// 2024-01-01.
    const NEW_YEAR_2024: i64 = 738_886;
    /// 2025-01-01.
    const NEW_YEAR_2025: i64 = 739_252;
    /// 2026-01-01.
    const NEW_YEAR_2026: i64 = 739_617;

    #[test]
    fn the_nine_stars_number_from_one_and_wrap() {
        for (position, star) in NineStar::ALL.iter().enumerate() {
            assert_eq!(star.number() as usize, position + 1);
            assert_eq!(NineStar::from_number(position as i64 + 1), *star);
        }
        assert_eq!(NineStar::from_number(10), NineStar::OneWhite);
        assert_eq!(NineStar::from_number(0), NineStar::NinePurple);
        assert_eq!(NineStar::from_number(-1), NineStar::EightWhite);
    }

    /// The colour and the phase are both written in the name, so the name
    /// can check the table: 一白水星 must be white and water.
    #[test]
    fn the_name_carries_the_colour_and_the_phase() {
        for star in NineStar::ALL {
            let name = star.japanese_name();
            assert_eq!(name.chars().count(), 4);
            let phase_character = match star.five_phase() {
                "water" => "水",
                "earth" => "土",
                "wood" => "木",
                "metal" => "金",
                _ => "火",
            };
            assert!(name.contains(phase_character), "{name}");
        }
        assert_eq!(NineStar::FiveYellow.direction(), "centre");
        assert_eq!(NineStar::FiveYellow.trigram(), None);
        for star in NineStar::ALL {
            assert_eq!(star.trigram().is_none(), star == NineStar::FiveYellow);
            assert!(!star.romaji().is_empty());
            assert!(!star.colour().is_empty());
        }
    }

    /// Published year stars: 2024 三碧木星, 2025 二黒土星, 2026 一白水星,
    /// 2027 九紫火星 (こよみる; 気学 references).
    #[test]
    fn the_published_year_stars_match() {
        assert_eq!(year_star_of_year(2024), NineStar::ThreeJade);
        assert_eq!(year_star_of_year(2025), NineStar::TwoBlack);
        assert_eq!(year_star_of_year(2026), NineStar::OneWhite);
        assert_eq!(year_star_of_year(2027), NineStar::NinePurple);
    }

    /// The year star counts backward, one per year, for ever.
    #[test]
    fn the_year_star_counts_backward() {
        for year in 1900..2100 {
            assert_eq!(
                year_star_of_year(year + 1),
                NineStar::from_number(i64::from(year_star_of_year(year).number()) - 1)
            );
        }
    }

    /// The 九星 year turns at 立春, not at New Year. A published almanac for
    /// 2 January 2025 still prints 三碧木星 — 2024's star — because 立春 2025
    /// fell on 3 February.
    #[test]
    fn the_nine_star_year_turns_at_the_beginning_of_spring() {
        let second_of_january = Rd(NEW_YEAR_2025 + 1);
        assert_eq!(nine_star_year(second_of_january, JAPAN), 2024);
        assert_eq!(year_star(second_of_january, JAPAN), NineStar::ThreeJade);

        let lichun = term_day(2025, SolarTerm::BEGINNING_OF_SPRING, JAPAN);
        assert_eq!(nine_star_year(lichun, JAPAN), 2025);
        assert_eq!(year_star(lichun, JAPAN), NineStar::TwoBlack);
        assert_eq!(nine_star_year(Rd(lichun.0 - 1), JAPAN), 2024);
    }

    /// Published month stars: 2025-12-21 is 一白水星 and 2025-01-02 is
    /// 四緑木星 (こよみる daily pages). The second is the interesting one —
    /// it is still 甲辰年 for 九星 purposes, and still 子月, because 小寒 2025
    /// fell on 5 January.
    #[test]
    fn the_published_month_stars_match() {
        let solstice_switch = Rd(739_606); // 2025-12-21
        assert_eq!(month_star(solstice_switch, JAPAN), NineStar::OneWhite);

        let second_of_january = Rd(NEW_YEAR_2025 + 1);
        assert_eq!(month_star(second_of_january, JAPAN), NineStar::FourGreen);
    }

    /// The three year-groups open the first 節月 at 八白, 五黄 and 二黒, and
    /// the count runs backward through the twelve 節月.
    #[test]
    fn the_month_star_counts_backward_from_its_year_group() {
        for (year, first) in [(2024, 5u8), (2025, 2), (2026, 8)] {
            assert_eq!(month_star_of(year, 1).number(), first, "year {year}");
            for month in 1..12u8 {
                assert_eq!(
                    month_star_of(year, month + 1),
                    NineStar::from_number(i64::from(month_star_of(year, month).number()) - 1),
                    "year {year} month {month}"
                );
            }
        }
        // 2024 is 甲辰, 2025 乙巳, 2026 丙午.
        assert_eq!(year_branch_index(2024), 4);
        assert_eq!(year_branch_index(2025), 5);
        assert_eq!(year_branch_index(2026), 6);
    }

    /// The nearest 甲子 to the December solstice of 2023 is ten days after
    /// it, not fifty days before it. Published almanacs print 一白水星 — the
    /// 陽遁 opening star — against 1 January 2024, which settles the rule.
    #[test]
    fn the_switch_is_the_nearest_sexagenary_head_not_the_preceding_one() {
        let solstice = term_day(2023, SolarTerm::WINTER_SOLSTICE, JAPAN);
        assert_eq!(solstice, Rd(738_876)); // 2023-12-22
        let switch = switch_day_near(solstice);
        assert_eq!(switch, Rd(NEW_YEAR_2024));
        assert_eq!(switch.0 - solstice.0, 10);
        assert_eq!(sexagenary_day(switch).index(), 0);
        // The preceding 甲子 was fifty days earlier and is not the switch.
        assert_eq!(sexagenary_day(Rd(738_826)).index(), 0);
    }

    /// The published switch days for 2024 to 2026, with their offsets from
    /// the solstice. Sources: 開運道's 九星陽遁陰遁切替日 tool, which states it
    /// follows 天象学会『萬年暦』, cross-checked against こよみる's daily
    /// pages.
    #[test]
    fn the_published_switch_days_match() {
        for (year, term, expected, offset) in [
            (2024, SolarTerm::SUMMER_SOLSTICE, 739_066, 8),
            (2024, SolarTerm::WINTER_SOLSTICE, 739_246, 5),
            (2025, SolarTerm::SUMMER_SOLSTICE, 739_426, 3),
            (2025, SolarTerm::WINTER_SOLSTICE, 739_606, -1),
            (2026, SolarTerm::SUMMER_SOLSTICE, 739_786, -2),
        ] {
            let solstice = term_day(year, term, JAPAN);
            let switch = switch_day_near(solstice);
            assert_eq!(switch, Rd(expected), "{year} {}", term.japanese_name());
            assert_eq!(switch.0 - solstice.0, offset, "{year}");
            assert_eq!(sexagenary_day(switch).index(), 0);
        }
    }

    /// The whole point of the module. Published almanac pages give
    /// 2025-12-20 (癸亥) as 一白水星 and 2025-12-21 (甲子) as 一白水星 — the
    /// last day of a 陰遁 and the first of a 陽遁 carry the same star. The
    /// June reversal mirrors it: 2026-06-18 (癸亥) and 2026-06-19 (甲子) are
    /// both 九紫火星.
    #[test]
    fn the_day_star_repeats_across_each_reversal() {
        let december_eve = Rd(739_605); // 2025-12-20, 癸亥
        let december_switch = Rd(739_606); // 2025-12-21, 甲子
        assert_eq!(sexagenary_day(december_eve).index(), 59);
        assert_eq!(sexagenary_day(december_switch).index(), 0);
        assert_eq!(day_star_period(december_eve, JAPAN).dun, Dun::Yin);
        assert_eq!(day_star_period(december_switch, JAPAN).dun, Dun::Yang);
        assert_eq!(day_star(december_eve, JAPAN), NineStar::OneWhite);
        assert_eq!(day_star(december_switch, JAPAN), NineStar::OneWhite);

        let june_eve = Rd(739_785); // 2026-06-18, 癸亥
        let june_switch = Rd(739_786); // 2026-06-19, 甲子
        assert_eq!(day_star_period(june_eve, JAPAN).dun, Dun::Yang);
        assert_eq!(day_star_period(june_switch, JAPAN).dun, Dun::Yin);
        assert_eq!(day_star(june_eve, JAPAN), NineStar::NinePurple);
        assert_eq!(day_star(june_switch, JAPAN), NineStar::NinePurple);
    }

    /// Published day stars: 2024-01-01 一白水星, 2025-01-02 八白土星,
    /// 2026-01-01 三碧木星 (こよみる daily pages).
    #[test]
    fn the_published_day_stars_match() {
        assert_eq!(day_star(Rd(NEW_YEAR_2024), JAPAN), NineStar::OneWhite);
        assert_eq!(day_star(Rd(NEW_YEAR_2025 + 1), JAPAN), NineStar::EightWhite);
        assert_eq!(day_star(Rd(NEW_YEAR_2026), JAPAN), NineStar::ThreeJade);
    }

    /// Inside a period the star advances by exactly one each day, forward
    /// under 陽遁 and backward under 陰遁, with no exception.
    #[test]
    fn the_day_star_steps_one_a_day_within_a_period() {
        for offset in 0..400 {
            let day = Rd(NEW_YEAR_2025 + offset);
            let next = Rd(day.0 + 1);
            let period = day_star_period(day, JAPAN);
            if day_star_period(next, JAPAN).start == period.start {
                let step = match period.dun {
                    Dun::Yang => 1,
                    Dun::Yin => -1,
                };
                assert_eq!(
                    day_star(next, JAPAN),
                    NineStar::from_number(i64::from(day_star(day, JAPAN).number()) + step),
                    "RD {}",
                    day.0
                );
            }
        }
    }

    /// The two reversals a year are the only days the star fails to step,
    /// and on those days it repeats rather than skipping.
    #[test]
    fn the_only_repeats_are_the_two_reversals_a_year() {
        let mut repeats = 0;
        for offset in 0..3_653 {
            let day = Rd(NEW_YEAR_2025 + offset);
            let next = Rd(day.0 + 1);
            let period = day_star_period(day, JAPAN);
            let following = day_star_period(next, JAPAN);
            if period.start != following.start {
                repeats += 1;
                assert_eq!(sexagenary_day(next).index(), 0, "RD {} is not 甲子", next.0);
                assert_ne!(period.dun, following.dun);
                if !period.is_leap_period() {
                    assert_eq!(
                        day_star(day, JAPAN),
                        day_star(next, JAPAN),
                        "RD {} does not repeat",
                        day.0
                    );
                }
            }
        }
        // Ten years hold twenty reversals — nineteen if the December switch
        // at the end of the window has drifted into the following January,
        // which it does whenever the nearest 甲子 falls after the solstice.
        assert!(
            matches!(repeats, 19 | 20),
            "expected nineteen or twenty reversals, got {repeats}"
        );
    }

    /// Every period runs 180 days, except the roughly one in twenty-three
    /// that runs 240 and needs a 閏 this crate does not implement. Over
    /// 2021–2030 there should be none; under the nearest-甲子 rule the
    /// 240-day periods either side open on 23 November 2019 and 24 May 2031.
    #[test]
    fn periods_run_a_hundred_and_eighty_days_except_at_a_leap() {
        // RD 737_791 is 2021-01-01; the loop runs to the end of 2030.
        for offset in 0..3_652 {
            let period = day_star_period(Rd(737_791 + offset), JAPAN);
            assert!(
                !period.is_leap_period(),
                "RD {} sits in a {}-day period",
                period.start.0,
                period.length_days()
            );
        }
        // The 陽遁 that opened on 2019-11-23 ran until the June solstice
        // switch of 2020-07-20 — 240 days, because the offset had drifted
        // past thirty and the nearest 甲子 jumped a whole sexagenary cycle.
        // The crate reports that rather than pretending otherwise.
        let inside_2020 = day_star_period(Rd(737_425), JAPAN); // 2020-01-01
        assert_eq!(inside_2020.dun, Dun::Yang);
        assert!(inside_2020.is_leap_period());
        assert_eq!(inside_2020.length_days(), 240);
        assert_eq!(inside_2020.start, Rd(737_386));
    }

    /// Both opening days are 甲子, and each carries its period's opening
    /// star.
    #[test]
    fn each_period_opens_on_its_own_star() {
        for offset in 0..1_200 {
            let period = day_star_period(Rd(NEW_YEAR_2024 + offset), JAPAN);
            assert_eq!(sexagenary_day(period.start).index(), 0);
            assert_eq!(day_star(period.start, JAPAN), period.dun.opening_star());
        }
        assert_eq!(Dun::Yang.opening_star(), NineStar::OneWhite);
        assert_eq!(Dun::Yin.opening_star(), NineStar::NinePurple);
        assert_eq!(Dun::Yang.japanese_name(), "陽遁");
        assert_eq!(Dun::Yin.japanese_name(), "陰遁");
    }

    #[test]
    fn the_three_stars_agree_with_the_individual_functions() {
        for offset in [0, 40, 100, 200, 300] {
            let day = Rd(NEW_YEAR_2025 + offset);
            let all = nine_stars(day, JAPAN);
            assert_eq!(all.year, year_star(day, JAPAN));
            assert_eq!(all.month, month_star(day, JAPAN));
            assert_eq!(all.day, day_star(day, JAPAN));
        }
    }
}
