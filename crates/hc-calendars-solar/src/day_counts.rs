//! Day counts other than the Julian Day: one origin, one number, no months.
//!
//! Every one of these exists because somebody needed a day number that fit
//! in the storage they had. The Julian Day Number passed two million in the
//! seventeenth century, so astronomy shortened it, then NASA shortened it
//! again, then CNES and CCSDS each picked their own epoch, then IBM picked
//! two more for business software. They are trivially inter-convertible and
//! constantly confused, which is exactly why each deserves a name.
//!
//! [`crate::julian_day`] keeps the Julian Day Number and the Modified
//! Julian Date, which have their own types and their own arithmetic. This
//! module is the rest of the family, all of the same shape: an epoch and a
//! number.
//!
//! # Epochs are dates here, not magic numbers
//!
//! Each count below is defined by the *calendar date* of its epoch and the
//! number that date carries, not by an offset from the Julian Day Number.
//! The offsets are the thing everyone transcribes wrongly; the dates are
//! what the defining documents state. The tests then check the classical
//! offsets — Lilian = JDN − 2 299 160, TJD = JDN − 2 440 001 — so both
//! forms have to agree.
//!
//! # A warning the family invites
//!
//! A day number is not an instant, and these do not all start their day at
//! the same time. The Reduced and Dublin counts inherit the Julian Day's
//! noon, and with it the Julian Day's naming: a day is the civil day on
//! whose noon it begins ([`DayNaming::ByStart`]), as the Dublin epoch
//! "1900 January 0.5" and the Reduced Julian Date's JD − 2 400 000 both
//! say. The rest begin at midnight. A conversion between two of them that
//! ignores that is wrong by half a day for half of each day.
//!
//! The Chronological Julian Day is the case that proves it: the same
//! integer as the Julian Day Number, on the day that begins at the
//! midnight before that number's noon, so it is a separate identifier
//! (policy.md §5) and not a setting of `julian-day`.
//!
//! # Vendor counts
//!
//! The Excel 1904 date system and the OLE Automation date are linear day
//! counts too, and are here with the ranges their vendor supports. The
//! Excel 1900 system is not linear — it counts a 29 February 1900 that
//! never was — so it is [`crate::spreadsheet`]'s, beside the OLE date's
//! fractional time; `docs/systems/spreadsheet-dates.md` describes all three.
//!
//! The statistical packages' days are here too: SAS dates and Stata's
//! `%td`, the same count from 1 January 1960 over two different ranges
//! and so two identifiers, and MATLAB's `datenum`, whose fractional form is
//! [`crate::spreadsheet::matlab_datenum`].
//! `docs/systems/statistical-software-dates.md` describes them, with the
//! datetimes that `hc_core::sas_stata` reads. Stata's weekly dates,
//! which name a day by a year, one of 52 weeks and a day of it, are
//! [`StataWeekCalendar`].

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, DayBoundary,
    DayNaming, Rd, YearKind,
};

use crate::gregorian;

/// A day number in one of the counts below.
///
/// Deliberately not one newtype per count: they are the same quantity in
/// different origins, and a caller who mixes them up is helped by the
/// calendar's identifier, not by a type they would have to name anyway.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DayNumber(pub i64);

/// How far a day count may stray from the Rata Die epoch before the
/// conversion is refused, matching [`crate::julian_day`].
const MAX_MAGNITUDE: i64 = 1 << 44;

/// A day count defined by an epoch date and the number that date carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayCount {
    id: CalendarId,
    english_name: &'static str,
    epoch: Rd,
    first_number: i64,
    boundary: DayBoundary,
    authority: &'static str,
    range: Option<(Rd, Rd)>,
}

impl DayCount {
    /// A count whose epoch is a Gregorian date.
    ///
    /// # Panics
    ///
    /// If the date does not exist. Every use below is a `const`, so that is
    /// a compile error at the definition rather than a surprise later.
    #[must_use]
    pub const fn from_gregorian(
        id: &'static str,
        english_name: &'static str,
        (year, month, day): (i64, u8, u8),
        first_number: i64,
        boundary: DayBoundary,
        authority: &'static str,
    ) -> Self {
        let epoch = match gregorian::to_fixed(year, month, day) {
            Ok(rd) => rd,
            Err(_) => panic!("a day count's epoch must be a real date"),
        };
        Self {
            id: CalendarId(id),
            english_name,
            epoch,
            first_number,
            boundary,
            authority,
            range: None,
        }
    }

    /// The same count, supported only from `first` to `last`, both
    /// Gregorian dates and both included: the range its defining vendor
    /// supports.
    ///
    /// # Panics
    ///
    /// If either date does not exist, which in a `const` is a compile
    /// error, as for [`Self::from_gregorian`].
    #[must_use]
    pub const fn between(self, first: (i64, u8, u8), last: (i64, u8, u8)) -> Self {
        let (Ok(first), Ok(last)) = (
            gregorian::to_fixed(first.0, first.1, first.2),
            gregorian::to_fixed(last.0, last.1, last.2),
        ) else {
            panic!("a day count's range must be real dates");
        };
        Self {
            range: Some((first, last)),
            ..self
        }
    }

