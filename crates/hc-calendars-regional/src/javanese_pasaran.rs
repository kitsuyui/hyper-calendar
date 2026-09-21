//! The Javanese *pasaran*, the five-day market week, and the 35-day
//! *wetonan* cycle it makes with the seven-day week.
//!
//! The pasaran is the same five-day cycle as the Balinese *pancawara* —
//! Legi, Pahing, Pon, Wage, Kliwon — under Javanese names. Run against the
//! ordinary seven-day week it produces a 35-day cycle, the *wetonan*, and a
//! person's *weton* is the pair they were born on: Jumat Legi, Rebo Pon.
//! The weton is still in everyday use for choosing wedding dates and for
//! commemorating a death every 35 days.
//!
//! # Neptu
//!
//! Each day of both weeks carries an *urip* or *neptu*, a numerological
//! weight, and a weton's neptu is the two added. Jumat Legi is 6 + 5 = 11.
//! The weights are the same numbers the Balinese Pawukon uses to build its
//! ten-day week; see [`crate::balinese_pawukon`].
//!
//! # Anchor
//!
//! The pasaran is anchored through the Balinese Pawukon's epoch, Julian Day
//! Number 146, because the two cycles are the same cycle and anchoring them
//! separately would be inviting them to drift apart in the source. It works
//! out that fixed day 0 — 0000-12-31 in the proleptic Gregorian calendar —
//! is Ahad Legi, so the wetonan ordinal is simply the fixed day modulo 35.
//!
//! The implementation is checked against the best-known weton in
//! Indonesia: the declaration of independence on 17 August 1945 was **Jumat
//! Legi**, a fact repeated in every Indonesian account of the date.
//!
//! # A cycle, not a calendar
//!
//! Neither the pasaran nor the wetonan counts anything larger than itself,
//! so [`WetonDate`] carries a `round` for the reason the crate
//! documentation gives. This module does **not** implement the Javanese
//! calendar proper (the Sultan Agung lunar year with its windu and its era
//! Anno Javanico); that is a different calendar and is not in this
//! workspace.

use core::fmt;

use hc_calendar::fields::ExtraFields;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind, weekday::DayCycle,
};

/// The length of the market week.
pub const PASARAN_CYCLE: i64 = 5;

/// The length of the *wetonan* cycle: five days against seven.
pub const WETONAN_CYCLE: i64 = 35;

/// The five *pasaran* day names, in cycle order.
///
/// Legi is also called Manis, which is the same word as the Balinese
/// *Umanis*. Other regional variants exist for the remaining four; this
/// crate ships only the names it can source, which are these.
pub const PASARAN: [&str; 5] = ["Legi", "Pahing", "Pon", "Wage", "Kliwon"];

/// The *neptu* of each *pasaran* day, in cycle order.
pub const PASARAN_NEPTU: [i64; 5] = [5, 9, 7, 4, 8];

/// The seven Javanese weekday names, Sunday first.
pub const DINA: [&str; 7] = [
    "Ahad", "Senen", "Selasa", "Rebo", "Kemis", "Jemuwah", "Setu",
];

/// The *neptu* of each weekday, Sunday first.
pub const DINA_NEPTU: [i64; 7] = [5, 4, 3, 7, 8, 6, 9];

/// The fixed day that is Ahad Legi, ordinal zero of the *wetonan*.
///
/// This is `0000-12-31` in the proleptic Gregorian calendar. It is not a
/// meaningful date in Java; it is simply where the two cycles happen to
/// coincide closest to the fixed-day origin, which makes the arithmetic
/// below a plain modulo.
pub const EPOCH: Rd = Rd(0);

/// A *weton*: a weekday, a *pasaran* day, and the cycle they fall in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WetonDate {
    /// Complete 35-day cycles elapsed since [`EPOCH`].
    pub round: i64,
    /// The weekday, 1 for Ahad (Sunday) through 7 for Setu (Saturday).
    pub dina: u8,
    /// The *pasaran* day, 1 for Legi through 5 for Kliwon.
    pub pasaran: u8,
}

impl WetonDate {
    /// A weton, without validation; the calendar validates it.
    #[must_use]
    pub const fn new(round: i64, dina: u8, pasaran: u8) -> Self {
        Self {
            round,
            dina,
            pasaran,
        }
    }

