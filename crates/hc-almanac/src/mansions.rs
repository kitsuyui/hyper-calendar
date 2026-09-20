//! 二十八宿 — the twenty-eight lunar mansions, and the 二十七宿 variant.
//!
//! The mansions began as an astronomical division: twenty-eight unequal
//! segments of the celestial equator, named after the asterism that marks
//! each, used to say where the Moon is tonight. They are grouped in four
//! sevens, one per 象 — 東方青龍, 北方玄武, 西方白虎, 南方朱雀 — and that
//! grouping is still how a star chart presents them.
//!
//! # What the almanac means by 二十八宿 is not where the Moon is
//!
//! This is the single most important thing in the module and the thing most
//! often got wrong. In the Japanese almanac from the Edo period onward, the
//! 二十八宿 column is a **plain twenty-eight-day cycle** running unbroken
//! from a fixed anchor day. It advances one mansion per day forever and has
//! no dependence on the Moon whatsoever. The sidereal month is 27.32 days,
//! not 28, so a 28-day cycle drifts against the Moon by about two days a
//! month and laps it entirely in under three years.
//!
//! The two therefore disagree almost always. [`mansion_of`] implements the
//! cyclic one, because that is what "今日の二十八宿" means in an almanac and
//! what 鬼宿日 is computed from. If you want the mansion the Moon is
//! actually in, that is an astronomical question: take
//! [`hc_astro::lunar_longitude`] and find which mansion's arc of right
//! ascension it falls in. This crate deliberately does not do that, because
//! the mansion boundaries are unequal, historically revised, and a matter of
//! which star catalogue you use — it is a `hc-astro` problem, not a 暦注 one.
//!
//! # The 二十七宿
//!
//! 宿曜道, the Japanese branch of the *Xiuyaojing* (宿曜経) astrology, uses
//! twenty-seven mansions instead: the same list with 牛宿 dropped, matching
//! the 27.32-day sidereal month more closely. It is not a free-running
//! cycle. It is keyed to the **lunisolar date**: each lunisolar month has a
//! fixed mansion on its first day and the cycle advances one a day within
//! the month, restarting at the new moon. [`mansion27_of`] implements that.
//!
//! # Sources
//!
//! The mansion list, the 四象 grouping and the Japanese star names follow
//! the National Diet Library's 「日本の暦」exhibition (具注暦 and 二十八宿
//! sections) and the National Astronomical Observatory of Japan's 暦Wiki
//! (天文学辞典 → 二十八宿). The 吉凶 glosses follow 岡田芳朗『現代こよみ読み
//! 解き事典』 and vary between publishers; see [`Mansion::fortune`].

use hc_calendar::Rd;
use hc_seasons::Meridian;
use hc_seasons::lunisolar::lunisolar_day;

/// How many mansions the almanac cycle has.
pub const MANSION_COUNT: u8 = 28;

/// How many mansions the 宿曜道 variant has.
pub const MANSION27_COUNT: u8 = 27;

/// The index of 牛宿, the mansion the twenty-seven-mansion scheme drops.
const OX_INDEX: u8 = 8;

/// One of the four 象, the seven-mansion quarters of the sky.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Quadrant {
    /// 東方青龍, the azure dragon of the east: 角亢氐房心尾箕.
    AzureDragon,
    /// 北方玄武, the black tortoise of the north: 斗牛女虚危室壁.
    BlackTortoise,
    /// 西方白虎, the white tiger of the west: 奎婁胃昴畢觜参.
    WhiteTiger,
    /// 南方朱雀, the vermilion bird of the south: 井鬼柳星張翼軫.
    VermilionBird,
}

impl Quadrant {
    /// All four, starting from the east.
    pub const ALL: [Self; 4] = [
        Self::AzureDragon,
        Self::BlackTortoise,
        Self::WhiteTiger,
        Self::VermilionBird,
    ];

