//! A date as a locale writes it, read back: the inverse of
//! [`write_date`](super::write_date).
//!
//! The reader walks the same templates the renderer fills — the entry's,
//! the calendar's notation, the locale's and the default, and inside a
//! date the year's, the first year's and the day's — and at each
//! placeholder tries what the renderer could have written there: every
//! name of every era, month, day and named extra value the locale's chain
//! holds, at every width and in both contexts, and numbers in the
//! locale's digits or in Latin ones. Every complete match is turned into
//! fields, converted to a fixed day and back, and kept only if the
//! calendar's own fields for that day agree with everything the text
//! said. One day is the answer; two are [`DateRefusal::Ambiguous`].
//!
//! `docs/systems/written-dates.md` in the repository explains the
//! matching, the refusals and what is not read, with a worked example.

use core::cell::{Cell, OnceCell, RefCell};
use core::fmt::{self, Write};

use hc_calendar::cycle::Sexagenary;
use hc_calendar::fields::{ExtraField, ExtraFields};
use hc_calendar::shape::MONTH;
use hc_calendar::{CalendarError, DateFields, DynCalendar, Month, Rd, Weekday};
use hc_i18n::Locale;
use hc_i18n::fields;
use hc_i18n::names::{self, NameContext, NameWidth};
use hc_i18n::numbering::{self, NumberingSystem};

use super::{Counter, Mode, Renderer, Spec, parse_placeholder};

/// A date read from its text: the calendar's fields for the day, as
/// [`DynCalendar::fixed_to_fields`] gives them, and the fixed day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedDate {
    /// The calendar's fields for the day, extra fields included.
    pub fields: DateFields,
    /// The fixed day.
    pub fixed: Rd,
}

/// Why a text was not read as a date.
///
/// Every variant has a stable numeric code and a stable name, listed beside
/// it and returned by [`DateRefusal::code`] and [`DateRefusal::name`], as
/// [`CalendarError`]'s have, so that a refusal can cross an ABI. The codes
/// start at 101, so that a refusal to read a text never shares a number
/// with a calendar's own refusal, which [`DateRefusal::NoSuchDate`]
/// passes on under its own code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateRefusal {
    /// The text is empty, or white space.
    ///
    /// Code 101, `empty`.
    Empty,
    /// No way the locale writes the calendar's dates matches the text.
    /// `offset` is the byte offset the furthest match reached.
    ///
    /// Code 102, `not-recognised`.
    NotRecognised {
        /// The byte offset into the text where the furthest match stopped.
        offset: usize,
    },
    /// The text reads as two days or more: an era name the calendar
    /// gives two eras, a repeated day that the text does not mark, a
    /// month name two months share. The two earliest found are named.
    ///
    /// Code 103, `ambiguous`.
    Ambiguous {
        /// One day the text reads as.
        first: Rd,
        /// Another.
        second: Rd,
    },
    /// The year is written in one or two digits, with no era, in a
    /// calendar that also has a year those digits could abbreviate: `26`
    /// may be 26 or 2026, and the reader does not choose a century.
    ///
    /// Code 104, `two-digit-year`.
    TwoDigitYear,
    /// The text does not say which year: it names the year only by its
    /// place in a cycle that recurs, 癸卯年, or not at all, as a Tzolkʼin
    /// day does.
    ///
    /// Code 105, `year-not-written`.
    YearNotWritten,
    /// The text names a weekday that is not the day's.
    ///
    /// Code 106, `weekday-mismatch`.
    WeekdayMismatch {
        /// The weekday the text names.
        written: Weekday,
        /// The day's weekday.
        actual: Weekday,
    },
    /// The text reads as fields the calendar has no day for — 31 February,
    /// an era outside the calendar's range — and the calendar's refusal.
    ///
    /// Its code and name are the calendar error's.
    NoSuchDate(CalendarError),
}

impl DateRefusal {
    /// The stable numeric code: 101 upwards for the reader's own
    /// refusals, and the calendar error's for [`DateRefusal::NoSuchDate`].
    #[must_use]
    pub const fn code(&self) -> u32 {
        match self {
            Self::Empty => 101,
            Self::NotRecognised { .. } => 102,
            Self::Ambiguous { .. } => 103,
            Self::TwoDigitYear => 104,
            Self::YearNotWritten => 105,
            Self::WeekdayMismatch { .. } => 106,
            Self::NoSuchDate(error) => error.code(),
        }
    }

    /// The stable name, lower case and hyphenated; the calendar error's
    /// for [`DateRefusal::NoSuchDate`].
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::NotRecognised { .. } => "not-recognised",
            Self::Ambiguous { .. } => "ambiguous",
            Self::TwoDigitYear => "two-digit-year",
            Self::YearNotWritten => "year-not-written",
            Self::WeekdayMismatch { .. } => "weekday-mismatch",
            Self::NoSuchDate(error) => error.name(),
        }
    }
}

impl fmt::Display for DateRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("the text is empty"),
            Self::NotRecognised { offset } => {
                write!(f, "not a date as the locale writes one, from byte {offset}")
            }
            Self::Ambiguous { first, second } => {
                write!(
                    f,
                    "the text reads as day {} and as day {}",
                    first.0, second.0
                )
            }
            Self::TwoDigitYear => f.write_str("a year of two digits names no century"),
            Self::YearNotWritten => f.write_str("the text does not say which year"),
            Self::WeekdayMismatch { written, actual } => write!(
                f,
                "the text names weekday {} and the day is weekday {}",
                written.iso_number(),
                actual.iso_number()
            ),
            Self::NoSuchDate(error) => write!(f, "the calendar has no such day: {error}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DateRefusal {}

/// Read a date the way `locale` writes it in `calendar`: the text
/// [`write_date`](super::write_date) writes for that locale and calendar,
/// or one a reader wrote the same way.
///
/// `locale` is the locale the date is written in, as the renderer takes
/// it; [`locale_for`](super::locale_for) is the one a rendered cell uses.
/// Beyond what the renderer writes, the reader accepts a name at any
/// width and in either context, a number in Latin digits where the locale
/// has its own, letters in either case, an abbreviation without its
/// period, spaces where the template has none and more than one where it
/// has one, and a weekday before or after the date, which must be the
/// day's.
///
/// ```
/// use hc_calendar::DynAdapter;
/// use hc_format::label;
/// use hc_i18n::Locale;
///
/// let gregorian = DynAdapter::new(hc_calendars_solar::GregorianCalendar);
/// let en = Locale::parse("en")?;
/// let read = label::parse_date(&gregorian, &en, "Monday, September 28, 2026")?;
/// assert_eq!(read.fixed.0, 739_887);
/// assert_eq!(
///     label::parse_date(&gregorian, &en, "September 28, 26"),
///     Err(label::DateRefusal::TwoDigitYear)
/// );
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
///
/// # Errors
///
/// A [`DateRefusal`] saying why the text is not one day.
pub fn parse_date(
    calendar: &dyn DynCalendar,
    locale: &Locale,
    text: &str,
) -> Result<ParsedDate, DateRefusal> {
    hc_core::memo::scope(|| {
        let blank = DateFields::new(0);
        let reader = Reader::new(calendar, &blank, locale, text);
        reader.read()
    })
}

/// What a match has read so far.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Read {
    /// The era written, if one was.
    era: Option<&'static str>,
    /// The year written, if one was.
    year: Option<i64>,
    /// Whether the year was written as a plain number of one or two
    /// digits without a sign.
    short_year: bool,
    /// The month written, if one was.
    month: Option<Month>,
    /// Whether the month was written as a number, which does not say
    /// whether it is the intercalary one.
    month_numbered: bool,
    /// Whether the month's name is the one it has only in a year with the
    /// intercalary month (`Some(true)`), only in one without
    /// (`Some(false)`), or in either.
    leap_year: Option<bool>,
    /// Whether the template had a month, whether or not it was written.
    month_seen: bool,
    /// The day of the month written, if one was.
    day: Option<u8>,
    /// Whether the template had a day of the month.
    day_seen: bool,
    /// The extra fields written.
    extra: ExtraFields,
    /// The weekday written, if one was.
    weekday: Option<Weekday>,
    /// Whether the template had an era, whether or not it was written.
    era_seen: bool,
    /// The levels of the template chain the match took.
    levels: Levels,
}

