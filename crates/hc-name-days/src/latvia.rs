//! Latvia's *vārda dienas*: the traditional and the extended list of the
//! Valsts valodas centrs, in two editions each.
//!
//! # Whose lists
//!
//! The Kalendārvārdu ekspertu komisija (calendar-name expert commission)
//! of the Valsts valodas centrs (State Language Centre, an institution
//! under the Ministry of Justice) keeps two lists: the *tradicionālais
//! kalendārvārdu saraksts*, about a thousand names printed in calendars,
//! and the *paplašinātais vārdadienu saraksts*, about five and a half
//! thousand names including borrowed and Latgalian forms. Its decisions are
//! recommendatory, and it meets as needed but not more often than once in
//! three years. The centre publishes both lists as open data under
//! CC0-1.0 [lva-vardadienas].
//!
//! # Editions
//!
//! Two revisions are carried, each as its own pair of lists (ADR 0007: an
//! authority's revisions are data, not corrections):
//!
//! | Edition | Decided | In force | File |
//! |---|---|---|---|
//! | 2023 | 16 March 2022 [vvc-2022] | 2023–2025 | the files published 2023-04-26, as the Wayback Machine captured them on 12–13 August 2024 |
//! | 2026 | 30 April 2025 [vvc-2025] | from 1 January 2026 | the files on data.gov.lv, retrieved 2026-09-25 |
//!
//! The 2022 decision added ten names to the traditional list and
//! thirty-nine to the extended one; the 2025 decision added nine and
//! fifty-six. The tests anchor every one of the nineteen traditional
//! additions, the 2014 additions the older edition already contains, and
//! a sample of the extended ones. A caller asking for 2025 gets the 2023
//! edition and one asking for 2026 gets the 2026 edition; [`crate::in_force`]
//! makes the choice from the year.
//!
//! # What the source prints that is not a name
//!
//! - `29.02.,–`: an en dash. [`LeapDayRule::NoNames`], and the slot is
//!   empty.
//! - `22.05.,Emīlija. Visu neparasto un kalendāros neierakstīto vārdu
//!   diena`: the name Emīlija, then the sentence "the day of all unusual
//!   names and names not written in calendars". The name is in the slot;
//!   the sentence is a [`Note`], and [`NameDayList::unlisted_names_day`]
//!   says 22 May.
//! - `Mora (LTG: Muora)` and five more in the 2026 extended list: a
//!   Latgalian form after its Latvian name. Both are in the slot, and a
//!   [`Note`] on the Latgalian form says which name it belongs to.
//!
//! Names are the list's own spelling, diacritics included. There is no
//! transliteration and no `hc-i18n` dependency: a name is data.

mod data;

use crate::list::{
    AttributionDate, LeapDayRule, Licence, MonthDay, NameDayList, Note, Provenance, Validity,
};

const AUTHORITY: &str = "Valsts valodas centrs, Kalendārvārdu ekspertu komisija";

const LICENCE: Licence = Licence::PublicDomainDedication("CC0-1.0");

const UNLISTED_NAMES_DAY: Option<MonthDay> = Some(MonthDay::new(5, 22));

const SOURCE_2023: &str = "Valsts valodas centrs, \"Latviešu tradicionālais un paplašinātais \
    kalendārvārdu saraksts\", data.gov.lv, CC0-1.0, the files published 2023-04-26 as captured \
    by the Wayback Machine on 2024-08-12 (tradic_vrdadienu_saraksts.xlsx) and 2024-08-13 \
    (paplasinatais_saraksts_.csv), retrieved 2026-09-25; the additions of the 16 March 2022 \
    decision per vvc.gov.lv, \"Par jauniem vārdiem kalendārā\", 17 March 2022";

const SOURCE_2026: &str = "Valsts valodas centrs, \"Latviešu tradicionālais un paplašinātais \
    kalendārvārdu saraksts\", https://data.gov.lv/dati/eng/dataset/latviesu-tradicionalais-un-paplasinatais-kalendarvardu-saraksts, \
    CC0-1.0, retrieved 2026-09-25 (traditional list published 2025-05-16, extended list \
    published 2026-03-16); the additions and the in-force date of the 30 April 2025 decision \
    per vvc.gov.lv, \"Par jauniem vārdiem kalendārā\", 16 May 2025";

