//! The Zoroastrian calendars: Qadimi, Shahanshahi and Fasli.
//!
//! One shape, three reckonings. Every Zoroastrian year is twelve months of
//! thirty days and then the five *Gatha* days, each day of the month named
//! for a divinity ([`DAY_NAMES`]) and each Gatha day for one of Zoroaster's
//! five hymns ([`GATHA_DAYS`]); the years count from the accession of
//! Yazdegerd III, the last Sasanian king, on 16 June 632 in the Julian
//! calendar, and are written with the suffix Y.Z. ([`ERA`]). Where the three
//! differ is in what they do about the quarter-day the 365-day year leaves
//! over, and that difference is the whole reason there are three:
//!
//! | Reckoning | Rule | Nowruz of 1370 Y.Z. |
//! | --- | --- | --- |
//! | [`Reckoning::Qadimi`] | Nothing. A wandering year of 365 days from the epoch, the calendar as it continued in Iran | 22 July 2000 |
//! | [`Reckoning::Shahanshahi`] | Nothing since one embolismic month, *Aspandarmad vahizak*, that the Parsis of India inserted in the 1120s; thirty days behind the Qadimi ever after | 21 August 2000 |
//! | [`Reckoning::Fasli`] | A sixth epagomenal day, *Avardad Sal Gah*, after the Gatha days of the years that end in a Gregorian leap year, so that 1 Fravardin is always 21 March | 21 March 2000 |
//!
//! The Qadimi and Shahanshahi are the calendars of the two Parsi factions
//! of the eighteenth century — *Kadmi*, "ancient", for those who in 1745
//! adopted the reckoning a visiting Iranian priest had shown them to be a
//! month ahead of their own, and *Shahanshahi*, "imperial", for the
//! majority who kept what they had — and both drift through the seasons at
//! a day every four years. The Fasli, "seasonal", is the 1906 proposal of a
//! Bombay society that tied the year to the equinox with a leap day; it won
//! little support in India and much in Iran, where the *Bastani* observance
//! that grew from it is kept on the civil Solar Hijri calendar, which is
//! [`crate::persian`] rather than this module.
//!
//! # The rules, exactly
//!
//! Nowruz, 1 Fravardin, of year *Y* Y.Z. falls on Julian Day Number
//! 1952063 + (*Y* − 1) × 365 by the Qadimi reckoning and
//! 1952093 + (*Y* − 1) × 365 by the Shahanshahi, the second holding for
//! every year from the intercalation on: 498 Y.Z. began on 12 February
//! 1129 (Julian) by the one and 14 March by the other. The Shahanshahi
//! calendar therefore converts nothing before 498 Y.Z., the first year the
//! source states it for; before the intercalation the Parsis kept the
//! Qadimi count, and that is the calendar to ask.
//!
//! The Fasli year *Y* begins on 21 March of Gregorian year *Y* + 630 and
//! has 366 days when Gregorian year *Y* + 631 is a leap year, the leap day
//! being the sixth day of the thirteenth "month": 1381 Y.Z. ran from
//! 21 March 2011 to *Avardad Sal Gah* on 20 March 2012, and 1379 Y.Z. ended
//! with the fifth Gatha day on 20 March 2010. This is the calendar as the
//! Parsi Fasli community prints it; it is not tied to the astronomical
//! equinox, which is why 1 Fravardin and the Iranian Nowruz part company in
//! the Gregorian leap years from 1996 to 2092, when the equinox falls on
//! 20 March in Tehran.
//!
//! # What this module is not
//!
//! It is not the *Bastani* calendar of Iranian Zoroastrians, which follows
//! the civil calendar's astronomical rule and is [`crate::persian`] with
//! Zoroastrian names; it does not carry the Zoroastrian Religious Era of
//! 1990, except as the arithmetic [`ZoroastrianDate::zre_year`] states; and
//! it says nothing of the *Denkard*'s intercalation of a month every 120
//! years, which no community has practised since the 1120s.
//!
//! The calendars, the split of 1745, the Fasli proposal and the era are
//! written up in `docs/systems/zoroastrian.md` in the repository.
//!
//! # Sources
//!
//! * Wikipedia, "Zoroastrian calendar", retrieved 2026-09-22 and
//!   2026-09-26 (`wikipedia-zoroastrian-calendar`): the epoch, the two
//!   Julian Day Number formulas, the 1129 and 2000 dates, the day and month
//!   dedications, the Zarathushtrian Religious Era of 1990 counted from the
//!   March equinox of 1738 BCE, and the history above.
//! * Antonio Panaino, "Calendars iv. Other modern calendars",
//!   *Encyclopaedia Iranica* IV/6–7 (1990) (`panaino1990`), read
//!   2026-09-26 in the Wayback Machine's copy of 5 September 2026: the
//!   Parsi split of 17 June 1745, or 1746 by Boyce and Hinnells, *qadīm*
//!   against *rasmī* or *Shenshai*, and the Fasli of 1906.
//! * Rohinton Erach Kadva, *Compendium of Fasli Zoroastrian Calendars 1379
//!   AY through 1400 AY*, Bangalore, 2009
//!   (zoroastrian.ru/files/eng/zoroastrian-calendars-1379-ay-1400-ay-fasli.pdf,
//!   retrieved 2026-09-22): the Fasli tables that the leap rule and the
//!   tests are read from, and the present-day forms of the day, Gatha and
//!   month names, which are the forms used here.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, gregorian};

