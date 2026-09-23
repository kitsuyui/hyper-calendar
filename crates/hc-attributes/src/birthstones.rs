//! Birthstones by month — six lists, six authorities, no default.
//!
//! **There is no such thing as "the" birthstone for a month.** There is the
//! American list, which has been revised three times since 1912; the British
//! list of 1937; Japan's list of 1958, substantially revised in 2021; and
//! the older Western stones that all three replaced. They disagree, the
//! disagreements are recent and documented, and this module's job is to make
//! them visible rather than to pick a winner.
//!
//! So there is no `birthstone(month)` function here. There are six tables,
//! each naming its authority, and [`all_for_month`] which asks them all at
//! once. `docs/policy.md` §5 is the reason: competing conventions get names,
//! not parameters, because a parameter can be forgotten and a default is a
//! position taken on the caller's behalf without saying so.
//!
//! ```
//! use hc_attributes::birthstones::{BIRTHSTONES_JP_2021, BIRTHSTONES_US_2016, stones};
//! use hc_calendar::Month;
//!
//! let march = Month::regular(3);
//! // The American list has two stones for March; Japan's 2021 list has four.
//! assert_eq!(stones(&BIRTHSTONES_US_2016, march), Ok(&["aquamarine", "bloodstone"][..]));
//! assert!(stones(&BIRTHSTONES_JP_2021, march)?.contains(&"coral"));
//! # Ok::<(), hc_calendar::CalendarError>(())
//! ```
//!
//! # What the six lists are
//!
//! | Table | Authority | Date |
//! |---|---|---|
//! | [`BIRTHSTONES_TRADITIONAL`] | none; Kunz's reconstruction of 15th–20th century usage | 1913 (recorded) |
//! | [`BIRTHSTONES_US_1912`] | National Association of Jewelers | August 1912 |
//! | [`BIRTHSTONES_US_2016`] | Jewelers of America / AGTA, as the GIA prints it | 1912, rev. 1952, 2002, 2016 |
//! | [`BIRTHSTONES_UK`] | National Association of Goldsmiths | 1937 |
//! | [`BIRTHSTONES_JP_1958`] | 全国宝石商協同組合 | 1958 |
//! | [`BIRTHSTONES_JP_2021`] | 全国宝石卸商協同組合, with JJA and YJA | 1958, rev. 20 December 2021 |
//!
//! The 2021 Japanese revision is the most recent change anywhere: it added
//! ten stones after sixty-three years: four following the American list, one
//! restored on historical grounds and five chosen in Japan. Chrysoberyl cat's-eye went to February
//! because 22 February is 猫の日 in Japan — which is as good an illustration
//! as the crate could ask for of what kind of fact a birthstone is.
//!
//! # Names are lowercase English
//!
//! Entries are the English gem names in lowercase, so that two lists naming
//! the same stone compare equal. Where a list's own language differs, the
//! difference is in the doc comment, not in the data: Japan's list prints
//! サンゴ and ヒスイ, which are "coral" and "jade" here.

use hc_calendar::{CalendarResult, Month};

use crate::authority::{
    AttributionDate, Authority, MonthTable, Provenance, Region, Validity, disagreement_count,
    month_index, unanimous,
};

