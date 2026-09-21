//! The shape of the locale data, and the lookup that finds it.
//!
//! # The CLDR model
//!
//! Unicode TR 35 §7 gives every relative field two independent things:
//!
//! * `<relative type="-1">` — the **special word**: *yesterday*, *last
//!   week*, *now*. There is at most one per offset and it takes no number.
//! * `<relativeTime type="past">` — the **numeric pattern**, one per plural
//!   category: *{0} day ago*, *{0} дня назад*, *{0} дней назад*.
//!
//! They are not interchangeable and neither is derivable from the other, so
//! [`UnitPatterns`] carries both. `Intl.RelativeTimeFormat`'s `numeric`
//! option is exactly the choice between them, and
//! [`crate::relative::Numeric`] is the same choice by the same name.
//!
//! A third set, [`UnitPatterns::count`], holds the same unit with **no
//! direction at all** — *{0} day*, *3 Tage*. CLDR keeps these in a separate
//! `<unit type="duration-day">` element for a reason: German says *3 Tage*
//! but *vor 3 Tagen*, because the preposition governs the dative. Deriving
//! one from the other by stripping a prefix works in English and breaks in
//! German, so both are stored.
//!
//! # Empty means inherit
//!
//! An empty `&'static str` is not an empty phrase, it is a statement that
//! the entry does not say. Lookup then tries, in order, the wider style
//! (narrow → short → long) and then the next locale up
//! [`hc_i18n::Locale::fallback`]. So a locale that abbreviates nothing
//! states its long forms and stops, and a plural category a language does
//! not use falls through to `other`.

use hc_i18n::PluralCategory;

use crate::unit::TimeUnit;

/// One pattern per CLDR plural category.
///
/// A category left empty falls back to `other`, which is the category every
/// language has.
#[derive(Debug, Clone, Copy)]
pub struct PluralForms {
    /// The `zero` pattern.
    pub zero: &'static str,
    /// The `one` pattern.
    pub one: &'static str,
    /// The `two` pattern.
    pub two: &'static str,
    /// The `few` pattern.
    pub few: &'static str,
    /// The `many` pattern.
    pub many: &'static str,
    /// The `other` pattern, which every language needs.
    pub other: &'static str,
}

impl PluralForms {
    /// No patterns at all: an entry that says nothing and inherits.
    pub const EMPTY: Self = Self {
        zero: "",
        one: "",
        two: "",
        few: "",
        many: "",
        other: "",
    };

    /// Whether this set states nothing.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.other.is_empty()
    }

    /// The pattern for a category, falling back to `other`.
    #[must_use]
    pub const fn get(&self, category: PluralCategory) -> &'static str {
        let chosen = match category {
            PluralCategory::Zero => self.zero,
            PluralCategory::One => self.one,
            PluralCategory::Two => self.two,
            PluralCategory::Few => self.few,
            PluralCategory::Many => self.many,
            PluralCategory::Other => self.other,
        };
        if chosen.is_empty() {
            self.other
        } else {
            chosen
        }
    }
}

/// Everything one locale says about one unit, in one style.
#[derive(Debug, Clone, Copy)]
pub struct UnitPatterns {
    /// `<relativeTime type="past">`: *{0} days ago*.
    pub past: PluralForms,
    /// `<relativeTime type="future">`: *in {0} days*.
    pub future: PluralForms,
    /// The undirected unit phrase: *{0} days*.
    pub count: PluralForms,
    /// `<relative type="-1">`: *yesterday*, *last week*.
    pub previous: &'static str,
    /// `<relative type="0">`: *today*, *this week*, *now*.
    pub current: &'static str,
    /// `<relative type="1">`: *tomorrow*, *next week*.
    pub next: &'static str,
    /// `<relative type="-2">`, where the language has a single word for it.
    pub previous_2: &'static str,
    /// `<relative type="2">`, likewise.
    pub next_2: &'static str,
    /// The idiom for exactly half a unit: *half an hour*.
    ///
    /// Not a CLDR field. Empty means the formatter should fall back to
    /// rendering the decimal `0.5` through the plural rules.
    pub half: &'static str,
    /// The idiom for one and a half units: *an hour and a half*.
    ///
    /// Not a CLDR field; empty falls back to the decimal `1.5`.
    pub one_and_a_half: &'static str,
}

