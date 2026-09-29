//! 方位神 — the deities of the directions: the 八将神 and the 金神.
//!
//! The almanac opens with a chart of the year's directions: where 歳徳神
//! stands, whose direction is the 恵方 ([`crate::lucky_direction`]), and
//! where the gods to be avoided stand. The National Diet Library's
//! 「日本の暦」, 干支② 方位神 (`ndl-koyomi-hoijin`, read 2026-09-29),
//! names them: the 八将神, eight gods whose directions follow the year's
//! earthly branch, 太歳 on the branch itself and 歳破 opposite it; and
//! 金神, "whose direction is extremely unlucky".
//! `docs/systems/east-asian-folk-days.md` describes them.
//!
//! # The tables
//!
//! * **八将神**, by the year's branch: Japanese Wikipedia's 「八将神」
//!   (`wikipedia-ja-hasshojin`, read 2026-09-29) tabulates all eight for
//!   the twelve years. いい日本再発見's table (`iinippon-daishogun`, read
//!   2026-09-29) agrees but for 歳刑神, where its row runs 卯 辰 巳 午 未
//!   申 辰 酉 戌 亥 子 丑, one branch on each year but at 午; 古文書ネット
//!   (`komonjyo-hasshojin`, read 2026-09-29) gives 2026, a 午 year, 午 for
//!   歳刑神 as Japanese Wikipedia does, and every other god as both
//!   tables do. Japanese Wikipedia's row is the classical 三刑, and it is
//!   carried; the other is named here and not yet carried, because it is
//!   taken for a copying slip that no second source shares.
//! * **金神** (巡金神), by the year's heavenly stem: Japanese Wikipedia's
//!   「金神」 (`wikipedia-ja-konjin`) and 古文書ネット's 「金神」
//!   (`komonjyo-konjin`, which cites 伊東和彦『暦を知る事典』, not read)
//!   give the same five rows; 古文書ネット gives 2025, 乙巳, 辰 and 巳.
//! * **大金神** and **姫金神**, always opposite each other, by the year's
//!   branch: Japanese Wikipedia's 「金神」 alone; 歳事暦 (`saijigoyomi-hoi`,
//!   read 2026-09-29) describes them without a table.
//!
//! # Which year
//!
//! いい日本再発見 says its table is 節区切り, the year running from 立春 to
//! the next 節分; [`year_pillar`] turns there. The functions of a year
//! take its 干支 and leave the boundary to the caller.
//!
//! # The 遊行 and 間日
//!
//! 大将軍 and 金神 leave their directions for five days at a time, their
//! 遊行, and the directions they leave may then be crossed; the one they
//! go to may not. Each source's reading that states a whole rule is a
//! [`WanderingRule`] of its own ([`docs/policy.md`] §5), none a default;
//! [`WanderingRule::whereabouts_on`] says where the god is on a day and
//! [`WanderingRule::stands_in`] whether it stands in a direction.
//!
//! | Identifier | Rule | Stated by |
//! | --- | --- | --- |
//! | `daishogun-iinippon` | 大将軍: five days from every 甲子 east, 丙子 south, 戊子 the middle of the house, 庚子 west, 壬子 north | いい日本再発見 (`iinippon-daishogun`, read 2026-09-29), its list of 2026 |
//! | `konjin-wikipedia-begun-in-season`, `konjin-wikipedia-days-in-season` | 金神: five days from every 甲寅 to 午, 丙寅 to 酉, 戊寅 the middle of the house, 庚寅 to 子, 壬寅 to 卯; and in spring from 乙卯 to 卯, in summer from 丙午 to 午, in autumn from 辛酉 to 酉, in winter from 壬子 to 子 | Japanese Wikipedia 「金神」 (`wikipedia-ja-konjin`, read 2026-09-29) |
//!
//! いい日本再発見 heads its rows 春, 夏, 秋, 冬 and 土用, and its list of
//! 2026 keeps every row in every sixty days, in June as in December, so
//! the heads name the phase of the direction, not a season of the year;
//! the rule follows the list. The list leaves out the runs to the east,
//! where 大将軍 stands all 2026. Its prose writes the autumn row 庚子～丙辰,
//! seventeen days, where its list has 庚子 to 甲辰, five.
//!
//! Japanese Wikipedia bounds each season from its 立 term to the day
//! before its 土用 (「立春から春の土用まで」), which [`hc_seasons::zassetsu`]
//! gives. It writes the summer direction 「牛」, which is no branch; it is
//! read as 午, the branch of the summer day 丙午, as each other seasonal
//! row goes to its own day's branch. The table does not say whether a
//! seasonal run that begins in its season keeps its five days when they
//! reach the 土用, nor whether the days of a run begun before its season
//! count once the season opens, so the two readings are two rules:
//! `-begun-in-season` gives a run all five of its days when its first day
//! is in the season, and `-days-in-season` gives it only the days the
//! season holds. Nor does it say where 金神 is when two runs overlap: a
//! spring 乙卯 run shares four days with the 甲寅 run before it, a summer
//! 丙午 run its first day with a 壬寅 run, and a winter 壬子 run three days
//! with the 甲寅 run after it, each to another direction; both rules
//! answer `None` on those days.
//!
//! [`konjin_rest_day`] is Japanese Wikipedia's 間日, the days 金神's
//! direction may be crossed: 丑 days in spring, 申 in summer, 未 in autumn
//! and 酉 in winter, the seasons as its table bounds them, so that it
//! names no 間日 in a 土用. It is a day of its own and not a whereabouts:
//! the page does not say how a 間日 and a 遊行 on the same day combine.
//!
//! # Not yet carried
//!
//! * **Japanese Wikipedia's 大将軍 遊行** (`wikipedia-ja-daishogun`, read
//!   2026-09-29): 甲子 to 戊辰 east in the spring 土用, 丙子 to 庚辰 south
//!   in the summer one, 庚子 to 甲辰 west in the autumn one and 壬子 to
//!   丙辰 north in the winter one. A 土用 is seventeen to nineteen days
//!   and holds a given 干支 in about three years in ten; the page does
//!   not say what happens in a 土用 that holds none, nor whether a run
//!   begun near its end runs past it.
//! * **古文書ネット's 大将軍 遊行** (`komonjyo-hasshojin`): the same five
//!   rows as いい日本再発見's, headed 春, 夏, 秋, 冬 and 土用 and dated for
//!   2026, but the page does not say what bounds its seasons. Its dates
//!   fit seasons of lunar months, as its 金神 page states them, and fit
//!   the 節月 seasons, except 30 August–3 September, 旧7月18–22日, after
//!   立秋, which it lists under 夏. Its days of 2026 are all days of
//!   `daishogun-iinippon`, to the same places.
//! * **古文書ネット's seasonal 金神 遊行** (`komonjyo-konjin`): its seasons
//!   are lunar months, 旧正・二・三 for spring and so on, without the leap
//!   months, and its autumn run opens on 「辛申」, which is no 干支 (辛 is
//!   a yin stem, 申 a yang branch). Its year-round rows are Japanese
//!   Wikipedia's, written 南, 西, 中央, 北 and 東. It gives no dates and no
//!   間日.
//! * **土公神**, which the Library says moves with the seasons, with no
//!   table read; and the other gods 歳事暦 names, 歳禄神 among them, with
//!   no table read.
//!
//! No source read dates a 金神 遊行 or 間日; 寒河江八幡宮's lists of the
//! 大将軍 遊行, which いい日本再発見 cites, are PDF files and were not read.
//!
//! [`docs/policy.md`]: https://github.com/kitsuyui/hyper-calendar/blob/main/docs/policy.md

