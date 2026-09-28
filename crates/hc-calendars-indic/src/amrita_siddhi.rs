//! The *amṛta siddhi yoga*: the auspicious conjunction of a weekday with
//! one nakṣatra, the example Sewell and Dikshit give of the yogas that are
//! "quite different" from the twenty-seven of [`crate::panchanga`].
//!
//! The yoga, its sources and the checks are written up in
//! `docs/systems/hindu-calendars.md` in the repository, under "The yoga and
//! the karaṇa". This page states the code's own facts.
//!
//! Sewell and Dikshit's Art. 39 (`sewell1896`) describes "certain
//! conjunctions, also called *yogas*, which only occur when certain
//! conditions, as, for instance, the conjunction of certain varas and
//! nakshatras, or varas and tithis, are fulfilled": when the nakṣatra Hasta
//! falls on a Sunday there is an amṛta siddhi yoga, and their pañcāṅga
//! extract of Art. 30, for Poona in September 1894, marks one on the 2nd,
//! 5th and 18th. They name only the Sunday's nakṣatra. The seven pairs
//! here are Prokerala's (`prokerala-amrit-siddhi`, a secondary source):
//! Hasta on Sunday, Mṛgaśīrṣa on Monday, Aśvinī on Tuesday, Anurādhā on
//! Wednesday, Puṣya on Thursday, Revatī on Friday and Rohiṇī on Saturday.
//! The test holds them to Sewell and Dikshit's three days of 1894 and to
//! Drik Panchang's New Delhi pages of January 2025, which print the yoga on
//! 7, 11 and 19 January and on no other day of the month.
//!
//! # Which part of the day
//!
//! The weekday is the day from sunrise to the next sunrise, and the yoga
//! holds for as much of it as the Moon spends in the nakṣatra: Drik
//! Panchang prints Tuesday 7 January 2025's as "05:50 PM to 07:15 AM,
//! Jan 08", from the Moon's entry into Aśvinī to the next sunrise, and
//! Saturday 11 January's as "07:15 AM to 12:29 PM", from sunrise to the
//! Moon's leaving Rohiṇī. Sewell and Dikshit's Wednesday, 5 September 1894,
//! has Anurādhā only from late in the day, after Viśākhā ends. So
//! [`amrita_siddhi`] is the overlap of the two, or `None`.

use hc_astro::riseset::Location;
use hc_calendar::fixed::Moment;
use hc_calendar::{Rd, Weekday};
use hc_seasons::zodiac::Ayanamsa;

use crate::kalam::Span;
use crate::nakshatra::{
    ANURADHA, ASHVINI, HASTA, MRIGASHIRSHA, PUSHYA, REVATI, ROHINI, nakshatra_span,
};
use crate::tithi::sunrise_of;

/// The nakṣatra of the amṛta siddhi yoga for each weekday, Sunday first,
/// 1 for Aśvinī through 27 for Revatī.
pub const NAKSHATRA_BY_WEEKDAY: [u8; 7] = [
    HASTA,
    MRIGASHIRSHA,
    ASHVINI,
    ANURADHA,
    PUSHYA,
    REVATI,
    ROHINI,
];

/// The yoga's name, as Drik Panchang's day pages print it in English
/// (`drik-day-panchang-2025`).
pub const NAME: &str = "Amrita Siddhi Yoga";

/// The yoga's name in Devanagari, as the Hindi edition of the same pages
/// prints it.
pub const NAME_DEVANAGARI: &str = "अमृत सिद्धि योग";

/// The nakṣatra of the yoga on a weekday.
#[must_use]
pub const fn nakshatra_of(weekday: Weekday) -> u8 {
    NAKSHATRA_BY_WEEKDAY[weekday.sunday_first_number() as usize]
}

