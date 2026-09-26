//! The Tibetan calendar and its versions: the Phugpa, the Tsurphu, the
//! Bhutanese, and the Mongolian New Genden.
//!
//! The systems are written up in the repository: `docs/systems/tibetan-phugpa.md`
//! for the arithmetic every version shares — the lunar day and the rule that
//! skips and repeats calendar days, the leap-month rule, the mean motions as
//! the exact rationals they are, Losar 2024 and a skipped and an extra day
//! worked by hand — and `docs/systems/tibetan-variants.md` for what the other
//! three versions change, with their epochs, their leap months and how each
//! was checked against Janson's tables and the published dates. This page
//! summarises them and states the code's own facts.
//!
//! A lunisolar calendar of twelve or thirteen months, each of thirty lunar
//! days, computed entirely by arithmetic: a mean motion for the date, the
//! sun and the moon's anomaly, two small tables in place of a sine, and a
//! leap-month rule on a 65-month cycle. A calendar day is named by the
//! lunar day current at its dawn, so a lunar day that ends within one
//! calendar day gives its number to none — the number is *skipped* — and
//! one that contains a whole calendar day gives it to two, the first of
//! which is the *extra* (leap) day. The year is numbered by the Western
//! year it begins in and named in the sixty-year cycle: 2007 is the
//! Fire–Pig year.
//!
//! # One engine, four versions
//!
//! Every version keeps the same mean motions, tables and day rule, and
//! differs in data only (Janson, Appendix A; Gantumur, "Parameters of the
//! Principal Traditions"): the epoch year, the intercalation index at the
//! epoch, the index at which a leap month is inserted, whether a leap month
//! takes the number of the month after it or before it, and the three epoch
//! values of the mean date, the mean sun and the moon's anomaly. A [`TibetanCalendar`] is
//! those values, and the four registered calendars are four of them:
//!
//! * [`TIBETAN`], `tibetan`: the Phugpa, from the epoch of 806.
//! * [`TIBETAN_TSURPHU`], `tibetan-tsurphu`: the Karma Kagyu's Tsurphu,
//!   from Jamgön Kongtrul's epoch of 1852.
//! * [`TIBETAN_BHUTAN`], `tibetan-bhutan`: Lhawang Lodrö's Bhutanese, from
//!   1754, whose leap month takes the number of the month before it.
//! * [`MONGOLIAN`], `mongolian`: Sumpa Khenpo Yeshe Paljor's New Genden,
//!   the *Tögs buyant* of Mongolia, from 1747.
//!
//! # Whose arithmetic
//!
//! Svante Janson's *Tibetan Calendar Mathematics* (2014), in modern
//! notation with exact rational constants: the true month count is his
//! (5.10) with the rounding each version's leap rule implies, the leap
//! rules his (5.8) and those of his Appendices A.2 to A.4, the inverse his
//! (5.19)–(5.22),
//! the mean date, mean sun and moon's anomaly his (7.1), (7.5) and (7.11)
//! with the almanacs' `a2 = 1/28`, the tables his (7.18) and (7.21), the
//! true date his (7.22), and the calendar day his (8.1) with the rule of
//! his Section 6. The epoch values are his Appendix A's, which are Edward
//! Henning's epoch data of the source texts; a test holds each to Henning's
//! own mixed-radix digits.
//!
//! # What is not carried
//!
//! The *Kālacakra* *karaṇa* calculation, the Sherab Ling and Sarnath
//! reforms, and the Inner Mongolian "yellow" calculation (his Appendix A);
//! the *karaṇa* solar equation some Tsurphu almanacs have used in the true
//! date; Henning's exact anomaly increment (7.24), which moves about one day
//! in four thousand; the Bhutanese weekday, one ahead of the world's; and
//! the almanac's further columns of his Section 10.
//!
//! # Sources
//!
//! Keyed as in `docs/references.bib`.
//!
//! * `janson2014`: Svante Janson, "Tibetan calendar mathematics",
//!   arXiv:1401.6285, revised 8 January 2014, read from its TeX source
//!   2026-09-22 and 2026-09-26.
//! * `kalacakra-org`: Edward Henning, *Kālacakra Calendar*, the pages
//!   "Epoch data", "Open source Tsurphu calendar software" and "Bhutan
//!   calendars", www.kalacakra.org, read 2026-09-26.
//! * `gantumur2026`: Tsogtgerel Gantumur, "Possible reforms of the Tibetan
//!   lunisolar calendar", arXiv:2604.01233v2, 2026, read from its TeX
//!   source 2026-09-26.

use core::fmt;

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, CivilTime, DateFields,
    DayBoundary, DayNaming, Month, Rd, Usage, YearKind,
};

/// Mean daybreak, 05:00 local mean solar time, when the Tibetan calendar day
/// begins.
///
/// The calendar day runs "from dawn to dawn" and is a constant 24 hours, so
/// no sunrise is computed; Janson's Remark 6 gives Henning's mean daybreak,
/// 5 a.m. local mean solar time, as the start (Janson, "Tibetan calendar
/// mathematics", Section 2 and Remark 6; Edward Henning, *Kālacakra and the
/// Tibetan Calendar*, 2007, pp. 10–11). No source read gives the Bhutanese
/// or the Mongolian version another.
pub const DAWN: CivilTime = match CivilTime::hms(5, 0, 0) {
    Ok(time) => time,
    Err(_) => panic!("05:00 is a time of day"),
};

/// The epoch month of every version, the third, *nag pa*: by tradition the
/// count of solar months starts there (Janson, Section 5).
pub const EPOCH_MONTH: i64 = 3;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1000;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 3000;

/// A rational number, kept reduced, for the calendar's exact arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ratio {
    num: i128,
    den: i128,
}

const fn gcd(mut a: i128, mut b: i128) -> i128 {
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }
    while b != 0 {
        let rest = a % b;
        a = b;
        b = rest;
    }
    a
}

impl Ratio {
    const fn new(num: i128, den: i128) -> Self {
        let g = gcd(num, den);
        let (num, den) = if g == 0 {
            (num, den)
        } else {
            (num / g, den / g)
        };
        if den < 0 {
            Self {
                num: -num,
                den: -den,
            }
        } else {
            Self { num, den }
        }
    }

    const fn int(value: i128) -> Self {
        Self { num: value, den: 1 }
    }

    const fn add(self, other: Self) -> Self {
        Self::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }

    const fn sub(self, other: Self) -> Self {
        Self::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        )
    }

    const fn mul(self, other: Self) -> Self {
        Self::new(self.num * other.num, self.den * other.den)
    }

    const fn scale(self, factor: i128) -> Self {
        Self::new(self.num * factor, self.den)
    }

    const fn floor(self) -> i128 {
        self.num.div_euclid(self.den)
    }

    /// The fractional part, in `[0, 1)`.
    const fn frac(self) -> Self {
        Self::new(self.num.rem_euclid(self.den), self.den)
    }
}

/// The mean lunar month, `m1`, in days.
const M1: Ratio = Ratio::new(167_025, 5_656);
/// The mean lunar day, `m2 = m1 / 30`.
const M2: Ratio = Ratio::new(11_135, 11_312);
/// The sun's mean motion per month, `s1`, in revolutions.
const S1: Ratio = Ratio::new(65, 804);
/// The sun's mean motion per lunar day, `s2`.
const S2: Ratio = Ratio::new(13, 4_824);
/// The moon's anomaly per month, `a1`.
const A1: Ratio = Ratio::new(253, 3_528);
/// The moon's anomaly per lunar day, `a2`, the almanacs' rounded value.
const A2: Ratio = Ratio::new(1, 28);

/// The moon's equation table, `moon_tab(i)` for `i = 0..=7`, extended by
/// symmetry to a period of 28.
const MOON_TABLE: [i128; 8] = [0, 5, 10, 15, 19, 22, 24, 25];
/// The sun's equation table, `sun_tab(i)` for `i = 0..=3`, extended by
/// symmetry to a period of 12.
const SUN_TABLE: [i128; 4] = [0, 6, 10, 11];

/// A table with the symmetries `tab(2h − i) = tab(i)` and
/// `tab(2h + i) = −tab(i)`, linearly interpolated.
fn table(values: &[i128], half: i128, x: Ratio) -> Ratio {
    let period = 4 * half;
    let mut x = Ratio::new(x.num.rem_euclid(x.den * period), x.den);
    let mut sign = 1;
    // Beyond the half period the table is negated, and within it mirrored.
    if x.num >= 2 * half * x.den {
        x = x.sub(Ratio::int(2 * half));
        sign = -1;
    }
    if x.num > half * x.den {
        x = Ratio::int(2 * half).sub(x);
    }
    let index = x.floor();
    let fraction = x.frac();
    let value = if index >= half {
        Ratio::int(values[half as usize])
    } else {
        let low = values[index as usize];
        let high = values[index as usize + 1];
        Ratio::int(low).add(fraction.scale(high - low))
    };
    value.scale(sign)
}

