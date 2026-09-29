//! The yoga and the karaṇa: the two limbs of the *pañcāṅga* beside the
//! tithi ([`crate::tithi`]), the nakṣatra ([`crate::nakshatra`]) and the
//! weekday.
//!
//! Both are written up with their sources, and the comparison with Drik
//! Panchang's times, in `docs/systems/hindu-calendars.md` in the
//! repository, under "The yoga and the karaṇa". This page states the
//! code's own facts.
//!
//! # The yoga
//!
//! A yoga is the time the *sum* of the Sun's and the Moon's sidereal
//! longitudes takes to grow by 13°20′: the sum, taken modulo 360°, is cut
//! into twenty-seven arcs from zero, Viṣkambha first and Vaidhṛti last
//! (`wikipedia-nityayoga`). Reingold and Dershowitz's `yoga` is the same
//! arithmetic on the *Sūrya Siddhānta*'s Sun and Moon: one plus the floor
//! of the sum, modulo 360°, over 13°20′ (`reingold2018code`, `yoga`).
//! Here the longitudes are the true Sun and Moon of `hc-astro` less the
//! ayanāṃśa, the machinery [`crate::nakshatra`] uses for the Moon's
//! station and the Sun's, so a yoga moves with the ayanāṃśa twice over: a
//! difference of 20″ in the zero point moves every yoga's end by about a
//! minute.
//!
//! # The karaṇa
//!
//! A karaṇa is half a tithi, the time the Moon takes to gain 6° on the Sun,
//! sixty to a month. Their names are eleven: four fixed — Śakuni for the
//! second half of kṛṣṇa 14, Catuṣpada and Nāga for the two halves of
//! amāvāsyā, Kiṃstughna for the first half of śukla 1 — and seven movable,
//! Bava, Bālava, Kaulava, Taitila, Gara, Vaṇija and Viṣṭi, which run round
//! eight times through the fifty-six halves between (`wikipedia-karana`;
//! Sewell and Dikshit, *The Indian Calendar*, 1896, Arts. 10 and 40,
//! `sewell1896`, who note that the *Sūrya Siddhānta* orders the fixed four
//! Śakuni, Nāga, Catuṣpada, Kiṃstughna and follow, as this module does,
//! the practice of western India that puts Catuṣpada before Nāga). Reingold and Dershowitz's `karana` maps the
//! half-tithi's number, 1 to 60, to the name's, 0 to 10: 0 for the first,
//! the number less 50 above 57, and otherwise the number less one taken
//! round 1 to 7 (`reingold2018code`, `karana`), which [`karana_name`] is.
//! The two orders of the fixed four are two conventions (docs/policy.md
//! §5), so the *Sūrya Siddhānta*'s own, Nāga before Catuṣpada (II.67,
//! `sastri1861`), is a function of its own, [`karana_name_surya_siddhanta`].
//! The elongation needs no zodiac, so the karaṇa, like the tithi, does not
//! depend on the ayanāṃśa.
//!
//! # Which moment a day takes
//!
//! A pañcāṅga prints for each day the yoga and the karaṇa in progress at
//! its sunrise and the moment each ends; [`yoga_of_day`] and
//! [`karana_of_day`] read sunrise as [`crate::tithi::tithi_of_day`] does,
//! and [`yoga_span`] and [`karana_span`] give the moments.
//!
//! # The names
//!
//! [`YOGA_NAMES`] and [`KARANA_NAMES`] are Drik Panchang's English
//! spellings, and [`YOGA_NAMES_DEVANAGARI`] and [`KARANA_NAMES_DEVANAGARI`]
//! its Hindi edition's, from the day pages for New Delhi of 1 to
//! 30 January 2025 (`drik-day-panchang-2025`), which between them print all
//! twenty-seven yogas and all eleven karaṇas; the order is Wikipedia's
//! (`wikipedia-nityayoga`, `wikipedia-karana`).

use hc_astro::riseset::Location;
use hc_astro::search::next_angle_crossing;
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::{floor, normalize_degrees};
use hc_seasons::zodiac::Ayanamsa;
use hc_seasons::zodiac::sidereal::sidereal_longitude;