impl UnitPatterns {
    /// An entry that says nothing and inherits everything.
    pub const EMPTY: Self = Self {
        past: PluralForms::EMPTY,
        future: PluralForms::EMPTY,
        count: PluralForms::EMPTY,
        previous: "",
        current: "",
        next: "",
        previous_2: "",
        next_2: "",
        half: "",
        one_and_a_half: "",
    };

    /// Whether this entry states no numeric pattern.
    ///
    /// Style fallback keys on this: a style that gives no numeric pattern
    /// for a unit is treated as not stating that unit at all, even if it
    /// happened to give a special word.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.past.is_empty() && self.future.is_empty() && self.count.is_empty()
    }

    /// The special word for an integer offset, if the language has one.
    #[must_use]
    pub const fn special(&self, offset: i64) -> &'static str {
        match offset {
            -2 => self.previous_2,
            -1 => self.previous,
            0 => self.current,
            1 => self.next,
            2 => self.next_2,
            _ => "",
        }
    }
}

/// One locale's patterns for every unit, in one style.
#[derive(Debug, Clone, Copy)]
pub struct StyleData {
    /// Seconds.
    pub second: UnitPatterns,
    /// Minutes.
    pub minute: UnitPatterns,
    /// Hours.
    pub hour: UnitPatterns,
    /// Days.
    pub day: UnitPatterns,
    /// Weeks.
    pub week: UnitPatterns,
    /// Months.
    pub month: UnitPatterns,
    /// Quarters.
    pub quarter: UnitPatterns,
    /// Years.
    pub year: UnitPatterns,
}

impl StyleData {
    /// A style that states nothing, so every unit inherits.
    pub const EMPTY: Self = Self {
        second: UnitPatterns::EMPTY,
        minute: UnitPatterns::EMPTY,
        hour: UnitPatterns::EMPTY,
        day: UnitPatterns::EMPTY,
        week: UnitPatterns::EMPTY,
        month: UnitPatterns::EMPTY,
        quarter: UnitPatterns::EMPTY,
        year: UnitPatterns::EMPTY,
    };

    /// The patterns for one unit.
    #[must_use]
    pub const fn get(&self, unit: TimeUnit) -> &UnitPatterns {
        match unit {
            TimeUnit::Second => &self.second,
            TimeUnit::Minute => &self.minute,
            TimeUnit::Hour => &self.hour,
            TimeUnit::Day => &self.day,
            TimeUnit::Week => &self.week,
            TimeUnit::Month => &self.month,
            TimeUnit::Quarter => &self.quarter,
            TimeUnit::Year => &self.year,
        }
    }
}

/// One plain string per unit, with no `{0}` in it.
///
/// Two fields of [`LocaleData`] have this shape and neither is a CLDR
/// pattern: the compact suffixes of *2h30m*, and the indefinite singular an
/// approximation prefers over a numeral — *about an hour*, not *about 1
/// hour*. A language with no indefinite article leaves the second one empty
/// and gets the numeral.
#[derive(Debug, Clone, Copy)]
pub struct UnitStrings {
    /// Seconds.
    pub second: &'static str,
    /// Minutes.
    pub minute: &'static str,
    /// Hours.
    pub hour: &'static str,
    /// Days.
    pub day: &'static str,
    /// Weeks.
    pub week: &'static str,
    /// Months.
    pub month: &'static str,
    /// Quarters.
    pub quarter: &'static str,
    /// Years.
    pub year: &'static str,
}

impl UnitStrings {
    /// Nothing stated.
    pub const EMPTY: Self = Self {
        second: "",
        minute: "",
        hour: "",
        day: "",
        week: "",
        month: "",
        quarter: "",
        year: "",
    };

    /// The string for one unit.
    #[must_use]
    pub const fn get(&self, unit: TimeUnit) -> &'static str {
        match unit {
            TimeUnit::Second => self.second,
            TimeUnit::Minute => self.minute,
            TimeUnit::Hour => self.hour,
            TimeUnit::Day => self.day,
            TimeUnit::Week => self.week,
            TimeUnit::Month => self.month,
            TimeUnit::Quarter => self.quarter,
            TimeUnit::Year => self.year,
        }
    }
}

