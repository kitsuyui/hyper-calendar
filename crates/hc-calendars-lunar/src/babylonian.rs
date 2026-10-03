//! The Babylonian calendar in its regular form — `babylonian`.
//!
//! The system is written up in `docs/systems/babylonian.md` in the
//! repository: what it is, how the cycle and the visibility criterion work
//! with a worked example, what is carried and what is not, and the
//! measurement against Parker and Dubberstein's table. This page summarises
//! it and states the code's own facts.
//!
//! # What this is
//!
//! The lunisolar calendar of Babylonia from 626 BCE to 76 CE: a month
//! begins on the evening the new crescent is first seen from Babylon, and
//! from the fourth century BCE seven years of every nineteen carry a
//! thirteenth month, a second Addaru in the years ≡ 1, 4, 7, 9, 12 and 15
//! (mod 19) of the Seleucid count and a second Ulūlu in the years ≡ 18.
//! Before SE −71 the thirteenth months are where Parker and Dubberstein's
//! table puts them, [`INTERCALATIONS_BEFORE_THE_RULE`]. The rules are those
//! Reingold and Dershowitz give in *Calendrical Calculations* (4th ed.,
//! Cambridge, 2018), following the `babylonian-*` functions of their
//! published source, `calendar.l` in the `calendar-code2` repository
//! (Apache License 2.0), read 2026-09-25; they in turn follow Parker and
//! Dubberstein, *Babylonian
//! Chronology 626 B.C.–A.D. 75* (Brown University Press, 1956; rev. 1971).
//!
//! # The year count
//!
//! Years are counted in the Seleucid era, whose year 1 begins on
//! 1 Nisanu = 3 April 311 BCE in the Julian calendar (RD −113 502). That is
//! the count the tablets themselves carry from the reign of Seleucus I on.
//! Years before SE 1 are the era continued backwards, so that year 0 is
//! 312/311 BCE and year −71 is 383/382 BCE: a modern convention, the one
//! Parker and Dubberstein's transcribers use, and not a count any Babylonian
//! wrote. The tablets of those years are dated by regnal years:
//! [`regnal_year`] gives the king and year van Gent's converter of the
//! table labels each year with, from 1 Interregnum in SE −314 to 11
//! Demetrius I in SE 160 ([`REIGNS`]).
//!
//! # The range, and why
//!
//! SE −314 to SE 386: from 1 Nisanu of 626 BCE, the accession year of
//! Nabopolassar, where Parker and Dubberstein's table begins, to the end of
//! Addaru of 76 CE. From SE −71, 383 BCE, the table follows the
//! nineteen-year rule without exception — its last intercalation outside
//! the rule is in SE −73 — and the rule places the thirteenth month; in the
//! centuries before, in which the king intercalated by decree, the table
//! does, and the months are the same criterion's. The upper bound is where their table ends, with the last
//! cuneiform texts; the Seleucid count went on in Syria for centuries, but
//! on other calendars. A date outside the range is refused
//! ([`CalendarError::BeforeEpoch`] or [`CalendarError::AfterSupportedRange`]),
//! not extrapolated.
//!
//! # The month
//!
//! The day begins at sunset. A month begins on the day whose eve passed the
//! *moonlag* criterion at Babylon (32.4794° N, 44.4328° E, 26 m): at sunset
//! the Moon is at least 24 hours past conjunction and short of first quarter,
//! and it sets more than 48 minutes after the Sun. This is a forecast of an
//! observation, as [`crate::islamic_observational`] is, and Parker and
//! Dubberstein's own month starts are likewise *computed* first visibilities
//! (by Schoch's criterion) rather than the sightings the tablets record.
//! Between the two computations the difference is measured, not assumed:
//! see the accuracy section.
//!
//! Where Reingold and Dershowitz look for the eve's moonset within the
//! standard-time day of UT+3:30, this module's [`hc_astro::moonset`] bounds
//! the day by Babylon's local mean time, 2:58 ahead of UT. The two windows
//! differ by half an hour at each end, and a moonset in that sliver is one
//! the criterion would in any case count as a lag of over twenty hours.
//!
//! # Month numbering
//!
//! Months are 1 Nīsannu, 2 Ayyāru, 3 Sīmannu, 4 Duʾūzu, 5 Ābu, 6 Ulūlu,
//! 7 Tašrītu, 8 Araḫsamna, 9 Kisilīmu, 10 Ṭebētu, 11 Šabāṭu, 12 Addāru, in
//! the normalisation van Gent's converter of Parker and Dubberstein's table
//! prints; `hc-i18n` carries them for English. The intercalary month
//! repeats the month it follows, so a second Addaru is
//! [`Month::leap(12)`](hc_calendar::Month::leap) and a second Ulūlu is
//! `Month::leap(6)`, the same convention the Chinese and Hebrew calendars use
//! here.
//!
//! The days run from 1 to whatever the criterion gives. It is applied to
//! each evening on its own, so a month here runs 29 or 30 days almost
//! always, but 31 when one first evening just clears the 48-minute lag and
//! the thirtieth evening after it just misses, and 28 in the opposite case.
//! The Babylonian rule that a month not seen to end on its thirtieth
//! evening ends anyway is not applied: neither Reingold and Dershowitz nor
//! Parker and Dubberstein apply it to their computed months — the 1971
//! table prints a lunation of 31 days — and applying it would make each
//! month's start depend on the month before it, all the way back. The
//! count of such months in the range is in the accuracy section.
//!
//! # Accuracy
//!
//! Measured against Parker and Dubberstein's table (1971 edition, in
//! R. H. van Gent's transcription at `webspace.science.uu.nl/~gent0113/
//! babylon/`, read 2026-09-25) over its 5 664 months from SE −71: the
//! intercalary month falls in the year and the place the rule gives in
//! every one of the 168 cases, and the first day of the month agrees with
//! the table on the share of months stated in [`PARKER_DUBBERSTEIN_AGREEMENT`],
//! never differing by more than a day. Over the 3 006 months before the
//! rule, whose 90 intercalations are the table's own, the first day agrees
//! on the share in [`PARKER_DUBBERSTEIN_AGREEMENT_BEFORE_THE_RULE`], and two
//! months are two days off. The transcription is not carried in
//! this repository beyond the intercalations; the test that measures against it,
//! `measured_against_parker_dubberstein`, is ignored unless the environment
//! variable `HC_PD_TABLE` names a copy, and the spot checks in the module's
//! tests are the rows of it that the documentation cites.

