//! Every annotation for one day: what a printed almanac page actually is.
//!
//! A page of the 神宮館 or 高島 almanac gives one day about twenty facts —
//! the 干支, the 六曜, the 十二直, the 二十八宿, the three 九星, then a
//! paragraph of 暦注下段 and 選日. Nobody wants those one function call at a
//! time, and computing them one at a time would run the solar and lunar
//! series over and over. [`day_notes`] computes the context once and reads
//! every annotation off it.
//!
//! ```
//! use hc_almanac::{Meridian, Rd, day_notes::day_notes};
//!
//! // 21 December 2025: 甲子, the head of the sexagenary cycle, a 天赦日,
//! // and the day the 九星 count reversed into 陽遁.
//! let notes = day_notes(Rd(739_606), Meridian::JAPAN);
//! assert_eq!(notes.sexagenary().index(), 0);
//! assert!(notes.lower_register().contains(hc_almanac::LowerRegister::TENSHANICHI));
//! ```
//!
//! # The combinations, and why they carry a warning
//!
//! [`DayNotes::combinations`] reports things like 天赦日 + 一粒万倍日. These
//! are **not** almanac doctrine. A traditional almanac prints each 暦注 in
//! its own column and defines no algebra over them; the "最強開運日" ranking
//! is a twenty-first-century retail construct, pushed by wallet sellers,
//! jewellers, banks and wedding halls. It is included because callers ask
//! for it and because it is better computed correctly than guessed at, and
//! it is labelled for what it is.
//!
//! The same applies to cancellation. Whether a 一粒万倍日 falling on a
//! 不成就日 is nullified, halved, or simply deferred to is genuinely
//! disputed between publishers — some list the date with an asterisk, some
//! drop it. This crate reports the overlap through
//! [`Combination::GRAIN_AND_NO_ACCOMPLISHMENT`] and takes no position on what
//! it means.

use hc_calendar::Weekday;
use hc_calendar::cycle::Sexagenary;
use hc_seasons::lunisolar::LunisolarDay;
use hc_seasons::{Meridian, Rd};

use crate::context::{DayContext, SolarMonth};
use crate::lower_register::{LowerRegister, LowerRegisterSet, lower_register_of_context};
use crate::mansions::{Mansion, Mansion27, mansion_of, mansion27_of};
use crate::nine_stars::{NineStars, nine_stars_of_context};
use crate::rokuyo::{Rokuyo, rokuyo_of};
use crate::selected_days::{SelectedDay, SelectedDaySet, selected_days_of_context};
use crate::seven_luminaries::{Luminary, luminary_of};
use crate::twelve_directs::{TwelveDirect, direct_of_context};

/// A combination of annotations that modern Japanese commerce cares about.
///
/// None of these is traditional. See the module documentation. The set is
/// invented by publishers and grows when one of them coins a pairing, which
/// is why it is a table and not an `enum` (ADR 0007): a new pairing is an
/// entry with its own test, and nothing else has to be edited. Two
/// combinations are equal when they have the same identifier.
#[derive(Debug, Clone, Copy)]
pub struct Combination {
    /// A short identifier, e.g. `pardon-and-grain`.
    pub id: &'static str,
    japanese_name: &'static str,
    clash: bool,
    holds: fn(Rokuyo, LowerRegisterSet, SelectedDaySet) -> bool,
}

impl PartialEq for Combination {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Combination {}

impl core::hash::Hash for Combination {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Byte-for-byte equality of two identifiers, usable in `const` context.
const fn same_id(left: &str, right: &str) -> bool {
    let (left, right) = (left.as_bytes(), right.as_bytes());
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }
    true
}

impl Combination {
    /// A combination from its identifier, its Japanese name, whether it is
    /// a clash, and the test that says whether a day has it.
    #[must_use]
    pub const fn new(
        id: &'static str,
        japanese_name: &'static str,
        clash: bool,
        holds: fn(Rokuyo, LowerRegisterSet, SelectedDaySet) -> bool,
    ) -> Self {
        Self {
            id,
            japanese_name,
            clash,
            holds,
        }
    }