/// The era code of the Yazdegerdi era, written Y.Z. (and also A.Y.) after
/// a year.
pub const ERA: &str = "yz";

/// How far the Zoroastrian Religious Era of 1990 runs ahead of the
/// Yazdegerdi era: 1370 Y.Z. is 3738 ZRE, the year that began at the March
/// equinox of 2000 (`wikipedia-zoroastrian-calendar`). The ZRE turns at the
/// equinox, so the pairing is nearly exact for the Fasli year and a pairing
/// of year numbers only for the wandering ones.
pub const ZRE_OFFSET: i64 = 2_368;

/// The first day of Fasli 1276, 21 March 1906, the year of the Bombay
/// society's proposal.
pub const FASLI_PROPOSED: Rd = match gregorian::to_fixed(1906, 3, 21) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Where the periods of use come from.
pub const USAGE_SOURCE: &str = "Wikipedia, \"Zoroastrian calendar\" [wikipedia-zoroastrian-calendar]: the wandering \
    year from the accession of Yazdegerd III, 16 June 632 Julian, continued in Iran and \
    adopted as Kadmi by some Parsis in 1745 [panaino1990]; the Shahanshahi a month behind it \
    from 498 Y.Z., 1129; the Fasli proposed by a Bombay society in 1906, the year only, so \
    its first Nowruz is taken";

/// The fixed day of 1 Fravardin 1 Y.Z. by the Qadimi reckoning: 16 June
/// 632 in the Julian calendar, Julian Day Number 1 952 063.
pub const QADIMI_EPOCH: Rd = Rd(230_638);

/// The day the Shahanshahi formula counts from: thirty days after
/// [`QADIMI_EPOCH`], Julian Day Number 1 952 093. Not a day any Parsi kept a
/// calendar on — the formula holds from 498 Y.Z., not from year one.
pub const SHAHANSHAHI_EPOCH: Rd = Rd(230_668);

/// The first Shahanshahi year the source states: 498 Y.Z., which began on
/// 14 March 1129 (Julian) by that reckoning, after the intercalation.
pub const SHAHANSHAHI_MIN_YEAR: i64 = 498;

/// The Gregorian year in which Fasli year 1 would have begun, less one:
/// Fasli year *Y* begins on 21 March of Gregorian year *Y* + 630.
pub const FASLI_GREGORIAN_OFFSET: i64 = 630;

/// The earliest year the Qadimi and Fasli reckonings convert.
pub const MIN_YEAR: i64 = 1;

/// The latest year any reckoning converts.
pub const MAX_YEAR: i64 = 99_999;

