//! The Lao lunisolar calendar, as Sylvain Dupertuis computes it from Prince
//! Phetsarath's *Horasat Lao*.
//!
//! The system is written up in `docs/systems/khmer-chhankitek.md` in the
//! repository, beside the Khmer calendar it shares its arithmetic with:
//! the rules, where the two countries' statements of them differ, why the
//! range stops where it does, and how the calendar was checked against
//! Dupertuis's tables. This page summarises it and states the code's facts.
//!
//! Twelve months, ເດືອນອ້າຍ (month 1) to ເດືອນສິບສອງ (month 12), of 29 days
//! when the month's number is odd and 30 when it is even: a normal year of
//! 354 days. A leap-month year (ອະທິກະມາດ) doubles month 8 for 384 days; a
//! leap-day year (ອະທິກະວານ) gives month 7 a 30th day for 355; a year is
//! never both. The days are counted 1 to 15 ຂຶ້ນ (waxing) and 1 to 14 or 15
//! ແຮມ (waning). The layout is the Thai and Khmer one and lives in
//! [`southeast_asian`](crate::southeast_asian).
//!
//! # The rule
//!
//! The *suryayatra* rule that [`khmer`](crate::khmer) computes, which
//! Dupertuis states for Laos in the same terms: a leap month when the solar
//! New Year's lunar day, the *dithy*, is 25 or more or 5 or less, with a
//! 24-then-6 exception that gives the first year the month; a leap day when
//! the avoman is under 137, or under 126 in a 366-day solar year; and a day
//! that falls in a leap-month year moved to the next year ("on effectue la
//! correction en reportant d'une année le 7e mois plein", p. 42). The
//! shared computation takes the Khmer bounds, 137 or less and 126 or less
//! with Tum's 137-then-0 exception (`docs/systems/khmer-chhankitek.md`);
//! over the years carried the two statements give the same years, which a
//! test asserts. Two cases
//! Dupertuis leaves open: a *dithy* of 25 followed by 5, for which his
//! sources give no rule (annex 7), and a *dithy* of 5 in a year the avoman
//! makes a leap-day year, which his table's year 1300 shows was not settled
//! as the rule above settles it. The range is the run of years between two
//! of the second, and holds neither.
//!
//! # The range
//!
//! The years 1301 to 1401 of the Chulasakarat era (ຈຸລະສັກກະຣາດ, the
//! "petite ère"), from day 1 of month 1, 23 November 1938, to the last day
//! of month 12, 15 November 2039. Every other day is refused.
//!
//! # How the year is numbered
//!
//! A year here is one run of months, month 1 to month 12, numbered by the
//! Chulasakarat year of the solar New Year that falls in it, as Dupertuis's
//! table numbers the years: year 1343 runs from 8 December 1980 and its New
//! Year was 15 April 1981. The number changes at the solar New Year, in
//! month 5 or 6, and not at day 1 of month 1; the solar New Year, the
//! animal year and the Buddhist Era are not carried.
//!
//! # How a date is written in [`DateFields`]
//!
//! The day is counted straight through the month, 1 to 30: days 1 to 15 are
//! 1 to 15 ຂຶ້ນ, days 16 to 29 or 30 are 1 to 14 or 15 ແຮມ. The half and the
//! day within it are the extra fields `waning` (0 or 1) and
//! `fortnight-day`. The first month 8 of a leap-month year, the extra one,
//! is `Month::leap(8)`, and the second `Month::regular(8)`, as in `khmer`
//! and `thai-lunar`.
//!
//! Sources: Sylvain Dupertuis, "Le calcul du calendrier laotien",
//! *Péninsule* 2/3 (1981), pp. 17–79, retrieved 2026-09-27, for the rules,
//! the year table of 1300–1350 and the month ends of 1979–1980; Lao
//! Wikipedia, "ບຸນສົງການ", revision 54397, retrieved 2026-09-27, for the
//! month names, after Maha Sila Viravong's *Hit Sip Song* (not read);
//! Phetsarath's *Horasat Lao* (1973), Dupertuis's source, not read. Keyed
//! in `docs/references.bib` as `dupertuis1981` and `wikipedia-lo-songkan`.

use core::fmt;

use hc_calendar::fields::ExtraFields;
use hc_calendar::shape::{CycleLength, CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd, YearKind,
};

use crate::southeast_asian::{Fortnight, YearType, Years, suryayatra_year_type, write_digits};

