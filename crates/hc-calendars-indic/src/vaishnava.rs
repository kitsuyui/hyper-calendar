//! The Vaiṣṇava reading of a festival's day, where it parts from the Smārta.
//!
//! Hindu festivals are tithis, and where the tithi does not coincide with a
//! civil day the sects decide the day differently. The Smārta reading, the
//! one the *Rashtriya Panchang* lists, takes the day that holds the part of
//! the day the rite belongs to (`hc-holiday`'s `JANMASHTAMI` is Śrāvaṇa kṛṣṇa
//! 8 at midnight). The Vaiṣṇava reading takes the day the tithi holds at
//! sunrise: Drik Panchang's ISKCON dates for Krishna Janmashtami, "Ashtami
//! Tithi and Rohini Nakshatra" preferred and never Saptami, always fall on
//! an Ashtami or Navami day, and the first sunrise of Śrāvaṇa kṛṣṇa at or
//! after Ashtami reproduces all eleven of them, 2024 to 2034
//! (`drik-iskcon-janmashtami`, for Tokyo). That is a reading fitted to
//! those dates; no source read states it as a rule, and the Rohini
//! preference is not modelled.
//!
//! The Vaiṣṇava Ekadashi is another reading: Drik Panchang says Vaiṣṇavas
//! "should fast only on Ekadashi mixed with Dwadashi" and break the fast
//! before Trayodashi, and its ISKCON list follows "same Ekadasi and Parana
//! rules as those followed by GCal" (`drik-vidhi-vidhan`,
//! `drik-iskcon-ekadashi`). GCal's rules were not read, so no function here
//! gives the Vaiṣṇava Ekadashi: the measured relation of the 2025 list to
//! the Smārta day is in `docs/systems/hindu-calendars.md`, and a test of
//! this module holds it.
//!
//! # The two readings under their own names
//!
//! The Smārta and the Vaiṣṇava reading of a festival are two conventions
//! (`docs/policy.md` §5), each under its own identifier, `smarta` and
//! `vaishnava`, in [`FestivalReading`]. For Kṛṣṇa Janmāṣṭamī, the festival
//! the sources read give both days of, [`janmashtami_by`] reads either of
//! them. `hc-holiday` carries the Smārta day as its `JANMASHTAMI` rule,
//! which this module's [`smarta_janmashtami`] states for any place, and the
//! national table's other readings are not touched: its tithi rules fall
//! back to the day *before* a skipped tithi, this reading to the day after.

use hc_calendar::{CalendarError, CalendarResult, Rd};

use crate::hindu_lunar::{GREGORIAN_YEAR_OFFSET, HinduLunarCalendar};
use crate::tithi::{Prevalence, TITHIS_PER_MONTH};

/// A reading of a festival's day where the sects part: how the tithi that
/// does not coincide with a civil day is placed on one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FestivalReading {
    /// The identifier, `smarta` or `vaishnava`.
    pub id: &'static str,
    /// The reading's English name.
    pub english_name: &'static str,
    /// How the day is taken, in words.
    pub rule: &'static str,
    /// Where the reading is from.
    pub source: &'static str,
}

hc_core::catalogue! {
    type: FestivalReading,
    id: |reading| reading.id,
    provenance: |reading| reading.source,
    tests: festival_reading_catalogue_tests,
    associated;

    /// The two readings, the Smārta first.
    pub const ALL;
    /// The reading with this identifier.
    pub fn by_id;

    entries: {
        /// The Smārta reading, the one the *Rashtriya Panchang* lists: the
        /// day on which the tithi holds the part of the day the rite belongs
        /// to, Janmāṣṭamī's being the night, at its middle, and the later of
        /// two such days.
        pub const SMARTA = Self {
            id: "smarta",
            english_name: "Smarta",
            rule: "the day whose midnight holds the tithi, the later of two; for a tithi that holds \
                   no midnight, the day that carries it at sunrise",
            source: "The Rashtriya Panchang's list of principal festivals, Saka 1945 and 1946 \
                     (rashtriya-panchang-1946), which hc-holiday's JANMASHTAMI reproduces; the \
                     part of the day is the library's reading of the dharmasastra, not read in \
                     Kane or the Nirnayasindhu",
        };
        /// The Vaiṣṇava reading: the day whose sunrise carries the tithi,
        /// and the day after when no sunrise does.
        pub const VAISHNAVA = Self {
            id: "vaishnava",
            english_name: "Vaishnava",
            rule: "the first day whose sunrise carries the tithi or a later one",
            source: "Drik Panchang's ISKCON Janmashtami dates for Tokyo, 2024 to 2034 \
                     (drik-iskcon-janmashtami), which this reading reproduces in every year; no \
                     source read states it as a rule, and the Rohini preference is not modelled",
        };
    }
}

