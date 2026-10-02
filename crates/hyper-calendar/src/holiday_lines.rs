//! The tab-separated lines the WebAssembly module and the C library write
//! about the holiday tables and the liturgical year, written once.
//!
//! * The holidays of a year in one table, and of a day across every table,
//!   from [`hc_holiday::engine`], and whether a day is a day off; the table
//!   an identifier names is [`rule_set`]'s.
//! * The tables themselves, in the order `hc_holiday_codes` lists them,
//!   with the kind each is, its names, its sources and the country an
//!   exchange keeps the holidays of, each read from the table's own data
//!   but for a country's name in a locale, which is CLDR's, from
//!   [`hc_i18n::place_names`].
//! * The lectionary cycles of a day, and the astronomical Easter of a year
//!   and the paschal full moon it is the Sunday after,
//!   from [`hc_holiday::lectionary`] and [`hc_holiday::computus`].
//! * The Holy Year a day falls in, from [`hc_holiday::holy_years`], and the
//!   rank of each *Common Worship* celebration kept on a day, from
//!   [`hc_holiday::common_worship`]. The celebrations themselves, and the
//!   years the Rules leave a Festival without a day, are already lines of
//!   `hc_holidays_on`, in the `common-worship` table; the rank is what
//!   these lines add.
//! * The Eastern Orthodox fast a day falls in, and the fasting seasons and
//!   fast-free weeks of a year, under each reckoning of
//!   [`hc_holiday::orthodox_fasts`]: two reckonings, two identifiers
//!   (`docs/policy.md` §5), which the caller names.

use alloc::string::{String, ToString};

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::common_worship::{CELEBRATIONS, COMMON_WORSHIP, Rank};
use hc_holiday::engine::UNREAD_SUBDIVISION_ID;
use hc_holiday::group::Group;
use hc_holiday::holy_years::{self, HolyYearOn, Jubilee, JubileeKind, TableDate};
use hc_holiday::orthodox_fasts::{self, Abstinence, PeriodKind, Reckoning, Status};
use hc_holiday::roman_calendar_1960;
use hc_holiday::rule::{Kind, RuleSet, Scope, region_parent};
use hc_holiday::{
    Gap, Holiday, HolidayCalendar, computus, countries, exchanges, international, lectionary,
    traditions,
};
use hc_i18n::Locale;
use hc_i18n::place_names::{self, Alt, Draft};

use crate::boundary::{Answer, Line, Refusal};

/// How many columns [`holiday_tables`] writes.
pub const HOLIDAY_TABLES_COLUMNS: usize = 13;

/// How many columns [`lectionary_line`] writes.
pub const LECTIONARY_COLUMNS: usize = 7;

/// Every table, in the order `hc_holiday_codes` lists them: the countries,
/// then the exchanges, the traditions and the international sets.
pub fn tables() -> impl Iterator<Item = &'static RuleSet> {
    countries::ALL
        .iter()
        .chain(exchanges::ALL)
        .chain(traditions::ALL)
        .chain(international::ALL)
        .copied()
}

/// What a table is, as the line writes it: read from the list the table
/// is in, and for a country's list from the shape of its code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableKind {
    /// A country's public holidays, keyed by ISO 3166-1 alpha-2.
    Country,
    /// A subdivision's, keyed by ISO 3166-2, `JP-13`: a table in the
    /// countries' list whose code carries a hyphen. None does yet; every
    /// subdivision's days are rules of its country's table, by region.
    Subdivision,
    /// An exchange's trading calendar, keyed by ISO 10383 MIC.
    Exchange,
    /// A religious or cultural tradition's calendar.
    Tradition,
    /// An international set of observances, `un-days`.
    Observance,
}

impl TableKind {
    /// The word a line carries.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Country => "country",
            Self::Subdivision => "subdivision",
            Self::Exchange => "exchange",
            Self::Tradition => "tradition",
            Self::Observance => "observance",
        }
    }
}

/// Every table with its kind, in [`tables`] order.
pub fn tables_with_kinds() -> impl Iterator<Item = (&'static RuleSet, TableKind)> {
    let national = countries::ALL.iter().map(|set| {
        let kind = if set.code.contains('-') {
            TableKind::Subdivision
        } else {
            TableKind::Country
        };
        (*set, kind)
    });
    national
        .chain(exchanges::ALL.iter().map(|set| (*set, TableKind::Exchange)))
        .chain(
            traditions::ALL
                .iter()
                .map(|set| (*set, TableKind::Tradition)),
        )
        .chain(
            international::ALL
                .iter()
                .map(|set| (*set, TableKind::Observance)),
        )
}

/// The ISO 3166-1 country a subdivision or an exchange belongs to, as its
/// table records it: a subdivision's code before the hyphen, and for an
/// exchange the country whose table it includes for its days off. An
/// exchange that lists every closed day itself names no country in its
/// data, and has none here.
#[must_use]
pub fn country_of(set: &RuleSet, kind: TableKind) -> Option<&'static str> {
    match kind {
        TableKind::Subdivision => set.code.split('-').next(),
        TableKind::Exchange => set
            .includes
            .iter()
            .map(|include| include.set.code)
            .find(|code| countries::by_code(code).is_some()),
        _ => None,
    }
}

/// The locale a tag asks for, read as the calendar lines read one: `None`
/// for `native`, which names no one locale (and a table has no language
/// of its own to ask for), and the root locale for a tag that does not
/// parse.
fn requested_locale(tag: &str) -> Option<Locale> {
    if tag == "native" {
        None
    } else {
        Some(Locale::parse(tag).unwrap_or(Locale::ROOT))
    }
}

/// A table's name in a locale, and the tag of the data that answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableName {
    /// What the locale calls the table.
    pub name: &'static str,
    /// The tag of the data that answered: `de` for a request for `de-AT`,
    /// `en` for a table CLDR does not name.
    pub tag: &'static str,
}

/// The draft levels a country's name is taken at: CLDR's release levels,
/// `approved` and `contributed`, as for the calendar names.
const RELEASED: Draft = Draft::Contributed;

/// What a table is called in a locale, and the tag of the data that
/// answered: a country's CLDR name by [`hc_i18n::place_names`]' lookup —
/// the locale's tables, their language-matching fallbacks, then CLDR's
/// `en`, answered by `en`, so that `en` stands for one name of a country
/// whichever locale was asked. Every other kind is named by the table's
/// English name, answered by `en` — CLDR names no exchange, tradition or
/// set of observances, and nothing here invents a name for one.
#[must_use]
pub fn table_name(set: &RuleSet, kind: TableKind, locale: Option<&Locale>) -> TableName {
    cldr_name(set, kind, locale).unwrap_or(TableName {
        name: set.english_name,
        tag: "en",
    })
}

/// A country's CLDR name in a locale, at the release levels: what
/// [`table_name`] answers for every country CLDR names, which is all of
/// them. Under `native`, which names no locale, the `en` one.
fn cldr_name(set: &RuleSet, kind: TableKind, locale: Option<&Locale>) -> Option<TableName> {
    if kind != TableKind::Country {
        return None;
    }
    let found = place_names::territory(set.code)?.name_at(locale, RELEASED)?;
    Some(TableName {
        name: found.name,
        tag: found.tag,
    })
}

/// A country's CLDR `alt="short"` name, from the same locale's data as the
/// name [`table_name`] gives it — `Hong Kong` for `HK` under `en`, `香港`
/// under `ja` — where that name is CLDR's and the data has a short one;
/// see [`place_names::Place::alternative_in`]. A country named from CLDR's
/// `en` by fallback has `en`'s short name; nothing for any other kind of
/// table, whose English name is its own and not CLDR's.
#[must_use]
pub fn short_table_name(
    set: &RuleSet,
    kind: TableKind,
    locale: Option<&Locale>,
) -> Option<&'static str> {
    if kind != TableKind::Country {
        return None;
    }
    let place = place_names::territory(set.code)?;
    place
        .alternative_in(locale, Alt::Short, RELEASED)
        .map(|found| found.name)
}

/// The lines of `hc_holiday_tables`, one per table in [`tables`] order:
/// the code, the kind, the name in the locale, the English name, the
/// locale that answered, the sources, the country of a subdivision or an
/// exchange, the short name in the locale, the subdivisions the table's
/// rules are scoped to, and the groups its rules are given to alone, by
/// identifier and by name in the locale.
///
/// Column 3 follows [`table_name`]: a country is named as the locale's
/// CLDR 48 data names it, where `hc-i18n` carries a name for it — 日本 under
/// `ja`, Deutschland under `de` — and column 5 is the tag of the data that
/// answered, `ja` or `de`, so that a request for `de-AT` says `de`. A
/// country the locale has no name for, and every country under `native`
/// or a tag whose chain reaches no territory names, is named as CLDR's
/// `en` names it — `Hong Kong SAR China`, not the table's own `Hong Kong`
/// — with `en` in column 5, so that `en` means one name of a country
/// wherever it appears. Every exchange, tradition and set of observances
/// is named by the table's own English name, column 4, with `en` in
/// column 5. So column 3 is never empty. Column 7
/// stays a code, the key of the country's own row, whose column 3 names it
/// in the same locale. Column 8 follows [`short_table_name`]: CLDR 48's
/// `alt="short"` name of a country column 3 names from CLDR, from the same
/// locale's data — `Hong Kong` for `HK` under `en`, `香港` under `ja`, `UK`
/// for `GB` — and empty where that data has none, which is most countries,
/// and for every table that is not a country. Column 9 is
/// [`RuleSet::regions`]: the ISO 3166-2 codes a caller may pass as the
/// region of the table, separated by `;` in code order, `JP-11;JP-12;…`,
/// and empty for a table with no subdivision's days. Column 10 is
/// [`RuleSet::groups`]: the identifiers of the groups a caller may pass as
/// the group of the table, `;`-separated in identifier order,
/// `children;military;women;youth` for `CN`, and empty for a table that
/// gives no day to a group alone. Column 11 names them, in the same order:
/// each group's name in the locale where `hc-i18n` carries one
/// ([`group_name`]), 妇女 for `women` under `zh-CN`, and else its English
/// name. Column 12 is [`RuleSet::region_groups`]: each subdivision and
/// group a rule is scoped to both of, `region:group`, `;`-separated in
/// code and then identifier order, `CN-XJ:women`, the scopes whose own
/// days neither the region alone nor the group alone has; empty for a
/// table with none. Column 13 is [`RuleSet::read_subdivisions`]: the
/// ISO 3166-2 codes of the subdivisions the table's sources were read
/// for, `;`-separated in code order — column 9's and those read and
/// found to keep no day of their own — and empty for a table with no
/// subdivisions and for one whose subdivisions were not read; a region
/// asked of the table that is not in it keeps the nationwide days and has
/// a gap for its own.
#[must_use]
pub fn holiday_tables(locale: &str) -> String {
    let requested = requested_locale(locale);
    let mut out = String::new();
    for (set, kind) in tables_with_kinds() {
        let named = table_name(set, kind, requested.as_ref());
        let mut line = Line::new(&mut out);
        line.cell(set.code)
            .cell(kind.name())
            .cell(named.name)
            .cell(set.english_name)
            .cell(named.tag)
            .cell(set.sources)
            .cell_or_empty(country_of(set, kind))
            .cell_or_empty(short_table_name(set, kind, requested.as_ref()))
            .cell(&set.regions().join(";"));
        let groups = set.groups();
        let ids: alloc::vec::Vec<&str> = groups.iter().map(|group| group.id).collect();
        let names: alloc::vec::Vec<&str> = groups
            .iter()
            .map(|group| group_name(group, requested.as_ref()))
            .collect();
        line.cell(&ids.join(";")).cell(&names.join(";"));
        let scoped: alloc::vec::Vec<String> = set
            .region_groups()
            .iter()
            .map(|(region, group)| alloc::format!("{region}:{}", group.id))
            .collect();
        line.cell(&scoped.join(";"))
            .cell(&set.read_subdivisions().join(";"));
        line.end();
    }
    out
}

