//! Full-moon names — a publication history, not an ethnographic record.
//!
//! Wolf, Snow, Worm, Pink, Flower, Strawberry, Buck, Sturgeon, Harvest,
//! Hunter's, Beaver, Cold. Almanac publishers print these as "Native
//! American moon names", and that attribution is contested and in several
//! cases demonstrably wrong. This module ships the names because they are in
//! wide use, and it ships them with their actual provenance, which is a
//! chain of publications.
//!
//! # What the record shows
//!
//! - **Jonathan Carver, 1778.** *Travels through the Interior Parts of
//!   North-America* gives twelve lunar-month names — Worm, Plants, Flowers,
//!   Hot, Buck, Sturgeon, Corn, Travelling, Beaver, Hunting, Cold, Snow —
//!   counted from the first moon after the March equinox. Carver names no
//!   nation; he writes only of "the Indians". These are month names, not
//!   full-moon names. Seven of the twelve modern names appear in it
//!   verbatim; popular accounts put the figure at nine by counting
//!   Flowers/Flower and Hunting/Hunter's as the same name, which is a claim
//!   about descent rather than about the text. [`MOON_NAMES_CARVER_1778`].
//! - **Vetromile 1856 and Peter Jones 1861** each published completely
//!   different lists, which is the first clue that no single list existed to
//!   be recorded.
//! - **Richard Irving Dodge, 1882.** *Our Wild Indians*, p. 397: he had
//!   never met a Plains people with "a permanent, common, conventional name
//!   for any moon", and observed that the same person would name the same
//!   moon differently on different days. His conclusion was that a fixed
//!   twelve, if it existed anywhere, was borrowed rather than indigenous.
//! - **The Improved Order of Red Men**, an all-white American fraternal
//!   order, adopted Carver's list in the nineteenth century and shifted it
//!   onto Gregorian months.
//! - **The Maine Farmers' Almanac, 1937**, printed a quite different set and
//!   said they "were named by our early English ancestors": Moon after Yule,
//!   Wolf, Lenten, Egg, Milk, Flower, Hay, Grain, Fruit, Harvest, Hunter's,
//!   Moon before Yule. [`MOON_NAMES_MAINE_1937`].
//! - **The Old Farmer's Almanac, 1964**, first published a twelve-name list,
//!   with June Moon rather than Strawberry, Hot Moon for July and Travel
//!   Moon for November. [`MOON_NAMES_OFA_1964`].
//! - **The Old Farmer's Almanac today** prints the list everyone now knows.
//!   [`MOON_NAMES_OFA_CURRENT`].
//!
//! Two of the modern names are European rather than American in origin:
//! "harvest moon" and "hunter's moon" are recorded in English from 1706 and
//! 1710 respectively, and the 1710 *British Apollo* attributes the second to
//! "the country people" — of England.
//!
//! So: this module ships four tables, each attributed to its publication,
//! and no table in it is labelled with the name of any Native American
//! nation. That is not squeamishness. It is that the sources do not support
//! such a label, and inventing one would be the exact failure this crate
//! exists to avoid.
//!
//! # Other traditions' moon names are not blended in
//!
//! Where another tradition names its moons, it does so over its own
//! calendar, and mapping those names onto Gregorian months would destroy the
//! thing being named. Japan's 中秋の名月 and 十三夜 are lunisolar dates and
//! live in [`hc_seasons::moon_calendar::mid_autumn_moon`] and
//! [`hc_seasons::moon_calendar::thirteenth_night`], where they belong.
//! Nothing here duplicates or absorbs them.
//!
//! # The Harvest Moon is a rule, not a row
//!
//! The Harvest Moon is defined as the full moon nearest the September
//! equinox, so in roughly a quarter of years it falls in October and
//! September's full moon takes its other name, the Corn Moon. A table cannot
//! express that. [`harvest_moon`] computes it from the equinox instant and
//! the lunar phases, both from `hc-seasons`, and [`harvest_moon_falls_in`]
//! reports which month it landed in.
//!
//! ```
//! use hc_attributes::moon_names::{harvest_moon, harvest_moon_falls_in};
//! use hc_seasons::Meridian;
//!
//! // 2025's harvest moon fell on 7 October: the September full moon was
//! // farther from the equinox than the October one.
//! assert_eq!(harvest_moon_falls_in(2025, Meridian::UNIVERSAL), 10);
//! assert_eq!(harvest_moon_falls_in(2024, Meridian::UNIVERSAL), 9);
//! let _ = harvest_moon(2025, Meridian::UNIVERSAL);
//! ```
//!
//! # Accuracy
//!
//! The lunar phase instants come from `hc-astro` through `hc-seasons` and
//! land within about a minute of the published ones, and so does the
//! equinox instant. The Harvest Moon rule compares two intervals of roughly a fortnight, so
//! neither error can change the answer except in a year where two full moons
//! are almost exactly equidistant from the equinox — which requires them to
//! be within seconds of 14.77 days either side, and does not occur in the
//! range this crate is tested over.

