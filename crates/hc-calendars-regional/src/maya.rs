//! The four Maya calendars: long count, tzolk'in, haab and Calendar Round.
//!
//! A Classic Maya inscription dates an event in all of them at once. The
//! long count says how many days have elapsed since a mythological zero;
//! the tzolk'in and haab say where the day falls in a 260-day ritual cycle
//! and a 365-day vague year; the Calendar Round is the pair of the last
//! two, which repeats every 18 980 days. Nothing in any of them names a
//! Western date: that takes a **correlation constant**, the Julian Day
//! Number of `0.0.0.0.0`, and this module registers three published
//! values as three calendars of each count — [`GMT_CORRELATION`] = 584 283
//! as `maya-longcount`, `maya-tzolkin`, `maya-haab` and `maya-round`;
//! [`GMT_PLUS_TWO_CORRELATION`] = 584 285 as `maya-longcount-gmt2`,
//! `maya-tzolkin-gmt2`, `maya-haab-gmt2` and `maya-round-gmt2`; and
//! [`MARTIN_SKIDMORE_CORRELATION`] = 584 286 as `maya-longcount-584286`,
//! `maya-tzolkin-584286`, `maya-haab-584286` and `maya-round-584286` —
//! rather than as a switch, because a correlation is a claim about history,
//! and a Calendar Round read beside a long count has to be read under the
//! same constant or it is out by the difference. No other constant can be
//! chosen: each calendar type is built only from these three. The counts, the history of the
//! constants, a worked reading of a monument and the sources are in
//! [`docs/systems/mesoamerican-counts.md`](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/systems/mesoamerican-counts.md).
//!
//! The tzolk'in, the haab and the Calendar Round are cycles, so each date
//! type here carries a `round`: how many complete cycles have elapsed since
//! [`EPOCH`]'s cycle began. See the crate documentation for why.
//!
//! The arithmetic follows Reingold and Dershowitz, *Calendrical
//! Calculations* (4th ed., 2018; `reingold2018`), chapter 11, "The Mayan
//! Calendars", as their published code
//! states it (`reingold2018code` in `docs/references.bib`); the day and
//! month names are the sixteenth-century Yucatec spelling.

use core::fmt;

use hc_calendar::fields::ExtraFields;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};

/// The Goodman–Martínez–Thompson correlation constant: the Julian Day
/// Number of long count `0.0.0.0.0`.
pub const GMT_CORRELATION: i64 = 584_283;

/// The alternative correlation constant, two days later: Thompson's earlier
/// value, kept in use by Lounsbury (`martin2012`, `wikipedia-long-count`).
///
/// Stated so that the difference is visible in the source rather than only
/// in prose; the `-gmt2` calendars are anchored to it.
pub const GMT_PLUS_TWO_CORRELATION: i64 = 584_285;

/// Martin and Skidmore's correlation constant, three days after GMT.
///
/// Santa Elena Poco Uinic Stela 3 records what they read as the total
/// solar eclipse of 16 July 790 (Julian), JDN 2 009 802, at 9.17.19.13.16
/// 5 Kib 14 Chʼen, whose Maya day number is 1 425 516; the difference is
/// 584 286 (`martin2012`). The `-584286` calendars are anchored to it.
pub const MARTIN_SKIDMORE_CORRELATION: i64 = 584_286;

/// The fixed day of long count `0.0.0.0.0` under [`GMT_CORRELATION`].
pub const EPOCH: Rd = Rd(GMT_CORRELATION - hc_calendar::fixed::JDN_OF_RD_ZERO);

/// Days in a tzolk'in cycle: 13 numbers by 20 day-names.
pub const TZOLKIN_CYCLE: i64 = 260;

/// Days in a haab year: 18 months of 20 days plus the five-day Uayeb.
pub const HAAB_CYCLE: i64 = 365;

/// Days in a Calendar Round: the least common multiple of 260 and 365.
pub const CALENDAR_ROUND_CYCLE: i64 = 18_980;

/// The twenty tzolk'in day-names, in cycle order.
pub const TZOLKIN_NAMES: [&str; 20] = [
    "Imix", "Ik", "Akbal", "Kan", "Chicchan", "Cimi", "Manik", "Lamat", "Muluc", "Oc", "Chuen",
    "Eb", "Ben", "Ix", "Men", "Cib", "Caban", "Etznab", "Cauac", "Ahau",
];

/// The nineteen haab months: eighteen of twenty days and the five-day
/// Uayeb.
pub const HAAB_MONTHS: [&str; 19] = [
    "Pop", "Uo", "Zip", "Zotz", "Tzec", "Xul", "Yaxkin", "Mol", "Chen", "Yax", "Zac", "Ceh", "Mac",
    "Kankin", "Muan", "Pax", "Kayab", "Cumku", "Uayeb",
];

/// A position in the 260-day tzolk'in, without saying which cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TzolkinPosition {
    /// The number, 1 to 13.
    pub number: u8,
    /// The day-name, 1 to 20, indexing [`TZOLKIN_NAMES`].
    pub name: u8,
}

impl TzolkinPosition {
    /// A position, without validation.
    #[must_use]
    pub const fn new(number: u8, name: u8) -> Self {
        Self { number, name }
    }

    /// The day-name as a string.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when the name is not in
    /// `1..=20`.
    pub const fn name_str(self) -> CalendarResult<&'static str> {
        if self.name == 0 || self.name > 20 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(TZOLKIN_NAMES[(self.name - 1) as usize])
    }

    /// The position's ordinal within the cycle, 0 to 259.
    ///
    /// The `39 * (number - name)` term is what makes the two wheels turn
    /// together: 39 is the inverse of 20 modulo 13 scaled to 260, so the
    /// expression solves the pair of congruences in one step.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] for a number outside
    /// `1..=13` or a name outside `1..=20`.
    pub const fn ordinal(self) -> CalendarResult<i64> {
        if self.number == 0 || self.number > 13 || self.name == 0 || self.name > 20 {
            return Err(CalendarError::DayOutOfRange);
        }
        let number = self.number as i64;
        let name = self.name as i64;
        Ok((number - 1 + 39 * (number - name)).rem_euclid(TZOLKIN_CYCLE))
    }

    /// The position `ordinal` days into the cycle.
    #[must_use]
    pub const fn from_ordinal(ordinal: i64) -> Self {
        let ordinal = ordinal.rem_euclid(TZOLKIN_CYCLE);
        Self {
            number: (ordinal.rem_euclid(13) + 1) as u8,
            name: (ordinal.rem_euclid(20) + 1) as u8,
        }
    }
}

impl fmt::Display for TzolkinPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name_str() {
            Ok(name) => write!(f, "{} {name}", self.number),
            Err(_) => write!(f, "{} ?{}", self.number, self.name),
        }
    }
}

