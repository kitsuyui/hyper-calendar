# The Samaritan calendar: the conjunction at Mount Gerizim and the Entry Era

Backs the identifier `samaritan` in `hc-calendars-lunar`.

## What it is

The Israelite Samaritans keep a lunisolar calendar of their own, on which
their seven festivals fall [samaritan-institute-calendar,
samaritans-net-calendar]. It is the Hebrew calendar's sibling in shape only. Its months were fixed by
observing "the birth of the moon" from Mount Gerizim, and in the twentieth
century the High Priest Avraham b. Pinchas "transferred the responsibility
for the calculation to a computer algorithm" [samaritan-institute-calendar];
the calendar each year is what the priesthood issues from that
calculation, which this library has not seen. The community describes a
nineteen-year cycle with seven leap years of thirteen months
[samaritans-net-calendar], not kept in step with the Jewish one, so that
"after a cycle of nineteen years the Seven Israelite Samaritan Festivals
fall one month after the corresponding Jewish festival"
[samaritan-institute-calendar]. The rule carried here has no molad and none
of the Rabbinic postponements: it reads the true conjunction.

The first month of the year is the month of Passover, following Exodus
12:2, but the year is counted from the entry into the land with Joshua,
and "on the six month of the year we go one year forward"
[samaritans-net-calendar]: Benyamim Tsedaka wrote in January 2017 that
"Samaritan year 3656 will start later this year, in the 6th month"
[samaritan-institute-calendar]. The day begins at the preceding sunset
[samaritans-net-calendar].

## How it works

Reingold and Dershowitz give an algorithm for the calendar, the
`samaritan-*` functions of their published code, marked there as a
"modern calculation" [reingold2018code]. It is what this library carries;
the book's own discussion [reingold2018] was not read, and the book's
errata correct only the Gregorian form of the epoch [reingold2018errata].

1. **The place.** Mount Gerizim, 32.1994° N, 35.2728° E, 881 m
   (`samaritan-location`).
2. **The month.** A month begins with the true conjunction, taken in
   apparent solar time at Gerizim: on the day of the conjunction if it
   falls at or before noon, on the day after if it falls after
   (`samaritan-new-moon-after`, the ceiling of the apparent moment less
   twelve hours). A month is 29 or 30 days, whatever the conjunctions make
   it.
3. **The first month** is the first month to begin after apparent noon at
   Gerizim on 11 March of the Julian calendar, 24 March Gregorian from
   1900 to 2100 (`samaritan-new-year-on-or-before`). A year from one
   first month to the next has twelve months, or thirteen when a
   thirteenth is needed to reach the next Julian 11 March.
4. **The year** is the Entry Era's: the epoch is 15 March 1639 BCE Julian
   (`samaritan-epoch`), 1 March −1638 Gregorian [reingold2018errata], and
   the year of a month is the whole number of Julian years of 365.25 days
   from the epoch to its first month, rounded, plus one from the sixth
   month on: `(+ (round (/ (- new-year samaritan-epoch) 365.25))
   (ceiling (- month 5) 8))`.

**Worked example: the Passover of 2019.** Julian 11 March 2019 is
24 March Gregorian; apparent noon at Gerizim falls about 09:45 UT. The
conjunction of 6 March is before it, so the first month waits for the
next, on 5 April 2019 at about 08:50 UT. At Gerizim that is 08:50 plus
2 h 21 min for the longitude and less some three minutes for the equation
of time: about 11:08 apparent, before noon, so the First Month begins on
5 April, and its fourteenth, the Passover sacrifice, is Thursday 18 April.
The epoch is RD −598 573 and 5 April 2019 is RD 737 154, 1 335 727 days on,
3657.02 years of 365.25 days: the year is 3657, and it had begun at the
Sixth Month of 2018. The community published Thursday 18 April 2019
[samaritan-institute-calendar].

The year of 2020 shows the rule's edge. The conjunction of 24 March 2020
came at about 09:28 UT, before noon at Gerizim that day, so it could not
open the year; the First Month began with the conjunction of 23 April, and
the sacrifice fell on Wednesday 6 May, as published.

## What is carried

- **Identifier** `samaritan`, `hc_calendars_lunar::samaritan`, era code
  `entry`; `SamaritanDate` carries the year, the month and the day.
- **Months** are numbered from where the year number changes, as `hebrew`
  numbers them from Tishrei: public month 1 is the Sixth Month, 7 the
  Twelfth, 8 the First and 12 the Fifth, and the Thirteenth Month of an
  intercalary year, between the Twelfth and the First, is
  `Month::leap(7)`. `SamaritanDate::biblical_month` gives the sources'
  ordinal, 1 to 13. English names are the ordinals, "First Month" to
  "Thirteenth Month", as the community's calendars print them; the sources
  read give no other names.
- **Leap years** are the years with a Thirteenth Month.
- **Range** Samaritan years 3539 to 3738, the Sixth Month of 1900
  (25 August) to the end of the Fifth Month of 2100. That bound is this
  library's choice: the code states no range, and the community's own
  calculation is of the twentieth century. Outside it the conversion
  refuses.
