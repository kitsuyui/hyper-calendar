//! The dog days, the *dies caniculares*: the hottest stretch of the
//! European summer, named for Sirius, under each convention that dates it.
//!
//! The span is fixed in the calendar, but the conventions do not agree on
//! it, so each is its own named convention (`docs/policy.md` §5):
//!
//! | Convention | Span |
//! | --- | --- |
//! | [`DogDaysConvention::OldFarmersAlmanac`] | 3 July to 11 August, "that specific 40-day period" of *The Old Farmer's Almanac* |
//! | [`DogDaysConvention::Hundstage`] | 23 July to 23 August, the *Hundstage* as MeteoSchweiz dates them |
//!
//! Both are Gregorian dates.
//!
//! Not carried: the Book of Common Prayer's spans. The English Wikipedia
//! gives "July 7 to September 5" for the sixteenth-century Prayer Books,
//! citing the calendars of 1552 and 1559, and 19 July to 20 August after
//! 1660 and 30 July to 7 September after 1752, citing Townsend's *Manual of
//! Dates* (1862); search excerpts of other pages, not read, give the
//! 1552 and 1559 calendars as 6 July to 17 August and 7 July to 18 August. The calendars
//! themselves could not be read here, Townsend was not read, and the
//! later spans do not follow from one another by the eleven days of 1752,
//! so none of them is carried until a Prayer Book's own calendar is.
//!
//! Sources:
//!
//! * `ofa-dog-days`: Catherine Boeckmann, "What Are the Dog Days of
//!   Summer?", *The Old Farmer's Almanac*,
//!   <https://www.almanac.com/content/what-are-dog-days-summer>, modified
//!   2026-09-09, retrieved 2026-09-26: "the Dog Days of Summer run from
//!   July 3 through August 11".
//! * `meteoschweiz-hundstage-2025`: MeteoSchweiz, "Die Hundstage – in der
//!   Regel die heisseste Zeit des Jahres", MeteoSchweiz-Blog, 22 July 2025,
//!   retrieved 2026-09-26: "Morgen, den 23. Juli, beginnen die sogenannten
//!   Hundstage und dauern bis zum 23. August"; the German Wikipedia,
//!   "Hundstage", retrieved 2026-09-26, gives the same span and cites it.
//! * Wikipedia, "Dog days", retrieved 2026-09-26, for the Prayer Book
//!   spans not carried.

use hc_calendar::Rd;

use crate::gregorian;

/// Whose dog days these are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DogDaysConvention {
    /// *The Old Farmer's Almanac*: 3 July to 11 August.
    OldFarmersAlmanac,
    /// The German-language *Hundstage*: 23 July to 23 August.
    Hundstage,
}

impl DogDaysConvention {
    /// Every convention, in the order of the module table.
    pub const ALL: [Self; 2] = [Self::OldFarmersAlmanac, Self::Hundstage];

    /// A short identifier, in kebab case.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::OldFarmersAlmanac => "dog-days-old-farmers-almanac",
            Self::Hundstage => "hundstage",
        }
    }

    /// The name in English.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::OldFarmersAlmanac => "Dog Days (The Old Farmer's Almanac)",
            Self::Hundstage => "Dog days (Hundstage)",
        }
    }

    /// The name in the convention's own language.
    #[must_use]
    pub const fn local_name(self) -> &'static str {
        match self {
            Self::OldFarmersAlmanac => "Dog Days",
            Self::Hundstage => "Hundstage",
        }
    }

    /// The first and last Gregorian month and day, both included.
    #[must_use]
    pub const fn month_days(self) -> ((u8, u8), (u8, u8)) {
        match self {
            Self::OldFarmersAlmanac => ((7, 3), (8, 11)),
            Self::Hundstage => ((7, 23), (8, 23)),
        }
    }

    /// The first and last day of the dog days in `year`, both included.
    #[must_use]
    pub const fn span(self, year: i64) -> (Rd, Rd) {
        let ((first_month, first_day), (last_month, last_day)) = self.month_days();
        (
            gregorian::from_year_month_day(year, first_month, first_day),
            gregorian::from_year_month_day(year, last_month, last_day),
        )
    }

    /// How many days the dog days last.
    #[must_use]
    pub const fn length(self) -> i64 {
        let (first, last) = self.span(2000);
        last.0 - first.0 + 1
    }

    /// Whether `day` is one of the dog days.
    #[must_use]
    pub const fn contains(self, day: Rd) -> bool {
        let (first, last) = self.span(gregorian::year_from_rd(day));
        first.0 <= day.0 && day.0 <= last.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::from_year_month_day;

    #[test]
    fn the_almanacs_forty_days_run_from_3_july_to_11_august() {
        let convention = DogDaysConvention::OldFarmersAlmanac;
        assert_eq!(convention.length(), 40);
        assert_eq!(
            convention.span(2026),
            (
                from_year_month_day(2026, 7, 3),
                from_year_month_day(2026, 8, 11)
            )
        );
        assert!(!convention.contains(from_year_month_day(2026, 7, 2)));
        assert!(convention.contains(from_year_month_day(2026, 7, 3)));
        assert!(convention.contains(from_year_month_day(2026, 8, 11)));
        assert!(!convention.contains(from_year_month_day(2026, 8, 12)));
    }

    #[test]
    fn the_hundstage_run_from_23_july_to_23_august() {
        // MeteoSchweiz, 22 July 2025: "Morgen, den 23. Juli, beginnen die
        // sogenannten Hundstage und dauern bis zum 23. August".
        let convention = DogDaysConvention::Hundstage;
        assert_eq!(
            convention.span(2025),
            (
                from_year_month_day(2025, 7, 23),
                from_year_month_day(2025, 8, 23)
            )
        );
        assert_eq!(convention.length(), 32);
        assert!(!convention.contains(from_year_month_day(2025, 7, 22)));
        assert!(convention.contains(from_year_month_day(2025, 8, 23)));
        assert!(!convention.contains(from_year_month_day(2025, 8, 24)));
    }

    #[test]
    fn the_conventions_have_distinct_identifiers() {
        let ids = DogDaysConvention::ALL.map(DogDaysConvention::id);
        assert_eq!(ids, ["dog-days-old-farmers-almanac", "hundstage"]);
        assert_eq!(DogDaysConvention::Hundstage.local_name(), "Hundstage");
        assert!(
            DogDaysConvention::OldFarmersAlmanac
                .english_name()
                .contains("Farmer")
        );
    }
}