/// A position in the 365-day haab, without saying which year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HaabPosition {
    /// The month, 1 to 19, indexing [`HAAB_MONTHS`].
    pub month: u8,
    /// The day within the month, **counting from 0**: day 0 is the seating
    /// of the month. Months 1 to 18 run 0 to 19; Uayeb runs 0 to 4.
    pub day: u8,
}

impl HaabPosition {
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
        Ok(HAAB_MONTHS[(self.month - 1) as usize])
    }

    /// The position's ordinal within the haab year, 0 to 364.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] or
    /// [`CalendarError::DayOutOfRange`]; Uayeb has only five days.
    pub const fn ordinal(self) -> CalendarResult<i64> {
        if self.month == 0 || self.month > 19 {
            return Err(CalendarError::MonthOutOfRange);
        }
        let limit = if self.month == 19 { 4 } else { 19 };
        if self.day > limit {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok((self.month as i64 - 1) * 20 + self.day as i64)
    }

    /// The position `ordinal` days into the haab year.
    #[must_use]
    pub const fn from_ordinal(ordinal: i64) -> Self {
        let ordinal = ordinal.rem_euclid(HAAB_CYCLE);
        Self {
            month: (ordinal / 20 + 1) as u8,
            day: (ordinal % 20) as u8,
        }
    }
}

impl fmt::Display for HaabPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.month_str() {
            Ok(month) => write!(f, "{} {month}", self.day),
            Err(_) => write!(f, "{} ?{}", self.day, self.month),
        }
    }
}

/// Days from the start of the tzolk'in cycle that contains `0.0.0.0.0` to
/// the epoch itself, which is 4 Ahau, ordinal 159 (`reingold2018code`,
/// `mayan-tzolkin-epoch`).
const TZOLKIN_EPOCH_OFFSET: i64 = 159;

/// Days from the seating of Pop of the haab year that contains `0.0.0.0.0`
/// to the epoch itself, which is 8 Cumku, ordinal 348 (`reingold2018code`,
/// `mayan-haab-epoch`).
const HAAB_EPOCH_OFFSET: i64 = 348;

/// The fixed day whose tzolk'in ordinal is 0 under [`GMT_CORRELATION`].
const TZOLKIN_EPOCH: i64 = tzolkin_epoch_under(EPOCH);

/// The fixed day whose haab ordinal is 0, the seating of Pop, under
/// [`GMT_CORRELATION`].
const HAAB_EPOCH: i64 = haab_epoch_under(EPOCH);

/// The fixed day whose tzolk'in ordinal is 0 when `0.0.0.0.0` is `epoch`.
const fn tzolkin_epoch_under(epoch: Rd) -> i64 {
    epoch.0 - TZOLKIN_EPOCH_OFFSET
}

/// The fixed day whose haab ordinal is 0 when `0.0.0.0.0` is `epoch`.
const fn haab_epoch_under(epoch: Rd) -> i64 {
    epoch.0 - HAAB_EPOCH_OFFSET
}

/// A long count date, `baktun.katun.tun.uinal.kin`.
///
/// The places are mixed-radix: 20 kin to a uinal, **18** uinal to a tun, 20
/// tun to a katun, 20 katun to a baktun. The 18 is what makes a tun 360
/// days rather than 400, and it is the only irregular place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MayaLongCountDate {
    /// The baktun, 0 to 19: 144 000 days.
    pub baktun: i64,
    /// The katun, 0 to 19: 7 200 days.
    pub katun: u8,
    /// The tun, 0 to 19: 360 days.
    pub tun: u8,
    /// The uinal, 0 to 17: 20 days.
    pub uinal: u8,
    /// The kin, 0 to 19: one day.
    pub kin: u8,
}

/// The largest baktun this implementation represents.
pub const MAX_BAKTUN: i64 = 19;

impl MayaLongCountDate {
    /// A long count, without validation; the calendar validates it.
    #[must_use]
    pub const fn new(baktun: i64, katun: u8, tun: u8, uinal: u8, kin: u8) -> Self {
        Self {
            baktun,
            katun,
            tun,
            uinal,
            kin,
        }
    }

    /// Days elapsed since `0.0.0.0.0`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when a place is outside its
    /// radix, or [`CalendarError::YearOutOfRange`] for a baktun outside
    /// `0..=`[`MAX_BAKTUN`].
    pub const fn days(self) -> CalendarResult<i64> {
        if self.baktun < 0 || self.baktun > MAX_BAKTUN {
            return Err(CalendarError::YearOutOfRange);
        }
        if self.katun > 19 || self.tun > 19 || self.uinal > 17 || self.kin > 19 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(self.baktun * 144_000
            + self.katun as i64 * 7_200
            + self.tun as i64 * 360
            + self.uinal as i64 * 20
            + self.kin as i64)
    }

    /// The long count `days` days after `0.0.0.0.0`.
    #[must_use]
    pub const fn from_days(days: i64) -> Self {
        Self {
            baktun: days.div_euclid(144_000),
            katun: (days.rem_euclid(144_000) / 7_200) as u8,
            tun: (days.rem_euclid(7_200) / 360) as u8,
            uinal: (days.rem_euclid(360) / 20) as u8,
            kin: days.rem_euclid(20) as u8,
        }
    }
}

impl fmt::Display for MayaLongCountDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.baktun, self.katun, self.tun, self.uinal, self.kin
        )
    }
}

/// A tzolk'in date: a position plus the cycle it falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MayaTzolkinDate {
    /// Complete 260-day cycles elapsed since [`EPOCH`]'s cycle began.
    pub round: i64,
    /// The position within the cycle.
    pub position: TzolkinPosition,
}

/// A haab date: a position plus the vague year it falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MayaHaabDate {
    /// Complete 365-day years elapsed since [`EPOCH`]'s haab year began.
    pub round: i64,
    /// The position within the year.
    pub position: HaabPosition,
}

/// A Calendar Round date: both positions plus the 52-year round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MayaCalendarRoundDate {
    /// Complete 18 980-day rounds elapsed since [`EPOCH`].
    pub round: i64,
    /// The tzolk'in position.
    pub tzolkin: TzolkinPosition,
    /// The haab position.
    pub haab: HaabPosition,
}

impl fmt::Display for MayaCalendarRoundDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.tzolkin, self.haab)
    }
}

/// Days from `0.0.0.0.0` to `7.16.3.2.13`, Stela 2 at Chiapa de Corzo, the
/// earliest contemporaneous Long Count yet found, in 36 BCE.
pub const EARLIEST_ATTESTED_DAYS: i64 = 7 * 144_000 + 16 * 7_200 + 3 * 360 + 2 * 20 + 13;

