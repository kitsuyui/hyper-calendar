//! Name-day lists: each list a named edition of a named authority.
//!
//! **This crate reports what lists say. It asserts none of them.** A name
//! day is a fact about somebody's list, and the list is always named — the
//! Latvian *vārda dienas* are the Valsts valodas centrs's, in the edition
//! its commission decided in 2022 and the one it decided in 2025, and the
//! crate carries both because a caller asking about 2024 should get the
//! list that was in force in 2024.
//!
//! ```
//! use hc_name_days::latvia::{LV_TRADITIONAL_2023, LV_TRADITIONAL_2026};
//! use hc_name_days::{MonthDay, NameDayError, Validity, days_of, in_force, names_on};
//!
//! // 1 January in Latvia is Laimnesis, Solvita and Solvija.
//! assert_eq!(
//!     names_on(&LV_TRADITIONAL_2026, 2026, 1, 1)?,
//!     ["Laimnesis", "Solvita", "Solvija"]
//! );
//! // Grēta was added by the decision of 30 April 2025, in force from 2026.
//! assert!(names_on(&LV_TRADITIONAL_2026, 2026, 1, 23)?.contains(&"Grēta"));
//! assert!(!names_on(&LV_TRADITIONAL_2023, 2025, 1, 23)?.contains(&"Grēta"));
//! // An edition does not answer for a year it does not cover.
//! assert_eq!(
//!     names_on(&LV_TRADITIONAL_2026, 2025, 1, 23),
//!     Err(NameDayError::OutsideValidity { year: 2025, validity: Validity::since(2026) })
//! );
//! // The year picks the editions; both Latvian lists are in force at once.
//! assert_eq!(in_force("lv", 2024).count(), 2);
//! // And a name finds its days.
//! assert!(days_of(&LV_TRADITIONAL_2026, "Jānis", 2026)?.eq([MonthDay::new(6, 24)]));
//! # Ok::<(), NameDayError>(())
//! ```
//!
//! # Layout
//!
//! | Module | Subject |
//! |---|---|
//! | [`list`] | the shape of a list, the leap-day rules, the licence, and the two functions that read a list |
//! | [`latvia`] | the traditional and extended lists of the Valsts valodas centrs, two editions each |
//! | [`load`] | a text format for lists a caller supplies, for the lists this crate may not ship |
//! | [`gaps`] | seventeen countries the crate declines to ship, each with its reason |
//!
//! # Licensing shapes what is vendored
//!
//! Only a list whose terms permit redistribution is in this crate, and a
//! test asserts it. Latvia publishes its lists as open data under
//! CC0-1.0. Finland's four lists are the University of Helsinki's and
//! licensed for a fee; Norway's is Almanakkforlaget's; Sweden's has no
//! stated terms. Those are read at run time from a text the caller
//! supplies, through the same [`NameDays`] trait the vendored tables
//! implement, so that holding a licence and using the list are the
//! caller's business and not this crate's.
//!
//! # What this crate refuses to do
//!
//! - **Extrapolate an edition** (ADR 0006). The 2026 Latvian edition does
//!   not answer for 2025, and the 2023 edition does not answer for 2026.
//! - **Pick a list.** [`in_force`] returns every list in force, since a
//!   country may keep more than one; there is no `current()` returning one.
//! - **Invent an authority.** Where publishers disagree and no body
//!   chooses, the country is a [`gaps::Gap`].
//! - **Translate or transliterate.** Names are the list's own spelling in
//!   the list's own script, and a lookup is exact. There is no `hc-i18n`
//!   dependency.
//! - **Carry the movable Orthodox name days yet.** They are rules on the
//!   Julian computus (Thomas Sunday, All Saints, St George moved after
//!   Pascha) and belong beside `hc_holiday::computus`; [`gaps::GREECE`]
//!   and [`gaps::BULGARIA`] record the rules until they arrive.
//!
//! # Features
//!
//! The tables and the evaluator are `&'static` data and integer
//! arithmetic and need neither `std` nor `alloc`. The loader in [`load`]
//! needs an allocator and is behind `alloc`. A `no_std` build enables
//! `libm`, which passes through to `hc-core`:
//! `--no-default-features --features alloc,libm`.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod gaps;
pub mod latvia;
pub mod list;
#[cfg(feature = "alloc")]
pub mod load;

