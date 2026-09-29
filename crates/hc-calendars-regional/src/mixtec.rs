//! The Mixtec year, as Alfonso Caso reconstructs it after Jiménez Moreno —
//! `mixtec-year`.
//!
//! The 365-day year of the Aztec [`xiuhpōhualli`](crate::aztec), begun two
//! months earlier: "los mixtecos y popolocas principiaban su año por
//! *Atemoztli*" (Caso, *Los calendarios prehispánicos*, 1967, p. 78,
//! `caso1967`, citing Jiménez Moreno and Mateos Higuera 1940,
//! `jimenezmoreno1940`, not read). The months are the Aztec ones, day for
//! day from Izcalli to Panquetzaliztli, and the five nemontemi come at the
//! end of the Mixtec year, after Panquetzaliztli, so the Mixtec Atemoztli
//! and Tititl run five days behind the Aztec ones. Caso does not place the
//! Mixtec nemontemi in one sentence: that they follow Panquetzaliztli is
//! this library's reading of two of his statements on p. 78, that the
//! months of all peoples agreed but for the nemontemi, "que iban al fin del
//! año", and that Panquetzaliztli was the "último de su año que principiaba
//! por *Atemoztli*". A year is named by the
//! day of the 260-day count on its last day of its last month, the 20th of
//! Panquetzaliztli, the 360th day: "el año de 1568 se llamó en el
//! Calendario popoloca-mixteco *10 Tecpatl* … por que el día *10 Tecpatl*
//! fue el último del mes *Panquetzaliztli*, último de su año que principiaba
//! por *Atemoztli*" (p. 78). The same day is the 20th of the Aztec
//! Panquetzaliztli, forty days before the Aztec bearer, so a Mixtec year is
//! named by the same sign as the Aztec year that overlaps its end, with a
//! number one lower.
//!
//! The Mixtec month names are not known — "Ignoramos completamente los
//! nombres de estos meses" (Caso, "El calendario mixteco", *Historia
//! Mexicana* 5, 1956, p. 493, `caso1956`) — so the months carry the Nahuatl
//! names of the Aztec months they coincide with, as Caso writes them. The
//! 260-day count is Caso's Mexica one, which [`crate::aztec`] carries. The
//! reconstruction, the worked example and the sources are in
//! [`docs/systems/mesoamerican-years.md`](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/systems/mesoamerican-years.md).

use core::fmt;

use hc_calendar::fields::ExtraFields;
use hc_calendar::shape::{CycleShape, MONTH};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};

use crate::aztec::{TONALPOHUALLI_CYCLE, TONALPOHUALLI_EPOCH, TonalpohualliPosition};
use crate::vague_year::{self, YEAR_DAYS};

/// The first day of the year Caso names *10 Tecpatl*, 1 Atemoztli,
/// 4 December 1567 (Julian): 365 days before the day after its nemontemi,
/// which follow 20 Panquetzaliztli, 27 November 1568. Round 0 is the year
/// it begins.
pub const NEW_YEAR_1567: Rd = match hc_calendars_solar::julian::to_fixed(1567, 12, 4) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The nineteen positions of the year: the eighteen months from Atemoztli
/// to Panquetzaliztli, in the spelling of [`crate::aztec::MONTHS`], and the
/// nemontemi after them.
pub const MONTHS: [&str; 19] = [
    "Atemoztli",
    "Tititl",
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
    "Nemontemi",
];

/// The four year bearers, Reed, Flint, House and Rabbit, in the order the
/// years take them (Caso 1956, pp. 494–495), by the Nahuatl names of
/// [`crate::aztec::DAY_SIGNS`].
pub const YEAR_BEARERS: [&str; 4] = ["Acatl", "Tecpatl", "Calli", "Tochtli"];

/// The four bearers' places in the order of the day-signs: the 13th, 18th,
/// 3rd and 8th.
pub const YEAR_BEARER_SIGNS: [u8; 4] = [13, 18, 3, 8];

/// The ordinal of the naming day in the year: the 20th of Panquetzaliztli,
/// the 360th day.
const BEARER_DAY: i64 = 359;

/// Days in a round of the year bearers: 52 years, 73 cycles of 260.
const BEARER_ROUND_YEARS: i64 = 52;

/// A position in the year, without saying which year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MixtecPosition {
    /// The month, 1 to 19, indexing [`MONTHS`]: 1 is Atemoztli, 18
    /// Panquetzaliztli, 19 the nemontemi.
    pub month: u8,
    /// The day within the month, counting from 1. Months 1 to 18 run 1 to
    /// 20; the nemontemi run 1 to 5.
    pub day: u8,
}