use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_seasons::hc_astro::lunar::MoonPhase;
use hc_seasons::moon_calendar::principal_phases_in_month;
use hc_seasons::solar_terms::term_moment;
use hc_seasons::{Meridian, Season};

use crate::authority::{
    AttributionDate, Authority, MonthTable, Provenance, Region, Validity, month_index,
};
use crate::gregorian::year_month_day_from_rd;

/// The list the *Old Farmer's Almanac* prints today.
///
/// September carries both Harvest and Corn, because which of the two applies
/// depends on where the equinox falls — see [`harvest_moon`]. October
/// carries both Hunter's and Harvest for the same reason.
///
/// [`Provenance::Contested`]: the almanac presents these as Native American,
/// and the documentation at the top of this module sets out why that cannot
/// be taken at face value.
pub static MOON_NAMES_OFA_CURRENT: MonthTable = MonthTable::new(
    Authority {
        id: "moon-names-ofa-current",
        english_name: "full-moon names as the Old Farmer's Almanac now prints them",
        body: Some("The Old Farmer's Almanac"),
        region: Region::NORTH_AMERICA,
        established: Some(AttributionDate::year(1964)),
        revised: None,
        validity: Validity::since(1964),
        provenance: Provenance::Contested,
        source: "The Old Farmer's Almanac, \"Full Moon Names\", https://www.almanac.com/\
                 full-moon-names; publication history per History.com, \"How Full Moons Got \
                 Their Names\", and Sky & Telescope, \"Native American Full Moon Names\"",
        caveat: Some(
            "Published as Native American moon names. The attribution is contested: nine of \
             the twelve descend from Jonathan Carver's 1778 list, which names no nation, and \
             harvest moon and hunter's moon are recorded in English from 1706 and 1710. This \
             is a publication history, not an ethnographic record.",
        ),
    },
    [
        &["Wolf Moon"],
        &["Snow Moon"],
        &["Worm Moon"],
        &["Pink Moon"],
        &["Flower Moon"],
        &["Strawberry Moon"],
        &["Buck Moon"],
        &["Sturgeon Moon"],
        &["Harvest Moon", "Corn Moon"],
        &["Hunter's Moon", "Harvest Moon"],
        &["Beaver Moon"],
        &["Cold Moon"],
    ],
);

/// The same almanac's list as first published in 1964.
///
/// Three months differ from what the same publisher prints now: June was
/// the generic "June Moon" rather than Strawberry, July was Hot Moon rather
/// than Buck, and November was Travel Moon rather than Beaver. Two of the
/// three 1964 readings, Hot and Travel, are closer to Carver's 1778 list.
///
/// One publisher, sixty years, three changes: shipping both is what lets a
/// caller see that "the traditional name" moved within living memory.
pub static MOON_NAMES_OFA_1964: MonthTable = MonthTable::new(
    Authority {
        id: "moon-names-ofa-1964",
        english_name: "full-moon names as the Old Farmer's Almanac first published them",
        body: Some("The Old Farmer's Almanac"),
        region: Region::NORTH_AMERICA,
        established: Some(AttributionDate::year(1964)),
        revised: None,
        validity: Validity::between(1964, 1964),
        provenance: Provenance::Contested,
        source: "History.com, \"How Full Moons Got Their Names\", reporting the Old Farmer's \
                 Almanac's first publication of moon names in 1964",
        caveat: Some(
            "The same caveat as the current list, plus: superseded by the publisher's own \
             later printings. Shipped so the drift can be measured.",
        ),
    },
    [
        &["Wolf Moon"],
        &["Snow Moon"],
        &["Worm Moon"],
        &["Pink Moon"],
        &["Flower Moon"],
        &["June Moon"],
        &["Hot Moon"],
        &["Sturgeon Moon"],
        &["Harvest Moon", "Corn Moon"],
        &["Hunter's Moon", "Harvest Moon"],
        &["Travel Moon"],
        &["Cold Moon"],
    ],
);

