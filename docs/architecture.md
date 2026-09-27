# Architecture

## The problem

A library that converts between `n` calendars pairwise needs `n²`
conversions. Time scales, uncertain dates, clocks on other bodies and
relativistic corrections multiply that count again. The design avoids the
multiplication by routing everything through two canonical
representations.

## Two pivots

### The day pivot: Rata Die

A **calendar** answers one question: *which day is this, and what is it
called?* Days can be counted, so every calendar converts to and from one
integer day number: the Rata Die, `Rd`. Day 1 is `0001-01-01` in the
proleptic Gregorian calendar, which applies the Gregorian rules before
1582 as well.

```text
Gregorian fields ──┐                     ┌── Hijri fields
Hebrew fields ─────┤                     ├── Japanese era fields
Chinese fields ────┼──▶  Rd (i64 day)  ──┼── Maya long count
Persian fields ────┤                     ├── ISO week date
Holocene fields ───┘                     └── Julian Day Number
```

A calendar author writes `to_fixed` and `from_fixed`. Conversion between
any two calendars, the registry, the formatter and the holiday engine follow
from those two functions. No calendar knows another exists. This is the
design of Reingold and Dershowitz's *Calendrical Calculations*. It makes the
work linear in the number of calendars.

`Rd` says nothing about time of day, time zone or time scale. A *day* is the
largest unit every calendar agrees on. Time of day is carried beside it, as
`CivilTime`.

### The instant pivot: TAI

A **time scale** answers a different question: *what does a clock read at
this event?* A uniform scale is one whose seconds all have the same length.
Every uniform scale here — TT, TCG, TDB, TCB, and the GPS, Galileo, BeiDou
and NavIC system times — is a function of TAI (International Atomic Time).
`Instant<S>` carries a zero-sized scale marker `S` and converts through TAI.
UT1, the Earth's rotation read as a time, is not uniform. `hc-astro` gives
it a marker too, modelled through ΔT (TT − UT1). It lives there rather than
in `hc-core` because the ΔT model does.

```text
Instant<Tt> ──┐                      ┌── Instant<Tcg>
Instant<Gps> ─┼──▶ Instant<Tai> ─────┼── Instant<Tdb>
Instant<Ut1> ─┘                      └── Instant<Tcb>
                     │
                     ▼  (leap-second table)
               UtcInstant ──▶ UnixTime
```

UTC is not one of those markers. UTC's seconds are SI seconds, but its
labels repeat a second at each leap second, so UTC is not a constant offset
from TAI. It goes through the leap-second table instead. `UtcInstant`
carries a `leap_second` flag, so `23:59:60` has a representation.

### Where the two meet

They meet only where the caller asks them to, at a time zone. Turning a
`CivilDateTime` (a day plus a wall-clock time) into an `Instant<Tai>`
requires a zone and a leap-second policy. Both are explicit arguments.
Nothing in the library assumes a local time zone or reads a clock.

## Crate graph

The table lists the crates in dependency order: a crate depends only on
crates in rows above it. Every crate depends on `hc-core`. Every crate from
`hc-calendars-solar` down also depends on `hc-calendar`. The third column
gives each crate's other direct dependencies. The workspace manifest,
`Cargo.toml`, is the list of record.