const JDN_OFFSET: i64 = 1_721_425;

/// Which month a leap month takes its number from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LeapNumbering {
    /// The month after it, so the leap month is the first of the two
    /// months with its number: the Phugpa, Tsurphu and Mongolian rule, and
    /// the Indian one (Janson, Section 5).
    Following,
    /// The month before it, so the leap month is the second: the Bhutanese
    /// rule, as in the Chinese calendar (Janson, Appendix A.4, after
    /// Henning's "Bhutan calendars").
    Preceding,
}

/// A version of the Tibetan calendar: the shared arithmetic under one
/// version's epoch and leap-month rule.
///
/// The versions differ only in the values held here (Janson, Appendix A);
/// the four registered are [`TIBETAN`], [`TIBETAN_TSURPHU`],
/// [`TIBETAN_BHUTAN`] and [`MONGOLIAN`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TibetanCalendar {
    id: CalendarId,
    english_name: &'static str,
    epoch_year: i64,
    epoch_index: i64,
    leap_index: i64,
    leap_numbering: LeapNumbering,
    m0: Ratio,
    s0: Ratio,
    a0: Ratio,
    first_year: Option<i64>,
    usage_source: &'static str,
    native_locales: &'static [&'static str],
}

impl Default for TibetanCalendar {
    fn default() -> Self {
        TIBETAN
    }
}

/// Where the period of use of `tibetan` comes from.
pub const USAGE_SOURCE: &str = "Janson 2014, §1 and Appendix A [janson2014]: the Phugpa tradition begun in 1447 by Phugpa \
    Lhundrub Gyatso, used by the Tibetan government from at least 1696 to 1959 and in the \
    almanacs published at Dharamsala since; the year only, so its Losar is taken";

/// Where the period of use of `tibetan-tsurphu` comes from.
pub const TSURPHU_USAGE_SOURCE: &str = "Janson 2014, Appendix A.2 [janson2014]: the Tsurphu version, also \
    introduced in 1447, by Jamyang Dondrub Wozer, and used by the Karma Kagyu in the calendars \
    published from Rumtek; Henning's \"Open source Tsurphu calendar software\" \
    [kalacakra-org]: \"the official date given is 1447\"; the year only, so its Losar is taken";

/// Where the period of use of `tibetan-bhutan` comes from.
pub const BHUTAN_USAGE_SOURCE: &str = "Janson 2014, Appendix A.4 [janson2014]: Bhutan's official calendar, \
    described by Lhawang Lodro in the 18th century \"but said to be older\", which dates no \
    beginning; Henning's \"Bhutan calendars\" [kalacakra-org] warns that its years before the \
    text's epoch of 1754 \"may not be relevant in historical references\"; the Ministry of \
    Home Affairs prints it in its calendars today [moha-bt-calendar-2026]";

/// Where the period of use of `mongolian` comes from.
pub const MONGOLIAN_USAGE_SOURCE: &str = "Janson 2014, Appendix A.3 [janson2014]: the New Genden version, \
    Mongolian Tögs buyant, created by Sumpa Khenpo Yeshe Paljor in 1786 according to Berzin \
    (cited through Janson, not read), official in Mongolia from 1911, replaced for civil use \
    in the 1920s and kept for Tsagaan Sar and the lunar holidays since; the year only, so its \
    Losar is taken";

/// The Phugpa version, from the epoch of month 3 of 806: `β* = 61`, a leap
/// month where the intercalation index is 48 or 49, and Janson's epoch
/// values (5.2), (7.2), (7.6), (7.12).
pub const TIBETAN: TibetanCalendar = TibetanCalendar {
    id: CalendarId("tibetan"),
    english_name: "Tibetan (Phugpa)",
    epoch_year: 806,
    epoch_index: 61,
    leap_index: 48,
    leap_numbering: LeapNumbering::Following,
    m0: Ratio::new(2_015_501 * 5_656 + 4_783, 5_656),
    s0: Ratio::new(743, 804),
    a0: Ratio::new(475, 3_528),
    first_year: Some(1_447),
    usage_source: USAGE_SOURCE,
    native_locales: &["bo"],
};

/// The Tsurphu version, from Jamgön Kongtrul's epoch of month 3 of 1852,
/// JD 2 397 598: `β* = 14`, a leap month where the index is 0 or 1, and
/// the epoch values of Janson's Appendix A.2 (E1852), which are the digits
/// of Henning's "Epoch data" for the *Compendium of Practical Astronomy*.
/// Kongtrul's is the epoch Henning's Tsurphu software uses; the earlier
/// one of 1732 gives the same calendar, which a test holds.
pub const TIBETAN_TSURPHU: TibetanCalendar = TibetanCalendar {
    id: CalendarId("tibetan-tsurphu"),
    english_name: "Tibetan (Tsurphu)",
    epoch_year: 1_852,
    epoch_index: 14,
    leap_index: 0,
    leap_numbering: LeapNumbering::Following,
    m0: Ratio::new(2_397_598 * 7_635_600 + 1_197_103, 7_635_600),
    s0: Ratio::new(23, 27_135),
    a0: Ratio::new(1, 49),
    first_year: Some(1_447),
    usage_source: TSURPHU_USAGE_SOURCE,
    native_locales: &["bo"],
};

/// The Bhutanese version, from Lhawang Lodrö's epoch of month 3 of 1754,
/// JD 2 361 807: `β* = 2`, a leap month numbered by the month before it
/// and inserted where the index of the month after it is 59 or 60, and the
/// epoch values of Janson's Appendix A.4, which are the digits of Henning's
/// "Epoch data".
pub const TIBETAN_BHUTAN: TibetanCalendar = TibetanCalendar {
    id: CalendarId("tibetan-bhutan"),
    english_name: "Tibetan (Bhutanese)",
    epoch_year: 1_754,
    epoch_index: 2,
    leap_index: 59,
    leap_numbering: LeapNumbering::Preceding,
    m0: Ratio::new(2_361_807 * 707 + 52, 707),
    s0: Ratio::new(1, 67),
    a0: Ratio::new(17, 147),
    first_year: None,
    usage_source: BHUTAN_USAGE_SOURCE,
    native_locales: &["dz"],
};

/// The Mongolian New Genden version, from Yeshe Paljor's epoch of month 3
/// of 1747, JD 2 359 237: `β* = 10`, a leap month where the index is 46 or
/// 47, and the epoch values of Janson's Appendix A.3, which are the digits
/// of Henning's "Epoch data" for the "New Genden Calculations".
pub const MONGOLIAN: TibetanCalendar = TibetanCalendar {
    id: CalendarId("mongolian"),
    english_name: "Mongolian (Tögs buyant)",
    epoch_year: 1_747,
    epoch_index: 10,
    leap_index: 46,
    leap_numbering: LeapNumbering::Following,
    m0: Ratio::new(2_359_237 * 2_828 + 2_603, 2_828),
    s0: Ratio::new(397, 402),
    a0: Ratio::new(1_523, 1_764),
    first_year: Some(1_786),
    usage_source: MONGOLIAN_USAGE_SOURCE,
    native_locales: &["mn"],
};

impl TibetanCalendar {
    /// The epoch year, `Y0`.
    #[must_use]
    pub const fn epoch_year(&self) -> i64 {
        self.epoch_year
    }

    /// The intercalation index at the epoch, `β*` of the source.
    #[must_use]
    pub const fn epoch_index(&self) -> i64 {
        self.epoch_index
    }

    /// The intercalation index of the regular month that follows a leap
    /// month, together with the one after it: 48 or 49 for the Phugpa
    /// (5.8), 0 or 1 for the Tsurphu, 46 or 47 for the Mongolian and 59 or
    /// 60 for the Bhutanese (Janson, Appendices A.2, A.3 and A.4).
    #[must_use]
    pub const fn leap_index(&self) -> i64 {
        self.leap_index
    }

    /// Which month a leap month takes its number from.
    #[must_use]
    pub const fn leap_numbering(&self) -> LeapNumbering {
        self.leap_numbering
    }

    /// What is added to `67 M' + β*` before the true month is divided by 65
    /// and rounded down: 17 for the Phugpa, as in (5.10), since a month
    /// whose index has reached the leap index is rounded up.
    const fn rounding(&self) -> i64 {
        (65 - self.leap_index).rem_euclid(65)
    }

    /// `β` of the inverse formulas (5.15): `184 − β*` for the Phugpa, 123
    /// from 806; 187 for the Tsurphu from 1852, 172 for the Mongolian and
    /// 191 for the Bhutanese, as Janson's Appendix A states them.
    #[must_use]
    pub const fn inverse_constant(&self) -> i64 {
        let shift = match self.leap_numbering {
            LeapNumbering::Following => 0,
            LeapNumbering::Preceding => 2,
        };
        67 * EPOCH_MONTH - self.epoch_index - self.rounding() - shift
    }

