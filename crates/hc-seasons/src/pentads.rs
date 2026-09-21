//! 七十二候 — the 72 pentads.
//!
//! Each of the 24 solar terms divides into three 候 of about five days, so
//! the pentad boundaries are the multiples of 5° of apparent solar longitude.
//! One shared algorithm, exactly as for the terms; what changes is the data.
//!
//! # Two name sets, and why conflating them is the usual mistake
//!
//! The pentads are named after what is supposed to be happening in nature,
//! and what is happening in nature depends on where you are. The classical
//! Chinese set — the one in 逸周書·時訓解, carried into the 宣明暦 that Japan
//! used from 862 to 1684 — describes the climate of the Yellow River valley,
//! and says things that are simply false in Japan: hawks turn into doves,
//! sparrows enter the sea and become clams.
//!
//! Japan therefore rewrote them. Shibukawa Harumi's 貞享暦 (1685) replaced
//! part of the set, and the 略本暦 revision of 1874 — 本朝七十二候 — produced
//! the list Japanese almanacs still print. Of the 72, fewer than half are
//! shared word for word.
//!
//! Both sets are shipped here, selected by [`PentadTradition`]. Neither is
//! the default, because a library that picked one would be asserting
//! something about its caller that it cannot know.
//!
//! # What this module does not claim
//!
//! The 5° division is the modern 定気 one: a pentad is an arc of the
//! ecliptic, so its length in days varies from about 4.7 near perihelion to
//! about 5.3 near aphelion. Pre-1685 Japanese and pre-1645 Chinese almanacs
//! used 平気, equal divisions *in time*, and those give different dates. This
//! module does not implement 平気.

use hc_astro::solar::{seasonal_event, solar_longitude, solar_longitude_after};
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::{ceil, floor};

use crate::meridian::Meridian;
use crate::solar_terms::{SolarTerm, TermOrder};

/// How many degrees of apparent solar longitude one pentad spans.
pub const DEGREES_PER_PENTAD: f64 = 5.0;

/// How many pentads make a year.
pub const PENTADS_PER_YEAR: usize = 72;

/// How many pentads make a solar term.
pub const PENTADS_PER_TERM: usize = 3;

/// A tradition that names the 72 pentads.
///
/// # Why this is a struct and not an enum
///
/// It was an enum of two, with a `PentadNames` struct behind it holding
/// **72 rows of four fields**. Adding a tradition meant two more fields and
/// an edit to every one of those rows, and the field list was a claim that
/// the 七十二候 have been named twice and no more — which is not this
/// crate's to make. The Korean 칠십이후 and several later Japanese
/// revisions name the same 72 intervals.
///
/// So the table is transposed. A tradition is one entry carrying its own 72
/// names, rather than two columns spread across 72 rows, and adding one is
/// adding one entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PentadTradition {
    /// A stable identifier, lowercase.
    pub id: &'static str,
    /// The name of the tradition in English.
    pub english_name: &'static str,
    /// The 72 names in characters, ordered by solar longitude from 春分.
    pub names: &'static [&'static str],
    /// The 72 English glosses, in the same order.
    ///
    /// Glosses, not translations, and nothing in this crate depends on them.
    pub glosses: &'static [&'static str],
    /// Which text the set comes from.
    pub authority: &'static str,
}

/// Where a pentad sits inside its solar term.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PentadPosition {
    /// 初候, the first pentad of the term.
    First,
    /// 次候, the second.
    Second,
    /// 末候, the third and last.
    Third,
}

impl PentadPosition {
    /// The name in characters: `"初候"`, `"次候"` or `"末候"`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::First => "初候",
            Self::Second => "次候",
            Self::Third => "末候",
        }
    }

    /// The Japanese reading in Hepburn romaji.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        match self {
            Self::First => "shokō",
            Self::Second => "jikō",
            Self::Third => "makkō",
        }
    }

    /// The ordinal in English.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::First => "first",
            Self::Second => "second",
            Self::Third => "third",
        }
    }

    /// The position at an offset of 0, 1 or 2 within a term.
    #[must_use]
    const fn from_offset(offset: u8) -> Self {
        match offset {
            0 => Self::First,
            1 => Self::Second,
            _ => Self::Third,
        }
    }
}

/// One of the 72 pentads.
///
/// Ordering is by apparent solar longitude from 0°, matching
/// [`SolarTerm`](crate::SolarTerm): pentad 0 is the first 候 of 春分.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pentad(u8);

