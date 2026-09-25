//! The shape of a name-day list, and the one evaluator that reads every
//! list in the crate.
//!
//! A list is 366 slots — one per day of a leap year, 29 February included —
//! each holding the names its authority prints on that day. A slot with no
//! names is a statement, not a hole: the Latvian lists print an en dash on
//! 29 February, the Finnish lists leave 1 January and 25 December empty, and
//! the Hungarian convention leaves 24 February empty in a leap year. Which
//! empty slots are *expected* is what [`LeapDayRule`] says, and a test
//! asserts that every vendored list's empty slots are exactly the ones its
//! rule and its notes account for.
//!
//! [`names_on`] and [`days_of`] are the only two functions that read a
//! list, and they read the vendored [`NameDayList`]s and the loaded
//! [`crate::load::OwnedNameDayList`]s alike, through [`NameDays`]. There is
//! no `latvian_name_day(month, day)`: a country is a table, not a function
//! (`docs/policy.md` §2).

use core::fmt;

use hc_calendar::gregorian;

/// How many slots a list holds: the days of a leap year.
pub const DAYS: usize = 366;

/// The slot of 24 February, the leap day of the old Roman reckoning.
pub const FEBRUARY_24: usize = 54;

/// The slot of 29 February.
pub const FEBRUARY_29: usize = 59;

/// A month and a day, without a year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonthDay {
    /// The month, 1–12.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl MonthDay {
    /// A month and a day.
    #[must_use]
    pub const fn new(month: u8, day: u8) -> Self {
        Self { month, day }
    }
}

impl fmt::Display for MonthDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}-{:02}", self.month, self.day)
    }
}

/// What a list does with the leap day.
///
/// This is an `enum` and not a table because a member arrives only when this
/// crate changes how it models the domain (ADR 0007): the four conventions
/// below are the four the survey behind this crate found, and a fifth would
/// be a modelling decision, not a discovery about a country.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum LeapDayRule {
    /// 29 February carries no names. Latvia prints an en dash; the Finnish
    /// and Norwegian lists leave the day empty. The slot [`FEBRUARY_29`] is
    /// empty in the layout.
    NoNames,
    /// 29 February carries names of its own, kept on that day in leap
    /// years, and the authority says nothing about what becomes of them in
    /// a common year. Czech calendars print Horymír; the Estonian list
    /// prints Ulmi and Une.
    OwnNames,
    /// 24 February is the leap day, as in the Roman *bissextus*: in a leap
    /// year it is empty and the names of 24–28 February move one day
    /// later, to 25–29 February. The Hungarian and Danish conventions.
    ///
    /// The layout is the leap year's: slot [`FEBRUARY_24`] is empty and the
    /// five names sit in the slots of 25–29 February. A common year reads
    /// 24–28 February from the slots of 25–29 February.
    ShiftAfter24February,
    /// 29 February carries a name the authority states is kept only in
    /// leap years — the French postal calendar's Auguste Chapdelaine,
    /// "fêté les années bissextiles". Evaluated exactly as [`Self::OwnNames`];
    /// the variant records that the authority said so rather than left it
    /// open.
    LeapYearsOnly,
}

impl LeapDayRule {
    /// The identifier the loader's text format uses.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::NoNames => "no-names",
            Self::OwnNames => "own-names",
            Self::ShiftAfter24February => "shift-after-24-february",
            Self::LeapYearsOnly => "leap-years-only",
        }
    }

    /// The rule with this identifier.
    #[must_use]
    pub fn by_id(id: &str) -> Option<Self> {
        [
            Self::NoNames,
            Self::OwnNames,
            Self::ShiftAfter24February,
            Self::LeapYearsOnly,
        ]
        .into_iter()
        .find(|rule| rule.id() == id)
    }

    /// A one-line description.
    #[must_use]
    pub const fn english_description(self) -> &'static str {
        match self {
            Self::NoNames => "29 February carries no names",
            Self::OwnNames => "29 February carries names of its own",
            Self::ShiftAfter24February => {
                "24 February is the leap day; the names of 24-28 February move one day later in a leap year"
            }
            Self::LeapYearsOnly => "29 February carries names kept only in leap years",
        }
    }

    /// The slot the rule requires to be empty, if it requires one.
    #[must_use]
    pub const fn empty_slot(self) -> Option<usize> {
        match self {
            Self::NoNames => Some(FEBRUARY_29),
            Self::ShiftAfter24February => Some(FEBRUARY_24),
            Self::OwnNames | Self::LeapYearsOnly => None,
        }
    }
}