    /// The first and last fixed days this count supports.
    #[must_use]
    pub const fn range(self) -> (Rd, Rd) {
        match self.range {
            Some(range) => range,
            None => (Rd(-MAX_MAGNITUDE), Rd(MAX_MAGNITUDE)),
        }
    }

    /// This count's identifier.
    #[must_use]
    pub const fn id(self) -> CalendarId {
        self.id
    }

    /// The fixed day this count's epoch falls on.
    #[must_use]
    pub const fn epoch(self) -> Rd {
        self.epoch
    }

    /// The number this count gives its epoch day.
    #[must_use]
    pub const fn first_number(self) -> i64 {
        self.first_number
    }

    /// Who defines this count.
    #[must_use]
    pub const fn authority(self) -> &'static str {
        self.authority
    }

    /// The day number of a fixed day.
    #[must_use]
    pub const fn number_of(self, rd: Rd) -> DayNumber {
        DayNumber(rd.0 - self.epoch.0 + self.first_number)
    }

    /// The fixed day of a day number.
    #[must_use]
    pub const fn day_of(self, number: DayNumber) -> Rd {
        Rd(number.0 - self.first_number + self.epoch.0)
    }
}

hc_core::catalogue! {
    type: DayCount,
    id: |count| count.id.0,
    provenance: |count| count.authority,
    tests: day_count_catalogue,

    /// Every count in this module.
    ///
    /// Generated together with the entries above, so a count cannot be
    /// defined and left off the list.
    pub const ALL;

    /// The count with this identifier.
    pub fn by_id;

    entries: {
    /// The Lilian date: day 1 is 15 October 1582, the first day of the
    /// Gregorian calendar.
    ///
    /// Named for Aloysius Lilius, who designed the reform. IBM defined the
    /// count in 1986 and uses it in `INTDATE(LILIAN)`.
    pub const LILIAN = DayCount::from_gregorian(
        "lilian",
        "Lilian date",
        (1582, 10, 15),
        1,
        DayBoundary::Midnight,
        "Bruce G. Ohms, IBM Systems Journal 25(2) (1986)",
    );

    /// The ANSI date: day 1 is 1 January 1601.
    ///
    /// The other IBM count, `INTDATE(ANSI)`. The epoch is the start of the
    /// Gregorian 400-year cycle containing the reform, which is also why
    /// Windows `FILETIME` counts from the same day.
    pub const ANSI = DayCount::from_gregorian(
        "ansi-date",
        "ANSI date",
        (1601, 1, 1),
        1,
        DayBoundary::Midnight,
        "IBM Enterprise COBOL for z/OS 6.3, compiler option INTDATE: INTDATE(ANSI) uses \
         the 85 COBOL Standard starting date, day 1 = Jan 1, 1601; retrieved 2026-09-26 \
         [ibm-cobol-intdate]",
    );

    /// The Dublin Julian Date: day 0 begins at noon on 31 December 1899.
    ///
    /// Introduced by the IAU at its Dublin meeting of 1955, as Wikipedia,
    /// "Julian day", retrieved 2026-09-26, reports it (`wikipedia-julian-day`;
    /// the IAU's transactions not read), with its epoch written
    /// "1900 January 0.5" —
    /// which is the trap. January 0 is 31 December of the year before, and the
    /// .5 is the Julian Day's noon. Writing the epoch as 1 January 1900 puts
    /// every Dublin date a day out; the offset test below holds it to
    /// JD − 2 415 020.
    pub const DUBLIN = DayCount::from_gregorian(
        "dublin-julian-day",
        "Dublin Julian Date",
        (1899, 12, 31),
        0,
        DayBoundary::Noon(DayNaming::ByStart),
        "IAU General Assembly, Dublin (1955), not read; JD - 2415020 as Wikipedia, \
         \"Julian day\", gives it [wikipedia-julian-day]",
    );

    /// The Reduced Julian Date: day 0 begins at noon on 16 November 1858.
    ///
    /// JD − 2 400 000, so it keeps the Julian Day's noon and differs from the
    /// Modified Julian Date by exactly half a day — which is the whole reason
    /// the two are confused.
    pub const REDUCED = DayCount::from_gregorian(
        "reduced-julian-day",
        "Reduced Julian Date",
        (1858, 11, 16),
        0,
        DayBoundary::Noon(DayNaming::ByStart),
        "JD - 2400000, the count from 12:00 on 16 November 1858, as Wikipedia, \
         \"Julian day\", tabulates it, citing Hopkins 2013, not read [wikipedia-julian-day]",
    );

    /// The Truncated Julian Date: day 0 is 24 May 1968.
    ///
    /// NASA defined it in 1979 for spacecraft telemetry, where four digits were
    /// all that fit: A. R. Chi, *A Grouped Binary Time Code for Telemetry and
    /// Space Applications*, NASA Technical Memorandum 80606, Goddard Space
    /// Flight Center, December 1979 (`chi1979`), read 2026-09-26: TJD "is
    /// arbitrarily chosen to begin from 0 at midnight May 24" 1968, JDN
    /// 2 440 000, and recycles after 9999.
    pub const TRUNCATED = DayCount::from_gregorian(
        "truncated-julian-day",
        "Truncated Julian Date",
        (1968, 5, 24),
        0,
        DayBoundary::Midnight,
        "A. R. Chi, NASA Technical Memorandum 80606, Goddard Space Flight Center, December \
         1979 [chi1979]",
    );

    /// The CNES Julian Date: day 0 is 1 January 1950.
    ///
    /// The French space agency's count, used throughout its mission products.
    /// The epoch is as Wikipedia, "Julian day", tabulates it, JD − 2 433 282.5,
    /// citing P.-M. Theveny, *The TPtime Handbook* (2001), not read; no CNES
    /// document was read.
    pub const CNES = DayCount::from_gregorian(
        "cnes-julian-day",
        "CNES Julian Date",
        (1950, 1, 1),
        0,
        DayBoundary::Midnight,
        "Centre national d'etudes spatiales; JD - 2433282.5 as Wikipedia, \"Julian day\", \
         gives it, citing Theveny 2001, not read [wikipedia-julian-day]",
    );

    /// The CCSDS day count: day 0 is 1 January 1958.
    ///
    /// The epoch of CCSDS Day Segmented time codes, and also the epoch TAI was
    /// aligned to UT2 at.
    pub const CCSDS = DayCount::from_gregorian(
        "ccsds-day",
        "CCSDS day count",
        (1958, 1, 1),
        0,
        DayBoundary::Midnight,
        "CCSDS 301.0-B-4, Time Code Formats, not read; JD - 2436204.5 as Wikipedia, \
         \"Julian day\", gives it [wikipedia-julian-day]",
    );

    /// The Chronological Julian Day: day 0 is the day that began at
    /// midnight on 1 January 4713 BCE, proleptic Julian.
    ///
    /// Peter Meyer, "Julian Day Numbers", Hermetic Systems, read 2026-09-26
    /// (`meyer-jdn`): "a count of nychthemerons, assumed to begin at
    /// midnight GMT". Chronological day 2 452 952 is 8 November 2003 from
    /// midnight, the day whose noon begins Julian Day 2 452 952, so the
    /// number is the same and the interval is not. Meyer defines the count
    /// at Greenwich and a local chronological date by the local midnight;
    /// as a day count here it names a civil day in the caller's zone, as the
    /// Modified Julian Date does.
    pub const CHRONOLOGICAL = DayCount::from_gregorian(
        "chronological-julian-day",
        "Chronological Julian Day",
        (-4713, 11, 24),
        0,
        DayBoundary::Midnight,
        "Peter Meyer, Julian Day Numbers, Hermetic Systems, hermetic.ch/cal_stud/jdn.htm, \
         retrieved 2026-09-26 [meyer-jdn]",
    );

    /// MJD2000: day 0 is 1 January 2000, JD − 2 451 544.5.
    ///
    /// The European Space Agency's count, as Wikipedia, "Julian day",
    /// tabulates it (`wikipedia-julian-day`); no ESA document was read.
    pub const MJD2000 = DayCount::from_gregorian(
        "modified-julian-day-2000",
        "Modified Julian Day 2000 (ESA)",
        (2000, 1, 1),
        0,
        DayBoundary::Midnight,
        "European Space Agency, not read; JD - 2451544.5 as Wikipedia, \"Julian day\", \
         gives it [wikipedia-julian-day]",
    );

    /// The Excel 1904 date system: serial 0 is 1 January 1904, supported
    /// to 31 December 9999.
    ///
    /// Microsoft Support, "Date systems in Excel", read 2026-09-26
    /// (`ms-excel-date-systems`): the 1900 system's serial for a day is
    /// always 1 462 more than the 1904 system's, "four years and one day
    /// (including one leap day)"; 5 July 2011 is 40 729 in the one and
    /// 39 267 in the other. Excel's own range ends on 9999-12-31.
    pub const EXCEL_1904 = DayCount::from_gregorian(
        "excel-1904",
        "Microsoft Excel 1904 date system",
        (1904, 1, 1),
        0,
        DayBoundary::Midnight,
        "Microsoft Support, Date systems in Excel, retrieved 2026-09-26 \
         [ms-excel-date-systems]",
    )
    .between((1904, 1, 1), (9999, 12, 31));

    /// The OLE Automation date's day: day 0 is 30 December 1899, supported
    /// from 1 January 100 to 31 December 9999.
    ///
    /// Microsoft Learn, `DateTime.ToOADate`, read 2026-09-26
    /// (`ms-tooadate`): "the number of days before or after midnight,
    /// 30 December 1899". The fractional time of day, which a negative
    /// value reads as a magnitude, is [`crate::spreadsheet::ole_automation`].
    pub const OLE_AUTOMATION = DayCount::from_gregorian(
        "ole-automation-date",
        "OLE Automation date",
        (1899, 12, 30),
        0,
        DayBoundary::Midnight,
        "Microsoft Learn, DateTime.ToOADate, retrieved 2026-09-26 [ms-tooadate]",
    )
    .between((100, 1, 1), (9999, 12, 31));

    /// The SAS date: day 0 is 1 January 1960, supported from 1 January
    /// 1582 to 31 December 19 900.
    ///
    /// SAS 9.3 Language Reference: Concepts, "About SAS Date, Time, and
    /// Datetime Values", read 2026-09-26 (`sas-lrcon-dates`): "the number
    /// of days between January 1, 1960, and a specified date. SAS can
    /// perform calculations on dates ranging from A.D. 1582 to A.D.
    /// 19,900"; its example `'26oct02'd` is 15 639. The range is read as the
    /// whole of those years.
    pub const SAS = DayCount::from_gregorian(
        "sas-date",
        "SAS date value",
        (1960, 1, 1),
        0,
        DayBoundary::Midnight,
        "SAS 9.3 Language Reference: Concepts, About SAS Date, Time, and Datetime Values, \
         retrieved 2026-09-26 [sas-lrcon-dates]",
    )
    .between((1582, 1, 1), (19900, 12, 31));

    /// Stata's `%td`: day 0 is 01jan1960, supported from 01jan0100 to
    /// 31dec9999.
    ///
    /// The SAS count over Stata's range, and so its own identifier. Stata,
    /// `help datetime`, read 2026-09-26 (`stata-help-datetime`): "days since
    /// 01jan1960 (01jan1960 = 0)", 20jan2010 being 18 282; `help
    /// datetime_functions` (`stata-help-datetime-functions`): "e_d dates
    /// 01jan0100 to 31dec9999 (integers -679,350 to 2,936,549)".
    pub const STATA = DayCount::from_gregorian(
        "stata-date",
        "Stata daily date (%td)",
        (1960, 1, 1),
        0,
        DayBoundary::Midnight,
        "Stata, help datetime and help datetime_functions (StataNow 19), retrieved \
         2026-09-26 [stata-help-datetime, stata-help-datetime-functions]",
    )
    .between((100, 1, 1), (9999, 12, 31));

    /// MATLAB's `datenum`: day 1 is 1 January 0000 of the proleptic
    /// Gregorian calendar, so day 0 is "January 0, 0000".
    ///
    /// MathWorks, `datenum` reference page, marked "Not recommended; use
    /// datetime or duration", read 2026-09-26 through a fetching tool, the
    /// site refusing direct requests (`mathworks-datenum`): "the whole and
    /// fractional number of days from a fixed, preset date (January 0,
    /// 0000) in the proleptic ISO calendar". That is the Rata Die plus 366,
    /// the days of the leap year 0. Its examples: `datenum(2001,12,19)` is
    /// 731 204 and 19 May 2001 is 730 990. MathWorks states no range.
    pub const MATLAB = DayCount::from_gregorian(
        "matlab-datenum",
        "MATLAB serial date number (datenum)",
        (0, 1, 1),
        1,
        DayBoundary::Midnight,
        "MathWorks, MATLAB datenum reference page, retrieved 2026-09-26 [mathworks-datenum]",
    );
    }
}