/// Landa's Colonial Maya New Year, 12 Kʼan 1 Pop on 16 July 1553 Julian:
/// the latest attestation of the Haabʼ and the Calendar Round in the sources
/// read, and the time by which the Long Count had fallen out of use.
pub const LANDA: Rd = match hc_calendars_solar::julian::to_fixed(1553, 7, 16) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Where the periods of use come from.
pub const USAGE_SOURCE: &str = "docs/systems/mesoamerican-counts.md: the earliest contemporaneous Long Count, Stela 2 at \
    Chiapa de Corzo, 7.16.3.2.13 in 36 BCE [wikipedia-long-count, wikipedia-chiapa-de-corzo]; \
    the Long Count out of use by Landa's time and his 12 Kʼan 1 Pop of 16 July 1553 \
    [martin2012], taken as the bound the sources give; the 260-day count never stopped and \
    is kept in the Guatemalan highlands today [wikipedia-tzolkin, wikipedia-maya-calendar]";

/// The Maya long count, under a stated correlation with the Julian day.
///
/// The long count is an unbroken count of days, so converting it to any other
/// calendar needs exactly one number: which Julian day `0.0.0.0.0` fell on.
/// Two values are in published use and they differ by two days, which is
/// enough to move any Maya date across a weekday boundary.
///
/// Rather than take a parameter that a caller can forget, each published
/// value is registered as its own calendar, as [`docs/policy.md`](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/policy.md)
/// §5 requires: `maya-longcount` uses the GMT 584 283 correlation,
/// `maya-longcount-gmt2` uses 584 285 and `maya-longcount-584286` uses
/// Martin and Skidmore's 584 286. Asking for all three and comparing them is
/// then a short loop rather than a question the caller has to know to ask.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MayaLongCountCalendar {
    correlation: i64,
}

impl Default for MayaLongCountCalendar {
    fn default() -> Self {
        Self::GMT
    }
}

impl MayaLongCountCalendar {
    /// The Goodman–Martínez–Thompson correlation, 584 283.
    ///
    /// The mainstream choice. Kennett et al.'s radiocarbon dating of a
    /// Tikal lintel (*Scientific Reports* 3, 1597, 2013; `kennett2013`)
    /// "strongly supports" the GMT correlation by that name; its abstract,
    /// which is all that was read, does not state the constant, and
    /// 584 283 is the value the literature calls GMT (`martin2012`,
    /// `wikipedia-long-count`).
    pub const GMT: Self = Self {
        correlation: GMT_CORRELATION,
    };

    /// The "Lounsbury" or GMT+2 correlation, 584 285.
    pub const GMT_PLUS_TWO: Self = Self {
        correlation: GMT_PLUS_TWO_CORRELATION,
    };

    /// Martin and Skidmore's correlation, 584 286 (`martin2012`).
    pub const MARTIN_SKIDMORE: Self = Self {
        correlation: MARTIN_SKIDMORE_CORRELATION,
    };

    /// The three calendars, one per published constant.
    pub const ALL: [Self; 3] = [Self::GMT, Self::GMT_PLUS_TWO, Self::MARTIN_SKIDMORE];

    /// The correlation constant this calendar uses.
    #[must_use]
    pub const fn correlation(self) -> i64 {
        self.correlation
    }

    /// The fixed day of `0.0.0.0.0` under this correlation.
    #[must_use]
    pub const fn epoch(self) -> Rd {
        Rd(self.correlation - hc_calendar::fixed::JDN_OF_RD_ZERO)
    }

    /// This calendar's identifier.
    #[must_use]
    pub const fn id(self) -> CalendarId {
        match self.correlation {
            GMT_PLUS_TWO_CORRELATION => CalendarId("maya-longcount-gmt2"),
            MARTIN_SKIDMORE_CORRELATION => CalendarId("maya-longcount-584286"),
            _ => CalendarId("maya-longcount"),
        }
    }
}

/// The cycles are anchored the same way as the long count: a correlation
/// constant places `0.0.0.0.0`, which is 4 Ahau 8 Cumku under any of them,
/// on a fixed day, and the tzolk'in and haab epochs are 159 and 348 days
/// before it (`reingold2018code`, `mayan-tzolkin-epoch`, `mayan-haab-epoch`).
/// Each of the three cycle calendars therefore carries the constant too,
/// as `GMT`, `GMT_PLUS_TWO` and `MARTIN_SKIDMORE`, so that a day read
/// through `maya-round-gmt2` beside `maya-longcount-gmt2` gives the Calendar
/// Round the inscription pairs with that long count. The constant is only
/// ever one of the three: there is no constructor for any other. This macro
/// writes the shared part.
macro_rules! correlated {
    ($calendar:ident, $gmt:literal, $plus_two:literal, $martin_skidmore:literal) => {
        impl Default for $calendar {
            fn default() -> Self {
                Self::GMT
            }
        }

        impl $calendar {
            /// Under the Goodman–Martínez–Thompson correlation, 584 283.
            pub const GMT: Self = Self {
                correlation: GMT_CORRELATION,
            };

            /// Under the GMT+2 correlation, 584 285.
            pub const GMT_PLUS_TWO: Self = Self {
                correlation: GMT_PLUS_TWO_CORRELATION,
            };

            /// Under Martin and Skidmore's correlation, 584 286.
            pub const MARTIN_SKIDMORE: Self = Self {
                correlation: MARTIN_SKIDMORE_CORRELATION,
            };

            /// The three calendars, one per published constant.
            pub const ALL: [Self; 3] = [Self::GMT, Self::GMT_PLUS_TWO, Self::MARTIN_SKIDMORE];

            /// The same cycle under the constant a long count uses.
            #[must_use]
            pub const fn beside(long_count: crate::maya::MayaLongCountCalendar) -> Self {
                Self {
                    correlation: long_count.correlation(),
                }
            }

            /// The correlation constant this calendar uses.
            #[must_use]
            pub const fn correlation(self) -> i64 {
                self.correlation
            }

            /// The fixed day of `0.0.0.0.0` under this correlation.
            #[must_use]
            pub const fn epoch(self) -> Rd {
                Rd(self.correlation - hc_calendar::fixed::JDN_OF_RD_ZERO)
            }

            /// This calendar's identifier.
            #[must_use]
            pub const fn id(self) -> CalendarId {
                match self.correlation {
                    GMT_PLUS_TWO_CORRELATION => CalendarId($plus_two),
                    MARTIN_SKIDMORE_CORRELATION => CalendarId($martin_skidmore),
                    _ => CalendarId($gmt),
                }
            }
        }
    };
}

pub(crate) use correlated;

/// The 260-day tzolk'in, under a correlation constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MayaTzolkinCalendar {
    correlation: i64,
}

correlated!(
    MayaTzolkinCalendar,
    "maya-tzolkin",
    "maya-tzolkin-gmt2",
    "maya-tzolkin-584286"
);

/// The 365-day haab, under a correlation constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MayaHaabCalendar {
    correlation: i64,
}

correlated!(
    MayaHaabCalendar,
    "maya-haab",
    "maya-haab-gmt2",
    "maya-haab-584286"
);

/// The 18 980-day Calendar Round, under a correlation constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MayaCalendarRoundCalendar {
    correlation: i64,
}

correlated!(
    MayaCalendarRoundCalendar,
    "maya-round",
    "maya-round-gmt2",
    "maya-round-584286"
);