hc_core::catalogue! {
    type: PentadTradition,
    id: |tradition| tradition.id,
    provenance: |tradition| tradition.authority,
    tests: pentad_tradition_catalogue,

    /// Every tradition this crate names the pentads in.
    pub const PENTAD_TRADITIONS;

    /// The tradition with this identifier.
    pub fn by_id;

    entries: {
        /// The classical Chinese set, as transmitted through the 宣明暦.
        ///
        /// The 逸周書·時訓解 list, in traditional characters.
        pub const CHINESE = PentadTradition {
            id: "chinese",
            english_name: "Chinese",
            names: &[
                // 春分 (0°)
                "玄鳥至",
                "雷乃発声",
                "始電",
                // 清明 (15°)
                "桐始華",
                "田鼠化為鴽",
                "虹始見",
                // 穀雨 (30°)
                "萍始生",
                "鳴鳩払其羽",
                "戴勝降于桑",
                // 立夏 (45°)
                "螻蟈鳴",
                "蚯蚓出",
                "王瓜生",
                // 小満 (60°)
                "苦菜秀",
                "靡草死",
                "麦秋至",
                // 芒種 (75°)
                "螳螂生",
                "鵙始鳴",
                "反舌無声",
                // 夏至 (90°)
                "鹿角解",
                "蜩始鳴",
                "半夏生",
                // 小暑 (105°)
                "温風至",
                "蟋蟀居壁",
                "鷹乃学習",
                // 大暑 (120°)
                "腐草為蛍",
                "土潤溽暑",
                "大雨時行",
                // 立秋 (135°)
                "涼風至",
                "白露降",
                "寒蝉鳴",
                // 処暑 (150°)
                "鷹乃祭鳥",
                "天地始粛",
                "禾乃登",
                // 白露 (165°)
                "鴻雁来",
                "玄鳥帰",
                "群鳥養羞",
                // 秋分 (180°)
                "雷乃収声",
                "蟄虫坏戸",
                "水始涸",
                // 寒露 (195°)
                "鴻雁来賓",
                "雀入大水為蛤",
                "菊有黄華",
                // 霜降 (210°)
                "豺乃祭獣",
                "草木黄落",
                "蟄虫咸俯",
                // 立冬 (225°)
                "水始氷",
                "地始凍",
                "野鶏入水為蜃",
                // 小雪 (240°)
                "虹蔵不見",
                "天気上騰地気下降",
                "閉塞而成冬",
                // 大雪 (255°)
                "鶡鴠不鳴",
                "虎始交",
                "茘挺出",
                // 冬至 (270°)
                "蚯蚓結",
                "麋角解",
                "水泉動",
                // 小寒 (285°)
                "雁北郷",
                "鵲始巣",
                "野鶏始雊",
                // 大寒 (300°)
                "鶏始乳",
                "鷙鳥厲疾",
                "水沢腹堅",
                // 立春 (315°)
                "東風解凍",
                "蟄虫始振",
                "魚上氷",
                // 雨水 (330°)
                "獺祭魚",
                "候雁北",
                "草木萌動",
                // 啓蟄 (345°)
                "桃始華",
                "倉庚鳴",
                "鷹化為鳩",
            ],
            glosses: &[
                // 春分 (0°)
                "the swallows return",
                "thunder begins to sound",
                "lightning is first seen",
                // 清明 (15°)
                "the paulownia flowers",
                "the field mice turn into quails",
                "rainbows are first seen",
                // 穀雨 (30°)
                "the duckweed begins to grow",
                "the cooing dove preens its wings",
                "the hoopoe alights on the mulberry",
                // 立夏 (45°)
                "the mole crickets chirp",
                "the earthworms surface",
                "the royal gourd puts out shoots",
                // 小満 (60°)
                "the sow thistle flowers",
                "the tender herbs wither",
                "the wheat harvest comes",
                // 芒種 (75°)
                "the mantises hatch",
                "the shrike begins to call",
                "the mockingbird falls silent",
                // 夏至 (90°)
                "the deer shed their antlers",
                "the cicadas begin to sing",
                "the crow-dipper sprouts",
                // 小暑 (105°)
                "the warm wind arrives",
                "the crickets move into the walls",
                "the young hawks learn to fly",
                // 大暑 (120°)
                "the rotting grass turns into fireflies",
                "the soil is damp and the air sultry",
                "heavy rains fall from time to time",
                // 立秋 (135°)
                "the cool wind arrives",
                "the white dew descends",
                "the autumn cicadas sing",
                // 処暑 (150°)
                "the hawk lays out its prey",
                "heaven and earth begin to cool",
                "the grain ripens",
                // 白露 (165°)
                "the wild geese arrive",
                "the swallows leave",
                "the birds lay in their winter store",
                // 秋分 (180°)
                "the thunder ceases",
                "the hibernating insects seal their burrows",
                "the waters begin to dry",
                // 寒露 (195°)
                "the last of the wild geese arrive",
                "the sparrows enter the sea and become clams",
                "the chrysanthemums show yellow flowers",
                // 霜降 (210°)
                "the jackal lays out its prey",
                "the leaves yellow and fall",
                "every hibernating creature lies down",
                // 立冬 (225°)
                "the waters begin to freeze",
                "the ground begins to freeze",
                "the pheasants enter the water and become clams",
                // 小雪 (240°)
                "the rainbows hide away",
                "the breath of heaven rises and that of earth sinks",
                "all is closed up and winter sets in",
                // 大雪 (255°)
                "the snow partridge falls silent",
                "the tigers begin to mate",
                "the broom sedge puts up shoots",
                // 冬至 (270°)
                "the earthworms knot together",
                "the elk shed their antlers",
                "the springs begin to move",
                // 小寒 (285°)
                "the geese turn north",
                "the magpies start to nest",
                "the pheasants begin to call",
                // 大寒 (300°)
                "the hens begin to lay",
                "the birds of prey fly fierce and fast",
                "the ice on the waters is thick and hard",
                // 立春 (315°)
                "the east wind melts the ice",
                "the hibernating creatures begin to stir",
                "the fish rise to the ice",
                // 雨水 (330°)
                "the otter lays out its fish",
                "the wild geese fly north",
                "the grasses and trees put out shoots",
                // 啓蟄 (345°)
                "the peach trees begin to blossom",
                "the orioles sing",
                "the hawk turns into a dove",
            ],
            authority: "逸周書·時訓解, as the 宣明暦 transmitted it",
        };

        /// The Japanese set of the 1874 略本暦 revision, 本朝七十二候.
        ///
        /// In shinjitai.
        pub const JAPANESE = PentadTradition {
            id: "japanese",
            english_name: "Japanese",
            names: &[
                // 春分 (0°)
                "雀始巣",
                "桜始開",
                "雷乃発声",
                // 清明 (15°)
                "玄鳥至",
                "鴻雁北",
                "虹始見",
                // 穀雨 (30°)
                "葭始生",
                "霜止出苗",
                "牡丹華",
                // 立夏 (45°)
                "蛙始鳴",
                "蚯蚓出",
                "竹笋生",
                // 小満 (60°)
                "蚕起食桑",
                "紅花栄",
                "麦秋至",
                // 芒種 (75°)
                "螳螂生",
                "腐草為蛍",
                "梅子黄",
                // 夏至 (90°)
                "乃東枯",
                "菖蒲華",
                "半夏生",
                // 小暑 (105°)
                "温風至",
                "蓮始開",
                "鷹乃学習",
                // 大暑 (120°)
                "桐始結花",
                "土潤溽暑",
                "大雨時行",
                // 立秋 (135°)
                "涼風至",
                "寒蝉鳴",
                "蒙霧升降",
                // 処暑 (150°)
                "綿柎開",
                "天地始粛",
                "禾乃登",
                // 白露 (165°)
                "草露白",
                "鶺鴒鳴",
                "玄鳥去",
                // 秋分 (180°)
                "雷乃収声",
                "蟄虫坏戸",
                "水始涸",
                // 寒露 (195°)
                "鴻雁来",
                "菊花開",
                "蟋蟀在戸",
                // 霜降 (210°)
                "霜始降",
                "霎時施",
                "楓蔦黄",
                // 立冬 (225°)
                "山茶始開",
                "地始凍",
                "金盞香",
                // 小雪 (240°)
                "虹蔵不見",
                "朔風払葉",
                "橘始黄",
                // 大雪 (255°)
                "閉塞成冬",
                "熊蟄穴",
                "鱖魚群",
                // 冬至 (270°)
                "乃東生",
                "麋角解",
                "雪下出麦",
                // 小寒 (285°)
                "芹乃栄",
                "水泉動",
                "雉始雊",
                // 大寒 (300°)
                "款冬華",
                "水沢腹堅",
                "鶏始乳",
                // 立春 (315°)
                "東風解凍",
                "黄鶯睍睆",
                "魚上氷",
                // 雨水 (330°)
                "土脉潤起",
                "霞始靆",
                "草木萌動",
                // 啓蟄 (345°)
                "蟄虫啓戸",
                "桃始笑",
                "菜虫化蝶",
            ],
            glosses: &[
                // 春分 (0°)
                "the sparrows start to nest",
                "the first cherry blossoms open",
                "distant thunder is first heard",
                // 清明 (15°)
                "the swallows return",
                "the wild geese fly north",
                "rainbows are first seen",
                // 穀雨 (30°)
                "the first reeds sprout",
                "the frosts end and the rice seedlings come up",
                "the peonies bloom",
                // 立夏 (45°)
                "the frogs start croaking",
                "the earthworms surface",
                "the bamboo shoots come up",
                // 小満 (60°)
                "the silkworms wake and eat mulberry",
                "the safflower blooms in profusion",
                "the wheat ripens",
                // 芒種 (75°)
                "the mantises hatch",
                "the rotting grass turns into fireflies",
                "the plums turn yellow",
                // 夏至 (90°)
                "the self-heal withers",
                "the irises bloom",
                "the crow-dipper sprouts",
                // 小暑 (105°)
                "the warm wind arrives",
                "the first lotus blossoms open",
                "the young hawks learn to fly",
                // 大暑 (120°)
                "the paulownia sets its seed",
                "the soil is damp and the air sultry",
                "heavy rains fall from time to time",
                // 立秋 (135°)
                "the cool wind arrives",
                "the evening cicadas sing",
                "thick fog drifts",
                // 処暑 (150°)
                "the cotton bolls open",
                "heaven and earth begin to cool",
                "the rice ripens",
                // 白露 (165°)
                "the dew on the grass turns white",
                "the wagtails begin to call",
                "the swallows depart",
                // 秋分 (180°)
                "the thunder ceases",
                "the hibernating insects seal their burrows",
                "the paddy fields are drained",
                // 寒露 (195°)
                "the wild geese arrive",
                "the chrysanthemums bloom",
                "the crickets sing by the door",
                // 霜降 (210°)
                "the first frost falls",
                "light rains fall now and then",
                "the maples and the ivy turn yellow",
                // 立冬 (225°)
                "the sasanqua camellias open",
                "the ground begins to freeze",
                "the daffodils are fragrant",
                // 小雪 (240°)
                "the rainbows hide away",
                "the north wind strips the leaves",
                "the tachibana leaves turn yellow",
                // 大雪 (255°)
                "the sky is shut and winter sets in",
                "the bears retire to their dens",
                "the salmon gather and swim upstream",
                // 冬至 (270°)
                "the self-heal sprouts",
                "the elk shed their antlers",
                "the wheat sprouts under the snow",
                // 小寒 (285°)
                "the parsley flourishes",
                "the springs begin to move",
                "the pheasants begin to call",
                // 大寒 (300°)
                "the butterbur buds open",
                "the ice on the marshes is thick and hard",
                "the hens begin to lay",
                // 立春 (315°)
                "the east wind melts the ice",
                "the bush warbler sings in the mountains",
                "the fish rise to the cracking ice",
                // 雨水 (330°)
                "the rain moistens the soil",
                "the mist begins to linger",
                "the grasses and trees put out shoots",
                // 啓蟄 (345°)
                "the hibernating creatures open their doors",
                "the peach trees begin to smile",
                "the caterpillars become butterflies",
            ],
            authority: "本朝七十二候 of the 1874 略本暦",
        };
    }
}

