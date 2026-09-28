//! The Hindu *drekkāṇa*: each sidereal sign cut into three parts of 10°,
//! each ruled by the lord of a sign of the same triplicity.
//!
//! The division, its source and how it differs from the faces are in
//! `docs/systems/solar-terms-and-pentads.md` in the repository, with the
//! sources keyed in `docs/references.bib`. This page states the code's own
//! facts.
//!
//! al-Bīrūnī gives the rule and a table: the Hindus' thirds of a sign have
//! lords "different from those of the faces, because the first decanate
//! has as lord the lord of the whole sign, the second, the lord of the
//! fifth sign from it, and the third, the lord of the ninth sign" (*The
//! Book of Instruction in the Elements of the Art of Astrology*, tr.
//! R. Ramsay Wright, 1934, §451, `biruni-wright1934`, read in the Internet
//! Archive's text 2026-09-29). Counted inclusively, the fifth sign from
//! Meṣa is Siṃha and the ninth Dhanus, so the three lords are those of one
//! triplicity, as his note says. The lord of a sign is
//! [`SiderealSign::ruling_planet`], the classical scheme; the test holds
//! all 36 lords to his table.
//!
//! The thirds are the same arcs as [`super::decans`]'s faces, but of the
//! sidereal signs, since the rāśi of Indian astronomy are sidereal; a
//! drekkāṇa at an instant therefore depends on the ayanāṃśa, as the sign
//! does. Nothing here is an astrological claim.

use hc_calendar::fixed::Moment;
use hc_core::math::{floor, normalize_degrees};

use crate::zodiac::sidereal::{Ayanamsa, SiderealSign, sidereal_longitude};
use crate::zodiac::tropical::RulingPlanet;
use crate::zodiac::{SIGNS_PER_ZODIAC, degrees_into_arc};

/// How many degrees of ecliptic longitude one drekkāṇa spans.
pub const DEGREES_PER_DREKKANA: f64 = 10.0;

/// How many drekkāṇas divide a sign.
pub const DREKKANAS_PER_SIGN: u8 = 3;

/// How many drekkāṇas make up the zodiac.
pub const DREKKANAS_PER_ZODIAC: usize = SIGNS_PER_ZODIAC * DREKKANAS_PER_SIGN as usize;

/// How many signs on from its own each third takes its lord from, counted
/// from zero: the sign itself, the fifth from it and the ninth, inclusively
/// (al-Bīrūnī, §451).
pub const LORD_OFFSETS: [u8; 3] = [0, 4, 8];

/// One of the 36 drekkāṇas of the sidereal zodiac.
///
/// Ordering is by sidereal longitude: the first drekkāṇa of Meṣa is least,
/// the third of Mīna greatest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Drekkana(u8);

impl Drekkana {
    /// Every drekkāṇa, from the first of Meṣa.
    pub const ALL: [Self; DREKKANAS_PER_ZODIAC] = {
        let mut all = [Self(0); DREKKANAS_PER_ZODIAC];
        let mut index = 0;
        while index < DREKKANAS_PER_ZODIAC {
            all[index] = Self(index as u8);
            index += 1;
        }
        all
    };

    /// The drekkāṇa with this index, 0 for the first of Meṣa to 35 for the
    /// third of Mīna.
    #[must_use]
    pub const fn from_index(index: u8) -> Option<Self> {
        if (index as usize) < DREKKANAS_PER_ZODIAC {
            Some(Self(index))
        } else {
            None
        }
    }

    /// The `part`th drekkāṇa of `sign`, counting 1 to 3.
    #[must_use]
    pub const fn new(sign: SiderealSign, part: u8) -> Option<Self> {
        if part < 1 || part > DREKKANAS_PER_SIGN {
            return None;
        }
        Some(Self(sign.index() * DREKKANAS_PER_SIGN + part - 1))
    }

    /// The drekkāṇa containing a sidereal longitude, reduced to 0°–360°.
    #[must_use]
    pub fn containing_degrees(sidereal_longitude_degrees: f64) -> Self {
        let index =
            floor(normalize_degrees(sidereal_longitude_degrees) / DEGREES_PER_DREKKANA) as i64;
        Self(index.clamp(0, DREKKANAS_PER_ZODIAC as i64 - 1) as u8)
    }

    /// The index, 0 to 35.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// The sign the drekkāṇa is a part of.
    #[must_use]
    pub const fn sign(self) -> SiderealSign {
        match SiderealSign::from_index(self.0 / DREKKANAS_PER_SIGN) {
            Some(sign) => sign,
            None => SiderealSign::MESHA,
        }
    }

    /// Which part of its sign the drekkāṇa is, 1 to 3.
    #[must_use]
    pub const fn part(self) -> u8 {
        self.0 % DREKKANAS_PER_SIGN + 1
    }

    /// The sidereal longitude the drekkāṇa begins at, in degrees.
    #[must_use]
    pub const fn start_longitude_degrees(self) -> f64 {
        self.0 as f64 * DEGREES_PER_DREKKANA
    }

