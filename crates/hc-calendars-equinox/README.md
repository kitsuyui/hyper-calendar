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
| `persian_afghan` | `persian-afghan` | the same: `persian`'s days under Afghanistan's month names | the same |
| `persian_apparent_noon` | `persian-apparent-noon` | the Sun's noon at Tehran (51.42° E) | the same |
| `bahai` | `bahai-astronomical` | sunset at Tehran | 1 BE to Gregorian 3000 |
| `french_republican` | `french-republican-equinox` | true midnight at the Paris Observatory | An I to Gregorian 3000 |

Each has an arithmetic sibling in `hc-calendars-solar` that approximates it
by a cycle — `persian-arithmetic` and `persian-arithmetic-33`,
`bahai-arithmetic`, `french-republican-arithmetic` — and the Badíʿ calendar also has the *as
kept* form `bahai` there, which carries the Bahá'í World Centre's published
table for 172–221 BE. The date types and the calendars' own month names are
shared, so a date converts between variants without ceremony.

`persian-afghan` is the Solar Hijri calendar as Afghanistan kept it: the
same days, the months named for the zodiac signs in their Arabic forms —
حمل, ثور … حوت in Dari, the calendar's own names, with the Pashto and
romanised names in `hc-i18n` — civil from 1 Hamal 1336 (21 March 1957),
when Afghanistan fixed the month lengths, to 7 Asad 1401 (29 July 2022),
the day before official correspondence moved to the lunar Hijri year, and
in use beside the lunar dates since. Its module states the sources, and
the four years of the civil period in which a noon at Kabul rather than
Tehran would have moved 1 Hamal.

`persian-apparent-noon` is the other reading of Iran's rule: the noon that
decides Nowruz is the Sun's transit at Tehran, as Heydari-Malayeri (2004)
and Reingold and Dershowitz state it, rather than the clock's. It gives
`persian`'s days from 1178 to 1469 and parts from it in twenty of the three
thousand years converted, the first after 1177 being 1470 (2091). The Solar
Hijri calendars, their readings and their arithmetic approximations are
written up in
[`docs/systems/solar-hijri.md`](../../docs/systems/solar-hijri.md).

The Badíʿ and French Republican calendars, with their siblings, are
written up in
[`docs/systems/equinox-calendars.md`](../../docs/systems/equinox-calendars.md):
the 2015 rules and the decree of 4 frimaire an II, Naw-Rúz 183 BE and
1 vendémiaire An IV worked by hand, and the years too close to call.

## What the tests are

The published record, not the model checking itself:

* the fourteen new years France kept, An I (22 September 1792) to An XIV
  (23 September 1805), and the sextile years among them;
* the two equinoxes the decree of 4 frimaire an II gives as observed,
  within a minute in true time at the Paris Observatory;
* the leap years Iran had between 1354 and 1419, and Nowruz 1404 on
  21 March 2025 — a day later than Birashk's cycle says — under both
  readings of noon;
* the Afghan dates the sources print: 6 Hamal 1401 as 26 March 2022 in
  Hasht-e Subh's dateline, and 8 Asad 1401 as the 1 Muharram 1444 from
  which the official lunar calendar runs;
* every row of the Bahá'í World Centre's *Badíʿ dates 172 to 221 BE* that
  the model can claim: fifty pairs of Twin Holy Birthdays from the eighth
  new moon, and forty-eight of the fifty Naw-Rúzes. The two others fell
  within a sunset's tolerance — one of them, 2026, is an equinox within
  seconds of Tehran's sunset — and the test names them rather than claims
  them, because a row like that is the table's to decide. The model agrees
  with the table on both.

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