/// The internal index of 立春初候 at 315°, i.e. how far the almanac ordering
/// is rotated from the longitude ordering.
const BEGINNING_OF_SPRING_PENTAD_INDEX: u8 = 63;

impl Pentad {
    /// The pentad at an internal index, reduced modulo 72.
    const fn at(index: u8) -> Self {
        Self(index % 72)
    }

    /// The pentad at a given index in a given ordering.
    ///
    /// Returns `None` for an index of 72 or more.
    #[must_use]
    pub const fn from_index(order: TermOrder, index: u8) -> Option<Self> {
        if index as usize >= PENTADS_PER_YEAR {
            return None;
        }
        match order {
            TermOrder::SpringEquinoxFirst => Some(Self(index)),
            TermOrder::BeginningOfSpringFirst => {
                Some(Self((index + BEGINNING_OF_SPRING_PENTAD_INDEX) % 72))
            }
        }
    }

    /// This pentad's index in a given ordering, from 0 to 71.
    #[must_use]
    pub const fn index(self, order: TermOrder) -> u8 {
        match order {
            TermOrder::SpringEquinoxFirst => self.0,
            TermOrder::BeginningOfSpringFirst => {
                (self.0 + 72 - BEGINNING_OF_SPRING_PENTAD_INDEX) % 72
            }
        }
    }

