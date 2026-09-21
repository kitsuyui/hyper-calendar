//! Cultural attributions to calendar units — birthstones, birth flowers,
//! moon names, traditional month names, weekday associations.
//!
//! **This crate reports what traditions claim. It asserts none of them.**
//! Nothing here is a fact about a month; everything here is a fact about
//! somebody's list, and the list is always named.
//!
//! # There is no such thing as "the" birthstone for a month
//!
//! There is the American list of 1912, revised in 1952, 2002 and 2016. There
//! is the British list of 1937. There is Japan's list of 1958, substantially
//! revised on 20 December 2021 — ten stones added after sixty-three years,
//! the most recent such change anywhere. There are the older European stones
//! that all three replaced. They disagree in eleven months out of twelve.
//!
//! So this crate has no `birthstone(month)` function, and will not grow one.
//! Every lookup names its authority:
//!
//! ```
//! use hc_attributes::birthstones::{BIRTHSTONES_JP_2021, BIRTHSTONES_US_2016, all_for_month, stones};
//! use hc_calendar::Month;
//!
//! let december = Month::regular(12);
//! assert_eq!(
//!     stones(&BIRTHSTONES_US_2016, december),
//!     Ok(&["turquoise", "zircon", "tanzanite"][..])
//! );
//! assert_eq!(
//!     stones(&BIRTHSTONES_JP_2021, december),
//!     Ok(&["turquoise", "lapis lazuli", "zircon", "tanzanite"][..])
//! );
//!
//! // Or ask everybody at once, which is usually the honest answer.
//! for (authority, entry) in all_for_month(december)? {
//!     assert!(!entry.is_empty(), "{}", authority.english_name);
//! }
//! # Ok::<(), hc_calendar::CalendarError>(())
//! ```
//!
//! This is `docs/policy.md` §5 — *competing conventions get names, not
//! parameters* — applied to the case that shows it most plainly. A parameter
//! can be forgotten, and the caller who does not know the question exists
//! gets a silent default. A name cannot be selected by accident, appears in
//! a listing, is self-documenting at the call site, and asking for all of
//! them and comparing is one loop.
//!
//! # Layout
//!
//! | Module | Subject | Keyed by |
//! |---|---|---|
//! | [`authority`] | the shared shape: who says so, where, when | — |
//! | [`birthstones`] | six month-by-month stone lists | [`hc_calendar::Month`] |
//! | [`zodiac_stones`] | the older sign-by-sign system | [`hc_seasons::TropicalSign`] |
//! | [`birth_flowers`] | two month flower lists, neither standardised | [`hc_calendar::Month`] |
//! | [`moon_names`] | full-moon names, plus the Harvest Moon *rule* | month, and a computation |
//! | [`month_names`] | Old English, Frankish, Finnish, Czech month names | [`hc_calendar::Month`] |
//! | [`weekday_attributions`] | planets, deities, colours, stones of the week | [`hc_calendar::Weekday`] |
//! | [`gaps`] | what the crate declined to ship, and why | — |
//!
//! Every table is an [`authority::AttributionTable`], carrying an
//! [`authority::Authority`] with a source, a region, a provenance, an
//! adoption or revision date and a validity span. One evaluator —
//! [`authority::AttributionTable::at`] — reads all of them, the same way
//! `hc_almanac::rules::rule_applies` evaluates thirty-six 暦注 from one
//! function. Data is data; the algorithm is one line long.
//!
//! # What this crate refuses to do
//!
//! - **Pick a default.** No function returns "the" anything.
//! - **Average two lists.** Where publishers disagree within one country,
//!   both are shipped, or the disagreement is recorded in [`gaps`].
//! - **Invent an authority.** Japan's day-by-day 誕生花 is a real tradition
//!   with four mutually inconsistent published lists and no published
//!   method, so it is a [`gaps::Gap`], not a table.
//! - **Repeat a bad attribution.** The full-moon names are published as
//!   Native American; the record shows a chain of publications from 1778
//!   onward, no named nation, and two names of documented English origin.
//!   [`moon_names`] says so in the authority's caveat, and a test forbids a
//!   nation's name appearing in any of those tables.
//! - **Launder a modern invention.** The "Celtic tree calendar" is Robert
//!   Graves's, from 1948, and is recorded as such in
//!   [`gaps::CELTIC_TREE_CALENDAR`] rather than shipped.
//! - **Duplicate a neighbour.** Japan's 和風月名 are `hc-i18n`'s and stay
//!   there; the 七曜 as an almanac annotation are `hc-almanac`'s; 中秋の名月
//!   is `hc-seasons`'. See [`month_names`] and
//!   [`weekday_attributions`].
//!
//! # Where the astronomy comes in
//!
//! Only once, and it is the Harvest Moon. It is defined as the full moon
//! nearest the September equinox, so about one year in four it falls in
//! October and September's moon takes its other name. That is a rule, not a
//! table row, and [`moon_names::harvest_moon`] computes it from
//! `hc-seasons`' equinox instant and lunar phases. Everything else in the
//! crate is a static lookup with no computation at all.
//!
//! # Precision
//!
//! The tables are exact: they are transcriptions, and the tests check them
//! against the published lists. The only thing that can be wrong by a day is
//! the Harvest Moon, which inherits `hc-seasons`' accuracy — lunar phases
//! within about a minute, the equinox with a systematic bias of about −4.5
//! minutes. Since the rule compares intervals of roughly a fortnight,
//! neither error can change which month the answer falls in for any year
//! this crate is tested over.
//!
//! # Features
//!
//! Every table is `&'static` data and every lookup is an array index, so
//! the tables need neither `std` nor `alloc`. The `std` and `alloc` features
//! exist only to propagate to the `hc-*` crates below, and the crate builds
//! under `--no-default-features --features alloc`.
//!
//! The one part that needs more is the Harvest Moon, because it reaches the
//! astronomy. `hc_core::math` panics without a floating-point backend, so a
//! `no_std` caller that wants [`moon_names::harvest_moon`],
//! [`moon_names::september_moon_name`] or [`zodiac_stones::stones_on`] must
//! also enable `hc-core/libm`. Everything else — all six birthstone lists,
//! both flower lists, every month-name set, every weekday table and the
//! moon-name tables themselves — works with `alloc` alone, because none of
//! it computes anything.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod authority;
pub mod birth_flowers;
pub mod birthstones;
pub mod gaps;
pub mod month_names;
pub mod moon_names;
pub mod weekday_attributions;
pub mod zodiac_stones;

