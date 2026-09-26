//! The moon-phase layer: what the Moon looks like on a given day, and when
//! the four principal phases fall.
//!
//! [`hc_astro`] answers "where is the Moon at this instant". This module
//! answers the calendar questions built on top of that: what phase is today,
//! how old is the Moon, how much of it is lit, and which days of this month
//! hold a new moon, a first quarter, a full moon and a last quarter.
//!
//! # Where in the day these are measured
//!
//! Phase, age and illuminated fraction all change through a day — the Moon's
//! elongation grows by about 12° a day — so a single number for a whole day
//! has to be quoted for some instant inside it. This module uses **local
//! noon**, and says so in every doc comment. That is the convention of
//! Japan's National Astronomical Observatory, whose 暦象年表 prints 正午月齢,
//! the Moon's age at noon of the day to a tenth of a day (国立天文台
//! 暦計算室, 「月齢について」, from the 暦象年表 2017,
//! `nao-topics-2017-getsurei`,
//! <https://eco.mtk.nao.ac.jp/koyomi/topics/html/topics2017_1.html>,
//! retrieved 2026-09-26). That page does not name the time scale of the
//! noon; the Observatory's 暦要項 gives its times in 中央標準時, Japan
//! Standard Time, and at [`Meridian::JAPAN`] this module's noon is noon
//! JST. [`moon_age_at`] takes any instant for callers who need to match a
//! source that quotes another.
//!
//! # 十五夜 and 十三夜
//!
//! The two moon-viewing nights are lunisolar dates, not astronomical events:
//! 十五夜 (中秋の名月) is the fifteenth day of the eighth lunisolar month and
//! 十三夜 the thirteenth of the ninth. Neither is reliably the night of the
//! actual full moon — a lunation is 29.53 days, so the full moon falls on the
//! fifteenth day only about half the time, and 中秋の名月 can be a day or two
//! off. That is not an error in this crate or in the tradition; the
//! observance is dated by the calendar, not by the sky.

use hc_astro::lunar::{MoonPhase, lunar_illuminated_fraction, lunar_phase, moon_phase_at_or_after};
use hc_astro::new_moon_before;
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;

use crate::lunisolar::ordinary_date_in_gregorian_year;
use crate::meridian::Meridian;

/// How many named phases the lunation is divided into for [`PhaseName`].
const PHASE_NAMES: usize = 8;

/// The width of one named phase, in degrees of elongation.
const DEGREES_PER_NAMED_PHASE: f64 = 360.0 / PHASE_NAMES as f64;

/// The largest number of principal-phase events a Gregorian month can hold.
///
/// A 31-day month is 1.05 lunations, so it holds four events and can hold a
/// fifth when one falls on the first day or two.
const MAXIMUM_MONTH_PHASES: usize = 6;

/// One of the eight conventional phase names.
///
/// The four "quarter" names are the instants; the four in between are the
/// stretches. A day is given the name whose 45° arc of elongation contains
/// the Moon at local noon, so `NewMoon` covers the day either side of the
/// conjunction rather than only the instant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhaseName {
    /// 新月 / 朔: elongation within 22.5° of 0°.
    NewMoon,
    /// 三日月: waxing, between 22.5° and 67.5°.
    WaxingCrescent,
    /// 上弦: waxing, within 22.5° of 90°.
    FirstQuarter,
    /// Waxing gibbous, between 112.5° and 157.5°.
    WaxingGibbous,
    /// 満月 / 望: elongation within 22.5° of 180°.
    FullMoon,
    /// Waning gibbous, between 202.5° and 247.5°.
    WaningGibbous,
    /// 下弦: waning, within 22.5° of 270°.
    LastQuarter,
    /// Waning crescent, between 292.5° and 337.5°.
    WaningCrescent,
}

impl PhaseName {
    /// All eight, in the order they occur through a lunation.
    pub const ALL: [Self; PHASE_NAMES] = [
        Self::NewMoon,
        Self::WaxingCrescent,
        Self::FirstQuarter,
        Self::WaxingGibbous,
        Self::FullMoon,
        Self::WaningGibbous,
        Self::LastQuarter,
        Self::WaningCrescent,
    ];