    /// The apparent solar longitude, in degrees, at which this pentad begins.
    #[must_use]
    pub const fn solar_longitude_degrees(self) -> f64 {
        self.0 as f64 * DEGREES_PER_PENTAD
    }

    /// The solar term this pentad belongs to.
    #[must_use]
    pub const fn term(self) -> SolarTerm {
        match SolarTerm::from_index(TermOrder::SpringEquinoxFirst, self.0 / 3) {
            Some(term) => term,
            // Unreachable: `self.0 / 3` is at most 23.
            None => SolarTerm::SPRING_EQUINOX,
        }
    }

    /// Whether this is the first, second or third 候 of its term.
    #[must_use]
    pub const fn position(self) -> PentadPosition {
        PentadPosition::from_offset(self.0 % 3)
    }

    /// The pentad's name in characters, in one tradition or the other.
    #[must_use]
    pub const fn name(self, tradition: PentadTradition) -> &'static str {
        tradition.names[self.0 as usize]
    }

    /// A short English gloss of the pentad's name in that tradition.
    #[must_use]
    pub const fn english_name(self, tradition: PentadTradition) -> &'static str {
        tradition.glosses[self.0 as usize]
    }

    /// Whether the two traditions write this pentad the same way.
    #[must_use]
    pub fn is_shared_between_traditions(self) -> bool {
        self.name(CHINESE) == self.name(JAPANESE)
    }

    /// The next pentad, 5° further along the ecliptic, wrapping at 360°.
    #[must_use]
    pub const fn next(self) -> Self {
        Self((self.0 + 1) % 72)
    }

    /// The previous pentad, wrapping at 0°.
    #[must_use]
    pub const fn previous(self) -> Self {
        Self((self.0 + 71) % 72)
    }

    /// All 72 pentads in a given ordering.
    #[must_use]
    pub const fn all(order: TermOrder) -> [Self; PENTADS_PER_YEAR] {
        let mut pentads = [Self(0); PENTADS_PER_YEAR];
        let mut index = 0;
        while index < PENTADS_PER_YEAR {
            pentads[index] = match Self::from_index(order, index as u8) {
                Some(pentad) => pentad,
                // Unreachable: the loop bound is the table length.
                None => Self(0),
            };
            index += 1;
        }
        pentads
    }
}