/// The twelve months and, thirteenth, the Gatha days, in the present-day
/// Parsi forms the *Compendium* prints them in. Wikipedia's normalised
/// forms (Farvardin, Shehrevar, Mehr, Aban, Azar, Asfand) and the Persian
/// script of the Iranian community are a locale's and belong to `hc-i18n`.
pub const MONTHS: [&str; 13] = [
    "Fravardin",
    "Ardibehesht",
    "Khordad",
    "Tir",
    "Amardad",
    "Shahrewar",
    "Meher",
    "Avan",
    "Adar",
    "Dae",
    "Bahman",
    "Aspandard",
    "Gatha",
];

/// The thirty day names, *roz*, in the same forms. The first, eighth,
/// fifteenth and twenty-third are the Creator's; the day whose name is its
/// month's name is that month's name-day feast — Meher 16 is Mehregan.
pub const DAY_NAMES: [&str; 30] = [
    "Hormazd",
    "Bahman",
    "Ardibehesht",
    "Shahrewar",
    "Aspandard",
    "Khordad",
    "Amardad",
    "Dai-pa-Adar",
    "Adar",
    "Avan",
    "Khorshed",
    "Mohor",
    "Tir",
    "Gosh",
    "Dai-pa-Meher",
    "Meher",
    "Srosh",
    "Rashne",
    "Fravardin",
    "Behram",
    "Ram",
    "Govad",
    "Dai-pa-Din",
    "Din",
    "Ashishvangh",
    "Ashtad",
    "Asman",
    "Zamyad",
    "Mareshpand",
    "Aneran",
];

/// The five Gatha days, named for the five Gathas of the *Yasna*.
pub const GATHA_DAYS: [&str; 5] = [
    "Ahunavaiti",
    "Ushtavaiti",
    "Spentamainyush",
    "Vohukhshathra",
    "Vahishtoishti",
];

/// The Fasli leap day, the sixth day after the twelfth month, kept as a
/// repetition of the fifth Gatha day.
pub const LEAP_DAY: &str = "Avardad Sal Gah";

/// Which of the three reckonings a calendar follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reckoning {
    /// The wandering year continued in Iran; *Kadmi* among the Parsis who
    /// adopted it in 1745.
    Qadimi,
    /// The wandering year of the Parsi majority, thirty days behind the
    /// Qadimi since the 1120s.
    Shahanshahi,
    /// The seasonal year of 1906: 1 Fravardin on 21 March, a leap day with
    /// the Gregorian calendar.
    Fasli,
}

impl Reckoning {
    /// The registry identifier of this reckoning's calendar.
    #[must_use]
    pub const fn id(self) -> CalendarId {
        match self {
            Self::Qadimi => CalendarId("zoroastrian-qadimi"),
            Self::Shahanshahi => CalendarId("zoroastrian-shahanshahi"),
            Self::Fasli => CalendarId("zoroastrian-fasli"),
        }
    }

