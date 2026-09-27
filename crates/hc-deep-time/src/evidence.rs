//! The earliest evidence of three firsts, as the literature dates it.
//!
//! A timeline wants three landmarks that no chronology in this crate
//! carries: the earliest evidence of life, the earliest *Homo sapiens* and
//! the earliest writing. None of them is a boundary anyone has ratified.
//! Each is a claim about one find — a rock, a fossil, a tablet — made in one
//! paper and dated by one method, and each is overtaken, qualified or
//! disputed by the next paper. So this module does not carry "the" date of
//! any of them. It carries the claims, one [`EarliestEvidence`] each, with
//! the shape of the date its source gives:
//!
//! * [`Dating::Age`] — an age, with its uncertainty where the source states
//!   one;
//! * [`Dating::AtLeast`] — a minimum age, where the source says "at least"
//!   or "by", and gives no older limit;
//! * [`Dating::Between`] — two ages, where the source gives a range.
//!
//! An uncertainty is carried only as a standard uncertainty, and only where
//! the source says what its `±` is. Vidal et al. print 233 ± 22 kyr "at
//! 2σ", which is a standard uncertainty of 11 kyr; Richter et al.'s
//! abstract prints 315 ± 34 ka without saying, and the full text was not
//! read, so that `±` is kept in the description and the standard
//! uncertainty is [`None`] rather than a guess. An age the source writes
//! "about", "ca." or "Myr-old" is [`EvidenceAge::approximate`].
//!
//! Where two claims compete for one landmark, or a claim is disputed, each
//! is its own entry under its own identifier ([policy §5]), with the
//! dispute named in [`EarliestEvidence::disputed_by`] and stated in the
//! description, and the entries are never merged into one number.
//!
//! The system document is `docs/systems/earliest-evidence.md`: why these
//! three, what each claim rests on, and what was read.
//!
//! # The datum
//!
//! Every age is in years before 1950, the BP datum of
//! [`crate::archaeology`], so that the entries sort with the archaeological
//! periods. The sources count from the year of measurement or from "now";
//! the difference is under a century and at least two decades below every
//! uncertainty here, which is the reasoning [`crate::timeline`] already
//! applies. The two writing entries are the exception that is exact: their
//! sources give calendar years BC, and a year *n* BC is 1949 + *n* years
//! before 1950.
//!
//! [policy §5]: https://github.com/kitsuyui/hyper-calendar/blob/main/docs/policy.md

/// The landmark every claim about the earliest evidence of life is for.
pub const EARLIEST_LIFE: &str = "earliest-life";

/// The landmark every claim about the earliest *Homo sapiens* is for.
pub const EARLIEST_HOMO_SAPIENS: &str = "earliest-homo-sapiens";

/// The landmark every claim about the earliest writing is for.
pub const EARLIEST_WRITING: &str = "earliest-writing";

/// The landmarks, in the order [`EVIDENCE`] lists them: oldest first.
pub const LANDMARKS: &[&str] = &[EARLIEST_LIFE, EARLIEST_HOMO_SAPIENS, EARLIEST_WRITING];

/// One age as a source prints it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EvidenceAge {
    /// The age, in years before 1950.
    pub years_before_1950: f64,
    /// The standard uncertainty, in years, where the source states what its
    /// `±` is; [`None`] where it gives none or does not say.
    pub std_dev_years: Option<f64>,
    /// How many significant figures the source prints.
    pub figures: u8,
    /// Whether the source writes the age as approximate: "about", "ca.",
    /// "~", or a round "Myr-old".
    pub approximate: bool,
}

impl EvidenceAge {
    /// An age with a stated standard uncertainty.
    #[must_use]
    pub const fn stated(years_before_1950: f64, std_dev_years: f64, figures: u8) -> Self {
        Self {
            years_before_1950,
            std_dev_years: Some(std_dev_years),
            figures,
            approximate: false,
        }
    }

    /// An age printed without a standard uncertainty: with no `±`, or with
    /// one whose confidence level the source read does not give.
    #[must_use]
    pub const fn printed(years_before_1950: f64, figures: u8) -> Self {
        Self {
            years_before_1950,
            std_dev_years: None,
            figures,
            approximate: false,
        }
    }

