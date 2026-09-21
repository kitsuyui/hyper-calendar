//! The shared shape: a named attribution list with a source and a date.
//!
//! Every table in this crate is an [`AttributionTable`]: an [`Authority`]
//! value saying *who says so, where, and when*, plus `N` entries saying
//! *what they say*. Twelve entries for a month table, seven for a weekday
//! table, twelve for a zodiac table. [`AttributionTable::at`] is the only
//! function that reads one, and [`unanimous`] and [`disagreement_count`] are
//! the only functions that compare several.
//!
//! That is deliberately the same split `hc_almanac::rules` makes: thirty-six
//! 暦注 there share one `rule_applies`, and nine birthstone and flower lists
//! here share one `at`. The alternative — a `birthstone(month)` function per
//! list — would be nine places to silently pick a default.
//!
//! # Why an entry is a list
//!
//! Several months carry more than one stone in the same list. The American
//! list gives December turquoise, zircon *and* tanzanite; Japan's 2021 list
//! gives March aquamarine, coral, bloodstone *and* iolite. An entry is
//! therefore `&[&str]`, never `&str`, and the order inside an entry is the
//! order the publishing body printed, which is not a ranking this crate
//! endorses.
//!
//! # Empty entries
//!
//! No table in this crate ships an empty entry, and a test asserts it for
//! every table. An authority with nothing to say about a month is not a
//! table with a hole in it; it is a table this crate does not ship. See
//! [`crate::gaps`].

use hc_calendar::{CalendarError, CalendarResult, Month, Weekday};

/// How many entries a month-keyed or sign-keyed table holds.
pub const MONTHS: usize = 12;

/// How many entries a weekday-keyed table holds.
pub const WEEKDAYS: usize = 7;

/// Where an attribution list is in use.
///
/// This is the region the *list* belongs to, not the region a stone or a
/// flower comes from. Tanzanite is Tanzanian and appears in the American
/// list; that makes the list American, not the stone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Region {
    /// The United States.
    UnitedStates,
    /// The United Kingdom.
    UnitedKingdom,
    /// Japan.
    Japan,
    /// Anglo-Saxon England.
    EarlyEngland,
    /// The Frankish empire under Charlemagne.
    Francia,
    /// Finland.
    Finland,
    /// The Czech lands.
    Czechia,
    /// Thailand.
    Thailand,
    /// The Indian subcontinent.
    India,
    /// The Norse and wider Germanic world.
    Germanic,
    /// The Roman world and its Latin inheritance.
    RomanWorld,
    /// Colonial and later North America, without a narrower attribution
    /// this crate is willing to make. See [`crate::moon_names`].
    NorthAmerica,
    /// No single region: a list circulated across the English-speaking
    /// world without a promulgating body.
    Unspecified,
}

impl Region {
    /// The name in English.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::UnitedStates => "United States",
            Self::UnitedKingdom => "United Kingdom",
            Self::Japan => "Japan",
            Self::EarlyEngland => "Anglo-Saxon England",
            Self::Francia => "Francia",
            Self::Finland => "Finland",
            Self::Czechia => "Czechia",
            Self::Thailand => "Thailand",
            Self::India => "India",
            Self::Germanic => "the Germanic world",
            Self::RomanWorld => "the Roman world",
            Self::NorthAmerica => "North America",
            Self::Unspecified => "unspecified",
        }
    }
}