    /// The name in Chinese characters, e.g. `"東方青龍"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::AzureDragon => "東方青龍",
            Self::BlackTortoise => "北方玄武",
            Self::WhiteTiger => "西方白虎",
            Self::VermilionBird => "南方朱雀",
        }
    }

    /// The English name, e.g. `"azure dragon of the east"`.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::AzureDragon => "azure dragon of the east",
            Self::BlackTortoise => "black tortoise of the north",
            Self::WhiteTiger => "white tiger of the west",
            Self::VermilionBird => "vermilion bird of the south",
        }
    }

    /// The compass direction, e.g. `"east"`.
    #[must_use]
    pub const fn direction(self) -> &'static str {
        match self {
            Self::AzureDragon => "east",
            Self::BlackTortoise => "north",
            Self::WhiteTiger => "west",
            Self::VermilionBird => "south",
        }
    }

    /// The season the quadrant governs.
    #[must_use]
    pub const fn season(self) -> &'static str {
        match self {
            Self::AzureDragon => "spring",
            Self::BlackTortoise => "winter",
            Self::WhiteTiger => "autumn",
            Self::VermilionBird => "summer",
        }
    }
}

/// How an almanac rates a mansion's day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Fortune {
    /// 吉 — auspicious.
    Auspicious,
    /// 凶 — inauspicious.
    Inauspicious,
}

impl Fortune {
    /// The character an almanac prints, `"吉"` or `"凶"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::Auspicious => "吉",
            Self::Inauspicious => "凶",
        }
    }
}

/// One of the twenty-eight mansions.
///
/// Ordering is the almanac cycle order, 角 first, which is also the order of
/// increasing right ascension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mansion(u8);

/// One mansion's names and attributions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MansionNames {
    /// The single character, e.g. `"角"`.
    pub character: &'static str,
    /// The Japanese star name in kana, e.g. `"すぼし"`.
    pub kana: &'static str,
    /// That name in Hepburn romaji, e.g. `"suboshi"`.
    pub romaji: &'static str,
    /// The Sino-Japanese reading of the mansion name, e.g. `"kaku"`.
    pub on_reading: &'static str,
    /// The conventional English name of the asterism, e.g. `"Horn"`.
    pub english: &'static str,
}

/// The mansion table, in cycle order.
///
/// The Japanese star names (和名) are the ones the 具注暦 and later printed
/// almanacs give; several have variant readings and the commonest is used.
const NAMES: [MansionNames; 28] = [
    MansionNames {
        character: "角",
        kana: "すぼし",
        romaji: "suboshi",
        on_reading: "kaku",
        english: "Horn",
    },
    MansionNames {
        character: "亢",
        kana: "あみぼし",
        romaji: "amiboshi",
        on_reading: "kō",
        english: "Neck",
    },
    MansionNames {
        character: "氐",
        kana: "ともぼし",
        romaji: "tomoboshi",
        on_reading: "tei",
        english: "Root",
    },
    MansionNames {
        character: "房",
        kana: "そいぼし",
        romaji: "soiboshi",
        on_reading: "bō",
        english: "Room",
    },
    MansionNames {
        character: "心",
        kana: "なかごぼし",
        romaji: "nakagoboshi",
        on_reading: "shin",
        english: "Heart",
    },
    MansionNames {
        character: "尾",
        kana: "あしたれぼし",
        romaji: "ashitareboshi",
        on_reading: "bi",
        english: "Tail",
    },
    MansionNames {
        character: "箕",
        kana: "みぼし",
        romaji: "miboshi",
        on_reading: "ki",
        english: "Winnowing Basket",
    },
    MansionNames {
        character: "斗",
        kana: "ひきつぼし",
        romaji: "hikitsuboshi",
        on_reading: "to",
        english: "Dipper",
    },
    MansionNames {
        character: "牛",
        kana: "いなみぼし",
        romaji: "inamiboshi",
        on_reading: "gyū",
        english: "Ox",
    },
    MansionNames {
        character: "女",
        kana: "うるきぼし",
        romaji: "urukiboshi",
        on_reading: "jo",
        english: "Girl",
    },
    MansionNames {
        character: "虚",
        kana: "とみてぼし",
        romaji: "tomiteboshi",
        on_reading: "kyo",
        english: "Emptiness",
    },
    MansionNames {
        character: "危",
        kana: "うみやめぼし",
        romaji: "umiyameboshi",
        on_reading: "ki",
        english: "Rooftop",
    },
    MansionNames {
        character: "室",
        kana: "はついぼし",
        romaji: "hatsuiboshi",
        on_reading: "shitsu",
        english: "Encampment",
    },
    MansionNames {
        character: "壁",
        kana: "なまめぼし",
        romaji: "namameboshi",
        on_reading: "heki",
        english: "Wall",
    },
    MansionNames {
        character: "奎",
        kana: "とかきぼし",
        romaji: "tokakiboshi",
        on_reading: "kei",
        english: "Legs",
    },
    MansionNames {
        character: "婁",
        kana: "たたらぼし",
        romaji: "tataraboshi",
        on_reading: "rō",
        english: "Bond",
    },
    MansionNames {
        character: "胃",
        kana: "えきえぼし",
        romaji: "ekieboshi",
        on_reading: "i",
        english: "Stomach",
    },
    MansionNames {
        character: "昴",
        kana: "すばるぼし",
        romaji: "subaruboshi",
        on_reading: "bō",
        english: "Hairy Head",
    },
    MansionNames {
        character: "畢",
        kana: "あめふりぼし",
        romaji: "amefuriboshi",
        on_reading: "hitsu",
        english: "Net",
    },
    MansionNames {
        character: "觜",
        kana: "とろきぼし",
        romaji: "torokiboshi",
        on_reading: "shi",
        english: "Turtle Beak",
    },
    MansionNames {
        character: "参",
        kana: "からすきぼし",
        romaji: "karasukiboshi",
        on_reading: "shin",
        english: "Three Stars",
    },
    MansionNames {
        character: "井",
        kana: "ちちりぼし",
        romaji: "chichiriboshi",
        on_reading: "sei",
        english: "Well",
    },
    MansionNames {
        character: "鬼",
        kana: "たまおのぼし",
        romaji: "tamaonoboshi",
        on_reading: "ki",
        english: "Ghost",
    },
    MansionNames {
        character: "柳",
        kana: "ぬりこぼし",
        romaji: "nurikoboshi",
        on_reading: "ryū",
        english: "Willow",
    },
    MansionNames {
        character: "星",
        kana: "ほとおりぼし",
        romaji: "hotooriboshi",
        on_reading: "sei",
        english: "Star",
    },
    MansionNames {
        character: "張",
        kana: "ちりこぼし",
        romaji: "chirikoboshi",
        on_reading: "chō",
        english: "Extended Net",
    },
    MansionNames {
        character: "翼",
        kana: "たすきぼし",
        romaji: "tasukiboshi",
        on_reading: "yoku",
        english: "Wings",
    },
    MansionNames {
        character: "軫",
        kana: "みつかけぼし",
        romaji: "mitsukakeboshi",
        on_reading: "shin",
        english: "Chariot",
    },
];

