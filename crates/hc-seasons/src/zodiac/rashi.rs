//! राशि — the rāśi as a *month*, and the Indian solar calendars that count
//! months that way.
//!
//! A [`SiderealSign`](super::SiderealSign) is not only a sign; it is a month.
//! The solar calendars of Tamil Nadu, Bengal, Assam, Odisha and Kerala define
//! a month as the interval the Sun spends in one rāśi, so their months begin
//! at a **saṅkrānti** and are 29 to 32 days long — unequal, because a 30° arc
//! of an elliptical orbit is. That is the whole calendar rule; everything
//! else regional is naming and where the year is taken to start.
//!
//! So this module adds no arithmetic. [`Rashi`] is a type alias for
//! [`SiderealSign`](super::SiderealSign), the boundaries are
//! [`super::sidereal`]'s, and what is here is the month names in four
//! traditions and the two saṅkrānti that are national festivals.
//!
//! # What this is not
//!
//! **Not a `Calendar` implementation.** A Tamil or Bengali date is a year, a
//! month and a day, and the year number, the epoch (Kollam, Bengali San,
//! Śaka) and the rule that assigns a day to a month when a saṅkrānti falls
//! late in the day are calendar matters. They belong in
//! `hc-calendars-regional`. This module names the months and finds their
//! boundaries; it does not number the days or the years.
//!
//! **Not the lunisolar Indian calendar.** The Hindu calendars of most of
//! northern India are lunisolar: their months are named after the *nakṣatra*
//! of the full moon and begin at a new or full moon, not at a saṅkrānti. The
//! national civil calendar of India (the reformed Śaka calendar, CLDR
//! `indian`) is a third thing again, with fixed month lengths tied to the
//! tropical equinox. None of those are here.
//!
//! **Not the festival calendar.** A saṅkrānti that falls after sunset is
//! often observed the next day, and the rules for that (*puṇya kāla*) differ
//! by region and by festival. This module gives the astronomical day.

use hc_calendar::Rd;
use hc_calendar::fixed::Moment;

use crate::meridian::Meridian;
use crate::zodiac::SignPeriod;
use crate::zodiac::sidereal::SiderealSign;
use crate::zodiac::sidereal::{self, Ayanamsa};

/// A rāśi: a sidereal sign, and so also an Indian solar month.
///
/// The alias exists because the two words name one thing seen from two sides,
/// and a reader looking for "rāśi" should not have to know that this crate
/// filed it under the sidereal zodiac.
pub type Rashi = SiderealSign;

/// Which regional naming of the twelve solar months to read.
///
/// The months are the same twelve intervals in every row; only the names
/// differ. The traditions also disagree about which month opens the year,
/// which [`Self::year_opening_month`] gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SolarMonthTradition {
    /// The Sanskrit rāśi names themselves: Meṣa, Vṛṣabha, and so on.
    ///
    /// Used as month names in the Odia and, with variations, the Assamese
    /// reckoning, and as the neutral reference everywhere else.
    Sanskrit,
    /// The Tamil months: Chithirai, Vaigasi, Aani, and so on.
    ///
    /// The Tamil year opens with Chithirai at the Meṣa saṅkrānti in April —
    /// the festival of Puthandu.
    Tamil,
    /// The Bengali months of the solar Bangabda: Boishakh, Jyoishtho, and so
    /// on, as romanised in Bangladesh and West Bengal.
    ///
    /// The year opens with Boishakh at the Meṣa saṅkrānti — Pohela Boishakh.
    /// Bangladesh fixed its version of these months to the Gregorian calendar
    /// by statute in 1966 and again in 2019, so the *Bangladeshi* civil
    /// months are no longer the sidereal ones; the West Bengal reckoning
    /// still is, and it is the one described here.
    Bengali,
    /// The Malayalam months of the Kollam era: Medam, Edavam, and so on.
    ///
    /// The Kollam year opens not at Meṣa but with Chingam, the Siṃha
    /// saṅkrānti in August, which is why the Malayalam new year (Vishu is the
    /// Meṣa one, Chingam 1 the calendrical one) is the odd row out here.
    Malayalam,
}

impl SolarMonthTradition {
    /// All four, Sanskrit first.
    pub const ALL: [Self; 4] = [Self::Sanskrit, Self::Tamil, Self::Bengali, Self::Malayalam];

