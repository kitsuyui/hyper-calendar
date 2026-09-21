//! Stones by zodiac sign — an older and different system from the month
//! stones, and the one they are routinely confused with.
//!
//! The month lists of [`crate::birthstones`] are a twentieth-century trade
//! standardisation. The sign lists are the older article: Josephus, in the
//! first century, already connected the twelve stones of Aaron's breastplate
//! with the twelve signs, and the sign assignments passed through the
//! lapidaries into the early modern period. The month lists grew out of the
//! practice of *wearing* one stone a month, which is attested only from the
//! sixteenth century in Germany or the eighteenth in Poland depending on
//! which authority is asked.
//!
//! # The two systems disagree, and they disagree for a structural reason
//!
//! A sign is not a month. Aries runs from about 21 March to about 19 April,
//! so it straddles the March/April boundary and always will; no assignment
//! of stones to signs can be made to agree with an assignment of stones to
//! months except by coincidence. On top of that the two lists were compiled
//! for different purposes centuries apart, so the coincidences are not few
//! but none: Kunz's sign stones and the modern American month stones share
//! **no** stone in any corresponding position, all twelve of them.
//!
//! [`month_and_sign_stones_overlap`] measures it, and a test asserts the
//! number rather than this comment claiming it.
//!
//! ```
//! use hc_attributes::zodiac_stones::{ZODIAC_STONES_KUNZ, stones_for_sign};
//! use hc_seasons::TropicalSign;
//!
//! // Aries is bloodstone in the sign tradition. In the modern American
//! // month list bloodstone is March's — and Aries begins in March but is
//! // mostly April. The two are not the same statement.
//! assert_eq!(stones_for_sign(TropicalSign::ARIES), &["bloodstone"]);
//! assert_eq!(ZODIAC_STONES_KUNZ.authority().region.english_name, "unspecified");
//! ```
//!
//! # Which zodiac
//!
//! The *tropical* one, measured from the March equinox, because that is what
//! the Western lapidary tradition the list comes from meant. The sidereal
//! signs of Indian astrology are about 24° behind and are a different
//! division; `hc_seasons::zodiac` implements both, and this module
//! deliberately does not offer the table against the sidereal signs, because
//! Kunz's sources were not using them.
//!
//! The sign a day falls in comes from `hc-seasons`, so [`stones_on`] carries
//! that crate's accuracy: a sign boundary within about ten minutes of local
//! midnight can be assigned the wrong day.

use hc_calendar::Rd;
use hc_seasons::{Meridian, TropicalSign};

use crate::authority::{Authority, Provenance, Region, SignTable, Validity};

/// The stones assigned to the twelve tropical signs, as Kunz records them.
///
/// From *The Curious Lore of Precious Stones* (Lippincott, 1913),
/// pp. 345–347. Kunz is reporting the Western lapidary tradition, not
/// promulgating a list, which is why the [`Provenance`] is
/// [`Provenance::Recorded`] and the [`Region`] is
/// [`Region::UNSPECIFIED`]: no body adopted this and no country owns it.
///
/// Index 0 is Aries, matching [`TropicalSign::index`].
///
/// Chrysolite (Libra) is the old name for what is now usually sold as
/// peridot, and the list keeps the old name because that is the word the
/// sources use. Beryl (Scorpio) is the species; aquamarine and emerald are
/// both beryl, and the sources do not narrow it.
pub static ZODIAC_STONES_KUNZ: SignTable = SignTable::new(
    Authority {
        id: "zodiac-stones-kunz-1913",
        english_name: "zodiacal stones of the Western lapidary tradition",
        body: None,
        region: Region::UNSPECIFIED,
        established: None,
        revised: None,
        validity: Validity::UNKNOWN,
        provenance: Provenance::Recorded,
        source: "George F. Kunz, The Curious Lore of Precious Stones (Lippincott, 1913), \
                 pp. 345–347; sign dates per Bruce G. Knuth, Gems in Myth, Legend and Lore \
                 (rev. ed., Jewelers Press, 2007), p. 318",
        caveat: Some(
            "A sign is not a month. These are not the month birthstones and share no stone \
             with them in any corresponding position.",
        ),
    },
    [
        &["bloodstone"],
        &["sapphire"],
        &["agate"],
        &["emerald"],
        &["onyx"],
        &["carnelian"],
        &["chrysolite"],
        &["beryl"],
        &["topaz"],
        &["ruby"],
        &["garnet"],
        &["amethyst"],
    ],
);

/// Every zodiac-stone authority this crate ships.
///
/// One, for now. The array exists so that the shape matches
/// [`crate::birthstones::ALL`] and a second authority can be added without
/// changing any caller.
pub static ALL: [&SignTable; 1] = [&ZODIAC_STONES_KUNZ];

/// The stones one authority names for a sign.
///
/// Infallible: [`TropicalSign::index`] is always 0..=11 and the table always
/// has twelve entries, so the empty fallback is unreachable.
#[must_use]
pub fn stones_for_sign_in(table: &SignTable, sign: TropicalSign) -> &'static [&'static str] {
    table.at(usize::from(sign.index())).unwrap_or(&[])
}

/// The stones the Western lapidary tradition names for a sign.
#[must_use]
pub fn stones_for_sign(sign: TropicalSign) -> &'static [&'static str] {
    stones_for_sign_in(&ZODIAC_STONES_KUNZ, sign)
}

/// The stones for the sign a day falls in.
///
/// The sign comes from [`hc_seasons::zodiac::tropical::sign_on_day`], so a
/// day on which a sign boundary falls within about ten minutes of local
/// midnight can be given the neighbouring sign's stone. That is the solar
/// series' accuracy, not a property of the stone list.
#[must_use]
pub fn stones_on(day: Rd, meridian: Meridian) -> &'static [&'static str] {
    stones_for_sign(hc_seasons::zodiac::tropical::sign_on_day(day, meridian))
}

