# 0001 — Rata Die as the calendar pivot

**Status:** Accepted

## Context

The brief asks for every calendar we can know about, usable side by side. A
naive design gives each pair of calendars its own conversion, which is `n²`
work and `n²` places for a bug to hide. It also makes every calendar depend on
every other, which defeats the modularity requirement.

## Decision

Every calendar converts to and from a single integer day number — the Rata Die
`Rd`, with day 1 being `0001-01-01` proleptic Gregorian. A calendar implements
`to_fixed` and `from_fixed`; nothing else.

`Rd` carries no time of day, no zone and no scale.

## Consequences

**Good.** `2n` conversions instead of `n²`. Calendars are mutually independent,
so they can live in separate crates and be feature-gated. Cross-calendar
conversion, the registry, formatting and the holiday engine all fall out of the
two functions. Weekday is derivable from `Rd` alone, because the seven-day
cycle survived every calendar reform.

**Costs.** Calendars whose day does not start at midnight (the Hebrew and
Islamic day starts at sunset; the Chinese traditional day started at 23:00)
cannot express that in `Rd` alone. The library handles it by pairing `Rd` with
a `CivilTime` and declaring the convention per calendar through
`Calendar::day_boundary`, rather than by bending the pivot. That started as
prose and is now a type: `DayBoundary` also distinguishes the boundaries a
clock can resolve from the ones that need a location and an ephemeris.

A second cost surfaced later. `Rd` counts days, so a calendar that
occasionally *repeats* a day number — the Tibetan *lhag*, the Hindu *adhika
tithi*, the Balinese *ngunaratri* — produces two consecutive fixed days that
it names identically, and the round-trip contract above cannot hold for them.
`DateFields::leap_day` carries that distinction, in the same shape as the leap
flag on the month.

Calendars without a fixed day at all — a purely relative or cyclic reckoning —
do not fit and are represented through `DateFields::extra` instead.

## Alternatives rejected

- **Julian Day Number as the pivot.** Equivalent in power, but JD starts at
  noon, which introduces a half-day offset into every civil conversion. `Rd`
  starts at midnight. JDN remains available as a calendar and as a conversion
  on `Rd`.
- **Unix seconds as the pivot.** Conflates the day question with the instant
  question and drags leap seconds into calendar arithmetic that has no business
  knowing about them.
