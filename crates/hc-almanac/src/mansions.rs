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
//! The mansion list, the 四象 grouping and the 二十七宿 reset table are from
//! the National Astronomical Observatory of Japan's 暦Wiki, 二十八宿
//! (<https://eco.mtk.nao.ac.jp/koyomi/wiki/C6F3BDBDC8ACBDC9.html>), which is
//! also the source for "貞享暦以降…年・月・日に対してそれぞれ連続的に割り当て
//! ます" — the statement that the almanac mansion is a counter and not an
//! ephemeris. The Japanese star names (和名) follow 上原貞治「二十八宿和名考」
//! and 精選版日本国語大辞典 via コトバンク; several have variant readings and
//! the article says outright that they differ between manuscripts, so the
//! commoner form is used and the variants are noted. The 吉凶 attribution is
//! a commercial-almanac one and publishers disagree; see [`Mansion::fortune`].
//!
//! Readings that genuinely vary between sources: 斗 is *hikitsuboshi* here
//! and *hitsukiboshi* in 精選版日本国語大辞典; 箕 is *miboshi* here and
//! *minoboshi* in some listings; 房 appears as そひぼし, 室 as はつゐぼし, 胃
//! as えきへぼし and 鬼 as たまをのぼし in historical orthography.

use hc_calendar::Rd;
use hc_calendar::shape::Naming;
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

/// The ways the 28 positions are named, one entry per language or
/// convention. See [`hc_calendar::shape::Naming`].
pub mod namings {
    use hc_calendar::shape::Naming;

