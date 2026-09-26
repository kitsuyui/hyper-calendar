# Mars timekeeping: MSD, MTC, local solar time, the Mars year, mission sols and the Darian calendars

Time on Mars as the people who operate spacecraft there keep it: a count of
sols, a clock on the prime meridian, local mean and true solar time at a
site, a count of Mars years from 1955, and the sol numbers each surface
mission runs. The Darian calendar, a proposed civil calendar on the same
sols, and its Martiana variant are included at the end of each section.

## What it is

A **sol** is the mean Martian solar day, 24 h 39 m 35.244 s, about 2.7 %
longer than an Earth day; the Martian sidereal day is 24 h 37 m 22.663 s
(`mars24-notes`). Nobody lives by a Martian calendar, but three groups keep
Martian time and need it to agree:

* **Scientists** comparing observations across decades count sols with the
  **Mars Sol Date** (MSD), the sequential count of sols from 1873 December 29
  near Greenwich noon (Julian Date 2 405 522.0), chosen before the perihelic
  opposition of 1877 so that almost every detailed observation of Mars has a
  positive MSD (`mars24-notes`). The clock on the Martian prime meridian,
  through the crater Airy-0, is **Coordinated Mars Time** (MTC), which Mars24
  calls mean solar time on the prime meridian or Airy Mean Time.
* **Seasonal studies** number **Mars years** from the northern spring
  equinox (areocentric solar longitude `Ls = 0`) of 1955 April 11, so that
  the planet-encircling dust storm of 1956 falls in Mars Year 1. The
  convention goes back to Clancy et al. 2000 (`clancy2000`) and is the one
  Piqueux et al. 2015 extend backwards (`piqueux2015`); neither paper was
  read here, and the date is as the module states it.
* **Mission operations** run a clock per lander. Each counts sols from its
  own landing, on its own meridian, and the rules differ from mission to
  mission.

The algorithm behind all of these is Allison's: Allison 1997 (`allison1997`)
and Allison and McEwen 2000 (`allison2000`), restated with later corrections
by NASA GISS in *Mars24 Sunclock — Algorithm and Worked Examples*
(`mars24-algorithm`), which says that typographical errors appeared in the
published paper and that equations B-1, B-2, C-2 and D-2 were revised in
2015 after later work by Allison. Mars24 itself "does not yet employ" any
Martian calendar (`mars24-notes`).

The **Darian calendar** is Thomas Gangale's proposal for a civil calendar on
Mars, first described in 1986: 24 months named alternately for the zodiac in
Latin and Sanskrit, seven-sol weeks, and years from the Martian vernal
equinox of 1609 March 11, the Martian year of Kepler's first two laws and of
the first telescopic observations (`gangale-darian`).

The **Martiana calendar** is Gangale's variant of 2002: the same months,
sols and epoch, with the week reconciled to the months by a modified form
of Robert G. Aitken's scheme of 1936 instead of by shortening it. The page
records "no popular support" for it, and the Darian calendar "remains the
preferred solution" (`gangale-darian`, sections 1.4.1 and 1.7).

## How it works

**MSD and MTC.** With `JD_TT` the Julian Date in Terrestrial Time, Mars24's
equation C-2 is

```text
MSD = (JD_TT − 2 451 549.5) / 1.027 491 251 7 + 44 796.0 − 0.000 962 6
MTC = 24 h × frac(MSD)
```

1.027 491 251 7 is the sol in Earth days, 44 796.0 the MSD at 2000 January
6.0, and 0.000 962 6 the adjustment that puts MSD sol boundaries on mean
midnight at Airy-0 (`mars24-algorithm`).