/// The last day [`MayaLongCountCalendar`] represents, `19.19.19.17.19`.
pub const LONG_COUNT_LATEST: Rd = Rd(EPOCH.0 + 20 * 144_000 - 1);

/// The tzolkʼin's two cycles: thirteen numbers against the twenty named
/// day-signs.
const TZOLKIN_SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::fixed("trecena", 13),
    hc_calendar::shape::CycleShape::named("day-sign", &TZOLKIN_NAMES),
];

/// The haabʼ's nineteen named months, Uayeb among them.
const HAAB_SHAPE: &[hc_calendar::shape::CycleShape] = &[hc_calendar::shape::CycleShape::named(
    hc_calendar::shape::MONTH,
    &HAAB_MONTHS,
)];

/// The calendar round: every cycle of the tzolkʼin and the haabʼ together.
const ROUND_SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::fixed("trecena", 13),
    hc_calendar::shape::CycleShape::named("day-sign", &TZOLKIN_NAMES),
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &HAAB_MONTHS),
];

impl Calendar for MayaLongCountCalendar {
    type Date = MayaLongCountDate;

    /// Attested from Stela 2 at Chiapa de Corzo, 36 BCE, and out of use by
    /// Landa's time; his date of 1553 is the bound the sources give, not the
    /// last inscription, which is centuries earlier.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(
            Rd(self.epoch().0 + EARLIEST_ATTESTED_DAYS),
            LANDA,
            USAGE_SOURCE,
        )
    }

    /// A long count is a place-value number; none of its places is a named
    /// position.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        &[]
    }

    /// A long count has no year: the `year` field carries the baktun.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id(),
            english_name: "Maya long count",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(self.epoch()),
            latest: Some(Rd(self.epoch().0 + 20 * 144_000 - 1)),
            native_locales: &["yua"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        Ok(Rd(self.epoch().0 + date.days()?))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        Ok(MayaLongCountDate::from_days(rd.0 - self.epoch().0))
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("baktun", date.baktun)?;
        extra.set("katun", date.katun.into())?;
        extra.set("tun", date.tun.into())?;
        extra.set("uinal", date.uinal.into())?;
        extra.set("kin", date.kin.into())?;
        Ok(DateFields {
            era: None,
            // The coarsest place, mirrored into `year` so that generic code
            // reading only that field still gets something meaningful.
            year: date.baktun,
            month: None,
            day: None,
            leap_day: false,
            extra,
        })
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        Ok(MayaLongCountDate {
            baktun: fields.extra.require("baktun")?,
            katun: small(fields.extra.require("katun")?)?,
            tun: small(fields.extra.require("tun")?)?,
            uinal: small(fields.extra.require("uinal")?)?,
            kin: small(fields.extra.require("kin")?)?,
        })
    }
}

/// Narrow an extra field to the byte-sized place it names.
fn small(value: i64) -> CalendarResult<u8> {
    u8::try_from(value).map_err(|_| CalendarError::DayOutOfRange)
}

impl Calendar for MayaTzolkinCalendar {
    type Date = MayaTzolkinDate;

    /// Attested from Stela 2 at Chiapa de Corzo, 36 BCE — the count is older,
    /// by the sources at least fifth-century BCE, but not to a day — and never
    /// stopped: it is kept in the Guatemalan highlands today.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(Rd(self.epoch().0 + EARLIEST_ATTESTED_DAYS), USAGE_SOURCE)
    }

    /// The thirteen numbers and the twenty day-signs.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        TZOLKIN_SHAPE
    }

    /// A cycle has no year: the `year` field carries the round.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id(),
            english_name: "Maya tzolk'in",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
            native_locales: &["yua"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        Ok(Rd(tzolkin_epoch_under(self.epoch())
            + date.round * TZOLKIN_CYCLE
            + date.position.ordinal()?))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let count = rd.0 - tzolkin_epoch_under(self.epoch());
        Ok(MayaTzolkinDate {
            round: count.div_euclid(TZOLKIN_CYCLE),
            position: TzolkinPosition::from_ordinal(count),
        })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("tzolkin_number", date.position.number.into())?;
        extra.set("tzolkin_name", date.position.name.into())?;
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
        Ok(MayaTzolkinDate {
            round: fields.year,
            position: TzolkinPosition::new(
                small(fields.extra.require("tzolkin_number")?)?,
                small(fields.extra.require("tzolkin_name")?)?,
            ),
        })
    }
}

impl Calendar for MayaHaabCalendar {
    type Date = MayaHaabDate;

    /// Attested from Stela 2 at Chiapa de Corzo, 36 BCE, to Landa's New Year
    /// of 1553, the last attestation in the sources read.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(
            Rd(self.epoch().0 + EARLIEST_ATTESTED_DAYS),
            LANDA,
            USAGE_SOURCE,
        )
    }

    /// Eighteen months of twenty days and the five-day Uayeb, which has a
    /// name and so is a nineteenth position.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        HAAB_SHAPE
    }

    /// A cycle has no year: the `year` field carries the round.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id(),
            english_name: "Maya haab",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: None,
            latest: None,
            native_locales: &["yua"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        Ok(Rd(haab_epoch_under(self.epoch())
            + date.round * HAAB_CYCLE
            + date.position.ordinal()?))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let count = rd.0 - haab_epoch_under(self.epoch());
        Ok(MayaHaabDate {
            round: count.div_euclid(HAAB_CYCLE),
            position: HaabPosition::from_ordinal(count),
        })
    }

    /// Describes the date with the **1-based** day that
    /// [`DateFields::day`] is contracted to hold.
    ///
    /// The haab numbers its days from 0, so the generic field is one more
    /// than [`HaabPosition::day`]: 0 Pop appears here as day 1.
    /// [`MayaHaabDate`] keeps the conventional numbering.
    ///
    /// # Errors
    ///
    /// Never fails in practice; the signature follows the trait.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields {
            era: None,
            year: date.round,
            month: Some(Month::regular(date.position.month)),
            day: Some(date.position.day.saturating_add(1)),
            leap_day: false,
            extra: ExtraFields::new(),
        })
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        if day == 0 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(MayaHaabDate {
            round: fields.year,
            position: HaabPosition::new(month.ordinal, day - 1),
        })
    }
}