    hc_core::catalogue! {
        type: Naming<28>,
        id: |naming| naming.id,
        provenance: |naming| naming.authority,
        tests: mansion_naming_tests,

        /// Every naming this crate ships.
        pub const ALL;
        /// The naming with this identifier.
        pub fn by_id;

        entries: {
            /// The single character, e.g. `"角"`.
            pub const CHARACTER = Naming {
                id: "hani",
                english_name: "Character",
                names: &[
                "角",
                "亢",
                "氐",
                "房",
                "心",
                "尾",
                "箕",
                "斗",
                "牛",
                "女",
                "虚",
                "危",
                "室",
                "壁",
                "奎",
                "婁",
                "胃",
                "昴",
                "畢",
                "觜",
                "参",
                "井",
                "鬼",
                "柳",
                "星",
                "張",
                "翼",
                "軫",
                ],
                authority: "The single character of each 宿, common to Chinese, Japanese and Korean",
            };
            /// The Japanese star name in kana, e.g. `"すぼし"`.
            pub const KANA = Naming {
                id: "ja-kana",
                english_name: "Japanese star names, in kana",
                names: &[
                "すぼし",
                "あみぼし",
                "ともぼし",
                "そいぼし",
                "なかごぼし",
                "あしたれぼし",
                "みぼし",
                "ひきつぼし",
                "いなみぼし",
                "うるきぼし",
                "とみてぼし",
                "うみやめぼし",
                "はついぼし",
                "なまめぼし",
                "とかきぼし",
                "たたらぼし",
                "えきえぼし",
                "すばるぼし",
                "あめふりぼし",
                "とろきぼし",
                "からすきぼし",
                "ちちりぼし",
                "たまおのぼし",
                "ぬりこぼし",
                "ほとおりぼし",
                "ちりこぼし",
                "たすきぼし",
                "みつかけぼし",
                ],
                authority: "The 和名 the 具注暦 and later printed almanacs give; where readings vary the commonest is used",
            };
            /// That name in Hepburn romaji, e.g. `"suboshi"`.
            pub const ROMAJI = Naming {
                id: "ja-latn",
                english_name: "Japanese star names, romanised",
                names: &[
                "suboshi",
                "amiboshi",
                "tomoboshi",
                "soiboshi",
                "nakagoboshi",
                "ashitareboshi",
                "miboshi",
                "hikitsuboshi",
                "inamiboshi",
                "urukiboshi",
                "tomiteboshi",
                "umiyameboshi",
                "hatsuiboshi",
                "namameboshi",
                "tokakiboshi",
                "tataraboshi",
                "ekieboshi",
                "subaruboshi",
                "amefuriboshi",
                "torokiboshi",
                "karasukiboshi",
                "chichiriboshi",
                "tamaonoboshi",
                "nurikoboshi",
                "hotooriboshi",
                "chirikoboshi",
                "tasukiboshi",
                "mitsukakeboshi",
                ],
                authority: "Hepburn romanisation of the kana",
            };
            /// The Sino-Japanese reading of the mansion name, e.g. `"kaku"`.
            pub const ON_READING = Naming {
                id: "ja-on",
                english_name: "Sino-Japanese readings",
                names: &[
                "kaku",
                "kō",
                "tei",
                "bō",
                "shin",
                "bi",
                "ki",
                "to",
                "gyū",
                "jo",
                "kyo",
                "ki",
                "shitsu",
                "heki",
                "kei",
                "rō",
                "i",
                "bō",
                "hitsu",
                "shi",
                "shin",
                "sei",
                "ki",
                "ryū",
                "sei",
                "chō",
                "yoku",
                "shin",
                ],
                authority: "The 音読み of the mansion characters, Hepburn-romanised",
            };
            /// The conventional English name of the asterism, e.g. `"Horn"`.
            pub const ENGLISH = Naming {
                id: "en",
                english_name: "English",
                names: &[
                "Horn",
                "Neck",
                "Root",
                "Room",
                "Heart",
                "Tail",
                "Winnowing Basket",
                "Dipper",
                "Ox",
                "Girl",
                "Emptiness",
                "Rooftop",
                "Encampment",
                "Wall",
                "Legs",
                "Bond",
                "Stomach",
                "Hairy Head",
                "Net",
                "Turtle Beak",
                "Three Stars",
                "Well",
                "Ghost",
                "Willow",
                "Star",
                "Extended Net",
                "Wings",
                "Chariot",
                ],
                authority: "The conventional English names of the asterisms",
            };
        }
    }
}

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

    /// The name of this mansion in one naming — `namings::KANA`,
    /// `namings::ENGLISH` and so on.
    #[must_use]
    pub const fn name(self, naming: &Naming<28>) -> &'static str {
        naming.names[self.0 as usize]
    }

    /// The single character, e.g. `"角"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        self.name(&namings::CHARACTER)
    }

    /// The Japanese star name in kana, e.g. `"すぼし"`.
    #[must_use]
    pub const fn kana(self) -> &'static str {
        self.name(&namings::KANA)
    }

    /// The conventional English name of the asterism, e.g. `"Horn"`.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        self.name(&namings::ENGLISH)
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
    /// # Publishers disagree, and neither national body arbitrates
    ///
    /// The National Astronomical Observatory of Japan's 暦Wiki gives no 吉凶
    /// for the mansions at all, and notes that the 二十八宿 began as a pure
    /// lunar-position coordinate system with no fortune attached. The
    /// National Diet Library gives none either. So this is a commercial
    /// almanac attribution, and the commercial almanacs differ from one
    /// another — materially, not in wording: 壁宿's 凶 is "nothing" in one
    /// published table and "expansion to the south" in another.
    ///
    /// Exactly two entries are agreed by every source consulted: **鬼宿 is
    /// auspicious** — proverbially the best day of the twenty-eight — and
    /// **牛宿 is auspicious** for everything. The remaining twenty-six follow
    /// the commonest Japanese listing (the KOTONOHA 二十八宿一覧 and
    /// うまずたゆまず tables). Treat the other twenty-six as one publisher's
    /// reading rather than as a fact about the tradition.
    #[must_use]
    pub const fn fortune(self) -> Fortune {
        match self.0 {
            // 角 氐 房 尾 箕 斗 牛 室 壁 奎 婁 胃 昴 畢 参 井 鬼 張 軫
            0 | 2 | 3 | 5 | 6 | 7 | 8 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 20 | 21 | 22 | 25
            | 27 => Fortune::Auspicious,
            // 亢 心 女 虚 危 觜 柳 星 翼
            _ => Fortune::Inauspicious,
        }
    }

    /// Whether the 吉凶 of this mansion is one every source agrees on.
    ///
    /// True only for 鬼宿 and 牛宿. See [`Mansion::fortune`].
    #[must_use]
    pub const fn fortune_is_undisputed(self) -> bool {
        self.0 == Self::GHOST.0 || self.0 == OX_INDEX
    }

    /// What the tradition says about this mansion's day, where every source
    /// agrees.
    ///
    /// `None` for twenty-six of the twenty-eight. The per-mansion lists of
    /// undertakings that commercial almanacs print — "good for marriage, bad
    /// for funerals" and so on — diverge enough between publishers that
    /// shipping one of them as *the* list would be inventing a tradition
    /// rather than reporting one. This crate therefore ships only the two
    /// statements that every consulted source makes, and leaves the rest as
    /// a documented gap. See the crate README.
    #[must_use]
    pub const fn undisputed_note(self) -> Option<&'static str> {
        match self.0 {
            22 => Some("the best day of the twenty-eight for every undertaking but marriage"),
            OX_INDEX => Some("auspicious in all things"),
            _ => None,
        }
    }
}

