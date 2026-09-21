//! A table of named things, with the parts nobody should write twice.
//!
//! # What this is for
//!
//! This library is largely catalogues: units of time, day counts, year-start
//! styles, calendar systems, astronomical periods. Every one of them has the
//! same shape — a set of `const` entries, a slice of all of them, a lookup by
//! identifier — and every one of them wants the same things asserted:
//! identifiers are unique, every entry is findable by its own identifier,
//! the table is in the order it claims, and nothing is missing its source.
//!
//! Those were written by hand each time, and the evidence that this was a bad
//! idea is in the repository's own history. Two hand-written property tests
//! were *wrong*: one asserted a span was "well past the range" of an
//! attosecond count when it was five million times inside it, and another
//! compared a function against itself through a re-export, so no change
//! could ever have failed it. Both would have been correct if generated.
//!
//! The larger win is not the tests. It is that declaring an entry, listing
//! it, and counting it stop being three separate edits. That class of
//! mistake — implemented but not registered — happened three times in a
//! single afternoon of work on this crate.
//!
//! # What it deliberately does not do
//!
//! It does not generate the entries, and it does not generate anything that
//! needs to know what a calendar *is*. Doc comments, values and provenance
//! stay written out where a reader finds them; the macro re-emits the
//! attributes untouched.
//!
//! And it cannot check that a value is *right*. Every transcription error
//! this library has caught was caught by stating the same fact two
//! independent ways — an epoch date against a classical offset, an epact
//! against a computus that shares no code with it. That second statement is
//! domain knowledge and has to be written. What the macro removes is the
//! ceremony around it, so that the anchors are what is left.