/// The table an identifier names: a country's ISO 3166-1 alpha-2 code, an
/// exchange's ISO 10383 Market Identifier Code, a tradition's slug or
/// `un-days`, tried in that order, each matched as `docs/policy.md` §5
/// says.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a code that names no table.
pub fn rule_set(code: &str) -> Answer<&'static RuleSet> {
    countries::by_code(code)
        .or_else(|| exchanges::by_code(code))
        .or_else(|| traditions::by_code(code))
        .or_else(|| international::by_code(code))
        .ok_or(Refusal::Unknown)
}

/// The lines of `hc_holiday_codes`: every table's identifier, one per
/// line, in [`tables`]' order.
#[must_use]
pub fn holiday_codes() -> String {
    let mut out = String::new();
    for set in tables() {
        let mut line = Line::new(&mut out);
        line.cell(set.code);
        line.end();
    }
    out
}

/// A day as ISO 8601 text, or `?` for one with no Gregorian date, which
/// no holiday of a year's table falls on.
fn iso(day: Rd) -> String {
    crate::civil::Date::from_ordinal(day.0)
        .map_or_else(|_| String::from("?"), |date| date.to_string())
}

/// What a group is called in a locale: its name in `hc-i18n`'s table for
/// the first locale of the chain that has one, and else its English name,
/// which is also its name under `native`, a request for no one locale.
#[must_use]
pub fn group_name(group: &Group, locale: Option<&Locale>) -> &'static str {
    locale
        .and_then(|locale| hc_i18n::holiday_groups::group_name(locale, group.id))
        .map_or(group.english_name, |named| named.name)
}

/// The group identifier of `table` that `group` names, as the table
/// writes it: `women` for ` Women `. `None` when the table gives no rule
/// to it, which leaves the caller with everyone's days.
fn table_group(table: &RuleSet, group: Option<&str>) -> Option<&'static str> {
    let group = group?;
    table
        .groups()
        .into_iter()
        .map(|named| named.id)
        .find(|id| hc_core::catalogue::matches(group, id))
}

/// The scope a caller asks for: `region` as given, and `group`, which must
/// name a group of [`hc_holiday::group::GROUPS`] when it is not blank. A
/// known group the table gives no day to alone has everyone's days; a
/// name that is no group is refused rather than answered as if it were
/// everyone.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a `group` that names no group.
fn requested_scope<'a>(region: Option<&'a str>, group: Option<&'a str>) -> Answer<Scope<'a>> {
    let group = group.filter(|group| !group.trim().is_empty());
    if let Some(group) = group {
        hc_holiday::group::by_id(group).ok_or(Refusal::Unknown)?;
    }
    Ok(Scope::new(region, group))
}

/// The subdivision code of `table` that `region` names, as the table
/// writes it: `JP-13` for `jp-13`. `None` when the table scopes no rule to
/// it, which leaves the caller with the nationwide days.
fn table_region(table: &RuleSet, region: Option<&str>) -> Option<&'static str> {
    let region = region?;
    table
        .regions()
        .into_iter()
        .find(|code| hc_core::catalogue::matches(region, code))
}

/// The holidays of a Gregorian year in a table, nationwide or in a
/// subdivision, for everyone or for a group, one line each: the ISO 8601
/// date, the name, the local name, the kind, the confidence, `1` for a
/// substitute day and `0` otherwise, the ISO 8601 date the substitute
/// stands in for or nothing, the subdivision whose own entry it is — the
/// region asked for, as the table writes its code, on an entry the
/// calendar for no region does not have — or nothing, and the group whose
/// own entry it is — the group asked for, as the table writes its
/// identifier, on an entry the calendar for everyone does not have — or
/// nothing, tab-separated.
///
/// Then the gaps: a holiday the table could not place in the year — its
/// calendar's range ended, no announcement was read, the year is before
/// the first its sources were read for, or the subdivision was not read —
/// one line each, as `hc_holidays_on` writes them: an empty date, the
/// name, the local name, the kind `gap`, an empty confidence, `0`, nothing,
/// and the subdivision and the group whose own gap it is, as for an entry.
///
/// Two cells follow, after the group: the holiday's identifier within its
/// table ([`hc_holiday::id`]), which a line of `hc_holidays_on` and of
/// `hc_common_worship_on` carries too, and the instrument its rule cites,
/// or nothing where the table's `sources` speaks for it; a gap's are its
/// rule's, and `unread-subdivision` and nothing for a subdivision not
/// read.
///
/// `kinds` keeps the entries of those kinds and the gaps of the rules of
/// those kinds, and all of them when it is empty; a subdivision not read
/// is a gap whatever the kinds, because none of its days is known.
#[must_use]
pub fn year_lines(table: &RuleSet, scope: Scope<'_>, kinds: &[Kind], year: i64) -> String {
    let calendar = HolidayCalendar::for_year_scoped(table, scope, year);
    // A subdivision the table was not read for has no code of the table's
    // own to write, and its gap is written with the code as asked.
    let own_region: Option<&str> = table_region(table, scope.region).or_else(|| {
        scope
            .region
            .map(str::trim)
            .filter(|region| !table.reads_region(region))
    });
    let own_group = table_group(table, scope.group);
    // The calendar without the region, and the calendar without the group:
    // an entry the one lacks is the region's own, the other's the group's.
    let without_region =
        own_region.map(|_| HolidayCalendar::for_year_scoped(table, scope.nationwide(), year));
    let without_group =
        own_group.map(|_| HolidayCalendar::for_year_scoped(table, scope.for_everyone(), year));
    // A municipality's entry that its subdivision has too is the
    // subdivision's own: the region column names the widest region above
    // the one asked for whose calendar has it.
    let above: alloc::vec::Vec<(&str, HolidayCalendar<'_>)> = own_region
        .into_iter()
        .flat_map(ancestors)
        .map(|parent| {
            let code = table_region(table, Some(parent)).unwrap_or(parent);
            let calendar = HolidayCalendar::for_year_scoped(
                table,
                Scope::new(Some(parent), scope.group),
                year,
            );
            (code, calendar)
        })
        .collect();
    let own = |parent: Option<&HolidayCalendar<'_>>, holiday: &Holiday, code| {
        parent.and_then(|parent| (!parent.all().contains(holiday)).then_some(code))
    };
    let wanted = |kind: Kind| kinds.is_empty() || kinds.contains(&kind);
    let mut out = String::new();
    for holiday in calendar.all().iter().filter(|holiday| wanted(holiday.kind)) {
        let regional = widest(
            own(without_region.as_ref(), holiday, own_region).flatten(),
            &above,
            |calendar| calendar.all().contains(holiday),
        );
        let grouped = own(without_group.as_ref(), holiday, own_group).flatten();
        let mut line = Line::new(&mut out);
        line.cell(&iso(holiday.date))
            .cell(holiday.name)
            .cell(holiday.local_name)
            .cell(holiday.kind.id())
            .cell(holiday.confidence.id())
            .flag(holiday.is_substitute())
            .cell_or_empty(holiday.observed_for.map(iso).as_deref())
            .cell_or_empty(regional)
            .cell_or_empty(grouped)
            .value(holiday.id)
            .cell(holiday.source);
        line.end();
    }
    let own_gap = |parent: Option<&HolidayCalendar<'_>>, gap: &Gap, code| {
        parent.and_then(|parent| (!parent.gaps().contains(gap)).then_some(code))
    };
    for gap in calendar
        .gaps()
        .iter()
        .filter(|gap| gap.id == UNREAD_SUBDIVISION_ID || wanted(gap.kind))
    {
        let regional = widest(
            own_gap(without_region.as_ref(), gap, own_region).flatten(),
            &above,
            |calendar| calendar.gaps().contains(gap),
        );
        let grouped = own_gap(without_group.as_ref(), gap, own_group).flatten();
        let mut line = Line::new(&mut out);
        line.empty()
            .cell(gap.name)
            .cell(gap.local_name)
            .cell("gap")
            .empty()
            .flag(false)
            .empty()
            .cell_or_empty(regional)
            .cell_or_empty(grouped)
            .value(gap.id)
            .cell(gap.source);
        line.end();
    }
    out
}

/// The region column of a line of [`year_lines`] asked for a
/// municipality: `code`, the region asked for, unless a region above it,
/// of `above`, nearest first, `has` the entry too, and then the widest
/// that does.
fn widest<'c>(
    code: Option<&'c str>,
    above: &[(&'c str, HolidayCalendar<'_>)],
    has: impl Fn(&HolidayCalendar<'_>) -> bool,
) -> Option<&'c str> {
    code.map(|code| {
        above
            .iter()
            .rev()
            .find(|(_, calendar)| has(calendar))
            .map_or(code, |(parent, _)| *parent)
    })
}

/// The kinds a caller asks for: `kind` as a `;`-separated list of
/// [`Kind::id`] words, matched as every identifier is, or nothing for
/// every kind.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a word that names no kind.
fn requested_kinds(kind: Option<&str>) -> Answer<alloc::vec::Vec<Kind>> {
    let mut kinds = alloc::vec::Vec::new();
    for word in kind.unwrap_or("").split(';') {
        let word = word.trim();
        if !word.is_empty() {
            kinds.push(Kind::by_id(word).ok_or(Refusal::Unknown)?);
        }
    }
    Ok(kinds)
}

/// The lines of `hc_holidays_in_year`: [`year_lines`] for the table
/// `code` names, in the subdivision `region` names, or nationwide for
/// none, for the group `group` names, or for everyone for none, and of
/// the kinds `kind` lists, `public;bank`, or of every kind for none.
///
/// # Errors
///
/// As [`rule_set`], and [`Refusal::Unknown`] for a `group` that names no
/// group of [`hc_holiday::group::GROUPS`] and a `kind` with a word that
/// names no [`Kind`].
pub fn holidays_in_year(
    code: &str,
    region: Option<&str>,
    group: Option<&str>,
    kind: Option<&str>,
    year: i64,
) -> Answer<String> {
    let table = rule_set(code)?;
    let scope = requested_scope(region, group)?;
    Ok(year_lines(table, scope, &requested_kinds(kind)?, year))
}