    /// The weekday name.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when `dina` is not in
    /// `1..=7`.
    pub const fn dina_name(self) -> CalendarResult<&'static str> {
        if self.dina == 0 || self.dina > 7 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(DINA[(self.dina - 1) as usize])
    }

    /// The *pasaran* name.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when `pasaran` is not in
    /// `1..=5`.
    pub const fn pasaran_name(self) -> CalendarResult<&'static str> {
        if self.pasaran == 0 || self.pasaran > 5 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(PASARAN[(self.pasaran - 1) as usize])
    }

    /// The *neptu*: the two weights added.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] for an out-of-range
    /// position.
    pub const fn neptu(self) -> CalendarResult<i64> {
        if self.dina == 0 || self.dina > 7 || self.pasaran == 0 || self.pasaran > 5 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(DINA_NEPTU[(self.dina - 1) as usize] + PASARAN_NEPTU[(self.pasaran - 1) as usize])
    }

    /// The position within the 35-day cycle, 0 to 34.
    ///
    /// The coefficients solve the two congruences at once: 21 is divisible
    /// by 7 and leaves 1 modulo 5, and 15 is divisible by 5 and leaves 1
    /// modulo 7.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] for an out-of-range
    /// position.
    pub const fn ordinal(self) -> CalendarResult<i64> {
        if self.dina == 0 || self.dina > 7 || self.pasaran == 0 || self.pasaran > 5 {
            return Err(CalendarError::DayOutOfRange);
        }
        let pasaran = (self.pasaran - 1) as i64;
        let dina = (self.dina - 1) as i64;
        Ok((21 * pasaran + 15 * dina).rem_euclid(WETONAN_CYCLE))
    }

    /// The weton `ordinal` days into a cycle.
    #[must_use]
    pub const fn from_ordinal(round: i64, ordinal: i64) -> Self {
        let ordinal = ordinal.rem_euclid(WETONAN_CYCLE);
        Self {
            round,
            dina: (ordinal.rem_euclid(7) + 1) as u8,
            pasaran: (ordinal.rem_euclid(5) + 1) as u8,
        }
    }
}

impl fmt::Display for WetonDate {
    /// Writes the weton the way it is spoken: weekday then pasaran, as in
    /// `Jemuwah Legi`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.dina_name(), self.pasaran_name()) {
            (Ok(dina), Ok(pasaran)) => write!(f, "{dina} {pasaran}"),
            _ => write!(f, "?{} ?{}", self.dina, self.pasaran),
        }
    }
}

/// The Javanese *pasaran* and *wetonan* cycle.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JavanesePasaranCalendar;

/// The five-day market week as a plain [`DayCycle`].
pub const PASARAN_DAY_CYCLE: DayCycle = DayCycle::new(PASARAN_CYCLE as u16, EPOCH);

/// The 35-day *wetonan* as a plain [`DayCycle`].
pub const WETONAN_DAY_CYCLE: DayCycle = DayCycle::new(WETONAN_CYCLE as u16, EPOCH);

/// The *pasaran* day of a fixed day, 1 for Legi through 5 for Kliwon.
#[must_use]
pub const fn pasaran_of(rd: Rd) -> u8 {
    (rd.0 - EPOCH.0).rem_euclid(PASARAN_CYCLE) as u8 + 1
}

impl Calendar for JavanesePasaranCalendar {
    type Date = WetonDate;