use hc_astro::{Location, MEAN_SYNODIC_MONTH, moonset, new_moon_before, sunset};
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_core::math::{floor, round};

/// The machine identifier of this calendar. CLDR has none for it.
pub const ID: CalendarId = CalendarId("babylonian");

/// The era code of the Seleucid era.
pub const ERA: &str = "se";

/// The fixed day of 1 Nisanu SE 1: 3 April 311 BCE Julian, which is 29 March
/// of the proleptic Gregorian year −310.
pub const EPOCH: Rd = Rd(-113_502);

/// Babylon: 32°28′46″ N, 44°25′58″ E, 26 m, as *Calendrical Calculations*
/// places it.
pub const BABYLON: Location = Location::new(32.4794, 44.4328, 26.0);

/// The earliest Seleucid year converted, 626/625 BCE, the accession year
/// of Nabopolassar, where Parker and Dubberstein's table begins.
pub const MIN_YEAR: i64 = -314;

/// The first year of the nineteen-year rule without exception, 383/382
/// BCE: before it, the intercalary months are
/// [`INTERCALATIONS_BEFORE_THE_RULE`]'s.
pub const RULE_START: i64 = -71;

/// The intercalary months of Parker and Dubberstein's table before the
/// rule, `(Seleucid year, month repeated)`: a second Ulūlu is 6 and a
/// second Addaru 12. Every year from [`MIN_YEAR`] to `RULE_START − 1` not
/// listed has twelve months. Transcribed from van Gent's copy of the 1971
/// table (`babycal_dat.js`, read 2026-09-25): 90 intercalations in 243
/// years, the last outside the rule in SE −73.
pub const INTERCALATIONS_BEFORE_THE_RULE: [(i16, u8); 90] = [
    (-312, 12),
    (-309, 6),
    (-307, 12),
    (-304, 6),
    (-302, 12),
    (-299, 6),
    (-295, 6),
    (-294, 12),
    (-291, 6),
    (-288, 6),
    (-286, 6),
    (-284, 6),
    (-282, 12),
    (-279, 12),
    (-276, 12),
    (-272, 6),
    (-270, 12),
    (-267, 12),
    (-265, 12),
    (-262, 6),
    (-260, 12),
    (-257, 12),
    (-252, 6),
    (-251, 12),
    (-248, 12),
    (-245, 12),
    (-243, 12),
    (-241, 12),
    (-238, 12),
    (-234, 6),
    (-232, 12),
    (-229, 12),
    (-225, 6),
    (-224, 12),
    (-221, 12),
    (-218, 6),
    (-215, 6),
    (-213, 12),
    (-210, 12),
    (-207, 6),
    (-205, 12),
    (-202, 12),
    (-199, 6),
    (-197, 12),
    (-194, 12),
    (-191, 6),
    (-188, 12),
    (-186, 12),
    (-183, 12),
    (-181, 12),
    (-178, 12),
    (-175, 12),
    (-172, 6),
    (-170, 12),
    (-167, 12),
    (-164, 12),
    (-162, 12),
    (-159, 12),
    (-156, 12),
    (-153, 6),
    (-151, 12),
    (-148, 12),
    (-145, 12),
    (-143, 12),
    (-140, 12),
    (-137, 12),
    (-134, 12),
    (-132, 12),
    (-129, 12),
    (-126, 12),
    (-124, 12),
    (-121, 12),
    (-118, 12),
    (-115, 12),
    (-113, 12),
    (-110, 12),
    (-107, 12),
    (-105, 12),
    (-102, 12),
    (-99, 12),
    (-96, 6),
    (-94, 12),
    (-91, 12),
    (-88, 12),
    (-86, 12),
    (-83, 12),
    (-80, 12),
    (-77, 6),
    (-75, 12),
    (-73, 12),
];

/// The latest Seleucid year converted, 75/76 CE.
pub const MAX_YEAR: i64 = 386;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Parker and Dubberstein 1956 [parker1956]: their table from 1 Nisanu of the accession \
    year of Nabopolassar, 626 BCE, the calendar being older, with the intercalations the king \
    decreed; the nineteen-year rule followed without exception from SE −71, 383 BCE, to \
    29 Addaru of SE 386, 76 CE, where the table ends with the last dated cuneiform texts; \
    later the Seleucid count went on in Syria on other calendars";

/// The earliest fixed day converted: 1 Nisanu SE −314, 5 April 626 BCE
/// Julian, as Parker and Dubberstein's table has it and as the criterion
/// places it.
pub const EARLIEST: Rd = Rd(-228_554);

/// The first day of the nineteen-year rule, 1 Nisanu SE −71: 18 April 383
/// BCE Julian, as the table has it and as the criterion places it.
pub const RULE_EARLIEST: Rd = Rd(-139_785);

/// The latest fixed day converted: 29 Addaru SE 386, 25 March 76 CE Julian,
/// the day before the criterion's 1 Nisanu SE 387.
pub const LATEST: Rd = Rd(27_475);

/// How many of the 5 664 month starts from SE −71 in Parker and
/// Dubberstein's table this module places on the same day, as the ignored
/// measurement test computes it: `(agreeing, total)`. Of the rest, 941 are a
/// day later here and 29 a day earlier; none is further off.
pub const PARKER_DUBBERSTEIN_AGREEMENT: (u32, u32) = (4_694, 5_664);

/// The same for the 3 006 month starts of the table before [`RULE_START`],
/// SE −314 to SE −72, with the intercalations taken from it: 465 are a day
/// later here and 14 a day earlier, and two are two days off: 1 Tašrītu of
/// SE −288, two days later here, and 1 Kisilīmu of SE −200, two days
/// earlier, the month after the one lunation of 31 days the table prints
/// before the rule.
pub const PARKER_DUBBERSTEIN_AGREEMENT_BEFORE_THE_RULE: (u32, u32) = (2_525, 3_006);

