//! The twelve Western signs, measured from the March equinox.
//!
//! A tropical sign is a 30° arc of apparent solar longitude counted from the
//! equinox: Aries 0°–30°, Taurus 30°–60°, and so on round to Pisces at
//! 330°–360°. That is the entire definition, and it is the one this crate
//! already computes — the sign boundaries are exactly the twelve 中気, the
//! principal solar terms of [`crate::solar_terms`], because the 中気 are the
//! multiples of 30° and so are the signs.
//!
//! So [`TropicalSign::opening_term`] is not a lookup table bolted on; it is
//! the same angle read twice, and
//! [`ingress_moment`] and [`crate::solar_terms::term_moment`] agree to the
//! floating-point noise of the search. A test asserts that.
//!
//! # What "tropical" means, and what it does not
//!
//! *Tropical* here means "measured from the tropic points", i.e. from the
//! equinox, not "near the equator". Because precession carries the equinox
//! westward about 50″ a year, a tropical sign is **not** the constellation of the same
//! name: the Sun is in tropical Aries in late March but in front of the stars
//! of Pisces. They last agreed around the second century CE. The division
//! that tracks the stars instead is [`super::sidereal`].
//!
//! Nothing here is an astrological claim. The signs are a coordinate system
//! with a long history of names attached to it; the element, modality and
//! ruling planet are shipped as the data they are, sourced below, and this
//! crate makes no statement about what any of it means.
//!
//! # The dates move, and the printed ones stopped moving
//!
//! A sign's dates drift with the equinox and with the leap-year cycle: the
//! March equinox falls on 20 March in most years now and on 19 March in some,
//! so Aries begins on different days in different years.
//! [`TropicalSign::conventional_period`] carries the fixed dates newspaper
//! columns print — "Aries: March 21 – April 19" — which were last a good fit
//! around the 1920s. `tests/zodiac_conventional_dates.rs` measures how far
//! apart the two have drifted rather than asserting that either is right.
//!
//! Accuracy: the underlying solar longitude carries a systematic bias of
//! about −4.5 minutes, so an ingress within roughly ten minutes of local
//! midnight can be assigned the wrong day. See the crate README.

use hc_astro::solar::{seasonal_event, solar_longitude, solar_longitude_after};
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::floor;

use crate::meridian::Meridian;
use crate::solar_terms::SolarTerm;
use crate::zodiac::{DEGREES_PER_SIGN, SIGNS_PER_ZODIAC, SignPeriod, degrees_into_arc};

/// One of the twelve tropical signs.
///
/// Ordering is by longitude from the March equinox, so `TropicalSign` values
/// compare the way their arcs do: Aries is least, Pisces greatest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TropicalSign(u8);

/// The four classical elements a sign is assigned to.
///
/// These are the Greek four of Empedocles, transmitted through Ptolemy's
/// *Tetrabiblos*. They are **not** 五行, the Chinese five phases, which are
/// five, are ordered by generation and conquest, and belong to a different
/// system entirely — see [`hc_calendar::cycle::FIVE_PHASES`]. Conflating the
/// two is the usual mistake.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    /// Fire: Aries, Leo, Sagittarius.
    Fire,
    /// Earth: Taurus, Virgo, Capricorn.
    Earth,
    /// Air: Gemini, Libra, Aquarius.
    Air,
    /// Water: Cancer, Scorpio, Pisces.
    Water,
}

impl Element {
    /// All four, in the order they run round the zodiac from Aries.
    pub const ALL: [Self; 4] = [Self::Fire, Self::Earth, Self::Air, Self::Water];

    /// The name in English.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Fire => "fire",
            Self::Earth => "earth",
            Self::Air => "air",
            Self::Water => "water",
        }
    }

    /// The Latin name: *ignis*, *terra*, *aër*, *aqua*.
    ///
    /// *Aër* is the Latin borrowing of Greek ἀήρ and is conventionally
    /// printed with the diaeresis, because the two vowels are separate
    /// syllables.
    #[must_use]
    pub const fn latin_name(self) -> &'static str {
        match self {
            Self::Fire => "ignis",
            Self::Earth => "terra",
            Self::Air => "aër",
            Self::Water => "aqua",
        }
    }
}

/// The three modalities, or *quadruplicities*, a sign is assigned to.
///
/// The name "quadruplicity" is the older one and counts the four signs that
/// share a modality; "modality" is the commoner modern term. The division is
/// by position within a season: the cardinal signs open a season, the fixed
/// signs sit in its middle, the mutable signs close it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Modality {
    /// Cardinal: Aries, Cancer, Libra, Capricorn — the four signs that open
    /// at an equinox or a solstice.
    Cardinal,
    /// Fixed: Taurus, Leo, Scorpio, Aquarius.
    Fixed,
    /// Mutable: Gemini, Virgo, Sagittarius, Pisces.
    Mutable,
}

impl Modality {
    /// All three, in the order they run round the zodiac from Aries.
    pub const ALL: [Self; 3] = [Self::Cardinal, Self::Fixed, Self::Mutable];

    /// The name in English.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Cardinal => "cardinal",
            Self::Fixed => "fixed",
            Self::Mutable => "mutable",
        }
    }
}

/// A body named as the ruler of a sign.
///
/// The seven classical rulers are the seven moving bodies visible to the
/// naked eye, which is why the Sun and the Moon are in the list: the scheme
/// is pre-Copernican and "planet" meant "wanderer". The three outer bodies
/// appear only in [`TropicalSign::modern_ruling_planet`].
///
/// These are names in a naming scheme. The actual bodies — their orbits,
/// rotation periods and clocks — are `hc-planetary`'s subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RulingPlanet {
    /// The Sun, ruler of Leo.
    Sun,
    /// The Moon, ruler of Cancer.
    Moon,
    /// Mercury, ruler of Gemini and Virgo.
    Mercury,
    /// Venus, ruler of Taurus and Libra.
    Venus,
    /// Mars, ruler of Aries and, classically, Scorpio.
    Mars,
    /// Jupiter, ruler of Sagittarius and, classically, Pisces.
    Jupiter,
    /// Saturn, ruler of Capricorn and, classically, Aquarius.
    Saturn,
    /// Uranus, modern ruler of Aquarius. Discovered 1781.
    Uranus,
    /// Neptune, modern ruler of Pisces. Discovered 1846.
    Neptune,
    /// Pluto, modern ruler of Scorpio. Discovered 1930; no longer a planet
    /// under the IAU's 2006 definition, and still used in this scheme.
    Pluto,
}

