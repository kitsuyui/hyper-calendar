//! The Balinese Pawukon: ten week cycles running at once over 210 days.
//!
//! The Pawukon is not a calendar of years. It is a 210-day period divided
//! simultaneously by weeks of one, two, three, four, five, six, seven,
//! eight, nine and ten days, and a Balinese day is named by its position in
//! several of them at once — *Buda Kliwon Dungulan* is the Wednesday
//! (4th of seven) that is also Kliwon (5th of five) in the *wuku*
//! Dungulan. Religious observances key off particular coincidences, which
//! is the whole point: Galungan falls on Buda Kliwon Dungulan, once every
//! 210 days.
//!
//! # Why 210, and what the irregular cycles are for
//!
//! 210 = 2 × 3 × 5 × 7, so the 1-, 2-, 3-, 5-, 6-, 7- and 10-day weeks
//! divide it evenly and simply repeat. The 4-, 8- and 9-day weeks do not,
//! and the tradition patches each one differently:
//!
//! * The **nine-day** week (Sangawara) does not start until day 4, so days
//!   0 to 3 all count as its first day.
//! * The **eight-day** week (Astawara) pauses for three days in the wuku
//!   Dungulan: days 70, 71 and 72 all count as its seventh day, Kala.
//! * The **four-day** week (Caturwara) has no rule of its own; it is the
//!   eight-day week taken modulo four, so it inherits the pause.
//!
//! The **two-day** week (Dwiwara) and the **one-day** week (Ekawara) are
//! not counted at all: they are read off the ten-day week's parity. The
//! ten-day week is itself computed by adding the *urip* — the numerological
//! weights — of the five- and seven-day weeks, which is why it is the only
//! cycle here that is not a modulo.
//!
//! # Sources
//!
//! The arithmetic, the epoch and the three irregular rules follow Reingold
//! and Dershowitz, *Calendrical Calculations* (4th ed., 2018), §10.6, which
//! anchors the Pawukon at Julian Day Number 146. The urip values are the
//! standard ones, and are the same numbers the Javanese *neptu* uses.
//!
//! The implementation was checked against five published Galungan dates —
//! 2024-02-28, 2024-09-25, 2025-04-23, 2025-11-19 and 2026-06-17 — each of
//! which must come out as Buda Kliwon Dungulan, day 73 of the Pawukon.
//!
//! # A cycle, not a calendar
//!
//! There is no Pawukon year. [`PawukonDate`] therefore carries a `round`:
//! how many complete 210-day periods have elapsed since the epoch. Nothing
//! in Bali counts those rounds; it exists so that the type can round-trip
//! through a fixed day, as [`hc_calendar::Calendar`] requires.

use core::fmt;

use hc_calendar::fields::ExtraFields;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind, weekday::DayCycle,
};

/// The length of the Pawukon in days.
pub const PAWUKON_CYCLE: i64 = 210;

/// The number of *wuku*, the seven-day weeks the Pawukon is divided into.
pub const WUKU_COUNT: usize = 30;

/// The fixed day of Pawukon day 0, Julian Day Number 146.
pub const EPOCH: Rd = Rd(146 - hc_calendar::fixed::JDN_OF_RD_ZERO);

/// The thirty *wuku*, the named seven-day weeks.
pub const WUKU: [&str; WUKU_COUNT] = [
    "Sinta",
    "Landep",
    "Ukir",
    "Kulantir",
    "Taulu",
    "Gumbreg",
    "Wariga",
    "Warigadean",
    "Julungwangi",
    "Sungsang",
    "Dungulan",
    "Kuningan",
    "Langkir",
    "Medangsia",
    "Pujut",
    "Pahang",
    "Krulut",
    "Merakih",
    "Tambir",
    "Medangkungan",
    "Matal",
    "Uye",
    "Menail",
    "Prangbakat",
    "Bala",
    "Ugu",
    "Wayang",
    "Kelawu",
    "Dukut",
    "Watugunung",
];

/// The one name of the one-day week, Ekawara.
pub const EKAWARA: [&str; 1] = ["Luang"];

/// The two names of the two-day week, Dwiwara.
pub const DWIWARA: [&str; 2] = ["Menga", "Pepet"];

/// The three names of the three-day week, Triwara.
pub const TRIWARA: [&str; 3] = ["Pasah", "Beteng", "Kajeng"];