    /// The solar months from the epoch to month `month` of `year`, `M'`.
    const fn solar_months(&self, year: i64, month: u8) -> i64 {
        12 * (year - self.epoch_year) + month as i64 - EPOCH_MONTH
    }

    /// The intercalation index of a count of solar months, (5.7).
    const fn index(&self, solar_months: i64) -> i64 {
        (2 * solar_months + self.epoch_index).rem_euclid(65)
    }

    /// Whether the regular month `solar_months` from the epoch is the one
    /// a leap month comes before.
    const fn follows_a_leap_month(&self, solar_months: i64) -> bool {
        let index = self.index(solar_months);
        index == self.leap_index || index == (self.leap_index + 1) % 65
    }

    /// The true month count of the regular month `solar_months` from the
    /// epoch.
    const fn regular_count(&self, solar_months: i64) -> i64 {
        (67 * solar_months + self.epoch_index + self.rounding()).div_euclid(65)
    }

    /// The true month count of month `month` (1–12) of `year`, or `None`
    /// when a leap month is asked for in a month that has none.
    #[must_use]
    pub const fn true_month_count(&self, year: i64, month: u8, leap: bool) -> Option<i64> {
        let solar_months = self.solar_months(year, month);
        if !leap {
            return Some(self.regular_count(solar_months));
        }
        // A leap month is the month before a regular month whose index is
        // the leap index: that regular month is the one of the same number
        // when the leap month takes the following number, and the next one
        // when it takes the preceding number.
        let after = match self.leap_numbering {
            LeapNumbering::Following => solar_months,
            LeapNumbering::Preceding => solar_months + 1,
        };
        if self.follows_a_leap_month(after) {
            Some(self.regular_count(after) - 1)
        } else {
            None
        }
    }

    /// `x` of (5.19): `12 (Y − Y0) + M` of the month with count `n`.
    const fn label(&self, n: i64) -> i64 {
        (65 * n + self.inverse_constant() + 66).div_euclid(67)
    }

    /// The year, month and leap flag of true month count `n`.
    #[must_use]
    pub const fn month_of_count(&self, n: i64) -> (i64, u8, bool) {
        let x = self.label(n);
        let month = (x - 1).rem_euclid(12) + 1;
        let year = (x + 11).div_euclid(12) - 1 + self.epoch_year;
        // A leap month shares its number with its neighbour on the side it
        // takes the number from.
        let neighbour = match self.leap_numbering {
            LeapNumbering::Following => self.label(n + 1),
            LeapNumbering::Preceding => self.label(n - 1),
        };
        (year, month as u8, neighbour == x)
    }

    /// The number of the leap month of `year`, or `None`.
    #[must_use]
    pub const fn leap_month_of(&self, year: i64) -> Option<u8> {
        let mut month = 1;
        while month <= 12 {
            if self.true_month_count(year, month, true).is_some() {
                return Some(month);
            }
            month += 1;
        }
        None
    }

    /// Whether `year` has a leap month.
    #[must_use]
    pub const fn is_leap_year(&self, year: i64) -> bool {
        self.leap_month_of(year).is_some()
    }

    /// The true month count of the first month of `year`: the leap month 1
    /// when the year has one and the leap month takes the following number,
    /// the regular month 1 otherwise.
    fn first_month_count(&self, year: i64) -> i64 {
        match (self.leap_numbering, self.true_month_count(year, 1, true)) {
            (LeapNumbering::Following, Some(n)) => n,
            _ => self.regular_count(self.solar_months(year, 1)),
        }
    }

    /// The true date at the end of lunar day `day` of true month `n`, as a
    /// Julian Date whose integer part is the calendar day.
    fn true_date(&self, day: i64, n: i64) -> Ratio {
        let n = Ratio::int(n as i128);
        let day = Ratio::int(day as i128);
        let mean_date = n.mul(M1).add(day.mul(M2)).add(self.m0);
        let mean_sun = n.mul(S1).add(day.mul(S2)).add(self.s0).frac();
        let anomaly_moon = n.mul(A1).add(day.mul(A2)).add(self.a0).frac();
        let moon_equation = table(&MOON_TABLE, 7, anomaly_moon.scale(28));
        let sun_equation = table(&SUN_TABLE, 3, mean_sun.sub(Ratio::new(1, 4)).scale(12));
        mean_date
            .add(moon_equation.mul(Ratio::new(1, 60)))
            .sub(sun_equation.mul(Ratio::new(1, 60)))
    }

    /// The Julian Day Number of the calendar day in which lunar day `day`
    /// of true month `n` ends.
    fn end_day(&self, day: i64, n: i64) -> i64 {
        self.true_date(day, n).floor() as i64
    }

    /// The end day of the lunar day before `day` of month `n`: day 30 of
    /// the previous month when `day` is 1.
    fn previous_end_day(&self, day: i64, n: i64) -> i64 {
        if day == 1 {
            self.end_day(30, n - 1)
        } else {
            self.end_day(day - 1, n)
        }
    }

    /// The Julian Day Number of Losar, the first day of `year`: the day
    /// after day 30 of the last month of the year before (Janson,
    /// Section 8), whether or not that day 30 is skipped.
    #[must_use]
    pub fn new_year_jdn(&self, year: i64) -> i64 {
        self.end_day(30, self.first_month_count(year) - 1) + 1
    }

    /// The fixed day of Losar of `year`, Tsagaan Sar in the Mongolian
    /// version.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`].
    pub fn new_year(&self, year: i64) -> CalendarResult<Rd> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(Rd(self.new_year_jdn(year) - JDN_OFFSET))
    }

    /// The earliest fixed day this implementation converts, Losar of
    /// [`MIN_YEAR`].
    #[must_use]
    pub fn earliest(&self) -> Rd {
        Rd(self.new_year_jdn(MIN_YEAR) - JDN_OFFSET)
    }

    /// The latest fixed day this implementation converts, the day before
    /// Losar of the year after [`MAX_YEAR`].
    #[must_use]
    pub fn latest(&self) -> Rd {
        Rd(self.new_year_jdn(MAX_YEAR + 1) - 1 - JDN_OFFSET)
    }

    /// The fixed day of a Tibetan date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside the years
    /// converted, [`CalendarError::MonthOutOfRange`] for a month outside
    /// `1..=12` or a leap month the year does not have, and
    /// [`CalendarError::DayOutOfRange`] for a day outside `1..=30`, a day
    /// the calendar skips, or an extra day where the day is not repeated.
    pub fn date_to_fixed(&self, date: TibetanDate) -> CalendarResult<Rd> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&date.year) {
            return Err(CalendarError::YearOutOfRange);
        }
        if date.month.ordinal == 0 || date.month.ordinal > 12 {
            return Err(CalendarError::MonthOutOfRange);
        }
        if date.day == 0 || date.day > 30 {
            return Err(CalendarError::DayOutOfRange);
        }
        let n = self
            .true_month_count(date.year, date.month.ordinal, date.month.leap)
            .ok_or(CalendarError::MonthOutOfRange)?;
        let day = i64::from(date.day);
        let end = self.end_day(day, n);
        let before = self.previous_end_day(day, n);
        match end - before {
            0 => Err(CalendarError::DayOutOfRange),
            1 if date.leap_day => Err(CalendarError::DayOutOfRange),
            1 => Ok(Rd(end - JDN_OFFSET)),
            _ if date.leap_day => Ok(Rd(before + 1 - JDN_OFFSET)),
            _ => Ok(Rd(end - JDN_OFFSET)),
        }
    }

    /// The Tibetan date of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside the years converted.
    pub fn date_from_fixed(&self, rd: Rd) -> CalendarResult<TibetanDate> {
        if rd < self.earliest() {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd > self.latest() {
            return Err(CalendarError::AfterSupportedRange);
        }
        let jdn = rd.0 + JDN_OFFSET;
        let estimate = Ratio::int(jdn as i128)
            .sub(self.m0)
            .mul(Ratio::new(M1.den, M1.num))
            .floor() as i64;
        for n in [estimate - 1, estimate, estimate + 1] {
            let mut before = self.previous_end_day(1, n);
            if before >= jdn {
                continue;
            }
            for day in 1..=30 {
                let end = self.end_day(day, n);
                if before < jdn && jdn <= end {
                    let (year, month, leap) = self.month_of_count(n);
                    return Ok(TibetanDate {
                        year,
                        month: if leap {
                            Month::leap(month)
                        } else {
                            Month::regular(month)
                        },
                        day: day as u8,
                        leap_day: end - before == 2 && jdn == before + 1,
                    });
                }
                before = end;
            }
        }
        Err(CalendarError::DayOutOfRange)
    }
}

