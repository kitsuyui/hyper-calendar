//! Traditional month-name sets that are not already in `hc-i18n`.
//!
//! `hc-i18n` owns *localisation*: the month names a locale actually uses
//! today, including Japan's 和風月名 — 睦月, 如月, 弥生 … 師走 — which live in
//! `hc_i18n::data` and are reachable through `hc_i18n::names`. **Nothing
//! here duplicates them.** This crate depends on `hc-core`, `hc-calendar` and
//! `hc-seasons` and deliberately not on `hc-i18n`, so a caller who wants the
//! 和風月名 should ask `hc-i18n` for them; a copy here would be a second
//! place to get 水無月 wrong.
//!
//! What is here is the sets `hc-i18n` has no reason to carry, because they
//! are not any locale's current vocabulary:
//!
//! - **Old English**, as Bede recorded it in 725. [`OLD_ENGLISH_BEDE`].
//! - **The Frankish names Charlemagne introduced**, as Einhard recorded
//!   them. [`FRANKISH_CHARLEMAGNE`].
//! - **Finnish** and **Czech**, which are living languages whose month names
//!   are descriptive rather than Latin, and which `hc-i18n` carries as
//!   strings without saying what they mean. [`FINNISH`], [`CZECH`].
//!
//! Each set carries an English gloss per month as well as the name, because
//! a descriptive month name whose description is missing is just a word.
//!
//! ```
//! use hc_attributes::month_names::{CZECH, FINNISH};
//! use hc_calendar::Month;
//!
//! // November in Finnish is "the month of the dead"; in Czech, "leaf-fall".
//! assert_eq!(FINNISH.name(Month::regular(11)), Ok("marraskuu"));
//! assert_eq!(FINNISH.gloss(Month::regular(11)), Ok("month of the dead"));
//! assert_eq!(CZECH.name(Month::regular(11)), Ok("listopad"));
//! ```

use hc_calendar::{CalendarResult, Month};

use crate::authority::{
    AttributionDate, Authority, MonthTable, Provenance, Region, Validity, month_index,
};

/// A set of month names with an English gloss for each.
///
/// The names are an [`MonthTable`] like every other table in the crate, so
/// that [`crate::authority::unanimous`] and the rest work on them; the
/// glosses ride alongside because they are not attributions, they are
/// translations.
#[derive(Debug, Clone, Copy)]
pub struct MonthNameSet {
    names: MonthTable,
    glosses: [&'static str; 12],
}

impl MonthNameSet {
    /// Build a set. Used only by this module's static data.
    #[must_use]
    pub const fn new(names: MonthTable, glosses: [&'static str; 12]) -> Self {
        Self { names, glosses }
    }

    /// The underlying table, for the comparison helpers in
    /// [`crate::authority`].
    #[must_use]
    pub const fn table(&self) -> &MonthTable {
        &self.names
    }

    /// Who says so.
    #[must_use]
    pub const fn authority(&self) -> &Authority {
        self.names.authority()
    }

    /// Every name the set gives for a month.
    ///
    /// More than one where the source gives more than one: Bede's two Giuli
    /// and two Litha are disambiguated in later Old English by *ærra* and
    /// *æfterra*, and both forms are carried.
    ///
    /// # Errors
    ///
    /// As [`month_index`].
    pub fn names(&self, month: Month) -> CalendarResult<&'static [&'static str]> {
        self.names.at(month_index(month)?)
    }

    /// The first, and usually only, name for a month.
    ///
    /// # Errors
    ///
    /// As [`month_index`], plus [`hc_calendar::CalendarError::MissingField`]
    /// if the entry were empty — which no shipped set allows, and a test
    /// asserts.
    pub fn name(&self, month: Month) -> CalendarResult<&'static str> {
        self.names(month)?
            .first()
            .copied()
            .ok_or(hc_calendar::CalendarError::MissingField("month name"))
    }

    /// What the name means, in English.
    ///
    /// # Errors
    ///
    /// As [`month_index`].
    pub fn gloss(&self, month: Month) -> CalendarResult<&'static str> {
        let index = month_index(month)?;
        self.glosses
            .get(index)
            .copied()
            .ok_or(hc_calendar::CalendarError::MonthOutOfRange)
    }

    /// Every month's name and gloss, January first.
    pub fn iter(&self) -> impl Iterator<Item = (&'static [&'static str], &'static str)> + '_ {
        self.names.iter().zip(self.glosses)
    }
}