/// How a list came to exist.
///
/// The same vocabulary as `hc_attributes::Provenance`, copied rather than
/// depended on: the two crates share a shape, not arithmetic, and the
/// shape moves to `hc-core` when a third crate needs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Provenance {
    /// A named body adopted the list on a date that can be cited.
    Promulgated,
    /// A named writer or publisher recorded it, and the record is what
    /// survives.
    Recorded,
    /// Ordinary usage, with no promulgating body and no single author.
    Vernacular,
    /// The list circulates widely and its usual attribution is disputed.
    Contested,
}

impl Provenance {
    /// A one-line description.
    #[must_use]
    pub const fn english_description(self) -> &'static str {
        match self {
            Self::Promulgated => "adopted by a named body on a citable date",
            Self::Recorded => "recorded by a named writer or publisher",
            Self::Vernacular => "ordinary usage, no promulgating body",
            Self::Contested => "widely circulated, usual attribution disputed",
        }
    }
}

/// A date a list was decided or published, to whatever precision the
/// source gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttributionDate {
    /// The proleptic Gregorian year.
    pub year: i32,
    /// The month, if the source gives one.
    pub month: Option<u8>,
    /// The day, if the source gives one.
    pub day: Option<u8>,
}

impl AttributionDate {
    /// A date known only to the year.
    #[must_use]
    pub const fn year(year: i32) -> Self {
        Self {
            year,
            month: None,
            day: None,
        }
    }

    /// A date known to the month.
    #[must_use]
    pub const fn year_month(year: i32, month: u8) -> Self {
        Self {
            year,
            month: Some(month),
            day: None,
        }
    }

    /// A date known to the day.
    #[must_use]
    pub const fn ymd(year: i32, month: u8, day: u8) -> Self {
        Self {
            year,
            month: Some(month),
            day: Some(day),
        }
    }
}

impl fmt::Display for AttributionDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.month, self.day) {
            (Some(month), Some(day)) => write!(f, "{}-{month:02}-{day:02}", self.year),
            (Some(month), None) => write!(f, "{}-{month:02}", self.year),
            _ => write!(f, "{}", self.year),
        }
    }
}

/// The span of years an edition was, or still is, the one in force.
///
/// `to == None` means "still in force as far as this crate knows", which is
/// a different claim from "in force forever": the Latvian commission
/// revises its lists every few years, and a caller asking about a year
/// after the next revision gets that revision's edition, not this one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Validity {
    /// The first year the edition was in force, if known.
    pub from: Option<i32>,
    /// The last year it was in force, or `None` if it still is.
    pub to: Option<i32>,
}

impl Validity {
    /// Neither endpoint known.
    pub const UNKNOWN: Self = Self {
        from: None,
        to: None,
    };

    /// In force from a year, and still in force.
    #[must_use]
    pub const fn since(year: i32) -> Self {
        Self {
            from: Some(year),
            to: None,
        }
    }

    /// In force between two years, both included.
    #[must_use]
    pub const fn between(from: i32, to: i32) -> Self {
        Self {
            from: Some(from),
            to: Some(to),
        }
    }

    /// In force up to a year, from a year the crate does not know.
    #[must_use]
    pub const fn until(year: i32) -> Self {
        Self {
            from: None,
            to: Some(year),
        }
    }

    /// Whether a year falls inside the span. An unknown endpoint is open.
    #[must_use]
    pub const fn contains(self, year: i32) -> bool {
        let after_start = match self.from {
            Some(start) => year >= start,
            None => true,
        };
        let before_end = match self.to {
            Some(end) => year <= end,
            None => true,
        };
        after_start && before_end
    }

    /// Whether the span is still open at its upper end.
    #[must_use]
    pub const fn is_current(self) -> bool {
        self.to.is_none()
    }
}

impl fmt::Display for Validity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(from) = self.from {
            write!(f, "{from}")?;
        }
        f.write_str("..")?;
        if let Some(to) = self.to {
            write!(f, "{to}")?;
        }
        Ok(())
    }
}