| Crate | Holds | Also depends on |
| --- | --- | --- |
| `hc-core` | `Duration`, `Instant<S>`, time scales, the leap-second table, epochs, timestamp formats, math, the per-call memo of pure functions | — |
| `hc-calendar` | `Rd`, `CivilTime`, the `Calendar` trait, the registry | — |
| `hc-uncertainty` | Significant figures, fuzzy dates, EDTF, intervals | `hc-calendar` |
| `hc-units` | Exact ratios, tempo and media rates | — |
| `hc-deep-time` | Planck time to cosmology | `hc-uncertainty` |
| `hc-orbital` | Milankovitch orbital elements and insolation, Berger 1978 | `hc-uncertainty` |
| `hc-relativity` | Lorentz transforms, Schwarzschild, worldlines | `hc-uncertainty` |
| `hc-calendars-solar` | Gregorian, Julian, ISO, Coptic, … | — |
| `hc-tz` | Offsets, POSIX TZ, TZif, where each zone is | — |
| `hc-i18n` | Locales, plurals, names; country names and zones' cities behind its `territories` and `exemplar-cities` features | — |
| `hc-astro` | ΔT, solar longitude, new moon, rise and set | — |
| `hc-format` | ISO 8601, RFC 3339, RFC 2822, patterns, Python's ISO profile | `hc-calendars-solar`, `hc-tz`, `hc-i18n` |
| `hc-humanize` | Relative times, spelled-out durations, Python `humanize`'s phrasing | `hc-i18n`, `hc-units`, `hc-format` |
| `hc-planetary` | Mars sols, MTC, Darian and Martiana; Titan and Galilean circad calendars | `hc-astro` |
| `hc-calendars-lunar` | Hijri, Hebrew, Samaritan, Babylonian, Chinese, Korean, Vietnamese, Tibetan and Mongolian, Javanese, the Japanese lunisolar systems | `hc-astro`, `hc-calendars-solar` |
| `hc-calendars-equinox` | Solar Hijri, Badíʿ and French Republican by the equinox | `hc-astro`, `hc-calendars-solar` |
| `hc-seasons` | 24 terms, 72 pentads, 雑節, 六曜, the zodiac | `hc-astro`, `hc-calendars-solar`; `hc-calendars-lunar` behind its own `lunar` feature |
| `hc-calendars-regional` | Japanese, Qing and Korean eras, Maya, Aztec, Zapotec, Pawukon, Burmese, Thai and Khmer lunar, Olympiads | `hc-calendars-solar`, `hc-calendars-lunar` |
| `hc-calendars-indic` | Hindu lunisolar and solar calendars, Bikram and Nepal Sambat | `hc-astro`, `hc-calendars-solar`, `hc-seasons` |
| `hc-almanac` | 暦注 | `hc-astro`, `hc-calendars-lunar`, `hc-seasons` |
| `hc-attributes` | Birthstones and the like | `hc-seasons` |
| `hc-name-days` | Name-day lists by authority and edition, and a loader for licensed ones | — |
| `hc-fiscal` | Fiscal and academic years | `hc-calendars-solar`, `hc-calendars-indic` |
| `hc-holiday` | The rule engine and the country, tradition and exchange tables | `hc-astro`, `hc-seasons` and every `hc-calendars-*` crate |
| `hyper-calendar` | The feature-gated facade, the `civil` layer, and the lines both boundary crates write | Every crate above, each behind a feature |
| `hyper-calendar-ffi` | `cdylib` and `staticlib`, C ABI | `hyper-calendar` |
| `hyper-calendar-wasm` | `cdylib`, WebAssembly | `hyper-calendar` |

The graph is a DAG (directed acyclic graph). No crate depends on a crate it
does not need. No library crate depends on the facade; only the two
boundary crates do.

## The facade and the boundary crates

`hyper-calendar`, the facade, holds little logic of its own:

- It re-exports each crate behind a feature, and `registry()` returns a
  `CalendarRegistry` holding every calendar the enabled features provide.
- `civil` is a layer shaped like Python's `datetime`: `Date`, `Time`,
  `DateTime` and `TimeDelta` over `Rd`, `CivilTime` and `Duration`.
  [python-parity.md](python-parity.md) maps it row by row.
- The `*_lines` modules (`lines`, `time_lines`, `holiday_lines`,
  `season_lines`, `sky_lines`, `astro_lines`, `panchanga_lines`,
  `deep_time_lines`, `planetary_lines`, `relativity_lines`) and
  `calendar_values` produce the
  answers the two boundary crates return. Each answer about a set of things
  is a set of UTF-8 lines, one per entry, with tab-separated cells in a
  fixed column order. `boundary::Refusal` is the one list of reasons a
  line-maker refuses, which each boundary spells as its own error code.

The two boundary crates, `hyper-calendar-wasm` and `hyper-calendar-ffi`,
marshal those answers across a WebAssembly or C interface. They expose the
same layers as Cargo features: `civil` (the default), `timestamps`,
`calendars`, `holiday`, `seasons`, `deep-time`, `tz`, `sky`, `orbital`,
`planetary`, `relativity` and `full`. A page or a host program builds only
the layer it loads. Each crate's README lists every export with the feature
it needs; `crates/hyper-calendar/tests/abi.rs` renders those tables from
the source and fails when they drift.

A calendar that cannot name a day answers with a line that says so. The
line carries the stable code and name that `CalendarError` gives every
refusal. The WebAssembly crate ships a dependency-free ES module under
`crates/hyper-calendar-wasm/js` that decodes the lines by the README's
columns. CI tests it against the built module. A method whose layer the
build lacks throws an error that names the missing export.