hc_core::catalogue! {
    type: NameDayList,
    id: |list| list.id,
    sorted_by: |list| list.id,
    provenance: |list| list.source,
    tests: latvia_catalogue_tests,

    /// Every Latvian edition, by identifier.
    pub const ALL;
    /// The edition with this identifier.
    pub fn by_id;

    entries: {
        /// The extended list as in force 2023–2025, from the 2022 revision.
        ///
        /// 5,589 names. Superseded on 1 January 2026 by
        /// [`LV_EXTENDED_2026`]; kept because a caller asking about 2024
        /// should get the list that was in force in 2024.
        pub const LV_EXTENDED_2023 = NameDayList {
            id: "lv-extended-2023",
            country: "lv",
            language: "lv",
            english_name: "Latvian extended name-day list, 2022 revision",
            authority: AUTHORITY,
            decided: Some(AttributionDate::ymd(2022, 3, 16)),
            provenance: Provenance::Promulgated,
            validity: Validity::between(2023, 2025),
            licence: LICENCE,
            leap_day: LeapDayRule::NoNames,
            days: &data::EXTENDED_2023,
            unlisted_names_day: UNLISTED_NAMES_DAY,
            notes: data::EXTENDED_2023_NOTES,
            source: SOURCE_2023,
            retrieved: AttributionDate::ymd(2026, 9, 25),
        };

        /// The extended list in force from 1 January 2026, from the 2025
        /// revision.
        ///
        /// 5,652 names: the 2023 edition plus the fifty-six the commission
        /// added, six Latgalian forms printed in parentheses, three
        /// Latgalian spellings corrected (Seimanis → Seimaņs, Bārtulis →
        /// Bārtuļs, Meikulis → Meikuļs), and Enia on 4 March, which the
        /// centre's file of 2026-03-16 has and its file of 2026-01-10 did
        /// not.
        pub const LV_EXTENDED_2026 = NameDayList {
            id: "lv-extended-2026",
            country: "lv",
            language: "lv",
            english_name: "Latvian extended name-day list, 2025 revision",
            authority: AUTHORITY,
            decided: Some(AttributionDate::ymd(2025, 4, 30)),
            provenance: Provenance::Promulgated,
            validity: Validity::since(2026),
            licence: LICENCE,
            leap_day: LeapDayRule::NoNames,
            days: &data::EXTENDED_2026,
            unlisted_names_day: UNLISTED_NAMES_DAY,
            notes: data::EXTENDED_2026_NOTES,
            source: SOURCE_2026,
            retrieved: AttributionDate::ymd(2026, 9, 25),
        };

        /// The traditional list as in force 2023–2025, from the 2022
        /// revision.
        ///
        /// 1,023 names. Superseded on 1 January 2026 by
        /// [`LV_TRADITIONAL_2026`].
        pub const LV_TRADITIONAL_2023 = NameDayList {
            id: "lv-traditional-2023",
            country: "lv",
            language: "lv",
            english_name: "Latvian traditional name-day list, 2022 revision",
            authority: AUTHORITY,
            decided: Some(AttributionDate::ymd(2022, 3, 16)),
            provenance: Provenance::Promulgated,
            validity: Validity::between(2023, 2025),
            licence: LICENCE,
            leap_day: LeapDayRule::NoNames,
            days: &data::TRADITIONAL_2023,
            unlisted_names_day: UNLISTED_NAMES_DAY,
            notes: data::TRADITIONAL_2023_NOTES,
            source: SOURCE_2023,
            retrieved: AttributionDate::ymd(2026, 9, 25),
        };

        /// The traditional list in force from 1 January 2026, from the
        /// 2025 revision.
        ///
        /// 1,032 names: the 2023 edition plus Aleksa (8 November),
        /// Dominika (4 February), Eliass (2 August), Francisks (11
        /// January), Grēta (23 January), Madlēna (23 July), Mīla (19 June),
        /// Vanesa (23 September) and Vestards (28 June).
        pub const LV_TRADITIONAL_2026 = NameDayList {
            id: "lv-traditional-2026",
            country: "lv",
            language: "lv",
            english_name: "Latvian traditional name-day list, 2025 revision",
            authority: AUTHORITY,
            decided: Some(AttributionDate::ymd(2025, 4, 30)),
            provenance: Provenance::Promulgated,
            validity: Validity::since(2026),
            licence: LICENCE,
            leap_day: LeapDayRule::NoNames,
            days: &data::TRADITIONAL_2026,
            unlisted_names_day: UNLISTED_NAMES_DAY,
            notes: data::TRADITIONAL_2026_NOTES,
            source: SOURCE_2026,
            retrieved: AttributionDate::ymd(2026, 9, 25),
        };
    }
}