use crate::nakshatra::sidereal_lunar_longitude;
use crate::tithi::sunrise_of;

/// The yogas in a revolution of the sum of the longitudes.
pub const YOGAS_PER_REVOLUTION: u8 = 27;

/// The arc of one yoga: 13°20′.
pub const DEGREES_PER_YOGA: f64 = 360.0 / 27.0;

/// The karaṇas in a lunar month: two to a tithi.
pub const KARANAS_PER_MONTH: u8 = 60;

/// The Moon's gain on the Sun over one karaṇa, in degrees.
pub const DEGREES_PER_KARANA: f64 = 6.0;

/// The twenty-seven yogas, Viṣkambha first, as Drik Panchang spells them
/// in English.
pub const YOGA_NAMES: [&str; 27] = [
    "Vishkambha",
    "Priti",
    "Ayushmana",
    "Saubhagya",
    "Shobhana",
    "Atiganda",
    "Sukarma",
    "Dhriti",
    "Shula",
    "Ganda",
    "Vriddhi",
    "Dhruva",
    "Vyaghata",
    "Harshana",
    "Vajra",
    "Siddhi",
    "Vyatipata",
    "Variyana",
    "Parigha",
    "Shiva",
    "Siddha",
    "Sadhya",
    "Shubha",
    "Shukla",
    "Brahma",
    "Indra",
    "Vaidhriti",
];

/// The twenty-seven yogas in Devanagari, as Drik Panchang's Hindi edition
/// prints them.
pub const YOGA_NAMES_DEVANAGARI: [&str; 27] = [
    "विष्कम्भ",
    "प्रीति",
    "आयुष्मान्",
    "सौभाग्य",
    "शोभन",
    "अतिगण्ड",
    "सुकर्मा",
    "धृति",
    "शूल",
    "गण्ड",
    "वृद्धि",
    "ध्रुव",
    "व्याघात",
    "हर्षण",
    "वज्र",
    "सिद्धि",
    "व्यतीपात",
    "वरीयान्",
    "परिघ",
    "शिव",
    "सिद्ध",
    "साध्य",
    "शुभ",
    "शुक्ल",
    "ब्रह्म",
    "इन्द्र",
    "वैधृति",
];

/// The eleven karaṇa names, numbered 0 to 10 as [`karana_name`] numbers
/// them — Kiṃstughna, the seven movable ones, then Śakuni, Catuṣpada and
/// Nāga — as Drik Panchang spells them in English.
pub const KARANA_NAMES: [&str; 11] = [
    "Kinstughna",
    "Bava",
    "Balava",
    "Kaulava",
    "Taitila",
    "Garaja",
    "Vanija",
    "Vishti",
    "Shakuni",
    "Chatushpada",
    "Nagava",
];

/// The eleven karaṇa names in Devanagari, as Drik Panchang's Hindi edition
/// prints them.
pub const KARANA_NAMES_DEVANAGARI: [&str; 11] = [
    "किंस्तुघ्न",
    "बव",
    "बालव",
    "कौलव",
    "तैतिल",
    "गर",
    "वणिज",
    "विष्टि",
    "शकुनि",
    "चतुष्पाद",
    "नाग",
];

/// The sum of the Sun's and the Moon's sidereal longitudes at a moment,
/// modulo 360°.
#[must_use]
pub fn yoga_longitude(moment: Moment, ayanamsa: Ayanamsa) -> f64 {
    normalize_degrees(
        sidereal_longitude(moment, ayanamsa) + sidereal_lunar_longitude(moment, ayanamsa),
    )
}

/// The yoga in progress at a moment, 1 for Viṣkambha through 27 for
/// Vaidhṛti.
#[must_use]
pub fn yoga_at(moment: Moment, ayanamsa: Ayanamsa) -> u8 {
    let arc = floor(yoga_longitude(moment, ayanamsa) / DEGREES_PER_YOGA);
    // Rounding can put the sum a hair past the last arc's end.
    (arc as u8).min(YOGAS_PER_REVOLUTION - 1) + 1
}

