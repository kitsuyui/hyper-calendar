//! Jupiter in the sidereal zodiac: its longitude, its sign, and the moments it
//! changes sign. Behind the `jupiter` feature.
//!
//! The twelve-year festivals of India (the Kumbh Mela and Pushkaram, in
//! `hc-calendars-indic`) are set by Jupiter's sidereal sign. This module
//! reads it from `hc_astro::jupiter`, the VSOP87B series for Jupiter, and the
//! same ayanāṃśa the Sun's sidereal longitude takes ([`super::sidereal`]);
//! `docs/systems/jupiter-festivals.md` has the worked examples and what the
//! agreement with Drik Panchang's published entries comes to.
//!
//! # The longitude
//!
//! The sidereal longitude is Jupiter's **apparent** geocentric longitude in
//! the true equinox of the date (nutation included) less the ayanāṃśa of the
//! date, exactly as [`super::sidereal::sidereal_longitude`] does for the Sun.
//! Against Drik Panchang's 49 entries from 2001 to 2030 that reading leaves a
//! difference that is the same to within 1″ for every one, 25″, which is the
//! difference of the ayanāṃśa's value; leaving the nutation out gives one
//! that wanders by ±17″ with the 18.6-year cycle of the node.
//!
//! # The ingresses
//!
//! Jupiter turns back out of a sign it has just entered about two years in
//! three, and enters again. [`Ingress`] is one crossing of a boundary, in
//! either direction, and [`ingresses`] finds them in order. The Pushkaram of a
//! river begins at an entry; which entry is the question [`EntryRule`] names.

use hc_astro::jupiter::{self, DEFAULT_TRUNCATION};
use hc_calendar::fixed::Moment;
use hc_core::math::{floor, modulo, normalize_degrees, signed_degrees};

use super::sidereal::{Ayanamsa, SiderealSign};
use super::{DEGREES_PER_SIGN, SIGNS_PER_ZODIAC};

/// The most Jupiter moves in a day, in degrees: its fastest geocentric speed
/// is about 0.25°, near opposition at perihelion. Used to say how far a step
/// cannot reach a boundary.
const FASTEST_DAY_DEGREES: f64 = 0.3;

/// The grid the search for an ingress is laid on, in days: every moment it
/// looks at is a whole number of these from the moment 0.0 (RD 0), and a
/// crossing is found in the one cell of the grid that holds it. A power of two,
/// so that the grid and the halves of a cell are exact in a `f64`.
///
/// The search is *canonical*: the ingress it finds is the same whatever span
/// it is asked for, because the cell and the bisection of the cell are fixed
/// by the sky, not by where the walk began. A step of the walk is a whole
/// number of cells and, by the speed Jupiter cannot exceed, one of more than a
/// cell cannot cross a boundary, so a crossing is always found in a single
/// cell. Where Jupiter is within 0.04° of a boundary the step is one cell,
/// and a station there makes the search slow, not wrong.
const GRID_DAYS: f64 = 0.125;

/// How closely an ingress is found, in days: about a tenth of a second.
const PRECISION_DAYS: f64 = 1e-6;

/// The slowest Jupiter may cross a boundary, in degrees a day, for the
/// crossing to be found by [`Ingresses::cross`]'s probes. Jupiter's
/// longitude has a second derivative of at most 0.004° a day² (measured over
/// 4 200 days at seven epochs), so across the 0.125 day of a cell its speed
/// changes by under 0.001°, and a bracket whose mean speed is 0.02° a day
/// or more has a longitude that rises all the way through it: it crosses the
/// boundary once.
const MIN_CROSSING_DEGREES_PER_DAY: f64 = 0.02;

/// How far from the boundary, in degrees, a longitude must be for the side of
/// the boundary it is on to settle the sides of the moments around it. The
/// longitude is not smooth at the last digits: the Earth's velocity for the
/// aberration is a difference over 0.001 day of positions whose argument is
/// rounded to the last bit of a number of some tens of centuries, which
/// leaves a jitter of up to 3 × 10⁻⁹° in the longitude. This is thirty
/// times that, and a moment within it of the boundary is evaluated.
const SURE_DEGREES: f64 = 1e-7;

/// Jupiter's apparent sidereal longitude at a Universal Time moment, in
/// degrees from 0 up to but not including 360: its apparent geocentric
/// longitude in the true equinox of the date less the ayanāṃśa.
#[must_use]
pub fn sidereal_longitude(moment: Moment, ayanamsa: Ayanamsa) -> f64 {
    normalize_degrees(
        jupiter::apparent(moment, DEFAULT_TRUNCATION).longitude_degrees
            - ayanamsa.degrees_at(moment),
    )
}

