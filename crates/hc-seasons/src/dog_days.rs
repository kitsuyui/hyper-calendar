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
//! | [`DogDaysConvention::PrayerBook1552`] | 7 July to 5 September in the Julian calendar, the kalendar of the English Book of Common Prayer of 1552 and 1559 |
//!
//! The first two are Gregorian dates. The Prayer Book's are Julian,
//! because England kept the Julian calendar until 1752, and
//! [`DogDaysConvention::calendar`] says which calendar a convention's
//! dates are in.
//!
//! The 1552 kalendar prints "Dog daies" against the Nones of July, the
//! 7th, and "Dog daies end" against the Nones of September, the 5th; the
//! 1559 kalendar prints the same two entries against the same two days.
//! The kalendar of 1662 marks no dog days: neither of the two 1662
//! printings read has an entry for them in July, August or September.
//! So the later spans the English Wikipedia gives, 19 July to 20 August
//! "after 1660" and 30 July to 7 September after 1752, citing Townsend's
//! *Manual of Dates* (1862, not read), are not the Prayer Book's, and they
//! are not carried.
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
//!   "Hundstage" (`wikipedia-de-hundstage`), retrieved 2026-09-26, gives
//!   the same span and cites it.
//! * `bcp1552-eebo`: *The Booke of Common Prayer, and Administration of the
//!   Sacramentes* (London, 1552), the kalendar pages for July and
//!   September, read in the Early English Books microfilm scan on the
//!   Internet Archive, retrieved 2026-09-27; the Everyman's Library
//!   reprint, *The First and Second Prayer-Books of King Edward the Sixth*
//!   (`everyman-bcp-1549-1552`), prints the same two entries.
//! * `bcp1559-eebo`: *The Booke of Common Prayer, and Administration of the
//!   Sacramentes* (London, 1559), the kalendar pages for July, August and
//!   September, read in the Early English Books microfilm scan on the
//!   Internet Archive, retrieved 2026-09-27.
//! * `bcp1662-eebo`: *The Book of Common Prayer* (London, 1662), the kalendar
//!   pages for July and August, read in two Early English Books microfilm
//!   scans of different printings on the Internet Archive, retrieved
//!   2026-09-27, for the absence of any entry.
//! * `wikipedia-dog-days`: Wikipedia, "Dog days", retrieved 2026-09-26,
//!   for the later spans not carried.

use hc_calendar::Rd;
use hc_calendars_solar::julian;

use crate::gregorian;

/// The calendar a convention's month and day are dates in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpanCalendar {
    /// The Gregorian calendar.
    Gregorian,
    /// The Julian calendar.
    Julian,
}

/// Whose dog days these are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DogDaysConvention {
    /// *The Old Farmer's Almanac*: 3 July to 11 August.
    OldFarmersAlmanac,
    /// The German-language *Hundstage*: 23 July to 23 August.
    Hundstage,
    /// The kalendar of the Book of Common Prayer of 1552 and 1559: 7 July
    /// to 5 September in the Julian calendar.
    PrayerBook1552,
}

impl DogDaysConvention {
    /// Every convention, in the order of the module table.
    pub const ALL: [Self; 3] = [
        Self::OldFarmersAlmanac,
        Self::Hundstage,
        Self::PrayerBook1552,
    ];