/// The level of the [`TemplateChain`](hc_i18n::names::TemplateChain)
/// each template a match took came from, so that a reading is kept only
/// where the renderer would have taken the same ones.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Levels {
    /// The date's template.
    date: usize,
    /// The year's, if the date has one.
    year: Option<usize>,
    /// Whether the year's is the template for the first year of an era.
    first_year: bool,
    /// The day's, if the date has one and it was written.
    day: Option<usize>,
}

impl Read {
    fn with_era(mut self, code: &'static str) -> Option<Self> {
        match self.era {
            Some(era) if !era.eq_ignore_ascii_case(code) => None,
            _ => {
                self.era = Some(code);
                Some(self)
            }
        }
    }

    fn with_year(mut self, year: i64, short: bool) -> Option<Self> {
        match self.year {
            Some(known) if known != year => None,
            Some(_) => Some(self),
            None => {
                self.year = Some(year);
                self.short_year = short;
                Some(self)
            }
        }
    }

    fn with_month(mut self, month: Month, numbered: bool, leap_year: Option<bool>) -> Option<Self> {
        self.month_seen = true;
        if let Some(known) = self.month {
            let same = if numbered || self.month_numbered {
                known.ordinal == month.ordinal
            } else {
                known == month
            };
            if !same {
                return None;
            }
            if self.month_numbered && !numbered {
                self.month = Some(month);
                self.month_numbered = false;
            }
        } else {
            self.month = Some(month);
            self.month_numbered = numbered;
        }
        if leap_year.is_some() {
            if self.leap_year.is_some_and(|known| Some(known) != leap_year) {
                return None;
            }
            self.leap_year = leap_year;
        }
        Some(self)
    }

    fn with_day(mut self, day: i64) -> Option<Self> {
        let day = u8::try_from(day).ok().filter(|day| *day > 0)?;
        self.day_seen = true;
        match self.day {
            Some(known) if known != day => None,
            _ => {
                self.day = Some(day);
                Some(self)
            }
        }
    }

    fn with_extra(mut self, field: &'static str, value: i64) -> Option<Self> {
        match self.extra.get(field) {
            Some(known) if known != value => None,
            Some(_) => Some(self),
            None => {
                self.extra.set(field, value).ok()?;
                Some(self)
            }
        }
    }

    const fn seen_month(mut self) -> Self {
        self.month_seen = true;
        self
    }

    const fn seen_day(mut self) -> Self {
        self.day_seen = true;
        self
    }
}

/// The characters a template's separator may be, which the renderer
/// drops where a field beside it is empty.
const SEPARATORS: [char; 3] = [',', '、', '،'];

/// Whether two characters are the same letter, in either case.
fn same_letter(a: char, b: char) -> bool {
    a == b || a.to_lowercase().eq(b.to_lowercase())
}

/// A sink that checks what a name writes against the text, and measures
/// how much of it the name covers: white space in the name covers any run
/// of it in the text, and a period in it may be missing from the text.
struct Prefix<'t> {
    text: &'t str,
    taken: usize,
}

impl Write for Prefix<'_> {
    fn write_str(&mut self, name: &str) -> fmt::Result {
        for character in name.chars() {
            let rest = &self.text[self.taken..];
            if character.is_whitespace() {
                let run = whitespace_len(rest);
                if run == 0 {
                    return Err(fmt::Error);
                }
                self.taken += run;
                continue;
            }
            match rest.chars().next() {
                Some(found) if same_letter(character, found) => self.taken += found.len_utf8(),
                _ if character == '.' => {}
                _ => return Err(fmt::Error),
            }
        }
        Ok(())
    }
}

/// A sink that writes the Latin digits it is given as a number in
/// another system: 9月 as 九月.
struct Renumber<'a, 'o> {
    out: &'a mut Prefix<'o>,
    system: &'static NumberingSystem,
    number: Option<i64>,
    /// Whether any digit was written.
    digits: bool,
}

impl Renumber<'_, '_> {
    fn flush(&mut self) -> fmt::Result {
        if let Some(number) = self.number.take() {
            self.system
                .write_integer(number, &mut *self.out)
                .map_err(|_| fmt::Error)?;
        }
        Ok(())
    }
}

impl Write for Renumber<'_, '_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for character in text.chars() {
            match character
                .to_digit(10)
                .filter(|_| character.is_ascii_digit())
            {
                Some(digit) => {
                    self.digits = true;
                    let number = self.number.unwrap_or(0);
                    self.number = Some(
                        number
                            .checked_mul(10)
                            .and_then(|number| number.checked_add(i64::from(digit)))
                            .ok_or(fmt::Error)?,
                    );
                }
                None => {
                    self.flush()?;
                    self.out.write_char(character)?;
                }
            }
        }
        Ok(())
    }
}

/// The length of the white space a text begins with.
fn whitespace_len(text: &str) -> usize {
    text.len() - text.trim_start().len()
}

/// The names one placeholder found at one place in the text: where each
/// ends and what it stands for, each pair once.
#[derive(Debug, Clone, Copy)]
struct Found<T> {
    items: [Option<(usize, T)>; 32],
    count: usize,
    /// Whether more were found than the list holds.
    overflowed: bool,
}

impl<T: Copy + PartialEq> Found<T> {
    const fn new() -> Self {
        Self {
            items: [None; 32],
            count: 0,
            overflowed: false,
        }
    }

    /// Add `(end, value)`, answering whether it is new.
    fn push(&mut self, end: usize, value: T) -> bool {
        if self.items[..self.count].contains(&Some((end, value))) {
            return false;
        }
        if self.count == self.items.len() {
            self.overflowed = true;
            return false;
        }
        self.items[self.count] = Some((end, value));
        self.count += 1;
        true
    }

    fn for_each(&self, found: &mut dyn FnMut(usize, T)) {
        for (end, value) in self.items[..self.count].iter().flatten() {
            found(*end, *value);
        }
    }
}

/// A [`Found`] list, with its key and its place in the text.
type Kept<T> = (usize, usize, Found<T>);

/// What generates the names of one kind at one place: it hands each to
/// the sink with where it ends.
type Generate<'g, T> = &'g dyn Fn(&mut dyn FnMut(usize, T));

/// The last few [`Found`] lists of one kind, by a key naming the kind
/// and the place in the text.
struct Memo<T> {
    slots: RefCell<[Option<Kept<T>>; 8]>,
    next: Cell<usize>,
}

impl<T: Copy + PartialEq> Memo<T> {
    fn new() -> Self {
        Self {
            slots: RefCell::new([None; 8]),
            next: Cell::new(0),
        }
    }

    fn get(&self, key: usize, pos: usize) -> Option<Found<T>> {
        self.slots
            .borrow()
            .iter()
            .flatten()
            .find(|(known, at, _)| *known == key && *at == pos)
            .map(|(_, _, found)| *found)
    }

    fn put(&self, key: usize, pos: usize, found: Found<T>) {
        let slot = self.next.get();
        self.slots.borrow_mut()[slot] = Some((key, pos, found));
        self.next.set((slot + 1) % 8);
    }
}

/// The lists the reader keeps, by the kind of name.
struct Memos {
    eras: Memo<&'static str>,
    months: Memo<(Month, Option<bool>)>,
    /// Day names, the sexagenary year and extra fields' values, by
    /// [`DAY_NAMES`], [`SEXAGENARY`] and the field's name.
    values: Memo<i64>,
    weekdays: Memo<Weekday>,
}