/// Where Jupiter is at a moment, in the tropical and the sidereal zodiac.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// The position as seen from the Earth, in the tropical zodiac: apparent
    /// longitude in the true equinox of the date, latitude and distance.
    pub apparent: jupiter::Apparent,
    /// The apparent sidereal longitude in degrees, 0 up to 360.
    pub sidereal_longitude_degrees: f64,
    /// The sidereal sign.
    pub sign: SiderealSign,
    /// How far into the sign, in degrees from 0 up to 30.
    pub degrees_into_sign: f64,
    /// The longitude's rate of change in degrees a day: negative while
    /// Jupiter is in retrograde.
    pub daily_motion_degrees: f64,
}

/// Jupiter's position at a Universal Time moment, tropical and sidereal by
/// `ayanamsa`.
#[must_use]
pub fn position(moment: Moment, ayanamsa: Ayanamsa) -> Position {
    let apparent = jupiter::apparent(moment, DEFAULT_TRUNCATION);
    let longitude = normalize_degrees(apparent.longitude_degrees - ayanamsa.degrees_at(moment));
    let sign = sign_of(longitude);
    Position {
        apparent,
        sidereal_longitude_degrees: longitude,
        sign,
        degrees_into_sign: longitude - sign.start_longitude_degrees(),
        daily_motion_degrees: jupiter::daily_motion_degrees(moment),
    }
}

/// The sidereal sign Jupiter is in at a moment.
#[must_use]
pub fn sign_at(moment: Moment, ayanamsa: Ayanamsa) -> SiderealSign {
    sign_of(sidereal_longitude(moment, ayanamsa))
}

/// The sign a sidereal longitude is in.
fn sign_of(longitude: f64) -> SiderealSign {
    SiderealSign::at(floor(longitude / DEGREES_PER_SIGN) as u8)
}

/// Jupiter's crossing of the boundary between two sidereal signs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ingress {
    /// The moment, in Universal Time, good to a tenth of a second of the
    /// model's own longitude.
    pub moment: Moment,
    /// The sign Jupiter leaves.
    pub from: SiderealSign,
    /// The sign Jupiter enters.
    pub to: SiderealSign,
}

impl Ingress {
    /// Whether Jupiter moves on to the next sign, as against turning back
    /// into the one it has left.
    #[must_use]
    pub fn is_forward(&self) -> bool {
        self.to == self.from.next()
    }
}

/// The ingresses of Jupiter from a moment until another, in order.
#[derive(Debug, Clone, Copy)]
pub struct Ingresses {
    /// The moment the walk is at: a whole number of [`GRID_DAYS`].
    cursor: Moment,
    /// The first moment an ingress is given for.
    from: f64,
    until: Moment,
    /// The sign Jupiter is in at `cursor`.
    sign: SiderealSign,
    ayanamsa: Ayanamsa,
    /// The sidereal longitude at `cursor`, when the search has just worked
    /// it out at the end of a step and the next step begins from there: the
    /// series is the cost, and it is the same number whichever step asks.
    known: Option<f64>,
}

/// The ingresses of Jupiter at or after `from` and before `until`, in
/// order, in the zodiac of `ayanamsa`.
///
/// The search is **canonical**: an ingress is the same instant, to the last
/// bit, from whatever `from` and `until` it is asked, so every export that
/// gives one agrees with every other. The walk goes forward from the grid
/// point at or before `from` in steps, a whole number of cells of 1/8 day,
/// that the distance to the nearest boundary and Jupiter's greatest speed say
/// cannot cross one; the cell in which the sign changes is the one cell that
/// holds the crossing, and is bisected from its two ends to
/// 10⁻⁶ day. Nothing in that depends on where the walk began.
#[must_use]
pub fn ingresses(from: Moment, until: Moment, ayanamsa: Ayanamsa) -> Ingresses {
    let start = floor(from.0 / GRID_DAYS) * GRID_DAYS;
    let longitude = sidereal_longitude(Moment(start), ayanamsa);
    Ingresses {
        cursor: Moment(start),
        from: from.0,
        until,
        sign: sign_of(longitude),
        ayanamsa,
        known: Some(longitude),
    }
}

impl Ingresses {
    /// Jupiter's sidereal longitude at a moment, for this search.
    fn longitude_at(&self, at: f64) -> f64 {
        sidereal_longitude(Moment(at), self.ayanamsa)
    }