/// The amṛta siddhi yoga of a local day, sunrise to the next sunrise at the
/// place: the part of it the Moon spends in the weekday's nakṣatra, in
/// Universal Time, or `None` when it spends none.
#[must_use]
pub fn amrita_siddhi(day: Rd, location: Location, ayanamsa: Ayanamsa) -> Option<Span> {
    let rise = sunrise_of(day, location);
    let next = sunrise_of(Rd(day.0 + 1), location);
    let nakshatra = nakshatra_of(Weekday::from_rd(day));
    let (entry, exit) = nakshatra_span(nakshatra, rise, ayanamsa);
    let start = entry.0.max(rise.0);
    let end = exit.0.min(next.0);
    (start < end).then_some(Span {
        start: Moment(start),
        end: Moment(end),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    /// New Delhi, as Drik Panchang's pages place it, at sea level.
    const NEW_DELHI: Location = Location::new(
        28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
        77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
        0.0,
    );

    /// Poona, 18°31′ N, 73°52′ E, where the pañcāṅga of Sewell and
    /// Dikshit's Art. 30 was published.
    const POONA: Location = Location::new(18.0 + 31.0 / 60.0, 73.0 + 52.0 / 60.0, 0.0);

    /// Minutes after midnight IST, on `day`, of a Universal Time moment.
    fn ist_minutes(moment: Moment, day: Rd) -> f64 {
        (moment.0 - day.0 as f64) * 1_440.0 + 330.0
    }

    #[test]
    fn the_yoga_of_january_2025_is_where_drik_panchang_prints_it() {
        // (day, start and end in minutes IST from the day's midnight): 7
        // January "05:50 PM to 07:15 AM, Jan 08", 11 January "07:15 AM to
        // 12:29 PM", 19 January "05:30 PM to 07:14 AM, Jan 20".
        let printed: [(u8, u16, u16); 3] = [
            (7, 17 * 60 + 50, 24 * 60 + 7 * 60 + 15),
            (11, 7 * 60 + 15, 12 * 60 + 29),
            (19, 17 * 60 + 30, 24 * 60 + 7 * 60 + 14),
        ];
        for date in 1..=31 {
            let day = gregorian::to_fixed(2025, 1, date).expect("a date");
            let yoga = amrita_siddhi(day, NEW_DELHI, Ayanamsa::LAHIRI);
            match printed
                .iter()
                .find(|(printed_day, _, _)| *printed_day == date)
            {
                Some(&(_, start, end)) => {
                    let span = yoga.expect("a yoga");
                    // Drik Panchang's Lahiri stands about 20″ from this one,
                    // which moves the Moon's entry by under a minute; the
                    // pages print whole minutes.
                    for (moment, minutes) in [(span.start, start), (span.end, end)] {
                        let offset = ist_minutes(moment, day) - f64::from(minutes);
                        assert!(offset.abs() < 1.5, "{date}: {offset} min");
                    }
                }
                None => assert_eq!(yoga, None, "{date}"),
            }
        }
    }

    #[test]
    fn sewell_and_dikshits_extract_has_it_on_the_2nd_5th_and_18th() {
        // Art. 39: "there is an amrita siddhiyoga on the 2nd, 5th and 18th
        // of September" in the extract of Art. 30, which runs from 31
        // August to 29 September 1894 at Poona.
        let first = gregorian::to_fixed(1894, 8, 31).expect("a date");
        let days: alloc::vec::Vec<(u8, u8)> = (0..30)
            .map(|offset| Rd(first.0 + offset))
            .filter(|day| amrita_siddhi(*day, POONA, Ayanamsa::LAHIRI).is_some())
            .map(|day| {
                let (_, month, date) = gregorian::from_fixed(day).expect("a date");
                (month, date)
            })
            .collect();
        assert_eq!(days, [(9, 2), (9, 5), (9, 18)]);
        // Sunday the 2nd is Hasta, as their example says.
        assert_eq!(nakshatra_of(Weekday::Sunday), HASTA);
    }

    #[test]
    fn the_yoga_never_leaves_its_day() {
        let first = gregorian::to_fixed(2025, 1, 1).expect("a date");
        for offset in 0..366 {
            let day = Rd(first.0 + offset);
            if let Some(span) = amrita_siddhi(day, NEW_DELHI, Ayanamsa::LAHIRI) {
                assert!(span.start.0 >= sunrise_of(day, NEW_DELHI).0);
                assert!(span.end.0 <= sunrise_of(Rd(day.0 + 1), NEW_DELHI).0);
                assert!(span.start.0 < span.end.0);
            }
        }
    }
}