/// The Old English month names, as Bede gives them.
///
/// Bede sets them out in *De temporum ratione* (725), chapter 15, in a
/// chapter written expressly because he thought it wrong to describe other
/// nations' years and stay silent about his own. He also gives the glosses:
/// *Thrimilchi* because the cattle were milked three times a day,
/// *Weodmonath* "month of weeds", *Blodmonath* "month of immolations"
/// because that was when the beasts to be slaughtered were consecrated.
///
/// Two features of the list are not translation artefacts and are carried as
/// they stand. The year had **two** months called *Giuli* and **two** called
/// *Litha*, since it was lunar and paired around the solstices; the later
/// Old English forms distinguish them as *Ærra* ("before") and *Æfterra*
/// ("after"), and both Bede's Latin-transmitted spelling and the Old English
/// form are given. And in an embolismic year a *third* Litha was inserted,
/// making a "Thrilithi" year of thirteen months — which is why this is a
/// lunar list being shown against Gregorian months and not a Gregorian list.
///
/// *Eosturmonath* is where the English word Easter comes from; Bede says it
/// was named for a goddess *Eostre*, and he is the only source for her.
pub static OLD_ENGLISH_BEDE: MonthNameSet = MonthNameSet::new(
    MonthTable::new(
        Authority {
            id: "month-names-old-english-bede",
            english_name: "Old English month names",
            body: None,
            region: Region::EARLY_ENGLAND,
            established: Some(AttributionDate::year(725)),
            revised: None,
            validity: Validity::between(500, 1100),
            provenance: Provenance::Recorded,
            source: "Bede, De temporum ratione (725), ch. 15, tr. Faith Wallis, The Reckoning \
                     of Time (Liverpool University Press, 1999); Old English forms per \
                     Gerhard Köbler, Althochdeutsches Wörterbuch",
            caveat: Some(
                "A lunar month list shown against Gregorian months. The year had two Giuli and \
                 two Litha, and an embolismic year had a third Litha, so the mapping to twelve \
                 solar months is an approximation Bede did not make.",
            ),
        },
        [
            &["Æfterra Gēola", "Giuli"],
            &["Solmōnaþ"],
            &["Hrēþmōnaþ"],
            &["Ēosturmōnaþ"],
            &["Þrimilcemōnaþ"],
            &["Ærra Līþa", "Litha"],
            &["Æftera Līþa", "Litha"],
            &["Wēodmōnaþ"],
            &["Hāligmōnaþ"],
            &["Winterfylleth"],
            &["Blōtmōnaþ"],
            &["Ærra Gēola", "Giuli"],
        ],
    ),
    [
        "after Yule",
        "month of cakes",
        "month of the goddess Hrēþ",
        "month of the goddess Ēostre",
        "month of three milkings",
        "before midsummer",
        "after midsummer",
        "month of weeds",
        "holy month",
        "winter full moon",
        "month of sacrifice",
        "before Yule",
    ],
);

/// The Frankish month names Charlemagne introduced.
///
/// Einhard's *Vita Karoli Magni* (c. 830), chapter 29, records that
/// Charlemagne gave the months names in his own tongue in place of the part
/// Latin, part barbarous names the Franks had been using — an act of
/// vernacular standardisation twelve centuries before a trade association
/// standardised a birthstone list, and with about as much staying power:
/// the Latin names stayed in general use and these survive only in German
/// dialect.
///
/// *Hornung* for February is the only name without the "month" suffix. It
/// is usually read as a collective of *horn*, for the antlers red deer shed
/// at that time; an older reading connects it to Old Frisian *horning*
/// "illegitimate son", taken as "disinherited" because February is the short
/// month. Both readings are in print and the gloss gives the first.
///
/// Two of the glosses have a second published reading: *Winnemānōth* is
/// "pasture month" in Turner's translation and "joy month" in De Rynck's,
/// and *Brāchmānōth* is "break-ground month" in the first and "plough month"
/// in the second. The gloss gives the first and the caveat says so, rather
/// than averaging two translators.
pub static FRANKISH_CHARLEMAGNE: MonthNameSet = MonthNameSet::new(
    MonthTable::new(
        Authority {
            id: "month-names-frankish-charlemagne",
            english_name: "Frankish month names introduced by Charlemagne",
            body: Some("Charlemagne"),
            region: Region::FRANCIA,
            established: Some(AttributionDate::year(800)),
            revised: None,
            validity: Validity::between(800, 1500),
            provenance: Provenance::Recorded,
            source: "Einhard, Vita Karoli Magni (c. 830), ch. 29, tr. Samuel Epes Turner, Life \
                     of Charlemagne (American Book Company, 1880), pp. 66–67, and tr. Patrick \
                     De Rynck (Athenaeum — Polak & Van Gennep, 1999), p. 59; forms per Wilhelm \
                     Braune, Althochdeutsches Lesebuch, 17th ed., p. 8",
            caveat: Some(
                "Two glosses have competing published translations: Winnemānōth is \"pasture\" \
                 in Turner and \"joy\" in De Rynck, Brāchmānōth \"break-ground\" and \"plough\" \
                 respectively. The gloss gives Turner's.",
            ),
        },
        [
            &["Wintarmānōth"],
            &["Hornung"],
            &["Lentzinmānōth"],
            &["Ōstarmānōth"],
            &["Winnemānōth"],
            &["Brāchmānōth"],
            &["Hewimānōth"],
            &["Aranmānōth"],
            &["Witumānōth"],
            &["Windumemānōth"],
            &["Herbistmānōth"],
            &["Heilagmānōth"],
        ],
    ),
    [
        "winter month",
        "horn-shedding, of stags",
        "spring month",
        "Easter month",
        "pasture month",
        "break-ground month",
        "hay month",
        "ears-of-grain month",
        "wood month",
        "vintage month",
        "harvest month",
        "holy month",
    ],
);