use hc_calendar::cycle::{
    Sexagenary, readings, sexagenary_day, sexagenary_year_from_solar_term_year,
};
use hc_calendar::{Rd, gregorian};
use hc_seasons::{Meridian, Season, zassetsu};

use crate::nine_stars::nine_star_year;

/// A direction as an earthly branch, 子 being north and each 30° on
/// clockwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BranchDirection(u8);

impl BranchDirection {
    /// The direction of a branch, 0 for 子; reduced modulo twelve.
    #[must_use]
    pub const fn of_branch(branch: u8) -> Self {
        Self(branch % 12)
    }

    /// The branch, 0 for 子.
    #[must_use]
    pub const fn branch_index(self) -> u8 {
        self.0
    }

    /// The branch's character, e.g. `"卯"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        readings::HAN.branches[self.0 as usize]
    }

    /// The azimuth of the branch's centre, in degrees clockwise from north.
    #[must_use]
    pub const fn azimuth_degrees(self) -> u16 {
        self.0 as u16 * 30
    }

    /// The opposite direction.
    #[must_use]
    pub const fn opposite(self) -> Self {
        Self((self.0 + 6) % 12)
    }
}

/// One of the 八将神.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum General {
    /// 太歳神, the spirit of Jupiter: stands on the year's branch.
    Taisai,
    /// 大将軍, the spirit of Venus: three years in each of four directions.
    Daishogun,
    /// 大陰神, the spirit of Saturn, 太歳's consort: two branches behind it.
    Daion,
    /// 歳刑神, the spirit of Mercury: the 三刑 of the year's branch.
    Saikyo,
    /// 歳破神, the spirit of Saturn: opposite 太歳.
    Saiha,
    /// 歳殺神, the spirit of Venus as the National Diet Library has it;
    /// Japanese Wikipedia has "金曜星（太白）または火曜星（熒惑星）", Venus or
    /// Mars.
    Saisetsu,
    /// 黄幡神, the spirit of 羅睺.
    Oban,
    /// 豹尾神, the spirit of 計都: opposite 黄幡.
    Hyobi,
}

/// 大将軍 by year branch, Japanese Wikipedia's row: 亥子丑 酉, 寅卯辰 子,
/// 巳午未 卯, 申酉戌 午.
const DAISHOGUN: [u8; 12] = [9, 9, 0, 0, 0, 3, 3, 3, 6, 6, 6, 9];
/// 歳刑神 by year branch, Japanese Wikipedia's row, the 三刑.
const SAIKYO: [u8; 12] = [3, 10, 5, 0, 4, 8, 6, 1, 2, 9, 7, 11];
/// 歳殺神 by year branch: 未 辰 丑 戌, repeating.
const SAISETSU: [u8; 4] = [7, 4, 1, 10];
/// 黄幡神 by year branch: 辰 丑 戌 未, repeating.
const OBAN: [u8; 4] = [4, 1, 10, 7];

impl General {
    /// The eight, in the order the almanacs and the Library list them.
    pub const ALL: [Self; 8] = [
        Self::Taisai,
        Self::Daishogun,
        Self::Daion,
        Self::Saikyo,
        Self::Saiha,
        Self::Saisetsu,
        Self::Oban,
        Self::Hyobi,
    ];