/// The four names of the four-day week, Caturwara.
pub const CATURWARA: [&str; 4] = ["Sri", "Laba", "Jaya", "Menala"];

/// The five names of the five-day week, Pancawara — the same cycle as the
/// Javanese *pasaran*.
pub const PANCAWARA: [&str; 5] = ["Umanis", "Paing", "Pon", "Wage", "Kliwon"];

/// The six names of the six-day week, Sadwara.
pub const SADWARA: [&str; 6] = ["Tungleh", "Aryang", "Urukung", "Paniron", "Was", "Maulu"];

/// The seven names of the seven-day week, Saptawara.
pub const SAPTAWARA: [&str; 7] = [
    "Redite",
    "Coma",
    "Anggara",
    "Buda",
    "Wraspati",
    "Sukra",
    "Saniscara",
];

/// The eight names of the eight-day week, Astawara.
pub const ASTAWARA: [&str; 8] = [
    "Sri", "Indra", "Guru", "Yama", "Ludra", "Brahma", "Kala", "Uma",
];

/// The nine names of the nine-day week, Sangawara.
pub const SANGAWARA: [&str; 9] = [
    "Dangu", "Jangur", "Gigis", "Nohan", "Ogan", "Erangan", "Urungan", "Tulus", "Dadi",
];

/// The ten names of the ten-day week, Dasawara.
pub const DASAWARA: [&str; 10] = [
    "Pandita", "Pati", "Suka", "Duka", "Sri", "Manuh", "Manusa", "Raja", "Dewa", "Raksasa",
];

/// The *urip* of the five-day week, in cycle order.
///
/// These weights are what the ten-day week is built from, and the same
/// numbers give the Javanese *neptu* of a *weton*.
pub const PANCAWARA_URIP: [i64; 5] = [5, 9, 7, 4, 8];

/// The *urip* of the seven-day week, in cycle order.
pub const SAPTAWARA_URIP: [i64; 7] = [5, 4, 3, 7, 8, 6, 9];

/// A day in the Pawukon: a position in the 210-day period, plus the round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PawukonDate {
    /// Complete 210-day periods elapsed since [`EPOCH`].
    pub round: i64,
    /// The day within the Pawukon, 0 to 209.
    pub day: u16,
}

impl PawukonDate {
    /// A date, without validation; the calendar validates it.
    #[must_use]
    pub const fn new(round: i64, day: u16) -> Self {
        Self { round, day }
    }

    /// The *wuku*, 1 to 30.
    #[must_use]
    pub const fn wuku(self) -> u8 {
        (self.day / 7 + 1) as u8
    }