/// The key of the day names in [`Memos::values`].
const DAY_NAMES: usize = 0;
/// The key of the sexagenary year in [`Memos::values`].
const SEXAGENARY: usize = 1;

/// The fields a calendar's probe days give, for reading a year the text
/// writes only through extra fields.
type Probe = [Option<DateFields>; 3];

/// The years a text's extra fields point to, each with the year either
/// side, for the extra fields that turn at another point of the year.
#[derive(Debug, Default)]
struct YearCandidates {
    years: [i64; 12],
    count: usize,
}

impl YearCandidates {
    fn around(&mut self, year: Option<i64>) {
        let Some(year) = year else {
            return;
        };
        for step in -1..=1 {
            let Some(candidate) = year.checked_add(step) else {
                continue;
            };
            if self.count < self.years.len() && !self.years[..self.count].contains(&candidate) {
                self.years[self.count] = candidate;
                self.count += 1;
            }
        }
    }

    const fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn iter(&self) -> impl Iterator<Item = i64> + '_ {
        self.years[..self.count].iter().copied()
    }
}

/// The year a cycle and a year of the cycle name, where the probes show
/// the calendar counting years as `N·outer + inner + k`: `N` and `k` from
/// the first two probes, and the third, where there is one, agreeing.
fn mixed_radix(
    near: &DateFields,
    far: &DateFields,
    third: Option<&DateFields>,
    outer: ExtraField,
    inner: ExtraField,
) -> Option<i64> {
    let values = |probe: &DateFields| {
        Some((
            probe.year,
            probe.extra.get(outer.name)?,
            probe.extra.get(inner.name)?,
        ))
    };
    let (y1, o1, i1) = values(near)?;
    let (y2, o2, i2) = values(far)?;
    let span = o2.checked_sub(o1).filter(|span| *span != 0)?;
    let rise = (y2 - i2).checked_sub(y1 - i1)?;
    if rise % span != 0 {
        return None;
    }
    let radix = rise / span;
    if radix < 2 {
        return None;
    }
    let offset = y1 - i1 - radix.checked_mul(o1)?;
    // A year of the cycle runs from zero or one to the cycle's length, on
    // every probe and in the text; and three probes must agree, since
    // any two points lie on some line.
    let (y3, o3, i3) = values(third?)?;
    let within = |inner: i64| (0..=radix).contains(&inner);
    if radix.checked_mul(o3)? + i3 + offset != y3
        || ![i1, i2, i3, inner.value].into_iter().all(within)
    {
        return None;
    }
    radix
        .checked_mul(outer.value)?
        .checked_add(inner.value)?
        .checked_add(offset)
}

/// The state of one reading.
struct Reader<'a> {
    renderer: Renderer<'a>,
    text: &'a str,
    english: Locale,
    /// The day read and its fields, and a second day the text reads as.
    found: Cell<Option<(Rd, DateFields)>>,
    second: Cell<Option<Rd>>,
    /// How far into the text a match got.
    furthest: Cell<usize>,
    /// The refusals met on complete matches, by kind.
    two_digit: Cell<bool>,
    year_not_written: Cell<bool>,
    weekday: Cell<Option<(Weekday, Weekday)>>,
    calendar_error: Cell<Option<CalendarError>>,
    probe: OnceCell<Probe>,
    memo: Memos,
    /// The numbering systems numbers are read in: the locale's, Latin,
    /// and for a locale written in Han characters its Han numerals and
    /// its positional Han digits.
    systems: [Option<&'static NumberingSystem>; 4],
    /// The names of the days of the month: the locale's for the calendar,
    /// or for a lunisolar calendar in a locale written in Han characters,
    /// the Chinese calendar's 初一 to 三十.
    day_names: &'static [&'static str],
    /// The last conversions, both ways: matches through different
    /// templates reach the same fields, and a day's neighbours are
    /// converted again by the next match.
    fixed: RefCell<Conversions<DateFields, Rd>>,
    fields: RefCell<Conversions<Rd, DateFields>>,
    leap_years: RefCell<Conversions<(Option<&'static str>, i64), bool>>,
    /// The last readings resolved, which other matches reach again.
    resolved: RefCell<[Option<Read>; 16]>,
    resolved_next: Cell<usize>,
}

/// The last few conversions one way: enough for a month of days, which
/// the reader converts one by one where the text leaves the day to the
/// extra fields.
struct Conversions<K, V> {
    entries: [Option<(K, Result<V, CalendarError>)>; 64],
    next: usize,
}

impl<K: Copy + PartialEq, V: Copy> Conversions<K, V> {
    const fn new() -> Self {
        Self {
            entries: [None; 64],
            next: 0,
        }
    }

    fn get(&self, key: &K) -> Option<Result<V, CalendarError>> {
        self.entries
            .iter()
            .flatten()
            .find(|(known, _)| known == key)
            .map(|(_, value)| *value)
    }

    fn put(&mut self, key: K, value: Result<V, CalendarError>) {
        self.entries[self.next] = Some((key, value));
        self.next = (self.next + 1) % self.entries.len();
    }
}

impl<'a> Reader<'a> {
    fn new(
        calendar: &'a dyn DynCalendar,
        blank: &'a DateFields,
        locale: &'a Locale,
        text: &'a str,
    ) -> Self {
        let renderer = Renderer::new(calendar, blank, locale);
        let han = match names::locale_data(locale).script {
            "Hans" => Some("hans"),
            "Hant" => Some("hant"),
            "Jpan" => Some("jpan"),
            _ => None,
        };
        let mut systems = [Some(renderer.numbering), None, None, None];
        for (slot, id) in [Some("latn"), han, han.map(|_| "hanidec")]
            .into_iter()
            .enumerate()
        {
            let system = id.and_then(NumberingSystem::from_id);
            if system.is_some_and(|system| system.id() != renderer.numbering.id()) {
                systems[slot + 1] = system;
            }
        }
        let day_names = if !renderer.merged.day_names.is_empty() {
            renderer.merged.day_names
        } else if han.is_some() && calendar.meta().has_leap_months {
            names::templates(locale, hc_calendar::CalendarId("chinese"))
                .merged()
                .day_names
        } else {
            &[]
        };
        Self {
            systems,
            day_names,
            renderer,
            text,
            english: names::english(),
            found: Cell::new(None),
            second: Cell::new(None),
            furthest: Cell::new(0),
            two_digit: Cell::new(false),
            year_not_written: Cell::new(false),
            weekday: Cell::new(None),
            calendar_error: Cell::new(None),
            probe: OnceCell::new(),
            memo: Memos {
                eras: Memo::new(),
                months: Memo::new(),
                values: Memo::new(),
                weekdays: Memo::new(),
            },
            fixed: RefCell::new(Conversions::new()),
            fields: RefCell::new(Conversions::new()),
            leap_years: RefCell::new(Conversions::new()),
            resolved: RefCell::new([None; 16]),
            resolved_next: Cell::new(0),
        }
    }

    /// The calendar's fixed day for `fields`, converted once.
    fn to_fixed(&self, fields: &DateFields) -> Result<Rd, CalendarError> {
        if let Some(known) = self.fixed.borrow().get(fields) {
            return known;
        }
        let converted = self.renderer.calendar.fields_to_fixed(fields);
        self.fixed.borrow_mut().put(*fields, converted);
        converted
    }

    /// Whether the year of `fields` has the calendar's intercalary month,
    /// asked once for each era and year: an astronomical calendar answers
    /// it as slowly as it converts a day.
    fn in_leap_year(&self, fields: &DateFields) -> Option<bool> {
        let key = (fields.era, fields.year);
        if let Some(known) = self.leap_years.borrow().get(&key) {
            return known.ok();
        }
        let answer = self.renderer.calendar.is_leap_year_of(fields);
        self.leap_years.borrow_mut().put(key, answer);
        answer.ok()
    }

