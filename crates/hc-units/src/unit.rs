//! The catalogue of exactly defined units, and conversion between them.
//!
//! # What earns a place here
//!
//! A unit belongs in this table when some authority *defined* it as a fixed
//! multiple of the second — a standards body, a decree, a rabbinic or
//! astronomical text, a published specification. It does not belong here
//! when somebody *measured* it. The sidereal day, the tropical year, the
//! synodic month and the galactic year are all measurements: they have error
//! bars, they are revised, and the crate that owns the model that produced
//! them is the crate that should state them. They live in `hc-astro`,
//! `hc-planetary` and `hc-deep-time` respectively, with their uncertainties
//! attached.
//!
//! The division is not pedantry. It is what lets everything in this module
//! be a [`Ratio`] with no error term, so that conversions compose without
//! ever asking how much precision survived.
//!
//! # Competing conventions
//!
//! Where two traditions use one name for two lengths, both appear, each
//! under its own name, per the repository's policy §5. The Chinese 刻 is
//! 1/100 of a day in the 百刻 system and 1/96 of a day after the 時憲曆
//! reform of 1645; [`KE_HUNDRED`] and [`KE_NINETY_SIX`] are separate
//! entries rather than one entry with a flag.

use crate::error::{UnitError, UnitResult};
use crate::ratio::Ratio;

/// Where a unit comes from, which is also how far you should trust it
/// outside its home domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Family {
    /// The second and its SI prefixes.
    Si,
    /// The units of the civil day, week and year.
    Civil,
    /// Subdivisions of the day from a historical timekeeping tradition.
    Horological,
    /// Decimal divisions of the day.
    Decimal,
    /// Binary and hexadecimal divisions of the day.
    Hexadecimal,
    /// Units from film, audio, broadcast and computing specifications.
    Media,
    /// Units defined by a scientific field for its own convenience.
    Scientific,
}

impl Family {
    /// The family's name in English, spelled as the field spells it.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Si => "SI",
            Self::Civil => "civil",
            Self::Horological => "horological",
            Self::Decimal => "decimal",
            Self::Hexadecimal => "hexadecimal",
            Self::Media => "media",
            Self::Scientific => "scientific",
        }
    }
}

/// A unit of time with an exactly defined length.
///
/// This is a plain struct rather than an enum on purpose. An enum would make
/// this crate the sole authority on which units exist, and the set is open —
/// a caller with a unit from a tradition this table has never heard of
/// should be able to write it down and convert with it, not send a pull
/// request first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Unit {
    /// A stable identifier, lowercase and hyphenated.
    pub id: &'static str,
    /// The name in English.
    pub name: &'static str,
    /// The conventional symbol or abbreviation, where one exists.
    pub symbol: Option<&'static str>,
    /// The exact length in seconds.
    pub seconds: Ratio,
    /// Which tradition defines it.
    pub family: Family,
    /// Who defines it, specifically enough to check.
    pub authority: &'static str,
}

impl Unit {
    /// How many of `other` make one of `self`, exactly.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the ratio does not fit in `i128`, which
    /// needs a span of more than 38 decades between the two units.
    pub const fn per(self, other: Self) -> UnitResult<Ratio> {
        self.seconds.checked_div(other.seconds)
    }
}

/// A count of a unit: the thing a conversion actually moves between scales.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity {
    /// How many, exactly — a [`Ratio`] rather than an integer so that
    /// "half a beat" and "2.5 frames" are first-class.
    pub count: Ratio,
    /// The unit counted.
    pub unit: Unit,
}

impl Quantity {
    /// A quantity of a unit.
    #[must_use]
    pub const fn new(count: Ratio, unit: Unit) -> Self {
        Self { count, unit }
    }

    /// A whole number of a unit.
    #[must_use]
    pub const fn whole(count: i128, unit: Unit) -> Self {
        Self {
            count: Ratio::from_secs(count),
            unit,
        }
    }