/// One reign of the regnal labels: the king as van Gent's converter of
/// Parker and Dubberstein's table spells him, the first Seleucid year
/// labelled with his name, and the regnal year that label carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reign {
    /// The king, or "Interregnum".
    pub king: &'static str,
    /// The first Seleucid year labelled with this reign.
    pub from: i64,
    /// The regnal year of that first label: 1, except for Alexander III,
    /// whose labels begin at 7, and Philip III and Alexander IV, whose
    /// labels begin at 2 because their first years carry their
    /// predecessors' last.
    pub first_labelled: i64,
}

/// The reigns by which the years from SE −314 to SE 160 are labelled,
/// transcribed from van Gent's converter of Parker and Dubberstein's table
/// (`babycal_dat.js`, `babylon_ruler_name` and `babylon_ruler_year`, and
/// the offsets of `babycal.js`; read 2026-09-29), in order. The scripts
/// state no licence, and the converter's page reads "© R.H. van Gent 2011,
/// 2015" (read 2026-09-29), so their code is read and not copied
/// (docs/policy.md §9): what is transcribed is the table's facts, each
/// reign's name and first year, which are Parker and Dubberstein's. A year belongs
/// wholly to the reign the converter names for it: the regnal year runs
/// from 1 Nisannu, and the year in which one king died and the next came to
/// the throne is the old king's last. The converter has no Labashi-Marduk,
/// Bardiya, Nebuchadnezzar III or IV, or Antigonus, whose reigns fall
/// inside a year it gives to another, and it labels no year after SE 160.
pub const REIGNS: [Reign; 29] = [
    Reign {
        king: "Interregnum",
        from: -314,
        first_labelled: 1,
    },
    Reign {
        king: "Nabopolassar",
        from: -313,
        first_labelled: 1,
    },
    Reign {
        king: "Nebuchadnezzar II",
        from: -292,
        first_labelled: 1,
    },
    Reign {
        king: "Amēl-Marduk",
        from: -249,
        first_labelled: 1,
    },
    Reign {
        king: "Nergal-šar-usur",
        from: -247,
        first_labelled: 1,
    },
    Reign {
        king: "Nabunaid",
        from: -243,
        first_labelled: 1,
    },
    Reign {
        king: "Cyrus",
        from: -226,
        first_labelled: 1,
    },
    Reign {
        king: "Cambyses",
        from: -217,
        first_labelled: 1,
    },
    Reign {
        king: "Darius I",
        from: -209,
        first_labelled: 1,
    },
    Reign {
        king: "Xerxes",
        from: -173,
        first_labelled: 1,
    },
    Reign {
        king: "Artaxerxes I",
        from: -152,
        first_labelled: 1,
    },
    Reign {
        king: "Darius II",
        from: -111,
        first_labelled: 1,
    },
    Reign {
        king: "Artaxerxes II Memnon",
        from: -92,
        first_labelled: 1,
    },
    Reign {
        king: "Artaxerxes III Ochus",
        from: -46,
        first_labelled: 1,
    },
    Reign {
        king: "Artaxerxes IV Arses",
        from: -25,
        first_labelled: 1,
    },
    Reign {
        king: "Darius III",
        from: -23,
        first_labelled: 1,
    },
    Reign {
        king: "Alexander III [the Great]",
        from: -18,
        first_labelled: 7,
    },
    Reign {
        king: "Philip III Arrhidaeus",
        from: -10,
        first_labelled: 2,
    },
    Reign {
        king: "Alexander IV Aegus",
        from: -3,
        first_labelled: 2,
    },
    Reign {
        king: "Seleucus I Nicator",
        from: 1,
        first_labelled: 1,
    },
    Reign {
        king: "Antiochus I Soter",
        from: 31,
        first_labelled: 1,
    },
    Reign {
        king: "Antiochus II Theos",
        from: 51,
        first_labelled: 1,
    },
    Reign {
        king: "Seleucus II Callinicus",
        from: 66,
        first_labelled: 1,
    },
    Reign {
        king: "Seleucus III Soter",
        from: 87,
        first_labelled: 1,
    },
    Reign {
        king: "Antiochus III [the Great]",
        from: 90,
        first_labelled: 1,
    },
    Reign {
        king: "Seleucus IV Philopater",
        from: 125,
        first_labelled: 1,
    },
    Reign {
        king: "Antiochus IV Epiphanes",
        from: 137,
        first_labelled: 1,
    },
    Reign {
        king: "Antiochus V Eupator",
        from: 148,
        first_labelled: 1,
    },
    Reign {
        king: "Demetrius I Soter",
        from: 150,
        first_labelled: 1,
    },
];

/// The last Seleucid year the regnal labels reach, SE 160, 152/151 BCE, 11
/// Demetrius I: the converter refreshes its label below SE 161 only.
pub const LAST_REGNAL_YEAR: i64 = 160;

/// The king and regnal year labelling the Seleucid year `year`, as van
/// Gent's converter of Parker and Dubberstein's table labels it: 1
/// Interregnum for SE −314, the accession year of Nabopolassar; 1
/// Nabopolassar for SE −313; 5 Darius III for SE −19; 1 Seleucus I Nicator
/// for SE 1. `None` outside SE −314 to [`LAST_REGNAL_YEAR`].
#[must_use]
pub const fn regnal_year(year: i64) -> Option<(&'static str, i64)> {
    if year < MIN_YEAR || year > LAST_REGNAL_YEAR {
        return None;
    }
    let mut index = REIGNS.len();
    while index > 0 {
        index -= 1;
        let reign = REIGNS[index];
        if reign.from <= year {
            return Some((reign.king, reign.first_labelled + year - reign.from));
        }
    }
    None
}

/// Years in the intercalation cycle.
pub const CYCLE_YEARS: i64 = 19;

/// Months in the intercalation cycle.
pub const CYCLE_MONTHS: i64 = 235;

/// The least lag of moonset behind sunset, in days, for the crescent to
/// count as seen: 48 minutes.
const MINIMUM_MOONLAG: f64 = 48.0 / (24.0 * 60.0);