/// The yoga a day carries: the one in progress at its sunrise.
#[must_use]
pub fn yoga_of_day(day: Rd, location: Location, ayanamsa: Ayanamsa) -> u8 {
    yoga_at(sunrise_of(day, location), ayanamsa)
}

/// The yoga in progress at a moment: when it began and when it ends.
#[must_use]
pub fn yoga_span(moment: Moment, ayanamsa: Ayanamsa) -> (Moment, Moment) {
    let angle = |at: Moment| yoga_longitude(at, ayanamsa);
    span_of(angle, DEGREES_PER_YOGA, moment)
}

/// The Moon's elongation from the Sun at a moment, 0° to 360°: the angle
/// the tithi and the karaṇa count.
fn elongation(moment: Moment) -> f64 {
    (crate::tithi::tithi_at(moment) - 1.0) * crate::tithi::DEGREES_PER_TITHI
}

/// The half-tithi in progress at a moment, 1 to 60: 1 the first half of
/// śukla 1, 60 the second half of amāvāsyā.
#[must_use]
pub fn karana_at(moment: Moment) -> u8 {
    let half = floor(elongation(moment) / DEGREES_PER_KARANA);
    (half as u8).min(KARANAS_PER_MONTH - 1) + 1
}

/// The name, 0 to 10, of the half-tithi `position`, 1 to 60, in the order
/// of western India that Sewell and Dikshit follow, Śakuni, Catuṣpada, Nāga
/// for halves 58 to 60: an index into [`KARANA_NAMES`]. Reingold and
/// Dershowitz's `karana`. `None` for a `position` outside 1 to 60, which is
/// no half of a month.
#[must_use]
pub const fn karana_name(position: u8) -> Option<u8> {
    if position < 1 || position > KARANAS_PER_MONTH {
        None
    } else if position == 1 {
        Some(0)
    } else if position > 57 {
        Some(position - 50)
    } else {
        Some((position - 2) % 7 + 1)
    }
}

/// The name, 0 to 10, of the half-tithi `position`, 1 to 60, in the
/// *Sūrya Siddhānta*'s order of the four fixed karaṇas, "S'akuni, Nága,
/// Chatushpada and Kinstughna" from the second half of kṛṣṇa 14 (II.67, in
/// Bapu Deva Sastri's translation, `sastri1861`), which Sewell and Dikshit
/// note beside the western order (`sewell1896`, Art. 10): Nāga for half 59
/// and Catuṣpada for half 60, and otherwise [`karana_name`]. `None` for a
/// `position` outside 1 to 60.
#[must_use]
pub const fn karana_name_surya_siddhanta(position: u8) -> Option<u8> {
    match position {
        59 => Some(10),
        60 => Some(9),
        _ => karana_name(position),
    }
}

/// The half-tithi a day carries: the one in progress at its sunrise.
#[must_use]
pub fn karana_of_day(day: Rd, location: Location) -> u8 {
    karana_at(sunrise_of(day, location))
}

/// The half-tithi in progress at a moment: when it began and when it ends.
#[must_use]
pub fn karana_span(moment: Moment) -> (Moment, Moment) {
    span_of(elongation, DEGREES_PER_KARANA, moment)
}

