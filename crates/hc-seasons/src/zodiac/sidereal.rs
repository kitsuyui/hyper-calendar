//! The same twelve divisions, measured from the fixed stars.
//!
//! A sidereal sign is a 30° arc of *sidereal* longitude: the same apparent
//! solar longitude the tropical signs use, minus the **ayanamsa** — the angle
//! by which precession has carried the March equinox away from the sidereal
//! zero point. The ayanamsa is currently about 24°, growing by about 50″ a year,
//! so a sidereal sign begins about twenty-four days after the tropical sign
//! of the same name.
//!
//! This is the division Indian (Vedic) astronomy and astrology use, where a
//! sign is a **rāśi** and the instant the Sun enters one is a **saṅkrānti**.
//! It is also the division the Indian solar calendars take their months
//! from — see [`super::rashi`].
//!
//! # Why this is not an optional extra
//!
//! Twenty-four degrees of a thirty-degree sign is four fifths. For four days
//! in five the same moment is in a different sign under the two systems: most
//! of tropical Aries is sidereal Pisces, most of tropical Taurus is sidereal
//! Aries, and so on round the year. A library that shipped only the tropical
//! signs would be answering "which sign is the Sun in" with one of two
//! answers and not saying which. A test in this module measures the
//! disagreement rate over a year and finds it near four fifths, as the
//! arithmetic requires.
//!
//! # Which ayanamsa
//!
//! There is no single one, because the sidereal zero point is a convention
//! and several are in use. [`Ayanamsa::LAHIRI`] — Chitrapaksha, fixing the
//! star Chitrā (Spica) at sidereal 180° — is the Indian government standard,
//! adopted on the recommendation of the Calendar Reform Committee of 1955 and
//! used by the *Indian Astronomical Ephemeris*. Three others are shipped for
//! comparison, and [`Ayanamsa::new`] takes any anchor at all, because a
//! library that hard-coded one would be taking a side in a live argument.
//!
//! # Accuracy, twice over
//!
//! The solar longitude underneath is `hc-astro`'s VSOP87 series, good to
//! about 1″, so only a saṅkrānti within about a minute of local midnight can
//! be given the wrong day. On top of that, published values for a named
//! ayanamsa disagree among themselves by a few tens of arcseconds — different
//! precession models, different rounding of the anchor — and 20″ of solar
//! longitude is about eight minutes of time. That second uncertainty is now
//! much the larger, and it is a disagreement between authorities rather than
//! an error of the model; both are stated rather than hidden. For a
//! saṅkrānti *day* the practical consequence is the same: a boundary near
//! midnight may move.

use hc_astro::julian_centuries;
use hc_astro::solar::{solar_longitude, solar_longitude_after};
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::{floor, normalize_degrees};

use crate::meridian::Meridian;
use crate::zodiac::tropical::{RulingPlanet, TropicalSign};
use crate::zodiac::{DEGREES_PER_SIGN, SIGNS_PER_ZODIAC, SignPeriod, degrees_into_arc};

/// The Julian date of J2000.0, the epoch the precession series is reckoned
/// from.
const J2000_JULIAN_DATE: f64 = 2_451_545.0;

/// Days in a Julian century.
const DAYS_PER_JULIAN_CENTURY: f64 = 36_525.0;

/// Coefficients of the general precession in longitude, p_A, in arcseconds,
/// as a polynomial in Julian centuries of TT from J2000.
///
/// Capitaine, Wallace and Chapront, "Expressions for IAU 2000 precession
/// quantities", *Astronomy & Astrophysics* 412 (2003), equation (39); adopted
/// as the IAU 2006 precession. The leading term, 5028.796195″ per century, is
/// the familiar "about 50 arcseconds a year" that moves the equinox.
const GENERAL_PRECESSION_ARCSECONDS: [f64; 6] = [
    0.0,
    5_028.796_195,
    1.105_434_8,
    0.000_079_64,
    -0.000_023_857,
    -0.000_000_038_3,
];

/// The general precession in longitude accumulated since J2000, in
/// arcseconds, evaluated by Horner's method.
fn general_precession_arcseconds(centuries: f64) -> f64 {
    let mut total = 0.0;
    let mut index = GENERAL_PRECESSION_ARCSECONDS.len();
    while index > 0 {
        index -= 1;
        total = total * centuries + GENERAL_PRECESSION_ARCSECONDS[index];
    }
    total
}

/// The angle between the tropical and the sidereal zero point.
///
/// An ayanamsa is fixed by one number at one epoch and then carried forward
/// and back by precession; the disagreements between the named ones are
/// disagreements about that one number, not about the physics. So this type
/// is an anchor and nothing else, and [`Ayanamsa::new`] lets a caller supply
/// an anchor this crate has never heard of.
///
/// The anchor values shipped here are the ones the Swiss Ephemeris uses,
/// which is the most widely deployed reference implementation; other
/// published tables for the same named ayanamsa differ by a few tens of
/// arcseconds. See the module documentation for what that costs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ayanamsa {
    name: &'static str,
    anchor_julian_date: f64,
    degrees_at_anchor: f64,
}

impl Ayanamsa {
    /// Lahiri, also called Chitrapaksha: the Indian government standard.
    ///
    /// Defined so that the star Chitrā — Spica, the brightest star of the
    /// constellation Virgo — sits at sidereal
    /// longitude 180°, which puts the sidereal zero point near the star ζ
    /// Piscium. Adopted on the recommendation of the Calendar Reform
    /// Committee chaired by Meghnad Saha, whose report of 1955 fixed the
    /// value at 23°15′00″ for 21 March 1956, and used by the *Indian
    /// Astronomical Ephemeris* and by the national civil calendar's
    /// astronomical appendix.
    ///
    /// Anchored here at 22.460148° for Julian date 2415020.0 (1900 January
    /// 0.5 TT), exactly one Julian century before J2000.
    pub const LAHIRI: Self = Self::new("Lahiri (Chitrapaksha)", 2_415_020.0, 22.460_148);

