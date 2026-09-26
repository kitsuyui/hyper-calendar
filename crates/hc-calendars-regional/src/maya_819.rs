//! The Maya 819-day count, with its four colour-directions and Linden and
//! Bricker's twenty stations — `maya-819` and `maya-819-gmt2`.
//!
//! The count is written up in a section of
//! [`docs/systems/mesoamerican-counts.md`](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/systems/mesoamerican-counts.md),
//! beside the Long Count, the Tzolkʼin, the Haabʼ and the Calendar Round it
//! is read with: the stations and their colours and directions, the base
//! three days before `0.0.0.0.0`, the initial date of Palenque's Temple of
//! the Cross worked by hand, the twenty-station cycle, and the sources.
//! This page summarises it and states the code's own facts.
//!
//! # What this is
//!
//! A Classic inscription that records the count gives, after its Initial
//! Series, a distance back to the last **station**, a day on which a count
//! of 819 days, 7 × 9 × 13, came round; each station stands under one of
//! the four quarters and its colour, and the next, 819 days on, under the
//! next (Berlin and Kelley 1961, `berlin1961`, and Thompson 1943,
//! `thompson1943`, neither read, as `vanlaningham-819` and
//! `lounsbury1976` give them):
//!
//! | Station | Direction | Colour |
//! | --- | --- | --- |
//! | 1st, 5th, … | East, *likin* | red, *chak* |
//! | 2nd, 6th, … | South, *nohol* | yellow, *kan* |
//! | 3rd, 7th, … | West, *chikin* | black, *ek* |
//! | 4th, 8th, … | North, *xaman* | white, *sak* |
//!
//! The count runs from a base **three days before `0.0.0.0.0`**, the day
//! 1 Caban 5 Cumku, an eastern station (`vanlaningham-819`,
//! `macleod2012`): a day *d* days after `0.0.0.0.0` is (*d* + 3) mod 819
//! days past its station. Because 13 divides 819, every station is a day
//! numbered 1 in the Tzolkʼin, and because 819 is one short of 820, each
//! station's day-sign is the one before the last's.
//!
//! Linden and Bricker (2023, `linden2023`, the abstract read) read the
//! count as twenty stations rather than four, 20 × 819 = 16 380 days,
//! which is also the least common multiple of 819 and 260, the span over
//! which the synodic periods of the five visible planets come round to
//! stations. This module carries that longer cycle: a date is the
//! **round** of 16 380 days, the **station** within it, 1 to 20, and the
//! days **elapsed** since the station, 0 to 818. The stations are numbered
//! from the base, which this library chose: the paper's own numbering was
//! not read. The four colour-directions are the station's position in
//! fours, so the station fixes both, and the station fixes its Tzolkʼin
//! day too, 1 and a day-sign, so the twenty stations are the twenty
//! day-signs.
//!
//! # Correlation
//!
//! Like the other Maya cycles the count is fixed to the Western calendar by
//! the correlation constant, and it is registered under each of the three
//! published ones, `maya-819` under [`GMT_CORRELATION`], `maya-819-gmt2`
//! under [`GMT_PLUS_TWO_CORRELATION`] and `maya-819-584286` under
//! [`MARTIN_SKIDMORE_CORRELATION`], so that the stations read beside
//! `maya-longcount-gmt2` or `maya-longcount-584286` are anchored as it is.

use hc_calendar::fields::ExtraFields;
use hc_calendar::shape::CycleShape;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

#[allow(
    unused_imports,
    reason = "the correlated! macro's documentation links to it"
)]
use crate::maya::MayaLongCountCalendar;
use crate::maya::{
    GMT_CORRELATION, GMT_PLUS_TWO_CORRELATION, MARTIN_SKIDMORE_CORRELATION, TzolkinPosition,
    correlated,
};

/// Days from one station to the next: 7 × 9 × 13.
pub const STATION_DAYS: i64 = 819;

/// The stations in Linden and Bricker's cycle.
pub const STATIONS: u8 = 20;

/// Days in Linden and Bricker's cycle: 20 × 819, the least common multiple
/// of 819 and 260.
pub const CYCLE_DAYS: i64 = STATION_DAYS * STATIONS as i64;

/// How many days before `0.0.0.0.0` the count's base falls: 1 Caban
/// 5 Cumku (`vanlaningham-819`).
pub const BASE_BEFORE_EPOCH: i64 = 3;

/// The four directions in the order the stations take them, from the east
/// of the base, in the Yucatec forms `vanlaningham-819` prints.
pub const DIRECTIONS: [&str; 4] = ["Likin", "Nohol", "Chikin", "Xaman"];

/// The four colours in the same order.
pub const COLOURS: [&str; 4] = ["Chak", "Kan", "Ek", "Sak"];

