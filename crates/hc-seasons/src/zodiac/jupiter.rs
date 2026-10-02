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

use super::DEGREES_PER_SIGN;
use super::sidereal::{Ayanamsa, SiderealSign};

/// The most Jupiter moves in a day, in degrees: its fastest geocentric speed
/// is about 0.25°, near opposition at perihelion. Used to say how far a step
/// cannot reach a boundary.
const FASTEST_DAY_DEGREES: f64 = 0.3;

/// The shortest step the search takes, in days. Where Jupiter is within
/// 0.03° of a boundary the step cannot be sized by the distance, and a station
/// there makes the search slow, not wrong.
const SHORTEST_STEP_DAYS: f64 = 0.1;

/// How closely an ingress is found, in days: about a tenth of a second.
const PRECISION_DAYS: f64 = 1e-6;

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
    cursor: Moment,
    until: Moment,
    sign: SiderealSign,
    ayanamsa: Ayanamsa,
}

/// The ingresses of Jupiter at or after `from` and before `until`, in
/// order, in the zodiac of `ayanamsa`.
///
/// Each search walks forward in steps the distance to the nearest boundary
/// and Jupiter's greatest speed say cannot cross it, then bisects the step
/// that did.
#[must_use]
pub fn ingresses(from: Moment, until: Moment, ayanamsa: Ayanamsa) -> Ingresses {
    Ingresses {
        cursor: from,
        until,
        sign: sign_at(from, ayanamsa),
        ayanamsa,
    }
}

impl Iterator for Ingresses {
    type Item = Ingress;

    fn next(&mut self) -> Option<Ingress> {
        while self.cursor.0 < self.until.0 {
            let longitude = sidereal_longitude(self.cursor, self.ayanamsa);
            let into = longitude - self.sign.start_longitude_degrees();
            let nearest = into.min(DEGREES_PER_SIGN - into).max(0.0);
            let step = (0.9 * nearest / FASTEST_DAY_DEGREES).max(SHORTEST_STEP_DAYS);
            let end = Moment((self.cursor.0 + step).min(self.until.0));
            let sign_there = sign_at(end, self.ayanamsa);
            if sign_there == self.sign {
                self.cursor = end;
                continue;
            }
            let (mut before, mut after) = (self.cursor.0, end.0);
            while after - before > PRECISION_DAYS {
                let middle = 0.5 * (before + after);
                if sign_at(Moment(middle), self.ayanamsa) == self.sign {
                    before = middle;
                } else {
                    after = middle;
                }
            }
            let ingress = Ingress {
                moment: Moment(after),
                from: self.sign,
                to: sign_there,
            };
            self.cursor = Moment(after);
            self.sign = sign_there;
            return Some(ingress);
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

/// The entry of Jupiter into `sign`, from the sign before it, that begins at
/// or after `from` and before `until`, by `rule`: `None` when there is none.
///
/// The rule looks past either end of the span where it must: whether an
/// entry is the first of a run depends on what came before it, and whether it
/// is the final one on where Jupiter goes next.
#[must_use]
pub fn entry_into(
    sign: SiderealSign,
    from: Moment,
    until: Moment,
    rule: EntryRule,
    ayanamsa: Ayanamsa,
) -> Option<Ingress> {
    // Jupiter stays in a sign for at most about fourteen months, so a
    // search that runs a little over two years either side of the span sees
    // what came before an entry and where the sign goes next.
    let horizon = Moment(until.0 + 800.0);
    let mut earlier = None;
    let mut entries = ingresses(Moment(from.0 - 800.0), horizon, ayanamsa).peekable();
    while let Some(ingress) = entries.next() {
        if ingress.moment.0 >= until.0 {
            break;
        }
        if ingress.moment.0 < from.0 || !(ingress.is_forward() && ingress.to == sign) {
            earlier = Some(ingress);
            continue;
        }
        let kept = match rule {
            // The first of a run: not a return, which is an entry that
            // follows an exit back out of the sign.
            EntryRule::FirstEntry => {
                !matches!(earlier, Some(e) if e.from == sign && !e.is_forward())
            }
            EntryRule::FinalEntry => {
                matches!(entries.peek(), Some(next) if next.from == sign && next.is_forward())
            }
        };
        if kept {
            return Some(ingress);
        }
        earlier = Some(ingress);
    }
    None
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

/// The first moment at or after `from` when the elongation falls through
/// `target`, searching on at most 800 days; the elongation only falls, by
/// 0.7° to 1.3° a day, so a step that cannot reach the target is safe.
fn elongation_falls_through(target: f64, from: Moment) -> Option<Moment> {
    let mut at = from.0;
    while at < from.0 + 800.0 {
        let to_go = modulo(elongation(Moment(at)) - target, 360.0);
        // Within a hair of the target, or past it by a hair: take it as at.
        let step = f64::max(0.8 * to_go / 1.3, 0.25);
        let next = at + step;
        let before = signed_degrees(elongation(Moment(at)) - target);
        let after = signed_degrees(elongation(Moment(next)) - target);
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
            return Some(Moment(high));
        }
        at = next;
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

    #[test]
    fn the_rules_have_the_identifiers_the_exports_name() {
        assert_eq!(EntryRule::FinalEntry.id(), "pushkaram-final-entry");
        assert_eq!(
            EntryRule::by_id(" Pushkaram-First-Entry "),
            Some(EntryRule::FirstEntry)
        );
    }
}