    /// The moment the sign changes within a cell of the grid, found by
    /// bisection of the cell: `behind` is the moment and the longitude at its
    /// start, in the sign, and `ahead` those at its end, in `entered`. The
    /// answer is the end of the last interval of the bisection.
    ///
    /// The bisection is the plain one, midpoint after midpoint, and its
    /// result is the same to the last bit; what is saved is the series
    /// evaluated at each midpoint. Where Jupiter crosses the boundary at
    /// [`MIN_CROSSING_DEGREES_PER_DAY`] or more, its longitude rises through
    /// the boundary once within the step, so a midpoint no later than a
    /// moment sure to be in the sign is in it, and one no earlier than a
    /// moment sure to be beyond it is beyond ("sure" being [`SURE_DEGREES`]
    /// from the boundary, so that the jitter of the last digits does not
    /// matter). Four probes find two such moments close to the crossing:
    /// where the line through the two ends crosses the boundary, again
    /// through the nearer ends it leaves, and a moment either side of that.
    /// Only a midpoint between them is evaluated. A step of more than a cell
    /// (which no crossing needs), or a crossing that is slow, is bisected with
    /// every midpoint evaluated.
    fn cross(&self, behind: (f64, f64), ahead: (f64, f64), entered: SiderealSign) -> f64 {
        let (start, end) = (behind.0, ahead.0);
        // How far past the boundary Jupiter is, in degrees: negative while
        // it is in the sign.
        let forward = entered == self.sign.next();
        let edge = normalize_degrees(
            self.sign.start_longitude_degrees() + if forward { DEGREES_PER_SIGN } else { 0.0 },
        );
        let past = |longitude: f64| {
            let degrees = signed_degrees(longitude - edge);
            if forward { degrees } else { -degrees }
        };
        let speed = (past(ahead.1) - past(behind.1)) / (end - start);
        let guided = end - start <= GRID_DAYS
            && speed >= MIN_CROSSING_DEGREES_PER_DAY
            && past(behind.1) <= 0.0
            && past(ahead.1) >= 0.0;
        // What is known: the nearest moments evaluated either side, and the
        // moments sure to be on each.
        let mut nearest = (behind, ahead);
        let mut sure = (start, end);
        // Evaluate at a moment, and say whether it is in the sign.
        let probe = |at: f64, nearest: &mut ((f64, f64), (f64, f64)), sure: &mut (f64, f64)| {
            let longitude = self.longitude_at(at);
            let in_sign = sign_of(longitude) == self.sign;
            let settles = past(longitude).abs() >= SURE_DEGREES;
            if in_sign {
                nearest.0 = (at, longitude);
                if settles {
                    sure.0 = sure.0.max(at);
                }
            } else {
                nearest.1 = (at, longitude);
                if settles {
                    sure.1 = sure.1.min(at);
                }
            }
            in_sign
        };
        // The moment at which the line through the two nearest moments
        // evaluated reaches the boundary.
        let secant = |nearest: ((f64, f64), (f64, f64))| {
            let (behind, ahead) = nearest;
            let (before, after) = (past(behind.1), past(ahead.1));
            behind.0 + (ahead.0 - behind.0) * (-before / (after - before))
        };
        if guided {
            for _ in 0..2 {
                let at = secant(nearest);
                if at > nearest.0.0 && at < nearest.1.0 {
                    probe(at, &mut nearest, &mut sure);
                }
            }
            let crossing = secant(nearest);
            let reach = 1.5 * SURE_DEGREES / speed;
            for at in [crossing - reach, crossing + reach] {
                if at > nearest.0.0 && at < nearest.1.0 {
                    probe(at, &mut nearest, &mut sure);
                }
            }
        }
        let (mut before, mut after) = (start, end);
        while after - before > PRECISION_DAYS {
            let middle = 0.5 * (before + after);
            let in_sign = if guided && middle <= sure.0 {
                true
            } else if guided && middle >= sure.1 {
                false
            } else {
                probe(middle, &mut nearest, &mut sure)
            };
            if in_sign {
                before = middle;
            } else {
                after = middle;
            }
        }
        after
    }
}

impl Iterator for Ingresses {
    type Item = Ingress;

    fn next(&mut self) -> Option<Ingress> {
        while self.cursor.0 < self.until.0 {
            // Every longitude is the series evaluated once: the one at the
            // end of a step is the one the next step begins from.
            let longitude = match self.known.take() {
                Some(longitude) => longitude,
                None => sidereal_longitude(self.cursor, self.ayanamsa),
            };
            let into = longitude - self.sign.start_longitude_degrees();
            let nearest = into.min(DEGREES_PER_SIGN - into).max(0.0);
            let cells = floor(0.9 * nearest / FASTEST_DAY_DEGREES / GRID_DAYS).max(1.0);
            let end = Moment(self.cursor.0 + cells * GRID_DAYS);
            let longitude_there = sidereal_longitude(end, self.ayanamsa);
            let sign_there = sign_of(longitude_there);
            let from = self.sign;
            if sign_there == from {
                self.cursor = end;
                self.known = Some(longitude_there);
                continue;
            }
            let moment = self.cross(
                (self.cursor.0, longitude),
                (end.0, longitude_there),
                sign_there,
            );
            // On from the far end of the cell, in the sign entered.
            self.cursor = end;
            self.known = Some(longitude_there);
            self.sign = sign_there;
            if moment >= self.until.0 {
                self.cursor = self.until;
                return None;
            }
            if moment >= self.from {
                return Some(Ingress {
                    moment: Moment(moment),
                    from,
                    to: sign_there,
                });
            }
        }
        None
    }
}