    /// An age the source writes as approximate.
    #[must_use]
    pub const fn about(years_before_1950: f64, figures: u8) -> Self {
        Self {
            years_before_1950,
            std_dev_years: None,
            figures,
            approximate: true,
        }
    }

    /// The age of a calendar year BC, exactly: *n* BC is 1949 + *n* years
    /// before 1950, because there is no year zero between 1 BC and AD 1.
    #[must_use]
    pub const fn about_year_bc(year_bc: u16, figures: u8) -> Self {
        // The widening is exact; `f64::from` is not const.
        Self::about(year_bc as f64 + 1949.0, figures)
    }
}

/// The shape of the date a source gives.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dating {
    /// One age for the evidence itself.
    Age(EvidenceAge),
    /// The evidence is at least this old; the source sets no older limit.
    AtLeast(EvidenceAge),
    /// The evidence lies between two ages.
    Between {
        /// The older limit.
        older: EvidenceAge,
        /// The younger limit.
        younger: EvidenceAge,
    },
}

impl Dating {
    /// The older limit, or [`None`] for a minimum age, which has none.
    #[must_use]
    pub const fn older(&self) -> Option<EvidenceAge> {
        match *self {
            Self::Age(age) => Some(age),
            Self::AtLeast(_) => None,
            Self::Between { older, .. } => Some(older),
        }
    }

    /// The younger limit: the age itself, the minimum, or the younger end of
    /// the range.
    #[must_use]
    pub const fn younger(&self) -> EvidenceAge {
        match *self {
            Self::Age(age) | Self::AtLeast(age) => age,
            Self::Between { younger, .. } => younger,
        }
    }
}

/// One published claim to be the earliest evidence of a landmark.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EarliestEvidence {
    /// A stable identifier, lower-case and hyphenated.
    pub id: &'static str,
    /// Which landmark the claim is for: one of [`LANDMARKS`].
    pub landmark: &'static str,
    /// The find, as the literature names it.
    pub name: &'static str,
    /// What was found, what dates it, and what is argued about it.
    pub description: &'static str,
    /// The paper the claim and its date come from.
    pub source: &'static str,
    /// The published rebuttal read for this library, where there is one.
    pub disputed_by: Option<&'static str>,
    /// The date, in the shape the source gives it.
    pub dating: Dating,
}