/// Whether `year` carries a thirteenth month by the nineteen-year rule.
///
/// `(7y + 13) mod 19 < 7` selects the years ≡ 1, 4, 7, 9, 12, 15 and 18
/// (mod 19); the test `the_closed_form_matches_the_listed_leap_years`
/// checks that.
#[must_use]
pub const fn rule_is_leap_year(year: i64) -> bool {
    (7 * year + 13).rem_euclid(CYCLE_YEARS) < 7
}

/// Whether `year` carries a thirteenth month: by
/// [`INTERCALATIONS_BEFORE_THE_RULE`] from [`MIN_YEAR`] to the year before
/// [`RULE_START`], and by [`rule_is_leap_year`] otherwise.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    leap_month(year).is_some()
}

/// Whether `year`'s thirteenth month is a second Ulūlu rather than a second
/// Addaru — under the rule, the years ≡ 18 (mod 19), all of which are leap
/// years.
#[must_use]
pub const fn has_second_ululu(year: i64) -> bool {
    matches!(leap_month(year), Some(6))
}

/// The intercalary month of a year before the rule, from the table.
const fn tabulated_leap_month(year: i64) -> Option<u8> {
    let mut index = 0;
    while index < INTERCALATIONS_BEFORE_THE_RULE.len() {
        let (listed, month) = INTERCALATIONS_BEFORE_THE_RULE[index];
        if listed as i64 == year {
            return Some(month);
        }
        index += 1;
    }
    None
}

/// The ordinal of the month that `year`'s thirteenth month repeats, or
/// `None` in a common year.
#[must_use]
pub const fn leap_month(year: i64) -> Option<u8> {
    if year >= MIN_YEAR && year < RULE_START {
        tabulated_leap_month(year)
    } else if !rule_is_leap_year(year) {
        None
    } else if year.rem_euclid(CYCLE_YEARS) == 18 {
        Some(6)
    } else {
        Some(12)
    }
}

/// Months elapsed from the epoch to 1 Nisanu of `year`: the rule's closed
/// form from [`RULE_START`], and the table's months counted back from it
/// before.
const fn months_before_year(year: i64) -> i64 {
    if year >= RULE_START {
        return rule_months_before_year(year);
    }
    let mut months = rule_months_before_year(RULE_START);
    let mut earlier = RULE_START - 1;
    while earlier >= year {
        months -= if leap_month(earlier).is_some() {
            13
        } else {
            12
        };
        earlier -= 1;
    }
    months
}

/// Months elapsed from the epoch to 1 Nisanu of `year` by the rule.
const fn rule_months_before_year(year: i64) -> i64 {
    ((year - 1) * CYCLE_MONTHS + 13).div_euclid(CYCLE_YEARS)
}

/// The year whose months, counted from the epoch, include the `months`th.
const fn year_of_month(months: i64) -> i64 {
    if months >= months_before_year(RULE_START) {
        return (CYCLE_YEARS * months + 5).div_euclid(CYCLE_MONTHS) + 1;
    }
    let mut year = RULE_START - 1;
    let mut start = months_before_year(RULE_START);
    loop {
        start -= if leap_month(year).is_some() { 13 } else { 12 };
        if months >= start || year < MIN_YEAR {
            return year;
        }
        year -= 1;
    }
}

/// The moonlag between the eve's sunset and the Moon's setting, in days.
///
/// A day on which the Moon does not set is given a lag of a full day, as
/// the source does; the criterion then passes, since a Moon still up at
/// midnight is a Moon that was up at dusk.
fn moonlag(eve: Rd, set: Moment) -> f64 {
    match moonset(eve, BABYLON) {
        Some(moon) => moon.0 - set.0,
        None => 1.0,
    }
}

/// Whether the crescent counts as seen on the evening that begins the day
/// `rd` — the sunset of `rd - 1`.
#[must_use]
pub fn crescent_visible_on_the_eve_of(rd: Rd) -> bool {
    let eve = Rd(rd.0 - 1);
    let Some(set) = sunset(eve, BABYLON) else {
        return false;
    };
    let phase = hc_astro::lunar_phase(set);
    if !(0.0..90.0).contains(&phase) {
        return false;
    }
    if new_moon_before(set).0 > set.0 - 1.0 {
        return false;
    }
    moonlag(eve, set) > MINIMUM_MOONLAG
}

/// The first day of the month containing `rd`.
///
/// # Errors
///
/// Returns [`CalendarError::AstronomicalModelFailure`] when no evening
/// within a lunation and a half passes the criterion, which the model does
/// not do at Babylon's latitude.
pub fn month_start_on_or_before(rd: Rd) -> CalendarResult<Rd> {
    let moon = floor(new_moon_before(Moment(rd.0 as f64)).0) as i64;
    let age = rd.0 - moon;
    // Within three days of the conjunction the crescent may not yet count
    // as seen, in which case the month began a lunation earlier.
    let first = if age <= 3 && !crescent_visible_on_the_eve_of(rd) {
        moon - 30
    } else {
        moon
    };
    for step in 0..45 {
        let candidate = Rd(first + step);
        if crescent_visible_on_the_eve_of(candidate) {
            return Ok(candidate);
        }
    }
    Err(CalendarError::AstronomicalModelFailure)
}

/// The fixed day of a Babylonian date, with no range check on the year.
///
/// # Errors
///
/// Returns [`CalendarError::MonthOutOfRange`] for a month that the year
/// does not have, [`CalendarError::DayOutOfRange`] for a day past 31, or
/// the criterion's failure.
fn to_fixed_unchecked(year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
    if month.ordinal == 0 || month.ordinal > 12 {
        return Err(CalendarError::MonthOutOfRange);
    }
    if month.leap && leap_month(year) != Some(month.ordinal) {
        return Err(CalendarError::MonthOutOfRange);
    }
    if day == 0 || day > 31 {
        return Err(CalendarError::DayOutOfRange);
    }
    // Months of the year elapsed before this one: the intercalary month
    // and, in a year with a second Ulūlu, every month after it come one
    // later than their ordinal says.
    let elapsed_this_year = if month.leap || (has_second_ululu(year) && month.ordinal > 6) {
        i64::from(month.ordinal)
    } else {
        i64::from(month.ordinal) - 1
    };
    let months = months_before_year(year) + elapsed_this_year;
    let midmonth = EPOCH.0 + round(MEAN_SYNODIC_MONTH * months as f64) as i64 + 15;
    let start = month_start_on_or_before(Rd(midmonth))?;
    Ok(Rd(start.0 + i64::from(day) - 1))
}

