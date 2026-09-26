//! The Akan *Adaduanan*, the 42-day cycle of the six-day and seven-day
//! weeks.
//!
//! The Akan of Ghana kept a six-day week, the *nnanson* — "seven days" by
//! inclusive counting — and run it against the seven-day week, the
//! *nnawɔtwe*, "eight days"; the two return to the same pair after
//! forty-two days, the *Adaduanan*, "forty days" by the same counting. A
//! day is named by its pair, six-day name first: *Fo-Dwo*, *Kuru-Wukuo*,
//! *Kuru-Kwasi*. Four pairs are the *dabɔne*, the sacred days on which
//! the stool rites fall: *Fɔdwo* opens the cycle, *Awukudae* is its tenth
//! day, *Fofi* its nineteenth and *Akwasidae* — the Sunday of *Kuru* — its
//! twenty-eighth, every sixth Sunday.
//!
//! # Anchor
//!
//! The source gives the first four *dabɔne* of 1978: *Akwasidae* on
//! 8 January, *Fɔdwo* on 23 January, *Awukudae* on 1 February and *Fofi*
//! on 10 February, and says the rest "may be calculated infinitely from
//! these by adding or subtracting six-week intervals". [`EPOCH`] is that
//! *Fɔdwo*, Monday 23 January 1978, ordinal zero of the cycle; the
//! module's arithmetic is a modulo from it. The source's list of the nine
//! *Akwasidae* of 1978 carries two dates the six-week rule does not give,
//! "2 March" and "30 July", where the rule gives 2 April and 6 August; the
//! other seven agree, and the rule is what is carried.
//!
//! # A cycle, not a calendar
//!
//! Nine cycles are 378 days and eight are 336, so the Adaduanan does not
//! make a year, and the names the cycles bear within a year vary by place
//! and time; none of that is modelled. [`AkanDate`] carries a `round`, the
//! number of complete cycles since the epoch, for the reason the crate
//! documentation gives, and nothing above it. The 28-day *bosome* month is
//! not carried either.
//!
//! Source: Wikipedia, "Akan calendar" (`wikipedia-akan-calendar`),
//! retrieved 2026-09-22, for the two weeks and their names, the 42-day
//! table, the four *dabɔne* and the 1978 dates. That is a secondary source;
//! the study it rests on, K. Bartle, "Forty Days: The Akan Calendar",
//! *Africa* 48(1), 1978, was not read, and is what would replace it. The
//! published code of *Calendrical Calculations* carries the same 42-day
//! cycle of six- and seven-day names (`reingold2018code`, `akan-day-name`,
//! `akan-day-name-epoch`), without the names.

use core::fmt;

use hc_calendar::fields::ExtraFields;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind, weekday::DayCycle,
};

/// The length of the six-day week.
pub const NNANSON_CYCLE: i64 = 6;

/// The length of the *Adaduanan*: six days against seven.
pub const ADADUANAN_CYCLE: i64 = 42;

/// The six *nnanson* day names, in cycle order, as the source's 42-day
/// table spells them.
pub const NNANSON: [&str; 6] = ["Fo", "Nwuna", "Nkyi", "Kuru", "Kwa", "Mono"];

/// The seven *nnawɔtwe* day names, Sunday first.
pub const NNAWOTWE: [&str; 7] = [
    "Kwasiada", "Ɛdwoada", "Ɛbenada", "Wukuada", "Yawuada", "Efiada", "Memeneda",
];

/// The short forms of the weekday names used in a day's compound name,
/// Sunday first: *Kuru-Kwasi*, *Fo-Dwo*.
pub const NNAWOTWE_SHORT: [&str; 7] = ["Kwasi", "Dwo", "Bena", "Wukuo", "Ya", "Afi", "Mene"];

/// The four *dabɔne* by their 1-based day of the cycle.
pub const DABONE: [(u8, &str); 4] = [
    (1, "Fɔdwo"),
    (10, "Awukudae"),
    (19, "Fofi"),
    (28, "Akwasidae"),
];

