//! What a Tibetan almanac prints beside the date: the five components
//! (*lnga-bsdus*) and the columns after them, the planets, the names of
//! the years, the Bhutanese weekday and winter solstice, the Mongolian
//! names of the months and colours of the years, and where a festival
//! falls when its date is skipped or repeated.
//!
//! The system is written up in the repository in
//! `docs/systems/tibetan-almanac.md`, with the days of Henning's computed
//! almanacs this module was checked against; the date itself is in
//! `docs/systems/tibetan-phugpa.md` and `docs/systems/tibetan-variants.md`.
//! This page states the code's own facts.
//!
//! # The day's columns
//!
//! The calendar is "the five components": the day of the week, the lunar
//! day, the lunar mansion, the *yoga* and the *karaṇa* (Janson, Section
//! 10). [`almanac_day`] gives them for a calendar day under a
//! [`TibetanCalendar`], with the columns an almanac prints after them:
//!
//! * the true weekday, the end of the lunar day in days after Saturday's
//!   dawn, item (i); on the first of two days with one number the almanac
//!   prints the end of the calendar day instead, `x;60,0`, and so does
//!   [`AlmanacDay::lunar_day_end`] by being `None`;
//! * the Moon's longitude at daybreak, item (iii), the lunar mansion its
//!   whole part, item (iv), and on that first of two days the longitude at the end
//!   of the lunar day less one mansion, as the almanac has it;
//! * the true Sun, item (v), the *yoga* longitude, the sum of the two, and
//!   the *yoga* its whole part, items (vi) and (vii);
//! * the *karaṇa* in effect at daybreak, item (viii), each lunar day
//!   halved into two equal parts of the elongation;
//! * the mean Sun, item (ix), in signs, degrees and minutes.
//!
//! The Tsurphu almanac's Sun is the *karaṇa* one (Janson, Appendix A.2), so
//! under [`tibetan::TIBETAN_TSURPHU`] and [`tibetan::TIBETAN_TSURPHU_KARANA`]
//! the Sun, the Moon, the *yoga*
//! and the mean Sun are the *karaṇa* ones.
//!
//! # Names
//!
//! The mansions, *yogas* and *karaṇas* are named in Sanskrit and Tibetan as
//! Henning's Tsurphu almanacs print them, "Shatabhishaj/mon gru", in his
//! ASCII transliteration and Wylie: [`MANSIONS`], [`YOGAS`], [`KARANAS`].
//! Henning's *Kālacakra and the Tibetan Calendar*, whose Appendix I lists
//! them, was not read. The twenty-seventh mansion's place is Abhijit's, *gro
//! zhin*, where Indian lists have Śravaṇa: Henning's almanacs print it so.
//!
//! # The attributes of a day
//!
//! Janson's Appendix E gives a lunar day an animal, an element, a trigram
//! and a number ([`lunar_day_attributes`]), and a calendar day a trigram
//! ([`day_trigram`]) and a number ([`janson_day_number`]); the Indian
//! system gives it the elements of its weekday and mansion
//! ([`element_pair`]). Henning's almanacs print the pair, the lunar day's
//! animal, trigram and number, and beside the solar day a Chinese mansion
//! ([`chinese_mansion`]) and a number that runs the other way from
//! Janson's ([`henning_almanac_day_number`]).
//!
//! # Sources
//!
//! * `janson2014`: Svante Janson, "Tibetan calendar mathematics",
//!   arXiv:1401.6285, read from its TeX source 2026-09-29: Sections 4, 9,
//!   10 and 11, Appendices A.2–A.5, B, D and E; Section 9's table of the
//!   weekdays and Appendix E again the same day, in ar5iv's HTML rendering.
//! * `kalacakra-org`: Edward Henning's computed Phugpa, Tsurphu and
//!   Bhutanese almanacs (`tdata/pl_*.txt`, `ts_*.txt`, `bh_*.txt`), "Open
//!   source Tibetan calendar software", "Open source Tsurphu calendar
//!   software", "Epoch data", "Bhutan calendars" and "The Bhutanese
//!   calendar", read 2026-09-29.
//! * `gantumur-mongolian-calendar`: Tsogtgerel Gantumur's Mongolian
//!   calendar pages, whose program writes the months' Mongolian names and
//!   the colours, read 2026-09-29 — a computed calendar, cited as a
//!   secondary source.

use hc_calendar::{CalendarError, CalendarResult, Month, Rd};
use hc_calendars_lunar::tibetan::TibetanDate;

use hc_calendars_lunar::tibetan::{self, Ratio, TibetanCalendar, table, true_sun};

/// How an almanac gives the Moon at daybreak on the first of two days with
/// one number, a day in which no lunar day ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtraDayMoon {
    /// The Moon at the end of the lunar day less one mansion, as Janson
    /// reads the Men-Tsee-Khang almanac of 2013 (Section 10) and as
    /// Henning's Phugpa and Bhutanese almanacs print it.
    LessOneMansion,
    /// The Moon at the end of the lunar day before, moved on one mansion a
    /// day to daybreak, as Henning's Tsurphu almanacs print it: this
    /// library's reading of his pages, which state no rule.
    FromThePreviousLunarDay,
}

/// The extra-day Moon of a version's almanac: the Tsurphu's, whose months
/// are the *karaṇa* ones, moves the Moon on from the lunar day before.
#[must_use]
pub const fn extra_day_moon(calendar: &TibetanCalendar) -> ExtraDayMoon {
    match calendar.karana_months() {
        Some(_) => ExtraDayMoon::FromThePreviousLunarDay,
        None => ExtraDayMoon::LessOneMansion,
    }
}

/// The mean Sun a version's almanac prints and computes its true Sun from:
/// the *karaṇa* one in the Tsurphu (Janson, Appendix A.2), the version's
/// own elsewhere, in revolutions.
#[must_use]
pub const fn printed_mean_sun(calendar: &TibetanCalendar, day: i64, n: i64) -> Ratio {
    match calendar.karana_mean_sun(day, n) {
        Some(karana) => karana,
        None => calendar.mean_sun(day, n),
    }
}

/// The quantities of one lunar day at its end (Janson, Section 7), as an
/// almanac prints them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LunarDay {
    /// The true month count `n`.
    pub month_count: i64,
    /// The lunar day, 1 to 30.
    pub day: u8,
    /// The true date, whose integer part is the calendar day it ends in.
    pub true_date: Ratio,
    /// The true solar longitude the almanac prints, in revolutions.
    pub true_sun: Ratio,
    /// The Moon's longitude at the end of the lunar day, the true Sun and
    /// `d/30`, in revolutions (Janson, Section 10, item (ii)).
    pub moon: Ratio,
}

/// Lunar day `day` of true month `n` of `calendar`, a skipped one among
/// them, for which the almanac prints the true weekday and the Sun.
#[must_use]
pub fn lunar_day(calendar: &TibetanCalendar, day: u8, n: i64) -> LunarDay {
    let d = i64::from(day);
    let sun = true_sun(printed_mean_sun(calendar, d, n));
    LunarDay {
        month_count: n,
        day,
        true_date: calendar.true_date(d, n),
        true_sun: sun,
        moon: sun.add(Ratio::new(i128::from(day), 30)).frac(),
    }
}

/// The 27 lunar mansions from the first point of the zodiac, in Sanskrit and
/// in Tibetan, as Henning's Tsurphu almanacs print them.
pub const MANSIONS: [(&str, &str); 27] = [
    ("Ashvini", "tha skar"),
    ("Bharani", "bra nye"),
    ("Krittika", "smin drug"),
    ("Rohini", "snar ma"),
    ("Mrigashiras", "mgo"),
    ("Ardra", "lag"),
    ("Punarvasu", "nabs so"),
    ("Pushya", "rgyal"),
    ("Ashlesha", "skag"),
    ("Magha", "mchu"),
    ("Purvaphalguni", "gre"),
    ("Uttaraphalguni", "dbo"),
    ("Hasta", "me bzhi"),
    ("Citra", "nag pa"),
    ("Svati", "sa ri"),
    ("Vishakha", "sa ga"),
    ("Anuradha", "lha mtshams"),
    ("Jyeshtha", "snron"),
    ("Mula", "snrubs"),
    ("Purvashadha", "chu stod"),
    ("Uttarashadha", "chu smad"),
    ("Abhijit", "gro zhin"),
    ("Dhanishtha", "mon gre"),
    ("Shatabhishaj", "mon gru"),
    ("Purvabhadrapada", "khrums stod"),
    ("Uttarabhadrapada", "khrums smad"),
    ("Revati", "nam gru"),
];

/// The 27 *yogas*, in Sanskrit and in Tibetan, as Henning's Tsurphu almanacs
/// print them.
pub const YOGAS: [(&str, &str); 27] = [
    ("Vishkambha", "rnam sel"),
    ("Priti", "mdza' bo"),
    ("Ayushmat", "tshe dang ldan pa"),
    ("Saubhagya", "skal bzang"),
    ("Shobhana", "dge byed"),
    ("Atiganda", "shin tu 'grams"),
    ("Sukarman", "las bzang"),
    ("Dhriti", "'dzin byed"),
    ("Shula", "zug rngu"),
    ("Ganda", "'grams"),
    ("Vriddhi", "'phel"),
    ("Dhruva", "brtan pa"),
    ("Vyaghata", "yongs bsnun"),
    ("Harshana", "dga' ba"),
    ("Vajra", "rdo rje"),
    ("Siddhi", "dngos grub"),
    ("Vyatipata", "phan tshun"),
    ("Variyas", "mchog can"),
    ("Parigha", "yongs 'joms"),
    ("Shiva", "zhi ba"),
    ("Siddha", "grub pa"),
    ("Sadhya", "bsgrub bya"),
    ("Shubha", "dge ba"),
    ("Shukla", "dkar po"),
    ("Brahman", "tshangs pa"),
    ("Indra", "dbang po"),
    ("Vaidhriti", "'khon 'dzin"),
];

/// The eleven *karaṇas*: the seven changing ones in their cycle, then the
/// four fixed ones of half-days 58, 59, 60 and 1, in Sanskrit and in
/// Tibetan as Henning's Tsurphu almanacs print them (his Phugpa almanacs
/// write the seventh *vishti*).
pub const KARANAS: [(&str, &str); 11] = [
    ("Vava", "gdab pa"),
    ("Valava", "byis pa"),
    ("Kaulava", "rigs can"),
    ("Taitila", "til rdung"),
    ("Gara", "khyim skyes"),
    ("Vanija", "tshong ba"),
    ("Vishti", "viSTi"),
    ("Shakuni", "bkra shis"),
    ("Catushpada", "rkang bzhi"),
    ("Naga", "klu"),
    ("Kintughna", "mi sdug pa"),
];

/// The *karaṇa* of half-day `half`, 1 to 60, of a month, as an index into
/// [`KARANAS`]: half-days 1, 58, 59 and 60 have the fixed ones, and the rest
/// the changing ones in turn, `(H − 1) amod 7` (Janson, Section 10, item (viii)).
#[must_use]
pub const fn karana_of_half_day(half: u8) -> u8 {
    match half {
        1 => 10,
        58 => 7,
        59 => 8,
        60 => 9,
        _ => (half - 2) % 7,
    }
}

/// The weekdays, from Saturday, 0, as the true weekday counts them, in
/// English and in Tibetan (Janson, Section 9, the table of the days of the week).
pub const WEEKDAYS: [(&str, &str); 7] = [
    ("Saturday", "spen pa"),
    ("Sunday", "nyi ma"),
    ("Monday", "zla ba"),
    ("Tuesday", "mig dmar"),
    ("Wednesday", "lhag pa"),
    ("Thursday", "phur bu"),
    ("Friday", "pa sangs"),
];

/// A calendar day's columns in the almanac of a [`TibetanCalendar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlmanacDay {
    /// The Tibetan date.
    pub date: TibetanDate,
    /// The true month count.
    pub month_count: i64,
    /// The weekday, 0 for Saturday to 6 for Friday, `(JD + 2) mod 7`
    /// (Janson, Section 9): the world's, which the Bhutanese almanac names one
    /// ahead ([`bhutanese_weekday`]).
    pub weekday: u8,
    /// The true weekday, the end of the day's lunar day in days from
    /// Saturday's dawn, `(true_date + 2) mod 7`; `None` on the first of two
    /// days with one number, where the almanac prints the end of the
    /// calendar day.
    pub lunar_day_end: Option<Ratio>,
    /// The Moon's longitude at daybreak, in lunar mansions, `[0, 27)`.
    pub moon: Ratio,
    /// The true solar longitude, in lunar mansions, `[0, 27)`.
    pub sun: Ratio,
    /// The *yoga* longitude, the Moon's and the Sun's, in mansions mod 27.
    pub yoga_longitude: Ratio,
    /// The mean Sun, in signs of the zodiac, `[0, 12)`.
    pub mean_sun: Ratio,
    /// The lunar mansion, an index into [`MANSIONS`].
    pub mansion: u8,
    /// The *yoga*, an index into [`YOGAS`].
    pub yoga: u8,
    /// The *karaṇa* in effect at daybreak, an index into [`KARANAS`].
    pub karana: u8,
    /// The half-day of the month in effect at daybreak, 1 to 60.
    pub half_day: u8,
}

fn modulo(value: Ratio, modulus: i128) -> Ratio {
    value.sub(Ratio::int(value.floor().div_euclid(modulus) * modulus))
}

impl AlmanacDay {
    /// The true Sun in signs of the zodiac, `[0, 12)`: the column the
    /// Tsurphu almanac prints after the *yoga* where the Phugpa prints the
    /// mean Sun (Henning, "Open source Tsurphu calendar software").
    #[must_use]
    pub fn sun_in_signs(&self) -> Ratio {
        self.sun.mul(Ratio::new(12, 27))
    }
}

/// The almanac's columns for the calendar day `rd` (Janson, Section 10).
///
/// # Errors
///
/// As [`TibetanCalendar::locate`], outside the years converted.
pub fn almanac_day(calendar: &TibetanCalendar, rd: Rd) -> CalendarResult<AlmanacDay> {
    let (date, n) = calendar.locate(rd)?;
    let jdn = rd.to_julian_day_number();
    let lunar = lunar_day(calendar, date.day, n);
    let day = i64::from(date.day);
    let moon_revolutions = match (date.leap_day, extra_day_moon(calendar)) {
        (false, _) => lunar
            .moon
            .sub(lunar.true_date.frac().mul(Ratio::new(1, 27)))
            .frac(),
        (true, ExtraDayMoon::LessOneMansion) => lunar.moon.sub(Ratio::new(1, 27)).frac(),
        (true, ExtraDayMoon::FromThePreviousLunarDay) => {
            let (previous_day, previous_month) = if day == 1 { (30, n - 1) } else { (day - 1, n) };
            let previous = lunar_day(calendar, previous_day as u8, previous_month);
            previous
                .moon
                .add(
                    Ratio::int(i128::from(jdn))
                        .sub(previous.true_date)
                        .mul(Ratio::new(1, 27)),
                )
                .frac()
        }
    };
    let moon = moon_revolutions.scale(27);
    let sun = lunar.true_sun.scale(27);
    let yoga_longitude = moon_revolutions.add(lunar.true_sun).frac().scale(27);
    // The half-day at daybreak: the elongation the almanac's Moon and Sun
    // give, in sixtieths of a revolution, each lunar day halved into two
    // equal parts of 6° (Janson, Section 10, item (viii), "Henning divides each lunar day
    // into two halves of equal lengths"). Henning's almanacs bear out the
    // elongation, not the midpoint in time, as the measure.
    let elongation = moon_revolutions.sub(lunar.true_sun).frac();
    let half_day = elongation.scale(60).floor() as u8 + 1;
    let printed_mean_sun = printed_mean_sun(calendar, day, n);
    Ok(AlmanacDay {
        date,
        month_count: n,
        weekday: (jdn + 2).rem_euclid(7) as u8,
        lunar_day_end: if date.leap_day {
            None
        } else {
            Some(modulo(lunar.true_date.add(Ratio::int(2)), 7))
        },
        moon,
        sun,
        yoga_longitude,
        mean_sun: printed_mean_sun.scale(12),
        mansion: moon.floor() as u8,
        yoga: yoga_longitude.floor() as u8,
        karana: karana_of_half_day(half_day),
        half_day,
    })
}