    /// A stable identifier, the reading without its macrons and without
    /// the suffix 神 is read with, e.g. `"daishogun"`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Taisai => "taisai",
            Self::Daishogun => "daishogun",
            Self::Daion => "daion",
            Self::Saikyo => "saikyo",
            Self::Saiha => "saiha",
            Self::Saisetsu => "saisetsu",
            Self::Oban => "oban",
            Self::Hyobi => "hyobi",
        }
    }

    /// The name, e.g. `"大将軍"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::Taisai => "太歳神",
            Self::Daishogun => "大将軍",
            Self::Daion => "大陰神",
            Self::Saikyo => "歳刑神",
            Self::Saiha => "歳破神",
            Self::Saisetsu => "歳殺神",
            Self::Oban => "黄幡神",
            Self::Hyobi => "豹尾神",
        }
    }

    /// The reading in Hepburn romaji, as the Library gives it.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        match self {
            Self::Taisai => "taisaijin",
            Self::Daishogun => "daishōgun",
            Self::Daion => "daionjin",
            Self::Saikyo => "saikyōjin",
            Self::Saiha => "saihajin",
            Self::Saisetsu => "saisetsujin",
            Self::Oban => "ōbanjin",
            Self::Hyobi => "hyōbijin",
        }
    }

    /// What the Library says the god's direction forbids or favours, in
    /// English.
    #[must_use]
    pub const fn meaning(self) -> &'static str {
        match self {
            Self::Taisai => "facing it everything goes well, but do not fell trees",
            Self::Daishogun => "unlucky; it stays three years in one direction, 三年ふさがり",
            Self::Daion => "do not marry or give birth facing it",
            Self::Saikyo => "unlucky in all things; do not till or sow facing it",
            Self::Saiha => {
                "do not move house, travel or board a ship facing it; buying livestock is lucky"
            }
            Self::Saisetsu => "do not marry or give birth facing it",
            Self::Oban => "do not move earth facing it; lucky for starting archery",
            Self::Hyobi => "do not relieve yourself or acquire livestock facing it",
        }
    }

    /// The god's direction in a year whose earthly branch is
    /// `year_branch`, 0 for 子; reduced modulo twelve.
    #[must_use]
    pub const fn direction(self, year_branch: u8) -> BranchDirection {
        let branch = year_branch % 12;
        let index = branch as usize;
        BranchDirection::of_branch(match self {
            Self::Taisai => branch,
            Self::Daishogun => DAISHOGUN[index],
            Self::Daion => branch + 10,
            Self::Saikyo => SAIKYO[index],
            Self::Saiha => branch + 6,
            Self::Saisetsu => SAISETSU[index % 4],
            Self::Oban => OBAN[index % 4],
            Self::Hyobi => OBAN[index % 4] + 6,
        })
    }

    /// The god's direction in the year of a 干支.
    #[must_use]
    pub const fn direction_in(self, year: Sexagenary) -> BranchDirection {
        self.direction(year.branch_index())
    }
}

/// 金神 (巡金神) by the year's stem, Japanese Wikipedia's and 古文書ネット's
/// rows: 甲己 午未申酉, 乙庚 辰巳, 丙辛 子丑寅卯午未, 丁壬 寅卯戌亥, 戊癸
/// 子丑申酉.
const KONJIN: [&[u8]; 5] = [
    &[6, 7, 8, 9],
    &[4, 5],
    &[0, 1, 2, 3, 6, 7],
    &[2, 3, 10, 11],
    &[0, 1, 8, 9],
];

/// The branches 金神 stands in, in a year whose heavenly stem is `stem`,
/// 0 for 甲; reduced modulo ten.
#[must_use]
pub const fn konjin_branches(stem: u8) -> &'static [u8] {
    KONJIN[(stem % 5) as usize]
}

/// Whether 金神 stands in a direction in the year of a 干支.
#[must_use]
pub fn konjin_is_in(year: Sexagenary, direction: BranchDirection) -> bool {
    konjin_branches(year.stem_index()).contains(&direction.branch_index())
}

/// 大金神's direction in a year whose branch is `year_branch`: three
/// branches behind it, 子 years 酉.
#[must_use]
pub const fn dai_konjin(year_branch: u8) -> BranchDirection {
    BranchDirection::of_branch(year_branch % 12 + 9)
}

/// 姫金神's direction, always opposite 大金神's: 子 years 卯.
#[must_use]
pub const fn hime_konjin(year_branch: u8) -> BranchDirection {
    dai_konjin(year_branch).opposite()
}

/// The year's 干支 in force on a day at a meridian, turning at 立春.
#[must_use]
pub fn year_pillar(day: Rd, meridian: Meridian) -> Sexagenary {
    sexagenary_year_from_solar_term_year(nine_star_year(day, meridian))
}

/// A place a god of direction goes to when it leaves its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Place {
    /// A direction, as a branch.
    Direction(BranchDirection),
    /// 中央, the middle of the house (「中央（家の中）」, 「中央（家内）」).
    Centre,
}

impl Place {
    /// The name, e.g. `"卯"` or `"中央"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::Direction(direction) => direction.japanese_name(),
            Self::Centre => "中央",
        }
    }
}

/// Where a god of direction is on a day, by a [`WanderingRule`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Whereabouts {
    /// In its own direction or directions for the year: no 遊行 today.
    Home,
    /// Gone to a place for the day, its own directions left open.
    Gone(Place),
}

/// A god a [`WanderingRule`] moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WanderingGod {
    /// 大将軍, whose own direction is [`General::Daishogun`]'s.
    Daishogun,
    /// 金神 (巡金神), whose own directions are [`konjin_branches`].
    Konjin,
}

impl WanderingGod {
    /// The name, e.g. `"金神"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::Daishogun => "大将軍",
            Self::Konjin => "金神",
        }
    }

    /// Whether a direction is one of the god's own in the year of a 干支.
    #[must_use]
    pub fn is_home(self, year: Sexagenary, direction: BranchDirection) -> bool {
        match self {
            Self::Daishogun => General::Daishogun.direction_in(year) == direction,
            Self::Konjin => konjin_is_in(year, direction),
        }
    }
}

