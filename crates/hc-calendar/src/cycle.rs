//! Repeating named cycles: sexagenary stems and branches, and the general
//! machinery behind any other cyclic naming scheme.
//!
//! Cycles are the clearest case of the data/algorithm split this library is
//! built on. "Which of sixty names does this year have" is one modulo; the
//! sixty names themselves are data that varies by culture and script. So the
//! arithmetic lives here and the names live in `hc-i18n`.

use crate::fixed::Rd;

/// The ten Heavenly Stems of the sexagenary cycle, romanised.
pub const HEAVENLY_STEMS: [&str; 10] = [
    "jia", "yi", "bing", "ding", "wu", "ji", "geng", "xin", "ren", "gui",
];

/// The twelve Earthly Branches of the sexagenary cycle, romanised.
pub const EARTHLY_BRANCHES: [&str; 12] = [
    "zi", "chou", "yin", "mao", "chen", "si", "wu", "wei", "shen", "you", "xu", "hai",
];

/// The twelve zodiac animals, in branch order.
pub const ZODIAC_ANIMALS: [&str; 12] = [
    "rat", "ox", "tiger", "rabbit", "dragon", "snake", "horse", "goat", "monkey", "rooster", "dog",
    "pig",
];

/// The five phases, in stem-pair order.
pub const FIVE_PHASES: [&str; 5] = ["wood", "fire", "earth", "metal", "water"];

/// A position in the sixty-term sexagenary cycle (干支 / ganzhi / eto).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sexagenary {
    /// The zero-based index within the sixty-term cycle.
    index: u8,
}

impl Sexagenary {
    /// Build from a zero-based index within the cycle.
    ///
    /// Indices outside `0..60` wrap, because a cycle position is by
    /// definition modular.
    #[must_use]
    pub const fn from_index(index: i64) -> Self {
        Self {
            index: index.rem_euclid(60) as u8,
        }
    }

    /// Build from a one-based stem and branch, as the pair is usually cited.
    ///
    /// Only 60 of the 120 stem-branch pairs occur: the stem and the branch
    /// advance together, so their indices always share a parity. Returns
    /// `None` for an impossible pair such as "jia-chou".
    #[must_use]
    pub const fn from_stem_branch(stem: u8, branch: u8) -> Option<Self> {
        if stem == 0 || stem > 10 || branch == 0 || branch > 12 {
            return None;
        }
        let stem_index = (stem - 1) as i64;
        let branch_index = (branch - 1) as i64;
        let mut index = 0i64;
        while index < 60 {
            if index % 10 == stem_index && index % 12 == branch_index {
                return Some(Self { index: index as u8 });
            }
            index += 1;
        }
        None
    }

    /// The zero-based index within the cycle.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.index
    }

    /// The one-based ordinal, as tables usually number it.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        self.index + 1
    }

    /// The zero-based Heavenly Stem.
    #[must_use]
    pub const fn stem_index(self) -> u8 {
        self.index % 10
    }

    /// The zero-based Earthly Branch.
    #[must_use]
    pub const fn branch_index(self) -> u8 {
        self.index % 12
    }

    /// The romanised stem name.
    #[must_use]
    pub const fn stem_name(self) -> &'static str {
        HEAVENLY_STEMS[(self.index % 10) as usize]
    }

    /// The romanised branch name.
    #[must_use]
    pub const fn branch_name(self) -> &'static str {
        EARTHLY_BRANCHES[(self.index % 12) as usize]
    }

    /// The zodiac animal associated with the branch.
    #[must_use]
    pub const fn zodiac_animal(self) -> &'static str {
        ZODIAC_ANIMALS[(self.index % 12) as usize]
    }

    /// The five-phase element associated with the stem.
    #[must_use]
    pub const fn five_phase(self) -> &'static str {
        FIVE_PHASES[((self.index % 10) / 2) as usize]
    }

    /// The next position in the cycle.
    #[must_use]
    pub const fn next(self) -> Self {
        Self::from_index(self.index as i64 + 1)
    }
}