    /// The exact length in seconds.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the product leaves `i128`.
    pub const fn seconds(self) -> UnitResult<Ratio> {
        self.count.checked_mul(self.unit.seconds)
    }

    /// The same length counted in another unit, exactly.
    ///
    /// This is the conversion the crate exists for: `Quantity::whole(1,
    /// FRAME_AT_24).to(FLICK)` is 29 400 000 flicks with no remainder, and
    /// the same call for a unit that does not divide evenly returns a
    /// fraction rather than a rounded integer.
    ///
    /// # Errors
    ///
    /// [`UnitError::Overflow`] if the intermediate product leaves `i128`.
    pub const fn to(self, unit: Unit) -> UnitResult<Self> {
        let Ok(seconds) = self.seconds() else {
            return Err(UnitError::Overflow);
        };
        let Ok(count) = seconds.checked_div(unit.seconds) else {
            return Err(UnitError::Overflow);
        };
        Ok(Self { count, unit })
    }

    /// Whether this quantity is a whole number of `unit`.
    ///
    /// The question a media pipeline asks: does a frame land on a sample
    /// boundary, does a bar land on a frame?
    #[must_use]
    pub const fn divides_into(self, unit: Unit) -> bool {
        match self.to(unit) {
            Ok(converted) => converted.count.is_integer(),
            Err(_) => false,
        }
    }
}

/// Shorthand for the table below.
const fn unit(
    id: &'static str,
    name: &'static str,
    symbol: Option<&'static str>,
    num: i128,
    den: i128,
    family: Family,
    authority: &'static str,
) -> Unit {
    Unit {
        id,
        name,
        symbol,
        seconds: Ratio::literal(num, den),
        family,
        authority,
    }
}

// --- SI ---------------------------------------------------------------------

/// The SI second, defined by the caesium-133 hyperfine transition.
pub const SECOND: Unit = unit(
    "second",
    "second",
    Some("s"),
    1,
    1,
    Family::Si,
    "BIPM, SI Brochure 9th edition (2019)",
);
/// A thousandth of a second.
pub const MILLISECOND: Unit = unit(
    "millisecond",
    "millisecond",
    Some("ms"),
    1,
    1_000,
    Family::Si,
    "BIPM, SI prefixes",
);
/// A millionth of a second.
pub const MICROSECOND: Unit = unit(
    "microsecond",
    "microsecond",
    Some("\u{b5}s"),
    1,
    1_000_000,
    Family::Si,
    "BIPM, SI prefixes",
);
/// A billionth of a second.
pub const NANOSECOND: Unit = unit(
    "nanosecond",
    "nanosecond",
    Some("ns"),
    1,
    1_000_000_000,
    Family::Si,
    "BIPM, SI prefixes",
);
/// A trillionth of a second.
pub const PICOSECOND: Unit = unit(
    "picosecond",
    "picosecond",
    Some("ps"),
    1,
    1_000_000_000_000,
    Family::Si,
    "BIPM, SI prefixes",
);
/// 10⁻¹⁵ s.
pub const FEMTOSECOND: Unit = unit(
    "femtosecond",
    "femtosecond",
    Some("fs"),
    1,
    1_000_000_000_000_000,
    Family::Si,
    "BIPM, SI prefixes",
);
/// 10⁻¹⁸ s, the resolution of [`hc_core::Duration`].
pub const ATTOSECOND: Unit = unit(
    "attosecond",
    "attosecond",
    Some("as"),
    1,
    1_000_000_000_000_000_000,
    Family::Si,
    "BIPM, SI prefixes",
);
/// 10⁻²¹ s.
pub const ZEPTOSECOND: Unit = unit(
    "zeptosecond",
    "zeptosecond",
    Some("zs"),
    1,
    1_000_000_000_000_000_000_000,
    Family::Si,
    "BIPM, SI prefixes",
);
/// 10⁻²⁴ s.
pub const YOCTOSECOND: Unit = unit(
    "yoctosecond",
    "yoctosecond",
    Some("ys"),
    1,
    1_000_000_000_000_000_000_000_000,
    Family::Si,
    "BIPM, SI prefixes",
);
/// 10⁻²⁷ s, one of the prefixes added in 2022.
pub const RONTOSECOND: Unit = unit(
    "rontosecond",
    "rontosecond",
    Some("rs"),
    1,
    1_000_000_000_000_000_000_000_000_000,
    Family::Si,
    "CGPM Resolution 3 (2022)",
);
/// 10⁻³⁰ s, the smallest SI prefix.
pub const QUECTOSECOND: Unit = unit(
    "quectosecond",
    "quectosecond",
    Some("qs"),
    1,
    1_000_000_000_000_000_000_000_000_000_000,
    Family::Si,
    "CGPM Resolution 3 (2022)",
);
/// A thousand seconds.
pub const KILOSECOND: Unit = unit(
    "kilosecond",
    "kilosecond",
    Some("ks"),
    1_000,
    1,
    Family::Si,
    "BIPM, SI prefixes",
);
/// A million seconds, about 11.6 days.
pub const MEGASECOND: Unit = unit(
    "megasecond",
    "megasecond",
    Some("Ms"),
    1_000_000,
    1,
    Family::Si,
    "BIPM, SI prefixes",
);
/// A billion seconds, about 31.7 years.
pub const GIGASECOND: Unit = unit(
    "gigasecond",
    "gigasecond",
    Some("Gs"),
    1_000_000_000,
    1,
    Family::Si,
    "BIPM, SI prefixes",
);

