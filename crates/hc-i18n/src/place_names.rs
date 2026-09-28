//! What a locale calls a place, as data: Unicode CLDR 48's names of every
//! territory and every ISO 3166-2 subdivision it names, in every locale
//! this crate carries.
//!
//! The system document `docs/systems/place-names.md` explains the data and
//! the lookup with examples; this is its summary.
//!
//! # The places
//!
//! Two lists, each in code order: the 295 **territories** CLDR 48's
//! `en.xml` names — the ISO 3166-1 countries, the UN M.49 areas such as
//! `001` (the world) and `419`, and CLDR's own `EU`, `EZ`, `UN`, `QO`,
//! `XA`, `XB` and `ZZ` — and the 5 503 **subdivisions** a carried locale's
//! file in `common/subdivisions/` names: the 5 027 codes CLDR's validity
//! data holds as `regular`, every one of which `en.xml` names, and 476 it
//! holds as `deprecated`. A code is kept in the form ISO 3166-2 writes it,
//! `JP-13`: CLDR writes `jp13`, the region's two letters and the suffix in
//! lower case with no hyphen, and the generator puts the hyphen back and
//! the letters in upper case ([`Place::code`]). Each place carries CLDR's
//! validity status ([`Status`]). The files are the `release-48` tag of
//! <https://github.com/unicode-org/cldr>, read 2026-09-28
//! [cldr48-territory-names] [cldr48-subdivision-names] [cldr48-validity].
//!
//! # The names
//!
//! A name is CLDR's plain value (no `alt` form) at the draft levels
//! `approved`, `contributed` and `provisional`, and the [`PlaceName`]
//! says which ([`Draft`]). TR35 lets an implementation "accept the
//! provisional data, especially if there is no translated alternative",
//! and the subdivision names are provisional in every locale but English,
//! except England, Scotland and Wales. An `unconfirmed` value is left out,
//! and so is the inheritance marker `↑↑↑`.
//!
//! # The lookup
//!
//! [`Place::name_in`] and [`named`] follow CLDR's inheritance:
//!
//! 1. the first table in the locale's [`Locale::fallback`] chain — `de-AT`
//!    finds `de`, `zh-TW` finds `zh-Hant`;
//! 2. that table's CLDR parents that are tables themselves: `pt-PT`'s is
//!    `pt`. Every other carried locale's parent is root, which names no
//!    place, `zh-Hant`'s and `yue-Hans`'s included, as CLDR's
//!    `parentLocales` has it;
//! 3. English, `en.xml`, the fallback locale TR35's inherited item lookup
//!    allows before root (Part 1, "Inheritance and Validity",
//!    [uts35-v48]);
//! 4. nothing: CLDR's own last resort is the code itself, which the caller
//!    holds.
//!
//! The [`PlaceName`] names the table that answered, so that `ja` for 東京都
//! and `en` for a name English supplied are told apart.
//!
//! # Compact by design
//!
//! The subdivision names are some 2.6 MB of text, so the module is behind
//! the `place-names` feature. A table keeps only what differs from
//! English: a bit per code saying whether the locale's files name it, and
//! a line per named code, empty where the name is English's, so that a
//! locale's own name that happens to be English's still answers under the
//! locale's tag. The file is `place_names/cldr48.rs`, which
//! `scripts/place-names-cldr.py` generates and checks.

use core::ops::Range;
use core::str::Split;

use hc_core::catalogue::matches;

use crate::locale::Locale;

mod cldr48;

use cldr48::{SUBDIVISION_CODES, SUBDIVISION_STATUS, TABLES, TERRITORY_CODES, TERRITORY_STATUS};

/// Where the names come from, for a `source` cell.
pub const SOURCE: &str = "Unicode CLDR 48, common/main/<locale>.xml localeDisplayNames/territories \
     and common/subdivisions/<locale>.xml, approved, contributed and provisional values, \
     release-48, read 2026-09-28 (cldr48-territory-names, cldr48-subdivision-names, \
     cldr48-validity)";

/// The tag of the English table, the fallback of every lookup.
pub const ENGLISH: &str = "en";

