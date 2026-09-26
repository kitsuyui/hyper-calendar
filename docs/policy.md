# Project policy

This document is the standing decision record for how `hyper-calendar` is built.
It exists so that a contributor — human or agent — can tell whether a change
belongs here without asking.

## 1. English is the repository language

All identifiers, comments, documentation, commit messages, issue text, test
names and error messages are in English.

This is not a claim that English is the important language. It is the opposite:
the *subject* of this library is that time is written differently everywhere, so
the library's own vocabulary has to stay out of the way. A single working
language keeps the code reviewable by anyone and keeps localisation where it
belongs — in data, behind `hc-i18n`, never hard-coded into logic.

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

The converse rule matters as much. When two crates need the same arithmetic,
it goes in the one whose definition it *is*, and the others adapt the shape.
Proleptic Gregorian conversion lives in `hc-calendar` because "day 1 is
0001-01-01" is what fixes the origin of `Rd`, not a fact about the Gregorian
calendar — the eras, validation and `Calendar` implementation stay in
`hc-calendars-solar`.

This is not tidiness. A second implementation is a second thing to be
wrong, and the measurable case is next door: `hc-almanac` reads
`hc-seasons`' simplified lunisolar derivation, which differs from the full
calculation on 89 of the 3,653 days of 2024–2033.

Where a shape change is genuinely wanted — bare integers instead of `Rd` and
`Result`, because the call site is a `const` table of published dates — keep
the adapter and make it a thin one, with a test that asserts it changes the
shape and nothing else.

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
  interpolates within a supplied series and refuses to extrapolate outside it;
  UT1 without one is the ΔT model, a separate type whose error is stated.
- **Historical proclamations.** Many calendars were, in practice, whatever an
  authority announced. Computed Hijri dates, pre-modern Chinese dates and
  pre-reform Julian dates can disagree with what was actually observed or
  decreed. Every such calendar is flagged `is_astronomical` or carries a
  documented range, and the README says what it can and cannot be trusted for.
- **Local time that does not exist.** When a DST transition skips an hour,
  `hc-tz` returns `Nonexistent` rather than silently shifting.

## 5. Competing conventions get names, not parameters

Where authorities genuinely disagree about what a calendar does, the library
registers **each convention as its own named calendar** rather than taking a
parameter, picking a default, or refusing to answer.

This is already how the Julian-to-Gregorian reform works: `julian-gregorian-gb`
and `julian-gregorian-ru` are separate calendars, because a date written in
Britain in 1700 and the same day written in Russia belong to different
calendars, not to one calendar with a setting. The rule generalises.

It applies to:

- The Japanese courts. Between 1331 and 1392 two courts proclaimed eras at the
  same time, so `japanese-northern` and `japanese-southern` are separate
  calendars. `japanese` is the unified stream and declines the years of the
  schism — not as a refusal to choose, but because outside those years there
  really is only one stream, and inside them the two named calendars are the
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

A **name** does both jobs. `japanese-southern` cannot be selected by accident,
it appears in a registry listing so the choice is discoverable, it is
self-documenting at the call site, and asking for both and comparing them is
one loop.

The cost is more identifiers. That is the right cost: the identifiers exist
because the disagreements exist, and hiding them behind one name does not make
a calendar less contested.

### Where a parameter is still right

When the variation is *continuous* or *open-ended* rather than a short list of
named conventions — an observation meridian, a location, a published
uncertainty series — a parameter is correct, because there is no finite set of
names to enumerate.

### Where the list itself is open

A short list of named conventions is still not an `enum` when the world can
lengthen it without asking: calendars, countries, the readings of a cycle,
units of time. Those are tables of data, and the guarantees an `enum` would
have given are asserted by tests instead.
[ADR 0007](adr/0007-sets-the-world-can-extend-are-data.md) gives the test —
discovery or decision — and what it costs.

## 6. Modularity is a compile-time property

Requirement 8 of the original brief: not everything should be compiled into
everything. The workspace is split so that a caller who wants Gregorian dates
and nothing else pays for Gregorian dates and nothing else.

- Each capability is its own crate.
- The `hyper-calendar` facade exposes each as an optional feature.
- `default` is deliberately modest (`std`, `civil`, `format`, `i18n`).
- The facade, and every `hc-*` crate on its own, builds under
  `--no-default-features --features alloc,libm`. Each crate's `libm` feature
  passes through to `hc-core`, which refuses to compile with neither `std`
  nor `libm`.
- No crate depends on another unless it genuinely needs it. The dependency
  graph is a DAG and is documented in [architecture.md](architecture.md).

## 7. Correctness is demonstrated, not asserted

- Every conversion is round-trip tested over its whole supported range.
- Every algorithm is anchored to at least one independently published
  reference value, cited in a comment.
- Boundaries are tested explicitly: epochs, calendar reforms, leap days, leap
  months, leap seconds, the first and last supported day.
- Error paths are tested. A function that can return `MonthOutOfRange` has a
  test that makes it do so.