    /// The *wuku*'s name.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] when the day is outside
    /// `0..210`.
    pub const fn wuku_name(self) -> CalendarResult<&'static str> {
        if self.day >= 210 {
            return Err(CalendarError::MonthOutOfRange);
        }
        Ok(WUKU[(self.day / 7) as usize])
    }

    /// Whether the day is *Luang*, the one-day week's single name.
    ///
    /// The one-day week is a parity rather than a rotation: a day is Luang
    /// exactly when the ten-day week's position is even.
    #[must_use]
    pub const fn is_luang(self) -> bool {
        self.dasawara_index() % 2 == 0
    }

    /// The two-day week, 1 or 2.
    #[must_use]
    pub const fn dwiwara(self) -> u8 {
        if self.dasawara_index() % 2 == 1 { 1 } else { 2 }
    }

    /// The three-day week, 1 to 3.
    #[must_use]
    pub const fn triwara(self) -> u8 {
        (self.day % 3) as u8 + 1
    }

    /// The four-day week, 1 to 4.
    ///
    /// It has no rule of its own: it is [`PawukonDate::astawara`] reduced
    /// modulo four, and so inherits that cycle's three-day pause.
    #[must_use]
    pub const fn caturwara(self) -> u8 {
        (self.astawara() - 1) % 4 + 1
    }

    /// The five-day week, 1 to 5.
    ///
    /// The `+ 1` is the offset between the Pawukon's day 0 and the start of
    /// the five-day week; it is what makes Galungan fall on Kliwon.
    #[must_use]
    pub const fn pancawara(self) -> u8 {
        ((self.day as i64 + 1).rem_euclid(5)) as u8 + 1
    }

    /// The six-day week, 1 to 6.
    #[must_use]
    pub const fn sadwara(self) -> u8 {
        (self.day % 6) as u8 + 1
    }

    /// The seven-day week, 1 to 7. Day 0 of the Pawukon is Redite, a
    /// Sunday, and the two have never slipped.
    #[must_use]
    pub const fn saptawara(self) -> u8 {
        (self.day % 7) as u8 + 1
    }

    /// The eight-day week, 1 to 8.
    ///
    /// The `max` is the irregularity: the cycle stalls on Kala for days 70,
    /// 71 and 72, which is the first half of the wuku Dungulan.
    #[must_use]
    pub const fn astawara(self) -> u8 {
        let shifted = (self.day as i64 - 70).rem_euclid(PAWUKON_CYCLE) + 4;
        let held = if shifted < 6 { 6 } else { shifted };
        (held.rem_euclid(8)) as u8 + 1
    }

    /// The nine-day week, 1 to 9.
    ///
    /// The irregularity here is at the other end: the cycle does not begin
    /// until day 4, so days 0 to 3 are all Dangu.
    #[must_use]
    pub const fn sangawara(self) -> u8 {
        let shifted = self.day as i64 - 3;
        let held = if shifted < 0 { 0 } else { shifted };
        (held.rem_euclid(9)) as u8 + 1
    }

    /// The ten-day week, 1 to 10.
    ///
    /// Unlike every other cycle this is not a modulo of the day number: it
    /// is one plus the *urip* of the five- and seven-day weeks, reduced
    /// modulo ten.
    #[must_use]
    pub const fn dasawara(self) -> u8 {
        self.dasawara_index() as u8 + 1
    }

    /// The zero-based ten-day-week position, which the one- and two-day
    /// weeks are read from.
    const fn dasawara_index(self) -> i64 {
        let five = PANCAWARA_URIP[(self.pancawara() - 1) as usize];
        let seven = SAPTAWARA_URIP[(self.saptawara() - 1) as usize];
        (1 + five + seven).rem_euclid(10)
    }

    /// The *urip* of the five-day week's position.
    #[must_use]
    pub const fn pancawara_urip(self) -> i64 {
        PANCAWARA_URIP[(self.pancawara() - 1) as usize]
    }

    /// The *urip* of the seven-day week's position.
    #[must_use]
    pub const fn saptawara_urip(self) -> i64 {
        SAPTAWARA_URIP[(self.saptawara() - 1) as usize]
    }

    /// The *urip* of the day: the five-day and seven-day weights added,
    /// which the Javanese call the *neptu*.
    #[must_use]
    pub const fn urip(self) -> i64 {
        self.pancawara_urip() + self.saptawara_urip()
    }
}

impl fmt::Display for PawukonDate {
    /// Writes the three cycles a Balinese date is usually given in:
    /// seven-day, five-day, *wuku* — `Buda Kliwon Dungulan`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let saptawara = SAPTAWARA[(self.saptawara() - 1) as usize];
        let pancawara = PANCAWARA[(self.pancawara() - 1) as usize];
        match self.wuku_name() {
            Ok(wuku) => write!(f, "{saptawara} {pancawara} {wuku}"),
            Err(_) => write!(f, "{saptawara} {pancawara} ?{}", self.day),
        }
    }
}

/// The Balinese Pawukon.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BalinesePawukonCalendar;

/// The Pawukon as a plain [`DayCycle`], for callers that only want the day
/// number.
pub const CYCLE: DayCycle = DayCycle::new(PAWUKON_CYCLE as u16, EPOCH);

/// The thirty *wuku* and the ten concurrent weeks, shortest first.
const PAWUKON_SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::fixed("wuku", 30),
    hc_calendar::shape::CycleShape::fixed("ekawara", 1),
    hc_calendar::shape::CycleShape::fixed("dwiwara", 2),
    hc_calendar::shape::CycleShape::fixed("triwara", 3),
    hc_calendar::shape::CycleShape::fixed("caturwara", 4),
    hc_calendar::shape::CycleShape::fixed("pancawara", 5),
    hc_calendar::shape::CycleShape::fixed("sadwara", 6),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
    hc_calendar::shape::CycleShape::fixed("astawara", 8),
    hc_calendar::shape::CycleShape::fixed("sangawara", 9),
    hc_calendar::shape::CycleShape::fixed("dasawara", 10),
];

impl Calendar for BalinesePawukonCalendar {
    type Date = PawukonDate;

