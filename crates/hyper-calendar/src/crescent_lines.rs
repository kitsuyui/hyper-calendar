//! The tab-separated line the WebAssembly module and the C library write
//! about the young crescent, written once.
//!
//! Whether the crescent should have been visible on the evening that
//! begins a day, from a place, by a named criterion — one of
//! [`NamedCriterion::ALL`] in `hc-calendars-lunar`'s
//! [`islamic_observational`](hc_calendars_lunar::islamic_observational),
//! Shaukat's, Yallop's, Odeh's, the Saudi rule, the Istanbul 2016 and
//! KHGT parameters or a reading of Neo-MABIMS — with the quantities the
//! criteria read at the moment the evening is judged. It is a forecast of
//! an observation in a clear sky, not a record of one. The day answers for
//! the sky layer's era, [`crate::astro_lines`].

use alloc::string::String;

use hc_astro::riseset::{Location, lunar_altitude};
use hc_calendar::Rd;
use hc_calendars_lunar::islamic_observational::{
    NamedCriterion, ObservationSite, arc_of_light, arc_of_vision, crescent_width_arcminutes,
};

use crate::astro_lines::{day_in_era, unix_from_moment};
use crate::boundary::{Answer, Line, Refusal};

/// The criterion an identifier names, one of [`NamedCriterion::ALL`], by
/// [`NamedCriterion::by_id`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text, the empty string included.
pub fn criterion(given: &str) -> Answer<NamedCriterion> {
    NamedCriterion::by_id(given).ok_or(Refusal::Unknown)
}

/// The line of `hc_crescent_visible`: whether the crescent should have been
/// visible on the evening that begins a fixed day — the evening of the day
/// before — from a place by a named criterion, 1 or 0; then the moment the
/// criterion judges the evening at, as whole POSIX seconds of Universal
/// Time, rounded down; and at that moment the Moon's longitude less the
/// Sun's, 0 to 360, its arc of light, its geocentric altitude, the arc of vision, all
/// in degrees, and the crescent's topocentric width in minutes of arc.
///
/// The moment is the Sun at 4.5° below the horizon for Shaukat's
/// criterion, Bruin's best time for Yallop's and Odeh's, and sunset for
/// the Saudi rule and the criteria of thresholds at sunset. Where there is
/// none — the Sun does not set or twilight does not end, or at Bruin's best
/// time the Moon sets before the Sun — no observation is
/// possible: the first cell is 0 and the other six are empty.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a criterion [`criterion`] does not name, and
/// the range error of [`day_in_era`] for the day.
pub fn crescent_line(criterion_id: &str, fixed: i64, place: Location) -> Answer<String> {
    let named = criterion(criterion_id)?;
    let day = day_in_era(fixed)?;
    let site = ObservationSite::new(place, named.criterion);
    let visible = site.crescent_visible_on_the_eve_of(day);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.flag(visible);
    match site.evaluation_moment(Rd(day.0 - 1)) {
        Some(moment) => line
            .value(unix_from_moment(moment))
            .value(hc_astro::lunar_phase(moment))
            .value(arc_of_light(moment))
            .value(lunar_altitude(moment, place))
            .value(arc_of_vision(moment, place))
            .value(crescent_width_arcminutes(moment, place)),
        None => line.empties(6),
    };
    line.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    use hc_calendars_lunar::islamic_observational::{IslamicObservationalCalendar, MECCA};

    fn cells(line: &str) -> Vec<&str> {
        line.strip_suffix('\n')
            .expect("a line")
            .split('\t')
            .collect()
    }

    /// The line's verdict is the calendar's: over the 60 days round the
    /// first of Ramadan 1445 at Mecca, each of the three days `islamic-rgsa`
    /// begins a month on is the first whose eve carries a crescent visible by
    /// Shaukat's criterion, the day before's eve carrying none.
    #[test]
    fn the_verdict_is_the_one_the_calendar_begins_its_months_by() {
        let calendar = IslamicObservationalCalendar::MECCA;
        let start = calendar.compose(1_445, 9, 1).expect("in range").0;
        let visible = |day: i64| {
            let line = crescent_line("shaukat", day, MECCA).expect("in the era");
            cells(&line)[0] == "1"
        };
        let mut starts = 0;
        for day in start - 30..start + 30 {
            let (_, _, of_month) = calendar.decompose(Rd(day)).expect("in range");
            if of_month == 1 {
                assert!(visible(day) && !visible(day - 1), "R.D. {day}");
                starts += 1;
            }
        }
        assert_eq!(starts, 3);
    }

    #[test]
    fn every_criterion_answers_in_seven_cells_and_others_are_refused() {
        for named in NamedCriterion::ALL {
            let line = crescent_line(named.id, 739_320, MECCA).expect("in the era");
            assert_eq!(cells(&line).len(), 7, "{}", named.id);
        }
        // Shaukat's and the Saudi rule judge every evening at Mecca.
        for id in ["shaukat", "saudi-rule"] {
            let line = crescent_line(id, 739_320, MECCA).expect("in the era");
            assert!(cells(&line)[1].parse::<i64>().is_ok(), "{id}: {line}");
        }
        assert_eq!(crescent_line("SHAUKAT", 739_320, MECCA).map(|_| ()), Ok(()));
        assert_eq!(
            crescent_line("danjon", 739_320, MECCA),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            crescent_line("yallop", 2_000_000, MECCA),
            Err(Refusal::OutOfRange)
        );
    }

    /// Under the midnight sun the Sun does not set, and no evening can be
    /// judged.
    #[test]
    fn a_sun_that_does_not_set_leaves_nothing_to_judge() {
        let tromso = Location::new(69.6496, 18.9560, 0.0);
        // 21 June 2024; its eve is the 20th.
        let line = crescent_line("shaukat", 739_058, tromso).expect("in the era");
        assert_eq!(cells(&line), ["0", "", "", "", "", "", ""]);
    }
}
