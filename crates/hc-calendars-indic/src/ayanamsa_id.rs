//! The identifier of a calendar over an ayanāṃśa.
//!
//! `docs/policy.md` §5 gives a convention a name, and the sidereal zero
//! point is one: the Raman reading of the amānta calendar is not
//! `hindu-lunar`, which is the Lahiri one the *Rashtriya Panchang* keeps. So
//! a calendar built over another named ayanāṃśa reports an identifier that
//! says so, `hindu-lunar-raman`, and one built over an anchor this crate
//! does not name, `hindu-lunar-other-ayanamsa`. The place is a parameter and
//! does not change the identifier, as it does not for `hindu-lunar` itself.
//!
//! The identifiers are not registered: the registry lists the calendars the
//! crate names, over [`Ayanamsa::LAHIRI`] at a place, and the ones a caller
//! builds carry the identifier of the convention they read.
//! [`tests::every_named_ayanamsa_has_its_identifier`] holds this table to
//! `Ayanamsa::ALL`, so an ayanāṃśa added there cannot be left without one.

use hc_seasons::zodiac::Ayanamsa;

/// Whether two ayanāṃśas are the same anchor.
pub(crate) const fn same(one: Ayanamsa, other: Ayanamsa) -> bool {
    let (one, other) = (one.key(), other.key());
    one[0] == other[0] && one[1] == other[1]
}

/// A `const fn` giving the identifier and English name of a calendar over
/// an ayanāṃśa: `$id` and `$english` for Lahiri's, `$id-suffix` and
/// `$english, label ayanamsa` for each other named one, and
/// `$id-other-ayanamsa` for any anchor not named.
macro_rules! by_ayanamsa {
    ($(#[$meta:meta])* $vis:vis fn $name:ident, $id:literal, $english:literal) => {
        $(#[$meta])*
        $vis const fn $name(
            ayanamsa: hc_seasons::zodiac::Ayanamsa,
        ) -> (hc_calendar::CalendarId, &'static str) {
            use $crate::ayanamsa_id::same;
            use hc_seasons::zodiac::Ayanamsa;
            if same(ayanamsa, Ayanamsa::LAHIRI) {
                return (hc_calendar::CalendarId($id), $english);
            }
            $crate::ayanamsa_id::by_ayanamsa!(@arms ayanamsa, $id, $english;
                LAHIRI_RASHTRIYA "lahiri-rashtriya" "Lahiri of the Rashtriya Panchang";
                LAHIRI_CRC_1955 "lahiri-crc-1955" "Lahiri of the Calendar Reform Committee";
                LAHIRI_DRIK "lahiri-drik" "Lahiri of Drik Panchang";
                RAMAN "raman" "Raman";
                KRISHNAMURTI "krishnamurti" "Krishnamurti";
                REINGOLD_DERSHOWITZ "reingold-dershowitz" "Reingold-Dershowitz";
                FAGAN_BRADLEY "fagan-bradley" "Fagan-Bradley"
            );
            (
                hc_calendar::CalendarId(concat!($id, "-other-ayanamsa")),
                concat!($english, ", another ayanamsa"),
            )
        }
    };
    (@arms $ayanamsa:ident, $id:literal, $english:literal; $($konst:ident $suffix:literal $label:literal);+) => {
        $(
            if same($ayanamsa, Ayanamsa::$konst) {
                return (
                    hc_calendar::CalendarId(concat!($id, "-", $suffix)),
                    concat!($english, ", ", $label, " ayanamsa"),
                );
            }
        )+
    };
}

pub(crate) use by_ayanamsa;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hindu_lunar::{HinduLunarCalendar, ID};
    use crate::places::UJJAIN;
    use hc_calendar::Calendar;

    #[test]
    fn every_named_ayanamsa_has_its_identifier() {
        for ayanamsa in Ayanamsa::ALL {
            let id = HinduLunarCalendar::new(UJJAIN, *ayanamsa).meta().id;
            if same(*ayanamsa, Ayanamsa::LAHIRI) {
                assert_eq!(id, ID);
            } else {
                assert_eq!(id.0, alloc::format!("{}-{}", ID.0, ayanamsa.id()));
            }
        }
        let invented = Ayanamsa::new("invented", "invented", 2_451_545.0, 25.0);
        assert_eq!(
            HinduLunarCalendar::new(UJJAIN, invented).meta().id.0,
            "hindu-lunar-other-ayanamsa"
        );
    }

    /// The calendars built over the amānta one carry its convention in their
    /// identifier too, and the registered ones keep theirs.
    #[test]
    fn the_calendars_over_another_ayanamsa_say_so() {
        use crate::{
            HinduPurnimantaCalendar, NepalSambatCalendar, NepalSambatFortnightCalendar,
            OdiaAnkaCalendar, ViraNirvanaCalendar, odia_anka::DIBYASINGHA_DEB,
        };
        let raman = HinduLunarCalendar::new(UJJAIN, Ayanamsa::RAMAN);
        let purnimanta = HinduPurnimantaCalendar::new(raman);
        assert_eq!(purnimanta.meta().id.0, "hindu-lunar-purnimanta-raman");
        let sambat = NepalSambatCalendar::new(raman);
        assert_eq!(sambat.meta().id.0, "nepal-sambat-raman");
        assert_eq!(
            NepalSambatFortnightCalendar::new(sambat).meta().id.0,
            "nepal-sambat-fortnight-raman"
        );
        assert_eq!(
            ViraNirvanaCalendar::new(raman).meta().id.0,
            "vira-nirvana-samvat-raman"
        );
        assert_eq!(
            OdiaAnkaCalendar::new(purnimanta, DIBYASINGHA_DEB)
                .meta()
                .id
                .0,
            "odia-anka-raman"
        );
        for registered in [
            HinduPurnimantaCalendar::RASHTRIYA.meta().id.0,
            NepalSambatCalendar::KATHMANDU.meta().id.0,
            NepalSambatFortnightCalendar::KATHMANDU.meta().id.0,
            ViraNirvanaCalendar::RASHTRIYA.meta().id.0,
            OdiaAnkaCalendar::PURI.meta().id.0,
        ] {
            assert!(!registered.contains("raman"), "{registered}");
        }
    }
}
