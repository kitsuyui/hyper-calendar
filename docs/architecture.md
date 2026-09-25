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
`Instant<S>` carries a zero-sized scale marker and converts through TAI. UT1
is not uniform, but `hc-astro` gives it a marker too, modelled through ΔT;
it lives there rather than in `hc-core` because the ΔT model does.

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

Every crate depends on `hc-core`, and every crate from `hc-calendars-solar`
down also depends on `hc-calendar`. The table gives each crate's other direct
dependencies. A crate depends only on crates in rows above it.

| Crate | Holds | Also depends on |
| --- | --- | --- |
| `hc-core` | `Duration`, `Instant<S>`, time scales, the leap-second table, epochs, math | — |
| `hc-calendar` | `Rd`, `CivilTime`, the `Calendar` trait, the registry | — |
| `hc-uncertainty` | Significant figures, fuzzy dates, EDTF, intervals | — |
| `hc-units` | Exact ratios, tempo and media rates | — |
| `hc-deep-time` | Planck time to cosmology | `hc-uncertainty` |
| `hc-orbital` | Milankovitch orbital elements and insolation, Berger 1978 | `hc-uncertainty` |
| `hc-relativity` | Lorentz transforms, Schwarzschild, worldlines | `hc-uncertainty` |
| `hc-calendars-solar` | Gregorian, Julian, ISO, Coptic, … | — |
| `hc-tz` | Offsets, POSIX TZ, TZif | — |
| `hc-i18n` | Locales, plurals, names | — |
| `hc-astro` | ΔT, solar longitude, new moon, rise and set | — |
| `hc-humanize` | Relative times and spelled-out durations | `hc-i18n`, `hc-units` |
| `hc-format` | ISO 8601, RFC 3339, RFC 2822, patterns | `hc-calendars-solar`, `hc-tz`, `hc-i18n` |
| `hc-planetary` | Mars sols, MTC, Darian | `hc-astro` |
| `hc-calendars-lunar` | Hijri, Hebrew, Chinese, Tibetan, the Japanese lunisolar systems | `hc-astro` |
| `hc-calendars-equinox` | Solar Hijri, Badíʿ and French Republican by the equinox | `hc-astro`, `hc-calendars-solar` |
| `hc-seasons` | 24 terms, 72 pentads | `hc-astro`, optionally `hc-calendars-lunar` |
| `hc-calendars-regional` | Japanese eras, Maya, Aztec, Pawukon, Burmese, Thai lunar | `hc-calendars-solar`, `hc-calendars-lunar` |
| `hc-calendars-indic` | Hindu lunisolar and solar calendars, Bikram and Nepal Sambat | `hc-astro`, `hc-calendars-solar`, `hc-seasons` |
| `hc-almanac` | 暦注 | `hc-astro`, `hc-calendars-lunar`, `hc-seasons` |
| `hc-attributes` | Birthstones and the like | `hc-seasons` |
| `hc-name-days` | Name-day lists by authority and edition, and a loader for licensed ones | — |
| `hc-fiscal` | Fiscal and academic years | `hc-calendars-solar`, `hc-calendars-indic` |
| `hc-holiday` | The rule engine and the country, tradition and exchange tables | `hc-astro`, `hc-seasons` and every `hc-calendars-*` crate |
| `hyper-calendar` | The feature-gated facade | Every crate above, each behind a feature |
| `hyper-calendar-ffi` | `cdylib` and `staticlib`, C ABI | `hyper-calendar` |
| `hyper-calendar-wasm` | `cdylib`, WebAssembly | `hyper-calendar` |

The two boundary crates expose the same layers as Cargo features — `civil`
(the default), `calendars`, `holiday`, `seasons`, `deep-time`, `tz`, `sky`,
`orbital` and `full` — so a page or a host program builds only the layer it loads,
and each crate's README lists every export with the feature it needs. Their
answers about a set of things are tab-separated lines with a fixed column
order, and a calendar that cannot name a day is a line that says so, with
the stable code and name `CalendarError` gives every refusal. The
WebAssembly crate ships the JavaScript that reads those lines — a
dependency-free ES module under `crates/hyper-calendar-wasm/js`, tested
in CI against the built module — so a page decodes them once, by the
README's columns, and a method whose layer is not loaded refuses by name.

The workspace manifest is the list of record.

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
`days_in_month`/`days_in_year` all follow. `is_leap_year` does not: it is a
required method of `Calendar`, answered by the calendar's own rule and
forwarded unchanged, because "longer than the previous year" is not what leap
means for a calendar with several common-year lengths, and the adapter cannot
tell a leap year from a long common one.

`DateFields` is a fixed-capacity bag — era, year, month (with a leap flag),
day (with a leap flag of its own, for calendars that repeat a day), and up
to eight named extras. The leap flag is not decoration: the Chinese
leap fourth month is "閏四月", not "month 13", and a type that cannot say that
forces every lunisolar calendar to invent a private encoding. The extras slot
is what lets the Maya long count and the Balinese Pawukon use the same type as
everything else.

### Open questions in the dynamic interface

- `DateFields` holds eight extras and the Pawukon uses all eight. A single
  view of a long count with its calendar round needs nine, and the 819-day
  count a tenth; the Borana calendar needs a star-month, a day name and a
  phase beside its month and day. `set` reports the overflow at run time for
  a shape known when the calendar is written.
- `DateFields::era` is a `&'static str`, so an era vocabulary must be
  compiled in. The nengō table is; a caller supplying eras across the FFI or
  WebAssembly boundary cannot, and a calendar whose "year" is itself a name
  chosen afterwards has no representation.

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
| The ice-age cycles | `hc-orbital` | A trigonometric series answered over ±1 Myr and refused beyond |
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