    /// This combination's position in [`Combination::ALL`], which is its
    /// bit in a [`CombinationSet`].
    ///
    /// # Panics
    ///
    /// If the combination is not in [`Combination::ALL`], which a value built
    /// with [`Combination::new`] outside this module would be; such a value
    /// can be tested with [`Combination::holds`] but not stored in a set.
    #[must_use]
    pub const fn index(self) -> u8 {
        let mut index = 0;
        while index < Self::ALL.len() {
            if same_id(Self::ALL[index].id, self.id) {
                return index as u8;
            }
            index += 1;
        }
        panic!("a combination that is not in Combination::ALL has no index")
    }

    /// The name in Japanese characters, e.g. `"天赦日＋一粒万倍日"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        self.japanese_name
    }

    /// Whether this combination is a clash — an auspicious day landing on an
    /// inauspicious one — rather than a reinforcement.
    #[must_use]
    pub const fn is_a_clash(self) -> bool {
        self.clash
    }

    /// Whether a day with these annotations has this combination.
    #[must_use]
    pub fn holds(self, rokuyo: Rokuyo, lower: LowerRegisterSet, selected: SelectedDaySet) -> bool {
        (self.holds)(rokuyo, lower, selected)
    }
}

fn is_taian(rokuyo: Rokuyo) -> bool {
    matches!(rokuyo, Rokuyo::Taian)
}

fn has_pardon(lower: LowerRegisterSet) -> bool {
    lower.contains(LowerRegister::TENSHANICHI)
}

fn has_grain(selected: SelectedDaySet) -> bool {
    selected.contains(SelectedDay::ICHIRYU_MANBAI)
}

fn pardon_and_grain(_: Rokuyo, lower: LowerRegisterSet, selected: SelectedDaySet) -> bool {
    has_pardon(lower) && has_grain(selected)
}

fn pardon_and_grain_and_taian(
    rokuyo: Rokuyo,
    lower: LowerRegisterSet,
    selected: SelectedDaySet,
) -> bool {
    is_taian(rokuyo) && has_pardon(lower) && has_grain(selected)
}

fn taian_and_grain(rokuyo: Rokuyo, _: LowerRegisterSet, selected: SelectedDaySet) -> bool {
    is_taian(rokuyo) && has_grain(selected)
}

fn pardon_and_taian(rokuyo: Rokuyo, lower: LowerRegisterSet, _: SelectedDaySet) -> bool {
    is_taian(rokuyo) && has_pardon(lower)
}

fn tiger_and_taian(rokuyo: Rokuyo, _: LowerRegisterSet, selected: SelectedDaySet) -> bool {
    is_taian(rokuyo) && selected.contains(SelectedDay::TIGER_DAY)
}

fn earth_serpent_and_taian(rokuyo: Rokuyo, _: LowerRegisterSet, selected: SelectedDaySet) -> bool {
    is_taian(rokuyo) && selected.contains(SelectedDay::TSUCHINOTO_MI)
}

fn grain_and_no_accomplishment(_: Rokuyo, _: LowerRegisterSet, selected: SelectedDaySet) -> bool {
    has_grain(selected) && selected.contains(SelectedDay::FUJOJU)
}

fn grain_and_three_neighbours(_: Rokuyo, _: LowerRegisterSet, selected: SelectedDaySet) -> bool {
    has_grain(selected) && selected.contains(SelectedDay::SANRINBO)
}