/// The first year carried, in the Chulasakarat era: the lunar year that
/// begins on 23 November 1938.
pub const FIRST_YEAR: i64 = 1301;

/// The last year carried: the lunar year that ends on 15 November 2039.
/// The year after it, 1402, is a *dithy*-5 leap-day year, which the rule
/// read does not settle.
pub const LAST_YEAR: i64 = 1401;

/// The twelve months in Lao, ເດືອນອ້າຍ first. Source: Lao Wikipedia,
/// "ບຸນສົງການ", revision 54397, retrieved 2026-09-27, where month 1 is also
/// called ເດືອນຈຽງ; Dupertuis's glossary gives ເດືອນຈຽງ, ເດືອນກຽງ and
/// ເດືອນອ້າຍ for the first month and ເດືອນຍີ່ for the second (p. 66). The
/// extra month 8 of a leap-month year is ເດືອນແປດ too: no name for it in
/// Lao script was found. The regular one after it is the later eighth
/// month, ເດືອນແປດຫລັງ, which `hc-i18n` names.
pub const MONTHS: [&str; 12] = [
    "ເດືອນອ້າຍ",
    "ເດືອນຍີ່",
    "ເດືອນສາມ",
    "ເດືອນສີ່",
    "ເດືອນຫ້າ",
    "ເດືອນຫົກ",
    "ເດືອນເຈັດ",
    "ເດືອນແປດ",
    "ເດືອນເກົ້າ",
    "ເດືອນສິບ",
    "ເດືອນສິບເອັດ",
    "ເດືອນສິບສອງ",
];

/// Twelve named months, thirteen in a leap-month year, and the week.
static SHAPE: &[CycleShape] = &[
    CycleShape {
        kind: MONTH,
        length: CycleLength::Intercalary {
            ordinary: 12,
            extended: 13,
        },
        names: &MONTHS,
    },
    CycleShape::fixed(WEEKDAY, 7),
];

/// The type of `year`, or `None` outside [`FIRST_YEAR`] to [`LAST_YEAR`].
#[must_use]
pub fn year_type(year: i64) -> Option<YearType> {
    (FIRST_YEAR..=LAST_YEAR)
        .contains(&year)
        .then(|| suryayatra_year_type(year))
}

/// Day 1 of month 1 of [`FIRST_YEAR`], 23 November 1938: the day that puts
/// the New Year of 1301, 16 April 1939, on 12 ແຮມ of month 5, as
/// Dupertuis's table has it.
const EPOCH: Rd = match hc_calendars_solar::gregorian::to_fixed(1938, 11, 23) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The years carried, laid out from [`EPOCH`].
static YEARS: Years = Years {
    first: FIRST_YEAR,
    last: LAST_YEAR,
    epoch: EPOCH,
    months_after_last: 0,
    year_type,
};

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The lunar calendar of Laos, computed by the horasat from the solar New \
    Year [dupertuis1981], as docs/systems/khmer-chhankitek.md states; no source read dates its \
    beginning or its end";

/// The earliest fixed day converted: day 1 of month 1 of [`FIRST_YEAR`].
#[must_use]
pub const fn earliest() -> Rd {
    EPOCH
}

/// The latest fixed day converted: the last day of month 12 of
/// [`LAST_YEAR`].
#[must_use]
pub fn latest() -> Rd {
    YEARS.latest()
}

/// Day 1 of month 1 of `year`, or `None` outside the years carried.
#[must_use]
pub fn new_year(year: i64) -> Option<Rd> {
    YEARS.kind(year)?;
    YEARS.new_year(year)
}

/// The number of days in `month` of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`](hc_calendar::CalendarError::YearOutOfRange) outside the years carried and
/// [`CalendarError::MonthOutOfRange`](hc_calendar::CalendarError::MonthOutOfRange) for a month the year does not have.
pub fn month_length(year: i64, month: Month) -> CalendarResult<u8> {
    YEARS.month_length(year, month)
}

/// A Lao lunar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LaoDate {
    /// The Chulasakarat year of the solar New Year that falls in this run
    /// of months.
    pub year: i64,
    /// The month, 1 to 12; the first month 8 of a leap-month year is
    /// `Month::leap(8)`.
    pub month: Month,
    /// The day of the month, 1 to 30, counted straight through: 16 is
    /// 1 ແຮມ and 30 is 15 ແຮມ.
    pub day: u8,
}