/// One source's reading of the days a god of direction leaves its own
/// direction, its 遊行.
#[derive(Debug, Clone, Copy)]
pub struct WanderingRule {
    /// The identifier, e.g. `"daishogun-iinippon"`.
    pub id: &'static str,
    /// The god the rule moves.
    pub god: WanderingGod,
    /// The rule as its source writes it.
    pub japanese_rule: &'static str,
    /// The source that states it.
    pub source: &'static str,
    /// Where the god is on a day at a meridian; `None` where the rule
    /// does not say, as the module documentation sets out.
    pub whereabouts: fn(Rd, Meridian) -> Option<Whereabouts>,
}

impl WanderingRule {
    /// Where the god is on a day; `None` where the rule does not say, or
    /// outside the years the astronomy answers for.
    #[must_use]
    pub fn whereabouts_on(&self, day: Rd, meridian: Meridian) -> Option<Whereabouts> {
        let year = gregorian::year_from_fixed(day);
        if !gregorian::year_in_range(year - 1) || !gregorian::year_in_range(year + 1) {
            return None;
        }
        (self.whereabouts)(day, meridian)
    }

    /// Whether the god stands in a direction on a day: in its own
    /// directions for the year (turning at 立春) when at home, in the one
    /// it has gone to when away, and in none when gone to the middle of
    /// the house; `None` where the rule does not say.
    #[must_use]
    pub fn stands_in(
        &self,
        day: Rd,
        meridian: Meridian,
        direction: BranchDirection,
    ) -> Option<bool> {
        Some(match self.whereabouts_on(day, meridian)? {
            Whereabouts::Home => self.god.is_home(year_pillar(day, meridian), direction),
            Whereabouts::Gone(Place::Direction(gone)) => gone == direction,
            Whereabouts::Gone(Place::Centre) => false,
        })
    }
}

/// How many days a 遊行 lasts: 「五日間」 in every source read.
const STRETCH_DAYS: u8 = 5;

/// The first day of a stretch of five that opens on the sexagenary day
/// `first` and holds `day`, if one does.
const fn stretch_start(day: Rd, first: u8) -> Option<Rd> {
    let offset = (sexagenary_day(day).index() + 60 - first) % 60;
    if offset < STRETCH_DAYS {
        Some(Rd(day.0 - offset as i64))
    } else {
        None
    }
}

const fn east() -> Place {
    Place::Direction(BranchDirection::of_branch(3))
}
const fn south() -> Place {
    Place::Direction(BranchDirection::of_branch(6))
}
const fn west() -> Place {
    Place::Direction(BranchDirection::of_branch(9))
}
const fn north() -> Place {
    Place::Direction(BranchDirection::of_branch(0))
}

/// The places of the stretches that hold a day; `None` if they are two
/// different places, which the rules do not reconcile.
fn one_place(places: impl IntoIterator<Item = Place>) -> Option<Whereabouts> {
    let mut found = None;
    for place in places {
        match found {
            None => found = Some(place),
            Some(earlier) if earlier == place => {}
            Some(_) => return None,
        }
    }
    Some(found.map_or(Whereabouts::Home, Whereabouts::Gone))
}

/// いい日本再発見's 大将軍 遊行, in the order it lists them: 甲子 east,
/// 丙子 south, 庚子 west, 壬子 north, 戊子 the middle of the house, each for
/// five days; its dated list of 2026 applies every row in every sixty
/// days.
const DAISHOGUN_IINIPPON: [(u8, Place); 5] = [
    (0, east()),
    (12, south()),
    (36, west()),
    (48, north()),
    (24, Place::Centre),
];

fn daishogun_iinippon(day: Rd, _meridian: Meridian) -> Option<Whereabouts> {
    one_place(
        DAISHOGUN_IINIPPON
            .iter()
            .filter(|(first, _)| stretch_start(day, *first).is_some())
            .map(|(_, place)| *place),
    )
}

/// A part of the year as Japanese Wikipedia's 「金神」 divides it: a
/// season from its 立 term to the day before its 土用, or the 土用.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum YearPart {
    /// 立春 to the day before the spring 土用, and so on.
    Season(Season),
    /// A season's 土用.
    Doyo(Season),
}

/// The part of the year a day falls in at a meridian, from the solar
/// terms and 土用 of [`hc_seasons::zassetsu`].
fn year_part(day: Rd, meridian: Meridian) -> YearPart {
    let year = gregorian::year_from_fixed(day);
    // The winter 土用 of a year runs from mid-January to the eve of its
    // 立春; the days before it are the winter begun at the last 立冬.
    let mut after = Season::Winter;
    for season in [
        Season::Winter,
        Season::Spring,
        Season::Summer,
        Season::Autumn,
    ] {
        let doyo = zassetsu::doyo(year, season, meridian);
        if day.0 < doyo.start.0 {
            return YearPart::Season(after);
        }
        if doyo.contains(day) {
            return YearPart::Doyo(season);
        }
        after = season.next();
    }
    YearPart::Season(Season::Winter)
}

/// Japanese Wikipedia's year-round 金神 遊行: 甲寅 to 午, 丙寅 to 酉, 戊寅
/// to the middle of the house, 庚寅 to 子, 壬寅 to 卯.
const KONJIN_YEAR_ROUND: [(u8, Place); 5] = [
    (50, south()),
    (2, west()),
    (14, Place::Centre),
    (26, north()),
    (38, east()),
];

/// Japanese Wikipedia's seasonal 金神 遊行: in spring 乙卯 to 卯, in summer
/// 丙午 to 午 (written 「牛」), in autumn 辛酉 to 酉, in winter 壬子 to 子.
const KONJIN_SEASONAL: [(Season, u8, Place); 4] = [
    (Season::Spring, 51, east()),
    (Season::Summer, 42, south()),
    (Season::Autumn, 57, west()),
    (Season::Winter, 48, north()),
];

/// Which days of a seasonal stretch the season holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SeasonalStretch {
    /// A stretch whose first day is in the season, all five days of it.
    BegunInSeason,
    /// The days of a stretch that are themselves in the season.
    DaysInSeason,
}