    /// Raman: B. V. Raman's ayanamsa, about 1.45° smaller than Lahiri.
    ///
    /// Anchored at 21.010833° for Julian date 2415020.0.
    pub const RAMAN: Self = Self::new("Raman", 2_415_020.0, 21.010_833);

    /// Krishnamurti: the ayanamsa of the Krishnamurti Paddhati school, about
    /// 0.48° smaller than Lahiri.
    ///
    /// Anchored at 21.978333° for Julian date 2415020.0.
    pub const KRISHNAMURTI: Self = Self::new("Krishnamurti", 2_415_020.0, 21.978_333);

    /// Fagan–Bradley: the Western sidereal school's ayanamsa, about 0.88°
    /// larger than Lahiri.
    ///
    /// Anchored at 24.042044° for Julian date 2433282.5 (1950 January 1.0),
    /// the epoch B1950 the scheme was defined at.
    pub const FAGAN_BRADLEY: Self = Self::new("Fagan-Bradley", 2_433_282.5, 24.042_044);

    /// The four named ayanamsas this crate ships, largest last.
    pub const ALL: [Self; 4] = [
        Self::RAMAN,
        Self::KRISHNAMURTI,
        Self::LAHIRI,
        Self::FAGAN_BRADLEY,
    ];

    /// An ayanamsa from its anchor: a value in degrees at a Julian date.
    ///
    /// Everything away from the anchor is IAU 2006 general precession in
    /// longitude, which is what every scheme in use agrees about; only the
    /// anchor differs. So a caller wanting Yukteswar, De Luce, Djwhal Khul or
    /// a house convention can have it without this crate taking a position.
    #[must_use]
    pub const fn new(name: &'static str, anchor_julian_date: f64, degrees_at_anchor: f64) -> Self {
        Self {
            name,
            anchor_julian_date,
            degrees_at_anchor,
        }
    }

    /// The name of the scheme, e.g. `"Lahiri (Chitrapaksha)"`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The Julian date the anchor value is quoted for.
    #[must_use]
    pub const fn anchor_julian_date(self) -> f64 {
        self.anchor_julian_date
    }

    /// The anchor value in degrees.
    #[must_use]
    pub const fn degrees_at_anchor(self) -> f64 {
        self.degrees_at_anchor
    }

    /// The ayanamsa, in degrees, at a given moment.
    ///
    /// About 24.2° in the 2020s, growing by about 0.014° — 50″ — a year, and
    /// passing through zero around 285 CE for the Lahiri anchor, which is the
    /// last time the two zodiacs coincided.
    ///
    /// ```
    /// use hc_seasons::zodiac::Ayanamsa;
    /// use hc_seasons::Moment;
    ///
    /// // Rata Die 739_252 is 1 January 2025.
    /// let degrees = Ayanamsa::LAHIRI.degrees_at(Moment(739_252.0));
    /// assert!((24.0..24.5).contains(&degrees), "{degrees}");
    /// ```
    #[must_use]
    pub fn degrees_at(self, moment: Moment) -> f64 {
        let anchor_centuries =
            (self.anchor_julian_date - J2000_JULIAN_DATE) / DAYS_PER_JULIAN_CENTURY;
        let accumulated = general_precession_arcseconds(julian_centuries(moment))
            - general_precession_arcseconds(anchor_centuries);
        self.degrees_at_anchor + accumulated / 3_600.0
    }
}

/// One of the twelve sidereal signs — a **rāśi**.
///
/// Ordering is by sidereal longitude from Meṣa, which matches
/// [`TropicalSign`]'s ordering index for index: Meṣa is Aries' counterpart,
/// Mīna is Pisces'. The *names* correspond; the *dates* do not, and that is
/// the entire point of the type existing separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SiderealSign(u8);

/// The ways the 12 positions are named, one entry per language or
/// convention. See [`hc_calendar::shape::Naming`].
pub mod namings {
    use hc_calendar::shape::Naming;

    hc_core::catalogue! {
        type: Naming<12>,
        id: |naming| naming.id,
        provenance: |naming| naming.authority,
        tests: rashi_naming_tests,

        /// Every naming this crate ships.
        pub const ALL;
        /// The naming with this identifier.
        pub fn by_id;

        entries: {
            /// The `sanskrit` column.
            pub const SANSKRIT = Naming {
                id: "sa-latn",
                english_name: "Sanskrit, IAST",
                names: &[
                "Meṣa",
                "Vṛṣabha",
                "Mithuna",
                "Karka",
                "Siṃha",
                "Kanyā",
                "Tulā",
                "Vṛścika",
                "Dhanus",
                "Makara",
                "Kumbha",
                "Mīna",
                ],
                authority: "IAST transliteration with diacritics",
            };
            /// The `devanagari` column.
            pub const DEVANAGARI = Naming {
                id: "sa",
                english_name: "Sanskrit, Devanagari",
                names: &[
                "मेष",
                "वृषभ",
                "मिथुन",
                "कर्क",
                "सिंह",
                "कन्या",
                "तुला",
                "वृश्चिक",
                "धनु",
                "मकर",
                "कुम्भ",
                "मीन",
                ],
                authority: "The names in the script they are written in",
            };
            /// The `english` column.
            pub const ENGLISH = Naming {
                id: "en",
                english_name: "English emblems",
                names: &[
                "the Ram",
                "the Bull",
                "the Twins",
                "the Crab",
                "the Lion",
                "the Maiden",
                "the Scales",
                "the Scorpion",
                "the Bow",
                "the Sea-creature",
                "the Pot",
                "the Fishes",
                ],
                authority: "The emblem each name means, in English",
            };
        }
    }
}

