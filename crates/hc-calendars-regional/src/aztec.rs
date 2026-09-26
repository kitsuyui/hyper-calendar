//! The two Aztec calendars: the 260-day tonalpōhualli and the 365-day
//! xiuhpōhualli.
//!
//! They are structurally the Mesoamerican pair that [`crate::maya`] also
//! implements — thirteen numbers against twenty day-signs, and a vague
//! year of eighteen twenty-day months plus five unlucky days — under
//! Nahuatl names and an anchor of their own: [`CORRELATION`], the fall of
//! Tenochtitlan on 13 August 1521 (Julian) as *1 Coatl*, 2 Xocotlhuetzi,
//! which is Caso's correlation as Reingold and Dershowitz tabulate it.
//! There is no Aztec long count, so both calendars are cycles and both
//! carry a round number, for the reasons the crate documentation gives.
//! The year is the uncorrected 365 days and drifts a day against the
//! seasons every four years. The anchor, the competing reconstructions,
//! the year bearers and the sources are in
//! [`docs/systems/mesoamerican-counts.md`](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/systems/mesoamerican-counts.md).

use core::fmt;

use hc_calendar::fields::ExtraFields;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};

/// The fixed day of 13 August 1521 in the Julian calendar, when
/// Tenochtitlan fell.
///
/// On that day the tonalpōhualli read *1 Coatl* and the xiuhpōhualli was at
/// day 2 of its eleventh month, Xocotlhuetzi.
pub const CORRELATION: Rd = Rd(555_403);

/// Days in a tonalpōhualli cycle.
pub const TONALPOHUALLI_CYCLE: i64 = 260;

/// Days in a xiuhpōhualli year.
pub const XIUHPOHUALLI_CYCLE: i64 = crate::vague_year::YEAR_DAYS;

/// The twenty day-signs of the tonalpōhualli, in cycle order.
pub const DAY_SIGNS: [&str; 20] = [
    "Cipactli",
    "Ehecatl",
    "Calli",
    "Cuetzpalin",
    "Coatl",
    "Miquiztli",
    "Mazatl",
    "Tochtli",
    "Atl",
    "Itzcuintli",
    "Ozomatli",
    "Malinalli",
    "Acatl",
    "Ocelotl",
    "Cuauhtli",
    "Cozcacuauhtli",
    "Ollin",
    "Tecpatl",
    "Quiahuitl",
    "Xochitl",
];

/// The nineteen xiuhpōhualli months: eighteen of twenty days and the
/// five-day Nemontemi.
pub const MONTHS: [&str; 19] = [
    "Izcalli",
    "Atlcahualo",
    "Tlacaxipehualiztli",
    "Tozoztontli",
    "Hueytozoztli",
    "Toxcatl",
    "Etzalcualiztli",
    "Tecuilhuitontli",
    "Hueytecuilhuitl",
    "Tlaxochimaco",
    "Xocotlhuetzi",
    "Ochpaniztli",
    "Teotleco",
    "Tepeilhuitl",
    "Quecholli",
    "Panquetzaliztli",
    "Atemoztli",
    "Tititl",
    "Nemontemi",
];

/// A position in the tonalpōhualli, without saying which cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TonalpohualliPosition {
    /// The number, 1 to 13.
    pub number: u8,
    /// The day-sign, 1 to 20, indexing [`DAY_SIGNS`].
    pub sign: u8,
}

impl TonalpohualliPosition {
    /// A position, without validation.
    #[must_use]
    pub const fn new(number: u8, sign: u8) -> Self {
        Self { number, sign }
    }