/// Which entry of Jupiter into a sign the Pushkaram of its rivers follows,
/// where Jupiter enters, turns back out and enters again.
///
/// Jupiter does that about two years in three, and the sources disagree on
/// which entry is reckoned (policy §5, one identifier each). The festivals
/// whose dates were read follow the final entry; Wikipedia's table of
/// future festivals opens two of them at a first entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryRule {
    /// The entry after which Jupiter stays until it moves on to the next
    /// sign: the second, where there are two. The Brahmaputra festival of
    /// 5 to 16 November 2019 and the Tungabhadra festival of 20 November to
    /// 1 December 2020 began at such entries, and each year's first entry
    /// into the sign had been in March.
    FinalEntry,
    /// The first entry into the sign, from the sign before it, of the run of
    /// entries and returns. Wikipedia's table of the Pushkaram dates opens
    /// the Tapti and Brahmaputra festival of 2019 and the Sindhu festival
    /// of 2021 at first entries (29 March 2019 and 6 April 2021), which no
    /// festival read followed.
    FirstEntry,
}

hc_core::catalogue! {
    type: EntryRule,
    tests: entry_rule_catalogue_tests,
    variants;

    /// Both rules, the one the festivals read follow first.
    pub const ALL;
    /// A short identifier, in kebab case.
    pub fn id;
    /// The rule with this identifier, by [`hc_core::catalogue::matches`].
    pub fn by_id;

    entries: {
        FinalEntry => "pushkaram-final-entry",
        FirstEntry => "pushkaram-first-entry",
    }
}

/// How far either side of a span the search for an entry looks. Consecutive
/// ingresses are at most 396.8 days apart (the most of 12 958 from −1000 to
/// 3000, under two ayanāṃśas), so a search that runs 430 days either side of
/// the span sees the ingress before an entry and the one after it. The
/// search is canonical ([`ingresses`]), so the lead is no more than the
/// distance to look back, not a way to reach the same bits.
const CONTEXT_DAYS: f64 = 430.0;

/// The entry of Jupiter into `sign`, from the sign before it, that begins at
/// or after `from` and before `until`, by `rule`: `None` when there is none.
///
/// The rule looks past either end of the span where it must: whether an
/// entry is the first of a run depends on what came before it, and whether it
/// is the final one on where Jupiter goes next.
///
/// This is [`entries_in`] for one sign, and gives what it gives for that
/// sign; asking it for several signs of the same span searches the sky
/// again each time, which [`entries_in`] does once.
#[must_use]
pub fn entry_into(
    sign: SiderealSign,
    from: Moment,
    until: Moment,
    rule: EntryRule,
    ayanamsa: Ayanamsa,
) -> Option<Ingress> {
    entries_in(from, until, rule, ayanamsa).find(|entry| entry.to == sign)
}

/// The entries of Jupiter into the signs, by `rule`, that begin at or after
/// `from` and before `until`: each sign's [`entry_into`], all the signs at
/// once, in the order of time.
///
/// Every sign's search walks the same ingresses, from the same moment, so
/// they are found once and each is tested against the rule in turn. The
/// lines of the festivals found are identical to those of the signs asked
/// for one by one, to the bit; what is saved is the walk, which is the whole
/// cost (`docs/systems/jupiter-festivals.md` §Accuracy has the timings).
/// The search is lazy: an iterator that is dropped has searched no further
/// than the entry it last gave and the ingress that follows it.
#[must_use]
pub fn entries_in(from: Moment, until: Moment, rule: EntryRule, ayanamsa: Ayanamsa) -> Entries {
    Entries {
        ingresses: ingresses(
            Moment(from.0 - CONTEXT_DAYS),
            Moment(until.0 + CONTEXT_DAYS),
            ayanamsa,
        )
        .peekable(),
        from,
        until,
        rule,
        earlier: None,
        given: [false; SIGNS_PER_ZODIAC],
        done: false,
    }
}

/// The entries of Jupiter into the signs in a span, from [`entries_in`].
#[derive(Debug, Clone)]
pub struct Entries {
    ingresses: core::iter::Peekable<Ingresses>,
    from: Moment,
    until: Moment,
    rule: EntryRule,
    /// The ingress before the one being weighed.
    earlier: Option<Ingress>,
    /// The signs that have had their entry: a sign has one in a span.
    given: [bool; SIGNS_PER_ZODIAC],
    /// Whether the span has been searched to its end.
    done: bool,
}