/// The Western stones in use before the 1912 standardisation.
///
/// George Frederick Kunz assembled this from the 15th to 20th century
/// European sources in *The Curious Lore of Precious Stones* (Lippincott,
/// 1913), p. 315. It is a reconstruction of usage, not a promulgation:
/// nobody adopted it, which is exactly why the 1912 meeting happened.
///
/// It is a long way from the modern lists. March is bloodstone and jasper
/// with no aquamarine; June is cat's-eye, turquoise and agate with no pearl;
/// December is bloodstone and ruby with no turquoise at all.
pub static BIRTHSTONES_TRADITIONAL: MonthTable = MonthTable::new(
    Authority {
        id: "birthstones-traditional",
        english_name: "traditional Western birthstones, 15th–20th century",
        body: None,
        region: Region::UNSPECIFIED,
        established: None,
        revised: None,
        validity: Validity::between(1400, 1912),
        provenance: Provenance::Recorded,
        source: "George F. Kunz, The Curious Lore of Precious Stones (Lippincott, 1913), p. 315",
        caveat: Some(
            "A reconstruction of European usage across five centuries, not a list any body \
             adopted. Usage varied by country and by century within the span it covers.",
        ),
    },
    [
        &["garnet"],
        &["amethyst", "hyacinth", "pearl"],
        &["bloodstone", "jasper"],
        &["diamond", "sapphire"],
        &["emerald", "agate"],
        &["cat's-eye", "turquoise", "agate"],
        &["turquoise", "onyx"],
        &["sardonyx", "carnelian", "moonstone", "topaz"],
        &["chrysolite"],
        &["opal", "aquamarine"],
        &["topaz", "pearl"],
        &["bloodstone", "ruby"],
    ],
);

/// The American list as first adopted, August 1912.
///
/// The National Association of Jewelers — now Jewelers of America — met in
/// Kansas City and adopted a standard list, printed in Kunz (1913),
/// pp. 319–320. Three later revisions changed it; [`BIRTHSTONES_US_2016`] is
/// where it ended up.
///
/// Three differences from the modern American list are worth keeping: March
/// led with bloodstone rather than aquamarine, August led with sardonyx
/// rather than peridot, and December was turquoise and lapis lazuli with no
/// zircon and no tanzanite.
pub static BIRTHSTONES_US_1912: MonthTable = MonthTable::new(
    Authority {
        id: "birthstones-us-1912",
        english_name: "American birthstones as adopted in 1912",
        body: Some("National Association of Jewelers"),
        region: Region::UNITED_STATES,
        established: Some(AttributionDate::year_month(1912, 8)),
        revised: None,
        validity: Validity::between(1912, 1951),
        provenance: Provenance::Promulgated,
        source: "George F. Kunz, The Curious Lore of Precious Stones (Lippincott, 1913), \
                 pp. 317, 319–320",
        caveat: Some("Superseded in 1952. Shipped so the revisions can be measured."),
    },
    [
        &["garnet"],
        &["amethyst"],
        &["bloodstone", "aquamarine"],
        &["diamond"],
        &["emerald"],
        &["pearl", "moonstone"],
        &["ruby"],
        &["sardonyx", "peridot"],
        &["sapphire"],
        &["opal", "tourmaline"],
        &["topaz"],
        &["turquoise", "lapis lazuli"],
    ],
);

/// The current American list: 1912, revised 1952, 2002 and 2016.
///
/// The Jewelry Industry Council of America's 1952 revision added
/// alexandrite to June, citrine to November and pink tourmaline to October,
/// replaced December's lapis lazuli with zircon, and swapped March's primary
/// and alternative. The American Gem Trade Association added tanzanite to
/// December in 2002 and, with Jewelers of America, spinel to August in 2016.
/// The entries here follow the Gemological Institute of America's published
/// chart, which is what the trade currently prints.
///
/// Three revisions in a century is the reason [`Validity::is_current`] means
/// "still current as far as this crate knows" and not "settled".
pub static BIRTHSTONES_US_2016: MonthTable = MonthTable::new(
    Authority {
        id: "birthstones-us-2016",
        english_name: "American birthstones, 2016 revision",
        body: Some("Jewelers of America and the American Gem Trade Association"),
        region: Region::UNITED_STATES,
        established: Some(AttributionDate::year_month(1912, 8)),
        revised: Some(AttributionDate::year(2016)),
        validity: Validity::since(2016),
        provenance: Provenance::Promulgated,
        source: "Gemological Institute of America, \"Birthstones by Month\", \
                 https://www.gia.edu/birthstones; spinel added per National Jeweler, \
                 \"JA, AGTA Add Spinel as August Birthstone\" (2016); tanzanite per Grande & \
                 Augustyn, Gems and Gemstones (Univ. of Chicago Press, 2009), p. 335",
        caveat: None,
    },
    [
        &["garnet"],
        &["amethyst"],
        &["aquamarine", "bloodstone"],
        &["diamond"],
        &["emerald"],
        &["pearl", "moonstone", "alexandrite"],
        &["ruby"],
        &["peridot", "spinel", "sardonyx"],
        &["sapphire"],
        &["opal", "tourmaline"],
        &["topaz", "citrine"],
        &["turquoise", "zircon", "tanzanite"],
    ],
);