    /// The day-sign as a string.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when the sign is not in
    /// `1..=20`.
    pub const fn sign_str(self) -> CalendarResult<&'static str> {
        if self.sign == 0 || self.sign > 20 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(DAY_SIGNS[(self.sign - 1) as usize])
    }

    /// The position's ordinal within the cycle, 0 to 259.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] for an impossible number or
    /// sign.
    pub const fn ordinal(self) -> CalendarResult<i64> {
        if self.number == 0 || self.number > 13 || self.sign == 0 || self.sign > 20 {
            return Err(CalendarError::DayOutOfRange);
        }
        let number = self.number as i64;
        let sign = self.sign as i64;
        Ok((number - 1 + 39 * (number - sign)).rem_euclid(TONALPOHUALLI_CYCLE))
    }

    /// The position `ordinal` days into the cycle.
    #[must_use]
    pub const fn from_ordinal(ordinal: i64) -> Self {
        let ordinal = ordinal.rem_euclid(TONALPOHUALLI_CYCLE);
        Self {
            number: (ordinal.rem_euclid(13) + 1) as u8,
            sign: (ordinal.rem_euclid(20) + 1) as u8,
        }
    }
}

impl fmt::Display for TonalpohualliPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.sign_str() {
            Ok(sign) => write!(f, "{} {sign}", self.number),
            Err(_) => write!(f, "{} ?{}", self.number, self.sign),
        }
    }
}

/// A position in the xiuhpōhualli, without saying which year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XiuhpohualliPosition {
    /// The month, 1 to 19, indexing [`MONTHS`].
    pub month: u8,
    /// The day within the month, counting from 1. Months 1 to 18 run 1 to
    /// 20; Nemontemi runs 1 to 5.
    pub day: u8,
}

impl XiuhpohualliPosition {
    /// A position, without validation.
    #[must_use]
    pub const fn new(month: u8, day: u8) -> Self {
        Self { month, day }
    }

    /// The month name as a string.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] when the month is not in
    /// `1..=19`.
    pub const fn month_str(self) -> CalendarResult<&'static str> {
        if self.month == 0 || self.month > 19 {
            return Err(CalendarError::MonthOutOfRange);
        }
        Ok(MONTHS[(self.month - 1) as usize])
    }

    /// The position's ordinal within the year, 0 to 364.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] or
    /// [`CalendarError::DayOutOfRange`]; Nemontemi has only five days.
    pub const fn ordinal(self) -> CalendarResult<i64> {
        crate::vague_year::ordinal(self.month, self.day)
    }

    /// The position `ordinal` days into the year.
    #[must_use]
    pub const fn from_ordinal(ordinal: i64) -> Self {
        let (month, day) = crate::vague_year::position(ordinal);
        Self { month, day }
    }
}

impl fmt::Display for XiuhpohualliPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.month_str() {
            Ok(month) => write!(f, "{} {month}", self.day),
            Err(_) => write!(f, "{} ?{}", self.day, self.month),
        }
    }
}

/// The fixed day whose tonalpōhualli ordinal is 0.
pub(crate) const TONALPOHUALLI_EPOCH: i64 = CORRELATION.0 - 104;

/// The fixed day whose xiuhpōhualli ordinal is 0, the first of Izcalli.
pub(crate) const XIUHPOHUALLI_EPOCH: i64 = CORRELATION.0 - 201;

/// A tonalpōhualli date: a position plus the cycle it falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AztecTonalpohualliDate {
    /// Complete 260-day cycles elapsed since the cycle containing
    /// [`CORRELATION`] began.
    pub round: i64,
    /// The position within the cycle.
    pub position: TonalpohualliPosition,
}

/// A xiuhpōhualli date: a position plus the year it falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AztecXiuhpohualliDate {
    /// Complete 365-day years elapsed since the year containing
    /// [`CORRELATION`] began.
    pub round: i64,
    /// The position within the year.
    pub position: XiuhpohualliPosition,
}

/// The 260-day tonalpōhualli.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AztecTonalpohualliCalendar;

/// The 365-day xiuhpōhualli.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AztecXiuhpohualliCalendar;

impl Calendar for AztecTonalpohualliCalendar {
    type Date = AztecTonalpohualliDate;