/// A calendar over one of the counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayCountCalendar(pub DayCount);

impl Calendar for DayCountCalendar {
    type Date = DayNumber;

    /// A day count names nothing: it has no months and no week, only a number.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        &[]
    }

    /// A day count has no year: the `year` field carries the count itself.
    fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
        Err(CalendarError::UnsupportedField("year"))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.0.id,
            english_name: self.0.english_name,
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(self.0.range().0),
            latest: Some(self.0.range().1),
            native_locales: &[],
        }
    }

    fn day_boundary(&self) -> DayBoundary {
        self.0.boundary
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        let rd = self.0.day_of(date);
        self.meta().check_range(rd)?;
        Ok(rd)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        Ok(self.0.number_of(rd))
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        // The count goes in `year` for the same reason as in
        // `crate::julian_day`: it is the only unbounded signed field a
        // generic consumer is guaranteed to have.
        DateFields::new(date.0).with_extra(
            "julian-day-number",
            self.0.day_of(date).to_julian_day_number(),
        )
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.day.is_some() {
            return Err(CalendarError::UnsupportedField("day"));
        }
        Ok(DayNumber(fields.year))
    }
}

/// Days in every Stata week but the year's last.
const STATA_WEEK_DAYS: i64 = 7;

/// Weeks in a Stata year.
pub const STATA_WEEKS_PER_YEAR: u8 = 52;