/// The Babylonian year, month and day of a fixed day, with no range check.
fn from_fixed_unchecked(rd: Rd) -> CalendarResult<(i64, Month, u8)> {
    let crescent = month_start_on_or_before(rd)?;
    let months = round((crescent.0 - EPOCH.0) as f64 / MEAN_SYNODIC_MONTH) as i64;
    let year = year_of_month(months);
    let approx = EPOCH.0 + round(months_before_year(year) as f64 * MEAN_SYNODIC_MONTH) as i64;
    let new_year = month_start_on_or_before(Rd(approx + 15))?;
    let position = 1 + round((crescent.0 - new_year.0) as f64 / 29.5) as i64;
    let special = has_second_ululu(year);
    let leap = if special {
        position == 7
    } else {
        position == 13
    };
    let ordinal = if leap || (special && position > 6) {
        position - 1
    } else {
        position
    };
    let month = Month {
        ordinal: ordinal as u8,
        leap,
    };
    let day = (rd.0 - crescent.0 + 1) as u8;
    Ok((year, month, day))
}

/// Range check.
fn check_range(rd: Rd) -> CalendarResult<()> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    Ok(())
}

/// The fixed day of a Babylonian date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside SE −314 to 386,
/// [`CalendarError::MonthOutOfRange`] for a month the year does not have —
/// a leap month in a common year, or the wrong one — and
/// [`CalendarError::DayOutOfRange`] for a day the month does not have.
pub fn to_fixed(year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    let rd = to_fixed_unchecked(year, month, day)?;
    check_range(rd)?;
    // The length of a month is not knowable before the sky is asked, so a
    // 30th day is validated by reading the date back.
    let (round_year, round_month, round_day) = from_fixed_unchecked(rd)?;
    if (round_year, round_month) != (year, month) {
        return Err(CalendarError::DayOutOfRange);
    }
    if round_day != day {
        return Err(CalendarError::DayOutOfRange);
    }
    Ok(rd)
}

/// The Babylonian year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before 1 Nisanu SE −314 and
/// [`CalendarError::AfterSupportedRange`] after 30 Addaru SE 386.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, Month, u8)> {
    check_range(rd)?;
    from_fixed_unchecked(rd)
}

/// A date in the Babylonian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BabylonianDate {
    /// The Seleucid year, negative before the era's first year.
    pub year: i64,
    /// The month, Nisanu first, with a second Ulūlu as `Month::leap(6)` and a
    /// second Addaru as `Month::leap(12)`.
    pub month: Month,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl BabylonianDate {
    /// A validated Babylonian date.
    ///
    /// # Errors
    ///
    /// The errors of [`to_fixed`].
    pub fn new(year: i64, month: Month, day: u8) -> CalendarResult<Self> {
        to_fixed(year, month, day)?;
        Ok(Self { year, month, day })
    }
}

/// The Babylonian calendar of the Seleucid era.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BabylonianCalendar;

impl Calendar for BabylonianCalendar {
    type Date = BabylonianDate;