/// The arc of `width` degrees that `angle` is in at `moment`, as the moments
/// it entered and leaves it. `angle` must grow steadily by more than a
/// width in a day and less than a revolution in half a day, as the sum of
/// the longitudes (12° to 16° a day) and the elongation (10° to 15°) do.
pub(crate) fn span_of(
    angle: impl Fn(Moment) -> f64,
    width: f64,
    moment: Moment,
) -> (Moment, Moment) {
    let now = angle(moment);
    let entered_at = floor(now / width) * width;
    let left_at = normalize_degrees(entered_at + width);
    // An arc of 13°20′ takes under a day and a quarter to cross, so it
    // was entered within that of `moment`.
    let entry = next_angle_crossing(&angle, entered_at, Moment(moment.0 - 1.25), 0.5);
    let exit = next_angle_crossing(&angle, left_at, moment, 0.5);
    (entry, exit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::places::CENTRAL_STATION;
    use hc_calendars_solar::gregorian;

    /// A moment given in Indian Standard Time, five and a half hours ahead
    /// of Universal Time.
    fn ist(month: u8, day: u8, hour: u8, minute: u8) -> Moment {
        let day = gregorian::to_fixed(2025, month, day).expect("a date");
        Moment(day.0 as f64 + (f64::from(hour) + f64::from(minute) / 60.0 - 5.5) / 24.0)
    }

    /// Drik Panchang's day pages for New Delhi, 1 to 30 January 2025
    /// (`drik-day-panchang-2025`): every yoga whose end the pages print,
    /// as the yoga, 1 to 27, and the month, day, hour and minute IST at
    /// which it ends.
    const DRIK_YOGAS: [(u8, u8, u8, u8, u8); 32] = [
        (13, 1, 1, 17, 7),
        (14, 1, 2, 14, 58),
        (15, 1, 3, 12, 38),
        (16, 1, 4, 10, 8),
        (17, 1, 5, 7, 32),
        (18, 1, 6, 4, 51),
        (19, 1, 7, 2, 5),
        (20, 1, 7, 23, 16),
        (21, 1, 8, 20, 23),
        (22, 1, 9, 17, 29),
        (23, 1, 10, 14, 37),
        (24, 1, 11, 11, 49),
        (25, 1, 12, 9, 9),
        (26, 1, 13, 6, 45),
        (27, 1, 14, 4, 39),
        (1, 1, 15, 2, 59),
        (2, 1, 16, 1, 47),
        (3, 1, 17, 1, 6),
        (4, 1, 18, 0, 57),
        (5, 1, 19, 1, 16),
        (6, 1, 20, 1, 58),
        (7, 1, 21, 2, 53),
        (8, 1, 22, 3, 50),
        (9, 1, 23, 4, 38),
        (10, 1, 24, 5, 7),
        (11, 1, 25, 5, 9),
        (12, 1, 26, 4, 38),
        (13, 1, 27, 3, 34),
        (14, 1, 28, 1, 57),
        (15, 1, 28, 23, 52),
        (16, 1, 29, 21, 22),
        (17, 1, 30, 18, 33),
    ];

    /// The same pages' karaṇas: the name, 0 to 10 as [`KARANA_NAMES`]
    /// numbers them, and the moment IST at which it ends. The pages print
    /// "Full Night" for the two karaṇas that last past the next sunrise,
    /// which are left out.
    const DRIK_KARANAS: [(u8, u8, u8, u8, u8); 59] = [
        (2, 1, 1, 14, 55),
        (3, 1, 2, 2, 24),
        (4, 1, 2, 13, 48),
        (5, 1, 3, 1, 8),
        (6, 1, 3, 12, 25),
        (7, 1, 3, 23, 39),
        (1, 1, 4, 10, 51),
        (2, 1, 4, 22, 0),
        (3, 1, 5, 9, 8),
        (4, 1, 5, 20, 15),
        (5, 1, 6, 7, 20),
        (6, 1, 6, 18, 23),
        (7, 1, 7, 5, 25),
        (1, 1, 7, 16, 26),
        (2, 1, 8, 3, 26),
        (3, 1, 8, 14, 25),
        (4, 1, 9, 1, 24),
        (5, 1, 9, 12, 22),
        (6, 1, 9, 23, 20),
        (7, 1, 10, 10, 19),
        (1, 1, 10, 21, 19),
        (3, 1, 11, 19, 25),
        (4, 1, 12, 6, 33),
        (5, 1, 12, 17, 45),
        (6, 1, 13, 5, 3),
        (7, 1, 13, 16, 26),
        (1, 1, 14, 3, 56),
        (2, 1, 14, 15, 34),
        (3, 1, 15, 3, 21),
        (4, 1, 15, 15, 17),
        (5, 1, 16, 3, 23),
        (6, 1, 16, 15, 39),
        (7, 1, 17, 4, 6),
        (1, 1, 17, 16, 43),
        (2, 1, 18, 5, 30),
        (3, 1, 18, 18, 26),
        (4, 1, 19, 7, 30),
        (5, 1, 19, 20, 41),
        (6, 1, 20, 9, 58),
        (7, 1, 20, 23, 18),
        (1, 1, 21, 12, 39),
        (2, 1, 22, 2, 0),
        (3, 1, 22, 15, 18),
        (4, 1, 23, 4, 31),
        (5, 1, 23, 17, 37),
        (6, 1, 24, 6, 36),
        (7, 1, 24, 19, 25),
        (2, 1, 25, 20, 31),
        (3, 1, 26, 8, 48),
        (4, 1, 26, 20, 54),
        (5, 1, 27, 8, 49),
        (6, 1, 27, 20, 34),
        (7, 1, 28, 8, 9),
        (8, 1, 28, 19, 35),
        (9, 1, 29, 6, 54),
        (10, 1, 29, 18, 5),
        (0, 1, 30, 5, 10),
        (1, 1, 30, 16, 10),
        (2, 1, 31, 3, 6),
    ];

    /// The offsets, in minutes, of the computed ends from the printed ones,
    /// as the lowest and the highest.
    fn spread(offsets: &[f64]) -> (f64, f64) {
        offsets.iter().fold((f64::MAX, f64::MIN), |(low, high), m| {
            (low.min(*m), high.max(*m))
        })
    }

    #[test]
    fn the_yogas_of_january_2025_end_when_drik_panchang_says() {
        let mut offsets = alloc::vec::Vec::new();
        for (yoga, month, day, hour, minute) in DRIK_YOGAS {
            let printed = ist(month, day, hour, minute);
            // Ten minutes before the printed end the yoga is the printed one.
            let before = Moment(printed.0 - 10.0 / 1440.0);
            assert_eq!(yoga_at(before, Ayanamsa::LAHIRI), yoga, "{month}-{day}");
            let (entry, exit) = yoga_span(before, Ayanamsa::LAHIRI);
            assert!(entry.0 < before.0 && before.0 < exit.0);
            assert_eq!(
                yoga_at(Moment(exit.0 + 1e-4), Ayanamsa::LAHIRI),
                yoga % YOGAS_PER_REVOLUTION + 1
            );
            offsets.push((exit.0 - printed.0) * 1440.0);
        }
        // Every end within a minute of the printed one, from 56 seconds
        // before it to 14 seconds after, held here to a minute before and
        // half a minute after. The karaṇas below, which need no ayanāṃśa,
        // come 35 to 94 seconds after theirs, so the yogas run about a
        // minute early against them: the Lahiri value here and Drik
        // Panchang's stand about 20″ apart, as `crate::nakshatra`'s Sun
        // transits show, and the yoga takes the ayanāṃśa twice.
        let (low, high) = spread(&offsets);
        assert!(low > -1.0 && high < 0.5, "{offsets:?}");
    }

    #[test]
    fn the_karanas_of_january_2025_end_when_drik_panchang_says() {
        let mut offsets = alloc::vec::Vec::new();
        for (name, month, day, hour, minute) in DRIK_KARANAS {
            let printed = ist(month, day, hour, minute);
            let before = Moment(printed.0 - 10.0 / 1440.0);
            assert_eq!(karana_name(karana_at(before)), Some(name), "{month}-{day}");
            let (_, exit) = karana_span(before);
            offsets.push((exit.0 - printed.0) * 1440.0);
        }
        // The karaṇa needs no ayanāṃśa, so the two agree to within the
        // minute the pages print: every end here is 35 to 94 seconds after
        // the printed minute, which the pages appear to truncate.
        let (low, high) = spread(&offsets);
        assert!(low > 0.5 && high < 1.75, "{offsets:?}");
    }

    #[test]
    fn the_first_of_january_2025_carries_vyaghata_and_balava() {
        // The page for 1 January 2025: "Yoga Vyaghata upto 05:07 PM",
        // "Karana Balava upto 02:55 PM", read at sunrise.
        let day = gregorian::to_fixed(2025, 1, 1).expect("a date");
        let yoga = yoga_of_day(day, CENTRAL_STATION, Ayanamsa::LAHIRI);
        assert_eq!(YOGA_NAMES[usize::from(yoga - 1)], "Vyaghata");
        assert_eq!(YOGA_NAMES_DEVANAGARI[usize::from(yoga - 1)], "व्याघात");
        let karana = karana_name(karana_of_day(day, CENTRAL_STATION)).expect("a half");
        assert_eq!(KARANA_NAMES[usize::from(karana)], "Balava");
        assert_eq!(KARANA_NAMES_DEVANAGARI[usize::from(karana)], "बालव");
    }

    #[test]
    fn the_sixty_halves_take_the_names_table_viii_prints() {
        // Sewell and Dikshit, Table VIII, cols. 4 and 5: the karaṇas of the
        // first and second halves of each tithi. Śukla 1 is Kiṃstughna and
        // Bava, śukla 2 Bālava and Kaulava, kṛṣṇa 14 Viṣṭi and Śakuni,
        // amāvāsyā Catuṣpada and Nāga.
        let name = |position: u8| KARANA_NAMES[usize::from(karana_name(position).expect("a half"))];
        for (tithi, first, second) in [
            (1, "Kinstughna", "Bava"),
            (2, "Balava", "Kaulava"),
            (29, "Vishti", "Shakuni"),
            (30, "Chatushpada", "Nagava"),
        ] {
            assert_eq!(
                (name(2 * tithi - 1), name(2 * tithi)),
                (first, second),
                "{tithi}"
            );
        }
        // Between them the seven movable names, Bava to Viṣṭi, run eight
        // times round the fifty-six halves from the second half of śukla 1
        // to the first half of kṛṣṇa 14 (Art. 40).
        let movable = [
            "Bava", "Balava", "Kaulava", "Taitila", "Garaja", "Vanija", "Vishti",
        ];
        for position in 2..=57 {
            assert_eq!(
                name(position),
                movable[usize::from(position - 2) % 7],
                "{position}"
            );
        }
        assert_eq!(karana_name(0), None);
        assert_eq!(karana_name(61), None);
    }

    #[test]
    fn the_surya_siddhanta_orders_naga_before_catushpada() {
        // Sūrya Siddhānta II.67: "S'akuni, Nága, Chatushpada and
        // Kinstughna" from the latter half of kṛṣṇa 14; the other
        // fifty-seven halves are named as in the western order.
        let name = |position: u8| {
            KARANA_NAMES[usize::from(karana_name_surya_siddhanta(position).expect("a half"))]
        };
        assert_eq!(
            [name(58), name(59), name(60), name(1)],
            ["Shakuni", "Nagava", "Chatushpada", "Kinstughna"]
        );
        for position in 1..=58 {
            assert_eq!(
                karana_name_surya_siddhanta(position),
                karana_name(position),
                "{position}"
            );
        }
        assert_eq!(karana_name_surya_siddhanta(0), None);
        assert_eq!(karana_name_surya_siddhanta(61), None);
    }

    #[test]
    fn a_karana_is_half_a_tithi() {
        let mut moment = ist(1, 1, 0, 0);
        for _ in 0..70 {
            let tithi = crate::tithi::tithi_number_at(moment);
            let half = karana_at(moment);
            assert_eq!(half.div_ceil(2), tithi);
            let (entry, exit) = karana_span(moment);
            let length = exit.0 - entry.0;
            assert!((0.35..0.7).contains(&length), "{length}");
            moment = Moment(exit.0 + 1e-4);
        }
    }

    #[test]
    fn a_yoga_lasts_about_a_day() {
        let mut moment = ist(1, 1, 0, 0);
        for _ in 0..40 {
            let (entry, exit) = yoga_span(moment, Ayanamsa::LAHIRI);
            let length = exit.0 - entry.0;
            // Sewell and Dikshit, Art. 9, p. 3 (`sewell1896`): a yoga lasts
            // from 20 h 52 m 48 s to 24 h 36 m 24 s, 22 h 35 m 44.7 s on
            // the mean; the true Moon runs a little wider.
            assert!((0.85..1.06).contains(&length), "{length}");
            assert!(entry.0 <= moment.0 && moment.0 < exit.0);
            moment = Moment(exit.0 + 1e-4);
        }
    }
}