/// The terms under which a list may be copied.
///
/// This field is why some lists are vendored and others only loaded. A list
/// is shipped in this crate only when its variant is one
/// [`Licence::permits_redistribution`] accepts, and a test asserts it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Licence {
    /// The publisher dedicated the list to the public domain under the
    /// named instrument, e.g. `CC0-1.0`.
    PublicDomainDedication(&'static str),
    /// The list is an official work excluded from copyright by the named
    /// statute.
    OfficialWork(&'static str),
    /// The holder asserts copyright; `terms` quotes what use it permits.
    Copyrighted {
        /// Who asserts the copyright.
        holder: &'static str,
        /// The permitted use, in the holder's words.
        terms: &'static str,
    },
    /// No statement of terms was found. Under the Nordic catalogue rule and
    /// the EU database right, silence is not permission.
    Unknown,
}

impl Licence {
    /// Whether the terms allow the list to be copied into a library and
    /// redistributed under it.
    #[must_use]
    pub const fn permits_redistribution(&self) -> bool {
        matches!(
            self,
            Self::PublicDomainDedication(_) | Self::OfficialWork(_)
        )
    }
}

impl fmt::Display for Licence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PublicDomainDedication(instrument) => {
                write!(f, "public domain dedication ({instrument})")
            }
            Self::OfficialWork(statute) => write!(f, "official work ({statute})"),
            Self::Copyrighted { holder, terms } => write!(f, "copyright {holder}: {terms}"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}

/// Something the source prints beside a day, or beside a name, that is not
/// itself a name.
///
/// The Latvian lists print, on 22 May, the sentence reserving the day for
/// names not in the calendar; the 2026 extended list prints four Latgalian
/// forms in parentheses after the Latvian name. Both are kept here, in the
/// source's own words, so the table can stay a table of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Note {
    /// The month the note is printed on.
    pub month: u8,
    /// The day it is printed on.
    pub day: u8,
    /// The name it is attached to, or `None` when it is about the day.
    pub name: Option<&'static str>,
    /// The note, as printed or as this crate explains it.
    pub text: &'static str,
}

/// One edition of one authority's name-day list.
///
/// A caller cannot obtain names without obtaining the authority, the
/// edition, the validity and the licence, because there is no constructor
/// that omits them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NameDayList {
    /// A stable identifier: country, list, edition year, e.g.
    /// `lv-traditional-2026`.
    pub id: &'static str,
    /// ISO 3166-1 alpha-2, lowercase.
    pub country: &'static str,
    /// BCP 47 tag of the language the names are written in.
    pub language: &'static str,
    /// The English name of the list.
    pub english_name: &'static str,
    /// The body whose list it is.
    pub authority: &'static str,
    /// When the body decided this edition, if known.
    pub decided: Option<AttributionDate>,
    /// How the list came to exist.
    pub provenance: Provenance,
    /// The years this edition is or was in force.
    pub validity: Validity,
    /// The terms it may be copied under.
    pub licence: Licence,
    /// What the list does with the leap day.
    pub leap_day: LeapDayRule,
    /// The names, one slot per day of a leap year: slot 0 is 1 January,
    /// slot [`FEBRUARY_29`] is 29 February, slot 365 is 31 December.
    pub days: &'static [&'static [&'static str]; DAYS],
    /// The day the authority reserves for names absent from the list, if
    /// it reserves one. Latvia: 22 May.
    pub unlisted_names_day: Option<MonthDay>,
    /// What the source prints that is not a name.
    pub notes: &'static [Note],
    /// The citation: dataset, URL, licence and retrieval date.
    pub source: &'static str,
    /// When the file the table was transcribed from was retrieved.
    pub retrieved: AttributionDate,
}