/// One table's nationwide lines of `hc_holidays_on` for one day: the
/// entries `calendar` has on the day, then the gaps it reports.
/// Tab-separated: the table's identifier, its English name, the holiday's
/// English name, its local name, the kind, the confidence, the instrument
/// the rule cites, `1` for a substitute day and `0` otherwise, the fixed
/// day a substitute stands in for or nothing, the subdivision and the
/// group, which are nothing here, and the holiday's identifier within its
/// table ([`hc_holiday::id`]); a gap has the kind `gap`, an empty
/// confidence, the instrument its rule cites, which for a year before its
/// sources were read says what was read, `0` and nothing.
pub fn push_day_lines(out: &mut String, table: &RuleSet, calendar: &HolidayCalendar<'_>, day: Rd) {
    push_entry_lines(
        out,
        table,
        Scope::EVERYONE,
        &calendar.on(day),
        calendar.gaps(),
    );
}

/// One subdivision's lines of `hc_holidays_on` for one day: the entries
/// and gaps `regional`, the table evaluated in `region`, has on the day
/// that `nationwide` does not, each with `region` in the subdivision's
/// column. For a municipality, less what its subdivisions have too
/// (`hc_holiday::rule::region_parent`), which are their own lines.
pub fn push_region_day_lines(
    out: &mut String,
    table: &RuleSet,
    region: &str,
    regional: &HolidayCalendar<'_>,
    nationwide: &HolidayCalendar<'_>,
    day: Rd,
) {
    let above: alloc::vec::Vec<HolidayCalendar<'_>> = ancestors(region)
        .map(|parent| HolidayCalendar::for_day(table, Some(parent), day))
        .collect();
    let mut parents: alloc::vec::Vec<&HolidayCalendar<'_>> = alloc::vec![nationwide];
    parents.extend(above.iter());
    push_scoped_day_lines(out, table, Scope::region(region), regional, &parents, day);
}

/// The regions a municipality's code lies within, nearest first: `JP-14`
/// for `JP-14-130`, and nothing for an ISO 3166-2 code.
fn ancestors(region: &str) -> impl Iterator<Item = &str> {
    core::iter::successors(region_parent(region), |code| region_parent(code))
}

/// One scope's lines of `hc_holidays_on` for one day: the entries and gaps
/// `scoped`, the table evaluated in `scope`, has on the day that none of
/// `parents` — the table evaluated without the scope's region, or without
/// its group — has, each with the scope's region and group in the last two
/// columns.
pub fn push_scoped_day_lines(
    out: &mut String,
    table: &RuleSet,
    scope: Scope<'_>,
    scoped: &HolidayCalendar<'_>,
    parents: &[&HolidayCalendar<'_>],
    day: Rd,
) {
    let elsewhere: alloc::vec::Vec<alloc::vec::Vec<Holiday>> =
        parents.iter().map(|parent| parent.on(day)).collect();
    let own: alloc::vec::Vec<Holiday> = scoped
        .on(day)
        .into_iter()
        .filter(|holiday| !elsewhere.iter().any(|entries| entries.contains(holiday)))
        .collect();
    let gaps: alloc::vec::Vec<Gap> = scoped
        .gaps()
        .iter()
        .filter(|gap| !parents.iter().any(|parent| parent.gaps().contains(gap)))
        .copied()
        .collect();
    push_entry_lines(out, table, scope, &own, &gaps);
}

/// The lines of [`push_day_lines`] and [`push_scoped_day_lines`].
fn push_entry_lines(
    out: &mut String,
    table: &RuleSet,
    scope: Scope<'_>,
    holidays: &[Holiday],
    gaps: &[Gap],
) {
    for holiday in holidays {
        let mut line = Line::new(out);
        line.cell(table.code)
            .cell(table.english_name)
            .cell(holiday.name)
            .cell(holiday.local_name)
            .cell(holiday.kind.id())
            .cell(holiday.confidence.id())
            .cell(holiday.source)
            .flag(holiday.is_substitute())
            .value_or_empty(holiday.observed_for.map(|day| day.0))
            .cell_or_empty(scope.region)
            .cell_or_empty(scope.group)
            .value(holiday.id);
        line.end();
    }
    // A gap is a holiday the table could not place this year — its
    // calendar's range ended, or no announcement was read — and it is
    // reported rather than left out, so that a caller can say so.
    for gap in gaps {
        let mut line = Line::new(out);
        line.cell(table.code)
            .cell(table.english_name)
            .cell(gap.name)
            .cell(gap.local_name)
            .cell("gap")
            .empty()
            .cell(gap.source)
            .flag(false)
            .empty()
            .cell_or_empty(scope.region)
            .cell_or_empty(scope.group)
            .value(gap.id);
        line.end();
    }
}

/// The lines of `hc_holidays_on`: for every table in [`tables`]' order,
/// [`push_day_lines`] for the table evaluated nationwide, then
/// [`push_region_day_lines`] for each subdivision of
/// [`RuleSet::regions`], in code order, so that a subdivision's own day —
/// Tokyo's 都民の日, a Canadian province's Civic Holiday — is a line with
/// its region, and a day the nationwide calendar already has is not
/// repeated; then [`push_scoped_day_lines`] for each group of
/// [`RuleSet::groups`], in identifier order, nationwide, so that China's
/// half day for women is a line with the group `women`; and last for each
/// pair of [`RuleSet::region_groups`], a day of one group in one
/// subdivision, less what the region alone and the group alone have.
///
/// One memo serves every table: the astronomy the tables share — the same
/// tithis, the same new moons — is done once, the answers the context
/// keeps in it and the solstices and new moons of the lunisolar
/// conversions in the scope.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day with no Gregorian year.
pub fn holidays_on(fixed: i64) -> Answer<String> {
    let day = Rd(fixed);
    gregorian::year_from_fixed(day).map_err(|_| Refusal::OutOfRange)?;
    let mut out = String::new();
    let mut context = hc_holiday::EvaluationContext::new();
    hc_core::memo::scope(|| {
        for table in tables() {
            let calendar = HolidayCalendar::for_day_with(table, None, day, &mut context);
            push_day_lines(&mut out, table, &calendar, day);
            for region in table.regions() {
                let regional =
                    HolidayCalendar::for_day_with(table, Some(region), day, &mut context);
                push_region_day_lines(&mut out, table, region, &regional, &calendar, day);
            }
            for group in table.groups() {
                let scope = Scope::group(group.id);
                let grouped = HolidayCalendar::for_day_scoped_with(table, scope, day, &mut context);
                push_scoped_day_lines(&mut out, table, scope, &grouped, &[&calendar], day);
            }
            for (region, group) in table.region_groups() {
                let scope = Scope::new(Some(region), Some(group.id));
                let both = HolidayCalendar::for_day_scoped_with(table, scope, day, &mut context);
                let regional = HolidayCalendar::for_day_scoped_with(
                    table,
                    scope.for_everyone(),
                    day,
                    &mut context,
                );
                let grouped = HolidayCalendar::for_day_scoped_with(
                    table,
                    scope.nationwide(),
                    day,
                    &mut context,
                );
                push_scoped_day_lines(&mut out, table, scope, &both, &[&regional, &grouped], day);
            }
        }
    });
    Ok(out)
}

/// Whether a fixed day is a day off in the table `code` names, nationwide
/// or in the subdivision `region` names, for everyone or for the group
/// `group` names.
///
/// # Errors
///
/// As [`rule_set`]; [`Refusal::Unknown`] for a `group` that names no group
/// of [`hc_holiday::group::GROUPS`]; and [`Refusal::OutOfRange`] for a day
/// with no Gregorian year.
pub fn is_day_off(
    code: &str,
    region: Option<&str>,
    group: Option<&str>,
    fixed: i64,
) -> Answer<bool> {
    let table = rule_set(code)?;
    let day = Rd(fixed);
    gregorian::year_from_fixed(day).map_err(|_| Refusal::OutOfRange)?;
    let scope = requested_scope(region, group)?;
    Ok(HolidayCalendar::for_day_scoped(table, scope, day).is_holiday(day))
}

/// The most business days [`add_business_days`] walks: a hundred years'
/// worth, so that the evaluation it needs stays bounded.
pub const MAX_BUSINESS_DAYS: i64 = 36_500;

/// A day's Gregorian year, as the tables evaluate it.
fn year_of(fixed: i64) -> Answer<i64> {
    gregorian::year_from_fixed(Rd(fixed)).map_err(|_| Refusal::OutOfRange)
}

/// A fixed day moved by `count` business days of a table in a scope, as
/// [`HolidayCalendar::add_business_days`] counts them: a business day is
/// neither a holiday of the scope nor a weekend day the table does not make
/// a working day; the starting day is never counted, and a count of zero
/// returns the day.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a code that names no table or a `group` that
/// names no group; [`Refusal::OutOfRange`] for a day with no Gregorian year, a count past
/// [`MAX_BUSINESS_DAYS`] either way, or a walk that leaves the years the
/// tables evaluate.
pub fn add_business_days(
    code: &str,
    region: Option<&str>,
    group: Option<&str>,
    fixed: i64,
    count: i64,
) -> Answer<i64> {
    let table = rule_set(code)?;
    let scope = requested_scope(region, group)?;
    let year = year_of(fixed)?;
    if count.unsigned_abs() > MAX_BUSINESS_DAYS.unsigned_abs() {
        return Err(Refusal::OutOfRange);
    }
    // A year holds more than a hundred business days in every table, so
    // the walk stays within `count / 100 + 1` years of the start.
    let reach = count.abs() / 100 + 1;
    let (first, last) = if count < 0 {
        (year - reach, year)
    } else {
        (year, year + reach)
    };
    let calendar = HolidayCalendar::scoped(table, scope, first, last);
    calendar
        .add_business_days(Rd(fixed), count)
        .map(|day| day.0)
        .ok_or(Refusal::OutOfRange)
}

/// The number of business days of a table in a scope in the half-open
/// interval from `from_fixed` up to but not including `to_fixed`, as
/// [`HolidayCalendar::business_days_between`] counts them; negative when
/// `to_fixed` is before `from_fixed`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a code that names no table or a `group` that
/// names no group, and [`Refusal::OutOfRange`] for a day with no Gregorian year or two days
/// more than a hundred years apart.
pub fn business_days_between(
    code: &str,
    region: Option<&str>,
    group: Option<&str>,
    from_fixed: i64,
    to_fixed: i64,
) -> Answer<i64> {
    let table = rule_set(code)?;
    let scope = requested_scope(region, group)?;
    let (from, to) = (year_of(from_fixed)?, year_of(to_fixed)?);
    let (first, last) = (from.min(to), from.max(to));
    if last - first > 100 {
        return Err(Refusal::OutOfRange);
    }
    HolidayCalendar::scoped(table, scope, first, last)
        .business_days_between(Rd(from_fixed), Rd(to_fixed))
        .ok_or(Refusal::OutOfRange)
}

/// The line of `hc_lectionary`: the liturgical year a day falls in, named
/// by the civil year of its Easter; the Sunday cycle, `A`, `B` or `C`; the
/// Roman weekday cycle, `I` or `II`; the Revised Common Lectionary's
/// Proper, 3 to 29, for a Sunday after Trinity Sunday, else empty; the
/// Roman number of a Sunday in Ordinary Time, 2 to 34, else empty; and the
/// week of Ordinary Time the day falls in, 1 to 34, on the universal
/// calendar and on one that keeps the Epiphany on a Sunday, else empty.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] before
/// [`lectionary::ROMAN_REFORM_IN_EFFECT`], 1 January 1970, and after the
/// liturgical year 4099, the last whose Easter the Gregorian computus gives.
pub fn lectionary_line(fixed: i64) -> Answer<String> {
    let day = Rd(fixed);
    let year = lectionary::liturgical_year(day).ok_or(Refusal::OutOfRange)?;
    let sunday = lectionary::sunday_cycle(day).ok_or(Refusal::OutOfRange)?;
    let weekday = lectionary::roman_weekday_cycle(day).ok_or(Refusal::OutOfRange)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(year)
        .value(sunday.letter())
        .value(weekday.numeral())
        .value_or_empty(lectionary::rcl_proper(day))
        .value_or_empty(lectionary::sunday_in_ordinary_time(day))
        .value_or_empty(lectionary::week_of_ordinary_time(day))
        .value_or_empty(lectionary::week_of_ordinary_time_epiphany_on_sunday(day));
    line.end();
    Ok(out)
}

/// Easter Sunday of a Gregorian year by the astronomical reckoning at the
/// meridian of Jerusalem, as a fixed day.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside
/// [`computus::ASTRONOMICAL_EASTER_FIRST_YEAR`] to
/// [`computus::ASTRONOMICAL_EASTER_LAST_YEAR`].
pub fn astronomical_easter(year: i64) -> Answer<i64> {
    computus::astronomical_easter(year)
        .map(|day| day.0)
        .ok_or(Refusal::OutOfRange)
}

/// The paschal full moon of a Gregorian year by the astronomical
/// reckoning, the day at Jerusalem of the first full moon at or after the
/// March equinox, as a fixed day: the day [`astronomical_easter`] is the
/// Sunday after.
///
/// # Errors
///
/// As [`astronomical_easter`].
pub fn astronomical_paschal_full_moon(year: i64) -> Answer<i64> {
    computus::astronomical_paschal_full_moon(year)
        .map(|day| day.0)
        .ok_or(Refusal::OutOfRange)
}

/// How many columns [`holy_year_line`] writes.
pub const HOLY_YEAR_COLUMNS: usize = 10;

/// How many columns [`common_worship_lines`] write.
pub const COMMON_WORSHIP_COLUMNS: usize = 4;

/// The fixed day of a table date, if the date exists.
fn table_day(date: TableDate) -> Option<i64> {
    let (year, month, day) = date;
    gregorian::to_fixed(year, month, day).ok().map(|day| day.0)
}

/// A jubilee's cells of [`holy_year_line`], after the first.
fn jubilee_cells(line: &mut Line<'_>, jubilee: &Jubilee) {
    line.cell(jubilee.title)
        .cell(match jubilee.kind {
            JubileeKind::Ordinary => "ordinary",
            JubileeKind::Extraordinary => "extraordinary",
        })
        .cell(jubilee.pope)
        .cell(jubilee.bull);
    for date in [jubilee.given, jubilee.opens, jubilee.closes] {
        line.value_or_empty(table_day(date));
    }
    for date in [
        jubilee.particular_churches.map(|(opens, _)| opens),
        jubilee.particular_churches.map(|(_, closes)| closes),
    ] {
        line.value_or_empty(date.and_then(table_day));
    }
}

/// The line of `hc_holy_year_on`: `within` when a day falls in a Holy Year
/// of the Catholic Church in Rome, from the opening of the Holy Door of
/// St Peter's to its closing, or `outside`; then, within one, the
/// jubilee's title, `ordinary` or `extraordinary`, the Pope who proclaimed
/// it, the bull of indiction by its opening words, the day the bull was
/// given and the jubilee's first and last days in Rome, as fixed days, and
/// its first and last days in the dioceses where the bull dates them, else
/// empty. Outside one the nine cells after the first are empty.
///
/// # Errors
///
/// [`Refusal::NoData`] before the first jubilee the table carries, 1975's
/// opening on 24 December 1974, and after the day its sources were checked,
/// [`holy_years::SOURCES_CHECKED`].
pub fn holy_year_line(fixed: i64) -> Answer<String> {
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    match holy_years::holy_year_on(Rd(fixed)) {
        HolyYearOn::Within(jubilee) => {
            line.cell("within");
            jubilee_cells(&mut line, jubilee);
        }
        HolyYearOn::Outside => {
            line.cell("outside").empties(HOLY_YEAR_COLUMNS - 1);
        }
        HolyYearOn::NotCarried => return Err(Refusal::NoData),
    }
    line.end();
    Ok(out)
}

/// The identifier a [`Rank`] is written as.
const fn rank_id(rank: Rank) -> &'static str {
    match rank {
        Rank::PrincipalFeast => "principal-feast",
        Rank::PrincipalHolyDay => "principal-holy-day",
        Rank::Festival => "festival",
    }
}

