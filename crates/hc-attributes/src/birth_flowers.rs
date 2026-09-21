//! Birth flowers by month, and the honest admission that nobody standardised
//! them.
//!
//! The birthstone lists of [`crate::birthstones`] have bodies and dates
//! behind them: a trade association met, voted and issued a press release.
//! The birth flowers have nothing of the kind. They come out of Victorian
//! floriography — the "language of flowers" of the 1800s — and no
//! association ever adopted them. What circulates is a single Anglo-American
//! list with a handful of substitutions, printed by almanacs and florists.
//!
//! So both tables here carry [`Provenance::Vernacular`] and a `body` of
//! `None`, and that is not laziness: it is the difference between "Jewelers
//! of America adopted spinel in 2016" and "this is what florists print",
//! and the crate would be lying if it dressed the second up as the first.
//!
//! ```
//! use hc_attributes::birth_flowers::{BIRTH_FLOWERS_ANGLO_AMERICAN, flowers};
//! use hc_calendar::Month;
//!
//! assert_eq!(
//!     flowers(&BIRTH_FLOWERS_ANGLO_AMERICAN, Month::regular(5)),
//!     Ok(&["lily of the valley", "hawthorn"][..])
//! );
//! assert!(BIRTH_FLOWERS_ANGLO_AMERICAN.authority().body.is_none());
//! ```
//!
//! # Two flowers, not one
//!
//! Almost every month carries a primary and a secondary flower, and the
//! usual explanation — that regions differ in what actually blooms and what
//! can be bought — is also the explanation for why the two tables here
//! differ from each other. Entries keep the printed order: primary first.
//!
//! November is the exception, with one flower in both lists.
//!
//! # Japan's day-by-day 誕生花 is not here, and why
//!
//! Japan has a 誕生花 tradition covering all 366 days rather than twelve
//! months, popularised by NHK's ラジオ深夜便, which broadcasts the day's
//! flower each morning and prints an annual calendar of them. It is not
//! shipped, because at least four published Japanese lists assign
//! *different* flowers to the same day, and NHK — the most widely heard of
//! them — has never published how its assignments were made. Shipping one of
//! them, or blending them, would be manufacturing an authority that does not
//! exist. See [`crate::gaps::JAPANESE_DAILY_BIRTH_FLOWERS`].

use hc_calendar::{CalendarResult, Month};

use crate::authority::{
    Authority, MonthTable, Provenance, Region, Validity, disagreement_count, month_index, unanimous,
};

/// The list the *Old Farmer's Almanac* prints, and the commonest one in the
/// English-speaking world.
///
/// Primary flower first, secondary second. November has only one.
pub static BIRTH_FLOWERS_ANGLO_AMERICAN: MonthTable = MonthTable::new(
    Authority {
        id: "birth-flowers-anglo-american",
        english_name: "Anglo-American birth flowers",
        body: None,
        region: Region::UNITED_STATES,
        established: None,
        revised: None,
        validity: Validity::UNKNOWN,
        provenance: Provenance::Vernacular,
        source: "The Old Farmer's Almanac, \"Birth Flowers by Month\", \
                 https://www.almanac.com/content/birth-month-flowers-and-their-meanings; \
                 the underlying flower meanings are Victorian floriography, for which see \
                 the \"Language of flowers\" literature of the 1800s",
        caveat: Some(
            "No body ever adopted this list. It is what almanacs and florists print, and it \
             has no promulgation date to cite.",
        ),
    },
    [
        &["carnation", "snowdrop"],
        &["violet", "primrose"],
        &["daffodil", "jonquil"],
        &["daisy", "sweet pea"],
        &["lily of the valley", "hawthorn"],
        &["rose", "honeysuckle"],
        &["larkspur", "water lily"],
        &["gladiolus", "poppy"],
        &["aster", "morning glory"],
        &["marigold", "cosmos"],
        &["chrysanthemum"],
        &["narcissus", "holly"],
    ],
);

