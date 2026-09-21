# 0007 — Sets the world can extend are data; sets we define are enums

**Status:** Accepted

## Context

Most of this library is catalogues: calendars, holiday tables, units of
time, readings of the sexagenary cycle, month-name traditions, the bodies
that dilate time. The natural first draft of each was a Rust `enum` — a
`CalendarSystem` of eight variants, a `Script` of six, a `Body` of six, a
struct with one field per tradition. An `enum` is exhaustive, needs no
lookup, and cannot be asked for an identifier it does not have.

What it cannot do is admit a member it did not foresee. A vocabulary whose
month type assumed twelve or thirteen could not hold the nineteen-month
Badíʿ calendar. A holiday engine whose calendar set was closed could not
date an Ethiopian feast. A `Script` of six variants could not write
Vietnamese *can chi*. None of these was a missing entry; each was a shape
that refused entries, and no amount of care in filling the shape would have
changed that.

## Decision

Ask whether a new member arrives by **discovery** or by **decision**.

- If the world can add a member without asking us, the set is **data**: a
  struct, a `const` per entry, a table, a lookup by identifier. Calendars,
  countries, readings, units, traditions, gravitating bodies.
- If a member appears only when *we* change how we model the domain, the
  set is an **enum**: `Confidence`, `DayBoundary`, `CycleLength`,
  `Polarity`, `Court`.

Fixed cardinality is not the test. There are seven weekdays and `Weekday`
is rightly an `enum`; using it as the key for *every* cycle was the error.
A struct with one field per thing in the world — `{ tamil, bengali,
malayalam }` — is exactly as closed as an `enum` and gets no credit for
being a struct.

Where the members of a set are structural — a reading has exactly ten stems
and twelve branches, whatever language it is in — that structure stays in
the type: `&'static [&'static str; 10]`, not a slice.

## What opening a set costs, and how it is paid for

An `enum` gives three guarantees for free: every member exists, every
`match` is exhaustive, and an identifier cannot be mistyped. Data gives
none of them, so they are bought back with tests.

`hc_core::catalogue!` generates, for every table declared with it, the
ones every table needs — identifiers are unique, every entry is findable by
its own identifier, the table is in the order it claims, every entry names
its source — so that declaring an
entry, listing it and counting it stop being three separate edits. The
guarantees that are domain knowledge — 甲子 in each reading, a feast on
its date, an epact against a computus that shares no code with it — cannot
be generated and are written as anchors beside the data.

A bare string key with no table behind it is the worst of both: open and
unguarded. Every identifier that can be asked for is drawn from a table
that some test walks.

## Consequences

**Good.** Coverage becomes a data problem anyone can solve rather than a
design problem only a maintainer can: adding Vietnamese to the sexagenary
cycle is one entry and two anchors. The generated index
(`docs/supported.md`) can list what exists because what exists is a table.
A gap is something the library states rather than something a reader
discovers.

**Costs.** A `match` on a calendar system becomes a comparison on its
identifier, and a struct carrying function pointers cannot appear in a
pattern at all. Lookups are linear over small tables. Every catalogue needs
its anchors written, and a catalogue without them is only structurally
sound, not right.