impl SiderealSign {
    /// Meṣa, the Ram — Aries' counterpart, sidereal 0°–30°.
    pub const MESHA: Self = Self(0);
    /// Vṛṣabha, the Bull — Taurus' counterpart.
    pub const VRISHABHA: Self = Self(1);
    /// Mithuna, the Twins — Gemini's counterpart.
    pub const MITHUNA: Self = Self(2);
    /// Karka, the Crab — Cancer's counterpart.
    pub const KARKA: Self = Self(3);
    /// Siṃha, the Lion — Leo's counterpart.
    pub const SIMHA: Self = Self(4);
    /// Kanyā, the Maiden — Virgo's counterpart.
    pub const KANYA: Self = Self(5);
    /// Tulā, the Scales — Libra's counterpart.
    pub const TULA: Self = Self(6);
    /// Vṛścika, the Scorpion — Scorpio's counterpart.
    pub const VRISHCHIKA: Self = Self(7);
    /// Dhanus, the Bow — Sagittarius' counterpart.
    pub const DHANUS: Self = Self(8);
    /// Makara, the Sea-creature — Capricorn's counterpart. Its saṅkrānti is
    /// the festival of Makara Saṅkrānti, around 14 January.
    pub const MAKARA: Self = Self(9);
    /// Kumbha, the Pot — Aquarius' counterpart.
    pub const KUMBHA: Self = Self(10);
    /// Mīna, the Fishes — Pisces' counterpart.
    pub const MINA: Self = Self(11);

    /// All twelve, in order from Meṣa.
    pub const ALL: [Self; SIGNS_PER_ZODIAC] = [
        Self::MESHA,
        Self::VRISHABHA,
        Self::MITHUNA,
        Self::KARKA,
        Self::SIMHA,
        Self::KANYA,
        Self::TULA,
        Self::VRISHCHIKA,
        Self::DHANUS,
        Self::MAKARA,
        Self::KUMBHA,
        Self::MINA,
    ];

    /// The sign at an index reduced modulo twelve.
    pub(crate) const fn at(index: u8) -> Self {
        Self(index % 12)
    }

    /// The sign at an index counted from Meṣa.
    ///
    /// Returns `None` for an index of 12 or more.
    #[must_use]
    pub const fn from_index(index: u8) -> Option<Self> {
        if index as usize >= SIGNS_PER_ZODIAC {
            return None;
        }
        Some(Self(index))
    }

    /// This sign's index from Meṣa, 0 to 11.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// The sidereal longitude, in degrees, at which the sign opens.
    ///
    /// Sidereal, so this is *not* a tropical longitude and cannot be handed
    /// to [`hc_astro::solar::solar_longitude_after`] directly; add an
    /// ayanamsa first, or use [`ingress_after`], which does.
    #[must_use]
    pub const fn start_longitude_degrees(self) -> f64 {
        self.0 as f64 * DEGREES_PER_SIGN
    }

    /// The name in IAST transliteration, e.g. `"Vṛṣabha"`.
    #[must_use]
    pub const fn sanskrit_name(self) -> &'static str {
        namings::SANSKRIT.names[self.0 as usize]
    }

    /// The name in Devanagari, e.g. `"वृषभ"`.
    #[must_use]
    pub const fn devanagari_name(self) -> &'static str {
        namings::DEVANAGARI.names[self.0 as usize]
    }

    /// The emblem in English, e.g. `"the Bull"`.
    #[must_use]
    pub const fn emblem(self) -> &'static str {
        namings::ENGLISH.names[self.0 as usize]
    }

    /// The Western name of the counterpart sign, e.g. `"Taurus"`.
    ///
    /// A convenience for a reader who knows one scheme and not the other. It
    /// says nothing about dates: the sidereal Vṛṣabha and the tropical Taurus
    /// are twenty-four days apart.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        self.tropical_counterpart().english_name()
    }

    /// The astrological symbol, shared with the tropical sign of the same
    /// index: U+2648 ♈ through U+2653 ♓.
    #[must_use]
    pub const fn symbol(self) -> char {
        self.tropical_counterpart().symbol()
    }

    /// The tropical sign of the same index and name.
    #[must_use]
    pub const fn tropical_counterpart(self) -> TropicalSign {
        TropicalSign::at(self.0)
    }

    /// The element the sign is assigned to, shared with the tropical scheme.
    #[must_use]
    pub const fn element(self) -> super::Element {
        self.tropical_counterpart().element()
    }

    /// The modality the sign is assigned to, shared with the tropical scheme.
    ///
    /// Indian usage calls these *chara* (movable), *sthira* (fixed) and
    /// *dvisvabhāva* (dual), which are the cardinal, fixed and mutable signs
    /// under other names.
    #[must_use]
    pub const fn modality(self) -> super::Modality {
        self.tropical_counterpart().modality()
    }

    /// The ruling planet — in Sanskrit the *svāmī* or lord of the rāśi.
    ///
    /// This is the **classical** scheme, identical to
    /// [`TropicalSign::ruling_planet`] and not to
    /// [`TropicalSign::modern_ruling_planet`]. Jyotiṣa uses the seven visible
    /// bodies (plus the two lunar nodes, which rule no sign) and made no
    /// reassignment when Uranus, Neptune and Pluto were discovered. That the
    /// two traditions still agree about rulership while disagreeing about
    /// dates by twenty-four days is the clearest sign they are one scheme
    /// that diverged.
    #[must_use]
    pub const fn ruling_planet(self) -> RulingPlanet {
        self.tropical_counterpart().ruling_planet()
    }

    /// The next sign, 30° further along the ecliptic, wrapping at 360°.
    #[must_use]
    pub const fn next(self) -> Self {
        Self((self.0 + 1) % 12)
    }

    /// The previous sign, wrapping at 0°.
    #[must_use]
    pub const fn previous(self) -> Self {
        Self((self.0 + 11) % 12)
    }

    /// The sign directly across the zodiac, 180° away.
    #[must_use]
    pub const fn opposite(self) -> Self {
        Self((self.0 + 6) % 12)
    }
}