/// The five elements of the sixty-year cycle, two years each.
pub const ELEMENTS: [&str; 5] = ["Wood", "Fire", "Earth", "Iron", "Water"];

/// The twelve animals of the cycle, from the Mouse.
pub const ANIMALS: [&str; 12] = [
    "Mouse", "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Sheep", "Monkey", "Bird", "Dog",
    "Pig",
];

/// The name of a year in the sixty-year cycle: its element, whether it is
/// male or female, and its animal. 2007 is Fire, female, Pig. Every version
/// names its years so (Janson, Section 4 and Appendix A); Mongolians often
/// give the element as a colour, which is not carried.
#[must_use]
pub fn year_name(year: i64) -> (&'static str, bool, &'static str) {
    let position = (year - 4).rem_euclid(60);
    (
        ELEMENTS[(position % 10 / 2) as usize],
        position % 2 == 0,
        ANIMALS[(position % 12) as usize],
    )
}

/// The year's place in the Prabhava (*rab byung*) cycles of sixty years
/// numbered from 1027: the cycle number and the year within it. 2007 is
/// the 21st year of the 17th cycle. Mongolia numbers its *jaran* cycles the
/// same way (Janson, Appendix A.3).
#[must_use]
pub fn prabhava(year: i64) -> (i64, i64) {
    (
        (year - 1026 + 59).div_euclid(60),
        (year - 1027).rem_euclid(60) + 1,
    )
}

/// A Tibetan date, in any of the versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TibetanDate {
    /// The year, numbered by the Western year it begins in.
    pub year: i64,
    /// The month, 1 to 12, with the leap month flagged. The leap month
    /// precedes the regular month of the same number, except in the
    /// Bhutanese version, where it follows it.
    pub month: Month,
    /// The lunar day, 1 to 30.
    pub day: u8,
    /// Whether this is the first of two calendar days with the same
    /// number, the *extra* day of the almanacs.
    pub leap_day: bool,
}

impl fmt::Display for TibetanDate {
    /// Writes the date as year, month and day, marking a leap month and an
    /// extra day: `2000-1L-1`, `2024-6-15x`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.year, self.month.ordinal)?;
        if self.month.leap {
            write!(f, "L")?;
        }
        write!(f, "-{}", self.day)?;
        if self.leap_day {
            write!(f, "x")?;
        }
        Ok(())
    }
}

impl Calendar for TibetanCalendar {
    type Date = TibetanDate;

    /// From Losar of the year the version's source dates it to — 1447 for
    /// the Phugpa and the Tsurphu, 1786 for the Mongolian — and kept
    /// today; the years from 1000 convert by the same arithmetic and are
    /// proleptic. The Bhutanese is attested without a beginning. See
    /// [`USAGE_SOURCE`], [`TSURPHU_USAGE_SOURCE`], [`BHUTAN_USAGE_SOURCE`]
    /// and [`MONGOLIAN_USAGE_SOURCE`].
    fn usage(&self) -> Usage {
        match self.first_year {
            Some(year) => match self.new_year(year) {
                Ok(losar) => Usage::since(losar, self.usage_source),
                Err(_) => Usage::UNRECORDED,
            },
            None => Usage::undated(self.usage_source),
        }
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// A year with a doubled month.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(TibetanCalendar::is_leap_year(self, year))
    }

    /// Mean daybreak, [`DAWN`]: the Tibetan day runs from dawn to dawn, and
    /// is named by the civil day on whose dawn it begins. Janson numbers
    /// each calendar day by the Julian Day Number of that civil day and
    /// treats the true date as "a kind of local Julian date" that takes
    /// "integer values at local (mean) dawn" where the astronomical one
    /// takes them at noon (Janson, "Tibetan calendar mathematics", Section 2 and
    /// Remark 6), so the hours before dawn belong to the day before, as the
    /// morning belongs to the previous Julian Day.
    fn day_boundary(&self) -> DayBoundary {
        DayBoundary::LocalTime(DAWN, DayNaming::ByStart)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id,
            english_name: self.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: false,
            earliest: Some(self.earliest()),
            latest: Some(self.latest()),
            native_locales: self.native_locales,
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.date_to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.date_from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut fields = DateFields::ymd(date.year, date.month.ordinal, date.day);
        fields.month = Some(date.month);
        fields.leap_day = date.leap_day;
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let date = TibetanDate {
            year: fields.year,
            month: fields.require_month()?,
            day: fields.require_day()?,
            leap_day: fields.leap_day,
        };
        self.date_to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::civil;

    const VERSIONS: [TibetanCalendar; 4] = [TIBETAN, TIBETAN_TSURPHU, MONGOLIAN, TIBETAN_BHUTAN];

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        civil::to_rd(year, month, day)
    }

    fn date(year: i64, month: Month, day: u8, leap_day: bool) -> TibetanDate {
        TibetanDate {
            year,
            month,
            day,
            leap_day,
        }
    }

    /// `Σ digits[i] / (radices[0] · … · radices[i])`: a mixed-radix
    /// fraction as the almanacs and Henning's epoch data write it.
    fn digits(values: &[i128], radices: &[i128]) -> Ratio {
        let mut total = Ratio::int(0);
        let mut unit = 1;
        for (value, radix) in values.iter().zip(radices) {
            unit *= radix;
            total = total.add(Ratio::new(*value, unit));
        }
        total
    }

    #[test]
    fn the_day_begins_at_mean_daybreak() {
        for calendar in VERSIONS {
            assert_eq!(
                calendar.day_boundary(),
                DayBoundary::LocalTime(CivilTime::hms(5, 0, 0).unwrap(), DayNaming::ByStart)
            );
        }
    }

    /// Losar 2024 is 10 February: from its dawn on, and until the next
    /// dawn, but not in the small hours before it, which are still the last
    /// day of 2023.
    #[test]
    fn losar_begins_at_the_dawn_of_its_civil_day() {
        let boundary = TIBETAN.day_boundary();
        let tibetan = |civil: Rd, hour: u8| {
            let offset = boundary
                .civil_day_offset(CivilTime::hms(hour, 0, 0).unwrap())
                .unwrap();
            let date = TIBETAN.date_from_fixed(civil + offset).expect("in range");
            (date.year, date.month.ordinal, date.day)
        };
        assert_eq!(tibetan(greg(2024, 2, 10), 5), (2024, 1, 1));
        assert_eq!(tibetan(greg(2024, 2, 10), 23), (2024, 1, 1));
        assert_eq!(tibetan(greg(2024, 2, 11), 4), (2024, 1, 1));
        assert_eq!(tibetan(greg(2024, 2, 10), 4).0, 2023);
    }

    #[test]
    fn the_sources_own_dates_and_losar_of_2000_are_reproduced() {
        // The paper is dated 31 December 2007, day 23 of month 11 of the
        // Fire–Pig year, and revised 8 January 2014, day 8 of month 11 of the
        // Water–Snake year.
        let first = TIBETAN
            .date_from_fixed(greg(2007, 12, 31))
            .expect("in range");
        assert_eq!(
            (first.year, first.month, first.day, first.leap_day),
            (2007, Month::regular(11), 23, false)
        );
        assert_eq!(year_name(2007), ("Fire", false, "Pig"));
        let second = TIBETAN.date_from_fixed(greg(2014, 1, 8)).expect("in range");
        assert_eq!(
            (second.year, second.month, second.day),
            (2013, Month::regular(11), 8)
        );
        assert_eq!(year_name(2013), ("Water", false, "Snake"));
        assert_eq!(prabhava(2007), (17, 21));
        // Losar 2000 fell on Sunday 6 February, the first day of a leap
        // month 1 (the source's footnote 28).
        assert_eq!(TIBETAN.new_year(2000), Ok(greg(2000, 2, 6)));
        let losar = TIBETAN.date_from_fixed(greg(2000, 2, 6)).expect("in range");
        assert_eq!(
            (losar.year, losar.month, losar.day),
            (2000, Month::leap(1), 1)
        );
        assert_eq!(TIBETAN.leap_month_of(2000), Some(1));
        assert_eq!(losar.to_string(), "2000-1L-1");
    }

    #[test]
    fn losar_falls_on_the_published_days_of_recent_years() {
        for (year, month, day) in [(2023, 2, 21), (2024, 2, 10), (2025, 2, 28), (2026, 2, 18)] {
            assert_eq!(TIBETAN.new_year(year), Ok(greg(year, month, day)), "{year}");
            let date = TIBETAN
                .date_from_fixed(greg(year, month, day))
                .expect("in range");
            assert_eq!(
                (date.year, date.month.ordinal, date.day),
                (year, 1, 1),
                "{year}"
            );
        }
        // Saga Dawa Düchen, the fifteenth of the fourth month, 23 May 2024.
        let saga_dawa = TIBETAN
            .date_from_fixed(greg(2024, 5, 23))
            .expect("in range");
        assert_eq!(
            (saga_dawa.year, saga_dawa.month, saga_dawa.day),
            (2024, Month::regular(4), 15)
        );
        assert_eq!(year_name(2024), ("Wood", true, "Dragon"));
    }