/// How an attribution list came to exist.
///
/// This is the field that stops the crate from flattening a trade
/// association's press release, a monk's treatise and a poet's invention
/// into one undifferentiated pile of "tradition".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Provenance {
    /// A named body adopted the list on a date that can be cited.
    ///
    /// The 1912 Kansas City meeting of the National Association of Jewelers
    /// and the 2021 announcement of the 全国宝石卸商協同組合 are both of
    /// this kind. It says the list is *documented*, not that it is *old*.
    Promulgated,
    /// A named writer recorded it, and the record is what survives.
    ///
    /// Bede on the Old English months, Einhard on Charlemagne's, Kunz on
    /// the pre-1912 stones.
    Recorded,
    /// Ordinary language, with no promulgating body and no single author.
    ///
    /// The Finnish and Czech month names are of this kind: they are simply
    /// what the words are.
    Vernacular,
    /// The list circulates widely and its usual attribution is disputed.
    ///
    /// The full-moon names are the case this variant exists for. See
    /// [`crate::moon_names`].
    Contested,
    /// Presented as ancient and demonstrably modern.
    ///
    /// No table in this crate carries this variant; it exists so that the
    /// type can say so if one ever does, and so that [`crate::gaps`] can
    /// name the category. Robert Graves's "Celtic tree calendar" is the
    /// standing example.
    ModernInvention,
}

impl Provenance {
    /// A one-line description, for diagnostics and for a caller that wants
    /// to print a warning beside a table.
    #[must_use]
    pub const fn english_description(self) -> &'static str {
        match self {
            Self::Promulgated => "adopted by a named body on a citable date",
            Self::Recorded => "recorded by a named writer",
            Self::Vernacular => "ordinary language, no promulgating body",
            Self::Contested => "widely circulated, usual attribution disputed",
            Self::ModernInvention => "modern invention presented as ancient",
        }
    }

    /// Whether a caller should print the authority's caveat beside any
    /// answer it takes from the table.
    ///
    /// True for [`Self::Contested`] and [`Self::ModernInvention`]. This is
    /// advice, not enforcement: the caveat is a public field and a caller
    /// is free to print it always, which is the better habit.
    #[must_use]
    pub const fn warrants_a_caveat(self) -> bool {
        matches!(self, Self::Contested | Self::ModernInvention)
    }
}

/// A date an attribution list was adopted or revised, to whatever precision
/// the source gives.
///
/// The 1912 American meeting is dated to a month (August); the 2021 Japanese
/// revision is dated to a day (20 December); Bede's list is dated to a work
/// (*De temporum ratione*, 725) and so to a year only. One type carries all
/// three rather than forcing a false precision on the first two.
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

    /// How precisely the date is known: 1 for a year, 2 for a month, 3 for
    /// a day.
    #[must_use]
    pub const fn precision(self) -> u8 {
        match (self.month, self.day) {
            (None, _) => 1,
            (Some(_), None) => 2,
            (Some(_), Some(_)) => 3,
        }
    }
}

/// The span of years an attribution list was, or still is, the current one.
///
/// `to == None` means "still current as far as this crate knows", which is
/// a different claim from "current forever". The American list has had four
/// revisions in a century and there is no reason to expect the fifth not to
/// come.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Validity {
    /// The first year the list was current, if known.
    pub from: Option<i32>,
    /// The last year it was current, or `None` if it still is.
    pub to: Option<i32>,
}

impl Validity {
    /// Neither endpoint known.
    pub const UNKNOWN: Self = Self {
        from: None,
        to: None,
    };

    /// Current from a year, and still current.
    #[must_use]
    pub const fn since(year: i32) -> Self {
        Self {
            from: Some(year),
            to: None,
        }
    }

    /// Current between two years, both included.
    #[must_use]
    pub const fn between(from: i32, to: i32) -> Self {
        Self {
            from: Some(from),
            to: Some(to),
        }
    }