hc_core::catalogue! {
    type: Combination,
    id: |combination| combination.id,
    tests: combination_tests,
    associated;

    /// Every combination, the two clashes included, in listing order. The
    /// position here is the bit in a [`CombinationSet`].
    pub const ALL;
    /// The combination with this identifier.
    pub fn by_id;

    entries: {
        /// 天赦日 + 一粒万倍日 — marketed as 最強開運日. Three or four a year.
        pub const PARDON_AND_GRAIN =
            Self::new("pardon-and-grain", "天赦日＋一粒万倍日", false, pardon_and_grain);
        /// 天赦日 + 一粒万倍日 + 大安 — 超最強開運日, the strongest of the
        /// marketing categories.
        pub const PARDON_AND_GRAIN_AND_TAIAN = Self::new(
            "pardon-and-grain-and-taian",
            "天赦日＋一粒万倍日＋大安",
            false,
            pardon_and_grain_and_taian,
        );
        /// 大安 + 一粒万倍日 — the common "good day to begin" pairing.
        pub const TAIAN_AND_GRAIN =
            Self::new("taian-and-grain", "大安＋一粒万倍日", false, taian_and_grain);
        /// 天赦日 + 大安.
        pub const PARDON_AND_TAIAN =
            Self::new("pardon-and-taian", "天赦日＋大安", false, pardon_and_taian);
        /// 寅の日 + 大安 — pushed for buying a wallet.
        pub const TIGER_AND_TAIAN =
            Self::new("tiger-and-taian", "寅の日＋大安", false, tiger_and_taian);
        /// 己巳 + 大安 — pushed for anything to do with money.
        pub const EARTH_SERPENT_AND_TAIAN =
            Self::new("earth-serpent-and-taian", "己巳＋大安", false, earth_serpent_and_taian);
        /// 一粒万倍日 + 不成就日 — the disputed one.
        ///
        /// Publishers differ on whether the 不成就日 cancels the 一粒万倍日,
        /// halves it, or takes precedence as a 凶日 should. This crate
        /// reports the overlap and decides nothing.
        pub const GRAIN_AND_NO_ACCOMPLISHMENT = Self::new(
            "grain-and-no-accomplishment",
            "一粒万倍日＋不成就日",
            true,
            grain_and_no_accomplishment,
        );
        /// 一粒万倍日 + 三隣亡 — the same kind of clash, and the same dispute.
        pub const GRAIN_AND_THREE_NEIGHBOURS = Self::new(
            "grain-and-three-neighbours",
            "一粒万倍日＋三隣亡",
            true,
            grain_and_three_neighbours,
        );
    }
}

/// A set of [`Combination`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CombinationSet {
    bits: u16,
}

impl CombinationSet {
    /// The empty set.
    pub const EMPTY: Self = Self { bits: 0 };

    /// Whether a combination is in the set.
    #[must_use]
    pub const fn contains(self, combination: Combination) -> bool {
        self.bits & (1 << combination.index()) != 0
    }

    /// Add a combination to the set.
    #[must_use]
    pub const fn with(self, combination: Combination) -> Self {
        Self {
            bits: self.bits | (1 << combination.index()),
        }
    }

    /// Whether the set is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    /// How many combinations are in the set.
    #[must_use]
    pub const fn len(self) -> u32 {
        self.bits.count_ones()
    }

    /// The combinations in the set, in [`Combination::ALL`] order.
    pub fn iter(self) -> impl Iterator<Item = Combination> {
        Combination::ALL
            .iter()
            .copied()
            .filter(move |combination| self.contains(*combination))
    }
}

/// Every almanac annotation for one day at one meridian.
///
/// This is the whole crate in one value. Build it with [`day_notes`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayNotes {
    context: DayContext,
    rokuyo: Rokuyo,
    mansion: Mansion,
    mansion27: Mansion27,
    direct: TwelveDirect,
    stars: NineStars,
    lower: LowerRegisterSet,
    selected: SelectedDaySet,
    combinations: CombinationSet,
}

impl DayNotes {
    /// The day these notes describe.
    #[must_use]
    pub const fn day(&self) -> Rd {
        self.context.day()
    }

    /// The meridian the day boundaries were read at.
    #[must_use]
    pub const fn meridian(&self) -> Meridian {
        self.context.meridian()
    }

    /// The shared context every rule was evaluated against.
    #[must_use]
    pub const fn context(&self) -> DayContext {
        self.context
    }

    /// 日の干支 — the day's sexagenary position. The 上段 of the page.
    #[must_use]
    pub const fn sexagenary(&self) -> Sexagenary {
        self.context.sexagenary()
    }

    /// The 節月 the day falls in.
    #[must_use]
    pub const fn solar_month(&self) -> SolarMonth {
        self.context.solar_month()
    }