/// A pentad together with when it began.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PentadEvent {
    /// Which pentad.
    pub pentad: Pentad,
    /// The instant the Sun reached the pentad's longitude, in Universal Time.
    pub moment: Moment,
    /// The day that instant falls on at the meridian it was asked for.
    pub day: Rd,
}

/// The Universal Time instant at which a pentad begins in a Gregorian year.
///
/// # A caveat the solar terms do not have
///
/// At 15° granularity every term falls exactly once in each Gregorian year.
/// At 5° it is tighter: the Sun stands at roughly 280° on 1 January, so in a
/// leap year whose 1 January falls just before that crossing, the 280°
/// pentad (冬至末候) happens twice in the same Gregorian year. This function
/// then returns the first. Use [`pentads_in_year`] to walk a year, which
/// cannot double-count.
#[must_use]
pub fn pentad_moment(year: i64, pentad: Pentad) -> Moment {
    seasonal_event(year, pentad.solar_longitude_degrees())
}

/// The day a pentad begins in a Gregorian year, at a given meridian.
#[must_use]
pub fn pentad_day(year: i64, pentad: Pentad, meridian: Meridian) -> Rd {
    meridian.day_of(pentad_moment(year, pentad))
}

/// A pentad of a Gregorian year as a full event.
#[must_use]
pub fn pentad_event(year: i64, pentad: Pentad, meridian: Meridian) -> PentadEvent {
    let moment = pentad_moment(year, pentad);
    PentadEvent {
        pentad,
        moment,
        day: meridian.day_of(moment),
    }
}

/// The pentad in effect on a day: the most recent one to have begun on or
/// before it.
#[must_use]
pub fn pentad_in_effect(day: Rd, meridian: Meridian) -> PentadEvent {
    let end_of_day = meridian.midnight(Rd(day.0 + 1));
    let longitude = solar_longitude(end_of_day);
    let pentad = Pentad::at(floor(longitude / DEGREES_PER_PENTAD) as u8);
    // A pentad is never longer than about 5.3 days, so eight days back
    // brackets exactly one crossing of its longitude.
    let moment =
        solar_longitude_after(pentad.solar_longitude_degrees(), Moment(end_of_day.0 - 8.0));
    PentadEvent {
        pentad,
        moment,
        day: meridian.day_of(moment),
    }
}

/// The pentad in effect on a day.
#[must_use]
pub fn pentad_on_day(day: Rd, meridian: Meridian) -> Pentad {
    pentad_in_effect(day, meridian).pentad
}

/// The pentad that begins on a day, if one does.
#[must_use]
pub fn pentad_beginning_on(day: Rd, meridian: Meridian) -> Option<PentadEvent> {
    let event = pentad_in_effect(day, meridian);
    if event.day == day { Some(event) } else { None }
}