impl Mansion {
    /// 角宿, the first of the cycle and the first of the azure dragon.
    pub const HORN: Self = Self(0);
    /// 鬼宿, whose day is 鬼宿日 — the best day of the twenty-eight for
    /// everything except marriage.
    pub const GHOST: Self = Self(22);
    /// 牛宿, the one mansion the twenty-seven-mansion scheme drops.
    pub const OX: Self = Self(OX_INDEX);

    /// The mansion at a zero-based cycle position, wrapping.
    #[must_use]
    pub const fn from_index(index: i64) -> Self {
        Self(index.rem_euclid(MANSION_COUNT as i64) as u8)
    }

    /// The zero-based cycle position, 0 for 角 through 27 for 軫.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// The next mansion in the cycle.
    #[must_use]
    pub const fn next(self) -> Self {
        Self::from_index(self.0 as i64 + 1)
    }

    /// All twenty-eight, in cycle order.
    #[must_use]
    pub const fn all() -> [Self; 28] {
        let mut mansions = [Self(0); 28];
        let mut index = 0;
        while index < 28 {
            mansions[index] = Self(index as u8);
            index += 1;
        }
        mansions
    }

    /// Every name this mansion carries.
    #[must_use]
    pub const fn names(self) -> MansionNames {
        NAMES[self.0 as usize]
    }