// --- The planets ------------------------------------------------------------

/// Henning's epoch of the Phugpa planets, Friday 1 April 1927, JD 2 424 972
/// (Janson, Appendix D; Henning, "Epoch data", the *Essence of the Kalkī*).
pub const PLANET_EPOCH_JDN: i64 = 2_424_972;

/// The five planets the almanac places.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Planet {
    /// Mercury, an inner, "peaceful" planet.
    Mercury,
    /// Venus, an inner planet.
    Venus,
    /// Mars, an outer, "wrathful" planet.
    Mars,
    /// Jupiter, an outer planet.
    Jupiter,
    /// Saturn, an outer planet.
    Saturn,
}

impl Planet {
    /// The five, in the order of Janson's tables.
    pub const ALL: [Self; 5] = [
        Self::Mercury,
        Self::Venus,
        Self::Mars,
        Self::Jupiter,
        Self::Saturn,
    ];

    /// The planet's constants: the multiplier of the general day, the
    /// modulus `R`, the epoch value `pd0`, the birth-sign, the equation
    /// table and the final correction table (Janson, the tables of Appendix D).
    /// The planet's identifier, its English name in lower case:
    /// `mercury`, `venus`, `mars`, `jupiter` or `saturn`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Mercury => "mercury",
            Self::Venus => "venus",
            Self::Mars => "mars",
            Self::Jupiter => "jupiter",
            Self::Saturn => "saturn",
        }
    }

    const fn constants(self) -> PlanetConstants {
        match self {
            Self::Mercury => PlanetConstants {
                multiplier: 100,
                modulus: 8_797,
                epoch: 4_639,
                birth_sign: Ratio::new(11, 18),
                equation: [0, 10, 17, 20],
                correction: [0, 16, 32, 47, 61, 74, 85, 92, 97, 97, 93, 82, 62, 34],
                inner: true,
            },
            Self::Venus => PlanetConstants {
                multiplier: 10,
                modulus: 2_247,
                epoch: 301,
                birth_sign: Ratio::new(2, 9),
                equation: [0, 5, 9, 10],
                correction: [
                    0, 25, 50, 75, 99, 123, 145, 167, 185, 200, 208, 202, 172, 83,
                ],
                inner: true,
            },
            Self::Mars => PlanetConstants {
                multiplier: 1,
                modulus: 687,
                epoch: 157,
                birth_sign: Ratio::new(19, 54),
                equation: [0, 25, 43, 50],
                correction: [
                    0, 24, 47, 70, 93, 114, 135, 153, 168, 179, 182, 171, 133, 53,
                ],
                inner: false,
            },
            Self::Jupiter => PlanetConstants {
                multiplier: 1,
                modulus: 4_332,
                epoch: 3_964,
                birth_sign: Ratio::new(4, 9),
                equation: [0, 11, 20, 23],
                correction: [0, 10, 20, 29, 37, 43, 49, 51, 52, 49, 43, 34, 23, 7],
                inner: false,
            },
            Self::Saturn => PlanetConstants {
                multiplier: 1,
                modulus: 10_766,
                epoch: 6_286,
                birth_sign: Ratio::new(2, 3),
                equation: [0, 22, 37, 43],
                correction: [0, 6, 11, 16, 20, 24, 26, 28, 28, 26, 22, 17, 11, 3],
                inner: false,
            },
        }
    }
}

struct PlanetConstants {
    multiplier: i64,
    modulus: i64,
    epoch: i64,
    birth_sign: Ratio,
    equation: [i128; 4],
    correction: [i128; 14],
    inner: bool,
}

/// A planet's place at the end of a calendar day, the longitudes in lunar
/// mansions, `[0, 27)` (Janson, Appendix D).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanetPlace {
    /// The particular day (*sgos zhag*), the day's place in the planet's
    /// heliocentric cycle.
    pub particular_day: i64,
    /// The mean heliocentric longitude.
    pub mean_heliocentric: Ratio,
    /// The true slow longitude (*dal dag*).
    pub true_slow: Ratio,
    /// The fast, geocentric, longitude (*myur ba*).
    pub fast: Ratio,
}

/// The general day (*spyi zhag*) of the Phugpa planets: days from Henning's
/// epoch of 1927.
#[must_use]
pub const fn general_day(rd: Rd) -> i64 {
    rd.to_julian_day_number() - PLANET_EPOCH_JDN
}

/// The mean solar longitude at the end of general day `day`, in
/// revolutions, `s1' · day + s0'` with `s1' = 18 382/6 714 405` and
/// `s0' = 1 − 458 772/6 714 405` (Janson, Appendix D).
#[must_use]
pub const fn mean_solar_longitude(day: i64) -> Ratio {
    Ratio::int(day as i128)
        .mul(Ratio::new(18_382, 6_714_405))
        .add(Ratio::new(6_714_405 - 458_772, 6_714_405))
        .frac()
}

/// The final correction's table, extended by `tab(27 − i) = −tab(i)` and a
/// period of 27, linearly interpolated.
fn correction(values: &[i128; 14], x: Ratio) -> Ratio {
    let x = Ratio::new(
        x.numerator().rem_euclid(x.denominator() * 27),
        x.denominator(),
    );
    let at = |i: i128| -> Ratio {
        let i = i.rem_euclid(27);
        if i <= 13 {
            Ratio::int(values[i as usize])
        } else {
            Ratio::int(-values[(27 - i) as usize])
        }
    };
    let index = x.floor();
    let low = at(index);
    low.add(x.frac().mul(at(index + 1).sub(low)))
}

/// Where the Phugpa almanac places `planet` at the end of calendar day `rd`
/// (Janson, Appendix D, with Henning's epoch values of 1927).
#[must_use]
pub fn planet_place(planet: Planet, rd: Rd) -> PlanetPlace {
    let constants = planet.constants();
    let day = general_day(rd);
    let particular_day =
        (constants.multiplier * day + constants.epoch).rem_euclid(constants.modulus);
    let heliocentric = Ratio::new(i128::from(particular_day), i128::from(constants.modulus));
    let solar = mean_solar_longitude(day);
    let (slow, step) = if constants.inner {
        (solar, heliocentric)
    } else {
        (heliocentric, solar)
    };
    let anomaly = slow.sub(constants.birth_sign).frac();
    let equation = table(&constants.equation, 3, anomaly.scale(12));
    let true_slow = slow.sub(equation.mul(Ratio::new(1, 27 * 60))).frac();
    let difference = step.sub(true_slow).frac();
    let correction = correction(&constants.correction, difference.scale(27));
    let fast = true_slow.add(correction.mul(Ratio::new(1, 27 * 60))).frac();
    PlanetPlace {
        particular_day,
        mean_heliocentric: heliocentric.scale(27),
        true_slow: true_slow.scale(27),
        fast: fast.scale(27),
    }
}

/// The longitude of the Head of Rāhu, the Moon's ascending node, at the end
/// of lunar day `day` of the Phugpa true month `n` counted from 806, in
/// lunar mansions: `−x/6 900` with `x = 30 (n + 187) + D` for `n` counted
/// from Henning's epoch of 1927, 13 866 months later (Janson, Appendix D). The Tail is half a circle on.
///
/// # Errors
///
/// [`CalendarError::UnsupportedField`] for a version other than the
/// Phugpa's, from the epoch of 806, whose epoch value is the only one read.
pub fn rahu_head(calendar: &TibetanCalendar, day: u8, n: i64) -> CalendarResult<Ratio> {
    if calendar.epoch_year() != 806 {
        return Err(CalendarError::UnsupportedField("rahu"));
    }
    // Henning's epoch of 1927 is 13 866 true months after 806 (Janson,
    // Section 7).
    let months = 13_866;
    let x = 30 * (n - months + 187) + i64::from(day);
    Ok(Ratio::new(-i128::from(x), 6_900).frac().scale(27))
}

// --- Years ------------------------------------------------------------------

/// The sixty names of the Indian (*rab byung*) cycle, from Prabhava, in
/// Tibetan and in Sanskrit as Janson's table of the cycle in Appendix B gives them, taken from
/// Henning; the transliterations are Janson's, among them *pramadi* for the
/// 4th, 13th and 47th years as he prints them.
pub const RAB_BYUNG_NAMES: [(&str, &str); 60] = [
    ("rab byung", "prabhava"),
    ("rnam byung", "vibhava"),
    ("dkar po", "suklata"),
    ("rab myos", "pramadi"),
    ("skyes bdag", "prajapati"),
    ("anggi ra", "ankira"),
    ("dpal gdong", "srimukha"),
    ("dngos po", "bhava"),
    ("na tshod ldan", "yuvika"),
    ("'dzin byed", "dhritu"),
    ("dbang phyug", "isvara"),
    ("'bru mang po", "vahudhvanya"),
    ("myos ldan", "pramadi"),
    ("rnam gnon", "vikrama"),
    ("khyu mchog", "brisabha"),
    ("sna tshogs", "citra"),
    ("nyi ma", "bhanu"),
    ("nyi sgrol byed", "bhanutara"),
    ("sa skyong", "virthapa"),
    ("mi zad", "aksaya"),
    ("thams cad 'dul", "sarvajit"),
    ("kun 'dzin", "sarvadhari"),
    ("'gal ba", "virodhi"),
    ("rnam 'gyur", "vikrita"),
    ("bong bu", "khara"),
    ("dga' ba", "nanda"),
    ("rnam rgyal", "vijaya"),
    ("rgyal ba", "jaya"),
    ("myos byed", "mada"),
    ("gdong ngan", "durmukha"),
    ("gser 'phyang", "hemalambha"),
    ("rnam 'phyang", "vilambhi"),
    ("sgyur byed", "vikari"),
    ("kun ldan", "sarvavati"),
    ("'phar ba", "slava"),
    ("dge byed", "subhakrita"),
    ("mdzes byed", "sobhana"),
    ("khro mo", "krodhi"),
    ("sna tshogs dbyig", "visvabandhu"),
    ("zil gnon", "parabhava"),
    ("spre'u", "pravamga"),
    ("phur bu", "kilaka"),
    ("zhi ba", "saumya"),
    ("thun mong", "sadharana"),
    ("'gal byed", "virobhakrita"),
    ("yongs 'dzin", "paradhari"),
    ("bag med", "pramadi"),
    ("kun dga'", "ananda"),
    ("srin bu", "raksasa"),
    ("me", "anala"),
    ("dmar ser can", "vingala"),
    ("dus kyi pho nya", "kaladuti"),
    ("don grub", "siddhartha"),
    ("drag po", "rudra"),
    ("blo ngan", "durmati"),
    ("rnga chen", "dundubhi"),
    ("khrag skyug", "rudhirura"),
    ("mig dmar", "raktaksi"),
    ("khro bo", "krodhana"),
    ("zad pa", "ksayaka"),
];

/// The name of year `year` (numbered by the Western year it begins in) in
/// the *rab byung* cycle, in Tibetan and in Sanskrit: position
/// `(Y − 6) amod 60` (Janson, Section 4). 2007 is *thams cad 'dul*, *sarvajit*.
#[must_use]
pub const fn rab_byung_name(year: i64) -> (&'static str, &'static str) {
    RAB_BYUNG_NAMES[(year - 1_027).rem_euclid(60) as usize]
}

/// The year counted from the traditional ascent of the first Tibetan king
/// in 127 BCE: the Tibetan year beginning in Western year `year` is
/// `year + 127` (Janson, Section 4). 2024 is 2151.
#[must_use]
pub const fn royal_year(year: i64) -> i64 {
    year + 127
}

// --- Elements, colours and animals -----------------------------------------

/// The five elements in the order of Janson's table of the five elements, from Wood, with the
/// colour of each in English as Mongolians give it: "green (blue)", red,
/// yellow, white and "dark blue (black)", the Mongolian use being the
/// colour in parentheses (Janson, Appendix E, the table of the elements, and Appendix A.3).
pub const COLOURS: [&str; 5] = ["blue", "red", "yellow", "white", "black"];

/// The Mongolian colour words, male and female, as Gantumur's calendar
/// writes the year: хөх and хөхөгчин, … хар and харагчин (secondary; see
/// the module's sources).
pub const MONGOLIAN_COLOURS: [(&str, &str); 5] = [
    ("хөх", "хөхөгчин"),
    ("улаан", "улаагчин"),
    ("шар", "шарагчин"),
    ("цагаан", "цагаагчин"),
    ("хар", "харагчин"),
];

/// An element, gender and animal of the Chinese-style cycles, as indexes:
/// the element into [`hc_calendars_lunar::tibetan::ELEMENTS`] and
/// [`COLOURS`], the animal into [`hc_calendars_lunar::tibetan::ANIMALS`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol {
    /// The element, 0 for Wood to 4 for Water.
    pub element: u8,
    /// Male (the Chinese *yang*) or female.
    pub male: bool,
    /// The animal, 0 for the Mouse to 11 for the Pig.
    pub animal: u8,
}

/// The year's element, gender and animal: its colour is
/// `COLOURS[element]`, so 1992, Water–Monkey, is the black monkey year.
#[must_use]
pub const fn year_symbol(year: i64) -> Symbol {
    let position = (year - 4).rem_euclid(60) as u8; // 0 is Wood–male–Mouse
    Symbol {
        element: (position % 10) / 2,
        male: position.is_multiple_of(2),
        animal: position % 12,
    }
}

/// Whose rule names the months by animal and element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonthCycle {
    /// The Phugpa's: month 11 of the year before is the Tiger, month 1 the
    /// Dragon, and the Tiger month's element follows the year's (Janson,
    /// Appendix E, attributes for months).
    Phugpa,
    /// The Tsurphu's, which is the Chinese one and the Mongolian's: month 1
    /// is the Tiger, and the elements run on month after month, year after
    /// year (Janson, Appendix E, attributes for months, and Appendix A.3).
    Tsurphu,
}

