//! Pushkaram: the river festival of the sign Jupiter enters, kept for
//! twelve days from its entry.
//!
//! `docs/systems/jupiter-festivals.md` in the repository describes the
//! system, works an example and states the sources; this page states the
//! code's own facts.
//!
//! Each sidereal sign has a river, and when Jupiter enters the sign the
//! river's festival is kept for the first twelve days, the *Ādi
//! Pushkaram* (`wikipedia-pushkaram`). Some signs have a river in one
//! region and another elsewhere, so [`PushkaramRiver`] has an entry for
//! each river, and [`rivers_of`] gives every river of a sign. When Jupiter
//! enters a sign, turns back out of it and enters it again, the second
//! entry is the one the festival follows: Wikipedia states it, citing
//! Pillai, *Panchang and Horoscope* (1996, not read), and the Tungabhadra
//! festival of 2020 was kept so.
//!
//! This crate has no ephemeris of Jupiter, so the entry is the caller's:
//! [`adi_pushkaram`] takes the moment and gives the twelve days. The first
//! day is the civil day of the entry, or the next day when the entry falls
//! after that day's sunset. That reading is fitted to the festivals whose
//! dates were read, against Drik Panchang's times of Jupiter's entries; no
//! source read states it.

use hc_astro::riseset::{Location, sunset};
use hc_astro::solar_time::MissingSolarEvent;
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_seasons::Meridian;
use hc_seasons::zodiac::SiderealSign;

/// The days of the *Ādi Pushkaram*.
pub const ADI_PUSHKARAM_DAYS: i64 = 12;

/// Where the river-to-sign table comes from.
const SOURCE: &str = "Wikipedia, \"Pushkaram\" (wikipedia-pushkaram), retrieved 2026-09-28, \
                      citing Dalal, Hinduism: An Alphabetical Guide (2014), not read";

/// The river of one sign, as one region keeps it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PushkaramRiver {
    /// The identifier.
    pub id: &'static str,
    /// The river, in English.
    pub river: &'static str,
    /// The sign Jupiter enters.
    pub sign: SiderealSign,
    /// Where the source says the river is kept for this sign, or `""`
    /// where it names no region.
    pub region: &'static str,
    /// Where the pairing comes from.
    pub source: &'static str,
}

/// A river with no region named.
const fn river(id: &'static str, name: &'static str, sign: SiderealSign) -> PushkaramRiver {
    PushkaramRiver {
        id,
        river: name,
        sign,
        region: "",
        source: SOURCE,
    }
}