/// The *Maine Farmers' Almanac* list of 1937, attributed by its own
/// publisher to "our early English ancestors".
///
/// A separate tradition, not a variant of the others: it is seasonal rather
/// than monthly, runs from the winter solstice, and shares only Wolf,
/// Flower, Harvest and Hunter's with the modern list. It is the source of the
/// original "blue moon" rule, and of the observation that these names were
/// being called English by the people printing them two decades before they
/// were widely called Native American.
///
/// The index is a Gregorian month only by approximation: the almanac's
/// twelve run from the solstice, three to a season, and the first ("Moon
/// after Yule") is the first full moon after the December solstice, which
/// is usually but not always January's. The caveat says so.
pub static MOON_NAMES_MAINE_1937: MonthTable = MonthTable::new(
    Authority {
        id: "moon-names-maine-1937",
        english_name: "full-moon names of the Maine Farmers' Almanac",
        body: Some("Maine Farmers' Almanac"),
        region: Region::NORTH_AMERICA,
        established: Some(AttributionDate::year(1937)),
        revised: None,
        validity: Validity::between(1937, 1937),
        provenance: Provenance::Recorded,
        source: "Maine Farmers' Almanac (1937), as transcribed in \
                 https://www.projectpluto.com/bluemoon.htm",
        caveat: Some(
            "Keyed to the four seasons from the winter solstice, three moons to a season, not \
             to Gregorian months. The index here is the usual month for each, which is an \
             approximation the almanac itself did not make.",
        ),
    },
    [
        &["Moon after Yule"],
        &["Wolf Moon"],
        &["Lenten Moon"],
        &["Egg Moon"],
        &["Milk Moon"],
        &["Flower Moon"],
        &["Hay Moon"],
        &["Grain Moon"],
        &["Fruit Moon"],
        &["Harvest Moon"],
        &["Hunter's Moon"],
        &["Moon before Yule"],
    ],
);

/// Carver's 1778 list, in its own key: lunar months from the March equinox.
///
/// Index 0 is the first lunar month beginning after the March equinox, not
/// January. Carver was recording *lunar month* names, not full-moon names,
/// and the difference is the reason this table has its own accessor,
/// [`carver_lunation_name`], rather than sharing the month accessor.
///
/// Carver attributes the list to "the Indians" without naming a people, and
/// every later writer who tried to check it published something different.
pub static MOON_NAMES_CARVER_1778: MonthTable = MonthTable::new(
    Authority {
        id: "moon-names-carver-1778",
        english_name: "lunar-month names as recorded by Jonathan Carver",
        body: None,
        region: Region::NORTH_AMERICA,
        established: Some(AttributionDate::year(1778)),
        revised: None,
        validity: Validity::between(1778, 1778),
        provenance: Provenance::Contested,
        source: "Jonathan Carver, Travels through the Interior Parts of North-America in the \
                 Years 1766, 1767, and 1768 (London, 1778), p. 250",
        caveat: Some(
            "Indexed by lunation from the March equinox, not by Gregorian month. Carver names \
             no nation. Vetromile (1856) and Peter Jones (1861) published entirely different \
             lists, and Dodge (1882) denied that any fixed twelve existed.",
        ),
    },
    [
        &["Worm Moon"],
        &["Plants Moon"],
        &["Flowers Moon"],
        &["Hot Moon"],
        &["Buck Moon"],
        &["Sturgeon Moon"],
        &["Corn Moon"],
        &["Travelling Moon"],
        &["Beaver Moon"],
        &["Hunting Moon"],
        &["Cold Moon"],
        &["Snow Moon"],
    ],
);

/// The four month-keyed full-moon tables, oldest publication first.
///
/// [`MOON_NAMES_CARVER_1778`] is excluded: it is keyed by lunation from the
/// March equinox, not by month, so putting it in an array a caller indexes
/// by month would be the very confusion the crate is built to prevent.
pub static ALL: [&MonthTable; 3] = [
    &MOON_NAMES_MAINE_1937,
    &MOON_NAMES_OFA_1964,
    &MOON_NAMES_OFA_CURRENT,
];