    /// The English name of this reckoning's calendar.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Qadimi => "Zoroastrian (Qadimi)",
            Self::Shahanshahi => "Zoroastrian (Shahanshahi)",
            Self::Fasli => "Zoroastrian (Fasli)",
        }
    }

    /// The languages this reckoning's sources are written in: Persian for
    /// the calendar as kept in Iran, Gujarati for the Parsi reckonings of
    /// India, and both for the Fasli, which both communities adopted.
    #[must_use]
    pub const fn native_locales(self) -> &'static [&'static str] {
        match self {
            Self::Qadimi => &["fa", "gu"],
            Self::Shahanshahi => &["gu", "fa"],
            Self::Fasli => &["fa", "gu"],
        }
    }

    /// The earliest year this reckoning converts.
    #[must_use]
    pub const fn min_year(self) -> i64 {
        match self {
            Self::Shahanshahi => SHAHANSHAHI_MIN_YEAR,
            Self::Qadimi | Self::Fasli => MIN_YEAR,
        }
    }

    /// Whether `year` has the leap day: never in the wandering reckonings,
    /// and in the Fasli when the Gregorian year the Zoroastrian year ends
    /// in is a leap year.
    #[must_use]
    pub const fn is_leap_year(self, year: i64) -> bool {
        match self {
            Self::Qadimi | Self::Shahanshahi => false,
            Self::Fasli => gregorian::is_leap_year(year + FASLI_GREGORIAN_OFFSET + 1),
        }
    }

    /// The number of days in `month` of `year`, or `None` when `month` is
    /// not in `1..=13`; the thirteenth has five days, or six in a Fasli leap
    /// year.
    #[must_use]
    pub const fn days_in_month(self, year: i64, month: u8) -> Option<u8> {
        match common::wandering_days_in_month(month) {
            Some(5) if self.is_leap_year(year) => Some(6),
            length => length,
        }
    }

    /// The number of days in `year`.
    #[must_use]
    pub const fn days_in_year(self, year: i64) -> u16 {
        if self.is_leap_year(year) { 366 } else { 365 }
    }

    /// The earliest fixed day this reckoning converts.
    #[must_use]
    pub const fn earliest(self) -> Rd {
        match self.to_fixed(self.min_year(), 1, 1) {
            Ok(rd) => rd,
            // Every reckoning's first year is a date it converts.
            Err(_) => Rd(i64::MIN),
        }
    }

    /// The latest fixed day this reckoning converts.
    #[must_use]
    pub const fn latest(self) -> Rd {
        match self.new_year_unchecked(MAX_YEAR + 1) {
            Ok(Rd(next_new_year)) => Rd(next_new_year - 1),
            // The year after the last is a date the arithmetic reaches.
            Err(_) => Rd(i64::MAX),
        }
    }

    /// The fixed day of 1 Fravardin of `year` — Nowruz — without the range
    /// check, so that [`Self::latest`] can ask for the year after the last.
    const fn new_year_unchecked(self, year: i64) -> CalendarResult<Rd> {
        match self {
            Self::Qadimi => Ok(Rd(common::wandering_to_fixed(QADIMI_EPOCH.0, year, 1, 1))),
            Self::Shahanshahi => Ok(Rd(common::wandering_to_fixed(
                SHAHANSHAHI_EPOCH.0,
                year,
                1,
                1,
            ))),
            Self::Fasli => gregorian::to_fixed(year + FASLI_GREGORIAN_OFFSET, 3, 21),
        }
    }

    /// The fixed day of a date in this reckoning.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`],
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
    pub const fn to_fixed(self, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        if year < self.min_year() || year > MAX_YEAR {
            return Err(CalendarError::YearOutOfRange);
        }
        if let Err(error) = common::check_day(day, self.days_in_month(year, month)) {
            return Err(error);
        }
        match self.new_year_unchecked(year) {
            Err(error) => Err(error),
            Ok(Rd(new_year)) => Ok(Rd(new_year + (month as i64 - 1) * 30 + day as i64 - 1)),
        }
    }

    /// The year, month and day of a fixed day in this reckoning.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside
    /// [`Self::earliest`]..=[`Self::latest`].
    pub const fn from_fixed(self, rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        if rd.0 < self.earliest().0 {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd.0 > self.latest().0 {
            return Err(CalendarError::AfterSupportedRange);
        }
        match self {
            Self::Qadimi => Ok(common::wandering_from_fixed(QADIMI_EPOCH.0, rd.0)),
            Self::Shahanshahi => Ok(common::wandering_from_fixed(SHAHANSHAHI_EPOCH.0, rd.0)),
            Self::Fasli => {
                let (gregorian_year, gregorian_month, gregorian_day) =
                    match gregorian::from_fixed(rd) {
                        Ok(date) => date,
                        Err(error) => return Err(error),
                    };
                let year = if gregorian_month > 3 || (gregorian_month == 3 && gregorian_day >= 21) {
                    gregorian_year - FASLI_GREGORIAN_OFFSET
                } else {
                    gregorian_year - FASLI_GREGORIAN_OFFSET - 1
                };
                let new_year = match self.new_year_unchecked(year) {
                    Ok(Rd(new_year)) => new_year,
                    Err(error) => return Err(error),
                };
                let ordinal = rd.0 - new_year;
                let month = if ordinal >= 360 {
                    13
                } else {
                    (ordinal / 30 + 1) as u8
                };
                let day = (ordinal - (month as i64 - 1) * 30 + 1) as u8;
                Ok((year, month, day))
            }
        }
    }
}