/// The lines of `hc_common_worship_on`: every Principal Feast, Principal
/// Holy Day and Festival of the Church of England's *Common Worship*
/// calendar kept on a day, after the transfers its Rules require, one line
/// each: the title as the Rules print it, which is the name
/// `hc_holidays_on` gives it in the `common-worship` table; the rank's
/// identifier, `principal-feast`, `principal-holy-day` or `festival`; and
/// the rank's English name; and the celebration's identifier, the one
/// `hc_holidays_on` writes in its last column for the same entry, which
/// is what joins the two. A day that keeps none writes nothing.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day with no Gregorian year.
pub fn common_worship_lines(fixed: i64) -> Answer<String> {
    let day = Rd(fixed);
    gregorian::year_from_fixed(day).map_err(|_| Refusal::OutOfRange)?;
    let calendar = HolidayCalendar::for_day(&COMMON_WORSHIP, None, day);
    let mut out = String::new();
    for holiday in calendar.on(day) {
        let Some(celebration) = CELEBRATIONS
            .iter()
            .find(|celebration| holiday.id.is(celebration.id))
        else {
            continue;
        };
        let mut line = Line::new(&mut out);
        line.cell(celebration.title)
            .cell(rank_id(celebration.rank))
            .cell(celebration.rank.english_name())
            .cell(celebration.id);
        line.end();
    }
    Ok(out)
}

/// How many columns each line of [`holiday_groups_lines`] writes.
pub const HOLIDAY_GROUP_COLUMNS: usize = 4;

/// The lines of `hc_holiday_groups`: every group of people a holiday may
/// be given to alone, [`hc_holiday::group::GROUPS`], in its order — the
/// identifier, which `hc_holidays_on`'s group column writes; the name in a
/// locale, by [`hc_i18n::holiday_groups::group_name`], from an instrument
/// in the language, else empty; the tag that named it; and the English
/// name.
#[must_use]
pub fn holiday_groups_lines(tag: &str) -> String {
    let locale = Locale::parse(tag).unwrap_or(Locale::ROOT);
    let mut out = String::new();
    for group in hc_holiday::group::GROUPS {
        let named = hc_i18n::holiday_groups::group_name(&locale, group.id);
        let mut line = Line::new(&mut out);
        line.cell(group.id)
            .cell_or_empty(named.map(|named| named.name))
            .cell_or_empty(named.map(|named| named.tag))
            .cell(group.english_name);
        line.end();
    }
    out
}

/// The lines of `hc_holidays_on_in`: [`holidays_on`]'s lines less their
/// last cell, with two more cells — the day's name in a locale, by
/// [`hc_i18n::holiday_names::holiday_name`], where a source in the
/// language names that day of that table by its identifier, and the tag
/// that named it, both empty where none does — and then the identifier,
/// the cell `hc_holidays_on` ends with, so that the first eleven cells and
/// the two names keep their places.
///
/// # Errors
///
/// As [`holidays_on`].
pub fn holidays_on_in_lines(fixed: i64, tag: &str) -> Answer<String> {
    let locale = Locale::parse(tag).unwrap_or(Locale::ROOT);
    let lines = holidays_on(fixed)?;
    let mut out = String::with_capacity(lines.len());
    for line in lines.lines() {
        // The holiday's identifier is the line's last cell, which goes
        // after the two new ones so that every cell of `hc_holidays_on`
        // before it, and the two names, keep their places.
        let (cells, id) = line.rsplit_once('\t').unwrap_or((line, ""));
        let table = cells.split('\t').next().unwrap_or("");
        let named = hc_i18n::holiday_names::holiday_name(&locale, table, id);
        out.push_str(cells);
        // The line's last cell goes on as an empty first cell of the rest,
        // so that the three more are each after a tab.
        let mut rest = Line::new(&mut out);
        rest.cell("")
            .cell_or_empty(named.map(|named| named.name))
            .cell_or_empty(named.map(|named| named.tag))
            .cell(id);
        rest.end();
    }
    Ok(out)
}

/// A class of the 1960 rubrics as the word a line carries: `first`,
/// `second`, `third`, `fourth` or `commemoration`.
const fn class_id(class: roman_calendar_1960::Class) -> &'static str {
    use roman_calendar_1960::Class;
    match class {
        Class::First => "first",
        Class::Second => "second",
        Class::Third => "third",
        Class::Fourth => "fourth",
        Class::Commemoration => "commemoration",
    }
}

/// How many columns each line of [`roman_1960_office_lines`] writes.
pub const ROMAN_1960_COLUMNS: usize = 5;

/// The lines of `hc_roman_1960_office_on`: what the Roman calendar of the
/// 1960 rubrics does on a day, [`roman_calendar_1960::office_on`] — first
/// the office kept (`office`), then each commemoration made, in order
/// (`commemoration`), each feast of the I class impeded here and
/// transferred (`transferred`), and each day the calendar lists here that
/// is neither kept, commemorated nor transferred (`omitted`). Each line:
/// that role; the title as the translation prints it; the class,
/// `first`, `second`, `third`, `fourth` or `commemoration`, and its English
/// name; and, on the office's line, the fixed day a feast of the I class
/// transferred here was impeded on, else empty.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside the years 1583 to 4099,
/// whose Easter the Gregorian computus gives.
pub fn roman_1960_office_lines(fixed: i64) -> Answer<String> {
    let office = roman_calendar_1960::office_on(Rd(fixed)).ok_or(Refusal::OutOfRange)?;
    let mut out = String::new();
    let mut push =
        |role: &str, celebration: &roman_calendar_1960::Celebration, from: Option<Rd>| {
            let mut line = Line::new(&mut out);
            line.cell(role)
                .cell(celebration.title)
                .cell(class_id(celebration.class))
                .cell(celebration.class.english_name())
                .value_or_empty(from.map(|day| day.0));
            line.end();
        };
    push("office", office.office, office.transferred_from);
    for celebration in &office.commemorations {
        push("commemoration", celebration, None);
    }
    for celebration in &office.transferred {
        push("transferred", celebration, None);
    }
    for celebration in &office.omitted {
        push("omitted", celebration, None);
    }
    Ok(out)
}