/// The names one table gives for a Gregorian month.
///
/// # Errors
///
/// As [`crate::birthstones::stones`].
pub fn names(
    table: &MonthTable,
    month: hc_calendar::Month,
) -> hc_calendar::CalendarResult<&'static [&'static str]> {
    table.at(month_index(month)?)
}

/// Carver's name for the *n*th lunation after the March equinox, counting
/// the first as zero.
///
/// Returns `None` for `lunation >= 12`. Carver noted that a thirteenth moon
/// was added every thirty and called "the lost moon", and gave it no name;
/// this function does not invent one.
#[must_use]
pub fn carver_lunation_name(lunation: usize) -> Option<&'static [&'static str]> {
    MOON_NAMES_CARVER_1778.at(lunation).ok()
}

/// The day of the Harvest Moon: the full moon nearest the September equinox.
///
/// This is a rule, not a table row. The September equinox is fixed to within
/// a day or so, the synodic month is 29.53 days, and the nearest full moon
/// can fall on either side of it — so the Harvest Moon lands in September in
/// most years and in October in the rest. [`harvest_moon_falls_in`] says
/// which.
///
/// Ties are broken towards the earlier moon, which cannot arise in practice:
/// it would require two full moons exactly 14.765 days either side of the
/// equinox instant.
#[must_use]
pub fn harvest_moon(year: i64, meridian: Meridian) -> Rd {
    let equinox = september_equinox(year);
    nearest_full_moon(equinox, year, meridian)
}

/// The day of the Hunter's Moon: the full moon after the Harvest Moon.
///
/// Defined relative to the Harvest Moon rather than to a month, so it moves
/// with it: an October Harvest Moon puts the Hunter's Moon in November.
#[must_use]
pub fn hunters_moon(year: i64, meridian: Meridian) -> Rd {
    let harvest = harvest_moon(year, meridian);
    full_moons_around_the_equinox(year, meridian)
        .into_iter()
        .flatten()
        .map(|event| event.0)
        .find(|day| day.0 > harvest.0)
        .unwrap_or(Rd(harvest.0 + SYNODIC_MONTH_WHOLE_DAYS))
}

/// The Gregorian month the Harvest Moon falls in: 9 or 10.
#[must_use]
pub fn harvest_moon_falls_in(year: i64, meridian: Meridian) -> u8 {
    let (_, month, _) = year_month_day_from_rd(harvest_moon(year, meridian));
    month
}

/// Whether the Harvest Moon falls in October in a given year.
///
/// True in roughly one year in four. When it does, September's full moon is
/// the Corn Moon rather than the Harvest Moon, which is why
/// [`MOON_NAMES_OFA_CURRENT`] lists both names against September and both
/// against October.
#[must_use]
pub fn harvest_moon_is_in_october(year: i64, meridian: Meridian) -> bool {
    harvest_moon_falls_in(year, meridian) == 10
}

/// The name the Old Farmer's Almanac gives September's full moon in a
/// particular year: Harvest if the rule puts the Harvest Moon in September,
/// Corn if it puts it in October.
///
/// This is the one place in the crate where the answer depends on the year,
/// and it is the reason the Harvest Moon could not simply be a table entry.
#[must_use]
pub fn september_moon_name(year: i64, meridian: Meridian) -> &'static str {
    if harvest_moon_is_in_october(year, meridian) {
        "Corn Moon"
    } else {
        "Harvest Moon"
    }
}

/// A whole-day approximation of the synodic month, used only as the
/// unreachable fallback in [`hunters_moon`].
const SYNODIC_MONTH_WHOLE_DAYS: i64 = 30;

/// The instant of the September equinox, in Universal Time.
///
/// Northern autumn opens at the equinox, so this is the cardinal term of
/// [`Season::Autumn`] — the same 180° of solar longitude `hc-seasons`
/// computes for 秋分.
fn september_equinox(year: i64) -> Moment {
    term_moment(year, Season::Autumn.cardinal_term())
}