    /// The five-day *pasaran* and the seven-day week it runs against.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        const SHAPE: &[hc_calendar::shape::CycleShape] = &[
            hc_calendar::shape::CycleShape::named("pasaran", &PASARAN),
            hc_calendar::shape::CycleShape::named(hc_calendar::shape::WEEKDAY, &DINA),
        ];
        SHAPE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("javanese-pasaran"),
            english_name: "Javanese pasaran (wetonan cycle)",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        Ok(Rd(EPOCH.0 + date.round * WETONAN_CYCLE + date.ordinal()?))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let count = rd.0 - EPOCH.0;
        Ok(WetonDate::from_ordinal(
            count.div_euclid(WETONAN_CYCLE),
            count,
        ))
    }

    /// Describes the weton with the 1-based position in the 35-day cycle as
    /// the `day` field, and the two component weeks as extra fields.
    ///
    /// There is no month: the wetonan has no subdivision between the day
    /// and the whole cycle.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::Overflow`] if the extra-field set fills,
    /// which two fields cannot make happen.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("dina", date.dina.into())?;
        extra.set("pasaran", date.pasaran.into())?;
        extra.set("neptu", date.neptu()?)?;
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
        Ok(WetonDate {
            round: fields.year,
            dina: u8::try_from(fields.extra.require("dina")?)
                .map_err(|_| CalendarError::DayOutOfRange)?,
            pasaran: u8::try_from(fields.extra.require("pasaran")?)
                .map_err(|_| CalendarError::DayOutOfRange)?,
        })
    }
}