    /// Whether a Gregorian year falls inside the span.
    ///
    /// An unknown endpoint is treated as open, so [`Self::UNKNOWN`] contains
    /// every year. That is the honest reading: the crate does not know when
    /// the Finnish month names started being used, and pretending they began
    /// in some particular year would be worse than answering "yes".
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

/// Who says so.
///
/// The whole point of this crate is that this value exists. A caller cannot
/// obtain an attribution without also obtaining the authority that made it,
/// because [`AttributionTable`] carries both and there is no constructor
/// that omits one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Authority {
    /// A stable identifier, lowercase and hyphenated, e.g.
    /// `birthstones-jp-2021`.
    pub id: &'static str,
    /// The English name of the list.
    pub english_name: &'static str,
    /// The organisation or person that issued it, if there is one.
    ///
    /// `None` for a vernacular list: nobody issued the Finnish month names.
    pub body: Option<&'static str>,
    /// Where the list is in use.
    pub region: Region,
    /// When the list was first adopted, if known.
    pub established: Option<AttributionDate>,
    /// When it was last revised, if it has been.
    pub revised: Option<AttributionDate>,
    /// The years the list was or is current.
    pub validity: Validity,
    /// How it came to exist.
    pub provenance: Provenance,
    /// A citation: a book, a gazette, an announcement, a URL.
    pub source: &'static str,
    /// What a caller should know before repeating the list.
    ///
    /// `None` only where there is genuinely nothing to warn about. Where a
    /// table's [`Provenance`] is [`Provenance::Contested`] this is always
    /// `Some`, and a test asserts it.
    pub caveat: Option<&'static str>,
}

impl Authority {
    /// The most recent date attached to the authority: the revision if there
    /// is one, otherwise the establishment.
    #[must_use]
    pub const fn latest_date(&self) -> Option<AttributionDate> {
        match self.revised {
            Some(date) => Some(date),
            None => self.established,
        }
    }
}

/// A named attribution list: one [`Authority`] and `N` entries.
///
/// `N` is 12 for a month or zodiac table and 7 for a weekday table. The
/// const generic is what lets one `at` serve all of them without a trait
/// object or a slice length check at every call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributionTable<const N: usize> {
    authority: Authority,
    entries: [&'static [&'static str]; N],
}

/// A month-keyed table: index 0 is January.
pub type MonthTable = AttributionTable<MONTHS>;

/// A weekday-keyed table: index 0 is Sunday.
///
/// Sunday-first, not ISO Monday-first, because every tradition in
/// [`crate::weekday_attributions`] counts the planetary week from the Sun.
/// [`hc_calendar::Weekday::sunday_first_number`] produces the index.
pub type WeekdayTable = AttributionTable<WEEKDAYS>;

/// A zodiac-sign-keyed table: index 0 is Aries.
///
/// The index is [`hc_seasons::TropicalSign::index`].
pub type SignTable = AttributionTable<MONTHS>;

impl<const N: usize> AttributionTable<N> {
    /// Build a table. Used only by this crate's static data.
    #[must_use]
    pub const fn new(authority: Authority, entries: [&'static [&'static str]; N]) -> Self {
        Self { authority, entries }
    }

    /// Who says so.
    #[must_use]
    pub const fn authority(&self) -> &Authority {
        &self.authority
    }

    /// How many entries the table holds. Always `N`.
    #[must_use]
    pub const fn len(&self) -> usize {
        N
    }

    /// Whether the table holds no entries, which no shipped table does.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// The entry at a zero-based index.
    ///
    /// # Errors
    ///
    /// [`CalendarError::MonthOutOfRange`] when `index >= N`. The month
    /// variant is used for weekday tables too: the error means "the key you
    /// gave is not a key of this table", and the crate would rather reuse
    /// the workspace's error type than add a seventh one.
    pub fn at(&self, index: usize) -> CalendarResult<&'static [&'static str]> {
        self.entries
            .get(index)
            .copied()
            .ok_or(CalendarError::MonthOutOfRange)
    }

    /// Every entry, in key order.
    pub fn iter(&self) -> impl Iterator<Item = &'static [&'static str]> + '_ {
        self.entries.iter().copied()
    }

    /// Every entry paired with its zero-based key.
    pub fn enumerate(&self) -> impl Iterator<Item = (usize, &'static [&'static str])> + '_ {
        self.entries.iter().copied().enumerate()
    }

    /// Whether every entry names at least one thing.
    ///
    /// Every table this crate ships satisfies it, and a test asserts so for
    /// each one. The method exists for a caller assembling a table of its
    /// own from this crate's types.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.entries.iter().all(|entry| !entry.is_empty())
    }

    /// How many things the table names in total, counting repeats.
    #[must_use]
    pub fn total_attributions(&self) -> usize {
        self.entries.iter().map(|entry| entry.len()).sum()
    }

    /// Whether the table names a given thing anywhere, case-sensitively.
    #[must_use]
    pub fn names(&self, subject: &str) -> bool {
        self.entries.iter().any(|entry| entry.contains(&subject))
    }

    /// The keys at which the table names a given thing.
    pub fn keys_naming<'a>(&'a self, subject: &'a str) -> impl Iterator<Item = usize> + 'a {
        self.enumerate()
            .filter(move |(_, entry)| entry.contains(&subject))
            .map(|(index, _)| index)
    }
}