    #[test]
    fn leap_years_follow_the_sixty_five_year_rule() {
        let leap: Vec<i64> = (2000..=2030)
            .filter(|year| TIBETAN.is_leap_year(*year))
            .collect();
        assert_eq!(
            leap,
            [
                2000, 2002, 2005, 2008, 2010, 2013, 2016, 2019, 2021, 2024, 2027, 2029
            ]
        );
        assert_eq!(TIBETAN.leap_month_of(2024), Some(6));
        assert_eq!(TIBETAN.leap_month_of(2023), None);
        assert!(TIBETAN.true_month_count(2024, 6, true).is_some());
        assert_eq!(TIBETAN.true_month_count(2024, 5, true), None);
        for year in 1000..=3000 {
            // (5.41) and (5.34).
            assert_eq!(
                TIBETAN.is_leap_year(year),
                (24 * year + 33).rem_euclid(65) >= 41,
                "{year}"
            );
            if let Some(month) = TIBETAN.leap_month_of(year) {
                let x = (24 * (year - 806) - 123).rem_euclid(65);
                assert_eq!(i64::from(month), 1 + (64 - x).div_euclid(2), "{year}");
            }
        }
        // The inverse of the true month count is exact across a cycle, and
        // the leap flag is (5.22).
        for n in 14_000..14_900 {
            let (year, month, leap) = TIBETAN.month_of_count(n);
            assert_eq!(
                TIBETAN.true_month_count(year, month, leap),
                Some(n),
                "n {n}"
            );
            assert_eq!(
                leap,
                [1, 2].contains(&(65 * n + 123).rem_euclid(67)),
                "n {n}"
            );
        }
    }

    /// The constants each version's section of Janson's Appendix A states:
    /// β of the inverse, and γ* of the general form of the leap-year rule
    /// (5.41), "(24 Y + γ*) mod 65 ≥ 41" — 33 for the Phugpa, 20 for the Tsurphu and the
    /// Mongolian, 28 for the Bhutanese.
    #[test]
    fn the_inverse_constants_and_leap_year_rules_are_jansons() {
        assert_eq!(TIBETAN.inverse_constant(), 123);
        assert_eq!(TIBETAN_TSURPHU.inverse_constant(), 187);
        assert_eq!(MONGOLIAN.inverse_constant(), 172);
        assert_eq!(TIBETAN_BHUTAN.inverse_constant(), 191);
        for (calendar, gamma) in [
            (TIBETAN, 33),
            (TIBETAN_TSURPHU, 20),
            (MONGOLIAN, 20),
            (TIBETAN_BHUTAN, 28),
        ] {
            for year in 1000..=3000 {
                assert_eq!(
                    calendar.is_leap_year(year),
                    (24 * year + gamma).rem_euclid(65) >= 41,
                    "{} {year}",
                    calendar.meta().id
                );
            }
            for n in 20_000..21_000 {
                let (year, month, leap) = calendar.month_of_count(n);
                assert_eq!(
                    calendar.true_month_count(year, month, leap),
                    Some(n),
                    "{} n {n}",
                    calendar.meta().id
                );
            }
        }
    }

    /// Henning's "Epoch data" gives each text's epoch in the almanacs'
    /// mixed radices; Janson's Appendix A turns them into the rationals the
    /// calendars carry, adding the Julian Day of the mean weekday's
    /// weekday 0 (his Remark 8). The shared mean motions are Henning's too,
    /// the month's with Janson's 28 added.
    #[test]
    fn the_epoch_values_are_hennings_digits() {
        assert_eq!(
            Ratio::int(29).add(digits(&[31, 50, 0, 480], &[60, 60, 6, 707])),
            M1
        );
        assert_eq!(digits(&[2, 10, 58, 1, 17], &[27, 60, 60, 6, 67]), S1);
        assert_eq!(digits(&[2, 1], &[28, 126]), A1);
        // Kongtrul, 1852: true month 0;14, mean weekday
        // 2;9,24,2,5,417 (13,707), anomaly 0;72, mean sun 0;1,22,2,4,18 (13,67).
        let tsurphu = TIBETAN_TSURPHU;
        assert_eq!(tsurphu.epoch_index(), 14);
        assert_eq!(
            Ratio::int(2_397_596 + 2).add(digits(&[9, 24, 2, 5, 417], &[60, 60, 6, 13, 707])),
            tsurphu.m0
        );
        assert_eq!(digits(&[0, 72], &[28, 126]), tsurphu.a0);
        assert_eq!(
            digits(&[0, 1, 22, 2, 4, 18], &[27, 60, 60, 6, 13, 67]),
            tsurphu.s0
        );
        // Yeshe Paljor, 1747: 0;10, 1;55,13,3,31,394 (67,707), 24;22,
        // 26;39,51,0,18.
        assert_eq!(MONGOLIAN.epoch_index(), 10);
        assert_eq!(
            Ratio::int(2_359_236 + 1).add(digits(&[55, 13, 3, 31, 394], &[60, 60, 6, 67, 707])),
            MONGOLIAN.m0
        );
        assert_eq!(digits(&[24, 22], &[28, 126]), MONGOLIAN.a0);
        assert_eq!(
            digits(&[26, 39, 51, 0, 18], &[27, 60, 60, 6, 67]),
            MONGOLIAN.s0
        );
        // Lhawang Lodrö, 1754: 0;2, 2;4,24,552 (707), 3;30, 0;24,10,50 (67).
        assert_eq!(TIBETAN_BHUTAN.epoch_index(), 2);
        assert_eq!(
            Ratio::int(2_361_805 + 2).add(digits(&[4, 24, 552], &[60, 60, 707])),
            TIBETAN_BHUTAN.m0
        );
        assert_eq!(digits(&[3, 30], &[28, 126]), TIBETAN_BHUTAN.a0);
        assert_eq!(
            digits(&[0, 24, 10, 50], &[27, 60, 60, 67]),
            TIBETAN_BHUTAN.s0
        );
    }

    /// Janson's Table of epoch data for the four versions at the common
    /// epoch of JD 2 015 531, 23 March 806: the mean date less 2 015 529,
    /// the mean sun and the anomaly, to six decimals (Appendix A.13).
    #[test]
    fn the_versions_meet_at_the_common_epoch_as_jansons_table_gives() {
        for (calendar, date, sun, anomaly) in [
            (TIBETAN, 2.376_238, 0.004_975, 0.206_349),
            (TIBETAN_TSURPHU, 2.422_338, 0.018_261, 0.210_317),
            (MONGOLIAN, 2.418_494, 0.023_632, 0.207_200),
            (TIBETAN_BHUTAN, 2.410_537, 0.017_413, 0.220_522),
        ] {
            let value = |ratio: Ratio| ratio.num as f64 / ratio.den as f64;
            let n = ((2_015_531.4 - value(calendar.m0)) / value(M1)).round() as i128;
            let n = Ratio::int(n);
            let mean_date = value(calendar.m0.add(n.mul(M1)).sub(Ratio::int(2_015_529)));
            let mean_sun = value(calendar.s0.add(n.mul(S1)).frac());
            let mean_anomaly = value(calendar.a0.add(n.mul(A1)).frac());
            let id = calendar.meta().id;
            assert!((mean_date - date).abs() < 5e-7, "{id}: {mean_date}");
            assert!((mean_sun - sun).abs() < 5e-7, "{id}: {mean_sun}");
            assert!(
                (mean_anomaly - anomaly).abs() < 5e-7,
                "{id}: {mean_anomaly}"
            );
        }
    }