/// Which list a place is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    /// A territory of `common/main/<locale>.xml`: a country, a UN M.49
    /// area or one of CLDR's own groupings.
    Territory,
    /// An ISO 3166-2 subdivision of `common/subdivisions/<locale>.xml`.
    Subdivision,
}

impl Kind {
    /// The fixed width of a code in the generated list.
    const fn width(self) -> usize {
        match self {
            Self::Territory => 3,
            Self::Subdivision => 6,
        }
    }

    /// The generated list of codes, each padded to [`Self::width`].
    const fn codes(self) -> &'static str {
        match self {
            Self::Territory => TERRITORY_CODES,
            Self::Subdivision => SUBDIVISION_CODES,
        }
    }

    /// How many places the list holds.
    #[must_use]
    pub const fn len(self) -> usize {
        self.codes().len() / self.width()
    }

    /// Whether the list is empty, which neither is.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// The code at `index`, without its padding.
    fn code_at(self, index: usize) -> &'static str {
        let width = self.width();
        self.codes()
            .get(index * width..(index + 1) * width)
            .map_or("", str::trim_end)
    }

    /// The position of the code `given` names, matched as
    /// [`hc_core::catalogue::matches`] matches: white space around it
    /// ignored, ASCII letters in either case.
    fn index_of(self, given: &str) -> Option<usize> {
        let wanted = given.trim();
        let mut low = 0;
        let mut high = self.len();
        while low < high {
            let middle = low + (high - low) / 2;
            let code = self.code_at(middle);
            match code
                .bytes()
                .cmp(wanted.bytes().map(|byte| byte.to_ascii_uppercase()))
            {
                core::cmp::Ordering::Less => low = middle + 1,
                core::cmp::Ordering::Greater => high = middle,
                core::cmp::Ordering::Equal => return matches(given, code).then_some(middle),
            }
        }
        None
    }

    /// This list's names in a table.
    const fn names(self, table: &'static Table) -> &'static Names {
        match self {
            Self::Territory => &table.territories,
            Self::Subdivision => &table.subdivisions,
        }
    }
}

/// How far CLDR's vetting went for a value: its `draft` attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Draft {
    /// "fully approved by the technical committee".
    Approved,
    /// "partially approved by the technical committee".
    Contributed,
    /// "partially confirmed"; carried because TR35 lets an implementation
    /// accept it where there is no translated alternative.
    Provisional,
}

impl Draft {
    /// The attribute's value: `approved`, `contributed` or `provisional`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Contributed => "contributed",
            Self::Provisional => "provisional",
        }
    }
}

/// A code's status in CLDR's validity data (`common/validity/region.xml`
/// and `subdivision.xml`), its `idStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    /// A code in current use: every country, and every subdivision
    /// ISO 3166-2 lists today.
    Regular,
    /// A code CLDR no longer uses: for a subdivision, one ISO removed or
    /// replaced, which "remains valid in CLDR, though marked as
    /// deprecated".
    Deprecated,
    /// A UN M.49 area or a grouping of regions: `001`, `419`, `EU`.
    Macroregion,
    /// A code CLDR reserves for a pseudo-locale: `XA`, `XB`.
    Special,
    /// The unknown region, `ZZ`.
    Unknown,
}

impl Status {
    /// The status as the validity data spells it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::Deprecated => "deprecated",
            Self::Macroregion => "macroregion",
            Self::Special => "special",
            Self::Unknown => "unknown",
        }
    }

    /// The status a byte of the generated data stands for. The generator
    /// writes only the five letters here, which a test holds it to.
    const fn from_byte(byte: u8) -> Self {
        match byte {
            b'r' => Self::Regular,
            b'd' => Self::Deprecated,
            b'm' => Self::Macroregion,
            b's' => Self::Special,
            _ => Self::Unknown,
        }
    }
}