// --- Civil ------------------------------------------------------------------

/// Sixty seconds. Not sixty-one: a minute containing a leap second is a
/// property of UTC, not of the unit.
pub const MINUTE: Unit = unit(
    "minute",
    "minute",
    Some("min"),
    60,
    1,
    Family::Civil,
    "BIPM, units accepted for use with the SI",
);
/// Sixty minutes.
pub const HOUR: Unit = unit(
    "hour",
    "hour",
    Some("h"),
    3_600,
    1,
    Family::Civil,
    "BIPM, units accepted for use with the SI",
);
/// The nominal day of 86 400 seconds.
pub const DAY: Unit = unit(
    "day",
    "day",
    Some("d"),
    86_400,
    1,
    Family::Civil,
    "BIPM, units accepted for use with the SI",
);
/// Seven nominal days.
pub const WEEK: Unit = unit(
    "week",
    "week",
    Some("wk"),
    604_800,
    1,
    Family::Civil,
    "ISO 8601-1:2019",
);
/// Fourteen nominal days.
pub const FORTNIGHT: Unit = unit(
    "fortnight",
    "fortnight",
    None,
    1_209_600,
    1,
    Family::Civil,
    "Oxford English Dictionary; traditional English usage",
);
/// 365 nominal days.
pub const COMMON_YEAR: Unit = unit(
    "common-year",
    "common year",
    None,
    31_536_000,
    1,
    Family::Civil,
    "Gregorian calendar, non-leap year",
);
/// 366 nominal days.
pub const LEAP_YEAR: Unit = unit(
    "leap-year",
    "leap year",
    None,
    31_622_400,
    1,
    Family::Civil,
    "Gregorian calendar, leap year",
);
/// 365.25 days exactly: the year astronomy counts in.
pub const JULIAN_YEAR: Unit = unit(
    "julian-year",
    "Julian year",
    Some("a"),
    31_557_600,
    1,
    Family::Civil,
    "IAU (1976) System of Astronomical Constants",
);
/// 36 525 days exactly, the interval the Meeus series are written in.
pub const JULIAN_CENTURY: Unit = unit(
    "julian-century",
    "Julian century",
    None,
    3_155_760_000,
    1,
    Family::Civil,
    "IAU (1976) System of Astronomical Constants",
);
/// 365 250 days exactly.
pub const JULIAN_MILLENNIUM: Unit = unit(
    "julian-millennium",
    "Julian millennium",
    None,
    31_557_600_000,
    1,
    Family::Civil,
    "IAU (1976) System of Astronomical Constants",
);
/// 365.2425 days: the mean Gregorian year, 400 years being 146 097 days.
pub const MEAN_GREGORIAN_YEAR: Unit = unit(
    "mean-gregorian-year",
    "mean Gregorian year",
    None,
    31_556_952,
    1,
    Family::Civil,
    "Gregorian calendar; CLDR relative-time means",
);
/// A twelfth of [`MEAN_GREGORIAN_YEAR`], which divides exactly.
pub const MEAN_GREGORIAN_MONTH: Unit = unit(
    "mean-gregorian-month",
    "mean Gregorian month",
    None,
    2_629_746,
    1,
    Family::Civil,
    "Gregorian calendar; CLDR relative-time means",
);
/// A quarter of [`MEAN_GREGORIAN_YEAR`], which divides exactly.
pub const MEAN_GREGORIAN_QUARTER: Unit = unit(
    "mean-gregorian-quarter",
    "mean Gregorian quarter",
    None,
    7_889_238,
    1,
    Family::Civil,
    "Gregorian calendar; CLDR relative-time means",
);

