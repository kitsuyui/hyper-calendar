# `hc-calendars-equinox`

Solar calendars defined by an equinox observed at a place, for
[`hyper-calendar`]: the Solar Hijri, Badíʿ and French Republican calendars
computed from the sky rather than from a cycle.

An equinox is one instant for the whole Earth; which *day* it falls on
depends on whose clock is asked. Each calendar here names its clock.

## What is here

| Module | Identifier | The clock | Range |
|---|---|---|---|
| `persian` | `persian` | noon, Iran Standard Time (52.5° E) | 1 AP to Gregorian 3000 |
| `bahai` | `bahai-astronomical` | sunset at Tehran | 1 BE to Gregorian 3000 |
| `french_republican` | `french-republican-equinox` | true midnight at the Paris Observatory | An I to Gregorian 3000 |

Each has an arithmetic sibling in `hc-calendars-solar` that approximates it
by a cycle — `persian-arithmetic`, `bahai-arithmetic`,
`french-republican-arithmetic` — and the Badíʿ calendar also has the *as
kept* form `bahai` there, which carries the Bahá'í World Centre's published
table for 172–221 BE. The date types and the calendars' own month names are
shared, so a date converts between variants without ceremony.

## What the tests are

The published record, not the model checking itself:

* the fourteen new years France kept, An I (22 September 1792) to An XIV
  (23 September 1805), and the sextile years among them;
* the leap years Iran had between 1354 and 1419, and Nowruz 1404 on
  21 March 2025 — a day later than Birashk's cycle says;
* every row of the Bahá'í World Centre's *Badíʿ dates 172 to 221 BE* that
  the model can claim: fifty pairs of Twin Holy Birthdays from the eighth
  new moon, and forty-eight of the fifty Naw-Rúzes. The two others fell
  within a sunset's tolerance — one of them, 2026, is an equinox within
  seconds of Tehran's sunset — and the test names them rather than claims
  them, because a row like that is the table's to decide.

## What it is not

Exact beyond the astronomy. `hc-astro` places an equinox to within seconds,
and the deciding clock to what it can be placed to — a standard-time noon
exactly, an apparent midnight to seconds, a sunset to a minute or two and
to the horizon the almanac assumed. Each module states its
`TOLERANCE_MINUTES` from those parts, and a year whose equinox falls that
close to its deciding instant is decided here by a model where the country
or the community decided by an ephemeris. Each module's `new_year_margin`
says how close the call was.

## Features

`std` (default) and `alloc`; with neither, the calendars still convert and
only the registry is absent. Nothing here allocates on the conversion path.

[`hyper-calendar`]: https://github.com/kitsuyui/hyper-calendar