/// The month of Krṣṇa Janmāṣṭamī in the amānta calendar: Śrāvaṇa.
pub const JANMASHTAMI_MONTH: u8 = 5;

/// Ashtami of the dark fortnight, Śrāvaṇa kṛṣṇa 8, as a tithi number.
pub const JANMASHTAMI_TITHI: u8 = 23;

/// The first day of the ordinary month `month` of a Śaka year whose sunrise
/// carries `tithi` or a later one: the Vaiṣṇava day of a tithi, where the
/// tithi that holds a sunrise is the day and one that holds none is kept on
/// the next.
///
/// # Errors
///
/// As [`HinduLunarCalendar::month_span`] for a month the year does not
/// have, and [`CalendarError::DayOutOfRange`] for a tithi outside 1 to 30.
pub fn day_of(
    lunar: &HinduLunarCalendar,
    saka_year: i64,
    month: u8,
    tithi: u8,
) -> CalendarResult<Rd> {
    if tithi == 0 || tithi > TITHIS_PER_MONTH {
        return Err(CalendarError::DayOutOfRange);
    }
    let (first, next) = lunar.month_span(saka_year, month, false)?;
    let mut day = first;
    while day < next {
        if lunar.from_fixed(day)?.day >= tithi {
            return Ok(day);
        }
        day = Rd(day.0 + 1);
    }
    Err(CalendarError::DayOutOfRange)
}

/// The Vaiṣṇava Krṣṇa Janmāṣṭamī of a Śaka year: Śrāvaṇa kṛṣṇa 8 at
/// sunrise, or the day after when the eighth tithi holds none.
///
/// # Errors
///
/// As [`day_of`].
pub fn janmashtami(lunar: &HinduLunarCalendar, saka_year: i64) -> CalendarResult<Rd> {
    day_of(lunar, saka_year, JANMASHTAMI_MONTH, JANMASHTAMI_TITHI)
}

/// The Smārta Krṣṇa Janmāṣṭamī of a Śaka year at a place: the later of the
/// days whose midnight holds Śrāvaṇa kṛṣṇa 8, and where none does the day
/// that carries the tithi at sunrise, or the day a skipped tithi falls in.
///
/// This is `hc-holiday`'s `JANMASHTAMI` read at any calendar's place, the
/// rule that crate evaluates with a memo over a year's festivals and this
/// function states once for one festival; a test of the facade holds them
/// equal.
///
/// # Errors
///
/// As [`HinduLunarCalendar::month_span`], and
/// [`CalendarError::DayOutOfRange`] where no day carries the tithi.
pub fn smarta_janmashtami(lunar: &HinduLunarCalendar, saka_year: i64) -> CalendarResult<Rd> {
    let (first, next) = lunar.month_span(saka_year, JANMASHTAMI_MONTH, false)?;
    // The first tithi begins at the new moon, which can fall after the
    // sunrise of the day before the month's first, so the scan opens there
    // and runs to the month's end.
    let low = Rd(first.0 - 1);
    let mut later = None;
    let mut day = low;
    while day <= next {
        if Prevalence::Midnight.tithi_on(day, lunar.location) == JANMASHTAMI_TITHI {
            later = Some(day);
        }
        day = Rd(day.0 + 1);
    }
    if let Some(day) = later {
        return Ok(day);
    }
    let mut day = low;
    while day <= next {
        let at_sunrise = Prevalence::Sunrise.tithi_on(day, lunar.location);
        if at_sunrise == JANMASHTAMI_TITHI {
            return Ok(day);
        }
        // A tithi no sunrise carries falls in the day of the sunrise before
        // it, the next sunrise carrying the one after.
        if at_sunrise + 1 == JANMASHTAMI_TITHI
            && Prevalence::Sunrise.tithi_on(Rd(day.0 + 1), lunar.location) == JANMASHTAMI_TITHI + 1
        {
            return Ok(day);
        }
        day = Rd(day.0 + 1);
    }
    Err(CalendarError::DayOutOfRange)
}