// --- Horological ------------------------------------------------------------

/// חלק, a 1080th of an hour: 3⅓ seconds exactly.
///
/// The unit the Hebrew calendar's molad is stated in, chosen because 1080
/// is divisible by 2, 3, 4, 5, 6, 8, 9 and 10, so the common fractions of
/// an hour are all whole numbers of chalakim.
pub const HELEK: Unit = unit(
    "helek",
    "helek",
    None,
    10,
    3,
    Family::Horological,
    "Mishnah; Maimonides, Hilchot Kiddush HaChodesh",
);
/// רגע, a 76th of a [`HELEK`].
pub const REGA: Unit = unit(
    "rega",
    "rega",
    None,
    5,
    114,
    Family::Horological,
    "Maimonides, Hilchot Kiddush HaChodesh 10:1",
);
/// A 40th of an hour: 90 seconds, the medieval European moment.
pub const MOMENT: Unit = unit(
    "moment",
    "moment",
    None,
    90,
    1,
    Family::Horological,
    "Bartholomeus Anglicus, De proprietatibus rerum (c. 1240)",
);
/// 刻 in the 百刻 system: a hundredth of a day, 864 seconds.
pub const KE_HUNDRED: Unit = unit(
    "ke-hundred",
    "ke (100 per day)",
    None,
    864,
    1,
    Family::Horological,
    "Chinese 百刻 system, in use to 1645",
);
/// 刻 after the 時憲曆 reform: a 96th of a day, 900 seconds.
pub const KE_NINETY_SIX: Unit = unit(
    "ke-ninety-six",
    "ke (96 per day)",
    None,
    900,
    1,
    Family::Horological,
    "時憲曆 (1645); standard in China, Japan and Korea thereafter",
);
/// 時辰, a twelfth of a day: the double hour of the East Asian clock.
pub const SHICHEN: Unit = unit(
    "shichen",
    "shichen",
    None,
    7_200,
    1,
    Family::Horological,
    "Chinese duodecimal day division",
);
/// घटी, a 60th of a day: 24 minutes.
pub const GHATI: Unit = unit(
    "ghati",
    "ghati",
    None,
    1_440,
    1,
    Family::Horological,
    "Surya Siddhanta 1.12",
);
/// विघटी, a 60th of a [`GHATI`]: 24 seconds.
pub const VIGHATI: Unit = unit(
    "vighati",
    "vighati",
    None,
    24,
    1,
    Family::Horological,
    "Surya Siddhanta 1.11",
);
/// प्राण, a sixth of a [`VIGHATI`]: four seconds, one respiration.
pub const PRANA: Unit = unit(
    "prana",
    "prana",
    None,
    4,
    1,
    Family::Horological,
    "Surya Siddhanta 1.11",
);
/// मुहूर्त, a thirtieth of a day: 48 minutes.
pub const MUHURTA: Unit = unit(
    "muhurta",
    "muhurta",
    None,
    2_880,
    1,
    Family::Horological,
    "Vedanga Jyotisha; Surya Siddhanta",
);