    /// The calendar's fields for `fixed`, converted once.
    fn to_fields(&self, fixed: Rd) -> Result<DateFields, CalendarError> {
        if let Some(known) = self.fields.borrow().get(&fixed) {
            return known;
        }
        let converted = self.renderer.calendar.fixed_to_fields(fixed);
        self.fields.borrow_mut().put(fixed, converted);
        converted
    }

    fn read(&self) -> Result<ParsedDate, DateRefusal> {
        let start = self.skip_space(0);
        if start == self.text.len() {
            return Err(DateRefusal::Empty);
        }
        self.dates(start, Read::default());
        self.weekdays(start, &|end, weekday| {
            let read = Read {
                weekday: Some(weekday),
                ..Read::default()
            };
            self.dates(self.skip_separators(end), read);
        });
        if self.two_digit.get() {
            return Err(DateRefusal::TwoDigitYear);
        }
        match (self.found.get(), self.second.get()) {
            (Some((first, _)), Some(second)) => Err(DateRefusal::Ambiguous {
                first: Rd(first.0.min(second.0)),
                second: Rd(first.0.max(second.0)),
            }),
            (Some((fixed, fields)), None) => Ok(ParsedDate { fields, fixed }),
            (None, _) => Err(if let Some((written, actual)) = self.weekday.get() {
                DateRefusal::WeekdayMismatch { written, actual }
            } else if self.year_not_written.get() {
                DateRefusal::YearNotWritten
            } else if let Some(error) = self.calendar_error.get() {
                DateRefusal::NoSuchDate(error)
            } else {
                DateRefusal::NotRecognised {
                    offset: self.furthest.get(),
                }
            }),
        }
    }

    /// Match each distinct date template from `pos`.
    fn dates(&self, pos: usize, read: Read) {
        let levels = self.renderer.chain.levels();
        for (index, level) in levels.iter().enumerate() {
            let template = level.date;
            if template.is_empty() || levels[..index].iter().any(|other| other.date == template) {
                continue;
            }
            let mut read = read;
            read.levels.date = index;
            self.walk(template, Mode::Date, pos, read, &|end, read| {
                self.finish(end, read);
            });
        }
    }

    /// A date template matched up to `pos`: the rest of the text may be
    /// white space, a separator and a weekday.
    fn finish(&self, pos: usize, read: Read) {
        let end = self.skip_separators(pos);
        self.reached(end);
        if end == self.text.len() {
            self.resolve(read);
            return;
        }
        if read.weekday.is_none() {
            self.weekdays(end, &|after, weekday| {
                let after = self.skip_separators(after);
                if after == self.text.len() {
                    self.resolve(Read {
                        weekday: Some(weekday),
                        ..read
                    });
                }
            });
        }
    }

    fn reached(&self, pos: usize) {
        if pos > self.furthest.get() {
            self.furthest.set(pos);
        }
    }

    fn skip_space(&self, pos: usize) -> usize {
        pos + whitespace_len(&self.text[pos..])
    }

    /// Skip white space and the separators around it.
    fn skip_separators(&self, mut pos: usize) -> usize {
        loop {
            pos = self.skip_space(pos);
            match self.text[pos..].chars().next() {
                Some(character) if SEPARATORS.contains(&character) => pos += character.len_utf8(),
                _ => return pos,
            }
        }
    }

    /// Match a template's literal text: white space covers any run of it,
    /// a separator may be missing, and every other character must be
    /// there, in either case, with white space before it allowed.
    fn literal(&self, literal: &str, mut pos: usize) -> Option<usize> {
        for character in literal.chars() {
            pos = self.skip_space(pos);
            if character.is_whitespace() {
                continue;
            }
            let found = self.text[pos..].chars().next();
            if SEPARATORS.contains(&character) {
                if found == Some(character) {
                    pos += character.len_utf8();
                }
                continue;
            }
            match found {
                Some(found) if same_letter(character, found) => pos += found.len_utf8(),
                _ => {
                    self.reached(pos);
                    return None;
                }
            }
        }
        self.reached(pos);
        Some(pos)
    }

    /// Match a template from `pos`, and hand every way it matches to
    /// `next`.
    fn walk(
        &self,
        template: &'static str,
        mode: Mode,
        pos: usize,
        read: Read,
        next: &dyn Fn(usize, Read),
    ) {
        let Some(start) = template.find('{') else {
            if let Some(end) = self.literal(template, pos) {
                next(end, read);
            }
            return;
        };
        let after = &template[start + 1..];
        let Some(close) = after.find('}') else {
            if let Some(end) = self.literal(template, pos) {
                next(end, read);
            }
            return;
        };
        let Some(pos) = self.literal(&template[..start], pos) else {
            return;
        };
        let (name, field, spec) = parse_placeholder(&after[..close]);
        let rest = &after[close + 1..];
        let pos = self.skip_space(pos);
        self.fill(name, field, spec, mode, pos, read, &|end, read| {
            self.walk(rest, mode, end, read, next);
        });
    }

    /// Every way a placeholder matches at `pos`, including writing nothing
    /// where the renderer may.
    #[allow(clippy::too_many_arguments)]
    fn fill(
        &self,
        name: &'static str,
        field: &'static str,
        spec: Spec,
        mode: Mode,
        pos: usize,
        read: Read,
        next: &dyn Fn(usize, Read),
    ) {
        let digits = |min: u8, set: &dyn Fn(Read, i64) -> Option<Read>| {
            self.numbers(pos, min, &|end, value, _| {
                if let Some(read) = set(read, value) {
                    next(end, read);
                }
            });
        };
        match (name, spec) {
            ("extra", Spec::Digits(min)) => {
                next(pos, read);
                digits(min, &|read, value| read.with_extra(field, value));
            }
            ("extra", spec) => {
                let _ = spec;
                next(pos, read);
                self.extra_names(field, pos, read, next);
            }
            ("year", Spec::Digits(min)) => digits(min, &|read, value| read.with_year(value, false)),
            ("month", Spec::Digits(min)) => {
                next(pos, read.seen_month());
                digits(min, &|read, value| {
                    let ordinal = u8::try_from(value).ok()?;
                    read.with_month(Month::regular(ordinal), true, None)
                });
            }
            ("day", Spec::Digits(min)) => {
                next(pos, read.seen_day());
                digits(min, &|read, value| read.with_day(value));
            }
            ("day", Spec::ZeroBased) => {
                next(pos, read.seen_day());
                digits(1, &|read, value| read.with_day(value.checked_add(1)?));
            }
            (_, Spec::Digits(_)) => next(pos, read),
            ("year", _) if mode == Mode::Date => self.year_unit(pos, read, next),
            ("day", _) if mode == Mode::Date => {
                next(pos, read.seen_day());
                let levels = self.renderer.chain.levels();
                for (index, level) in levels.iter().enumerate() {
                    let template = level.day;
                    if template.is_empty()
                        || levels[..index].iter().any(|other| other.day == template)
                    {
                        continue;
                    }
                    let mut read = read;
                    read.levels.day = Some(index);
                    self.walk(template, Mode::Unit, pos, read, next);
                }
                // The Chinese day names a lunisolar calendar's data does
                // not give it are written bare, 初一, as the Chinese
                // calendar's are.
                if !self.day_names.is_empty()
                    && self.day_names.as_ptr() != self.renderer.merged.day_names.as_ptr()
                {
                    for (index, name) in self.day_names.iter().enumerate() {
                        if let Some(end) = self.name_at(pos, |out| out.write_str(name))
                            && let Some(read) = read.with_day(i64::try_from(index).unwrap_or(0) + 1)
                        {
                            next(end, read);
                        }
                    }
                }
            }
            ("month", spec) => {
                let _ = spec;
                next(pos, read.seen_month());
                self.months(pos, read, next);
            }
            ("era", _) => {
                let read = Read {
                    era_seen: true,
                    ..read
                };
                next(pos, read);
                self.eras(pos, read, next);
            }
            ("year", _) => {
                // 元, the first year of an era, where one is written.
                if read.era.is_some()
                    && self.text[pos..].starts_with(numbering::FIRST_YEAR_MARKER)
                    && let Some(read) = read.with_year(1, false)
                {
                    next(pos + numbering::FIRST_YEAR_MARKER.len(), read);
                }
                self.numbers(pos, 1, &|end, value, short| {
                    if let Some(read) = read.with_year(value, short) {
                        next(end, read);
                    }
                });
            }
            ("sexagenary", _) => {
                next(pos, read);
                self.sexagenary(pos, read, next);
            }
            ("day", spec) => {
                let _ = spec;
                next(pos, read.seen_day());
                self.days(pos, read, next);
            }
            _ => next(pos, read),
        }
    }