fn konjin_wikipedia(day: Rd, meridian: Meridian, reading: SeasonalStretch) -> Option<Whereabouts> {
    let year_round = KONJIN_YEAR_ROUND
        .iter()
        .filter(|(first, _)| stretch_start(day, *first).is_some())
        .map(|(_, place)| *place);
    let seasonal = KONJIN_SEASONAL.iter().filter_map(|(season, first, place)| {
        let start = stretch_start(day, *first)?;
        let counted = match reading {
            SeasonalStretch::BegunInSeason => start,
            SeasonalStretch::DaysInSeason => day,
        };
        (year_part(counted, meridian) == YearPart::Season(*season)).then_some(*place)
    });
    one_place(year_round.chain(seasonal))
}

fn konjin_wikipedia_begun_in_season(day: Rd, meridian: Meridian) -> Option<Whereabouts> {
    konjin_wikipedia(day, meridian, SeasonalStretch::BegunInSeason)
}

fn konjin_wikipedia_days_in_season(day: Rd, meridian: Meridian) -> Option<Whereabouts> {
    konjin_wikipedia(day, meridian, SeasonalStretch::DaysInSeason)
}

/// Japanese Wikipedia's 金神 遊行 table, as the source writes it.
const KONJIN_WIKIPEDIA_RULE: &str = "通年: 甲寅の日から5日間 午の方へ, 丙寅 酉, 戊寅 中央（家の中）, 庚寅 子, 壬寅 卯; 春（立春から春の土用まで）: 乙卯の日から5日間 卯の方へ; 夏（立夏から夏の土用まで）: 丙午 「牛」の方へ; 秋（立秋から秋の土用まで）: 辛酉 酉; 冬（立冬から冬の土用まで）: 壬子 子";

hc_core::catalogue! {
    type: WanderingRule,
    id: |rule| rule.id,
    provenance: |rule| rule.source,
    tests: wandering_rule_tests,
    associated;

    /// Every reading carried, in the order of the module documentation.
    pub const ALL;
    /// The reading with this identifier.
    pub fn by_id;

    entries: {
        /// 大将軍 as いい日本再発見's 2026 list keeps it: every 甲子, 丙子,
        /// 戊子, 庚子 and 壬子 opens five days away.
        pub const DAISHOGUN_IINIPPON = Self {
            id: "daishogun-iinippon",
            god: WanderingGod::Daishogun,
            japanese_rule: "春：甲子～戊辰の日は東方に遊行; 夏：丙子～庚辰の日は南方に遊行; 秋：庚子～丙辰の日は西方に遊行; 冬：壬子～丙辰の日は北方に遊行; 土用：戊子～壬辰の日は中央に遊行",
            source: "いい日本再発見 「大将軍（方位神）とは？2026年の方位と遊行日も！」, its 2026 list",
            whereabouts: daishogun_iinippon,
        };
        /// 金神 by Japanese Wikipedia's table, a seasonal stretch belonging
        /// to the season its first day is in.
        pub const KONJIN_WIKIPEDIA_BEGUN_IN_SEASON = Self {
            id: "konjin-wikipedia-begun-in-season",
            god: WanderingGod::Konjin,
            japanese_rule: KONJIN_WIKIPEDIA_RULE,
            source: "Japanese Wikipedia 「金神」, 金神の遊行・間日",
            whereabouts: konjin_wikipedia_begun_in_season,
        };
        /// 金神 by Japanese Wikipedia's table, a seasonal stretch holding
        /// only the days its season holds.
        pub const KONJIN_WIKIPEDIA_DAYS_IN_SEASON = Self {
            id: "konjin-wikipedia-days-in-season",
            god: WanderingGod::Konjin,
            japanese_rule: KONJIN_WIKIPEDIA_RULE,
            source: "Japanese Wikipedia 「金神」, 金神の遊行・間日",
            whereabouts: konjin_wikipedia_days_in_season,
        };
    }
}

/// 金神の間日 by season, Japanese Wikipedia's: spring 丑, summer 申, autumn
/// 未, winter 酉.
const KONJIN_REST_BRANCHES: [(Season, u8); 4] = [
    (Season::Spring, 1),
    (Season::Summer, 8),
    (Season::Autumn, 7),
    (Season::Winter, 9),
];

/// Whether a day is 金神の間日, a day 金神's direction may be crossed, by
/// Japanese Wikipedia's 「金神」: a 丑 day in spring, 申 in summer, 未 in
/// autumn, 酉 in winter, each season as the page's table bounds it, from
/// its 立 term to the day before its 土用, so that no day of a 土用 is
/// one. `None` outside the years the astronomy answers for.
#[must_use]
pub fn konjin_rest_day(day: Rd, meridian: Meridian) -> Option<bool> {
    let year = gregorian::year_from_fixed(day);
    if !gregorian::year_in_range(year - 1) || !gregorian::year_in_range(year + 1) {
        return None;
    }
    let branch = sexagenary_day(day).branch_index();
    Some(match year_part(day, meridian) {
        YearPart::Season(season) => KONJIN_REST_BRANCHES
            .iter()
            .any(|(of, rest)| *of == season && *rest == branch),
        YearPart::Doyo(_) => false,
    })
}

#[cfg(test)]
mod tests {
    use hc_calendar::cycle::sexagenary_year_from_gregorian_year;

    use super::*;

    fn names(year_branch: u8) -> [&'static str; 8] {
        General::ALL.map(|god| god.direction(year_branch).japanese_name())
    }