/// The quarter a station stands under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    /// East, *likin*, red.
    East,
    /// South, *nohol*, yellow.
    South,
    /// West, *chikin*, black.
    West,
    /// North, *xaman*, white.
    North,
}

impl Direction {
    /// The four, in the order the stations take them.
    pub const ALL: [Self; 4] = [Self::East, Self::South, Self::West, Self::North];

    /// The direction's position in [`Direction::ALL`], 0 to 3.
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::East => 0,
            Self::South => 1,
            Self::West => 2,
            Self::North => 3,
        }
    }

    /// The direction's Yucatec name, from [`DIRECTIONS`].
    #[must_use]
    pub const fn name(self) -> &'static str {
        DIRECTIONS[self.index()]
    }

    /// The colour of the direction's stations, from [`COLOURS`].
    #[must_use]
    pub const fn colour(self) -> &'static str {
        COLOURS[self.index()]
    }
}

/// A date in the 819-day count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Maya819Date {
    /// Complete 16 380-day cycles since the base, 1 Caban 5 Cumku three
    /// days before `0.0.0.0.0`.
    pub round: i64,
    /// The station within the cycle, 1 to 20, the base's the first.
    pub station: u8,
    /// Days since the station, 0 to 818: the distance an inscription
    /// counts back.
    pub elapsed: u16,
}

impl Maya819Date {
    /// The quarter the station stands under.
    #[must_use]
    pub const fn direction(self) -> Direction {
        Direction::ALL[(self.station.saturating_sub(1) % 4) as usize]
    }

    /// The station's day in the Tzolkʼin: always numbered 1, its day-sign
    /// one before the previous station's, 1 Caban at the first station.
    #[must_use]
    pub const fn station_tzolkin(self) -> TzolkinPosition {
        let before = self.station.saturating_sub(1) as i64;
        // Caban is the seventeenth day-sign; each station steps one back.
        TzolkinPosition::new(1, ((16 - before).rem_euclid(20) + 1) as u8)
    }
}

/// The 819-day count under a correlation constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Maya819Calendar {
    correlation: i64,
}

correlated!(
    Maya819Calendar,
    "Maya 819-day count",
    "maya-819",
    "maya-819-gmt2",
    "maya-819-584286"
);

impl Maya819Calendar {
    /// The fixed day of the count's base, 1 Caban 5 Cumku, three days
    /// before this correlation's `0.0.0.0.0`.
    #[must_use]
    pub const fn base(self) -> Rd {
        Rd(self.epoch().0 - BASE_BEFORE_EPOCH)
    }

    /// The fixed day of a date's station.
    ///
    /// # Errors
    ///
    /// As [`Calendar::to_fixed`], for a station or distance out of range.
    pub fn station_day(self, date: Maya819Date) -> CalendarResult<Rd> {
        self.to_fixed(Maya819Date { elapsed: 0, ..date })
    }
}

/// The count's named cycles: the four directions and their colours. The
/// twenty stations and the 819 days are numbered, and carried as fields.
const SHAPE: &[CycleShape] = &[
    CycleShape::named("direction", &DIRECTIONS),
    CycleShape::named("colour", &COLOURS),
];

/// Narrow a field to the station it names.
fn station_of(value: i64) -> CalendarResult<u8> {
    u8::try_from(value).map_err(|_| CalendarError::DayOutOfRange)
}

/// Narrow a field to the distance it names.
fn elapsed_of(value: i64) -> CalendarResult<u16> {
    u16::try_from(value).map_err(|_| CalendarError::DayOutOfRange)
}

impl Calendar for Maya819Calendar {
    type Date = Maya819Date;