    /// The year of a date: each level's template for the first year of an
    /// era, which writes the year 1 as a word, and for any year.
    fn year_unit(&self, pos: usize, read: Read, next: &dyn Fn(usize, Read)) {
        let levels = self.renderer.chain.levels();
        for (index, level) in levels.iter().enumerate() {
            let first = level.first_year;
            if !first.is_empty()
                && !levels[..index]
                    .iter()
                    .any(|other| other.first_year == first)
                && let Some(mut read) = read.with_year(1, false)
            {
                read.levels.year = Some(index);
                read.levels.first_year = true;
                self.walk(first, Mode::Unit, pos, read, next);
            }
            let year = level.year;
            if !year.is_empty() && !levels[..index].iter().any(|other| other.year == year) {
                let mut read = read;
                read.levels.year = Some(index);
                self.walk(year, Mode::Unit, pos, read, next);
            }
        }
    }

    /// A name, if the text at `pos` begins with what `write` writes: the
    /// end of the name in the text.
    fn name_at(
        &self,
        pos: usize,
        write: impl FnOnce(&mut Prefix<'_>) -> fmt::Result,
    ) -> Option<usize> {
        let mut prefix = Prefix {
            text: &self.text[pos..],
            taken: 0,
        };
        match write(&mut prefix) {
            Ok(()) if prefix.taken > 0 => {
                // An abbreviation may be written with its period where the
                // data has none: *Sep.* for *Sep*.
                let end = pos + prefix.taken;
                let letter = self.text[..end]
                    .chars()
                    .next_back()
                    .is_some_and(char::is_alphabetic);
                Some(if letter && self.text[end..].starts_with('.') {
                    end + 1
                } else {
                    end
                })
            }
            _ => None,
        }
    }

    /// Numbers at `pos`, in each of the reader's numbering systems:
    /// the longest run of a positional system's digits, of at least `min`
    /// digits, and every Han numeral that is one as the system writes it.
    /// `next` is told whether the number is a plain one of one or two
    /// digits.
    fn numbers(&self, pos: usize, min: u8, next: &dyn Fn(usize, i64, bool)) {
        let rest = &self.text[pos..];
        let (sign, body) = match rest.strip_prefix('-') {
            Some(body) => (1, body),
            None => (0, rest),
        };
        for system in self.systems.iter().flatten() {
            match system.digits() {
                Some(digits) => {
                    let run: usize = body
                        .chars()
                        .take_while(|character| digits.contains(character))
                        .map(char::len_utf8)
                        .sum();
                    let count = body[..run].chars().count();
                    if run == 0 || count < usize::from(min.max(1)) {
                        continue;
                    }
                    let end = pos + sign + run;
                    if let Ok(value) = system.parse_integer(&self.text[pos..end]) {
                        next(end, value, sign == 0 && count <= 2);
                    }
                }
                None => self.han_numbers(system, pos, sign, body, next),
            }
        }
    }

    /// Every prefix of `body` that is a Han numeral as `system` writes
    /// it: 二十八, and not 二〇二六 read as six.
    fn han_numbers(
        &self,
        system: &NumberingSystem,
        pos: usize,
        sign: usize,
        body: &str,
        next: &dyn Fn(usize, i64, bool),
    ) {
        let mut end = 0;
        for (count, character) in body.chars().enumerate() {
            if count >= 16 {
                break;
            }
            end += character.len_utf8();
            let stop = pos + sign + end;
            let text = &self.text[pos..stop];
            if let Ok(value) = system.parse_integer(text) {
                let mut written = Prefix { text, taken: 0 };
                if system.write_integer(value, &mut written).is_ok() && written.taken == text.len()
                {
                    next(stop, value, false);
                }
            }
        }
    }

    /// The names a placeholder of one kind has at `pos`, generated once
    /// and kept for the other matches that reach the same place.
    fn candidates<T: Copy + PartialEq>(
        &self,
        memo: &Memo<T>,
        key: usize,
        pos: usize,
        generate: Generate<'_, T>,
        found: &mut dyn FnMut(usize, T),
    ) {
        if let Some(known) = memo.get(key, pos) {
            known.for_each(found);
            return;
        }
        let mut list = Found::new();
        generate(&mut |end, value| {
            list.push(end, value);
        });
        if list.overflowed {
            // Too many to keep: generate them again, straight through.
            let mut sent = Found::new();
            generate(&mut |end, value| {
                if sent.push(end, value) || sent.overflowed {
                    found(end, value);
                }
            });
            return;
        }
        memo.put(key, pos, list);
        list.for_each(found);
    }

    /// The era names the locale, English and the calendar give.
    fn eras(&self, pos: usize, read: Read, next: &dyn Fn(usize, Read)) {
        let generate = |sink: &mut dyn FnMut(usize, &'static str)| {
            let mut offer = |code: &'static str, name: &'static str| {
                if !name.is_empty()
                    && let Some(end) = self.name_at(pos, |out| out.write_str(name))
                {
                    sink(end, code);
                }
            };
            let id = self.renderer.id;
            names::for_each_era_name(self.renderer.locale, id, &mut offer);
            names::for_each_era_name(&self.english, id, &mut offer);
            let calendar = self.renderer.calendar;
            for index in 0.. {
                let Some(code) = calendar.era_code(index) else {
                    break;
                };
                if let Some(own) = calendar.era_name(code) {
                    offer(code, own.native);
                    offer(code, own.romanised);
                }
            }
        };
        self.candidates(&self.memo.eras, 0, pos, &generate, &mut |end, code| {
            if let Some(read) = read.with_era(code) {
                next(end, read);
            }
        });
    }

    /// The month names at `pos`: each month's label at every width and in
    /// both contexts, in a year with and without the intercalary month,
    /// as the renderer writes it — the name, else the calendar's own, else
    /// the number.
    fn months(&self, pos: usize, read: Read, next: &dyn Fn(usize, Read)) {
        let renderer = &self.renderer;
        let Some(count) = renderer
            .calendar
            .cycles()
            .iter()
            .find(|cycle| cycle.kind == MONTH)
            .map(|cycle| cycle.length.maximum())
        else {
            return;
        };
        let generate = |sink: &mut dyn FnMut(usize, (Month, Option<bool>))| {
            let leap_months: &[bool] = if renderer.calendar.meta().has_leap_months {
                &[false, true]
            } else {
                &[false]
            };
            let leap_names = names::has_leap_year_month_names(renderer.locale, renderer.id);
            for ordinal in 1..=u8::try_from(count).unwrap_or(u8::MAX) {
                for leap in leap_months {
                    let month = Month {
                        ordinal,
                        leap: *leap,
                    };
                    for width in NameWidth::ALL {
                        for context in [NameContext::Format, NameContext::Standalone] {
                            let write = |in_leap_year: bool| {
                                self.name_at(pos, |out| {
                                    renderer.write_month_in(
                                        month,
                                        in_leap_year,
                                        width,
                                        context,
                                        out,
                                    )
                                })
                            };
                            let common = write(false)
                                .or_else(|| self.renumbered(pos, month, false, width, context));
                            let in_leap = if leap_names {
                                write(true)
                                    .or_else(|| self.renumbered(pos, month, true, width, context))
                            } else {
                                common
                            };
                            if common == in_leap {
                                if let Some(end) = common {
                                    sink(end, (month, None));
                                }
                                continue;
                            }
                            if let Some(end) = common {
                                sink(end, (month, Some(false)));
                            }
                            if let Some(end) = in_leap {
                                sink(end, (month, Some(true)));
                            }
                        }
                    }
                }
            }
        };
        self.candidates(&self.memo.months, 0, pos, &generate, &mut |end,
                                                                    (
            month,
            leap,
        )| {
            if let Some(read) = read.with_month(month, false, leap) {
                next(end, read);
            }
        });
    }

    /// A month's label whose number is written in another of the
    /// reader's systems than the data's: 九月 for the 9月 of the data.
    fn renumbered(
        &self,
        pos: usize,
        month: Month,
        in_leap_year: bool,
        width: NameWidth,
        context: NameContext,
    ) -> Option<usize> {
        self.systems[1..].iter().flatten().find_map(|system| {
            let mut prefix = Prefix {
                text: &self.text[pos..],
                taken: 0,
            };
            let mut renumber = Renumber {
                out: &mut prefix,
                system,
                number: None,
                digits: false,
            };
            self.renderer
                .write_month_in(month, in_leap_year, width, context, &mut renumber)
                .ok()?;
            if !renumber.digits {
                return None;
            }
            renumber.flush().ok()?;
            (prefix.taken > 0).then_some(pos + prefix.taken)
        })
    }

    /// The days at `pos`: the locale's names for them, their names in a
    /// named cycle, and numbers.
    fn days(&self, pos: usize, read: Read, next: &dyn Fn(usize, Read)) {
        let renderer = &self.renderer;
        let generate = |sink: &mut dyn FnMut(usize, i64)| {
            for (index, name) in self.day_names.iter().enumerate() {
                if let Some(end) = self.name_at(pos, |out| out.write_str(name)) {
                    sink(end, i64::try_from(index).unwrap_or(0) + 1);
                }
            }
            if let Some(named) = fields::values_of(renderer.id, "day") {
                self.value_names("day", named.first, pos, sink);
            }
        };
        let found = |end: usize, value: i64| {
            if let Some(read) = read.with_day(value) {
                next(end, read);
            }
        };
        self.candidates(
            &self.memo.values,
            DAY_NAMES,
            pos,
            &generate,
            &mut |end, value| {
                found(end, value);
            },
        );
        self.numbers(pos, 1, &|end, value, _| found(end, value));
    }

    /// An extra field's value at `pos`: the name of its value where its
    /// values are named, and its number.
    fn extra_names(&self, field: &'static str, pos: usize, read: Read, next: &dyn Fn(usize, Read)) {
        let found = |end: usize, value: i64| {
            if let Some(read) = read.with_extra(field, value) {
                next(end, read);
            }
        };
        if let Some(named) = fields::values_of(self.renderer.id, field) {
            let generate = |sink: &mut dyn FnMut(usize, i64)| {
                self.value_names(field, named.first, pos, sink);
            };
            // Keyed by the field's name, which the table holds once.
            let key = field.as_ptr() as usize;
            self.candidates(&self.memo.values, key, pos, &generate, &mut |end, value| {
                found(end, value);
            });
        }
        self.numbers(pos, 1, &|end, value, _| found(end, value));
    }

    /// Every named value of a field whose name the text has at `pos`.
    fn value_names(&self, field: &str, first: i64, pos: usize, sink: &mut dyn FnMut(usize, i64)) {
        let renderer = &self.renderer;
        for index in 0..4096 {
            let value = first + index;
            let named = fields::write_value_name(
                renderer.locale,
                renderer.id,
                renderer.calendar.cycles(),
                field,
                value,
                NameWidth::Wide,
                NameContext::Format,
                &mut Counter,
            );
            if named != Ok(true) {
                break;
            }
            for width in NameWidth::ALL {
                for context in [NameContext::Format, NameContext::Standalone] {
                    if let Some(end) = self.name_at(pos, |out| {
                        renderer.write_extra_value(field, value, width, context, out)
                    }) {
                        sink(end, value);
                    }
                }
            }
        }
    }

    /// The year's stem and branch at `pos`, in the locale's reading.
    fn sexagenary(&self, pos: usize, read: Read, next: &dyn Fn(usize, Read)) {
        let locale = self.renderer.locale;
        let generate = |sink: &mut dyn FnMut(usize, i64)| {
            let joiner = names::sexagenary_joiner(locale);
            for index in 0..60 {
                let Some((stem, branch)) =
                    names::sexagenary_names(locale, Sexagenary::from_index(index))
                else {
                    return;
                };
                if let Some(end) = self.name_at(pos, |out| {
                    out.write_str(stem)?;
                    out.write_str(joiner)?;
                    out.write_str(branch)
                }) {
                    sink(end, index);
                }
            }
        };
        self.candidates(
            &self.memo.values,
            SEXAGENARY,
            pos,
            &generate,
            &mut |end, index| {
                if let Some(read) = read.with_extra("sexagenary_year", index) {
                    next(end, read);
                }
            },
        );
    }

    /// The weekday names at `pos`, at every width and in both contexts.
    fn weekdays(&self, pos: usize, next: &dyn Fn(usize, Weekday)) {
        let locale = self.renderer.locale;
        let generate = |sink: &mut dyn FnMut(usize, Weekday)| {
            for weekday in Weekday::ALL {
                for width in NameWidth::ALL {
                    for context in [NameContext::Format, NameContext::Standalone] {
                        if let Some(name) = names::weekday_name(locale, weekday, width, context)
                            && let Some(end) = self.name_at(pos, |out| out.write_str(name))
                        {
                            sink(end, weekday);
                        }
                    }
                }
            }
        };
        self.candidates(
            &self.memo.weekdays,
            0,
            pos,
            &generate,
            &mut |end, weekday| {
                next(end, weekday);
            },
        );
    }

    /// A complete match: turn what it read into days, once for each
    /// distinct reading.
    fn resolve(&self, read: Read) {
        {
            let mut resolved = self.resolved.borrow_mut();
            if resolved.contains(&Some(read)) {
                return;
            }
            let slot = self.resolved_next.get();
            resolved[slot] = Some(read);
            self.resolved_next.set((slot + 1) % resolved.len());
        }
        if read.short_year
            && read.era.is_none()
            && (read.month.is_some() || read.day.is_some())
            && self.abbreviates(&read)
        {
            self.two_digit.set(true);
            return;
        }
        let implied = self.renderer.merged.implied_era;
        let eras: [Option<&'static str>; 2] = match read.era {
            Some(era) => [Some(era), None],
            None if implied.is_empty() => [None, None],
            None => [None, Some(implied)],
        };
        let months: [Option<Month>; 2] = match read.month {
            Some(month) if read.month_numbered => [Some(month), Some(Month::leap(month.ordinal))],
            month => [month, None],
        };
        for (era_index, era) in eras.iter().enumerate() {
            if era_index == 1 && era.is_none() {
                continue;
            }
            for (month_index, month) in months.iter().enumerate() {
                if month_index == 1 && month.is_none() {
                    continue;
                }
                for leap_day in [false, true] {
                    let mut fields = DateFields::new(read.year.unwrap_or(0));
                    fields.era = *era;
                    fields.month = *month;
                    fields.day = read.day;
                    fields.leap_day = leap_day;
                    fields.extra = read.extra;
                    if read.year.is_some() {
                        let days = self.solve(&fields, &read, true);
                        self.record(&days, &read);
                    } else {
                        self.unwritten_year(fields, &read);
                    }
                }
            }
        }
    }

    /// A year the text does not write: from the extra fields that count
    /// years with it — an era's year, a cycle and a year of the cycle —
    /// else only if the day does not depend on the year.
    fn unwritten_year(&self, fields: DateFields, read: &Read) {
        let probes = self.probe.get_or_init(|| self.probe());
        let mut years = YearCandidates::default();
        if let [Some(near), Some(far), third] = probes {
            let probes: [&DateFields; 2] = [near, far];
            for extra in read.extra.iter() {
                // An era's year, which may turn at another point of the
                // year than the calendar's: the offset holds within one.
                let offsets = probes.map(|probe| {
                    probe
                        .extra
                        .get(extra.name)
                        .and_then(|value| value.checked_sub(probe.year))
                });
                if let [Some(a), Some(b)] = offsets
                    && a.abs_diff(b) <= 1
                    && third.is_none_or(|third| {
                        third
                            .extra
                            .get(extra.name)
                            .is_some_and(|value| (value - third.year).abs_diff(a) <= 1)
                    })
                {
                    years.around(extra.value.checked_sub(a));
                }
                // A cycle and a year of the cycle: year = N·cycle + year
                // of the cycle + k, measured on two probes and checked on
                // the third.
                for inner in read.extra.iter() {
                    if inner.name == extra.name {
                        continue;
                    }
                    if let Some(year) = mixed_radix(near, far, third.as_ref(), extra, inner) {
                        years.around(Some(year));
                    }
                }
            }
        }
        if !years.is_empty() {
            for year in years.iter() {
                let mut attempt = fields;
                attempt.year = year;
                let days = self.solve(&attempt, read, true);
                self.record(&days, read);
            }
            return;
        }
        let base = probes[0].map_or(0, |probe| probe.year);
        let mut found = [Days::default(), Days::default()];
        for (index, year) in [base, base + 1].into_iter().enumerate() {
            let mut attempt = fields;
            attempt.year = year;
            found[index] = self.solve(&attempt, read, false);
        }
        if found[0].same_as(&found[1]) {
            self.record(&found[0], read);
        } else {
            self.year_not_written.set(true);
        }
    }

    /// The calendar's fields on its sample day and on two days far enough
    /// from it, and from each other, that an extra field running in a
    /// cycle of years has come round: what an extra field that counts
    /// years is measured against.
    fn probe(&self) -> Probe {
        let calendar = self.renderer.calendar;
        let meta = calendar.meta();
        let near = meta.sample_day(Rd(739_617));
        let Ok(fields) = calendar.fixed_to_fields(near) else {
            return [None, None, None];
        };
        let mut probes = [Some(fields), None, None];
        let mut slot = 1;
        for step in [400_000, -400_000, 40_000, -40_000, 4_000, -4_000, 800, -800] {
            if slot == probes.len() {
                break;
            }
            let Some(day) = near.0.checked_add(step).map(Rd) else {
                continue;
            };
            if !meta.supports(day) {
                continue;
            }
            if let Ok(far) = calendar.fixed_to_fields(day)
                && probes[..slot]
                    .iter()
                    .flatten()
                    .all(|probe| probe.year.abs_diff(far.year) >= 2)
            {
                probes[slot] = Some(far);
                slot += 1;
            }
        }
        probes
    }

    /// Whether a year of one or two digits could abbreviate a longer one:
    /// the calendar's years are written in three digits or more on its
    /// sample day, and it has a year a hundred or four hundred on from
    /// the one read with the same month and day.
    fn abbreviates(&self, read: &Read) -> bool {
        let Some(year) = read.year else {
            return false;
        };
        let [near, ..] = *self.probe.get_or_init(|| self.probe());
        if near.is_none_or(|near| near.year.unsigned_abs() < 100) {
            return false;
        }
        let implied = self.renderer.merged.implied_era;
        let calendar = self.renderer.calendar;
        [100, 400].into_iter().any(|step| {
            let mut fields = DateFields::new(year + step);
            fields.month = read.month;
            fields.day = read.day;
            fields.extra = read.extra;
            calendar.fields_to_fixed(&fields).is_ok() || {
                fields.era = (!implied.is_empty()).then_some(implied);
                calendar.fields_to_fixed(&fields).is_ok()
            }
        })
    }

    /// The days `fields` read as: converted, and where the calendar needs
    /// a day or a month the text does not write, with each in turn.
    fn solve(&self, fields: &DateFields, read: &Read, year: bool) -> Days {
        let mut days = Days::default();
        match self.attempt(fields, read, year, &mut days) {
            Err(CalendarError::MissingField("day")) if !read.day_seen => {
                let mut first = *fields;
                first.day = Some(1);
                first.extra = ExtraFields::new();
                first.leap_day = false;
                let length = self
                    .renderer
                    .calendar
                    .days_in_month(&first)
                    .map_or(u8::MAX, |length| u8::try_from(length).unwrap_or(u8::MAX));
                for day in 1..=length {
                    let mut attempt = *fields;
                    attempt.day = Some(day);
                    let _ = self.attempt(&attempt, read, year, &mut days);
                }
            }
            Err(CalendarError::MissingField("month")) if !read.month_seen => {
                for ordinal in 1..=19 {
                    for month in [Month::regular(ordinal), Month::leap(ordinal)] {
                        let mut attempt = *fields;
                        attempt.month = Some(month);
                        let _ = self.attempt(&attempt, read, year, &mut days);
                    }
                }
            }
            _ => {}
        }
        days
    }

    /// Convert `fields` and back, and keep the day if the calendar's
    /// fields for it agree with what the text said, with the day before
    /// and the day after where the text reads as them too, where it writes no day: a week the
    /// text names by its number, a lunar day that the calendar repeats.
    fn attempt(
        &self,
        fields: &DateFields,
        read: &Read,
        year: bool,
        days: &mut Days,
    ) -> Result<(), CalendarError> {
        let calendar = self.renderer.calendar;
        let fixed = self.to_fixed(fields).inspect_err(|error| {
            if self.calendar_error.get().is_none() {
                self.calendar_error.set(Some(*error));
            }
        })?;
        let own = self.to_fields(fixed)?;
        if !self.agrees(&own, fields, read, year, true) || !self.chosen(&own, read) {
            return Ok(());
        }
        days.add(fixed, own);
        let meta = calendar.meta();
        // Two days with the same fields would convert to one, so only a
        // text that leaves the day unwritten can name its neighbour too.
        let day_written = read.day.is_some();
        for step in [-1, 1] {
            if day_written {
                break;
            }
            let beside = Rd(fixed.0.saturating_add(step));
            if meta.supports(beside)
                && let Ok(fields) = self.to_fields(beside)
                && self.agrees(&fields, &own, read, year, false)
            {
                days.add(beside, fields);
            }
        }
        // A flag of the day's that the text does not write, the Burmese
        // late month, may name another day the text reads as too.
        for extra in own.extra.iter() {
            let flag = fields::values_of(self.renderer.id, extra.name)
                .is_some_and(|named| matches!(named.names, fields::ValueNames::Flag));
            if !flag || read.extra.get(extra.name).is_some() {
                continue;
            }
            let mut flipped = own;
            if flipped.extra.set(extra.name, 1 - extra.value).is_err() {
                continue;
            }
            if let Ok(other) = self.to_fixed(&flipped)
                && other != fixed
                && let Ok(fields) = self.to_fields(other)
                && self.agrees(&fields, &own, read, year, false)
            {
                days.add(other, fields);
            }
        }
        Ok(())
    }

    /// Whether the renderer, writing `own`, takes the templates the match
    /// took: no level before the date's, the year's or the day's says
    /// anything for these fields, and the year of an era's first year is
    /// written by its own template where one says something.
    fn chosen(&self, own: &DateFields, read: &Read) -> bool {
        let renderer = Renderer::new(self.renderer.calendar, own, self.renderer.locale);
        if names::has_leap_year_month_names(self.renderer.locale, self.renderer.id) {
            let _ = renderer
                .in_leap_year
                .set(self.in_leap_year(own).unwrap_or(false));
        }
        let levels = renderer.chain.levels();
        let says = |template: &'static str, mode: Mode| {
            !template.is_empty()
                && renderer
                    .write_template(template, mode, &mut Counter)
                    .is_ok_and(|filled| filled > 0)
        };
        let chosen = read.levels;
        if levels[..chosen.date]
            .iter()
            .any(|level| says(level.date, Mode::Date))
        {
            return false;
        }
        if let Some(year) = chosen.year {
            let first_years = own.era.is_some() && own.year == 1;
            let earlier = &levels[..year];
            let passed_over = if chosen.first_year {
                earlier
                    .iter()
                    .any(|level| says(level.first_year, Mode::Unit))
            } else {
                (first_years
                    && levels
                        .iter()
                        .any(|level| says(level.first_year, Mode::Unit)))
                    || earlier.iter().any(|level| says(level.year, Mode::Unit))
            };
            if passed_over {
                return false;
            }
        }
        if let Some(day) = chosen.day
            && own.day.is_some()
            && levels[..day]
                .iter()
                .any(|level| says(level.day, Mode::Unit))
        {
            return false;
        }
        true
    }