impl LaoDate {
    /// A date from its year, month and day.
    #[must_use]
    pub const fn new(year: i64, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }

    /// The half of the month.
    #[must_use]
    pub const fn fortnight(&self) -> Fortnight {
        Fortnight::of(self.day)
    }

    /// The day within its half, the number before ຄ່ຳ: 1 to 15.
    #[must_use]
    pub const fn fortnight_day(&self) -> u8 {
        Fortnight::day_within(self.day)
    }

    /// The month's name in Lao.
    #[must_use]
    pub fn month_name(&self) -> &'static str {
        MONTHS[(self.month.ordinal as usize).saturating_sub(1) % 12]
    }
}

/// The Lao digit zero, ໐; the nine after it follow in order.
const LAO_ZERO: char = '\u{0ED0}';

impl fmt::Display for LaoDate {
    /// Writes the month, the half and the day in Lao digits, in the order of
    /// Dupertuis's glossary, «ເດືອນ ໕ ຂຶ້ນ ໗ ຄ່ຳ», with the month's name and
    /// the year after it: `ເດືອນຫ້າ ຂຶ້ນ ໑໑ ຄ່ຳ ປີ ໑໓໔໓`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let half = match self.fortnight() {
            Fortnight::Waxing => "ຂຶ້ນ",
            Fortnight::Waning => "ແຮມ",
        };
        write!(f, "{} {half} ", self.month_name())?;
        write_digits(f, i64::from(self.fortnight_day()), LAO_ZERO)?;
        write!(f, " ຄ່ຳ ປີ ")?;
        write_digits(f, self.year, LAO_ZERO)
    }
}

/// The Lao lunar date of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`](hc_calendar::CalendarError::BeforeEpoch) or
/// [`CalendarError::AfterSupportedRange`](hc_calendar::CalendarError::AfterSupportedRange) outside [`earliest`] to
/// [`latest`].
pub fn from_fixed(rd: Rd) -> CalendarResult<LaoDate> {
    let (year, month, day) = YEARS.from_fixed(rd)?;
    Ok(LaoDate { year, month, day })
}

/// The fixed day of a Lao lunar date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`](hc_calendar::CalendarError::YearOutOfRange) outside the years carried,
/// [`CalendarError::MonthOutOfRange`](hc_calendar::CalendarError::MonthOutOfRange) for a month the year does not have
/// and [`CalendarError::DayOutOfRange`](hc_calendar::CalendarError::DayOutOfRange) for a day the month does not have.
pub fn to_fixed(date: LaoDate) -> CalendarResult<Rd> {
    YEARS.to_fixed(date.year, date.month, date.day)
}

/// The Lao lunar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LaoCalendar;

impl Calendar for LaoCalendar {
    type Date = LaoDate;

    /// The lunar calendar of Laos, undated at either end; the years carried
    /// bound the range and not the use.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// An ອະທິກະມາດ year, with its two months 8, or an ອະທິກະວານ year, with
    /// its thirtieth day of month 7.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        YEARS.is_leap_year(year)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("lao"),
            english_name: "Lao lunar",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: false,
            earliest: Some(earliest()),
            latest: Some(latest()),
            native_locales: &["lo"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        from_fixed(rd)
    }

    /// The year, month and day, with the half of the month and the day
    /// within it as the extra fields `waning` and `fortnight-day`.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        to_fixed(date)?;
        let mut extra = ExtraFields::new();
        extra.set("waning", i64::from(date.fortnight() == Fortnight::Waning))?;
        extra.set("fortnight-day", i64::from(date.fortnight_day()))?;
        Ok(DateFields {
            era: None,
            year: date.year,
            month: Some(date.month),
            day: Some(date.day),
            leap_day: false,
            extra,
        })
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let date = LaoDate::new(fields.year, fields.require_month()?, fields.require_day()?);
        to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;

    use hc_calendar::CalendarError;

    use super::*;
    use crate::southeast_asian::{
        avoman, is_solar_leap_year, months_of, new_year_tithi, suryayatra_has_leap_day,
        suryayatra_has_leap_month,
    };

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    /// Rd 0 is a Sunday: 0 Sunday to 6 Saturday.
    fn weekday(rd: Rd) -> i64 {
        rd.0.rem_euclid(7)
    }

    /// A row of the table below: year, day within the half, waning, month,
    /// weekday, and the Gregorian year, month and day.
    type NewYearRow = (i64, u8, bool, u8, i64, i64, u8, u8);

