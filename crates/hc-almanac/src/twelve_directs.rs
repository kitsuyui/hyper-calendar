//! 十二直 — the twelve "directs", the oldest of the daily annotations.
//!
//! 建除満平定執破危成納開閉. From the Han dynasty onward this was the column
//! a Chinese or Japanese almanac led with: the 中段 of the 具注暦, printed
//! above the 暦注下段 and below the date, and for most of Japanese history
//! the annotation people actually consulted. 六曜 displaced it only in the
//! Meiji period.
//!
//! # The rule, and the one subtlety in it
//!
//! The cycle advances one place per day. What makes it more than a
//! twelve-day week is that it is *anchored to the 節月*, not free-running:
//!
//! > In 正月 — 寅月, the 節月 that opens at 立春 — the day whose earthly
//! > branch is 寅 is 建. In 二月 (卯月, from 啓蟄) the 卯 day is 建. And so
//! > on round the twelve.
//!
//! So the position is `(day branch − 節月 branch) mod 12`, and the whole of
//! the module is that one line plus the table of names.
//!
//! The subtlety is what that implies at a 節気. Both the day branch and the
//! 節月 branch advance by one — the day's every day, the month's at the
//! term — so on the day a sectional term begins, the two increments cancel
//! and **the same direct is printed two days running**. That is not an
//! error and not a fudge; it is a direct consequence of the anchoring, it
//! happens twelve times a year, and it is the reason the 十二直 column of a
//! real almanac is not a clean twelve-day repeat.
//!
//! The converse — a skipped direct — cannot happen, because consecutive 節月
//! branches always differ by exactly one in the same direction as the day
//! branch. The module's tests prove that over a decade of consecutive days.
//!
//! # Sources
//!
//! The rule and the readings are as given by the National Astronomical
//! Observatory of Japan's 暦Wiki (暦の雑節・暦注 → 十二直) and by the National
//! Diet Library's 「日本の暦」exhibition, 具注暦 section. The 吉凶 glosses
//! follow 岡田芳朗『現代こよみ読み解き事典』; they vary in wording between
//! almanac publishers and nothing in this crate depends on them.

use hc_calendar::Rd;
use hc_seasons::Meridian;

use crate::context::DayContext;

/// One of the twelve directs.
///
/// Ordering is the cycle order, 建 first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TwelveDirect {
    /// 建 (たつ) — "establish". The month's own branch day; broadly the best
    /// of the twelve.
    Establish,
    /// 除 (のぞく) — "remove". Good for clearing away, bad for beginnings.
    Remove,
    /// 満 (みつ) — "full". Good for anything that should be filled.
    Full,
    /// 平 (たいら) — "level". Good for anything that should be even.
    Level,
    /// 定 (さだん) — "settle". Good for fixing things in place.
    Settle,
    /// 執 (とる) — "grasp". Good for taking hold, bad for letting go.
    Grasp,
    /// 破 (やぶる) — "break". Good for breaking off, bad for joining.
    Break,
    /// 危 (あやぶ) — "danger". A day for caution.
    Danger,
    /// 成 (なる) — "complete". Good for finishing and for founding.
    Complete,
    /// 納 (おさん) — "store". Good for taking in, bad for giving out.
    Store,
    /// 開 (ひらく) — "open". Good for openings, bad for funerals.
    Open,
    /// 閉 (とづ) — "close". Good for closing, bad for openings.
    Close,
}

/// The twelve in cycle order, 建 first.
const ALL: [TwelveDirect; 12] = [
    TwelveDirect::Establish,
    TwelveDirect::Remove,
    TwelveDirect::Full,
    TwelveDirect::Level,
    TwelveDirect::Settle,
    TwelveDirect::Grasp,
    TwelveDirect::Break,
    TwelveDirect::Danger,
    TwelveDirect::Complete,
    TwelveDirect::Store,
    TwelveDirect::Open,
    TwelveDirect::Close,
];

impl TwelveDirect {
    /// All twelve, in cycle order.
    pub const ALL: [Self; 12] = ALL;

    /// The direct at a zero-based cycle position, wrapping.
    #[must_use]
    pub const fn from_index(index: i64) -> Self {
        ALL[index.rem_euclid(12) as usize]
    }

    /// The zero-based cycle position, 0 for 建 through 11 for 閉.
    #[must_use]
    pub const fn index(self) -> u8 {
        match self {
            Self::Establish => 0,
            Self::Remove => 1,
            Self::Full => 2,
            Self::Level => 3,
            Self::Settle => 4,
            Self::Grasp => 5,
            Self::Break => 6,
            Self::Danger => 7,
            Self::Complete => 8,
            Self::Store => 9,
            Self::Open => 10,
            Self::Close => 11,
        }
    }