/// The animal, gender and element of month `month` (1 to 12) of `year`; a
/// leap month has its regular month's (Janson, Appendix E, attributes for months).
#[must_use]
pub const fn month_symbol(cycle: MonthCycle, year: i64, month: u8) -> Symbol {
    let m = month as i64;
    let (element, animal) = match cycle {
        MonthCycle::Phugpa => {
            let element = if m <= 10 {
                (year - 1).div_euclid(2) + (year - 1).rem_euclid(2) + (m + 1) / 2
            } else {
                year.div_euclid(2) + year.rem_euclid(2) + (m - 11) / 2
            };
            (element, m + 3)
        }
        MonthCycle::Tsurphu => (year - 2 + (m - 1) / 2, m + 1),
    };
    // Janson numbers both from 1 with `amod`; the indexes count from 0.
    Symbol {
        element: (element - 1).rem_euclid(5) as u8,
        male: m % 2 == 1,
        animal: animal.rem_euclid(12) as u8,
    }
}

/// The calendar day's element, gender and animal: the element number
/// `⌈JD/2⌉ amod 5`, male when the Julian Day Number is odd, and the animal
/// `(JD + 2) amod 12`, as in the Chinese calendar (Janson, Appendix E, attributes for calendar days).
/// 12 February 1992 is the yellow horse day.
#[must_use]
pub const fn day_symbol(rd: Rd) -> Symbol {
    let jdn = rd.to_julian_day_number();
    Symbol {
        element: ((jdn + 1).div_euclid(2) - 1).rem_euclid(5) as u8,
        male: jdn.rem_euclid(2) == 1,
        animal: (jdn + 1).rem_euclid(12) as u8,
    }
}

// --- The astrological attributes of a day ------------------------------------

/// One of the eight trigrams (*spar kha*), with the attributes Janson's
/// table of the trigrams gives it (Appendix E, Table 15).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Trigram {
    /// The Tibetan name, as Janson and Henning's almanacs print it: `li`.
    pub tibetan: &'static str,
    /// The Chinese name in Pinyin: `lí`.
    pub chinese: &'static str,
    /// The direction: `S`.
    pub direction: &'static str,
    /// The attribute Janson calls its element: `fire`.
    pub element: &'static str,
}

/// The eight trigrams in Janson's order, the King Wen or Later Heaven one,
/// *li* first, which his formulas number 1 to 8 (Appendix E, Table 15).
pub const TRIGRAMS: [Trigram; 8] = [
    Trigram {
        tibetan: "li",
        chinese: "lí",
        direction: "S",
        element: "fire",
    },
    Trigram {
        tibetan: "khon",
        chinese: "kūn",
        direction: "SW",
        element: "earth",
    },
    Trigram {
        tibetan: "dwa",
        chinese: "duì",
        direction: "W",
        element: "iron",
    },
    Trigram {
        tibetan: "khen",
        chinese: "qián",
        direction: "NW",
        element: "sky",
    },
    Trigram {
        tibetan: "kham",
        chinese: "kǎn",
        direction: "N",
        element: "water",
    },
    Trigram {
        tibetan: "gin",
        chinese: "gèn",
        direction: "NE",
        element: "mountain",
    },
    Trigram {
        tibetan: "zin",
        chinese: "zhèn",
        direction: "E",
        element: "wood",
    },
    Trigram {
        tibetan: "zon",
        chinese: "xùn",
        direction: "SE",
        element: "wind",
    },
];

/// One of the nine numbers (*sme ba*), 1 to 9, with the colour, element
/// and direction Janson's table of the numbers gives it (Appendix E,
/// Table 16).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NineNumber {
    /// The colour: `white` for 1.
    pub colour: &'static str,
    /// The element: `iron` for 1.
    pub element: &'static str,
    /// The direction in the magic square: `N` for 1.
    pub direction: &'static str,
}

/// The nine numbers, 1 first (Janson, Appendix E, Table 16).
pub const NINE_NUMBERS: [NineNumber; 9] = [
    NineNumber {
        colour: "white",
        element: "iron",
        direction: "N",
    },
    NineNumber {
        colour: "black",
        element: "water",
        direction: "SW",
    },
    NineNumber {
        colour: "blue",
        element: "water",
        direction: "E",
    },
    NineNumber {
        colour: "green",
        element: "wood",
        direction: "SE",
    },
    NineNumber {
        colour: "yellow",
        element: "earth",
        direction: "Centre",
    },
    NineNumber {
        colour: "white",
        element: "iron",
        direction: "NW",
    },
    NineNumber {
        colour: "red",
        element: "fire",
        direction: "W",
    },
    NineNumber {
        colour: "white",
        element: "iron",
        direction: "NE",
    },
    NineNumber {
        colour: "red",
        element: "fire",
        direction: "S",
    },
];

/// What the Chinese-style system gives a lunar day (Janson, Appendix E,
/// attributes for lunar days), as indexes: the animal into
/// [`hc_calendars_lunar::tibetan::ANIMALS`], the element into
/// [`hc_calendars_lunar::tibetan::ELEMENTS`], the trigram into
/// [`TRIGRAMS`]; the number is itself, 1 to 9, and names [`NINE_NUMBERS`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LunarDayAttributes {
    /// The animal, 0 for the Mouse.
    pub animal: u8,
    /// The element, 0 for Wood.
    pub element: u8,
    /// The trigram, 0 for *li*.
    pub trigram: u8,
    /// The number, 1 to 9.
    pub number: u8,
}

/// The animal, element, trigram and number of lunar day `day` (1 to 30) of
/// month `month` (1 to 12) of `year`, by Janson's rules (Appendix E,
/// attributes for lunar days): the animal `(D + 6M + 8) amod 12`, an odd
/// month beginning with the Tiger and an even one with the Monkey (E.9);
/// the element the month's, under `cycle`, advanced by `D` (`(x + D) amod
/// 5`); the trigram `(D + 6A + 6) amod 8` and the number `(D + 3A) amod 9`,
/// `A` the month's animal from the Mouse as 1 (E.10, E.11), so that a Tiger
/// month begins with *li* and 1. A leap month has its regular month's
/// attributes, as its symbol does.
///
/// Henning's almanacs print the animal, trigram and number on the calendar
/// day a lunar day ends in, and for a skipped one after the month, "26.
/// Omitted: Rabbit gin 5"; the first of two days with one number, in which
/// no lunar day ends, has none. They do not print the element, which rests
/// on Janson's rule alone.
///
/// `None` for a month outside 1 to 12 or a day outside 1 to 30.
#[must_use]
pub const fn lunar_day_attributes(
    cycle: MonthCycle,
    year: i64,
    month: u8,
    day: u8,
) -> Option<LunarDayAttributes> {
    if month < 1 || month > 12 || day < 1 || day > 30 {
        return None;
    }
    let symbol = month_symbol(cycle, year, month);
    let d = day as u16;
    let m = month as u16;
    // Janson numbers the animals from the Mouse as 1.
    let a = symbol.animal as u16 + 1;
    Some(LunarDayAttributes {
        animal: ((d + 6 * m + 8 - 1) % 12) as u8,
        element: ((symbol.element as u16 + d) % 5) as u8,
        trigram: ((d + 6 * a + 6 - 1) % 8) as u8,
        number: ((d + 3 * a - 1) % 9 + 1) as u8,
    })
}

/// The calendar day's trigram, `(JD + 2) amod 8` (Janson, Appendix E,
/// attributes for calendar days), as an index into [`TRIGRAMS`]. No almanac
/// read prints it, so it rests on Janson's rule alone.
#[must_use]
pub const fn day_trigram(rd: Rd) -> u8 {
    (rd.to_julian_day_number() + 1).rem_euclid(8) as u8
}

/// The calendar day's number, 1 to 9, by Janson's rule, `(−JD) amod 9`,
/// one less each day (Appendix E, attributes for calendar days); he reports
/// Henning's book as computing the same, "10 − ((JD + 1) amod 9)" (Remark
/// 36). Henning's computed almanacs print another count,
/// [`henning_almanac_day_number`]: the two are two conventions, and each is
/// a function (docs/policy.md §5).
#[must_use]
pub const fn janson_day_number(rd: Rd) -> u8 {
    ((-rd.to_julian_day_number() - 1).rem_euclid(9) + 1) as u8
}

/// The number, 1 to 9, Henning's computed Phugpa and Bhutanese almanacs
/// print after a calendar day's Chinese mansion, "Solar: Earth-Monkey. Bi
/// 9" for 11 February 2013: `(JD − 1) amod 9`, one more each day, without a
/// turn at either solstice in the years read. No text read states it, and
/// it runs the other way from Janson's rule ([`janson_day_number`]); this
/// is the library's reading of the pages. The Tsurphu almanacs print none,
/// Janson reporting that Tsurphu calendars count the numbers from the
/// solstices (Remark 37), which is not carried.
#[must_use]
pub const fn henning_almanac_day_number(rd: Rd) -> u8 {
    ((rd.to_julian_day_number() - 2).rem_euclid(9) + 1) as u8
}

/// The twenty-eight Chinese lunar mansions a Henning almanac names a
/// calendar day by, from *Jiao*, in his spelling: three are *Wei* and two
/// *Bi*, as he prints them.
pub const CHINESE_MANSIONS: [&str; 28] = [
    "Jiao", "Kang", "Di", "Fang", "Xin", "Wei", "Ji", "Dou", "Niu", "Nu", "Xu", "Wei", "Shi", "Bi",
    "Kui", "Lou", "Wei", "Mao", "Bi", "Zui", "Can", "Jing", "Gui", "Liu", "Xing", "Zhang", "Yi",
    "Zhen",
];

/// The calendar day's Chinese mansion, as an index into
/// [`CHINESE_MANSIONS`]: a cycle of twenty-eight days, *Jiao* on a day
/// whose Julian Day Number is 17 mod 28, as Henning's Phugpa, Tsurphu and
/// Bhutanese almanacs print it on every day read. No text read states the
/// rule. It is the same count as the Japanese almanac's 二十八宿,
/// `hc-almanac`'s `mansions::mansion_of`, which the facade's tests hold.
#[must_use]
pub const fn chinese_mansion(rd: Rd) -> u8 {
    (rd.to_julian_day_number() - 17).rem_euclid(28) as u8
}

/// The four elements of the Indian system, as Henning's almanacs print
/// them: an index of [`WEEKDAY_ELEMENTS`] and [`MANSION_ELEMENTS`] names
/// one of these.
pub const INDIAN_ELEMENTS: [&str; 4] = ["Earth", "Fire", "Water", "Wind"];

/// The element of each weekday, from Saturday, as an index into
/// [`INDIAN_ELEMENTS`] (Janson, Section 9, the table of the days of the
/// week): Saturn's earth, the Sun's fire, the Moon's water, Mars' fire,
/// Mercury's water, Jupiter's wind and Venus' earth.
pub const WEEKDAY_ELEMENTS: [u8; 7] = [0, 1, 2, 1, 2, 3, 0];

/// The element of each lunar mansion of [`MANSIONS`], as an index into
/// [`INDIAN_ELEMENTS`]. Janson refers to Henning's book for the list,
/// which was not read; these are the elements Henning's almanacs print
/// after the mansion, each the same on every one of the 40 161 days of
/// his Phugpa, Bhutanese and Tsurphu almanacs read.
pub const MANSION_ELEMENTS: [u8; 27] = [
    3, 1, 1, 0, 3, 2, 3, 1, 2, 1, 1, 3, 3, 3, 3, 1, 0, 0, 2, 2, 0, 0, 2, 0, 1, 2, 2,
];

/// The day's two elements, the weekday's and the mansion's (Janson,
/// Appendix E, elemental *yoga*), as indexes into [`INDIAN_ELEMENTS`], in
/// the order Henning's almanacs print them: "mon gre. Water-Water" for
/// Monday 11 February 2013. The weekday is the one the almanac names the
/// day by, so a Bhutanese almanac's is [`bhutanese_weekday`]. Janson
/// regards the pair as unordered and names ten combinations after
/// Henning's book, whose names were not read and are not carried. `None`
/// for a weekday past 6 or a mansion past 26.
#[must_use]
pub const fn element_pair(weekday: u8, mansion: u8) -> Option<(u8, u8)> {
    if weekday > 6 || mansion > 26 {
        return None;
    }
    Some((
        WEEKDAY_ELEMENTS[weekday as usize],
        MANSION_ELEMENTS[mansion as usize],
    ))
}

// --- Mongolia ----------------------------------------------------------------

/// A season of the Mongolian month names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Season {
    /// Spring, months 1 to 3.
    Spring,
    /// Summer, months 4 to 6.
    Summer,
    /// Autumn, months 7 to 9.
    Autumn,
    /// Winter, months 10 to 12.
    Winter,
}

/// The Mongolian names of the months, as the beginning, middle and end of
/// the four seasons from the first spring month (Janson, Appendix A.3,
/// after Sanders and Bat-Iredüi): the season and the month's place in it,
/// 0 to 2.
#[must_use]
pub const fn mongolian_month(month: u8) -> (Season, u8) {
    let index = (month.saturating_sub(1)) % 12;
    let season = match index / 3 {
        0 => Season::Spring,
        1 => Season::Summer,
        2 => Season::Autumn,
        _ => Season::Winter,
    };
    (season, index % 3)
}

/// The months' Mongolian names as Gantumur's calendar writes them in its
/// month headings, "Хаврын тэргүүн" to "Өвлийн сүүл", before сар (secondary;
/// its month menu has "хаврын эхэн" for the first).
pub const MONGOLIAN_MONTH_NAMES: [&str; 12] = [
    "Хаврын тэргүүн",
    "Хаврын дунд",
    "Хаврын сүүл",
    "Зуны эхэн",
    "Зуны дунд",
    "Зуны сүүл",
    "Намрын эхэн",
    "Намрын дунд",
    "Намрын сүүл",
    "Өвлийн эхэн",
    "Өвлийн дунд",
    "Өвлийн сүүл",
];

/// The word for a leap month in Gantumur's Mongolian calendar, which heads
/// a leap month "Зуны эхэн илүү сар", the leap first summer month
/// (secondary). No `mn` locale is carried, so no date is written with it.
pub const MONGOLIAN_LEAP_WORD: &str = "илүү";

// --- Bhutan ---------------------------------------------------------------------

/// The weekday the Bhutanese almanac names a day by, one ahead of the
/// world's and the other Tibetan calendars', as an index into [`WEEKDAYS`]:
/// `(JD + 3) mod 7`. Henning's Monday 5 May 2008 was a Tuesday, *mig dmar*,
/// in Bhutan (Janson, Appendix A.4; Henning, "The Bhutanese calendar").
#[must_use]
pub const fn bhutanese_weekday(rd: Rd) -> u8 {
    (rd.to_julian_day_number() + 3).rem_euclid(7) as u8
}