hc_core::catalogue! {
    type: PushkaramRiver,
    id: |river| river.id,
    provenance: |river| river.source,
    tests: pushkaram_catalogue_tests,
    associated;

    /// The rivers, Meṣa's first.
    pub const ALL;
    /// The river with this identifier.
    pub fn by_id;

    entries: {
        /// The Ganga, Meṣa.
        pub const GANGA = river("pushkaram-ganga", "Ganga", SiderealSign::MESHA);
        /// The Narmada, Vṛṣabha.
        pub const NARMADA = river("pushkaram-narmada", "Narmada", SiderealSign::VRISHABHA);
        /// The Sarasvatī, Mithuna.
        pub const SARASVATI = river("pushkaram-sarasvati", "Sarasvati", SiderealSign::MITHUNA);
        /// The Yamuna, Karka.
        pub const YAMUNA = river("pushkaram-yamuna", "Yamuna", SiderealSign::KARKA);
        /// The Godavari, Siṃha.
        pub const GODAVARI = river("pushkaram-godavari", "Godavari", SiderealSign::SIMHA);
        /// The Krishna, Kanyā.
        pub const KRISHNA = river("pushkaram-krishna", "Krishna", SiderealSign::KANYA);
        /// The Kaveri, Tulā.
        pub const KAVERI = river("pushkaram-kaveri", "Kaveri", SiderealSign::TULA);
        /// The Bhima, Vṛścika, in Maharashtra, Karnataka and Telangana.
        pub const BHIMA = PushkaramRiver {
            region: "Maharashtra, Karnataka, Telangana",
            ..river("pushkaram-bhima", "Bhima", SiderealSign::VRISHCHIKA)
        };
        /// The Tamraparni, Vṛścika, in Tamil Nadu.
        pub const TAMRAPARNI = PushkaramRiver {
            region: "Tamil Nadu",
            ..river("pushkaram-tamraparni", "Tamraparni", SiderealSign::VRISHCHIKA)
        };
        /// The Tapti, Dhanus.
        pub const TAPTI = river("pushkaram-tapti", "Tapti", SiderealSign::DHANUS);
        /// The Brahmaputra, Dhanus, in Assam.
        pub const BRAHMAPUTRA = PushkaramRiver {
            region: "Assam",
            ..river("pushkaram-brahmaputra", "Brahmaputra", SiderealSign::DHANUS)
        };
        /// The Tungabhadra, Makara.
        pub const TUNGABHADRA = river("pushkaram-tungabhadra", "Tungabhadra", SiderealSign::MAKARA);
        /// The Sindhu, the Indus, Kumbha.
        pub const SINDHU = river("pushkaram-sindhu", "Sindhu", SiderealSign::KUMBHA);
        /// The Pranahita, Mīna.
        pub const PRANAHITA = river("pushkaram-pranahita", "Pranahita", SiderealSign::MINA);
    }
}

/// Every river of a sign.
pub fn rivers_of(sign: SiderealSign) -> impl Iterator<Item = PushkaramRiver> {
    PushkaramRiver::ALL
        .iter()
        .copied()
        .filter(move |river| river.sign == sign)
}

/// A run of civil days, first and last included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DaySpan {
    /// The first day.
    pub first: Rd,
    /// The last day.
    pub last: Rd,
}