    /// The phase name for an elongation from the Sun, in degrees.
    ///
    /// The arcs are the conventional eighths, each centred on its name: 0°,
    /// 45°, 90° and so on, so the boundaries fall at 22.5° plus a multiple of
    /// 45°.
    #[must_use]
    pub fn from_elongation_degrees(degrees: f64) -> Self {
        let shifted = hc_core::math::normalize_degrees(degrees + DEGREES_PER_NAMED_PHASE / 2.0);
        let index = hc_core::math::floor(shifted / DEGREES_PER_NAMED_PHASE) as usize;
        Self::ALL[index % PHASE_NAMES]
    }

    /// Whether the Moon is growing towards full.
    #[must_use]
    pub const fn is_waxing(self) -> bool {
        matches!(
            self,
            Self::WaxingCrescent | Self::FirstQuarter | Self::WaxingGibbous
        )
    }

    /// Whether the Moon is shrinking towards new.
    #[must_use]
    pub const fn is_waning(self) -> bool {
        matches!(
            self,
            Self::WaningGibbous | Self::LastQuarter | Self::WaningCrescent
        )
    }

    /// The name in Japanese characters, where the phase has a common one.
    ///
    /// The gibbous phases have no single customary name, so they are given
    /// the descriptive 十日余りの月 and 更待月 forms used in classical poetry.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        match self {
            Self::NewMoon => "新月",
            Self::WaxingCrescent => "三日月",
            Self::FirstQuarter => "上弦の月",
            Self::WaxingGibbous => "十日余りの月",
            Self::FullMoon => "満月",
            Self::WaningGibbous => "更待月",
            Self::LastQuarter => "下弦の月",
            Self::WaningCrescent => "有明月",
        }
    }

    /// The name in English.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::NewMoon => "new moon",
            Self::WaxingCrescent => "waxing crescent",
            Self::FirstQuarter => "first quarter",
            Self::WaxingGibbous => "waxing gibbous",
            Self::FullMoon => "full moon",
            Self::WaningGibbous => "waning gibbous",
            Self::LastQuarter => "last quarter",
            Self::WaningCrescent => "waning crescent",
        }
    }
}

/// The Moon's elongation from the Sun at local noon on a day, in degrees.
///
/// 0° is new, 180° is full. This is the ecliptic-longitude difference, which
/// is the quantity every phase rule is stated in, not the angular separation
/// on the sky.
#[must_use]
pub fn elongation_degrees(day: Rd, meridian: Meridian) -> f64 {
    lunar_phase(meridian.noon(day))
}

/// The phase name of a day, measured at local noon.
#[must_use]
pub fn phase_name(day: Rd, meridian: Meridian) -> PhaseName {
    PhaseName::from_elongation_degrees(elongation_degrees(day, meridian))
}

/// The age of the Moon at an instant, in days since the preceding
/// conjunction.
///
/// This is 月齢, and it runs from 0 to about 29.53 and then resets. The
/// conjunction it counts from is found by search rather than by a mean
/// lunation, so the answer tracks the real Moon, which runs up to half a day
/// either side of the mean.
#[must_use]
pub fn moon_age_at(moment: Moment) -> f64 {
    moment.0 - new_moon_before(moment).0
}

/// The age of the Moon at local noon on a day, in days.
///
/// Quoted at noon so that one number describes a whole day, as the National
/// Astronomical Observatory of Japan quotes its 正午月齢: at
/// [`Meridian::JAPAN`] this is that figure before rounding. Use
/// [`moon_age_at`] for any other instant.
#[must_use]
pub fn moon_age(day: Rd, meridian: Meridian) -> f64 {
    moon_age_at(meridian.noon(day))
}