    /// Japanese Wikipedia's table, column by column.
    #[test]
    fn the_eight_generals_are_the_published_table() {
        let columns = [
            ["子", "酉", "戌", "卯", "午", "未", "辰", "戌"],
            ["丑", "酉", "亥", "戌", "未", "辰", "丑", "未"],
            ["寅", "子", "子", "巳", "申", "丑", "戌", "辰"],
            ["卯", "子", "丑", "子", "酉", "戌", "未", "丑"],
            ["辰", "子", "寅", "辰", "戌", "未", "辰", "戌"],
            ["巳", "卯", "卯", "申", "亥", "辰", "丑", "未"],
            ["午", "卯", "辰", "午", "子", "丑", "戌", "辰"],
            ["未", "卯", "巳", "丑", "丑", "戌", "未", "丑"],
            ["申", "午", "午", "寅", "寅", "未", "辰", "戌"],
            ["酉", "午", "未", "酉", "卯", "辰", "丑", "未"],
            ["戌", "午", "申", "未", "辰", "丑", "戌", "辰"],
            ["亥", "酉", "酉", "亥", "巳", "戌", "未", "丑"],
        ];
        for (branch, column) in columns.iter().enumerate() {
            assert_eq!(&names(branch as u8), column, "{branch}");
        }
    }

    /// The Library's statements: 太歳 on the year's branch (子 years
    /// north), 歳破 opposite it, 豹尾 opposite 黄幡.
    /// Each general's identifier is its reading, less its macrons and the
    /// 神 it is read with, and no two are the same.
    #[test]
    fn every_general_has_its_own_identifier() {
        for god in General::ALL {
            let reading = god
                .romaji()
                .replace('ō', "o")
                .trim_end_matches("jin")
                .to_owned();
            assert_eq!(god.id(), reading, "{}", god.japanese_name());
            assert_eq!(
                General::ALL
                    .iter()
                    .filter(|other| other.id() == god.id())
                    .count(),
                1
            );
        }
    }

    #[test]
    fn the_library_s_relations_hold() {
        for branch in 0..12 {
            let at = |god: General| god.direction(branch);
            assert_eq!(at(General::Taisai).branch_index(), branch);
            assert_eq!(at(General::Saiha), at(General::Taisai).opposite());
            assert_eq!(at(General::Hyobi), at(General::Oban).opposite());
        }
        assert_eq!(General::Taisai.direction(0).azimuth_degrees(), 0);
    }

    /// 古文書ネット for 2026, 丙午: 太歳 午, 大将軍 卯, 大陰 辰, 歳刑 午,
    /// 歳破 子, 歳殺 丑, 黄幡 戌, 豹尾 辰; いい日本再発見 gives 大将軍 in 卯
    /// before 立春 2026 too, the 巳 year's.
    #[test]
    fn the_published_directions_of_2026_match() {
        let year = sexagenary_year_from_gregorian_year(2026);
        assert_eq!(
            General::ALL.map(|god| god.direction_in(year).japanese_name()),
            ["午", "卯", "辰", "午", "子", "丑", "戌", "辰"]
        );
        let before = year_pillar(
            hc_calendar::gregorian::to_fixed(2026, 2, 3).expect("a date"),
            Meridian::JAPAN,
        );
        let after = year_pillar(
            hc_calendar::gregorian::to_fixed(2026, 2, 4).expect("a date"),
            Meridian::JAPAN,
        );
        assert_eq!(before.branch_index(), 5);
        assert_eq!(after.branch_index(), 6);
        assert_eq!(
            General::Daishogun.direction_in(before).japanese_name(),
            "卯"
        );
    }

    /// Japanese Wikipedia's 金神 rows; 古文書ネット's 2025, 乙巳: 辰 and 巳.
    #[test]
    fn the_konjin_rows_and_2025_match() {
        let year = sexagenary_year_from_gregorian_year(2025);
        let found: Vec<&str> = konjin_branches(year.stem_index())
            .iter()
            .map(|branch| BranchDirection::of_branch(*branch).japanese_name())
            .collect();
        assert_eq!(found, ["辰", "巳"]);
        assert!(konjin_is_in(year, BranchDirection::of_branch(4)));
        assert!(!konjin_is_in(year, BranchDirection::of_branch(6)));
        assert_eq!(konjin_branches(0), konjin_branches(5));
        assert_eq!(konjin_branches(2).len(), 6);
    }