/// A Zoroastrian date, in whichever reckoning the calendar that produced it
/// follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZoroastrianDate {
    /// The year of the Yazdegerdi era, counting from 1.
    pub year: i64,
    /// The month, 1 through 13; month 13 holds the Gatha days and, in a
    /// Fasli leap year, the leap day.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl ZoroastrianDate {
    /// A validated date in `reckoning`.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist in that
    /// reckoning.
    pub const fn new(reckoning: Reckoning, year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        match reckoning.to_fixed(year, month, day) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self { year, month, day }),
        }
    }

    /// Whether this date is one of the Gatha days, or the leap day.
    #[must_use]
    pub const fn is_gatha_day(self) -> bool {
        self.month == 13
    }

    /// Whether this date is the Fasli leap day, *Avardad Sal Gah*.
    #[must_use]
    pub const fn is_leap_day(self) -> bool {
        self.month == 13 && self.day == 6
    }

    /// The name of the month, or "Gatha" for the thirteenth.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }

    /// The name of the day: its *roz* in a month, its Gatha among the Gatha
    /// days, or [`LEAP_DAY`].
    #[must_use]
    pub const fn day_name(self) -> &'static str {
        if self.month != 13 {
            DAY_NAMES[self.day as usize - 1]
        } else if self.day == 6 {
            LEAP_DAY
        } else {
            GATHA_DAYS[self.day as usize - 1]
        }
    }

    /// Whether this is the name-day feast of its month: the day whose name
    /// is the month's name, such as Meher 16, Mehregan.
    #[must_use]
    pub fn is_name_day_feast(self) -> bool {
        self.month != 13 && self.day_name() == self.month_name()
    }

    /// The same year counted in the Zoroastrian Religious Era of 1990.
    #[must_use]
    pub const fn zre_year(self) -> i64 {
        self.year + ZRE_OFFSET
    }
}

/// A Zoroastrian calendar in one of the three reckonings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZoroastrianCalendar {
    /// The reckoning this calendar follows.
    pub reckoning: Reckoning,
}

impl ZoroastrianCalendar {
    /// The Qadimi reckoning, `zoroastrian-qadimi`.
    pub const QADIMI: Self = Self {
        reckoning: Reckoning::Qadimi,
    };
    /// The Shahanshahi reckoning, `zoroastrian-shahanshahi`.
    pub const SHAHANSHAHI: Self = Self {
        reckoning: Reckoning::Shahanshahi,
    };
    /// The Fasli reckoning, `zoroastrian-fasli`.
    pub const FASLI: Self = Self {
        reckoning: Reckoning::Fasli,
    };

    /// All three, in the order of the module table.
    pub const ALL: [Self; 3] = [Self::QADIMI, Self::SHAHANSHAHI, Self::FASLI];
}