/// The fixed day that is *Fɔdwo*, ordinal zero of the cycle: Monday
/// 23 January 1978, the first *Fɔdwo* the source dates.
pub const EPOCH: Rd = Rd(722_107);

/// A day of the *Adaduanan*: its six-day name, its weekday, and the cycle
/// it falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AkanDate {
    /// Complete 42-day cycles elapsed since [`EPOCH`].
    pub round: i64,
    /// The *nnanson* day, 1 for Fo through 6 for Mono.
    pub nnanson: u8,
    /// The weekday, 1 for Kwasiada (Sunday) through 7 for Memeneda
    /// (Saturday).
    pub nnawotwe: u8,
}

impl AkanDate {
    /// A day, without validation; the calendar validates it.
    #[must_use]
    pub const fn new(round: i64, nnanson: u8, nnawotwe: u8) -> Self {
        Self {
            round,
            nnanson,
            nnawotwe,
        }
    }

    const fn in_range(self) -> bool {
        self.nnanson >= 1 && self.nnanson <= 6 && self.nnawotwe >= 1 && self.nnawotwe <= 7
    }

    /// The *nnanson* name.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when `nnanson` is not in
    /// `1..=6`.
    pub const fn nnanson_name(self) -> CalendarResult<&'static str> {
        if self.nnanson == 0 || self.nnanson > 6 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(NNANSON[(self.nnanson - 1) as usize])
    }

    /// The weekday name.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when `nnawotwe` is not in
    /// `1..=7`.
    pub const fn nnawotwe_name(self) -> CalendarResult<&'static str> {
        if self.nnawotwe == 0 || self.nnawotwe > 7 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(NNAWOTWE[(self.nnawotwe - 1) as usize])
    }

    /// The position within the 42-day cycle, 0 to 41, *Fɔdwo* being 0.
    ///
    /// The coefficients solve the two congruences at once: 7 leaves 1
    /// modulo 6 and 36 leaves 1 modulo 7, and the weekday is counted from
    /// the Monday the cycle opens on.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] for an out-of-range
    /// position.
    pub const fn ordinal(self) -> CalendarResult<i64> {
        if !self.in_range() {
            return Err(CalendarError::DayOutOfRange);
        }
        let six = (self.nnanson - 1) as i64;
        let from_monday = (self.nnawotwe as i64 - 2).rem_euclid(7);
        Ok((7 * six + 36 * from_monday).rem_euclid(ADADUANAN_CYCLE))
    }

    /// The day `ordinal` days into a cycle.
    #[must_use]
    pub const fn from_ordinal(round: i64, ordinal: i64) -> Self {
        let ordinal = ordinal.rem_euclid(ADADUANAN_CYCLE);
        Self {
            round,
            nnanson: (ordinal.rem_euclid(NNANSON_CYCLE) + 1) as u8,
            // Ordinal zero is a Monday, index 2 Sunday-first.
            nnawotwe: ((ordinal + 1).rem_euclid(7) + 1) as u8,
        }
    }

    /// The *dabɔne* this day is, if it is one of the four.
    #[must_use]
    pub const fn dabone(self) -> Option<&'static str> {
        let ordinal = match self.ordinal() {
            Ok(ordinal) => ordinal,
            Err(_) => return None,
        };
        let mut index = 0;
        while index < DABONE.len() {
            let (day, name) = DABONE[index];
            if ordinal + 1 == day as i64 {
                return Some(name);
            }
            index += 1;
        }
        None
    }
}

impl fmt::Display for AkanDate {
    /// Writes the day as its compound name, six-day name first, as in
    /// `Kuru-Kwasi`. The source elides the vowel of *Afi* after *Fo*, writing
    /// `Fo-Fi`; this writes the parts unchanged.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.nnanson_name(), self.nnawotwe_name()) {
            (Ok(six), Ok(_)) => write!(f, "{six}-{}", NNAWOTWE_SHORT[(self.nnawotwe - 1) as usize]),
            _ => write!(f, "?{} ?{}", self.nnanson, self.nnawotwe),
        }
    }
}

/// The Akan *Adaduanan* cycle.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AkanCalendar;