/// The Finnish month names, every one of them descriptive.
///
/// Finnish never took the Latin names. Each month is a word plus *-kuu*,
/// "moon" — a lunar suffix on a solar month, which is itself a fossil.
///
/// The glosses are the standard etymologies; two are not settled.
/// *Tammikuu* is not "oak month" despite *tammi* meaning oak today: in
/// dialect *tammi* is the hub of a wheel or the central beam of a mill, and
/// the month is the core of the winter. *Maaliskuu* is uncertain and is
/// usually connected to *maa*, "earth", for the ground emerging from the
/// snow. The caveat records both.
///
/// *Marraskuu*, "month of the dead", is the same idea as Welsh *Tachwedd*
/// and Old English *Blōtmōnaþ*: November is when the beasts were killed.
pub static FINNISH: MonthNameSet = MonthNameSet::new(
    MonthTable::new(
        Authority {
            id: "month-names-finnish",
            english_name: "Finnish month names",
            body: None,
            region: Region::FINLAND,
            established: None,
            revised: None,
            validity: Validity::UNKNOWN,
            provenance: Provenance::Vernacular,
            source: "Standard Finnish usage; etymologies per Uusi kielemme, \"The Meaning of \
                     the Finnish Months\", and the Institute for the Languages of Finland \
                     (Kotus) material summarised in Wiktionary's Finnish month entries",
            caveat: Some(
                "Two etymologies are unsettled: tammikuu is not \"oak month\" but from a \
                 dialectal tammi \"hub, core\"; maaliskuu is uncertain and only probably from \
                 maa \"earth\".",
            ),
        },
        [
            &["tammikuu"],
            &["helmikuu"],
            &["maaliskuu"],
            &["huhtikuu"],
            &["toukokuu"],
            &["kesäkuu"],
            &["heinäkuu"],
            &["elokuu"],
            &["syyskuu"],
            &["lokakuu"],
            &["marraskuu"],
            &["joulukuu"],
        ],
    ),
    [
        "core month, the heart of winter",
        "pearl month, for the ice beads on the branches",
        "earth month, when the ground reappears",
        "slash-and-burn month",
        "sowing month",
        "summer month",
        "hay month",
        "harvest month",
        "autumn month",
        "mud month",
        "month of the dead",
        "Yule month",
    ],
);