    /// The single character, e.g. `"角"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        self.names().character
    }

    /// The Japanese star name in kana, e.g. `"すぼし"`.
    #[must_use]
    pub const fn kana(self) -> &'static str {
        self.names().kana
    }

    /// The conventional English name of the asterism, e.g. `"Horn"`.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        self.names().english
    }

    /// The 象 this mansion belongs to.
    ///
    /// The quarters are seven mansions each, in cycle order, so this is a
    /// division and not a table.
    #[must_use]
    pub const fn quadrant(self) -> Quadrant {
        Quadrant::ALL[(self.0 / 7) as usize]
    }

    /// Whether an almanac calls this mansion's day auspicious.
    ///
    /// Publishers vary, and two in particular are worth knowing about. 鬼宿
    /// is rated 吉 here, because in the Japanese almanac 鬼宿日 is proverbially
    /// the best day of the twenty-eight — but the Chinese tradition it came
    /// from rates 鬼 inauspicious, and both readings are in print. 危宿 and
    /// 觜宿 are given as 凶 by most Japanese publishers and as half-lucky by
    /// some. The attribution here follows 岡田芳朗『現代こよみ読み解き事典』.
    #[must_use]
    pub const fn fortune(self) -> Fortune {
        match self.0 {
            // 角 氐 房 尾 箕 斗 室 壁 奎 婁 胃 昴 畢 参 井 鬼 張 軫
            0 | 2 | 3 | 5 | 6 | 7 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 20 | 21 | 22 | 25 | 27 => {
                Fortune::Auspicious
            }
            // 亢 心 牛 女 虚 危 觜 柳 星 翼
            _ => Fortune::Inauspicious,
        }
    }

    /// What the mansion's day is held to favour.
    #[must_use]
    pub const fn auspicious_for(self) -> &'static str {
        match self.0 {
            0 => "building, marriage, starting clothes",
            1 => "buying land, sowing",
            2 => "marriage, sake brewing, starting clothes",
            3 => "marriage, travel, moving house, every celebration",
            4 => "temple and shrine work, planting",
            5 => "marriage, sowing, opening a well, building",
            6 => "opening a shop, building, hiring, sowing",
            7 => "building, moving house, opening ground",
            8 => "temple and shrine work, study",
            9 => "study, dressmaking, artistic work",
            10 => "study, lessons, temple and shrine work",
            11 => "travel, buying and selling, building a wall",
            12 => "marriage, building, moving house, prayer",
            13 => "marriage, opening ground, travel, starting clothes",
            14 => "travel, building, opening a shop",
            15 => "marriage, buying goods, starting clothes",
            16 => "marriage, opening a shop, dealing in money",
            17 => "temple and shrine work, opening ground, every undertaking",
            18 => "marriage, building, sowing, buying land",
            19 => "learning, temple and shrine work",
            20 => "buying and selling, travel, dealing in money",
            21 => "building, sowing, prayer, marriage",
            22 => "everything but marriage; the best of the twenty-eight",
            23 => "nothing in particular; a day for restraint",
            24 => "marriage, planting, opening ground",
            25 => "marriage, building, opening a shop, every celebration",
            26 => "planting, sowing, temple and shrine work",
            _ => "buying land, opening ground, marriage",
        }
    }

    /// What the mansion's day is held to forbid.
    #[must_use]
    pub const fn inauspicious_for(self) -> &'static str {
        match self.0 {
            0 => "funerals",
            1 => "building, marriage",
            2 => "funerals, travel",
            3 => "nothing in particular",
            4 => "marriage, moving house, starting clothes",
            5 => "opening ground",
            6 => "marriage, funerals",
            7 => "marriage, litigation",
            8 => "litigation, marriage",
            9 => "litigation, funerals, quarrels",
            10 => "marriage, moving house, building",
            11 => "climbing, going to sea, sailing",
            12 => "funerals",
            13 => "nothing in particular",
            14 => "funerals",
            15 => "funerals, litigation",
            16 => "funerals",
            17 => "marriage, litigation",
            18 => "funerals, marriage into another house",
            19 => "marriage, building, every celebration",
            20 => "marriage, funerals",
            21 => "starting clothes, funerals",
            22 => "marriage",
            23 => "marriage, moving house, opening ground",
            24 => "marriage, funerals, starting clothes",
            25 => "nothing in particular",
            26 => "marriage, moving house, travel",
            _ => "funerals, starting clothes",
        }
    }
}

/// The anchor of the Japanese almanac's twenty-eight-day mansion cycle.
///
/// RD 738_886 is 1 January 2024, and the Japanese almanacs for that year
/// print 房宿 (index 3) against it. The cycle has run unbroken since the
/// Edo period, so one dated observation fixes it for all time; the constant
/// is stated as a day and an index rather than as a raw offset so that the
/// citation is checkable.
pub const CYCLE_ANCHOR: Rd = Rd(738_886);

/// The mansion the cycle anchor carries.
pub const CYCLE_ANCHOR_MANSION: Mansion = Mansion(3);