/// How many signs share at least one stone with the month that contains the
/// greater part of the sign, under the current American list.
///
/// Zero of twelve. The function exists so the disagreement between the two
/// systems is a number a caller can print rather than a claim in a comment,
/// and so that a later revision of either list is caught by a test rather
/// than by a reader.
///
/// The "greater part" mapping is the conventional one: Aries is April's
/// sign, Taurus May's, and so on round to Pisces, which is March's. That
/// mapping is itself an approximation — the signs move against the calendar
/// — and it is used here only to make the comparison possible at all.
#[must_use]
pub fn month_and_sign_stones_overlap() -> usize {
    TropicalSign::ALL
        .into_iter()
        .filter(|sign| {
            let sign_stones = stones_for_sign(*sign);
            // Aries (index 0) sits mostly in April (month index 3).
            let month_slot = (usize::from(sign.index()) + 3) % 12;
            let month_stones = crate::birthstones::BIRTHSTONES_US_2016
                .at(month_slot)
                .unwrap_or(&[]);
            sign_stones.iter().any(|stone| month_stones.contains(stone))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_sign_has_an_entry_and_none_is_empty() {
        for table in ALL {
            assert_eq!(table.len(), 12);
            assert!(table.is_complete(), "{}", table.authority().id);
        }
        for sign in TropicalSign::ALL {
            assert!(!stones_for_sign(sign).is_empty(), "{}", sign.english_name());
        }
    }

    #[test]
    fn the_table_is_indexed_the_way_hc_seasons_indexes_signs() {
        assert_eq!(stones_for_sign(TropicalSign::ARIES), &["bloodstone"]);
        assert_eq!(stones_for_sign(TropicalSign::PISCES), &["amethyst"]);
        assert_eq!(TropicalSign::ARIES.index(), 0);
        assert_eq!(TropicalSign::PISCES.index(), 11);
    }

    /// The whole reason this module is separate from [`crate::birthstones`].
    #[test]
    fn the_sign_stones_and_the_month_stones_disagree_where_they_disagree() {
        assert_eq!(month_and_sign_stones_overlap(), 0);

        // Aries is bloodstone; April, the month Aries mostly occupies, is
        // diamond. Cancer is emerald; July is ruby. Scorpio is beryl;
        // November is topaz and citrine.
        assert_eq!(stones_for_sign(TropicalSign::ARIES), &["bloodstone"]);
        assert_eq!(
            crate::birthstones::BIRTHSTONES_US_2016.at(3),
            Ok(&["diamond"][..])
        );
        assert_eq!(stones_for_sign(TropicalSign::CANCER), &["emerald"]);
        assert_eq!(
            crate::birthstones::BIRTHSTONES_US_2016.at(6),
            Ok(&["ruby"][..])
        );
    }

    /// Bloodstone is March's in the month list and Aries's in the sign
    /// list, and Aries begins in March. That near-miss is precisely the
    /// coincidence that makes people believe the two systems are one.
    #[test]
    fn bloodstone_belongs_to_march_and_to_aries_which_is_mostly_april() {
        assert!(crate::birthstones::BIRTHSTONES_US_2016.names("bloodstone"));
        let march: [usize; 1] = [2];
        assert!(
            crate::birthstones::BIRTHSTONES_US_2016
                .keys_naming("bloodstone")
                .eq(march)
        );
        assert_eq!(stones_for_sign(TropicalSign::ARIES), &["bloodstone"]);
        // Aries opens at the March equinox and runs a month, so most of it
        // is April.
        assert_eq!(
            TropicalSign::ARIES.opening_term().solar_longitude_degrees(),
            0.0
        );
    }

    #[test]
    fn the_authority_carries_a_caveat_because_the_confusion_is_the_norm() {
        let authority = ZODIAC_STONES_KUNZ.authority();
        assert!(authority.caveat.is_some());
        assert_eq!(authority.provenance, Provenance::Recorded);
        assert!(authority.source.contains("Kunz"));
    }

    /// RD 739 267 is 16 January 2025; the Sun is in Capricorn, whose stone
    /// is ruby. January's month stone is garnet in every list there is.
    #[test]
    fn a_day_in_mid_january_takes_capricorns_stone_not_januarys() {
        let day = Rd(739_267);
        let sign = hc_seasons::zodiac::tropical::sign_on_day(day, Meridian::UNIVERSAL);
        assert_eq!(sign, TropicalSign::CAPRICORN);
        assert_eq!(stones_on(day, Meridian::UNIVERSAL), &["ruby"]);
        assert_eq!(
            crate::birthstones::BIRTHSTONES_US_2016.at(0),
            Ok(&["garnet"][..])
        );
    }

    #[test]
    fn every_sign_of_a_year_is_reachable_by_walking_the_days() {
        // Twelve signs over 366 days: each must appear at least once, which
        // checks that the index mapping never falls out of range.
        let mut seen = [false; 12];
        for offset in 0..366i64 {
            let sign = hc_seasons::zodiac::tropical::sign_on_day(
                Rd(739_252 + offset),
                Meridian::UNIVERSAL,
            );
            seen[usize::from(sign.index())] = true;
            assert!(!stones_for_sign(sign).is_empty());
        }
        assert!(seen.into_iter().all(|hit| hit));
    }

    #[test]
    fn the_stone_names_are_lowercase_like_the_month_lists() {
        for entry in ZODIAC_STONES_KUNZ.iter() {
            for name in entry {
                assert!(!name.chars().any(char::is_uppercase), "{name}");
            }
        }
    }
}