    /// The day's lunisolar month and day.
    #[must_use]
    pub const fn lunisolar(&self) -> LunisolarDay {
        self.context.lunisolar()
    }

    /// The day of the seven-day week.
    #[must_use]
    pub const fn weekday(&self) -> Weekday {
        Weekday::from_rd(self.context.day())
    }

    /// 七曜 — the luminary the weekday is named after.
    #[must_use]
    pub const fn luminary(&self) -> Luminary {
        luminary_of(self.context.day())
    }

    /// 六曜.
    #[must_use]
    pub const fn rokuyo(&self) -> Rokuyo {
        self.rokuyo
    }

    /// 二十八宿, as the almanac's twenty-eight-day cycle gives it.
    #[must_use]
    pub const fn mansion(&self) -> Mansion {
        self.mansion
    }

    /// 二十七宿, as 宿曜道 gives it — reset at every new moon.
    #[must_use]
    pub const fn mansion27(&self) -> Mansion27 {
        self.mansion27
    }

    /// 十二直 — the 中段 of the page.
    #[must_use]
    pub const fn twelve_direct(&self) -> TwelveDirect {
        self.direct
    }

    /// 九星 — the year, month and day stars.
    #[must_use]
    pub const fn nine_stars(&self) -> NineStars {
        self.stars
    }

    /// 暦注下段 — every lower-register annotation in force.
    #[must_use]
    pub const fn lower_register(&self) -> LowerRegisterSet {
        self.lower
    }

    /// 選日 — every selected day in force.
    #[must_use]
    pub const fn selected_days(&self) -> SelectedDaySet {
        self.selected
    }

    /// The modern combinations. See the module documentation for the health
    /// warning they come with.
    #[must_use]
    pub const fn combinations(&self) -> CombinationSet {
        self.combinations
    }

    /// Whether the day is 大安, the luckiest of the 六曜.
    #[must_use]
    pub const fn is_taian(&self) -> bool {
        matches!(self.rokuyo, Rokuyo::Taian)
    }
}

/// Every almanac annotation for a day at a meridian.
///
/// One [`DayContext`] is built and shared, so this costs one solar-longitude
/// solve, two new-moon searches and six solar-term solves for the 九星
/// period — not one set per annotation.
#[must_use]
pub fn day_notes(day: Rd, meridian: Meridian) -> DayNotes {
    let context = DayContext::new(day, meridian);
    let rokuyo = rokuyo_of(context.lunisolar());
    let lower = lower_register_of_context(&context);
    let selected = selected_days_of_context(&context);
    let combinations = combinations_of(rokuyo, lower, selected);
    DayNotes {
        context,
        rokuyo,
        mansion: mansion_of(day),
        mansion27: mansion27_of(day, meridian),
        direct: direct_of_context(&context),
        stars: nine_stars_of_context(&context),
        lower,
        selected,
        combinations,
    }
}

/// The modern combinations implied by a day's 六曜, 暦注下段 and 選日.
///
/// Separated out so that the rule is one readable function and not buried in
/// [`day_notes`].
#[must_use]
fn combinations_of(
    rokuyo: Rokuyo,
    lower: LowerRegisterSet,
    selected: SelectedDaySet,
) -> CombinationSet {
    let mut set = CombinationSet::EMPTY;
    for combination in Combination::ALL {
        if combination.holds(rokuyo, lower, selected) {
            set = set.with(*combination);
        }
    }
    set
}

#[cfg(test)]
mod tests {
    use crate::mansions::CYCLE_ANCHOR;
    use crate::nine_stars::NineStar;
    use crate::selected_days::selected_days;
    use crate::twelve_directs::direct_of;

    use super::*;

    const JAPAN: Meridian = Meridian::JAPAN;

    /// 2024-01-01, 2025-01-01.
    const NEW_YEAR_2024: i64 = 738_886;
    const NEW_YEAR_2025: i64 = 739_252;