- **Native locales** `smp`, Samaritan Hebrew, and `he`.
- **Day boundary** sunset, named by the civil day the day ends on,
  `DayBoundary::Sunset(DayNaming::ByEnd)`: the day runs from the
  preceding sunset [samaritans-net-calendar], and the computation places
  days at the civil day whose daylight they hold, as `hebrew` does.
- **Not carried.** The priesthood's own computation, which was not
  read; the festivals as named days (they are the dates the tests
  read); the Samaritan script.

## Accuracy

| Check | Source | Test | Result |
| --- | --- | --- | --- |
| The Passover sacrifice on 10 April 2017, 29 April 2018, 18 April 2019 and 6 May 2020 | [samaritan-institute-calendar] | `the_published_passover_sacrifices_of_2017_to_2020` | all four |
| The Passover sacrifice on 20 April 2016 | [samaritan-institute-calendar] | `the_rule_puts_the_passover_of_2016_a_day_after_the_community` | **one day late**: 21 April |
| The Sixth Month on 11 September 2026, the Seventh on 11 October, the Day of Atonement on 20 October, Sukkot on 25 October, the Eighth and Ninth Months on 9 November and 9 December | [samaritans-net-calendar] | `the_months_of_autumn_2026_are_the_published_ones` | all six |
| Year 3656 begins at the Sixth Month of 2017 | [samaritan-institute-calendar] | `year_3656_began_at_the_sixth_month_of_2017` | yes |
| The epoch is 1 March −1638 Gregorian | [reingold2018errata] | `the_epoch_is_the_first_of_march_1638_bce_gregorian` | yes |

**The disagreement of 2016.** The conjunction of 7 April 2016 came at
about 11:24 UT, 13:43 apparent at Gerizim, well past noon, so the rule
begins the month on the 8th; the community kept the sacrifice on the 20th,
which puts the month's first day on the 7th. It is not a matter of
minutes, so it is a difference of rule, not of ephemeris. Every published
date read would also follow from a month beginning on the day of the
conjunction when it falls before sunset — the rule and that alternative
part only in 2016 among them, and the one published month of 2026 with a
conjunction after noon (10 October, about 18:11 apparent) is after sunset
too — but no source read states such a rule, and the library does not
invent one.

**Passover on 7 April.** The community's page says its sages "fixed the
date of Passover in the calendar so that it will not be earlier than
April 8" [samaritan-institute-calendar]. The rule allows 7 April, when the
conjunction falls just after noon on 24 March; in 1900–2100 it does so in
1906, 1925, 1944, 2001, 2039 and 2058, held by
`the_rule_allows_a_passover_on_7_april_where_the_community_says_8`. No
published date for any of those years was read.

The structure is checked too: every year has twelve or thirteen months of
29 or 30 days, with fourteen intercalary years in thirty-eight; every day
of four years round-trips through the dates and the generic fields; a
sample of the whole range round-trips, and the days either side of it
refuse. The astronomy is `hc-astro`'s — the Moon to about 10″, the
conjunction to a minute or two — so a conjunction within minutes of
apparent noon at Gerizim is decided by that model.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [reingold2018code] | The algorithm: `samaritan-location`, `samaritan-epoch`, `samaritan-noon`, `samaritan-new-moon-after`, `samaritan-new-moon-at-or-before`, `samaritan-new-year-on-or-before`, `samaritan-from-fixed`, `fixed-from-samaritan`, and the definitions they use, `apparent-from-universal`, `midday`, `julian-in-gregorian` | Yes, 2026-09-26 |
| [reingold2018] | The book the code accompanies | Not read |
| [reingold2018errata] | The epoch's Gregorian form, 1 March −1638 | Yes, 2026-09-26 |
| [samaritan-institute-calendar] | The computer calculation of Avraham b. Pinchas; the nineteen-year cycle and the month's difference from the Jewish festivals; the Passover sacrifices of 2016–2020; "not earlier than April 8"; Tsedaka on year 3656 | Yes, 2026-09-26 |
| [samaritans-net-calendar] | The day from sunset; the year moving on at the sixth month; the numbered months; the festivals of autumn 2026 | Yes, 2026-09-26 |

## Code

`crates/hc-calendars-lunar/src/samaritan.rs` (`GERIZIM`, `EPOCH`,
`MIN_YEAR`, `MAX_YEAR`, `SamaritanCalendar`, `SamaritanDate`,
`passover_sacrifice`, `is_leap_year`). Anchors:
`the_published_passover_sacrifices_of_2017_to_2020`,
`the_rule_puts_the_passover_of_2016_a_day_after_the_community`,
`the_first_month_of_2020_waited_for_the_conjunction_after_noon_on_24_march`,
`the_months_of_autumn_2026_are_the_published_ones`,
`year_3656_began_at_the_sixth_month_of_2017`,
`the_epoch_is_the_first_of_march_1638_bce_gregorian`; the structure:
`years_have_twelve_or_thirteen_months_of_twenty_nine_or_thirty_days`,
`every_day_of_four_years_round_trips`,
`a_sample_of_the_whole_range_round_trips_and_the_edges_refuse`. The
Julian dates come from `hc-calendars-solar`.
