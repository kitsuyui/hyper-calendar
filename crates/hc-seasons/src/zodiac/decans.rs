//! The astrological decans, or faces: each tropical sign cut into three
//! parts of 10°, each ruled by a planet.
//!
//! The division, its source and what it is not are in
//! `docs/systems/solar-terms-and-pentads.md` in the repository, with the
//! sources keyed in `docs/references.bib`. This page states the code's own
//! facts.
//!
//! al-Bīrūnī gives the rule and the table: "each third of a sign — ten
//! degrees — is called a face (*wajh*)", and "the lord of the first face of
//! Aries is Mars, of the second the Sun, of the third Venus; of the first of
//! Taurus, Mercury, and so on in the order of the planets from above
//! downwards till the last face of Pisces" (*The Book of Instruction in the
//! Elements of the Art of Astrology*, tr. R. Ramsay Wright, 1934, §449,
//! `biruni-wright1934`, read 2026-09-27). The order "from above downwards"
//! is the Chaldean order, [`RulingPlanet::CHALDEAN_ORDER`], so the 36 faces
//! run through the seven planets from Mars at 0° of Aries, and a face's
//! ruler is arithmetic, not a table. The test holds all 36 to al-Bīrūnī's.
//!
//! Two other things share the name and are not these. The Hindu *drekkāṇa*,
//! the same thirds of a sign, takes the lords of the sign and of the fifth
//! and ninth signs from it (§451 of the same book), and is not carried.
//! Ptolemy's "face" (*Tetrabiblos* I.23) is a planet's aspect to the Sun or
//! Moon, not a 10° part. The Egyptian decans of the star clocks are groups
//! of stars and are out of scope.
//!
//! A face is a coordinate on the tropical ecliptic, so the Sun's face at an
//! instant is its apparent longitude divided by ten.
//! [`decan_at_moment`] is that, and its boundaries are as good as
//! [`super::tropical`]'s. Nothing here is an astrological claim.

use hc_astro::solar::solar_longitude;
use hc_calendar::fixed::Moment;
use hc_core::math::{floor, normalize_degrees};

use crate::zodiac::tropical::{RulingPlanet, TropicalSign};
use crate::zodiac::{SIGNS_PER_ZODIAC, degrees_into_arc};

/// How many degrees of ecliptic longitude one decan spans.
pub const DEGREES_PER_DECAN: f64 = 10.0;

/// How many decans divide a sign.
pub const DECANS_PER_SIGN: u8 = 3;

/// How many decans make up the zodiac.
pub const DECANS_PER_ZODIAC: usize = SIGNS_PER_ZODIAC * DECANS_PER_SIGN as usize;

/// Where in the Chaldean order the first face of Aries begins: Mars, the
/// third of Saturn, Jupiter, Mars, Sun, Venus, Mercury, Moon.
const FIRST_RULER: usize = 2;

/// One of the 36 decans of the tropical zodiac.
///
/// Ordering is by longitude from the March equinox: the first decan of
/// Aries is least, the third of Pisces greatest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decan(u8);

impl Decan {
    /// Every decan, from the first of Aries.
    pub const ALL: [Self; DECANS_PER_ZODIAC] = {
        let mut all = [Self(0); DECANS_PER_ZODIAC];
        let mut index = 0;
        while index < DECANS_PER_ZODIAC {
            all[index] = Self(index as u8);
            index += 1;
        }
        all
    };

    /// The decan with this index, 0 for the first of Aries to 35 for the
    /// third of Pisces.
    #[must_use]
    pub const fn from_index(index: u8) -> Option<Self> {
        if (index as usize) < DECANS_PER_ZODIAC {
            Some(Self(index))
        } else {
            None
        }
    }

    /// The `part`th decan of `sign`, counting 1 to 3.
    #[must_use]
    pub const fn new(sign: TropicalSign, part: u8) -> Option<Self> {
        if part < 1 || part > DECANS_PER_SIGN {
            return None;
        }
        Some(Self(sign.index() * DECANS_PER_SIGN + part - 1))
    }

    /// The decan containing an ecliptic longitude, reduced to 0°–360°.
    #[must_use]
    pub fn containing_degrees(longitude_degrees: f64) -> Self {
        let index = floor(normalize_degrees(longitude_degrees) / DEGREES_PER_DECAN) as i64;
        Self(index.clamp(0, DECANS_PER_ZODIAC as i64 - 1) as u8)
    }

    /// The index, 0 to 35.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// The sign the decan is a part of.
    #[must_use]
    pub const fn sign(self) -> TropicalSign {
        match TropicalSign::from_index(self.0 / DECANS_PER_SIGN) {
            Some(sign) => sign,
            None => TropicalSign::ARIES,
        }
    }

