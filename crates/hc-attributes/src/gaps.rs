//! What this crate deliberately does not ship, and why.
//!
//! A crate about disputed attributions has to be able to say "there is no
//! answer" as clearly as it says "here are six". `hc_almanac::rules` does it
//! with `AlmanacRule::Undetermined`, which makes `rule_applies` answer `None`
//! rather than `false`. This module does the same job for a whole table: a
//! [`Gap`] is a subject the crate was asked for, looked into, and declined to
//! ship, with the reason attached.
//!
//! Each of these is a list a caller might reasonably expect to find here,
//! and each is absent for a reason that is a fact about the sources rather
//! than about this crate's effort.
//!
//! ```
//! use hc_attributes::gaps::{ALL, GapReason};
//!
//! // Every gap says why, and the reason is a value, not prose to grep.
//! assert!(ALL.iter().any(|gap| gap.reason == GapReason::ModernInvention));
//! ```

/// Why a subject is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum GapReason {
    /// Several published lists exist and they disagree, with no body to
    /// choose between them.
    ///
    /// Different from the birthstone case: there, the disagreeing lists each
    /// have a named authority and a date, so each can be shipped under its
    /// own name. Here they do not.
    SourcesDisagreeWithNoAuthority,
    /// The most widely circulated list exists but its compiler has never
    /// published how it was made.
    MethodUnpublished,
    /// The tradition is real but is not keyed to a calendar unit this crate
    /// handles, so presenting it as one would misrepresent it.
    DifferentKey,
    /// The list circulates as ancient and is demonstrably modern, usually
    /// with a single identifiable author.
    ModernInvention,
    /// The underlying text exists but its translation is genuinely
    /// undetermined, so any table would be a choice of translator presented
    /// as a fact.
    TranslationUndetermined,
}

impl GapReason {
    /// A one-line description.
    #[must_use]
    pub const fn english_description(self) -> &'static str {
        match self {
            Self::SourcesDisagreeWithNoAuthority => {
                "several published lists disagree and none is authoritative"
            }
            Self::MethodUnpublished => "the compiler has never published the method",
            Self::DifferentKey => "the tradition is not keyed to this calendar unit",
            Self::ModernInvention => "a modern invention circulated as ancient",
            Self::TranslationUndetermined => "the underlying text's translation is undetermined",
        }
    }
}

/// A subject the crate declined to ship, and the reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gap {
    /// A stable identifier, lowercase and hyphenated.
    pub id: &'static str,
    /// What is missing.
    pub subject: &'static str,
    /// Why.
    pub reason: GapReason,
    /// The reasoning in full.
    pub explanation: &'static str,
    /// What was consulted before declining.
    pub sources: &'static str,
}

/// Japan's day-by-day 誕生花, covering all 366 days.
///
/// The tradition is real and current: NHK's ラジオ深夜便 broadcasts the day's
/// flower every morning and publishes an annual 誕生日の花カレンダー. It is
/// not shipped because there is no single list to ship. The Japanese
/// Wikipedia article on 誕生花 gives three or four *different* flowers for
/// most days, each cited to a different book — 瀧井 (1995), 植松 (1996),
/// NHKサービスセンター (2008), 中居 (2011), 全国花育活動推進協議会 — and the
/// disagreement is the normal case, not the exception.
///
/// NHK's own FAQ, moreover, says only that two horticulturalists chose
/// flowers suited to the Japanese seasons, and has never published the
/// method. Shipping one of the lists would give it an authority it does not
/// have; blending them would manufacture one outright.
///
/// Japan's twelve-month 誕生花 is in the same position: the Japanese article
/// records it as an idea some people advocate, modelled on the French
/// *fleur de naissance*, rather than as a settled list.
pub const JAPANESE_DAILY_BIRTH_FLOWERS: Gap = Gap {
    id: "japanese-daily-birth-flowers",
    subject: "Japan's day-by-day 誕生花 for all 366 days",
    reason: GapReason::MethodUnpublished,
    explanation: "At least four published Japanese lists assign different flowers to the same day, and \
         the most widely heard of them — NHK's ラジオ深夜便 — has never published how its \
         assignments were made. There is no authority to name, so there is no table to ship \
         under a name.",
    sources: "誕生花, Japanese Wikipedia, which cites 瀧井 (1995), 植松 (1996), \
              NHKサービスセンター (2008), 中居 (2011) and 全国花育活動推進協議会 side by side \
              for each day; NHK ラジオ深夜便 FAQ",
};