/// A day named as Stata's weekly dates name it: the year, the week, 1 to
/// 52, and the day of that week, 1 to 7, or to 8 or 9 in week 52.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StataWeekDate {
    /// The Gregorian year.
    pub year: i64,
    /// The week, 1 to 52.
    pub week: u8,
    /// The day of the week counted from the week's first day, 1 to 7, and
    /// to 8 or 9 in week 52.
    pub day: u8,
}

impl StataWeekDate {
    /// The days in this date's week: 7, or what remains of the year in
    /// week 52.
    #[must_use]
    pub const fn days_in_week(year: i64, week: u8) -> u8 {
        if week == STATA_WEEKS_PER_YEAR {
            // 365 or 366 less the 357 days of weeks 1 to 51.
            (gregorian::days_in_year(year) - 357) as u8
        } else {
            STATA_WEEK_DAYS as u8
        }
    }

    /// Stata's weekly date of this week, `%tw`: weeks since 1960w1, so
    /// 52 × (year − 1960) + week − 1.
    #[must_use]
    pub const fn weekly(self) -> i64 {
        52 * (self.year - 1960) + self.week as i64 - 1
    }

    /// The first day of a `%tw` weekly date, as Stata's `dofw` gives it.
    #[must_use]
    pub const fn from_weekly(weekly: i64) -> Self {
        Self {
            year: 1960 + weekly.div_euclid(52),
            week: (weekly.rem_euclid(52) + 1) as u8,
            day: 1,
        }
    }
}