    /// Unrecorded: the sources read give the correlation anchor of 1521 and
    /// the painted books, not a span of use, and the count went on in
    /// colonial books after the anchor.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// The thirteen numbers and the twenty day-signs.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        const SHAPE: &[hc_calendar::shape::CycleShape] = &[
            hc_calendar::shape::CycleShape::fixed("trecena", 13),
            hc_calendar::shape::CycleShape::named("day-sign", &DAY_SIGNS),
        ];
        SHAPE
    }

    /// A cycle has no year: the `year` field carries the round.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("aztec-tonalpohualli"),
            english_name: "Aztec tonalpōhualli",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
            native_locales: &["nah"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        Ok(Rd(TONALPOHUALLI_EPOCH
            + date.round * TONALPOHUALLI_CYCLE
            + date.position.ordinal()?))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let count = rd.0 - TONALPOHUALLI_EPOCH;
        Ok(AztecTonalpohualliDate {
            round: count.div_euclid(TONALPOHUALLI_CYCLE),
            position: TonalpohualliPosition::from_ordinal(count),
        })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("tonalpohualli_number", date.position.number.into())?;
        extra.set("tonalpohualli_sign", date.position.sign.into())?;
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
        let number = fields.extra.require("tonalpohualli_number")?;
        let sign = fields.extra.require("tonalpohualli_sign")?;
        Ok(AztecTonalpohualliDate {
            round: fields.year,
            position: TonalpohualliPosition::new(byte(number)?, byte(sign)?),
        })
    }
}

/// Narrow an extra field to the byte-sized place it names.
fn byte(value: i64) -> CalendarResult<u8> {
    u8::try_from(value).map_err(|_| CalendarError::DayOutOfRange)
}

impl Calendar for AztecXiuhpohualliCalendar {
    type Date = AztecXiuhpohualliDate;