/// The anchor of the Japanese almanac's twenty-eight-day mansion cycle.
///
/// RD 738_886 is 1 January 2024, and the Japanese almanacs for that year
/// print 畢宿 (index 18) against it.
///
/// The cycle has run without a break since the 貞享 reform, so one dated
/// observation fixes it for all time. The historical epoch is recorded: the
/// Japanese Wikipedia article 二十八宿 states that 貞享2年正月朔日 — a
/// Sunday, 4 February 1685 Gregorian, RD 615_104 — was set to 星宿, and
/// adds that the reason for that particular choice is unknown. The anchor
/// below reproduces it, which the tests check; the 明治 reform banned the
/// 暦注 from official calendars but did not renumber the cycle.
///
/// The constant is stated as a day and a mansion rather than as a raw
/// offset so that the citation can be checked against a published almanac.
pub const CYCLE_ANCHOR: Rd = Rd(738_886);

/// The mansion the cycle anchor carries: 畢宿.
pub const CYCLE_ANCHOR_MANSION: Mansion = Mansion(18);

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

    /// The name of this mansion in one naming, from the shared table.
    #[must_use]
    pub const fn name(self, naming: &Naming<28>) -> &'static str {
        self.to_twenty_eight().name(naming)
    }

    /// The single character, e.g. `"角"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        self.name(&namings::CHARACTER)
    }
}