    /// Whether a day's fields, `own`, agree with the fields `expected` in
    /// everything the text wrote: the era, the year when `year` asks, the
    /// month and the day where the template has them, the leap day when
    /// `leap_day` asks, every extra field read, and the kind of year a
    /// month's name implies.
    fn agrees(
        &self,
        own: &DateFields,
        expected: &DateFields,
        read: &Read,
        year: bool,
        leap_day: bool,
    ) -> bool {
        let era = match (own.era, expected.era) {
            (Some(own), Some(expected)) => own.eq_ignore_ascii_case(expected),
            (None, None) => true,
            // A template without an era leaves it to the calendar.
            (Some(_), None) => !read.era_seen && read.era.is_none(),
            (None, Some(_)) => false,
        };
        era && (!year || own.year == expected.year)
            && (!read.month_seen && read.month.is_none() || own.month == expected.month)
            && (!read.day_seen && read.day.is_none() || own.day == expected.day)
            && (!leap_day || own.leap_day == expected.leap_day)
            && read
                .extra
                .iter()
                .all(|extra| own.extra.get(extra.name) == Some(extra.value))
            && read
                .leap_year
                .is_none_or(|leap| self.in_leap_year(own) == Some(leap))
    }

    /// Keep the days a match reads as, those of the weekday the text
    /// names where it names one; a second day makes the text ambiguous.
    fn record(&self, days: &Days, read: &Read) {
        for (fixed, fields) in days.days.iter().flatten() {
            if let Some(weekday) = read.weekday {
                let actual = Weekday::from_rd(*fixed);
                if actual != weekday {
                    self.weekday.set(Some((weekday, actual)));
                    continue;
                }
            }
            match self.found.get() {
                None => self.found.set(Some((*fixed, *fields))),
                Some((known, _)) if known != *fixed && self.second.get().is_none() => {
                    self.second.set(Some(*fixed));
                }
                Some(_) => {}
            }
        }
    }
}