/// The Sun's apparent sidereal longitude at a moment, in degrees.
///
/// The apparent tropical longitude less the ayanamsa, reduced to 0°–360°.
#[must_use]
pub fn sidereal_longitude(moment: Moment, ayanamsa: Ayanamsa) -> f64 {
    normalize_degrees(solar_longitude(moment) - ayanamsa.degrees_at(moment))
}

/// The sidereal sign the Sun is in at an instant.
#[must_use]
pub fn sign_at_moment(moment: Moment, ayanamsa: Ayanamsa) -> SiderealSign {
    SiderealSign::at(floor(sidereal_longitude(moment, ayanamsa) / DEGREES_PER_SIGN) as u8)
}

/// How far into its sidereal sign the Sun is at an instant, in degrees from 0
/// up to but not including 30.
#[must_use]
pub fn degrees_into_sign(moment: Moment, ayanamsa: Ayanamsa) -> f64 {
    let longitude = sidereal_longitude(moment, ayanamsa);
    degrees_into_arc(
        longitude,
        sign_at_moment(moment, ayanamsa).start_longitude_degrees(),
    )
}

/// The sign and the degrees into it at one instant.
#[must_use]
pub fn sign_and_degrees(moment: Moment, ayanamsa: Ayanamsa) -> (SiderealSign, f64) {
    let longitude = sidereal_longitude(moment, ayanamsa);
    let sign = SiderealSign::at(floor(longitude / DEGREES_PER_SIGN) as u8);
    (
        sign,
        degrees_into_arc(longitude, sign.start_longitude_degrees()),
    )
}

/// How many refinement passes the ingress search makes.
///
/// The target tropical longitude of a sidereal ingress depends on the
/// ayanamsa *at the ingress*, which is not known until the ingress is found.
/// The ayanamsa moves about 50″ a year, so a first guess taken up to a year
/// early is at most that far out — twenty minutes of the Sun's motion — and
/// one pass removes it. The second is below floating-point noise and is kept
/// only so that the answer does not depend on where the search began.
const INGRESS_REFINEMENTS: usize = 2;

/// The first moment at or after `moment` when the Sun enters a sidereal sign.
///
/// This is a **saṅkrānti**, the instant an Indian solar month begins.
#[must_use]
pub fn ingress_after(sign: SiderealSign, ayanamsa: Ayanamsa, moment: Moment) -> Moment {
    let target =
        |at: Moment| normalize_degrees(sign.start_longitude_degrees() + ayanamsa.degrees_at(at));
    let mut found = solar_longitude_after(target(moment), moment);
    let mut pass = 0;
    while pass < INGRESS_REFINEMENTS {
        // Search from five days before the previous answer: the Sun is then
        // about five degrees short of the target, comfortably inside the
        // previous sign, so the first crossing found is the same one.
        found = solar_longitude_after(target(found), Moment(found.0 - 5.0));
        pass += 1;
    }
    found
}

/// The Universal Time instant of a sign's saṅkrānti in a Gregorian year.
///
/// # The once-a-year guarantee, and where it lapses
///
/// On 1 January the Sun stands at about 280° tropical, which with a modern
/// ayanamsa is about 256° sidereal — well inside Dhanus, far from a boundary
/// — so each of the twelve saṅkrānti falls exactly once in each Gregorian
/// year. That argument depends on the ayanamsa: it was about 10° around the
/// year 1000, which put a boundary on 1 January itself, and a Gregorian year
/// near then can contain a saṅkrānti twice or not at all. For the Lahiri
/// anchor this function is safe from roughly 1100 CE onwards.
/// [`signs_in_year`] walks the year instead and cannot double-count.
#[must_use]
pub fn ingress_moment(year: i64, sign: SiderealSign, ayanamsa: Ayanamsa) -> Moment {
    ingress_after(
        sign,
        ayanamsa,
        Moment(crate::gregorian::new_year(year).0 as f64),
    )
}

/// The day a sign's saṅkrānti falls on in a Gregorian year, at a meridian.
///
/// ```
/// use hc_seasons::{Meridian, zodiac::{Ayanamsa, SiderealSign}};
/// use hc_seasons::zodiac::sidereal::ingress_day;
///
/// // Makara Sankranti 2025 fell on 14 January in India, at 09:03 IST.
/// let day = ingress_day(2025, SiderealSign::MAKARA, Ayanamsa::LAHIRI, Meridian::INDIA);
/// assert_eq!(day, hc_calendar::Rd(739_265));
/// ```
#[must_use]
pub fn ingress_day(year: i64, sign: SiderealSign, ayanamsa: Ayanamsa, meridian: Meridian) -> Rd {
    meridian.day_of(ingress_moment(year, sign, ayanamsa))
}

/// A period built from a known saṅkrānti instant.
fn period_from_start(
    sign: SiderealSign,
    start: Moment,
    ayanamsa: Ayanamsa,
    meridian: Meridian,
) -> SignPeriod<SiderealSign> {
    let end = ingress_after(sign.next(), ayanamsa, Moment(start.0 + 1.0));
    SignPeriod {
        sign,
        start,
        end,
        start_day: meridian.day_of(start),
        end_day: Rd(meridian.day_of(end).0 - 1),
    }
}