/// The zero-based index of a Gregorian month in a twelve-entry table.
///
/// # Errors
///
/// - [`CalendarError::UnsupportedField`] for a leap month. No tradition in
///   this crate assigns anything to an intercalary month, and answering with
///   the ordinary month's entry would be inventing an attribution rather
///   than reporting one.
/// - [`CalendarError::MonthOutOfRange`] for an ordinal outside 1..=12.
pub fn month_index(month: Month) -> CalendarResult<usize> {
    if month.leap {
        return Err(CalendarError::UnsupportedField("leap month"));
    }
    match month.ordinal {
        1..=12 => Ok(usize::from(month.ordinal - 1)),
        _ => Err(CalendarError::MonthOutOfRange),
    }
}

/// The zero-based index of a weekday in a seven-entry table, Sunday first.
#[must_use]
pub fn weekday_index(weekday: Weekday) -> usize {
    usize::from(weekday.sunday_first_number())
}

/// Whether every table gives exactly the same entry at a key.
///
/// "The same" means the same names in the same order. Two lists that name
/// aquamarine and bloodstone in opposite orders are *not* unanimous by this
/// function, which is the conservative reading: the order is the publishing
/// body's and this crate does not know that reordering is meaningless.
///
/// An empty `tables` is vacuously unanimous.
///
/// # Errors
///
/// [`CalendarError::MonthOutOfRange`] when the key is out of range for the
/// tables.
pub fn unanimous<const N: usize>(
    tables: &[&AttributionTable<N>],
    index: usize,
) -> CalendarResult<bool> {
    let Some((first, rest)) = tables.split_first() else {
        return Ok(true);
    };
    let reference = first.at(index)?;
    for table in rest {
        if table.at(index)? != reference {
            return Ok(false);
        }
    }
    Ok(true)
}