/// The "Ayurvedic" birthstone list.
///
/// Circulated widely by jewellery retailers as an ancient Indian
/// month-by-month list. No primary source for a month-keyed Ayurvedic list
/// could be found, the retail versions differ from one another, and none
/// cites a text.
pub const AYURVEDIC_BIRTHSTONES: Gap = Gap {
    id: "ayurvedic-birthstones",
    subject: "a month-by-month \"Ayurvedic\" birthstone list",
    reason: GapReason::SourcesDisagreeWithNoAuthority,
    explanation: "Retail versions disagree with one another and none cites a text. A list that exists \
         only in jewellery marketing is not a tradition this crate can attribute to anyone.",
    sources: "Surveyed retail birthstone charts; no primary source located",
};

/// The "Tibetan" birthstone list. As [`AYURVEDIC_BIRTHSTONES`].
pub const TIBETAN_BIRTHSTONES: Gap = Gap {
    id: "tibetan-birthstones",
    subject: "a month-by-month \"Tibetan\" birthstone list",
    reason: GapReason::SourcesDisagreeWithNoAuthority,
    explanation: "As with the Ayurvedic list: widely printed by retailers, mutually inconsistent, and \
         untraceable to any text.",
    sources: "Surveyed retail birthstone charts; no primary source located",
};

/// The Hindu gemstone system, which is real and is not a birthstone list.
///
/// Hinduism does associate gemstones with birth, but through the
/// *navaratna*, the nine gems of the *navagraha* — the celestial bodies —
/// not through the month of birth. Which stones are recommended follows from
/// an individual's chart, computed for the exact place and time of birth.
///
/// That is a different key, not a different list, and flattening it onto
/// twelve months would misrepresent the system rather than translate it.
/// The planetary half of it is visible in
/// [`crate::weekday_attributions::WEEKDAY_VASARA_SANSKRIT`], which names the
/// same nine bodies over the week.
pub const HINDU_BIRTHSTONES: Gap = Gap {
    id: "hindu-birthstones",
    subject: "Hindu gemstones as a month-by-month birthstone list",
    reason: GapReason::DifferentKey,
    explanation: "The navaratna are keyed to the navagraha and to an individual's chart, not to a birth \
         month. Presenting them as twelve monthly stones would be a mistranslation of the \
         system, not a localisation of it.",
    sources: "Harish Johari, The Healing Power of Gemstones (Destiny Books, 1986), pp. 15–34",
};

/// The twelve stones of Aaron's breastplate as a birthstone list.
///
/// Exodus 28:17–21 lists twelve stones, and Josephus in the first century
/// connected them with the twelve months and the twelve signs — which is the
/// origin story every birthstone chart repeats. The list itself, however,
/// cannot be settled: the Hebrew names are identified differently by almost
/// every translation, and Josephus gave two different lists himself.
///
/// A worked example of the scale of the problem: three standard modern
/// Japanese translations of the same twelve Hebrew words produce three
/// substantially different stone lists. Any table here would be a choice of
/// translator dressed up as a fact about the text.
pub const HEBREW_BREASTPLATE_STONES: Gap = Gap {
    id: "hebrew-breastplate-stones",
    subject: "the twelve stones of Aaron's breastplate as a month list",
    reason: GapReason::TranslationUndetermined,
    explanation: "The Hebrew gem names of Exodus 28:17–21 have no settled identifications; translations \
         differ widely and Josephus gave two lists himself. Kunz argues Josephus was describing \
         the Second Temple breastplate rather than the one Exodus describes. Modern birthstone \
         lists have, in any case, little to do with it.",
    sources: "George F. Kunz, The Curious Lore of Precious Stones (Lippincott, 1913), \
              pp. 275–306; Rupert Gleadow, The Origin of the Zodiac, pp. 130–131",
};

/// The "Celtic tree calendar", which is neither Celtic nor a calendar.
///
/// It is Robert Graves's invention, published in *The White Goddess* (1948)
/// and derived from his reading of the *Ogham* alphabet's tree names. Graves
/// presented it as a reconstruction; it has no attestation in any Celtic
/// source and is rejected by Celticists. It is nonetheless printed
/// everywhere as an ancient Celtic system, which is exactly the pattern this
/// crate exists to interrupt.
///
/// It is not shipped. If it ever were, it would carry
/// [`crate::authority::Provenance::ModernInvention`], be attributed to
/// Graves by name and dated 1948, and it would have thirteen entries rather
/// than twelve — which is itself a reason a twelve-entry month table is the
/// wrong shape for it.
pub const CELTIC_TREE_CALENDAR: Gap = Gap {
    id: "celtic-tree-calendar",
    subject: "the \"Celtic tree calendar\" of thirteen tree months",
    reason: GapReason::ModernInvention,
    explanation: "Robert Graves's construction in The White Goddess (1948), from his reading of the \
         Ogham tree alphabet. It is not a Celtic survival and has no attestation in any Celtic \
         source. It is also thirteen months, not twelve, so it is not even the shape of a \
         month table.",
    sources: "Robert Graves, The White Goddess (Faber, 1948)",
};