/// The Bhutanese winter solstice of Gregorian year `year`: the instant the
/// mean Sun of the Bhutanese calendar reaches 250°, 18;45 in mansions
/// (Janson, Appendix A.4; Henning, "Bhutan calendars"), as a local Julian
/// Date whose integer part is the day, found with Janson's rule of Section 10 for the special days, the
/// lunar day `d` at which the mean Sun has a given value.
///
/// # Errors
///
/// [`CalendarError::YearOutOfRange`] outside the Bhutanese calendar's
/// range.
pub fn bhutanese_winter_solstice(calendar: &TibetanCalendar, year: i64) -> CalendarResult<Ratio> {
    if !(hc_calendars_lunar::tibetan::MIN_YEAR..=hc_calendars_lunar::tibetan::MAX_YEAR)
        .contains(&year)
    {
        return Err(CalendarError::YearOutOfRange);
    }
    let target = Ratio::new(25, 36);
    let first = hc_calendar::gregorian::to_fixed_saturating(year, 1, 1).to_julian_day_number();
    let last = hc_calendar::gregorian::to_fixed_saturating(year, 12, 31).to_julian_day_number();
    // Lunar days `L` from the epoch at which the mean Sun is `k + 250°`:
    // `L = (k + λ − s0) / s2`, and the mean date `L · m2 + m0`.
    let s2 = tibetan::S2;
    let m2 = tibetan::M2;
    let s0 = calendar.mean_sun(0, 0);
    let m0 = calendar.mean_date(0, 0);
    let revolutions = Ratio::int(i128::from(first))
        .sub(m0)
        .mul(Ratio::new(m2.denominator(), m2.numerator()))
        .mul(s2)
        .floor();
    for k in revolutions - 1..=revolutions + 1 {
        let lunar_days = Ratio::int(k)
            .add(target)
            .sub(s0)
            .mul(Ratio::new(s2.denominator(), s2.numerator()));
        let date = lunar_days.mul(m2).add(m0);
        let day = date.floor() as i64;
        if (first..=last).contains(&day) {
            return Ok(date);
        }
    }
    Err(CalendarError::DayOutOfRange)
}

// --- Festivals on skipped and repeated dates ---------------------------------

/// A festival on a fixed Tibetan date, as Henning's computed Phugpa
/// almanacs mark it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Festival {
    /// The month, 1 to 12.
    pub month: u8,
    /// The day, 1 to 30.
    pub day: u8,
    /// The almanac's English words.
    pub english_name: &'static str,
}

/// The festivals Henning's computed Phugpa almanacs mark, every year from
/// 1960 to 2045 as read, with his English words (`tdata/pl_*.txt`,
/// [kalacakra-org]). Janson refers to a list of fixed-date holidays in
/// Henning's book, Appendix II, which was not read; whether it holds more
/// is not known here.
pub const HENNING_FESTIVALS: [Festival; 7] = [
    Festival {
        month: 1,
        day: 1,
        english_name: "From 1st to 15th, Demonstration of Miracles.",
    },
    Festival {
        month: 3,
        day: 15,
        english_name: "Revelation of the Kalacakra Tantra.",
    },
    Festival {
        month: 4,
        day: 7,
        english_name: "Birth of the Buddha.",
    },
    Festival {
        month: 4,
        day: 15,
        english_name: "Enlightenment and Parinirvana of the Buddha.",
    },
    Festival {
        month: 6,
        day: 4,
        english_name: "Turning of the Wheel of the Dharma.",
    },
    Festival {
        month: 6,
        day: 15,
        english_name: "The Buddha's entry into the womb of his mother.",
    },
    Festival {
        month: 9,
        day: 22,
        english_name: "Descent of the Buddha from the realm of the gods.",
    },
];

/// The day a festival on `day` of `month` of `year` is kept by the rule
/// Janson gives from Berzin: on the day itself; on the day before when the
/// number is skipped, which is the calendar day the skipped lunar day ends
/// in; on the first of the two days when it is repeated (Janson, Section
/// 11). Janson has "not checked them against published calendars".
///
/// # Errors
///
/// [`CalendarError::YearOutOfRange`] outside the years converted,
/// [`CalendarError::MonthOutOfRange`] for a month the year does not have
/// and [`CalendarError::DayOutOfRange`] for a day outside 1 to 30.
pub fn berzin_day(
    calendar: &TibetanCalendar,
    year: i64,
    month: Month,
    day: u8,
) -> CalendarResult<Rd> {
    let (before, end) = span(calendar, year, month, day)?;
    Ok(Rd::from_julian_day_number(if end - before == 2 {
        before + 1
    } else {
        end
    }))
}

/// The day Henning's computed almanacs mark a festival on `day` of `month`
/// of `year`: the day itself; the second of the two days when the number is
/// repeated; and no day when it is skipped — his Birth of the Buddha is not
/// marked in 1966, 1975 or 1990, whose 7th of month 4 is skipped. In a year
/// whose month is doubled his almanacs mark the festival in both months:
/// the Turning of the Wheel of 2024 on 10 July, the 4th of the leap month
/// 6, and on 8 August, the 4th of month 6, so a caller asks for each
/// [`Month`], the leap one and the regular one. The rule is this library's
/// reading of his pages, which state none.
///
/// # Errors
///
/// As [`berzin_day`].
pub fn henning_almanac_day(
    calendar: &TibetanCalendar,
    year: i64,
    month: Month,
    day: u8,
) -> CalendarResult<Option<Rd>> {
    let (before, end) = span(calendar, year, month, day)?;
    Ok(if end == before {
        None
    } else {
        Some(Rd::from_julian_day_number(end))
    })
}

/// A rule for the day a festival on a skipped or repeated date is kept, by
/// the identifier a caller selects it by (policy §5).
#[derive(Debug, Clone, Copy)]
pub struct FestivalRule {
    /// A stable identifier, lowercase and hyphenated.
    pub id: &'static str,
    /// The day the rule keeps `day` of `month` of `year` on, or `None`
    /// where it keeps none.
    pub day: fn(&TibetanCalendar, i64, Month, u8) -> CalendarResult<Option<Rd>>,
}

/// [`berzin_day`], as a [`FestivalRule`]'s function.
fn berzin_rule(
    calendar: &TibetanCalendar,
    year: i64,
    month: Month,
    day: u8,
) -> CalendarResult<Option<Rd>> {
    berzin_day(calendar, year, month, day).map(Some)
}

hc_core::catalogue! {
    type: FestivalRule,
    id: |rule| rule.id,
    tests: festival_rule_catalogue_tests,
    associated;

    /// The two rules, Berzin's first.
    pub const ALL;
    /// The rule with this identifier.
    pub fn by_id;

    entries: {
        /// [`berzin_day`]: the day before a skipped number, the first of a
        /// repeated one.
        pub const BERZIN = Self { id: "berzin", day: berzin_rule };
        /// [`henning_almanac_day`]: the second of a repeated number, and no
        /// day for a skipped one.
        pub const HENNING_ALMANAC = Self { id: "henning-almanac", day: henning_almanac_day };
    }
}