impl RulingPlanet {
    /// The seven bodies of the classical scheme, in the Chaldean order of
    /// decreasing apparent orbital period: Saturn, Jupiter, Mars, Sun, Venus,
    /// Mercury, Moon.
    ///
    /// This is the order the planetary hours and the names of the days of the
    /// week come from, which is why it is worth having as data.
    pub const CHALDEAN_ORDER: [Self; 7] = [
        Self::Saturn,
        Self::Jupiter,
        Self::Mars,
        Self::Sun,
        Self::Venus,
        Self::Mercury,
        Self::Moon,
    ];

    /// The name in English.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Sun => "Sun",
            Self::Moon => "Moon",
            Self::Mercury => "Mercury",
            Self::Venus => "Venus",
            Self::Mars => "Mars",
            Self::Jupiter => "Jupiter",
            Self::Saturn => "Saturn",
            Self::Uranus => "Uranus",
            Self::Neptune => "Neptune",
            Self::Pluto => "Pluto",
        }
    }

    /// Whether this is one of the seven bodies visible to the naked eye.
    #[must_use]
    pub const fn is_classical(self) -> bool {
        matches!(
            self,
            Self::Sun
                | Self::Moon
                | Self::Mercury
                | Self::Venus
                | Self::Mars
                | Self::Jupiter
                | Self::Saturn
        )
    }
}

/// The fixed calendar dates a sign is conventionally printed with.
///
/// These are dates in a Gregorian year with no year attached, because that is
/// exactly what they are: a newspaper column does not recompute them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConventionalPeriod {
    /// The month the sign conventionally starts in, 1 to 12.
    pub start_month: u8,
    /// The day of that month.
    pub start_day: u8,
    /// The month the sign conventionally ends in.
    pub end_month: u8,
    /// The day of that month.
    pub end_day: u8,
}

/// One row of the sign table.
struct SignNames {
    english: &'static str,
    latin: &'static str,
    emblem: &'static str,
    japanese: &'static str,
    symbol: char,
    conventional: ConventionalPeriod,
}

/// A shorthand for a row's conventional dates.
const fn dates(start_month: u8, start_day: u8, end_month: u8, end_day: u8) -> ConventionalPeriod {
    ConventionalPeriod {
        start_month,
        start_day,
        end_month,
        end_day,
    }
}

/// The sign table, indexed by longitude / 30 from the March equinox.
///
/// The English and Latin columns differ for exactly two signs: English has
/// anglicised *Scorpius* to "Scorpio" and *Capricornus* to "Capricorn", and
/// the Latin column keeps the forms that are also the IAU constellation
/// names. The Japanese column is the 黄道十二宮 names as Japanese almanacs
/// and dictionaries print them.
///
/// The symbols are U+2648 ARIES through U+2653 PISCES, consecutive in the
/// Miscellaneous Symbols block, which is why the table can be checked against
/// arithmetic rather than transcribed twice.
///
/// The conventional dates are the set English-language newspaper columns
/// settled on in the early twentieth century; they are data to be compared
/// against, not an authority.
const SIGN_NAMES: [SignNames; SIGNS_PER_ZODIAC] = [
    SignNames {
        english: "Aries",
        latin: "Aries",
        emblem: "the Ram",
        japanese: "白羊宮",
        symbol: '\u{2648}',
        conventional: dates(3, 21, 4, 19),
    },
    SignNames {
        english: "Taurus",
        latin: "Taurus",
        emblem: "the Bull",
        japanese: "金牛宮",
        symbol: '\u{2649}',
        conventional: dates(4, 20, 5, 20),
    },
    SignNames {
        english: "Gemini",
        latin: "Gemini",
        emblem: "the Twins",
        japanese: "双子宮",
        symbol: '\u{264A}',
        conventional: dates(5, 21, 6, 20),
    },
    SignNames {
        english: "Cancer",
        latin: "Cancer",
        emblem: "the Crab",
        japanese: "巨蟹宮",
        symbol: '\u{264B}',
        conventional: dates(6, 21, 7, 22),
    },
    SignNames {
        english: "Leo",
        latin: "Leo",
        emblem: "the Lion",
        japanese: "獅子宮",
        symbol: '\u{264C}',
        conventional: dates(7, 23, 8, 22),
    },
    SignNames {
        english: "Virgo",
        latin: "Virgo",
        emblem: "the Maiden",
        japanese: "処女宮",
        symbol: '\u{264D}',
        conventional: dates(8, 23, 9, 22),
    },
    SignNames {
        english: "Libra",
        latin: "Libra",
        emblem: "the Scales",
        japanese: "天秤宮",
        symbol: '\u{264E}',
        conventional: dates(9, 23, 10, 22),
    },
    SignNames {
        english: "Scorpio",
        latin: "Scorpius",
        emblem: "the Scorpion",
        japanese: "天蠍宮",
        symbol: '\u{264F}',
        conventional: dates(10, 23, 11, 21),
    },
    SignNames {
        english: "Sagittarius",
        latin: "Sagittarius",
        emblem: "the Archer",
        japanese: "人馬宮",
        symbol: '\u{2650}',
        conventional: dates(11, 22, 12, 21),
    },
    SignNames {
        english: "Capricorn",
        latin: "Capricornus",
        emblem: "the Sea-goat",
        japanese: "磨羯宮",
        symbol: '\u{2651}',
        conventional: dates(12, 22, 1, 19),
    },
    SignNames {
        english: "Aquarius",
        latin: "Aquarius",
        emblem: "the Water-bearer",
        japanese: "宝瓶宮",
        symbol: '\u{2652}',
        conventional: dates(1, 20, 2, 18),
    },
    SignNames {
        english: "Pisces",
        latin: "Pisces",
        emblem: "the Fishes",
        japanese: "双魚宮",
        symbol: '\u{2653}',
        conventional: dates(2, 19, 3, 20),
    },
];

/// The index of Aquarius, the first sign the Sun enters in a Gregorian year.
///
/// On 1 January the Sun stands at about 280° of apparent longitude, in
/// Capricorn, so the first ingress of the year is Aquarius at 300° around
/// 20 January and the last is Capricorn at 270° around 21 December. That
/// 280° drifts by well under a degree per millennium — the Gregorian year is
/// tuned to the tropical year — so the rotation is safe across the whole era
/// this crate supports.
const FIRST_SIGN_OF_GREGORIAN_YEAR: u8 = 10;