/// The fraction of the Moon's disc lit at local noon on a day, from 0 to 1.
///
/// Accurate to about 0.001, which is `hc-astro`'s figure for the underlying
/// Meeus series.
#[must_use]
pub fn illuminated_fraction(day: Rd, meridian: Meridian) -> f64 {
    lunar_illuminated_fraction(meridian.noon(day))
}

/// One principal-phase event.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhaseEvent {
    /// Which of the four.
    pub phase: MoonPhase,
    /// The instant, in Universal Time.
    pub moment: Moment,
    /// The day it falls on at the meridian it was asked for.
    pub day: Rd,
}

/// The principal-phase events of a Gregorian month, in date order.
///
/// A month usually holds exactly four, one of each; a 31-day month can hold
/// five, which is what a "blue moon" or a "black moon" is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MonthPhases {
    events: [Option<PhaseEvent>; MAXIMUM_MONTH_PHASES],
    count: usize,
}

impl MonthPhases {
    /// How many events the month holds.
    #[must_use]
    pub const fn len(self) -> usize {
        self.count
    }

    /// Whether the month holds none, which cannot happen for a real month.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.count == 0
    }

    /// The first event of a given phase in the month, if there is one.
    #[must_use]
    pub fn first(self, phase: MoonPhase) -> Option<PhaseEvent> {
        self.into_iter().find(|event| event.phase == phase)
    }

    /// The month's new moon, if it has one.
    #[must_use]
    pub fn new_moon(self) -> Option<PhaseEvent> {
        self.first(MoonPhase::New)
    }

    /// The month's first quarter, if it has one.
    #[must_use]
    pub fn first_quarter(self) -> Option<PhaseEvent> {
        self.first(MoonPhase::FirstQuarter)
    }

    /// The month's full moon, if it has one.
    #[must_use]
    pub fn full_moon(self) -> Option<PhaseEvent> {
        self.first(MoonPhase::Full)
    }

    /// The month's last quarter, if it has one.
    #[must_use]
    pub fn last_quarter(self) -> Option<PhaseEvent> {
        self.first(MoonPhase::LastQuarter)
    }

    /// The second event of a given phase, when the month holds two.
    ///
    /// A second full moon in a Gregorian month is the "blue moon" of the
    /// modern usage; a second new moon is sometimes called a black moon.
    /// Neither is astronomy, only an artefact of 29.53 not dividing 31.
    #[must_use]
    pub fn second(self, phase: MoonPhase) -> Option<PhaseEvent> {
        self.into_iter().filter(|event| event.phase == phase).nth(1)
    }

    /// Add an event, keeping the list ordered by instant.
    fn insert(&mut self, event: PhaseEvent) {
        if self.count >= MAXIMUM_MONTH_PHASES {
            // Unreachable for a real month: 1.05 lunations cannot hold six
            // principal phases. Dropping beats panicking.
            return;
        }
        let mut position = self.count;
        while position > 0 {
            let earlier = self.events[position - 1];
            match earlier {
                Some(earlier) if earlier.moment.0 > event.moment.0 => {
                    self.events[position] = Some(earlier);
                    position -= 1;
                }
                _ => break,
            }
        }
        self.events[position] = Some(event);
        self.count += 1;
    }
}

impl IntoIterator for MonthPhases {
    type Item = PhaseEvent;
    type IntoIter = MonthPhasesIter;

    fn into_iter(self) -> Self::IntoIter {
        MonthPhasesIter {
            phases: self,
            position: 0,
        }
    }
}

/// The iterator over a [`MonthPhases`].
#[derive(Debug, Clone, Copy)]
pub struct MonthPhasesIter {
    phases: MonthPhases,
    position: usize,
}