    /// Dupertuis's table of the years 1300 to 1350 (pp. 46–47): the year,
    /// the lunar date of its solar New Year — the day within the half,
    /// waning or not, the month — the weekday, 0 Sunday, and the Gregorian
    /// date. Year 1300 is outside the range and not listed.
    const NEW_YEARS: &[NewYearRow] = &[
        (1301, 12, true, 5, 0, 1939, 4, 16),
        (1302, 8, false, 5, 1, 1940, 4, 15),
        (1303, 4, true, 5, 2, 1941, 4, 15),
        (1304, 1, false, 6, 3, 1942, 4, 15),
        (1305, 12, false, 5, 5, 1943, 4, 16),
        (1306, 7, true, 5, 6, 1944, 4, 15),
        (1307, 4, false, 6, 0, 1945, 4, 15),
        (1308, 14, false, 5, 1, 1946, 4, 15),
        (1309, 11, true, 5, 3, 1947, 4, 16),
        (1310, 7, false, 5, 4, 1948, 4, 15),
        (1311, 2, true, 5, 5, 1949, 4, 15),
        (1312, 13, true, 5, 6, 1950, 4, 15),
        (1313, 10, false, 5, 1, 1951, 4, 16),
        (1314, 6, true, 5, 2, 1952, 4, 15),
        (1315, 3, false, 6, 3, 1953, 4, 15),
        (1316, 13, false, 5, 4, 1954, 4, 15),
        (1317, 9, true, 5, 6, 1955, 4, 16),
        (1318, 6, false, 6, 0, 1956, 4, 15),
        (1319, 1, true, 5, 1, 1957, 4, 15),
        // The table prints "ma", Tuesday; 16 April 1958 was a Wednesday,
        // 366 days after the Monday before, as the solar year of 1319 is.
        (1320, 13, true, 5, 3, 1958, 4, 16),
        (1321, 9, false, 5, 4, 1959, 4, 16),
        (1322, 4, true, 5, 5, 1960, 4, 15),
        (1323, 1, false, 6, 6, 1961, 4, 15),
        (1324, 12, false, 5, 1, 1962, 4, 16),
        (1325, 8, true, 5, 2, 1963, 4, 16),
        (1326, 4, false, 6, 3, 1964, 4, 15),
        (1327, 14, false, 5, 4, 1965, 4, 15),
        (1328, 11, true, 5, 6, 1966, 4, 16),
        (1329, 7, false, 5, 0, 1967, 4, 16),
        (1330, 3, true, 5, 1, 1968, 4, 15),
        (1331, 13, true, 5, 2, 1969, 4, 15),
        (1332, 10, false, 5, 4, 1970, 4, 16),
        (1333, 6, true, 5, 5, 1971, 4, 16),
        (1334, 3, false, 6, 6, 1972, 4, 15),
        (1335, 13, false, 5, 0, 1973, 4, 15),
        (1336, 9, true, 5, 2, 1974, 4, 16),
        (1337, 6, false, 6, 3, 1975, 4, 16),
        (1338, 1, true, 5, 4, 1976, 4, 15),
        (1339, 12, true, 5, 5, 1977, 4, 15),
        (1340, 9, false, 5, 0, 1978, 4, 16),
        (1341, 4, true, 5, 1, 1979, 4, 16),
        (1342, 1, false, 6, 2, 1980, 4, 15),
        (1343, 11, false, 5, 3, 1981, 4, 15),
        (1344, 8, true, 5, 5, 1982, 4, 16),
        (1345, 5, false, 6, 6, 1983, 4, 16),
        (1346, 15, false, 5, 0, 1984, 4, 15),
        (1347, 11, true, 5, 2, 1985, 4, 16),
        (1348, 7, false, 5, 3, 1986, 4, 16),
        (1349, 3, true, 5, 4, 1987, 4, 16),
        (1350, 14, true, 5, 5, 1988, 4, 15),
    ];

