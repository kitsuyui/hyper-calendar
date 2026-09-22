# Architecture

## The problem

A library that handles `n` calendars and converts between them naively needs
`n²` conversions. A library that also handles multiple time scales, uncertain
dates, non-terrestrial clocks and relativistic corrections multiplies that
again. The whole design here is about refusing that multiplication.

## Two pivots

Everything in `hyper-calendar` routes through exactly two canonical
representations.

### The day pivot: Rata Die

A **calendar** answers one question: *which day is this, and what is it
called?* Days can be counted, so every calendar converts to and from a single
integer day number — the Rata Die, `Rd`, with day 1 being `0001-01-01` in the
proleptic Gregorian calendar.

```text
Gregorian fields ──┐                     ┌── Hijri fields
Hebrew fields ─────┤                     ├── Japanese era fields
Chinese fields ────┼──▶  Rd (i64 day)  ──┼── Maya long count
Persian fields ────┤                     ├── ISO week date
Holocene fields ───┘                     └── Julian Day Number
```

A calendar author writes `to_fixed` and `from_fixed`. Conversion between any
two calendars, the registry, the formatter and the holiday engine all follow
from those two functions without any calendar knowing another exists. This is
the design from Reingold and Dershowitz, *Calendrical Calculations*, and it is
what makes "support every calendar" a linear amount of work instead of a
quadratic one.

`Rd` says nothing about time of day, time zone or time scale. That is the
point: a *day* is the largest unit every calendar agrees on, and mixing in
sub-day concerns is what makes date libraries collapse.

### The instant pivot: TAI

A **time scale** answers a different question: *what does a clock read at this
event?* All uniform scales — TT, TCG, TDB, TCB, GPS — are functions of TAI, so
`Instant<S>` carries a zero-sized scale marker and converts through TAI.

```text
Instant<Tt> ──┐                      ┌── Instant<Tcg>
Instant<Gps> ─┼──▶ Instant<Tai> ─────┼── Instant<Tdb>
Instant<Ut1> ─┘                      └── Instant<Tcb>
                     │
                     ▼  (leap-second table)
               UtcInstant ──▶ UnixTime
```

UTC is deliberately *not* one of those markers. UTC's seconds are SI seconds
but its labelling occasionally repeats one, so it cannot be a constant offset
from TAI. It goes through the leap-second table instead, and `UtcInstant`
carries an explicit `leap_second` flag so that `23:59:60` is nameable rather
than being something every caller has to route around.

### Where the two meet

They meet only where the caller asks them to, at a time zone. Turning a
`CivilDateTime` (a day plus a wall-clock time) into an `Instant<Tai>` requires
a zone *and* a leap-second policy, and both are explicit arguments. Nothing in
the library silently assumes "local time" or "now".

## Crate graph

```text
                          hc-core
                 (Duration, Instant<S>, scales,
                  leap table, epochs, math)
                             │
        ┌────────────────────┼─────────────────┬──────────────┐
        │                    │                 │              │
  hc-uncertainty        hc-calendar           hc-units
  (sig figs, fuzzy,   (Rd, CivilTime,
   EDTF, intervals)    Calendar trait,
        │               registry)
        │                    │
        ├──────────┐         ├──────────────┬────────────┬──────────┐
        │          │         │              │            │          │
 hc-deep-time  hc-relativity │       hc-calendars-solar  hc-tz   hc-i18n
 (Planck →     (Lorentz,     │       (Gregorian, Julian,  (offsets, (locales,
  cosmology)    Schwarzschild,│        ISO, Coptic, …)    POSIX TZ,  plurals,
                worldlines)   │              │            TZif)     names)
                              │              │              │          │
                         hc-astro            │              │          │
                    (ΔT, solar longitude,    │              │     hc-humanize
                     new moon, rise/set)     │              │
                          │    │             │              │
                          │    └─────────────┼──────────────┴──▶ hc-format
                          │                  │                   (ISO 8601,
                   hc-calendars-lunar        │                    RFC 3339)
                   (Hijri, Hebrew,           │
                    Chinese, Tenpō)          │
                          │                  │
                   hc-calendars-regional ◀───┘
                   (Japanese eras, Maya,
                    Pawukon)
                          │
                    hc-seasons ──▶ hc-holiday
                 (24 terms, 72 pentads)  (rule engine + country data)

                   hc-planetary  (Mars sols, MTC, Darian)

                          hyper-calendar  (feature-gated facade)
                            ├── hyper-calendar-ffi   (cdylib / staticlib, C ABI)
                            └── hyper-calendar-wasm  (cdylib, WebAssembly)
```