/// The reckoning of the fasts an identifier names, one of
/// [`Reckoning::ALL`] — `orthodox-fasts`, `orthodox-fasts-revised-julian`,
/// `armenian-fasts`, `armenian-fasts-jerusalem`,
/// `armenian-fasts-fifty-days`, `coptic-fasts` or `ethiopian-fasts` — by
/// [`Reckoning::by_id`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other.
pub fn orthodox_fasts_reckoning(id: &str) -> Answer<Reckoning> {
    Reckoning::by_id(id).ok_or(Refusal::Unknown)
}

/// The identifier a [`PeriodKind`] is written as.
const fn period_kind_id(kind: PeriodKind) -> &'static str {
    match kind {
        PeriodKind::Fast => "fast",
        PeriodKind::FastFree => "fast-free",
        PeriodKind::MeatExcluded => "meat-excluded",
    }
}

/// How many columns [`orthodox_fast_line`] writes.
pub const ORTHODOX_FAST_COLUMNS: usize = 6;

/// The line of `hc_orthodox_fast_on`: what a day is in the fasting scheme
/// of a reckoning — `1` if it is a fast day, else `0`; `period`,
/// `weekly-fast` for a Wednesday or Friday in no period, or `none`; and
/// for a period its identifier, its English name as the OCA's outline
/// gives it and its kind, `fast`, `fast-free` or `meat-excluded`, else three
/// empty cells; and what the day abstains from, `nothing`, `meat` or
/// `fast`, which alone tells a day of the Meatfast, not a fast day but one
/// without meat, from an ordinary one.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a reckoning [`orthodox_fasts_reckoning`] does
/// not name, and [`Refusal::OutOfRange`] for a day outside the years 326
/// to 4099 of the reckoning's calendar.
pub fn orthodox_fast_line(reckoning: &str, fixed: i64) -> Answer<String> {
    let reckoning = orthodox_fasts_reckoning(reckoning)?;
    // A generous bound first, so that no calendar arithmetic meets a day
    // near the ends of an `i64`: the years 300 to 4200.
    if !(100_000..=1_600_000).contains(&fixed) {
        return Err(Refusal::OutOfRange);
    }
    let status = orthodox_fasts::status(&reckoning, Rd(fixed)).ok_or(Refusal::OutOfRange)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.flag(status.is_fast_day());
    match status {
        Status::InPeriod(period) => line
            .cell("period")
            .cell(period.id)
            .cell(period.english_name)
            .cell(period_kind_id(period.kind)),
        Status::WeeklyFast(_) => line.cell("weekly-fast").empties(3),
        Status::NotFasting => line.cell("none").empties(3),
    };
    line.cell(match status.abstinence() {
        Abstinence::Nothing => "nothing",
        Abstinence::Meat => "meat",
        Abstinence::Fast => "fast",
    });
    line.end();
    Ok(out)
}

/// How many columns [`orthodox_fast_seasons_lines`] writes.
pub const ORTHODOX_FAST_SEASON_COLUMNS: usize = 5;