    /// The name of the tradition in English.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Sanskrit => "Sanskrit",
            Self::Tamil => "Tamil",
            Self::Bengali => "Bengali",
            Self::Malayalam => "Malayalam",
        }
    }

    /// The rāśi whose month opens the year in this tradition.
    ///
    /// Meṣa for the Sanskrit, Tamil and Bengali reckonings — mid-April — and
    /// Siṃha for the Malayalam one, in mid-August.
    #[must_use]
    pub const fn year_opening_month(self) -> Rashi {
        match self {
            Self::Malayalam => SiderealSign::SIMHA,
            _ => SiderealSign::MESHA,
        }
    }
}

/// The month-name table, indexed by rāśi.
struct MonthNames {
    tamil: &'static str,
    bengali: &'static str,
    malayalam: &'static str,
}

/// The regional month names, in the Latin romanisations these calendars are
/// usually printed in outside their own scripts.
///
/// Sources: the Tamil Nadu government's almanac month order, the West Bengal
/// Bangabda month order, and the Kerala Kollam-era month order, each of which
/// is a fixed list that every published panchāṅga agrees about. The Sanskrit
/// column is not repeated here because it is
/// [`SiderealSign::sanskrit_name`](super::SiderealSign::sanskrit_name).
///
/// Several of the Malayalam names are transparently the Sanskrit rāśi name
/// with a Malayalam ending — Mithunam, Karkadakam, Thulam, Vrischikam,
/// Makaram, Kumbham, Meenam — which is the clearest single piece of evidence
/// that these months *are* the rāśi and not a parallel scheme.
const MONTH_NAMES: [MonthNames; crate::zodiac::SIGNS_PER_ZODIAC] = [
    MonthNames {
        tamil: "Chithirai",
        bengali: "Boishakh",
        malayalam: "Medam",
    },
    MonthNames {
        tamil: "Vaigasi",
        bengali: "Jyoishtho",
        malayalam: "Edavam",
    },
    MonthNames {
        tamil: "Aani",
        bengali: "Ashar",
        malayalam: "Mithunam",
    },
    MonthNames {
        tamil: "Aadi",
        bengali: "Shrabon",
        malayalam: "Karkadakam",
    },
    MonthNames {
        tamil: "Aavani",
        bengali: "Bhadro",
        malayalam: "Chingam",
    },
    MonthNames {
        tamil: "Purattasi",
        bengali: "Ashwin",
        malayalam: "Kanni",
    },
    MonthNames {
        tamil: "Aippasi",
        bengali: "Kartik",
        malayalam: "Thulam",
    },
    MonthNames {
        tamil: "Kaarthigai",
        bengali: "Ogrohayon",
        malayalam: "Vrischikam",
    },
    MonthNames {
        tamil: "Margazhi",
        bengali: "Poush",
        malayalam: "Dhanu",
    },
    MonthNames {
        tamil: "Thai",
        bengali: "Magh",
        malayalam: "Makaram",
    },
    MonthNames {
        tamil: "Maasi",
        bengali: "Falgun",
        malayalam: "Kumbham",
    },
    MonthNames {
        tamil: "Panguni",
        bengali: "Choitro",
        malayalam: "Meenam",
    },
];

/// The name of a rāśi as a month in a given tradition.
///
/// ```
/// use hc_seasons::zodiac::{SiderealSign, SolarMonthTradition, rashi::month_name};
///
/// assert_eq!(month_name(SiderealSign::MESHA, SolarMonthTradition::Tamil), "Chithirai");
/// assert_eq!(month_name(SiderealSign::MESHA, SolarMonthTradition::Bengali), "Boishakh");
/// assert_eq!(month_name(SiderealSign::MESHA, SolarMonthTradition::Sanskrit), "Meṣa");
/// ```
#[must_use]
pub const fn month_name(rashi: Rashi, tradition: SolarMonthTradition) -> &'static str {
    let index = rashi.index() as usize;
    match tradition {
        SolarMonthTradition::Sanskrit => rashi.sanskrit_name(),
        SolarMonthTradition::Tamil => MONTH_NAMES[index].tamil,
        SolarMonthTradition::Bengali => MONTH_NAMES[index].bengali,
        SolarMonthTradition::Malayalam => MONTH_NAMES[index].malayalam,
    }
}