/// The ordinal within a Calendar Round of a tzolk'in and haab pair, counted
/// from 4 Ahau 8 Cumku.
///
/// The ordinal does not depend on the correlation: `0.0.0.0.0` is 4 Ahau
/// 8 Cumku under every constant, and the count from it to a pairing is the
/// same however the epoch is placed in Western days.
///
/// Only one combination in five occurs: the tzolk'in advances 365 mod 260 =
/// 105 places per haab year, and `gcd(105, 260) = 5`, so a pairing whose
/// day counts differ by anything but a multiple of five never happens. That
/// is why a Maya "year bearer" can only be one of four day-names.
///
/// # Errors
///
/// Returns [`CalendarError::DayOutOfRange`] for an impossible combination,
/// or for an out-of-range position.
pub fn calendar_round_ordinal(tzolkin: TzolkinPosition, haab: HaabPosition) -> CalendarResult<i64> {
    let tzolkin_count = TZOLKIN_EPOCH + tzolkin.ordinal()?;
    let haab_count = HAAB_EPOCH + haab.ordinal()?;
    let difference = tzolkin_count - haab_count;
    if difference.rem_euclid(5) != 0 {
        return Err(CalendarError::DayOutOfRange);
    }
    // Stepping whole haab years keeps the haab position fixed; 365 steps of
    // them move the tzolk'in by exactly `difference` places modulo 260 when
    // five divides it, which is the congruence solved in one line.
    let day = haab_count + HAAB_CYCLE * difference;
    Ok((day - EPOCH.0).rem_euclid(CALENDAR_ROUND_CYCLE))
}

/// The last day [`MayaCalendarRoundCalendar::GMT`] represents; the GMT+2
/// calendar's is two days later.
///
/// The Calendar Round is bounded here only so that the round number stays
/// meaningful next to the long count it usually accompanies.
pub const CALENDAR_ROUND_LATEST: Rd = Rd(EPOCH.0 + 20 * 144_000 - 1);

impl Calendar for MayaCalendarRoundCalendar {
    type Date = MayaCalendarRoundDate;

    /// Attested from Stela 2 at Chiapa de Corzo, 36 BCE, to Landa's 12 Kʼan
    /// 1 Pop of 1553, itself a Calendar Round date and the last attestation in
    /// the sources read.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(
            Rd(self.epoch().0 + EARLIEST_ATTESTED_DAYS),
            LANDA,
            USAGE_SOURCE,
        )
    }

    /// Both cycles of the round: the tzolkʼin's two and the haabʼ's one.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        ROUND_SHAPE
    }

    /// A cycle has no year: the `year` field carries the round.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id(),
            english_name: "Maya Calendar Round",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(self.epoch()),
            latest: Some(Rd(self.epoch().0 + 20 * 144_000 - 1)),
            native_locales: &["yua"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        let ordinal = calendar_round_ordinal(date.tzolkin, date.haab)?;
        let rd = Rd(self.epoch().0 + date.round * CALENDAR_ROUND_CYCLE + ordinal);
        self.meta().check_range(rd)?;
        Ok(rd)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        Ok(MayaCalendarRoundDate {
            round: (rd.0 - self.epoch().0).div_euclid(CALENDAR_ROUND_CYCLE),
            tzolkin: TzolkinPosition::from_ordinal(rd.0 - tzolkin_epoch_under(self.epoch())),
            haab: HaabPosition::from_ordinal(rd.0 - haab_epoch_under(self.epoch())),
        })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut extra = ExtraFields::new();
        extra.set("tzolkin_number", date.tzolkin.number.into())?;
        extra.set("tzolkin_name", date.tzolkin.name.into())?;
        Ok(DateFields {
            era: None,
            year: date.round,
            month: Some(Month::regular(date.haab.month)),
            day: Some(date.haab.day.saturating_add(1)),
            leap_day: false,
            extra,
        })
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        let day = fields.require_day()?;
        if day == 0 {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(MayaCalendarRoundDate {
            round: fields.year,
            tzolkin: TzolkinPosition::new(
                small(fields.extra.require("tzolkin_number")?)?,
                small(fields.extra.require("tzolkin_name")?)?,
            ),
            haab: HaabPosition::new(month.ordinal, day - 1),
        })
    }
}

#[cfg(test)]
mod correlation_tests {
    use super::*;

    #[test]
    fn the_two_correlations_are_separate_calendars() {
        assert_eq!(
            MayaLongCountCalendar::GMT.id(),
            CalendarId("maya-longcount")
        );
        assert_eq!(
            MayaLongCountCalendar::GMT_PLUS_TWO.id(),
            CalendarId("maya-longcount-gmt2")
        );
        assert_eq!(MayaLongCountCalendar::default(), MayaLongCountCalendar::GMT);
    }

    #[test]
    fn the_two_correlations_differ_by_exactly_two_days() {
        let gmt = MayaLongCountCalendar::GMT;
        let plus_two = MayaLongCountCalendar::GMT_PLUS_TWO;
        assert_eq!(plus_two.epoch().0 - gmt.epoch().0, 2);

        // The same long count lands two days apart, which is the whole
        // reason both are registered: a caller who silently got one of them
        // would be two days out with no indication.
        let date = MayaLongCountDate::from_days(13 * 144_000);
        let under_gmt = gmt.to_fixed(date).unwrap();
        let under_plus_two = plus_two.to_fixed(date).unwrap();
        assert_eq!(under_plus_two.0 - under_gmt.0, 2);
    }

    #[test]
    fn the_thirteenth_baktun_lands_where_each_correlation_says() {
        // 13.0.0.0.0 is 2012-12-21 under GMT, and 2012-12-23 under GMT+2.
        let thirteen = MayaLongCountDate::from_days(13 * 144_000);
        let gmt_day = MayaLongCountCalendar::GMT.to_fixed(thirteen).unwrap();
        assert_eq!(
            hc_calendars_solar::gregorian::from_fixed(gmt_day),
            Ok((2012, 12, 21))
        );
        let other_day = MayaLongCountCalendar::GMT_PLUS_TWO
            .to_fixed(thirteen)
            .unwrap();
        assert_eq!(
            hc_calendars_solar::gregorian::from_fixed(other_day),
            Ok((2012, 12, 23))
        );
    }

    #[test]
    fn both_correlations_round_trip_over_their_whole_range() {
        for calendar in [
            MayaLongCountCalendar::GMT,
            MayaLongCountCalendar::GMT_PLUS_TWO,
        ] {
            for rd in (calendar.epoch().0..calendar.epoch().0 + 1_000_000).step_by(7_919) {
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            }
        }
    }

    #[test]
    fn the_cycles_under_each_correlation_are_separate_calendars() {
        assert_eq!(MayaTzolkinCalendar::GMT.id(), CalendarId("maya-tzolkin"));
        assert_eq!(
            MayaTzolkinCalendar::GMT_PLUS_TWO.id(),
            CalendarId("maya-tzolkin-gmt2")
        );
        assert_eq!(MayaHaabCalendar::GMT.id(), CalendarId("maya-haab"));
        assert_eq!(
            MayaHaabCalendar::GMT_PLUS_TWO.id(),
            CalendarId("maya-haab-gmt2")
        );
        assert_eq!(
            MayaCalendarRoundCalendar::GMT.id(),
            CalendarId("maya-round")
        );
        assert_eq!(
            MayaCalendarRoundCalendar::GMT_PLUS_TWO.id(),
            CalendarId("maya-round-gmt2")
        );
        assert_eq!(MayaTzolkinCalendar::default(), MayaTzolkinCalendar::GMT);
        assert_eq!(MayaHaabCalendar::default(), MayaHaabCalendar::GMT);
        assert_eq!(
            MayaCalendarRoundCalendar::default(),
            MayaCalendarRoundCalendar::GMT
        );
        assert_eq!(
            MayaCalendarRoundCalendar::MARTIN_SKIDMORE.id(),
            CalendarId("maya-round-584286")
        );
        assert_eq!(
            MayaTzolkinCalendar::MARTIN_SKIDMORE.id(),
            CalendarId("maya-tzolkin-584286")
        );
        assert_eq!(
            MayaHaabCalendar::MARTIN_SKIDMORE.id(),
            CalendarId("maya-haab-584286")
        );
    }