/// Declare a catalogue: its entries, the slice of all of them, its lookup,
/// and the properties every such table wants.
///
/// # Shape
///
/// ```
/// # use hc_core::catalogue;
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// pub struct Spice {
///     pub id: &'static str,
///     pub heat: u8,
///     pub source: &'static str,
/// }
///
/// catalogue! {
///     type: Spice,
///     id: |spice| spice.id,
///     sorted_by: |spice| spice.heat,
///     provenance: |spice| spice.source,
///     tests: spice_catalogue_tests,
///
///     /// Every spice, mildest first.
///     pub const ALL;
///     /// The spice with this identifier.
///     pub fn by_id;
///
///     entries: {
///         /// Not actually hot.
///         pub const PAPRIKA = Spice { id: "paprika", heat: 1, source: "the shelf" };
///         /// Hotter.
///         pub const CAYENNE = Spice { id: "cayenne", heat: 9, source: "the shelf" };
///     }
/// }
/// # assert_eq!(ALL.len(), 2);
/// # assert_eq!(by_id("cayenne"), Some(CAYENNE));
/// ```
///
/// `sorted_by` and `provenance` are optional. Give `sorted_by` when the
/// table's order is a claim the documentation makes, and `provenance` when
/// every entry names a source — both then become assertions instead of
/// intentions.
///
/// The generated tests are named after the module you pass as `tests:`, so
/// a module may hold more than one catalogue.
///
/// # Entries as associated constants
///
/// When the entries should hang off the type — `Spice::PAPRIKA` rather than
/// `PAPRIKA` — put `associated;` after `tests:`. The entries, the table and
/// the lookup are then emitted inside an `impl` block, `$value` may say
/// `Self`, and the generated tests are the same.
///
/// ```
/// # use hc_core::catalogue;
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// pub struct Spice {
///     pub id: &'static str,
///     pub heat: u8,
/// }
///
/// catalogue! {
///     type: Spice,
///     id: |spice| spice.id,
///     tests: spice_catalogue_tests,
///     associated;
///
///     /// Every spice.
///     pub const ALL;
///     /// The spice with this identifier.
///     pub fn by_id;
///
///     entries: {
///         /// Not actually hot.
///         pub const PAPRIKA = Self { id: "paprika", heat: 1 };
///     }
/// }
/// # assert_eq!(Spice::ALL.len(), 1);
/// # assert_eq!(Spice::by_id("paprika"), Some(Spice::PAPRIKA));
/// ```
#[macro_export]
macro_rules! catalogue {
    (
        type: $ty:ty,
        id: |$id_arg:ident| $id_expr:expr,
        $(sorted_by: |$sort_arg:ident| $sort_expr:expr,)?
        $(provenance: |$prov_arg:ident| $prov_expr:expr,)?
        tests: $tests:ident,

        $(#[$all_meta:meta])*
        $all_vis:vis const $all:ident;
        $(#[$lookup_meta:meta])*
        $lookup_vis:vis fn $lookup:ident;

        entries: {
            $(
                $(#[$entry_meta:meta])*
                $entry_vis:vis const $entry:ident = $value:expr;
            )+
        }
    ) => {
        $(
            $(#[$entry_meta])*
            $entry_vis const $entry: $ty = $value;
        )+

        $(#[$all_meta])*
        $all_vis const $all: &[$ty] = &[$($entry),+];

        $(#[$lookup_meta])*
        ///
        /// Generated by [`hc_core::catalogue!`], together with the table it
        /// searches, so the two cannot disagree about what exists.
        #[must_use]
        $lookup_vis fn $lookup(id: &str) -> Option<$ty> {
            $all.iter().copied().find(|entry| {
                let $id_arg = entry;
                $id_expr == id
            })
        }

        $crate::__catalogue_tests! {
            type: $ty,
            id: |$id_arg| $id_expr,
            $(sorted_by: |$sort_arg| $sort_expr,)?
            $(provenance: |$prov_arg| $prov_expr,)?
            tests: $tests,
            all: $all,
            lookup: $lookup,
        }
    };

    (
        type: $ty:ty,
        id: |$id_arg:ident| $id_expr:expr,
        $(sorted_by: |$sort_arg:ident| $sort_expr:expr,)?
        $(provenance: |$prov_arg:ident| $prov_expr:expr,)?
        tests: $tests:ident,
        associated;

        $(#[$all_meta:meta])*
        $all_vis:vis const $all:ident;
        $(#[$lookup_meta:meta])*
        $lookup_vis:vis fn $lookup:ident;

        entries: {
            $(
                $(#[$entry_meta:meta])*
                $entry_vis:vis const $entry:ident = $value:expr;
            )+
        }
    ) => {
        impl $ty {
            $(
                $(#[$entry_meta])*
                $entry_vis const $entry: $ty = $value;
            )+

            $(#[$all_meta])*
            $all_vis const $all: &'static [$ty] = &[$(Self::$entry),+];

            $(#[$lookup_meta])*
            ///
            /// Generated by [`hc_core::catalogue!`], together with the table
            /// it searches, so the two cannot disagree about what exists.
            #[must_use]
            $lookup_vis fn $lookup(id: &str) -> Option<$ty> {
                Self::$all.iter().copied().find(|entry| {
                    let $id_arg = entry;
                    $id_expr == id
                })
            }
        }

        $crate::__catalogue_tests! {
            type: $ty,
            id: |$id_arg| $id_expr,
            $(sorted_by: |$sort_arg| $sort_expr,)?
            $(provenance: |$prov_arg| $prov_expr,)?
            tests: $tests,
            all: <$ty>::$all,
            lookup: <$ty>::$lookup,
        }
    };
}

/// The tests both forms of [`catalogue!`] generate. Not for direct use.
#[doc(hidden)]
#[macro_export]
macro_rules! __catalogue_tests {
    (
        type: $ty:ty,
        id: |$id_arg:ident| $id_expr:expr,
        $(sorted_by: |$sort_arg:ident| $sort_expr:expr,)?
        $(provenance: |$prov_arg:ident| $prov_expr:expr,)?
        tests: $tests:ident,
        all: $all:expr,
        lookup: $lookup:expr,
    ) => {
        #[cfg(test)]
        mod $tests {
            use super::*;

            /// The table under test.
            fn all() -> &'static [$ty] {
                $all
            }

            /// The lookup under test.
            fn find(id: &str) -> Option<$ty> {
                ($lookup)(id)
            }

            /// The identifier of an entry.
            fn identity(entry: &$ty) -> &'static str {
                let $id_arg = entry;
                $id_expr
            }

            /// Two entries may not claim one identifier, or the lookup
            /// silently answers with whichever comes first.
            #[test]
            fn identifiers_are_unique() {
                let all = all();
                for (index, entry) in all.iter().enumerate() {
                    for other in &all[index + 1..] {
                        assert_ne!(
                            identity(entry),
                            identity(other),
                            "duplicate identifier {}",
                            identity(entry)
                        );
                    }
                }
            }

            /// Every entry must be reachable by its own identifier. This is
            /// what fails when an entry is declared and not listed — the
            /// mistake that used to need a third edit to avoid.
            #[test]
            fn every_entry_is_findable_by_its_own_identifier() {
                for entry in all() {
                    assert_eq!(
                        find(identity(entry)).as_ref(),
                        Some(entry),
                        "{} is in the table but not findable",
                        identity(entry)
                    );
                }
            }

            /// An identifier nothing has must find nothing.
            #[test]
            fn an_unknown_identifier_finds_nothing() {
                assert!(find("").is_none());
                assert!(find("no-such-entry-exists-here").is_none());
            }

            $(
                /// The table is in the order its documentation claims.
                #[test]
                fn the_table_is_sorted() {
                    for pair in all().windows(2) {
                        let $sort_arg = &pair[0];
                        let first = $sort_expr;
                        let $sort_arg = &pair[1];
                        let second = $sort_expr;
                        assert!(
                            first <= second,
                            "{} is listed before {}",
                            identity(&pair[0]),
                            identity(&pair[1])
                        );
                    }
                }
            )?

            $(
                /// Every entry names where it came from. Policy §10: a set
                /// without an authority is an opinion.
                #[test]
                fn every_entry_names_its_source() {
                    for entry in all() {
                        let $prov_arg = entry;
                        assert!(
                            !$prov_expr.is_empty(),
                            "{} does not say where it came from",
                            identity(entry)
                        );
                    }
                }
            )?
        }
    };
}