/// One locale's names of one list: which codes its files name, and the
/// text of those that differ from English.
#[derive(Debug)]
struct Names {
    /// A bit per code, least significant first, set where the files name
    /// the code; the bytes after the last set bit are left out.
    named: &'static [u8],
    /// A line per set bit, in code order, each ending in a line feed:
    /// the name, or nothing where it is English's.
    text: &'static str,
    /// The draft level of most of the names.
    draft: Draft,
    /// The names of another draft level, by index, in index order.
    exceptions: &'static [(u16, Draft)],
}

impl Names {
    /// A list the locale names nothing of.
    const EMPTY: Self = Self {
        named: &[],
        text: "",
        draft: Draft::Approved,
        exceptions: &[],
    };

    /// Whether the files name the code at `index`.
    fn names(&self, index: usize) -> bool {
        self.named
            .get(index / 8)
            .is_some_and(|byte| byte & (1 << (index % 8)) != 0)
    }

    /// How many codes before `index` the files name: the line the code at
    /// `index` has, when they name it.
    fn rank(&self, index: usize) -> usize {
        let whole = (index / 8).min(self.named.len());
        let mut count = self.named[..whole]
            .iter()
            .map(|byte| byte.count_ones() as usize)
            .sum::<usize>();
        if let Some(byte) = self.named.get(index / 8) {
            let below = (1_u16 << (index % 8)) - 1;
            count += (u16::from(*byte) & below).count_ones() as usize;
        }
        count
    }

    /// The draft level of the name at `index`.
    fn draft_at(&self, index: usize) -> Draft {
        self.exceptions
            .binary_search_by_key(&index, |(at, _)| usize::from(*at))
            .map_or(self.draft, |found| self.exceptions[found].1)
    }
}

/// One carried locale's names.
#[derive(Debug)]
struct Table {
    /// The BCP 47 tag, spelled as [`crate::data::LOCALES`] spells it.
    tag: &'static str,
    /// The table of the locale's CLDR parent, where that is a table:
    /// `pt` for `pt-PT`.
    parent: Option<&'static str>,
    territories: Names,
    subdivisions: Names,
}

/// The table for a data tag.
fn table(tag: &str) -> Option<&'static Table> {
    TABLES.iter().find(|table| table.tag == tag)
}

/// The tag of every table, in tag order.
pub fn tags() -> impl Iterator<Item = &'static str> {
    TABLES.iter().map(|table| table.tag)
}

/// One territory or subdivision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Place {
    kind: Kind,
    index: usize,
}

impl Place {
    /// Which list the place is in.
    #[must_use]
    pub const fn kind(self) -> Kind {
        self.kind
    }

    /// The place's position in its list, in code order.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }

    /// The code as ISO writes it: `JP` or `001` for a territory, `JP-13`
    /// for a subdivision, whose CLDR id is `jp13`.
    #[must_use]
    pub fn code(self) -> &'static str {
        self.kind.code_at(self.index)
    }

    /// The country a subdivision is in, its code's first two letters, or
    /// a territory's own code.
    #[must_use]
    pub fn country(self) -> &'static str {
        let code = self.code();
        match self.kind {
            Kind::Territory => code,
            Kind::Subdivision => code.get(..2).unwrap_or(code),
        }
    }

    /// CLDR's validity status of the code.
    #[must_use]
    pub fn status(self) -> Status {
        let marks = match self.kind {
            Kind::Territory => TERRITORY_STATUS,
            Kind::Subdivision => SUBDIVISION_STATUS,
        };
        Status::from_byte(marks.get(self.index).copied().unwrap_or(b'u'))
    }

    /// What `en.xml` calls the place, if it names it: every territory, and
    /// every subdivision but 104 deprecated ones only other locales name.
    #[must_use]
    pub fn english_name(self) -> Option<&'static str> {
        let english = table(ENGLISH)?;
        let names = self.kind.names(english);
        if !names.names(self.index) {
            return None;
        }
        names.text.split('\n').nth(names.rank(self.index))
    }

    /// What `locale` calls the place, by the lookup the module
    /// documentation gives; `None` for `locale` names no locale, as
    /// `native` does, which asks for English. `None` when neither the
    /// locale nor English names it.
    #[must_use]
    pub fn name_in(self, locale: Option<&Locale>) -> Option<PlaceName> {
        named(Places::from(self), locale)
            .next()
            .and_then(|named| named.name)
    }
}