/// How many of the twelve — or seven — keys the tables disagree at.
///
/// This is the number the crate exists to make printable. For the six
/// birthstone lists it is twelve: no month is agreed by all six, which is
/// the honest headline and not a defect in the data.
///
/// # Errors
///
/// [`CalendarError::MonthOutOfRange`] cannot in fact occur, because the
/// function only asks for keys `0..N`; the signature carries the `Result`
/// because [`unanimous`] does.
pub fn disagreement_count<const N: usize>(
    tables: &[&AttributionTable<N>],
) -> CalendarResult<usize> {
    let mut count = 0;
    for index in 0..N {
        if !unanimous(tables, index)? {
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_AUTHORITY: Authority = Authority {
        id: "test",
        english_name: "a test list",
        body: None,
        region: Region::Unspecified,
        established: Some(AttributionDate::year(2000)),
        revised: None,
        validity: Validity::since(2000),
        provenance: Provenance::Vernacular,
        source: "this test module",
        caveat: None,
    };

    fn table(entries: [&'static [&'static str]; 3]) -> AttributionTable<3> {
        AttributionTable::new(TEST_AUTHORITY, entries)
    }

    #[test]
    fn an_index_past_the_end_of_a_table_is_an_error_not_a_wrap() {
        let subject = table([&["a"], &["b"], &["c"]]);
        assert_eq!(subject.at(2), Ok(&["c"] as &[&str]));
        assert_eq!(subject.at(3), Err(CalendarError::MonthOutOfRange));
        assert_eq!(subject.at(usize::MAX), Err(CalendarError::MonthOutOfRange));
    }

    #[test]
    fn a_leap_month_has_no_attribution_and_does_not_borrow_the_ordinary_months() {
        assert_eq!(month_index(Month::regular(4)), Ok(3));
        assert_eq!(
            month_index(Month::leap(4)),
            Err(CalendarError::UnsupportedField("leap month"))
        );
    }

    #[test]
    fn a_month_ordinal_outside_one_to_twelve_is_out_of_range() {
        assert_eq!(
            month_index(Month::regular(0)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            month_index(Month::regular(13)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(month_index(Month::regular(1)), Ok(0));
        assert_eq!(month_index(Month::regular(12)), Ok(11));
    }

    #[test]
    fn the_weekday_index_counts_from_sunday_because_the_planetary_week_does() {
        assert_eq!(weekday_index(Weekday::Sunday), 0);
        assert_eq!(weekday_index(Weekday::Monday), 1);
        assert_eq!(weekday_index(Weekday::Saturday), 6);
    }

    #[test]
    fn unanimity_is_order_sensitive_because_the_order_is_the_publishers() {
        let a = table([&["x", "y"], &["z"], &["w"]]);
        let b = table([&["y", "x"], &["z"], &["w"]]);
        assert_eq!(unanimous(&[&a, &b], 0), Ok(false));
        assert_eq!(unanimous(&[&a, &b], 1), Ok(true));
        assert_eq!(disagreement_count(&[&a, &b]), Ok(1));
    }

    #[test]
    fn an_empty_set_of_tables_is_vacuously_unanimous() {
        let none: [&AttributionTable<3>; 0] = [];
        assert_eq!(unanimous(&none, 0), Ok(true));
        assert_eq!(disagreement_count(&none), Ok(0));
    }

    #[test]
    fn a_table_with_an_empty_entry_is_incomplete() {
        assert!(table([&["a"], &["b"], &["c"]]).is_complete());
        assert!(!table([&["a"], &[], &["c"]]).is_complete());
    }

    #[test]
    fn a_table_can_be_asked_where_it_names_something() {
        let subject = table([&["opal"], &["opal", "tourmaline"], &["topaz"]]);
        assert!(subject.names("opal"));
        assert!(!subject.names("Opal"));
        let keys: [usize; 2] = [0, 1];
        assert!(subject.keys_naming("opal").eq(keys));
        assert_eq!(subject.total_attributions(), 4);
    }

    #[test]
    fn an_unknown_validity_span_contains_every_year() {
        assert!(Validity::UNKNOWN.contains(-3000));
        assert!(Validity::UNKNOWN.contains(3000));
        assert!(Validity::UNKNOWN.is_current());
    }

    #[test]
    fn a_closed_validity_span_excludes_the_years_outside_it() {
        let span = Validity::between(1912, 1951);
        assert!(!span.contains(1911));
        assert!(span.contains(1912));
        assert!(span.contains(1951));
        assert!(!span.contains(1952));
        assert!(!span.is_current());
    }

    #[test]
    fn a_date_reports_the_precision_its_source_gave() {
        assert_eq!(AttributionDate::year(725).precision(), 1);
        assert_eq!(AttributionDate::year_month(1912, 8).precision(), 2);
        assert_eq!(AttributionDate::ymd(2021, 12, 20).precision(), 3);
    }

    #[test]
    fn the_latest_date_prefers_the_revision_over_the_establishment() {
        let mut authority = TEST_AUTHORITY;
        assert_eq!(authority.latest_date(), Some(AttributionDate::year(2000)));
        authority.revised = Some(AttributionDate::year(2010));
        assert_eq!(authority.latest_date(), Some(AttributionDate::year(2010)));
    }

    #[test]
    fn a_contested_provenance_warrants_a_caveat_and_a_vernacular_one_does_not() {
        assert!(Provenance::Contested.warrants_a_caveat());
        assert!(Provenance::ModernInvention.warrants_a_caveat());
        assert!(!Provenance::Promulgated.warrants_a_caveat());
        assert!(!Provenance::Vernacular.warrants_a_caveat());
        assert!(!Provenance::Recorded.warrants_a_caveat());
    }
}