    /// A cycle with no year: the `year` field carries the round.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    /// The four directions and the four colours.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id(),
            english_name: self.english_name(),
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
            native_locales: &["yua"],
        }
    }

    /// # Errors
    ///
    /// [`CalendarError::DayOutOfRange`] for a station outside 1 to 20 or a
    /// distance past 818; [`CalendarError::Overflow`] for a round too far
    /// out to count.
    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        if date.station == 0 || date.station > STATIONS || i64::from(date.elapsed) >= STATION_DAYS {
            return Err(CalendarError::DayOutOfRange);
        }
        let within = i64::from(date.station - 1) * STATION_DAYS + i64::from(date.elapsed);
        date.round
            .checked_mul(CYCLE_DAYS)
            .and_then(|days| days.checked_add(self.base().0 + within))
            .map(Rd)
            .ok_or(CalendarError::Overflow)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let count =
            rd.0.checked_sub(self.base().0)
                .ok_or(CalendarError::Overflow)?;
        let within = count.rem_euclid(CYCLE_DAYS);
        Ok(Maya819Date {
            round: count.div_euclid(CYCLE_DAYS),
            station: (within / STATION_DAYS) as u8 + 1,
            elapsed: (within % STATION_DAYS) as u16,
        })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("station", date.station.into())?;
        extra.set("elapsed", date.elapsed.into())?;
        extra.set("direction", date.direction().index() as i64 + 1)?;
        Ok(DateFields {
            era: None,
            year: date.round,
            month: None,
            day: None,
            leap_day: false,
            extra,
        })
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let date = Maya819Date {
            round: fields.year,
            station: station_of(fields.extra.require("station")?)?,
            elapsed: elapsed_of(fields.extra.require("elapsed")?)?,
        };
        if let Some(direction) = fields.extra.get("direction")
            && direction != date.direction().index() as i64 + 1
        {
            return Err(CalendarError::DayOutOfRange);
        }
        self.to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maya::{
        EPOCH, HaabPosition, MayaHaabCalendar, MayaLongCountCalendar, MayaLongCountDate,
        MayaTzolkinCalendar,
    };

    #[test]
    fn the_two_correlations_are_separate_calendars_two_days_apart() {
        assert_eq!(Maya819Calendar::GMT.id(), CalendarId("maya-819"));
        assert_eq!(
            Maya819Calendar::GMT_PLUS_TWO.id(),
            CalendarId("maya-819-gmt2")
        );
        assert_eq!(Maya819Calendar::default(), Maya819Calendar::GMT);
        assert_eq!(
            Maya819Calendar::GMT_PLUS_TWO.base().0 - Maya819Calendar::GMT.base().0,
            2
        );
        assert_eq!(
            Maya819Calendar::MARTIN_SKIDMORE.id(),
            CalendarId("maya-819-584286")
        );
        assert_eq!(
            Maya819Calendar::MARTIN_SKIDMORE.base().0 - Maya819Calendar::GMT.base().0,
            3
        );
    }

    #[test]
    fn the_base_is_one_caban_five_cumku_an_eastern_station() {
        // Van Laningham: the count "begins on day -3 of linear time,
        // 1 Kaban 5 Kumk'u"; MacLeod and Kinsman: a station in the east.
        let base = Maya819Calendar::GMT.base();
        assert_eq!(base, Rd(EPOCH.0 - 3));
        let date = Maya819Calendar::GMT.from_fixed(base).expect("any day");
        assert_eq!(
            date,
            Maya819Date {
                round: 0,
                station: 1,
                elapsed: 0
            }
        );
        assert_eq!(date.direction(), Direction::East);
        assert_eq!(date.direction().colour(), "Chak");
        let tzolkin = MayaTzolkinCalendar::GMT.from_fixed(base).expect("any day");
        assert_eq!(tzolkin.position.to_string(), "1 Caban");
        assert_eq!(date.station_tzolkin(), tzolkin.position);
        let haab = MayaHaabCalendar::GMT.from_fixed(base).expect("any day");
        assert_eq!(haab.position, HaabPosition::new(18, 5));
    }

    #[test]
    fn the_temple_of_the_cross_stands_twenty_days_after_a_south_station() {
        // Lounsbury (1976): the initial date of the Temple of the Cross,
        // 12.19.13.4.0 8 Ahau 18 Zec in the era before 13.0.0.0.0, is
        // "20 days after a south station in the 4x819-day cycle", and so
        // "duplicates exactly the position of Pacal's birth date",
        // 9.8.9.13.0 8 Ahau 13 Pop. The era before ends where this one
        // begins, so the initial date is 1.0.0.0.0 − 12.19.13.4.0 =
        // 2 440 days before 0.0.0.0.0. Under each of the three constants
        // alike.
        for long_count in MayaLongCountCalendar::ALL {
            let count = Maya819Calendar::beside(long_count);
            let before_era = 13 * 144_000 - (12 * 144_000 + 19 * 7_200 + 13 * 360 + 4 * 20);
            assert_eq!(before_era, 2_440);
            let cross = Rd(long_count.epoch().0 - before_era);
            let pakal = long_count
                .to_fixed(MayaLongCountDate::new(9, 8, 9, 13, 0))
                .expect("in range");
            for day in [cross, pakal] {
                let date = count.from_fixed(day).expect("any day");
                assert_eq!(date.elapsed, 20, "{day}");
                assert_eq!(date.direction(), Direction::South, "{day}");
                assert_eq!(date.station_tzolkin().to_string(), "1 Ahau");
                let station = count.station_day(date).expect("valid");
                assert_eq!(station, Rd(day.0 - 20));
            }
            let tzolkin = MayaTzolkinCalendar::beside(long_count).from_fixed(cross);
            assert_eq!(tzolkin.expect("any day").position.to_string(), "8 Ahau");
            let haab = MayaHaabCalendar::beside(long_count)
                .from_fixed(cross)
                .expect("any day");
            assert_eq!(haab.position.to_string(), "18 Tzec");
        }
    }

    #[test]
    fn van_laningham_s_worked_example_under_584_285() {
        // "If your preferred CC is 584285 and your JPDAY is, for example,
        // 2450765 … it is 801 days past the last 819 day station", the
        // station 1 Kawak 7 Mol, black and west.
        let day = Rd(2_450_765 - hc_calendar::fixed::JDN_OF_RD_ZERO);
        let date = Maya819Calendar::GMT_PLUS_TWO
            .from_fixed(day)
            .expect("any day");
        assert_eq!(date.elapsed, 801);
        assert_eq!(date.direction(), Direction::West);
        assert_eq!(date.direction().colour(), "Ek");
        assert_eq!(date.station_tzolkin().to_string(), "1 Cauac");
        let station = Maya819Calendar::GMT_PLUS_TWO
            .station_day(date)
            .expect("valid");
        let haab = MayaHaabCalendar::GMT_PLUS_TWO
            .from_fixed(station)
            .expect("any day");
        assert_eq!(haab.position.to_string(), "7 Mol");
        // Under the other constant the same day is two days further on.
        let other = Maya819Calendar::GMT.from_fixed(day).expect("any day");
        assert_eq!(other.elapsed, 803);
    }

    #[test]
    fn the_twenty_stations_are_the_twenty_day_signs_numbered_one() {
        assert_eq!(CYCLE_DAYS, 16_380);
        // 16 380 is the least common multiple of 819 and 260.
        assert_eq!(CYCLE_DAYS % 260, 0);
        assert_eq!(CYCLE_DAYS % STATION_DAYS, 0);
        for divisor in 1..CYCLE_DAYS {
            assert!(
                divisor % 260 != 0 || divisor % STATION_DAYS != 0,
                "{divisor}"
            );
        }
        let calendar = Maya819Calendar::GMT;
        let mut signs = alloc::vec::Vec::new();
        for station in 1..=STATIONS {
            let date = Maya819Date {
                round: 3,
                station,
                elapsed: 0,
            };
            let day = calendar.to_fixed(date).expect("valid");
            let tzolkin = MayaTzolkinCalendar::GMT.from_fixed(day).expect("any day");
            assert_eq!(tzolkin.position, date.station_tzolkin(), "{station}");
            assert_eq!(tzolkin.position.number, 1);
            signs.push(tzolkin.position.name);
            assert_eq!(
                date.direction(),
                Direction::ALL[usize::from((station - 1) % 4)]
            );
        }
        signs.sort_unstable();
        signs.dedup();
        assert_eq!(signs.len(), 20);
    }

    #[test]
    fn every_day_of_a_whole_cycle_round_trips_and_steps_by_one() {
        for calendar in Maya819Calendar::ALL {
            let start = calendar.base().0 - 5;
            let mut previous = calendar.from_fixed(Rd(start - 1)).expect("any day");
            for day in start..start + CYCLE_DAYS + 10 {
                let date = calendar.from_fixed(Rd(day)).expect("any day");
                assert_eq!(calendar.to_fixed(date), Ok(Rd(day)), "{date:?}");
                if date.elapsed > 0 {
                    assert_eq!(date.elapsed, previous.elapsed + 1);
                    assert_eq!(date.station, previous.station);
                } else {
                    assert_eq!(previous.elapsed, 818);
                }
                let fields = calendar.to_fields(date).expect("fields");
                assert_eq!(calendar.from_fields(&fields), Ok(date));
                previous = date;
            }
        }
    }

    #[test]
    fn impossible_dates_are_refused() {
        let calendar = Maya819Calendar::GMT;
        for (station, elapsed) in [(0, 0), (21, 0), (1, 819)] {
            assert_eq!(
                calendar.to_fixed(Maya819Date {
                    round: 0,
                    station,
                    elapsed
                }),
                Err(CalendarError::DayOutOfRange)
            );
        }
        assert_eq!(
            calendar.to_fixed(Maya819Date {
                round: i64::MAX,
                station: 1,
                elapsed: 0
            }),
            Err(CalendarError::Overflow)
        );
        let mut fields = calendar
            .to_fields(calendar.from_fixed(Rd(0)).expect("any day"))
            .expect("fields");
        assert!(calendar.from_fields(&fields).is_ok());
        let direction = fields.extra.get("direction").expect("set");
        fields
            .extra
            .set("direction", direction % 4 + 1)
            .expect("settable");
        assert_eq!(
            calendar.from_fields(&fields),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            calendar.is_leap_year(3),
            Err(CalendarError::UnsupportedField("year"))
        );
    }
}