/// The same list as the British florist trade prints it.
///
/// Two months differ, and both differences are small and instructive.
/// February's secondary is iris rather than primrose. July's primary is
/// printed as delphinium rather than larkspur — which is not a different
/// flower at all but a different name for it, the British trade using the
/// botanical genus where the American uses the vernacular. December's two
/// are printed in the other order.
///
/// A crate that averaged the two lists would lose all three facts. This one
/// ships both and lets [`differences_from_the_anglo_american_list`] count
/// them.
pub static BIRTH_FLOWERS_BRITISH_TRADE: MonthTable = MonthTable::new(
    Authority {
        id: "birth-flowers-uk-trade",
        english_name: "British florists' birth flowers",
        body: None,
        region: Region::UNITED_KINGDOM,
        established: None,
        revised: None,
        validity: Validity::UNKNOWN,
        provenance: Provenance::Vernacular,
        source: "Bloom & Wild, \"Birth Month Flowers\", \
                 https://www.bloomandwild.com/the-blog/birth-month-flowers-guide-whats-my-birth-flower, \
                 as representative of British florists' usage",
        caveat: Some(
            "A trade usage, not a standard. \"Delphinium\" for July is the same plant the \
             American list calls larkspur.",
        ),
    },
    [
        &["carnation", "snowdrop"],
        &["violet", "iris"],
        &["daffodil", "jonquil"],
        &["daisy", "sweet pea"],
        &["lily of the valley", "hawthorn"],
        &["rose", "honeysuckle"],
        &["delphinium", "water lily"],
        &["gladiolus", "poppy"],
        &["aster", "morning glory"],
        &["marigold", "cosmos"],
        &["chrysanthemum"],
        &["holly", "narcissus"],
    ],
);

/// Every birth-flower authority this crate ships.
pub static ALL: [&MonthTable; 2] = [&BIRTH_FLOWERS_ANGLO_AMERICAN, &BIRTH_FLOWERS_BRITISH_TRADE];

/// The flowers one list names for a month.
///
/// # Errors
///
/// As [`crate::birthstones::stones`]: a leap month is
/// [`hc_calendar::CalendarError::UnsupportedField`], an ordinal outside
/// 1..=12 is [`hc_calendar::CalendarError::MonthOutOfRange`].
pub fn flowers(table: &MonthTable, month: Month) -> CalendarResult<&'static [&'static str]> {
    table.at(month_index(month)?)
}