impl MixtecPosition {
    /// A position, without validation.
    #[must_use]
    pub const fn new(month: u8, day: u8) -> Self {
        Self { month, day }
    }

    /// The month name.
    ///
    /// # Errors
    ///
    /// [`CalendarError::MonthOutOfRange`] outside months 1 to 19.
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
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`];
    /// the nemontemi have five days.
    pub const fn ordinal(self) -> CalendarResult<i64> {
        vague_year::ordinal(self.month, self.day)
    }

    /// The position `ordinal` days into the year.
    #[must_use]
    pub const fn from_ordinal(ordinal: i64) -> Self {
        let (month, day) = vague_year::position(ordinal);
        Self { month, day }
    }
}

impl fmt::Display for MixtecPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.month_str() {
            Ok(month) => write!(f, "{} {month}", self.day),
            Err(_) => write!(f, "{} ?{}", self.day, self.month),
        }
    }
}

/// A day of the Mixtec year: a position plus the year it falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MixtecYearDate {
    /// Complete years since the one that began on [`NEW_YEAR_1567`].
    pub round: i64,
    /// The position within the year.
    pub position: MixtecPosition,
}

impl MixtecYearDate {
    /// The year's bearer: the day of the 260-day count on the 20th of its
    /// Panquetzaliztli, with its day-sign in the order of
    /// [`crate::aztec::DAY_SIGNS`].
    #[must_use]
    pub const fn year_bearer(self) -> TonalpohualliPosition {
        // A bearer round of 52 years is a whole number of 260-day cycles,
        // so reducing the round first keeps the product small.
        let day = NEW_YEAR_1567.0 - TONALPOHUALLI_EPOCH
            + BEARER_DAY
            + self.round.rem_euclid(BEARER_ROUND_YEARS) * YEAR_DAYS;
        TonalpohualliPosition::from_ordinal(day.rem_euclid(TONALPOHUALLI_CYCLE))
    }

    /// Which of the four [`YEAR_BEARERS`] names the year, 1 to 4. The year
    /// that ends in 1568 is a Flint year, the second.
    #[must_use]
    pub const fn year_bearer_index(self) -> u8 {
        ((self.round + 1).rem_euclid(4) + 1) as u8
    }

    /// The year's name, its bearer's number and sign: `10 Tecpatl` for the
    /// year that ends in 1568.
    #[must_use]
    pub const fn year_name(self) -> (u8, &'static str) {
        (
            self.year_bearer().number,
            YEAR_BEARERS[(self.year_bearer_index() - 1) as usize],
        )
    }
}

/// The Mixtec year under Caso's reconstruction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MixtecYearCalendar;

/// The nineteen positions of the year and the four year bearers.
const SHAPE: &[CycleShape] = &[
    CycleShape::named(MONTH, &MONTHS),
    CycleShape::named("year-bearer", &YEAR_BEARERS),
];

/// Narrow a field to the byte-sized value it names.
fn byte(value: i64) -> CalendarResult<u8> {
    u8::try_from(value).map_err(|_| CalendarError::DayOutOfRange)
}

impl Calendar for MixtecYearCalendar {
    type Date = MixtecYearDate;

    /// Unrecorded: the sources read place the year names in the codices and
    /// in the sixteenth century, and neither end of the year's use on a day.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// The nineteen positions and the four year bearers.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// The year has no leap day ever, and the `year` field is a round
    /// counted from an anchor of this library's choosing, not a year of an
    /// era: there is no year to ask about.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("mixtec-year"),
            english_name: "Mixtec year",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
            native_locales: &["mix"],
        }
    }

    /// # Errors
    ///
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`]
    /// for an impossible position; [`CalendarError::Overflow`] for a round
    /// too far out to count.
    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        let within = date.position.ordinal()?;
        date.round
            .checked_mul(YEAR_DAYS)
            .and_then(|days| days.checked_add(NEW_YEAR_1567.0 + within))
            .map(Rd)
            .ok_or(CalendarError::Overflow)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let count =
            rd.0.checked_sub(NEW_YEAR_1567.0)
                .ok_or(CalendarError::Overflow)?;
        Ok(MixtecYearDate {
            round: count.div_euclid(YEAR_DAYS),
            position: MixtecPosition::from_ordinal(count),
        })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("year_bearer_number", date.year_bearer().number.into())?;
        extra.set("year_bearer", date.year_bearer_index().into())?;
        Ok(DateFields {
            era: None,
            year: date.round,
            month: Some(Month::regular(date.position.month)),
            day: Some(date.position.day),
            leap_day: false,
            extra,
        })
    }

    /// The year bearer is carried for reading, and follows from the round;
    /// fields that name a different one are refused.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = MixtecYearDate {
            round: fields.year,
            position: MixtecPosition::new(month.ordinal, fields.require_day()?),
        };
        date.position.ordinal()?;
        if let Some(number) = fields.extra.get("year_bearer_number")
            && byte(number)? != date.year_bearer().number
        {
            return Err(CalendarError::DayOutOfRange);
        }
        if let Some(bearer) = fields.extra.get("year_bearer")
            && byte(bearer)? != date.year_bearer_index()
        {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::julian;

    use super::*;
    use crate::aztec::{
        AztecTonalpohualliCalendar, AztecXiuhpohualliCalendar, XiuhpohualliPosition,
    };

    fn jul(year: i64, month: u8, day: u8) -> Rd {
        julian::to_fixed(year, month, day).expect("valid Julian date")
    }

    fn mixtec(rd: Rd) -> MixtecYearDate {
        MixtecYearCalendar.from_fixed(rd).expect("any day")
    }

    fn tonalpohualli(rd: Rd) -> TonalpohualliPosition {
        AztecTonalpohualliCalendar
            .from_fixed(rd)
            .expect("any day")
            .position
    }

    fn first_of(round: i64) -> MixtecYearDate {
        MixtecYearDate {
            round,
            position: MixtecPosition::new(1, 1),
        }
    }

    /// Caso 1967, p. 78: 1568 was *10 Tecpatl* to the Mixtecs because
    /// 10 Tecpatl was the last day of Panquetzaliztli, the last month of a
    /// year begun at Atemoztli; the same day was the last of the Tenochca
    /// Panquetzaliztli, and the Tenochca year, which ended in Tititl, was
    /// *11 Tecpatl*.
    #[test]
    fn the_year_of_1568_is_ten_tecpatl_and_the_tenochca_year_eleven() {
        let naming_day = jul(1568, 11, 27);
        // docs/systems/mesoamerican-years.md, worked by hand.
        assert_eq!(naming_day, Rd(572_676));
        assert_eq!(
            tonalpohualli(naming_day),
            TonalpohualliPosition::new(10, 18)
        );
        let date = mixtec(naming_day);
        assert_eq!(date.round, 0);
        assert_eq!(date.position, MixtecPosition::new(18, 20));
        assert_eq!(date.position.to_string(), "20 Panquetzaliztli");
        assert_eq!(date.year_bearer(), TonalpohualliPosition::new(10, 18));
        assert_eq!(date.year_name(), (10, "Tecpatl"));
        // The same day in the xiuhpōhualli: 20 Panquetzaliztli, month 16 as
        // Reingold and Dershowitz number it from Izcalli.
        let aztec = AztecXiuhpohualliCalendar
            .from_fixed(naming_day)
            .expect("any day");
        assert_eq!(aztec.position, XiuhpohualliPosition::new(16, 20));
        // The Tenochca year names itself on 20 Tititl, forty days on:
        // 11 Tecpatl.
        let tititl = Rd(naming_day.0 + 40);
        assert_eq!(
            AztecXiuhpohualliCalendar
                .from_fixed(tititl)
                .expect("any day")
                .position,
            XiuhpohualliPosition::new(18, 20)
        );
        assert_eq!(tonalpohualli(tititl), TonalpohualliPosition::new(11, 18));
        // The year began on 1 Atemoztli, 4 December 1567, and the
        // nemontemi after the naming day end it on 2 December 1568.
        assert_eq!(NEW_YEAR_1567, jul(1567, 12, 4));
        assert_eq!(
            mixtec(jul(1568, 12, 2)).position,
            MixtecPosition::new(19, 5)
        );
        assert_eq!(mixtec(jul(1568, 12, 3)), first_of(1));
    }

    /// Caso 1956, p. 494, after Jiménez Moreno: with the year begun at
    /// Atemoztli and named by the last day of its last month, the day
    /// *11 Miquiztli* of the Cuilapan stone's year *10 Tecpatl* falls in
    /// Panquetzaliztli, and the days *11 Coatl* and *6 Acatl*, nine days
    /// apart counting both, of its year *10 Acatl* in Tecuilhuitontli.
    #[test]
    fn the_cuilapan_days_fall_in_the_months_of_their_glyphs() {
        let round_named = |number: u8, sign: u8| {
            (0..52)
                .find(|round| {
                    first_of(*round).year_bearer() == TonalpohualliPosition::new(number, sign)
                })
                .expect("every name comes round in 52 years")
        };
        let day_named = |round: i64, number: u8, sign: u8| {
            let start = MixtecYearCalendar
                .to_fixed(first_of(round))
                .expect("in range");
            (0..YEAR_DAYS)
                .map(|offset| Rd(start.0 + offset))
                .filter(|rd| tonalpohualli(*rd) == TonalpohualliPosition::new(number, sign))
                .map(|rd| mixtec(rd).position)
                .collect::<Vec<_>>()
        };
        let flint = round_named(10, 18);
        // A year of 365 days holds 105 days of the 260-day count twice, and
        // 11 Miquiztli is one of them: its second fall is in
        // Panquetzaliztli, under the glyph Caso reads as that month's
        // banner. The two Reed-year days fall once each.
        assert_eq!(
            day_named(flint, 11, 6),
            [MixtecPosition::new(5, 8), MixtecPosition::new(18, 8)],
            "11 Miquiztli in 10 Tecpatl"
        );
        let reed = round_named(10, 13);
        let serpent = day_named(reed, 11, 5);
        let reed_day = day_named(reed, 6, 13);
        assert_eq!(serpent, [MixtecPosition::new(10, 12)]);
        assert_eq!(reed_day, [MixtecPosition::new(10, 20)]);
        // Nine days, counting both.
        assert_eq!(
            reed_day[0].ordinal().expect("valid") - serpent[0].ordinal().expect("valid") + 1,
            9
        );
        assert_eq!(MONTHS[9], "Tecuilhuitontli");
    }

    /// Milbrath (2022), after Jiménez Moreno and Caso: the Mixtec year ends
    /// forty days before the Aztec one and carries the same sign one
    /// number lower, 1519 being Aztec *1 Acatl* and Mixtec *13 Acatl*.
    #[test]
    fn a_mixtec_year_is_the_aztec_one_forty_days_earlier_and_one_number_lower() {
        for round in -52..52 {
            let date = first_of(round);
            let first = MixtecYearCalendar.to_fixed(date).expect("in range");
            // 1 Atemoztli here is 6 Atemoztli there, and the months agree
            // from Izcalli to Panquetzaliztli.
            let aztec = AztecXiuhpohualliCalendar
                .from_fixed(first)
                .expect("any day");
            assert_eq!(aztec.position, XiuhpohualliPosition::new(17, 6), "{round}");
            let izcalli = Rd(first.0 + 40);
            assert_eq!(
                AztecXiuhpohualliCalendar
                    .from_fixed(izcalli)
                    .expect("any day")
                    .position,
                XiuhpohualliPosition::new(1, 1)
            );
            assert_eq!(mixtec(izcalli).position.to_string(), "1 Izcalli");
            // The Aztec bearer, 20 Tititl, is forty days after the Mixtec.
            let naming = Rd(first.0 + BEARER_DAY);
            let aztec_bearer = tonalpohualli(Rd(naming.0 + 40));
            assert_eq!(aztec_bearer.sign, date.year_bearer().sign);
            assert_eq!(aztec_bearer.number, date.year_bearer().number % 13 + 1);
        }
        let reed_1519 = (-52..0)
            .map(first_of)
            .find(|date| {
                let naming =
                    Rd(MixtecYearCalendar.to_fixed(*date).expect("in range").0 + BEARER_DAY);
                julian::from_fixed(naming).expect("in range").0 == 1519
            })
            .expect("a year named in 1519");
        assert_eq!(reed_1519.year_name(), (13, "Acatl"));
    }

    #[test]
    fn the_year_bearers_are_the_four_and_run_fifty_two_years() {
        for round in 0..52 {
            let date = first_of(round);
            let index = usize::from(date.year_bearer_index()) - 1;
            assert_eq!(date.year_bearer().sign, YEAR_BEARER_SIGNS[index], "{round}");
            let naming = Rd(MixtecYearCalendar.to_fixed(date).expect("in range").0 + BEARER_DAY);
            assert_eq!(tonalpohualli(naming), date.year_bearer());
            assert_eq!(first_of(round + 52).year_bearer(), date.year_bearer());
        }
        // After 10 Tecpatl, 11 Calli, 12 Tochtli, 13 Acatl.
        assert_eq!(first_of(1).year_name(), (11, "Calli"));
        assert_eq!(first_of(2).year_name(), (12, "Tochtli"));
        assert_eq!(first_of(3).year_name(), (13, "Acatl"));
        for round in [-1_000_000_007, -53, 0, 1, 52, 999_999_937] {
            let date = first_of(round);
            let direct = i128::from(NEW_YEAR_1567.0 - TONALPOHUALLI_EPOCH + BEARER_DAY)
                + i128::from(round) * i128::from(YEAR_DAYS);
            assert_eq!(
                date.year_bearer().ordinal(),
                Ok(direct.rem_euclid(260) as i64)
            );
        }
    }

    /// Every day of the calendar round before and after 1567's new year
    /// round-trips: every day in a release build; in a debug one every
    /// seventh day and every new year with the day before it
    /// (docs/policy.md §7).
    #[test]
    fn every_day_of_a_calendar_round_round_trips() {
        let new_years = (-26..=26).map(|round| NEW_YEAR_1567.0 + round * YEAR_DAYS);
        let (first, last) = (NEW_YEAR_1567.0 - 9_490, NEW_YEAR_1567.0 + 9_489);
        for day in crate::sweep_days(first, last, 7, new_years) {
            let rd = Rd(day);
            let date = mixtec(rd);
            assert_eq!(MixtecYearCalendar.to_fixed(date), Ok(rd));
            let fields = MixtecYearCalendar.to_fields(date).expect("describable");
            assert_eq!(MixtecYearCalendar.from_fields(&fields), Ok(date));
        }
    }

    #[test]
    fn impossible_positions_and_fields_are_refused() {
        for (month, day, error) in [
            (19, 6, CalendarError::DayOutOfRange),
            (18, 21, CalendarError::DayOutOfRange),
            (1, 0, CalendarError::DayOutOfRange),
            (20, 1, CalendarError::MonthOutOfRange),
            (0, 1, CalendarError::MonthOutOfRange),
        ] {
            let date = MixtecYearDate {
                round: 0,
                position: MixtecPosition::new(month, day),
            };
            assert_eq!(MixtecYearCalendar.to_fixed(date), Err(error));
        }
        assert_eq!(MixtecPosition::new(20, 1).to_string(), "1 ?20");
        assert_eq!(
            MixtecYearCalendar.to_fixed(MixtecYearDate {
                round: i64::MAX,
                position: MixtecPosition::new(1, 1),
            }),
            Err(CalendarError::Overflow)
        );
        assert_eq!(
            MixtecYearCalendar.from_fixed(Rd(i64::MIN)),
            Err(CalendarError::Overflow)
        );
        let fields = MixtecYearCalendar
            .to_fields(mixtec(jul(1568, 11, 27)))
            .expect("describable");
        assert_eq!(fields.extra.get("year_bearer_number"), Some(10));
        assert_eq!(fields.extra.get("year_bearer"), Some(2));
        let mut leap = fields;
        leap.month = Some(Month::leap(1));
        assert_eq!(
            MixtecYearCalendar.from_fields(&leap),
            Err(CalendarError::MonthOutOfRange)
        );
        let mut wrong_number = fields;
        wrong_number
            .extra
            .set("year_bearer_number", 11)
            .expect("room");
        assert_eq!(
            MixtecYearCalendar.from_fields(&wrong_number),
            Err(CalendarError::DayOutOfRange)
        );
        let mut wrong_bearer = fields;
        wrong_bearer.extra.set("year_bearer", 3).expect("room");
        assert_eq!(
            MixtecYearCalendar.from_fields(&wrong_bearer),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn the_metadata_says_the_year_is_unbounded_and_has_no_leap_year() {
        let meta = MixtecYearCalendar.meta();
        assert_eq!(meta.id, CalendarId("mixtec-year"));
        assert!(meta.earliest.is_none() && meta.latest.is_none());
        assert!(!meta.is_astronomical && !meta.has_leap_months);
        assert_eq!(
            MixtecYearCalendar.is_leap_year(0),
            Err(CalendarError::UnsupportedField("year"))
        );
        assert!(!MixtecYearCalendar.usage().is_recorded());
        // Every month name is an Aztec one.
        for name in MONTHS {
            assert!(crate::aztec::MONTHS.contains(&name), "{name}");
        }
        for (name, sign) in YEAR_BEARERS.iter().zip(YEAR_BEARER_SIGNS) {
            assert_eq!(crate::aztec::DAY_SIGNS[usize::from(sign) - 1], *name);
        }
    }
}