    /// SE −314 to SE 386, which is also the whole of the range it
    /// converts: the years Parker and Dubberstein's table covers, the
    /// calendar itself being older.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(EARLIEST, LATEST, USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// A year with a second Addaru or a second Ulūlu.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    /// The Babylonian day begins at sunset, which is when its month is
    /// decided, and is named by the civil day it ends on: the crescent is
    /// judged "on eve of" a day, at the sunset of the day before (Reingold
    /// and Dershowitz, `calendar-code2`, `babylonian-criterion`).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset(hc_calendar::DayNaming::ByEnd)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Babylonian (Seleucid era)",
            year_kind: YearKind::Astronomical,
            has_leap_months: true,
            is_astronomical: true,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["akk"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(BabylonianDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut fields = DateFields::new(date.year).with_era(ERA);
        fields.month = Some(date.month);
        fields.day = Some(date.day);
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        BabylonianDate::new(fields.year, fields.require_month()?, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::gregorian;

    /// Parker and Dubberstein's first days of the month, as fixed days, for
    /// the rows the module documentation cites: 1 Nisanu of every
    /// nineteenth year from SE −56, and every intercalary month of the first
    /// two cycles of the era. `(year, month, leap, fixed day)`; the comment
    /// is the table's Julian date.
    const PARKER_DUBBERSTEIN: &[(i64, u8, bool, i64)] = &[
        (-56, 1, false, -134_322), // -367-04-02
        (-37, 1, false, -127_382), // -348-04-02
        (-18, 1, false, -120_442), // -329-04-03
        (1, 1, false, -113_502),   // -310-04-03
        (1, 12, true, -113_148),   // -309-03-23
        (4, 12, true, -112_056),   // -306-03-19
        (7, 12, true, -110_963),   // -303-03-16
        (9, 12, true, -110_224),   // -301-03-25
        (12, 12, true, -109_132),  // -298-03-21
        (15, 12, true, -108_040),  // -295-03-17
        (18, 6, true, -107_124),   // -293-09-19
        (20, 1, false, -106_562),  // -291-04-03
        (20, 12, true, -106_208),  // -290-03-23
        (23, 12, true, -105_116),  // -287-03-19
        (26, 12, true, -104_023),  // -284-03-16
        (28, 12, true, -103_284),  // -282-03-25
        (31, 12, true, -102_193),  // -279-03-20
        (34, 12, true, -101_100),  // -276-03-17
        (37, 6, true, -100_185),   // -274-09-18
        (39, 1, false, -99_623),   // -272-04-02
        (58, 1, false, -92_683),   // -253-04-03
        (77, 1, false, -85_744),   // -234-04-02
        (96, 1, false, -78_804),   // -215-04-02
        (115, 1, false, -71_864),  // -196-04-02
        (134, 1, false, -64_924),  // -177-04-03
        (153, 1, false, -57_984),  // -158-04-03
        (172, 1, false, -51_045),  // -139-04-02
        (191, 1, false, -44_106),  // -120-04-01
        (210, 1, false, -37_166),  // -101-04-02
        (229, 1, false, -30_226),  // -82-04-02
        (248, 1, false, -23_286),  // -63-04-02
        (267, 1, false, -16_346),  // -44-04-02
        (286, 1, false, -9_407),   // -25-04-02
        (305, 1, false, -2_467),   // -6-04-02
        (324, 1, false, 4_472),    // 13-04-01
        (343, 1, false, 11_412),   // 32-04-01
        (362, 1, false, 18_351),   // 51-04-01
        (381, 1, false, 25_291),   // 70-04-01
    ];

    fn month(ordinal: u8, leap: bool) -> Month {
        Month { ordinal, leap }
    }

    #[test]
    fn the_epoch_is_the_third_of_april_311_bce() {
        // 3 April 311 BCE Julian is 29 March of the proleptic Gregorian
        // year −310, five days earlier, as every date of that century is.
        assert_eq!(EPOCH, gregorian::to_fixed_saturating(-310, 3, 29));
        assert_eq!(from_fixed(EPOCH), Ok((1, month(1, false), 1)));
        assert_eq!(to_fixed(1, month(1, false), 1), Ok(EPOCH));
    }

    #[test]
    fn the_closed_form_matches_the_listed_leap_years() {
        let leap_years: [i64; 7] = [1, 4, 7, 9, 12, 15, 18];
        for position in 0..19 {
            assert_eq!(
                is_leap_year(position),
                leap_years.contains(&position),
                "year ≡ {position}"
            );
            assert_eq!(is_leap_year(position - 19), is_leap_year(position));
            assert_eq!(is_leap_year(position + 380), is_leap_year(position));
        }
        assert_eq!(leap_month(18), Some(6));
        assert_eq!(leap_month(-1), Some(6));
        assert_eq!(leap_month(1), Some(12));
        assert_eq!(leap_month(2), None);
    }

    #[test]
    fn the_cited_rows_of_parker_and_dubberstein() {
        let mut exact = 0;
        for &(year, ordinal, leap, table) in PARKER_DUBBERSTEIN {
            let computed = to_fixed(year, month(ordinal, leap), 1).expect("in range");
            let difference = computed.0 - table;
            assert!(
                (0..=1).contains(&difference),
                "SE {year} month {ordinal}{}: {difference} days off",
                if leap { "b" } else { "" }
            );
            if difference == 0 {
                exact += 1;
            }
            // Whatever the day, the month is the table's month.
            let (round_year, round_month, _) = from_fixed(Rd(table + 5)).expect("in range");
            assert_eq!((round_year, round_month), (year, month(ordinal, leap)));
        }
        // Six of the thirty-eight cited rows are a day later here: the
        // intercalary months of SE 15, 31, 34 and 37 and the new years of
        // SE 362 and 381.
        assert_eq!(exact, PARKER_DUBBERSTEIN.len() - 6, "rows on the same day");
    }

    #[test]
    fn the_range_is_the_tables_regular_span() {
        assert_eq!(from_fixed(EARLIEST), Ok((MIN_YEAR, month(1, false), 1)));
        assert_eq!(
            from_fixed(LATEST).map(|(y, m, _)| (y, m)),
            Ok((MAX_YEAR, month(12, false)))
        );
        assert_eq!(
            from_fixed_unchecked(Rd(LATEST.0 + 1)),
            Ok((MAX_YEAR + 1, month(1, false), 1))
        );
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            to_fixed(MIN_YEAR - 1, month(12, false), 29),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(MAX_YEAR + 1, month(1, false), 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    /// The intercalations before the rule are the table's, the year before
    /// the rule is common where the rule would have made it leap, and the
    /// months of every year before the rule come twelve or thirteen, the
    /// thirteenth where the table puts it.
    #[test]
    fn the_years_before_the_rule_intercalate_as_the_table_does() {
        assert_eq!(leap_month(-73), Some(12));
        assert_eq!(leap_month(-72), None);
        assert!(rule_is_leap_year(-72));
        assert_eq!(leap_month(-71), None);
        assert_eq!(leap_month(-69), Some(12));
        assert_eq!(leap_month(-312), Some(12));
        assert_eq!(leap_month(-309), Some(6));
        assert!(has_second_ululu(-309));
        assert_eq!(leap_month(-314), None);
        assert_eq!(
            (MIN_YEAR..RULE_START)
                .filter(|year| is_leap_year(*year))
                .count(),
            INTERCALATIONS_BEFORE_THE_RULE.len()
        );
        assert!(
            INTERCALATIONS_BEFORE_THE_RULE
                .windows(2)
                .all(|pair| pair[0].0 < pair[1].0)
        );
        // The table's first and last rows: 1 Nisannu SE −314 on 5 April
        // 626 BCE, 1 Nisannu SE −71 on 18 April 383 BCE.
        assert_eq!(to_fixed(MIN_YEAR, month(1, false), 1), Ok(EARLIEST));
        assert_eq!(EARLIEST, julian_to_fixed(-625, 4, 5));
        assert_eq!(to_fixed(RULE_START, month(1, false), 1), Ok(RULE_EARLIEST));
        let step = if cfg!(debug_assertions) { 37 } else { 1 };
        hc_core::memo::scope(|| {
            for year in (MIN_YEAR..RULE_START).step_by(step) {
                let end = to_fixed(year + 1, month(1, false), 1).expect("in range");
                let mut cursor = to_fixed(year, month(1, false), 1).expect("in range");
                let mut count = 0u8;
                let mut leap = None;
                while cursor < end {
                    let (_, this, day) = from_fixed(cursor).expect("in range");
                    assert_eq!(day, 1, "RD {cursor} is not a month start");
                    if this.leap {
                        leap = Some(this.ordinal);
                    }
                    count += 1;
                    cursor = month_start_on_or_before(Rd(cursor.0 + 32)).expect("converges");
                }
                assert_eq!(cursor, end, "SE {year}");
                assert_eq!(count, if is_leap_year(year) { 13 } else { 12 }, "SE {year}");
                assert_eq!(leap, leap_month(year), "SE {year} leap");
            }
        });
    }

    /// The regnal labels, and the dated examples that pair one with a
    /// Julian day: "13 Ulūlū in the 5th year of Darius III [20 September
    /// 331 BCE]" (van Gent's introduction, the eclipse of BM 36390); "on
    /// the second day of the month of Adar [16 March]" in the seventh year
    /// of Nebuchadnezzar (the Babylonian Chronicle, in Wikipedia's "Siege
    /// of Jerusalem (597 BC)", secondary); 568 BC as his thirty-seventh
    /// year (VAT 4956, in Wikipedia, secondary).
    #[test]
    fn the_regnal_labels_are_the_converters() {
        let julian = julian_to_fixed;
        assert_eq!(regnal_year(-315), None);
        assert_eq!(regnal_year(-314), Some(("Interregnum", 1)));
        assert_eq!(regnal_year(-313), Some(("Nabopolassar", 1)));
        assert_eq!(regnal_year(-293), Some(("Nabopolassar", 21)));
        assert_eq!(regnal_year(-292), Some(("Nebuchadnezzar II", 1)));
        assert_eq!(regnal_year(-256), Some(("Nebuchadnezzar II", 37)));
        assert_eq!(regnal_year(-227), Some(("Nabunaid", 17)));
        assert_eq!(regnal_year(-226), Some(("Cyrus", 1)));
        assert_eq!(regnal_year(-19), Some(("Darius III", 5)));
        assert_eq!(regnal_year(-18), Some(("Alexander III [the Great]", 7)));
        assert_eq!(regnal_year(-11), Some(("Alexander III [the Great]", 14)));
        assert_eq!(regnal_year(-10), Some(("Philip III Arrhidaeus", 2)));
        assert_eq!(regnal_year(0), Some(("Alexander IV Aegus", 5)));
        assert_eq!(regnal_year(1), Some(("Seleucus I Nicator", 1)));
        assert_eq!(
            regnal_year(LAST_REGNAL_YEAR),
            Some(("Demetrius I Soter", 11))
        );
        assert_eq!(regnal_year(LAST_REGNAL_YEAR + 1), None);
        assert!(REIGNS.windows(2).all(|pair| pair[0].from < pair[1].from));
        // The table puts 13 Ulūlu on 20 September; the criterion begins
        // that Ulūlu a day later, as it does 941 of the 5 664 months from
        // the rule on (`PARKER_DUBBERSTEIN_AGREEMENT`).
        assert_eq!(to_fixed(-19, month(6, false), 12), Ok(julian(-330, 9, 20)));
        assert_eq!(to_fixed(-286, month(12, false), 2), Ok(julian(-596, 3, 16)));
        let (year, _, _) = from_fixed(julian(-567, 6, 1)).expect("in range");
        assert_eq!(regnal_year(year), Some(("Nebuchadnezzar II", 37)));
    }

    /// The fixed day of a Julian date in astronomical years, by the Julian
    /// Day Number's arithmetic (Richards, *Explanatory Supplement*, 3rd
    /// ed., 2013, §15.11.3, Algorithm 3 with the Julian parameters), less
    /// the 1 721 425 days from JDN 0 to RD 0.
    fn julian_to_fixed(year: i64, month: u8, day: u8) -> Rd {
        let a = (14 - i64::from(month)) / 12;
        let y = year + 4800 - a;
        let m = i64::from(month) + 12 * a - 3;
        let jdn = i64::from(day) + (153 * m + 2) / 5 + 365 * y + y.div_euclid(4) - 32_083;
        Rd(jdn - 1_721_425)
    }

    #[test]
    fn the_calendar_round_trips_every_day_of_its_range() {
        // Every day of the 256 030 is two crescent searches to read and
        // three to write back, about 1.5 ms of one core in a release build,
        // so a release build walks every one spread over the machine's
        // threads. A debug build, which the coverage job runs instrumented,
        // takes every 776th day, 40 days at each end, and every 1 Nisanu,
        // where the year turns, with the day before it (docs/policy.md §7);
        // a build instrumented for coverage takes a third as many of the
        // days, 13 days at each end and the 1 Nisanu of every seventh year.
        // `arsacid-era` in hc-calendars-regional is this calendar renamed
        // and rests on this sweep for its own days.
        let openings: std::vec::Vec<i64> = if cfg!(debug_assertions) {
            (MIN_YEAR..=MAX_YEAR)
                .step_by(hc_core::sweep::year_step())
                .map(|year| to_fixed(year, month(1, false), 1).expect("in range").0)
                .chain([LATEST.0 + 1])
                .collect()
        } else {
            std::vec::Vec::new()
        };
        let ends = 400 / crate::sweep_stride(10) as i64;
        let mut days: std::vec::Vec<i64> =
            crate::sweep_days(EARLIEST.0, LATEST.0, 776, openings.iter().copied())
                .chain((0..ends).flat_map(|offset| [EARLIEST.0 + offset, LATEST.0 - offset]))
                .collect();
        days.sort_unstable();
        days.dedup();
        assert!(days.contains(&EARLIEST.0) && days.contains(&LATEST.0));
        if cfg!(debug_assertions) {
            // SE −314 opens on the first day, and SE 387's eve is the last.
            let held = |day: i64| days.binary_search(&day).is_ok();
            assert!(openings.iter().all(|&day| {
                (day > LATEST.0 || held(day)) && (day <= EARLIEST.0 || held(day - 1))
            }));
        } else {
            assert_eq!(days.len() as i64, LATEST.0 - EARLIEST.0 + 1);
        }
        crate::check_days(&days, |day| {
            let rd = Rd(day);
            let (year, month, day) = from_fixed(rd).expect("in range");
            assert!(
                (MIN_YEAR..=MAX_YEAR).contains(&year),
                "RD {rd:?} gave SE {year}"
            );
            assert_eq!(to_fixed(year, month, day), Ok(rd), "RD {rd:?}");
        });
    }

    #[test]
    fn years_have_twelve_or_thirteen_months_in_the_cycles_places() {
        // One memo for the nineteen-year cycle's years: each month's
        // search asks again the evenings the search before it judged.
        hc_core::memo::scope(|| {
            for year in 1..=38 {
                let end = to_fixed(year + 1, month(1, false), 1).expect("in range");
                let mut cursor = to_fixed(year, month(1, false), 1).expect("in range");
                let mut count = 0u8;
                let mut second_ululu = false;
                while cursor < end {
                    let (_, this, day) = from_fixed(cursor).expect("in range");
                    assert_eq!(day, 1, "RD {cursor} is not a month start");
                    second_ululu |= this == month(6, true);
                    count += 1;
                    let next = month_start_on_or_before(Rd(cursor.0 + 32)).expect("converges");
                    let length = next.0 - cursor.0;
                    assert!(
                        (28..=31).contains(&length),
                        "SE {year}: a month of {length} days"
                    );
                    cursor = next;
                }
                assert_eq!(cursor, end, "SE {year}");
                assert_eq!(count, if is_leap_year(year) { 13 } else { 12 }, "SE {year}");
                assert_eq!(second_ululu, has_second_ululu(year), "SE {year}");
            }
        });
    }

    #[test]
    fn a_leap_month_is_refused_where_the_cycle_has_none() {
        assert_eq!(
            to_fixed(2, month(12, true), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(1, month(6, true), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(18, month(12, true), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert!(to_fixed(18, month(6, true), 1).is_ok());
        assert_eq!(
            to_fixed(1, month(13, false), 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(1, month(1, false), 31),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(1, month(1, false), 0),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn a_day_exists_only_where_the_month_reaches_it() {
        // The month's length is whatever the criterion gave, so the last day
        // is accepted and the one after it refused, month by month.
        let mut start = EPOCH;
        for _ in 0..12 {
            let next = month_start_on_or_before(Rd(start.0 + 32)).expect("converges");
            let length = (next.0 - start.0) as u8;
            let (year, month, _) = from_fixed(start).expect("in range");
            assert_eq!(to_fixed(year, month, length), Ok(Rd(next.0 - 1)));
            assert_eq!(
                to_fixed(year, month, length + 1),
                Err(CalendarError::DayOutOfRange)
            );
            start = next;
        }
        // Nisanu SE 1 ran 29 days: the table's 1 Aiaru is 2 May, and so is this one's.
        assert_eq!(
            to_fixed(1, month(1, false), 30),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(to_fixed(1, month(2, false), 1), Ok(Rd(EPOCH.0 + 29)));
    }

    #[test]
    fn fields_round_trip_through_the_generic_interface() {
        let calendar = BabylonianCalendar;
        let date = calendar.from_fixed(Rd(-107_124)).expect("in range");
        assert_eq!(
            date,
            BabylonianDate {
                year: 18,
                month: month(6, true),
                day: 1
            }
        );
        let fields = calendar.to_fields(date).expect("fields");
        assert_eq!(fields.era, Some("se"));
        assert_eq!(fields.month, Some(month(6, true)));
        assert_eq!(calendar.from_fields(&fields), Ok(date));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(18, 1, 1).with_era("AH")),
            Err(CalendarError::UnknownEra)
        );
        let meta = calendar.meta();
        assert_eq!(meta.id, CalendarId("babylonian"));
        assert!(meta.is_astronomical);
        assert!(meta.has_leap_months);
        assert_eq!(meta.year_kind, YearKind::Astronomical);
    }

    /// Measures every month start from SE −314 against a copy of Parker and
    /// Dubberstein's table, one line per month, `<SE year> <month> <leap
    /// 0|1> <fixed day>`, named by `HC_PD_TABLE`. Run with
    /// `HC_PD_TABLE=... cargo test -p hc-calendars-lunar --release
    /// -- --ignored --nocapture measured_against`.
    #[test]
    #[ignore = "needs a copy of the table, named by HC_PD_TABLE"]
    fn measured_against_parker_dubberstein() {
        let Ok(path) = std::env::var("HC_PD_TABLE") else {
            return;
        };
        let table = std::fs::read_to_string(path).expect("the table is readable");
        for before_the_rule in [false, true] {
            measure(&table, before_the_rule);
        }
    }

    /// One half of [`measured_against_parker_dubberstein`]: the months from
    /// [`RULE_START`], or the months before it.
    fn measure(table: &str, before_the_rule: bool) {
        let mut total = 0u32;
        let mut same = 0u32;
        let mut early = 0u32;
        let mut late = 0u32;
        let mut worst = 0i64;
        let mut leap_wrong = 0u32;
        let mut leaps = 0u32;
        let mut lengths = [0u32; 32];
        let mut previous: Option<i64> = None;
        for line in table.lines() {
            let fields: Vec<i64> = line
                .split_whitespace()
                .map(|f| f.parse().unwrap())
                .collect();
            let [year, ordinal, leap, rd] = fields[..] else {
                continue;
            };
            if year < MIN_YEAR {
                continue;
            }
            if (year < RULE_START) != before_the_rule {
                continue;
            }
            total += 1;
            let month = month(ordinal as u8, leap == 1);
            if month.leap {
                leaps += 1;
                if leap_month(year) != Some(month.ordinal) {
                    leap_wrong += 1;
                    continue;
                }
            }
            let computed = to_fixed(year, month, 1).expect("in range");
            if let Some(last) = previous {
                lengths[(computed.0 - last) as usize] += 1;
            }
            previous = Some(computed.0);
            let difference = computed.0 - rd;
            worst = worst.max(difference.abs());
            match difference {
                0 => same += 1,
                d if d < 0 => early += 1,
                _ => late += 1,
            }
            if difference != 0 {
                std::println!(
                    "SE {year} month {ordinal}{}: {difference:+}",
                    if leap == 1 { "b" } else { "" }
                );
            }
        }
        std::println!(
            "months {total}, same day {same}, earlier {early}, later {late}, worst {worst}; intercalary {leaps}, misplaced {leap_wrong}"
        );
        std::println!(
            "month lengths: 28 × {}, 29 × {}, 30 × {}, 31 × {}",
            lengths[28],
            lengths[29],
            lengths[30],
            lengths[31]
        );
        let expected = if before_the_rule {
            PARKER_DUBBERSTEIN_AGREEMENT_BEFORE_THE_RULE
        } else {
            PARKER_DUBBERSTEIN_AGREEMENT
        };
        assert_eq!(total, expected.1);
        assert_eq!(same, expected.0);
        assert_eq!(leap_wrong, 0);
        assert!(worst <= if before_the_rule { 2 } else { 1 });
    }
}