    #[test]
    fn the_cycles_follow_their_long_count_under_each_correlation() {
        // Chiapa de Corzo Stela 2, 7.16.3.2.13, is 6 Ben 16 Xul under any
        // constant (docs/systems/mesoamerican-counts.md, the worked
        // example): a Calendar Round is a count from 4 Ahau 8 Cumku. Read
        // through the cycles anchored to the *other* constant, the same
        // fixed day comes out two positions on.
        let stela = MayaLongCountDate::new(7, 16, 3, 2, 13);
        for (long_count, round, other) in [
            (
                MayaLongCountCalendar::GMT,
                MayaCalendarRoundCalendar::GMT,
                MayaCalendarRoundCalendar::GMT_PLUS_TWO,
            ),
            (
                MayaLongCountCalendar::GMT_PLUS_TWO,
                MayaCalendarRoundCalendar::GMT_PLUS_TWO,
                MayaCalendarRoundCalendar::GMT,
            ),
        ] {
            let day = long_count.to_fixed(stela).unwrap();
            let matching = round.from_fixed(day).unwrap();
            assert_eq!(matching.to_string(), "6 Ben 16 Xul");
            assert_eq!(matching.round, 59);
            let mismatched = other.from_fixed(day).unwrap();
            assert_ne!(mismatched.to_string(), "6 Ben 16 Xul");
        }
        // 13.0.0.0.0 is 4 Ahau 3 Kankin on 23 December 2012 under 584 285.
        let thirteen = MayaLongCountCalendar::GMT_PLUS_TWO
            .to_fixed(MayaLongCountDate::from_days(13 * 144_000))
            .unwrap();
        assert_eq!(
            MayaCalendarRoundCalendar::GMT_PLUS_TWO
                .from_fixed(thirteen)
                .unwrap()
                .to_string(),
            "4 Ahau 3 Kankin"
        );
        assert_eq!(
            MayaTzolkinCalendar::GMT_PLUS_TWO
                .from_fixed(thirteen)
                .unwrap()
                .position
                .to_string(),
            "4 Ahau"
        );
        assert_eq!(
            MayaHaabCalendar::GMT_PLUS_TWO
                .from_fixed(thirteen)
                .unwrap()
                .position
                .to_string(),
            "3 Kankin"
        );
        // Every day differs by exactly two between the two anchorings.
        for rd in (-1_200_000..800_000).step_by(7_919) {
            let day = Rd(rd);
            let tzolkin = MayaTzolkinCalendar::GMT.from_fixed(day).unwrap();
            assert_eq!(
                MayaTzolkinCalendar::GMT_PLUS_TWO.to_fixed(tzolkin),
                Ok(Rd(rd + 2))
            );
            let haab = MayaHaabCalendar::GMT.from_fixed(day).unwrap();
            assert_eq!(
                MayaHaabCalendar::GMT_PLUS_TWO.to_fixed(haab),
                Ok(Rd(rd + 2))
            );
        }
    }

    /// Every calendar a correlation can build names its own constant: there
    /// is no fourth constant and no calendar that reports another's id.
    #[test]
    fn each_constant_has_its_own_identifier() {
        let ids =
            MayaLongCountCalendar::ALL.map(|calendar| (calendar.correlation(), calendar.id()));
        assert_eq!(
            ids,
            [
                (584_283, CalendarId("maya-longcount")),
                (584_285, CalendarId("maya-longcount-gmt2")),
                (584_286, CalendarId("maya-longcount-584286")),
            ]
        );
        for calendar in MayaLongCountCalendar::ALL {
            assert_eq!(calendar.meta().id, calendar.id());
            assert_eq!(
                MayaTzolkinCalendar::beside(calendar).correlation(),
                calendar.correlation()
            );
            assert_eq!(
                MayaHaabCalendar::beside(calendar).correlation(),
                calendar.correlation()
            );
            assert_eq!(
                MayaCalendarRoundCalendar::beside(calendar).correlation(),
                calendar.correlation()
            );
        }
    }

    /// Martin and Skidmore: the Poco Uinic eclipse of 16 July 790 (Julian),
    /// JDN 2 009 802, is 9.17.19.13.16 5 Kib 14 Chʼen, and 2 009 802 −
    /// 1 425 516 = 584 286 (`martin2012`). Kib is this module's Cib and
    /// Chʼen its Chen.
    #[test]
    fn the_poco_uinic_eclipse_anchors_the_martin_skidmore_constant() {
        let eclipse = Rd(2_009_802 - hc_calendar::fixed::JDN_OF_RD_ZERO);
        assert_eq!(
            hc_calendars_solar::julian::from_fixed(eclipse),
            Ok((790, 7, 16))
        );
        let stela = MayaLongCountDate::new(9, 17, 19, 13, 16);
        assert_eq!(stela.days(), Ok(1_425_516));
        let calendar = MayaLongCountCalendar::MARTIN_SKIDMORE;
        assert_eq!(calendar.to_fixed(stela), Ok(eclipse));
        assert_eq!(calendar.from_fixed(eclipse), Ok(stela));
        assert_eq!(
            MayaCalendarRoundCalendar::MARTIN_SKIDMORE
                .from_fixed(eclipse)
                .expect("in range")
                .to_string(),
            "5 Cib 14 Chen"
        );
        // The same long count is one day later than under GMT+2 and three
        // later than under GMT.
        assert_eq!(
            MayaLongCountCalendar::GMT_PLUS_TWO.to_fixed(stela),
            Ok(Rd(eclipse.0 - 1))
        );
        assert_eq!(
            MayaLongCountCalendar::GMT.to_fixed(stela),
            Ok(Rd(eclipse.0 - 3))
        );
        // 13.0.0.0.0 falls on 24 December 2012 under 584 286.
        let thirteen = calendar
            .to_fixed(MayaLongCountDate::from_days(13 * 144_000))
            .expect("in range");
        assert_eq!(
            hc_calendars_solar::gregorian::from_fixed(thirteen),
            Ok((2012, 12, 24))
        );
    }
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;

    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    #[test]
    fn the_correlation_puts_the_epoch_where_the_constant_says() {
        assert_eq!(EPOCH.to_julian_day_number(), GMT_CORRELATION);
        assert_eq!(EPOCH, Rd(-1_137_142));
        assert_eq!(GMT_PLUS_TWO_CORRELATION - GMT_CORRELATION, 2);
        // 11 August 3114 BCE proleptic Gregorian, which the astronomical
        // year numbering writes -3113, and 6 September in the Julian
        // calendar the same sources often quote instead.
        assert_eq!(gregorian::from_fixed(EPOCH), Ok((-3113, 8, 11)));
        assert_eq!(
            hc_calendars_solar::julian::from_fixed(EPOCH),
            Ok((-3113, 9, 6))
        );
    }

