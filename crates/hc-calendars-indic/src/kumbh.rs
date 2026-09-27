//! The Kumbh Mela: the positions of Jupiter, the Sun and the Moon that set
//! the festival at each of its four sites.
//!
//! `docs/systems/jupiter-festivals.md` in the repository describes the
//! system, works an example and states the sources; this page states the
//! code's own facts.
//!
//! The Mela Adhikari of the 2013 Kumbh at Allahabad gives, with the verse
//! for each, the conditions under which the festival is held at each site
//! (`kumbh-allahabad-astrology`): at Haridwar when Jupiter is in Kumbha and
//! the Sun enters Meṣa; at Prayag when Jupiter is in Vṛṣabha and the Sun
//! enters Makara, or when Jupiter is in Meṣa and the Sun and Moon are in
//! Makara at the new moon; at Nashik when the Sun and Jupiter are in Siṃha,
//! or when Jupiter, the Sun and the Moon are in Karka at the new moon; at
//! Ujjain when Jupiter is in Siṃha and the Sun enters Meṣa, or when Jupiter
//! is in Tulā and the Sun and Moon meet at the new moon of Kārttika. Each
//! condition is a [`KumbhYoga`] with its own identifier, since two
//! conditions for one site can fall in different years (`docs/policy.md`
//! §5): the Meṣa condition for Prayag holds in 2024, the Vṛṣabha one in
//! 2025, and the festival was held in 2025.
//!
//! This crate has no ephemeris of Jupiter, so Jupiter's sign is the
//! caller's: [`in_year`] takes it as a function of the moment. The Sun's
//! ingress and the new moon are computed here, with the ayanamsa as a
//! parameter.
//!
//! The dates of each festival are fixed and announced by the government of
//! the state that holds it; this module says whether a year's sky meets a
//! site's condition, not when the bathing days are.

use hc_astro::lunar::new_moon_at_or_after;
use hc_calendar::fixed::Moment;
use hc_seasons::zodiac::sidereal::{ingress_after, ingress_moment};
use hc_seasons::zodiac::{Ayanamsa, SiderealSign};

/// The sources of every condition: the Mela Adhikari's page, which gives
/// each condition in English with its verse.
const SOURCE: &str = "Mela Adhikari, Kumbh Mela 2013, \"Astrological Aspect\" \
                      (kumbhmelaallahabad.gov.in/english/astrological_aspect.html, \
                      archived 29 October 2019, kumbh-allahabad-astrology), \
                      retrieved 2026-09-28";

/// One condition under which the Kumbh Mela is held at a site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KumbhYoga {
    /// The identifier.
    pub id: &'static str,
    /// The site, in English.
    pub site: &'static str,
    /// The river the site stands on, in English.
    pub river: &'static str,
    /// The sign Jupiter must be in.
    pub jupiter: SiderealSign,
    /// The sign the Sun must be in.
    pub sun: SiderealSign,
    /// Whether the Moon must be with the Sun, at the new moon, as well.
    pub at_new_moon: bool,
    /// Where the condition comes from.
    pub source: &'static str,
}