/// The position of a rāśi in a tradition's year, counting its opening month
/// as 1.
///
/// Meṣa is month 1 in the Tamil and Bengali reckonings and month 9 in the
/// Malayalam one, because the Kollam year opens at Siṃha.
#[must_use]
pub const fn month_number(rashi: Rashi, tradition: SolarMonthTradition) -> u8 {
    let opening = tradition.year_opening_month().index();
    (rashi.index() + 12 - opening) % 12 + 1
}

/// The Universal Time instant of a saṅkrānti: the Sun's entry into a rāśi,
/// and so the start of the corresponding solar month.
///
/// The Gregorian year the saṅkrānti falls in, not the Indian year, because
/// this crate does not number Indian years. See
/// [`super::sidereal::ingress_moment`] for the once-a-year guarantee and
/// where it lapses.
#[must_use]
pub fn sankranti_moment(year: i64, rashi: Rashi, ayanamsa: Ayanamsa) -> Moment {
    sidereal::ingress_moment(year, rashi, ayanamsa)
}

/// The day a saṅkrānti falls on, at a given meridian.
///
/// For Indian dates that meridian is [`Meridian::INDIA`], which is the
/// 82°30′E meridian Indian Standard Time is defined at and the one the
/// *Indian Astronomical Ephemeris* computes at.
#[must_use]
pub fn sankranti_day(year: i64, rashi: Rashi, ayanamsa: Ayanamsa, meridian: Meridian) -> Rd {
    sidereal::ingress_day(year, rashi, ayanamsa, meridian)
}

/// The solar month a day falls in.
#[must_use]
pub fn month_on_day(day: Rd, ayanamsa: Ayanamsa, meridian: Meridian) -> Rashi {
    sidereal::sign_on_day(day, ayanamsa, meridian)
}

/// The full period of the solar month a day falls in.
#[must_use]
pub fn month_in_effect(day: Rd, ayanamsa: Ayanamsa, meridian: Meridian) -> SignPeriod<Rashi> {
    sidereal::sign_in_effect(day, ayanamsa, meridian)
}

/// The day of Makara Saṅkrānti in a Gregorian year: the Sun's entry into
/// Makara, around 14 January.
///
/// The most widely kept of the twelve, marked as Pongal in Tamil Nadu, Magh
/// Bihu in Assam, Maghi in Punjab and Uttarayan in Gujarat. It is nearly
/// fixed against the Gregorian calendar over a human lifetime and not at all
/// fixed over centuries: precession moves it about a day every seventy
/// years, so it was in late December in the twelfth century.
#[must_use]
pub fn makara_sankranti(year: i64, ayanamsa: Ayanamsa, meridian: Meridian) -> Rd {
    sankranti_day(year, SiderealSign::MAKARA, ayanamsa, meridian)
}

/// The day of Meṣa Saṅkrānti in a Gregorian year: the Sun's entry into Meṣa,
/// around 14 April.
///
/// The solar new year of the Tamil, Bengali, Assamese, Odia and Punjabi
/// reckonings — Puthandu, Pohela Boishakh, Bohag Bihu, Pana Sankranti,
/// Vaisakhi — and the Malayalam festival of Vishu, which is a festival there
/// without being the start of the Kollam year.
#[must_use]
pub fn mesha_sankranti(year: i64, ayanamsa: Ayanamsa, meridian: Meridian) -> Rd {
    sankranti_day(year, SiderealSign::MESHA, ayanamsa, meridian)
}