    /// The next in the cycle.
    #[must_use]
    pub const fn next(self) -> Self {
        Self::from_index(self.index() as i64 + 1)
    }

    /// The single character the almanac column prints, e.g. `"建"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::Establish => "建",
            Self::Remove => "除",
            Self::Full => "満",
            Self::Level => "平",
            Self::Settle => "定",
            Self::Grasp => "執",
            Self::Break => "破",
            Self::Danger => "危",
            Self::Complete => "成",
            Self::Store => "納",
            Self::Open => "開",
            Self::Close => "閉",
        }
    }

    /// The Japanese reading in kana, e.g. `"たつ"`.
    ///
    /// These are the classical readings the almanacs print, which are verb
    /// forms rather than Sino-Japanese ones: 定 is *sadan*, not *tei*, and
    /// 閉 is *tozu*, not *hei*.
    #[must_use]
    pub const fn kana(self) -> &'static str {
        match self {
            Self::Establish => "たつ",
            Self::Remove => "のぞく",
            Self::Full => "みつ",
            Self::Level => "たいら",
            Self::Settle => "さだん",
            Self::Grasp => "とる",
            Self::Break => "やぶる",
            Self::Danger => "あやぶ",
            Self::Complete => "なる",
            Self::Store => "おさん",
            Self::Open => "ひらく",
            Self::Close => "とづ",
        }
    }

    /// The reading in Hepburn romaji, e.g. `"tatsu"`.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        match self {
            Self::Establish => "tatsu",
            Self::Remove => "nozoku",
            Self::Full => "mitsu",
            Self::Level => "taira",
            Self::Settle => "sadan",
            Self::Grasp => "toru",
            Self::Break => "yaburu",
            Self::Danger => "ayabu",
            Self::Complete => "naru",
            Self::Store => "osan",
            Self::Open => "hiraku",
            Self::Close => "tozu",
        }
    }

    /// A one-line English gloss of the day's character.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Establish => "establish; all things are founded",
            Self::Remove => "remove; the hundred ills are swept away",
            Self::Full => "full; all things are filled",
            Self::Level => "level; all things are made even",
            Self::Settle => "settle; good and ill are fixed",
            Self::Grasp => "grasp; all things are taken hold of",
            Self::Break => "break; all things are broken through",
            Self::Danger => "danger; go carefully",
            Self::Complete => "complete; all things come to fruition",
            Self::Store => "store; all things are gathered in",
            Self::Open => "open; all things are opened through",
            Self::Close => "close; all things are shut up",
        }
    }

    /// What the day is held to favour.
    ///
    /// Publishers word these differently and a few disagree outright — 満 is
    /// given as good for taking medicine by some and bad for it by others —
    /// so treat them as a gloss, not a specification.
    #[must_use]
    pub const fn auspicious_for(self) -> &'static str {
        match self {
            Self::Establish => "building, moving house, opening a shop, travel",
            Self::Remove => "medicine, well-digging, clearing ground, sowing",
            Self::Full => "building, moving house, marriage, new undertakings",
            Self::Level => "marriage, travel, road and wall repair",
            Self::Settle => "marriage, opening a shop, sowing, moving house",
            Self::Grasp => "marriage, building, sowing, taking things in",
            Self::Break => "litigation, setting out to fight, fishing and hunting",
            Self::Danger => "nothing in particular; a day for restraint",
            Self::Complete => "building, opening a shop, sowing, new undertakings",
            Self::Store => "harvest, buying goods, taking money in",
            Self::Open => "building, moving house, marriage, opening anything",
            Self::Close => "settling accounts, building a wall, making a grave",
        }
    }

    /// What the day is held to forbid.
    #[must_use]
    pub const fn inauspicious_for(self) -> &'static str {
        match self {
            Self::Establish => "breaking ground, opening a storehouse",
            Self::Remove => "marriage, breaking ground",
            Self::Full => "breaking ground, funerals",
            Self::Level => "digging, sowing seed",
            Self::Settle => "litigation, travel",
            Self::Grasp => "paying money out, moving house",
            Self::Break => "marriage and every other celebration",
            Self::Danger => "travel, climbing, going to sea",
            Self::Complete => "litigation, argument",
            Self::Store => "marriage, formal introductions",
            Self::Open => "funerals and every other unclean matter",
            Self::Close => "opening a shop, marriage, raising a roof beam",
        }
    }
}

/// The 十二直 of a day, from a context.
///
/// `(day branch − 節月 branch) mod 12`, with 0 landing on 建. See the module
/// documentation for why that produces a repeated day at every 節気.
#[must_use]
pub const fn direct_of_context(context: &DayContext) -> TwelveDirect {
    let day = context.branch_index() as i64;
    let month = context.solar_month().branch_index() as i64;
    TwelveDirect::from_index(day - month)
}