/// The lines of `hc_orthodox_fast_seasons`: every period of the scheme
/// that begins in a year of a reckoning's calendar, one a line in the
/// order a day is tested against them, as its identifier, its English
/// name, its kind (`fast`, `fast-free` or `meat-excluded`), and its first and last days as fixed days, both
/// included; both are empty in a year the period does not happen, which
/// only the Apostles' Fast does, on the Revised Julian reckoning when
/// Pascha is late. Christmastide ends in the next year.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a reckoning [`orthodox_fasts_reckoning`] does
/// not name, and [`Refusal::OutOfRange`] for a year outside 326 to 4099.
pub fn orthodox_fast_seasons_lines(reckoning: &str, year: i64) -> Answer<String> {
    let reckoning = orthodox_fasts_reckoning(reckoning)?;
    let (first_year, last_year) = reckoning.years;
    if !(first_year..=last_year).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let mut out = String::new();
    for period in reckoning.periods {
        let mut line = Line::new(&mut out);
        line.cell(period.id)
            .cell(period.english_name)
            .cell(period_kind_id(period.kind));
        if let Some((first, last)) = orthodox_fasts::span(&reckoning, period, year) {
            line.value(first.0).value(last.0);
        } else {
            line.empties(2);
        }
        line.end();
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::{BTreeMap, BTreeSet};
    use alloc::vec::Vec;
    use hc_holiday::orthodox_fasts::Period;

    fn ymd(year: i64, month: u8, day: u8) -> i64 {
        gregorian::to_fixed(year, month, day).expect("a date").0
    }

    /// A municipality's lines (ADR 0014): asked for さいたま市, a year's
    /// line of Saitama's 県民の日 carries the prefecture's code and the
    /// city's own day the city's; `hc_holidays_on` writes the prefecture's
    /// day once, under `JP-11`, and the city's under `JP-11-100`. (14
    /// November 2026 is also Yamagata's education day, a second Saturday.)
    #[test]
    fn a_city_s_lines_name_the_widest_region_whose_entry_each_is() {
        let lines = holidays_in_year("JP", Some("jp-11-100"), None, None, 2026).expect("JP");
        let region_of = |date: &str, name: &str| {
            lines
                .lines()
                .map(|line| line.split('\t').collect::<Vec<_>>())
                .find(|cells| cells[0] == date && cells[2] == name)
                .map(|cells| String::from(cells[7]))
        };
        assert_eq!(
            region_of("2026-11-14", "県民の日").as_deref(),
            Some("JP-11")
        );
        assert_eq!(
            region_of("2026-05-01", "さいたま市民の日").as_deref(),
            Some("JP-11-100")
        );
        assert_eq!(region_of("2026-01-01", "元日").as_deref(), Some(""));
        let on = |month, day| {
            holidays_on(ymd(2026, month, day))
                .expect("in range")
                .lines()
                .filter(|line| line.starts_with("JP\t"))
                .map(|line| line.split('\t').collect::<Vec<_>>())
                .filter(|cells| cells[4] != "gap" && cells[9].starts_with("JP-11"))
                .map(|cells| (String::from(cells[3]), String::from(cells[9])))
                .collect::<Vec<_>>()
        };
        let saitama = |name: &str, region: &str| (String::from(name), String::from(region));
        assert_eq!(on(11, 14), [saitama("県民の日", "JP-11")]);
        assert_eq!(on(5, 1), [saitama("さいたま市民の日", "JP-11-100")]);
        // An unread town's gap is written with its code as asked.
        let town = holidays_in_year("JP", Some("JP-14-204"), None, None, 2026).expect("JP");
        assert!(
            town.lines()
                .any(|line| line.contains("\tgap\t") && line.contains("\tJP-14-204\t")),
            "{town}"
        );
    }

    /// The worked example of `docs/systems/orthodox-fasts.md`, from the
    /// OCA's outline (`oca-fasting-seasons`): in 2025 Great Lent began on
    /// 3 March, Bright Wednesday, 23 April, was fast-free, and the week
    /// after the Publican and the Pharisee made Wednesday 12 February
    /// fast-free, as the Holy Trinity calendar marks it
    /// (`holy-trinity-calendar`).
    #[test]
    fn the_fasts_of_2025_are_the_worked_examples() {
        let line = |reckoning: &str, fixed: i64| {
            let line = orthodox_fast_line(reckoning, fixed).expect("in range");
            let cells: Vec<String> = line
                .trim_end_matches('\n')
                .split('\t')
                .map(String::from)
                .collect();
            assert_eq!(cells.len(), ORTHODOX_FAST_COLUMNS);
            cells
        };
        assert_eq!(
            line("orthodox-fasts", ymd(2025, 3, 3)),
            [
                "1",
                "period",
                "great-lent",
                "Great Lent & Holy Week",
                "fast",
                "fast"
            ]
        );
        assert_eq!(
            line("orthodox-fasts", ymd(2025, 4, 23))[..3],
            ["0", "period", "bright-week"]
        );
        assert_eq!(
            line("orthodox-fasts-revised-julian", ymd(2025, 2, 12))[..3],
            ["0", "period", "publican-and-pharisee-week"]
        );
        // Wednesday 1 October 2025 is in no period on either reckoning.
        assert_eq!(
            line("ORTHODOX-FASTS", ymd(2025, 10, 1)),
            ["1", "weekly-fast", "", "", "", "fast"]
        );
        // Wednesday 18 February 2026 is in the Meatfast, Cheesefare week,
        // Pascha being 12 April: no fast day, though a Wednesday, but no
        // meat (`oca-fasting-seasons`).
        assert_eq!(
            line("orthodox-fasts", ymd(2026, 2, 18)),
            [
                "0",
                "period",
                "meatfast",
                "Meatfast",
                "meat-excluded",
                "meat"
            ]
        );
        assert_eq!(
            line("orthodox-fasts", ymd(2025, 10, 2)),
            ["0", "none", "", "", "", "nothing"]
        );
        assert_eq!(
            orthodox_fast_line("coptic", ymd(2025, 1, 1)),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            orthodox_fast_line("orthodox-fasts", i64::MIN),
            Err(Refusal::OutOfRange)
        );
    }

    /// The Apostles' Fast of 2025 ran from Monday 16 June to 11 July on the
    /// Julian reckoning and to 28 June on the Revised Julian one
    /// (`wikipedia-apostles-fast`); in 2024 it did not happen on the
    /// Revised Julian reckoning, the Monday after All Saints' being 1 July
    /// (`trueorthodox-apostles-2024`).
    #[test]
    fn the_apostles_fast_moves_and_vanishes() {
        let apostles = |reckoning: &str, year: i64| {
            let text = orthodox_fast_seasons_lines(reckoning, year).expect("in range");
            let rows: Vec<Vec<String>> = text
                .lines()
                .map(|line| line.split('\t').map(String::from).collect())
                .collect();
            assert_eq!(rows.len(), Period::ALL.len());
            for row in &rows {
                assert_eq!(row.len(), ORTHODOX_FAST_SEASON_COLUMNS);
            }
            rows.into_iter()
                .find(|row| row[0] == "apostles-fast")
                .expect("the Apostles' Fast has a line")
        };
        let julian = apostles("orthodox-fasts", 2025);
        assert_eq!(
            julian[2..],
            [
                "fast".to_owned(),
                ymd(2025, 6, 16).to_string(),
                ymd(2025, 7, 11).to_string()
            ]
        );
        let revised = apostles("orthodox-fasts-revised-julian", 2025);
        assert_eq!(revised[4], ymd(2025, 6, 28).to_string());
        let vanished = apostles("orthodox-fasts-revised-julian", 2024);
        assert_eq!(vanished[3..], ["", ""]);
        assert_eq!(
            orthodox_fast_seasons_lines("orthodox-fasts", 325),
            Err(Refusal::OutOfRange)
        );
    }

    fn day(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    #[test]
    fn every_table_has_an_english_name_unique_within_its_kind() {
        let mut seen = BTreeSet::new();
        let mut count = 0;
        for (set, kind) in tables_with_kinds() {
            count += 1;
            assert!(!set.english_name.trim().is_empty(), "{}", set.code);
            assert!(
                seen.insert((kind.name(), set.english_name)),
                "two {} tables are called {}",
                kind.name(),
                set.english_name
            );
        }
        assert_eq!(count, tables().count());
    }

    /// The WebAssembly README's timing of one day states how many tables
    /// the day was read across; that number is this list's length.
    #[test]
    fn the_wasm_readme_counts_the_tables_there_are() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../hyper-calendar-wasm/README.md"
        );
        let readme = std::fs::read_to_string(path).expect("the WebAssembly README");
        let stated = format!("across all {} tables", tables().count());
        assert!(
            readme
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .contains(&stated),
            "the README should say {stated:?}"
        );
    }

    #[test]
    fn the_tables_are_listed_in_code_order_with_their_kinds() {
        let english = holiday_tables("en-GB");
        let rows: Vec<Vec<&str>> = english
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert!(rows.iter().all(|row| row.len() == HOLIDAY_TABLES_COLUMNS));
        let codes: Vec<&str> = tables().map(|set| set.code).collect();
        assert_eq!(rows.iter().map(|row| row[0]).collect::<Vec<_>>(), codes);
        let row = |code: &str| rows.iter().find(|row| row[0] == code).expect("a table");
        assert_eq!(
            row("JP")[1..6],
            ["country", "Japan", "Japan", "en", row("JP")[5]]
        );
        assert_eq!(row("XJPX")[1], "exchange");
        assert_eq!(row("XJPX")[6], "JP");
        assert_eq!(row("XLON")[6], "GB");
        assert_eq!(row("XNYS")[6], "");
        assert_eq!(row("christian-western")[1], "tradition");
        assert_eq!(row("un-days")[1], "observance");
        assert!(rows.iter().all(|row| row[1] != "subdivision"));
    }

    /// Columns 10 and 11 list the groups a table gives days to alone, by
    /// identifier and by name, in the locale where `hc-i18n` names them and
    /// in English otherwise; a table that names none has both empty.
    #[test]
    fn the_tables_list_their_groups_by_identifier_and_name() {
        let english = rows_in("en");
        assert_eq!(english["CN"][9], "children;military;women;youth");
        assert_eq!(english["CN"][10], "children;military personnel;women;youth");
        let chinese = rows_in("zh-CN");
        assert_eq!(chinese["CN"][9], "children;military;women;youth");
        assert_eq!(chinese["CN"][10], "少年儿童;现役军人;妇女;青年");
        assert_eq!(
            chinese["TW"][9],
            "coast-guard;firefighters;indigenous-peoples;military;police"
        );
        assert_eq!(chinese["TW"][10].split(';').nth(3), Some("现役军人"));
        assert_eq!(english["JP"][9], "");
        assert_eq!(english["JP"][10], "");
        assert_eq!(rows_in("native")["CN"][10], english["CN"][10]);
    }

    /// A group's own day is a line of the year with the group in the last
    /// column, and is not a line of everyone's year; a day everyone has
    /// keeps the column empty.
    #[test]
    fn a_group_s_own_day_is_marked_with_its_group() {
        let china = rule_set("CN").expect("China");
        let women = year_lines(china, Scope::group(" WOMEN "), &[], 2026);
        let own: Vec<&str> = women
            .lines()
            .filter(|line| line.split('\t').nth(8) == Some("women"))
            .collect();
        assert_eq!(
            own,
            [concat!(
                "2026-03-08\tWomen's Day\t妇女节\thalf-day\texact\t0\t\t\twomen\twomens-day\t",
                "全国年节及纪念日放假办法, 第三条 (一): 妇女放假半天"
            )]
        );
        assert!(women.starts_with(
            "2026-01-01\tNew Year's Day\t元旦\tpublic\texact\t0\t\t\t\tnew-years-day\t"
        ));
        let everyone = year_lines(china, Scope::EVERYONE, &[], 2026);
        assert!(!everyone.contains("妇女节"));
        assert_eq!(
            everyone.lines().count() + 1,
            women.lines().count(),
            "{women}"
        );
    }

    /// On 8 March the day's lines have China's half day once, as a line of
    /// the group; the nationwide lines do not have it.
    #[test]
    fn the_day_s_lines_carry_each_group_s_own_day() {
        let day = ymd(2026, 3, 8);
        let text = holidays_on(day).expect("a day");
        let women: Vec<&str> = text
            .lines()
            .filter(|line| line.starts_with("CN\t") && line.contains("妇女节"))
            .collect();
        assert_eq!(women.len(), 1, "{text}");
        assert!(women[0].ends_with(
            "\thalf-day\texact\t全国年节及纪念日放假办法, 第三条 (一): 妇女放假半天\t0\t\t\twomen\twomens-day"
        ));
        assert!(
            text.lines().all(|line| line.split('\t').count() == 12),
            "{text}"
        );
        assert_eq!(
            is_day_off("CN", None, Some("children"), ymd(2026, 6, 1)),
            Ok(true)
        );
        assert_eq!(is_day_off("CN", None, None, ymd(2026, 6, 1)), Ok(false));
        assert_eq!(
            is_day_off("CN", None, Some("women"), ymd(2027, 3, 8)),
            Ok(false)
        );
    }

    /// The rows of [`holiday_tables`] in a locale, by code.
    fn rows_in(locale: &str) -> BTreeMap<String, Vec<String>> {
        holiday_tables(locale)
            .lines()
            .map(|line| {
                let cells: Vec<String> = line.split('\t').map(String::from).collect();
                (cells[0].clone(), cells)
            })
            .collect()
    }

    /// CLDR 48 `ja.xml` has `<territory type="JP">日本</territory>` and
    /// `de.xml` `<territory type="DE">Deutschland</territory>`, both
    /// approved.
    #[test]
    fn japanese_names_japan_and_german_names_germany() {
        let japanese = rows_in("ja");
        assert_eq!(japanese["JP"][2..5], ["日本", "Japan", "ja"]);
        assert_eq!(rows_in("ja-JP")["JP"][2..5], ["日本", "Japan", "ja"]);
        let german = rows_in("de-AT");
        assert_eq!(german["DE"][2..5], ["Deutschland", "Germany", "de"]);
        assert_eq!(german["JP"][2], "Japan");
        assert_eq!(german["JP"][4], "de");
        // English is CLDR's English: its plain value for HK, not the
        // table's own name, which column 4 keeps.
        let english = rows_in("en");
        assert_eq!(
            english["HK"][2..5],
            ["Hong Kong SAR China", "Hong Kong", "en"]
        );
        // Column 8 is CLDR's short name in the same data: `en.xml` and
        // `ja.xml` both shorten Hong Kong, and neither Japan.
        assert_eq!(english["HK"][7], "Hong Kong");
        assert_eq!(english["GB"][7], "UK");
        assert_eq!(english["JP"][7], "");
        assert_eq!(japanese["HK"][2], "中華人民共和国香港特別行政区");
        assert_eq!(japanese["HK"][7], "香港");
        assert_eq!(japanese["MO"][7], "マカオ");
        assert_eq!(japanese["PS"][7], "パレスチナ");
        assert_eq!(japanese["JP"][7], "");
        assert_eq!(rows_in("ja-JP")["HK"][7], "香港");
        assert_eq!(rows_in("en-GB")["US"][7], "US");
        assert_eq!(japanese["XHKG"][7], "", "an exchange has no short name");
        // An exchange and a tradition stay English, whatever the locale;
        // column 7 stays the code of the country's own row.
        assert_eq!(
            japanese["XJPX"][2..5],
            [
                "Tokyo Stock Exchange (JPX)",
                "Tokyo Stock Exchange (JPX)",
                "en"
            ]
        );
        assert_eq!(japanese["XJPX"][6], "JP");
        assert_eq!(japanese["christian-western"][4], "en");
    }

    /// Kabyle's `HK` is unconfirmed in CLDR 48 `kab.xml`, and every Coptic
    /// value is: each falls back to CLDR's English name, answered by `en`,
    /// beside the names the same locales do carry — the same name and short
    /// name `en` itself gives, not the table's own English name where the
    /// two differ. Tibetan's `FR` is unconfirmed in `bo.xml` too, but
    /// language matching gives Tibetan Simplified Chinese as its fallback
    /// before English (`languageInfo.xml`, `bo` ⇒ `zh`), so `zh.xml`'s 法国
    /// answers.
    #[test]
    fn a_country_the_locale_does_not_name_falls_back_to_cldr_english() {
        let english = rows_in("en");
        let kabyle = rows_in("kab");
        assert_eq!(
            kabyle["HK"][2..5],
            ["Hong Kong SAR China", "Hong Kong", "en"]
        );
        assert_eq!(kabyle["HK"][7], "Hong Kong");
        assert_eq!(kabyle["JP"][2..5], ["Jappu", "Japan", "kab"]);
        let tibetan = rows_in("bo");
        assert_eq!(tibetan["FR"][2..5], ["法国", "France", "zh-Hans"]);
        assert_eq!(tibetan["JP"][4], "bo");
        for tag in ["cop", "native", "und", "not a tag"] {
            let rows = rows_in(tag);
            for (code, row) in &rows {
                assert_eq!(row[4], "en", "{tag} {code}");
                let in_english = &english[code];
                assert_eq!(
                    (&row[2], &row[7]),
                    (&in_english[2], &in_english[7]),
                    "{tag} {code}"
                );
            }
        }
        // Wherever `en` answers for a country, it gives `en`'s own name.
        for tag in ["kab", "zgh", "bo", "sa"] {
            for (code, row) in rows_in(tag) {
                if row[4] == "en" && row[1] == "country" {
                    assert_eq!(row[2], english[&code][2], "{tag} {code}");
                }
            }
        }
    }

    /// Every table a line names a country by is one `hc-i18n` has names
    /// for, and CLDR's English names them all.
    #[test]
    fn every_country_and_every_exchanges_country_is_a_territory() {
        for (set, kind) in tables_with_kinds() {
            let code = match kind {
                TableKind::Country => Some(set.code),
                _ => country_of(set, kind),
            };
            if let Some(code) = code {
                let place = place_names::territory(code).expect("a territory");
                assert_eq!(place.status(), place_names::Status::Regular, "{code}");
                assert!(place.english_name().is_some(), "{code}");
            }
        }
    }

    /// The names a page shows, column 3, are distinct within a kind in
    /// every locale `hc-i18n` carries, in `native` and in English, as the
    /// English names are.
    #[test]
    fn no_two_tables_of_a_kind_share_a_name_in_any_locale() {
        let tags = ["en", "native", "und"]
            .into_iter()
            .chain(hc_i18n::data::LOCALES.iter().map(|data| data.tag));
        for tag in tags {
            let mut seen = BTreeMap::new();
            for row in rows_in(tag).into_values() {
                assert!(!row[2].is_empty(), "{tag} {}", row[0]);
                let key = (row[1].clone(), row[2].clone());
                if let Some(other) = seen.insert(key, row[0].clone()) {
                    panic!("{tag}: {other} and {} are both {}", row[0], row[2]);
                }
            }
        }
    }

    /// The system page's worked year: Advent 2025 begins 2026, Year A and
    /// Year II (`docs/systems/lectionary-cycles.md`), and Christ the King,
    /// 22 November 2026, is Proper 29.
    #[test]
    fn the_liturgical_year_2026_is_year_a_and_year_ii() {
        assert_eq!(
            lectionary_line(day(2025, 11, 30)).as_deref(),
            Ok("2026\tA\tII\t\t\t\t\n")
        );
        assert_eq!(
            lectionary_line(day(2025, 11, 29)).as_deref(),
            Ok("2025\tC\tI\t\t\t34\t34\n")
        );
        assert_eq!(
            lectionary_line(day(2026, 11, 22)).as_deref(),
            Ok("2026\tA\tII\t29\t34\t34\t34\n")
        );
        // 12 January 2026, the Monday after the Baptism on both calendars.
        assert_eq!(
            lectionary_line(day(2026, 1, 12)).as_deref(),
            Ok("2026\tA\tII\t\t\t1\t1\n")
        );
        // 9 January 2023: Ordinary Time on the universal calendar, the
        // Baptism where the Epiphany was Sunday 8 January.
        assert_eq!(
            lectionary_line(day(2023, 1, 9)).as_deref(),
            Ok("2023\tA\tI\t\t\t1\t\n")
        );
        assert_eq!(lectionary_line(day(1500, 1, 1)), Err(Refusal::OutOfRange));
    }

    /// The World Council of Churches' Aleppo table, as `computus`'s tests
    /// read it: the full moon of Sunday 8 April 2001 puts Easter on
    /// 15 April.
    #[test]
    fn the_astronomical_easter_of_2001_is_the_fifteenth_of_april() {
        assert_eq!(astronomical_easter(2001), Ok(day(2001, 4, 15)));
        assert_eq!(astronomical_easter(1582), Err(Refusal::OutOfRange));
    }

    /// The Aleppo statement's table (`wcc-aleppo-1997`) puts the
    /// astronomical vernal full moon of 2001 on Sunday 8 April, a week
    /// before Easter, and that of 2019 on 21 March, the day of the equinox.
    #[test]
    fn the_paschal_full_moons_of_2001_and_2019_are_the_aleppo_tables() {
        assert_eq!(astronomical_paschal_full_moon(2001), Ok(day(2001, 4, 8)));
        assert_eq!(astronomical_paschal_full_moon(2019), Ok(day(2019, 3, 21)));
        assert_eq!(astronomical_easter(2019), Ok(day(2019, 3, 24)));
        assert_eq!(
            astronomical_paschal_full_moon(2151),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            astronomical_paschal_full_moon(1582),
            Err(Refusal::OutOfRange)
        );
    }

    /// *Spes non confundit*, 6: the Holy Door of St Peter's opened on
    /// 24 December 2024 and closed on 6 January 2026, and the jubilee ran in
    /// the dioceses from 29 December 2024 to 28 December 2025.
    #[test]
    fn the_jubilee_of_2025_is_the_bulls() {
        let line = holy_year_line(day(2025, 6, 1)).expect("carried");
        let row: Vec<&str> = line.trim_end_matches('\n').split('\t').collect();
        assert_eq!(row.len(), HOLY_YEAR_COLUMNS);
        assert_eq!(
            row[..5],
            [
                "within",
                "Ordinary Jubilee of the Year 2025",
                "ordinary",
                "Francis",
                "Spes non confundit"
            ]
        );
        let days = [
            day(2024, 5, 9),
            day(2024, 12, 24),
            day(2026, 1, 6),
            day(2024, 12, 29),
            day(2025, 12, 28),
        ]
        .map(|day| day.to_string());
        assert_eq!(row[5..], days);
        let mercy = holy_year_line(day(2016, 1, 1)).expect("carried");
        assert!(
            mercy.contains("\textraordinary\tFrancis\tMisericordiae vultus\t"),
            "{mercy}"
        );
        assert!(mercy.ends_with("\t\t\n"), "{mercy}");
        assert_eq!(
            holy_year_line(day(2026, 1, 7)).as_deref(),
            Ok("outside\t\t\t\t\t\t\t\t\t\n")
        );
        assert_eq!(holy_year_line(day(1974, 12, 23)), Err(Refusal::NoData));
        assert_eq!(holy_year_line(day(2026, 9, 28)), Err(Refusal::NoData));
    }

    /// Full Fact: St George's Day was kept on Monday 28 April 2025, Easter
    /// being 20 April; Christmas Day is a Principal Feast and Good Friday a
    /// Principal Holy Day.
    #[test]
    fn each_celebration_carries_its_rank() {
        assert_eq!(
            common_worship_lines(day(2025, 4, 28)).as_deref(),
            Ok(
                "George, Martyr, Patron of England\tfestival\tFestival\tgeorge-martyr-patron-of-england\n"
            )
        );
        assert_eq!(common_worship_lines(day(2025, 4, 23)).as_deref(), Ok(""));
        assert_eq!(
            common_worship_lines(day(2025, 12, 25)).as_deref(),
            Ok("Christmas Day\tprincipal-feast\tPrincipal Feast\tchristmas-day\n")
        );
        assert_eq!(
            common_worship_lines(day(2025, 4, 18)).as_deref(),
            Ok("Good Friday\tprincipal-holy-day\tPrincipal Holy Day\tgood-friday\n")
        );
        assert_eq!(common_worship_lines(i64::MAX), Err(Refusal::OutOfRange));
    }

    /// A table of one holiday whose names hold a tab and line breaks, as
    /// data can.
    fn table_with_separators_in_its_names() -> &'static RuleSet {
        use hc_holiday::rule::{HolidayRule, Rule, SATURDAY_SUNDAY, SourceDate, Subdivisions};
        static RULES: [HolidayRule; 1] = [HolidayRule::public(
            "New\tYear's\nDay",
            "元\r\n日",
            Rule::FixedGregorian { month: 1, day: 1 },
        )];
        static TABLE: RuleSet = RuleSet {
            code: "XX",
            english_name: "A\ttest",
            rules: &RULES,
            substitution: &[],
            bridges: &[],
            includes: &[],
            weekend: SATURDAY_SUNDAY,
            sources_checked: SourceDate::new(2026, 9, 28),
            sources: "invented for the test",
            subdivisions: Subdivisions::Undivided,
        };
        &TABLE
    }

    /// The names are cells: their tab and line breaks are spaces, and the
    /// line keeps its nine columns. The year's lines wrote them raw
    /// before, so a tab in a name shifted every column after it.
    #[test]
    fn a_holiday_name_with_a_tab_or_a_line_break_keeps_its_columns() {
        let text = year_lines(
            table_with_separators_in_its_names(),
            Scope::EVERYONE,
            &[],
            2026,
        );
        assert_eq!(
            text,
            "2026-01-01\tNew Year's Day\t元  日\tpublic\texact\t0\t\t\t\tnew-years-day\t\n"
        );
    }

    /// The lines of a day write the same names as cells, the table's name
    /// too.
    #[test]
    fn a_holiday_name_with_a_tab_or_a_line_break_keeps_its_columns_on_a_day() {
        let table = table_with_separators_in_its_names();
        let day = Rd(ymd(2026, 1, 1));
        let calendar = HolidayCalendar::for_day(table, None, day);
        let mut out = String::new();
        push_day_lines(&mut out, table, &calendar, day);
        assert_eq!(
            out,
            "XX\tA test\tNew Year's Day\t元  日\tpublic\texact\t\t0\t\t\t\tnew-years-day\n"
        );
    }

    /// The year's lines end with its gaps, as the day's lines write them:
    /// China's statute text read begins in 1999, so the half day of women
    /// in 1998 is a gap line with the group; a state whose code was not
    /// read is a gap line with its region.
    #[test]
    fn the_year_s_lines_write_its_gaps() {
        let women = holidays_in_year("CN", None, Some("women"), None, 1998).unwrap_or_default();
        assert!(
            women.lines().any(|line| line.starts_with(concat!(
                "\tWomen's Day\t妇女节\tgap\t\t0\t\t\twomen\twomens-day\t",
                "全国年节及纪念日放假办法"
            ))),
            "{women}"
        );
        // Everyone's lines of 1998 do not carry the group's gap.
        let everyone = holidays_in_year("CN", None, None, None, 1998).unwrap_or_default();
        assert!(!everyone.lines().any(|line| line.contains("Women's Day")));
        let hampshire = holidays_in_year("US", Some("US-NH"), None, None, 2026).unwrap_or_default();
        assert!(
            hampshire.lines().any(|line| line
                == "\tThe subdivision's own days\t\tgap\t\t0\t\tUS-NH\t\tunread-subdivision\t"),
            "{hampshire}"
        );
        // Every line keeps its nine columns and ends with the identifier and
        // the source.
        for line in women.lines().chain(hampshire.lines()) {
            assert_eq!(line.split('\t').count(), 11, "{line}");
        }
        // A day's gap line carries the rule's source.
        let day = hc_holiday::rule::Scope::group("women");
        let table = rule_set("CN").unwrap_or(&countries::CHINA);
        let march = Rd(ymd(1998, 3, 8));
        let grouped = HolidayCalendar::for_day_scoped(table, day, march);
        let everyone = HolidayCalendar::for_day(table, None, march);
        let mut out = String::new();
        push_scoped_day_lines(&mut out, table, day, &grouped, &[&everyone], march);
        assert!(
            out.lines()
                .any(|line| line.contains("\tgap\t\t全国年节及纪念日放假办法")),
            "{out}"
        );
    }

    /// Every table is found by its own code, and a code that names none is
    /// refused.
    #[test]
    fn every_table_is_the_rule_set_its_code_names() {
        for table in tables() {
            assert_eq!(rule_set(table.code).map(|set| set.code), Ok(table.code));
        }
        assert_eq!(rule_set("ZZ").map(|set| set.code), Err(Refusal::Unknown));
        assert_eq!(holiday_codes().lines().count(), tables().count());
    }

    /// `hc-holiday`'s example: 25 March 1962 was the Third Sunday of Lent,
    /// and the Annunciation, impeded, went to Monday 26 March.
    #[test]
    fn the_1960_office_is_the_rubrics() {
        let sunday = gregorian::to_fixed(1962, 3, 25).expect("a date").0;
        let text = roman_1960_office_lines(sunday).expect("in range");
        let rows: alloc::vec::Vec<alloc::vec::Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert!(rows.iter().all(|row| row.len() == ROMAN_1960_COLUMNS));
        assert_eq!(rows[0][..3], ["office", "Third Sunday of Lent", "first"]);
        assert!(rows.iter().any(|row| row[0] == "transferred"
            && row[1] == "The Annunciation of the Blessed Virgin Mary"));
        let monday = roman_1960_office_lines(sunday + 1).expect("in range");
        assert!(monday.starts_with(&alloc::format!(
            "office\tThe Annunciation of the Blessed Virgin Mary\tfirst\tI class\t{sunday}\n"
        )));
        assert_eq!(roman_1960_office_lines(0), Err(Refusal::OutOfRange));
    }

    /// Wikipedia's "Nayrouz" gives the Bohairic name of the new year,
    /// ⲡⲓⲭⲗⲟⲙ ⲛ̀ⲧⲉ ϯⲣⲟⲙⲡⲓ (`wikipedia-nayrouz`), which `hc-i18n` carries
    /// for `cop`; 11 September 2025 was 1 Thout 1742. The groups are
    /// `hc-holiday`'s, China's women first.
    #[test]
    fn a_day_is_named_in_the_locale_and_the_groups_are_listed() {
        let day = gregorian::to_fixed(2025, 9, 11).expect("a date").0;
        let text = holidays_on_in_lines(day, "cop").expect("in range");
        let plain = holidays_on(day).expect("in range");
        assert_eq!(text.lines().count(), plain.lines().count());
        let nayrouz = text
            .lines()
            .find(|line| line.starts_with("coptic-orthodox\t"))
            .expect("Nayrouz");
        assert!(
            nayrouz.ends_with("\tⲡⲓⲭⲗⲟⲙ ⲛ̀ⲧⲉ ϯⲣⲟⲙⲡⲓ\tcop\tnayrouz-new-year"),
            "{nayrouz}"
        );
        // A gap line keeps the rule's source, as `hc_holidays_on` writes it,
        // and gains the two cells: China's women's half day of 1998, before
        // the statute text read begins.
        let march = gregorian::to_fixed(1998, 3, 8).expect("a date").0;
        let text = holidays_on_in_lines(march, "zh-Hans").expect("in range");
        let gap = text
            .lines()
            .find(|line| line.starts_with("CN\t") && line.contains("\tWomen's Day\t"))
            .expect("a gap line");
        assert!(gap.contains("全国年节及纪念日放假办法"), "{gap}");
        assert_eq!(gap.split('\t').count(), 14, "{gap}");
        let groups = holiday_groups_lines("en");
        assert!(groups.starts_with("women\t"));
        assert!(
            groups
                .lines()
                .all(|line| line.split('\t').count() == HOLIDAY_GROUP_COLUMNS)
        );
    }

    /// Japan's Golden Week of 2026 as the `JP` table has it: 29 April
    /// (昭和の日), 3 to 5 May and the substitute holiday of 6 May, so five
    /// business days after Tuesday 28 April is Monday 11 May, and the
    /// fortnight from 27 April holds six.
    #[test]
    fn business_days_skip_the_holidays() {
        let day = |month, day| gregorian::to_fixed(2026, month, day).expect("a date").0;
        assert_eq!(
            add_business_days("JP", None, None, day(4, 28), 5),
            Ok(day(5, 11))
        );
        assert_eq!(
            add_business_days("JP", None, None, day(5, 11), -5),
            Ok(day(4, 28))
        );
        assert_eq!(
            add_business_days("JP", None, None, day(4, 28), 0),
            Ok(day(4, 28))
        );
        assert_eq!(
            business_days_between("JP", None, None, day(4, 27), day(5, 11)),
            Ok(6)
        );
        assert_eq!(
            add_business_days("XX", None, None, day(4, 28), 1),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            add_business_days("JP", None, None, day(4, 28), MAX_BUSINESS_DAYS + 1),
            Err(Refusal::OutOfRange)
        );
    }

    /// A line of `hc_common_worship_on` and the entry of `hc_holidays_on`
    /// it describes carry the same identifier, so that they are joined on
    /// it and not on the title: every celebration kept in 2025 is the
    /// entry of the same identifier in the year's lines, and the reverse.
    #[test]
    fn a_common_worship_line_joins_its_holidays_on_entry_by_identifier() {
        let christmas = day(2025, 12, 25);
        let rank = common_worship_lines(christmas).expect("in range");
        assert_eq!(
            rank,
            "Christmas Day\tprincipal-feast\tPrincipal Feast\tchristmas-day\n"
        );
        let on = holidays_on(christmas).expect("in range");
        assert!(
            on.lines()
                .any(|line| line.starts_with("common-worship\t")
                    && line.ends_with("\tchristmas-day")),
            "{on}"
        );
        let mut kept = alloc::vec::Vec::new();
        for fixed in day(2025, 1, 1)..=day(2025, 12, 31) {
            let text = common_worship_lines(fixed).expect("in range");
            kept.extend(text.lines().map(|line| {
                let cells: alloc::vec::Vec<&str> = line.split('\t').collect();
                assert_eq!(cells.len(), COMMON_WORSHIP_COLUMNS, "{line}");
                String::from(cells[3])
            }));
        }
        let year = holidays_in_year("common-worship", None, None, None, 2025).expect("table");
        let mut placed: alloc::vec::Vec<String> = year
            .lines()
            .map(|line| line.split('\t').collect::<alloc::vec::Vec<_>>())
            .filter(|cells| !cells[0].is_empty())
            .map(|cells| String::from(cells[9]))
            .collect();
        kept.sort();
        placed.sort();
        assert_eq!(kept, placed);
        assert!(placed.len() >= 40, "{}", placed.len());
    }

    /// A year's line ends with the holiday's identifier and the instrument
    /// its rule cites, and a kind filter keeps the entries of those kinds
    /// and the gaps of the rules of those kinds.
    #[test]
    fn a_year_s_lines_carry_an_identifier_and_a_source_and_are_filtered_by_kind() {
        let kinds = |text: &str| -> alloc::collections::BTreeSet<String> {
            text.lines()
                .map(|line| String::from(line.split('\t').nth(3).unwrap_or("")))
                .collect()
        };
        let all = holidays_in_year("US", None, None, None, 2026).expect("US");
        for line in all.lines() {
            assert_eq!(line.split('\t').count(), 11, "{line}");
        }
        assert!(all.lines().any(|line| line.contains("\tnew-years-day\t")));
        // A state with observances: the days of its own kinds go and the
        // public ones stay.
        let table = rule_set("US").expect("US");
        let state = table
            .regions()
            .into_iter()
            .find(|region| {
                let text =
                    holidays_in_year("US", Some(region), None, None, 2026).unwrap_or_default();
                kinds(&text).contains("observance")
            })
            .expect("a state with observances");
        let every = holidays_in_year("US", Some(state), None, None, 2026).expect("US");
        let public = holidays_in_year("US", Some(state), None, Some("public"), 2026).expect("US");
        assert!(public.lines().count() < every.lines().count());
        assert!(
            kinds(&public)
                .iter()
                .all(|kind| kind == "public" || kind == "gap"),
            "{:?}",
            kinds(&public)
        );
        assert!(
            public
                .lines()
                .any(|line| line.contains("\tnew-years-day\t"))
        );
        let observances =
            holidays_in_year("US", Some(state), None, Some("observance"), 2026).expect("US");
        assert!(
            kinds(&observances)
                .iter()
                .all(|k| k == "observance" || k == "gap")
        );
        assert!(observances.lines().count() + public.lines().count() <= every.lines().count() + 4);
        // A list, in any case, and white space around each word.
        let both =
            holidays_in_year("US", Some(state), None, Some(" PUBLIC ; Bank"), 2026).expect("US");
        assert!(
            kinds(&both)
                .iter()
                .all(|k| ["public", "bank", "gap"].contains(&k.as_str()))
        );
        // An empty filter is every kind; a word that is no kind is refused.
        assert_eq!(
            holidays_in_year("US", None, None, Some(""), 2026).as_deref(),
            Ok(all.as_str())
        );
        assert_eq!(
            holidays_in_year("US", None, None, Some("public;festival"), 2026),
            Err(Refusal::Unknown)
        );
        // A subdivision not read is a gap whatever the kind asked for.
        let hampshire =
            holidays_in_year("US", Some("US-NH"), None, Some("observance"), 2026).expect("US");
        assert!(
            hampshire
                .lines()
                .any(|line| line.ends_with("\tunread-subdivision\t")),
            "{hampshire}"
        );
        // An entry cites the instrument its rule does: the United Nations'
        // days cite their resolutions.
        let un = holidays_in_year("un-days", None, None, None, 2026).expect("table");
        assert!(un.lines().any(|line| {
            line.split('\t')
                .nth(10)
                .is_some_and(|s| s.contains("A/RES"))
        }));
        let un_public =
            holidays_in_year("un-days", None, None, Some("public"), 2026).expect("table");
        assert_eq!(un_public, "");
    }

    /// Column 12 of `hc_holiday_tables` is the pairs of a subdivision and a
    /// group a rule is scoped to both of, and column 13 the subdivisions
    /// the sources were read for.
    #[test]
    fn the_tables_say_their_region_groups_and_the_subdivisions_read() {
        let text = holiday_tables("en");
        for line in text.lines() {
            let cells: alloc::vec::Vec<&str> = line.split('\t').collect();
            assert_eq!(cells.len(), HOLIDAY_TABLES_COLUMNS, "{line}");
            let table = rule_set(cells[0]).expect("a table");
            let expected: alloc::vec::Vec<String> = table
                .region_groups()
                .iter()
                .map(|(region, group)| alloc::format!("{region}:{}", group.id))
                .collect();
            assert_eq!(cells[11], expected.join(";"), "{}", cells[0]);
            assert_eq!(
                cells[12],
                table.read_subdivisions().join(";"),
                "{}",
                cells[0]
            );
            // What the rules are scoped to is read.
            for region in cells[8].split(';').filter(|code| !code.is_empty()) {
                assert!(cells[12].split(';').any(|read| read == region), "{region}");
            }
        }
        let row = |code: &str| -> alloc::vec::Vec<&str> {
            text.lines()
                .map(|line| line.split('\t').collect::<alloc::vec::Vec<_>>())
                .find(|cells| cells[0] == code)
                .expect("a row")
        };
        // The designated cities are read, as the municipalities of their
        // prefectures; an exchange has no subdivisions.
        assert!(row("JP")[12].split(';').any(|code| code == "JP-14-130"));
        assert_eq!(row("XNYS")[11], "");
        assert_eq!(row("XNYS")[12], "");
    }
}