hc_core::catalogue! {
    type: KumbhYoga,
    id: |yoga| yoga.id,
    provenance: |yoga| yoga.source,
    tests: kumbh_catalogue_tests,
    associated;

    /// The seven conditions, site by site in the order the source gives
    /// them.
    pub const ALL;
    /// The condition with this identifier.
    pub fn by_id;

    entries: {
        /// Haridwar, on the Ganga: Jupiter in Kumbha and the Sun in Meṣa.
        pub const HARIDWAR = Self {
            id: "kumbh-haridwar",
            site: "Haridwar",
            river: "Ganga",
            jupiter: SiderealSign::KUMBHA,
            sun: SiderealSign::MESHA,
            at_new_moon: false,
            source: SOURCE,
        };
        /// Prayag, at the meeting of the Ganga and the Yamuna: Jupiter in
        /// Vṛṣabha and the Sun in Makara. The condition of 2013 and 2025.
        pub const PRAYAG_VRISHABHA = Self {
            id: "kumbh-prayag-vrishabha",
            site: "Prayag",
            river: "Ganga and Yamuna",
            jupiter: SiderealSign::VRISHABHA,
            sun: SiderealSign::MAKARA,
            at_new_moon: false,
            source: SOURCE,
        };
        /// Prayag: Jupiter in Meṣa, and the Sun and Moon in Makara at the
        /// new moon.
        pub const PRAYAG_MESHA = Self {
            id: "kumbh-prayag-mesha",
            site: "Prayag",
            river: "Ganga and Yamuna",
            jupiter: SiderealSign::MESHA,
            sun: SiderealSign::MAKARA,
            at_new_moon: true,
            source: SOURCE,
        };
        /// Nashik, on the Godavari: the Sun and Jupiter in Siṃha, the
        /// Siṃhastha.
        pub const NASHIK_SIMHA = Self {
            id: "kumbh-nashik-simha",
            site: "Nashik",
            river: "Godavari",
            jupiter: SiderealSign::SIMHA,
            sun: SiderealSign::SIMHA,
            at_new_moon: false,
            source: SOURCE,
        };
        /// Nashik: Jupiter, the Sun and the Moon in Karka at the new moon.
        pub const NASHIK_KARKA = Self {
            id: "kumbh-nashik-karka",
            site: "Nashik",
            river: "Godavari",
            jupiter: SiderealSign::KARKA,
            sun: SiderealSign::KARKA,
            at_new_moon: true,
            source: SOURCE,
        };
        /// Ujjain, on the Shipra: Jupiter in Siṃha and the Sun in Meṣa.
        pub const UJJAIN_MESHA = Self {
            id: "kumbh-ujjain-mesha",
            site: "Ujjain",
            river: "Shipra",
            jupiter: SiderealSign::SIMHA,
            sun: SiderealSign::MESHA,
            at_new_moon: false,
            source: SOURCE,
        };
        /// Ujjain: Jupiter in Tulā, and the Sun and Moon together at the
        /// new moon of Kārttika, which falls with the Sun in Tulā.
        pub const UJJAIN_TULA = Self {
            id: "kumbh-ujjain-tula",
            site: "Ujjain",
            river: "Shipra",
            jupiter: SiderealSign::TULA,
            sun: SiderealSign::TULA,
            at_new_moon: true,
            source: SOURCE,
        };
    }
}

/// When the Sun, and the Moon where the condition asks for it, stand as a
/// condition requires in one year.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Occasion {
    /// The first moment, in Universal Time: the Sun's entry into the sign,
    /// or the new moon.
    pub from: Moment,
    /// The last moment: the Sun's entry into the next sign, or the new
    /// moon again.
    pub to: Moment,
}

impl KumbhYoga {
    /// When in Gregorian year `year` the Sun, and the Moon where the
    /// condition asks for it, stand as the condition requires: the Sun's
    /// stay in its sign that begins in the year, or the new moon within
    /// that stay. `None` for a new-moon condition when no new moon falls
    /// in the stay, which a stay shorter than a lunation allows.
    #[must_use]
    pub fn occasion(&self, year: i64, ayanamsa: Ayanamsa) -> Option<Occasion> {
        let enters = ingress_moment(year, self.sun, ayanamsa);
        let leaves = ingress_after(self.sun.next(), ayanamsa, enters);
        if !self.at_new_moon {
            return Some(Occasion {
                from: enters,
                to: leaves,
            });
        }
        let conjunction = new_moon_at_or_after(enters);
        (conjunction.0 < leaves.0).then_some(Occasion {
            from: conjunction,
            to: conjunction,
        })
    }
}