/// The British list, standardised by the National Association of Goldsmiths
/// in 1937.
///
/// Four months differ from the American list in ways that are not merely a
/// matter of which stone is printed first: April carries rock crystal, May
/// carries chrysoprase, July carries carnelian and September carries lapis
/// lazuli, none of which appear in the current American list. Lapis lazuli
/// was in the 1912 one, for December. October is opal alone,
/// where the American list has had tourmaline beside it since 1912.
pub static BIRTHSTONES_UK: MonthTable = MonthTable::new(
    Authority {
        id: "birthstones-uk-1937",
        english_name: "British birthstones",
        body: Some("National Association of Goldsmiths"),
        region: Region::UNITED_KINGDOM,
        established: Some(AttributionDate::year(1937)),
        revised: None,
        validity: Validity::since(1937),
        provenance: Provenance::Promulgated,
        source: "The National Association of Goldsmiths, \"Tips & Tools: Birthstones\", \
                 jewellers-online.org (archived 2007); standardisation date per Harold Osborne \
                 (ed.), The Oxford Companion to the Decorative Arts (OUP, 1985), p. 513",
        caveat: None,
    },
    [
        &["garnet"],
        &["amethyst"],
        &["aquamarine", "bloodstone"],
        &["diamond", "rock crystal"],
        &["emerald", "chrysoprase"],
        &["pearl", "moonstone"],
        &["ruby", "carnelian"],
        &["peridot", "sardonyx"],
        &["sapphire", "lapis lazuli"],
        &["opal"],
        &["topaz", "citrine"],
        &["tanzanite", "turquoise"],
    ],
);

/// Japan's original list, 1958.
///
/// 全国宝石商協同組合 — since renamed 全国宝石卸商協同組合 — took the
/// American list and added two stones of its own: coral to March and jade to
/// May. The association's chairman 巽忠春 explained the reason in *あなたの宝石*
/// (徳間書店, 1962), p. 77, and it is a commercial one. March's aquamarine
/// and bloodstone are cheap stones, so a dearer option was wanted; May's
/// emerald is dear, so a cheaper one was.
///
/// That is what a birthstone list is: a trade deciding what to stock. Saying
/// so is not a criticism of the list, it is the provenance.
pub static BIRTHSTONES_JP_1958: MonthTable = MonthTable::new(
    Authority {
        id: "birthstones-jp-1958",
        english_name: "Japanese birthstones as adopted in 1958",
        body: Some("全国宝石商協同組合 (National Association of Gem Retailers, Japan)"),
        region: Region::JAPAN,
        established: Some(AttributionDate::year(1958)),
        revised: None,
        validity: Validity::between(1958, 2021),
        provenance: Provenance::Promulgated,
        source: "全国宝石卸商協同組合, 組織沿革, https://zho.or.jp/history/; rationale for coral \
                 and jade per 巽忠春, あなたの宝石 (徳間書店, 1962), p. 77",
        caveat: Some("Superseded by the 2021 revision. Shipped so the revision can be measured."),
    },
    [
        &["garnet"],
        &["amethyst"],
        &["aquamarine", "coral"],
        &["diamond"],
        &["emerald", "jade"],
        &["pearl", "moonstone"],
        &["ruby"],
        &["peridot", "sardonyx"],
        &["sapphire"],
        &["opal", "tourmaline"],
        &["topaz", "citrine"],
        &["turquoise", "lapis lazuli"],
    ],
);