/// CLDR list patterns: how a language joins two, and how it joins more.
///
/// The four slots are TR 35 §12's own: `two` for a two-item list, and
/// `start`/`middle`/`end` for longer ones. English needs all four because
/// only the last join carries the conjunction.
#[derive(Debug, Clone, Copy)]
pub struct ListForms {
    /// Joining exactly two items.
    pub two: &'static str,
    /// Joining the first item to the rest.
    pub start: &'static str,
    /// Joining items in the middle.
    pub middle: &'static str,
    /// Joining the last item to the rest.
    pub end: &'static str,
}

impl ListForms {
    /// Nothing stated.
    pub const EMPTY: Self = Self {
        two: "",
        start: "",
        middle: "",
        end: "",
    };

    /// Whether this set states nothing.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.two.is_empty()
    }
}

/// The three CLDR list types this crate uses.
#[derive(Debug, Clone, Copy)]
pub struct ListPatterns {
    /// `listPattern type="standard"`: the one with the conjunction.
    pub standard: ListForms,
    /// `listPattern type="unit"`: comma-joined, no conjunction.
    pub unit: ListForms,
    /// `listPattern type="unit-narrow"`: usually just a space.
    pub narrow: ListForms,
}

impl ListPatterns {
    /// Nothing stated.
    pub const EMPTY: Self = Self {
        standard: ListForms::EMPTY,
        unit: ListForms::EMPTY,
        narrow: ListForms::EMPTY,
    };
}

/// The hedges an approximation wraps a phrase in.
///
/// Not CLDR: CLDR has no vocabulary for *just over a week*. These are
/// ordinary translations kept in the same table so that adding a language
/// stays a single edit.
#[derive(Debug, Clone, Copy)]
pub struct ApproximatePatterns {
    /// No hedge at all — normally just `{0}`.
    pub exactly: &'static str,
    /// *about {0}*.
    pub about: &'static str,
    /// *just over {0}*.
    pub just_over: &'static str,
    /// *over {0}*.
    pub over: &'static str,
    /// *nearly {0}*.
    pub nearly: &'static str,
    /// *less than {0}*.
    pub less_than: &'static str,
    /// *more than {0}*.
    pub more_than: &'static str,
}

impl ApproximatePatterns {
    /// Nothing stated.
    pub const EMPTY: Self = Self {
        exactly: "",
        about: "",
        just_over: "",
        over: "",
        nearly: "",
        less_than: "",
        more_than: "",
    };
}

/// How a language points at a named weekday: *last Tuesday*.
#[derive(Debug, Clone, Copy)]
pub struct WeekdayPatterns {
    /// *last {0}*.
    pub previous: &'static str,
    /// *this {0}*.
    pub current: &'static str,
    /// *next {0}*.
    pub next: &'static str,
}

impl WeekdayPatterns {
    /// Nothing stated.
    pub const EMPTY: Self = Self {
        previous: "",
        current: "",
        next: "",
    };
}

/// Everything this crate knows about one locale.
///
/// This struct is the unit of data. Adding a language means writing one
/// value of it in [`crate::data`] and adding its name to
/// [`crate::data::LOCALES`]; no function in this crate gains a branch.
#[derive(Debug, Clone, Copy)]
pub struct LocaleData {
    /// The canonical tag this entry answers for, such as `zh-Hans`.
    pub tag: &'static str,
    /// Full phrases: *3 days ago*.
    pub long: StyleData,
    /// Abbreviated phrases: *3 days ago*, *3 d. ago*, locale depending.
    pub short: StyleData,
    /// The tightest phrases the language has: *3d ago*.
    pub narrow: StyleData,
    /// Suffixes for the compact form *2h30m*.
    pub compact: UnitStrings,
    /// The indefinite singular of each unit — *an hour*, *ein Jahr* — used
    /// by [`crate::approximate`] in place of the numeral `1`. Empty for a
    /// language with no indefinite article, which then gets the numeral.
    pub indefinite: UnitStrings,
    /// How the language joins a list of components.
    pub list: ListPatterns,
    /// The approximation hedges.
    pub approximate: ApproximatePatterns,
    /// How the language points at a named weekday.
    pub weekday: WeekdayPatterns,
    /// The decimal separator, needed by the half-unit forms.
    pub decimal_separator: &'static str,
    /// How a day phrase and a time of day combine: *{0} at {1}*.
    pub at_pattern: &'static str,
}