impl Iterator for MonthPhasesIter {
    type Item = PhaseEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.phases.count {
            return None;
        }
        let event = self.phases.events[self.position];
        self.position += 1;
        event
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.phases.count - self.position;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for MonthPhasesIter {}

/// The four principal phases falling inside a Gregorian month, at a meridian.
///
/// ```
/// use hc_seasons::{Meridian, moon_calendar::principal_phases_in_month};
///
/// // September 2024 held a full moon on the 18th, JST.
/// let phases = principal_phases_in_month(2024, 9, Meridian::JAPAN);
/// assert_eq!(
///     phases.full_moon().map(|event| event.day),
///     Some(hc_calendar::Rd(739_147))
/// );
/// ```
#[must_use]
pub fn principal_phases_in_month(year: i64, month: u8, meridian: Meridian) -> MonthPhases {
    let first = crate::gregorian::from_year_month_day(year, month, 1);
    let next = if month >= 12 {
        crate::gregorian::from_year_month_day(year + 1, 1, 1)
    } else {
        crate::gregorian::from_year_month_day(year, month + 1, 1)
    };
    let start = meridian.midnight(first);
    let end = meridian.midnight(next);
    let mut phases = MonthPhases {
        events: [None; MAXIMUM_MONTH_PHASES],
        count: 0,
    };
    for phase in [
        MoonPhase::New,
        MoonPhase::FirstQuarter,
        MoonPhase::Full,
        MoonPhase::LastQuarter,
    ] {
        let mut cursor = start;
        // Two of the same phase can fall in one Gregorian month; three
        // cannot, because that would need two full lunations in 31 days.
        for _ in 0..2 {
            let moment = moon_phase_at_or_after(phase.elongation_degrees(), cursor);
            if moment.0 >= end.0 {
                break;
            }
            phases.insert(PhaseEvent {
                phase,
                moment,
                day: meridian.day_of(moment),
            });
            cursor = Moment(moment.0 + 1.0);
        }
    }
    phases
}

/// 十五夜, the 中秋の名月: the fifteenth day of the eighth lunisolar month.
///
/// Returns `None` only if the date somehow falls outside the Gregorian year
/// asked for, which the eighth month never does in practice.
///
/// This is a *calendar* date. The full moon is on the same night only about
/// half the time; see the module documentation.
#[must_use]
pub fn mid_autumn_moon(year: i64, meridian: Meridian) -> Option<Rd> {
    ordinary_date_in_gregorian_year(year, 8, 15, meridian)
}

/// 十三夜, the 後の月: the thirteenth day of the ninth lunisolar month.
///
/// The companion to 十五夜, about a month later. Viewing one and not the
/// other was 片見月 and held to be unlucky.
#[must_use]
pub fn thirteenth_night(year: i64, meridian: Meridian) -> Option<Rd> {
    ordinary_date_in_gregorian_year(year, 9, 13, meridian)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::from_year_month_day;
    use hc_astro::lunar::MEAN_SYNODIC_MONTH;

    const JAPAN: Meridian = Meridian::JAPAN;

    #[test]
    fn the_eight_phase_names_tile_the_lunation() {
        for degrees in 0..720 {
            let name = PhaseName::from_elongation_degrees(f64::from(degrees) / 2.0);
            assert!(PhaseName::ALL.contains(&name));
        }
        // Each name owns a 45-degree arc centred on its own elongation.
        assert_eq!(PhaseName::from_elongation_degrees(0.0), PhaseName::NewMoon);
        assert_eq!(
            PhaseName::from_elongation_degrees(359.0),
            PhaseName::NewMoon
        );
        assert_eq!(PhaseName::from_elongation_degrees(22.0), PhaseName::NewMoon);
        assert_eq!(
            PhaseName::from_elongation_degrees(23.0),
            PhaseName::WaxingCrescent
        );
        assert_eq!(
            PhaseName::from_elongation_degrees(90.0),
            PhaseName::FirstQuarter
        );
        assert_eq!(
            PhaseName::from_elongation_degrees(180.0),
            PhaseName::FullMoon
        );
        assert_eq!(
            PhaseName::from_elongation_degrees(270.0),
            PhaseName::LastQuarter
        );
    }

    #[test]
    fn waxing_and_waning_are_complementary_and_exclude_the_four_instants() {
        let mut waxing = 0;
        let mut waning = 0;
        for name in PhaseName::ALL {
            assert!(!(name.is_waxing() && name.is_waning()));
            if name.is_waxing() {
                waxing += 1;
            }
            if name.is_waning() {
                waning += 1;
            }
            assert!(!name.japanese_name().is_empty());
            assert!(name.english_name().is_ascii());
        }
        assert_eq!(waxing, 3);
        assert_eq!(waning, 3);
        assert!(!PhaseName::NewMoon.is_waxing());
        assert!(!PhaseName::FullMoon.is_waning());
    }

    #[test]
    fn the_phase_names_run_in_order_through_a_lunation() {
        // Starting from a known new moon, the names must appear in cycle
        // order and never go backwards within one lunation.
        // Twenty-seven days, not twenty-nine: a lunation is 29.53 days, so
        // by day 28 the next new moon has already come round and the names
        // legitimately wrap.
        let start = JAPAN.day_of(hc_astro::nth_new_moon(300));
        let mut expected = 0;
        for offset in 0..27 {
            let name = phase_name(Rd(start.0 + offset), JAPAN);
            let index = PhaseName::ALL
                .iter()
                .position(|candidate| *candidate == name)
                .unwrap();
            assert!(
                index >= expected,
                "the phase went backwards on day {offset}: {name:?}"
            );
            expected = index;
        }
        assert!(expected >= 6, "a lunation did not reach the last quarter");
    }

    #[test]
    fn the_moon_is_dark_when_new_and_bright_when_full() {
        let new_moon = hc_astro::nth_new_moon(300);
        assert!(lunar_illuminated_fraction(new_moon) < 0.01);
        let full = moon_phase_at_or_after(180.0, new_moon);
        assert!(lunar_illuminated_fraction(full) > 0.99);
        let quarter = moon_phase_at_or_after(90.0, new_moon);
        assert!((lunar_illuminated_fraction(quarter) - 0.5).abs() < 0.02);
    }

    #[test]
    fn the_moons_age_runs_from_zero_to_a_synodic_month_and_resets() {
        let start = from_year_month_day(2024, 1, 1);
        let mut previous = moon_age(start, JAPAN);
        let mut resets = 0;
        for offset in 1..400 {
            let age = moon_age(Rd(start.0 + offset), JAPAN);
            assert!(
                (0.0..MEAN_SYNODIC_MONTH + 0.5).contains(&age),
                "an age of {age} days"
            );
            if age < previous {
                resets += 1;
                assert!(age < 1.0, "the age reset to {age} rather than to nearly 0");
            } else {
                assert!(
                    (age - previous - 1.0).abs() < 1e-6,
                    "the age jumped by {}",
                    age - previous
                );
            }
            previous = age;
        }
        assert!(
            (12..=15).contains(&resets),
            "{resets} lunations in 400 days"
        );
    }

    #[test]
    fn the_age_and_the_illuminated_fraction_tell_the_same_story() {
        let start = from_year_month_day(2024, 1, 1);
        for offset in 0..200 {
            let day = Rd(start.0 + offset);
            let age = moon_age(day, JAPAN);
            let lit = illuminated_fraction(day, JAPAN);
            assert!((0.0..=1.0).contains(&lit));
            if !(0.6..=MEAN_SYNODIC_MONTH - 0.6).contains(&age) {
                assert!(lit < 0.05, "age {age} but {lit} lit");
            }
            if (age - MEAN_SYNODIC_MONTH / 2.0).abs() < 0.4 {
                assert!(lit > 0.95, "age {age} but only {lit} lit");
            }
        }
    }

    /// The illuminated fraction and the phase name must not contradict each
    /// other: a day called full cannot be a sliver.
    #[test]
    fn the_phase_name_and_the_illuminated_fraction_agree() {
        let start = from_year_month_day(2024, 1, 1);
        for offset in 0..366 {
            let day = Rd(start.0 + offset);
            let lit = illuminated_fraction(day, JAPAN);
            match phase_name(day, JAPAN) {
                PhaseName::NewMoon => assert!(lit < 0.09, "new moon {lit} lit"),
                PhaseName::FullMoon => assert!(lit > 0.91, "full moon {lit} lit"),
                PhaseName::FirstQuarter | PhaseName::LastQuarter => {
                    assert!((0.2..0.8).contains(&lit), "a quarter {lit} lit");
                }
                PhaseName::WaxingCrescent | PhaseName::WaningCrescent => {
                    assert!(lit < 0.6, "a crescent {lit} lit");
                }
                PhaseName::WaxingGibbous | PhaseName::WaningGibbous => {
                    assert!(lit > 0.4, "a gibbous moon only {lit} lit");
                }
            }
        }
    }

    /// Published principal phases for September 2024, JST: new moon on the
    /// 3rd, first quarter on the 11th, full moon on the 18th, last quarter on
    /// the 25th.
    #[test]
    fn the_phases_of_september_2024_fall_where_the_almanac_puts_them() {
        let phases = principal_phases_in_month(2024, 9, JAPAN);
        assert_eq!(phases.len(), 4);
        assert_eq!(
            phases.new_moon().map(|event| event.day),
            Some(from_year_month_day(2024, 9, 3))
        );
        assert_eq!(
            phases.first_quarter().map(|event| event.day),
            Some(from_year_month_day(2024, 9, 11))
        );
        assert_eq!(
            phases.full_moon().map(|event| event.day),
            Some(from_year_month_day(2024, 9, 18))
        );
        assert_eq!(
            phases.last_quarter().map(|event| event.day),
            Some(from_year_month_day(2024, 9, 25))
        );
    }

    #[test]
    fn a_month_holds_four_or_five_principal_phases_in_date_order() {
        for year in 2000..2030 {
            for month in 1..=12u8 {
                let phases = principal_phases_in_month(year, month, JAPAN);
                assert!(
                    (3..=5).contains(&phases.len()),
                    "{year}-{month:02} held {} phases",
                    phases.len()
                );
                assert!(!phases.is_empty());
                let mut previous: Option<PhaseEvent> = None;
                for event in phases {
                    if let Some(earlier) = previous {
                        assert!(
                            event.moment.0 > earlier.moment.0,
                            "{year}-{month:02} listed phases out of order"
                        );
                    }
                    assert_eq!(
                        crate::gregorian::year_month_day_from_rd(event.day).1,
                        month,
                        "a phase escaped its month"
                    );
                    previous = Some(event);
                }
            }
        }
    }

    /// Over thirty years a Gregorian month holds five principal phases often
    /// enough to be worth handling — that is the "blue moon" case.
    #[test]
    fn some_months_hold_a_second_full_moon() {
        let mut blue_moons = 0;
        for year in 2000..2030 {
            for month in 1..=12u8 {
                let phases = principal_phases_in_month(year, month, JAPAN);
                if phases.second(MoonPhase::Full).is_some() {
                    blue_moons += 1;
                    assert_eq!(phases.len(), 5);
                }
            }
        }
        // A second full moon in a calendar month happens about every 2.7
        // years.
        assert!(
            (7..=15).contains(&blue_moons),
            "{blue_moons} blue moons in thirty years"
        );
    }

    /// February can miss a phase entirely: 28 days is shorter than a
    /// lunation, so a February with no full moon happens a few times a
    /// century.
    #[test]
    fn february_can_lack_a_full_moon() {
        let mut missing = 0;
        for year in 1900..2100 {
            if principal_phases_in_month(year, 2, JAPAN)
                .full_moon()
                .is_none()
            {
                missing += 1;
            }
        }
        assert!(
            missing > 0,
            "no February in two centuries lacked a full moon"
        );
        assert!(missing < 12, "{missing} Februaries is too many");
    }

    /// Published 中秋の名月 dates, JST: these are printed in every Japanese
    /// calendar and reported in the newspapers each year.
    #[test]
    fn the_mid_autumn_moon_falls_where_the_almanacs_put_it() {
        let expected = [
            (2020, 10, 1),
            (2021, 9, 21),
            (2022, 9, 10),
            (2023, 9, 29),
            (2024, 9, 17),
            (2025, 10, 6),
        ];
        for (year, month, day) in expected {
            assert_eq!(
                mid_autumn_moon(year, JAPAN),
                Some(from_year_month_day(year, month, day)),
                "中秋の名月 of {year}"
            );
        }
    }

    /// Published 十三夜 dates, JST.
    #[test]
    fn the_thirteenth_night_falls_where_the_almanacs_put_it() {
        let expected = [
            (2021, 10, 18),
            (2022, 10, 8),
            (2023, 10, 27),
            (2024, 10, 15),
            (2025, 11, 2),
        ];
        for (year, month, day) in expected {
            assert_eq!(
                thirteenth_night(year, JAPAN),
                Some(from_year_month_day(year, month, day)),
                "十三夜 of {year}"
            );
        }
    }

    /// 十三夜 comes about a month after 十五夜 — unless a leap eighth month
    /// falls between them, in which case it comes about two. 1995 had a
    /// 閏八月 and its two viewing nights were 57 days apart, which is not a
    /// bug but the calendar working.
    #[test]
    fn the_thirteenth_night_follows_the_mid_autumn_moon_by_a_month_or_by_two() {
        let mut intercalated = 0;
        for year in 1980..2060 {
            let fifteenth = mid_autumn_moon(year, JAPAN);
            let thirteenth = thirteenth_night(year, JAPAN);
            let (Some(fifteenth), Some(thirteenth)) = (fifteenth, thirteenth) else {
                panic!("{year} was missing one of the two moon-viewing nights");
            };
            let gap = thirteenth.0 - fifteenth.0;
            if gap > 40 {
                intercalated += 1;
                assert!(
                    (55..=61).contains(&gap),
                    "{year}: {gap} days, which is neither one month nor two"
                );
                // A leap eighth month is the only thing that can do this.
                let between = crate::lunisolar::lunisolar_day(Rd(fifteenth.0 + 30), JAPAN);
                assert!(
                    between.leap_month,
                    "{year}: the long gap was not a leap month"
                );
            } else {
                assert!(
                    (26..=31).contains(&gap),
                    "{year}: {gap} days between the two viewings"
                );
            }
        }
        assert!(
            (1..=6).contains(&intercalated),
            "{intercalated} leap eighth months in eighty years"
        );
    }

    /// The tradition dates 十五夜 by the calendar, not by the sky, so it is
    /// the actual full moon rather less than half the time. Stating that as a
    /// test stops anyone "fixing" it later.
    #[test]
    fn the_mid_autumn_moon_is_often_not_the_full_moon() {
        let mut exact = 0;
        let mut total = 0;
        for year in 1980..2060 {
            let Some(night) = mid_autumn_moon(year, JAPAN) else {
                continue;
            };
            total += 1;
            let phases = principal_phases_in_month(
                crate::gregorian::year_month_day_from_rd(night).0,
                crate::gregorian::year_month_day_from_rd(night).1,
                JAPAN,
            );
            if phases.full_moon().map(|event| event.day) == Some(night) {
                exact += 1;
            }
            // Whether or not it is exact, the Moon is near enough full to be
            // worth looking at.
            assert!(
                illuminated_fraction(night, JAPAN) > 0.93,
                "{year}: the harvest moon was only {} lit",
                illuminated_fraction(night, JAPAN)
            );
        }
        assert!(total > 70);
        assert!(
            exact * 2 < total,
            "{exact} of {total} were the exact full moon, which is suspiciously many"
        );
    }

    #[test]
    fn the_month_phase_list_reports_its_own_length() {
        let phases = principal_phases_in_month(2024, 9, JAPAN);
        let iterator = phases.into_iter();
        assert_eq!(iterator.len(), phases.len());
        assert_eq!(iterator.count(), 4);
        assert!(phases.first(MoonPhase::New).is_some());
        assert!(phases.second(MoonPhase::New).is_none());
    }
}