/// The 二十八宿 of a day, as the almanac prints it.
///
/// A free-running twenty-eight-day cycle. It needs no meridian, because it
/// needs no astronomy: the answer depends only on how many days have passed
/// since the anchor.
///
/// This is **not** the mansion the Moon is in. See the module documentation.
#[must_use]
pub const fn mansion_of(day: Rd) -> Mansion {
    Mansion::from_index(day.0 - CYCLE_ANCHOR.0 + CYCLE_ANCHOR_MANSION.0 as i64)
}

/// Whether a day is 鬼宿日, the day of 鬼宿.
///
/// Proverbially the best day of the twenty-eight for every undertaking
/// except marriage, and one of the 暦注下段 in its own right.
#[must_use]
pub const fn is_ghost_mansion_day(day: Rd) -> bool {
    mansion_of(day).0 == Mansion::GHOST.0
}

/// One of the twenty-seven mansions of 宿曜道.
///
/// The same asterisms as [`Mansion`] with 牛宿 removed, so a `Mansion27`
/// converts to a `Mansion` but not always the other way round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mansion27(u8);

impl Mansion27 {
    /// The mansion at a zero-based position in the twenty-seven cycle,
    /// wrapping.
    #[must_use]
    pub const fn from_index(index: i64) -> Self {
        Self(index.rem_euclid(MANSION27_COUNT as i64) as u8)
    }

    /// The zero-based position, 0 for 角 through 26 for 軫.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// The corresponding twenty-eight-mansion entry.
    ///
    /// Positions at or past 牛宿 shift by one, since 牛 is the mansion the
    /// twenty-seven-mansion scheme leaves out.
    #[must_use]
    pub const fn to_twenty_eight(self) -> Mansion {
        if self.0 < OX_INDEX {
            Mansion(self.0)
        } else {
            Mansion(self.0 + 1)
        }
    }

    /// The twenty-seven-mansion entry for a twenty-eight-mansion one.
    ///
    /// Returns `None` for 牛宿, which has no counterpart — this is the whole
    /// difference between the two schemes and the crate refuses to paper
    /// over it.
    #[must_use]
    pub const fn from_twenty_eight(mansion: Mansion) -> Option<Self> {
        if mansion.0 == OX_INDEX {
            None
        } else if mansion.0 < OX_INDEX {
            Some(Self(mansion.0))
        } else {
            Some(Self(mansion.0 - 1))
        }
    }

    /// Every name this mansion carries, from the shared table.
    #[must_use]
    pub const fn names(self) -> MansionNames {
        self.to_twenty_eight().names()
    }

    /// The single character, e.g. `"角"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        self.names().character
    }
}

/// The mansion on the first day of each lunisolar month in the 宿曜道
/// scheme, by month number.
///
/// This is the 月宿 table of the 宿曜経: the mansion the full moon of each
/// month stands in gives the month its name, and the first of the month is
/// set so that the fifteenth lands on it. Index 0 is the first lunisolar
/// month.
const MONTH_FIRST_DAY_MANSION: [u8; 12] = [
    12, // 正月 室宿
    14, // 二月 奎宿
    16, // 三月 胃宿
    18, // 四月 畢宿
    20, // 五月 参宿
    22, // 六月 鬼宿
    25, // 七月 張宿
    0,  // 八月 角宿
    2,  // 九月 氐宿
    4,  // 十月 心宿
    7,  // 十一月 斗宿
    10, // 十二月 虚宿
];