/// Which of the three styles a lookup wants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum RelativeStyle {
    /// *3 days ago*. The default, and the only style every entry states.
    #[default]
    Long,
    /// *3 d. ago* where the language abbreviates, *3 days ago* where it
    /// does not.
    Short,
    /// *3d ago*.
    Narrow,
}

impl RelativeStyle {
    /// Every style, widest first.
    pub const ALL: [Self; 3] = [Self::Long, Self::Short, Self::Narrow];

    /// The style to try when this one has no data.
    ///
    /// Narrow degrades to short and short to long, which is CLDR's own
    /// width inheritance and the reason a locale need only state the widths
    /// it actually distinguishes.
    #[must_use]
    pub const fn wider(self) -> Option<Self> {
        match self {
            Self::Narrow => Some(Self::Short),
            Self::Short => Some(Self::Long),
            Self::Long => None,
        }
    }

    /// The style's data within one locale entry.
    #[must_use]
    pub const fn within(self, data: &LocaleData) -> &StyleData {
        match self {
            Self::Long => &data.long,
            Self::Short => &data.short,
            Self::Narrow => &data.narrow,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUSSIAN_DAY: PluralForms = PluralForms {
        zero: "",
        one: "{0} день",
        two: "",
        few: "{0} дня",
        many: "{0} дней",
        other: "{0} дня",
    };

    #[test]
    fn a_category_the_data_leaves_empty_falls_back_to_other() {
        // Russian has no dual, so `two` is empty and must land on `other`.
        assert_eq!(RUSSIAN_DAY.get(PluralCategory::Two), "{0} дня");
        assert_eq!(RUSSIAN_DAY.get(PluralCategory::Zero), "{0} дня");
        assert_eq!(RUSSIAN_DAY.get(PluralCategory::One), "{0} день");
        assert_eq!(RUSSIAN_DAY.get(PluralCategory::Many), "{0} дней");
    }

    #[test]
    fn an_entry_that_states_nothing_is_empty_in_every_category() {
        assert!(PluralForms::EMPTY.is_empty());
        for category in PluralCategory::ALL {
            assert_eq!(PluralForms::EMPTY.get(category), "");
        }
        assert!(!RUSSIAN_DAY.is_empty());
    }

    #[test]
    fn styles_degrade_narrow_to_short_to_long_and_stop() {
        assert_eq!(RelativeStyle::Narrow.wider(), Some(RelativeStyle::Short));
        assert_eq!(RelativeStyle::Short.wider(), Some(RelativeStyle::Long));
        assert_eq!(RelativeStyle::Long.wider(), None);
        assert_eq!(RelativeStyle::default(), RelativeStyle::Long);
        assert_eq!(RelativeStyle::ALL.len(), 3);
    }

    #[test]
    fn special_words_are_indexed_by_offset_and_stop_at_two() {
        let mut day = UnitPatterns::EMPTY;
        day.previous_2 = "the day before yesterday";
        day.previous = "yesterday";
        day.current = "today";
        day.next = "tomorrow";
        day.next_2 = "the day after tomorrow";
        assert_eq!(day.special(-2), "the day before yesterday");
        assert_eq!(day.special(-1), "yesterday");
        assert_eq!(day.special(0), "today");
        assert_eq!(day.special(1), "tomorrow");
        assert_eq!(day.special(2), "the day after tomorrow");
        assert_eq!(day.special(3), "");
        assert_eq!(day.special(-3), "");
        assert_eq!(day.special(i64::MIN), "");
    }

    #[test]
    fn a_unit_with_only_special_words_still_counts_as_unstated() {
        // Style fallback keys on the numeric patterns, so a style that has
        // a word for "yesterday" and no way to say "5 days ago" must not
        // block the wider style from being consulted.
        let mut day = UnitPatterns::EMPTY;
        day.previous = "yesterday";
        assert!(day.is_empty());
    }

    #[test]
    fn every_unit_of_a_style_is_reachable_by_its_own_name() {
        let data = StyleData::EMPTY;
        for unit in TimeUnit::ALL {
            assert!(data.get(unit).is_empty());
        }
        let strings = UnitStrings::EMPTY;
        for unit in TimeUnit::ALL {
            assert_eq!(strings.get(unit), "");
        }
    }
}