impl TropicalSign {
    /// ♈ Aries, 0°–30°, opening at 春分, the March equinox.
    pub const ARIES: Self = Self(0);
    /// ♉ Taurus, 30°–60°, opening at 穀雨.
    pub const TAURUS: Self = Self(1);
    /// ♊ Gemini, 60°–90°, opening at 小満.
    pub const GEMINI: Self = Self(2);
    /// ♋ Cancer, 90°–120°, opening at 夏至, the June solstice.
    pub const CANCER: Self = Self(3);
    /// ♌ Leo, 120°–150°, opening at 大暑.
    pub const LEO: Self = Self(4);
    /// ♍ Virgo, 150°–180°, opening at 処暑.
    pub const VIRGO: Self = Self(5);
    /// ♎ Libra, 180°–210°, opening at 秋分, the September equinox.
    pub const LIBRA: Self = Self(6);
    /// ♏ Scorpio, 210°–240°, opening at 霜降.
    pub const SCORPIO: Self = Self(7);
    /// ♐ Sagittarius, 240°–270°, opening at 小雪.
    pub const SAGITTARIUS: Self = Self(8);
    /// ♑ Capricorn, 270°–300°, opening at 冬至, the December solstice.
    pub const CAPRICORN: Self = Self(9);
    /// ♒ Aquarius, 300°–330°, opening at 大寒.
    pub const AQUARIUS: Self = Self(10);
    /// ♓ Pisces, 330°–360°, opening at 雨水.
    pub const PISCES: Self = Self(11);

    /// All twelve, in order from Aries.
    pub const ALL: [Self; SIGNS_PER_ZODIAC] = [
        Self::ARIES,
        Self::TAURUS,
        Self::GEMINI,
        Self::CANCER,
        Self::LEO,
        Self::VIRGO,
        Self::LIBRA,
        Self::SCORPIO,
        Self::SAGITTARIUS,
        Self::CAPRICORN,
        Self::AQUARIUS,
        Self::PISCES,
    ];

    /// The sign at an index reduced modulo twelve.
    ///
    /// Internal, so that call sites that have already proved the index is in
    /// range do not have to unwrap an `Option`.
    pub(crate) const fn at(index: u8) -> Self {
        Self(index % 12)
    }

    /// The sign at an index counted from Aries.
    ///
    /// Returns `None` for an index of 12 or more.
    #[must_use]
    pub const fn from_index(index: u8) -> Option<Self> {
        if index as usize >= SIGNS_PER_ZODIAC {
            return None;
        }
        Some(Self(index))
    }

    /// This sign's index from Aries, 0 to 11.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// The sign a whole number of degrees opens, if it opens one.
    ///
    /// Returns `None` unless the angle is a multiple of 30°. Negative and
    /// over-full angles are reduced first, so −30° is Pisces.
    #[must_use]
    pub const fn from_start_degrees(degrees: i32) -> Option<Self> {
        let reduced = degrees.rem_euclid(360);
        if reduced % 30 != 0 {
            return None;
        }
        Some(Self((reduced / 30) as u8))
    }

    /// The sign an arbitrary apparent tropical longitude falls in.
    #[must_use]
    pub fn containing_degrees(longitude_degrees: f64) -> Self {
        let reduced = hc_core::math::normalize_degrees(longitude_degrees);
        Self::at(floor(reduced / DEGREES_PER_SIGN) as u8)
    }

    /// The apparent tropical longitude, in degrees, at which the sign opens.
    #[must_use]
    pub const fn start_longitude_degrees(self) -> f64 {
        self.0 as f64 * DEGREES_PER_SIGN
    }

    /// The longitude at which the sign closes, i.e. at which the next opens.
    ///
    /// 360° for Pisces rather than 0°, because the arc is `start ..= end` and
    /// saying 0° there would make the arc look empty.
    #[must_use]
    pub const fn end_longitude_degrees(self) -> f64 {
        (self.0 as f64 + 1.0) * DEGREES_PER_SIGN
    }

    /// The 中気 at which this sign opens.
    ///
    /// The twelve tropical sign boundaries and the twelve principal solar
    /// terms are the same twelve angles. This is that identity as a function,
    /// and it is why this module lives in `hc-seasons`.
    #[must_use]
    pub const fn opening_term(self) -> SolarTerm {
        match SolarTerm::from_degrees(self.0 as i32 * 30) {
            Some(term) => term,
            // Unreachable: a multiple of 30 is a multiple of 15.
            None => SolarTerm::SPRING_EQUINOX,
        }
    }

    /// The 節気 that falls in the middle of this sign, 15° in.
    ///
    /// Every sign contains exactly one sectional term, because the two
    /// classes of solar term alternate strictly round the year.
    #[must_use]
    pub const fn midpoint_term(self) -> SolarTerm {
        match SolarTerm::from_degrees(self.0 as i32 * 30 + 15) {
            Some(term) => term,
            // Unreachable: 30n + 15 is a multiple of 15.
            None => SolarTerm::SPRING_EQUINOX,
        }
    }