/// The sexagenary year of a Chinese-style calendar year number.
///
/// Year 1 of the traditional Chinese reckoning is *jia-zi*, index 0, but the
/// commonly used anchor is that Gregorian 1984 was a *jia-zi* year.
#[must_use]
pub const fn sexagenary_year(chinese_year: i64) -> Sexagenary {
    Sexagenary::from_index(chinese_year - 1)
}

/// The sexagenary day of a fixed day.
///
/// The day cycle has run without interruption for longer than any surviving
/// calendar; the anchor used here is that RD 1 (`0001-01-01` proleptic
/// Gregorian) was *jia-zi* day index 14.
#[must_use]
pub const fn sexagenary_day(rd: Rd) -> Sexagenary {
    Sexagenary::from_index(rd.0 + 14)
}

/// A generic named cycle: `n` positions repeating from an anchor day.
#[derive(Debug, Clone, Copy)]
pub struct NamedDayCycle<'a> {
    /// The names, in cycle order.
    pub names: &'a [&'a str],
    /// A fixed day that occupies position zero.
    pub anchor: Rd,
}

impl<'a> NamedDayCycle<'a> {
    /// Build a cycle from its names and anchor.
    #[must_use]
    pub const fn new(names: &'a [&'a str], anchor: Rd) -> Self {
        Self { names, anchor }
    }

    /// The zero-based position of a day within the cycle.
    #[must_use]
    pub fn position(&self, rd: Rd) -> Option<usize> {
        if self.names.is_empty() {
            return None;
        }
        Some((rd.0 - self.anchor.0).rem_euclid(self.names.len() as i64) as usize)
    }

    /// The name of a day's position.
    #[must_use]
    pub fn name(&self, rd: Rd) -> Option<&'a str> {
        self.position(rd).map(|index| self.names[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cycle_starts_at_jia_zi() {
        let first = Sexagenary::from_index(0);
        assert_eq!(first.stem_name(), "jia");
        assert_eq!(first.branch_name(), "zi");
        assert_eq!(first.zodiac_animal(), "rat");
        assert_eq!(first.five_phase(), "wood");
        assert_eq!(first.ordinal(), 1);
    }

    #[test]
    fn the_cycle_closes_after_sixty_steps() {
        let mut position = Sexagenary::from_index(0);
        for _ in 0..60 {
            position = position.next();
        }
        assert_eq!(position, Sexagenary::from_index(0));
    }

    #[test]
    fn stems_and_branches_advance_together() {
        for index in 0..60 {
            let position = Sexagenary::from_index(index);
            assert_eq!(position.stem_index() % 2, position.branch_index() % 2);
        }
    }

    #[test]
    fn impossible_stem_branch_pairs_are_rejected() {
        assert_eq!(
            Sexagenary::from_stem_branch(1, 1),
            Some(Sexagenary::from_index(0))
        );
        // jia (odd) with chou (even) never occurs.
        assert_eq!(Sexagenary::from_stem_branch(1, 2), None);
        assert_eq!(Sexagenary::from_stem_branch(0, 1), None);
        assert_eq!(Sexagenary::from_stem_branch(11, 1), None);
    }

    #[test]
    fn nineteen_eighty_four_is_a_jia_zi_year() {
        // Chinese year 4681 corresponds to Gregorian 1984.
        assert_eq!(sexagenary_year(4_681).index(), 0);
    }

    #[test]
    fn the_day_cycle_advances_by_one_per_day() {
        let today = sexagenary_day(Rd(1));
        assert_eq!(sexagenary_day(Rd(2)), today.next());
        assert_eq!(sexagenary_day(Rd(61)), today);
    }

    #[test]
    fn negative_indices_wrap_into_the_cycle() {
        assert_eq!(Sexagenary::from_index(-1).index(), 59);
        assert_eq!(Sexagenary::from_index(-61).index(), 59);
    }

    #[test]
    fn named_cycles_wrap_in_both_directions() {
        let cycle = NamedDayCycle::new(&["a", "b", "c"], Rd(0));
        assert_eq!(cycle.name(Rd(0)), Some("a"));
        assert_eq!(cycle.name(Rd(4)), Some("b"));
        assert_eq!(cycle.name(Rd(-1)), Some("c"));
        assert_eq!(NamedDayCycle::new(&[], Rd(0)).name(Rd(0)), None);
    }
}