/// The 十二直 of a day at a meridian.
///
/// The meridian decides which day a sectional term falls on, and therefore
/// which day the cycle is re-anchored: Japan and China can print different
/// directs for the same date. See [`crate::context`].
#[must_use]
pub fn direct_of(day: Rd, meridian: Meridian) -> TwelveDirect {
    direct_of_context(&DayContext::new(day, meridian))
}

#[cfg(test)]
mod tests {
    use hc_seasons::solar_terms::term_beginning_on;

    use super::*;

    /// 2024-01-01 (RD 738886) is a 甲子 day inside 節月 子月, so the day
    /// branch and the month branch are both 子 and the direct is 建.
    #[test]
    fn new_years_day_2024_was_a_day_of_establish() {
        assert_eq!(
            direct_of(Rd(738_886), Meridian::JAPAN),
            TwelveDirect::Establish
        );
        assert_eq!(
            direct_of(Rd(738_886), Meridian::JAPAN).japanese_name(),
            "建"
        );
    }

    #[test]
    fn the_directs_form_a_closed_twelve_cycle() {
        let mut direct = TwelveDirect::Establish;
        for _ in 0..12 {
            direct = direct.next();
        }
        assert_eq!(direct, TwelveDirect::Establish);
        for (position, direct) in TwelveDirect::ALL.iter().enumerate() {
            assert_eq!(direct.index() as usize, position);
            assert_eq!(TwelveDirect::from_index(position as i64), *direct);
        }
        assert_eq!(TwelveDirect::from_index(-1), TwelveDirect::Close);
        assert_eq!(TwelveDirect::from_index(12), TwelveDirect::Establish);
    }

    /// Inside a 節月 the cycle is a plain twelve-day repeat.
    #[test]
    fn the_directs_advance_one_a_day_away_from_a_sectional_term() {
        // 2024-01-10 through 2024-01-20 sit entirely inside the 節月 opened
        // by 小寒 on 6 January 2024.
        for offset in 0..10 {
            let day = Rd(738_895 + offset);
            assert_eq!(
                direct_of(day, Meridian::JAPAN),
                direct_of(Rd(day.0 - 1), Meridian::JAPAN).next(),
                "at offset {offset}"
            );
        }
    }

    /// 立春 2024 fell on 4 February (RD 738920). The 節月 branch steps from
    /// 子 to 寅 across 小寒 and 立春, and on each sectional term day the
    /// direct repeats the previous day's.
    #[test]
    fn the_direct_repeats_on_the_day_a_sectional_term_begins() {
        let lichun = Rd(738_920);
        assert!(term_beginning_on(lichun, Meridian::JAPAN).is_some());
        assert_eq!(
            direct_of(lichun, Meridian::JAPAN),
            direct_of(Rd(lichun.0 - 1), Meridian::JAPAN),
        );
    }

    /// The whole point of the module: the cycle repeats a day at every one
    /// of the twelve sectional terms and never skips one, for a century of
    /// consecutive days.
    #[test]
    fn the_directs_repeat_but_never_skip_at_a_sectional_term() {
        let start = Rd(730_120); // 2000-01-01
        let mut repeats = 0;
        let mut previous = direct_of(start, Meridian::JAPAN);
        for offset in 1..=3_653 {
            let day = Rd(start.0 + offset);
            let direct = direct_of(day, Meridian::JAPAN);
            if direct == previous {
                repeats += 1;
            } else {
                assert_eq!(direct, previous.next(), "skipped a direct at RD {}", day.0);
            }
            previous = direct;
        }
        // Ten Gregorian years hold 120 sectional terms, and every one of
        // them repeats exactly one day.
        assert_eq!(repeats, 120);
    }

    /// Every repeated day must be the day a sectional term begins — the
    /// repeat has no other cause.
    #[test]
    fn every_repeated_direct_sits_on_a_sectional_term() {
        let start = Rd(738_886); // 2024-01-01
        for offset in 1..=730 {
            let day = Rd(start.0 + offset);
            if direct_of(day, Meridian::JAPAN) == direct_of(Rd(day.0 - 1), Meridian::JAPAN) {
                let event = term_beginning_on(day, Meridian::JAPAN);
                assert!(
                    event.is_some_and(|event| event.term.is_sectional()),
                    "RD {} repeats a direct without a sectional term",
                    day.0
                );
            }
        }
    }

    #[test]
    fn every_direct_carries_a_name_a_reading_and_a_gloss() {
        for direct in TwelveDirect::ALL {
            assert_eq!(direct.japanese_name().chars().count(), 1);
            assert!(!direct.kana().is_empty());
            assert!(!direct.romaji().is_empty());
            assert!(!direct.english_name().is_empty());
            assert!(!direct.auspicious_for().is_empty());
            assert!(!direct.inauspicious_for().is_empty());
        }
    }
}