/// The Czech month names, also descriptive and also not Latin.
///
/// Czech, like Polish and Croatian and unlike Slovak, kept Slavic month
/// names. The etymologies are from Vojtěch Kebrle's article in *Naše řeč*
/// (1939), the journal of what is now the Czech Language Institute.
///
/// *Červen* and *červenec* are the same word, June being the first and July
/// formerly "the second červen"; the root is *červ*, the scale insect
/// harvested for dye. *Září* and *říjen* both refer to the deer rut, which
/// is why the glosses repeat. *Listopad*, "leaf-fall", is the one everybody
/// knows.
pub static CZECH: MonthNameSet = MonthNameSet::new(
    MonthTable::new(
        Authority {
            id: "month-names-czech",
            english_name: "Czech month names",
            body: None,
            region: Region::CZECHIA,
            established: None,
            revised: None,
            validity: Validity::UNKNOWN,
            provenance: Provenance::Vernacular,
            source: "Vojtěch Kebrle, \"Česká jména měsíců, jejich význam a původ\", Naše řeč \
                     23 (1939), no. 3, pp. 65–67",
            caveat: None,
        },
        [
            &["leden"],
            &["únor"],
            &["březen"],
            &["duben"],
            &["květen"],
            &["červen"],
            &["červenec"],
            &["srpen"],
            &["září"],
            &["říjen"],
            &["listopad"],
            &["prosinec"],
        ],
    ),
    [
        "ice month",
        "month the ice sinks as it melts",
        "birch month",
        "oak month",
        "flower month",
        "month of the dye insect",
        "the second červen",
        "sickle month",
        "month of the deer rut",
        "month of the deer rut",
        "leaf-fall",
        "month the sun shines through",
    ],
);

/// Every month-name set this crate ships.
pub static ALL: [&MonthNameSet; 4] = [&OLD_ENGLISH_BEDE, &FRANKISH_CHARLEMAGNE, &FINNISH, &CZECH];

#[cfg(test)]
mod tests {
    use hc_calendar::CalendarError;

    use super::*;

    #[test]
    fn every_set_names_and_glosses_all_twelve_months() {
        for set in ALL {
            for ordinal in 1..=12u8 {
                let month = Month::regular(ordinal);
                let name = set.name(month);
                assert!(name.is_ok(), "{} month {ordinal}", set.authority().id);
                assert!(!name.unwrap().is_empty());
                let gloss = set.gloss(month);
                assert!(gloss.is_ok(), "{} month {ordinal}", set.authority().id);
                assert!(!gloss.unwrap().is_empty());
            }
            assert!(set.table().is_complete(), "{}", set.authority().id);
        }
    }

    #[test]
    fn every_set_names_its_source() {
        for set in ALL {
            assert!(!set.authority().source.is_empty(), "{}", set.authority().id);
        }
    }

    #[test]
    fn iterating_a_set_yields_twelve_name_and_gloss_pairs() {
        for set in ALL {
            let pairs: usize = set.iter().count();
            assert_eq!(pairs, 12, "{}", set.authority().id);
            for (names, gloss) in set.iter() {
                assert!(!names.is_empty());
                assert!(!gloss.is_empty());
            }
        }
    }

    /// Bede's year has two Giuli and two Litha, and the set keeps both
    /// forms of each rather than silently picking one.
    #[test]
    fn bedes_year_has_two_yule_months_and_two_midsummer_months() {
        assert_eq!(
            OLD_ENGLISH_BEDE.name(Month::regular(1)),
            Ok("Æfterra Gēola")
        );
        assert_eq!(OLD_ENGLISH_BEDE.name(Month::regular(12)), Ok("Ærra Gēola"));
        assert_eq!(OLD_ENGLISH_BEDE.name(Month::regular(6)), Ok("Ærra Līþa"));
        assert_eq!(OLD_ENGLISH_BEDE.name(Month::regular(7)), Ok("Æftera Līþa"));
        let giuli: [usize; 2] = [0, 11];
        let litha: [usize; 2] = [5, 6];
        assert!(OLD_ENGLISH_BEDE.table().keys_naming("Giuli").eq(giuli));
        assert!(OLD_ENGLISH_BEDE.table().keys_naming("Litha").eq(litha));
    }

    #[test]
    fn bedes_list_is_flagged_as_lunar_because_it_is() {
        let caveat = OLD_ENGLISH_BEDE.authority().caveat.unwrap_or("");
        assert!(caveat.contains("lunar"));
        assert_eq!(
            OLD_ENGLISH_BEDE.authority().provenance,
            Provenance::Recorded
        );
    }

    #[test]
    fn eosturmonath_is_the_source_of_the_english_word_easter() {
        assert_eq!(OLD_ENGLISH_BEDE.name(Month::regular(4)), Ok("Ēosturmōnaþ"));
        assert_eq!(
            OLD_ENGLISH_BEDE.gloss(Month::regular(4)),
            Ok("month of the goddess Ēostre")
        );
        // Charlemagne's April is the same idea, Christianised.
        assert_eq!(
            FRANKISH_CHARLEMAGNE.name(Month::regular(4)),
            Ok("Ōstarmānōth")
        );
        assert_eq!(
            FRANKISH_CHARLEMAGNE.gloss(Month::regular(4)),
            Ok("Easter month")
        );
    }