/// Seventy-two consecutive pentads, beginning with the first one that starts
/// on or after a given day.
///
/// Together they cover one tropical year with no gap and no overlap, which is
/// the sense in which the 72 候 partition the year.
#[must_use]
pub fn pentads_from(day: Rd, meridian: Meridian) -> PentadsFrom {
    let probe = meridian.midnight(day);
    let longitude = solar_longitude(probe);
    PentadsFrom {
        meridian,
        cursor: probe,
        pentad: Pentad::at(ceil(longitude / DEGREES_PER_PENTAD) as u8 % 72),
        remaining: PENTADS_PER_YEAR,
    }
}

/// The 72 pentads of a Gregorian year, in date order.
///
/// The first is 小寒初候 or 冬至末候 depending on where the Sun stands at
/// midnight on 1 January, which is why this is not a fixed rotation of the
/// table.
#[must_use]
pub fn pentads_in_year(year: i64, meridian: Meridian) -> PentadsFrom {
    pentads_from(crate::gregorian::new_year(year), meridian)
}

/// The iterator returned by [`pentads_from`] and [`pentads_in_year`].
#[derive(Debug, Clone, Copy)]
pub struct PentadsFrom {
    meridian: Meridian,
    cursor: Moment,
    pentad: Pentad,
    remaining: usize,
}