## Why this many crates

A caller should compile in only what they use ([ADR
0004](adr/0004-one-crate-per-capability.md)). A split at the crate boundary
lets Cargo drop the code entirely. It also shrinks the `cargo audit`
surface. A WebAssembly build that formats Gregorian dates does not carry a
lunar ephemeris.

The facade turns each crate into a feature. Some features bundle others:
`holiday`, for example, turns on every calendar crate it dates by.
[supported.md](supported.md#facade-features) lists what each feature
implies.

## The two calendar interfaces

`Calendar` is statically typed: each calendar has its own `Date`, so a Hebrew
date cannot be passed where a Gregorian one is expected. `DynCalendar` is
object-safe and speaks `DateFields`, through `fields_to_fixed` and
`fixed_to_fields`. A registry can therefore hold calendars chosen at run
time, and an FFI caller can name one by string.

`DynAdapter<C>` derives the second from the first. A calendar author
implements `Calendar` once. The dynamic interface, the registry entry and the
default `days_in_month` and `days_in_year` follow. `is_leap_year` does not
follow: it is a required method of `Calendar`, answered by the calendar's own
rule. A calendar can have several common-year lengths, so "longer than the
previous year" does not identify a leap year.

`DateFields` is a fixed-capacity record: era, year, month with a leap flag,
day with a leap flag of its own for calendars that repeat a day, and up to
eight named extras. The Chinese leap fourth month is "閏四月", not
"month 13". Without the month's leap flag, every lunisolar calendar would
need a private encoding for it. The extras let the Maya long count and the
Balinese Pawukon use the same type as every other calendar.

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

Each row names a concern that is easy to get wrong and the crate that owns it.

| Concern | Crate | Note |
| --- | --- | --- |
| Exactness | `hc-core` | `i128` seconds + `u64` attoseconds; no float in the civil path |
| Leap seconds | `hc-core::leap` | A table. Replaceable without touching conversion code |
| Timestamp formats | `hc-core`, `hc-format` | TAI64, GNSS weeks, NTP eras, UUID timestamps, SAS and Stata counts, FAT words; see [time-scales.md](time-scales.md) |
| Calendar reform | `hc-calendars-solar` | Parameterised by adoption date, with a national table |
| Lunisolar rules | `hc-astro` + `hc-calendars-lunar` | Astronomy is a separate crate so the arithmetic calendars do not pay for it |
| Equinox rules | `hc-astro` + `hc-calendars-equinox` | Solar calendars judged by a clock at a place — Solar Hijri, Badíʿ, French Republican — beside their arithmetic siblings |
| Indian reckonings | `hc-astro` + `hc-seasons` + `hc-calendars-indic` | The tithi at sunrise and the month named by its saṅkrānti, downstream of the sidereal zodiac |
| Uncertainty | `hc-uncertainty` | EDTF, Allen interval relations, significant figures |
| Scale beyond seconds | `hc-deep-time` | Logarithmic magnitudes for Planck time and cosmology |
| The ice-age cycles | `hc-orbital` | A trigonometric series answered over ±1 Myr and refused beyond |
| Off-Earth clocks | `hc-planetary` | Mars sols, MSD, MTC, Darian, Martiana; circad calendars for Titan and the Galilean moons |
| Relativity | `hc-relativity` | Special and gravitational time dilation; worldlines integrated to proper time |
| Ambiguous local time | `hc-tz` | A three-way `LocalResolution`, never a silent pick |
| Localisation | `hc-i18n` | Static data plus a fallback chain; country names behind the `territories` feature, which only the facade's `holiday` feature turns on |
| Holidays, observances, exchanges | `hc-holiday` | One rule engine; every country, tradition and exchange is a table ([ADR 0008](adr/0008-exchange-calendars-are-rule-sets.md), [ADR 0009](adr/0009-a-working-day-is-an-entry.md)) |

## Further reading

- [policy.md](policy.md) — the standing rules this design serves
- [calendars.md](calendars.md) — the calendar coverage roadmap
- [observances.md](observances.md) — the holiday and religious-day roadmap
- [time-scales.md](time-scales.md) — what each scale is and how they relate
- [supported.md](supported.md) — the generated list of what exists
- [adr/](adr/README.md) — the decisions that could reasonably have gone the other way