impl Iterator for Entries {
    type Item = Ingress;

    fn next(&mut self) -> Option<Ingress> {
        while !self.done {
            let Some(ingress) = self.ingresses.next() else {
                break;
            };
            if ingress.moment.0 >= self.until.0 {
                break;
            }
            let sign = ingress.to;
            let earlier = self.earlier.replace(ingress);
            if ingress.moment.0 < self.from.0 || !ingress.is_forward() {
                continue;
            }
            let kept = match self.rule {
                // The first of a run: not a return, which is an entry that
                // follows an exit back out of the sign.
                EntryRule::FirstEntry => {
                    !matches!(earlier, Some(e) if e.from == sign && !e.is_forward())
                }
                EntryRule::FinalEntry => {
                    matches!(self.ingresses.peek(), Some(next) if next.from == sign && next.is_forward())
                }
            };
            let given = &mut self.given[usize::from(sign.index())];
            if kept && !*given {
                *given = true;
                return Some(ingress);
            }
        }
        self.done = true;
        None
    }
}

/// The arc of visibility Indian astronomy gives Jupiter: it is lost in the
/// Sun's light, and so sets, when its longitude comes within 11° of the
/// Sun's, and rises again when it has left that arc. Varāhamihira's
/// *Pañcasiddhāntikā* has 11° for Jupiter (the Moon 12°, Mars 17°, Mercury
/// 13°, Venus 9°, Saturn 15°), and Bhāskara I's *Laghubhāskarīya* 7.1–2 the
/// same for Jupiter, Mercury, Saturn and Mars, 11°, 13°, 15° and 17°
/// (`subbarayappa1985`, read in the translation `wisdomlib` prints); Drik
/// Panchang quotes the *Sūrya Siddhānta* as "angular separation of 11° on
/// both sides of the Sun" for Jupiter (`drik-guru-asta`). Bhāskara II's is
/// 14° of the *śīghra* anomaly, another measure.
pub const VISIBILITY_ARC_DEGREES: f64 = 11.0;

/// One disappearance of Jupiter in the Sun's light and its reappearance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeliacalRising {
    /// When Jupiter's longitude, east of the Sun's in the evening, comes
    /// within the arc: its last evening setting, the start of its *asta*.
    pub setting: Moment,
    /// When, west of the Sun after the conjunction, it has left the arc: its
    /// heliacal rising, the end of the *asta*, and the beginning of a year of
    /// Jupiter by its risings.
    pub rising: Moment,
}

/// Jupiter's longitude east of the Sun's, apparent in the true equinox of
/// the date, in degrees from −180 to 180: it falls through 0 at each
/// conjunction, since the Sun overtakes Jupiter.
fn elongation(moment: Moment) -> f64 {
    signed_degrees(jupiter::longitude(moment) - hc_astro::solar::solar_longitude(moment))
}

/// The grid the search for a rising or a setting is laid on, in days: as
/// [`GRID_DAYS`], so that what is found does not depend on where the search
/// began, with the elongation's pace of 0.7° to 1.3° a day and a longer cell.
const ELONGATION_GRID_DAYS: f64 = 0.25;

/// The first moment at or after `from` when the elongation falls through
/// `target`, searching on at most 800 days; the elongation only falls, by
/// 0.7° to 1.3° a day, so a step that cannot reach the target is safe.
///
/// The search is canonical as [`ingresses`] is: it walks the grid of
/// [`ELONGATION_GRID_DAYS`] from the point at or before `from`, in steps of
/// whole cells, and bisects the one cell the crossing is in, so the moment is
/// the same whatever `from` it is asked from.
fn elongation_falls_through(target: f64, from: Moment) -> Option<Moment> {
    let mut at = floor(from.0 / ELONGATION_GRID_DAYS) * ELONGATION_GRID_DAYS;
    // The elongation at `at`, which each step works out once for the step
    // that follows it.
    let mut here = elongation(Moment(at));
    while at < from.0 + 800.0 {
        let to_go = modulo(here - target, 360.0);
        let cells = floor(0.8 * to_go / 1.3 / ELONGATION_GRID_DAYS).max(1.0);
        let next = at + cells * ELONGATION_GRID_DAYS;
        let there = elongation(Moment(next));
        let before = signed_degrees(here - target);
        let after = signed_degrees(there - target);
        if before > 0.0 && after <= 0.0 && before - after < 90.0 {
            let (mut low, mut high) = (at, next);
            while high - low > PRECISION_DAYS {
                let middle = 0.5 * (low + high);
                if signed_degrees(elongation(Moment(middle)) - target) > 0.0 {
                    low = middle;
                } else {
                    high = middle;
                }
            }
            // A crossing before `from`, in the cell `from` is in, is not
            // the one asked for.
            if high >= from.0 {
                return Some(Moment(high));
            }
        }
        at = next;
        here = there;
    }
    None
}