/// A place's name and where it came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaceName {
    /// What the locale calls the place.
    pub name: &'static str,
    /// The tag of the table that answered: `ja`, `pt` for a request for
    /// `pt-PT`, or [`ENGLISH`].
    pub tag: &'static str,
    /// The draft level of the value in that table's files.
    pub draft: Draft,
}

/// A run of places of one list, in code order.
#[derive(Debug, Clone)]
pub struct Places {
    kind: Kind,
    indices: Range<usize>,
}

impl Iterator for Places {
    type Item = Place;

    fn next(&mut self) -> Option<Place> {
        let index = self.indices.next()?;
        Some(Place {
            kind: self.kind,
            index,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}

impl ExactSizeIterator for Places {}

impl From<Place> for Places {
    /// The run of one place.
    fn from(place: Place) -> Self {
        Self {
            kind: place.kind,
            indices: place.index..place.index + 1,
        }
    }
}

/// Every territory, in code order.
#[must_use]
pub fn territories() -> Places {
    Places {
        kind: Kind::Territory,
        indices: 0..Kind::Territory.len(),
    }
}

/// Every subdivision, in code order.
#[must_use]
pub fn subdivisions() -> Places {
    Places {
        kind: Kind::Subdivision,
        indices: 0..Kind::Subdivision.len(),
    }
}

/// The subdivisions of `country`, a territory's code in any case: those
/// whose code begins with it and a hyphen. `None` when `country` is not a
/// territory; a territory with no subdivision, `AQ` or `001`, has none.
#[must_use]
pub fn subdivisions_of(country: &str) -> Option<Places> {
    let country = territory(country)?.code();
    let codes = Kind::Subdivision;
    let starts = |index: usize| codes.code_at(index).get(..2) >= Some(country);
    let within = |index: usize| codes.code_at(index).get(..2) > Some(country);
    let start = partition(codes.len(), starts);
    let end = partition(codes.len(), within);
    Some(Places {
        kind: Kind::Subdivision,
        indices: start..end.max(start),
    })
}

/// The first index in `0..len` where `past` holds, for a `past` that is
/// false and then true.
fn partition(len: usize, past: impl Fn(usize) -> bool) -> usize {
    let (mut low, mut high) = (0, len);
    while low < high {
        let middle = low + (high - low) / 2;
        if past(middle) {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    low
}

/// The territory `code` names, `JP` or `001`, in any case.
#[must_use]
pub fn territory(code: &str) -> Option<Place> {
    let kind = Kind::Territory;
    kind.index_of(code).map(|index| Place { kind, index })
}

/// The subdivision `code` names in ISO form, `JP-13`, in any case.
#[must_use]
pub fn subdivision(code: &str) -> Option<Place> {
    let kind = Kind::Subdivision;
    kind.index_of(code).map(|index| Place { kind, index })
}

/// The territory or the subdivision `code` names.
#[must_use]
pub fn place(code: &str) -> Option<Place> {
    territory(code).or_else(|| subdivision(code))
}

/// The most tables a chain can hold: a table and its parents. A test holds
/// every chain to it.
const MAX_CHAIN: usize = 4;

/// The tables a locale's lookup reads before English: the first table of
/// its fallback chain, then that table's parents.
fn chain(locale: Option<&Locale>) -> [Option<&'static Table>; MAX_CHAIN] {
    let mut out = [None; MAX_CHAIN];
    let first = locale.and_then(|locale| {
        locale
            .fallback()
            .find_map(|candidate| table(candidate.rendered()?.as_str()))
    });
    let mut next = first;
    for slot in &mut out {
        let Some(found) = next else { break };
        *slot = Some(found);
        next = found.parent.and_then(table);
    }
    out
}

/// A reader of one table's lines, kept in step with an index that rises by
/// one.
#[derive(Debug, Clone)]
struct Cursor {
    names: &'static Names,
    lines: Split<'static, char>,
}

impl Cursor {
    /// A cursor whose next [`Self::step`] reads the code at `start`.
    fn at(names: &'static Names, start: usize) -> Self {
        let mut lines = names.text.split('\n');
        for _ in 0..names.rank(start) {
            lines.next();
        }
        Self { names, lines }
    }

    /// The line of the code at `index`, the one after the last stepped,
    /// if the table names it.
    fn step(&mut self, index: usize) -> Option<&'static str> {
        if self.names.names(index) {
            self.lines.next()
        } else {
            None
        }
    }
}

/// A place with what a locale calls it and what English does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedPlace {
    /// The place.
    pub place: Place,
    /// What the locale calls it, by the lookup the module documentation
    /// gives; `None` when neither the locale nor English names it.
    pub name: Option<PlaceName>,
    /// What `en.xml` calls it: [`Place::english_name`].
    pub english: Option<&'static str>,
}

/// Places with their names in a locale, read in one pass over each table.
#[derive(Debug, Clone)]
pub struct Named {
    places: Places,
    chain: [Option<(&'static str, Cursor)>; MAX_CHAIN],
    english: Option<Cursor>,
}

impl Iterator for Named {
    type Item = NamedPlace;

    fn next(&mut self) -> Option<NamedPlace> {
        let place = self.places.next()?;
        let index = place.index;
        let english = self.english.as_mut().and_then(|cursor| cursor.step(index));
        let mut found = None;
        for (tag, cursor) in self.chain.iter_mut().flatten() {
            let line = cursor.step(index);
            if found.is_none()
                && let Some(line) = line
            {
                found = Some((*tag, cursor.names, line));
            }
        }
        let name = match found {
            Some((tag, names, line)) => {
                let name = if line.is_empty() { english } else { Some(line) };
                name.map(|name| PlaceName {
                    name,
                    tag,
                    draft: names.draft_at(index),
                })
            }
            None => english.and_then(|name| {
                let names = place.kind.names(table(ENGLISH)?);
                Some(PlaceName {
                    name,
                    tag: ENGLISH,
                    draft: names.draft_at(index),
                })
            }),
        };
        Some(NamedPlace {
            place,
            name,
            english,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.places.size_hint()
    }
}

/// `places` with what `locale` calls each, by the lookup the module
/// documentation gives; `None` for `locale` asks for English.
#[must_use]
pub fn named(places: Places, locale: Option<&Locale>) -> Named {
    let start = places.indices.start;
    let kind = places.kind;
    let tables = chain(locale);
    let mut cursors: [Option<(&'static str, Cursor)>; MAX_CHAIN] = Default::default();
    for (slot, table) in cursors.iter_mut().zip(tables) {
        *slot = table.map(|table| (table.tag, Cursor::at(kind.names(table), start)));
    }
    Named {
        places,
        chain: cursors,
        english: table(ENGLISH).map(|english| Cursor::at(kind.names(english), start)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).expect("a tag")
    }

    fn name(code: &str, tag: &str) -> Option<(&'static str, &'static str, &'static str)> {
        place(code)
            .expect("a place")
            .name_in(Some(&locale(tag)))
            .map(|found| (found.name, found.tag, found.draft.name()))
    }

    /// CLDR 48 `subdivisions/ja.xml`: `<subdivision type="jp13"
    /// draft="provisional">東京都</subdivision>`; `subdivisions/en.xml`:
    /// `jp13` `Tokyo`, approved.
    #[test]
    fn tokyo_is_tokyo_to_in_japanese_and_tokyo_in_english() {
        let tokyo = subdivision("JP-13").expect("JP-13");
        assert_eq!(tokyo.code(), "JP-13");
        assert_eq!(tokyo.country(), "JP");
        assert_eq!(tokyo.status(), Status::Regular);
        assert_eq!(tokyo.english_name(), Some("Tokyo"));
        assert_eq!(name("JP-13", "ja"), Some(("東京都", "ja", "provisional")));
        assert_eq!(
            name("jp-13", "ja-JP"),
            Some(("東京都", "ja", "provisional"))
        );
        assert_eq!(name("JP-13", "en"), Some(("Tokyo", "en", "approved")));
    }

    /// `subdivisions/de.xml` `debe` Bayern and `usca` Kalifornien, `fr.xml`
    /// Bavière and Californie, `es.xml` `usca` California, English's own
    /// spelling, so answered by `es`; all provisional. `en.xml` Bavaria.
    #[test]
    fn a_german_and_an_american_subdivision_in_two_locales() {
        assert_eq!(name("DE-BY", "de"), Some(("Bayern", "de", "provisional")));
        assert_eq!(name("DE-BY", "fr"), Some(("Bavière", "fr", "provisional")));
        assert_eq!(name("DE-BY", "en-GB"), Some(("Bavaria", "en", "approved")));
        assert_eq!(
            name("US-CA", "de-AT"),
            Some(("Kalifornien", "de", "provisional"))
        );
        assert_eq!(
            name("US-CA", "fr"),
            Some(("Californie", "fr", "provisional"))
        );
        assert_eq!(
            name("US-CA", "es"),
            Some(("California", "es", "provisional"))
        );
        assert_eq!(
            name("US-CA", "ja"),
            Some(("カリフォルニア州", "ja", "provisional"))
        );
    }

    /// `zh_Hant`'s parent is root (`supplementalData.xml` `parentLocales`),
    /// and `subdivisions/zh_Hant.xml` names only England, Scotland and
    /// Wales, so Tokyo is English's there although `zh.xml` has 東京都;
    /// `pt_PT.xml` names no subdivision of its own, so `pt.xml`'s Tóquio;
    /// Swedish has no table; Coptic's every value is unconfirmed; and
    /// `native`, no locale, asks for English.
    #[test]
    fn a_name_falls_back_along_cldrs_chain_then_to_english() {
        assert_eq!(name("JP-13", "zh-TW"), Some(("Tokyo", "en", "approved")));
        assert_eq!(
            name("GB-SCT", "zh-Hant"),
            Some(("蘇格蘭", "zh-Hant", "approved"))
        );
        assert_eq!(
            name("JP-13", "zh"),
            Some(("東京都", "zh-Hans", "provisional"))
        );
        assert_eq!(
            name("JP-13", "pt-PT"),
            Some(("Tóquio", "pt", "provisional"))
        );
        assert_eq!(name("JP-13", "sv"), Some(("Tokyo", "en", "approved")));
        assert_eq!(name("EG", "cop"), Some(("Egypt", "en", "approved")));
        let tokyo = subdivision("JP-13").expect("JP-13").name_in(None);
        assert_eq!(tokyo.map(|found| found.tag), Some(ENGLISH));
    }

    /// `pt_PT.xml`'s `BH` Barém where `pt.xml` has Barein, as the
    /// `territories` module's test has it; `ja.xml` `001` 世界.
    #[test]
    fn territories_follow_the_same_chain() {
        assert_eq!(name("JP", "ja"), Some(("日本", "ja", "approved")));
        assert_eq!(name("001", "ja"), Some(("世界", "ja", "approved")));
        assert_eq!(name("BH", "pt-PT"), Some(("Barém", "pt-PT", "approved")));
        assert_eq!(name("BH", "pt"), Some(("Barein", "pt", "approved")));
        assert_eq!(name("CQ", "de"), Some(("Sark", "de", "contributed")));
        assert_eq!(
            territory("001").map(Place::status),
            Some(Status::Macroregion)
        );
        assert_eq!(territory("zz").map(Place::status), Some(Status::Unknown));
        assert_eq!(territory("XA").map(Place::status), Some(Status::Special));
    }

    /// `fr75` is deprecated in CLDR 48's validity data, beside the regular
    /// `fr75c` Paris, and only `subdivisions/ha.xml` names it: Pariis.
    #[test]
    fn a_deprecated_code_only_one_locale_names() {
        let paris = subdivision("FR-75").expect("FR-75");
        assert_eq!(paris.status(), Status::Deprecated);
        assert_eq!(paris.english_name(), None);
        assert_eq!(name("FR-75", "ha"), Some(("Pariis", "ha", "provisional")));
        assert_eq!(name("FR-75", "ja"), None);
        assert_eq!(name("FR-75C", "ja"), Some(("パリ", "ja", "provisional")));
    }

    #[test]
    fn codes_match_in_any_case_and_padded() {
        for code in ["JP-13", "jp-13", " Jp-13 "] {
            assert_eq!(subdivision(code).map(Place::code), Some("JP-13"), "{code}");
        }
        assert_eq!(territory(" jp ").map(Place::code), Some("JP"));
        assert!(subdivision("jp13").is_none(), "CLDR's form is not ISO's");
        assert!(subdivision("JP-99").is_none());
        assert!(territory("JPN").is_none());
        assert_eq!(place("jp").map(Place::kind), Some(Kind::Territory));
        assert_eq!(place("jp-01").map(Place::kind), Some(Kind::Subdivision));
    }

    #[test]
    fn every_code_is_found_by_its_own_spelling() {
        for place in territories().chain(subdivisions()) {
            let code = place.code();
            assert_eq!(super::place(code), Some(place), "{code}");
            assert_eq!(
                super::place(&code.to_ascii_lowercase()),
                Some(place),
                "{code}"
            );
        }
    }

    #[test]
    fn the_lists_are_in_code_order_and_iso_shaped() {
        assert_eq!(Kind::Territory.len(), 295);
        assert_eq!(Kind::Subdivision.len(), 5503);
        assert!(!Kind::Territory.is_empty());
        for kind in [Kind::Territory, Kind::Subdivision] {
            assert_eq!(kind.codes().len() % kind.width(), 0);
            for index in 1..kind.len() {
                assert!(kind.code_at(index - 1) < kind.code_at(index));
            }
        }
        for place in subdivisions() {
            let code = place.code().as_bytes();
            assert!(
                (4..=6).contains(&code.len())
                    && code[..2].iter().all(u8::is_ascii_uppercase)
                    && code[2] == b'-'
                    && code[3..]
                        .iter()
                        .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit()),
                "{}",
                place.code()
            );
            assert!(territory(place.country()).is_some(), "{}", place.code());
        }
        for marks in [TERRITORY_STATUS, SUBDIVISION_STATUS] {
            assert!(marks.iter().all(|byte| b"rdmsu".contains(byte)));
        }
        assert_eq!(TERRITORY_STATUS.len(), Kind::Territory.len());
        assert_eq!(SUBDIVISION_STATUS.len(), Kind::Subdivision.len());
    }

    #[test]
    fn every_regular_subdivision_has_an_english_name() {
        let mut deprecated = 0;
        let mut unnamed = 0;
        for NamedPlace { place, english, .. } in named(subdivisions(), None) {
            if place.index % 97 == 0 {
                assert_eq!(place.english_name(), english);
            }
            match place.status() {
                Status::Regular => assert!(english.is_some(), "{}", place.code()),
                _ => {
                    deprecated += 1;
                    unnamed += usize::from(english.is_none());
                }
            }
        }
        assert_eq!((deprecated, unnamed), (476, 104));
        assert!(named(territories(), None).all(|named| named.english.is_some()));
    }

    #[test]
    fn a_country_has_the_subdivisions_its_code_begins() {
        let japan = subdivisions_of("jp").expect("JP");
        assert_eq!(japan.len(), 47);
        assert_eq!(japan.clone().next().map(Place::code), Some("JP-01"));
        assert_eq!(japan.last().map(Place::code), Some("JP-47"));
        assert_eq!(subdivisions_of("AQ").map(|places| places.len()), Some(0));
        assert_eq!(subdivisions_of("001").map(|places| places.len()), Some(0));
        assert!(subdivisions_of("JPN").is_none());
        let total: usize = territories()
            .filter_map(|place| subdivisions_of(place.code()))
            .map(|places| places.len())
            .sum();
        assert_eq!(total, Kind::Subdivision.len());
    }

    /// Reading a whole list in one pass gives what one lookup at a time
    /// gives, on a sample of the places in every table's locale.
    #[test]
    fn a_pass_and_a_lookup_agree() {
        let step = if cfg!(debug_assertions) { 101 } else { 13 };
        for tag in tags().chain(["und", "zh-TW", "sv"]) {
            let requested = locale(tag);
            for places in [territories(), subdivisions()] {
                for named in named(places, Some(&requested)).step_by(step) {
                    let place = named.place;
                    let code = place.code();
                    assert_eq!(place.name_in(Some(&requested)), named.name, "{tag} {code}");
                    assert_eq!(place.english_name(), named.english, "{code}");
                }
            }
        }
    }

    fn all(kind: Kind) -> Places {
        Places {
            kind,
            indices: 0..kind.len(),
        }
    }

    #[test]
    fn every_table_is_consistent() {
        for pair in TABLES.windows(2) {
            assert!(pair[0].tag < pair[1].tag);
        }
        for table in TABLES {
            assert!(
                crate::data::LOCALES
                    .iter()
                    .any(|entry| entry.tag == table.tag)
            );
            let chain = chain(Some(&locale(table.tag)));
            assert_eq!(chain[0].map(|found| found.tag), Some(table.tag));
            assert!(chain[MAX_CHAIN - 1].is_none(), "{}", table.tag);
            for kind in [Kind::Territory, Kind::Subdivision] {
                let names = kind.names(table);
                let lines = names.text.split('\n').count() - 1;
                assert_eq!(lines, names.rank(kind.len()), "{} {kind:?}", table.tag);
                assert!(names.text.is_empty() || names.text.ends_with('\n'));
                assert!(names.named.last() != Some(&0), "{}", table.tag);
                assert!(names.named.len() * 8 < kind.len() + 8);
                for pair in names.exceptions.windows(2) {
                    assert!(pair[0].0 < pair[1].0);
                }
                for (index, draft) in names.exceptions {
                    assert!(names.names(usize::from(*index)));
                    assert_ne!(*draft, names.draft);
                }
                let mut own = Cursor::at(names, 0);
                let requested = locale(table.tag);
                for NamedPlace {
                    place,
                    name,
                    english,
                } in named(all(kind), Some(&requested))
                {
                    let Some(line) = own.step(place.index) else {
                        continue;
                    };
                    let found = name.expect("a name");
                    assert_eq!(found.tag, table.tag, "{}", place.code());
                    assert_eq!(found.name, found.name.trim());
                    assert!(!found.name.is_empty() && !found.name.contains(['\t', '↑']));
                    if table.tag != ENGLISH {
                        let same = english == Some(found.name);
                        assert_eq!(line.is_empty(), same, "{} {}", table.tag, place.code());
                    }
                }
            }
        }
    }

    /// The parents the generator read from `parentLocales` agree with the
    /// chain [`Locale::fallback`] walks: a table's parent is the next table
    /// of its tag's own chain.
    #[test]
    fn a_tables_parent_is_the_next_table_of_its_chain() {
        for table in TABLES {
            let Some(parent) = table.parent else { continue };
            let next = locale(table.tag)
                .fallback()
                .skip(1)
                .find_map(|candidate| super::table(candidate.rendered()?.as_str()));
            assert_eq!(next.map(|found| found.tag), Some(parent));
        }
        assert_eq!(table("pt-PT").and_then(|found| found.parent), Some("pt"));
    }

    /// The names the `territories` module carries for the holiday tables,
    /// CLDR's approved and contributed values of the 195 regions, are this
    /// module's names from the same table.
    #[cfg(feature = "territories")]
    #[test]
    fn the_holiday_territory_names_are_these() {
        for older in crate::territories::TABLES {
            for code in crate::territories::REGIONS {
                let Some(expected) = older.name_of(code) else {
                    continue;
                };
                let found = territory(code)
                    .expect("a territory")
                    .name_in(Some(&locale(older.tag)))
                    .expect("a name");
                assert_eq!((found.name, found.tag), (expected, older.tag), "{code}");
            }
        }
    }
}