/// The twelve days of the *Ādi Pushkaram* for Jupiter's entry into a sign
/// at `entry`: from the civil day of the entry at `meridian`, or the next
/// day when the entry falls after that day's sunset at `location`.
///
/// Where Jupiter enters, turns back and enters again, pass the second
/// entry.
///
/// # Errors
///
/// [`MissingSolarEvent::Sunset`] where the Sun does not set on the day of
/// the entry at `location`.
pub fn adi_pushkaram(
    entry: Moment,
    location: Location,
    meridian: Meridian,
) -> Result<DaySpan, MissingSolarEvent> {
    let day = meridian.day_of(entry);
    let set = sunset(day, location).ok_or(MissingSolarEvent::Sunset(day))?;
    let first = if entry.0 >= set.0 { day + 1 } else { day };
    Ok(DaySpan {
        first,
        last: first + (ADI_PUSHKARAM_DAYS - 1),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kumbh::tests::{DRIK_JUPITER, ist};
    use hc_calendars_solar::gregorian;

    /// New Delhi, the place Drik Panchang's times are for.
    const NEW_DELHI: Location = Location::new(28.6356, 77.2244, 0.0);

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    /// Drik Panchang's moment of Jupiter's entry into `sign` in a month.
    fn entry(year: i64, month: u8, sign: SiderealSign) -> Moment {
        DRIK_JUPITER
            .iter()
            .find(|row| row.0 == year && row.1 == month && row.5 == sign.index())
            .map(|&(year, month, day, hour, minute, _)| ist(year, month, day, hour, minute))
            .expect("an entry")
    }

    #[test]
    fn the_festivals_whose_dates_were_read_fall_on_the_twelve_days() {
        for (river, year, first, last, source) in [
            // Jupiter entered Siṃha at 07:07 on 14 July 2015.
            (
                PushkaramRiver::GODAVARI,
                2015,
                (7, 14),
                (7, 25),
                "wikipedia-godavari-pushkaram",
            ),
            // Tulā at 08:00 on 12 September 2017.
            (
                PushkaramRiver::KAVERI,
                2017,
                (9, 12),
                (9, 23),
                "wikipedia-kaveri-pushkaram",
            ),
            // Makara at 14:55 on 20 November 2020, the second entry of the
            // year: the first was on 30 March, and Jupiter went back into
            // Dhanus on 30 June.
            (
                PushkaramRiver::TUNGABHADRA,
                2020,
                (11, 20),
                (12, 1),
                "kurnool-tungabhadra-2020",
            ),
            // Kanyā at 22:24 on 11 August 2016, after sunset: the festival
            // began the next day.
            (
                PushkaramRiver::KRISHNA,
                2016,
                (8, 12),
                (8, 23),
                "vijayawadapolice-krishna-2016",
            ),
            // Dhanus at 06:41 on 5 November 2019, the second entry of the
            // year: the first was on 30 March, and Jupiter went back into
            // Vṛścika on 22 April.
            (
                PushkaramRiver::BRAHMAPUTRA,
                2019,
                (11, 5),
                (11, 16),
                "sentinel-brahmaputra-2019",
            ),
            // Mīna at 16:57 on 13 April 2022.
            (
                PushkaramRiver::PRANAHITA,
                2022,
                (4, 13),
                (4, 24),
                "hansindia-pranahita-2022",
            ),
            // Mithuna at 23:20 on 14 May 2025, after sunset: the festival
            // began the next day.
            (
                PushkaramRiver::SARASVATI,
                2025,
                (5, 15),
                (5, 26),
                "wikipedia-sarasvati-pushkaram",
            ),
            // Siṃha at 05:43 on 26 June 2027, the festival the East
            // Godavari district announces for 26 June to 7 July 2027.
            (
                PushkaramRiver::GODAVARI,
                2027,
                (6, 26),
                (7, 7),
                "eastgodavari-pushkaralu-2027",
            ),
        ] {
            let span = adi_pushkaram(entry(year, first.0, river.sign), NEW_DELHI, Meridian::INDIA)
                .expect("a sunset");
            assert_eq!(
                span,
                DaySpan {
                    first: ymd(year, first.0, first.1),
                    last: ymd(year, last.0, last.1),
                },
                "{} {year}, {source}",
                river.id
            );
        }
    }

    #[test]
    fn the_ganga_festival_of_2023_began_on_the_day_of_the_entry() {
        // Wikipedia's table gives 22 April to 5 May 2023, fourteen days;
        // the first agrees with Jupiter's entry into Meṣa at 06:12 on
        // 22 April.
        let span = adi_pushkaram(
            entry(2023, 4, SiderealSign::MESHA),
            NEW_DELHI,
            Meridian::INDIA,
        )
        .expect("a sunset");
        assert_eq!(span.first, ymd(2023, 4, 22));
    }

    #[test]
    fn two_signs_have_two_rivers() {
        let two: alloc::vec::Vec<&str> = rivers_of(SiderealSign::VRISHCHIKA)
            .map(|river| river.river)
            .collect();
        assert_eq!(two, ["Bhima", "Tamraparni"]);
        assert_eq!(rivers_of(SiderealSign::DHANUS).count(), 2);
        for sign in SiderealSign::ALL {
            assert!(rivers_of(sign).count() >= 1, "{sign:?}");
        }
        assert_eq!(
            PushkaramRiver::by_id("pushkaram-kaveri"),
            Some(PushkaramRiver::KAVERI)
        );
    }

    #[test]
    fn there_is_no_pushkaram_day_where_the_sun_does_not_set() {
        let tromso = Location::new(69.6496, 18.9560, 0.0);
        let midsummer = ist(2024, 6, 21, 12, 0);
        assert_eq!(
            adi_pushkaram(midsummer, tromso, Meridian::INDIA),
            Err(MissingSolarEvent::Sunset(ymd(2024, 6, 21)))
        );
    }
}