/// The weton of a fixed day, with the weekday as a [`Weekday`].
#[must_use]
pub fn weekday_and_pasaran(rd: Rd) -> (Weekday, u8) {
    (Weekday::from_rd(rd), pasaran_of(rd))
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;

    use super::*;
    use crate::balinese_pawukon;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    #[test]
    fn indonesian_independence_was_declared_on_jumat_legi() {
        // 17 August 1945, the best-attested weton in Indonesia.
        let day = greg(1945, 8, 17);
        let weton = JavanesePasaranCalendar.from_fixed(day).expect("any day");
        assert_eq!(weton.to_string(), "Jemuwah Legi");
        assert_eq!(weton.pasaran_name(), Ok("Legi"));
        assert_eq!(Weekday::from_rd(day), Weekday::Friday);
        // Its neptu is the number Javanese sources quote: 6 + 5 = 11.
        assert_eq!(weton.neptu(), Ok(11));
    }

    #[test]
    fn the_epoch_is_ahad_legi() {
        let weton = JavanesePasaranCalendar.from_fixed(EPOCH).expect("any day");
        assert_eq!(weton.to_string(), "Ahad Legi");
        assert_eq!(weton.round, 0);
        assert_eq!(weton.ordinal(), Ok(0));
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Sunday);
    }

    #[test]
    fn the_pasaran_is_the_balinese_pancawara_under_other_names() {
        // Two implementations of the same cycle, anchored independently in
        // the source, must agree on every day.
        for offset in (0..10_000).step_by(7) {
            let rd = Rd(greg(1800, 1, 1).0 + offset);
            let balinese = crate::BalinesePawukonCalendar
                .from_fixed(rd)
                .expect("any day");
            assert_eq!(pasaran_of(rd), balinese.pancawara(), "{rd}");
        }
        assert_eq!(PASARAN_NEPTU, balinese_pawukon::PANCAWARA_URIP);
        assert_eq!(DINA_NEPTU, balinese_pawukon::SAPTAWARA_URIP);
    }

    #[test]
    fn the_wetonan_is_thirty_five_days_and_visits_every_pair() {
        assert_eq!(WETONAN_CYCLE, PASARAN_CYCLE * 7);
        let mut seen = [false; 35];
        for offset in 0..35 {
            let weton = JavanesePasaranCalendar
                .from_fixed(Rd(greg(2026, 1, 1).0 + offset))
                .expect("any day");
            let ordinal = weton.ordinal().expect("valid weton") as usize;
            assert!(!seen[ordinal], "repeat at {offset}");
            seen[ordinal] = true;
        }
        assert!(seen.iter().all(|hit| *hit));
    }

    #[test]
    fn a_weton_recurs_every_thirty_five_days() {
        let birth = greg(1945, 8, 17);
        for multiple in 1..20 {
            let later = Rd(birth.0 + 35 * multiple);
            let weton = JavanesePasaranCalendar.from_fixed(later).expect("any day");
            assert_eq!(weton.to_string(), "Jemuwah Legi", "{multiple}");
            assert_eq!(
                weton.round,
                JavanesePasaranCalendar
                    .from_fixed(birth)
                    .expect("any day")
                    .round
                    + multiple
            );
        }
    }

    #[test]
    fn the_five_day_week_advances_by_one_a_day() {
        let start = greg(2026, 9, 21);
        for offset in 0..50 {
            let here = pasaran_of(Rd(start.0 + offset));
            let next = pasaran_of(Rd(start.0 + offset + 1));
            assert_eq!(here % 5 + 1, next, "{offset}");
        }
        assert_eq!(pasaran_of(start), pasaran_of(Rd(start.0 + 5)));
    }

    #[test]
    fn neptu_adds_the_two_weights() {
        for dina in 1..=7u8 {
            for pasaran in 1..=5u8 {
                let weton = WetonDate::new(0, dina, pasaran);
                assert_eq!(
                    weton.neptu(),
                    Ok(DINA_NEPTU[(dina - 1) as usize] + PASARAN_NEPTU[(pasaran - 1) as usize])
                );
            }
        }
        // The extremes: Ahad Wage is the lightest, Setu Pahing the heaviest.
        assert_eq!(WetonDate::new(0, 1, 4).neptu(), Ok(9));
        assert_eq!(WetonDate::new(0, 7, 2).neptu(), Ok(18));
    }

    #[test]
    fn every_weton_of_a_whole_cycle_round_trips() {
        for round in [-3i64, 0, 5_000] {
            for ordinal in 0..35 {
                let weton = WetonDate::from_ordinal(round, ordinal);
                let rd = JavanesePasaranCalendar
                    .to_fixed(weton)
                    .expect("a valid weton");
                assert_eq!(JavanesePasaranCalendar.from_fixed(rd), Ok(weton));
                let fields = JavanesePasaranCalendar
                    .to_fields(weton)
                    .expect("describable");
                assert_eq!(JavanesePasaranCalendar.from_fields(&fields), Ok(weton));
            }
        }
    }

    #[test]
    fn fields_carry_both_weeks_and_the_neptu() {
        let weton = JavanesePasaranCalendar
            .from_fixed(greg(1945, 8, 17))
            .expect("any day");
        let fields = JavanesePasaranCalendar
            .to_fields(weton)
            .expect("describable");
        assert_eq!(fields.month, None);
        assert_eq!(fields.extra.get("dina"), Some(6));
        assert_eq!(fields.extra.get("pasaran"), Some(1));
        assert_eq!(fields.extra.get("neptu"), Some(11));
        assert_eq!(fields.day, Some(weton.ordinal().expect("valid") as u8 + 1));
        assert_eq!(JavanesePasaranCalendar.from_fields(&fields), Ok(weton));
        assert_eq!(
            JavanesePasaranCalendar.from_fields(&DateFields::new(0)),
            Err(CalendarError::MissingField("dina"))
        );
    }

    #[test]
    fn impossible_wetons_are_refused() {
        assert_eq!(
            WetonDate::new(0, 8, 1).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            WetonDate::new(0, 1, 6).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            WetonDate::new(0, 0, 1).dina_name(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            WetonDate::new(0, 1, 0).pasaran_name(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            WetonDate::new(0, 8, 8).neptu(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            JavanesePasaranCalendar.to_fixed(WetonDate::new(0, 9, 1)),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn the_name_tables_are_complete() {
        assert_eq!(PASARAN.len(), 5);
        assert_eq!(PASARAN_NEPTU.len(), 5);
        assert_eq!(DINA.len(), 7);
        assert_eq!(DINA_NEPTU.len(), 7);
        assert_eq!(PASARAN[0], "Legi");
        assert_eq!(DINA[0], "Ahad");
    }

    #[test]
    fn the_plain_day_cycles_agree_with_the_calendar() {
        for offset in (0..3_000).step_by(13) {
            let rd = Rd(greg(1900, 1, 1).0 + offset);
            let weton = JavanesePasaranCalendar.from_fixed(rd).expect("any day");
            assert_eq!(
                PASARAN_DAY_CYCLE.position(rd),
                Some(u16::from(weton.pasaran - 1))
            );
            assert_eq!(
                WETONAN_DAY_CYCLE.position(rd),
                Some(weton.ordinal().expect("valid") as u16)
            );
            let (weekday, pasaran) = weekday_and_pasaran(rd);
            assert_eq!(weekday.sunday_first_number() + 1, weton.dina);
            assert_eq!(pasaran, weton.pasaran);
        }
    }
}