    /// Janson's comparison of the four versions, Appendix A.13: Losar
    /// 2000–2030 and the leap months 2000–2019.
    #[test]
    fn losar_and_the_leap_months_are_those_of_jansons_comparison() {
        #[rustfmt::skip]
        const LOSAR: [[(u8, u8); 4]; 31] = [
            [(2, 6), (2, 6), (2, 6), (2, 6)],
            [(2, 24), (2, 24), (2, 24), (2, 24)],
            [(2, 13), (2, 13), (2, 13), (2, 13)],
            [(3, 3), (2, 2), (2, 2), (3, 4)],
            [(2, 21), (2, 21), (2, 21), (2, 21)],
            [(2, 9), (2, 9), (2, 9), (2, 9)],
            [(2, 28), (1, 30), (1, 30), (2, 28)],
            [(2, 18), (2, 18), (2, 18), (2, 18)],
            [(2, 7), (2, 8), (2, 8), (2, 8)],
            [(2, 25), (2, 25), (2, 25), (2, 25)],
            [(2, 14), (2, 14), (2, 14), (2, 14)],
            [(3, 5), (2, 3), (2, 3), (2, 3)],
            [(2, 22), (2, 22), (2, 22), (2, 22)],
            [(2, 11), (2, 11), (2, 11), (2, 11)],
            [(3, 2), (1, 31), (1, 31), (3, 2)],
            [(2, 19), (2, 19), (2, 19), (2, 19)],
            [(2, 9), (2, 9), (2, 9), (2, 9)],
            [(2, 27), (2, 27), (2, 27), (2, 27)],
            [(2, 16), (2, 16), (2, 16), (2, 16)],
            [(2, 5), (2, 5), (2, 5), (2, 5)],
            [(2, 24), (2, 24), (2, 24), (2, 24)],
            [(2, 12), (2, 12), (2, 12), (2, 12)],
            [(3, 3), (2, 2), (2, 2), (3, 3)],
            [(2, 21), (2, 21), (2, 21), (2, 21)],
            [(2, 10), (2, 10), (2, 10), (2, 10)],
            [(2, 28), (3, 1), (3, 1), (2, 28)],
            [(2, 18), (2, 18), (2, 18), (2, 18)],
            [(2, 7), (2, 7), (2, 7), (2, 7)],
            [(2, 26), (2, 26), (2, 26), (2, 26)],
            [(2, 14), (2, 14), (2, 14), (2, 14)],
            [(3, 5), (2, 3), (2, 3), (2, 3)],
        ];
        #[rustfmt::skip]
        const LEAP: [[u8; 4]; 20] = [
            [1, 8, 8, 4], [0, 0, 0, 0], [10, 0, 0, 12], [0, 4, 4, 0], [0, 0, 0, 0],
            [6, 0, 0, 9], [0, 1, 1, 0], [0, 0, 0, 0], [3, 9, 9, 5], [0, 0, 0, 0],
            [11, 0, 0, 0], [0, 6, 6, 2], [0, 0, 0, 0], [8, 0, 0, 10], [0, 2, 2, 0],
            [0, 0, 0, 0], [4, 11, 11, 7], [0, 0, 0, 0], [0, 0, 0, 0], [1, 7, 7, 3],
        ];
        for (offset, row) in LOSAR.iter().enumerate() {
            let year = 2000 + offset as i64;
            for (calendar, (month, day)) in VERSIONS.iter().zip(row) {
                assert_eq!(
                    calendar.new_year(year),
                    Ok(greg(year, *month, *day)),
                    "{} {year}",
                    calendar.meta().id
                );
            }
        }
        for (offset, row) in LEAP.iter().enumerate() {
            let year = 2000 + offset as i64;
            for (calendar, month) in VERSIONS.iter().zip(row) {
                let expected = (*month != 0).then_some(*month);
                assert_eq!(
                    calendar.leap_month_of(year),
                    expected,
                    "{} {year}",
                    calendar.meta().id
                );
            }
        }
    }

    /// Janson's table of the repeated and skipped (negative) days of every
    /// month of 2012 in the four versions, Appendix A.13.
    #[test]
    fn the_skipped_and_repeated_days_of_2012_are_jansons() {
        #[rustfmt::skip]
        const DAYS: [[&[i8]; 12]; 4] = [
            [&[5, -19], &[9, -12, -25, 27], &[-17], &[3, -10], &[-13, 29], &[-6],
             &[-9, 25], &[-1], &[-5, 20, -29], &[], &[-3, 13, -27], &[17, -21]],
            [&[4, -20], &[8, -13], &[-17], &[2, -11], &[-14, 28], &[-6],
             &[-9, 25], &[-2], &[-6, 19, -29], &[], &[-3, 12, -28], &[15, -22]],
            [&[4, -20], &[8, -13], &[-17], &[2, -11], &[-14, 28], &[-6],
             &[-9, 25], &[-2], &[-6, 20, -29], &[], &[-4, 12, -28], &[15, -22]],
            [&[4, -19], &[8, -13], &[-17], &[2, -10], &[-13, 28], &[-6],
             &[-9, 24], &[-1], &[-5, 19, -29], &[], &[-3, 12, -27], &[15, -21]],
        ];
        for (calendar, months) in VERSIONS.iter().zip(DAYS) {
            for (index, expected) in months.iter().enumerate() {
                let month = Month::regular(index as u8 + 1);
                let mut found = Vec::new();
                for day in 1..=30u8 {
                    if calendar
                        .date_to_fixed(date(2012, month, day, false))
                        .is_err()
                    {
                        found.push(-(day as i8));
                    } else if calendar.date_to_fixed(date(2012, month, day, true)).is_ok() {
                        found.push(day as i8);
                    }
                }
                assert_eq!(
                    &found,
                    expected,
                    "{} month {}",
                    calendar.meta().id,
                    month.ordinal
                );
            }
        }
    }

    #[test]
    fn every_day_of_four_decades_round_trips_and_years_have_the_five_lengths() {
        for calendar in VERSIONS {
            let id = calendar.meta().id;
            let start = greg(1990, 1, 1).0;
            let end = greg(2031, 1, 1).0;
            let mut previous: Option<TibetanDate> = None;
            let mut skipped = 0;
            let mut repeated = 0;
            for rd in start..end {
                let date = calendar.from_fixed(Rd(rd)).expect("in range");
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{id} rd {rd} {date}");
                let fields = calendar.to_fields(date).expect("describable");
                assert_eq!(calendar.from_fields(&fields), Ok(date), "{id} rd {rd}");
                if let Some(before) = previous
                    && before.month == date.month
                    && before.year == date.year
                {
                    if before.day == date.day {
                        repeated += 1;
                        assert!(before.leap_day && !date.leap_day, "{id} rd {rd}");
                    } else {
                        assert!(date.day > before.day, "{id} rd {rd}");
                        if date.day - before.day == 2 {
                            skipped += 1;
                        }
                    }
                }
                previous = Some(date);
            }
            assert!(skipped > 0 && repeated > 0, "{id}");
            for year in 1990..2030 {
                let length =
                    calendar.new_year(year + 1).unwrap().0 - calendar.new_year(year).unwrap().0;
                assert!(
                    [354, 355, 383, 384, 385].contains(&length),
                    "{id} {year}: {length}"
                );
                assert_eq!(length >= 383, calendar.is_leap_year(year), "{id} {year}");
            }
        }
    }

    /// The leap month is the first of the two months with its number in
    /// the Phugpa, Tsurphu and Mongolian versions, and the second in the
    /// Bhutanese.
    #[test]
    fn the_bhutanese_leap_month_follows_the_month_it_repeats() {
        let first_day = |calendar: TibetanCalendar, year, month| {
            calendar.date_to_fixed(date(year, month, 1, false)).unwrap()
        };
        assert!(
            first_day(TIBETAN, 2024, Month::leap(6)) < first_day(TIBETAN, 2024, Month::regular(6))
        );
        assert!(
            first_day(TIBETAN_BHUTAN, 2008, Month::leap(5))
                > first_day(TIBETAN_BHUTAN, 2008, Month::regular(5))
        );
        assert_eq!(TIBETAN_BHUTAN.leap_numbering(), LeapNumbering::Preceding);
        assert_eq!(TIBETAN_TSURPHU.leap_numbering(), LeapNumbering::Following);
    }

    /// Janson's first Tsurphu epoch, 1732 (the *Excellent Flask of
    /// Essentials*), is "equivalent and giving the same calendar" as
    /// Kongtrul's of 1852 (Appendix A.2): every Losar of 1600–2300 agrees,
    /// and every day of 1800–2200 in a release build.
    #[test]
    fn the_two_tsurphu_epochs_give_one_calendar() {
        let e1732 = TibetanCalendar {
            epoch_year: 1_732,
            epoch_index: 59,
            m0: Ratio::new(2_353_745 * 7_635_600 + 1_795_153, 7_635_600),
            s0: Ratio::new(-5_983, 108_540),
            a0: Ratio::new(207, 392),
            ..TIBETAN_TSURPHU
        };
        assert_eq!(e1732.inverse_constant(), 142);
        assert_eq!(
            Ratio::int(2_353_741 + 4).add(digits(&[14, 6, 2, 2, 666], &[60, 60, 6, 13, 707])),
            e1732.m0
        );
        assert_eq!(
            Ratio::int(0).sub(digits(&[1, 29, 17, 5, 6, 1], &[27, 60, 60, 6, 13, 67])),
            e1732.s0
        );
        assert_eq!(digits(&[14, 99], &[28, 126]), e1732.a0);
        for year in 1600..=2300 {
            assert_eq!(
                e1732.new_year(year),
                TIBETAN_TSURPHU.new_year(year),
                "{year}"
            );
        }
        let step = if cfg!(debug_assertions) { 7 } else { 1 };
        for rd in (greg(1800, 1, 1).0..greg(2200, 1, 1).0).step_by(step) {
            assert_eq!(
                e1732.date_from_fixed(Rd(rd)),
                TIBETAN_TSURPHU.date_from_fixed(Rd(rd)),
                "rd {rd}"
            );
        }
    }