/// The full moons of August through November, as (day, instant) pairs.
///
/// Four months is more than enough: the nearest full moon to the equinox is
/// at most half a lunation — fifteen days — away, so September and October
/// alone would do. August and November are included so that
/// [`hunters_moon`] has a successor to find even when the Harvest Moon is
/// late in October, and so a blue-moon month cannot hide a candidate.
fn full_moons_around_the_equinox(year: i64, meridian: Meridian) -> [Option<(Rd, Moment)>; 8] {
    let mut found = [None; 8];
    let mut next = 0usize;
    for month in 8..=11u8 {
        let phases = principal_phases_in_month(year, month, meridian);
        for event in [phases.full_moon(), phases.second(MoonPhase::Full)]
            .into_iter()
            .flatten()
        {
            if next < found.len() {
                found[next] = Some((event.day, event.moment));
                next += 1;
            }
        }
    }
    found
}

/// The full moon whose instant is nearest a given one.
///
/// Falls back to the September full moon, and then to the equinox day
/// itself, if the phase search somehow yields nothing — which it does not
/// for any year in the supported range, but the crate forbids `unwrap`
/// outside tests and a fallback is better than a panic in a date library.
fn nearest_full_moon(target: Moment, year: i64, meridian: Meridian) -> Rd {
    let mut best: Option<(f64, Rd)> = None;
    for (day, moment) in full_moons_around_the_equinox(year, meridian)
        .into_iter()
        .flatten()
    {
        let distance = hc_core::math::abs(moment.0 - target.0);
        let closer = match best {
            Some((best_distance, _)) => distance < best_distance,
            None => true,
        };
        if closer {
            best = Some((distance, day));
        }
    }
    match best {
        Some((_, day)) => day,
        None => meridian.day_of(target),
    }
}

#[cfg(test)]
mod tests {
    use hc_calendar::{CalendarError, Month};

    use super::*;

    #[test]
    fn every_month_keyed_table_has_twelve_non_empty_entries() {
        for table in ALL {
            assert_eq!(table.len(), 12, "{}", table.authority().id);
            assert!(table.is_complete(), "{}", table.authority().id);
        }
        assert!(MOON_NAMES_CARVER_1778.is_complete());
        assert_eq!(MOON_NAMES_CARVER_1778.len(), 12);
    }

    /// Every one of these tables is contested or is a bare transcription,
    /// and every one of them must say so where a caller will see it.
    #[test]
    fn every_moon_table_carries_a_caveat_and_names_its_publication() {
        for table in [
            &MOON_NAMES_OFA_CURRENT,
            &MOON_NAMES_OFA_1964,
            &MOON_NAMES_MAINE_1937,
            &MOON_NAMES_CARVER_1778,
        ] {
            let authority = table.authority();
            assert!(authority.caveat.is_some(), "{}", authority.id);
            assert!(!authority.source.is_empty(), "{}", authority.id);
            assert!(authority.established.is_some(), "{}", authority.id);
        }
    }

    /// The crate refuses to attach any of these lists to a named nation,
    /// because the sources do not support it. A test is the only way to
    /// keep a future edit from quietly adding one.
    #[test]
    fn no_moon_table_is_attributed_to_a_named_native_american_nation() {
        for table in [
            &MOON_NAMES_OFA_CURRENT,
            &MOON_NAMES_OFA_1964,
            &MOON_NAMES_MAINE_1937,
            &MOON_NAMES_CARVER_1778,
        ] {
            let authority = table.authority();
            assert_eq!(authority.region, Region::NORTH_AMERICA, "{}", authority.id);
            for nation in ["Algonquian", "Ojibwe", "Lakota", "Cherokee", "Sioux"] {
                assert!(
                    !authority.english_name.contains(nation),
                    "{} names {nation}",
                    authority.id
                );
                assert!(
                    !authority.body.unwrap_or("").contains(nation),
                    "{} names {nation}",
                    authority.id
                );
            }
        }
    }

    #[test]
    fn the_two_contested_tables_are_marked_contested_and_the_transcription_is_not() {
        assert_eq!(
            MOON_NAMES_OFA_CURRENT.authority().provenance,
            Provenance::Contested
        );
        assert!(
            MOON_NAMES_OFA_CURRENT
                .authority()
                .provenance
                .warrants_a_caveat()
        );
        assert_eq!(
            MOON_NAMES_MAINE_1937.authority().provenance,
            Provenance::Recorded
        );
    }