/// Stata's weekly dates as a calendar: `stata-week`.
///
/// Stata, `help datetime_functions` (StataNow 19), read 2026-09-26
/// (`stata-help-datetime-functions`): `week(e_d)` is "the numeric week of
/// the year" with range "integers 1 to 52", and "The first week of a year
/// is the first 7-day period of the year"; weekly dates run "0100w1 to
/// 9999w52 (integers -96,720 to 418,079)", weeks since 1960w1, and `dofw`
/// of 9999w52 is 24dec9999. So week 1 begins on 1 January, each week is
/// seven days, and week 52 begins on the year's 358th day and runs to its
/// end, eight days or, in a leap year, nine: every year has 52 weeks, and
/// a week never crosses New Year. The table in `help datetime` gives
/// 2010w3 as 2 601, which the formula that the ranges above follow puts
/// at 2010w2; 2010w3 is 2 602 here.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StataWeekCalendar;

impl StataWeekCalendar {
    /// The first day Stata's weekly dates reach, 01jan0100.
    pub const EARLIEST: Rd = match gregorian::to_fixed(100, 1, 1) {
        Ok(rd) => rd,
        Err(_) => panic!("a real date"),
    };

    /// The last, 31dec9999.
    pub const LATEST: Rd = match gregorian::to_fixed(9999, 12, 31) {
        Ok(rd) => rd,
        Err(_) => panic!("a real date"),
    };
}

impl Calendar for StataWeekCalendar {
    type Date = StataWeekDate;

