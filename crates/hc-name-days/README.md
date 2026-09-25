# hc-name-days

Name-day lists: each list a named edition of a named authority, with a
loader for the lists this crate may not ship.

**This crate reports what lists say. It asserts none of them.** A name day
is not a fact about a date. It is a fact about somebody's list, the list is
always named, and so is the edition. The system — what a name day is in the
countries that keep one, how the crate models it, how the data was checked,
and every source by key — is in
[`docs/systems/name-days.md`](../../docs/systems/name-days.md); this README
summarises it.

## There is no such thing as "the" name day for a name

There is the Latvian traditional list the Valsts valodas centrs's commission
decided on 16 March 2022, in force 2023–2025, and the one it decided on 30
April 2025, in force from 1 January 2026, which added Grēta to 23 January.
Both are real, and a caller asking about 2024 should get the one that was in
force in 2024. So every edition is its own `const`, every edition refuses the
years it does not cover, and the year picks the edition:

```rust
use hc_name_days::latvia::{LV_TRADITIONAL_2023, LV_TRADITIONAL_2026};
use hc_name_days::{MonthDay, NameDayError, Validity, days_of, in_force, names_on};

assert_eq!(
    names_on(&LV_TRADITIONAL_2026, 2026, 1, 1)?,
    ["Laimnesis", "Solvita", "Solvija"]
);
assert!(names_on(&LV_TRADITIONAL_2026, 2026, 1, 23)?.contains(&"Grēta"));
assert!(!names_on(&LV_TRADITIONAL_2023, 2025, 1, 23)?.contains(&"Grēta"));

// An edition does not answer for a year it does not cover.
assert_eq!(
    names_on(&LV_TRADITIONAL_2026, 2025, 1, 23),
    Err(NameDayError::OutsideValidity { year: 2025, validity: Validity::since(2026) })
);

// Both Latvian lists are in force at once; nothing picks one for you.
assert_eq!(in_force("lv", 2024).count(), 2);

// A name finds its days; the match is exact, in the list's own spelling.
assert!(days_of(&LV_TRADITIONAL_2026, "Jānis", 2026)?.eq([MonthDay::new(6, 24)]));
# Ok::<(), NameDayError>(())
```

This is [`docs/policy.md`](../../docs/policy.md) §5 and §10 — competing
conventions get names, and an authority's revisions are data, not
corrections — and [ADR 0006](../../docs/adr/0006-refuse-to-extrapolate.md):
a list revised for 2026 is not extrapolated back to 2025.

## The model

A `NameDayList` carries:

| Field | What it says |
|---|---|
| `id`, `country`, `language`, `english_name` | `lv-traditional-2026`, ISO 3166-1, BCP 47, a printable name |
| `authority`, `decided`, `provenance` | whose list, when they decided this edition, and how it came to exist |
| `validity` | the years the edition is or was in force |
| `licence` | the terms it may be copied under; only `PublicDomainDedication` and `OfficialWork` are ever vendored, and a test asserts it |
| `leap_day` | one of four `LeapDayRule`s, below |
| `days` | 366 slots, one per day of a leap year, each the authority's names in the authority's order |
| `unlisted_names_day`, `notes` | the day reserved for names not listed, and whatever the source prints that is not a name |
| `source`, `retrieved` | the citation with its URL and licence, and when the file was read |

Two functions read every list: `names_on(list, year, month, day)` and
`days_of(list, name, year)`. They read the vendored tables and a loaded list
alike, through the `NameDays` trait, and both refuse a year the edition does
not cover.

## The leap day is an enum