    /// Which part of its sign the decan is, 1 to 3.
    #[must_use]
    pub const fn part(self) -> u8 {
        self.0 % DECANS_PER_SIGN + 1
    }

    /// The longitude the decan begins at, in degrees.
    #[must_use]
    pub const fn start_longitude_degrees(self) -> f64 {
        self.0 as f64 * DEGREES_PER_DECAN
    }

    /// The planet that rules the decan: the Chaldean order from Mars at the
    /// first face of Aries (al-Bīrūnī, §449).
    #[must_use]
    pub const fn ruler(self) -> RulingPlanet {
        RulingPlanet::CHALDEAN_ORDER[(FIRST_RULER + self.0 as usize) % 7]
    }

    /// The next decan, 10° further along the ecliptic, wrapping at 360°.
    #[must_use]
    pub const fn next(self) -> Self {
        Self((self.0 + 1) % DECANS_PER_ZODIAC as u8)
    }
}

/// The decan the Sun is in at an instant, by its apparent longitude.
#[must_use]
pub fn decan_at_moment(moment: Moment) -> Decan {
    Decan::containing_degrees(solar_longitude(moment))
}

/// How far into its decan the Sun is at an instant, in degrees from 0 up to
/// but not including 10.
#[must_use]
pub fn degrees_into_decan(moment: Moment) -> f64 {
    let longitude = solar_longitude(moment);
    degrees_into_arc(
        longitude,
        Decan::containing_degrees(longitude).start_longitude_degrees(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zodiac::tropical::ingress_moment;

    /// al-Bīrūnī's table of the lords of the faces, §449, sign by sign.
    const BIRUNI: [[&str; 3]; 12] = [
        ["mars", "sun", "venus"],
        ["mercury", "moon", "saturn"],
        ["jupiter", "mars", "sun"],
        ["venus", "mercury", "moon"],
        ["saturn", "jupiter", "mars"],
        ["sun", "venus", "mercury"],
        ["moon", "saturn", "jupiter"],
        ["mars", "sun", "venus"],
        ["mercury", "moon", "saturn"],
        ["jupiter", "mars", "sun"],
        ["venus", "mercury", "moon"],
        ["saturn", "jupiter", "mars"],
    ];

    #[test]
    fn the_rulers_are_al_birunis_table() {
        for sign in TropicalSign::ALL {
            for part in 1..=DECANS_PER_SIGN {
                let decan = Decan::new(sign, part).unwrap();
                assert_eq!(
                    decan.ruler().id,
                    BIRUNI[usize::from(sign.index())][usize::from(part - 1)],
                    "{} {part}",
                    sign.english_name()
                );
                assert_eq!((decan.sign(), decan.part()), (sign, part));
            }
        }
        assert_eq!(Decan::new(TropicalSign::ARIES, 0), None);
        assert_eq!(Decan::new(TropicalSign::ARIES, 4), None);
    }

    #[test]
    fn thirty_six_decans_tile_the_ecliptic_at_ten_degrees() {
        for decan in Decan::ALL {
            let start = decan.start_longitude_degrees();
            assert_eq!(Decan::containing_degrees(start), decan);
            assert_eq!(Decan::containing_degrees(start + 9.999), decan);
            assert_eq!(Decan::containing_degrees(start + 10.0), decan.next());
            assert_eq!(Decan::from_index(decan.index()), Some(decan));
        }
        assert_eq!(Decan::from_index(36), None);
        assert_eq!(Decan::containing_degrees(-0.5).index(), 35);
        assert_eq!(Decan::containing_degrees(360.0).index(), 0);
        // Three decans to a sign, each inside it.
        for sign in TropicalSign::ALL {
            let first = Decan::new(sign, 1).unwrap();
            assert!(
                (first.start_longitude_degrees() - sign.start_longitude_degrees()).abs() < 1e-9
            );
        }
    }

    #[test]
    fn the_sun_enters_the_first_decan_of_each_sign_at_the_ingress() {
        // The ingresses are the principal solar terms, which the tropical
        // module anchors to the almanacs; half a day either side of each,
        // the Sun is in the last decan of the sign before and the first of
        // the sign itself.
        for sign in TropicalSign::ALL {
            let ingress = ingress_moment(2024, sign);
            assert_eq!(
                decan_at_moment(Moment(ingress.0 + 0.5)),
                Decan::new(sign, 1).unwrap()
            );
            assert_eq!(
                decan_at_moment(Moment(ingress.0 - 0.5)),
                Decan::new(sign.previous(), 3).unwrap()
            );
            let into = degrees_into_decan(Moment(ingress.0 + 0.5));
            assert!((0.0..1.0).contains(&into), "{into}");
        }
    }
}