Four crates are left out of the drawing to keep it legible: `hc-units`
(exact ratios, tempo and media rates; on `hc-core` alone), `hc-attributes`
(birthstones and the like; on `hc-calendar` and `hc-seasons`), `hc-almanac`
(暦注; on `hc-seasons`, `hc-astro` and `hc-calendars-lunar`) and `hc-fiscal`
(fiscal and academic years; on `hc-calendars-solar`). The workspace manifest
is the list of record.

The graph is a DAG. No crate depends on a crate it does not need, and nothing
depends on the facade.

## Why this many crates

Requirement 8 of the brief: a caller should be able to compile in only what
they use. Splitting at the crate boundary rather than the module boundary means
Cargo can genuinely drop the code, the `cargo audit` surface shrinks, and a
WebAssembly build that only formats Gregorian dates does not carry a lunar
ephemeris.

The `hyper-calendar` facade turns each crate into a feature, with bundles that
match how people actually ask for things (`civil`, `lunar`, `seasons`,
`holiday`, `deep-time`, `planetary`, `relativity`, `full`).

## The two calendar interfaces

`Calendar` is statically typed: each calendar has its own `Date`, so a Hebrew
date cannot be passed where a Gregorian one is expected. `DynCalendar` is
object-safe and speaks `DateFields`, so a registry can hold calendars chosen at
run time and an FFI caller can name one by string.

`DynAdapter<C>` derives the second from the first. A calendar author implements
`Calendar` once; the dynamic interface, the registry entry and the default
`days_in_month`/`days_in_year`/`is_leap_year` all follow.

`DateFields` is a fixed-capacity bag — era, year, month (with a leap flag),
day, and up to eight named extras. The leap flag is not decoration: the Chinese
leap fourth month is "閏四月", not "month 13", and a type that cannot say that
forces every lunisolar calendar to invent a private encoding. The extras slot
is what lets the Maya long count and the Balinese Pawukon use the same type as
everything else.

## Where the hard parts live

| Concern | Crate | Note |
| --- | --- | --- |
| Exactness | `hc-core` | `i128` seconds + `u64` attoseconds; no float in the civil path |
| Leap seconds | `hc-core::leap` | A table. Replaceable without touching conversion code |
| Calendar reform | `hc-calendars-solar` | Parameterised by adoption date, with a national table |
| Lunisolar rules | `hc-astro` + `hc-calendars-lunar` | Astronomy is a separate crate so the arithmetic calendars do not pay for it |
| Equinox rules | `hc-astro` + `hc-calendars-equinox` | Solar calendars judged by a clock at a place — Solar Hijri, Badíʿ, French Republican — beside their arithmetic siblings |
| Indian reckonings | `hc-astro` + `hc-seasons` + `hc-calendars-indic` | The tithi at sunrise and the month named by its saṅkrānti, downstream of the sidereal zodiac |
| Uncertainty | `hc-uncertainty` | EDTF, Allen interval relations, significant figures |
| Scale beyond seconds | `hc-deep-time` | Logarithmic magnitudes for Planck time and cosmology |
| Off-Earth clocks | `hc-planetary` | Mars sols, MSD, MTC, Darian |
| Relativity | `hc-relativity` | Worldline integration, so an SF timeline is computable |
| Ambiguous local time | `hc-tz` | A three-way `LocalResolution`, never a silent pick |
| Localisation | `hc-i18n` | Static data plus a fallback chain |

## Further reading

- [policy.md](policy.md) — the standing rules this design serves
- [calendars.md](calendars.md) — the calendar coverage roadmap
- [observances.md](observances.md) — the holiday and religious-day roadmap
- [time-scales.md](time-scales.md) — what each scale is and how they relate
- [adr/](adr/) — the decisions that could reasonably have gone the other way
