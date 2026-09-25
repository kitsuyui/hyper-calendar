# hc-calendar

The calendar abstraction shared by every calendar in the workspace: the day
number they all convert through, the two interfaces they implement, and the
cycles that name days without counting them.

## The pivot

There is no useful "common calendar". What every calendar shares is a way to
name a day, and days can be counted. `Rd`, the Rata Die fixed day number, is
that count, with day 1 being `0001-01-01` in the proleptic Gregorian calendar.
Every calendar implements two operations against it:

```text
fields  --to_fixed-->  Rd  --from_fixed-->  fields
```

so a library of *n* calendars needs 2*n* conversions rather than *n*², and two
calendars can be shown side by side without either knowing the other exists.
This is the design of Reingold and Dershowitz's *Calendrical Calculations*.

Because the sentence "day 1 is `0001-01-01`" is what fixes the origin, the
proleptic Gregorian arithmetic that implements it lives here, in `gregorian`,
rather than in a calendar crate, where every crate below the calendars can
reach it.

## Two interfaces

| | Dates are | For |
| --- | --- | --- |
| `Calendar` | the calendar's own `Date` type | code compiled against the calendar, so a Hebrew date cannot be passed where a Gregorian one is expected |
| `DynCalendar` | `DateFields`, a small fixed-capacity bag of named numbers | a registry, a formatter or an FFI caller that names a calendar by string |

`DynAdapter` derives the second from the first, so no calendar writes the
bridge twice. `CalendarRegistry` (behind `alloc`) holds any number of them
behind one interface.

## What a calendar has to say about itself

- **Its shape.** `Calendar::cycles` has no default: every calendar declares
  the positional cycles it is made of — months, weekdays, a thirteen-day
  trecena, the ten concurrent weeks of the Pawukon — each with its length and,
  where the calendar names its positions itself, the names in the orthography
  its sources use. A calendar whose dates carry a month declares a `month`
  cycle, and one whose dates carry none declares none; a calendar that says
  nothing does not compile. This is what the vocabulary in `hc-i18n` is keyed
  to, and it is why a nineteen-month calendar can be named.
- **Its metadata.** `CalendarMeta`: identifier, English name, how it counts
  years, whether it has leap months, whether it is astronomical, and the range
  its arithmetic is defined over.
- **Where its day begins, and when it was in use.** `day_boundary` and `usage`
  default to midnight and "unrecorded"; a calendar that differs — the Julian
  Day at noon, the Hebrew day at sunset, the Hindu day at sunrise — overrides
  them, so the override itself is the documentation. A period of use names
  its source, and can end twice: the Chinese calendar's civil use ended in
  1912 and its use for the festivals has not, which `Usage::civil_until`
  carries beside `Usage::until`.

## Cycles that are not calendars

`weekday` holds the seven-day week and, as `DayCycle`, the weeks that are not
seven days long. `cycle` holds the East Asian sexagenary cycle: the sixty
stem–branch pairs, the readings they are written in (a catalogue of nine —
characters, pinyin with and without tones, the Japanese kun readings in kana
and romanised, the romanised on readings, Hangul and its romanisation, and
Vietnamese — each anchored at 甲子 and 癸亥), the
twelve double-hours, and the four pillars of year, month, day and hour. Every
one is a pure function of `Rd`, which is why they live here; and every one
stops where astronomy would begin. The solar term that fixes a month pillar is
an argument, supplied by `hc-seasons`.

## Accuracy

| Claim | |
| --- | --- |
| `Rd` ↔ proleptic Gregorian | exact over years −9 999 999 to 9 999 999; outside that range the conversion refuses rather than wraps |
| the weekday | `Rd` 1, 0001-01-01, is a Monday, and the seven-day cycle has never been interrupted |
| the day pillar | anchored to the published rule that the stem is (JDN + 9) mod 10 and the branch (JDN + 1) mod 12, and to 1984-02-02 as the first day of a 甲子 year |
| the year pillar | three conventions — 立春, the lunisolar new year, 1 January — as three named functions, since they disagree for days at a time |
| the hour pillar | both schools of the 子 hour, named, since they give different pillars for the same instant |

## What this crate deliberately does not do

- No astronomy. Nothing here computes a solstice, a new moon or a solar term.
- No time zones. Pillars are computed from local civil time, and which local
  time is `hc-tz`'s question.
- No translation. The English names here — `jia`, `rat`, `wood` — are the
  library's convention for prose, not a locale; `hc-i18n` chooses what a
  locale writes, and falls back to a calendar's own names when it has none.

## Feature flags

| Feature | Effect |
| --- | --- |
| `std` (default) | implies `alloc` |
| `alloc` | `CalendarRegistry` and the dynamic interface's boxed calendars |

Without either, the static interface, the fixed-capacity `DateFields`, the
Gregorian arithmetic and every cycle are all available.