impl NameDayList {
    /// The names in a slot, by index into the leap-year layout.
    ///
    /// An index of [`DAYS`] or more finds nothing; [`names_on`] never
    /// produces one.
    #[must_use]
    pub fn slot(&self, index: usize) -> &'static [&'static str] {
        self.days.get(index).copied().unwrap_or(&[])
    }

    /// How many names the list carries, counting a name that appears on
    /// several days once per day.
    #[must_use]
    pub fn total_names(&self) -> usize {
        self.days.iter().map(|slot| slot.len()).sum()
    }

    /// The slots that carry no names, as month and day of the layout.
    pub fn empty_days(&self) -> impl Iterator<Item = MonthDay> + '_ {
        self.days
            .iter()
            .enumerate()
            .filter(|(_, slot)| slot.is_empty())
            .filter_map(|(index, _)| layout_month_day(index))
    }

    /// Whether the list carries a name anywhere, case-sensitively.
    #[must_use]
    pub fn names(&self, name: &str) -> bool {
        self.days.iter().any(|slot| slot.contains(&name))
    }
}

/// What the evaluator needs from a list, so that the vendored tables and a
/// caller's loaded list are read by the same code.
pub trait NameDays {
    /// How a name is stored: `&'static str` in a vendored table, an owned
    /// string in a loaded one.
    type Name: AsRef<str>;

    /// The years the list is in force.
    fn validity(&self) -> Validity;

    /// What the list does with the leap day.
    fn leap_day(&self) -> LeapDayRule;

    /// The names in a slot of the leap-year layout.
    fn slot(&self, index: usize) -> &[Self::Name];
}

impl NameDays for NameDayList {
    type Name = &'static str;

    fn validity(&self) -> Validity {
        self.validity
    }

    fn leap_day(&self) -> LeapDayRule {
        self.leap_day
    }

    fn slot(&self, index: usize) -> &[&'static str] {
        Self::slot(self, index)
    }
}

/// Why a list has no answer for a date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NameDayError {
    /// The edition is not in force in that year, and this crate does not
    /// extrapolate an edition past its validity (ADR 0006).
    OutsideValidity {
        /// The year asked for.
        year: i32,
        /// The years the edition covers.
        validity: Validity,
    },
    /// No such day exists in the proleptic Gregorian calendar of that year.
    InvalidDate {
        /// The year asked for.
        year: i32,
        /// The month asked for.
        month: u8,
        /// The day asked for.
        day: u8,
    },
}

impl fmt::Display for NameDayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutsideValidity { year, validity } => {
                write!(f, "the edition covers {validity}, not {year}")
            }
            Self::InvalidDate { year, month, day } => {
                write!(f, "{year}-{month:02}-{day:02} is not a date")
            }
        }
    }
}

impl core::error::Error for NameDayError {}

/// The slot a date reads, under a leap-day rule.
///
/// This is the whole of the evaluator's arithmetic. A valid date of a leap
/// year reads its own slot. A valid date of a common year reads its own
/// slot too, except under [`LeapDayRule::ShiftAfter24February`], where
/// 24–28 February read the slots of 25–29 February, because the layout is
/// the leap year's and the leap year is the one that has moved.
///
/// # Errors
///
/// [`NameDayError::InvalidDate`] when the date does not exist in that year,
/// 29 February of a common year included: a list is not asked what it says
/// on a day that is not there.
pub fn slot_index(rule: LeapDayRule, year: i32, month: u8, day: u8) -> Result<usize, NameDayError> {
    let invalid = NameDayError::InvalidDate { year, month, day };
    let Some(length) = gregorian::days_in_month(i64::from(year), month) else {
        return Err(invalid);
    };
    if day == 0 || day > length {
        return Err(invalid);
    }
    let leap = gregorian::is_leap_year(i64::from(year));
    let read_day = match rule {
        LeapDayRule::ShiftAfter24February if !leap && month == 2 && day >= 24 => day + 1,
        _ => day,
    };
    layout_index(month, read_day).ok_or(invalid)
}

/// The names a list gives for a date.
///
/// # Errors
///
/// - [`NameDayError::OutsideValidity`] when the edition is not in force in
///   `year`. A list revised for 2026 does not answer for 2025, and a list
///   that ran out in 2025 does not answer for 2026; the caller picks the
///   edition, and [`crate::in_force`] finds it.
/// - [`NameDayError::InvalidDate`] when the date does not exist.
pub fn names_on<L: NameDays + ?Sized>(
    list: &L,
    year: i32,
    month: u8,
    day: u8,
) -> Result<&[L::Name], NameDayError> {
    let validity = list.validity();
    if !validity.contains(year) {
        return Err(NameDayError::OutsideValidity { year, validity });
    }
    let index = slot_index(list.leap_day(), year, month, day)?;
    Ok(list.slot(index))
}