Four conventions, one variant each ([ADR 0007](../../docs/adr/0007-sets-the-world-can-extend-are-data.md):
a fifth would be this crate's modelling decision, not a discovery):

| `LeapDayRule` | Convention | Where |
|---|---|---|
| `NoNames` | 29 February carries no names | Latvia (the source prints an en dash), Finland, Norway |
| `OwnNames` | 29 February carries names of its own, kept in leap years | Czechia (Horymír), Estonia |
| `ShiftAfter24February` | 24 February is the leap day: in a leap year it is empty and the names of 24–28 February move to 25–29 | Hungary, Denmark |
| `LeapYearsOnly` | 29 February carries a name the authority says is kept only in leap years | France (Auguste Chapdelaine) |

An empty slot is a statement, and a test asserts that every vendored list's
empty slots are exactly the ones its rule names.

## Coverage

| Identifier | List | Names | In force | Licence |
|---|---|---|---|---|
| `lv-traditional-2023` | Latvian traditional, 2022 revision | 1,023 | 2023–2025 | CC0-1.0 |
| `lv-traditional-2026` | Latvian traditional, 2025 revision | 1,032 | from 2026 | CC0-1.0 |
| `lv-extended-2023` | Latvian extended, 2022 revision | 5,589 | 2023–2025 | CC0-1.0 |
| `lv-extended-2026` | Latvian extended, 2025 revision | 5,652 | from 2026 | CC0-1.0 |

All four are the Valsts valodas centrs's dataset *Latviešu tradicionālais un
paplašinātais kalendārvārdu saraksts*,
<https://data.gov.lv/dati/eng/dataset/latviesu-tradicionalais-un-paplasinatais-kalendarvardu-saraksts>,
licence CC0-1.0, retrieved 2026-09-25 — the current files for the 2026
editions, and the files published 2023-04-26 as the Wayback Machine captured
them on 12–13 August 2024 for the 2023 editions. CC0 asks for no attribution;
this project's policy does, so the centre is named in every `source`.

The source's own statements are kept as it makes them: `29.02.,–` is an
empty slot; `22.05.,Emīlija. Visu neparasto un kalendāros neierakstīto vārdu
diena` is the name Emīlija, a note with the sentence, and
`unlisted_names_day` set to 22 May; `Mora (LTG: Muora)` and five more in the
2026 extended list are the Latgalian form carried as a name with a note
saying whose form it is.

## Lists this crate may not ship are loaded, not vendored

The University of Helsinki holds the copyright to its four name-day lists,
confirmed by the Supreme Court in 2000, and states the limit of free use:

> Name day information can be published freely if no more than two weeks of
> name day information or more than 15 names from the alphabetical list are
> published at a time.
>
> — University of Helsinki Almanac Office, *Copyright to name days*,
> <https://almanakka.helsinki.fi/en/name-days/copyright-to-name-days/>

A library carrying the whole year is exactly the case the Office charges
for, so the Finnish lists are not here, and neither is Norway's
(Almanakkforlaget's, free for editorial use with credit and otherwise on its
terms) nor Sweden's (no terms stated by anyone). What is here is a loader,
`hc_name_days::load`, behind the `alloc` feature: a text format the
evaluator reads exactly as it reads a vendored table, so that a caller who
holds a licence can use the list without this crate distributing it.

```text
# key = value; blank lines and # comments are ignored
id = fi-fi-2025
validity = 2025..2029
leap-day = no-names
source = the licence and the file it covers
01-01 =
01-02 = Aapeli
…
02-29 =
…
12-31 = Sylvester; Silvo
```

Every one of the 366 days must appear once, an empty value is a day with no
names, `;` separates names, and a list that contradicts its own leap-day
rule is refused. `NameDayList::to_text` writes a vendored list in the same
format, which is how the tests prove the round trip on every Latvian slot.
This crate distributes no licensed list and no fixture of one.

## Not carried

Seventeen countries, each a `gaps::Gap` with its reason as a value. Present
tense: this is what the sources say as read on 2026-09-25.

| Country | Reason | Because |
|---|---|---|
| Finland | `LicensedForAFee` | The University of Helsinki's copyright, KKO 2000:56; free use stops at two weeks or 15 names. Load it |
| Norway | `LicensedForAFee` | Almanakkforlaget's copyright; editorial use free with credit. Load it |
| Sweden | `LicenceUnknown` | The Namnlängdskommittén states no terms; not official since 1972; catalogue protection can apply |
| Slovakia | `LicenceUnknown` | The Ministry of Culture states no terms; official-work status under Zákon č. 185/2015 Z. z. § 5 unchecked |
| Croatia | `LicenceUnknown` | Only Bosnia and Herzegovina's episcopal list is compiled; no terms stated |
| France | `LicenceUnknown` | A publishers' compilation; Nominis's terms not retrieved |
| Czechia | `SourcesDisagreeWithNoAuthority` | "No official calendar exists"; the reference list is a book and not binding |
| Poland | `SourcesDisagreeWithNoAuthority` | Publishers differ; nothing regulates the dates |
| Denmark | `SourcesDisagreeWithNoAuthority` | The old almanac sanctorale, unsourced; no keeper |
| Lithuania | `SourcesDisagreeWithNoAuthority` | Church calendars, Catholic and Orthodox differing; no keeper found |
| Hungary | `MethodUnpublished` | No central body; the printed list rests on a copyrighted book |
| Estonia | `MethodUnpublished` | Statistics Estonia cites a commercial book; the university claim is unconfirmed |
| Greece | `SaintsNotNames` | The church names saints; the compilations are private and copyrighted |
| Bulgaria | `SaintsNotNames` | The church's calendar names saints and permits citation; no name list |
| Russia | `SaintsNotNames` | The saint nearest after the birthday: a rule on the caller's birthday, not a list |
| Germany and Austria | `SaintsNotNames` | The Regionalkalender is a liturgical calendar |
| Spain | `SaintsNotNames` | The santoral is the liturgical calendar |

## The movable Orthodox name days are not yet carried

Greek and Bulgarian name days partly move with Pascha on the Julian
computus: Thomas Sunday (Pascha + 7) for Θωμάς and Θωμαΐς; All Saints
(Pascha + 56) for names with no saint of their own; St George on 23 April,
moved to Easter Monday when 23 April falls before Pascha; Bulgaria's Цветница
on Palm Sunday, Великден, Тодоровден on the Saturday of the first week of
Lent, Спасовден on Ascension. That arithmetic is `hc_holiday::computus`'s and
is not duplicated here; the rules are recorded in `gaps::GREECE` and
`gaps::BULGARIA` so that nothing is lost until they are carried.

## What this crate deliberately does not do

- **Pick a list.** `in_force` returns every list in force in a year.
- **Extrapolate an edition.** Outside its validity, an edition is an error.
- **Invent an authority.** Where nobody publishes the list, the country is
  a gap.
- **Translate or transliterate.** Names are the list's spelling in the
  list's script; a lookup is exact; diminutives are separate entries where
  the authority prints them and absent where it does not. No `hc-i18n`.
- **Ship what it may not.** A `Licence` that does not permit
  redistribution never reaches a `const`.

## Accuracy

The tables are transcriptions, generated from the centre's files by a script
and checked against the centre's announcements: the nine names of 2025 on
their announced days and absent from the 2023 edition, the ten of 2022 and
the eight of 2014 in both, 22 May Emīlija in every edition, 29 February the
only empty day, the traditional list contained in the extended one day by
day, and the totals as counted from the files. One discrepancy: the 2022
announcement as read gives Dafne 18 February and both files give 8 February;
the file is the record.

## Features

The tables and the evaluator are `&'static` data and integer arithmetic and
need neither `std` nor `alloc`. The loader needs an allocator and is behind
`alloc`. `hc-core` refuses to compile with neither `std` nor a
floating-point backend, so a `no_std` build enables `libm`:

```sh
cargo build -p hc-name-days --no-default-features --features alloc,libm
```