    /// The thirty *wuku* and the ten concurrent weeks.
    ///
    /// The seven-day *saptawara* is the same week as everyone else's, Redite
    /// being Sunday, so it is declared as `weekday` and a locale's weekday
    /// names serve it. The other nine keep their Balinese names.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        PAWUKON_SHAPE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("balinese-pawukon"),
            english_name: "Balinese Pawukon",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        if date.day >= 210 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(Rd(EPOCH.0
            + date.round * PAWUKON_CYCLE
            + i64::from(date.day)))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let count = rd.0 - EPOCH.0;
        Ok(PawukonDate {
            round: count.div_euclid(PAWUKON_CYCLE),
            day: count.rem_euclid(PAWUKON_CYCLE) as u16,
        })
    }

    /// Describes the day by *wuku* and seven-day week, with the other
    /// concurrent cycles as extra fields.
    ///
    /// Eight of the ten fit: [`hc_calendar::ExtraFields`] holds eight, the
    /// seven-day week is already the `day` field, and the one-day week is a
    /// parity of the ten-day week rather than a number. Everything is
    /// reconstructible from the `month` and `day` pair alone, so nothing is
    /// lost.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::Overflow`] if the extra-field set fills,
    /// which the eight fields here cannot make happen.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("dwiwara", date.dwiwara().into())?;
        extra.set("triwara", date.triwara().into())?;
        extra.set("caturwara", date.caturwara().into())?;
        extra.set("pancawara", date.pancawara().into())?;
        extra.set("sadwara", date.sadwara().into())?;
        extra.set("astawara", date.astawara().into())?;
        extra.set("sangawara", date.sangawara().into())?;
        extra.set("dasawara", date.dasawara().into())?;
        Ok(DateFields {
            era: None,
            year: date.round,
            month: Some(Month::regular(date.wuku())),
            day: Some(date.saptawara()),
            leap_day: false,
            extra,
        })
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap || month.ordinal == 0 || month.ordinal as usize > WUKU_COUNT {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        if day == 0 || day > 7 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(PawukonDate {
            round: fields.year,
            day: u16::from(month.ordinal - 1) * 7 + u16::from(day - 1),
        })
    }
}

#[cfg(test)]
mod tests {
    use hc_calendar::Weekday;
    use hc_calendars_solar::gregorian;

    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    fn pawukon(year: i64, month: u8, day: u8) -> PawukonDate {
        BalinesePawukonCalendar
            .from_fixed(greg(year, month, day))
            .expect("any day")
    }

    #[test]
    fn galungan_always_falls_on_buda_kliwon_dungulan() {
        // Five published Galungan dates, 210 days apart. Getting all five
        // right at once exercises the epoch and the five- and seven-day
        // cycles together.
        for (year, month, day) in [
            (2024, 2, 28),
            (2024, 9, 25),
            (2025, 4, 23),
            (2025, 11, 19),
            (2026, 6, 17),
        ] {
            let date = pawukon(year, month, day);
            assert_eq!(date.to_string(), "Buda Kliwon Dungulan", "{year}-{month}");
            assert_eq!(date.day, 73);
            assert_eq!(date.wuku(), 11);
            assert_eq!(Weekday::from_rd(greg(year, month, day)), Weekday::Wednesday);
        }
        // And they really are 210 days apart.
        assert_eq!(greg(2024, 9, 25).0 - greg(2024, 2, 28).0, 210);
    }

    #[test]
    fn the_seven_day_week_never_slipped_against_the_gregorian_one() {
        // Redite is Sunday. If the epoch were off by a day this would fail
        // on every date at once.
        for offset in 0..500 {
            let rd = Rd(greg(2000, 1, 1).0 + offset);
            let date = BalinesePawukonCalendar.from_fixed(rd).expect("any day");
            let expected = Weekday::from_rd(rd).sunday_first_number() + 1;
            assert_eq!(date.saptawara(), expected, "{rd}");
        }
    }