/// The dates in a year on which a list gives a name.
///
/// A name may fall on several days — the extended Latvian list has
/// several — or on none, and the iterator says so by its length. The match
/// is exact and case-sensitive: the list's own spelling, diacritics
/// included, and no diminutive the authority did not print.
///
/// # Errors
///
/// [`NameDayError::OutsideValidity`] when the edition is not in force in
/// `year`.
pub fn days_of<'a, L: NameDays + ?Sized>(
    list: &'a L,
    name: &'a str,
    year: i32,
) -> Result<impl Iterator<Item = MonthDay> + 'a, NameDayError> {
    let validity = list.validity();
    if !validity.contains(year) {
        return Err(NameDayError::OutsideValidity { year, validity });
    }
    let rule = list.leap_day();
    Ok((1..=12u8).flat_map(move |month| {
        let length = gregorian::days_in_month(i64::from(year), month).unwrap_or(0);
        (1..=length).filter_map(move |day| {
            let index = slot_index(rule, year, month, day).ok()?;
            list.slot(index)
                .iter()
                .any(|candidate| candidate.as_ref() == name)
                .then_some(MonthDay { month, day })
        })
    }))
}

/// The slot of a month and day in the leap-year layout.
#[must_use]
pub const fn layout_index(month: u8, day: u8) -> Option<usize> {
    const BEFORE: [u16; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
    if month == 0 || month > 12 {
        return None;
    }
    let Some(length) = gregorian::days_in_month(2000, month) else {
        return None;
    };
    if day == 0 || day > length {
        return None;
    }
    Some(BEFORE[(month - 1) as usize] as usize + day as usize - 1)
}

/// The month and day of a slot in the leap-year layout.
#[must_use]
pub const fn layout_month_day(index: usize) -> Option<MonthDay> {
    if index >= DAYS {
        return None;
    }
    let mut month = 1u8;
    let mut remaining = index;
    while month <= 12 {
        let Some(length) = gregorian::days_in_month(2000, month) else {
            return None;
        };
        if remaining < length as usize {
            return Some(MonthDay {
                month,
                day: remaining as u8 + 1,
            });
        }
        remaining -= length as usize;
        month += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[&str] = &[];

    /// A three-name synthetic list for the evaluator: enough to see the
    /// leap-day rules move names, and nothing anyone could mistake for a
    /// country.
    const fn synthetic(
        rule: LeapDayRule,
        days: &'static [&'static [&'static str]; DAYS],
    ) -> NameDayList {
        NameDayList {
            id: "test",
            country: "zz",
            language: "und",
            english_name: "a synthetic list",
            authority: "this test module",
            decided: None,
            provenance: Provenance::Recorded,
            validity: Validity::between(2000, 2099),
            licence: Licence::PublicDomainDedication("CC0-1.0"),
            leap_day: rule,
            days,
            unlisted_names_day: None,
            notes: &[],
            source: "this test module",
            retrieved: AttributionDate::year(2026),
        }
    }

    /// 24–29 February filled with letters, everything else empty.
    static FEBRUARY: [&[&str]; DAYS] = {
        let mut days: [&[&str]; DAYS] = [EMPTY; DAYS];
        days[FEBRUARY_24] = &["a"];
        days[FEBRUARY_24 + 1] = &["b"];
        days[FEBRUARY_24 + 2] = &["c"];
        days[FEBRUARY_24 + 3] = &["d"];
        days[FEBRUARY_24 + 4] = &["e"];
        days[FEBRUARY_29] = &["f"];
        days[0] = &["new-year"];
        days[365] = &["silvester"];
        days
    };

    /// The Hungarian layout: 24 February empty, the names in 25–29.
    static SHIFTED: [&[&str]; DAYS] = {
        let mut days: [&[&str]; DAYS] = [EMPTY; DAYS];
        days[FEBRUARY_24 + 1] = &["a"];
        days[FEBRUARY_24 + 2] = &["b"];
        days[FEBRUARY_24 + 3] = &["c"];
        days[FEBRUARY_24 + 4] = &["d"];
        days[FEBRUARY_29] = &["e"];
        days
    };

    #[test]
    fn the_layout_is_the_leap_year_and_round_trips() {
        assert_eq!(layout_index(1, 1), Some(0));
        assert_eq!(layout_index(2, 24), Some(FEBRUARY_24));
        assert_eq!(layout_index(2, 29), Some(FEBRUARY_29));
        assert_eq!(layout_index(3, 1), Some(60));
        assert_eq!(layout_index(12, 31), Some(365));
        assert_eq!(layout_index(2, 30), None);
        assert_eq!(layout_index(13, 1), None);
        assert_eq!(layout_index(0, 1), None);
        assert_eq!(layout_index(4, 0), None);
        for index in 0..DAYS {
            let Some(md) = layout_month_day(index) else {
                panic!("slot {index} has no date")
            };
            assert_eq!(layout_index(md.month, md.day), Some(index));
        }
        assert_eq!(layout_month_day(DAYS), None);
    }

    #[test]
    fn a_leap_year_reads_every_slot_as_itself() {
        let list = synthetic(LeapDayRule::NoNames, &FEBRUARY);
        assert_eq!(names_on(&list, 2024, 1, 1), Ok(&["new-year"][..]));
        assert_eq!(names_on(&list, 2024, 2, 24), Ok(&["a"][..]));
        assert_eq!(names_on(&list, 2024, 2, 28), Ok(&["e"][..]));
        assert_eq!(names_on(&list, 2024, 2, 29), Ok(&["f"][..]));
        assert_eq!(names_on(&list, 2024, 12, 31), Ok(&["silvester"][..]));
    }

    #[test]
    fn a_common_year_has_no_twenty_ninth_and_says_so() {
        let list = synthetic(LeapDayRule::OwnNames, &FEBRUARY);
        assert_eq!(names_on(&list, 2023, 2, 28), Ok(&["e"][..]));
        assert_eq!(
            names_on(&list, 2023, 2, 29),
            Err(NameDayError::InvalidDate {
                year: 2023,
                month: 2,
                day: 29
            })
        );
        assert_eq!(
            names_on(&list, 2024, 4, 31),
            Err(NameDayError::InvalidDate {
                year: 2024,
                month: 4,
                day: 31
            })
        );
        assert_eq!(
            names_on(&list, 2024, 13, 1),
            Err(NameDayError::InvalidDate {
                year: 2024,
                month: 13,
                day: 1
            })
        );
    }

    #[test]
    fn the_bissextus_rule_moves_late_february_one_day_later_in_a_leap_year() {
        let list = synthetic(LeapDayRule::ShiftAfter24February, &SHIFTED);
        // Leap year: the layout as it stands.
        assert_eq!(names_on(&list, 2024, 2, 24), Ok(EMPTY));
        assert_eq!(names_on(&list, 2024, 2, 25), Ok(&["a"][..]));
        assert_eq!(names_on(&list, 2024, 2, 29), Ok(&["e"][..]));
        // Common year: 24–28 February read the slots of 25–29.
        assert_eq!(names_on(&list, 2023, 2, 23), Ok(EMPTY));
        assert_eq!(names_on(&list, 2023, 2, 24), Ok(&["a"][..]));
        assert_eq!(names_on(&list, 2023, 2, 28), Ok(&["e"][..]));
        assert_eq!(names_on(&list, 2023, 3, 1), Ok(EMPTY));
    }

    #[test]
    fn an_edition_refuses_a_year_it_does_not_cover() {
        let list = synthetic(LeapDayRule::NoNames, &FEBRUARY);
        let outside = NameDayError::OutsideValidity {
            year: 2100,
            validity: Validity::between(2000, 2099),
        };
        assert_eq!(names_on(&list, 2100, 1, 1), Err(outside));
        assert_eq!(names_on(&list, 1999, 1, 1).ok(), None);
        assert!(days_of(&list, "a", 2100).is_err());
        assert_eq!(names_on(&list, 2099, 1, 1), Ok(&["new-year"][..]));
    }

    #[test]
    fn the_reverse_lookup_follows_the_leap_day_rule() {
        let shifted = synthetic(LeapDayRule::ShiftAfter24February, &SHIFTED);
        let Ok(leap) = days_of(&shifted, "e", 2024) else {
            panic!("2024 is covered")
        };
        assert!(leap.eq([MonthDay::new(2, 29)]));
        let Ok(common) = days_of(&shifted, "e", 2023) else {
            panic!("2023 is covered")
        };
        assert!(common.eq([MonthDay::new(2, 28)]));

        let plain = synthetic(LeapDayRule::NoNames, &FEBRUARY);
        let Ok(none) = days_of(&plain, "f", 2023) else {
            panic!("2023 is covered")
        };
        assert_eq!(none.count(), 0, "29 February does not exist in 2023");
        let Ok(missing) = days_of(&plain, "nobody", 2024) else {
            panic!("2024 is covered")
        };
        assert_eq!(missing.count(), 0);
    }

    #[test]
    fn the_leap_day_rules_are_findable_by_their_identifiers() {
        for rule in [
            LeapDayRule::NoNames,
            LeapDayRule::OwnNames,
            LeapDayRule::ShiftAfter24February,
            LeapDayRule::LeapYearsOnly,
        ] {
            assert_eq!(LeapDayRule::by_id(rule.id()), Some(rule));
            assert!(!rule.english_description().is_empty());
        }
        assert_eq!(LeapDayRule::by_id("bissextus"), None);
        assert_eq!(LeapDayRule::NoNames.empty_slot(), Some(FEBRUARY_29));
        assert_eq!(
            LeapDayRule::ShiftAfter24February.empty_slot(),
            Some(FEBRUARY_24)
        );
        assert_eq!(LeapDayRule::OwnNames.empty_slot(), None);
    }

    #[test]
    fn a_validity_span_prints_and_contains_as_the_loader_reads_it() {
        assert_eq!(Validity::between(2025, 2029).to_string(), "2025..2029");
        assert_eq!(Validity::since(2026).to_string(), "2026..");
        assert_eq!(Validity::until(2025).to_string(), "..2025");
        assert_eq!(Validity::UNKNOWN.to_string(), "..");
        assert!(Validity::until(2025).contains(1900));
        assert!(!Validity::until(2025).contains(2026));
        assert!(Validity::since(2026).is_current());
        assert!(!Validity::between(2023, 2025).is_current());
    }

    #[test]
    fn only_a_dedication_or_an_official_work_may_be_vendored() {
        assert!(Licence::PublicDomainDedication("CC0-1.0").permits_redistribution());
        assert!(Licence::OfficialWork("some statute").permits_redistribution());
        assert!(
            !Licence::Copyrighted {
                holder: "someone",
                terms: "ask"
            }
            .permits_redistribution()
        );
        assert!(!Licence::Unknown.permits_redistribution());
        assert_eq!(
            Licence::PublicDomainDedication("CC0-1.0").to_string(),
            "public domain dedication (CC0-1.0)"
        );
    }

    #[test]
    fn the_error_and_the_dates_print_readably() {
        assert_eq!(
            NameDayError::OutsideValidity {
                year: 2030,
                validity: Validity::between(2025, 2029)
            }
            .to_string(),
            "the edition covers 2025..2029, not 2030"
        );
        assert_eq!(
            NameDayError::InvalidDate {
                year: 2023,
                month: 2,
                day: 29
            }
            .to_string(),
            "2023-02-29 is not a date"
        );
        assert_eq!(AttributionDate::ymd(2025, 4, 30).to_string(), "2025-04-30");
        assert_eq!(AttributionDate::year_month(2025, 5).to_string(), "2025-05");
        assert_eq!(AttributionDate::year(2022).to_string(), "2022");
        assert_eq!(MonthDay::new(5, 22).to_string(), "05-22");
    }

    #[test]
    fn a_list_reports_its_empty_days_and_its_size() {
        let list = synthetic(LeapDayRule::NoNames, &SHIFTED);
        assert_eq!(list.total_names(), 5);
        assert!(list.names("a"));
        assert!(!list.names("A"));
        assert_eq!(list.empty_days().count(), DAYS - 5);
        assert!(list.empty_days().any(|md| md == MonthDay::new(2, 24)));
        assert_eq!(list.slot(DAYS), EMPTY);
    }
}