    /// The sign whose lord rules the drekkāṇa: the sign itself, the fifth
    /// from it or the ninth (al-Bīrūnī, §451).
    #[must_use]
    pub const fn lord_sign(self) -> SiderealSign {
        let sign = self.sign().index();
        let offset = LORD_OFFSETS[(self.0 % DREKKANAS_PER_SIGN) as usize];
        match SiderealSign::from_index((sign + offset) % SIGNS_PER_ZODIAC as u8) {
            Some(sign) => sign,
            None => SiderealSign::MESHA,
        }
    }

    /// The planet that rules the drekkāṇa: the lord of [`Self::lord_sign`].
    #[must_use]
    pub const fn lord(self) -> RulingPlanet {
        self.lord_sign().ruling_planet()
    }

    /// The next drekkāṇa, 10° further along the ecliptic, wrapping at 360°.
    #[must_use]
    pub const fn next(self) -> Self {
        Self((self.0 + 1) % DREKKANAS_PER_ZODIAC as u8)
    }
}

/// The drekkāṇa the Sun is in at an instant, by its sidereal longitude
/// under an ayanāṃśa.
#[must_use]
pub fn drekkana_at_moment(moment: Moment, ayanamsa: Ayanamsa) -> Drekkana {
    Drekkana::containing_degrees(sidereal_longitude(moment, ayanamsa))
}

/// How far into its drekkāṇa the Sun is at an instant, in degrees from 0 up
/// to but not including 10.
#[must_use]
pub fn degrees_into_drekkana(moment: Moment, ayanamsa: Ayanamsa) -> f64 {
    let longitude = sidereal_longitude(moment, ayanamsa);
    degrees_into_arc(
        longitude,
        Drekkana::containing_degrees(longitude).start_longitude_degrees(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zodiac::sidereal::ingress_moment;

    /// al-Bīrūnī's table of the lords "of *darījān*", §451, sign by sign
    /// from Aries: the three columns of 10°, 20° and 30°.
    const BIRUNI: [[&str; 3]; 12] = [
        ["mars", "sun", "jupiter"],
        ["venus", "mercury", "saturn"],
        ["mercury", "venus", "saturn"],
        ["moon", "mars", "jupiter"],
        ["sun", "jupiter", "mars"],
        ["mercury", "saturn", "venus"],
        ["venus", "saturn", "mercury"],
        ["mars", "jupiter", "moon"],
        ["jupiter", "mars", "sun"],
        ["saturn", "venus", "mercury"],
        ["saturn", "mercury", "venus"],
        ["jupiter", "moon", "mars"],
    ];

    #[test]
    fn the_lords_are_al_birunis_table() {
        for sign in SiderealSign::ALL {
            for part in 1..=DREKKANAS_PER_SIGN {
                let third = Drekkana::new(sign, part).unwrap();
                assert_eq!(
                    third.lord().id,
                    BIRUNI[usize::from(sign.index())][usize::from(part - 1)],
                    "{} {part}",
                    sign.id()
                );
                assert_eq!((third.sign(), third.part()), (sign, part));
            }
        }
        assert_eq!(Drekkana::new(SiderealSign::MESHA, 0), None);
        assert_eq!(Drekkana::new(SiderealSign::MESHA, 4), None);
    }

    #[test]
    fn the_first_third_of_every_sign_is_its_own_lords() {
        // "The first decanate has as lord the lord of the whole sign", and
        // the three of a sign take the lords of one triplicity: Meṣa,
        // Siṃha, Dhanus for the first of them.
        for sign in SiderealSign::ALL {
            let first = Drekkana::new(sign, 1).unwrap();
            assert_eq!(first.lord_sign(), sign);
            assert_eq!(first.lord(), sign.ruling_planet());
        }
        let mesha = SiderealSign::MESHA;
        let signs: [&str; 3] =
            [1, 2, 3].map(|part| Drekkana::new(mesha, part).unwrap().lord_sign().id());
        assert_eq!(signs, ["mesha", "simha", "dhanus"]);
    }

    #[test]
    fn thirty_six_thirds_tile_the_ecliptic_at_ten_degrees() {
        for third in Drekkana::ALL {
            let start = third.start_longitude_degrees();
            assert_eq!(Drekkana::containing_degrees(start), third);
            assert_eq!(Drekkana::containing_degrees(start + 9.999), third);
            assert_eq!(Drekkana::containing_degrees(start + 10.0), third.next());
            assert_eq!(Drekkana::from_index(third.index()), Some(third));
        }
        assert_eq!(Drekkana::from_index(36), None);
        assert_eq!(Drekkana::containing_degrees(-0.5).index(), 35);
    }

    #[test]
    fn the_sun_enters_the_first_third_of_each_sign_at_the_sankranti() {
        for sign in SiderealSign::ALL {
            let ingress = ingress_moment(2024, sign, Ayanamsa::LAHIRI);
            assert_eq!(
                drekkana_at_moment(Moment(ingress.0 + 0.5), Ayanamsa::LAHIRI),
                Drekkana::new(sign, 1).unwrap()
            );
            assert_eq!(
                drekkana_at_moment(Moment(ingress.0 - 0.5), Ayanamsa::LAHIRI),
                Drekkana::new(sign.previous(), 3).unwrap()
            );
            let into = degrees_into_drekkana(Moment(ingress.0 + 0.5), Ayanamsa::LAHIRI);
            assert!((0.0..1.0).contains(&into), "{into}");
        }
    }
}