    /// Henning's Tsurphu calendar for 2013, as his "Open source Tsurphu
    /// calendar software" page prints its first month: "New Year: 2013,
    /// Water-female-Snake", "Month: 1989;39", "Anomaly: 18;45", "Mean
    /// Weekday: 1;29,39,2,574", and day 1 on 11 February 2013. Its "Mean
    /// Sun" is not compared: the page says the program takes the Sun's
    /// longitude from the *karaṇa* calculation, which this module does not
    /// carry, and which "makes no difference to the structure of the
    /// calendar". And the Karmapa's office, for which "the Tibetan
    /// Year of the Male Wood Horse had arrived" on 31 January 2014.
    #[test]
    fn the_tsurphu_calendar_is_hennings_and_the_karmapas() {
        let tsurphu = TIBETAN_TSURPHU;
        let n = tsurphu.true_month_count(2013, 1, false).unwrap();
        assert_eq!(n, 1_989);
        assert_eq!(tsurphu.index(tsurphu.solar_months(2013, 1)), 39);
        let count = Ratio::int(i128::from(n));
        assert_eq!(
            tsurphu.a0.add(count.mul(A1)).frac(),
            digits(&[18, 45], &[28, 126])
        );
        // The weekday is printed truncated to its last place.
        let weekday = tsurphu.m0.add(count.mul(M1)).sub(Ratio::int(2_397_596));
        let weekday = Ratio::new(weekday.num.rem_euclid(7 * weekday.den), weekday.den);
        let printed = Ratio::int(1).add(digits(&[29, 39, 2, 574], &[60, 60, 6, 707]));
        let excess = weekday.sub(printed);
        assert!(
            excess.num >= 0 && excess.num * 21_600 * 707 < excess.den,
            "{excess:?}"
        );
        assert_eq!(tsurphu.new_year(2013), Ok(greg(2013, 2, 11)));
        assert_eq!(year_name(2013), ("Water", false, "Snake"));
        assert_eq!(tsurphu.new_year(2014), Ok(greg(2014, 1, 31)));
        assert_eq!(year_name(2014), ("Wood", true, "Horse"));
        // The Phugpa Losar of 2014 was a month later.
        assert_eq!(TIBETAN.new_year(2014), Ok(greg(2014, 3, 2)));
    }