    #[test]
    fn the_thirteenth_baktun_ended_on_the_twenty_first_of_december_2012() {
        // The most widely published Maya date there is.
        let day = greg(2012, 12, 21);
        let long_count = MayaLongCountCalendar::GMT
            .from_fixed(day)
            .expect("in range");
        assert_eq!(long_count, MayaLongCountDate::new(13, 0, 0, 0, 0));
        assert_eq!(long_count.to_string(), "13.0.0.0.0");
        assert_eq!(MayaLongCountCalendar::GMT.to_fixed(long_count), Ok(day));
    }

    #[test]
    fn the_epoch_is_four_ahau_eight_cumku() {
        // `mayan-tzolkin-epoch` and `mayan-haab-epoch` in the published code
        // place the epoch at (mayan-tzolkin-date 4 20) and (mayan-haab-date
        // 18 8): 4 Ahau 8 Cumku (`reingold2018code`).
        let tzolkin = MayaTzolkinCalendar::GMT.from_fixed(EPOCH).expect("any day");
        assert_eq!(tzolkin.position, TzolkinPosition::new(4, 20));
        assert_eq!(tzolkin.position.name_str(), Ok("Ahau"));
        assert_eq!(tzolkin.position.to_string(), "4 Ahau");

        let haab = MayaHaabCalendar::GMT.from_fixed(EPOCH).expect("any day");
        assert_eq!(haab.position, HaabPosition::new(18, 8));
        assert_eq!(haab.position.month_str(), Ok("Cumku"));
        assert_eq!(haab.position.to_string(), "8 Cumku");

        let round = MayaCalendarRoundCalendar::GMT
            .from_fixed(EPOCH)
            .expect("in range");
        assert_eq!(round.round, 0);
        assert_eq!(round.to_string(), "4 Ahau 8 Cumku");
    }

    #[test]
    fn the_thirteenth_baktun_ended_on_four_ahau_three_kankin() {
        // The Calendar Round of 13.0.0.0.0: 4 Ajaw 3 Kʼankʼin
        // (`famsi-vanstone-2012`), which the published code's ordinals give
        // from the epoch's 4 Ahau 8 Cumku (`reingold2018code`).
        let day = greg(2012, 12, 21);
        let round = MayaCalendarRoundCalendar::GMT
            .from_fixed(day)
            .expect("in range");
        assert_eq!(round.tzolkin, TzolkinPosition::new(4, 20));
        assert_eq!(round.haab, HaabPosition::new(14, 3));
        assert_eq!(round.to_string(), "4 Ahau 3 Kankin");
    }

    #[test]
    fn a_published_modern_long_count_matches() {
        // Cross-checked against azteccalendar.com, which prints the long
        // count for an arbitrary Gregorian date under the same correlation.
        let day = greg(2026, 9, 21);
        assert_eq!(
            MayaLongCountCalendar::GMT
                .from_fixed(day)
                .expect("in range")
                .to_string(),
            "13.0.13.17.2"
        );
    }

    #[test]
    fn the_long_count_places_have_the_radices_they_should() {
        assert_eq!(MayaLongCountDate::new(0, 0, 0, 0, 1).days(), Ok(1));
        assert_eq!(MayaLongCountDate::new(0, 0, 0, 1, 0).days(), Ok(20));
        assert_eq!(MayaLongCountDate::new(0, 0, 1, 0, 0).days(), Ok(360));
        assert_eq!(MayaLongCountDate::new(0, 1, 0, 0, 0).days(), Ok(7_200));
        assert_eq!(MayaLongCountDate::new(1, 0, 0, 0, 0).days(), Ok(144_000));
        // The uinal is the irregular place: 18 to a tun, not 20.
        assert_eq!(
            MayaLongCountDate::from_days(360),
            MayaLongCountDate::new(0, 0, 1, 0, 0)
        );
        assert_eq!(
            MayaLongCountDate::from_days(359),
            MayaLongCountDate::new(0, 0, 0, 17, 19)
        );
    }