/// Thirteen named months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for ZoroastrianCalendar {
    type Date = ZoroastrianDate;

    /// Each reckoning from the day its own count begins: the Qadimi from the
    /// Yazdegerdi epoch, the Shahanshahi from 498 Y.Z., the Fasli from the
    /// Nowruz of 1906. All three are kept today.
    fn usage(&self) -> hc_calendar::Usage {
        match self.reckoning {
            Reckoning::Qadimi => hc_calendar::Usage::since(QADIMI_EPOCH, USAGE_SOURCE),
            Reckoning::Shahanshahi => {
                hc_calendar::Usage::since(Reckoning::Shahanshahi.earliest(), USAGE_SOURCE)
            }
            Reckoning::Fasli => hc_calendar::Usage::since(FASLI_PROPOSED, USAGE_SOURCE),
        }
    }

    /// Twelve months, the Gatha days as a thirteenth, and the seven-day
    /// week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(self.reckoning.is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.reckoning.id(),
            english_name: self.reckoning.english_name(),
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(self.reckoning.earliest()),
            latest: Some(self.reckoning.latest()),
            native_locales: self.reckoning.native_locales(),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.reckoning.to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = self.reckoning.from_fixed(rd)?;
        Ok(ZoroastrianDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(ERA))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        ZoroastrianDate::new(
            self.reckoning,
            fields.year,
            month.ordinal,
            fields.require_day()?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::julian;

    fn gregorian(rd: Rd) -> (i64, u8, u8) {
        gregorian::from_fixed(rd).unwrap()
    }

    #[test]
    fn the_qadimi_epoch_is_the_accession_of_yazdegerd() {
        // 16 June 632 in the Julian calendar, Julian Day Number 1952063.
        assert_eq!(Reckoning::Qadimi.to_fixed(1, 1, 1), Ok(QADIMI_EPOCH));
        assert_eq!(julian::from_fixed(QADIMI_EPOCH), Ok((632, 6, 16)));
        assert_eq!(QADIMI_EPOCH.to_julian_day_number(), 1_952_063);
        assert_eq!(SHAHANSHAHI_EPOCH.to_julian_day_number(), 1_952_093);
    }

    #[test]
    fn nowruz_of_1370_is_where_the_source_puts_it_in_each_reckoning() {
        assert_eq!(
            gregorian(Reckoning::Qadimi.to_fixed(1370, 1, 1).unwrap()),
            (2000, 7, 22)
        );
        assert_eq!(
            gregorian(Reckoning::Shahanshahi.to_fixed(1370, 1, 1).unwrap()),
            (2000, 8, 21)
        );
        assert_eq!(
            gregorian(Reckoning::Fasli.to_fixed(1370, 1, 1).unwrap()),
            (2000, 3, 21)
        );
        assert_eq!(
            ZoroastrianDate {
                year: 1370,
                month: 1,
                day: 1
            }
            .zre_year(),
            3738
        );
    }

    #[test]
    fn the_intercalation_set_the_two_wandering_years_a_month_apart() {
        // "498 YZ began on 12 February by the Qadimi reckoning, but 14 March
        // by the recently introduced Shahanshahi", in the Julian year 1129.
        assert_eq!(
            julian::from_fixed(Reckoning::Qadimi.to_fixed(498, 1, 1).unwrap()),
            Ok((1129, 2, 12))
        );
        assert_eq!(
            julian::from_fixed(Reckoning::Shahanshahi.to_fixed(498, 1, 1).unwrap()),
            Ok((1129, 3, 14))
        );
        for year in (498..3_000).step_by(7) {
            let qadimi = Reckoning::Qadimi.to_fixed(year, 1, 1).unwrap();
            let shahanshahi = Reckoning::Shahanshahi.to_fixed(year, 1, 1).unwrap();
            assert_eq!(shahanshahi.0 - qadimi.0, 30, "year {year}");
        }
        // Before the intercalation there was no Shahanshahi count to state.
        assert_eq!(
            Reckoning::Shahanshahi.to_fixed(497, 13, 5),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            Reckoning::Shahanshahi.from_fixed(Rd(Reckoning::Shahanshahi.earliest().0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
    }

    #[test]
    fn the_wandering_years_return_to_march_when_the_source_says() {
        // "the unrevised Qadimi calendar would eventually coincide with the
        // Fasli calendar in Gregorian year 2508, the Shahanshahi New Year
        // will next fall on 21 March in 2632."
        assert_eq!(
            gregorian(Reckoning::Qadimi.to_fixed(1878, 1, 1).unwrap()),
            (2508, 3, 21)
        );
        assert_eq!(
            gregorian(Reckoning::Shahanshahi.to_fixed(2002, 1, 1).unwrap()),
            (2632, 3, 21)
        );
        assert_eq!(
            Reckoning::Qadimi.to_fixed(1878, 1, 1),
            Reckoning::Fasli.to_fixed(1878, 1, 1)
        );
    }

    #[test]
    fn the_fasli_year_is_the_compendium_table() {
        // 1381 A.Y. (2011–2012): Bahman 1 on 15 January 2012, Aspandard 1 on
        // 14 February, Aneran on 14 March, the Gatha days 15–19 March and
        // Avardad Sal Gah on 20 March, the 366th day.
        let fasli = Reckoning::Fasli;
        assert_eq!(
            gregorian(fasli.to_fixed(1381, 1, 1).unwrap()),
            (2011, 3, 21)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(1381, 11, 1).unwrap()),
            (2012, 1, 15)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(1381, 12, 1).unwrap()),
            (2012, 2, 14)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(1381, 12, 30).unwrap()),
            (2012, 3, 14)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(1381, 13, 1).unwrap()),
            (2012, 3, 15)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(1381, 13, 5).unwrap()),
            (2012, 3, 19)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(1381, 13, 6).unwrap()),
            (2012, 3, 20)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(1382, 1, 1).unwrap()),
            (2012, 3, 21)
        );
        assert!(fasli.is_leap_year(1381));
        assert_eq!(fasli.days_in_year(1381), 366);
        assert_eq!(fasli.days_in_month(1381, 13), Some(6));
        assert!(
            ZoroastrianDate::new(fasli, 1381, 13, 6)
                .unwrap()
                .is_leap_day()
        );
        assert_eq!(
            ZoroastrianDate::new(fasli, 1381, 13, 6).unwrap().day_name(),
            LEAP_DAY
        );

        // 1379 A.Y. (2009–2010) is a common year: the fifth Gatha day on
        // 20 March 2010 is its last, and there is no sixth.
        assert_eq!(
            gregorian(fasli.to_fixed(1379, 1, 1).unwrap()),
            (2009, 3, 21)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(1379, 13, 1).unwrap()),
            (2010, 3, 16)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(1379, 13, 5).unwrap()),
            (2010, 3, 20)
        );
        assert_eq!(
            fasli.to_fixed(1379, 13, 6),
            Err(CalendarError::DayOutOfRange)
        );
        assert!(!fasli.is_leap_year(1379));
        assert_eq!(fasli.days_in_month(1379, 13), Some(5));

        // Nowruz never moves off 21 March, in the Gregorian century year
        // included.
        for year in (1..MAX_YEAR).step_by(97) {
            let (_, month, day) = gregorian(fasli.to_fixed(year, 1, 1).unwrap());
            assert_eq!((month, day), (3, 21), "year {year}");
        }
        assert!(!fasli.is_leap_year(2100 - FASLI_GREGORIAN_OFFSET - 1));
    }

    #[test]
    fn the_fasli_gahambars_fall_on_the_seasonal_dates_the_source_gives() {
        // Wikipedia, "Gahambars": Maidyozarem 30 April – 4 May,
        // Maidyoshahem 29 June – 3 July, Paitishahem 12–16 September,
        // Ayathrem 12–16 October, Maidyarem 31 December – 4 January,
        // Hamaspathmaidyem 16–20 March; the Compendium puts them on
        // Ardibehesht 11–15, Tir 11–15, Shahrewar 26–30, Meher 26–30,
        // Dae 16–20 and the Gatha days.
        let fasli = Reckoning::Fasli;
        let year = 1379;
        assert_eq!(
            gregorian(fasli.to_fixed(year, 2, 11).unwrap()),
            (2009, 4, 30)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(year, 4, 11).unwrap()),
            (2009, 6, 29)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(year, 6, 26).unwrap()),
            (2009, 9, 12)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(year, 7, 26).unwrap()),
            (2009, 10, 12)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(year, 10, 16).unwrap()),
            (2009, 12, 31)
        );
        assert_eq!(
            gregorian(fasli.to_fixed(year, 13, 1).unwrap()),
            (2010, 3, 16)
        );
    }

    #[test]
    fn the_day_names_intersect_the_month_names_on_the_feast_days() {
        let mehregan = ZoroastrianDate {
            year: 1380,
            month: 7,
            day: 16,
        };
        assert_eq!(mehregan.day_name(), "Meher");
        assert_eq!(mehregan.month_name(), "Meher");
        assert!(mehregan.is_name_day_feast());
        let tiragan = ZoroastrianDate {
            year: 1380,
            month: 4,
            day: 13,
        };
        assert!(tiragan.is_name_day_feast());
        let khordad_sal = ZoroastrianDate {
            year: 1380,
            month: 1,
            day: 6,
        };
        assert_eq!(khordad_sal.day_name(), "Khordad");
        assert!(!khordad_sal.is_name_day_feast());
        assert_eq!(
            ZoroastrianDate {
                year: 1380,
                month: 1,
                day: 1
            }
            .day_name(),
            "Hormazd"
        );
        assert_eq!(
            ZoroastrianDate {
                year: 1380,
                month: 12,
                day: 30
            }
            .day_name(),
            "Aneran"
        );
        let first_gatha = ZoroastrianDate {
            year: 1380,
            month: 13,
            day: 1,
        };
        assert_eq!(first_gatha.day_name(), "Ahunavaiti");
        assert!(first_gatha.is_gatha_day());
        assert!(!first_gatha.is_name_day_feast());
        // Every month has exactly one name-day feast except Dae, whose name
        // is the Creator's epithet and matches none of His four days.
        for month in 1..=12u8 {
            let feasts = (1..=30u8)
                .filter(|&day| {
                    ZoroastrianDate {
                        year: 1,
                        month,
                        day,
                    }
                    .is_name_day_feast()
                })
                .count();
            assert_eq!(feasts, usize::from(month != 10), "month {month}");
        }
    }

    #[test]
    fn every_calendar_round_trips_a_wide_range() {
        for calendar in ZoroastrianCalendar::ALL {
            let meta = calendar.meta();
            let (earliest, latest) = (meta.earliest.unwrap(), meta.latest.unwrap());
            for rd in (earliest.0..=earliest.0 + 3_000_000).step_by(37) {
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{} rd {rd}", meta.id);
                let fields = calendar.to_fields(date).unwrap();
                assert_eq!(fields.era, Some(ERA));
                assert_eq!(
                    calendar.from_fields(&fields),
                    Ok(date),
                    "{} rd {rd}",
                    meta.id
                );
            }
            for rd in latest.0 - 800..=latest.0 {
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{} rd {rd}", meta.id);
            }
            assert_eq!(
                calendar.from_fixed(Rd(earliest.0 - 1)),
                Err(CalendarError::BeforeEpoch)
            );
            assert_eq!(
                calendar.from_fixed(Rd(latest.0 + 1)),
                Err(CalendarError::AfterSupportedRange)
            );
            assert_eq!(
                calendar.to_fixed(ZoroastrianDate {
                    year: MAX_YEAR + 1,
                    month: 1,
                    day: 1
                }),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(meta.year_kind, YearKind::EpochForward);
        }
    }

    #[test]
    fn every_single_day_of_a_fasli_leap_year_and_its_neighbours_round_trips() {
        let calendar = ZoroastrianCalendar::FASLI;
        let start = Reckoning::Fasli.to_fixed(1380, 1, 1).unwrap().0;
        let end = Reckoning::Fasli.to_fixed(1383, 1, 1).unwrap().0;
        assert_eq!(end - start, 365 + 366 + 365);
        for rd in start..end {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            assert!(
                date.day
                    <= Reckoning::Fasli
                        .days_in_month(date.year, date.month)
                        .unwrap()
            );
        }
    }

    #[test]
    fn a_wrong_era_or_a_leap_month_is_refused() {
        assert_eq!(
            ZoroastrianCalendar::QADIMI.from_fields(&DateFields::ymd(1370, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            Reckoning::Qadimi.to_fixed(1370, 14, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            Reckoning::Qadimi.to_fixed(1370, 13, 6),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            Reckoning::Qadimi.to_fixed(0, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }
}
