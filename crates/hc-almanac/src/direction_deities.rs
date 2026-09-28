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
//!   carried; the other is named here and not carried, as a copying slip
//!   that no second source shares.
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
//! # Not carried
//!
//! The days the gods leave their directions, 大将軍's and 金神's 遊行
//! and 金神's 間日, which Japanese Wikipedia, 古文書ネット and いい日本再発見
//! tabulate and do not agree on the seasons of (Japanese Wikipedia's
//! 金神 seasons run from 立春 to the 土用, 古文書ネット's by lunar
//! month); 土公神, which the Library says moves with the seasons, with no
//! table read; and the other gods 歳事暦 names, 歳禄神 among them, with no
//! table read.

use hc_calendar::Rd;
use hc_calendar::cycle::{Sexagenary, readings, sexagenary_year_from_solar_term_year};
use hc_seasons::Meridian;

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
    /// 歳殺神, the spirit of Venus.
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
}