- CI runs `cargo test`, `cargo clippy -D warnings`, `cargo fmt --check`,
  `cargo doc` with warnings denied, `no_std` builds, a WebAssembly build, a
  shared-library build and `cargo audit` on every pull request. Coverage is reported by octocov with a
  70% floor.

## 8. `unwrap` and `expect` are forbidden outside tests

`.cargo/config.toml` sets `-Dclippy::unwrap_used` and
`-Dclippy::expect_used`. A library that panics is a library that cannot be
embedded in a WebAssembly runtime or behind an FFI boundary. Fallible
operations return `Result`; infallible ones are proved infallible by
construction.

The arithmetic operators are the deliberate exception: on `Duration`, `Add`,
`Sub`, `Neg`, `Mul<i64>`, `Div<i64>` and `Rem`, and the `AddAssign` and
`SubAssign` built on them; the `Add` and `Sub` operators on `Rd` and
`Rd::days_since`; and in the facade's `civil` layer, the same operators on
`TimeDelta`, `TimeDelta::abs`, and `Date` and `DateTime` plus or minus a
`TimeDelta`. They panic on overflow — and `/` and `%` on a zero divisor, as
integer division does — in release builds too, so that ordinary arithmetic
reads normally, and as Python's `datetime` reads, and each has a `checked_*`
twin.

## 9. No dependencies without a reason

The workspace has exactly one optional external dependency: `libm`, for
floating-point math on `no_std` targets that lack it. Everything else —
parsing, formatting, locale data, astronomy, the TZif reader — is implemented
here.

This is a cost, and it is paid deliberately. A date library is a dependency of
everything else; it should not drag a tree behind it. It also keeps the
WebAssembly artefact small and the `cargo audit` surface near zero.

The same care applies to code that would never appear in `Cargo.toml`. The
algorithms here are written from published rules and the reference dates
that accompany them — *Calendrical Calculations* above all — and anchored to
those dates with tests of their own. A rule is a procedure and free to
implement; a reference's own source code is a separate work under its own
licence, and it is read only where that licence allows it and named where it
was read, as the Babylonian module names the book's Apache-licensed
`calendar-code2`. This workspace is BSD-3-Clause, and nothing under a
licence that BSD-3-Clause cannot carry is copied into it.

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

The test is not "is this important" but "**could this repository be wrong about
the list, and would anyone be able to tell?**" An externally defined set can be
checked against its source, cited, dated, and corrected when the source
changes — which is what `sources_checked` already exists for. A set this
project curates cannot be checked against anything, because there is nothing
to check it against.

That is why "notable world events" is out of scope and a monarch's birthday is
not automatically out: where the birthday is a public holiday, a gazette
defines it and it belongs in `hc-holiday`; where it is not, no authority
defines the list of birthdays worth recording, so there is no list to be right
about.

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

A calendar, a holiday table, a solar-term convention or a reconstruction is
only ever *somebody's* statement of how the world counts days, and this
library carries the statement, not the world. So every rule, constant, table
and reference date that comes from a document names that document: author,
title, edition and year for a book or paper; the issuing body, number and
date for a statute, decree, gazette or exchange notice; the URL and the date
retrieved for anything read on the web, with an archive copy named where the
live page is gone or blocked. "As is well known" is not a source, and neither
is another library.

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

A source that was *not* read is named as not read: "which cites Hildebrand
1882, not read here" is honest; silently copying the citation is not. Where
the only source readable was secondary, the code says so and the roadmap row
says what primary source would replace it.

## 12. A complex system is written up before it is coded

Code states a rule exactly and explains it badly. A calendar with year
types, a reconstruction with competing readings, a holiday regime of annual
decrees and transferred days, a month scheme that depends on the sky at a
named place — for anything a maintainer cannot be expected to know already,
the explanation is written first, as a document under
[`systems/`](systems/README.md), from the sources, and the code then refers
to the document rather than carrying the explanation in comments alone.

The document says what the system is in the world, how it works, with a
worked example the reader can follow by hand, what this library carries of
it and what it deliberately does not, how well the implementation agrees
with the published reference and how that was measured, and where every
statement comes from. The module documentation summarises it in a paragraph
and names the document; the document names the module and the tests that
anchor it. Neither repeats the other.

The rule exists for the reader who arrives at a module without the context
its author had, which is every reader after the author, and for the reviewer
who has to judge a change to it. It also exists for the author: a rule that
cannot be written down in prose from its sources is not yet understood well
enough to be coded.

Which systems count as complex is a judgement, and the index in
[`systems/README.md`](systems/README.md) records the judgement: it lists the
systems that have a document, and the ones that should and do not yet.

## 13. Scope

`hyper-calendar` computes and formats. It has no UI, no I/O beyond optionally
reading a TZif file, no clock (the caller supplies the current time), no
network access and no global state. A web front end will eventually consume it
through the WebAssembly or C surface; that front end is not part of this
repository.