    #[test]
    fn charlemagnes_february_is_the_only_name_without_a_month_suffix() {
        assert_eq!(FRANKISH_CHARLEMAGNE.name(Month::regular(2)), Ok("Hornung"));
        for ordinal in 1..=12u8 {
            let name = FRANKISH_CHARLEMAGNE.name(Month::regular(ordinal)).unwrap();
            let has_suffix = name.ends_with("mānōth");
            assert_eq!(has_suffix, ordinal != 2, "{name}");
        }
    }

    #[test]
    fn charlemagnes_set_records_that_two_glosses_have_rival_translations() {
        let caveat = FRANKISH_CHARLEMAGNE.authority().caveat.unwrap_or("");
        assert!(caveat.contains("Winnemānōth"));
        assert!(caveat.contains("Brāchmānōth"));
        assert_eq!(
            FRANKISH_CHARLEMAGNE.gloss(Month::regular(5)),
            Ok("pasture month")
        );
    }

    /// Three unrelated traditions agree that November is about killing
    /// livestock for the winter, which is the kind of thing having the
    /// glosses makes visible.
    #[test]
    fn three_traditions_independently_call_november_the_month_of_death() {
        assert_eq!(FINNISH.gloss(Month::regular(11)), Ok("month of the dead"));
        assert_eq!(
            OLD_ENGLISH_BEDE.gloss(Month::regular(11)),
            Ok("month of sacrifice")
        );
        assert_eq!(FINNISH.name(Month::regular(11)), Ok("marraskuu"));
        assert_eq!(OLD_ENGLISH_BEDE.name(Month::regular(11)), Ok("Blōtmōnaþ"));
    }

    #[test]
    fn every_finnish_month_ends_in_the_moon_suffix() {
        for ordinal in 1..=12u8 {
            let name = FINNISH.name(Month::regular(ordinal)).unwrap();
            assert!(name.ends_with("kuu"), "{name}");
        }
    }

    #[test]
    fn the_finnish_set_refuses_the_folk_etymology_of_january() {
        assert_eq!(FINNISH.name(Month::regular(1)), Ok("tammikuu"));
        let gloss = FINNISH.gloss(Month::regular(1)).unwrap();
        assert!(!gloss.contains("oak"), "{gloss}");
        assert!(FINNISH.authority().caveat.unwrap_or("").contains("oak"));
    }

    #[test]
    fn july_in_czech_is_literally_the_second_june() {
        assert_eq!(CZECH.name(Month::regular(6)), Ok("červen"));
        assert_eq!(CZECH.name(Month::regular(7)), Ok("červenec"));
        assert_eq!(CZECH.gloss(Month::regular(7)), Ok("the second červen"));
    }

    #[test]
    fn september_and_october_in_czech_share_a_gloss_because_they_share_a_root() {
        assert_eq!(
            CZECH.gloss(Month::regular(9)),
            CZECH.gloss(Month::regular(10))
        );
        assert_ne!(
            CZECH.name(Month::regular(9)),
            CZECH.name(Month::regular(10))
        );
    }

    #[test]
    fn a_leap_month_has_no_traditional_name_in_any_set() {
        for set in ALL {
            assert_eq!(
                set.name(Month::leap(3)),
                Err(CalendarError::UnsupportedField("leap month"))
            );
            assert_eq!(
                set.gloss(Month::leap(3)),
                Err(CalendarError::UnsupportedField("leap month"))
            );
        }
    }

    #[test]
    fn a_month_outside_the_year_is_out_of_range_in_any_set() {
        for set in ALL {
            assert_eq!(
                set.name(Month::regular(0)),
                Err(CalendarError::MonthOutOfRange)
            );
            assert_eq!(
                set.gloss(Month::regular(13)),
                Err(CalendarError::MonthOutOfRange)
            );
        }
    }

    /// The crate deliberately does not carry the Japanese 和風月名; they are
    /// `hc-i18n`'s, and this test is what stops a future edit adding them.
    #[test]
    fn no_set_here_duplicates_the_japanese_traditional_month_names() {
        for set in ALL {
            for name in ["睦月", "如月", "弥生", "水無月", "師走"] {
                assert!(
                    !set.table().names(name),
                    "{} carries {name}",
                    set.authority().id
                );
            }
        }
    }
}