hc_core::catalogue! {
    type: EarliestEvidence,
    id: |entry| entry.id,
    provenance: |entry| entry.source,
    tests: evidence_catalogue,

    /// Every claim, grouped by landmark in the order of [`LANDMARKS`], and
    /// within a landmark by its younger limit, oldest first.
    pub const EVIDENCE;

    /// The claim with this identifier.
    pub fn by_id;

    entries: {
        /// Graphite inclusions in a 4.10 Ga Jack Hills zircon.
        pub const JACK_HILLS = EarliestEvidence {
            id: "earliest-life-jack-hills",
            landmark: EARLIEST_LIFE,
            name: "Jack Hills zircon graphite",
            description: "Graphite inclusions, delta13C -24 +/- 5 permil, sealed inside one \
                          concordant detrital zircon from the Jack Hills, Western Australia, \
                          dated 4.10 +/- 0.01 Ga; the abstract read does not say whether the \
                          +/- is one or two standard deviations, so no standard uncertainty \
                          is carried. The carbon is at least as old as the crystal that \
                          encloses it, hence a minimum age. The authors call it 'potentially \
                          biogenic': the isotopes are consistent with life, and no fossil is \
                          claimed.",
            source: "Bell, Boehnke, Harrison & Mao, PNAS 112, 14518 (2015), abstract",
            disputed_by: None,
            dating: Dating::AtLeast(EvidenceAge::printed(4.10e9, 3)),
        };

        /// Isotopically light carbon in apatite, Akilia island, Greenland.
        pub const AKILIA = EarliestEvidence {
            id: "earliest-life-akilia",
            landmark: EARLIEST_LIFE,
            name: "Akilia apatite carbon",
            description: "Isotopically light carbon inclusions in apatite from a banded iron \
                          formation on Akilia island, possibly older than 3850 Myr, and from \
                          Isua, about 3800 Myr, read as evidence of life 'by at least 3,800 \
                          Myr before present'. Disputed: Fedo & Whitehouse (2002) found the \
                          Akilia rock to be an altered ultramafic igneous rock rather than a \
                          sediment, which 'invalidates claims' that its graphite records life.",
            source: "Mojzsis, Arrhenius, McKeegan, Harrison, Nutman & Friend, Nature 384, 55 \
                     (1996), abstract",
            disputed_by: Some("Fedo & Whitehouse, Science 296, 1448 (2002), abstract"),
            dating: Dating::AtLeast(EvidenceAge::about(3.8e9, 2)),
        };

        /// Haematite tubes and filaments, Nuvvuagittuq belt, Quebec.
        pub const NUVVUAGITTUQ = EarliestEvidence {
            id: "earliest-life-nuvvuagittuq",
            landmark: EARLIEST_LIFE,
            name: "Nuvvuagittuq haematite filaments",
            description: "Micrometre-scale haematite tubes and filaments in hydrothermal-vent \
                          precipitates of the Nuvvuagittuq belt, Quebec, which the authors call \
                          'putative fossilized microorganisms' and date 'at least 3,770 million \
                          and possibly 4,280 million years old', carried as the range it is \
                          given in rather than as a mean. No rebuttal was read for this \
                          library.",
            source: "Dodd, Papineau, Grenne, Slack, Rittner, Pirajno, O'Neil & Little, Nature \
                     543, 60 (2017), abstract",
            disputed_by: None,
            dating: Dating::Between {
                older: EvidenceAge::printed(4.28e9, 3),
                younger: EvidenceAge::printed(3.77e9, 3),
            },
        };

        /// 13C-depleted graphite globules in Isua sea-floor sediments.
        pub const ISUA_GRAPHITE = EarliestEvidence {
            id: "earliest-life-isua-graphite",
            landmark: EARLIEST_LIFE,
            name: "Isua graphite globules",
            description: "Graphite globules of 2 to 5 micrometres, delta13C about -19 permil, \
                          in turbiditic and pelagic sediments of the Isua supracrustal belt, \
                          West Greenland, 'more than 3700 million years ago', read as biogenic \
                          detritus, perhaps from plankton. Isotopic evidence only; the 2016 \
                          Isua paper counts such signatures among the 'debated' ones.",
            source: "Rosing, Science 283, 674 (1999), abstract",
            disputed_by: None,
            dating: Dating::AtLeast(EvidenceAge::about(3.7e9, 2)),
        };

        /// Stromatolites in 3,700-Myr-old Isua metacarbonates.
        pub const ISUA_STROMATOLITES = EarliestEvidence {
            id: "earliest-life-isua-stromatolites",
            landmark: EARLIEST_LIFE,
            name: "Isua stromatolites",
            description: "Conical and domical structures 1 to 4 cm high in 3,700-Myr-old \
                          metacarbonate rocks newly exposed in the Isua supracrustal belt, \
                          Greenland, read as stromatolites grown by microbial mats in shallow \
                          sea water, 220 Myr older than the Dresser Formation. Disputed: \
                          Allwood et al. (2018) re-examined the outcrop in three dimensions and \
                          read the structures as deformation features formed long after burial, \
                          not as life. The age is that of the rocks, which neither side \
                          contests.",
            source: "Nutman, Bennett, Friend, Van Kranendonk & Chivas, Nature 537, 535 (2016), \
                     abstract",
            disputed_by: Some("Allwood, Rosing, Flannery, Hurowitz & Heirwegh, Nature 563, 241 \
                               (2018), abstract"),
            dating: Dating::Age(EvidenceAge::about(3.7e9, 2)),
        };

        /// Stromatolites, microfossils and hot-spring deposits, Dresser
        /// Formation, Pilbara.
        pub const DRESSER = EarliestEvidence {
            id: "earliest-life-dresser",
            landmark: EARLIEST_LIFE,
            name: "Dresser Formation",
            description: "Stromatolites, fractionated sulfur and carbon isotopes and \
                          microfossils in the ca. 3.48 Ga Dresser Formation of the Pilbara \
                          Craton, Western Australia, and hot-spring deposits there that carry \
                          microbial textures, 'the earliest life on land'. The claim the \
                          Isua paper of 2016 called 'the previous most convincing and \
                          generally accepted' evidence for the oldest life.",
            source: "Djokic, Van Kranendonk, Campbell, Walter & Ward, Nat. Commun. 8, 15263 \
                     (2017); Nutman et al., Nature 537, 535 (2016), abstract",
            disputed_by: None,
            dating: Dating::Age(EvidenceAge::about(3.48e9, 3)),
        };

        /// Filamentous microfossils of the Apex chert.
        pub const APEX_CHERT = EarliestEvidence {
            id: "earliest-life-apex-chert",
            landmark: EARLIEST_LIFE,
            name: "Apex chert microfossils",
            description: "Eleven taxa of cellularly preserved filamentous microbes described \
                          from a bedded chert of the Apex Basalt, Western Australia, 'at least \
                          as early as approximately 3465 million years ago'. Disputed: \
                          Brasier et al. (2002) reinterpreted the structures as artefacts of \
                          amorphous graphite in hydrothermal vein chert, with 'no support for \
                          primary biological morphology'.",
            source: "Schopf, Science 260, 640 (1993), abstract",
            disputed_by: Some("Brasier, Green, Jephcoat, Kleppe, Van Kranendonk, Lindsay, \
                               Steele & Grassineau, Nature 416, 76 (2002), abstract"),
            dating: Dating::AtLeast(EvidenceAge::about(3.465e9, 4)),
        };

        /// The stromatolite reef of the Strelley Pool Chert.
        pub const STRELLEY_POOL = EarliestEvidence {
            id: "earliest-life-strelley-pool",
            landmark: EARLIEST_LIFE,
            name: "Strelley Pool stromatolites",
            description: "Seven stromatolite morphotypes across a peritidal carbonate platform in \
                          the 3,430-million-year-old Strelley Pool Chert, Pilbara, Western \
                          Australia, whose diversity and environmental setting the authors \
                          argue reflect organisms, refuting the abiogenic hypotheses of the \
                          time. 'Probable biological origin' is theirs.",
            source: "Allwood, Walter, Kamber, Marshall & Burch, Nature 441, 714 (2006), abstract",
            disputed_by: None,
            dating: Dating::Age(EvidenceAge::about(3.43e9, 3)),
        };

        /// The Middle Stone Age hominins of Jebel Irhoud, Morocco.
        pub const JEBEL_IRHOUD = EarliestEvidence {
            id: "earliest-homo-sapiens-jebel-irhoud",
            landmark: EARLIEST_HOMO_SAPIENS,
            name: "Jebel Irhoud hominins",
            description: "Human fossils from Jebel Irhoud, Morocco, whose face, mandible and \
                          teeth align with early or recent modern humans and whose neurocranium \
                          is more primitive: 'early stages of the H. sapiens clade', in Hublin \
                          et al.'s words, rather than fully modern humans. Thermoluminescence of heated flints in the same \
                          layer gives a weighted average of 315 +/- 34 ka, supported by a \
                          U-series and ESR age of 286 +/- 32 ka for the Irhoud 3 tooth; the \
                          abstract read does not say what the +/- is, so no standard \
                          uncertainty is carried.",
            source: "Richter et al., Nature 546, 293 (2017), abstract, for the date; Hublin et \
                     al., Nature 546, 289 (2017), abstract, for the fossils",
            disputed_by: None,
            dating: Dating::Age(EvidenceAge::printed(315_000.0, 3)),
        };

        /// Omo I, Kibish Formation, Ethiopia.
        pub const OMO_KIBISH = EarliestEvidence {
            id: "earliest-homo-sapiens-omo-kibish",
            landmark: EARLIEST_HOMO_SAPIENS,
            name: "Omo I, Omo-Kibish",
            description: "The Omo I remains from the Kibish Formation, Ethiopia, with Herto \
                          'the oldest modern human fossils in eastern Africa'. The KHS Tuff that overlies them \
                          is matched to the Qi2 eruption of Shala volcano, whose 40Ar/39Ar age \
                          of 233 +/- 22 kyr at 2 sigma is a minimum age for the fossils; the \
                          standard uncertainty is half of that. It replaces the age of \
                          about 197 kyr generally reported before.",
            source: "Vidal et al., Nature 601, 579 (2022)",
            disputed_by: None,
            dating: Dating::AtLeast(EvidenceAge::stated(233_000.0, 11_000.0, 3)),
        };

        /// The inscribed labels and pots of tomb U-j, Abydos.
        pub const ABYDOS_U_J = EarliestEvidence {
            id: "earliest-writing-abydos-u-j",
            landmark: EARLIEST_WRITING,
            name: "Tomb U-j, Abydos",
            description: "Bone and ivory labels and inscribed pottery from tomb U-j of Cemetery U \
                          at Umm el-Qaab, Abydos, 'so far the earliest evidence of hieroglyphic \
                          writing from Egypt'. The tomb is of phase Naqada IIIa2, which \
                          radiocarbon on the wood of its tombs, wiggle-matched to the archaeological \
                          sequence, places in 'the middle of the 34th century BC'; the two samples \
                          from U-j itself calibrate to ranges between 3490 and 3030 cal BC as \
                          the paper lists them. \
                          A separate reading of the earliest writing from Uruk IV, with which it \
                          is not merged.",
            source: "Goersdorf, Dreyer & Hartung, Radiocarbon 40(2), 641 (1998)",
            disputed_by: None,
            dating: Dating::Age(EvidenceAge::about_year_bc(3350, 2)),
        };

        /// The proto-cuneiform tablets of Uruk IV.
        pub const URUK_IV = EarliestEvidence {
            id: "earliest-writing-uruk-iv",
            landmark: EARLIEST_WRITING,
            name: "Proto-cuneiform, Uruk IV",
            description: "The tablets of writing phase Uruk IV, which 'derive without apparent \
                          exception from Uruk', the earliest proto-cuneiform, after the clay \
                          bullae and numerical tablets that preceded them; Englund dates the emergence \
                          of proto-cuneiform 'ca. 3300 BC' and the following Uruk III tablets \
                          ca. 3100-3000 BC. A historian's date from the archaeological sequence, \
                          not a measurement. A separate reading of the earliest writing from \
                          Abydos, with which it is not merged.",
            source: "Englund, 'Proto-Cuneiform Account-Books and Journals', in Hudson & Wunsch \
                     (eds.), Creating Economic Order (CDL Press, 2004), pp. 23-46",
            disputed_by: None,
            dating: Dating::Age(EvidenceAge::about_year_bc(3300, 2)),
        };
    }
}

