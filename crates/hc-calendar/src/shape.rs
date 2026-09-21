//! What a calendar is *made of*: the positional cycles it runs.
//!
//! # The hole this closes
//!
//! Everything a calendar needs said in words — month names, weekday names,
//! the names of a ten-day décade or a thirteen-day trecena — has to be
//! stored somewhere, and that store has to know how many names to expect.
//!
//! `hc-i18n` used to assume the Gregorian answer: twelve or thirteen
//! months, seven weekdays. That assumption was not a limitation to be
//! worked around later; it was a *hole generator*. The Badíʿ calendar has
//! nineteen months and could not be given names at all. The Maya Haabʼ has
//! nineteen. The Aztec Xiuhpōhualli has eighteen and a remainder. The
//! French Republican week is ten days long, so it had nowhere to live even
//! though the calendar itself was implemented. A test asserted the twelve
//! or thirteen, which meant the library would *reject* the correct data if
//! anyone supplied it.
//!
//! So the shape moves to the calendar, where it is known, and the
//! vocabulary is keyed to it. A calendar that declares nineteen months can
//! be given nineteen names; one that declares a ten-day week can be given
//! ten. Nothing has to be special-cased, because nothing is assumed.
//!
//! # Why every calendar must declare one
//!
//! [`crate::Calendar::cycles`] has no default. It began as a defaulted
//! method, so that a calendar could declare its shape when it had one and
//! stay silent otherwise, with the silence reported by a test. Half the
//! registry stayed silent. A gap a test can only report is a gap that
//! persists; a gap the compiler refuses cannot. So a calendar with no named
//! cycles — a day count — says so with an empty slice, and a calendar that
//! says nothing does not build.

/// How many positions a cycle has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CycleLength {
    /// Always this many. Twelve Gregorian months, seven weekdays, sixty
    /// stems-and-branches.
    Fixed(u16),
    /// This many in an ordinary year and one more in some years — a
    /// lunisolar calendar's intercalary month, a wandering calendar's
    /// epagomenal remainder.
    ///
    /// The vocabulary may carry either count; both are correct.
    Intercalary {
        /// The ordinary count.
        ordinary: u16,
        /// The count in a year that intercalates.
        extended: u16,
    },
}

impl CycleLength {
    /// Whether `count` is a length this cycle can have.
    #[must_use]
    pub const fn accepts(self, count: u16) -> bool {
        match self {
            Self::Fixed(length) => count == length,
            Self::Intercalary { ordinary, extended } => count == ordinary || count == extended,
        }
    }

    /// The largest number of positions the cycle ever has.
    #[must_use]
    pub const fn maximum(self) -> u16 {
        match self {
            Self::Fixed(length) => length,
            Self::Intercalary { extended, .. } => extended,
        }
    }
}

/// A positional cycle a calendar runs, what it is called, and what the
/// calendar itself calls its positions.
///
/// "Cycle" covers everything a date is built out of that repeats and whose
/// positions have names or numbers: months, weekdays, the ten concurrent
/// weeks of the Pawukon, the stems and branches, the twenty day-signs of
/// the tzolkʼin. They differ in length and in number, and nothing here
/// assumes otherwise.
///
/// # The month slot
///
/// A calendar whose [`DateFields`](crate::DateFields) carry a `month`
/// declares a cycle of kind [`MONTH`], and one whose fields carry none
/// declares no such cycle. The Pawukon's *wuku* is its month by that rule,
/// and the ISO week calendar has none. `hyper-calendar`'s vocabulary test
/// holds every registered calendar to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CycleShape {
    /// A stable identifier for the *kind* of cycle, lowercase and
    /// hyphenated: `month`, `weekday`, `decade-day`, `trecena`,
    /// `day-sign`, `stem`.
    ///
    /// Shared across calendars on purpose. Two calendars that both run a
    /// seven-day week name it `weekday`, so a locale's weekday names serve
    /// both without being stored twice.
    pub kind: &'static str,
    /// How many positions.
    pub length: CycleLength,
    /// The calendar's own names for the positions, position 1 at index 0,
    /// in the orthography the calendar's sources use.
    ///
    /// Empty when the positions are numbered rather than named (the
    /// tzolkʼin's thirteen, a lunisolar calendar's months), when the count
    /// varies between years, or when a name is a matter of locale rather
    /// than of the calendar — a Gregorian month is January to English and
    /// janvier to French, and this slice is not where either belongs.
    /// `hc-i18n` consults a locale first and falls back to these.
    pub names: &'static [&'static str],
}

impl CycleShape {
    /// A cycle of a fixed number of numbered positions.
    #[must_use]
    pub const fn fixed(kind: &'static str, length: u16) -> Self {
        Self {
            kind,
            length: CycleLength::Fixed(length),
            names: &[],
        }
    }

    /// A cycle whose positions the calendar names itself. Its length is the
    /// number of names, so the two cannot disagree.
    ///
    /// # Panics
    ///
    /// If there are more than `u16::MAX` names, which no calendar has.
    #[must_use]
    pub const fn named(kind: &'static str, names: &'static [&'static str]) -> Self {
        assert!(names.len() <= u16::MAX as usize);
        Self {
            kind,
            length: CycleLength::Fixed(names.len() as u16),
            names,
        }
    }

    /// A cycle that gains a position in some years.
    #[must_use]
    pub const fn intercalary(kind: &'static str, ordinary: u16, extended: u16) -> Self {
        Self {
            kind,
            length: CycleLength::Intercalary { ordinary, extended },
            names: &[],
        }
    }

    /// The calendar's own name for a zero-based position, if it has one.
    #[must_use]
    pub const fn name(&self, index: usize) -> Option<&'static str> {
        if index < self.names.len() {
            Some(self.names[index])
        } else {
            None
        }
    }
}

