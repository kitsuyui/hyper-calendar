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

/// A positional cycle a calendar runs, and what it is called.
///
/// "Cycle" covers everything a date is built out of that repeats and whose
/// positions have names: months, weekdays, the ten concurrent weeks of the
/// Pawukon, the sexagenary cycle, the twenty day-signs of the tzolkʼin.
/// They differ in length and in number, and nothing here assumes otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CycleShape {
    /// A stable identifier for the *kind* of cycle, lowercase and
    /// hyphenated: `month`, `weekday`, `decade-day`, `trecena`,
    /// `day-sign`, `sexagenary`.
    ///
    /// Shared across calendars on purpose. Two calendars that both run a
    /// seven-day week name it `weekday`, so a locale's weekday names serve
    /// both without being stored twice.
    pub kind: &'static str,
    /// How many positions.
    pub length: CycleLength,
}

impl CycleShape {
    /// A cycle of a fixed number of positions.
    #[must_use]
    pub const fn fixed(kind: &'static str, length: u16) -> Self {
        Self {
            kind,
            length: CycleLength::Fixed(length),
        }
    }

    /// A cycle that gains a position in some years.
    #[must_use]
    pub const fn intercalary(kind: &'static str, ordinary: u16, extended: u16) -> Self {
        Self {
            kind,
            length: CycleLength::Intercalary { ordinary, extended },
        }
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
    fn the_named_shapes_are_what_they_say() {
        assert_eq!(SOLAR_TWELVE.len(), 2);
        assert!(SOLAR_TWELVE[0].length.accepts(12));
        assert!(!SOLAR_TWELVE[0].length.accepts(13));
        assert!(LUNISOLAR_TWELVE[0].length.accepts(13));
        assert!(WANDERING_THIRTEEN[0].length.accepts(13));
    }
}