    /// Japanese Wikipedia's 大金神 and 姫金神: 子 酉 卯, 丑 戌 辰, … 亥 申 寅.
    #[test]
    fn the_great_and_princess_konjin_are_the_published_table() {
        let rows = [
            ("酉", "卯"),
            ("戌", "辰"),
            ("亥", "巳"),
            ("子", "午"),
            ("丑", "未"),
            ("寅", "申"),
            ("卯", "酉"),
            ("辰", "戌"),
            ("巳", "亥"),
            ("午", "子"),
            ("未", "丑"),
            ("申", "寅"),
        ];
        for (branch, (great, princess)) in rows.iter().enumerate() {
            let branch = branch as u8;
            assert_eq!(dai_konjin(branch).japanese_name(), *great);
            assert_eq!(hime_konjin(branch).japanese_name(), *princess);
        }
    }

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        hc_calendar::gregorian::to_fixed(year, month, day).expect("a date")
    }

    const JAPAN: Meridian = Meridian::JAPAN;

    const fn to(branch: u8) -> Whereabouts {
        Whereabouts::Gone(Place::Direction(BranchDirection::of_branch(branch)))
    }

    const CENTRE: Whereabouts = Whereabouts::Gone(Place::Centre);

    /// The days of a printed run, first to last.
    fn run(year: i64, month: u8, first: u8, last: u8) -> impl Iterator<Item = Rd> {
        let start = ymd(year, month, first);
        (0..=i64::from(last - first)).map(move |offset| Rd(start.0 + offset))
    }

    /// いい日本再発見's 大将軍 遊行 of 2026 (`iinippon-daishogun`, read
    /// 2026-09-29), month by month as it prints them, 子 north, 卯 east,
    /// 午 south, 酉 west. Every printed day is a day of the rule, to the
    /// printed place, but for three runs that begin early: 21–30 October
    /// south, where the 丙子 is 29 October and the rule has 21 October the
    /// last day of a 甲子 run to the east, and which with 1–2 November
    /// leaves out 31 October, a day of the run; 21–26 November west, where the
    /// 庚子 is the 22nd; and 21–31 December south, where the 丙子 is the
    /// 28th. Every day the rule sends the god elsewhere than east is
    /// printed; the 甲子 runs to the east are not, 大将軍 standing in the
    /// east, on 卯, the whole year.
    #[test]
    fn the_published_daishogun_days_of_2026_match() {
        let rule = WanderingRule::DAISHOGUN_IINIPPON;
        let (s, c, w, n) = (to(6), CENTRE, to(9), to(0));
        let printed = [
            (1, 2, 6, s),
            (1, 14, 18, c),
            (1, 26, 30, w),
            (2, 7, 11, n),
            (3, 3, 7, s),
            (3, 15, 19, c),
            (3, 27, 31, w),
            (4, 8, 12, n),
            (5, 2, 6, s),
            (5, 14, 18, c),
            (5, 26, 30, w),
            (6, 7, 11, n),
            (7, 1, 5, s),
            (7, 13, 17, c),
            (7, 25, 29, w),
            (8, 6, 10, n),
            (8, 30, 31, s),
            (9, 1, 3, s),
            (9, 11, 15, c),
            (9, 23, 27, w),
            (10, 5, 9, n),
            (10, 21, 30, s),
            (11, 1, 2, s),
            (11, 10, 14, c),
            (11, 21, 26, w),
            (12, 4, 8, n),
            (12, 21, 31, s),
        ];
        let early: Vec<Rd> = run(2026, 10, 21, 28)
            .chain(run(2026, 11, 21, 21))
            .chain(run(2026, 12, 21, 27))
            .collect();
        let east = BranchDirection::of_branch(3);
        let mut listed = Vec::new();
        hc_core::memo::scope(|| {
            for (month, first, last, place) in printed {
                for day in run(2026, month, first, last) {
                    listed.push(day);
                    let found = rule.whereabouts_on(day, JAPAN);
                    if day == ymd(2026, 10, 21) {
                        assert_eq!(found, Some(to(3)));
                    } else if early.contains(&day) {
                        assert_eq!(found, Some(Whereabouts::Home), "{day:?}");
                    } else {
                        assert_eq!(found, Some(place), "{day:?}");
                    }
                }
            }
            for day in run(2026, 1, 1, 1).flat_map(|first| (0..365).map(move |n| Rd(first.0 + n))) {
                assert!(rule.god.is_home(year_pillar(day, JAPAN), east), "{day:?}");
                match rule.whereabouts_on(day, JAPAN) {
                    Some(Whereabouts::Home) => {
                        assert!(!listed.contains(&day) || early.contains(&day), "{day:?}");
                    }
                    Some(gone) if gone == to(3) => {
                        assert!(!listed.contains(&day) || day == ymd(2026, 10, 21));
                        assert_eq!(rule.stands_in(day, JAPAN, east), Some(true));
                    }
                    Some(_) if day == ymd(2026, 10, 31) => assert!(!listed.contains(&day)),
                    Some(_) => assert!(listed.contains(&day), "{day:?}"),
                    None => panic!("{day:?}"),
                }
            }
        });
    }

    /// 古文書ネット's 大将軍 遊行 of 2026 (`komonjyo-hasshojin`, read
    /// 2026-09-29): 19–23 February and 20–24 April east, 1–5 July and
    /// 30 August–3 September south, 23–27 September west, 4–8 December
    /// north. Each is a run of いい日本再発見's rule to the same place; the
    /// rule has more.
    #[test]
    fn the_other_published_daishogun_days_of_2026_are_among_the_rule_s() {
        let rule = WanderingRule::DAISHOGUN_IINIPPON;
        let runs = [
            ((2, 19), to(3)),
            ((4, 20), to(3)),
            ((7, 1), to(6)),
            ((8, 30), to(6)),
            ((9, 23), to(9)),
            ((12, 4), to(0)),
        ];
        for ((month, first), place) in runs {
            let start = ymd(2026, month, first);
            for offset in 0..5 {
                let day = Rd(start.0 + offset);
                assert_eq!(rule.whereabouts_on(day, JAPAN), Some(place), "{day:?}");
            }
            assert_eq!(
                rule.whereabouts_on(Rd(start.0 - 1), JAPAN),
                Some(Whereabouts::Home)
            );
            assert_eq!(
                rule.whereabouts_on(Rd(start.0 + 5), JAPAN),
                Some(Whereabouts::Home)
            );
        }
    }

    /// Japanese Wikipedia's 金神 table on the days of 2026, which no source
    /// read dates: 戊寅 4 January to the middle of the house; 甲寅 9 February
    /// to 午; the spring 乙卯 run of 10–14 February overlapping the 甲寅 run
    /// on four days, which the rule answers `None`, and 卯 on its fifth,
    /// 己未; 丙寅 21 February to 酉; the 丙午 of 1 June, the last day of a
    /// 壬寅 run to 卯 and the first of the summer run to 午; the winter 壬子
    /// of 4 December to 子, overlapping the 甲寅 run on 6–8 December.
    #[test]
    fn the_konjin_table_sends_the_god_where_it_says() {
        for rule in [
            WanderingRule::KONJIN_WIKIPEDIA_BEGUN_IN_SEASON,
            WanderingRule::KONJIN_WIKIPEDIA_DAYS_IN_SEASON,
        ] {
            let at = |month, day| rule.whereabouts_on(ymd(2026, month, day), JAPAN);
            assert_eq!(at(1, 1), Some(Whereabouts::Home));
            assert_eq!(at(1, 4), Some(CENTRE));
            assert_eq!(at(1, 16), Some(to(0)));
            assert_eq!(at(2, 9), Some(to(6)));
            for day in 10..=13 {
                assert_eq!(at(2, day), None, "{day}");
            }
            assert_eq!(at(2, 14), Some(to(3)));
            assert_eq!(at(2, 21), Some(to(9)));
            assert_eq!(at(5, 31), Some(to(3)));
            assert_eq!(at(6, 1), None);
            assert_eq!(at(6, 2), Some(to(6)));
            assert_eq!(at(12, 4), Some(to(0)));
            for day in 6..=8 {
                assert_eq!(at(12, day), None, "{day}");
            }
            assert_eq!(at(12, 9), Some(to(6)));
        }
    }

    /// Where the two readings part: the 辛酉 of 19 October 2025 is the
    /// last day of autumn before its 土用 of 20 October, so its run to 酉
    /// holds 19–23 October when a run belongs to the season it begins in,
    /// and 19 October alone when it holds only the season's days; and the
    /// spring 乙卯 of 16 April 2025, the eve of the spring 土用, sends the
    /// god to 卯 on 20 April, 己未, in the first reading and leaves it at
    /// home in the second.
    #[test]
    fn the_two_readings_part_where_a_run_crosses_into_the_doyo() {
        let begun = WanderingRule::KONJIN_WIKIPEDIA_BEGUN_IN_SEASON;
        let within = WanderingRule::KONJIN_WIKIPEDIA_DAYS_IN_SEASON;
        let day = |month, day| ymd(2025, month, day);
        assert_eq!(begun.whereabouts_on(day(10, 19), JAPAN), Some(to(9)));
        assert_eq!(within.whereabouts_on(day(10, 19), JAPAN), Some(to(9)));
        for date in 20..=23 {
            assert_eq!(begun.whereabouts_on(day(10, date), JAPAN), Some(to(9)));
            assert_eq!(
                within.whereabouts_on(day(10, date), JAPAN),
                Some(Whereabouts::Home)
            );
        }
        assert_eq!(begun.whereabouts_on(day(4, 20), JAPAN), Some(to(3)));
        assert_eq!(
            within.whereabouts_on(day(4, 20), JAPAN),
            Some(Whereabouts::Home)
        );
        assert_eq!(begun.whereabouts_on(day(4, 17), JAPAN), None);
        assert_eq!(within.whereabouts_on(day(4, 17), JAPAN), Some(to(6)));
    }

    /// 2026 is 丙午 from 立春, 金神 on 子 丑 寅 卯 午 未: at home on
    /// 15 February it stands in 子 and not in 辰; gone to 酉 on 21 February
    /// it stands in 酉 alone; gone to the middle of the house on 5 March it
    /// stands in no direction; on the overlapping 10 February the rule
    /// does not say.
    #[test]
    fn the_konjin_stands_in_its_own_directions_or_where_it_has_gone() {
        let rule = WanderingRule::KONJIN_WIKIPEDIA_BEGUN_IN_SEASON;
        let stands = |month, day, branch| {
            rule.stands_in(
                ymd(2026, month, day),
                JAPAN,
                BranchDirection::of_branch(branch),
            )
        };
        assert_eq!(stands(2, 15, 0), Some(true));
        assert_eq!(stands(2, 15, 4), Some(false));
        assert_eq!(stands(2, 21, 9), Some(true));
        assert_eq!(stands(2, 21, 0), Some(false));
        assert!((0..12).all(|branch| stands(3, 5, branch) == Some(false)));
        assert_eq!(stands(2, 10, 0), None);
        assert_eq!(rule.god.japanese_name(), "金神");
        assert_eq!(
            WanderingRule::DAISHOGUN_IINIPPON.god.japanese_name(),
            "大将軍"
        );
        assert_eq!(Place::Centre.japanese_name(), "中央");
        assert_eq!(east().japanese_name(), "卯");
    }

    /// Japanese Wikipedia's 間日 on days of 2026: the 丑 day 8 February in
    /// spring, the 申 day 10 May in summer, the 未 day 13 August in autumn
    /// and the 酉 day 7 November, 立冬 itself, in winter; not the 丑 day
    /// 21 April, in the spring 土用, and not the 申 day 15 February, in
    /// spring; 24 days in the year.
    #[test]
    fn the_konjin_rest_days_are_the_season_s_branch() {
        let rest = |month, day| konjin_rest_day(ymd(2026, month, day), JAPAN);
        assert_eq!(rest(2, 8), Some(true));
        assert_eq!(rest(5, 10), Some(true));
        assert_eq!(rest(8, 13), Some(true));
        assert_eq!(rest(11, 7), Some(true));
        assert_eq!(rest(4, 21), Some(false));
        assert_eq!(rest(2, 15), Some(false));
        let rest_days = (0..365)
            .filter(|offset| konjin_rest_day(Rd(ymd(2026, 1, 1).0 + offset), JAPAN) == Some(true))
            .count();
        // One day in twelve of the 293 days of 2026 outside a 土用.
        assert_eq!(rest_days, 24);
    }

    #[test]
    fn a_year_the_calendar_does_not_reach_is_refused() {
        let far = ymd(hc_calendar::gregorian::MAX_YEAR, 6, 1);
        for rule in WanderingRule::ALL {
            assert_eq!(rule.whereabouts_on(far, JAPAN), None, "{}", rule.id);
            assert_eq!(
                rule.stands_in(far, JAPAN, BranchDirection::of_branch(0)),
                None
            );
        }
        assert_eq!(konjin_rest_day(far, JAPAN), None);
    }
}