/// The period a sidereal sign occupies in a Gregorian year.
#[must_use]
pub fn sign_period(
    year: i64,
    sign: SiderealSign,
    ayanamsa: Ayanamsa,
    meridian: Meridian,
) -> SignPeriod<SiderealSign> {
    period_from_start(
        sign,
        ingress_moment(year, sign, ayanamsa),
        ayanamsa,
        meridian,
    )
}

/// The period of the sidereal sign in effect on a day.
#[must_use]
pub fn sign_in_effect(day: Rd, ayanamsa: Ayanamsa, meridian: Meridian) -> SignPeriod<SiderealSign> {
    let end_of_day = meridian.midnight(Rd(day.0 + 1));
    let sign = sign_at_moment(end_of_day, ayanamsa);
    // A sign is at most about 31.5 days long and the Sun covers at least
    // 33.4° in 35 days, so a search begun 35 days back is inside the previous
    // sign and brackets exactly one crossing.
    let start = ingress_after(sign, ayanamsa, Moment(end_of_day.0 - 35.0));
    period_from_start(sign, start, ayanamsa, meridian)
}

/// The sidereal sign in effect on a day.
#[must_use]
pub fn sign_on_day(day: Rd, ayanamsa: Ayanamsa, meridian: Meridian) -> SiderealSign {
    sign_in_effect(day, ayanamsa, meridian).sign
}

/// The sign whose period begins on a day, if one does.
#[must_use]
pub fn sign_beginning_on(
    day: Rd,
    ayanamsa: Ayanamsa,
    meridian: Meridian,
) -> Option<SignPeriod<SiderealSign>> {
    let period = sign_in_effect(day, ayanamsa, meridian);
    if period.start_day == day {
        Some(period)
    } else {
        None
    }
}

/// The twelve sidereal sign periods of a Gregorian year, in date order.
///
/// The first is the sign whose saṅkrānti is the first one on or after
/// 1 January; with a modern ayanamsa that is Makara, around 14 January.
/// Walking the year rather than rotating a fixed table means this cannot
/// double-count however far the ayanamsa has drifted.
#[must_use]
pub fn signs_in_year(year: i64, ayanamsa: Ayanamsa, meridian: Meridian) -> SiderealSignsInYear {
    let probe = Moment(crate::gregorian::new_year(year).0 as f64);
    let sign = sign_at_moment(probe, ayanamsa).next();
    SiderealSignsInYear {
        ayanamsa,
        meridian,
        sign,
        start: ingress_after(sign, ayanamsa, probe),
        remaining: SIGNS_PER_ZODIAC,
    }
}

/// The iterator returned by [`signs_in_year`].
#[derive(Debug, Clone, Copy)]
pub struct SiderealSignsInYear {
    ayanamsa: Ayanamsa,
    meridian: Meridian,
    sign: SiderealSign,
    start: Moment,
    remaining: usize,
}

