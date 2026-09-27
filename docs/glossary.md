# Glossary

This glossary lists the words the documents use in a sense of their own,
and the one term each document uses for each idea. It is for a reader who
meets a word such as *pivot*, *layer* or *anchor* and wants its meaning
here. Each entry names the document that explains it in full.

Calendar and astronomical terms that belong to one system, such as
*tithi* or *saṅkrānti*, are defined in that system's document under
[`systems/`](systems/README.md), not here.

## Days, instants and time scales

| Term | Meaning | Explained in |
| --- | --- | --- |
| **Rata Die**, `Rd` | The integer day number every calendar converts to and from. Day 1 is 1 January of year 1 in the proleptic Gregorian calendar | [architecture.md](architecture.md#the-day-pivot-rata-die) |
| **Fixed date** | *Calendrical Calculations*' name for a Rata Die day. It survives in function names such as `to_fixed` and `from_fixed`; the prose says `Rd` | [architecture.md](architecture.md#the-day-pivot-rata-die) |
| **Pivot** | One of the two canonical representations that conversions pass through: `Rd` for days and TAI for instants | [architecture.md](architecture.md#two-pivots) |
| **Proleptic** | A calendar's rules applied to days before the calendar was adopted, such as the Gregorian rules before 1582 | [architecture.md](architecture.md#the-day-pivot-rata-die) |
| **Uniform time scale** | A time scale whose seconds all have the same length: TAI, TT, TCG, TDB, TCB, and the GPS, Galileo, BeiDou and NavIC system times. Each is a function of TAI, and `Instant<S>` carries it as the marker `S` | [architecture.md](architecture.md#the-instant-pivot-tai), [time-scales.md](time-scales.md) |
| **TAI** | International Atomic Time, the instant pivot | [time-scales.md](time-scales.md) |
| **UTC** | Coordinated Universal Time. Its seconds are SI seconds, but a leap second is inserted now and then, labelled `23:59:60`, so it is not a constant offset from TAI and not one of the `Instant<S>` scales. It has its own type, `UtcInstant` | [architecture.md](architecture.md#the-instant-pivot-tai), [time-scales.md](time-scales.md) |
| **Leap second** | A second inserted into UTC, labelled `23:59:60`. The library reads them from a table and does not predict them | [time-scales.md](time-scales.md), [policy.md §4](policy.md#4-the-library-refuses-to-guess) |
| **UT1** | The Earth's rotation read as a time. It is not uniform | [time-scales.md](time-scales.md) |
| **ΔT** | TT − UT1. `hc-astro` models it, with a stated error | [time-scales.md](time-scales.md) |

## Calendars

| Term | Meaning | Explained in |
| --- | --- | --- |
| **Calendar** | Anything that names days and converts them to and from `Rd`: a civil or religious calendar, an era count, or a day count such as the Julian Day Number | [architecture.md](architecture.md#the-day-pivot-rata-die) |
| **Identifier** | The lower-case string that names a calendar or a convention, such as `hebrew` or `japanese-northern`. It follows Unicode CLDR where CLDR has one. The roadmap's Id columns give it | [calendars.md](calendars.md#agreement-with-icu) |
| **Registry** | The `CalendarRegistry` that `registry()` returns: every calendar the enabled features provide, looked up by identifier | [architecture.md](architecture.md#the-facade-and-the-boundary-crates) |
| **Convention** | One authority's way of doing something that another authority does differently. Each convention gets its own identifier or function, not a parameter | [policy.md §5](policy.md#5-competing-conventions-get-names-not-parameters) |
| **Range** | The days an implementation converts, from its earliest to its latest day where it is bounded. Outside its range a calendar refuses | [supported.md](supported.md#calendars), the Earliest and Latest columns |
| **Usage** | When a calendar was actually in use, with the source that says so. It is distinct from the range, where the arithmetic is defined | `hc_calendar::Usage` |
| **Day boundary** | The point in the day at which a calendar's date changes: midnight, noon, sunset, sunrise or a fixed local time | `hc_calendar::DayBoundary`, [supported.md](supported.md#calendars), the Day begins column |
| **Astronomical calendar** | A calendar whose rules depend on astronomical computation or observation rather than arithmetic alone. It is exact only to the precision of its model | [policy.md §4](policy.md#4-the-library-refuses-to-guess) |
| **Lunisolar** | A calendar whose months follow the Moon and whose years are kept in step with the Sun by leap months | [calendars.md](calendars.md#stage-2--lunar-and-lunisolar-calendars) |

## Holidays

| Term | Meaning | Explained in |
| --- | --- | --- |
| **Rule engine** | The one piece of code in `hc-holiday` that evaluates every holiday table | [architecture.md](architecture.md#where-the-hard-parts-live) |
| **Table** | The data for one country, tradition, UN list or exchange: rules that the rule engine evaluates. A new country adds a table, not a branch | [policy.md §2](policy.md#2-data-is-separated-from-algorithm), [observances.md](observances.md) |
| **`sources_checked`** | The date on which a table's sources were last checked | [policy.md §10](policy.md#10-recurring-events-need-an-authority-not-an-opinion) |

## Crates and boundaries

| Term | Meaning | Explained in |
| --- | --- | --- |
| **Facade** | The `hyper-calendar` crate. It re-exports each capability crate behind a Cargo feature | [architecture.md](architecture.md#the-facade-and-the-boundary-crates) |
| **Boundary crates** | `hyper-calendar-wasm` and `hyper-calendar-ffi`, which carry the library across a WebAssembly or C interface | [architecture.md](architecture.md#the-facade-and-the-boundary-crates) |
| **Layer** | A Cargo feature of a boundary crate, such as `civil`, `holiday` or `tz`. A build carries only the layers it names | [architecture.md](architecture.md#the-facade-and-the-boundary-crates) |
| **Line** | One entry of an answer the boundary crates return: a UTF-8 line of tab-separated cells in a fixed column order | [architecture.md](architecture.md#the-facade-and-the-boundary-crates) |
| **Refusal** | An error returned instead of a guessed answer. `CalendarError` gives each a stable code and name, and `boundary::Refusal` lists the reasons a line-maker refuses | [policy.md §4](policy.md#4-the-library-refuses-to-guess), [architecture.md](architecture.md#the-facade-and-the-boundary-crates) |

## Documents and tests

| Term | Meaning | Explained in |
| --- | --- | --- |
| **Roadmap** | [calendars.md](calendars.md) and [observances.md](observances.md): everything considered, with a status of Done, Partial, Planned, Researching or Out of scope | [calendars.md](calendars.md) |
| **Inventory** | [supported.md](supported.md), generated from the code: what exists | [calendars.md](calendars.md) |
| **System** | A calendar, holiday regime, time code or other reckoning that a maintainer cannot be expected to know already | [policy.md §12](policy.md#12-a-complex-system-is-written-up-before-it-is-coded) |
| **System document** | The document under [`systems/`](systems/README.md) that explains one system, in six fixed sections | [systems/README.md](systems/README.md) |
| **Anchor** | A published reference value that a test checks the implementation against. Every algorithm has at least one | [policy.md §7](policy.md#7-correctness-is-demonstrated-not-asserted) |
| **Round trip** | Converting a value to another representation and back, and checking that the result is the value started from | [policy.md §7](policy.md#7-correctness-is-demonstrated-not-asserted) |
| **Sweep** | A test that walks every day of a range. The longest sweeps walk a fixed sample of the days in a debug build | [policy.md §7](policy.md#7-correctness-is-demonstrated-not-asserted) |
| **Not read** | The mark on a source that is cited but was not read, such as a source that a read source cites | [policy.md §11](policy.md#11-every-rule-from-the-literature-cites-it) |