pub use gaps::{Gap, GapReason};
pub use list::{
    AttributionDate, DAYS, LeapDayRule, Licence, MonthDay, NameDayError, NameDayList, NameDays,
    Note, Provenance, Validity, days_of, names_on,
};
#[cfg(feature = "alloc")]
pub use load::OwnedNameDayList;

pub use hc_calendar;

/// Every list the crate ships, across all countries, by identifier.
///
/// Hand-assembled from the per-country tables because a `const` slice
/// cannot be concatenated; a test asserts that it holds exactly the union
/// of them.
pub static ALL: [NameDayList; 4] = [
    latvia::LV_EXTENDED_2023,
    latvia::LV_EXTENDED_2026,
    latvia::LV_TRADITIONAL_2023,
    latvia::LV_TRADITIONAL_2026,
];

/// The list with this identifier, from any country.
#[must_use]
pub fn by_id(id: &str) -> Option<&'static NameDayList> {
    ALL.iter().find(|list| list.id == id)
}

/// Every list of a country that is in force in a year.
///
/// Several at once is the normal case: Latvia keeps a traditional and an
/// extended list side by side. None means the crate carries no edition for
/// that country and year — not that the country has no name days.
pub fn in_force(country: &str, year: i32) -> impl Iterator<Item = &'static NameDayList> {
    let code: Option<[u8; 2]> = match country.as_bytes() {
        [first, second] => Some([first.to_ascii_lowercase(), second.to_ascii_lowercase()]),
        _ => None,
    };
    ALL.iter().filter(move |list| {
        code.is_some_and(|code| list.country.as_bytes() == code) && list.validity.contains(year)
    })
}

hc_core::catalogue_tests! {
    type: NameDayList,
    id: |list| list.id,
    sorted_by: |list| list.id,
    provenance: |list| list.source,
    tests: all_lists_tests,
    all: &ALL,
    lookup: by_id,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_crate_table_is_exactly_the_union_of_the_country_tables() {
        assert_eq!(ALL.len(), latvia::ALL.len());
        for list in latvia::ALL {
            assert_eq!(by_id(list.id), Some(list));
        }
    }

    /// The crate's one global invariant: nothing is shipped without terms
    /// that allow shipping it.
    #[test]
    fn every_shipped_list_may_be_redistributed() {
        for list in &ALL {
            assert!(list.licence.permits_redistribution(), "{}", list.id);
            assert!(list.source.len() > 40, "{} has no real source", list.id);
            assert!(!list.authority.is_empty(), "{}", list.id);
            assert_eq!(list.country.len(), 2, "{}", list.id);
            assert!(
                list.country.bytes().all(|b| b.is_ascii_lowercase()),
                "{}",
                list.id
            );
        }
    }

    /// Every shipped list's empty slots are exactly the ones its leap-day
    /// rule accounts for, so a hole is a statement and never an accident.
    #[test]
    fn every_empty_slot_is_the_one_the_leap_day_rule_names() {
        for list in &ALL {
            let expected = list.leap_day.empty_slot();
            for (index, slot) in list.days.iter().enumerate() {
                assert_eq!(
                    slot.is_empty(),
                    Some(index) == expected,
                    "{} slot {index}",
                    list.id
                );
            }
        }
    }

    #[test]
    fn a_superseded_edition_has_a_closed_span_and_a_current_one_is_open() {
        let mut current = 0;
        for list in &ALL {
            if list.validity.is_current() {
                current += 1;
            }
        }
        assert_eq!(
            current, 2,
            "one traditional and one extended list are current"
        );
    }

    #[test]
    fn the_year_picks_the_editions_and_the_country_code_is_case_insensitive() {
        assert!(
            in_force("lv", 2023)
                .map(|l| l.id)
                .eq(["lv-extended-2023", "lv-traditional-2023"])
        );
        assert!(
            in_force("LV", 2025)
                .map(|l| l.id)
                .eq(["lv-extended-2023", "lv-traditional-2023"])
        );
        assert!(
            in_force("lv", 2026)
                .map(|l| l.id)
                .eq(["lv-extended-2026", "lv-traditional-2026"])
        );
        assert_eq!(
            in_force("lv", 2022).count(),
            0,
            "before the oldest edition carried"
        );
        assert_eq!(in_force("fi", 2026).count(), 0, "a gap, not a list");
        assert_eq!(in_force("", 2026).count(), 0);
        assert_eq!(in_force("latvia", 2026).count(), 0);
    }
}