impl Iterator for SiderealSignsInYear {
    type Item = SignPeriod<SiderealSign>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let period = period_from_start(self.sign, self.start, self.ayanamsa, self.meridian);
        self.remaining -= 1;
        self.sign = self.sign.next();
        self.start = period.end;
        Some(period)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for SiderealSignsInYear {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::{from_year_month_day, year_month_day_from_rd};
    use crate::zodiac::tropical;

    const INDIA: Meridian = Meridian::INDIA;
    const LAHIRI: Ayanamsa = Ayanamsa::LAHIRI;

    /// The Lahiri ayanamsa is published as about 23°51′ at J2000 and about
    /// 24°12′ in the mid-2020s. Rata Die 730_120 is 1 January 2000.
    #[test]
    fn the_lahiri_ayanamsa_is_about_twenty_four_degrees_today() {
        let at_2000 = LAHIRI.degrees_at(Moment(730_120.0));
        assert!(
            (23.84..=23.88).contains(&at_2000),
            "Lahiri at J2000 was {at_2000}"
        );
        let at_2025 = LAHIRI.degrees_at(Moment(739_252.0));
        assert!(
            (24.15..=24.25).contains(&at_2025),
            "Lahiri in 2025 was {at_2025}"
        );
    }

    /// Fifty arcseconds a year is the whole of precession, and the ayanamsa
    /// is nothing but precession measured from a chosen zero.
    #[test]
    fn the_ayanamsa_grows_by_about_fifty_arcseconds_a_year() {
        for ayanamsa in Ayanamsa::ALL {
            let earlier = ayanamsa.degrees_at(Moment(730_120.0));
            let later = ayanamsa.degrees_at(Moment(730_120.0 + 365.25 * 100.0));
            let per_year = (later - earlier) / 100.0 * 3_600.0;
            assert!(
                (50.1..=50.5).contains(&per_year),
                "{} moved {per_year} arcseconds a year",
                ayanamsa.name()
            );
        }
    }

    /// Every named ayanamsa is the same precession with a different anchor,
    /// so the differences between them are constant to within the tiny
    /// second-order terms.
    #[test]
    fn the_named_ayanamsas_differ_by_a_constant_offset() {
        for ayanamsa in Ayanamsa::ALL {
            let early =
                ayanamsa.degrees_at(Moment(693_596.0)) - LAHIRI.degrees_at(Moment(693_596.0));
            let late =
                ayanamsa.degrees_at(Moment(766_644.0)) - LAHIRI.degrees_at(Moment(766_644.0));
            assert!(
                (early - late).abs() < 1e-6,
                "{} drifted against Lahiri by {}",
                ayanamsa.name(),
                early - late
            );
        }
        let at = Moment(739_252.0);
        assert!(LAHIRI.degrees_at(at) > Ayanamsa::RAMAN.degrees_at(at));
        assert!(LAHIRI.degrees_at(at) > Ayanamsa::KRISHNAMURTI.degrees_at(at));
        assert!(LAHIRI.degrees_at(at) < Ayanamsa::FAGAN_BRADLEY.degrees_at(at));
        // Raman is about 1.45 degrees behind Lahiri.
        let gap = LAHIRI.degrees_at(at) - Ayanamsa::RAMAN.degrees_at(at);
        assert!((1.4..=1.5).contains(&gap), "the Raman gap was {gap}");
    }

    /// The two zodiacs last coincided in the third century CE, which is what
    /// "the ayanamsa passes through zero around then" means. Rata Die 103_996
    /// is roughly the year 285.
    #[test]
    fn the_two_zodiacs_coincided_in_the_third_century() {
        let ayanamsa = LAHIRI.degrees_at(Moment(from_year_month_day(285, 3, 21).0 as f64));
        assert!(
            ayanamsa.abs() < 0.05,
            "Lahiri at 285 CE was {ayanamsa}, not near zero"
        );
        // A thousand years earlier the sidereal zero point was ahead of the
        // equinox, so the ayanamsa is negative.
        assert!(LAHIRI.degrees_at(Moment(from_year_month_day(-715, 1, 1).0 as f64)) < -13.0);
    }

    /// Not a published scheme: an anchor invented here, to show that the
    /// constructor takes one and that the rest of the module does not care
    /// where it came from.
    #[test]
    fn a_caller_can_supply_an_ayanamsa_this_crate_has_never_heard_of() {
        let invented = Ayanamsa::new("invented", 2_451_545.0, 25.0);
        assert_eq!(invented.name(), "invented");
        assert!((invented.anchor_julian_date() - 2_451_545.0).abs() < 1e-9);
        assert!((invented.degrees_at_anchor() - 25.0).abs() < 1e-9);
        // Rata Die 730_120 is 1 January 2000, near enough the anchor epoch
        // that the invented value should read back almost unchanged.
        let at_2000 = invented.degrees_at(Moment(730_120.0));
        assert!((at_2000 - 25.0).abs() < 0.01, "read back {at_2000}");
        // And the whole module works with it.
        let period = sign_period(2024, SiderealSign::MESHA, invented, INDIA);
        assert_eq!(period.sign, SiderealSign::MESHA);
        assert!((29..=32).contains(&period.length_days()));
        // A larger ayanamsa puts the sidereal zero point further behind the
        // equinox, so every boundary falls later.
        assert!(period.start.0 > ingress_moment(2024, SiderealSign::MESHA, LAHIRI).0);
    }

    /// The defining relation. Tropical longitude minus ayanamsa is sidereal
    /// longitude, at every instant, by construction.
    #[test]
    fn the_sidereal_longitude_is_the_tropical_one_less_the_ayanamsa() {
        for offset in (0..365).step_by(11) {
            let moment = Moment(739_251.0 + f64::from(offset));
            let tropical = solar_longitude(moment);
            let sidereal = sidereal_longitude(moment, LAHIRI);
            let difference = normalize_degrees(tropical - sidereal);
            assert!(
                (difference - LAHIRI.degrees_at(moment)).abs() < 1e-9,
                "the gap was {difference}"
            );
        }
    }

    /// The headline disagreement: twenty-four degrees out of thirty is four
    /// fifths, so for about 292 days of a year the two systems name different
    /// signs. The task this module exists for.
    #[test]
    fn the_two_zodiacs_name_different_signs_for_four_days_in_five() {
        let start = from_year_month_day(2024, 1, 1);
        let mut disagreements = 0;
        for offset in 0..366 {
            let day = Rd(start.0 + offset);
            let tropical = tropical::sign_on_day(day, INDIA);
            let sidereal = sign_on_day(day, LAHIRI, INDIA);
            if sidereal.tropical_counterpart() != tropical {
                disagreements += 1;
                // Where they disagree the sidereal sign is always the
                // tropical one's immediate predecessor, because the ayanamsa
                // is between 0 and 30 degrees.
                assert_eq!(
                    sidereal.tropical_counterpart(),
                    tropical.previous(),
                    "at {day} the gap was more than one sign"
                );
            }
        }
        // 24/30 of 366 is 293.
        assert!(
            (285..=300).contains(&disagreements),
            "{disagreements} days of disagreement out of 366"
        );
    }

    /// The example the module documentation gives: for most of the northern
    /// spring the Sun is tropically in Aries and siderally in Pisces.
    #[test]
    fn late_march_is_aries_tropically_and_pisces_siderally() {
        let day = from_year_month_day(2024, 3, 25);
        assert_eq!(tropical::sign_on_day(day, INDIA), TropicalSign::ARIES);
        assert_eq!(sign_on_day(day, LAHIRI, INDIA), SiderealSign::MINA);
        assert_eq!(
            SiderealSign::MINA.tropical_counterpart(),
            TropicalSign::PISCES
        );
    }

    /// A sidereal ingress lags the tropical one of the same index by the
    /// ayanamsa, which at about 24.2° and about 0.985°/day is about 24 or 25
    /// days.
    #[test]
    fn a_sidereal_ingress_lags_the_tropical_one_by_the_ayanamsa() {
        for sign in SiderealSign::ALL {
            let sidereal = ingress_moment(2024, sign, LAHIRI);
            // The tropical crossing of the same numbered boundary just
            // before it, rather than the one in the same Gregorian year,
            // because for some signs those are a year apart.
            let tropical_before = solar_longitude_after(
                sign.tropical_counterpart().start_longitude_degrees(),
                Moment(sidereal.0 - 40.0),
            );
            let lag = sidereal.0 - tropical_before.0;
            assert!(
                (23.0..=27.0).contains(&lag),
                "{} lagged by {lag} days",
                sign.sanskrit_name()
            );
        }
    }

    /// At every saṅkrānti the Sun's sidereal longitude is exactly the sign's
    /// opening longitude. This is what the refinement passes are for: without
    /// them the ayanamsa used to build the target would be the one at the
    /// start of the search, not at the crossing.
    #[test]
    fn the_sun_stands_exactly_on_the_boundary_at_a_sankranti() {
        for year in [1200i64, 1700, 1900, 2024, 2200] {
            for sign in SiderealSign::ALL {
                let moment = ingress_moment(year, sign, LAHIRI);
                let longitude = sidereal_longitude(moment, LAHIRI);
                let error = degrees_into_arc(longitude, sign.start_longitude_degrees());
                let error = if error > 180.0 { error - 360.0 } else { error };
                assert!(
                    error.abs() < 1e-6,
                    "{} of {year} was {error} degrees off",
                    sign.sanskrit_name()
                );
            }
        }
    }

    #[test]
    fn the_twelve_sidereal_periods_partition_the_year() {
        for year in 1990..2040 {
            let mut previous: Option<SignPeriod<SiderealSign>> = None;
            let mut count = 0;
            for period in signs_in_year(year, LAHIRI, INDIA) {
                if let Some(earlier) = previous {
                    assert!(
                        (earlier.end.0 - period.start.0).abs() < 1e-9,
                        "{} of {year} did not hand over exactly",
                        earlier.sign.sanskrit_name()
                    );
                    assert_eq!(earlier.end_day.0 + 1, period.start_day.0);
                    assert_eq!(earlier.sign.next(), period.sign);
                }
                assert!(
                    (29..=32).contains(&period.length_days()),
                    "{} of {year} ran {} days",
                    period.sign.sanskrit_name(),
                    period.length_days()
                );
                previous = Some(period);
                count += 1;
            }
            assert_eq!(count, SIGNS_PER_ZODIAC);
        }
    }

    #[test]
    fn a_sidereal_years_iteration_opens_with_makara() {
        let mut iterator = signs_in_year(2024, LAHIRI, INDIA);
        assert_eq!(iterator.len(), 12);
        let first = iterator.next().unwrap();
        assert_eq!(first.sign, SiderealSign::MAKARA);
        let (_, month, day) = year_month_day_from_rd(first.start_day);
        assert_eq!(month, 1);
        assert!(
            (14..=15).contains(&day),
            "Makara Sankranti was on the {day}"
        );
        let last = iterator.last().unwrap();
        assert_eq!(last.sign, SiderealSign::DHANUS);
        assert_eq!(year_month_day_from_rd(last.start_day).1, 12);
    }

    /// Makara Saṅkrānti is the one major Indian festival fixed to a solar,
    /// not a lunar, event, which is why it stays near 14 January while Diwali
    /// and Holi wander through the Gregorian calendar. The published
    /// astronomical instants for 2020–2025 fall on 14 or 15 January IST; the
    /// *festival* is sometimes kept on the following day when the saṅkrānti
    /// is late in the evening, which is a ritual rule and not an
    /// astronomical one, so it is not modelled here.
    #[test]
    fn makara_sankranti_falls_on_the_fourteenth_of_january() {
        for year in 2020..=2025 {
            let day = ingress_day(year, SiderealSign::MAKARA, LAHIRI, INDIA);
            let (got_year, month, got_day) = year_month_day_from_rd(day);
            assert_eq!(got_year, year);
            assert_eq!(month, 1);
            assert!(
                (14..=15).contains(&got_day),
                "Makara Sankranti {year} landed on January {got_day}"
            );
        }
    }

    /// The festival has been creeping later: precession moves a sidereal
    /// boundary about a day per seventy years against the Gregorian calendar,
    /// which is why the festival was in late December in the middle ages and
    /// will be in February in a few centuries.
    #[test]
    fn a_sidereal_boundary_creeps_later_against_the_gregorian_calendar() {
        let early = ingress_day(1500, SiderealSign::MAKARA, LAHIRI, INDIA);
        let late = ingress_day(2500, SiderealSign::MAKARA, LAHIRI, INDIA);
        let early_fraction = early.0 - crate::gregorian::new_year(1500).0;
        let late_fraction = late.0 - crate::gregorian::new_year(2500).0;
        let creep = late_fraction - early_fraction;
        // A thousand years at about 50 arcseconds a year is 13.9 degrees,
        // which the Sun covers in about fourteen days.
        assert!(
            (12..=16).contains(&creep),
            "the boundary crept {creep} days in a thousand years"
        );
    }

    #[test]
    fn the_sign_in_effect_agrees_with_the_years_own_table() {
        for period in signs_in_year(2024, LAHIRI, INDIA) {
            assert_eq!(sign_on_day(period.start_day, LAHIRI, INDIA), period.sign);
            assert_eq!(sign_on_day(period.end_day, LAHIRI, INDIA), period.sign);
            assert_eq!(
                sign_on_day(Rd(period.start_day.0 - 1), LAHIRI, INDIA),
                period.sign.previous()
            );
            assert!(period.contains(period.start_day));
            assert!(!period.contains(Rd(period.end_day.0 + 1)));
            assert!(
                sign_beginning_on(period.start_day, LAHIRI, INDIA).is_some(),
                "{} did not report its own start",
                period.sign.sanskrit_name()
            );
        }
    }

    #[test]
    fn exactly_twelve_days_of_a_year_begin_a_sidereal_sign() {
        let start = from_year_month_day(2024, 1, 1);
        let end = from_year_month_day(2025, 1, 1);
        let mut beginnings = 0;
        for offset in 0..(end.0 - start.0) {
            if sign_beginning_on(Rd(start.0 + offset), LAHIRI, INDIA).is_some() {
                beginnings += 1;
            }
        }
        assert_eq!(beginnings, 12);
    }

    #[test]
    fn degrees_into_a_sidereal_sign_run_from_zero_to_thirty() {
        for period in signs_in_year(2024, LAHIRI, INDIA) {
            let early = degrees_into_sign(Moment(period.start.0 + 0.001), LAHIRI);
            let late = degrees_into_sign(Moment(period.end.0 - 0.001), LAHIRI);
            assert!(early < 0.01, "{early} degrees in just after the sankranti");
            assert!(late > 29.99, "{late} degrees in just before the next");
            let (sign, degrees) = sign_and_degrees(Moment(period.start.0 + 10.0), LAHIRI);
            assert_eq!(sign, period.sign);
            assert!((9.0..=11.0).contains(&degrees));
        }
    }

    #[test]
    fn the_twelve_rashi_are_distinct_and_fully_named() {
        for (index, sign) in SiderealSign::ALL.into_iter().enumerate() {
            assert_eq!(sign.index() as usize, index);
            assert_eq!(SiderealSign::from_index(index as u8), Some(sign));
            assert!(!sign.sanskrit_name().is_empty());
            assert!(!sign.devanagari_name().is_empty());
            assert!(sign.emblem().starts_with("the "));
            assert!(!sign.english_name().is_empty());
            assert_eq!(sign.tropical_counterpart().index(), sign.index());
            assert_eq!(sign.symbol(), sign.tropical_counterpart().symbol());
            for other in &SiderealSign::ALL[index + 1..] {
                assert_ne!(sign.sanskrit_name(), other.sanskrit_name());
                assert_ne!(sign.devanagari_name(), other.devanagari_name());
            }
        }
        assert_eq!(SiderealSign::from_index(12), None);
        assert_eq!(SiderealSign::MESHA.sanskrit_name(), "Meṣa");
        assert_eq!(SiderealSign::MAKARA.devanagari_name(), "मकर");
    }

    /// Nine of the twelve emblems are word for word the Western ones, which
    /// is the transmission showing through. The three that differ differ in
    /// a revealing way: each is the Western emblem's *instrument or vessel*
    /// rather than its person. Dhanus is the bow, not the archer; Kumbha the
    /// pot, not the water-bearer; and Makara a sea-creature of Indian
    /// description rather than the Greek goat-fish.
    #[test]
    fn the_rashi_emblems_are_the_western_ones_but_for_three() {
        let differing: usize = SiderealSign::ALL
            .iter()
            .filter(|sign| sign.emblem() != sign.tropical_counterpart().emblem())
            .count();
        assert_eq!(differing, 3);
        assert_eq!(SiderealSign::DHANUS.emblem(), "the Bow");
        assert_eq!(SiderealSign::KUMBHA.emblem(), "the Pot");
        assert_eq!(SiderealSign::MAKARA.emblem(), "the Sea-creature");
        // And the nine that agree really are identical strings.
        assert_eq!(SiderealSign::MESHA.emblem(), TropicalSign::ARIES.emblem());
        assert_eq!(SiderealSign::TULA.emblem(), TropicalSign::LIBRA.emblem());
    }

    /// Jyotiṣa never reassigned a sign to Uranus, Neptune or Pluto, so the
    /// rāśi lords are the classical seven and agree with the tropical
    /// classical scheme sign for sign.
    #[test]
    fn the_rashi_lords_are_the_classical_seven() {
        for sign in SiderealSign::ALL {
            assert!(sign.ruling_planet().is_classical());
            assert_eq!(
                sign.ruling_planet(),
                sign.tropical_counterpart().ruling_planet()
            );
            assert_eq!(sign.element(), sign.tropical_counterpart().element());
            assert_eq!(sign.modality(), sign.tropical_counterpart().modality());
        }
        assert_eq!(SiderealSign::SIMHA.ruling_planet(), RulingPlanet::SUN);
        assert_eq!(SiderealSign::KARKA.ruling_planet(), RulingPlanet::MOON);
    }

    #[test]
    fn stepping_round_the_sidereal_zodiac_returns_to_the_same_sign() {
        for sign in SiderealSign::ALL {
            assert_eq!(sign.next().previous(), sign);
            assert_eq!(sign.previous().next(), sign);
            assert_eq!(sign.opposite().opposite(), sign);
            assert_ne!(sign.opposite(), sign);
            assert!(
                (sign.start_longitude_degrees() - f64::from(sign.index()) * 30.0).abs() < 1e-12
            );
        }
        assert_eq!(SiderealSign::MINA.next(), SiderealSign::MESHA);
        assert_eq!(SiderealSign::MESHA.opposite(), SiderealSign::TULA);
    }

    /// A different ayanamsa moves every boundary by the difference between
    /// the two, which for Raman against Lahiri is about 1.45° — about a day
    /// and a half of the Sun's motion.
    #[test]
    fn a_different_ayanamsa_moves_every_boundary() {
        for sign in SiderealSign::ALL {
            let lahiri = ingress_moment(2024, sign, LAHIRI);
            let raman = ingress_moment(2024, sign, Ayanamsa::RAMAN);
            let gap = lahiri.0 - raman.0;
            assert!(
                (1.3..=1.6).contains(&gap),
                "{} moved {gap} days between the two ayanamsas",
                sign.sanskrit_name()
            );
        }
    }
}