    /// Unrecorded, for the reason [`AztecTonalpohualliCalendar::usage`]
    /// gives.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Eighteen months of twenty days and the five-day Nemontemi, which has
    /// a name and so is a nineteenth position.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        const SHAPE: &[hc_calendar::shape::CycleShape] = &[hc_calendar::shape::CycleShape::named(
            hc_calendar::shape::MONTH,
            &MONTHS,
        )];
        SHAPE
    }

    /// A cycle has no year: the `year` field carries the round.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("aztec-xiuhpohualli"),
            english_name: "Aztec xiuhpōhualli",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
            native_locales: &["nah"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        Ok(Rd(XIUHPOHUALLI_EPOCH
            + date.round * XIUHPOHUALLI_CYCLE
            + date.position.ordinal()?))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let count = rd.0 - XIUHPOHUALLI_EPOCH;
        Ok(AztecXiuhpohualliDate {
            round: count.div_euclid(XIUHPOHUALLI_CYCLE),
            position: XiuhpohualliPosition::from_ordinal(count),
        })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields {
            era: None,
            year: date.round,
            month: Some(Month::regular(date.position.month)),
            day: Some(date.position.day),
            leap_day: false,
            extra: ExtraFields::new(),
        })
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        Ok(AztecXiuhpohualliDate {
            round: fields.year,
            position: XiuhpohualliPosition::new(month.ordinal, fields.require_day()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::{gregorian, julian};

    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    #[test]
    fn the_correlation_is_the_fall_of_tenochtitlan() {
        assert_eq!(CORRELATION, julian::to_fixed(1521, 8, 13).expect("valid"));
        // The same day in the Gregorian calendar, ten days later.
        assert_eq!(CORRELATION, greg(1521, 8, 23));
    }

    #[test]
    fn tenochtitlan_fell_on_one_coatl_two_xocotlhuetzi() {
        let tonal = AztecTonalpohualliCalendar
            .from_fixed(CORRELATION)
            .expect("any day");
        assert_eq!(tonal.position, TonalpohualliPosition::new(1, 5));
        assert_eq!(tonal.position.sign_str(), Ok("Coatl"));
        assert_eq!(tonal.position.to_string(), "1 Coatl");

        let xiuh = AztecXiuhpohualliCalendar
            .from_fixed(CORRELATION)
            .expect("any day");
        assert_eq!(xiuh.position, XiuhpohualliPosition::new(11, 2));
        assert_eq!(xiuh.position.month_str(), Ok("Xocotlhuetzi"));
        assert_eq!(xiuh.position.to_string(), "2 Xocotlhuetzi");
    }

    #[test]
    fn a_published_modern_date_matches() {
        // Checked against azteccalendar.com, which uses the same
        // correlation: 2026-09-21 is 8 Ehecatl, 14 Tititl.
        let day = greg(2026, 9, 21);
        assert_eq!(
            AztecTonalpohualliCalendar
                .from_fixed(day)
                .expect("any day")
                .position
                .to_string(),
            "8 Ehecatl"
        );
        assert_eq!(
            AztecXiuhpohualliCalendar
                .from_fixed(day)
                .expect("any day")
                .position
                .to_string(),
            "14 Tititl"
        );
    }

    #[test]
    fn the_tonalpohualli_visits_every_position_once_per_cycle() {
        let start = greg(1500, 1, 1);
        let mut seen = [false; 260];
        for offset in 0..260 {
            let ordinal = AztecTonalpohualliCalendar
                .from_fixed(Rd(start.0 + offset))
                .expect("any day")
                .position
                .ordinal()
                .expect("valid position");
            assert!(!seen[ordinal as usize], "repeat at {offset}");
            seen[ordinal as usize] = true;
        }
        assert!(seen.iter().all(|hit| *hit));
        assert_eq!(
            AztecTonalpohualliCalendar
                .from_fixed(Rd(start.0 + 260))
                .expect("any day")
                .position,
            AztecTonalpohualliCalendar
                .from_fixed(start)
                .expect("any day")
                .position
        );
    }

    #[test]
    fn the_xiuhpohualli_has_eighteen_months_of_twenty_and_five_nameless_days() {
        assert_eq!(MONTHS.len(), 19);
        assert_eq!(MONTHS[18], "Nemontemi");
        assert_eq!(XiuhpohualliPosition::new(1, 1).ordinal(), Ok(0));
        assert_eq!(XiuhpohualliPosition::new(19, 5).ordinal(), Ok(364));
        assert_eq!(
            XiuhpohualliPosition::new(19, 6).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            XiuhpohualliPosition::new(1, 0).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            XiuhpohualliPosition::new(20, 1).ordinal(),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            XiuhpohualliPosition::new(0, 1).month_str(),
            Err(CalendarError::MonthOutOfRange)
        );
        for ordinal in 0..365 {
            assert_eq!(
                XiuhpohualliPosition::from_ordinal(ordinal).ordinal(),
                Ok(ordinal)
            );
        }
    }

    #[test]
    fn the_two_cycles_realign_every_fifty_two_years() {
        // The Aztec "binding of the years" fell when the pair repeated.
        let start = CORRELATION;
        let repeat = Rd(start.0 + 18_980);
        assert_eq!(
            AztecTonalpohualliCalendar
                .from_fixed(repeat)
                .expect("any day")
                .position,
            AztecTonalpohualliCalendar
                .from_fixed(start)
                .expect("any day")
                .position
        );
        assert_eq!(
            AztecXiuhpohualliCalendar
                .from_fixed(repeat)
                .expect("any day")
                .position,
            AztecXiuhpohualliCalendar
                .from_fixed(start)
                .expect("any day")
                .position
        );
    }

    #[test]
    fn both_cycles_round_trip_over_a_full_calendar_round() {
        for offset in 0..18_980 {
            let rd = Rd(CORRELATION.0 + offset);
            let tonal = AztecTonalpohualliCalendar.from_fixed(rd).expect("any day");
            assert_eq!(AztecTonalpohualliCalendar.to_fixed(tonal), Ok(rd));
            let xiuh = AztecXiuhpohualliCalendar.from_fixed(rd).expect("any day");
            assert_eq!(AztecXiuhpohualliCalendar.to_fixed(xiuh), Ok(rd));
        }
    }

    #[test]
    fn the_vague_year_drifts_one_day_every_four_years() {
        // No intercalation: 400 xiuhpōhualli years are 400 * 365 days, a
        // whole 97 days short of 400 Gregorian ones.
        let start = greg(1600, 1, 1);
        let later = Rd(start.0 + 400 * 365);
        let (year, month, day) = gregorian::from_fixed(later).expect("in range");
        assert_eq!((year, month, day), (1999, 9, 26));
        assert_eq!(
            AztecXiuhpohualliCalendar
                .from_fixed(later)
                .expect("any day")
                .position,
            AztecXiuhpohualliCalendar
                .from_fixed(start)
                .expect("any day")
                .position
        );
    }

    #[test]
    fn fields_round_trip_through_the_generic_interface() {
        let day = greg(2026, 9, 21);
        let tonal = AztecTonalpohualliCalendar.from_fixed(day).expect("any day");
        let fields = AztecTonalpohualliCalendar
            .to_fields(tonal)
            .expect("describable");
        assert_eq!(fields.extra.get("tonalpohualli_number"), Some(8));
        assert_eq!(fields.extra.get("tonalpohualli_sign"), Some(2));
        assert_eq!(AztecTonalpohualliCalendar.from_fields(&fields), Ok(tonal));
        assert_eq!(
            AztecTonalpohualliCalendar.from_fields(&DateFields::new(0)),
            Err(CalendarError::MissingField("tonalpohualli_number"))
        );

        let xiuh = AztecXiuhpohualliCalendar.from_fixed(day).expect("any day");
        let fields = AztecXiuhpohualliCalendar
            .to_fields(xiuh)
            .expect("describable");
        assert_eq!(fields.month, Some(Month::regular(18)));
        assert_eq!(fields.day, Some(14));
        assert_eq!(AztecXiuhpohualliCalendar.from_fields(&fields), Ok(xiuh));
        let mut leap = fields;
        leap.month = Some(Month::leap(18));
        assert_eq!(
            AztecXiuhpohualliCalendar.from_fields(&leap),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn impossible_positions_are_refused() {
        assert_eq!(
            TonalpohualliPosition::new(14, 1).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            TonalpohualliPosition::new(1, 21).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            TonalpohualliPosition::new(1, 0).sign_str(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(TonalpohualliPosition::new(1, 1).sign_str(), Ok("Cipactli"));
    }

    #[test]
    fn the_cycles_run_backwards_before_the_correlation() {
        let long_ago = greg(1000, 1, 1);
        let tonal = AztecTonalpohualliCalendar
            .from_fixed(long_ago)
            .expect("any day");
        assert!(tonal.round < 0);
        assert_eq!(AztecTonalpohualliCalendar.to_fixed(tonal), Ok(long_ago));
        let xiuh = AztecXiuhpohualliCalendar
            .from_fixed(long_ago)
            .expect("any day");
        assert!(xiuh.round < 0);
        assert_eq!(AztecXiuhpohualliCalendar.to_fixed(xiuh), Ok(long_ago));
    }

    #[test]
    fn the_metadata_says_the_cycles_are_unbounded_and_arithmetic() {
        for meta in [
            AztecTonalpohualliCalendar.meta(),
            AztecXiuhpohualliCalendar.meta(),
        ] {
            assert!(meta.earliest.is_none());
            assert!(meta.latest.is_none());
            assert!(!meta.is_astronomical);
            assert!(!meta.has_leap_months);
        }
        assert_eq!(
            AztecTonalpohualliCalendar.meta().id,
            CalendarId("aztec-tonalpohualli")
        );
        assert_eq!(
            AztecXiuhpohualliCalendar.meta().id,
            CalendarId("aztec-xiuhpohualli")
        );
    }
}