    /// The 52 numbered weeks; the days within a week are numbered, not
    /// named, and are not the weekdays.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        const SHAPE: &[hc_calendar::shape::CycleShape] =
            &[hc_calendar::shape::CycleShape::fixed("stata-week", 52)];
        SHAPE
    }

    /// A leap year's week 52 has nine days; the year is the Gregorian one.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(gregorian::is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("stata-week"),
            english_name: "Stata weekly date (%tw)",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(Self::EARLIEST),
            latest: Some(Self::LATEST),
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        if date.week == 0 || date.week > STATA_WEEKS_PER_YEAR {
            return Err(CalendarError::MonthOutOfRange);
        }
        if date.day == 0 || date.day > StataWeekDate::days_in_week(date.year, date.week) {
            return Err(CalendarError::DayOutOfRange);
        }
        let start = gregorian::new_year(date.year)?;
        let rd =
            Rd(start.0 + STATA_WEEK_DAYS * (i64::from(date.week) - 1) + i64::from(date.day) - 1);
        self.meta().check_range(rd)?;
        Ok(rd)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        let year = gregorian::year_from_fixed(rd)?;
        let day_of_year = rd.0 - gregorian::new_year(year)?.0;
        let week = (day_of_year / STATA_WEEK_DAYS).min(51) + 1;
        let day = day_of_year - STATA_WEEK_DAYS * (week - 1) + 1;
        // Week at most 52 and day at most 9, by the arithmetic above.
        Ok(StataWeekDate {
            year,
            week: week as u8,
            day: day as u8,
        })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::new(date.year)
            .with_extra("week", i64::from(date.week))?
            .with_extra("day-of-stata-week", i64::from(date.day))?
            .with_extra("stata-weekly", date.weekly())
    }

    /// As for the ISO week calendar, a missing week or day defaults to 1.
    /// The day is `day-of-stata-week`, not `day-of-week`: it counts from
    /// the week's first day, which is whatever weekday 1 January was.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let week = fields.extra.get("week").unwrap_or(1);
        let day = fields.extra.get("day-of-stata-week").unwrap_or(1);
        let week = u8::try_from(week).map_err(|_| CalendarError::MonthOutOfRange)?;
        let day = u8::try_from(day).map_err(|_| CalendarError::DayOutOfRange)?;
        Ok(StataWeekDate {
            year: fields.year,
            week,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::julian_day::JulianDayCalendar;

    /// Each count's epoch carries the number its defining document says it
    /// does. This is the definition, so it is the first test.
    ///
    /// Identifier uniqueness, the presence of an authority and findability
    /// are checked by the tests `hc_core::catalogue!` generates.
    #[test]
    fn every_epoch_carries_its_stated_number() {
        for count in ALL {
            assert_eq!(
                count.number_of(count.epoch()),
                DayNumber(count.first_number()),
                "{}",
                count.english_name
            );
            assert_eq!(count.day_of(DayNumber(count.first_number())), count.epoch());
        }
    }

    /// The classical offsets from the Julian Day Number, which is how these
    /// are usually quoted. The epochs above and these offsets are two
    /// independent statements of the same fact, so if either were
    /// mistranscribed this would fail. It did, four times over.
    ///
    /// The midnight counts are quoted against a *fractional* Julian Date
    /// ending in .5 — TJD = JD − 2 440 000.5 — and the whole-day offset from
    /// the Julian Day *Number* is therefore one larger, because JDN is
    /// assigned at the following noon. Three of the offsets here were
    /// transcribed from the fractional form and were a day short. And the
    /// Dublin epoch is written "1900 January 0.5", which is 31 December
    /// 1899, not 1 January 1900.
    #[test]
    fn the_offsets_from_the_julian_day_number_are_the_published_ones() {
        let day = gregorian::to_fixed(2024, 1, 1).expect("exists");
        let jdn = JulianDayCalendar.from_fixed(day).expect("in range").0;
        for (count, offset) in [
            (LILIAN, 2_299_160),
            (ANSI, 2_305_813),
            (DUBLIN, 2_415_020),
            (REDUCED, 2_400_000),
            (TRUNCATED, 2_440_001),
            (CNES, 2_433_283),
            (CCSDS, 2_436_205),
            (CHRONOLOGICAL, 0),
            (MJD2000, 2_451_545),
            (EXCEL_1904, 2_416_481),
            (OLE_AUTOMATION, 2_415_019),
            (SAS, 2_436_935),
            (STATA, 2_436_935),
            (MATLAB, 1_721_059),
        ] {
            assert_eq!(
                count.number_of(day),
                DayNumber(jdn - offset),
                "{} should be JDN − {offset}",
                count.english_name
            );
        }
    }

    /// The noon counts and the midnight counts are both here, and mixing
    /// them is the mistake the module header warns about.
    #[test]
    fn the_counts_disagree_about_when_a_day_starts() {
        let noon = DayBoundary::Noon(DayNaming::ByStart);
        assert_eq!(DayCountCalendar(REDUCED).day_boundary(), noon);
        assert_eq!(DayCountCalendar(DUBLIN).day_boundary(), noon);
        for count in [
            LILIAN,
            ANSI,
            TRUNCATED,
            CNES,
            CCSDS,
            CHRONOLOGICAL,
            MJD2000,
            EXCEL_1904,
            OLE_AUTOMATION,
            SAS,
            STATA,
            MATLAB,
        ] {
            assert_eq!(
                DayCountCalendar(count).day_boundary(),
                DayBoundary::Midnight,
                "{}",
                count.english_name
            );
        }
    }

    /// The Reduced Julian Date and the Modified Julian Date are half a day
    /// apart, which as whole day numbers means the reduced count is one
    /// ahead on the same fixed day.
    #[test]
    fn the_reduced_and_modified_counts_differ_by_one_whole_day() {
        for offset in [-10_000, -1, 0, 1, 10_000, 100_000] {
            let rd = Rd(gregorian::to_fixed(2000, 1, 1).expect("exists").0 + offset);
            let reduced = REDUCED.number_of(rd).0;
            let modified = rd.to_modified_julian_day();
            assert_eq!(reduced - modified, 1, "{rd}");
        }
    }

    /// A noon count names its day by the civil day on whose noon it
    /// begins, as the Julian Day does: day 0 of the Reduced count begins at
    /// noon on 16 November 1858, so the afternoon of that day is day 0 and
    /// its morning is day −1.
    #[test]
    fn a_noon_count_names_the_day_its_noon_begins() {
        use hc_calendar::CivilTime;

        for (count, epoch) in [(REDUCED, (1858, 11, 16)), (DUBLIN, (1899, 12, 31))] {
            let calendar = DayCountCalendar(count);
            let civil = gregorian::to_fixed(epoch.0, epoch.1, epoch.2).expect("exists");
            let boundary = calendar.day_boundary();
            let afternoon = boundary
                .civil_day_offset(CivilTime::hms(13, 0, 0).unwrap())
                .unwrap();
            let morning = boundary
                .civil_day_offset(CivilTime::hms(11, 0, 0).unwrap())
                .unwrap();
            assert_eq!(
                calendar.from_fixed(civil + afternoon),
                Ok(DayNumber(0)),
                "{}",
                count.english_name
            );
            assert_eq!(
                calendar.from_fixed(civil + morning),
                Ok(DayNumber(-1)),
                "{}",
                count.english_name
            );
        }
    }

    #[test]
    fn every_count_round_trips_over_a_wide_span() {
        for count in ALL {
            let calendar = DayCountCalendar(*count);
            for offset in (-800_000..3_700_000).step_by(9_973) {
                let rd = Rd(offset);
                if !calendar.meta().supports(rd) {
                    continue;
                }
                let number = calendar.from_fixed(rd).expect("in range");
                assert_eq!(
                    calendar.to_fixed(number),
                    Ok(rd),
                    "{} {rd}",
                    count.english_name
                );
            }
        }
    }

    /// Meyer's example: chronological Julian day 2 452 952 is the period
    /// from midnight on 8 November 2003, the same number as the Julian Day
    /// Number of that day's noon and a different interval.
    #[test]
    fn the_chronological_julian_day_is_the_jdn_from_the_midnight_before() {
        let day = gregorian::to_fixed(2003, 11, 8).expect("exists");
        assert_eq!(CHRONOLOGICAL.number_of(day), DayNumber(2_452_952));
        assert_eq!(
            JulianDayCalendar.from_fixed(day).map(|jdn| jdn.0),
            Ok(2_452_952)
        );
        assert_eq!(
            DayCountCalendar(CHRONOLOGICAL).day_boundary(),
            DayBoundary::Midnight
        );
        assert_eq!(
            JulianDayCalendar.day_boundary(),
            DayBoundary::Noon(DayNaming::ByStart)
        );
    }

    /// MJD2000 is 0 on 1 January 2000 and 51 544 behind the Modified
    /// Julian Date.
    #[test]
    fn mjd2000_starts_on_1_january_2000() {
        let day = gregorian::to_fixed(2000, 1, 1).expect("exists");
        assert_eq!(MJD2000.number_of(day), DayNumber(0));
        assert_eq!(day.to_modified_julian_day(), 51_544);
    }

    /// The vendor counts refuse the days their vendor does not support.
    #[test]
    fn the_vendor_counts_refuse_what_the_vendor_does_not_support() {
        let excel = DayCountCalendar(EXCEL_1904);
        let first = gregorian::to_fixed(1904, 1, 1).expect("exists");
        let last = gregorian::to_fixed(9999, 12, 31).expect("exists");
        assert_eq!(excel.from_fixed(first), Ok(DayNumber(0)));
        assert_eq!(excel.from_fixed(last), Ok(DayNumber(2_957_003)));
        assert_eq!(
            excel.to_fixed(DayNumber(-1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            excel.to_fixed(DayNumber(2_957_004)),
            Err(CalendarError::AfterSupportedRange)
        );
        // Microsoft Support's example: 5 July 2011 is 39 267.
        let example = gregorian::to_fixed(2011, 7, 5).expect("exists");
        assert_eq!(excel.from_fixed(example), Ok(DayNumber(39_267)));

        let ole = DayCountCalendar(OLE_AUTOMATION);
        assert_eq!(
            ole.from_fixed(gregorian::to_fixed(100, 1, 1).expect("exists")),
            Ok(DayNumber(-657_434))
        );
        assert_eq!(ole.from_fixed(last), Ok(DayNumber(2_958_465)));
        assert_eq!(
            ole.to_fixed(DayNumber(-657_435)),
            Err(CalendarError::BeforeEpoch)
        );
    }

    /// SAS's example, `'26oct02'd` = 15 639, and its range, whose ends
    /// `hc_core::sas_stata` states as numbers of its own.
    #[test]
    fn sas_dates_count_from_1960_between_1582_and_19900() {
        let sas = DayCountCalendar(SAS);
        let day = gregorian::to_fixed(2002, 10, 26).expect("exists");
        assert_eq!(sas.from_fixed(day), Ok(DayNumber(15_639)));
        let (first, last) = SAS.range();
        assert_eq!(
            SAS.number_of(first),
            DayNumber(hc_core::sas_stata::SAS_FIRST_DAY)
        );
        assert_eq!(
            SAS.number_of(last),
            DayNumber(hc_core::sas_stata::SAS_LAST_DAY)
        );
        assert_eq!(
            sas.to_fixed(DayNumber(-138_062)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            sas.to_fixed(DayNumber(hc_core::sas_stata::SAS_LAST_DAY + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    /// Stata, `help datetime`: 20jan2010 is 18 282, 22jul2010 is 18 465
    /// and 15jun2004 is 16 237; `help datetime_functions` gives the range
    /// as −679 350 to 2 936 549.
    #[test]
    fn stata_dates_are_the_same_count_over_another_range() {
        let stata = DayCountCalendar(STATA);
        for ((year, month, day), number) in [
            ((2010, 1, 20), 18_282),
            ((2010, 7, 22), 18_465),
            ((2004, 6, 15), 16_237),
            ((100, 1, 1), -679_350),
            ((9999, 12, 31), 2_936_549),
        ] {
            let rd = gregorian::to_fixed(year, month, day).expect("exists");
            assert_eq!(
                stata.from_fixed(rd),
                Ok(DayNumber(number)),
                "{year}-{month}-{day}"
            );
            assert_eq!(SAS.number_of(rd), DayNumber(number));
        }
        let (first, last) = STATA.range();
        assert_eq!(
            STATA.number_of(first).0,
            hc_core::sas_stata::STATA_FIRST_DAY
        );
        assert_eq!(STATA.number_of(last).0, hc_core::sas_stata::STATA_LAST_DAY);
        // 1582 is a SAS date and a Stata one; 1500 only Stata's.
        let early = gregorian::to_fixed(1500, 1, 1).expect("exists");
        assert!(stata.from_fixed(early).is_ok());
        assert_eq!(
            DayCountCalendar(SAS).from_fixed(early),
            Err(CalendarError::BeforeEpoch)
        );
    }

    /// MathWorks' examples: `datenum(2001,12,19)` is 731 204,
    /// '19-May-2001' 730 990, '12-jun-17' 736 858, and with the pivot year
    /// 1400 the same text is 12 June 1417, 517 712; the dates of
    /// '09/16/2007', '05/14/1996' and '11/29/2010' are 733 301, 729 159
    /// and 734 471. Each is the Rata Die plus 366.
    #[test]
    fn matlab_datenum_is_the_rata_die_plus_366() {
        for ((year, month, day), number) in [
            ((2001, 12, 19), 731_204),
            ((2001, 5, 19), 730_990),
            ((2017, 6, 12), 736_858),
            ((1417, 6, 12), 517_712),
            ((2007, 9, 16), 733_301),
            ((1996, 5, 14), 729_159),
            ((2010, 11, 29), 734_471),
        ] {
            let rd = gregorian::to_fixed(year, month, day).expect("exists");
            assert_eq!(
                MATLAB.number_of(rd),
                DayNumber(number),
                "{year}-{month}-{day}"
            );
            assert_eq!(number - rd.0, 366);
        }
        // Day 0 is "January 0, 0000", 31 December of the year −1.
        assert_eq!(
            MATLAB.day_of(DayNumber(0)),
            gregorian::to_fixed(-1, 12, 31).expect("exists")
        );
    }

    #[test]
    fn a_day_count_has_no_day_of_the_month() {
        let calendar = DayCountCalendar(LILIAN);
        let fields = DateFields {
            day: Some(1),
            ..DateFields::new(1)
        };
        assert_eq!(
            calendar.from_fields(&fields),
            Err(CalendarError::UnsupportedField("day"))
        );
    }

    /// Stata's weekly dates: `tw(1960w2)` is 1; 0100w1 is −96 720 and
    /// 9999w52 418 079; `dofw` of those is 01jan0100 and 24dec9999
    /// (−679 350 and 2 936 542 in `%td`); `week(mdy(7,5,2013))` is 27.
    #[test]
    fn stata_weeks_are_52_from_1_january() {
        let calendar = DayCountCalendar(STATA);
        let weeks = StataWeekCalendar;
        assert_eq!(
            StataWeekDate {
                year: 1960,
                week: 2,
                day: 1
            }
            .weekly(),
            1
        );
        for (weekly, first_day) in [(-96_720, -679_350), (418_079, 2_936_542), (0, 0)] {
            let date = StataWeekDate::from_weekly(weekly);
            assert_eq!(date.weekly(), weekly);
            let rd = weeks.to_fixed(date).expect("in range");
            assert_eq!(
                calendar.from_fixed(rd),
                Ok(DayNumber(first_day)),
                "{weekly}"
            );
        }
        assert_eq!(
            StataWeekDate::from_weekly(418_079),
            StataWeekDate {
                year: 9999,
                week: 52,
                day: 1
            }
        );
        let july = gregorian::to_fixed(2013, 7, 5).expect("exists");
        assert_eq!(weeks.from_fixed(july).map(|date| date.week), Ok(27));
        // 20jan2010, the day of `help datetime`'s example, is in 2010w3.
        let example = gregorian::to_fixed(2010, 1, 20).expect("exists");
        let date = weeks.from_fixed(example).expect("in range");
        assert_eq!((date.week, date.weekly()), (3, 2_602));
    }

    /// Week 52 takes the rest of the year: eight days in a common year and
    /// nine in a leap year, and the next day is week 1.
    #[test]
    fn week_52_has_eight_or_nine_days() {
        let weeks = StataWeekCalendar;
        for (year, days) in [(2023, 8), (2024, 9), (1900, 8), (2000, 9)] {
            let last = gregorian::to_fixed(year, 12, 31).expect("exists");
            assert_eq!(
                weeks.from_fixed(last),
                Ok(StataWeekDate {
                    year,
                    week: 52,
                    day: days
                }),
                "{year}"
            );
            assert_eq!(
                weeks.from_fixed(Rd(last.0 + 1)),
                Ok(StataWeekDate {
                    year: year + 1,
                    week: 1,
                    day: 1
                })
            );
            let start = weeks
                .to_fixed(StataWeekDate {
                    year,
                    week: 52,
                    day: 1,
                })
                .expect("exists");
            assert_eq!(
                gregorian::from_fixed(start).map(|(_, m, d)| (m, d)),
                Ok((12, if days == 9 { 23 } else { 24 }))
            );
            assert_eq!(
                weeks.to_fixed(StataWeekDate {
                    year,
                    week: 52,
                    day: days + 1
                }),
                Err(CalendarError::DayOutOfRange)
            );
        }
        assert_eq!(
            weeks.to_fixed(StataWeekDate {
                year: 2024,
                week: 53,
                day: 1
            }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            weeks.to_fixed(StataWeekDate {
                year: 2024,
                week: 1,
                day: 8
            }),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn stata_weeks_round_trip_over_their_range() {
        let weeks = StataWeekCalendar;
        for rd in (StataWeekCalendar::EARLIEST.0..=StataWeekCalendar::LATEST.0).step_by(997) {
            let date = weeks.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(weeks.to_fixed(date), Ok(Rd(rd)));
            let fields = weeks.to_fields(date).expect("fields");
            assert_eq!(weeks.from_fields(&fields), Ok(date));
        }
        assert_eq!(
            weeks.from_fixed(Rd(StataWeekCalendar::EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
    }
}