/// One way of naming the `N` positions of a cycle: a language, a script or
/// a convention, with its source.
///
/// A cycle that many languages name — the twenty-four solar terms, the
/// twenty-eight mansions, the twelve signs — has no single set of names, and
/// the set of ways to name it is open: a Korean or Vietnamese column is an
/// entry somebody adds, not a field a struct has to be given (ADR 0007). So
/// each cycle keeps a catalogue of `Naming`s, one per language or
/// convention, each holding exactly `N` names and saying where they came
/// from. [`CycleShape::names`] is the special case of a calendar's own
/// names; [`crate::cycle::readings::Reading`] is the sexagenary cycle's,
/// which has two arrays rather than one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Naming<const N: usize> {
    /// A short identifier: the language, then the script or romanisation
    /// where the language has more than one, or the convention.
    pub id: &'static str,
    /// What to call the naming in English.
    pub english_name: &'static str,
    /// The names, position 0 first.
    pub names: &'static [&'static str; N],
    /// Where the names come from.
    pub authority: &'static str,
}

impl<const N: usize> Naming<N> {
    /// The name of a zero-based position.
    ///
    /// # Panics
    ///
    /// If `index` is `N` or more.
    #[must_use]
    pub const fn name(&self, index: usize) -> &'static str {
        self.names[index]
    }
}

/// The kind name for a calendar's months.
pub const MONTH: &str = "month";

/// The kind name for a seven-day week.
pub const WEEKDAY: &str = "weekday";

/// The twelve months and seven weekdays every Gregorian-shaped calendar
/// runs, which is most of them.
pub const SOLAR_TWELVE: &[CycleShape] =
    &[CycleShape::fixed(MONTH, 12), CycleShape::fixed(WEEKDAY, 7)];

/// Twelve months with a thirteenth in a leap year, plus the seven-day week:
/// the lunisolar shape.
pub const LUNISOLAR_TWELVE: &[CycleShape] = &[
    CycleShape::intercalary(MONTH, 12, 13),
    CycleShape::fixed(WEEKDAY, 7),
];

/// Twelve months of thirty days and a short thirteenth, plus the seven-day
/// week: the wandering-year shape of the Egyptian, Coptic, Ethiopic and
/// Armenian calendars.
pub const WANDERING_THIRTEEN: &[CycleShape] =
    &[CycleShape::fixed(MONTH, 13), CycleShape::fixed(WEEKDAY, 7)];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fixed_cycle_accepts_only_its_own_length() {
        let week = CycleShape::fixed(WEEKDAY, 7);
        assert!(week.length.accepts(7));
        assert!(!week.length.accepts(6));
        assert!(!week.length.accepts(8));
        assert_eq!(week.length.maximum(), 7);
    }

    #[test]
    fn an_intercalary_cycle_accepts_both_of_its_lengths() {
        let months = CycleShape::intercalary(MONTH, 12, 13);
        assert!(months.length.accepts(12));
        assert!(months.length.accepts(13));
        assert!(!months.length.accepts(14));
        assert_eq!(months.length.maximum(), 13);
    }

    /// The shapes this module names are the common ones; the point of the
    /// type is that a calendar is not limited to them.
    #[test]
    fn an_unusual_shape_is_as_expressible_as_a_common_one() {
        // The Badíʿ calendar: nineteen months of nineteen days, plus the
        // intercalary Ayyám-i-Há, and a nineteen-day week of its own.
        let badi = [
            CycleShape::fixed(MONTH, 19),
            CycleShape::fixed("badi-day", 19),
        ];
        assert!(badi[0].length.accepts(19));

        // The French Republican décade: ten days, not seven.
        let republican = [
            CycleShape::fixed(MONTH, 12),
            CycleShape::fixed("decade-day", 10),
        ];
        assert!(republican[1].length.accepts(10));
        assert!(!republican[1].length.accepts(7));

        // The Pawukon runs ten concurrent cycles at once. Its seven-day
        // saptawara is the ordinary week, so it takes the shared kind.
        let pawukon: [CycleShape; 10] = [
            CycleShape::fixed("ekawara", 1),
            CycleShape::fixed("dwiwara", 2),
            CycleShape::fixed("triwara", 3),
            CycleShape::fixed("caturwara", 4),
            CycleShape::fixed("pancawara", 5),
            CycleShape::fixed("sadwara", 6),
            CycleShape::fixed(WEEKDAY, 7),
            CycleShape::fixed("astawara", 8),
            CycleShape::fixed("sangawara", 9),
            CycleShape::fixed("dasawara", 10),
        ];
        for (index, cycle) in pawukon.iter().enumerate() {
            assert!(cycle.length.accepts(index as u16 + 1));
        }
    }

    #[test]
    fn a_named_cycle_is_as_long_as_its_names_and_answers_for_each() {
        const SIGNS: &[&str] = &["Imix", "Ikʼ", "Akʼbʼal"];
        let cycle = CycleShape::named("day-sign", SIGNS);
        assert_eq!(cycle.length, CycleLength::Fixed(3));
        assert_eq!(cycle.name(0), Some("Imix"));
        assert_eq!(cycle.name(2), Some("Akʼbʼal"));
        assert_eq!(cycle.name(3), None);
        assert_eq!(CycleShape::fixed("trecena", 13).name(0), None);
    }

    #[test]
    fn the_named_shapes_are_what_they_say() {
        assert_eq!(SOLAR_TWELVE.len(), 2);
        assert!(SOLAR_TWELVE[0].length.accepts(12));
        assert!(!SOLAR_TWELVE[0].length.accepts(13));
        assert!(LUNISOLAR_TWELVE[0].length.accepts(13));
        assert!(WANDERING_THIRTEEN[0].length.accepts(13));
    }
}