/// The mansion on the first day of each lunisolar month in the 宿曜道
/// scheme, by month number.
///
/// 「朔日の宿はあらかじめ下表のように配当されており、朔日の後は（牛宿を除いて）
/// 毎日順番に配当されます。また朔日の宿＝その月の宿です。」 — National
/// Astronomical Observatory of Japan, 暦Wiki 二十八宿. Index 0 is the first
/// lunisolar month, and the values are positions in the *twenty-seven*
/// cycle, not the twenty-eight.
const MONTH_FIRST_DAY_MANSION: [u8; 12] = [
    11, // 正月 室宿
    13, // 二月 奎宿
    15, // 三月 胃宿
    17, // 四月 畢宿
    19, // 五月 参宿
    21, // 六月 鬼宿
    24, // 七月 張宿
    0,  // 八月 角宿
    2,  // 九月 氐宿
    4,  // 十月 心宿
    7,  // 十一月 斗宿
    9,  // 十二月 虚宿
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
    use hc_calendar::Weekday;

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

    /// The 暦注 cycle keeps 牛宿, in 北方玄武, and puts 奎宿 in 西方白虎 —
    /// the grouping is the astronomical one, unchanged. It is the
    /// twenty-seven-mansion scheme that drops 牛.
    #[test]
    fn the_quadrants_open_with_the_mansions_they_are_named_for() {
        assert_eq!(Mansion::from_index(0).quadrant(), Quadrant::AzureDragon);
        assert_eq!(Mansion::from_index(6).quadrant(), Quadrant::AzureDragon);
        assert_eq!(Mansion::from_index(7).quadrant(), Quadrant::BlackTortoise);
        assert_eq!(Mansion::OX.quadrant(), Quadrant::BlackTortoise);
        assert_eq!(Mansion::OX.japanese_name(), "牛");
        assert_eq!(Mansion::from_index(13).quadrant(), Quadrant::BlackTortoise);
        assert_eq!(Mansion::from_index(14).japanese_name(), "奎");
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

    /// The historical epoch. 貞享2年正月朔日 — the day the 貞享暦 took effect,
    /// 4 February 1685 Gregorian, RD 615_104 — was set to 星宿, and it was a
    /// Sunday. The Japanese Wikipedia article 二十八宿 records both facts and
    /// says the reason for choosing 星宿 is unknown. Reproducing the mansion
    /// *and* the weekday 339 years back is the strongest available evidence
    /// that the count has run unbroken, including across the 明治 reform that
    /// banned the 暦注 from official calendars.
    #[test]
    fn the_cycle_reproduces_the_1685_epoch_three_centuries_back() {
        let epoch = Rd(615_104);
        assert_eq!(Weekday::from_rd(epoch), Weekday::Sunday);
        assert_eq!(mansion_of(epoch).japanese_name(), "星");
        assert_eq!(mansion_of(epoch).index(), 24);
    }

    /// Published almanac values: 2024-01-01 is 畢宿, 2025-01-01 参宿,
    /// 2026-01-01 井宿, 2027-01-01 鬼宿 (こよみる daily pages).
    #[test]
    fn the_published_new_year_mansions_match() {
        for (rd, name) in [
            (738_886, "畢"),
            (739_252, "参"),
            (739_617, "井"),
            (739_982, "鬼"),
        ] {
            assert_eq!(mansion_of(Rd(rd)).japanese_name(), name, "RD {rd}");
        }
    }

    /// The published 鬼宿日 for 2024: 5 January, then every twenty-eight days.
    /// Sources: 暦注下段ナビ, こよみる.
    #[test]
    fn the_published_2024_ghost_mansion_days_match() {
        const NEW_YEAR_2024: i64 = 738_886;
        const CUMULATIVE: [i64; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
        let expected = [
            (1, 5),
            (2, 2),
            (3, 1),
            (3, 29),
            (4, 26),
            (5, 24),
            (6, 21),
            (7, 19),
            (8, 16),
            (9, 13),
            (10, 11),
            (11, 8),
            (12, 6),
        ];
        for (month, day) in expected {
            let rd = Rd(NEW_YEAR_2024 + CUMULATIVE[month - 1] + day - 1);
            assert!(
                is_ghost_mansion_day(rd),
                "2024-{month:02}-{day:02} should be 鬼宿日"
            );
        }
        let ghost_days = (0..366)
            .filter(|offset| is_ghost_mansion_day(Rd(NEW_YEAR_2024 + offset)))
            .count();
        assert_eq!(ghost_days, expected.len());
    }

    /// Twenty-eight is four sevens, so the mansion and the weekday are locked
    /// together for ever: 鬼宿日 is always a Friday. And gcd(28, 12) is four,
    /// so 鬼宿 only ever falls on a 子, 辰 or 申 day.
    #[test]
    fn the_mansion_is_permanently_locked_to_the_weekday_and_the_branch() {
        for offset in 0..28 * 12 {
            let day = Rd(CYCLE_ANCHOR.0 + offset);
            if is_ghost_mansion_day(day) {
                assert_eq!(Weekday::from_rd(day), Weekday::Friday, "RD {}", day.0);
                let branch = hc_calendar::cycle::sexagenary_day(day).branch_index();
                assert!(matches!(branch, 0 | 4 | 8), "RD {} branch {branch}", day.0);
            }
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
    fn every_mansion_carries_a_character_a_star_name_and_a_reading() {
        for mansion in Mansion::all() {
            assert_eq!(mansion.name(&namings::CHARACTER).chars().count(), 1);
            assert!(mansion.name(&namings::KANA).ends_with("ぼし"));
            assert!(mansion.name(&namings::ROMAJI).ends_with("boshi"));
            assert!(!mansion.name(&namings::ENGLISH).is_empty());
            assert!(!mansion.name(&namings::ON_READING).is_empty());
        }
    }

    /// Only 鬼宿 and 牛宿 carry a note, because only those two are agreed on
    /// by every source. The rest are a documented gap, not an oversight.
    #[test]
    fn only_the_two_undisputed_mansions_carry_a_note() {
        let noted = Mansion::all()
            .into_iter()
            .filter(|m| m.undisputed_note().is_some())
            .count();
        assert_eq!(noted, 2);
        assert!(Mansion::OX.undisputed_note().is_some());
        assert!(Mansion::GHOST.undisputed_note().is_some());
        for mansion in Mansion::all() {
            assert_eq!(
                mansion.undisputed_note().is_some(),
                mansion.fortune_is_undisputed()
            );
            if mansion.fortune_is_undisputed() {
                assert_eq!(mansion.fortune(), Fortune::Auspicious);
            }
        }
        assert_eq!(Fortune::Auspicious.japanese_name(), "吉");
        assert_eq!(Fortune::Inauspicious.japanese_name(), "凶");
    }

    /// 暦Wiki's own worked examples for the 二十七宿: the fifteenth of the
    /// eighth month and the thirteenth of the ninth are both 婁宿, which is
    /// why 中秋の名月 and 十三夜 share a mansion. The eighth month opens on
    /// 角宿 and the ninth on 氐宿.
    #[test]
    fn the_twenty_seven_mansion_reset_reproduces_the_koyomi_wiki_examples() {
        let eighth_fifteenth = Mansion27::from_index(i64::from(MONTH_FIRST_DAY_MANSION[7]) + 14);
        let ninth_thirteenth = Mansion27::from_index(i64::from(MONTH_FIRST_DAY_MANSION[8]) + 12);
        assert_eq!(eighth_fifteenth.japanese_name(), "婁");
        assert_eq!(ninth_thirteenth.japanese_name(), "婁");
        assert_eq!(
            Mansion27::from_index(i64::from(MONTH_FIRST_DAY_MANSION[7])).japanese_name(),
            "角"
        );
        assert_eq!(
            Mansion27::from_index(i64::from(MONTH_FIRST_DAY_MANSION[0])).japanese_name(),
            "室"
        );
    }

    /// The reset table names each lunisolar month's own mansion, so the
    /// twelve entries must be twelve distinct mansions and none of them 牛.
    #[test]
    fn the_twenty_seven_mansion_reset_table_is_twelve_distinct_mansions() {
        let mut seen = [false; 27];
        for first in MONTH_FIRST_DAY_MANSION {
            assert!(first < MANSION27_COUNT);
            assert!(!seen[first as usize]);
            seen[first as usize] = true;
            assert_ne!(
                Mansion27::from_index(i64::from(first)).to_twenty_eight(),
                Mansion::OX
            );
        }
    }

    /// Published 二十七宿 values, from the 2026 September calendar at
    /// KOYOMI NOTE. Two things show in four consecutive days: 18 September
    /// is 斗 and the 19th is 女, skipping 牛 — the mansion the scheme drops
    /// — and 10 September is 翼 while the 11th jumps back to 角, because the
    /// 11th is 旧暦八月一日 and the table sets the eighth month's first day
    /// to 角.
    #[test]
    fn the_published_september_2026_twenty_seven_mansions_match() {
        for (rd, name) in [
            (739_869, "翼"),
            (739_870, "角"),
            (739_877, "斗"),
            (739_878, "女"),
        ] {
            assert_eq!(
                mansion27_of(Rd(rd), Meridian::JAPAN).japanese_name(),
                name,
                "RD {rd}"
            );
        }
    }

    /// The twenty-seven-mansion cycle is reset at every new moon, and that
    /// reset is *sometimes invisible*: the table's month-to-month step is two
    /// or three, which is exactly what a 29- or 30-day month would give a
    /// free-running 27-cycle anyway. So the count looks continuous for long
    /// stretches and jumps only when a month's length does not match its
    /// table step. What matters is that the jumps happen at all — a
    /// free-running cycle would never produce one — and that they only ever
    /// land on the first of a lunisolar month.
    #[test]
    fn the_twenty_seven_mansion_cycle_restarts_at_every_new_moon() {
        let mut resets = 0;
        for offset in 1..380 {
            let day = Rd(738_886 + offset);
            let today = mansion27_of(day, Meridian::JAPAN);
            let yesterday = mansion27_of(Rd(day.0 - 1), Meridian::JAPAN);
            if today != Mansion27::from_index(i64::from(yesterday.index()) + 1) {
                resets += 1;
                assert_eq!(
                    lunisolar_day(day, Meridian::JAPAN).day,
                    1,
                    "RD {} broke the count away from a new moon",
                    day.0
                );
            }
        }
        assert!(resets > 0, "a free-running cycle would never jump");
    }

    /// The twenty-eight-day cycle has no such reset — that is the difference
    /// between the two schemes, and 暦Wiki says so: 「一般に朔日の宿＝その月
    /// の宿とはなりません」.
    #[test]
    fn the_twenty_eight_mansion_cycle_never_resets() {
        for offset in 1..380 {
            let day = Rd(738_886 + offset);
            assert_eq!(mansion_of(day), mansion_of(Rd(day.0 - 1)).next());
        }
    }
}
