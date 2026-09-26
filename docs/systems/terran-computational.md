# The Terran Computational Calendar: an instant system over TAI

Backs `hc-calendars-solar::terran`, a pair of functions rather than a
registered calendar; the identifier `terran-computational` names it in the
roadmap and nowhere in the registry.

## What it is

A constructed calendar published at terrancalendar.com, under the Creative
Commons Public Domain Mark, which counts the years, months, days, hours,
minutes and seconds that have *elapsed* since its epoch, 0TC, near the
northern winter solstice of 1969 [terran-calendar]. Every field is
zero-based and every unit but the year's last is of constant length, so a
date is an unambiguous instant rather than the name of a day: "exactly 42
years, 13 months, 1 days, 0 hours, 0 minutes, and 0 seconds have passed
since the Terran Computational Epoch" is how the site reads `42.13.1,0.0.0
TC`. Nobody keeps it; it is carried because its rule is stated exactly, is
free, and is the one calendar in the roadmap that is defined on atomic time
and on the IERS leap seconds rather than on days.

## How it works

**The epoch.** 0TC is "221788790 SI seconds (measured at the geoid) before
1977-01-01 00:00:00 TAI", which the site derives as
`(365·5 + 366·2 + 10)·86400 − 10` [terran-calendar]: 2 567 days of 86 400
seconds before 1977 TAI, less ten seconds, so 1969-12-22 00:00:10 TAI. The
site's own gloss, "for most practical purposes ... 1969-12-22 00:00:00 UTC
or −864000 seconds of (10 days before) the UNIX Epoch", is ten seconds of
TAI − UTC read back past 1972, where UTC had no whole-second offset; the
TAI instant is the definition.

**Years and the minimonth.** A year is thirteen months of 28 days, 364 days,
followed by a *minimonth*, month 13, that holds the year's "leap duration":
one leap day, or two "in years that are a multiple of 4 but not of 128",
and every leap second "issued by the IERS within the bounds of a given
terran computational year" [terran-calendar]. Year 0 and every multiple of
128 has one leap day; the mean year is 365 + 1/4 − 1/128 = 365.2421875
days before the leap seconds. The leap seconds follow the leap days in the
minimonth, "all leap units have the same field placement", so the 1972 leap
second of TC year 2, which has one leap day, is `2.13.1,0.0.0 TC`: the
instant after the minimonth's day 0 [terran-calendar].

**Which year a leap second belongs to.** A UTC leap second is counted in
the TC year whose bounds contain it — the year whose start precedes the
inserted second's instant and whose end, including its own minimonth,
follows it. UTC's leap seconds fall at the ends of June and December, and
the TC year turns about 21 December, so a 31 December leap second belongs
to the TC year that has just begun: 1972-12-31 23:59:60 UTC is in TC year
3, at `3.0.11,0.0.0 TC`, and 1972-06-30 in year 2, at `2.6.24,0.0.0 TC`
[terran-calendar]. Inside the year, UTC and TC then differ by one more
second until the year's minimonth absorbs it.

**Year bases.** A designator `TCn` counts only the leap duration of the
years before nTC and none after, so that a date written ahead of the IERS's
announcements does not move when they are made; `TC0` counts none, and
plain `TC` counts every one [terran-calendar]. The instant is the same; the
fields differ by the leap seconds left out.

**Worked example.** The site puts the solstice of 2011-12-22 05:30:00 UTC
at `42-00-00 05:30:00 TC` [terran-calendar]. TAI − UTC was 34 seconds, so
the instant is 2011-12-22 05:30:34 TAI, and 15 340 days, 5 hours, 30
minutes and 24 seconds after the epoch: the TC timestamp 1 325 395 824.
Years 0 to 41 hold 42 · 364 days, 42 leap days and 10 more for the years
4, 8, …, 40 (0 is a multiple of 128), 15 340 days, and 24 leap seconds,
those of years 2 to 39. Year 42 therefore begins 15 340 · 86 400 + 24 =
1 325 376 024 seconds after the epoch, and the solstice is 19 800 seconds,
5 hours 30 minutes, into it: `42.0.0,5.30.0 TC`. Under `TC0` the year
begins 24 seconds earlier and the same instant is `42.0.0,5.30.24 TC0`.
Nine months later, 2012-09-22 14:49:00 UTC is `42.9.23,14.49.1 TC`, one
second ahead of the UTC clock face, because the leap second of 30 June
2012 falls inside TC year 42 and waits for its minimonth.

## What is carried

- **A function pair, not a calendar.** `terran::from_tai` and
  `terran::to_tai` convert between an `hc_core::Instant<Tai>` and a
  `TerranDate` — year, month 0 to 13, day, hour, minute, second and
  attoseconds — under a `YearBase`, `Full` for `TC` or `Base(n)` for `TCn`;
  `terran::timestamp` is the `TC+` count of seconds. The `Calendar` trait
  maps a fixed day to a date and back, and a TC day is not a fixed day:
  after a leap second inside a TC year its days begin a second before UTC
  midnight until the year ends, and under `TC` its years begin wherever the
  accumulated leap seconds put them. The system is defined on SI seconds
  from a TAI instant, so it takes the instant type `hc-core` already has,
  and it lives in `hc-calendars-solar` beside the arithmetic calendars
  because `hc-core` carries no calendar knowledge by its own rule. It is
  therefore not in the registry, has no `DateFields` and no vocabulary; the
  site says "there is really no requirement to associate any names or
  labels with the calendar at all".
- **The leap seconds** are `hc_core::leap::TABLE`, the IANA copy of IERS
  Bulletin C, one table for the whole workspace. That table has 27 leap
  seconds; the site's tables stop at the 25th, of 2012, and the rule places
  the two since, of 2015-06-30 and 2016-12-31, in TC years 45 and 47.
- **Refusals.** Under `TC` an instant past the table's announced validity
  is refused with `TimeError::AfterModelEnd`, since an unannounced leap
  second would move it; under `TCn` only a base whose year begins past the
  validity is refused, which is what year bases are for. Before 0TC there
  are no leap seconds, and negative years are the arithmetic continued.
- **Display** writes the dotted form, `42.0.0,5.30.0 TC` or with the base,
  `TC0`, and the fraction of a second where there is one.
- **Not carried:** the parser for the site's eight delimiters and optional
  fields, and *datemods*, which are durations added to the instant
  (`Instant::checked_add` already does that); the site's pre-1977 caveat,
  "converted dates before 1977 TAI may not be exact", concerns TAI's
  realisation and is not modelled, since `Instant<Tai>` is a uniform
  reading.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The epoch is 221 788 790 s before 1977-01-01 TAI; TAI64 leads TC by 2⁶² − 863 990 s | `the_epoch_is_the_sites` | exact |
| The 25 UTC leap seconds the site lists fall at its 25 TC dates, and in its 25 TC years | `the_utc_leap_seconds_fall_where_the_site_puts_them` | all 25 |
| The site's 25 "terran computational leap seconds" are its UTC instants | `the_minimonth_leap_seconds_are_the_sites` | all 25 |
| The site's solstices and equinoxes of 2010–2020, 44 instants, including `42.9.23,14.49.1` | `the_seasons_table` | 40 exact; 4 one second later here |
| The worked example above, under `TC` and `TC0` | `the_worked_example` | exact |
| `44.6.14TC = TC+1404172825`, `44TC+39W = 44.9.21TC` | `the_datemod_examples` | exact |
| Leap days: one, two in multiples of 4 and not of 128 | `the_leap_days` | all |
| Round trips under `TC`, `TC0` and `TC44` across the leap seconds | `every_instant_round_trips` | all |
| Past the table under `TC`, a base the table does not reach, a minimonth past its leap duration | `what_cannot_be_known_is_refused` | refused |

The four seasons that differ are the ones after the two leap seconds the
site's tables stop short of: 2015-09-23 in TC year 45, after the leap
second of 30 June 2015, and 2017-03-20, 2017-06-21 and 2017-09-22 in TC
year 47, after that of 31 December 2016. The site's own conversions count
neither, and read those instants a second earlier into the year; the rule
as the site states it counts both, as it counts the leap second of 30 June
2012 in `42.9.23,14.49.1`, so this library follows the rule and records
the disagreement in the test.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [terran-calendar] | The whole definition, the epoch and its derivation, the leap-day rule, the minimonth, year bases, the leap-second and seasons tables, the datemod examples, the public-domain mark | Yes, 2026-09-26 |
| [iana-leap-seconds-list] | The leap seconds, through `hc_core::leap` | Through `hc-core` |

## Code

`crates/hc-calendars-solar/src/terran.rs`. Anchors:
`the_utc_leap_seconds_fall_where_the_site_puts_them`, `the_seasons_table`,
`the_datemod_examples`. The year boundary is `year_start`; the assignment of
leap seconds to years `LEAP_SECOND_YEARS`.