/// The first heliacal rising of Jupiter at or after `from`, with the
/// setting that began its *asta*, for an arc of visibility in degrees,
/// [`VISIBILITY_ARC_DEGREES`], the arc Indian astronomy gives. The setting
/// may be before `from`, when `from` is in the *asta*.
///
/// A conjunction of Jupiter and the Sun comes every 398.9 days, and each
/// has one rising after it.
#[must_use]
pub fn next_heliacal_rising(from: Moment, arc_degrees: f64) -> Option<HeliacalRising> {
    let rising = elongation_falls_through(-arc_degrees, from)?;
    // The setting is the last crossing of +arc before the rising: search
    // from 90 days earlier, when Jupiter is still well east of the Sun.
    let setting = elongation_falls_through(arc_degrees, Moment(rising.0 - 90.0))?;
    Some(HeliacalRising { setting, rising })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A moment at 0 h UT on a Julian date, as Horizons' tables are.
    fn at(julian_date: f64) -> Moment {
        Moment::from_julian_date(julian_date)
    }

    /// JPL Horizons' apparent ecliptic longitude of Jupiter (DE441, true
    /// equinox of date) at 0 h UT on 2024-12-07, the day of its opposition,
    /// is 76.3751533°; Lahiri is 24.2° at the date, so Jupiter was at about
    /// 52.2° sidereal, 22° into Vṛṣabha, and moving back at 0.2° a day.
    #[test]
    fn jupiter_at_its_opposition_of_2024_is_in_vrishabha_in_retrograde() {
        let position = position(at(2_460_651.5), Ayanamsa::LAHIRI);
        assert_eq!(position.sign, SiderealSign::VRISHABHA);
        assert!((position.apparent.longitude_degrees - 76.375_153_3).abs() < 0.0002);
        let ayanamsa = Ayanamsa::LAHIRI.degrees_at(at(2_460_651.5));
        assert!((ayanamsa - 24.2).abs() < 0.1, "{ayanamsa}");
        assert!((position.sidereal_longitude_degrees - 52.2).abs() < 0.1);
        assert!((position.degrees_into_sign - 22.2).abs() < 0.1);
        assert!((-0.25..-0.1).contains(&position.daily_motion_degrees));
    }

    /// Between two ingresses the sign is the one entered, and Jupiter enters
    /// every sign of the zodiac in order over a twelve-year span, with the
    /// returns between.
    #[test]
    fn the_ingresses_are_in_order_and_chain() {
        let from = at(2_451_545.0);
        let ingresses: Vec<Ingress> =
            ingresses(from, at(2_451_545.0 + 12.0 * 365.25), Ayanamsa::LAHIRI).collect();
        assert!((12..=18).contains(&ingresses.len()), "{}", ingresses.len());
        assert_eq!(ingresses[0].from, sign_at(from, Ayanamsa::LAHIRI));
        for pair in ingresses.windows(2) {
            assert!(pair[0].moment.0 < pair[1].moment.0);
            assert_eq!(pair[0].to, pair[1].from);
        }
        for ingress in &ingresses {
            let before = Moment(ingress.moment.0 - 0.01);
            let after = Moment(ingress.moment.0 + 0.01);
            assert_eq!(sign_at(before, Ayanamsa::LAHIRI), ingress.from);
            assert_eq!(sign_at(after, Ayanamsa::LAHIRI), ingress.to);
        }
        assert_eq!(
            ingresses
                .iter()
                .filter(|ingress| ingress.is_forward())
                .count(),
            12 + ingresses
                .iter()
                .filter(|ingress| !ingress.is_forward())
                .count()
        );
    }

    /// The canonical search written the plain way: every step and every
    /// midpoint evaluates the series, nothing is reused or inferred.
    fn plain_ingresses(from: Moment, until: Moment, ayanamsa: Ayanamsa) -> Vec<Ingress> {
        let mut found = Vec::new();
        let mut cursor = floor(from.0 / GRID_DAYS) * GRID_DAYS;
        let mut sign = sign_at(Moment(cursor), ayanamsa);
        while cursor < until.0 {
            let longitude = sidereal_longitude(Moment(cursor), ayanamsa);
            let into = longitude - sign.start_longitude_degrees();
            let nearest = into.min(DEGREES_PER_SIGN - into).max(0.0);
            let cells = floor(0.9 * nearest / FASTEST_DAY_DEGREES / GRID_DAYS).max(1.0);
            let end = cursor + cells * GRID_DAYS;
            let sign_there = sign_at(Moment(end), ayanamsa);
            if sign_there == sign {
                cursor = end;
                continue;
            }
            let (mut before, mut after) = (cursor, end);
            while after - before > PRECISION_DAYS {
                let middle = 0.5 * (before + after);
                if sign_at(Moment(middle), ayanamsa) == sign {
                    before = middle;
                } else {
                    after = middle;
                }
            }
            if from.0 <= after && after < until.0 {
                found.push(Ingress {
                    moment: Moment(after),
                    from: sign,
                    to: sign_there,
                });
            }
            cursor = end;
            sign = sign_there;
        }
        found
    }

    /// The search reuses the longitudes it has worked out and infers the
    /// side of a midpoint from the probes, and what it finds is what the
    /// plain search finds, to the last bit, in the span of the era's ends
    /// and in the middle of it, for two ayanāṃśas, a few ingresses short of
    /// every kind: forward, retrograde, at a station's edge.
    #[test]
    fn the_ingresses_are_the_plain_search_to_the_last_bit() {
        for (start, years, ayanamsa) in [
            (-365_000.0, 5.0, Ayanamsa::LAHIRI),
            (-140_700.0, 5.0, Ayanamsa::LAHIRI),
            (738_000.0, 6.0, Ayanamsa::RAMAN),
            (1_100_000.0, 5.0, Ayanamsa::LAHIRI),
        ] {
            let (from, until) = (Moment(start), Moment(start + years * 365.25));
            let fast: Vec<Ingress> = ingresses(from, until, ayanamsa).collect();
            let plain = plain_ingresses(from, until, ayanamsa);
            assert!(fast.len() >= 4, "{start}: {}", fast.len());
            assert_eq!(fast, plain, "from {start} by {ayanamsa:?}");
        }
    }

    /// An ingress is one instant, to the last bit, whatever span it is asked
    /// for: the span's start, which the walk began from and which used to
    /// decide the bits, and its end, which used to clip the last step, change
    /// nothing. The ingresses of a long span, and of spans that begin at a
    /// grid point, a hair after one, a day, days and months into it, and that
    /// end at all sorts of moments, are the same ingresses, each exactly.
    #[test]
    fn an_ingress_is_the_same_instant_from_every_start() {
        let ayanamsa = Ayanamsa::LAHIRI;
        let (from, until) = (738_000.0, 741_000.0);
        let whole: Vec<Ingress> = ingresses(Moment(from), Moment(until), ayanamsa).collect();
        assert!(whole.len() >= 8, "{}", whole.len());
        for (start, end) in [
            (0.0, 0.0),
            (0.125, 0.0),
            (0.3, -1.7),
            (1.0, 0.01),
            (3.7, -40.0),
            (17.25, -3.0),
            (101.5, -200.5),
            (250.1, -9.0),
            (333.3, 0.0),
            (1000.0, -1000.0),
        ] {
            let (a, b) = (from + start, until + end);
            let part: Vec<Ingress> = ingresses(Moment(a), Moment(b), ayanamsa).collect();
            let expected: Vec<Ingress> = whole
                .iter()
                .copied()
                .filter(|ingress| a <= ingress.moment.0 && ingress.moment.0 < b)
                .collect();
            assert_eq!(part, expected, "from {a} until {b}");
        }
    }

    /// The jitter of the longitude's last digits decides which side of a
    /// boundary a moment within some 3 × 10⁻⁹° of it is on. A bisection
    /// midpoint inferred from a probe that close to the boundary disagreed
    /// with the plain bisection at the ingress found from an arbitrary start
    /// in 385 BCE (a midpoint 8 × 10⁻¹⁰° from the boundary), and the probes now
    /// keep 10⁻⁷° from it. Two ingresses are tests of it: that one, Meṣa to
    /// Vṛṣabha by Lahiri, which the canonical search finds at the instant
    /// below whatever it is asked from; and the one of the era that comes
    /// closest, a midpoint 4.5 × 10⁻¹² degree from the boundary, Jupiter
    /// leaving Vṛścika for Dhanus in 200 BCE.
    #[test]
    fn an_ingress_at_the_jitter_of_the_last_digits_is_the_plain_search() {
        for (from, moment) in [
            (-140_522.388_875_832_75, -140_522.336_361_885_07),
            (-72_800.0, -72_796.392_545_700_07),
        ] {
            let (from, until) = (Moment(from), Moment(from + 30.0));
            let fast: Vec<Ingress> = ingresses(from, until, Ayanamsa::LAHIRI).collect();
            assert_eq!(fast.len(), 1);
            assert_eq!(fast[0].moment.0, moment);
            assert_eq!(fast, plain_ingresses(from, until, Ayanamsa::LAHIRI));
        }
    }

    /// The entries of all the signs at once are each sign's entry, asked
    /// one at a time, for both rules, in the years where Jupiter enters two
    /// signs, where it enters a sign twice and where it enters none.
    #[test]
    fn the_entries_of_a_span_are_each_signs_entry() {
        let ayanamsa = Ayanamsa::LAHIRI;
        let mut seen = 0;
        for year in [2019, 2020] {
            let from = Moment(f64::from(year - 2000) * 365.2425 + 730_120.0 - 1.0);
            let until = Moment(from.0 + 365.2425);
            for rule in EntryRule::ALL {
                let all: Vec<Ingress> = entries_in(from, until, *rule, ayanamsa).collect();
                for pair in all.windows(2) {
                    assert!(pair[0].moment.0 < pair[1].moment.0);
                }
                for entry in &all {
                    assert!(from.0 <= entry.moment.0 && entry.moment.0 < until.0);
                    assert!(entry.is_forward());
                }
                let asked = all
                    .iter()
                    .map(|entry| entry.to)
                    .chain([SiderealSign::SIMHA]);
                for sign in asked {
                    let one = entry_into(sign, from, until, *rule, ayanamsa);
                    let of_all = all.iter().find(|entry| entry.to == sign).copied();
                    assert_eq!(one, of_all, "{year} {sign:?} {rule:?}");
                }
                seen += all.len();
            }
        }
        // 2019 has two entries into Dhanus and 2020 one into Makara.
        assert!(seen >= 3, "{seen}");
    }

    #[test]
    fn an_empty_span_has_no_ingress() {
        let moment = at(2_460_000.5);
        assert_eq!(ingresses(moment, moment, Ayanamsa::LAHIRI).count(), 0);
        assert_eq!(
            ingresses(Moment(moment.0 + 5.0), moment, Ayanamsa::LAHIRI).count(),
            0
        );
    }

    /// An entry that no run has is none: Jupiter is not in Siṃha in 2020.
    #[test]
    fn a_year_without_an_entry_has_none() {
        let (from, to) = (at(2_458_849.5), at(2_459_215.5));
        for rule in EntryRule::ALL {
            assert!(entry_into(SiderealSign::SIMHA, from, to, *rule, Ayanamsa::LAHIRI).is_none());
        }
    }

    /// Asked in the middle of an *asta*, the next rising is the one that ends
    /// it, with the setting that began it before the moment asked from.
    #[test]
    fn a_rising_asked_for_in_the_asta_is_the_one_that_ends_it() {
        // 2026-08-01, in the asta that ran from about 14 July to 13 August.
        let from = at(2_461_253.5);
        let rising = next_heliacal_rising(from, VISIBILITY_ARC_DEGREES).expect("a rising");
        assert!(rising.setting.0 < from.0 && from.0 < rising.rising.0);
        assert!(
            (10.0..15.0).contains(&(rising.rising.0 - from.0)),
            "{rising:?}"
        );
        assert!((26.0..34.0).contains(&(rising.rising.0 - rising.setting.0)));
        // At the ends the elongation is the arc, east then west.
        let east = elongation(rising.setting);
        let west = elongation(rising.rising);
        assert!(
            (east - 11.0).abs() < 1e-3 && (west + 11.0).abs() < 1e-3,
            "{east} {west}"
        );
        // A narrower arc shortens the asta.
        let narrow = next_heliacal_rising(from, 8.0).expect("a rising");
        assert!(narrow.rising.0 - narrow.setting.0 < rising.rising.0 - rising.setting.0 - 5.0);
    }

    /// The rising of 2026 and the setting that began its *asta* are one
    /// instant each, to the last bit, from every start between the previous
    /// rising's and this one's.
    #[test]
    fn a_rising_is_the_same_instant_from_every_start() {
        let reference =
            next_heliacal_rising(at(2_461_150.0), VISIBILITY_ARC_DEGREES).expect("a rising");
        assert!(reference.rising.0 > at(2_461_253.5).0);
        for start in [
            2_461_150.0,
            2_461_150.1,
            2_461_203.7,
            2_461_240.0,
            2_461_240.01,
            2_461_253.5,
            2_461_262.9,
            2_461_262.99,
        ] {
            let found = next_heliacal_rising(at(start), VISIBILITY_ARC_DEGREES).expect("a rising");
            assert_eq!(found.rising, reference.rising, "from {start}");
            assert_eq!(found.setting, reference.setting, "from {start}");
        }
    }

    #[test]
    fn the_rules_have_the_identifiers_the_exports_name() {
        assert_eq!(EntryRule::FinalEntry.id(), "pushkaram-final-entry");
        assert_eq!(
            EntryRule::by_id(" Pushkaram-First-Entry "),
            Some(EntryRule::FirstEntry)
        );
    }
}