// --- Decimal ----------------------------------------------------------------

/// A tenth of a day, under the French Republican decree of 4 Frimaire An II.
pub const DECIMAL_HOUR: Unit = unit(
    "decimal-hour",
    "decimal hour",
    None,
    8_640,
    1,
    Family::Decimal,
    "Décret du 4 frimaire an II (24 November 1793)",
);
/// A hundredth of a [`DECIMAL_HOUR`]: 86.4 seconds.
pub const DECIMAL_MINUTE: Unit = unit(
    "decimal-minute",
    "decimal minute",
    None,
    432,
    5,
    Family::Decimal,
    "Décret du 4 frimaire an II (24 November 1793)",
);
/// A hundredth of a [`DECIMAL_MINUTE`]: 0.864 seconds.
pub const DECIMAL_SECOND: Unit = unit(
    "decimal-second",
    "decimal second",
    None,
    108,
    125,
    Family::Decimal,
    "Décret du 4 frimaire an II (24 November 1793)",
);
/// Swatch Internet Time's `.beat`: a thousandth of a day.
///
/// Exactly the length of a [`DECIMAL_MINUTE`], two centuries apart, because
/// both are a day divided by a power of ten.
pub const BEAT: Unit = unit(
    "beat",
    "Swatch .beat",
    Some(".beat"),
    432,
    5,
    Family::Decimal,
    "Swatch Internet Time (1998)",
);

// --- Hexadecimal ------------------------------------------------------------

/// A sixteenth of a day: 90 minutes.
pub const HEX_HOUR: Unit = unit(
    "hex-hour",
    "hexadecimal hour",
    None,
    5_400,
    1,
    Family::Hexadecimal,
    "John W. Nystrom, Project of a New System of Arithmetic (1862)",
);
/// A 256th of a day: 337.5 seconds.
pub const HEX_MAXIME: Unit = unit(
    "hex-maxime",
    "hexadecimal maxime",
    None,
    675,
    2,
    Family::Hexadecimal,
    "John W. Nystrom, Project of a New System of Arithmetic (1862)",
);
/// A 4096th of a day: 21.09375 seconds.
pub const HEX_MINUTE: Unit = unit(
    "hex-minute",
    "hexadecimal minute",
    None,
    675,
    32,
    Family::Hexadecimal,
    "John W. Nystrom, Project of a New System of Arithmetic (1862)",
);
/// A 65 536th of a day: 1.318359375 seconds.
pub const HEX_SECOND: Unit = unit(
    "hex-second",
    "hexadecimal second",
    None,
    675,
    512,
    Family::Hexadecimal,
    "John W. Nystrom, Project of a New System of Arithmetic (1862)",
);

// --- Media and computing ----------------------------------------------------