/// The six-day week as a plain [`DayCycle`].
pub const NNANSON_DAY_CYCLE: DayCycle = DayCycle::new(NNANSON_CYCLE as u16, EPOCH);

/// The 42-day cycle as a plain [`DayCycle`].
pub const ADADUANAN_DAY_CYCLE: DayCycle = DayCycle::new(ADADUANAN_CYCLE as u16, EPOCH);

/// The *nnanson* day of a fixed day, 1 for Fo through 6 for Mono.
#[must_use]
pub const fn nnanson_of(rd: Rd) -> u8 {
    (rd.0 - EPOCH.0).rem_euclid(NNANSON_CYCLE) as u8 + 1
}

impl Calendar for AkanCalendar {
    type Date = AkanDate;

    /// Unrecorded: the source dates the *dabɔne* of 1978 and nothing about
    /// when the cycle began or whether it is kept today.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// The six-day *nnanson* and the seven-day week it runs against.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        const SHAPE: &[hc_calendar::shape::CycleShape] = &[
            hc_calendar::shape::CycleShape::named("nnanson", &NNANSON),
            hc_calendar::shape::CycleShape::named(hc_calendar::shape::WEEKDAY, &NNAWOTWE),
        ];
        SHAPE
    }

    /// A cycle has no year: the `year` field carries the round.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("akan"),
            english_name: "Akan Adaduanan (42-day cycle)",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
            native_locales: &["ak"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        Ok(Rd(EPOCH.0
            + date.round * ADADUANAN_CYCLE
            + date.ordinal()?))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let count = rd.0 - EPOCH.0;
        Ok(AkanDate::from_ordinal(
            count.div_euclid(ADADUANAN_CYCLE),
            count,
        ))
    }

    /// Describes the day with the 1-based position in the 42-day cycle as
    /// the `day` field and the two component weeks as extra fields. There
    /// is no month.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::Overflow`] if the extra-field set fills,
    /// which two fields cannot make happen.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("nnanson", date.nnanson.into())?;
        extra.set("nnawotwe", date.nnawotwe.into())?;
        Ok(DateFields {
            era: None,
            year: date.round,
            month: None,
            day: Some(u8::try_from(date.ordinal()? + 1).unwrap_or(1)),
            leap_day: false,
            extra,
        })
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        Ok(AkanDate {
            round: fields.year,
            nnanson: u8::try_from(fields.extra.require("nnanson")?)
                .map_err(|_| CalendarError::DayOutOfRange)?,
            nnawotwe: u8::try_from(fields.extra.require("nnawotwe")?)
                .map_err(|_| CalendarError::DayOutOfRange)?,
        })
    }
}