    /// The name in English, e.g. `"Capricorn"`.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        SIGN_NAMES[self.0 as usize].english
    }

    /// The name in Latin, e.g. `"Capricornus"`.
    ///
    /// Differs from [`Self::english_name`] for exactly two signs: Scorpius
    /// and Capricornus, which English has clipped.
    #[must_use]
    pub const fn latin_name(self) -> &'static str {
        SIGN_NAMES[self.0 as usize].latin
    }

    /// What the sign depicts, in English, e.g. `"the Sea-goat"`.
    #[must_use]
    pub const fn emblem(self) -> &'static str {
        SIGN_NAMES[self.0 as usize].emblem
    }

    /// The 黄道十二宮 name in Japanese characters, e.g. `"磨羯宮"`.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        SIGN_NAMES[self.0 as usize].japanese
    }

    /// The astrological symbol, U+2648 ♈ through U+2653 ♓.
    #[must_use]
    pub const fn symbol(self) -> char {
        SIGN_NAMES[self.0 as usize].symbol
    }

    /// The element the sign is assigned to.
    ///
    /// The elements run fire, earth, air, water and repeat every four signs,
    /// so this is the index modulo four rather than a table.
    #[must_use]
    pub const fn element(self) -> Element {
        Element::ALL[(self.0 % 4) as usize]
    }

    /// The modality the sign is assigned to.
    ///
    /// Cardinal, fixed and mutable repeat every three signs, so this is the
    /// index modulo three. Because three and four are coprime, every
    /// element/modality pair occurs on exactly one of the twelve signs —
    /// which is the property the scheme was built to have.
    #[must_use]
    pub const fn modality(self) -> Modality {
        Modality::ALL[(self.0 % 3) as usize]
    }

    /// The classical ruling planet: the domicile scheme of Ptolemy's
    /// *Tetrabiblos*.
    ///
    /// The Sun rules Leo and the Moon rules Cancer, and the five remaining
    /// bodies are assigned outward from that pair in both directions in
    /// order of orbital period: Mercury, Venus, Mars, Jupiter, Saturn. So
    /// every body but the two lights rules two signs, one on each side.
    #[must_use]
    pub const fn ruling_planet(self) -> RulingPlanet {
        match self.0 {
            0 => RulingPlanet::Mars,
            1 => RulingPlanet::Venus,
            2 => RulingPlanet::Mercury,
            3 => RulingPlanet::Moon,
            4 => RulingPlanet::Sun,
            5 => RulingPlanet::Mercury,
            6 => RulingPlanet::Venus,
            7 => RulingPlanet::Mars,
            8 => RulingPlanet::Jupiter,
            9 => RulingPlanet::Saturn,
            10 => RulingPlanet::Saturn,
            _ => RulingPlanet::Jupiter,
        }
    }

    /// The ruling planet as modern Western astrology assigns it.
    ///
    /// Identical to [`Self::ruling_planet`] except for the three signs that
    /// were reassigned as the outer bodies were discovered: Scorpio to Pluto,
    /// Aquarius to Uranus, Pisces to Neptune. Indian astrology made no such
    /// reassignment and keeps the classical set — see
    /// [`super::SiderealSign::ruling_planet`].
    #[must_use]
    pub const fn modern_ruling_planet(self) -> RulingPlanet {
        match self.0 {
            7 => RulingPlanet::Pluto,
            10 => RulingPlanet::Uranus,
            11 => RulingPlanet::Neptune,
            _ => self.ruling_planet(),
        }
    }

    /// The fixed dates the sign is conventionally printed with.
    ///
    /// These are **not** computed: they are the dates an English-language
    /// astrology column prints, unchanged since roughly the 1920s. Compare
    /// them against [`ingress_day`] to see the drift; the integration test
    /// `zodiac_conventional_dates` does exactly that and reports the number.
    #[must_use]
    pub const fn conventional_period(self) -> ConventionalPeriod {
        SIGN_NAMES[self.0 as usize].conventional
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

    /// The sidereal sign of the same name and index.
    ///
    /// Same name, same number, different zero point: this is a change of
    /// coordinate convention, not of date. The two divisions put the same
    /// moment in different signs for most of the year.
    #[must_use]
    pub const fn as_sidereal(self) -> super::SiderealSign {
        super::SiderealSign::at(self.0)
    }
}

/// The Universal Time instant at which the Sun enters a sign in a Gregorian
/// year.
///
/// Each of the twelve ingresses happens exactly once in each Gregorian year,
/// for the same reason each 中気 does: the year opens with the Sun at about
/// 280°, so the first ingress is Aquarius around 20 January and the last is
/// Capricorn around 21 December, and none is ever pushed out of its year.
///
/// This is [`crate::solar_terms::term_moment`] of the sign's opening 中気,
/// and the two agree to the noise of the search.
#[must_use]
pub fn ingress_moment(year: i64, sign: TropicalSign) -> Moment {
    seasonal_event(year, sign.start_longitude_degrees())
}

/// The day the Sun enters a sign in a Gregorian year, at a given meridian.
///
/// ```
/// use hc_seasons::{Meridian, zodiac::{TropicalSign, tropical::ingress_day}};
///
/// // The Sun entered Aries — crossed the March equinox — on 20 March 2024
/// // in Japan, so 2024 began Aries a day before the conventional 21 March.
/// let day = ingress_day(2024, TropicalSign::ARIES, Meridian::JAPAN);
/// assert_eq!(day, hc_calendar::Rd(738_965));
/// ```
#[must_use]
pub fn ingress_day(year: i64, sign: TropicalSign, meridian: Meridian) -> Rd {
    meridian.day_of(ingress_moment(year, sign))
}

/// The period a sign occupies in a Gregorian year.
///
/// The year is the one the sign's *ingress* falls in, so Capricorn of 2024
/// starts on 21 December 2024 and ends in January 2025.
#[must_use]
pub fn sign_period(year: i64, sign: TropicalSign, meridian: Meridian) -> SignPeriod<TropicalSign> {
    period_from_start(sign, ingress_moment(year, sign), meridian)
}

/// A period built from a known ingress instant.
///
/// The closing instant is found by searching forward from a day after the
/// opening one: a sign is never shorter than about 29.4 days, so a day is a
/// safe step and the search cannot find the boundary it started from again.
fn period_from_start(
    sign: TropicalSign,
    start: Moment,
    meridian: Meridian,
) -> SignPeriod<TropicalSign> {
    let end = solar_longitude_after(sign.next().start_longitude_degrees(), Moment(start.0 + 1.0));
    SignPeriod {
        sign,
        start,
        end,
        start_day: meridian.day_of(start),
        end_day: Rd(meridian.day_of(end).0 - 1),
    }
}

/// The sign the Sun is in at an instant.
#[must_use]
pub fn sign_at_moment(moment: Moment) -> TropicalSign {
    TropicalSign::containing_degrees(solar_longitude(moment))
}

/// How far into its sign the Sun is at an instant, in degrees from 0 up to
/// but not including 30.
///
/// This is what an ephemeris prints as "12° Taurus": the sign from
/// [`sign_at_moment`] and the degrees from here.
#[must_use]
pub fn degrees_into_sign(moment: Moment) -> f64 {
    let longitude = solar_longitude(moment);
    degrees_into_arc(longitude, sign_at_moment(moment).start_longitude_degrees())
}

/// The sign and the degrees into it at one instant, computed together.
#[must_use]
pub fn sign_and_degrees(moment: Moment) -> (TropicalSign, f64) {
    let longitude = solar_longitude(moment);
    let sign = TropicalSign::containing_degrees(longitude);
    (
        sign,
        degrees_into_arc(longitude, sign.start_longitude_degrees()),
    )
}

/// The period of the sign in effect on a day, at a given meridian.
///
/// A sign "begins" on the day its ingress instant falls on, even if that
/// instant is at eleven at night, because that is how a calendar prints it.
#[must_use]
pub fn sign_in_effect(day: Rd, meridian: Meridian) -> SignPeriod<TropicalSign> {
    let end_of_day = meridian.midnight(Rd(day.0 + 1));
    let sign = TropicalSign::containing_degrees(solar_longitude(end_of_day));
    // The Sun covers 30° in at most about 31.5 days, and covers at least
    // 33.4° in any 35 days, so a search begun 35 days back is always inside
    // the previous sign and brackets exactly one crossing: the one wanted.
    let start = solar_longitude_after(sign.start_longitude_degrees(), Moment(end_of_day.0 - 35.0));
    period_from_start(sign, start, meridian)
}