/// The note the source prints on 22 May, in its own words.
pub const UNLISTED_NAMES_NOTE: Note = Note {
    month: 5,
    day: 22,
    name: None,
    text: "Visu neparasto un kalendāros neierakstīto vārdu diena",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list::{FEBRUARY_29, NameDayError, days_of, names_on};

    /// The nine names of the 30 April 2025 decision, with their days, as
    /// vvc.gov.lv printed them on 16 May 2025.
    const ADDED_2025: [(&str, u8, u8); 9] = [
        ("Aleksa", 11, 8),
        ("Dominika", 2, 4),
        ("Eliass", 8, 2),
        ("Francisks", 1, 11),
        ("Grēta", 1, 23),
        ("Madlēna", 7, 23),
        ("Mīla", 6, 19),
        ("Vanesa", 9, 23),
        ("Vestards", 6, 28),
    ];

    /// The ten names of the 16 March 2022 decision, as vvc.gov.lv printed
    /// them on 17 March 2022.
    const ADDED_2022: [(&str, u8, u8); 10] = [
        ("Aurika", 3, 5),
        ("Meija", 4, 20),
        ("Jurgita", 4, 23),
        ("Vaiva", 9, 17),
        ("Pērle", 11, 7),
        ("Daris", 2, 6),
        ("Gerhards", 3, 17),
        ("Dzintis", 4, 6),
        ("Jumis", 9, 29),
        ("Ako", 12, 3),
    ];

    /// The eight names of the 2014 revision, per lvportals.lv (2014).
    const ADDED_2014: [(&str, u8, u8); 8] = [
        ("Brīve", 11, 18),
        ("Daiva", 4, 3),
        ("Egle", 1, 24),
        ("Liepa", 7, 16),
        ("Mare", 3, 25),
        ("Enriko", 5, 7),
        ("Lūkass", 10, 18),
        ("Vigo", 11, 20),
    ];

    fn on(list: &'static NameDayList, year: i32, month: u8, day: u8) -> &'static [&'static str] {
        match names_on(list, year, month, day) {
            Ok(names) => names,
            Err(error) => panic!("{} on {year}-{month}-{day}: {error}", list.id),
        }
    }

    #[test]
    fn the_first_of_january_is_laimnesis_solvita_solvija_in_every_edition() {
        assert_eq!(
            on(&LV_TRADITIONAL_2023, 2024, 1, 1),
            &["Laimnesis", "Solvita", "Solvija"]
        );
        assert_eq!(
            on(&LV_TRADITIONAL_2026, 2026, 1, 1),
            &["Laimnesis", "Solvita", "Solvija"]
        );
        assert_eq!(
            &on(&LV_EXTENDED_2026, 2026, 1, 1)[..3],
            &["Laimnesis", "Solvita", "Solvija"]
        );
        assert_eq!(on(&LV_EXTENDED_2026, 2026, 1, 1).len(), 12);
    }

    #[test]
    fn the_leap_day_carries_no_names_because_the_source_prints_a_dash() {
        for list in ALL {
            assert_eq!(list.leap_day, LeapDayRule::NoNames);
            assert!(list.slot(FEBRUARY_29).is_empty(), "{}", list.id);
            let year = if list.validity.contains(2024) {
                2024
            } else {
                2028
            };
            assert_eq!(on(list, year, 2, 29), &[] as &[&str], "{}", list.id);
            assert!(!on(list, year, 2, 28).is_empty(), "{}", list.id);
        }
        assert_eq!(
            on(&LV_TRADITIONAL_2026, 2028, 2, 28),
            &["Skaidrīte", "Justs", "Skaidra"]
        );
        assert_eq!(on(&LV_TRADITIONAL_2026, 2028, 3, 1), &["Ivars", "Ilgvars"]);
    }

    #[test]
    fn the_leap_day_is_the_only_empty_day_in_every_edition() {
        for list in ALL {
            let empty: [MonthDay; 1] = [MonthDay::new(2, 29)];
            assert!(list.empty_days().eq(empty), "{}", list.id);
        }
    }

    #[test]
    fn the_twenty_second_of_may_is_emilija_and_the_day_for_unlisted_names() {
        for list in ALL {
            assert_eq!(list.unlisted_names_day, Some(MonthDay::new(5, 22)));
            assert!(list.notes.contains(&UNLISTED_NAMES_NOTE), "{}", list.id);
            let year = if list.validity.contains(2024) {
                2024
            } else {
                2026
            };
            assert_eq!(
                on(list, year, 5, 22).first(),
                Some(&"Emīlija"),
                "{}",
                list.id
            );
        }
        assert_eq!(on(&LV_TRADITIONAL_2026, 2026, 5, 22), &["Emīlija"]);
        assert!(on(&LV_EXTENDED_2026, 2026, 5, 22).contains(&"Fatima"));
    }

    #[test]
    fn the_nine_names_of_2025_are_in_the_2026_edition_and_not_in_the_2023_one() {
        for (name, month, day) in ADDED_2025 {
            assert!(
                on(&LV_TRADITIONAL_2026, 2026, month, day).contains(&name),
                "{name} on {month}-{day}"
            );
            assert!(
                !on(&LV_TRADITIONAL_2023, 2025, month, day).contains(&name),
                "{name} already on {month}-{day} in 2023"
            );
            assert!(!LV_TRADITIONAL_2023.names(name), "{name} in the 2023 list");
            assert!(
                on(&LV_EXTENDED_2026, 2026, month, day).contains(&name),
                "{name} missing from the extended list"
            );
        }
        assert_eq!(
            LV_TRADITIONAL_2026.total_names(),
            LV_TRADITIONAL_2023.total_names() + 9
        );
    }

    #[test]
    fn the_ten_names_of_2022_and_the_eight_of_2014_are_in_both_editions() {
        for (name, month, day) in ADDED_2022.into_iter().chain(ADDED_2014) {
            assert!(
                on(&LV_TRADITIONAL_2023, 2023, month, day).contains(&name),
                "{name} on {month}-{day} in 2023"
            );
            assert!(
                on(&LV_TRADITIONAL_2026, 2026, month, day).contains(&name),
                "{name} on {month}-{day} in 2026"
            );
        }
    }

    #[test]
    fn a_sample_of_the_fifty_six_extended_additions_of_2025() {
        for (name, month, day) in [
            ("Karlo", 1, 28),
            ("Aidans", 2, 3),
            ("Selesta", 2, 5),
            ("Artemīda", 12, 20),
        ] {
            assert!(
                on(&LV_EXTENDED_2026, 2026, month, day).contains(&name),
                "{name}"
            );
            assert!(!LV_EXTENDED_2023.names(name), "{name} in the 2023 list");
        }
        // And the fourteen of 2022, present in both.
        for (name, month, day) in [
            ("Karls", 1, 28),
            ("Dafne", 2, 8),
            ("Kaupo", 7, 25),
            ("Ugne", 11, 17),
        ] {
            assert!(
                on(&LV_EXTENDED_2023, 2024, month, day).contains(&name),
                "{name}"
            );
            assert!(
                on(&LV_EXTENDED_2026, 2026, month, day).contains(&name),
                "{name}"
            );
        }
    }

    #[test]
    fn the_latgalian_forms_of_2026_are_names_with_a_note_saying_whose_they_are() {
        let forms = [
            ("Muora", "Mora", 3, 25),
            ("Buorbala", "Borbala", 4, 25),
            ("Juoņs", "Joņs", 6, 24),
            ("Puovuls", "Povuls", 6, 29),
        ];
        for (form, base, month, day) in forms {
            let names = on(&LV_EXTENDED_2026, 2026, month, day);
            assert!(names.contains(&form), "{form}");
            assert!(names.contains(&base), "{base}");
            let Some(note) = LV_EXTENDED_2026
                .notes
                .iter()
                .find(|note| note.name == Some(form))
            else {
                panic!("no note for {form}")
            };
            assert_eq!((note.month, note.day), (month, day));
            assert!(note.text.contains(base));
            assert!(!LV_EXTENDED_2023.names(form), "{form} in the 2023 list");
        }
        assert_eq!(
            LV_EXTENDED_2026.notes.len(),
            7,
            "six Latgalian forms and 22 May"
        );
        assert_eq!(LV_EXTENDED_2023.notes.len(), 1);
    }

    #[test]
    fn the_editions_refuse_the_years_they_do_not_cover() {
        assert_eq!(
            names_on(&LV_TRADITIONAL_2026, 2025, 1, 1),
            Err(NameDayError::OutsideValidity {
                year: 2025,
                validity: Validity::since(2026)
            })
        );
        assert_eq!(
            names_on(&LV_TRADITIONAL_2023, 2026, 1, 1),
            Err(NameDayError::OutsideValidity {
                year: 2026,
                validity: Validity::between(2023, 2025)
            })
        );
        assert!(names_on(&LV_TRADITIONAL_2023, 2022, 1, 1).is_err());
        assert!(names_on(&LV_TRADITIONAL_2023, 2023, 1, 1).is_ok());
        assert!(names_on(&LV_TRADITIONAL_2026, 2040, 1, 1).is_ok());
    }

    #[test]
    fn a_name_is_found_on_its_days_by_the_reverse_lookup() {
        let Ok(days) = days_of(&LV_TRADITIONAL_2026, "Jānis", 2026) else {
            panic!("2026 is covered")
        };
        assert!(days.eq([MonthDay::new(6, 24)]));
        let Ok(days) = days_of(&LV_TRADITIONAL_2026, "Grēta", 2026) else {
            panic!("2026 is covered")
        };
        assert!(days.eq([MonthDay::new(1, 23)]));
        let Ok(days) = days_of(&LV_TRADITIONAL_2026, "Greta", 2026) else {
            panic!("2026 is covered")
        };
        assert_eq!(days.count(), 0, "the match is exact: Greta is not Grēta");
        let Ok(days) = days_of(&LV_EXTENDED_2026, "Greta", 2026) else {
            panic!("2026 is covered")
        };
        assert!(
            days.eq([MonthDay::new(1, 23)]),
            "the extended list has Greta"
        );
    }

    /// The traditional list is a subset of the extended one, day by day,
    /// in each edition, as the centre describes it.
    #[test]
    fn the_traditional_list_is_contained_in_the_extended_list_day_by_day() {
        for (traditional, extended) in [
            (&LV_TRADITIONAL_2023, &LV_EXTENDED_2023),
            (&LV_TRADITIONAL_2026, &LV_EXTENDED_2026),
        ] {
            for index in 0..crate::list::DAYS {
                for name in traditional.slot(index) {
                    assert!(
                        extended.slot(index).contains(name),
                        "{name} in {} but not {} at slot {index}",
                        traditional.id,
                        extended.id
                    );
                }
            }
        }
    }

    #[test]
    fn no_slot_repeats_a_name_and_no_name_is_empty() {
        for list in ALL {
            for (index, slot) in list.days.iter().enumerate() {
                for (position, name) in slot.iter().enumerate() {
                    assert!(!name.is_empty(), "{} slot {index}", list.id);
                    assert!(
                        !name.contains(char::is_whitespace),
                        "{} slot {index}: {name:?}",
                        list.id
                    );
                    assert!(
                        !slot[position + 1..].contains(name),
                        "{} slot {index}: {name} twice",
                        list.id
                    );
                }
            }
        }
    }

    #[test]
    fn the_sizes_are_as_counted_from_the_files() {
        assert_eq!(LV_TRADITIONAL_2023.total_names(), 1023);
        assert_eq!(LV_TRADITIONAL_2026.total_names(), 1032);
        assert_eq!(LV_EXTENDED_2023.total_names(), 5589);
        assert_eq!(LV_EXTENDED_2026.total_names(), 5652);
    }

    #[test]
    fn every_edition_is_open_data_from_the_named_centre() {
        for list in ALL {
            assert_eq!(list.licence, Licence::PublicDomainDedication("CC0-1.0"));
            assert!(list.licence.permits_redistribution());
            assert_eq!(list.country, "lv");
            assert_eq!(list.language, "lv");
            assert_eq!(list.authority, AUTHORITY);
            assert_eq!(list.provenance, Provenance::Promulgated);
            assert!(list.source.contains("CC0-1.0"), "{}", list.id);
            assert!(list.source.contains("retrieved 2026-09-25"), "{}", list.id);
            assert!(list.decided.is_some(), "{}", list.id);
        }
    }

    #[test]
    fn the_two_editions_do_not_overlap_and_leave_no_year_between_them() {
        assert_eq!(LV_TRADITIONAL_2023.validity.to, Some(2025));
        assert_eq!(LV_TRADITIONAL_2026.validity.from, Some(2026));
        assert_eq!(LV_EXTENDED_2023.validity, LV_TRADITIONAL_2023.validity);
        assert_eq!(LV_EXTENDED_2026.validity, LV_TRADITIONAL_2026.validity);
    }
}