/// The day of Kṛṣṇa Janmāṣṭamī in the Gregorian year `year` by a reading:
/// the Śaka year the Gregorian one less 78 holds Śrāvaṇa in August or
/// September.
///
/// # Errors
///
/// As [`smarta_janmashtami`] and [`janmashtami`].
pub fn janmashtami_by(
    reading: FestivalReading,
    lunar: &HinduLunarCalendar,
    year: i64,
) -> CalendarResult<Rd> {
    let saka = year - GREGORIAN_YEAR_OFFSET;
    if reading == FestivalReading::SMARTA {
        smarta_janmashtami(lunar, saka)
    } else {
        janmashtami(lunar, saka)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_astro::riseset::Location;
    use hc_calendars_solar::gregorian;
    use hc_seasons::zodiac::Ayanamsa;

    /// Tokyo, the place Drik Panchang's ISKCON page is set for.
    const TOKYO: Location = Location::new(35.6894, 139.6917, 0.0);
    const NEW_DELHI: Location = Location::new(28.6356, 77.2244, 0.0);

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    /// Drik Panchang's "ISKCON Janmashtami Dates (2024-2034)" for Tokyo
    /// (`drik-iskcon-janmashtami`, read 2026-10-03).
    const DRIK_ISKCON_JANMASHTAMI: [(i64, u8, u8); 11] = [
        (2024, 8, 27),
        (2025, 8, 16),
        (2026, 9, 5),
        (2027, 8, 25),
        (2028, 8, 14),
        (2029, 9, 1),
        (2030, 8, 21),
        (2031, 8, 10),
        (2032, 8, 28),
        (2033, 8, 18),
        (2034, 9, 6),
    ];

    #[test]
    fn the_first_sunrise_at_or_after_ashtami_is_the_iskcon_day_in_all_eleven_years() {
        let tokyo = HinduLunarCalendar::new(TOKYO, Ayanamsa::LAHIRI);
        let mut navami = 0;
        for (year, month, day) in DRIK_ISKCON_JANMASHTAMI {
            // Śrāvaṇa falls in August or September, the Śaka year the
            // Gregorian one less 78.
            let found = janmashtami(&tokyo, year - 78).expect("a day");
            assert_eq!(found, ymd(year, month, day), "{year}");
            // Always an Ashtami or a Navami day, as Drik says.
            let tithi = tokyo.from_fixed(found).expect("a date").day;
            assert!((23..=24).contains(&tithi), "{year}: {tithi}");
            navami += usize::from(tithi == 24);
        }
        // Ashtami holds no sunrise only in 2026 at Tokyo.
        assert_eq!(navami, 1);
    }

    /// The Smārta day is the one the national almanac lists, 6 September
    /// 2023 and 26 August 2024 (`rashtriya-panchang-1946`), and, as the
    /// central government's lists place the Vaiṣṇava day a day after it, 15
    /// August 2025 and 24 August 2027 are the Smārta ones where the lists
    /// keep the 16th and the 25th (`dopt-holidays-2025-2027`).
    #[test]
    fn the_smarta_day_is_the_national_almanacs_and_the_vaishnava_one_the_later() {
        let rashtriya = HinduLunarCalendar::RASHTRIYA;
        for (year, month, day) in [(2023, 9, 6), (2024, 8, 26), (2025, 8, 15), (2027, 8, 24)] {
            assert_eq!(
                janmashtami_by(FestivalReading::SMARTA, &rashtriya, year),
                Ok(ymd(year, month, day)),
                "{year}"
            );
        }
        for (year, month, day) in [(2025, 8, 16), (2027, 8, 25)] {
            assert_eq!(
                janmashtami_by(FestivalReading::VAISHNAVA, &rashtriya, year),
                Ok(ymd(year, month, day)),
                "{year}"
            );
        }
        // At the Central Station the Vaiṣṇava day is never before the
        // Smārta and never more than a day after it, 2017 to 2034.
        for year in 2017..=2034 {
            let smarta = janmashtami_by(FestivalReading::SMARTA, &rashtriya, year).expect("a day");
            let vaishnava =
                janmashtami_by(FestivalReading::VAISHNAVA, &rashtriya, year).expect("a day");
            assert!((0..=1).contains(&(vaishnava.0 - smarta.0)), "{year}");
        }
    }

    #[test]
    fn the_two_readings_are_named_and_found_by_their_names() {
        assert_eq!(FestivalReading::ALL.len(), 2);
        assert_eq!(
            FestivalReading::by_id("smarta"),
            Some(FestivalReading::SMARTA)
        );
        assert_eq!(
            FestivalReading::by_id("vaishnava"),
            Some(FestivalReading::VAISHNAVA)
        );
        assert_eq!(FestivalReading::by_id("iskcon"), None);
    }

    #[test]
    fn the_day_is_the_places_and_the_same_year_can_be_another_day_elsewhere() {
        // Ashtami of 2026 holds New Delhi's sunrise of 4 September and not
        // Tokyo's, whose sunrise comes hours earlier in the tithi's course:
        // the Vaiṣṇava day is 4 September at New Delhi and the 5th at Tokyo,
        // which is why the anchor above is Tokyo's list. In 2025 both are
        // 16 August.
        let delhi = HinduLunarCalendar::new(NEW_DELHI, Ayanamsa::LAHIRI);
        assert_eq!(janmashtami(&delhi, 2025 - 78), Ok(ymd(2025, 8, 16)));
        assert_eq!(janmashtami(&delhi, 2026 - 78), Ok(ymd(2026, 9, 4)));
    }

    #[test]
    fn a_tithi_outside_one_to_thirty_is_refused() {
        let lunar = HinduLunarCalendar::RASHTRIYA;
        assert_eq!(
            day_of(&lunar, 1947, 5, 0),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            day_of(&lunar, 1947, 5, 31),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            day_of(&lunar, 1947, 13, 1),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    /// Drik Panchang's Vaiṣṇava Ekadashi fasting days for New Delhi, 2025
    /// (`drik-iskcon-ekadashi`, read 2026-10-03), against the Smārta day,
    /// the one whose sunrise carries the Ekadashi tithi, 11 or 26.
    #[test]
    fn the_vaishnava_ekadashi_list_of_2025_parts_from_the_sunrise_day_on_five_dates() {
        let delhi = HinduLunarCalendar::new(NEW_DELHI, Ayanamsa::LAHIRI);
        let list = [
            (1, 10),
            (1, 25),
            (2, 8),
            (2, 24),
            (3, 10),
            (3, 26),
            (4, 8),
            (4, 24),
            (5, 8),
            (5, 23),
            (6, 7),
            (6, 22),
            (7, 6),
            (7, 21),
            (8, 5),
            (8, 19),
            (9, 3),
            (9, 17),
            (10, 3),
            (10, 17),
            (11, 2),
            (11, 15),
            (12, 1),
            (12, 16),
            (12, 31),
        ];
        let (mut same, mut later, mut unheld) = (
            alloc::vec::Vec::new(),
            alloc::vec::Vec::new(),
            alloc::vec::Vec::new(),
        );
        for (month, day) in list {
            let drik = ymd(2025, month, day);
            let tithi_at = |d: Rd| delhi.from_fixed(d).expect("a date").day;
            if matches!(tithi_at(drik), 11 | 26) {
                same.push((month, day));
            } else if matches!(tithi_at(Rd(drik.0 - 1)), 11 | 26) {
                later.push((month, day));
            } else {
                // The Ekadashi holds no sunrise; Drik's day is the one
                // whose sunrise carries the Dvadashi.
                assert!(matches!(tithi_at(drik), 12 | 27), "{month}-{day}");
                unheld.push((month, day));
            }
        }
        assert_eq!(same.len(), 20);
        assert_eq!(later, [(3, 26), (6, 7), (12, 16)]);
        assert_eq!(unheld, [(6, 22), (12, 31)]);
    }
}