/// Up to two of the days one set of fields reads as.
#[derive(Debug, Clone, Copy, Default)]
struct Days {
    days: [Option<(Rd, DateFields)>; 2],
}

impl Days {
    fn add(&mut self, fixed: Rd, fields: DateFields) {
        for slot in &mut self.days {
            match slot {
                Some((known, _)) if *known == fixed => return,
                Some(_) => {}
                None => {
                    *slot = Some((fixed, fields));
                    return;
                }
            }
        }
    }

    /// Whether two attempts read as the same days, and as some.
    fn same_as(&self, other: &Self) -> bool {
        let fixed = |days: &Self| days.days.map(|day| day.map(|(fixed, _)| fixed));
        self.days[0].is_some() && fixed(self) == fixed(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::DynAdapter;

    fn read(tag: &str, text: &str) -> Result<i64, DateRefusal> {
        let gregorian = DynAdapter::new(hc_calendars_solar::GregorianCalendar);
        let locale = Locale::parse(tag).unwrap();
        parse_date(&gregorian, &locale, text).map(|parsed| parsed.fixed.0)
    }

    /// 28 September 2026 as the Gregorian calendar is written, and as a
    /// reader writes it.
    #[test]
    fn a_gregorian_date_reads_in_the_locales_forms() {
        for (tag, text) in [
            ("en", "September 28, 2026"),
            ("en", "SEPTEMBER 28 2026"),
            ("en", "Mon, Sep 28, 2026"),
            ("de", "28. September 2026"),
            ("ja", "2026年9月28日"),
            ("ja", "二〇二六年九月二十八日"),
            ("zh-Hant", "2026年9月28日"),
            ("ar", "٢٨ سبتمبر ٢٠٢٦"),
            ("fa", "۲۸ سپتامبر ۲۰۲۶"),
        ] {
            assert_eq!(read(tag, text), Ok(739_887), "{tag} {text}");
        }
    }

    #[test]
    fn every_refusal_has_its_code_and_name() {
        assert_eq!(read("en", " \t"), Err(DateRefusal::Empty));
        assert_eq!(
            read("en", "September 28, 2026!"),
            Err(DateRefusal::NotRecognised { offset: 18 })
        );
        assert_eq!(read("en", "Sep 28, 26"), Err(DateRefusal::TwoDigitYear));
        assert!(matches!(
            read("en", "Sunday, September 28, 2026"),
            Err(DateRefusal::WeekdayMismatch { .. })
        ));
        assert_eq!(
            read("en", "September 31, 2026"),
            Err(DateRefusal::NoSuchDate(CalendarError::DayOutOfRange))
        );
        let refusals = [
            DateRefusal::Empty,
            DateRefusal::NotRecognised { offset: 0 },
            DateRefusal::Ambiguous {
                first: Rd(1),
                second: Rd(2),
            },
            DateRefusal::TwoDigitYear,
            DateRefusal::YearNotWritten,
            DateRefusal::WeekdayMismatch {
                written: Weekday::Monday,
                actual: Weekday::Sunday,
            },
        ];
        let codes: alloc::vec::Vec<u32> = refusals.iter().map(DateRefusal::code).collect();
        assert_eq!(codes, [101, 102, 103, 104, 105, 106]);
        for refusal in refusals {
            assert!(!refusal.to_string().is_empty());
            assert!(
                refusal
                    .name()
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b == b'-')
            );
        }
        let own = DateRefusal::NoSuchDate(CalendarError::MonthOutOfRange);
        assert_eq!((own.code(), own.name()), (2, "month-out-of-range"));
    }

    /// A cycle of 52 years and a year of it: the year is 52 × cycle + the
    /// year of the cycle + 3, on the probes and in the text; a pair that
    /// does not hold on the third probe is no count at all.
    #[test]
    fn a_cycle_and_a_year_of_it_count_years() {
        let probe = |year: i64, cycle: i64, of_cycle: i64| {
            let mut fields = DateFields::new(year);
            fields.extra.set("cycle", cycle).unwrap();
            fields.extra.set("year-of-cycle", of_cycle).unwrap();
            fields
        };
        let near = probe(52 * 38 + 10 + 3, 38, 10);
        let far = probe(52 * 60 + 1 + 3, 60, 1);
        let third = probe(52 * 40 + 51 + 3, 40, 51);
        let outer = ExtraField {
            name: "cycle",
            value: 39,
        };
        let inner = ExtraField {
            name: "year-of-cycle",
            value: 7,
        };
        assert_eq!(
            mixed_radix(&near, &far, Some(&third), outer, inner),
            Some(52 * 39 + 7 + 3)
        );
        let wrong = probe(52 * 40 + 50 + 3, 40, 51);
        assert_eq!(mixed_radix(&near, &far, Some(&wrong), outer, inner), None);
        assert_eq!(mixed_radix(&near, &far, None, outer, inner), None);
    }
}