/// Every gap this crate records.
pub static ALL: [Gap; 6] = [
    JAPANESE_DAILY_BIRTH_FLOWERS,
    AYURVEDIC_BIRTHSTONES,
    TIBETAN_BIRTHSTONES,
    HINDU_BIRTHSTONES,
    HEBREW_BREASTPLATE_STONES,
    CELTIC_TREE_CALENDAR,
];

/// The gap with a given identifier, if there is one.
#[must_use]
pub fn by_id(id: &str) -> Option<&'static Gap> {
    ALL.iter().find(|gap| gap.id == id)
}

/// Every gap recorded for a given reason.
pub fn for_reason(reason: GapReason) -> impl Iterator<Item = &'static Gap> {
    ALL.iter().filter(move |gap| gap.reason == reason)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_gap_states_a_subject_an_explanation_and_its_sources() {
        for gap in &ALL {
            assert!(!gap.id.is_empty());
            assert!(!gap.subject.is_empty(), "{}", gap.id);
            assert!(!gap.explanation.is_empty(), "{}", gap.id);
            assert!(!gap.sources.is_empty(), "{}", gap.id);
            assert!(!gap.reason.english_description().is_empty());
        }
    }

    #[test]
    fn every_gap_id_is_unique_and_findable() {
        for (position, gap) in ALL.iter().enumerate() {
            assert_eq!(by_id(gap.id), Some(gap));
            for other in &ALL[position + 1..] {
                assert_ne!(gap.id, other.id);
            }
        }
        assert_eq!(by_id("no-such-gap"), None);
    }

    /// The Celtic tree calendar is the standing example of a modern
    /// invention presented as ancient, and the crate labels it as one.
    #[test]
    fn the_celtic_tree_calendar_is_recorded_as_graves_invention_of_1948() {
        assert_eq!(CELTIC_TREE_CALENDAR.reason, GapReason::ModernInvention);
        assert!(CELTIC_TREE_CALENDAR.sources.contains("Graves"));
        assert!(CELTIC_TREE_CALENDAR.sources.contains("1948"));
        assert!(
            CELTIC_TREE_CALENDAR
                .explanation
                .contains("not a Celtic survival")
        );
        assert_eq!(for_reason(GapReason::ModernInvention).count(), 1);
    }

    #[test]
    fn the_japanese_daily_flowers_are_a_gap_because_the_method_is_unpublished() {
        assert_eq!(
            JAPANESE_DAILY_BIRTH_FLOWERS.reason,
            GapReason::MethodUnpublished
        );
        assert!(JAPANESE_DAILY_BIRTH_FLOWERS.sources.contains("NHK"));
        assert_eq!(
            by_id("japanese-daily-birth-flowers"),
            Some(&JAPANESE_DAILY_BIRTH_FLOWERS)
        );
    }

    #[test]
    fn the_hindu_system_is_a_different_key_not_a_missing_list() {
        assert_eq!(HINDU_BIRTHSTONES.reason, GapReason::DifferentKey);
        assert!(HINDU_BIRTHSTONES.explanation.contains("navagraha"));
    }

    #[test]
    fn two_gaps_exist_because_retail_lists_disagree_with_no_authority() {
        assert_eq!(
            for_reason(GapReason::SourcesDisagreeWithNoAuthority).count(),
            2
        );
        for gap in for_reason(GapReason::SourcesDisagreeWithNoAuthority) {
            assert!(gap.sources.contains("no primary source located"));
        }
    }

    /// Nothing in `gaps` may quietly become a shipped table without the
    /// gap being removed: an id that is both a gap and an authority would
    /// mean the crate says two different things.
    #[test]
    fn no_gap_shares_an_identifier_with_a_shipped_authority() {
        for gap in &ALL {
            for table in crate::birthstones::ALL {
                assert_ne!(gap.id, table.authority().id);
            }
            for table in crate::birth_flowers::ALL {
                assert_ne!(gap.id, table.authority().id);
            }
            for table in crate::moon_names::ALL {
                assert_ne!(gap.id, table.authority().id);
            }
        }
    }
}

hc_core::catalogue_tests! {
    type: Gap,
    id: |gap| gap.id,
    provenance: |gap| gap.sources,
    tests: gap_table_tests,
    all: &ALL,
    lookup: by_id,
}