/// The twelve solar months of a Gregorian year, in date order.
///
/// This is [`super::sidereal::signs_in_year`] under its calendrical name.
#[must_use]
pub fn months_in_year(
    year: i64,
    ayanamsa: Ayanamsa,
    meridian: Meridian,
) -> sidereal::SiderealSignsInYear {
    sidereal::signs_in_year(year, ayanamsa, meridian)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::year_month_day_from_rd;

    const INDIA: Meridian = Meridian::INDIA;
    const LAHIRI: Ayanamsa = Ayanamsa::LAHIRI;

    #[test]
    fn every_rashi_has_a_month_name_in_every_tradition() {
        for rashi in SiderealSign::ALL {
            for tradition in SolarMonthTradition::ALL {
                let name = month_name(rashi, tradition);
                assert!(
                    !name.is_empty(),
                    "{} has no {} name",
                    rashi.sanskrit_name(),
                    tradition.english_name()
                );
            }
        }
    }

    #[test]
    fn the_month_names_of_a_tradition_are_all_different() {
        for tradition in SolarMonthTradition::ALL {
            for (index, rashi) in SiderealSign::ALL.into_iter().enumerate() {
                for other in &SiderealSign::ALL[index + 1..] {
                    assert_ne!(
                        month_name(rashi, tradition),
                        month_name(*other, tradition),
                        "{} repeats a name",
                        tradition.english_name()
                    );
                }
            }
        }
    }

    /// Seven of the twelve Malayalam month names are the Sanskrit rāśi name
    /// with a Malayalam ending, which is what makes it obvious that these
    /// months are the rāśi rather than a parallel scheme that happens to
    /// line up.
    #[test]
    fn the_malayalam_months_are_mostly_the_sanskrit_rashi_names() {
        let recognisable = [
            SiderealSign::MITHUNA,
            SiderealSign::KARKA,
            SiderealSign::TULA,
            SiderealSign::VRISHCHIKA,
            SiderealSign::MAKARA,
            SiderealSign::KUMBHA,
            SiderealSign::MINA,
        ];
        for rashi in recognisable {
            let malayalam = month_name(rashi, SolarMonthTradition::Malayalam);
            assert!(
                malayalam.ends_with('m') || malayalam.ends_with("am"),
                "{malayalam} does not look like a Sanskrit borrowing"
            );
        }
        assert_eq!(
            month_name(SiderealSign::MAKARA, SolarMonthTradition::Malayalam),
            "Makaram"
        );
        assert_eq!(
            month_name(SiderealSign::MINA, SolarMonthTradition::Malayalam),
            "Meenam"
        );
    }

    /// Three traditions open the year at Meṣa in April; Kerala opens it at
    /// Siṃha in August, so the same month carries two different numbers.
    #[test]
    fn the_traditions_disagree_about_which_month_opens_the_year() {
        assert_eq!(
            SolarMonthTradition::Tamil.year_opening_month(),
            SiderealSign::MESHA
        );
        assert_eq!(
            SolarMonthTradition::Bengali.year_opening_month(),
            SiderealSign::MESHA
        );
        assert_eq!(
            SolarMonthTradition::Malayalam.year_opening_month(),
            SiderealSign::SIMHA
        );
        assert_eq!(
            month_number(SiderealSign::MESHA, SolarMonthTradition::Tamil),
            1
        );
        assert_eq!(
            month_number(SiderealSign::MESHA, SolarMonthTradition::Malayalam),
            9
        );
        assert_eq!(
            month_number(SiderealSign::SIMHA, SolarMonthTradition::Malayalam),
            1
        );
    }

    #[test]
    fn every_tradition_numbers_its_months_one_to_twelve_exactly_once() {
        for tradition in SolarMonthTradition::ALL {
            let mut seen = [false; 12];
            for rashi in SiderealSign::ALL {
                let number = month_number(rashi, tradition);
                assert!((1..=12).contains(&number));
                assert!(!seen[(number - 1) as usize]);
                seen[(number - 1) as usize] = true;
            }
            assert!(seen.iter().all(|flag| *flag));
            assert_eq!(
                month_number(tradition.year_opening_month(), tradition),
                1,
                "{} did not number its own first month 1",
                tradition.english_name()
            );
        }
    }

    /// Both national saṅkrānti are anchored to the middle of a Gregorian
    /// month and stay there for a human lifetime.
    #[test]
    fn the_two_festival_sankranti_land_in_mid_january_and_mid_april() {
        for year in 2015..=2030 {
            let makara = year_month_day_from_rd(makara_sankranti(year, LAHIRI, INDIA));
            assert_eq!(makara.0, year);
            assert_eq!(makara.1, 1);
            assert!(
                (14..=15).contains(&makara.2),
                "Makara Sankranti {year} on January {}",
                makara.2
            );

            let mesha = year_month_day_from_rd(mesha_sankranti(year, LAHIRI, INDIA));
            assert_eq!(mesha.0, year);
            assert_eq!(mesha.1, 4);
            assert!(
                (13..=15).contains(&mesha.2),
                "Mesha Sankranti {year} on April {}",
                mesha.2
            );
        }
    }

    /// Solar months are unequal by construction, and the spread is the
    /// eccentricity of the Earth's orbit showing through a calendar: the
    /// months crossed near aphelion in July are three days longer than those
    /// crossed near perihelion in January.
    #[test]
    fn the_solar_months_are_of_unequal_length() {
        let mut shortest = 99i64;
        let mut longest = 0i64;
        let mut total = 0i64;
        for period in months_in_year(2024, LAHIRI, INDIA) {
            let length = period.length_days();
            assert!(
                (29..=32).contains(&length),
                "{} ran {length} days",
                month_name(period.sign, SolarMonthTradition::Tamil)
            );
            shortest = shortest.min(length);
            longest = longest.max(length);
            total += length;
        }
        assert!(
            longest - shortest >= 2,
            "the months varied by only {} days",
            longest - shortest
        );
        assert!(
            (365..=366).contains(&total),
            "the twelve months covered {total} days"
        );
    }

    #[test]
    fn a_day_belongs_to_the_month_whose_period_contains_it() {
        let start = crate::gregorian::from_year_month_day(2024, 1, 1);
        for offset in (0..366).step_by(5) {
            let day = Rd(start.0 + offset);
            let period = month_in_effect(day, LAHIRI, INDIA);
            assert!(period.contains(day));
            assert_eq!(month_on_day(day, LAHIRI, INDIA), period.sign);
        }
    }

    /// A month begins at its own saṅkrānti: the calendrical and the
    /// astronomical entry points are the same function under two names.
    #[test]
    fn a_month_begins_at_its_own_sankranti() {
        for rashi in SiderealSign::ALL {
            let day = sankranti_day(2024, rashi, LAHIRI, INDIA);
            assert_eq!(month_on_day(day, LAHIRI, INDIA), rashi);
            assert_eq!(month_on_day(Rd(day.0 - 1), LAHIRI, INDIA), rashi.previous());
            let moment = sankranti_moment(2024, rashi, LAHIRI);
            assert!((moment.0 - sidereal::ingress_moment(2024, rashi, LAHIRI).0).abs() < 1e-12);
        }
    }

    /// The Tamil and Bengali new year are the same instant under two names,
    /// which they have to be, because both are the Meṣa saṅkrānti.
    #[test]
    fn the_tamil_and_bengali_new_years_are_the_same_day() {
        for year in 2015..=2030 {
            let day = mesha_sankranti(year, LAHIRI, INDIA);
            assert_eq!(month_on_day(day, LAHIRI, INDIA), SiderealSign::MESHA);
            assert_eq!(
                month_name(SiderealSign::MESHA, SolarMonthTradition::Tamil),
                "Chithirai"
            );
            assert_eq!(
                month_name(SiderealSign::MESHA, SolarMonthTradition::Bengali),
                "Boishakh"
            );
        }
    }

    /// The Kollam year opens in August, so the Malayalam first month is four
    /// months adrift of the Tamil one.
    #[test]
    fn the_kollam_year_opens_in_august() {
        let day = sankranti_day(2024, SiderealSign::SIMHA, LAHIRI, INDIA);
        let (year, month, _) = year_month_day_from_rd(day);
        assert_eq!(year, 2024);
        assert_eq!(month, 8);
        assert_eq!(
            month_name(SiderealSign::SIMHA, SolarMonthTradition::Malayalam),
            "Chingam"
        );
    }

    /// A different ayanamsa gives a different calendar: the Raman anchor is
    /// about 1.45° behind Lahiri, so every month begins a day or two earlier,
    /// and on some days the two disagree about which month it is.
    #[test]
    fn the_choice_of_ayanamsa_changes_the_month_boundaries() {
        let mut moved = 0;
        for rashi in SiderealSign::ALL {
            let lahiri = sankranti_day(2024, rashi, LAHIRI, INDIA);
            let raman = sankranti_day(2024, rashi, Ayanamsa::RAMAN, INDIA);
            assert!(raman.0 <= lahiri.0);
            if raman != lahiri {
                moved += 1;
            }
        }
        assert_eq!(
            moved, 12,
            "a 1.45 degree shift should move every boundary by at least a day"
        );
    }
}