/// Japan's list as revised on 20 December 2021 — the most recent revision
/// anywhere.
///
/// 全国宝石卸商協同組合 (ZHO), with the agreement of 日本ジュエリー協会 (JJA)
/// and 山梨県水晶宝飾協同組合 (YJA), added ten stones after sixty-three
/// years, taking the total to twenty-nine. The stated motive was that rival
/// lists had proliferated and were confusing customers — and, per NHK's
/// report, that the domestic jewellery market had fallen to under a third of
/// its peak.
///
/// Four of the ten follow Jewelers of America: alexandrite (June), spinel
/// (August), tanzanite and zircon (December). Bloodstone (March) was added
/// on historical grounds. The other five were chosen in Japan: chrysoberyl
/// cat's-eye for February, because 22 February is 猫の日 and 17 February is
/// World Cat Day; and iolite (March), morganite (April), sphene (July) and
/// kunzite (September) for their colour or for the birth month of someone
/// associated with them.
///
/// Compare with [`BIRTHSTONES_US_2016`]: the two still disagree in seven
/// months, and [`crate::birthstones::us_and_japan_differ_in`] counts them.
pub static BIRTHSTONES_JP_2021: MonthTable = MonthTable::new(
    Authority {
        id: "birthstones-jp-2021",
        english_name: "Japanese birthstones, 2021 revision",
        body: Some(
            "全国宝石卸商協同組合 (ZHO), with 日本ジュエリー協会 and 山梨県水晶宝飾協同組合",
        ),
        region: Region::JAPAN,
        established: Some(AttributionDate::year(1958)),
        revised: Some(AttributionDate::ymd(2021, 12, 20)),
        validity: Validity::since(2021),
        provenance: Provenance::Promulgated,
        source: "全国宝石卸商協同組合, 誕生石改訂事業 (20 December 2021), \
                 https://zho.or.jp/wp-content/uploads/2021/12/誕生石の改訂.pdf; reported by NHK \
                 and まいどなニュース, 20 December 2021",
        caveat: None,
    },
    [
        &["garnet"],
        &["amethyst", "chrysoberyl cat's-eye"],
        &["aquamarine", "coral", "bloodstone", "iolite"],
        &["diamond", "morganite"],
        &["emerald", "jade"],
        &["pearl", "moonstone", "alexandrite"],
        &["ruby", "sphene"],
        &["peridot", "sardonyx", "spinel"],
        &["sapphire", "kunzite"],
        &["opal", "tourmaline"],
        &["topaz", "citrine"],
        &["turquoise", "lapis lazuli", "zircon", "tanzanite"],
    ],
);

/// Every birthstone authority this crate ships, oldest first.
///
/// Iterating this is how a caller asks "whose?" without having to know the
/// list of names in advance, and how a new authority becomes available to
/// every caller at once.
pub static ALL: [&MonthTable; 6] = [
    &BIRTHSTONES_TRADITIONAL,
    &BIRTHSTONES_US_1912,
    &BIRTHSTONES_UK,
    &BIRTHSTONES_JP_1958,
    &BIRTHSTONES_US_2016,
    &BIRTHSTONES_JP_2021,
];

/// The authorities that are current rather than superseded.
///
/// [`BIRTHSTONES_TRADITIONAL`], [`BIRTHSTONES_US_1912`] and
/// [`BIRTHSTONES_JP_1958`] have closed validity spans; the other three do
/// not.
pub fn current() -> impl Iterator<Item = &'static MonthTable> {
    ALL.into_iter()
        .filter(|table| table.authority().validity.is_current())
}

/// The stones one authority names for a month.
///
/// # Errors
///
/// Whatever [`month_index`] returns: [`hc_calendar::CalendarError::UnsupportedField`]
/// for a leap month, [`hc_calendar::CalendarError::MonthOutOfRange`] for an
/// ordinal outside 1..=12.
pub fn stones(table: &MonthTable, month: Month) -> CalendarResult<&'static [&'static str]> {
    table.at(month_index(month)?)
}