    /// 2024-01-01 was 甲子, a Monday, 畢宿, 建 and 一白水星, and the almanacs
    /// print 天恩日 and 天赦日 against it. Six independent rules in one page.
    #[test]
    fn the_first_of_january_2024_reads_as_the_almanac_prints_it() {
        let notes = day_notes(Rd(NEW_YEAR_2024), JAPAN);
        assert_eq!(notes.sexagenary().index(), 0);
        assert_eq!(notes.sexagenary().stem_name(), "jia");
        assert_eq!(notes.sexagenary().branch_name(), "zi");
        assert_eq!(notes.weekday(), Weekday::Monday);
        assert_eq!(notes.luminary(), Luminary::Moon);
        assert_eq!(notes.mansion().japanese_name(), "畢");
        assert_eq!(notes.twelve_direct().japanese_name(), "建");
        assert_eq!(notes.nine_stars().day, NineStar::OneWhite);
        assert!(notes.lower_register().contains(LowerRegister::TENONNICHI));
        assert!(notes.lower_register().contains(LowerRegister::TENSHANICHI));
        assert!(notes.selected_days().contains(SelectedDay::KINOENE));
        assert_eq!(notes.solar_month().number(), 11);
    }

    /// 21 December 2025 is the day everything happens at once: 甲子, 天赦日,
    /// a 一粒万倍日, and the 甲子 the 九星 count reverses on into 陽遁. The
    /// almanacs print 赤口 against it, so it is not a 大安 and the
    /// 天赦日＋一粒万倍日＋大安 combination does not fire.
    #[test]
    fn the_twenty_first_of_december_2025_carries_the_strongest_combination() {
        let notes = day_notes(Rd(739_606), JAPAN);
        assert_eq!(notes.sexagenary().index(), 0);
        assert!(notes.lower_register().contains(LowerRegister::TENSHANICHI));
        assert!(notes.selected_days().contains(SelectedDay::ICHIRYU_MANBAI));
        assert!(notes.selected_days().contains(SelectedDay::KINOENE));
        assert!(notes.combinations().contains(Combination::PARDON_AND_GRAIN));
        assert_eq!(notes.rokuyo(), Rokuyo::Shakko);
        assert!(!notes.is_taian());
        assert!(
            !notes
                .combinations()
                .contains(Combination::PARDON_AND_GRAIN_AND_TAIAN)
        );
        assert_eq!(notes.nine_stars().day, NineStar::OneWhite);
    }

    /// 26 December 2025 is 己巳 and 大安, which an almanac prints as 「巳の日、
    /// 己巳の日」 and a wallet advertisement prints as a money day.
    #[test]
    fn the_twenty_sixth_of_december_2025_is_the_earth_serpent_on_a_taian() {
        let notes = day_notes(Rd(739_611), JAPAN);
        assert_eq!(notes.sexagenary().index(), 5);
        assert_eq!(notes.rokuyo(), Rokuyo::Taian);
        assert!(notes.is_taian());
        assert!(notes.selected_days().contains(SelectedDay::TSUCHINOTO_MI));
        assert!(notes.selected_days().contains(SelectedDay::SNAKE_DAY));
        assert!(
            notes
                .combinations()
                .contains(Combination::EARTH_SERPENT_AND_TAIAN)
        );
    }