    #[test]
    fn out_of_radix_long_counts_are_refused() {
        assert_eq!(
            MayaLongCountDate::new(0, 0, 0, 18, 0).days(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            MayaLongCountDate::new(0, 0, 0, 0, 20).days(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            MayaLongCountDate::new(0, 20, 0, 0, 0).days(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            MayaLongCountDate::new(-1, 0, 0, 0, 0).days(),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            MayaLongCountDate::new(20, 0, 0, 0, 0).days(),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_long_count_covers_exactly_twenty_baktun() {
        assert_eq!(
            MayaLongCountCalendar::GMT.from_fixed(EPOCH),
            Ok(MayaLongCountDate::new(0, 0, 0, 0, 0))
        );
        assert_eq!(
            MayaLongCountCalendar::GMT.from_fixed(LONG_COUNT_LATEST),
            Ok(MayaLongCountDate::new(19, 19, 19, 17, 19))
        );
        assert_eq!(
            MayaLongCountCalendar::GMT.from_fixed(Rd(EPOCH.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            MayaLongCountCalendar::GMT.from_fixed(Rd(LONG_COUNT_LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn the_tzolkin_repeats_every_two_hundred_and_sixty_days() {
        let start = greg(2026, 9, 21);
        let first = MayaTzolkinCalendar::GMT.from_fixed(start).expect("any day");
        let later = MayaTzolkinCalendar::GMT
            .from_fixed(Rd(start.0 + 260))
            .expect("any day");
        assert_eq!(first.position, later.position);
        assert_eq!(later.round, first.round + 1);
        // Every one of the 260 positions occurs exactly once per cycle.
        let mut seen = [false; 260];
        for offset in 0..260 {
            let ordinal = MayaTzolkinCalendar::GMT
                .from_fixed(Rd(start.0 + offset))
                .expect("any day")
                .position
                .ordinal()
                .expect("valid position");
            assert!(!seen[ordinal as usize], "repeat at {offset}");
            seen[ordinal as usize] = true;
        }
        assert!(seen.iter().all(|hit| *hit));
    }

    #[test]
    fn the_haab_has_eighteen_months_of_twenty_and_a_five_day_uayeb() {
        assert_eq!(HAAB_MONTHS.len(), 19);
        assert_eq!(HAAB_MONTHS[18], "Uayeb");
        assert_eq!(HaabPosition::new(1, 0).ordinal(), Ok(0));
        assert_eq!(HaabPosition::new(19, 4).ordinal(), Ok(364));
        assert_eq!(
            HaabPosition::new(19, 5).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            HaabPosition::new(18, 20).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            HaabPosition::new(20, 0).ordinal(),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            HaabPosition::new(0, 0).month_str(),
            Err(CalendarError::MonthOutOfRange)
        );
        // 18 * 20 + 5 = 365, and every ordinal maps back.
        for ordinal in 0..365 {
            let position = HaabPosition::from_ordinal(ordinal);
            assert_eq!(position.ordinal(), Ok(ordinal));
        }
    }

    #[test]
    fn only_one_tzolkin_haab_pairing_in_five_can_occur() {
        let mut possible = 0;
        for number in 1..=13u8 {
            for name in 1..=20u8 {
                for month in 1..=19u8 {
                    let limit = if month == 19 { 4 } else { 19 };
                    for day in 0..=limit {
                        let ordinal = calendar_round_ordinal(
                            TzolkinPosition::new(number, name),
                            HaabPosition::new(month, day),
                        );
                        if ordinal.is_ok() {
                            possible += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(possible, CALENDAR_ROUND_CYCLE);
        assert_eq!(possible * 5, 260 * 365);
    }

    #[test]
    fn an_impossible_calendar_round_pairing_is_refused() {
        // 4 Ahau 8 Cumku happens; 4 Ahau 9 Cumku cannot.
        assert!(
            calendar_round_ordinal(TzolkinPosition::new(4, 20), HaabPosition::new(18, 8)).is_ok()
        );
        assert_eq!(
            calendar_round_ordinal(TzolkinPosition::new(4, 20), HaabPosition::new(18, 9)),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn the_calendar_round_closes_after_fifty_two_vague_years() {
        assert_eq!(CALENDAR_ROUND_CYCLE, 52 * HAAB_CYCLE);
        assert_eq!(CALENDAR_ROUND_CYCLE, 73 * TZOLKIN_CYCLE);
        let start = greg(2026, 9, 21);
        let first = MayaCalendarRoundCalendar::GMT
            .from_fixed(start)
            .expect("in range");
        let later = MayaCalendarRoundCalendar::GMT
            .from_fixed(Rd(start.0 + CALENDAR_ROUND_CYCLE))
            .expect("in range");
        assert_eq!(first.tzolkin, later.tzolkin);
        assert_eq!(first.haab, later.haab);
        assert_eq!(later.round, first.round + 1);
    }

    #[test]
    fn every_day_of_a_whole_calendar_round_round_trips() {
        // The full 18 980 days, because the cycles only disagree at the
        // places where the two wheels slip past each other.
        let start = EPOCH;
        for offset in 0..CALENDAR_ROUND_CYCLE {
            let rd = Rd(start.0 + offset);
            let date = MayaCalendarRoundCalendar::GMT
                .from_fixed(rd)
                .expect("in range");
            assert_eq!(
                MayaCalendarRoundCalendar::GMT.to_fixed(date),
                Ok(rd),
                "{offset}"
            );
            let tzolkin = MayaTzolkinCalendar::GMT.from_fixed(rd).expect("any day");
            assert_eq!(
                MayaTzolkinCalendar::GMT.to_fixed(tzolkin),
                Ok(rd),
                "{offset}"
            );
            let haab = MayaHaabCalendar::GMT.from_fixed(rd).expect("any day");
            assert_eq!(MayaHaabCalendar::GMT.to_fixed(haab), Ok(rd), "{offset}");
        }
    }

    #[test]
    fn fields_carry_the_five_long_count_places() {
        let date = MayaLongCountDate::new(13, 0, 13, 17, 2);
        let fields = MayaLongCountCalendar::GMT
            .to_fields(date)
            .expect("describable");
        assert_eq!(fields.extra.get("baktun"), Some(13));
        assert_eq!(fields.extra.get("katun"), Some(0));
        assert_eq!(fields.extra.get("tun"), Some(13));
        assert_eq!(fields.extra.get("uinal"), Some(17));
        assert_eq!(fields.extra.get("kin"), Some(2));
        assert_eq!(fields.extra.len(), 5);
        assert_eq!(fields.year, 13);
        assert_eq!(MayaLongCountCalendar::GMT.from_fields(&fields), Ok(date));
        let empty = DateFields::new(0);
        assert_eq!(
            MayaLongCountCalendar::GMT.from_fields(&empty),
            Err(CalendarError::MissingField("baktun"))
        );
    }

    #[test]
    fn the_haab_day_field_is_one_based_although_the_haab_is_not() {
        let date = MayaHaabCalendar::GMT.from_fixed(EPOCH).expect("any day");
        assert_eq!(date.position.day, 8);
        let fields = MayaHaabCalendar::GMT.to_fields(date).expect("describable");
        assert_eq!(fields.day, Some(9));
        assert_eq!(fields.month, Some(Month::regular(18)));
        assert_eq!(MayaHaabCalendar::GMT.from_fields(&fields), Ok(date));
        let mut broken = fields;
        broken.day = Some(0);
        assert_eq!(
            MayaHaabCalendar::GMT.from_fields(&broken),
            Err(CalendarError::DayOutOfRange)
        );
        let mut leap = fields;
        leap.month = Some(Month::leap(18));
        assert_eq!(
            MayaHaabCalendar::GMT.from_fields(&leap),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn tzolkin_positions_reject_impossible_numbers_and_names() {
        assert_eq!(
            TzolkinPosition::new(14, 1).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            TzolkinPosition::new(0, 1).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            TzolkinPosition::new(1, 21).ordinal(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            TzolkinPosition::new(1, 0).name_str(),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(TzolkinPosition::new(1, 1).name_str(), Ok("Imix"));
    }

    #[test]
    fn the_cycles_run_backwards_through_the_epoch_as_well() {
        // Days before 0.0.0.0.0 still have a tzolk'in and a haab; only the
        // long count refuses them. The positions follow from the epoch's
        // 4 Ahau 8 Cumku (`reingold2018code`) by one step back.
        let before = Rd(EPOCH.0 - 1);
        let tzolkin = MayaTzolkinCalendar::GMT
            .from_fixed(before)
            .expect("any day");
        assert_eq!(tzolkin.position, TzolkinPosition::new(3, 19));
        // The tzolk'in cycle containing the epoch began 159 days earlier,
        // so the day before the epoch is still inside round 0.
        assert_eq!(tzolkin.round, 0);
        assert_eq!(
            MayaTzolkinCalendar::GMT
                .from_fixed(Rd(EPOCH.0 - 160))
                .expect("any day")
                .round,
            -1
        );
        assert_eq!(MayaTzolkinCalendar::GMT.to_fixed(tzolkin), Ok(before));
        let haab = MayaHaabCalendar::GMT.from_fixed(before).expect("any day");
        assert_eq!(haab.position, HaabPosition::new(18, 7));
        assert_eq!(MayaHaabCalendar::GMT.to_fixed(haab), Ok(before));
    }
}