impl Iterator for PentadsFrom {
    type Item = PentadEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let pentad = self.pentad;
        let moment = solar_longitude_after(pentad.solar_longitude_degrees(), self.cursor);
        self.remaining -= 1;
        self.pentad = pentad.next();
        // Step a day past the crossing just found, so that the next search
        // cannot return it again through floating-point noise. The following
        // pentad is at least 4.6 days away, so a day is safe.
        self.cursor = Moment(moment.0 + 1.0);
        Some(PentadEvent {
            pentad,
            moment,
            day: self.meridian.day_of(moment),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for PentadsFrom {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::{from_year_month_day, new_year};

    /// Transposing the table cost a guarantee the old shape gave for free.
    ///
    /// `[PentadNames; PENTADS_PER_YEAR]` could not hold 71 rows; a slice
    /// can. So the length is asserted here instead of by the type, which is
    /// the price of a tradition being an entry rather than two columns.
    #[test]
    fn every_tradition_names_all_seventy_two_pentads() {
        for tradition in PENTAD_TRADITIONS {
            assert_eq!(
                tradition.names.len(),
                PENTADS_PER_YEAR,
                "{} names {} pentads",
                tradition.english_name,
                tradition.names.len()
            );
            assert_eq!(
                tradition.glosses.len(),
                PENTADS_PER_YEAR,
                "{} glosses {} pentads",
                tradition.english_name,
                tradition.glosses.len()
            );
        }
    }

    /// No tradition may name two pentads the same, or the name stops
    /// identifying the pentad.
    #[test]
    fn a_traditions_pentad_names_are_distinct() {
        for tradition in PENTAD_TRADITIONS {
            for (index, name) in tradition.names.iter().enumerate() {
                assert!(
                    !tradition.names[..index].contains(name),
                    "{} uses {name} twice",
                    tradition.english_name
                );
            }
        }
    }

    #[test]
    fn seventy_two_pentads_are_three_to_a_term() {
        for term in SolarTerm::all(TermOrder::SpringEquinoxFirst) {
            let mut found = 0;
            for pentad in Pentad::all(TermOrder::SpringEquinoxFirst) {
                if pentad.term() == term {
                    found += 1;
                }
            }
            assert_eq!(found, PENTADS_PER_TERM, "{}", term.japanese_name());
        }
    }

    #[test]
    fn each_terms_three_pentads_are_first_second_and_third() {
        for term in SolarTerm::all(TermOrder::SpringEquinoxFirst) {
            let positions: [PentadPosition; 3] = [
                PentadPosition::First,
                PentadPosition::Second,
                PentadPosition::Third,
            ];
            for (offset, expected) in positions.into_iter().enumerate() {
                let pentad = Pentad::all(TermOrder::SpringEquinoxFirst)
                    [term.index(TermOrder::SpringEquinoxFirst) as usize * 3 + offset];
                assert_eq!(pentad.term(), term);
                assert_eq!(pentad.position(), expected);
            }
        }
    }

    #[test]
    fn a_pentad_starts_where_its_term_does_plus_five_degrees_a_step() {
        for pentad in Pentad::all(TermOrder::SpringEquinoxFirst) {
            let offset = pentad.solar_longitude_degrees() - pentad.term().solar_longitude_degrees();
            let expected = match pentad.position() {
                PentadPosition::First => 0.0,
                PentadPosition::Second => 5.0,
                PentadPosition::Third => 10.0,
            };
            assert!((offset - expected).abs() < 1e-12, "offset {offset}");
        }
    }

    #[test]
    fn the_two_orderings_are_rotations_of_one_another() {
        for index in 0..72u8 {
            let from_equinox = Pentad::from_index(TermOrder::SpringEquinoxFirst, index).unwrap();
            assert_eq!(from_equinox.index(TermOrder::SpringEquinoxFirst), index);
            let from_spring = Pentad::from_index(TermOrder::BeginningOfSpringFirst, index).unwrap();
            assert_eq!(from_spring.index(TermOrder::BeginningOfSpringFirst), index);
        }
        assert_eq!(Pentad::from_index(TermOrder::SpringEquinoxFirst, 72), None);
        let first_of_almanac = Pentad::from_index(TermOrder::BeginningOfSpringFirst, 0).unwrap();
        assert_eq!(first_of_almanac.term(), SolarTerm::BEGINNING_OF_SPRING);
        assert_eq!(first_of_almanac.position(), PentadPosition::First);
        assert_eq!(first_of_almanac.name(JAPANESE), "東風解凍");
    }

    #[test]
    fn stepping_forward_and_back_returns_to_the_same_pentad() {
        for pentad in Pentad::all(TermOrder::SpringEquinoxFirst) {
            assert_eq!(pentad.next().previous(), pentad);
            assert_eq!(pentad.previous().next(), pentad);
        }
    }

    #[test]
    fn every_pentad_has_a_name_and_a_gloss_in_both_traditions() {
        for pentad in Pentad::all(TermOrder::SpringEquinoxFirst) {
            for tradition in [CHINESE, JAPANESE] {
                assert!(!pentad.name(tradition).is_empty());
                assert!(!pentad.english_name(tradition).is_empty());
                assert!(pentad.english_name(tradition).is_ascii());
            }
        }
    }

    /// The point of shipping both sets: they are genuinely different lists.
    /// Fewer than half the 72 are written the same way, and the Chinese set
    /// contains entries — sparrows becoming clams, hawks becoming doves —
    /// that Japan replaced precisely because they are not observations.
    #[test]
    fn the_two_traditions_disagree_about_most_of_the_year() {
        let shared = Pentad::all(TermOrder::SpringEquinoxFirst)
            .iter()
            .filter(|pentad| pentad.is_shared_between_traditions())
            .count();
        assert_eq!(
            shared, 21,
            "{shared} of 72 pentads are written identically in both sets"
        );
        let clams = Pentad::all(TermOrder::SpringEquinoxFirst)
            .iter()
            .find(|pentad| pentad.name(CHINESE) == "雀入大水為蛤")
            .copied();
        let clams = clams.unwrap();
        assert_eq!(clams.name(JAPANESE), "菊花開");
    }

    #[test]
    fn no_pentad_appears_twice_within_one_tradition() {
        let pentads = Pentad::all(TermOrder::SpringEquinoxFirst);
        for (position, pentad) in pentads.iter().enumerate() {
            for other in &pentads[position + 1..] {
                assert_ne!(pentad, other);
            }
        }
    }

    /// Both traditions reuse a handful of phrases at different points in the
    /// year — 水泉動 is 冬至末候 in China and 小寒次候 in Japan — so the name
    /// lists are not sets, and a lookup by name would be ambiguous. Stating
    /// that here stops anyone building one.
    #[test]
    fn a_few_names_are_reused_at_different_points_of_the_year() {
        let mut chinese_duplicates = 0;
        let pentads = Pentad::all(TermOrder::SpringEquinoxFirst);
        for (position, pentad) in pentads.iter().enumerate() {
            for other in &pentads[position + 1..] {
                if pentad.name(CHINESE) == other.name(CHINESE) {
                    chinese_duplicates += 1;
                }
            }
        }
        assert_eq!(chinese_duplicates, 0, "the Chinese set has no repeats");
        // Across the two sets, though, the same phrase turns up at different
        // longitudes.
        let japanese_springs = pentads
            .iter()
            .filter(|pentad| pentad.name(JAPANESE) == "水泉動")
            .count();
        assert_eq!(japanese_springs, 1);
    }

    /// The National Astronomical Observatory of Japan's 暦要項 for 2024 puts
    /// 立春 on 4 February; 東風解凍, its 初候, therefore starts that day.
    #[test]
    fn the_first_pentad_of_a_term_starts_on_the_terms_own_day() {
        for year in [1990i64, 2000, 2024, 2030] {
            for term in SolarTerm::all(TermOrder::BeginningOfSpringFirst) {
                let term_day = crate::solar_terms::term_day(year, term, Meridian::JAPAN);
                let pentad = pentad_on_day(term_day, Meridian::JAPAN);
                assert_eq!(
                    pentad.term(),
                    term,
                    "{} of {year} was not in its own term",
                    term.japanese_name()
                );
                assert_eq!(
                    pentad.position(),
                    PentadPosition::First,
                    "{} of {year} did not start its first pentad",
                    term.japanese_name()
                );
            }
        }
    }

    #[test]
    fn the_seventy_two_pentads_partition_the_year() {
        for year in [1900i64, 2000, 2024, 2100] {
            let mut previous: Option<PentadEvent> = None;
            let mut first: Option<PentadEvent> = None;
            let mut count = 0;
            let mut seen = [false; PENTADS_PER_YEAR];
            for event in pentads_in_year(year, Meridian::JAPAN) {
                let index = event.pentad.index(TermOrder::SpringEquinoxFirst) as usize;
                assert!(!seen[index], "a pentad repeated in {year}");
                seen[index] = true;
                if let Some(earlier) = previous {
                    assert_eq!(event.pentad, earlier.pentad.next(), "a pentad was skipped");
                    let gap = event.moment.0 - earlier.moment.0;
                    assert!(
                        (4.5..=5.5).contains(&gap),
                        "a pentad of {gap} days in {year}"
                    );
                    let day_gap = event.day.0 - earlier.day.0;
                    assert!((4..=6).contains(&day_gap), "a pentad of {day_gap} days");
                } else {
                    first = Some(event);
                }
                previous = Some(event);
                count += 1;
            }
            assert_eq!(count, PENTADS_PER_YEAR);
            assert!(seen.iter().all(|&flag| flag), "not every pentad appeared");
            // Seventy-two five-degree arcs are one full turn of the ecliptic.
            let span = previous.unwrap().moment.0 - first.unwrap().moment.0;
            assert!(
                (355.0..372.0).contains(&span),
                "seventy-one pentads spanned {span} days in {year}"
            );
        }
    }

    #[test]
    fn a_years_pentads_all_start_inside_that_year() {
        for year in [1950i64, 2000, 2024] {
            let start = new_year(year);
            let end = new_year(year + 1);
            for event in pentads_in_year(year, Meridian::JAPAN) {
                assert!(
                    event.day >= start && event.day < end,
                    "a pentad of {year} started outside it"
                );
            }
        }
    }

    #[test]
    fn every_day_of_a_year_belongs_to_the_pentad_that_covers_it() {
        let start = from_year_month_day(2024, 1, 10);
        for offset in 0..365 {
            let day = Rd(start.0 + offset);
            let event = pentad_in_effect(day, Meridian::JAPAN);
            assert!(event.day <= day);
            assert!(
                day.0 - event.day.0 <= 5,
                "a pentad covered {} days",
                day.0 - event.day.0
            );
            // The pentad in effect must be the one whose term is in effect.
            assert_eq!(
                event.pentad.term(),
                crate::solar_terms::term_on_day(day, Meridian::JAPAN)
            );
        }
    }

    #[test]
    fn seventy_two_days_of_a_year_begin_a_pentad() {
        let start = from_year_month_day(2024, 1, 10);
        let mut beginnings = 0;
        for offset in 0..365 {
            if pentad_beginning_on(Rd(start.0 + offset), Meridian::JAPAN).is_some() {
                beginnings += 1;
            }
        }
        assert_eq!(beginnings, 72);
    }

    #[test]
    fn a_pentad_event_and_the_pentad_in_effect_tell_the_same_story() {
        for event in pentads_in_year(2024, Meridian::JAPAN) {
            let in_effect = pentad_in_effect(event.day, Meridian::JAPAN);
            assert_eq!(in_effect.pentad, event.pentad);
            assert!((in_effect.moment.0 - event.moment.0).abs() < 1e-5);
        }
    }

    #[test]
    fn a_named_pentad_of_a_year_lands_on_the_day_the_iterator_gives_it() {
        let pentad = Pentad::from_index(TermOrder::BeginningOfSpringFirst, 0).unwrap();
        let event = pentad_event(2024, pentad, Meridian::JAPAN);
        assert_eq!(event.day, from_year_month_day(2024, 2, 4));
        assert_eq!(
            pentad_day(2024, pentad, Meridian::JAPAN),
            from_year_month_day(2024, 2, 4)
        );
    }

    #[test]
    fn the_iterator_reports_its_own_length() {
        let mut iterator = pentads_in_year(2024, Meridian::JAPAN);
        assert_eq!(iterator.len(), 72);
        let _ = iterator.next();
        assert_eq!(iterator.len(), 71);
    }
}