    /// One publisher changed its own "traditional" names three times in
    /// sixty years. That is the fact the 1964 table exists to record.
    #[test]
    fn the_almanac_changed_three_of_its_own_names_between_1964_and_now() {
        let mut changed = 0;
        for ordinal in 1..=12u8 {
            let month = Month::regular(ordinal);
            if names(&MOON_NAMES_OFA_1964, month) != names(&MOON_NAMES_OFA_CURRENT, month) {
                changed += 1;
            }
        }
        assert_eq!(changed, 3);
        assert_eq!(
            names(&MOON_NAMES_OFA_1964, Month::regular(6)),
            Ok(&["June Moon"][..])
        );
        assert_eq!(
            names(&MOON_NAMES_OFA_CURRENT, Month::regular(6)),
            Ok(&["Strawberry Moon"][..])
        );
        assert_eq!(
            names(&MOON_NAMES_OFA_1964, Month::regular(11)),
            Ok(&["Travel Moon"][..])
        );
        assert_eq!(
            names(&MOON_NAMES_OFA_CURRENT, Month::regular(11)),
            Ok(&["Beaver Moon"][..])
        );
    }

    /// Seven of the twelve modern names appear in Carver verbatim.
    ///
    /// Popular accounts say nine "descend from" Carver, and that is a
    /// looser claim than this test makes: it counts Flower/Flowers and
    /// Hunter's/Hunting as the same name. This test compares exact strings,
    /// because a crate that reports what traditions claim should be able to
    /// tell a citation from a resemblance. Both numbers are in the module
    /// documentation.
    #[test]
    fn seven_of_the_twelve_modern_names_appear_in_carvers_1778_list_verbatim() {
        let shared = (1..=12u8)
            .filter(|ordinal| {
                let modern =
                    names(&MOON_NAMES_OFA_CURRENT, Month::regular(*ordinal)).unwrap_or(&[]);
                modern.iter().any(|name| MOON_NAMES_CARVER_1778.names(name))
            })
            .count();
        assert_eq!(shared, 7);
        // Absent from Carver outright.
        assert!(!MOON_NAMES_CARVER_1778.names("Pink Moon"));
        assert!(!MOON_NAMES_CARVER_1778.names("Strawberry Moon"));
        assert!(!MOON_NAMES_CARVER_1778.names("Wolf Moon"));
        // Present only in a different form, which is the gap between
        // "descends from" and "is".
        assert!(MOON_NAMES_CARVER_1778.names("Flowers Moon"));
        assert!(!MOON_NAMES_CARVER_1778.names("Flower Moon"));
        assert!(MOON_NAMES_CARVER_1778.names("Hunting Moon"));
        assert!(!MOON_NAMES_CARVER_1778.names("Hunter\'s Moon"));
        // Harvest is European, not Carver's, in any form.
        assert!(!MOON_NAMES_CARVER_1778.names("Harvest Moon"));
    }

    #[test]
    fn carvers_list_starts_after_the_march_equinox_and_has_no_thirteenth() {
        assert_eq!(carver_lunation_name(0), Some(&["Worm Moon"][..]));
        assert_eq!(carver_lunation_name(11), Some(&["Snow Moon"][..]));
        // Carver's "lost moon", the thirteenth, is unnamed in his account
        // and is unnamed here.
        assert_eq!(carver_lunation_name(12), None);
    }

    #[test]
    fn the_maine_1937_list_shares_only_four_names_with_the_modern_one() {
        let shared = (1..=12u8)
            .filter(|ordinal| {
                names(&MOON_NAMES_MAINE_1937, Month::regular(*ordinal))
                    .unwrap_or(&[])
                    .iter()
                    .any(|name| MOON_NAMES_OFA_CURRENT.names(name))
            })
            .count();
        assert_eq!(shared, 4);
        for shared_name in ["Wolf Moon", "Flower Moon", "Harvest Moon", "Hunter's Moon"] {
            assert!(MOON_NAMES_MAINE_1937.names(shared_name), "{shared_name}");
            assert!(MOON_NAMES_OFA_CURRENT.names(shared_name), "{shared_name}");
        }
    }

    /// The Harvest Moon rule, against years whose harvest moon dates are
    /// published. 2024's fell on 17–18 September and 2025's on 6–7 October;
    /// 2020's fell on 1 October, the earliest an October harvest moon can.
    #[test]
    fn the_harvest_moon_falls_in_october_in_the_years_it_does() {
        let universal = Meridian::UNIVERSAL;
        assert_eq!(harvest_moon_falls_in(2024, universal), 9);
        assert_eq!(harvest_moon_falls_in(2025, universal), 10);
        assert_eq!(harvest_moon_falls_in(2020, universal), 10);
        assert!(harvest_moon_is_in_october(2025, universal));
        assert!(!harvest_moon_is_in_october(2024, universal));
    }