/// Every authority's answer for one month, paired with the authority.
///
/// This is the honest answer to "what is the birthstone for March": six
/// rows, not one stone. A caller that wants a single value has to choose a
/// row, and choosing a row means naming an authority.
///
/// # Errors
///
/// As [`stones`].
pub fn all_for_month(
    month: Month,
) -> CalendarResult<impl Iterator<Item = (&'static Authority, &'static [&'static str])>> {
    let index = month_index(month)?;
    Ok(ALL
        .into_iter()
        .filter_map(move |table| Some((table.authority(), table.at(index).ok()?))))
}

/// Whether every shipped authority names exactly the same stones for a
/// month.
///
/// It is true for exactly one month out of twelve. January has been garnet
/// in every list this crate ships, from the pre-1912 European usage to the
/// Japanese revision of 2021 — the single point on which five centuries and
/// three countries have never disagreed. A test asserts it rather than this
/// doc comment merely claiming it.
///
/// # Errors
///
/// As [`stones`].
pub fn authorities_agree(month: Month) -> CalendarResult<bool> {
    unanimous(&ALL, month_index(month)?)
}

/// How many months the shipped authorities disagree about. Eleven of twelve.
///
/// # Errors
///
/// Cannot occur in practice; see [`disagreement_count`].
pub fn months_in_dispute() -> CalendarResult<usize> {
    disagreement_count(&ALL)
}

/// The months in which the current American and Japanese lists differ.
///
/// Both were revised recently, both are current, and they still disagree —
/// which is the fact that makes a single `birthstone(month)` function
/// impossible to write honestly.
pub fn us_and_japan_differ_in() -> impl Iterator<Item = Month> {
    (0..12u8).filter_map(|index| {
        let position = usize::from(index);
        let us = BIRTHSTONES_US_2016.at(position).ok()?;
        let jp = BIRTHSTONES_JP_2021.at(position).ok()?;
        (us != jp).then(|| Month::regular(index + 1))
    })
}

#[cfg(test)]
mod tests {
    use hc_calendar::CalendarError;

    use super::*;

    #[test]
    fn every_authority_has_an_entry_for_all_twelve_months() {
        for table in ALL {
            for ordinal in 1..=12u8 {
                let entry = stones(table, Month::regular(ordinal));
                assert!(
                    entry.is_ok(),
                    "{} has no entry for month {ordinal}",
                    table.authority().id
                );
            }
            assert_eq!(table.len(), 12, "{}", table.authority().id);
        }
    }

    #[test]
    fn no_authority_ships_an_empty_entry() {
        for table in ALL {
            assert!(
                table.is_complete(),
                "{} has an empty month",
                table.authority().id
            );
        }
    }

    #[test]
    fn every_authority_names_a_source() {
        for table in ALL {
            let authority = table.authority();
            assert!(!authority.source.is_empty(), "{}", authority.id);
            assert!(!authority.id.is_empty());
            assert!(!authority.english_name.is_empty());
        }
    }

    #[test]
    fn every_authority_id_is_unique() {
        for (position, table) in ALL.iter().enumerate() {
            for other in &ALL[position + 1..] {
                assert_ne!(table.authority().id, other.authority().id);
            }
        }
    }

    /// The point of the crate. If this ever passes, a list has been
    /// flattened and the crate has started lying.
    #[test]
    fn the_american_and_japanese_lists_of_the_2010s_and_2020s_genuinely_differ() {
        assert_ne!(
            BIRTHSTONES_US_2016.authority().id,
            BIRTHSTONES_JP_2021.authority().id
        );
        let differing: [u8; 8] = [2, 3, 4, 5, 7, 8, 9, 12];
        assert!(
            us_and_japan_differ_in().eq(differing.map(Month::regular)),
            "the two current lists should differ in exactly those eight months"
        );
        // Coral and jade are Japanese additions of 1958 and appear in no
        // American list at all.
        assert!(BIRTHSTONES_JP_2021.names("coral"));
        assert!(BIRTHSTONES_JP_2021.names("jade"));
        assert!(!BIRTHSTONES_US_2016.names("coral"));
        assert!(!BIRTHSTONES_US_2016.names("jade"));
    }

    /// January is garnet everywhere and always has been. Every other month
    /// is contested by somebody.
    #[test]
    fn january_is_the_only_month_all_six_authorities_agree_on() {
        assert_eq!(authorities_agree(Month::regular(1)), Ok(true));
        for ordinal in 2..=12u8 {
            assert_eq!(
                authorities_agree(Month::regular(ordinal)),
                Ok(false),
                "month {ordinal} unexpectedly unanimous"
            );
        }
        assert_eq!(months_in_dispute(), Ok(11));
        for table in ALL {
            assert_eq!(stones(table, Month::regular(1)), Ok(&["garnet"][..]));
        }
    }

    /// March is the example the module documentation leads with, so it is
    /// the one the tests pin down hardest.
    #[test]
    fn march_is_aquamarine_in_one_list_and_bloodstone_in_another() {
        let march = Month::regular(3);
        assert_eq!(
            stones(&BIRTHSTONES_US_1912, march),
            Ok(&["bloodstone", "aquamarine"][..])
        );
        assert_eq!(
            stones(&BIRTHSTONES_US_2016, march),
            Ok(&["aquamarine", "bloodstone"][..])
        );
        assert_eq!(
            stones(&BIRTHSTONES_TRADITIONAL, march),
            Ok(&["bloodstone", "jasper"][..])
        );
        assert_eq!(
            stones(&BIRTHSTONES_JP_1958, march),
            Ok(&["aquamarine", "coral"][..])
        );
    }

    #[test]
    fn the_1952_revision_moved_march_and_added_alexandrite_citrine_and_zircon() {
        // Each of these is absent from the 1912 list and present in the
        // modern one; that is what the 1952 revision did.
        for added in ["alexandrite", "citrine", "zircon"] {
            assert!(!BIRTHSTONES_US_1912.names(added), "{added}");
            assert!(BIRTHSTONES_US_2016.names(added), "{added}");
        }
        assert!(BIRTHSTONES_US_1912.names("lapis lazuli"));
        assert!(!BIRTHSTONES_US_2016.names("lapis lazuli"));
    }

    #[test]
    fn tanzanite_reached_december_in_2002_and_spinel_august_in_2016() {
        assert!(!BIRTHSTONES_US_1912.names("tanzanite"));
        assert!(!BIRTHSTONES_US_1912.names("spinel"));
        let december: [usize; 1] = [11];
        let august: [usize; 1] = [7];
        assert!(BIRTHSTONES_US_2016.keys_naming("tanzanite").eq(december));
        assert!(BIRTHSTONES_US_2016.keys_naming("spinel").eq(august));
    }

    /// The headline of the 2021 announcement: ten stones added, twenty-nine
    /// in total.
    #[test]
    fn the_japanese_revision_of_2021_added_exactly_ten_stones() {
        let added =
            BIRTHSTONES_JP_2021.total_attributions() - BIRTHSTONES_JP_1958.total_attributions();
        assert_eq!(added, 10);
        assert_eq!(BIRTHSTONES_JP_2021.total_attributions(), 29);
        for stone in [
            "chrysoberyl cat's-eye",
            "bloodstone",
            "iolite",
            "morganite",
            "alexandrite",
            "sphene",
            "spinel",
            "kunzite",
            "zircon",
            "tanzanite",
        ] {
            assert!(
                BIRTHSTONES_JP_2021.names(stone),
                "{stone} missing from 2021"
            );
            assert!(!BIRTHSTONES_JP_1958.names(stone), "{stone} present in 1958");
        }
    }

    #[test]
    fn the_2021_revision_only_added_and_never_removed() {
        for ordinal in 1..=12u8 {
            let month = Month::regular(ordinal);
            let old = stones(&BIRTHSTONES_JP_1958, month).unwrap();
            let new = stones(&BIRTHSTONES_JP_2021, month).unwrap();
            for stone in old {
                assert!(new.contains(stone), "{stone} lost from month {ordinal}");
            }
        }
    }

    #[test]
    fn the_british_list_names_four_stones_the_current_american_list_does_not() {
        for only_british in ["rock crystal", "chrysoprase", "carnelian", "lapis lazuli"] {
            assert!(BIRTHSTONES_UK.names(only_british), "{only_british}");
            assert!(!BIRTHSTONES_US_2016.names(only_british), "{only_british}");
        }
        // Three of them were never American. Lapis lazuli was, in 1912, but
        // for December, where Britain has it for September.
        for never_american in ["rock crystal", "chrysoprase", "carnelian"] {
            assert!(
                !BIRTHSTONES_US_1912.names(never_american),
                "{never_american}"
            );
        }
        assert!(
            stones(&BIRTHSTONES_US_1912, Month::regular(12))
                .is_ok_and(|december| december.contains(&"lapis lazuli"))
        );
        assert!(
            stones(&BIRTHSTONES_UK, Month::regular(9))
                .is_ok_and(|september| september.contains(&"lapis lazuli"))
        );
        assert!(
            stones(&BIRTHSTONES_US_1912, Month::regular(9))
                .is_ok_and(|september| !september.contains(&"lapis lazuli"))
        );
        // October is opal alone in Britain and opal with tourmaline in the US.
        assert_eq!(
            stones(&BIRTHSTONES_UK, Month::regular(10)),
            Ok(&["opal"][..])
        );
    }

    #[test]
    fn asking_every_authority_at_once_returns_one_row_per_authority() {
        let rows: usize = all_for_month(Month::regular(6)).unwrap().count();
        assert_eq!(rows, ALL.len());
        for (authority, entry) in all_for_month(Month::regular(6)).unwrap() {
            assert!(!entry.is_empty(), "{}", authority.id);
        }
    }

    #[test]
    fn a_leap_month_has_no_birthstone_in_any_list() {
        for table in ALL {
            assert_eq!(
                stones(table, Month::leap(3)),
                Err(CalendarError::UnsupportedField("leap month"))
            );
        }
        assert_eq!(
            authorities_agree(Month::leap(3)),
            Err(CalendarError::UnsupportedField("leap month"))
        );
    }

    #[test]
    fn a_month_outside_the_year_is_out_of_range_in_every_list() {
        for table in ALL {
            assert_eq!(
                stones(table, Month::regular(0)),
                Err(CalendarError::MonthOutOfRange)
            );
            assert_eq!(
                stones(table, Month::regular(13)),
                Err(CalendarError::MonthOutOfRange)
            );
        }
    }

    #[test]
    fn three_of_the_six_lists_are_superseded_and_three_are_current() {
        assert_eq!(current().count(), 3);
        for table in current() {
            assert!(table.authority().validity.is_current());
        }
        for superseded in [
            &BIRTHSTONES_TRADITIONAL,
            &BIRTHSTONES_US_1912,
            &BIRTHSTONES_JP_1958,
        ] {
            assert!(!superseded.authority().validity.is_current());
            assert!(superseded.authority().caveat.is_some());
        }
    }

    #[test]
    fn every_revision_date_is_no_earlier_than_its_establishment() {
        for table in ALL {
            let authority = table.authority();
            if let (Some(established), Some(revised)) = (authority.established, authority.revised) {
                assert!(revised.year >= established.year, "{}", authority.id);
            }
        }
    }

    #[test]
    fn the_validity_spans_place_each_list_in_the_years_it_was_current() {
        assert!(BIRTHSTONES_US_1912.authority().validity.contains(1930));
        assert!(!BIRTHSTONES_US_1912.authority().validity.contains(1960));
        assert!(BIRTHSTONES_JP_1958.authority().validity.contains(2000));
        assert!(!BIRTHSTONES_JP_1958.authority().validity.contains(2025));
        assert!(BIRTHSTONES_JP_2021.authority().validity.contains(2025));
    }

    #[test]
    fn every_stone_name_is_lowercase_so_two_lists_compare_equal() {
        for table in ALL {
            for entry in table.iter() {
                for name in entry {
                    assert!(
                        !name.chars().any(char::is_uppercase),
                        "{name} in {} is not lowercase",
                        table.authority().id
                    );
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
    tests: birthstone_table_tests,
    all: &ALL,
    lookup: by_id,
}