    /// Every New Year of Dupertuis's table from 1301 to 1350 falls on the
    /// lunar day and the weekday he gives. The four years after a year of
    /// both a leap month and a leap day by the rule, the table's arrows —
    /// 1305, 1310, 1316 and 1321 — carry the moved day; keeping the day in
    /// the leap-month year instead would put each of them a day earlier.
    #[test]
    fn dupertuis_s_new_years_of_1301_to_1350_are_reproduced() {
        for &(year, day, waning, month, week, gy, gm, gd) in NEW_YEARS {
            let rd = greg(gy, gm, gd);
            let date = from_fixed(rd).expect("in range");
            assert_eq!(date.year, year, "{year}");
            assert_eq!(date.month, Month::regular(month), "{year}");
            assert_eq!(date.fortnight_day(), day, "{year}");
            assert_eq!(date.fortnight() == Fortnight::Waning, waning, "{year}");
            assert_eq!(weekday(rd), week, "{year}");
        }
        for collided in [1304, 1309, 1315, 1320] {
            assert!(suryayatra_has_leap_month(collided));
            assert!(suryayatra_has_leap_day(collided));
            assert_eq!(year_type(collided), Some(YearType::ExtraMonth));
            assert_eq!(year_type(collided + 1), Some(YearType::ExtraDay));
        }
    }

    /// Dupertuis's worked example: the New Year of 1343, Wednesday 15 April
    /// 1981, horakhoune 490 543, avamane 407, dithy 11 (pp. 27–45).
    #[test]
    fn the_worked_example_of_1343_is_reproduced() {
        assert_eq!(crate::southeast_asian::ahargana(1343), 490_543);
        assert_eq!(avoman(1343), 407);
        assert_eq!(new_year_tithi(1343), 11);
        let date = from_fixed(greg(1981, 4, 15)).expect("in range");
        assert_eq!(date, LaoDate::new(1343, Month::regular(5), 11));
        assert_eq!(weekday(greg(1981, 4, 15)), 3);
    }

    /// The last days of the Lao months of 1979 and 1980, as Dupertuis
    /// tabulates them beside the new moons (pp. 70–71): each is the last day
    /// of a month here and the next day the first of the next. His labels
    /// agree to month 7 of 1342; 1342 is a leap-month year in his own year
    /// table, and his labels after it count its two months 8 as one, so
    /// the month ending on 11 August 1980 that he calls the 9th is the
    /// second month 8 here, and the 7 December he leaves unlabelled is the
    /// end of month 12.
    #[test]
    fn the_month_ends_of_1979_and_1980_are_reproduced() {
        let ends: [(i64, Month, i64, u8, u8); 24] = [
            (1341, Month::regular(2), 1979, 1, 28),
            (1341, Month::regular(3), 1979, 2, 26),
            (1341, Month::regular(4), 1979, 3, 28),
            (1341, Month::regular(5), 1979, 4, 26),
            (1341, Month::regular(6), 1979, 5, 26),
            (1341, Month::regular(7), 1979, 6, 24),
            (1341, Month::regular(8), 1979, 7, 24),
            (1341, Month::regular(9), 1979, 8, 22),
            (1341, Month::regular(10), 1979, 9, 21),
            (1341, Month::regular(11), 1979, 10, 20),
            (1341, Month::regular(12), 1979, 11, 19),
            (1342, Month::regular(1), 1979, 12, 18),
            (1342, Month::regular(2), 1980, 1, 17),
            (1342, Month::regular(3), 1980, 2, 15),
            (1342, Month::regular(4), 1980, 3, 16),
            (1342, Month::regular(5), 1980, 4, 14),
            (1342, Month::regular(6), 1980, 5, 14),
            (1342, Month::regular(7), 1980, 6, 12),
            (1342, Month::leap(8), 1980, 7, 12),
            (1342, Month::regular(8), 1980, 8, 11),
            (1342, Month::regular(9), 1980, 9, 9),
            (1342, Month::regular(10), 1980, 10, 9),
            (1342, Month::regular(11), 1980, 11, 7),
            (1342, Month::regular(12), 1980, 12, 7),
        ];
        for (year, month, gy, gm, gd) in ends {
            let last = greg(gy, gm, gd);
            let date = from_fixed(last).expect("in range");
            assert_eq!((date.year, date.month), (year, month), "{gy}-{gm}-{gd}");
            assert_eq!(Ok(date.day), month_length(year, month), "{gy}-{gm}-{gd}");
            assert_eq!(from_fixed(last + 1).expect("in range").day, 1);
        }
        assert_eq!(year_type(1342), Some(YearType::ExtraMonth));
        assert_eq!(new_year(1343), Some(greg(1980, 12, 8)));
    }