mod gregorian;

pub use authority::{
    AttributionDate, AttributionTable, Authority, MonthTable, Provenance, Region, SignTable,
    Validity, WeekdayTable,
};
pub use gaps::{Gap, GapReason};
pub use month_names::MonthNameSet;

pub use hc_calendar;
pub use hc_calendar::{Month, Weekday};
pub use hc_seasons;
pub use hc_seasons::{Meridian, TropicalSign};

/// Every attribution authority the crate ships, across all subjects.
///
/// The count is checked by a test rather than written down twice.
#[must_use]
pub fn authority_count() -> usize {
    birthstones::ALL.len()
        + birth_flowers::ALL.len()
        + moon_names::ALL.len()
        + 1 // moon_names::MOON_NAMES_CARVER_1778, which is not month-keyed
        + zodiac_stones::ALL.len()
        + month_names::ALL.len()
        + weekday_attributions::ALL.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The crate's one global invariant: nothing is attributed without an
    /// authority, and no authority is without a source. If this fails, the
    /// crate has started repeating folklore, which is the thing it exists
    /// to disentangle.
    #[test]
    fn every_shipped_table_names_an_authority_with_a_source() {
        let mut checked = 0;
        for authority in every_authority() {
            assert!(!authority.id.is_empty());
            assert!(!authority.english_name.is_empty(), "{}", authority.id);
            assert!(
                authority.source.len() > 10,
                "{} has no real source",
                authority.id
            );
            assert!(!authority.region.english_name.is_empty());
            assert!(!authority.provenance.english_description().is_empty());
            checked += 1;
        }
        assert_eq!(checked, authority_count());
        assert!(checked >= 20, "only {checked} authorities");
    }

    /// Identifiers must be unique across the whole crate, not merely within
    /// a module, because [`gaps`] and a caller's own index both key on them.
    ///
    /// The quadratic comparison is over twenty-five items and keeps the test
    /// free of `alloc`, which the crate otherwise does not need.
    #[test]
    fn every_authority_identifier_is_unique_across_the_whole_crate() {
        let total = authority_count();
        for position in 0..total {
            let Some(id) = every_authority().nth(position).map(|a| a.id) else {
                panic!("authority_count() over-counts at {position}");
            };
            for other in every_authority().skip(position + 1) {
                assert_ne!(id, other.id, "duplicate authority id {id}");
            }
        }
        assert!(every_authority().nth(total).is_none());
    }

    /// A contested list must carry its caveat where a caller will meet it.
    #[test]
    fn every_contested_authority_carries_a_caveat() {
        for authority in every_authority() {
            if authority.provenance.warrants_a_caveat() {
                assert!(
                    authority.caveat.is_some(),
                    "{} is contested and says nothing about it",
                    authority.id
                );
            }
        }
    }

    #[test]
    fn a_superseded_authority_always_explains_that_it_is_superseded() {
        for authority in every_authority() {
            if !authority.validity.is_current() {
                assert!(
                    authority.caveat.is_some(),
                    "{} is closed-ended and says nothing about it",
                    authority.id
                );
            }
        }
    }

    #[test]
    fn the_crate_ships_at_least_twenty_named_authorities_and_six_gaps() {
        assert!(authority_count() >= 20);
        assert_eq!(gaps::ALL.len(), 6);
    }

    /// Every table is complete — no month, sign or weekday is left blank in
    /// any list the crate ships. An authority with nothing to say about a
    /// key is a gap, not a hole.
    #[test]
    fn no_shipped_table_anywhere_has_an_empty_entry() {
        for table in birthstones::ALL {
            assert!(table.is_complete(), "{}", table.authority().id);
        }
        for table in birth_flowers::ALL {
            assert!(table.is_complete(), "{}", table.authority().id);
        }
        for table in moon_names::ALL {
            assert!(table.is_complete(), "{}", table.authority().id);
        }
        assert!(moon_names::MOON_NAMES_CARVER_1778.is_complete());
        for table in zodiac_stones::ALL {
            assert!(table.is_complete(), "{}", table.authority().id);
        }
        for set in month_names::ALL {
            assert!(set.table().is_complete(), "{}", set.authority().id);
        }
        for table in weekday_attributions::ALL {
            assert!(table.is_complete(), "{}", table.authority().id);
        }
    }

    /// Three systems assign stones to three different things, and the crate
    /// keeps them apart: months, zodiac signs and weekdays. Conflating them
    /// is the commonest error about birthstones, so a test states the
    /// separation.
    #[test]
    fn the_three_stone_systems_are_three_and_not_one() {
        let january = birthstones::stones(&birthstones::BIRTHSTONES_US_2016, Month::regular(1))
            .unwrap_or(&[]);
        let capricorn = zodiac_stones::stones_for_sign(TropicalSign::CAPRICORN);
        let sunday = weekday_attributions::attribution(
            &weekday_attributions::WEEKDAY_STONES_KUNZ,
            Weekday::Sunday,
        );
        assert_eq!(january, &["garnet"]);
        assert_eq!(capricorn, &["ruby"]);
        assert_eq!(sunday, &["topaz", "diamond"]);
        assert_ne!(january, capricorn);
        assert_ne!(january, sunday);
        assert_ne!(capricorn, sunday);
    }

    /// Every authority in the crate, as one iterator, for the invariants
    /// above. Deliberately not public: a caller wants the tables, not a
    /// heterogeneous parade of authorities.
    fn every_authority() -> impl Iterator<Item = &'static Authority> {
        birthstones::ALL
            .into_iter()
            .map(MonthTable::authority)
            .chain(birth_flowers::ALL.into_iter().map(MonthTable::authority))
            .chain(moon_names::ALL.into_iter().map(MonthTable::authority))
            .chain(core::iter::once(
                moon_names::MOON_NAMES_CARVER_1778.authority(),
            ))
            .chain(zodiac_stones::ALL.into_iter().map(SignTable::authority))
            .chain(month_names::ALL.into_iter().map(MonthNameSet::authority))
            .chain(
                weekday_attributions::ALL
                    .into_iter()
                    .map(WeekdayTable::authority),
            )
    }
}