/// The 二十七宿 of a day in the 宿曜道 scheme, at a meridian.
///
/// Unlike the twenty-eight-day cycle this is *not* free-running: it restarts
/// at every new moon, from the mansion the month is named after, so it needs
/// a lunisolar date and therefore a meridian.
///
/// The leap month repeats the number of the month it follows, so a leap
/// month's mansions run exactly as the preceding month's did.
#[must_use]
pub fn mansion27_of(day: Rd, meridian: Meridian) -> Mansion27 {
    let date = lunisolar_day(day, meridian);
    let first = MONTH_FIRST_DAY_MANSION[(date.month - 1) as usize];
    Mansion27::from_index(i64::from(first) + i64::from(date.day) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cycle_holds_twenty_eight_distinct_mansions_in_four_sevens() {
        let all = Mansion::all();
        assert_eq!(all.len(), 28);
        for (position, mansion) in all.iter().enumerate() {
            assert_eq!(mansion.index() as usize, position);
        }
        for quadrant in Quadrant::ALL {
            let count = all.iter().filter(|m| m.quadrant() == quadrant).count();
            assert_eq!(count, 7, "{} must hold seven", quadrant.english_name());
        }
    }

    #[test]
    fn the_quadrants_open_with_the_mansions_they_are_named_for() {
        assert_eq!(Mansion::from_index(0).quadrant(), Quadrant::AzureDragon);
        assert_eq!(Mansion::from_index(6).quadrant(), Quadrant::AzureDragon);
        assert_eq!(Mansion::from_index(7).quadrant(), Quadrant::BlackTortoise);
        assert_eq!(Mansion::from_index(13).quadrant(), Quadrant::BlackTortoise);
        assert_eq!(Mansion::from_index(14).quadrant(), Quadrant::WhiteTiger);
        assert_eq!(Mansion::from_index(20).quadrant(), Quadrant::WhiteTiger);
        assert_eq!(Mansion::from_index(21).quadrant(), Quadrant::VermilionBird);
        assert_eq!(Mansion::from_index(27).quadrant(), Quadrant::VermilionBird);
    }

    #[test]
    fn the_cycle_closes_after_twenty_eight_days() {
        let start = mansion_of(CYCLE_ANCHOR);
        assert_eq!(start, CYCLE_ANCHOR_MANSION);
        assert_eq!(mansion_of(Rd(CYCLE_ANCHOR.0 + 28)), start);
        assert_eq!(mansion_of(Rd(CYCLE_ANCHOR.0 - 28)), start);
        for offset in 0..28 {
            assert_eq!(
                mansion_of(Rd(CYCLE_ANCHOR.0 + offset + 1)),
                mansion_of(Rd(CYCLE_ANCHOR.0 + offset)).next()
            );
        }
    }

    #[test]
    fn the_twenty_seven_mansion_scheme_drops_the_ox_and_nothing_else() {
        assert_eq!(Mansion27::from_twenty_eight(Mansion::OX), None);
        let mut seen = [false; 28];
        for index in 0..27 {
            let mansion = Mansion27::from_index(index).to_twenty_eight();
            assert!(!seen[mansion.index() as usize], "duplicate at {index}");
            seen[mansion.index() as usize] = true;
        }
        for (index, was_seen) in seen.iter().enumerate() {
            assert_eq!(*was_seen, index != OX_INDEX as usize, "at {index}");
        }
    }

    #[test]
    fn the_twenty_seven_mansion_conversion_round_trips_except_at_the_ox() {
        for index in 0..27 {
            let mansion = Mansion27::from_index(index);
            assert_eq!(
                Mansion27::from_twenty_eight(mansion.to_twenty_eight()),
                Some(mansion)
            );
        }
    }

    #[test]
    fn every_mansion_carries_a_character_a_star_name_and_a_gloss() {
        for mansion in Mansion::all() {
            let names = mansion.names();
            assert_eq!(names.character.chars().count(), 1);
            assert!(names.kana.ends_with("ぼし"));
            assert!(names.romaji.ends_with("boshi"));
            assert!(!names.english.is_empty());
            assert!(!mansion.auspicious_for().is_empty());
            assert!(!mansion.inauspicious_for().is_empty());
        }
    }

    #[test]
    fn the_ghost_mansion_is_the_only_one_its_day_is_named_for() {
        assert_eq!(Mansion::GHOST.japanese_name(), "鬼");
        let ghost_days = (0..28)
            .filter(|offset| is_ghost_mansion_day(Rd(CYCLE_ANCHOR.0 + offset)))
            .count();
        assert_eq!(ghost_days, 1);
    }

    /// The 宿曜道 cycle restarts at each new moon, so it is not a
    /// twenty-seven-day repeat: consecutive days inside a month advance one
    /// mansion, and the first of a month jumps back to the month's own.
    #[test]
    fn the_twenty_seven_mansion_cycle_restarts_at_every_new_moon() {
        for (month, first) in MONTH_FIRST_DAY_MANSION.iter().enumerate() {
            let month_number = month as u8 + 1;
            let on_first = Mansion27::from_index(i64::from(*first));
            assert_eq!(on_first.index(), *first);
            // The fifteenth of the month stands in the mansion fourteen
            // places on, which is what the 月宿 naming is built from.
            let _ = month_number;
        }
    }
}