    #[test]
    fn the_harvest_moon_is_always_in_september_or_october_over_two_centuries() {
        let mut octobers = 0;
        for year in 1900..2100i64 {
            let month = harvest_moon_falls_in(year, Meridian::UNIVERSAL);
            assert!(
                month == 9 || month == 10,
                "{year} put the harvest moon in month {month}"
            );
            if month == 10 {
                octobers += 1;
            }
        }
        // Roughly a quarter of years, which is what a 29.53-day month
        // sliding against a fixed equinox produces.
        assert!(
            (40..=70).contains(&octobers),
            "{octobers} October harvest moons in 200 years"
        );
    }

    /// The defining property, asserted directly rather than via the month:
    /// no other full moon of the year is closer to the equinox.
    #[test]
    fn no_full_moon_is_nearer_the_september_equinox_than_the_harvest_moon() {
        for year in 1990..2050i64 {
            let equinox = september_equinox(year);
            let harvest = harvest_moon(year, Meridian::UNIVERSAL);
            let chosen = full_moons_around_the_equinox(year, Meridian::UNIVERSAL)
                .into_iter()
                .flatten()
                .find(|(day, _)| *day == harvest)
                .map(|(_, moment)| hc_core::math::abs(moment.0 - equinox.0));
            let best = chosen.unwrap_or(f64::MAX);
            for (_, moment) in full_moons_around_the_equinox(year, Meridian::UNIVERSAL)
                .into_iter()
                .flatten()
            {
                assert!(
                    hc_core::math::abs(moment.0 - equinox.0) >= best - 1e-9,
                    "a nearer full moon exists in {year}"
                );
            }
            // Half a synodic month is the most it can ever be.
            assert!(best <= 14.77, "{year}: {best} days from the equinox");
        }
    }

    #[test]
    fn the_hunters_moon_is_the_full_moon_after_the_harvest_moon() {
        for year in 2000..2040i64 {
            let harvest = harvest_moon(year, Meridian::UNIVERSAL);
            let hunters = hunters_moon(year, Meridian::UNIVERSAL);
            let gap = hunters.0 - harvest.0;
            assert!(
                (28..=31).contains(&gap),
                "{year}: {gap} days between harvest and hunter's"
            );
        }
    }

    #[test]
    fn september_is_the_corn_moon_exactly_when_the_harvest_moon_is_in_october() {
        for year in 2015..2035i64 {
            let name = september_moon_name(year, Meridian::UNIVERSAL);
            if harvest_moon_is_in_october(year, Meridian::UNIVERSAL) {
                assert_eq!(name, "Corn Moon", "{year}");
            } else {
                assert_eq!(name, "Harvest Moon", "{year}");
            }
            // Whichever it is, the almanac's September entry lists it.
            assert!(
                names(&MOON_NAMES_OFA_CURRENT, Month::regular(9))
                    .unwrap_or(&[])
                    .contains(&name),
                "{year}"
            );
        }
    }

    /// The meridian shifts which local day a phase instant lands on, but it
    /// cannot move the harvest moon a whole month, because the rule compares
    /// intervals of about a fortnight.
    #[test]
    fn the_meridian_can_move_the_harvest_moon_a_day_but_not_a_month() {
        for year in 2000..2040i64 {
            let universal = harvest_moon(year, Meridian::UNIVERSAL);
            let japan = harvest_moon(year, Meridian::JAPAN);
            assert!(
                (japan.0 - universal.0).abs() <= 1,
                "{year}: {} vs {}",
                universal.0,
                japan.0
            );
        }
    }

    #[test]
    fn a_leap_month_has_no_moon_name() {
        assert_eq!(
            names(&MOON_NAMES_OFA_CURRENT, Month::leap(9)),
            Err(CalendarError::UnsupportedField("leap month"))
        );
    }

    #[test]
    fn a_month_outside_the_year_is_out_of_range() {
        assert_eq!(
            names(&MOON_NAMES_OFA_CURRENT, Month::regular(13)),
            Err(CalendarError::MonthOutOfRange)
        );
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
    tests: moon_name_table_tests,
    all: &ALL,
    lookup: by_id,
}