    #[test]
    fn the_pawukon_is_thirty_wuku_of_seven_days() {
        assert_eq!(WUKU.len(), WUKU_COUNT);
        assert_eq!(WUKU_COUNT as i64 * 7, PAWUKON_CYCLE);
        assert_eq!(WUKU[0], "Sinta");
        assert_eq!(WUKU[WUKU_COUNT - 1], "Watugunung");
        for day in 0..210u16 {
            let date = PawukonDate::new(0, day);
            assert_eq!(date.wuku(), (day / 7 + 1) as u8);
            assert_eq!(date.wuku_name(), Ok(WUKU[(day / 7) as usize]));
        }
        assert_eq!(
            PawukonDate::new(0, 210).wuku_name(),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn the_regular_cycles_divide_the_pawukon_evenly() {
        // 210 = 2 * 3 * 5 * 7, so these repeat without a patch.
        for day in 0..210u16 {
            let here = PawukonDate::new(0, day);
            let next = PawukonDate::new(0, (day + 1) % 210);
            assert_eq!(here.triwara() % 3 + 1, next.triwara());
            assert_eq!(here.sadwara() % 6 + 1, next.sadwara());
            assert_eq!(here.saptawara() % 7 + 1, next.saptawara());
            assert_eq!(here.pancawara() % 5 + 1, next.pancawara());
        }
    }

    #[test]
    fn the_nine_day_week_does_not_start_until_day_four() {
        for day in 0..4u16 {
            assert_eq!(PawukonDate::new(0, day).sangawara(), 1, "day {day}");
        }
        assert_eq!(PawukonDate::new(0, 4).sangawara(), 2);
        assert_eq!(PawukonDate::new(0, 12).sangawara(), 1);
        // From day 4 on it advances by one a day for the rest of the cycle.
        for day in 4..209u16 {
            let here = PawukonDate::new(0, day).sangawara();
            let next = PawukonDate::new(0, day + 1).sangawara();
            assert_eq!(here % 9 + 1, next, "day {day}");
        }
        // 210 - 3 = 207 = 23 * 9, so the cycle closes exactly at the wrap.
        assert_eq!(PawukonDate::new(0, 209).sangawara(), 9);
    }

    #[test]
    fn the_eight_day_week_pauses_for_three_days_in_dungulan() {
        // Days 70, 71 and 72 are all Kala, the seventh name.
        for day in 70..=72u16 {
            let date = PawukonDate::new(0, day);
            assert_eq!(date.astawara(), 7, "day {day}");
            assert_eq!(ASTAWARA[(date.astawara() - 1) as usize], "Kala");
            assert_eq!(date.wuku_name(), Ok("Dungulan"));
        }
        assert_eq!(PawukonDate::new(0, 69).astawara(), 6);
        assert_eq!(PawukonDate::new(0, 73).astawara(), 8);
        // Everywhere else it advances by one a day.
        let mut pauses = 0;
        for day in 0..209u16 {
            let here = PawukonDate::new(0, day).astawara();
            let next = PawukonDate::new(0, day + 1).astawara();
            if here == next {
                pauses += 1;
            } else {
                assert_eq!(here % 8 + 1, next, "day {day}");
            }
        }
        assert_eq!(pauses, 2, "the pause holds one name for three days");
    }

    #[test]
    fn the_four_day_week_inherits_the_eight_day_weeks_pause() {
        for day in 70..=72u16 {
            assert_eq!(PawukonDate::new(0, day).caturwara(), 3);
            assert_eq!(CATURWARA[2], "Jaya");
        }
        for day in 0..210u16 {
            let date = PawukonDate::new(0, day);
            assert_eq!(date.caturwara(), (date.astawara() - 1) % 4 + 1);
            assert!((1..=4).contains(&date.caturwara()));
        }
    }

    #[test]
    fn the_ten_day_week_is_built_from_urip_not_from_the_day_number() {
        assert_eq!(PANCAWARA_URIP.iter().sum::<i64>(), 33);
        assert_eq!(SAPTAWARA_URIP.iter().sum::<i64>(), 42);
        for day in 0..210u16 {
            let date = PawukonDate::new(0, day);
            let expected = (1
                + PANCAWARA_URIP[(date.pancawara() - 1) as usize]
                + SAPTAWARA_URIP[(date.saptawara() - 1) as usize])
                .rem_euclid(10) as u8
                + 1;
            assert_eq!(date.dasawara(), expected, "day {day}");
            assert!((1..=10).contains(&date.dasawara()));
            assert_eq!(date.urip(), date.pancawara_urip() + date.saptawara_urip());
        }
    }

    #[test]
    fn the_one_and_two_day_weeks_are_the_ten_day_weeks_parity() {
        for day in 0..210u16 {
            let date = PawukonDate::new(0, day);
            assert_eq!(date.is_luang(), date.dwiwara() == 2);
            assert!(date.dwiwara() == 1 || date.dwiwara() == 2);
        }
        // Both names are used somewhere in the cycle.
        assert!((0..210).any(|day| PawukonDate::new(0, day).is_luang()));
        assert!((0..210).any(|day| !PawukonDate::new(0, day).is_luang()));
        assert_eq!(EKAWARA.len(), 1);
        assert_eq!(DWIWARA.len(), 2);
    }

    #[test]
    fn every_cycle_has_exactly_as_many_names_as_it_has_days() {
        assert_eq!(TRIWARA.len(), 3);
        assert_eq!(CATURWARA.len(), 4);
        assert_eq!(PANCAWARA.len(), 5);
        assert_eq!(SADWARA.len(), 6);
        assert_eq!(SAPTAWARA.len(), 7);
        assert_eq!(ASTAWARA.len(), 8);
        assert_eq!(SANGAWARA.len(), 9);
        assert_eq!(DASAWARA.len(), 10);
        assert_eq!(PANCAWARA_URIP.len(), PANCAWARA.len());
        assert_eq!(SAPTAWARA_URIP.len(), SAPTAWARA.len());
    }

    #[test]
    fn every_day_of_a_whole_pawukon_round_trips() {
        for day in 0..210u16 {
            let date = PawukonDate::new(3, day);
            let rd = BalinesePawukonCalendar.to_fixed(date).expect("a valid day");
            assert_eq!(BalinesePawukonCalendar.from_fixed(rd), Ok(date));
            let fields = BalinesePawukonCalendar
                .to_fields(date)
                .expect("describable");
            assert_eq!(BalinesePawukonCalendar.from_fields(&fields), Ok(date));
        }
    }

    #[test]
    fn fields_carry_eight_of_the_ten_cycles() {
        let date = pawukon(2026, 6, 17);
        let fields = BalinesePawukonCalendar
            .to_fields(date)
            .expect("describable");
        assert_eq!(fields.extra.len(), 8);
        assert_eq!(fields.month, Some(Month::regular(11)));
        assert_eq!(fields.day, Some(4));
        assert_eq!(fields.extra.get("pancawara"), Some(5));
        assert_eq!(fields.extra.get("triwara"), Some(2));
        assert_eq!(fields.extra.get("astawara"), Some(8));
        assert_eq!(fields.extra.get("sangawara"), Some(8));
        assert_eq!(BalinesePawukonCalendar.from_fields(&fields), Ok(date));
    }

    #[test]
    fn impossible_positions_are_refused() {
        assert_eq!(
            BalinesePawukonCalendar.to_fixed(PawukonDate::new(0, 210)),
            Err(CalendarError::DayOutOfRange)
        );
        let good = BalinesePawukonCalendar
            .to_fields(PawukonDate::new(0, 0))
            .expect("describable");
        let mut bad = good;
        bad.month = Some(Month::regular(31));
        assert_eq!(
            BalinesePawukonCalendar.from_fields(&bad),
            Err(CalendarError::MonthOutOfRange)
        );
        let mut bad = good;
        bad.month = Some(Month::leap(1));
        assert_eq!(
            BalinesePawukonCalendar.from_fields(&bad),
            Err(CalendarError::MonthOutOfRange)
        );
        let mut bad = good;
        bad.day = Some(8);
        assert_eq!(
            BalinesePawukonCalendar.from_fields(&bad),
            Err(CalendarError::DayOutOfRange)
        );
        let mut bad = good;
        bad.day = Some(0);
        assert_eq!(
            BalinesePawukonCalendar.from_fields(&bad),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn the_plain_day_cycle_agrees_with_the_calendar() {
        for offset in (0..4_000).step_by(11) {
            let rd = Rd(greg(1900, 1, 1).0 + offset);
            let date = BalinesePawukonCalendar.from_fixed(rd).expect("any day");
            assert_eq!(CYCLE.position(rd), Some(date.day));
        }
    }

    #[test]
    fn the_epoch_is_julian_day_one_hundred_and_forty_six() {
        assert_eq!(EPOCH.to_julian_day_number(), 146);
        let start = BalinesePawukonCalendar.from_fixed(EPOCH).expect("any day");
        assert_eq!(start.day, 0);
        assert_eq!(start.round, 0);
        assert_eq!(start.wuku_name(), Ok("Sinta"));
        assert_eq!(start.saptawara(), 1);
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Sunday);
    }
}