/// Every claim for one landmark, oldest first.
pub fn for_landmark(landmark: &str) -> impl Iterator<Item = &'static EarliestEvidence> + '_ {
    EVIDENCE
        .iter()
        .filter(move |entry| entry.landmark == landmark)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_claim_is_for_a_landmark() {
        for entry in EVIDENCE {
            assert!(LANDMARKS.contains(&entry.landmark), "{}", entry.id);
            assert!(
                entry.id.starts_with(entry.landmark),
                "{} does not name its landmark",
                entry.id
            );
        }
        for landmark in LANDMARKS {
            assert!(for_landmark(landmark).count() >= 2, "{landmark}");
        }
    }

    #[test]
    fn the_table_is_grouped_by_landmark_and_oldest_first() {
        let order = |entry: &EarliestEvidence| {
            LANDMARKS
                .iter()
                .position(|landmark| *landmark == entry.landmark)
        };
        for pair in EVIDENCE.windows(2) {
            let (first, second) = (order(&pair[0]), order(&pair[1]));
            assert!(first <= second, "{} before {}", pair[0].id, pair[1].id);
            if first == second {
                assert!(
                    pair[0].dating.younger().years_before_1950
                        >= pair[1].dating.younger().years_before_1950,
                    "{} is younger than {}",
                    pair[0].id,
                    pair[1].id
                );
            }
        }
    }

    #[test]
    fn a_range_runs_from_older_to_younger() {
        for entry in EVIDENCE {
            if let Some(older) = entry.dating.older() {
                assert!(
                    older.years_before_1950 >= entry.dating.younger().years_before_1950,
                    "{}",
                    entry.id
                );
            }
        }
    }

    #[test]
    fn no_uncertainty_is_invented() {
        // Only Vidal et al. say what their +/- is; every other age carries
        // none, and the approximate ones say so.
        let mut stated = EVIDENCE
            .iter()
            .filter(|entry| entry.dating.younger().std_dev_years.is_some());
        assert_eq!(stated.next().map(|entry| entry.id), Some(OMO_KIBISH.id));
        assert!(stated.next().is_none());
        for entry in EVIDENCE {
            for age in [entry.dating.older(), Some(entry.dating.younger())]
                .into_iter()
                .flatten()
            {
                assert!(age.std_dev_years.is_none_or(|sigma| sigma > 0.0));
                assert!(age.years_before_1950 > 0.0 && age.figures >= 2);
            }
        }
    }

    #[test]
    fn a_disputed_claim_says_so_in_its_description() {
        for entry in EVIDENCE {
            let says = entry.description.contains("Disputed");
            assert_eq!(says, entry.disputed_by.is_some(), "{}", entry.id);
        }
    }

    #[test]
    fn the_published_anchors_are_carried() {
        // Vidal et al. 2022: a minimum of 233 +/- 22 kyr at 2 sigma.
        assert_eq!(
            OMO_KIBISH.dating,
            Dating::AtLeast(EvidenceAge::stated(233_000.0, 11_000.0, 3))
        );
        assert!(OMO_KIBISH.dating.older().is_none());
        // Richter et al. 2017: 315 +/- 34 ka, carried without a sigma.
        assert_eq!(
            JEBEL_IRHOUD.dating.younger(),
            EvidenceAge::printed(315_000.0, 3)
        );
        // Dodd et al. 2017: at least 3770 and possibly 4280 Myr.
        assert_eq!(
            NUVVUAGITTUQ.dating.older().map(|age| age.years_before_1950),
            Some(4.28e9)
        );
        assert!((NUVVUAGITTUQ.dating.younger().years_before_1950 - 3.77e9).abs() < 1.0);
        // Nutman et al. 2016: 3,700 Myr, 220 Myr before the Dresser's 3,480.
        let gap = ISUA_STROMATOLITES.dating.younger().years_before_1950
            - DRESSER.dating.younger().years_before_1950;
        assert!((gap - 2.2e8).abs() < 1.0, "{gap}");
    }

    #[test]
    fn the_claims_are_placed_against_the_round_numbered_periods() {
        // Uruk IV's ca. 3300 BC is 5249 BP, a year inside this library's
        // round-numbered Bronze Age; Jebel Irhoud's 315 ka is inside its
        // Lower Palaeolithic, whose end at 300 ka is as round.
        let place = |entry: &EarliestEvidence| {
            crate::timeline::place_years_ago(entry.dating.younger().years_before_1950, 0.0)
                .ok()
                .and_then(|placement| placement.archaeological)
                .map(|period| period.id)
        };
        assert_eq!(place(&URUK_IV), Some("bronze-age"));
        assert_eq!(place(&ABYDOS_U_J), Some("chalcolithic"));
        assert_eq!(place(&JEBEL_IRHOUD), Some("lower-palaeolithic"));
        assert_eq!(place(&OMO_KIBISH), Some("middle-palaeolithic"));
        assert_eq!(place(&DRESSER), None, "before the first stone tools");
    }

    #[test]
    fn a_year_bc_is_counted_across_the_missing_year_zero() {
        // 1 BC is 1950 years before 1950; 3300 BC is 5249.
        assert!((EvidenceAge::about_year_bc(1, 1).years_before_1950 - 1950.0).abs() < 1e-9);
        assert!((URUK_IV.dating.younger().years_before_1950 - 5249.0).abs() < 1e-9);
        assert!((ABYDOS_U_J.dating.younger().years_before_1950 - 5299.0).abs() < 1e-9);
        assert!(URUK_IV.dating.younger().approximate);
    }
}