/// The day's pair for a fixed day, with the weekday as a [`Weekday`].
#[must_use]
pub fn weekday_and_nnanson(rd: Rd) -> (Weekday, u8) {
    (Weekday::from_rd(rd), nnanson_of(rd))
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;

    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    #[test]
    fn the_epoch_is_the_fodwo_of_23_january_1978() {
        assert_eq!(EPOCH, greg(1978, 1, 23));
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Monday);
        let day = AkanCalendar.from_fixed(EPOCH).expect("any day");
        assert_eq!(day.to_string(), "Fo-Dwo");
        assert_eq!(day.dabone(), Some("Fɔdwo"));
        assert_eq!((day.round, day.ordinal()), (0, Ok(0)));
    }

    #[test]
    fn the_first_four_dabone_of_1978_are_the_sources() {
        let awukudae = AkanCalendar.from_fixed(greg(1978, 2, 1)).expect("any day");
        assert_eq!(awukudae.to_string(), "Kuru-Wukuo");
        assert_eq!(awukudae.dabone(), Some("Awukudae"));
        let fofi = AkanCalendar.from_fixed(greg(1978, 2, 10)).expect("any day");
        assert_eq!(fofi.to_string(), "Fo-Afi");
        assert_eq!(fofi.dabone(), Some("Fofi"));
        let akwasidae = AkanCalendar.from_fixed(greg(1978, 1, 8)).expect("any day");
        assert_eq!(akwasidae.to_string(), "Kuru-Kwasi");
        assert_eq!(akwasidae.dabone(), Some("Akwasidae"));
        assert_eq!(akwasidae.round, -1);
    }

    #[test]
    fn akwasidae_falls_every_sixth_sunday_through_1978() {
        // The source's list by the six-week rule it states; its "2 March"
        // and "30 July" are 2 April and 6 August by that rule.
        let sundays = [
            (1, 8),
            (2, 19),
            (4, 2),
            (5, 14),
            (6, 25),
            (8, 6),
            (9, 17),
            (10, 29),
            (12, 10),
        ];
        for (month, day) in sundays {
            let rd = greg(1978, month, day);
            assert_eq!(Weekday::from_rd(rd), Weekday::Sunday, "{month}-{day}");
            let date = AkanCalendar.from_fixed(rd).expect("any day");
            assert_eq!(date.dabone(), Some("Akwasidae"), "{month}-{day}");
            assert_eq!(date.ordinal(), Ok(27));
        }
        // No other Sunday of the year is one.
        let mut count = 0;
        for offset in 0..365 {
            let rd = Rd(greg(1978, 1, 1).0 + offset);
            if AkanCalendar.from_fixed(rd).expect("any day").dabone() == Some("Akwasidae") {
                count += 1;
            }
        }
        assert_eq!(count, 9);
    }

    #[test]
    fn the_cycle_is_forty_two_days_and_visits_every_pair() {
        assert_eq!(ADADUANAN_CYCLE, NNANSON_CYCLE * 7);
        let mut seen = [false; 42];
        for offset in 0..42 {
            let date = AkanCalendar
                .from_fixed(Rd(greg(2026, 1, 1).0 + offset))
                .expect("any day");
            let ordinal = date.ordinal().expect("valid") as usize;
            assert!(!seen[ordinal], "repeat at {offset}");
            seen[ordinal] = true;
        }
        assert!(seen.iter().all(|hit| *hit));
        // The table's first column: Fo-Dwo, Nwuna-Bena, Nkyi-Wukuo, Kuru-Ya,
        // Kwa-Afi, Mono-Mene, Fo-Kwasi.
        let names: Vec<String> = (0..7)
            .map(|offset| {
                AkanCalendar
                    .from_fixed(Rd(EPOCH.0 + offset))
                    .expect("any day")
                    .to_string()
            })
            .collect();
        assert_eq!(
            names,
            [
                "Fo-Dwo",
                "Nwuna-Bena",
                "Nkyi-Wukuo",
                "Kuru-Ya",
                "Kwa-Afi",
                "Mono-Mene",
                "Fo-Kwasi"
            ]
        );
    }

    #[test]
    fn every_day_of_a_whole_cycle_round_trips() {
        for round in [-3i64, 0, 5_000] {
            for ordinal in 0..42 {
                let date = AkanDate::from_ordinal(round, ordinal);
                let rd = AkanCalendar.to_fixed(date).expect("a valid day");
                assert_eq!(AkanCalendar.from_fixed(rd), Ok(date));
                let fields = AkanCalendar.to_fields(date).expect("describable");
                assert_eq!(fields.day, Some(ordinal as u8 + 1));
                assert_eq!(AkanCalendar.from_fields(&fields), Ok(date));
            }
        }
        assert_eq!(
            AkanCalendar.from_fields(&DateFields::new(0)),
            Err(CalendarError::MissingField("nnanson"))
        );
        assert_eq!(
            AkanDate::new(0, 7, 1).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(AkanDate::new(0, 7, 1).dabone(), None);
    }

    #[test]
    fn the_six_day_week_advances_by_one_a_day() {
        let start = greg(2026, 9, 21);
        for offset in 0..50 {
            let here = nnanson_of(Rd(start.0 + offset));
            let next = nnanson_of(Rd(start.0 + offset + 1));
            assert_eq!(here % 6 + 1, next, "{offset}");
        }
        assert_eq!(weekday_and_nnanson(EPOCH), (Weekday::Monday, 1));
    }
}