*Worked example* (Mars24's first): 2000 January 6, 00:00:00 UTC.

1. TT − UTC was 64.184 s, so `JD_TT = 2 451 549.5 + 64.184/86 400 =
   2 451 549.500 742 9`.
2. `(JD_TT − 2 451 549.5) / 1.027 491 251 7 = 0.000 742 9 / 1.027 491 251 7
   = 0.000 723 0` sol.
3. `MSD = 44 796.0 + 0.000 723 0 − 0.000 962 6 = 44 795.999 760 4`.
4. `MTC = 0.999 760 4 × 24 h = 23.994 25 h = 23:59:39`.

That is the "21 Mars-seconds away from also being mean midnight" that Mars24
reports for this instant: the sol whose MSD is 44 796 had not quite begun.

**Local solar time.** Local mean solar time at a planetographic longitude
Λ, in degrees west, is `LMST = MTC − Λ × (24 h / 360°)`. Local true solar
time adds the equation of time, `LTST = LMST + EOT × (24 h / 360°)`, where
EOT comes from the areocentric solar longitude `Ls` through Mars24's
equations B-1 to C-1: the mean anomaly, the fictitious mean sun, seven
planetary perturbation terms, the equation of centre, then `Ls` and EOT
(`mars24-algorithm`). Over a Mars year EOT runs from about −51 to +40
minutes, which the module's test measures against the extremes the GISS
technical notes give.

**The Mars year.** Mars Year `n` begins at the `Ls = 0` crossing after the
one that begins year `n − 1`, with Mars Year 1 beginning on 1955 April 11.
The tropical year is 668.5921 sols, 686.9725 days (`mars24-notes`). A year
before 1 is 0, then −1; the count is a plain integer.

**Mission sols.** A mission's sol `n` begins at midnight on the mission
clock, `n` sols after the midnight that began the landing sol. Four things
vary between missions, and each is a column of the mission table:

1. *The meridian.* The clock is built before launch on the planned landing
   longitude and not reset after landing, so it can run ahead of or behind
   local time at the site actually reached (InSight's by about 85 s,
   Phoenix's by about 3.5 min).
2. *Mean or true midnight.* Viking 1 and 2 and Pathfinder set their clocks
   to local *true* solar midnight on the landing sol and then ticked at the
   mean rate; every mission since has used local mean solar time.
3. *Sol 0 or sol 1.* Viking 1 and 2, Phoenix, Curiosity, InSight and
   Perseverance number the landing sol 0; Pathfinder, Spirit and Opportunity
   number it 1.
4. *Whether a convention was published at all.* Zhurong's was not.

**The Darian year.** A Darian year is 668 or 669 sols. Year `Y` is long if
it is odd or divisible by 10, except that years divisible by 100 are short
unless they are divisible by 500; the number of long years up to `Y` is
`(Y−1)\2 + Y\10 − Y\100 + Y\500`, with `\` integer division
(`gangale-darian`, section 1.2). Each quarter is five months of 28 sols and
one of 27; in a long year the last month, Vrishika, keeps a 28th sol. A
28-sol month is four weeks, so every month begins on Sol Solis.

*Worked example.* Year 219, in which Perseverance landed: it is odd, so it
is long, 669 sols. Year 220 is even and divisible by 10: long. Year 300 is
divisible by 100 and not by 500: short, 668 sols. Year 500: long. Over 500
years the rule gives 250 odd years, plus 50 multiples of 10 that are even
(every multiple of 10 is even), less 5 multiples of 100, plus 1 multiple of
500: 296 long years, a mean year of `668 + 296/500 = 668.592` sols against
the 668.5921-sol tropical year.

**The Martiana week.** The Darian calendar keeps every month starting on
Sol Solis by dropping the last Sol Saturni of each 27-sol month. The
Martiana calendar never drops a sol from the week. A quarter of five
28-sol months and one of 27 is 167 sols, 23 weeks and six sols, so every
month of a quarter begins on the same sol of the week and each quarter a
sol earlier than the one before: in even years Sol Solis, Sol Saturni,
Sol Veneris and Sol Jovis; in odd years Sol Mercurii, Sol Martis, Sol Lunae
and Sol Solis (`gangale-darian`, section 1.4.1, and its Table 1-13). An odd
year's leap sol, the 28th of Vrishika, is a sol of the week, so the next
year begins on Sol Solis again and the pattern repeats every two years. The
sol added every tenth year, also the 28th of Vrishika, is *epagomenal*,
"not counted as part of the week, thus the two-year rotation of the sols of
the week is not disrupted". Aitken's own scheme put both extra sols in even
years, giving years of 668, 669 and 670 sols, and the epagomenal sol at
mid-year; Gangale moved the leap sol to the odd years and the epagomenal
sol to the end.

The page's text says "In the even-numbered years" for both sets of quarter
starts; the second is the odd years, as Table 1-13's "Odd-Numbered Years"
column and the arithmetic agree: an even year's winter quarter begins on
Sol Jovis, and 167 sols later it is Sol Mercurii.

*Worked example.* On which sol of the week does 1 Gemini 219 fall? Year
219 is odd, and Gemini is the first month of the third quarter, so it
begins on Sol Lunae; its 1st is Sol Lunae. And Vrishika 28 of 220: 220 is
even and decennial, so the sol exists and is epagomenal, the one sol of
the year with no name in the week.

The Martiana text gives the long years as the odd and the decennial ones
and states no exception for the centuries, where the Darian rule makes
years divisible by 100 and not by 500 short. Taken as written, the mean
year is 668.6 sols, Aitken's, and the two calendars share every date of
years 0 to 99 and then part by a sol at each century not divisible by 500.

## What is carried

In `hc-planetary`:

* `mars::MarsMoment` and the free functions `mars_sol_date`,
  `coordinated_mars_time`, `solar_longitude` and `mars_year`, over
  `hc_core::Instant<Tai>`; local mean and true solar time at a longitude,
  east or west.
* The Mars year under the convention above: `mars_year_start` solves for
  the `Ls = 0` crossing from the seed `MARS_YEAR_1_START_MSD`, which is this
  library's own solution near 1955 April 11 and not a published constant,
  because the sources give the date without a time of day.
* `mars::missions`: the surface missions from Viking 1 to Zhurong, with
  landing instant, site and clock longitudes, mean or true midnight, the
  first sol's number and whether the convention was published, and a
  `MissionClock` that turns an instant into a mission sol and time.
* `mars::darian`: the Darian calendar, whose conversions are sol-indexed.
  It also implements `hc_calendar::Calendar`, where its `Rd` is a Darian sol
  count and not an Earth day.
* `mars::martiana`: `martiana`, the Martiana calendar, on the same sol count
  from the same epoch, so that a Darian and a Martiana date convert into
  each other through the `Rd`. The sol of the week is `Option`al: the
  epagomenal sol has none. The long years are taken as the text gives them,
  every odd and every decennial year, with no century exception; if a
  source is found that applies the Darian `\100` and `\500` terms, that
  would be a second calendar under its own name, not a change to this one.

Not carried:

* **The 2000 midnight adjustment as the default.** Allison and McEwen's
  0.000 72, as the module states it, puts MSD 44 796.0 exactly at 2000 January
  6.0 UTC; Mars24's revised 0.000 962 6 moves sol boundaries by 21.5 Martian
  seconds. Carrying Mars24's value is this library's choice, so that MSD and
  MTC agree with the tool and worked examples Mars operations use; the 2000
  value is exported as `MSD_MIDNIGHT_ADJUSTMENT_2000` for reproducing the
  paper.
* **MER hybrid local solar time.** Spirit and Opportunity ran a constant
  offset from site LMST; the sol numbers are reproduced, but the time of sol
  for those rovers is LMST.
* **Gangale's extended intercalation scheme.** His page offers a series of
  formulas fitted to the 668.5907-sol vernal-equinox year, starting with
  `\1000` for years 0–2000, "as an example of the accuracy that is
  achievable", not as the calendar's rule, and no dated correspondence under
  it was found to test against (`gangale-darian`, section 1.2.1).
* **Aitken's calendar of 1936.** Section 1.4.1 describes it in a sentence —
  both extra sols in even years, the epagomenal one at mid-year — and links
  a page on `pweb.jps.net`, which did not answer on 2026-09-26; without its
  epoch, its months or the position of its epagomenal sol, it is not
  carried.
* **Other Martian calendars and year counts.** Mars24 notes that dozens have
  been proposed; only the Clancy count, the Darian calendar and its Martiana
  variant are carried.

## Accuracy

Allison and McEwen state a maximum error in `Ls` of about 0.008° over ±100
years of J2000, about three seconds of true solar time, as the module
quotes them; outside roughly 1900–2100 the model extrapolates, because it
carries no secular change of the orbital elements.

Checked against:

* **Mars24's two worked examples** (`mars24-algorithm`): 2000 January 6
  00:00:00 UTC gives `Ls` 277.187 58°, EOT −5.187 74° and MTC 23:59:39;
  2004 January 3 13:46:31 UTC gives LTST 00:00:00 at 184.702° W, Spirit's
  planned site. Both reproduce to the last printed digit.
* **Published mission sols**: ten sol-to-date correspondences, four of them
  from the Mars24 lander list (`mars24-landers`): Viking 1 sol 2243 on
  1982-11-11, Viking 2 sol 1280 on 1980-04-11, Pathfinder sol 93 on
  1997-10-07, Spirit sol 2210, Opportunity sol 5111, Phoenix sol 156,
  Curiosity sols 1000 and 4032, InSight sol 1440 and Perseverance sol 997.
  Each falls within the Earth day the literature gives.
* **Mars-year starts**: eight `Ls = 0` crossings from Mars Year 24 to 38 each
  fall on the Gregorian day that circulating Mars-year tables give; those
  tables were not traced to a primary source here, and the time of day of
  the 1955 start is where published figures disagree (the module's
  documentation of `MARS_YEAR_1_START_MSD` gives the measurement).
* **The Darian calendar**: the dates of the Viking 1 and Perseverance
  landings, 14 Mina 195 and 13 Sagittarius 219, as the comparison table in
  Wikipedia's article on the calendar gives them (a secondary source), and
  the leap rule's mean year of 668.592 sols.
* **The Martiana calendar**: the weekday on which every month of every
  quarter begins in Table 1-13, even years and odd, the leap sol on Sol
  Saturni and the decennial sol outside the week, and agreement with the
  Darian calendar on every sol of years −99 to 99. No dated Martiana date
  was found published.

## Sources

| Key | What it was used for | Read |
| --- | --- | --- |
| `mars24-algorithm` | Every equation from MSD to LTST, the 2015 revisions, the two worked examples | yes, retrieved 2026-09-26 |
| `mars24-notes` | The sol, the sidereal day, the tropical and sidereal years, the MSD epoch, Airy-0 | yes, retrieved 2026-09-26 |
| `mars24-landers` | Landing dates and sites, mission sols | yes, retrieved 2026-09-26 |
| `allison1997`, `allison2000` | The algorithm Mars24 restates; the 0.008° accuracy claim | no |
| `clancy2000` | The Mars Year convention from 1955 April 11 | no |
| `piqueux2015` | The enumeration of Mars years back to the telescopic era | no |
| `gangale-darian` | The Darian months, weeks, both intercalation schemes, the epoch; the Martiana calendar (section 1.4.1 and Table 1-13 at `t2002martiana.htm`) | yes, retrieved 2026-09-26 |
| — | Aitken's calendar of 1936, which the Martiana page links | no |
| `gangale2006` | Gangale's SAE paper on the Darian system | no |

## Code

* `crates/hc-planetary/src/mars.rs`: MSD, MTC, `Ls`, local times, the Mars
  year. Tests: `the_first_published_worked_example_reproduces`,
  `the_second_published_worked_example_reproduces`,
  `mars_year_starts_land_on_the_published_dates`,
  `the_landings_fall_in_the_mars_years_the_literature_gives_them`.
* `crates/hc-planetary/src/mars/missions.rs`: the mission table and clocks.
  Tests: `the_published_sol_anchors_all_land_in_the_right_sol`,
  `the_sol_numbering_conventions_are_the_ones_the_missions_used`,
  `the_insight_clock_runs_the_published_eighty_five_seconds_fast`.
* `crates/hc-planetary/src/mars/darian.rs`: the Darian calendar. Tests:
  `the_leap_rule_is_gangales`, `the_published_event_conversions_reproduce`,
  `darian_years_are_mars_years_plus_one_hundred_and_eighty_three`.
* `crates/hc-planetary/src/mars/martiana.rs`: the Martiana calendar. Tests:
  `every_month_of_a_quarter_begins_on_the_table_1_13_weekday`,
  `the_week_never_breaks_except_for_the_epagomenal_sol`,
  `the_leap_sol_is_a_sol_saturni_and_the_epagomenal_sol_has_no_weekday`,
  `martiana_and_darian_agree_before_the_first_century`,
  `the_two_calendars_convert_through_the_shared_sol_count`.
* `crates/hyper-calendar/src/planetary_lines.rs`: the lines the
  WebAssembly module and the C library write, behind their `planetary`
  layer — `hc_mars_time`, `hc_missions`, `hc_mission_sol`, `hc_bodies`
  and `hc_body_time` — answering only within 100 Julian years of J2000.0
  and refusing Zhurong's sol, whose convention was not published. Tests:
  `the_first_mars24_worked_example_reproduces`,
  `the_second_worked_example_is_true_midnight_at_spirits_planned_site`,
  `a_mission_sol_follows_the_missions_own_clock`,
  `an_unpublished_convention_is_listed_empty`.