/// The sign in effect on a day.
///
/// ```
/// use hc_seasons::{Meridian, zodiac::{TropicalSign, tropical::sign_on_day}};
///
/// // 10 May 2024.
/// let sign = sign_on_day(hc_calendar::Rd(739_016), Meridian::JAPAN);
/// assert_eq!(sign, TropicalSign::TAURUS);
/// assert_eq!(sign.symbol(), '♉');
/// ```
#[must_use]
pub fn sign_on_day(day: Rd, meridian: Meridian) -> TropicalSign {
    sign_in_effect(day, meridian).sign
}

/// The sign whose period begins on a day, if one does.
///
/// Twelve days a year answer `Some`; the rest answer `None`.
#[must_use]
pub fn sign_beginning_on(day: Rd, meridian: Meridian) -> Option<SignPeriod<TropicalSign>> {
    let period = sign_in_effect(day, meridian);
    if period.start_day == day {
        Some(period)
    } else {
        None
    }
}

/// How many days a day is into the sign in effect, counting the ingress day
/// as 0.
#[must_use]
pub fn days_into_sign(day: Rd, meridian: Meridian) -> i64 {
    day.0 - sign_in_effect(day, meridian).start_day.0
}

/// The twelve sign periods of a Gregorian year, in date order.
///
/// Date order starts with Aquarius around 20 January, not with Aries, because
/// that is the order the signs appear in a Gregorian year. The last period is
/// Capricorn's, which starts around 21 December and ends in the following
/// January. Use [`TropicalSign::ALL`] for zodiacal order instead.
///
/// Consecutive periods share an instant exactly, so the twelve partition the
/// year with no gap and no overlap.
#[must_use]
pub fn signs_in_year(year: i64, meridian: Meridian) -> SignsInYear {
    let sign = TropicalSign::at(FIRST_SIGN_OF_GREGORIAN_YEAR);
    SignsInYear {
        meridian,
        sign,
        start: ingress_moment(year, sign),
        remaining: SIGNS_PER_ZODIAC,
    }
}

/// The iterator returned by [`signs_in_year`].
#[derive(Debug, Clone, Copy)]
pub struct SignsInYear {
    meridian: Meridian,
    sign: TropicalSign,
    start: Moment,
    remaining: usize,
}