/// Every list's answer for one month, paired with its authority.
///
/// # Errors
///
/// As [`flowers`].
pub fn all_for_month(
    month: Month,
) -> CalendarResult<impl Iterator<Item = (&'static Authority, &'static [&'static str])>> {
    let index = month_index(month)?;
    Ok(ALL
        .into_iter()
        .filter_map(move |table| Some((table.authority(), table.at(index).ok()?))))
}

/// The months in which the two lists print different text.
///
/// Three: February, July and December. July's is a naming difference rather
/// than a botanical one and December's is an ordering difference, which is
/// why the function returns the months and leaves the interpretation to the
/// caller instead of pretending to classify them.
pub fn differences_from_the_anglo_american_list() -> impl Iterator<Item = Month> {
    (0..12u8).filter_map(|index| {
        let position = usize::from(index);
        let anglo = BIRTH_FLOWERS_ANGLO_AMERICAN.at(position).ok()?;
        let british = BIRTH_FLOWERS_BRITISH_TRADE.at(position).ok()?;
        (anglo != british).then(|| Month::regular(index + 1))
    })
}

/// How many months the two lists disagree about. Three.
///
/// # Errors
///
/// Cannot occur in practice; see [`disagreement_count`].
pub fn months_in_dispute() -> CalendarResult<usize> {
    disagreement_count(&ALL)
}

/// Whether both lists name exactly the same flowers for a month.
///
/// # Errors
///
/// As [`flowers`].
pub fn lists_agree(month: Month) -> CalendarResult<bool> {
    unanimous(&ALL, month_index(month)?)
}

#[cfg(test)]
mod tests {
    use hc_calendar::CalendarError;

    use super::*;

    #[test]
    fn every_list_has_an_entry_for_all_twelve_months() {
        for table in ALL {
            assert_eq!(table.len(), 12);
            for ordinal in 1..=12u8 {
                assert!(
                    flowers(table, Month::regular(ordinal)).is_ok(),
                    "{} month {ordinal}",
                    table.authority().id
                );
            }
        }
    }

    #[test]
    fn no_list_ships_an_empty_entry() {
        for table in ALL {
            assert!(table.is_complete(), "{}", table.authority().id);
        }
    }

    #[test]
    fn the_two_lists_differ_in_february_july_and_december() {
        let expected: [u8; 3] = [2, 7, 12];
        assert!(differences_from_the_anglo_american_list().eq(expected.map(Month::regular)));
        assert_eq!(months_in_dispute(), Ok(3));
        for ordinal in [1u8, 3, 4, 5, 6, 8, 9, 10, 11] {
            assert_eq!(lists_agree(Month::regular(ordinal)), Ok(true));
        }
    }

    #[test]
    fn larkspur_and_delphinium_are_the_same_july_flower_under_two_names() {
        assert_eq!(
            flowers(&BIRTH_FLOWERS_ANGLO_AMERICAN, Month::regular(7)),
            Ok(&["larkspur", "water lily"][..])
        );
        assert_eq!(
            flowers(&BIRTH_FLOWERS_BRITISH_TRADE, Month::regular(7)),
            Ok(&["delphinium", "water lily"][..])
        );
        // Neither list names the other's word for it, which is why a naive
        // string comparison calls July a disagreement.
        assert!(!BIRTH_FLOWERS_ANGLO_AMERICAN.names("delphinium"));
        assert!(!BIRTH_FLOWERS_BRITISH_TRADE.names("larkspur"));
    }

    #[test]
    fn december_differs_only_in_the_order_the_two_trades_print() {
        let anglo = flowers(&BIRTH_FLOWERS_ANGLO_AMERICAN, Month::regular(12)).unwrap();
        let british = flowers(&BIRTH_FLOWERS_BRITISH_TRADE, Month::regular(12)).unwrap();
        assert_ne!(anglo, british);
        for flower in anglo {
            assert!(british.contains(flower), "{flower}");
        }
        assert_eq!(anglo.len(), british.len());
    }

    #[test]
    fn november_is_the_only_month_with_a_single_flower() {
        for table in ALL {
            for ordinal in 1..=12u8 {
                let entry = flowers(table, Month::regular(ordinal)).unwrap();
                let expected = if ordinal == 11 { 1 } else { 2 };
                assert_eq!(entry.len(), expected, "month {ordinal}");
            }
        }
    }

    /// The structural point of the module: these lists have no promulgating
    /// body, and the type says so rather than a comment.
    #[test]
    fn neither_list_claims_a_promulgating_body_or_a_date() {
        for table in ALL {
            let authority = table.authority();
            assert!(authority.body.is_none(), "{}", authority.id);
            assert!(authority.established.is_none(), "{}", authority.id);
            assert!(authority.revised.is_none(), "{}", authority.id);
            assert_eq!(authority.provenance, Provenance::Vernacular);
            assert!(authority.caveat.is_some(), "{}", authority.id);
            assert_eq!(authority.validity, Validity::UNKNOWN);
        }
    }

    #[test]
    fn a_leap_month_has_no_birth_flower() {
        for table in ALL {
            assert_eq!(
                flowers(table, Month::leap(6)),
                Err(CalendarError::UnsupportedField("leap month"))
            );
        }
    }

    #[test]
    fn a_month_outside_the_year_is_out_of_range() {
        assert_eq!(
            flowers(&BIRTH_FLOWERS_ANGLO_AMERICAN, Month::regular(13)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            lists_agree(Month::regular(0)),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn asking_both_lists_at_once_returns_one_row_each() {
        assert_eq!(all_for_month(Month::regular(2)).unwrap().count(), 2);
        let names: [&str; 2] = ["birth-flowers-anglo-american", "birth-flowers-uk-trade"];
        assert!(
            all_for_month(Month::regular(2))
                .unwrap()
                .map(|(authority, _)| authority.id)
                .eq(names)
        );
    }

    #[test]
    fn the_flower_names_are_lowercase_like_the_stone_names() {
        for table in ALL {
            for entry in table.iter() {
                for name in entry {
                    assert!(!name.chars().any(char::is_uppercase), "{name}");
                }
            }
        }
    }
}

/// The table whose authority has this identifier.
#[must_use]
pub fn by_id(id: &str) -> Option<&'static MonthTable> {
    ALL.iter().copied().find(|table| table.authority().id == id)
}

hc_core::catalogue_tests! {
    type: &'static MonthTable,
    id: |table| table.authority().id,
    provenance: |table| table.authority().source,
    tests: birth_flower_table_tests,
    all: &ALL,
    lookup: by_id,
}