/// A 705 600 000th of a second.
///
/// Chosen so that a single frame at 24, 25, 30, 48, 50, 60, 90, 100 or 120
/// fps, a single sample at 8, 16, 22.05, 24, 32, 44.1, 48, 88.2, 96 or 192
/// kHz, and the NTSC 1000/1001 pull-down of any of them, are each a whole
/// number of flicks. `crate::media` tests exactly that.
pub const FLICK: Unit = unit(
    "flick",
    "flick",
    None,
    1,
    705_600_000,
    Family::Media,
    "Facebook, github.com/facebookarchive/Flicks (2017)",
);
/// Ten nanoseconds, the unit neutron transport is timed in.
pub const SHAKE: Unit = unit(
    "shake",
    "shake",
    None,
    1,
    100_000_000,
    Family::Scientific,
    "Manhattan Project nuclear physics usage",
);
/// 10⁻¹³ s, the sedimentation coefficient unit.
pub const SVEDBERG: Unit = unit(
    "svedberg",
    "svedberg",
    Some("S"),
    1,
    10_000_000_000_000,
    Family::Scientific,
    "Theodor Svedberg; IUPAC Gold Book",
);
/// The time light takes to travel one centimetre in vacuum.
///
/// Exact, because the metre is defined from a fixed speed of light:
/// 1/29 979 245 800 s.
pub const JIFFY_LIGHT_CENTIMETRE: Unit = unit(
    "jiffy-light-centimetre",
    "jiffy (light-centimetre)",
    None,
    1,
    29_979_245_800,
    Family::Scientific,
    "Gilbert Lewis, after the defined value of c (BIPM, 1983)",
);
/// A jiffy as one cycle of 60 Hz mains: 1/60 s.
pub const JIFFY_MAINS_60: Unit = unit(
    "jiffy-mains-60",
    "jiffy (60 Hz mains)",
    None,
    1,
    60,
    Family::Media,
    "North American electrical engineering usage",
);
/// A jiffy as one cycle of 50 Hz mains: 1/50 s.
pub const JIFFY_MAINS_50: Unit = unit(
    "jiffy-mains-50",
    "jiffy (50 Hz mains)",
    None,
    1,
    50,
    Family::Media,
    "European electrical engineering usage",
);
/// 100 nanoseconds: the tick of Windows `FILETIME` and .NET `DateTime`.
pub const TICK_FILETIME: Unit = unit(
    "tick-filetime",
    "FILETIME tick",
    None,
    1,
    10_000_000,
    Family::Media,
    "Microsoft Windows FILETIME; .NET DateTime.Ticks",
);
/// 1.2096 seconds, the delay VMS measures password retries in.
pub const MICROFORTNIGHT: Unit = unit(
    "microfortnight",
    "microfortnight",
    None,
    756,
    625,
    Family::Media,
    "DEC VMS system parameter documentation",
);

/// Every unit this crate defines, shortest first.
///
/// The order is by length, so a humaniser can scan it for the largest unit
/// that fits. It is a slice rather than an array so that adding a unit is
/// not a breaking change for anyone who matched on its length.
pub const ALL: &[Unit] = &[
    QUECTOSECOND,
    RONTOSECOND,
    YOCTOSECOND,
    ZEPTOSECOND,
    ATTOSECOND,
    FEMTOSECOND,
    SVEDBERG,
    PICOSECOND,
    JIFFY_LIGHT_CENTIMETRE,
    NANOSECOND,
    FLICK,
    SHAKE,
    TICK_FILETIME,
    MICROSECOND,
    MILLISECOND,
    JIFFY_MAINS_60,
    JIFFY_MAINS_50,
    REGA,
    DECIMAL_SECOND,
    SECOND,
    MICROFORTNIGHT,
    HEX_SECOND,
    HELEK,
    PRANA,
    HEX_MINUTE,
    VIGHATI,
    MINUTE,
    DECIMAL_MINUTE,
    BEAT,
    MOMENT,
    HEX_MAXIME,
    KE_HUNDRED,
    KE_NINETY_SIX,
    KILOSECOND,
    GHATI,
    MUHURTA,
    HOUR,
    HEX_HOUR,
    SHICHEN,
    DECIMAL_HOUR,
    DAY,
    WEEK,
    MEGASECOND,
    FORTNIGHT,
    MEAN_GREGORIAN_MONTH,
    MEAN_GREGORIAN_QUARTER,
    COMMON_YEAR,
    MEAN_GREGORIAN_YEAR,
    JULIAN_YEAR,
    LEAP_YEAR,
    GIGASECOND,
    JULIAN_CENTURY,
    JULIAN_MILLENNIUM,
];

/// The unit with this id, if the table has one.
#[must_use]
pub fn by_id(id: &str) -> Option<Unit> {
    ALL.iter().copied().find(|unit| unit.id == id)
}