    /// A short identifier, in kebab case.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::OldFarmersAlmanac => "dog-days-old-farmers-almanac",
            Self::Hundstage => "hundstage",
            Self::PrayerBook1552 => "dog-days-prayer-book-1552",
        }
    }

    /// The name in English.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::OldFarmersAlmanac => "Dog Days (The Old Farmer's Almanac)",
            Self::Hundstage => "Dog days (Hundstage)",
            Self::PrayerBook1552 => "Dog days (Book of Common Prayer, 1552)",
        }
    }

    /// The name in the convention's own language.
    #[must_use]
    pub const fn local_name(self) -> &'static str {
        match self {
            Self::OldFarmersAlmanac => "Dog Days",
            Self::Hundstage => "Hundstage",
            Self::PrayerBook1552 => "Dog daies",
        }
    }

    /// The calendar [`month_days`](Self::month_days) and
    /// [`span`](Self::span) count in.
    #[must_use]
    pub const fn calendar(self) -> SpanCalendar {
        match self {
            Self::OldFarmersAlmanac | Self::Hundstage => SpanCalendar::Gregorian,
            Self::PrayerBook1552 => SpanCalendar::Julian,
        }
    }

    /// The first and last month and day, both included, in the
    /// convention's [`calendar`](Self::calendar).
    #[must_use]
    pub const fn month_days(self) -> ((u8, u8), (u8, u8)) {
        match self {
            Self::OldFarmersAlmanac => ((7, 3), (8, 11)),
            Self::Hundstage => ((7, 23), (8, 23)),
            Self::PrayerBook1552 => ((7, 7), (9, 5)),
        }
    }

    /// The first and last day of the dog days in `year` of the
    /// convention's [`calendar`](Self::calendar), both included.
    #[must_use]
    pub const fn span(self, year: i64) -> (Rd, Rd) {
        let ((first_month, first_day), (last_month, last_day)) = self.month_days();
        (
            self.date(year, first_month, first_day),
            self.date(year, last_month, last_day),
        )
    }

    /// The fixed day of a date in the convention's calendar.
    ///
    /// The Julian arm saturates as the Gregorian adapter does: every date
    /// passed here is a constant from the table above, which exists in
    /// every year, and the two calendars cover the same years.
    const fn date(self, year: i64, month: u8, day: u8) -> Rd {
        match self.calendar() {
            SpanCalendar::Gregorian => gregorian::from_year_month_day(year, month, day),
            SpanCalendar::Julian => match julian::to_fixed(year, month, day) {
                Ok(rd) => rd,
                Err(_) => gregorian::new_year(year),
            },
        }
    }

    /// The year of the convention's calendar that `day` falls in.
    const fn year_of(self, day: Rd) -> i64 {
        match self.calendar() {
            SpanCalendar::Gregorian => gregorian::year_from_rd(day),
            SpanCalendar::Julian => match julian::from_fixed(day) {
                Ok((year, _, _)) => year,
                Err(_) => gregorian::year_from_rd(day),
            },
        }
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
        let (first, last) = self.span(self.year_of(day));
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

    /// The 1552 kalendar prints "Dog daies" on the Nones of July and "Dog
    /// daies end" on the Nones of September, and the 1559 kalendar the
    /// same, both in the Julian calendar England then kept: Julian 7 July
    /// 1559 was Gregorian 17 July.
    #[test]
    fn the_prayer_books_dog_days_run_from_the_nones_of_july_to_the_nones_of_september() {
        let convention = DogDaysConvention::PrayerBook1552;
        assert_eq!(convention.calendar(), SpanCalendar::Julian);
        assert_eq!(convention.length(), 61);
        let (first, last) = convention.span(1559);
        assert_eq!(julian::from_fixed(first), Ok((1559, 7, 7)));
        assert_eq!(julian::from_fixed(last), Ok((1559, 9, 5)));
        assert_eq!(first, from_year_month_day(1559, 7, 17));
        assert_eq!(last, from_year_month_day(1559, 9, 15));
        assert!(!convention.contains(first - 1));
        assert!(convention.contains(first));
        assert!(convention.contains(last));
        assert!(!convention.contains(last + 1));
        // Gregorian 7 July is Julian 27 June, before the span.
        assert!(!convention.contains(from_year_month_day(1559, 7, 7)));
    }

    #[test]
    fn the_conventions_have_distinct_identifiers() {
        let ids = DogDaysConvention::ALL.map(DogDaysConvention::id);
        assert_eq!(
            ids,
            [
                "dog-days-old-farmers-almanac",
                "hundstage",
                "dog-days-prayer-book-1552"
            ]
        );
        assert_eq!(DogDaysConvention::Hundstage.local_name(), "Hundstage");
        assert!(
            DogDaysConvention::OldFarmersAlmanac
                .english_name()
                .contains("Farmer")
        );
    }
}
