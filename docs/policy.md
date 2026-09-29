# Project policy

This document is the standing decision record for how `hyper-calendar` is
built. It exists so that a contributor, human or agent, can tell whether a
change belongs here without asking.

Each numbered section states one rule, and most then give the reasons for
it. Other documents cite the sections by number, such as §5 or §12.

| § | Rule |
| --- | --- |
| [1](#1-english-is-the-repository-language) | English is the repository language |
| [2](#2-data-is-separated-from-algorithm) | Data is separated from algorithm |
| [3](#3-precision-is-stated-never-implied) | Precision is stated, never implied |
| [4](#4-the-library-refuses-to-guess) | The library refuses to guess |
| [5](#5-competing-conventions-get-names-not-parameters) | Competing conventions get names, not parameters |
| [6](#6-modularity-is-a-compile-time-property) | Modularity is a compile-time property |
| [7](#7-correctness-is-demonstrated-not-asserted) | Correctness is demonstrated, not asserted |
| [8](#8-unwrap-and-expect-are-forbidden-outside-tests) | `unwrap` and `expect` are forbidden outside tests |
| [9](#9-no-dependencies-without-a-reason) | No dependencies without a reason |
| [10](#10-recurring-events-need-an-authority-not-an-opinion) | Recurring events need an authority, not an opinion |
| [11](#11-every-rule-from-the-literature-cites-it) | Every rule from the literature cites it |
| [12](#12-a-complex-system-is-written-up-before-it-is-coded) | A complex system is written up before it is coded |
| [13](#13-scope) | Scope |

## 1. English is the repository language

All identifiers, comments, documentation, commit messages, issue text, test
names and error messages are in English.

This is not a claim that English is the important language. It is the
opposite. The *subject* of this library is that time is written differently
everywhere, so the library's own vocabulary has to stay out of the way. A
single working language keeps the code reviewable by anyone. It also keeps
localisation where it belongs: in data, behind `hc-i18n`, never hard-coded
into logic.

Non-English text is welcome and expected **as data**: month names, era names,
holiday names, script samples and the test fixtures that verify them. A
Japanese era name belongs in a `&'static str` in a data table. It does not
belong in a function name.

## 2. Data is separated from algorithm

Every calendar, every holiday rule and every locale is expressed as data
interpreted by a small, shared algorithm. Concretely:

- Calendars implement one trait with two operations against a fixed day number
  (see [architecture.md](architecture.md)).
- Leap seconds are a table, not a formula.
- Holidays are rule values evaluated by one engine, not per-country code.
- Locale vocabulary is a static table with a fallback chain.

The test is simple: **adding Bolivia's holidays, or the Tibetan calendar, or
Welsh plural rules, should mean adding a data entry, not editing control flow.**
When a change forces a branch into shared logic, that is a signal the
abstraction is wrong, not that the new case is special.

### One implementation, in the crate that owns the idea

The converse rule matters as much. When two crates need the same
arithmetic, it goes in the crate whose definition it *is*, and the others
adapt the shape. For example, proleptic Gregorian conversion lives in
`hc-calendar`. There, "day 1 is 0001-01-01" is what fixes the origin of
`Rd`, not a fact about the Gregorian calendar. The Gregorian eras,
validation and `Calendar` implementation stay in `hc-calendars-solar`.

This is not tidiness. A second implementation is a second thing to be
wrong. A measured case: `hc-almanac` read a simplified lunisolar derivation
`hc-seasons` kept for 六曜, which differed from `hc-calendars-lunar`'s
calendar on 89 of the 3,653 days of 2024–2033, until it was replaced by the
calendar itself.

Sometimes a different shape is genuinely wanted. An example is bare
integers instead of `Rd` and `Result`, because the call site is a `const`
table of published dates. Then keep the adapter, keep it thin, write it
once beside the owner rather than once per caller (as
`hc_calendar::gregorian::to_fixed_saturating` is), and give it a test that
asserts it changes the shape and nothing else.

## 3. Precision is stated, never implied

Anything this library returns carries a claim about how well it is known.

- Where a value is exact, it is computed exactly. Durations are integers of
  attoseconds, not floats. Calendar conversions are integer arithmetic.
- Where a value comes from a fitted physical model — ΔT, TDB, solar longitude,
  a sunrise time — the crate's README and the function's doc comment state the
  accuracy and the era over which it holds.
- Where a value is genuinely unknown, `hc-uncertainty` says so rather than
  returning a number with false confidence.

A wrong answer delivered confidently is worse than a refusal.

## 4. The library refuses to guess

Some questions have no answer, and the API says so instead of inventing one:

- **Future leap seconds.** The IERS announces them about six months ahead. Past
  that, `LeapPolicy::Strict` returns `AfterModelEnd`. A caller who wants a
  forecast must ask for one.
- **UT1 before it was measured.** `DUT1` is observational. The library
  interpolates within a supplied series and refuses to extrapolate outside
  it. Without a series, UT1 comes from the ΔT model, a separate type whose
  error is stated.
- **Historical proclamations.** Many calendars were, in practice, whatever
  an authority announced. Computed Hijri dates, pre-modern Chinese dates and
  pre-reform Julian dates can disagree with what was actually observed or
  decreed. Every such calendar is flagged `is_astronomical` or carries a
  documented range. Its README says what it can and cannot be trusted for.
- **Local time that does not exist.** When a DST transition skips an hour,
  `hc-tz` returns `Nonexistent` rather than silently shifting.
- **A holiday before the sources read.** A holiday rule is absent only
  before the year a source says it was established, or after it was
  abolished. A year before the first its sources were read for, and a
  subdivision they were not read for, is a gap that the engine reports,
  not a year or a place without the day
  ([ADR 0013](adr/0013-a-year-the-sources-do-not-reach-is-a-gap.md)).

## 5. Competing conventions get names, not parameters

A *convention* here is one authority's way of doing something that another
authority does differently. Where authorities genuinely disagree about what
a calendar does, the library registers **each convention as its own named
calendar**. It does not take a parameter, pick a default, or refuse to
answer.

The Julian-to-Gregorian reform calendars follow this rule.
`julian-gregorian-gb` and `julian-gregorian-ru` are separate calendars. A
date written in Britain in 1700 and the same day written in Russia belong
to different calendars, not to one calendar with a setting.

The rule generalises. It applies to:

- The Japanese courts. Between 1331 and 1392 two courts proclaimed eras at
  the same time, so `japanese-northern` and `japanese-southern` are separate
  calendars. `japanese` is the unified stream and declines the years of the
  schism. This is not a refusal to choose. Outside those years there really
  is only one stream, and inside them the two named calendars are the
  answer.
- Correlation constants, where two are in published use.
- Leap-year schemes over the same structure and epoch.
- Intercalation schools, where practitioners' almanacs disagree.

### Why this beats the alternatives

A **parameter** can be forgotten. The caller who does not know the question
exists gets whatever the default is, silently, and the library has taken a
position on their behalf without saying so.

A **refusal** is honest but unhelpful. "Two courts proclaimed eras that year"
is true and leaves the caller with nothing to compute.

A **name** does both jobs. `japanese-southern`:

- cannot be selected by accident;
- appears in a registry listing, so the choice is discoverable;
- documents itself at the call site;
- can be compared with `japanese-northern` in one loop over both.

The cost is more identifiers. That is the right cost. The identifiers exist
because the disagreements exist, and hiding them behind one name does not
make a calendar less contested.

### Where a parameter is still right

A parameter is correct when the variation is *continuous* or *open-ended*
rather than a short list of named conventions. Examples are an observation
meridian, a location and a published uncertainty series. There is no finite
set of names to enumerate.

### Where a function is the name

Some conventions are not calendars. They are ways of computing one time
of day. Each is a function of its own, with its own documentation and test
anchor:

| Convention | Functions |
| --- | --- |
| The Jewish temporal hours by the GRA or the MGA | `zman_gra`, `zman_mga_72_minutes`, `zman_mga_16_1_degrees` |
| The Edo dawn by the 寛政暦's angle or the Observatory's | `japanese_dawn_kansei`, `japanese_dawn_naoj` |
| Rāhu kālam over the daylight or over a fixed day | `kalam::by_sunrise`, `kalam::by_fixed_day` |

A function cannot be chosen by accident either. It is as discoverable in
the documentation as an entry is in a registry.

Whether such a convention also has a string name depends on whether
anything looks it up by a string:

- **Nothing looks it up by a string**, as nothing looks up a Greenwich
  sidereal time: the IAU 2006 and the IAU 1982 conventions are two
  functions and two exports. The function's name is its only name. The roadmap writes "—" as its
  identifier and names the functions.
- **A caller selects it by a string at the boundary.** That string is its
  name, and it is registered as one: an entry of a table in the crate that
  owns the convention, with the string as its identifier and the function
  as its value. The boundary looks the string up in that table and keeps
  no list of its own. The roadmap row gives the identifiers. `hc-astro`'s
  `ZMANIM_RECKONINGS` holds `zmanim-gra`, `mga-72-minutes` and
  `mga-16-1-degrees`, and its `SolarEvent::ALL` the named times of day,
  the Edo dawn `japanese-dawn-kansei` among them. `hc-calendars-indic`'s
  `KalamConvention::ALL` holds `rahu-kalam-sunrise` and
  `rahu-kalam-fixed`. A prayer method, a six-hour reckoning, a meridian
  and a TAI64 format are table entries in the same way. The lists the
  documents give — the rustdoc of each export, the READMEs, the binding's
  `.d.ts` and the roadmap — are held to the tables by
  `crates/hyper-calendar/tests/convention_names.rs`.

### How a name is matched

Every identifier a caller gives — a calendar, a convention, a table, a
zone — is matched by one rule, `hc_core::catalogue::matches`: the white
space around it is ignored, and its ASCII letters match in either case,
so ` Gregory ` finds `gregory` and `us` finds `US`. The lookups
`catalogue!` generates follow it, and `catalogue_tests!` checks that a
hand-written one does too, by looking every entry up in upper case, in
lower case and padded. Two identifiers of one table therefore may not
differ only in case, and the same tests fail if they do. The rule is
ASCII only, as the identifiers are. A symbol whose case is its meaning —
the Planck time `t_P` beside the Planck temperature `T_P` — is not an
identifier, and its table says `matching: exact`.

### Where the list itself is open

A short list of named conventions is still not an `enum` when the world
can lengthen it without asking. Calendars, countries, the readings of a
cycle and units of time are such lists. They are tables of data, and tests
assert the guarantees an `enum` would have given.
[ADR 0007](adr/0007-sets-the-world-can-extend-are-data.md) gives the test —
discovery or decision — and what it costs.

## 6. Modularity is a compile-time property

Not everything should be compiled into everything. The workspace is split
so that a caller who wants Gregorian dates and nothing else pays for
Gregorian dates and nothing else.

- Each capability is its own crate.
- The `hyper-calendar` facade exposes each as an optional feature.
- `default` is deliberately modest (`std`, `civil`, `format`, `i18n`).
- The facade, and every `hc-*` crate on its own, builds under
  `--no-default-features --features alloc,libm`. Each crate's `libm` feature
  passes through to `hc-core`, which refuses to compile with neither `std`
  nor `libm`.
- Every `hc-*` crate on its own also builds without `alloc`, under
  `--no-default-features --features libm`: what needs an allocator is behind
  the crate's `alloc` feature. `scripts/no-std-builds.sh` builds each crate
  both ways for `aarch64-unknown-none`, a target with no `std`, and CI runs
  it.
- No crate depends on another unless it genuinely needs it. The dependency
  graph is a DAG and is documented in [architecture.md](architecture.md).

## 7. Correctness is demonstrated, not asserted

- Every conversion is round-trip tested over its whole supported range. CI
  runs the test suite in a release build and in a debug build.
  - The longest sweeps walk every day in a release build. In a debug build
    they walk a fixed, deterministic sample of the days, so that the
    instrumented coverage run stays inside its time limit.
  - Some sweeps also check every year's boundary, and the boundaries cost
    more than the sampled days: hundreds of years of astronomical or
    lunisolar conversions. A debug build samples those years too. It takes
    every k-th year and the last, with k prime to the cycle the years run
    in. The sample then still holds each kind of boundary the test is
    about: leap years, common years and the exceptions.
  - The published anchors are checked in full in both builds.
  - The tests that render every calendar in every carried locale render,
    in a debug build, a staggered share of the pairings of day and locale
    that still renders every calendar in every locale, and each calendar's
    own language and the days they must render in full; a release build
    renders every pairing. The page of every calendar's names,
    `crates/hyper-calendar/tests/distinct_names.rs`, is the exception: a
    debug build reads it in every fourth locale, and the next test there
    holds every locale's names to the same rule from `hc-i18n` directly.
  - Some calendars are another calendar renamed: the same days under a year
    shifted by a constant and another era, as the Arsacid era is the
    Babylonian calendar's. Such a calendar rests on the other's every-day
    sweep. Its own tests check the range at both ends, every year boundary,
    and the renaming on a sample of days. Walking every day again would
    repeat the other's computation and test nothing new.
- Every algorithm is anchored to at least one independently published
  reference value, cited in a comment.
- Boundaries are tested explicitly: epochs, calendar reforms, leap days, leap
  months, leap seconds, the first and last supported day.
- Error paths are tested. A function that can return `MonthOutOfRange` has a
  test that makes it do so.
- CI runs `cargo test`, `cargo clippy -D warnings`, `cargo fmt --check`,
  `cargo doc` with warnings denied, `no_std` builds, a WebAssembly build, a
  shared-library build and `cargo audit` on every pull request. Coverage is
  reported by octocov with a 70% floor.

## 8. `unwrap` and `expect` are forbidden outside tests

`.cargo/config.toml` sets `-Dclippy::unwrap_used` and
`-Dclippy::expect_used`. A library that panics is a library that cannot be
embedded in a WebAssembly runtime or behind an FFI boundary. Fallible
operations return `Result`; infallible ones are proved infallible by
construction.

The arithmetic operators are the deliberate exception:

- on `Duration`: `Add`, `Sub`, `Neg`, `Mul<i64>`, `Div<i64>` and `Rem`, and
  the `AddAssign` and `SubAssign` built on them;
- on `Rd`: the `Add` and `Sub` operators, and `Rd::days_since`;
- in the facade's `civil` layer: the same operators on `TimeDelta`,
  `TimeDelta::abs`, and `Date` and `DateTime` plus or minus a `TimeDelta`.

They panic on overflow, in release builds too. `/` and `%` also panic on a
zero divisor, as integer division does. The exception lets ordinary
arithmetic read normally, as it reads in Python's `datetime`. Each of these
operators has a `checked_*` twin.

## 9. No dependencies without a reason

The workspace has exactly one optional external dependency: `libm`, for
floating-point math on `no_std` targets that lack it. Everything else —
parsing, formatting, locale data, astronomy, the TZif reader — is implemented
here.

This is a cost, and it is paid deliberately. A date library is a dependency of
everything else; it should not drag a tree behind it. It also keeps the
WebAssembly artefact small and the `cargo audit` surface near zero.

The same care applies to code that would never appear in `Cargo.toml`:

- The algorithms here are written from published rules and the reference
  dates that accompany them, *Calendrical Calculations* above all. Tests of
  their own anchor them to those dates.
- A rule is a procedure, and free to implement. A reference's own source
  code is a separate work under its own licence. It is read only where that
  licence allows it, and named where it was read. For example, the
  Babylonian module names the book's Apache-licensed `calendar-code2`.
- This workspace is BSD-3-Clause. Nothing under a licence that BSD-3-Clause
  cannot carry is copied into it.

## 10. Recurring events need an authority, not an opinion

Periodic events — Olympiads, World Cups, election years, Jubilee years — are
legitimately calendrical: the question they answer is "which year in the cycle
is this" or "when is the next one". A general historical timeline is not.

The line between them is **not** this project's judgement about what matters.
It is whether **a disciplined external authority defines the set**.

| In scope, because someone else defines the set | Who defines it |
| --- | --- |
| The modern Olympic Games | The International Olympic Committee |
| The FIFA World Cup | FIFA |
| Leap seconds | The IERS |
| The geological time scale | The International Commission on Stratigraphy |
| Jubilee years | The Holy See |
| A national election cycle | That country's electoral law |
| A public holiday, including a monarch's official birthday | That country's statute or gazette |

The test is not "is this important" but "**could this repository be wrong
about the list, and would anyone be able to tell?**" An externally defined
set can be checked against its source, cited, dated, and corrected when the
source changes. Each holiday table's `sources_checked` date exists for this.
A set this project curates cannot be checked against anything, because
there is nothing to check it against.

That is why "notable world events" is out of scope, and why a monarch's
birthday is not automatically out:

- Where the birthday is a public holiday, a gazette defines it, and it
  belongs in `hc-holiday`.
- Where it is not, no authority defines the list of birthdays worth
  recording, so there is no list to be right about.

### The operational consequences

**A closure condition must be stateable.** "Every celebration of the modern
Games, plus the scheduled future ones" is a set that can be complete. "Notable
events" is not, so its coverage can never be stated honestly — and stating
coverage honestly is what §3 and §4 of this document are for.

**Exceptions must stay a minority.** A cycle with exceptions is still a cycle:
the Olympics have four (1916, 1940 and 1944 cancelled; 2020 held in 2021)
against roughly thirty-five celebrations. If the exceptions ever outnumber the
entries the rule produces, it has stopped being a rule and become a list, and
it leaves scope at that point. Record the ratio where it is close.

**An authority's own revisions are data, not corrections.** When the source
changes, the old version stays and the new one is added under its own name, as
§5 requires. The 1912 American birthstone list and the 2016 one are both real.

## 11. Every rule from the literature cites it

A calendar, a holiday table, a solar-term convention or a reconstruction
is only ever *somebody's* statement of how the world counts days. This
library carries the statement, not the world. So every rule, constant,
table and reference date that comes from a document names that document:

- for a book or paper: author, title, edition and year;
- for a statute, decree, gazette or exchange notice: the issuing body,
  number and date;
- for anything read on the web: the URL and the date retrieved, with an
  archive copy named where the live page is gone or blocked.

"As is well known" is not a source, and neither is another library.

Citations live in three places, and the same source is spelled the same way
in all of them:

- **In the code**, in the module documentation and in the `sources` string
  of every data table, so that a reader of the code sees where it came from
  without leaving it.
- **In the system document** under [`systems/`](systems/README.md), for
  anything complex enough to have one (§12).
- **In [`references.bib`](references.bib)**, one BibTeX entry per source
  cited more than once or cited from a system document, keyed as the
  documents cite it. BibTeX is the format because it is the one every
  reference manager reads and because a key like `parker1956` is shorter
  and more stable than a title.

A source that was *not* read is named as not read. "Which cites Hildebrand
1882, not read here" is honest; silently copying the citation is not. Where
the only readable source was secondary, the code says so, and the roadmap
row says what primary source would replace it.

## 12. A complex system is written up before it is coded

Code states a rule exactly and explains it badly. Some systems are more
than a maintainer can be expected to know already, for example:

- a calendar with year types;
- a reconstruction with competing readings;
- a holiday regime of annual decrees and transferred days;
- a month scheme that depends on the sky at a named place.

For such a system the explanation is written first, from the sources, as a
*system document* under [`systems/`](systems/README.md). The code then
refers to the document rather than carrying the explanation in comments
alone.

The document says, in six fixed sections:

- what the system is in the world;
- how it works, with a worked example the reader can follow by hand;
- what this library carries of it, and what it deliberately does not;
- how well the implementation agrees with the published reference, and how
  that was measured;
- where every statement comes from;
- which module implements it, and which tests anchor it.

The module documentation summarises the document in a paragraph and names
it. Neither repeats the other. [`systems/README.md`](systems/README.md)
gives the six section headings.

The rule exists for the reader who arrives at a module without the context
its author had, which is every reader after the author. It exists for the
reviewer who has to judge a change to the module. It also exists for the
author: a rule that cannot be written down in prose from its sources is not
yet understood well enough to be coded.

Which systems count as complex is a judgement. The index in
[`systems/README.md`](systems/README.md) records it: it lists the systems
that have a document, and the ones that should have one and do not yet.

## 13. Scope

`hyper-calendar` computes and formats. It has:

- no user interface;
- no I/O beyond optionally reading a TZif file;
- no clock: the caller supplies the current time;
- no network access;
- one piece of state that outlives a call. This is the table of zones a
  caller hands the WebAssembly module or the C library through
  `hc_zone_load`. The table keeps each zone by name for the life of the
  process, and the zone exports read that name's rules from it. It holds
  only what the caller loaded, and is kept once, in
  `hyper_calendar::zone_lines`, for both boundaries.

Within a call, `hc_core::memo` caches the results of pure functions in
thread-local storage. The storage is emptied when the call's scope ends. So
the cache changes how long a call takes, and never what it returns.

A web front end will eventually consume the library through the
WebAssembly or C surface. That front end is not part of this repository.