    /// The dates of the Mongolian calendar the sources give: the
    /// constitution of 1992 came into force on "the auspicious yellow horse
    /// day of the black tiger first spring month of the water monkey year of
    /// the seventeenth 60-year cycle", which Janson dates 9/1 = 12 February
    /// 1992 (his footnote to Appendix A.3); the Government's resolution 109
    /// of 26 February 2025 finds that "шинийн 1 тасарч, шинийн 2, 3-ны өдөр
    /// Бямба, Ням гарагт тохиож" — the first day of the first spring month is
    /// omitted and its second and third fall on Saturday and Sunday, 1 and
    /// 2 March 2025; MONTSAME dates Tsagaan Sar to 24–26 February 2020 and
    /// its first three days to 18, 19 and 20 February 2026; and Gantumur's
    /// worked Tsagaan Sar 2026 has true month 3449 and JD 2 461 090.
    #[test]
    fn tsagaan_sar_falls_on_the_published_days() {
        let mongolian = |y, m, d| {
            let t = MONGOLIAN.date_from_fixed(greg(y, m, d)).unwrap();
            (t.year, t.month, t.day, t.leap_day)
        };
        assert_eq!(mongolian(1992, 2, 12), (1992, Month::regular(1), 9, false));
        assert_eq!(year_name(1992), ("Water", true, "Monkey"));
        assert_eq!(prabhava(1992).0, 17);
        assert_eq!(
            mongolian(2025, 2, 28),
            (2024, Month::regular(12), 30, false)
        );
        assert_eq!(mongolian(2025, 3, 1), (2025, Month::regular(1), 2, false));
        assert_eq!(mongolian(2025, 3, 2), (2025, Month::regular(1), 3, false));
        assert_eq!(
            MONGOLIAN.date_to_fixed(date(2025, Month::regular(1), 1, false)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(MONGOLIAN.new_year(2025), Ok(greg(2025, 3, 1)));
        for (year, month, day) in [(2020, 2, 24), (2026, 2, 18)] {
            assert_eq!(
                MONGOLIAN.new_year(year),
                Ok(greg(year, month, day)),
                "{year}"
            );
            for offset in 0..3u8 {
                assert_eq!(
                    mongolian(year, month, day + offset),
                    (year, Month::regular(1), offset + 1, false),
                    "{year}"
                );
            }
        }
        assert_eq!(MONGOLIAN.true_month_count(2026, 1, false), Some(3_449));
        assert_eq!(MONGOLIAN.new_year_jdn(2026), 2_461_090);
        // The New Genden and the Tsurphu have the same leap months
        // (Janson, Appendix A.3; Gantumur, Section 2.1).
        for year in 1000..=3000 {
            assert_eq!(
                MONGOLIAN.leap_month_of(year),
                TIBETAN_TSURPHU.leap_month_of(year),
                "{year}"
            );
        }
    }

    /// Tsagaan Sar is not "the second new moon after the winter solstice":
    /// the first month of the New Genden calendar begins the day of or after
    /// the second or the third astronomical new moon after the December
    /// solstice, the third in the years listed, 2020 and 2025 among them.
    /// New moons are dated in Ulaanbaatar's standard time, UTC+8.
    #[test]
    fn tsagaan_sar_follows_the_second_or_third_new_moon_after_the_solstice() {
        let mut third = Vec::new();
        for year in 2000..=2030 {
            let solstice = hc_astro::solstice(year - 1, hc_astro::Solstice::December);
            let losar = MONGOLIAN.new_year(year).unwrap().0;
            let local_day = |moment: hc_astro::Moment| (moment.0 + 8.0 / 24.0).floor() as i64;
            let mut count = 0;
            let mut last = 0;
            let mut moon = hc_astro::new_moon_at_or_after(solstice);
            while local_day(moon) <= losar {
                count += 1;
                last = local_day(moon);
                moon = hc_astro::new_moon_at_or_after(hc_astro::Moment(moon.0 + 1.0));
            }
            assert!(losar - last <= 1, "{year}");
            assert!([2, 3].contains(&count), "{year}: {count}");
            if count == 3 {
                third.push(year);
            }
        }
        assert_eq!(
            third,
            [2001, 2004, 2009, 2012, 2015, 2017, 2020, 2023, 2025, 2028]
        );
    }

    /// The Bhutanese dates the sources give: the Election Act was enacted
    /// on the "26th Day of the Second 5th Month of the Earth Male Rat Year
    /// corresponding to the 28th Day of the 7th Month of the Year 2008"
    /// (Janson's footnote to his Bhutanese definition points, in the appendix
    /// on leap months and the mean sun, citing the National Assembly);
    /// and the Ministry of Home Affairs' Dzongkha holiday lists for 2025 and
    /// 2026 give each holiday's Bhutanese month and day beside the
    /// Gregorian date.
    #[test]
    fn the_bhutanese_calendar_gives_the_governments_dates() {
        let bhutan = |y, m, d| {
            let t = TIBETAN_BHUTAN.date_from_fixed(greg(y, m, d)).unwrap();
            (t.year, t.month, t.day)
        };
        assert_eq!(bhutan(2008, 7, 28), (2008, Month::leap(5), 26));
        assert_eq!(year_name(2008), ("Earth", true, "Mouse"));
        assert_eq!(
            TIBETAN_BHUTAN
                .date_from_fixed(greg(2008, 7, 28))
                .unwrap()
                .to_string(),
            "2008-5L-26"
        );
        /// A Gregorian date and the Bhutanese one the list gives it.
        type Listed = ((i64, u8, u8), (i64, u8, u8));
        #[rustfmt::skip]
        const LISTED: &[Listed] = &[
            // 2025: winter solstice, day of offering, the King's birthday,
            // Losar, the Third King's birthday, Zhabdrung Kuchoe,
            // Parinirvana, Guru Rinpoche, First Sermon, Blessed Rainy Day,
            // Thimphu Drubchoe, Thimphu Tshechu and Dassain, the
            // Coronation, the Fourth King's birthday, National Day.
            ((2025, 1, 2), (2024, 11, 3)), ((2025, 1, 30), (2024, 12, 1)),
            ((2025, 2, 21), (2024, 12, 24)), ((2025, 2, 23), (2024, 12, 25)),
            ((2025, 2, 28), (2025, 1, 1)), ((2025, 3, 1), (2025, 1, 2)),
            ((2025, 5, 2), (2025, 3, 5)), ((2025, 5, 7), (2025, 3, 10)),
            ((2025, 6, 11), (2025, 4, 15)), ((2025, 7, 5), (2025, 5, 10)),
            ((2025, 7, 28), (2025, 6, 4)), ((2025, 9, 23), (2025, 8, 2)),
            ((2025, 9, 28), (2025, 8, 6)), ((2025, 10, 2), (2025, 8, 10)),
            ((2025, 10, 4), (2025, 8, 12)), ((2025, 11, 1), (2025, 9, 11)),
            ((2025, 11, 11), (2025, 9, 22)), ((2025, 12, 17), (2025, 10, 28)),
            // 2026, the same holidays, with the Descending Day.
            ((2026, 1, 2), (2025, 11, 14)), ((2026, 1, 19), (2025, 12, 1)),
            ((2026, 2, 18), (2026, 1, 1)), ((2026, 2, 19), (2026, 1, 2)),
            ((2026, 2, 21), (2026, 1, 4)), ((2026, 2, 23), (2026, 1, 6)),
            ((2026, 4, 26), (2026, 3, 10)), ((2026, 5, 2), (2026, 3, 16)),
            ((2026, 5, 31), (2026, 4, 15)), ((2026, 6, 24), (2026, 5, 10)),
            ((2026, 7, 18), (2026, 6, 4)), ((2026, 9, 17), (2026, 8, 6)),
            ((2026, 9, 21), (2026, 8, 10)), ((2026, 9, 23), (2026, 8, 12)),
            ((2026, 10, 21), (2026, 9, 10)), ((2026, 11, 1), (2026, 9, 22)),
            ((2026, 11, 11), (2026, 10, 2)), ((2026, 12, 17), (2026, 11, 8)),
        ];
        for &((y, m, d), (year, month, day)) in LISTED {
            assert_eq!(
                bhutan(y, m, d),
                (year, Month::regular(month), day),
                "{y}-{m}-{d}"
            );
        }
        assert_eq!(year_name(2025), ("Wood", false, "Snake"));
        assert_eq!(year_name(2026), ("Fire", true, "Horse"));
    }

    /// The Ministry of Home Affairs' calendars for 2025 and 2026 print the
    /// Bhutanese day number above every Gregorian day; this is that
    /// transcription, month by month, read from the Ministry's pages and
    /// checked against them by eye. A number that appears twice is a
    /// repeated day, the first of the two the extra one; a number that
    /// does not appear is skipped.
    #[test]
    fn every_day_of_the_ministrys_calendars_for_2025_and_2026_is_reproduced() {
        #[rustfmt::skip]
        const PRINTED: &[(i64, u8, &[u8])] = &[
        (2025, 1, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19, 20, 21, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2]),
        (2025, 2, &[3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 24, 25, 26, 27, 28, 29, 1]),
        (2025, 3, &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2]),
        (2025, 4, &[3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 29, 30, 1, 2, 3]),
        (2025, 5, &[4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 3, 4, 5]),
        (2025, 6, &[6, 7, 8, 9, 10, 11, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5]),
        (2025, 7, &[6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 29, 30, 1, 2, 3, 4, 5, 6, 7]),
        (2025, 8, &[8, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8]),
        (2025, 9, &[9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 4, 5, 6, 7, 8]),
        (2025, 10, &[9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        (2025, 11, &[11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 28, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        (2025, 12, &[11, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12]),
        (2026, 1, &[13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14]),
        (2026, 2, &[15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 24, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12]),
        (2026, 3, &[13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 28, 29, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14]),
        (2026, 4, &[15, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14]),
        (2026, 5, &[15, 16, 17, 18, 19, 20, 21, 21, 22, 23, 24, 25, 26, 27, 28, 30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
        (2026, 6, &[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
        (2026, 7, &[16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]),
        (2026, 8, &[18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 13, 14, 15, 16, 17, 18]),
        (2026, 9, &[19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]),
        (2026, 10, &[20, 21, 22, 23, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21]),
        (2026, 11, &[22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22]),
        (2026, 12, &[23, 24, 25, 26, 27, 28, 29, 30, 1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, 23]),
        ];
        for &(year, month, days) in PRINTED {
            let mut previous: Option<TibetanDate> = None;
            for (index, printed) in days.iter().enumerate() {
                let day = index as u8 + 1;
                let date = TIBETAN_BHUTAN
                    .date_from_fixed(greg(year, month, day))
                    .unwrap();
                assert_eq!(date.day, *printed, "{year}-{month}-{day}");
                if let Some(before) = previous {
                    assert_eq!(
                        before.leap_day,
                        before.day == date.day,
                        "{year}-{month}-{day}"
                    );
                }
                previous = Some(date);
            }
        }
    }

    /// Bhutan's Losar of 2003: Henning reports that the government calendar
    /// had day 1 of month 1 on both 3 and 4 March, where the arithmetic,
    /// his and Janson's, makes 3 March a repeated day 30 of the leap
    /// month 12 of 2002 and Losar 4 March (Janson's footnote to Appendix
    /// A.13). The module carries the arithmetic.
    #[test]
    fn bhutans_losar_of_2003_is_the_arithmetics_not_the_governments() {
        let bhutan = |y, m, d| TIBETAN_BHUTAN.date_from_fixed(greg(y, m, d)).unwrap();
        assert_eq!(bhutan(2003, 3, 3).to_string(), "2002-12L-30");
        assert_eq!(bhutan(2003, 3, 4).to_string(), "2003-1-1");
        assert_eq!(TIBETAN_BHUTAN.new_year(2003), Ok(greg(2003, 3, 4)));
    }

    #[test]
    fn the_versions_are_registered_under_their_own_names() {
        let ids: Vec<&str> = VERSIONS.iter().map(|c| c.meta().id.0).collect();
        assert_eq!(
            ids,
            ["tibetan", "tibetan-tsurphu", "mongolian", "tibetan-bhutan"]
        );
        assert_eq!(TIBETAN_TSURPHU.meta().native_locales, &["bo"]);
        assert_eq!(MONGOLIAN.meta().native_locales, &["mn"]);
        assert_eq!(TIBETAN_BHUTAN.meta().native_locales, &["dz"]);
        assert_eq!(TibetanCalendar::default(), TIBETAN);
        let today = greg(2026, 9, 26);
        for calendar in VERSIONS {
            assert_eq!(
                calendar.standing(today),
                hc_calendar::Standing::InUse,
                "{}",
                calendar.meta().id
            );
            assert!(!calendar.usage().source.is_empty());
        }
        assert_eq!(
            TIBETAN_TSURPHU.usage().from,
            TIBETAN_TSURPHU.new_year(1_447).ok()
        );
        assert_eq!(MONGOLIAN.usage().from, MONGOLIAN.new_year(1_786).ok());
        assert_eq!(TIBETAN_BHUTAN.usage().from, None);
    }

    #[test]
    fn impossible_dates_are_refused() {
        let refuse = |calendar: TibetanCalendar, year, month, day, leap_day| {
            calendar.date_to_fixed(date(year, month, day, leap_day))
        };
        assert_eq!(
            refuse(TIBETAN, 2023, Month::leap(3), 1, false),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            refuse(TIBETAN, 2023, Month::regular(13), 1, false),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            refuse(TIBETAN, 2023, Month::regular(1), 31, false),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            refuse(TIBETAN, 999, Month::regular(1), 1, false),
            Err(CalendarError::YearOutOfRange)
        );
        // Losar of 2024 is not an extra day.
        assert_eq!(
            refuse(TIBETAN, 2024, Month::regular(1), 1, true),
            Err(CalendarError::DayOutOfRange)
        );
        // The Bhutanese 2008 has a leap month 5 and not a leap month 4,
        // which the Phugpa rule's numbering would have given it.
        assert_eq!(
            refuse(TIBETAN_BHUTAN, 2008, Month::leap(4), 1, false),
            Err(CalendarError::MonthOutOfRange)
        );
        for calendar in VERSIONS {
            assert_eq!(
                calendar.date_from_fixed(Rd(calendar.earliest().0 - 1)),
                Err(CalendarError::BeforeEpoch)
            );
            assert_eq!(
                calendar.date_from_fixed(Rd(calendar.latest().0 + 1)),
                Err(CalendarError::AfterSupportedRange)
            );
            assert_eq!(calendar.new_year(3_001), Err(CalendarError::YearOutOfRange));
        }
    }
}