    /// The rule settles every year of the range as Dupertuis's text does:
    /// no 25-then-5 pair and no dithy-5 leap-day year falls in it, and a
    /// 366-day solar year never has the avoman 126 his threshold and the
    /// Khmer one would read differently.
    #[test]
    fn the_range_holds_none_of_the_cases_the_source_leaves_open() {
        for year in FIRST_YEAR - 1..=LAST_YEAR + 1 {
            let tithi = new_year_tithi(year);
            assert!(!(tithi == 25 && new_year_tithi(year + 1) == 5), "{year}");
            assert!(!(is_solar_leap_year(year) && avoman(year) == 126), "{year}");
            if (FIRST_YEAR..=LAST_YEAR).contains(&year) {
                assert!(!(tithi == 5 && suryayatra_has_leap_day(year)), "{year}");
            }
        }
        // The two that bound it: 1299, whose next New Year Dupertuis puts on
        // 15 ຂຶ້ນ where the rule gives 1 ແຮມ, and 1402.
        for bound in [1299, 1402] {
            assert_eq!(new_year_tithi(bound), 5);
            assert!(suryayatra_has_leap_day(bound));
        }
    }

    #[test]
    fn month_lengths_follow_the_year_type() {
        for year in FIRST_YEAR..=LAST_YEAR {
            let kind = year_type(year).expect("in range");
            let total: i64 = months_of(kind)
                .map(|(month, _)| i64::from(month_length(year, month).expect("has it")))
                .sum();
            assert_eq!(total, kind.days(), "{year}");
            assert_eq!(LaoCalendar.is_leap_year(year), Ok(kind != YearType::Normal));
        }
    }

    #[test]
    fn every_day_of_the_range_round_trips() {
        let calendar = LaoCalendar;
        let mut rd = earliest();
        while rd <= latest() {
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd));
            let fields = calendar.to_fields(date).expect("describable");
            assert_eq!(calendar.from_fields(&fields), Ok(date));
            rd = rd + 1;
        }
    }

    #[test]
    fn the_range_ends_where_the_rule_is_carried() {
        assert_eq!(earliest(), greg(1938, 11, 23));
        assert_eq!(latest(), greg(2039, 11, 15));
        assert_eq!(from_fixed(earliest() - 1), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            from_fixed(latest() + 1),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(new_year(FIRST_YEAR - 1), None);
        assert_eq!(new_year(LAST_YEAR + 1), None);
        assert_eq!(
            month_length(LAST_YEAR + 1, Month::regular(1)),
            Err(CalendarError::YearOutOfRange)
        );
        for year in [FIRST_YEAR - 1, LAST_YEAR + 1] {
            assert_eq!(
                to_fixed(LaoDate::new(year, Month::regular(1), 1)),
                Err(CalendarError::YearOutOfRange),
                "{year}"
            );
        }
    }

    /// Dupertuis's "under 137, under 126" and the shared Khmer bounds,
    /// 137 or less and 126 or less with the 137-then-0 exception, call for
    /// a leap day in the same years of the range.
    #[test]
    fn dupertuis_s_leap_day_bounds_agree_with_the_shared_rule() {
        for year in FIRST_YEAR..=LAST_YEAR {
            let bound = if is_solar_leap_year(year) { 126 } else { 137 };
            assert_eq!(
                avoman(year) < bound,
                suryayatra_has_leap_day(year),
                "{year}"
            );
        }
    }

    #[test]
    fn impossible_dates_are_refused() {
        let normal = (FIRST_YEAR..=LAST_YEAR)
            .find(|year| year_type(*year) == Some(YearType::Normal))
            .expect("a normal year");
        assert_eq!(
            to_fixed(LaoDate::new(normal, Month::leap(8), 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(LaoDate::new(normal, Month::regular(7), 30)),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(LaoDate::new(normal, Month::regular(13), 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(LaoDate::new(normal, Month::regular(1), 0)),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn a_date_reads_as_the_glossary_writes_it() {
        let date = from_fixed(greg(1981, 4, 15)).expect("in range");
        assert_eq!(date.to_string(), "ເດືອນຫ້າ ຂຶ້ນ ໑໑ ຄ່ຳ ປີ ໑໓໔໓");
        let waning = from_fixed(greg(1939, 4, 16)).expect("in range");
        assert_eq!(waning.to_string(), "ເດືອນຫ້າ ແຮມ ໑໒ ຄ່ຳ ປີ ໑໓໐໑");
        let fields = LaoCalendar.to_fields(waning).expect("describable");
        assert_eq!(fields.extra.get("waning"), Some(1));
        assert_eq!(fields.extra.get("fortnight-day"), Some(12));
    }
}