    /// The published 天赦日 + 一粒万倍日 days for 2025 are 10 March, 24 July,
    /// 6 October and 21 December. That is the whole year and nothing else.
    #[test]
    fn the_published_2025_strongest_days_match() {
        const CUMULATIVE: [i64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let expected = [(3usize, 10i64), (7, 24), (10, 6), (12, 21)];
        let mut found = 0;
        for offset in 0..365 {
            let notes = day_notes(Rd(NEW_YEAR_2025 + offset), JAPAN);
            if notes.combinations().contains(Combination::PARDON_AND_GRAIN) {
                found += 1;
                let matched = expected.iter().any(|(month, day)| {
                    NEW_YEAR_2025 + CUMULATIVE[month - 1] + day - 1 == NEW_YEAR_2025 + offset
                });
                assert!(matched, "unexpected day at offset {offset}");
            }
        }
        assert_eq!(found, expected.len());
    }

    /// The assembled page must agree with every individual query, which is
    /// the only guarantee that the shared context has not diverged from the
    /// standalone functions.
    #[test]
    fn the_assembled_page_agrees_with_every_individual_query() {
        for offset in [0, 17, 33, 97, 181, 264, 350] {
            let day = Rd(NEW_YEAR_2025 + offset);
            let notes = day_notes(day, JAPAN);
            assert_eq!(notes.day(), day);
            assert_eq!(notes.meridian(), JAPAN);
            assert_eq!(notes.mansion(), mansion_of(day));
            assert_eq!(notes.mansion27(), mansion27_of(day, JAPAN));
            assert_eq!(notes.twelve_direct(), direct_of(day, JAPAN));
            assert_eq!(notes.selected_days(), selected_days(day, JAPAN));
            assert_eq!(
                notes.lower_register(),
                crate::lower_register::lower_register(day, JAPAN)
            );
            assert_eq!(
                notes.nine_stars(),
                crate::nine_stars::nine_stars(day, JAPAN)
            );
            assert_eq!(notes.rokuyo(), crate::rokuyo::rokuyo(day, JAPAN));
            assert_eq!(notes.weekday(), Weekday::from_rd(day));
        }
    }

    /// Japan and China read the same instant at different meridians, so a
    /// solar term occasionally lands on different days and every 節月-keyed
    /// annotation can differ with it. Over a decade the two must disagree
    /// about the 十二直 at least once — if they never did, the meridian
    /// argument would be decorative.
    #[test]
    fn the_meridian_changes_answers_at_least_sometimes() {
        let mut disagreements = 0;
        for offset in 0..3_653 {
            let day = Rd(NEW_YEAR_2025 + offset);
            if direct_of(day, Meridian::JAPAN) != direct_of(day, Meridian::CHINA) {
                disagreements += 1;
            }
        }
        assert!(
            disagreements > 0,
            "Japan and China must disagree about a 十二直 at least once a decade"
        );
    }

    /// Nothing in a day's page may be silently undetermined: every rule the
    /// two catalogues carry must answer yes or no.
    #[test]
    fn no_annotation_in_the_two_catalogues_is_undetermined() {
        let context = DayContext::new(Rd(NEW_YEAR_2025), JAPAN);
        for note in LowerRegister::ALL.iter().copied() {
            assert!(
                note.applies_to(&context).is_some(),
                "{}",
                note.japanese_name()
            );
        }
        for day in SelectedDay::ALL.iter().copied() {
            assert!(
                day.applies_to(&context).is_some(),
                "{}",
                day.japanese_name()
            );
        }
    }

    #[test]
    fn the_combination_set_holds_every_combination_distinctly() {
        let mut set = CombinationSet::EMPTY;
        assert!(set.is_empty());
        for combination in Combination::ALL.iter().copied() {
            assert!(!set.contains(combination));
            set = set.with(combination);
            assert!(!combination.japanese_name().is_empty());
            assert_eq!(
                usize::from(combination.index()),
                Combination::ALL
                    .iter()
                    .position(|c| c == &combination)
                    .unwrap_or(usize::MAX)
            );
        }
        assert_eq!(set.len() as usize, Combination::ALL.len());
        assert_eq!(set.iter().count(), Combination::ALL.len());
        // Eight, the two clashes included.
        assert_eq!(Combination::ALL.len(), 8);
        let clashes = Combination::ALL
            .iter()
            .filter(|combination| combination.is_a_clash())
            .count();
        assert_eq!(clashes, 2);
    }

    /// The mansion cycle needs no meridian, because it needs no astronomy —
    /// the same day is the same mansion in Tokyo and in Beijing.
    #[test]
    fn the_mansion_is_the_same_at_every_meridian() {
        for offset in 0..60 {
            let day = Rd(CYCLE_ANCHOR.0 + offset);
            assert_eq!(
                day_notes(day, Meridian::JAPAN).mansion(),
                day_notes(day, Meridian::CHINA).mansion()
            );
        }
    }
}