/// The occasion in `year` on which the condition holds, given Jupiter's
/// sign as a function of the moment: the occasion, when Jupiter is in the
/// condition's sign at its first moment, the Sun's entry into its sign or
/// the new moon; otherwise `None`.
///
/// The first moment is the one read because the Sun's stay can hold a
/// change of Jupiter's sign: in 2009 Jupiter entered Kumbha on 1 May,
/// during the Sun's stay in Meṣa, and the Haridwar festival was held in
/// 2010, when Jupiter was in Kumbha at the Sun's entry.
///
/// `jupiter` is the caller's ephemeris, reduced to the sidereal sign with
/// the same ayanamsa.
#[must_use]
pub fn in_year(
    yoga: &KumbhYoga,
    year: i64,
    ayanamsa: Ayanamsa,
    jupiter: impl Fn(Moment) -> SiderealSign,
) -> Option<Occasion> {
    let occasion = yoga.occasion(year, ayanamsa)?;
    (jupiter(occasion.from) == yoga.jupiter).then_some(occasion)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    /// A moment given in Indian Standard Time.
    pub(crate) fn ist(year: i64, month: u8, day: u8, hour: u8, minute: u8) -> Moment {
        let day = gregorian::to_fixed(year, month, day).expect("a date");
        Moment(day.0 as f64 + (f64::from(hour) + f64::from(minute) / 60.0 - 5.5) / 24.0)
    }

    /// Jupiter's sidereal ingresses, Lahiri, as Drik Panchang's "Guru
    /// Gochar" pages for New Delhi print them for 2001 to 2030
    /// (`drik-guru-gochar`, retrieved 2026-09-28): the year, month, day,
    /// hour and minute IST, and the sign entered, as its index from Meṣa.
    /// Jupiter was in Vṛṣabha from before 2001 to the first row.
    pub(crate) const DRIK_JUPITER: [(i64, u8, u8, u8, u8, u8); 49] = [
        (2001, 6, 16, 8, 38, 2),
        (2002, 7, 5, 13, 33, 3),
        (2003, 7, 30, 13, 4, 4),
        (2004, 8, 28, 0, 40, 5),
        (2005, 9, 28, 6, 27, 6),
        (2006, 10, 27, 23, 2, 7),
        (2007, 11, 22, 5, 36, 8),
        (2008, 12, 10, 0, 6, 9),
        (2009, 5, 1, 19, 13, 10),
        (2009, 7, 30, 19, 16, 9),
        (2009, 12, 20, 0, 35, 10),
        (2010, 5, 2, 8, 25, 11),
        (2010, 11, 1, 11, 56, 10),
        (2010, 12, 6, 10, 3, 11),
        (2011, 5, 8, 14, 26, 0),
        (2012, 5, 17, 9, 49, 1),
        (2013, 5, 31, 7, 10, 2),
        (2014, 6, 19, 9, 16, 3),
        (2015, 7, 14, 7, 7, 4),
        (2016, 8, 11, 22, 24, 5),
        (2017, 9, 12, 8, 0, 6),
        (2018, 10, 11, 20, 39, 7),
        (2019, 3, 30, 3, 9, 8),
        (2019, 4, 22, 17, 53, 7),
        (2019, 11, 5, 6, 41, 8),
        (2020, 3, 30, 5, 59, 9),
        (2020, 6, 30, 3, 6, 8),
        (2020, 11, 20, 14, 55, 9),
        (2021, 4, 6, 1, 50, 10),
        (2021, 9, 14, 11, 42, 9),
        (2021, 11, 21, 2, 5, 10),
        (2022, 4, 13, 16, 57, 11),
        (2023, 4, 22, 6, 12, 0),
        (2024, 5, 1, 13, 50, 1),
        (2025, 5, 14, 23, 20, 2),
        (2025, 10, 18, 21, 39, 3),
        (2025, 12, 5, 15, 38, 2),
        (2026, 6, 2, 2, 25, 3),
        (2026, 10, 31, 12, 50, 4),
        (2027, 1, 25, 0, 52, 3),
        (2027, 6, 26, 5, 43, 4),
        (2027, 11, 26, 19, 17, 5),
        (2028, 2, 28, 18, 50, 4),
        (2028, 7, 24, 15, 51, 5),
        (2028, 12, 26, 14, 1, 6),
        (2029, 3, 29, 14, 6, 5),
        (2029, 8, 25, 1, 13, 6),
        (2030, 1, 25, 2, 10, 7),
        (2030, 5, 1, 13, 48, 6),
    ];

    /// Jupiter's sign at a moment by Drik Panchang's table: the sign of the
    /// last ingress before it.
    pub(crate) fn jupiter(moment: Moment) -> SiderealSign {
        let mut sign = SiderealSign::VRISHABHA;
        for (year, month, day, hour, minute, entered) in DRIK_JUPITER {
            if ist(year, month, day, hour, minute).0 > moment.0 {
                break;
            }
            sign = SiderealSign::from_index(entered).expect("a sign");
        }
        sign
    }

    fn held(yoga: &KumbhYoga, year: i64) -> bool {
        in_year(yoga, year, Ayanamsa::LAHIRI, jupiter).is_some()
    }

    #[test]
    fn the_festivals_held_from_2001_to_2028_meet_their_sites_conditions() {
        // Wikipedia's table of Kumbh years (`wikipedia-kumbh-mela`): Prayag
        // 2001, 2013 and 2025; Nashik 2003, 2015 and 2027; Ujjain 2004,
        // 2016 and 2028; Haridwar 2010 and 2021. Nashik's condition holds
        // in 2004 as well: the Sun entered Siṃha on 16 August 2004, twelve
        // days before Jupiter left it, and the table lists 2003 alone.
        for (yoga, years) in [
            (&KumbhYoga::PRAYAG_VRISHABHA, &[2001, 2013, 2025][..]),
            (&KumbhYoga::NASHIK_SIMHA, &[2003, 2004, 2015, 2027]),
            (&KumbhYoga::UJJAIN_MESHA, &[2004, 2016, 2028]),
            (&KumbhYoga::HARIDWAR, &[2010, 2021]),
        ] {
            for year in 2001..=2029 {
                assert_eq!(
                    held(yoga, year),
                    years.contains(&year),
                    "{} {year}",
                    yoga.id
                );
            }
        }
    }

    #[test]
    fn the_second_conditions_fall_in_other_years() {
        // Prayag's Meṣa condition holds at the new moon of 9 February 2024,
        // a year before the festival of 2025; Nashik's Karka condition at
        // the new moon of July 2014, a year before 2015; Ujjain's Tulā
        // condition at the new moon of October 2017, a year after 2016.
        // Jupiter stays in a sign for about a year, sometimes more, so its
        // stay can take in the Sun's month in the same sign twice, and a
        // condition hold two years running: 2002 and 2003, 2005 and 2006.
        // Nashik's Siṃha condition did the same in 2003 and 2004.
        for (yoga, years) in [
            (&KumbhYoga::PRAYAG_MESHA, &[2012, 2024][..]),
            (&KumbhYoga::NASHIK_KARKA, &[2002, 2003, 2014, 2026]),
            (&KumbhYoga::UJJAIN_TULA, &[2005, 2006, 2017, 2029]),
        ] {
            for year in 2001..=2029 {
                assert_eq!(
                    held(yoga, year),
                    years.contains(&year),
                    "{} {year}",
                    yoga.id
                );
            }
        }
        let prayag = in_year(&KumbhYoga::PRAYAG_MESHA, 2024, Ayanamsa::LAHIRI, jupiter)
            .expect("the condition holds");
        // The new moon of the night of 9 to 10 February 2024 falls on the
        // 10th in IST.
        assert!(
            ist(2024, 2, 10, 0, 0).0 < prayag.from.0 && prayag.from.0 < ist(2024, 2, 10, 12, 0).0
        );
    }

    #[test]
    fn the_maha_kumbh_of_2025_opened_in_the_suns_stay_in_makara() {
        // 13 January to 26 February 2025 (`wikipedia-kumbh-mela`), its
        // first great bath on Makara Saṅkrānti, 14 January: the Sun's
        // stay in Makara, with Jupiter in Vṛṣabha throughout, runs from
        // that morning to 12 February.
        let occasion = in_year(
            &KumbhYoga::PRAYAG_VRISHABHA,
            2025,
            Ayanamsa::LAHIRI,
            jupiter,
        )
        .expect("held");
        assert!(
            ist(2025, 1, 14, 6, 0).0 < occasion.from.0
                && occasion.from.0 < ist(2025, 1, 14, 12, 0).0
        );
        assert!(
            ist(2025, 2, 12, 0, 0).0 < occasion.to.0 && occasion.to.0 < ist(2025, 2, 13, 0, 0).0
        );
        assert_eq!(
            KumbhYoga::by_id("kumbh-haridwar"),
            Some(KumbhYoga::HARIDWAR)
        );
    }
}