/// The end days of the lunar day before `day` of `month` of `year` and of
/// `day` itself.
fn span(
    calendar: &TibetanCalendar,
    year: i64,
    month: Month,
    day: u8,
) -> CalendarResult<(i64, i64)> {
    if !(tibetan::MIN_YEAR..=tibetan::MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    if day == 0 || day > 30 {
        return Err(CalendarError::DayOutOfRange);
    }
    let n = calendar
        .true_month_count(year, month.ordinal, month.leap)
        .ok_or(CalendarError::MonthOutOfRange)?;
    Ok(calendar.lunar_day_span(i64::from(day), n))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_lunar::tibetan::{
        TIBETAN_BHUTAN, TIBETAN_BHUTAN_LOCHEN, TIBETAN_LOCHEN, TIBETAN_TSURPHU_KARANA,
    };

    /// Henning's Bhutanese almanacs: the Bhutanese under Lochen's anomaly.
    const HENNING_BHUTAN: TibetanCalendar = TIBETAN_BHUTAN_LOCHEN;
    use hc_calendar::gregorian;
    use hc_calendars_lunar::tibetan::{ANIMALS, ELEMENTS};

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed_saturating(year, month, day)
    }

    /// A day of Henning's almanac: the Gregorian date, the Tibetan month and
    /// day, the true weekday, the Moon at daybreak, the true Sun, the *yoga*
    /// longitude, the Sun in signs (the mean one in the Phugpa and
    /// Bhutanese, the true *karaṇa* one in the Tsurphu), and the mansion,
    /// *yoga* and *karaṇa* in Tibetan, which the first of two days with one
    /// number leaves blank.
    type Row = (
        i64,
        u8,
        u8,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    );

    // Transcribed from Henning's computed almanacs, `tdata/pl_2013.txt`
    // months 1 and 2, `tdata/ts_2012.txt` months 1 and 2 and
    // `tdata/bh_2019.txt` month 1 [kalacakra-org], read 2026-09-29.
    const PHUGPA_2013: &[Row] = &[
        (
            2013,
            2,
            11,
            "1-1",
            "2;6,31",
            "22;14,22",
            "21;26,54",
            "16;41,17",
            "9;15,17",
            "mon gre",
            "phan tshun",
            "gdab pa",
        ),
        (
            2013,
            2,
            12,
            "1-2",
            "3;4,1",
            "23;15,26",
            "21;31,27",
            "17;46,54",
            "9;16,16",
            "mon gru",
            "mchog can",
            "rigs can",
        ),
        (
            2013,
            2,
            13,
            "1-3",
            "4;2,46",
            "24;15,15",
            "21;36,1",
            "18;51,16",
            "9;17,14",
            "khrums stod",
            "yongs 'joms",
            "khyim skyes",
        ),
        (
            2013,
            2,
            14,
            "1-4",
            "5;3,16",
            "25;13,18",
            "21;40,34",
            "19;53,53",
            "9;18,12",
            "khrums smad",
            "zhi ba",
            "vishti",
        ),
        (
            2013, 2, 15, "1-5", "6;4,46", "26;10,21", "21;45,8", "20;55,30", "9;19,10", "nam gru",
            "grub pa", "byis pa",
        ),
        (
            2013,
            2,
            16,
            "1-6",
            "0;7,16",
            "0;6,25",
            "21;49,42",
            "21;56,7",
            "9;20,8",
            "tha skar",
            "bsgrub bya",
            "til rdung",
        ),
        (
            2013,
            2,
            17,
            "1-7",
            "1;10,46",
            "1;1,28",
            "21;54,15",
            "22;55,44",
            "9;21,7",
            "bra nye",
            "dge ba",
            "tshong ba",
        ),
        (
            2013, 2, 18, "1-8", "2;15,2", "1;55,46", "21;58,49", "23;54,36", "9;22,5", "bra nye",
            "dkar po", "gdab pa",
        ),
        (
            2013,
            2,
            19,
            "1-9",
            "3;19,17",
            "2;50,5",
            "22;3,22",
            "24;53,27",
            "9;23,3",
            "smin drug",
            "tshangs pa",
            "rigs can",
        ),
        (
            2013,
            2,
            20,
            "1-10",
            "4;23,33",
            "3;44,23",
            "22;7,56",
            "25;52,19",
            "9;24,1",
            "snar ma",
            "dbang po",
            "khyim skyes",
        ),
        (
            2013,
            2,
            21,
            "1-11",
            "5;27,48",
            "4;38,41",
            "22;12,30",
            "26;51,11",
            "9;25,0",
            "mgo",
            "'khon 'dzin",
            "tshong ba",
        ),
        (
            2013, 2, 22, "1-12", "6;32,3", "5;32,59", "22;17,3", "0;50,3", "9;25,58", "lag",
            "rnam sel", "gdab pa",
        ),
        (
            2013, 2, 23, "1-13", "0;36,4", "6;27,32", "22;21,37", "1;49,9", "9;26,56", "nabs so",
            "mdza' bo", "rigs can",
        ),
        (
            2013,
            2,
            24,
            "1-14",
            "1;39,4",
            "7;23,5",
            "22;26,10",
            "2;49,16",
            "9;27,54",
            "rgyal",
            "tshe dang ldan pa",
            "khyim skyes",
        ),
        (
            2013,
            2,
            25,
            "1-15",
            "2;41,5",
            "8;19,39",
            "22;30,44",
            "3;50,23",
            "9;28,52",
            "skag",
            "skal bzang",
            "vishti",
        ),
        (
            2013, 2, 26, "1-16", "3;42,5", "9;17,12", "22;35,17", "4;52,30", "9;29,51", "mchu",
            "dge byed", "byis pa",
        ),
        (
            2013,
            2,
            27,
            "1-17",
            "4;41,47",
            "10;16,0",
            "22;39,48",
            "5;55,48",
            "10;0,49",
            "gre",
            "shin tu 'grams",
            "til rdung",
        ),
        (
            2013,
            2,
            28,
            "1-18",
            "5;39,43",
            "11;16,34",
            "22;44,17",
            "7;0,52",
            "10;1,47",
            "dbo",
            "'dzin byed",
            "tshong ba",
        ),
        (
            2013, 3, 1, "1-19", "6;36,40", "12;18,7", "22;48,47", "8;6,55", "10;2,45", "me bzhi",
            "zug rngu", "gdab pa",
        ),
        (
            2013, 3, 2, "1-20", "0;32,36", "13;20,40", "22;53,17", "9;13,58", "10;3,43", "nag pa",
            "'grams", "rigs can",
        ),
        (
            2013,
            3,
            3,
            "1-21",
            "1;27,32",
            "14;24,14",
            "22;57,47",
            "10;22,1",
            "10;4,42",
            "sa ri",
            "'phel",
            "khyim skyes",
        ),
        (
            2013, 3, 4, "1-22", "2;21,44", "15;28,32", "23;2,16", "11;30,49", "10;5,40", "sa ga",
            "brtan pa", "gdab pa",
        ),
        (
            2013,
            3,
            5,
            "1-23",
            "3;15,55",
            "16;32,50",
            "23;6,46",
            "12;39,37",
            "10;6,38",
            "lha mtshams",
            "yongs bsnun",
            "rigs can",
        ),
        (
            2013,
            3,
            6,
            "1-24",
            "4;10,6",
            "17;37,9",
            "23;11,16",
            "13;48,25",
            "10;7,36",
            "snron",
            "dga' ba",
            "khyim skyes",
        ),
        (
            2013, 3, 7, "1-25", "5;4,18", "18;41,27", "23;15,45", "14;57,13", "10;8,34", "snrubs",
            "rdo rje", "vishti",
        ),
        (
            2013,
            3,
            8,
            "1-27",
            "6;52,56",
            "19;49,49",
            "23;24,45",
            "16;14,34",
            "10;10,31",
            "chu stod",
            "phan tshun",
            "rigs can",
        ),
        (
            2013,
            3,
            9,
            "1-28",
            "0;48,22",
            "20;52,52",
            "23;29,14",
            "17;22,7",
            "10;11,29",
            "chu smad",
            "mchog can",
            "khyim skyes",
        ),
        (
            2013,
            3,
            10,
            "1-29",
            "1;44,49",
            "21;54,55",
            "23;33,44",
            "18;28,40",
            "10;12,27",
            "gro zhin",
            "yongs 'joms",
            "vishti",
        ),
        (
            2013,
            3,
            11,
            "1-30",
            "2;42,15",
            "22;55,58",
            "23;38,14",
            "19;34,12",
            "10;13,25",
            "mon gre",
            "zhi ba",
            "rkang bzhi",
        ),
        (
            2013,
            3,
            12,
            "2-1",
            "3;40,57",
            "23;55,46",
            "23;42,44",
            "20;38,30",
            "10;14,24",
            "mon gru",
            "grub pa",
            "mi sdug pa",
        ),
        (
            2013,
            3,
            13,
            "2-2",
            "4;41,24",
            "24;53,49",
            "23;47,13",
            "21;41,2",
            "10;15,22",
            "khrums stod",
            "bsgrub bya",
            "byis pa",
        ),
        (
            2013,
            3,
            14,
            "2-3",
            "5;42,51",
            "25;50,52",
            "23;51,43",
            "22;42,35",
            "10;16,20",
            "khrums smad",
            "dge ba",
            "til rdung",
        ),
        (
            2013,
            3,
            15,
            "2-4",
            "6;45,18",
            "26;46,55",
            "23;56,13",
            "23;43,8",
            "10;17,18",
            "nam gru",
            "dkar po",
            "tshong ba",
        ),
        (
            2013,
            3,
            16,
            "2-5",
            "0;48,44",
            "0;41,58",
            "24;0,42",
            "24;42,40",
            "10;18,17",
            "tha skar",
            "tshangs pa",
            "gdab pa",
        ),
        (
            2013, 3, 17, "2-6", "1;52,56", "1;36,16", "24;5,12", "25;41,28", "10;19,15", "bra nye",
            "dbang po", "rigs can",
        ),
        (
            2013,
            3,
            18,
            "2-7",
            "2;57,7",
            "2;30,34",
            "24;9,42",
            "26;40,16",
            "10;20,13",
            "smin drug",
            "'khon 'dzin",
            "til rdung",
        ),
        (
            2013, 3, 19, "2-8", "3;60,0", "3;26,11", "24;14,11", "0;40,23", "10;21,11", "snar ma",
            "", "",
        ),
        (
            2013, 3, 20, "2-8", "4;1,19", "4;24,52", "24;14,11", "1;39,4", "10;21,11", "mgo",
            "mdza' bo", "gdab pa",
        ),
        (
            2013,
            3,
            21,
            "2-9",
            "5;5,30",
            "5;19,10",
            "24;18,41",
            "2;37,52",
            "10;22,9",
            "lag",
            "tshe dang ldan pa",
            "rigs can",
        ),
        (
            2013,
            3,
            22,
            "2-10",
            "6;9,42",
            "6;13,28",
            "24;23,11",
            "3;36,40",
            "10;23,8",
            "nabs so",
            "skal bzang",
            "khyim skyes",
        ),
        (
            2013, 3, 23, "2-11", "0;13,38", "7;8,2", "24;27,41", "4;35,43", "10;24,6", "rgyal",
            "dge byed", "vishti",
        ),
        (
            2013,
            3,
            24,
            "2-12",
            "1;16,34",
            "8;3,36",
            "24;32,10",
            "5;35,46",
            "10;25,4",
            "skag",
            "shin tu 'grams",
            "byis pa",
        ),
        (
            2013,
            3,
            25,
            "2-13",
            "2;18,30",
            "9;0,9",
            "24;36,40",
            "6;36,50",
            "10;26,2",
            "mchu",
            "las bzang",
            "til rdung",
        ),
        (
            2013,
            3,
            26,
            "2-14",
            "3;19,26",
            "9;57,43",
            "24;41,10",
            "7;38,53",
            "10;27,0",
            "mchu",
            "'dzin byed",
            "tshong ba",
        ),
        (
            2013, 3, 27, "2-15", "4;19,7", "10;56,32", "24;45,39", "8;42,12", "10;27,59", "gre",
            "zug rngu", "gdab pa",
        ),
        (
            2013, 3, 28, "2-16", "5;17,3", "11;57,6", "24;50,9", "9;47,16", "10;28,57", "dbo",
            "'grams", "rigs can",
        ),
        (
            2013,
            3,
            29,
            "2-17",
            "6;13,58",
            "12;58,40",
            "24;54,39",
            "10;53,19",
            "10;29,55",
            "me bzhi",
            "'phel",
            "khyim skyes",
        ),
        (
            2013,
            3,
            30,
            "2-18",
            "0;9,49",
            "14;1,14",
            "24;59,3",
            "12;0,17",
            "11;0,53",
            "sa ri",
            "yongs bsnun",
            "vishti",
        ),
        (
            2013, 3, 31, "2-19", "1;4,39", "15;4,48", "25;3,27", "13;8,15", "11;1,51", "sa ga",
            "dga' ba", "byis pa",
        ),
        (
            2013,
            4,
            1,
            "2-21",
            "2;52,50",
            "16;13,24",
            "25;12,15",
            "14;25,39",
            "11;3,48",
            "lha mtshams",
            "rdo rje",
            "khyim skyes",
        ),
        (
            2013,
            4,
            2,
            "2-22",
            "3;46,55",
            "17;17,43",
            "25;16,39",
            "15;34,22",
            "11;4,46",
            "snron",
            "dngos grub",
            "vishti",
        ),
        (
            2013,
            4,
            3,
            "2-23",
            "4;41,1",
            "18;22,1",
            "25;21,2",
            "16;43,4",
            "11;5,44",
            "snrubs",
            "phan tshun",
            "byis pa",
        ),
        (
            2013,
            4,
            4,
            "2-24",
            "5;35,7",
            "19;26,19",
            "25;25,26",
            "17;51,46",
            "11;6,42",
            "chu stod",
            "mchog can",
            "til rdung",
        ),
        (
            2013,
            4,
            5,
            "2-25",
            "6;29,28",
            "20;30,22",
            "25;29,50",
            "19;0,13",
            "11;7,41",
            "chu smad",
            "zhi ba",
            "tshong ba",
        ),
        (
            2013, 4, 6, "2-26", "0;24,49", "21;33,25", "25;34,14", "20;7,39", "11;8,39",
            "gro zhin", "grub pa", "byis pa",
        ),
        (
            2013,
            4,
            7,
            "2-27",
            "1;21,10",
            "22;35,27",
            "25;38,38",
            "21;14,6",
            "11;9,37",
            "mon gre",
            "bsgrub bya",
            "til rdung",
        ),
        (
            2013,
            4,
            8,
            "2-28",
            "2;18,31",
            "23;36,30",
            "25;43,2",
            "22;19,32",
            "11;10,35",
            "mon gru",
            "dge ba",
            "tshong ba",
        ),
        (
            2013,
            4,
            9,
            "2-29",
            "3;17,8",
            "24;36,17",
            "25;47,26",
            "23;23,43",
            "11;11,34",
            "khrums stod",
            "dkar po",
            "bkra shis",
        ),
        (
            2013,
            4,
            10,
            "2-30",
            "4;17,30",
            "25;34,19",
            "25;51,50",
            "24;26,10",
            "11;12,32",
            "khrums smad",
            "tshangs pa",
            "klu",
        ),
    ];
    const TSURPHU_2012: &[Row] = &[
        (
            2012,
            2,
            22,
            "1-1",
            "4;52,31",
            "23;13,26",
            "23;11,57",
            "19;25,24",
            "10;9,19",
            "mon gru",
            "zhi ba",
            "mi sdug pa",
        ),
        (
            2012,
            2,
            23,
            "1-2",
            "5;54,58",
            "24;9,29",
            "23;16,27",
            "20;25,56",
            "10;10,19",
            "khrums stod",
            "grub pa",
            "gdab pa",
        ),
        (
            2012,
            2,
            24,
            "1-3",
            "6;58,25",
            "25;4,31",
            "23;20,57",
            "21;25,28",
            "10;11,19",
            "khrums smad",
            "bsgrub bya",
            "rigs can",
        ),
        (
            2012,
            2,
            25,
            "1-4",
            "0;60,0",
            "26;4,31",
            "23;25,27",
            "22;29,58",
            "10;12,19",
            "nam gru",
            "dge ba",
            "khyim skyes",
        ),
        (
            2012, 2, 26, "1-4", "1;2,37", "26;58,49", "23;25,27", "23;24,16", "10;12,19",
            "nam gru", "dkar po", "viSTi",
        ),
        (
            2012,
            2,
            27,
            "1-5",
            "2;6,48",
            "0;53,8",
            "23;29,56",
            "24;23,4",
            "10;13,19",
            "tha skar",
            "tshangs pa",
            "byis pa",
        ),
        (
            2012,
            2,
            28,
            "1-6",
            "3;11,0",
            "1;47,26",
            "23;34,26",
            "25;21,52",
            "10;14,19",
            "bra nye",
            "dbang po",
            "til rdung",
        ),
        (
            2012,
            2,
            29,
            "1-7",
            "4;15,11",
            "2;41,44",
            "23;38,56",
            "26;20,40",
            "10;15,19",
            "smin drug",
            "'khon 'dzin",
            "tshong ba",
        ),
        (
            2012, 3, 1, "1-8", "5;19,23", "3;36,2", "23;43,25", "0;19,28", "10;16,19", "snar ma",
            "rnam sel", "gdab pa",
        ),
        (
            2012, 3, 2, "1-9", "6;23,18", "4;30,36", "23;47,55", "1;18,32", "10;17,19", "mgo",
            "mdza' bo", "rigs can",
        ),
        (
            2012,
            3,
            3,
            "1-10",
            "0;26,14",
            "5;26,10",
            "23;52,25",
            "2;18,36",
            "10;18,18",
            "lag",
            "tshe dang ldan pa",
            "khyim skyes",
        ),
        (
            2012,
            3,
            4,
            "1-11",
            "1;28,9",
            "6;22,44",
            "23;56,54",
            "3;19,39",
            "10;19,18",
            "nabs so",
            "skal bzang",
            "tshong ba",
        ),
        (
            2012, 3, 5, "1-12", "2;29,5", "7;20,19", "24;1,24", "4;21,43", "10;20,18", "rgyal",
            "dge byed", "gdab pa",
        ),
        (
            2012,
            3,
            6,
            "1-13",
            "3;28,45",
            "8;19,9",
            "24;5,54",
            "5;25,3",
            "10;21,18",
            "skag",
            "shin tu 'grams",
            "rigs can",
        ),
        (
            2012,
            3,
            7,
            "1-14",
            "4;26,40",
            "9;19,43",
            "24;10,24",
            "6;30,7",
            "10;22,18",
            "mchu",
            "las bzang",
            "tshong ba",
        ),
        (
            2012,
            3,
            8,
            "1-15",
            "5;23,36",
            "10;21,17",
            "24;14,53",
            "7;36,11",
            "10;23,18",
            "gre",
            "'dzin byed",
            "gdab pa",
        ),
        (
            2012, 3, 9, "1-16", "6;19,31", "11;23,52", "24;19,23", "8;43,15", "10;24,18", "dbo",
            "zug rngu", "rigs can",
        ),
        (
            2012,
            3,
            10,
            "1-17",
            "0;14,26",
            "12;27,26",
            "24;23,53",
            "9;51,19",
            "10;25,18",
            "me bzhi",
            "'grams",
            "khyim skyes",
        ),
        (
            2012, 3, 11, "1-18", "1;8,38", "13;31,44", "24;28,22", "11;0,7", "10;26,18", "nag pa",
            "brtan pa", "viSTi",
        ),
        (
            2012,
            3,
            12,
            "1-19",
            "2;2,49",
            "14;36,3",
            "24;32,52",
            "12;8,55",
            "10;27,18",
            "sa ri",
            "yongs bsnun",
            "byis pa",
        ),
        (
            2012,
            3,
            13,
            "1-21",
            "3;51,12",
            "15;44,39",
            "24;41,52",
            "13;26,31",
            "10;29,18",
            "sa ga",
            "dga' ba",
            "khyim skyes",
        ),
        (
            2012,
            3,
            14,
            "1-22",
            "4;45,23",
            "16;48,58",
            "24;46,21",
            "14;35,19",
            "11;0,18",
            "lha mtshams",
            "rdo rje",
            "viSTi",
        ),
        (
            2012,
            3,
            15,
            "1-23",
            "5;39,51",
            "17;53,0",
            "24;50,51",
            "15;43,51",
            "11;1,18",
            "snron",
            "dngos grub",
            "byis pa",
        ),
        (
            2012,
            3,
            16,
            "1-24",
            "6;35,18",
            "18;56,2",
            "24;55,20",
            "16;51,23",
            "11;2,17",
            "snrubs",
            "phan tshun",
            "til rdung",
        ),
        (
            2012,
            3,
            17,
            "1-25",
            "0;31,39",
            "19;58,4",
            "24;59,44",
            "17;57,49",
            "11;3,16",
            "chu stod",
            "mchog can",
            "tshong ba",
        ),
        (
            2012, 3, 18, "1-26", "1;29,1", "20;59,7", "25;4,8", "19;3,15", "11;4,15", "chu smad",
            "zhi ba", "gdab pa",
        ),
        (
            2012, 3, 19, "1-27", "2;27,39", "21;58,53", "25;8,32", "20;7,25", "11;5,13",
            "gro zhin", "grub pa", "rigs can",
        ),
        (
            2012,
            3,
            20,
            "1-28",
            "3;28,1",
            "22;56,55",
            "25;12,56",
            "21;9,51",
            "11;6,12",
            "mon gre",
            "bsgrub bya",
            "khyim skyes",
        ),
        (
            2012, 3, 21, "1-29", "4;29,22", "23;53,57", "25;17,20", "22;11,17", "11;7,11",
            "mon gru", "dge ba", "viSTi",
        ),
        (
            2012,
            3,
            22,
            "1-30",
            "5;31,44",
            "24;49,59",
            "25;21,44",
            "23;11,43",
            "11;8,9",
            "khrums stod",
            "dkar po",
            "rkang bzhi",
        ),
        (
            2012,
            3,
            23,
            "2-1",
            "6;35,6",
            "25;45,1",
            "25;26,7",
            "24;11,9",
            "11;9,8",
            "khrums smad",
            "tshangs pa",
            "mi sdug pa",
        ),
        (
            2012, 3, 24, "2-2", "0;39,12", "26;39,19", "25;30,31", "25;9,51", "11;10,7", "nam gru",
            "dbang po", "byis pa",
        ),
        (
            2012,
            3,
            25,
            "2-3",
            "1;43,18",
            "0;33,37",
            "25;34,55",
            "26;8,33",
            "11;11,5",
            "tha skar",
            "'khon 'dzin",
            "til rdung",
        ),
        (
            2012,
            3,
            26,
            "2-4",
            "2;47,23",
            "1;27,55",
            "25;39,19",
            "0;7,15",
            "11;12,4",
            "bra nye",
            "rnam sel",
            "tshong ba",
        ),
        (
            2012,
            3,
            27,
            "2-5",
            "3;51,29",
            "2;22,14",
            "25;43,43",
            "1;5,57",
            "11;13,3",
            "smin drug",
            "mdza' bo",
            "gdab pa",
        ),
        (
            2012,
            3,
            28,
            "2-6",
            "4;55,35",
            "3;16,32",
            "25;48,7",
            "2;4,39",
            "11;14,1",
            "snar ma",
            "tshe dang ldan pa",
            "byis pa",
        ),
        (
            2012,
            3,
            29,
            "2-7",
            "5;59,24",
            "4;11,6",
            "25;52,31",
            "3;3,37",
            "11;15,0",
            "mgo",
            "skal bzang",
            "til rdung",
        ),
        (
            2012,
            3,
            30,
            "2-8",
            "6;60,0",
            "5;11,6",
            "25;56,55",
            "4;8,1",
            "11;15,58",
            "lag",
            "dge byed",
            "tshong ba",
        ),
        (
            2012,
            3,
            31,
            "2-8",
            "0;2,13",
            "6;6,41",
            "25;56,55",
            "5;3,36",
            "11;15,58",
            "nabs so",
            "shin tu 'grams",
            "gdab pa",
        ),
        (
            2012,
            4,
            1,
            "2-9",
            "1;4,3",
            "7;3,15",
            "26;1,19",
            "6;4,34",
            "11;16,57",
            "rgyal",
            "las bzang",
            "rigs can",
        ),
        (
            2012,
            4,
            2,
            "2-10",
            "2;4,52",
            "8;0,50",
            "26;5,42",
            "7;6,33",
            "11;17,56",
            "skag",
            "'dzin byed",
            "khyim skyes",
        ),
        (
            2012, 4, 3, "2-11", "3;4,25", "8;59,41", "26;10,6", "8;9,48", "11;18,54", "skag",
            "zug rngu", "viSTi",
        ),
        (
            2012, 4, 4, "2-12", "4;2,14", "10;0,16", "26;14,30", "9;14,46", "11;19,53", "gre",
            "'grams", "byis pa",
        ),
        (
            2012,
            4,
            5,
            "2-14",
            "5;54,52",
            "11;4,25",
            "26;23,18",
            "10;27,44",
            "11;21,50",
            "dbo",
            "'phel",
            "til rdung",
        ),
        (
            2012, 4, 6, "2-15", "6;49,42", "12;8,0", "26;27,42", "11;35,42", "11;22,49", "me bzhi",
            "brtan pa", "viSTi",
        ),
        (
            2012,
            4,
            7,
            "2-16",
            "0;43,47",
            "13;12,18",
            "26;32,6",
            "12;44,24",
            "11;23,48",
            "nag pa",
            "yongs bsnun",
            "byis pa",
        ),
        (
            2012,
            4,
            8,
            "2-17",
            "1;37,53",
            "14;16,37",
            "26;36,30",
            "13;53,7",
            "11;24,46",
            "sa ri",
            "dga' ba",
            "til rdung",
        ),
        (
            2012,
            4,
            9,
            "2-18",
            "2;31,58",
            "15;20,55",
            "26;40,54",
            "15;1,49",
            "11;25,45",
            "sa ga",
            "dngos grub",
            "tshong ba",
        ),
        (
            2012,
            4,
            10,
            "2-19",
            "3;26,4",
            "16;25,13",
            "26;45,17",
            "16;10,31",
            "11;26,43",
            "lha mtshams",
            "phan tshun",
            "byis pa",
        ),
        (
            2012,
            4,
            11,
            "2-20",
            "4;20,9",
            "17;29,32",
            "26;49,41",
            "17;19,14",
            "11;27,42",
            "snron",
            "mchog can",
            "til rdung",
        ),
        (
            2012,
            4,
            12,
            "2-21",
            "5;14,31",
            "18;33,33",
            "26;54,5",
            "18;27,39",
            "11;28,41",
            "snrubs",
            "yongs 'joms",
            "tshong ba",
        ),
        (
            2012, 4, 13, "2-22", "6;9,53", "19;36,35", "26;58,29", "19;35,5", "11;29,39",
            "chu stod", "zhi ba", "gdab pa",
        ),
        (
            2012, 4, 14, "2-23", "0;6,15", "20;38,37", "0;2,53", "20;41,31", "0;0,38", "chu smad",
            "grub pa", "rigs can",
        ),
        (
            2012,
            4,
            15,
            "2-24",
            "1;3,38",
            "21;39,39",
            "0;7,17",
            "21;46,56",
            "0;1,37",
            "gro zhin",
            "bsgrub bya",
            "khyim skyes",
        ),
        (
            2012, 4, 16, "2-25", "2;2,16", "22;39,24", "0;11,40", "22;51,5", "0;2,35", "mon gre",
            "dge ba", "viSTi",
        ),
        (
            2012, 4, 17, "2-26", "3;2,34", "23;37,26", "0;16,0", "23;53,26", "0;3,33", "mon gru",
            "dkar po", "byis pa",
        ),
        (
            2012,
            4,
            18,
            "2-27",
            "4;3,53",
            "24;34,27",
            "0;20,20",
            "24;54,48",
            "0;4,31",
            "khrums stod",
            "tshangs pa",
            "til rdung",
        ),
        (
            2012,
            4,
            19,
            "2-28",
            "5;6,11",
            "25;30,29",
            "0;24,40",
            "25;55,9",
            "0;5,29",
            "khrums smad",
            "dbang po",
            "tshong ba",
        ),
        (
            2012,
            4,
            20,
            "2-29",
            "6;9,29",
            "26;25,30",
            "0;29,0",
            "26;54,31",
            "0;6,26",
            "nam gru",
            "'khon 'dzin",
            "bkra shis",
        ),
        (
            2012, 4, 21, "2-30", "0;13,31", "0;19,49", "0;33,20", "0;53,9", "0;7,24", "tha skar",
            "rnam sel", "klu",
        ),
    ];
    const BHUTAN_2019: &[Row] = &[
        (
            2019,
            2,
            5,
            "1-1",
            "3;47,40",
            "21;24,48",
            "21;18,29",
            "15;43,17",
            "9;13,30",
            "gro zhin",
            "dngos grub",
            "mi sdug pa",
        ),
        (
            2019,
            2,
            6,
            "1-2",
            "4;51,55",
            "22;19,7",
            "21;23,2",
            "16;42,9",
            "9;14,28",
            "mon gre",
            "phan tshun",
            "byis pa",
        ),
        (
            2019,
            2,
            7,
            "1-3",
            "5;56,11",
            "23;13,25",
            "21;27,36",
            "17;41,1",
            "9;15,26",
            "mon gru",
            "mchog can",
            "rigs can",
        ),
        (
            2019,
            2,
            8,
            "1-4",
            "6;60,0",
            "24;8,9",
            "21;32,9",
            "18;40,19",
            "9;16,25",
            "khrums stod",
            "",
            "",
        ),
        (
            2019,
            2,
            9,
            "1-4",
            "0;0,12",
            "25;7,57",
            "21;32,9",
            "19;40,7",
            "9;16,25",
            "khrums smad",
            "zhi ba",
            "vishti",
        ),
        (
            2019, 2, 10, "1-5", "1;3,14", "26;3,29", "21;36,43", "20;40,12", "9;17,23", "nam gru",
            "grub pa", "byis pa",
        ),
        (
            2019,
            2,
            11,
            "1-6",
            "2;5,15",
            "0;0,1",
            "21;41,17",
            "21;41,18",
            "9;18,21",
            "tha skar",
            "bsgrub bya",
            "til rdung",
        ),
        (
            2019,
            2,
            12,
            "1-7",
            "3;6,16",
            "0;57,33",
            "21;45,50",
            "22;43,24",
            "9;19,19",
            "tha skar",
            "dge ba",
            "tshong ba",
        ),
        (
            2019, 2, 13, "1-8", "4;6,4", "1;56,19", "21;50,24", "23;46,43", "9;20,17", "bra nye",
            "dkar po", "gdab pa",
        ),
        (
            2019,
            2,
            14,
            "1-9",
            "5;4,5",
            "2;56,52",
            "21;54,57",
            "24;51,49",
            "9;21,16",
            "smin drug",
            "tshangs pa",
            "rigs can",
        ),
        (
            2019,
            2,
            15,
            "1-10",
            "6;1,7",
            "3;58,24",
            "21;59,31",
            "25;57,55",
            "9;22,14",
            "snar ma",
            "dbang po",
            "khyim skyes",
        ),
        (
            2019, 2, 16, "1-12", "0;52,9", "5;4,28", "22;8,38", "0;13,7", "9;24,10", "lag",
            "rnam sel", "gdab pa",
        ),
        (
            2019, 2, 17, "1-13", "1;46,24", "6;8,47", "22;13,12", "1;21,59", "9;25,8", "nabs so",
            "mdza' bo", "rigs can",
        ),
        (
            2019,
            2,
            18,
            "1-14",
            "2;40,40",
            "7;13,5",
            "22;17,45",
            "2;30,51",
            "9;26,7",
            "rgyal",
            "tshe dang ldan pa",
            "khyim skyes",
        ),
        (
            2019,
            2,
            19,
            "1-15",
            "3;34,55",
            "8;17,24",
            "22;22,19",
            "3;39,43",
            "9;27,5",
            "skag",
            "skal bzang",
            "vishti",
        ),
        (
            2019, 2, 20, "1-16", "4;29,10", "9;21,42", "22;26,52", "4;48,35", "9;28,3", "mchu",
            "dge byed", "byis pa",
        ),
        (
            2019,
            2,
            21,
            "1-17",
            "5;23,25",
            "10;26,0",
            "22;31,26",
            "5;57,27",
            "9;29,1",
            "gre",
            "shin tu 'grams",
            "khyim skyes",
        ),
        (
            2019,
            2,
            22,
            "1-18",
            "6;17,55",
            "11;30,4",
            "22;36,0",
            "7;6,4",
            "10;0,0",
            "dbo",
            "'dzin byed",
            "vishti",
        ),
        (
            2019, 2, 23, "1-19", "0;13,20", "12;33,9", "22;40,29", "8;13,38", "10;0,58", "me bzhi",
            "zug rngu", "byis pa",
        ),
        (
            2019,
            2,
            24,
            "1-20",
            "1;9,46",
            "13;35,13",
            "22;44,59",
            "9;20,12",
            "10;1,56",
            "nag pa",
            "'grams",
            "til rdung",
        ),
        (
            2019,
            2,
            25,
            "1-21",
            "2;7,11",
            "14;36,17",
            "22;49,29",
            "10;25,46",
            "10;2,54",
            "sa ri",
            "'phel",
            "tshong ba",
        ),
        (
            2019, 2, 26, "1-22", "3;5,51", "15;36,7", "22;53,58", "11;30,6", "10;3,52", "sa ga",
            "brtan pa", "gdab pa",
        ),
        (
            2019,
            2,
            27,
            "1-23",
            "4;6,16",
            "16;34,11",
            "22;58,28",
            "12;32,40",
            "10;4,51",
            "lha mtshams",
            "yongs bsnun",
            "rigs can",
        ),
        (
            2019,
            2,
            28,
            "1-24",
            "5;7,42",
            "17;31,15",
            "23;2,58",
            "13;34,13",
            "10;5,49",
            "snron",
            "dga' ba",
            "khyim skyes",
        ),
        (
            2019, 3, 1, "1-25", "6;10,8", "18;27,19", "23;7,27", "14;34,47", "10;6,47", "snrubs",
            "rdo rje", "vishti",
        ),
        (
            2019,
            3,
            2,
            "1-26",
            "0;13,33",
            "19;22,23",
            "23;11,57",
            "15;34,21",
            "10;7,45",
            "chu stod",
            "dngos grub",
            "byis pa",
        ),
        (
            2019,
            3,
            3,
            "1-27",
            "1;17,45",
            "20;16,41",
            "23;16,27",
            "16;33,9",
            "10;8,43",
            "chu smad",
            "phan tshun",
            "til rdung",
        ),
        (
            2019,
            3,
            4,
            "1-28",
            "2;21,57",
            "21;11,0",
            "23;20,57",
            "17;31,57",
            "10;9,42",
            "gro zhin",
            "mchog can",
            "tshong ba",
        ),
        (
            2019,
            3,
            5,
            "1-29",
            "3;26,8",
            "22;5,18",
            "23;25,26",
            "18;30,44",
            "10;10,40",
            "mon gre",
            "yongs 'joms",
            "bkra shis",
        ),
        (
            2019,
            3,
            6,
            "1-30",
            "4;30,20",
            "22;59,36",
            "23;29,56",
            "19;29,32",
            "10;11,38",
            "mon gre",
            "zhi ba",
            "rkang bzhi",
        ),
    ];

    /// A reading in whole, first and second sexagesimal places, as pala.
    fn pala(text: &str) -> i128 {
        let (whole, rest) = text.split_once(';').unwrap();
        let (first, second) = rest.split_once(',').unwrap();
        whole.parse::<i128>().unwrap() * 3_600
            + first.parse::<i128>().unwrap() * 60
            + second.parse::<i128>().unwrap()
    }

    fn truncated(value: Ratio) -> i128 {
        value.scale(3_600).floor()
    }

    /// The Sun in signs, degrees and minutes as "minutes of arc".
    fn arc_minutes(text: &str) -> i128 {
        let (signs, rest) = text.split_once(';').unwrap();
        let (degrees, minutes) = rest.split_once(',').unwrap();
        (signs.parse::<i128>().unwrap() * 30 + degrees.parse::<i128>().unwrap()) * 60
            + minutes.parse::<i128>().unwrap()
    }

    fn check(reckoning: &TibetanCalendar, rows: &[Row], true_sun_in_signs: bool) -> usize {
        let mut exact = 0;
        for &(y, m, d, date, weekday, moon, sun, yoga, signs, mansion, yoga_name, karana) in rows {
            let day = almanac_day(reckoning, greg(y, m, d)).expect("in range");
            let label = std::format!(
                "{}{}-{}",
                day.date.month.ordinal,
                if day.date.month.leap { "L" } else { "" },
                day.date.day
            );
            assert_eq!(label, date, "{y}-{m}-{d}");
            let end = match day.lunar_day_end {
                Some(end) => truncated(end),
                // The first of two days: the almanac prints `x;60,0`.
                None => (i128::from(day.weekday) + 1) * 3_600,
            };
            let in_signs = if true_sun_in_signs {
                day.sun_in_signs()
            } else {
                day.mean_sun
            };
            let readings = [
                (end, pala(weekday)),
                (truncated(day.moon), pala(moon)),
                (truncated(day.sun), pala(sun)),
                (truncated(day.yoga_longitude), pala(yoga)),
                (in_signs.scale(1_800).floor(), arc_minutes(signs)),
            ];
            // Every reading within one unit of its last printed place;
            // Henning's intermediate truncations, which no source read
            // states, account for the rest.
            for (ours, printed) in readings {
                assert!(
                    (ours - printed).abs() <= 1,
                    "{y}-{m}-{d}: {ours} vs {printed}"
                );
            }
            if readings.iter().all(|(ours, printed)| ours == printed) {
                exact += 1;
            }
            assert_eq!(MANSIONS[usize::from(day.mansion)].1, mansion, "{y}-{m}-{d}");
            if !yoga_name.is_empty() {
                assert_eq!(YOGAS[usize::from(day.yoga)].1, yoga_name, "{y}-{m}-{d}");
                let (_, tibetan) = KARANAS[usize::from(day.karana)];
                let printed = if karana == "vishti" { "viSTi" } else { karana };
                assert_eq!(tibetan, printed, "{y}-{m}-{d}");
            }
        }
        exact
    }

    #[test]
    fn the_columns_are_those_of_hennings_almanacs() {
        assert_eq!(
            check(&TIBETAN_LOCHEN, PHUGPA_2013, false),
            PHUGPA_2013.len()
        );
        // One Bhutanese day's Moon is a pala short: 19 February 2019,
        // 8;17,23 where the almanac prints 8;17,24.
        assert_eq!(
            check(&HENNING_BHUTAN, BHUTAN_2019, false),
            BHUTAN_2019.len() - 1
        );
        let tsurphu = check(&TIBETAN_TSURPHU_KARANA, TSURPHU_2012, true);
        assert!(tsurphu >= TSURPHU_2012.len() * 3 / 4, "{tsurphu}");
    }

    /// The first day of 2013 as Henning's Tsurphu almanac prints it in
    /// "Open source Tsurphu calendar software": "1: Mon.
    /// Shatabhishaj/mon gru …; 11 Feb 2013 / Parigha/yongs 'joms,
    /// Vava/gdab pa … / 2;11,24 23;4,36 22;22,1 18;26,38".
    #[test]
    fn the_tsurphu_programs_worked_day_is_reproduced() {
        let day = almanac_day(&TIBETAN_TSURPHU_KARANA, greg(2013, 2, 11)).expect("in range");
        assert_eq!(day.date.to_string(), "2013-1-1");
        assert_eq!(WEEKDAYS[usize::from(day.weekday)].0, "Monday");
        assert_eq!(
            MANSIONS[usize::from(day.mansion)],
            ("Shatabhishaj", "mon gru")
        );
        assert_eq!(YOGAS[usize::from(day.yoga)], ("Parigha", "yongs 'joms"));
        assert_eq!(KARANAS[usize::from(day.karana)], ("Vava", "gdab pa"));
        assert_eq!(day.lunar_day_end.map(Ratio::sexagesimal), Some((2, 11, 24)));
        assert_eq!(day.sun.sexagesimal(), (22, 22, 1));
        assert_eq!(day.yoga_longitude.sexagesimal(), (18, 26, 38));
        // The Moon a pala over the printed 23;4,36: one of Henning's
        // truncations, as above.
        assert_eq!(truncated(day.moon) - pala("23;4,36"), 1);
    }

    #[test]
    fn the_karanas_follow_the_half_days() {
        // Half-days 1, 58, 59 and 60 have the fixed karaṇas; the seven
        // others run in turn from half-day 2 (Janson, Section 10, item (viii)).
        assert_eq!(KARANAS[usize::from(karana_of_half_day(1))].0, "Kintughna");
        assert_eq!(KARANAS[usize::from(karana_of_half_day(58))].0, "Shakuni");
        assert_eq!(KARANAS[usize::from(karana_of_half_day(59))].0, "Catushpada");
        assert_eq!(KARANAS[usize::from(karana_of_half_day(60))].0, "Naga");
        for half in 2..=57 {
            assert_eq!(karana_of_half_day(half), (half - 2) % 7);
        }
        assert_eq!(KARANAS[usize::from(karana_of_half_day(57))].0, "Vishti");
    }

    /// Henning's worked planets for the 2nd of the leap month 11 of 2010,
    /// Thursday 6 January 2011, JD 2 455 568: "MARS sgos zhag = 525 -
    /// 20;12,3,2,100 - myur: 19;37,33,2,26" ("Open source Tibetan calendar
    /// software").
    #[test]
    fn marss_place_is_hennings_worked_one() {
        let rd = Rd::from_julian_day_number(2_455_568);
        assert_eq!(general_day(rd), 30_596);
        let mars = planet_place(Planet::Mars, rd);
        assert_eq!(mars.particular_day, 525);
        assert_eq!(mars.true_slow.sexagesimal(), (20, 12, 3));
        // The fast longitude comes out one pala short of the printed
        // 19;37,33, which Henning's "general day factors", not described
        // there, may account for.
        assert!((truncated(mars.fast) - pala("19;37,33")).abs() <= 1);
        for planet in Planet::ALL {
            let place = planet_place(planet, rd);
            assert!(place.fast.lt(Ratio::int(27)) && !place.fast.lt(Ratio::int(0)));
        }
    }

    /// Henning's search example: the Buddha's enlightenment as Norzang
    /// Gyatso gives it, "Weekday: 1;38, Moon: 16;0, Sun: 2;30, Rāhu:
    /// 16;29", found on Sunday 17 March 927 BCE ("Open source Tibetan
    /// calendar software"), the 15th of month 4 by the Phugpa arithmetic.
    #[test]
    fn the_enlightenment_day_of_hennings_search_is_reproduced() {
        let rd = hc_calendars_solar::julian::to_fixed(-926, 3, 17).expect("a date");
        // Month 4 of the year beginning in 927 BCE, whose 15th ends on the
        // day, is found by the month count, which the conversion's range of
        // 1000–3000 does not bound.
        let n = TIBETAN_LOCHEN
            .true_month_count(-926, 4, false)
            .expect("a regular month");
        assert_eq!(TIBETAN_LOCHEN.end_day(15, n), rd.to_julian_day_number());
        assert_eq!(TIBETAN_LOCHEN.end_day(14, n), rd.to_julian_day_number() - 1);
        let date = TibetanDate {
            year: -926,
            month: Month::regular(4),
            day: 15,
            leap_day: false,
        };
        assert_eq!(
            WEEKDAYS[(rd.to_julian_day_number() + 2).rem_euclid(7) as usize].0,
            "Sunday"
        );
        let lunar = lunar_day(&TIBETAN_LOCHEN, date.day, n);
        let weekday = lunar.true_date.add(Ratio::int(2));
        let weekday = weekday.sub(Ratio::int(weekday.floor().div_euclid(7) * 7));
        let (whole, nadi, _) = weekday.sexagesimal();
        assert_eq!((whole, nadi), (1, 38));
        // The text's Sun and Moon are to the nāḍī.
        let nearest = |value: Ratio| value.add(Ratio::new(1, 120)).sexagesimal();
        assert_eq!(nearest(lunar.true_sun.scale(27)).0, 2);
        assert_eq!(nearest(lunar.true_sun.scale(27)).1, 30);
        assert_eq!(nearest(lunar.moon.scale(27)).0, 16);
        assert_eq!(nearest(lunar.moon.scale(27)).1, 0);
        let rahu = rahu_head(&TIBETAN_LOCHEN, date.day, n).expect("the Phugpa");
        assert_eq!((rahu.sexagesimal().0, rahu.sexagesimal().1), (16, 29));
        assert!(rahu_head(&TIBETAN_TSURPHU_KARANA, date.day, n).is_err());
    }

    #[test]
    fn the_years_are_named_and_counted_as_janson_gives_them() {
        // Janson, Section 4 and Appendix B.
        assert_eq!(rab_byung_name(2007), ("thams cad 'dul", "sarvajit"));
        assert_eq!(rab_byung_name(1987), ("rab byung", "prabhava"));
        assert_eq!(rab_byung_name(1927), ("rab byung", "prabhava"));
        assert_eq!(rab_byung_name(2024), ("khro mo", "krodhi"));
        assert_eq!(rab_byung_name(2046), ("zad pa", "ksayaka"));
        assert_eq!(
            rab_byung_name(2007),
            RAB_BYUNG_NAMES[hc_calendars_lunar::tibetan::prabhava(2007).1 as usize - 1]
        );
        // The count from 127 BCE: 2130 for 2003 in the Tibetan title of
        // the calendar Janson cites, 2151 for the Wood Dragon year of the
        // Tibetan Nuns Project, 2152 for the Central Tibetan
        // Administration's Wood Snake year.
        assert_eq!(royal_year(2003), 2_130);
        assert_eq!(royal_year(2024), 2_151);
        assert_eq!(royal_year(2025), 2_152);
    }

    fn named(symbol: Symbol) -> (&'static str, bool, &'static str) {
        (
            ELEMENTS[usize::from(symbol.element)],
            symbol.male,
            ANIMALS[usize::from(symbol.animal)],
        )
    }

    /// The constitution of Mongolia came into force "from the horse hour of
    /// the auspicious yellow horse day of the black tiger first spring
    /// month of the water monkey year of the seventeenth 60-year cycle",
    /// 12 February 1992 (Janson, Appendix A.3, citing Sanders and
    /// Bat-Iredüi, p. 240, not read).
    #[test]
    fn the_constitutions_day_month_and_year_have_their_colours() {
        let year = year_symbol(1992);
        assert_eq!(named(year), ("Water", true, "Monkey"));
        assert_eq!(hc_calendars_lunar::tibetan::prabhava(1992).0, 17);
        let month = month_symbol(MonthCycle::Tsurphu, 1992, 1);
        assert_eq!(COLOURS[usize::from(month.element)], "black");
        assert_eq!(ANIMALS[usize::from(month.animal)], "Tiger");
        assert_eq!(mongolian_month(1), (Season::Spring, 0));
        let day = day_symbol(greg(1992, 2, 12));
        assert_eq!(COLOURS[usize::from(day.element)], "yellow");
        assert_eq!(ANIMALS[usize::from(day.animal)], "Horse");
        // "white tiger" for the first month of 1996 (Janson, Appendix A.3,
        // after the same book, p. 242).
        let month = month_symbol(MonthCycle::Tsurphu, 1996, 1);
        assert_eq!(COLOURS[usize::from(month.element)], "white");
        assert_eq!(MONGOLIAN_COLOURS[usize::from(month.element)].0, "цагаан");
        assert_eq!(MONGOLIAN_MONTH_NAMES[0], "Хаврын тэргүүн");
        assert_eq!(mongolian_month(12), (Season::Winter, 2));
        assert_eq!(mongolian_month(5), (Season::Summer, 1));
    }

    /// Henning's almanac headings and solar days: "Tibetan Lunar Month: 1 -
    /// Fire-male-Dragon" in the Phugpa 2013, "1 - Wood-male-Tiger" in the
    /// Tsurphu 2013, "2 - Fire-female-Snake" in the Phugpa, "1 -
    /// Water-male-Dragon" in the Bhutanese 2019, and 11 February 2013's
    /// "Solar: Earth-Monkey".
    #[test]
    fn the_months_and_days_are_named_as_hennings_almanacs_name_them() {
        assert_eq!(
            named(month_symbol(MonthCycle::Phugpa, 2013, 1)),
            ("Fire", true, "Dragon")
        );
        assert_eq!(
            named(month_symbol(MonthCycle::Tsurphu, 2013, 1)),
            ("Wood", true, "Tiger")
        );
        assert_eq!(
            named(month_symbol(MonthCycle::Phugpa, 2013, 2)),
            ("Fire", false, "Snake")
        );
        assert_eq!(
            named(month_symbol(MonthCycle::Phugpa, 2019, 1)),
            ("Water", true, "Dragon")
        );
        assert_eq!(
            named(month_symbol(MonthCycle::Phugpa, 2013, 11)),
            ("Fire", true, "Tiger")
        );
        let day = day_symbol(greg(2013, 2, 11));
        assert_eq!(
            (
                ELEMENTS[usize::from(day.element)],
                ANIMALS[usize::from(day.animal)]
            ),
            ("Earth", "Monkey")
        );
        assert_eq!(named(year_symbol(2007)), ("Fire", false, "Pig"));
    }

    #[test]
    fn the_bhutanese_weekday_is_one_ahead() {
        // Henning: "I am sitting writing this on 5th May 2008. It is a
        // Monday … However, in Bhutan, it is a Tuesday."
        let rd = greg(2008, 5, 5);
        assert_eq!(
            WEEKDAYS[(rd.to_julian_day_number() + 2).rem_euclid(7) as usize].0,
            "Monday"
        );
        assert_eq!(
            WEEKDAYS[usize::from(bhutanese_weekday(rd))],
            ("Tuesday", "mig dmar")
        );
        // His Bhutanese almanac's Losar of 2019, "1: Wed. …; 5 Feb 2019", a
        // Tuesday.
        let losar = greg(2019, 2, 5);
        assert_eq!(TIBETAN_BHUTAN.new_year(2019), Ok(losar));
        assert_eq!(
            WEEKDAYS[usize::from(bhutanese_weekday(losar))].0,
            "Wednesday"
        );
    }

    /// The day and time of the mean Sun's 250°: Henning's Bhutanese
    /// almanacs, "Winter solstice, time: …" in `tdata/bh_2000.txt` to
    /// `bh_2019.txt`, and the Ministry of Home Affairs' lists for 2025 and
    /// 2026, 2 January both [moha-bt-calendar-2025, moha-bt-calendar-2026].
    /// Janson: "It will be 3 January for the first time in 2020."
    #[test]
    fn the_bhutanese_winter_solstice_is_the_mean_suns_250_degrees() {
        let weekday_time = |value: Ratio| {
            let value = value.add(Ratio::int(2));
            value
                .sub(Ratio::int(value.floor().div_euclid(7) * 7))
                .sexagesimal()
        };
        for (year, day, time) in [
            (2001, 1, Some((2, 51, 38))),
            (2002, 2, Some((4, 7, 52))),
            (2011, 2, Some((1, 34, 1))),
            // The page prints 0;35,16, which its own mean-Sun column does
            // not bear out: 249°46′ at the end of the day's lunar day, at
            // 0;37.
            (2016, 2, None),
            (2017, 2, Some((2, 11, 27))),
            (2018, 2, Some((3, 27, 41))),
            (2019, 2, Some((4, 43, 56))),
            (2020, 3, Some((6, 0, 10))),
            (2025, 2, None),
            (2026, 2, None),
        ] {
            let instant = bhutanese_winter_solstice(&TIBETAN_BHUTAN, year).expect("in range");
            assert_eq!(
                Rd::from_julian_day_number(instant.floor() as i64),
                greg(year, 1, day),
                "{year}"
            );
            if let Some(time) = time {
                assert_eq!(weekday_time(instant), time, "{year}");
            }
            assert_eq!(
                bhutanese_winter_solstice(&HENNING_BHUTAN, year),
                Ok(instant)
            );
        }
        let first_third = (2000..=2030).find(|&year| {
            let instant = bhutanese_winter_solstice(&TIBETAN_BHUTAN, year).expect("in range");
            Rd::from_julian_day_number(instant.floor() as i64) == greg(year, 1, 3)
        });
        assert_eq!(first_third, Some(2020));
        assert!(bhutanese_winter_solstice(&TIBETAN_BHUTAN, 3_001).is_err());
    }

    #[test]
    fn a_festival_on_a_skipped_or_repeated_date_follows_each_rule() {
        let reckoning = TIBETAN_LOCHEN;
        // 2024: the 4th of the leap month 6 is repeated, on 9 and 10 July.
        // Berzin's rule keeps it on the first, as the Tibetan Nuns Project
        // kept Chökhor Düchen [tnp-losar]; Henning's almanac marks the
        // second.
        let leap_six = Month::leap(6);
        assert_eq!(
            berzin_day(&reckoning, 2024, leap_six, 4),
            Ok(greg(2024, 7, 9))
        );
        assert_eq!(
            henning_almanac_day(&reckoning, 2024, leap_six, 4),
            Ok(Some(greg(2024, 7, 10)))
        );
        // The month is doubled, and the almanac marks the festival in the
        // regular month 6 too, on 8 August (`tdata/pl_2024.txt`).
        assert_eq!(
            henning_almanac_day(&reckoning, 2024, Month::regular(6), 4),
            Ok(Some(greg(2024, 8, 8)))
        );
        // 1990: the 7th of month 4, the Birth of the Buddha, is skipped.
        // Berzin's rule keeps it on the 6th, 30 May; Henning's almanac
        // marks no day.
        let four = Month::regular(4);
        assert_eq!(berzin_day(&reckoning, 1990, four, 7), Ok(greg(1990, 5, 30)));
        assert_eq!(henning_almanac_day(&reckoning, 1990, four, 7), Ok(None));
        for year in [1966, 1975] {
            assert_eq!(henning_almanac_day(&reckoning, year, four, 7), Ok(None));
        }
        // Where Henning's almanacs mark a repeated date's festival on the
        // second day: the Turning of the Wheel on 17 July 1961, the Birth
        // of the Buddha on 23 May 1988.
        assert_eq!(
            henning_almanac_day(&reckoning, 1961, Month::regular(6), 4),
            Ok(Some(greg(1961, 7, 17)))
        );
        assert_eq!(
            henning_almanac_day(&reckoning, 1988, four, 7),
            Ok(Some(greg(1988, 5, 23)))
        );
        // An ordinary date is the same day by both.
        assert_eq!(
            berzin_day(&reckoning, 2013, four, 15),
            Ok(greg(2013, 5, 25))
        );
        assert_eq!(
            henning_almanac_day(&reckoning, 2013, four, 15),
            Ok(Some(greg(2013, 5, 25)))
        );
        assert!(berzin_day(&reckoning, 999, four, 15).is_err());
        assert_eq!(HENNING_FESTIVALS.len(), 7);
        assert!(
            HENNING_FESTIVALS
                .iter()
                .all(|f| f.day <= 30 && f.month <= 12)
        );
    }

    #[test]
    fn the_name_tables_are_distinct() {
        for names in [&MANSIONS[..], &YOGAS[..], &KARANAS[..]] {
            for (i, a) in names.iter().enumerate() {
                for b in &names[i + 1..] {
                    assert_ne!(a.0, b.0);
                    assert_ne!(a.1, b.1);
                }
            }
        }
    }

    /// A calendar day of Henning's almanacs with the attributes they print
    /// for it: the Gregorian date, the weekday the almanac names it by,
    /// the mansion in Tibetan, the two elements, the Chinese mansion and,
    /// in the Phugpa and Bhutanese, the number after it; then the lunar
    /// day's month, day, animal, trigram and number, `None` on the first of
    /// two days with one number, which prints none.
    type AttributeRow = (
        (i64, u8, u8),
        &'static str,
        &'static str,
        (&'static str, &'static str),
        (&'static str, Option<u8>),
        Option<(u8, u8, &'static str, &'static str, u8)>,
    );

    // Transcribed from `tdata/pl_2013.txt`, month 1, and `tdata/bh_2019.txt`,
    // month 1 [kalacakra-org], read 2026-09-29.
    const PHUGPA_2013_ATTRIBUTES: &[AttributeRow] = &[
        (
            (2013, 2, 11),
            "Mon",
            "mon gre",
            ("Water", "Water"),
            ("Bi", Some(9)),
            Some((1, 1, "Tiger", "kham", 7)),
        ),
        (
            (2013, 2, 12),
            "Tue",
            "mon gru",
            ("Fire", "Earth"),
            ("Zui", Some(1)),
            Some((1, 2, "Rabbit", "gin", 8)),
        ),
        (
            (2013, 2, 13),
            "Wed",
            "khrums stod",
            ("Water", "Fire"),
            ("Can", Some(2)),
            Some((1, 3, "Dragon", "zin", 9)),
        ),
        (
            (2013, 2, 14),
            "Thu",
            "khrums smad",
            ("Wind", "Water"),
            ("Jing", Some(3)),
            Some((1, 4, "Snake", "zon", 1)),
        ),
        (
            (2013, 2, 15),
            "Fri",
            "nam gru",
            ("Earth", "Water"),
            ("Gui", Some(4)),
            Some((1, 5, "Horse", "li", 2)),
        ),
        (
            (2013, 2, 16),
            "Sat",
            "tha skar",
            ("Earth", "Wind"),
            ("Liu", Some(5)),
            Some((1, 6, "Sheep", "khon", 3)),
        ),
        (
            (2013, 2, 17),
            "Sun",
            "bra nye",
            ("Fire", "Fire"),
            ("Xing", Some(6)),
            Some((1, 7, "Monkey", "dwa", 4)),
        ),
        (
            (2013, 2, 18),
            "Mon",
            "bra nye",
            ("Water", "Fire"),
            ("Zhang", Some(7)),
            Some((1, 8, "Bird", "khen", 5)),
        ),
        (
            (2013, 2, 19),
            "Tue",
            "smin drug",
            ("Fire", "Fire"),
            ("Yi", Some(8)),
            Some((1, 9, "Dog", "kham", 6)),
        ),
        (
            (2013, 2, 20),
            "Wed",
            "snar ma",
            ("Water", "Earth"),
            ("Zhen", Some(9)),
            Some((1, 10, "Pig", "gin", 7)),
        ),
        (
            (2013, 2, 21),
            "Thu",
            "mgo",
            ("Wind", "Wind"),
            ("Jiao", Some(1)),
            Some((1, 11, "Mouse", "zin", 8)),
        ),
        (
            (2013, 2, 22),
            "Fri",
            "lag",
            ("Earth", "Water"),
            ("Kang", Some(2)),
            Some((1, 12, "Ox", "zon", 9)),
        ),
        (
            (2013, 2, 23),
            "Sat",
            "nabs so",
            ("Earth", "Wind"),
            ("Di", Some(3)),
            Some((1, 13, "Tiger", "li", 1)),
        ),
        (
            (2013, 2, 24),
            "Sun",
            "rgyal",
            ("Fire", "Fire"),
            ("Fang", Some(4)),
            Some((1, 14, "Rabbit", "khon", 2)),
        ),
        (
            (2013, 2, 25),
            "Mon",
            "skag",
            ("Water", "Water"),
            ("Xin", Some(5)),
            Some((1, 15, "Dragon", "dwa", 3)),
        ),
        (
            (2013, 2, 26),
            "Tue",
            "mchu",
            ("Fire", "Fire"),
            ("Wei", Some(6)),
            Some((1, 16, "Snake", "khen", 4)),
        ),
        (
            (2013, 2, 27),
            "Wed",
            "gre",
            ("Water", "Fire"),
            ("Ji", Some(7)),
            Some((1, 17, "Horse", "kham", 5)),
        ),
        (
            (2013, 2, 28),
            "Thu",
            "dbo",
            ("Wind", "Wind"),
            ("Dou", Some(8)),
            Some((1, 18, "Sheep", "gin", 6)),
        ),
        // The Bhutanese almanac names 5 February 2019, a Tuesday, Wednesday.
        (
            (2019, 2, 5),
            "Wed",
            "gro zhin",
            ("Water", "Earth"),
            ("Zui", Some(7)),
            Some((1, 1, "Tiger", "kham", 7)),
        ),
        (
            (2019, 2, 6),
            "Thu",
            "mon gre",
            ("Wind", "Water"),
            ("Can", Some(8)),
            Some((1, 2, "Rabbit", "gin", 8)),
        ),
        (
            (2019, 2, 7),
            "Fri",
            "mon gru",
            ("Earth", "Earth"),
            ("Jing", Some(9)),
            Some((1, 3, "Dragon", "zin", 9)),
        ),
        (
            (2019, 2, 8),
            "Sat",
            "khrums stod",
            ("Earth", "Fire"),
            ("Gui", Some(1)),
            None,
        ),
        (
            (2019, 2, 9),
            "Sun",
            "khrums smad",
            ("Fire", "Water"),
            ("Liu", Some(2)),
            Some((1, 4, "Snake", "zon", 1)),
        ),
    ];

    // `tdata/ts_2013.txt`, month 1, the Tsurphu's Tiger month, whose
    // Chinese mansion carries no number.
    const TSURPHU_2013_ATTRIBUTES: &[AttributeRow] = &[
        (
            (2013, 2, 11),
            "Mon",
            "mon gru",
            ("Water", "Earth"),
            ("Bi", None),
            Some((1, 1, "Tiger", "li", 1)),
        ),
        (
            (2013, 2, 12),
            "Tue",
            "khrums stod",
            ("Fire", "Fire"),
            ("Zui", None),
            Some((1, 2, "Rabbit", "khon", 2)),
        ),
        (
            (2013, 2, 13),
            "Wed",
            "khrums smad",
            ("Water", "Water"),
            ("Can", None),
            Some((1, 3, "Dragon", "dwa", 3)),
        ),
        (
            (2013, 2, 14),
            "Thu",
            "nam gru",
            ("Wind", "Water"),
            ("Jing", None),
            Some((1, 4, "Snake", "khen", 4)),
        ),
        (
            (2013, 2, 15),
            "Fri",
            "tha skar",
            ("Earth", "Wind"),
            ("Gui", None),
            Some((1, 5, "Horse", "kham", 5)),
        ),
        (
            (2013, 2, 16),
            "Sat",
            "tha skar",
            ("Earth", "Wind"),
            ("Liu", None),
            Some((1, 6, "Sheep", "gin", 6)),
        ),
        (
            (2013, 2, 17),
            "Sun",
            "bra nye",
            ("Fire", "Fire"),
            ("Xing", None),
            Some((1, 7, "Monkey", "zin", 7)),
        ),
    ];

    fn check_attributes(rows: &[AttributeRow], cycle: MonthCycle, year: i64) {
        let weekdays = ["Sat", "Sun", "Mon", "Tue", "Wed", "Thu", "Fri"];
        for &((y, m, d), weekday, mansion, (first, second), (chinese, number), lunar) in rows {
            let rd = greg(y, m, d);
            let weekday = weekdays
                .iter()
                .position(|w| *w == weekday)
                .expect("a weekday") as u8;
            let mansion = MANSIONS
                .iter()
                .position(|(_, tibetan)| *tibetan == mansion)
                .expect("a mansion") as u8;
            let (w, n) = element_pair(weekday, mansion).expect("in range");
            assert_eq!(
                (
                    INDIAN_ELEMENTS[usize::from(w)],
                    INDIAN_ELEMENTS[usize::from(n)]
                ),
                (first, second),
                "{rd:?}"
            );
            assert_eq!(
                CHINESE_MANSIONS[usize::from(chinese_mansion(rd))],
                chinese,
                "{rd:?}"
            );
            if let Some(number) = number {
                assert_eq!(henning_almanac_day_number(rd), number, "{rd:?}");
            }
            if let Some((month, day, animal, trigram, number)) = lunar {
                let attributes = lunar_day_attributes(cycle, year, month, day).expect("in range");
                assert_eq!(ANIMALS[usize::from(attributes.animal)], animal, "{rd:?}");
                assert_eq!(
                    TRIGRAMS[usize::from(attributes.trigram)].tibetan,
                    trigram,
                    "{rd:?}"
                );
                assert_eq!(attributes.number, number, "{rd:?}");
            }
        }
    }

    #[test]
    fn the_attributes_are_those_hennings_almanacs_print() {
        check_attributes(&PHUGPA_2013_ATTRIBUTES[..18], MonthCycle::Phugpa, 2013);
        check_attributes(&PHUGPA_2013_ATTRIBUTES[18..], MonthCycle::Phugpa, 2019);
        check_attributes(TSURPHU_2013_ATTRIBUTES, MonthCycle::Tsurphu, 2013);
        // The Bhutanese almanac's weekday is the Bhutanese one.
        assert_eq!(bhutanese_weekday(greg(2019, 2, 5)), 4);
        // A skipped lunar day's line, "26. Omitted: Rabbit gin 5", in month
        // 1 of 2013; and the doubled month 6 of 2024, whose 4th is "Pig,
        // gin 7" on 10 July and on 8 August (`tdata/pl_2024.txt`).
        let omitted = lunar_day_attributes(MonthCycle::Phugpa, 2013, 1, 26).expect("in range");
        assert_eq!(
            (
                ANIMALS[usize::from(omitted.animal)],
                TRIGRAMS[usize::from(omitted.trigram)].tibetan,
                omitted.number
            ),
            ("Rabbit", "gin", 5)
        );
        let doubled = lunar_day_attributes(MonthCycle::Phugpa, 2024, 6, 4).expect("in range");
        assert_eq!(
            (
                ANIMALS[usize::from(doubled.animal)],
                TRIGRAMS[usize::from(doubled.trigram)].tibetan,
                doubled.number
            ),
            ("Pig", "gin", 7)
        );
    }

    #[test]
    fn the_lunar_day_attributes_follow_jansons_rules() {
        // Janson, Appendix E: "months Tiger, Horse, Dog begin with Li;
        // Rabbit, Sheep, Pig begin with Zin; Mouse, Dragon, Monkey begin
        // with Kham; Ox, Snake, Bird begin with Dwa", and "Tiger, Snake,
        // Monkey, Pig begin with 1 (white); Mouse, Rabbit, Horse, Bird
        // begin with 4 (green); Ox, Dragon, Sheep, Dog begin with 7 (red)".
        let trigrams = [
            "kham", "dwa", "li", "zin", "kham", "dwa", "li", "zin", "kham", "dwa", "li", "zin",
        ];
        let numbers = [4, 7, 1, 4, 7, 1, 4, 7, 1, 4, 7, 1];
        for month in 1..=12 {
            let animal = month_symbol(MonthCycle::Tsurphu, 2013, month).animal;
            let first =
                lunar_day_attributes(MonthCycle::Tsurphu, 2013, month, 1).expect("in range");
            assert_eq!(
                TRIGRAMS[usize::from(first.trigram)].tibetan,
                trigrams[usize::from(animal)]
            );
            assert_eq!(first.number, numbers[usize::from(animal)]);
            // An odd month begins with the Tiger, an even one with the
            // Monkey, and the element is the month's advanced by the day.
            assert_eq!(first.animal, if month % 2 == 1 { 2 } else { 8 });
            let symbol = month_symbol(MonthCycle::Tsurphu, 2013, month);
            assert_eq!(first.element, (symbol.element + 1) % 5);
        }
        assert_eq!(NINE_NUMBERS[0].colour, "white");
        assert_eq!(NINE_NUMBERS[3].colour, "green");
        assert_eq!(NINE_NUMBERS[6].colour, "red");
        assert_eq!(lunar_day_attributes(MonthCycle::Phugpa, 2013, 0, 1), None);
        assert_eq!(lunar_day_attributes(MonthCycle::Phugpa, 2013, 13, 1), None);
        assert_eq!(lunar_day_attributes(MonthCycle::Phugpa, 2013, 1, 0), None);
        assert_eq!(lunar_day_attributes(MonthCycle::Phugpa, 2013, 1, 31), None);
        assert_eq!(element_pair(7, 0), None);
        assert_eq!(element_pair(0, 27), None);
    }

    #[test]
    fn the_calendar_days_numbers_run_both_ways() {
        // Janson's (−JD) amod 9 and his report of Henning's book, "10 −
        // ((JD + 1) amod 9)" (Remark 36), are one rule, one less each day;
        // Henning's almanacs print one more each day.
        for jdn in 2_456_300..2_456_400_i64 {
            let rd = Rd::from_julian_day_number(jdn);
            let book = 10 - ((jdn + 1 - 1).rem_euclid(9) + 1);
            assert_eq!(i64::from(janson_day_number(rd)), book);
            let next = Rd::from_julian_day_number(jdn + 1);
            assert_eq!(janson_day_number(next) % 9 + 1, janson_day_number(rd));
            assert_eq!(
                henning_almanac_day_number(rd) % 9 + 1,
                henning_almanac_day_number(next)
            );
            assert_eq!(day_trigram(next), (day_trigram(rd) + 1) % 8);
        }
        // 11 February 2013 is 9 in the almanac and 8 by Janson's rule.
        assert_eq!(henning_almanac_day_number(greg(2013, 2, 11)), 9);
        assert_eq!(janson_day_number(greg(2013, 2, 11)), 8);
    }
}