impl Iterator for SignsInYear {
    type Item = SignPeriod<TropicalSign>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let period = period_from_start(self.sign, self.start, self.meridian);
        self.remaining -= 1;
        self.sign = self.sign.next();
        // The closing instant of one sign is the opening instant of the next,
        // carried forward exactly so the twelve cannot drift apart.
        self.start = period.end;
        Some(period)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for SignsInYear {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::{from_year_month_day, year_month_day_from_rd};
    use crate::solar_terms::{TermKind, term_moment};

    const JAPAN: Meridian = Meridian::JAPAN;

    #[test]
    fn the_twelve_signs_are_thirty_degree_arcs_from_the_equinox() {
        for (index, sign) in TropicalSign::ALL.into_iter().enumerate() {
            assert_eq!(sign.index() as usize, index);
            assert_eq!(TropicalSign::from_index(index as u8), Some(sign));
            assert!((sign.start_longitude_degrees() - index as f64 * 30.0).abs() < 1e-12);
            assert!(
                (sign.end_longitude_degrees() - sign.start_longitude_degrees() - 30.0).abs()
                    < 1e-12
            );
        }
        assert_eq!(TropicalSign::from_index(12), None);
        assert_eq!(TropicalSign::from_index(200), None);
    }

    #[test]
    fn only_multiples_of_thirty_degrees_open_a_sign() {
        assert_eq!(
            TropicalSign::from_start_degrees(0),
            Some(TropicalSign::ARIES)
        );
        assert_eq!(
            TropicalSign::from_start_degrees(270),
            Some(TropicalSign::CAPRICORN)
        );
        assert_eq!(TropicalSign::from_start_degrees(15), None);
        assert_eq!(TropicalSign::from_start_degrees(29), None);
        // Negative and over-full angles reduce first.
        assert_eq!(
            TropicalSign::from_start_degrees(-30),
            Some(TropicalSign::PISCES)
        );
        assert_eq!(
            TropicalSign::from_start_degrees(720),
            Some(TropicalSign::ARIES)
        );
    }

    #[test]
    fn an_arbitrary_longitude_lands_in_the_arc_that_covers_it() {
        assert_eq!(TropicalSign::containing_degrees(0.0), TropicalSign::ARIES);
        assert_eq!(
            TropicalSign::containing_degrees(29.999),
            TropicalSign::ARIES
        );
        assert_eq!(TropicalSign::containing_degrees(30.0), TropicalSign::TAURUS);
        assert_eq!(
            TropicalSign::containing_degrees(359.999),
            TropicalSign::PISCES
        );
        // Out-of-range angles reduce rather than panic.
        assert_eq!(TropicalSign::containing_degrees(-1.0), TropicalSign::PISCES);
        assert_eq!(
            TropicalSign::containing_degrees(400.0),
            TropicalSign::TAURUS
        );
    }

    /// The point of putting this module in `hc-seasons`: the twelve tropical
    /// sign boundaries are the twelve 中気, not something merely similar to
    /// them.
    #[test]
    fn every_sign_opens_at_a_principal_solar_term() {
        for sign in TropicalSign::ALL {
            let term = sign.opening_term();
            assert_eq!(term.kind(), TermKind::Principal);
            assert!(
                (term.solar_longitude_degrees() - sign.start_longitude_degrees()).abs() < 1e-12
            );
        }
        assert_eq!(
            TropicalSign::ARIES.opening_term(),
            crate::solar_terms::SolarTerm::SPRING_EQUINOX
        );
        assert_eq!(
            TropicalSign::CANCER.opening_term(),
            crate::solar_terms::SolarTerm::SUMMER_SOLSTICE
        );
        assert_eq!(
            TropicalSign::LIBRA.opening_term(),
            crate::solar_terms::SolarTerm::AUTUMN_EQUINOX
        );
        assert_eq!(
            TropicalSign::CAPRICORN.opening_term(),
            crate::solar_terms::SolarTerm::WINTER_SOLSTICE
        );
    }

    #[test]
    fn every_sign_has_exactly_one_sectional_term_in_its_middle() {
        for sign in TropicalSign::ALL {
            let term = sign.midpoint_term();
            assert_eq!(term.kind(), TermKind::Sectional);
            let into = term.solar_longitude_degrees() - sign.start_longitude_degrees();
            assert!(
                ((into + 360.0) % 360.0 - 15.0).abs() < 1e-12,
                "{} midpoint was {into} degrees in",
                sign.english_name()
            );
        }
        assert_eq!(
            TropicalSign::CAPRICORN.midpoint_term(),
            crate::solar_terms::SolarTerm::from_degrees(285).unwrap()
        );
    }

    #[test]
    fn a_sign_ingress_is_the_same_instant_as_its_opening_term() {
        for year in [1900i64, 1950, 2000, 2024, 2050, 2099] {
            for sign in TropicalSign::ALL {
                let ingress = ingress_moment(year, sign);
                let term = term_moment(year, sign.opening_term());
                assert!(
                    (ingress.0 - term.0).abs() < 1e-6,
                    "{} of {year}: {} vs {}",
                    sign.english_name(),
                    ingress.0,
                    term.0
                );
            }
        }
    }

    /// Aries, Cancer, Libra and Capricorn are the four cardinal signs
    /// precisely because they open at the equinoxes and solstices.
    #[test]
    fn the_cardinal_signs_open_at_the_equinoxes_and_solstices() {
        use hc_astro::solar::{Equinox, Solstice, equinox, solstice};

        for year in 1950..2050 {
            let expected: [(TropicalSign, Moment); 4] = [
                (TropicalSign::ARIES, equinox(year, Equinox::March)),
                (TropicalSign::CANCER, solstice(year, Solstice::June)),
                (TropicalSign::LIBRA, equinox(year, Equinox::September)),
                (TropicalSign::CAPRICORN, solstice(year, Solstice::December)),
            ];
            for (sign, moment) in expected {
                assert!(
                    (ingress_moment(year, sign).0 - moment.0).abs() < 1e-6,
                    "{} of {year}",
                    sign.english_name()
                );
                assert!(
                    matches!(sign.modality(), Modality::Cardinal),
                    "{} is not cardinal",
                    sign.english_name()
                );
            }
        }
    }

    #[test]
    fn only_the_four_cardinal_signs_are_cardinal() {
        let cardinal: usize = TropicalSign::ALL
            .iter()
            .filter(|sign| matches!(sign.modality(), Modality::Cardinal))
            .count();
        assert_eq!(cardinal, 4);
        for sign in [
            TropicalSign::ARIES,
            TropicalSign::CANCER,
            TropicalSign::LIBRA,
            TropicalSign::CAPRICORN,
        ] {
            assert_eq!(sign.modality(), Modality::Cardinal);
        }
    }

    /// Three modalities and four elements are coprime, so the twelve signs
    /// realise each of the twelve pairs exactly once. That is the structural
    /// fact the scheme was built around, and it is worth asserting rather
    /// than trusting a hand-written table.
    #[test]
    fn every_element_and_modality_pair_occurs_on_exactly_one_sign() {
        let mut seen = [[0u8; 3]; 4];
        for sign in TropicalSign::ALL {
            let element = Element::ALL
                .iter()
                .position(|item| *item == sign.element())
                .unwrap();
            let modality = Modality::ALL
                .iter()
                .position(|item| *item == sign.modality())
                .unwrap();
            seen[element][modality] += 1;
        }
        for row in seen {
            for count in row {
                assert_eq!(count, 1);
            }
        }
    }

    #[test]
    fn each_element_covers_three_signs_and_each_modality_four() {
        for element in Element::ALL {
            let count = TropicalSign::ALL
                .iter()
                .filter(|sign| sign.element() == element)
                .count();
            assert_eq!(count, 3, "{}", element.english_name());
            assert!(!element.latin_name().is_empty());
        }
        for modality in Modality::ALL {
            let count = TropicalSign::ALL
                .iter()
                .filter(|sign| sign.modality() == modality)
                .count();
            assert_eq!(count, 4, "{}", modality.english_name());
        }
    }

    /// The domicile scheme is symmetric about the Leo–Cancer axis: the sign
    /// `n` places after Leo and the sign `n` places before Cancer share a
    /// ruler.
    #[test]
    fn the_classical_rulerships_are_symmetric_about_the_two_lights() {
        assert_eq!(TropicalSign::LEO.ruling_planet(), RulingPlanet::Sun);
        assert_eq!(TropicalSign::CANCER.ruling_planet(), RulingPlanet::Moon);
        for step in 1..=5u8 {
            let after_leo = TropicalSign::at(TropicalSign::LEO.index() + step);
            let before_cancer = TropicalSign::at(TropicalSign::CANCER.index() + 12 - step);
            assert_eq!(
                after_leo.ruling_planet(),
                before_cancer.ruling_planet(),
                "{} and {} should share a ruler",
                after_leo.english_name(),
                before_cancer.english_name()
            );
        }
        // All seven classical bodies are used, and nothing else.
        for sign in TropicalSign::ALL {
            assert!(
                sign.ruling_planet().is_classical(),
                "{} is ruled by a body Ptolemy could not see",
                sign.english_name()
            );
        }
        assert_eq!(RulingPlanet::CHALDEAN_ORDER.len(), 7);
        for planet in RulingPlanet::CHALDEAN_ORDER {
            assert!(planet.is_classical());
            assert!(!planet.english_name().is_empty());
        }
    }

    #[test]
    fn the_modern_scheme_reassigns_exactly_three_signs() {
        let changed: usize = TropicalSign::ALL
            .iter()
            .filter(|sign| sign.ruling_planet() != sign.modern_ruling_planet())
            .count();
        assert_eq!(changed, 3);
        assert_eq!(
            TropicalSign::SCORPIO.modern_ruling_planet(),
            RulingPlanet::Pluto
        );
        assert_eq!(
            TropicalSign::AQUARIUS.modern_ruling_planet(),
            RulingPlanet::Uranus
        );
        assert_eq!(
            TropicalSign::PISCES.modern_ruling_planet(),
            RulingPlanet::Neptune
        );
        for planet in [
            RulingPlanet::Uranus,
            RulingPlanet::Neptune,
            RulingPlanet::Pluto,
        ] {
            assert!(!planet.is_classical());
        }
    }

    /// The symbols are consecutive from U+2648, so the table can be checked
    /// against arithmetic rather than against a second copy of itself.
    #[test]
    fn the_symbols_run_consecutively_from_the_aries_code_point() {
        for sign in TropicalSign::ALL {
            let expected = char::from_u32(0x2648 + u32::from(sign.index())).unwrap();
            assert_eq!(sign.symbol(), expected, "{}", sign.english_name());
        }
        assert_eq!(TropicalSign::ARIES.symbol(), '♈');
        assert_eq!(TropicalSign::PISCES.symbol(), '♓');
    }

    #[test]
    fn every_sign_has_a_full_set_of_names() {
        for sign in TropicalSign::ALL {
            assert!(!sign.english_name().is_empty());
            assert!(sign.english_name().is_ascii());
            assert!(!sign.latin_name().is_empty());
            assert!(sign.latin_name().is_ascii());
            assert!(sign.emblem().starts_with("the "));
            assert!(sign.japanese_name().ends_with('宮'));
        }
    }

    /// English clipped two of the Latin names and left ten alone.
    #[test]
    fn two_signs_are_spelled_differently_in_english_and_latin() {
        let differing: usize = TropicalSign::ALL
            .iter()
            .filter(|sign| sign.english_name() != sign.latin_name())
            .count();
        assert_eq!(differing, 2);
        assert_eq!(TropicalSign::SCORPIO.latin_name(), "Scorpius");
        assert_eq!(TropicalSign::CAPRICORN.latin_name(), "Capricornus");
    }

    #[test]
    fn stepping_round_the_zodiac_returns_to_the_same_sign() {
        for sign in TropicalSign::ALL {
            assert_eq!(sign.next().previous(), sign);
            assert_eq!(sign.previous().next(), sign);
            assert_eq!(sign.opposite().opposite(), sign);
            assert_ne!(sign.opposite(), sign);
            assert_eq!(sign.as_sidereal().index(), sign.index());
        }
        assert_eq!(TropicalSign::PISCES.next(), TropicalSign::ARIES);
        assert_eq!(TropicalSign::ARIES.previous(), TropicalSign::PISCES);
        assert_eq!(TropicalSign::ARIES.opposite(), TropicalSign::LIBRA);
    }

    #[test]
    fn all_twelve_signs_are_distinct() {
        for (position, sign) in TropicalSign::ALL.iter().enumerate() {
            for other in &TropicalSign::ALL[position + 1..] {
                assert_ne!(sign, other);
                assert_ne!(sign.english_name(), other.english_name());
                assert_ne!(sign.symbol(), other.symbol());
                assert_ne!(sign.japanese_name(), other.japanese_name());
            }
        }
    }

    /// The equinox has moved since the conventional dates were printed, so
    /// the 2024 ingresses land a day earlier than the newspaper says for most
    /// signs. This is the assertion; the integration test reports the rate.
    #[test]
    fn the_ingresses_of_2024_land_where_the_astronomy_puts_them() {
        let expected = [
            (TropicalSign::ARIES, (3, 20)),
            (TropicalSign::CANCER, (6, 21)),
            (TropicalSign::LIBRA, (9, 22)),
            (TropicalSign::CAPRICORN, (12, 21)),
        ];
        for (sign, (month, day)) in expected {
            assert_eq!(
                ingress_day(2024, sign, JAPAN),
                from_year_month_day(2024, month, day),
                "{} of 2024",
                sign.english_name()
            );
        }
    }

    #[test]
    fn a_year_has_twelve_periods_in_strictly_increasing_order() {
        for year in [1900i64, 1950, 2000, 2024, 2050, 2099] {
            let mut previous: Option<SignPeriod<TropicalSign>> = None;
            let mut count = 0;
            for period in signs_in_year(year, JAPAN) {
                if let Some(earlier) = previous {
                    assert!(period.start.0 > earlier.start.0);
                    assert!(period.start_day.0 > earlier.start_day.0);
                }
                previous = Some(period);
                count += 1;
            }
            assert_eq!(count, SIGNS_PER_ZODIAC);
        }
    }

    /// The headline property: the twelve periods tile the year exactly. The
    /// end instant of one is the start instant of the next, and the end day
    /// of one is the day before the start day of the next.
    #[test]
    fn the_twelve_periods_partition_the_year_with_no_gap_and_no_overlap() {
        for year in 1990..2040 {
            let mut previous: Option<SignPeriod<TropicalSign>> = None;
            for period in signs_in_year(year, JAPAN) {
                if let Some(earlier) = previous {
                    assert!(
                        (earlier.end.0 - period.start.0).abs() < 1e-9,
                        "{} of {year} did not hand over exactly",
                        earlier.sign.english_name()
                    );
                    assert_eq!(
                        earlier.end_day.0 + 1,
                        period.start_day.0,
                        "{} of {year} left a gap before {}",
                        earlier.sign.english_name(),
                        period.sign.english_name()
                    );
                    assert_eq!(earlier.sign.next(), period.sign);
                }
                previous = Some(period);
            }
        }
    }

    #[test]
    fn a_years_periods_are_the_twelve_signs_exactly_once_each() {
        let mut seen = [false; SIGNS_PER_ZODIAC];
        for period in signs_in_year(2024, JAPAN) {
            let index = period.sign.index() as usize;
            assert!(
                !seen[index],
                "{} appeared twice",
                period.sign.english_name()
            );
            seen[index] = true;
        }
        assert!(seen.iter().all(|flag| *flag));
    }

    #[test]
    fn a_years_iteration_opens_with_aquarius_and_closes_with_capricorn() {
        let mut iterator = signs_in_year(2024, JAPAN);
        assert_eq!(iterator.len(), 12);
        let first = iterator.next().unwrap();
        assert_eq!(first.sign, TropicalSign::AQUARIUS);
        assert_eq!(year_month_day_from_rd(first.start_day).1, 1);
        let last = iterator.last().unwrap();
        assert_eq!(last.sign, TropicalSign::CAPRICORN);
        assert_eq!(year_month_day_from_rd(last.start_day).1, 12);
        // Capricorn's period runs into the next Gregorian year.
        assert_eq!(year_month_day_from_rd(last.end_day).0, 2025);
    }

    /// The Earth's orbit is an ellipse, so a 30° arc takes between about 29.4
    /// and 31.5 days. Capricorn and Sagittarius — crossed near perihelion in
    /// January — are the short ones; Gemini and Cancer the long ones.
    #[test]
    fn the_signs_are_not_of_equal_length_and_the_short_ones_are_the_winter_ones() {
        let mut shortest = (TropicalSign::ARIES, 99i64);
        let mut longest = (TropicalSign::ARIES, 0i64);
        for period in signs_in_year(2024, JAPAN) {
            let length = period.length_days();
            assert!(
                (29..=32).contains(&length),
                "{} ran {length} days",
                period.sign.english_name()
            );
            assert!((29.0..=32.0).contains(&period.duration_days()));
            if length < shortest.1 {
                shortest = (period.sign, length);
            }
            if length > longest.1 {
                longest = (period.sign, length);
            }
        }
        assert!(
            matches!(
                shortest.0,
                TropicalSign::SAGITTARIUS | TropicalSign::CAPRICORN
            ),
            "the shortest sign was {}",
            shortest.0.english_name()
        );
        assert!(
            matches!(longest.0, TropicalSign::GEMINI | TropicalSign::CANCER),
            "the longest sign was {}",
            longest.0.english_name()
        );
        assert!(longest.1 - shortest.1 >= 2);
    }

    #[test]
    fn every_day_of_a_year_falls_in_exactly_one_sign() {
        let start = from_year_month_day(2024, 1, 1);
        let mut counts = [0i64; SIGNS_PER_ZODIAC];
        for offset in 0..366 {
            let day = Rd(start.0 + offset);
            let period = sign_in_effect(day, JAPAN);
            assert!(
                period.contains(day),
                "{day} was not inside the period it reported"
            );
            counts[period.sign.index() as usize] += 1;
        }
        assert_eq!(counts.iter().sum::<i64>(), 366);
        for (index, count) in counts.into_iter().enumerate() {
            assert!(
                (29..=32).contains(&count),
                "{} got {count} days",
                TropicalSign::at(index as u8).english_name()
            );
        }
    }

    #[test]
    fn exactly_twelve_days_of_a_year_begin_a_sign() {
        let start = from_year_month_day(2024, 1, 1);
        let end = from_year_month_day(2025, 1, 1);
        let mut beginnings = 0;
        for offset in 0..(end.0 - start.0) {
            if sign_beginning_on(Rd(start.0 + offset), JAPAN).is_some() {
                beginnings += 1;
            }
        }
        assert_eq!(beginnings, 12);
    }

    #[test]
    fn the_sign_in_effect_agrees_with_the_years_own_table() {
        for period in signs_in_year(2024, JAPAN) {
            assert_eq!(sign_on_day(period.start_day, JAPAN), period.sign);
            assert_eq!(sign_on_day(period.end_day, JAPAN), period.sign);
            assert_eq!(
                sign_on_day(Rd(period.start_day.0 - 1), JAPAN),
                period.sign.previous()
            );
            assert_eq!(days_into_sign(period.start_day, JAPAN), 0);
            assert_eq!(
                days_into_sign(period.end_day, JAPAN),
                period.length_days() - 1
            );
        }
    }

    #[test]
    fn degrees_into_a_sign_run_from_zero_to_thirty_and_climb_through_it() {
        for period in signs_in_year(2024, JAPAN) {
            // Just after the ingress the Sun is barely into the sign; just
            // before the next it is nearly out of it.
            let early = degrees_into_sign(Moment(period.start.0 + 0.001));
            let late = degrees_into_sign(Moment(period.end.0 - 0.001));
            assert!(early < 0.01, "{early} degrees in just after the ingress");
            assert!(late > 29.99, "{late} degrees in just before the next");
            let (sign, degrees) = sign_and_degrees(Moment(period.start.0 + 10.0));
            assert_eq!(sign, period.sign);
            assert!(
                (9.0..=11.0).contains(&degrees),
                "{degrees} degrees ten days in"
            );
            assert_eq!(sign_at_moment(Moment(period.start.0 + 10.0)), period.sign);
        }
    }

    /// A sign boundary moves through the leap-year cycle the way the equinox
    /// does: back by about a quarter of a day each common year, forward by
    /// three-quarters of a day at each leap year. Over a four-year cycle the
    /// date is nearly restored, but it is a day earlier at the end of a
    /// century that is not a leap year.
    #[test]
    fn a_sign_boundary_moves_by_about_a_day_across_a_leap_year_cycle() {
        for sign in TropicalSign::ALL {
            let mut earliest = f64::MAX;
            let mut latest = f64::MIN;
            for year in 2020..2024 {
                let ingress = ingress_moment(year, sign);
                // The fraction of the year the ingress falls at, so the
                // comparison is not confused by the year changing.
                let fraction = ingress.0 - crate::gregorian::new_year(year).0 as f64;
                earliest = if fraction < earliest {
                    fraction
                } else {
                    earliest
                };
                latest = if fraction > latest { fraction } else { latest };
            }
            let spread = latest - earliest;
            assert!(
                (0.5..=1.3).contains(&spread),
                "{} moved {spread} days across the cycle",
                sign.english_name()
            );
        }
    }

    #[test]
    fn a_sign_boundary_falls_on_one_of_two_days_over_a_leap_year_cycle() {
        for sign in TropicalSign::ALL {
            let mut days = [0u8; 40];
            for year in 2000..2040 {
                let day = ingress_day(year, sign, JAPAN);
                days[(year - 2000) as usize] = year_month_day_from_rd(day).2;
            }
            let smallest = days.iter().copied().min().unwrap();
            let largest = days.iter().copied().max().unwrap();
            // Four decades of a signature drift: the day of the month takes
            // two, occasionally three, adjacent values.
            assert!(
                largest - smallest <= 2,
                "{} ranged over {smallest}..{largest}",
                sign.english_name()
            );
            assert!(largest > smallest, "{} never moved", sign.english_name());
        }
    }

    /// The meridian is not decoration here either: an ingress instant lands
    /// in the hour between Beijing midnight and Tokyo midnight about one year
    /// in twenty-four, so the two put a sign boundary on different dates.
    #[test]
    fn tokyo_and_beijing_do_not_always_agree_on_a_sign_boundary() {
        let mut disagreements = 0;
        for year in 1950..2050 {
            for sign in TropicalSign::ALL {
                if ingress_day(year, sign, Meridian::JAPAN)
                    != ingress_day(year, sign, Meridian::CHINA)
                {
                    disagreements += 1;
                }
            }
        }
        assert!(
            (20..=120).contains(&disagreements),
            "{disagreements} disagreements out of 1200 is not the expected rate"
        );
    }

    #[test]
    fn the_conventional_dates_are_a_closed_cycle_of_twelve() {
        for sign in TropicalSign::ALL {
            let period = sign.conventional_period();
            assert!((1..=12).contains(&period.start_month));
            assert!((1..=31).contains(&period.start_day));
            let next = sign.next().conventional_period();
            // The day after one sign conventionally ends is the day the next
            // conventionally begins, so the printed dates tile the year too.
            let ends = from_year_month_day(2001, period.end_month, period.end_day);
            let begins = from_year_month_day(2001, next.start_month, next.start_day);
            let gap = (begins.0 - ends.0).rem_euclid(365);
            assert_eq!(
                gap,
                1,
                "{} ends and {} begins {gap} days apart",
                sign.english_name(),
                sign.next().english_name()
            );
        }
    }
}
