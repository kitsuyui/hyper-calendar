# Circad calendars: the Darian calendar for Titan and the Gregorian-based calendars of the Galilean moons

Backs the identifiers `darian-titan`, `gregorian-io`, `gregorian-europa`,
`gregorian-ganymede` and `gregorian-callisto` in `hc-planetary`.

## What it is

Thomas Gangale, the author of the Darian calendar for Mars, extended it in
1998 to Jupiter's four large moons and in 2003 to Saturn's moon Titan
(`gangale-jupiter`, `gangale-titan`). Nobody lives by these calendars; they
are proposals, published on the same site as the Darian calendar, for
settlements that do not exist.

The problem they solve is that none of these moons has a day a person could
live by. Each is tidally locked, so its solar day is its orbit around its
planet: 1.77 Earth days on Io, 16.75 on Callisto, 15.97 on Titan. Gangale
divides each solar day into a whole number of *circads* (from *circa dies*,
"about a day") chosen to come out near 21 to 24 hours, and builds weeks,
months and years out of circads. The year is not the moon's own — Jupiter's
year is twelve Earth years and Saturn's nearly thirty — but a borrowed one:
Earth's for one family of Galilean calendars, Mars's vernal-equinox year for
the other and for Titan (`gangale-jupiter`, §2.3 and §2.8; `gangale-titan`,
§3.2).

## How it works

### Circads and weeks

| Body | Solar day (Earth days) | Circads per solar day | Circad (hours) |
| --- | --- | --- | --- |
| Io | 1.769 860 | 2 | 21.24 |
| Europa | 3.554 094 | 4 | 21.32 |
| Ganymede | 7.166 386 | 8 | 21.50 |
| Callisto | 16.753 548 | 19 | 21.16 |
| Titan | 15.969 095 | 16 | 23.95 |

The Galilean solar days are Table 2-1 of `gangale-jupiter` and the divisors
its Table 2-2. Titan's solar day is derived on the Titan page from Saturn's
sidereal year of 29.447 498 years and Titan's orbit of 15.945 420 68 days,
and its circad is printed as 0.998 068 439 days, 23 h 57 m 13.11 s
(`gangale-titan`, §§3.2–3.3). The week is eight circads everywhere: half a
Titan solar day, one Ganymede solar day, two on Europa and four on Io; on
Callisto it has no astronomical meaning and is kept for uniformity
(`gangale-jupiter`, §2.4). The eight circads are named for the seven planets
of the terrestrial week and the Earth, with a prefix for the moon: Io Solis,
Io Lunae, Io Terrae, Io Martis, Io Mercurii, Io Jovis, Io Veneris, Io
Saturni, and likewise Eu, Gan and Cal (Table 2-3). Titan uses the same names;
the page says a "Ti" prefix "might be added as needed", and this library does
not add it.

### Titan: 24 months in the Martian year

The year is the Martian vernal-equinox year of 686.9711 Earth days, which is
688.3006 circads. A common year of 688 circads is 24 months, of which the
third month of each quarter (Capricornus, Aries, Cancer and Libra) has 32
circads and the rest 28 — weeks split evenly across two months. The month
names are the Darian calendar's, Sagittarius to Vrishika (Table 3-1 and the
perpetual Table 3-2 of `gangale-titan`).

The page offers two intercalations and prefers the first:

* **The eight-circad system.** A leap year of 696 circads adds half a week
  to each of the 12th and 24th months, Rishabha and Vrishika, which become
  32 circads. The total intercalated through year `Y` is
  `8·(Y\25 − Y\400)`: every 25th year is a leap year except those divisible
  by 400, and year 0, "being counted as divisible by 400", is not
  (`gangale-titan`, §3.5 and §3.6).
* **The sixteen-circad system**, `16·(Y\50 − Y\800)`, which adds half a week
  to each of the 6th, 12th, 18th and 24th months so that every year begins
  at solar midnight. The page concludes that "the eight-circad
  intercalation system might be the better choice for Titan as well".

The page prints the mean years of the eight-circad system as
"(688 · 25 + 8) / 25 circads = 668.3200 circads" and
"(688.3200 · 400 − 8) / 400 circads = 668.3000 circads". The arithmetic gives
688.32 and 688.30, and the year it is fitted to is 688.3006 circads; this
library follows the arithmetic. The same slip recurs in "668.3006" two lines
later and is read the same way.

Every Titan year is a whole number of weeks, so every year begins on the
first circad of the week and the week runs on across months without a
break. The solar day does not: a leap year is 43⅓ solar days, so a year
that begins at solar midnight on the prime meridian ends at solar noon and
the common years after it begin at noon, until the next leap year swaps
them back. The page's Table 3-3 lists the position in the solar day
(0 midnight, 4 sunrise, 8 noon, 12 sunset) at which each month begins in
the four resulting kinds of year.

**The epoch.** The page calibrates against the superior conjunction of
Titan at 10.7 h UTC on 18 December 2002, JD 2 452 626.945 83, less than a
day after an opposition of Saturn, when "it was solar noon on the prime
meridian of Titan" (`gangale-titan`, §3.6, citing the *Astronomical
Almanac* for 2002, not read). Counting circads back to the Martian year it
reaches Julian Circad 144 096 at the conjunction, year 209 beginning at
Julian Circad 143 856 on JD 2 452 387.409 40 (22 April 2002, 21:49:32 UTC),
and Julian Circad 0, which begins year 0, on JD 2 308 809.276 07
(15 March 1609, 18:37:32).

*Worked example* — the page's own: what was the Titan date at the
conjunction?

1. Year 209 began at Julian Circad 143 856, and 144 096 − 143 856 = 240, so
   the conjunction began the 241st circad of the year.
2. Year 209 is not a leap year (209 is not a multiple of 25), so the months
   run 28, 28, 32, 28, 28, 28, 28, 28 for Sagittarius to Mina: 228 circads.
3. 240 − 228 = 12, so the circad is the 13th of Aries: **13 Aries 209**,
   "209 Ari 13" on the page.
4. 144 096 = 8 × 18 012, so it is the first circad of a week, Solis; and
   30 weeks after a year that began at noon, it is noon again.

The count of circads before year 209 checks the leap rule: eight leap years
(25, 50, …, 200) lie in years 0–208, and 201 × 688 + 8 × 696 = 143 856.

### The Galilean moons: 13 months in the Earth year

The Gregorian-based family keeps the Earth year and the Gregorian year
number. Twelve months of 32 circads would give a year of about 34 circads a
month, so there are thirteen, with the Roman intercalary month *Mercedonius*
between Februarius and Martius, and a year of 408 or 416 circads — thirteen
four-week months, or the same with the last month a week short
(`gangale-jupiter`, §2.5, Tables 2-4 and 2-5). On Io, Europa and Callisto
the short month is the 13th, December, of 24 circads. Ganymede's year is
just under 408 circads, so its 7th month, Junius, always has 24 and its
year is normally 408, with an occasional 400-circad year in which December
is also cut. Every month is whole weeks, so every month begins on Solis.

The years of each moon follow Table 2-7, a ten-year sequence by the last
digit of the year:

| Last digit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | per 10 years |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Io | 416 | 416 | 408 | 416 | 408 | 416 | 416 | 408 | 416 | 408 | 4128 |
| Europa | 416 | 408 | 416 | 408 | 408 | 416 | 408 | 416 | 408 | 408 | 4112 |
| Ganymede | 408 | 408 | 408 | 408 | 408 | 408 | 408 | 408 | 408 | 408 | 4080 |
| Callisto | 416 | 416 | 408 | 416 | 416 | 416 | 416 | 408 | 416 | 416 | 4144 |

The page gives the residual error of each ten-year sequence against ten
solar years — 0.6425, 1.3333, 2.7162 and 1.8299 circads — and says further
errors "can be corrected by an extended intercalation scheme", Table 2-8.

**The epoch.** Each calendar is calibrated on the opposition of Jupiter of
1 January 2002, when the inferior conjunction of each moon with the Sun,
seen from Earth, marks midnight on its prime meridian. Table 2-9 gives the
conjunctions, corrected for light time, "all times UTC": Io 2001 Dec 31
16:07:45, Europa 2002 Jan 02 17:12:57, Ganymede 2002 Jan 01 11:08:29,
Callisto 2001 Dec 28 12:27:23, and "all of the dates and times in Table 2-9
correspond to 2002 January 01 00:00:00 in their respective timekeeping
systems" (`gangale-jupiter`, §2.7, citing NASA Ames' Jupiter Ephemeris
Generator 1.2, not read).

*Worked example.* When does Io's year 2003 begin? Year 2002 ends in 2, so
by Table 2-7 it is a 408-circad year, and its Io December has 24 circads.
An Io circad is 1.769 860 / 2 = 0.884 930 days, and 408 of them are
361.051 44 days, which from 2001 December 31 16:07:45 UTC reaches
**2002 December 27, 17:21:49 UTC**, the first circad of Io Januarius 2003
and a Solis.

## What is carried

In `hc-planetary`:

* `circad`: the shared machinery — a `CircadRule` of data (the body, the
  circads per solar day, the solar day, the epoch, the month names and
  lengths, the week names and the year rule) and one `CircadCalendar` that
  interprets any rule. It implements `hc_calendar::Calendar`, and in it, as
  in the Darian calendar for Mars, **the `Rd` is a circad number on that
  moon, not an Earth day**: a circad `Rd` must never be handed to a
  terrestrial calendar. `CircadCalendar::circad_at` and `date_at` cross from
  an `Instant<Tai>`; `circad_start` crosses back.
* `titan::DARIAN_TITAN`, `darian-titan`: the eight-circad system with the
  month table of Tables 3-2 and 3-3, `titan::is_leap_year`, and
  `titan::solar_day_position`, the position of a circad in the solar day on
  the prime meridian (0 midnight to 15), from the conjunction at noon. Its
  `Rd` is Gangale's Julian Circad.
* `galilean::GREGORIAN_IO`, `GREGORIAN_EUROPA`, `GREGORIAN_GANYMEDE` and
  `GREGORIAN_CALLISTO`: the Gregorian-based family with the ten-year
  sequences of Table 2-7. Their `Rd` counts circads from the first circad of
  2002, which is 0.

The circad count is uniform in Terrestrial Time. The page's instants are
UTC in 2002; they are read as UTC, with TAI − UTC = 32 s, and the count
runs uniformly from there. Gangale extends his Julian Days back to 1609
without a ΔT, and so does this library: the count is a physical clock, and
the printed 1609 time is what the uniform count gives, not a UT reading.

The month names are carried as Table 2-5 prints them, with the prefix:
*Io Januarius* … *Io December*. Table 2-5 prints *Eu Septembris* where the
other three moons have *September*, and the perpetual Table 2-6 prints
*Septembris*, *Octobris*, *Novembris* and *Decembris* for all four; the
naming table is carried as printed, and the other spellings are not.

Not carried:

* **Titan's sixteen-circad system.** The page offers it and prefers the
  eight-circad one; it would be a second calendar under its own name
  ([policy.md](../policy.md) §5), and is left until someone wants it.
* **Titan's later formula.** The page changes the rule to
  `8·(Y\25 − Y\600)` in the year 3600 to follow the lengthening vernal-equinox
  year, and presents it with its error figure "as an example of the accuracy
  that is achievable"; as with the Darian calendar's extended scheme, it is
  not the calendar's rule and is not carried. The eight-circad rule is
  applied to every year.
* **The Galilean extended scheme**, Table 2-8. Its formulas do not
  reproduce Table 2-7: Ganymede's `8·(Y\10 − Y\20)` would add a week in
  years ending in 10, 30, 50 … to a year Table 2-7 keeps at 408 throughout,
  where the text says the correction Ganymede needs is a 400-circad year;
  and Io's `8·[−Y\5 − (Y−3)\5 + …]` would shorten years ending in 0 and 5,
  which Table 2-7 makes 416. Without a statement of the base each formula
  adds to, the table cannot be read, and Ganymede's 400-circad year is
  therefore never produced.
* **The Darian-based Galilean family** (`gangale-jupiter`, §2.8). It is
  read — 24 Darian months of 32 circads, years of 760 to 784 circads,
  Table 2-16's ten-year sequences, Table 2-17's series of formulas and the
  1609 epochs of Table 2-18 — and not implemented, because the page
  contradicts itself where the dates depend on it. For Io and Callisto the
  text puts a 776-circad year's fifth week in the 24th month, Vrishika,
  while Table 2-11 prints the 12th month, Rishabha, as 40 circads and
  Vrishika as "32-40"; the perpetual Tables 2-12 and 2-15 show both months
  able to reach 40 without saying when. The page says a series of formulas
  "must be applied", but Table 2-17 prints Callisto's without the factor of
  eight the others carry and identically for both of its ranges of years.
  And the 1609 epochs do not follow from the 2002 conjunctions by Table
  2-1's periods: counting back gives times 7 to 75 minutes from Table 2-18's.
  The roadmap row says what would settle each.
* **The circads' times of day.** Table 2-3 labels each Galilean circad with
  a part of the solar day ("1st AM", "Before Dawn"), and Figure 2-1 draws
  them; the labels are descriptive and are not carried. The clock inside a
  circad — Gangale transplants 24 hours of 60 minutes — is the circad
  fraction `CircadCalendar::circad_fraction` returns.

## Accuracy

The calendars are exact arithmetic on the page's constants; what can be
wrong is the constants.

* **Titan's circad.** 15.969 095 / 16 is 0.998 068 437 5 days; the page
  prints and calculates with 0.998 068 439. The library carries the printed
  value, which is 0.13 ms a circad longer, 19 ms over the 144 096 circads
  from 1609 to the calibration.
* **Titan's epoch.** The page's chain of rounded Julian Days gives Julian
  Circad 0 on JD 2 308 809.276 07; counting 144 096 printed circads back from
  the conjunction gives JD 2 308 809.276 04, 2.3 s earlier. The library
  anchors the count at the conjunction, where the page states it, and the
  test checks that the printed value is within 3 s of that and that Julian
  Circad 143 856 begins within a second of the printed 21:49:32.
* **The Galilean circads.** The page's Table 2-4 gives the circads in an
  Earth year from the same solar days, 412.7358, 411.0667, 407.7284 and
  414.2170; Table 2-1's days with its 365.242 38-day year give 412.7359,
  411.0667, 407.7284 and 414.2170. The solar days agree with those `bodies`
  derives from the NSSDC periods to better than 4 parts in 10⁷ for the
  Galilean moons and 1 part in 10⁸ for Titan.
* **The ten-year sequences drift.** Table 2-7 is the page's first
  approximation, and its residuals are real: over ten Earth years the
  calendars run 0.64 (Io) to 2.72 (Ganymede) circads long. By this
  library's arithmetic 1 Gan Januarius 2100 falls on 25 January 2100 and
  1 Gan Januarius 2500 on 2 May 2500. The page expects the extended scheme to
  correct this; it is not carried (above), so the drift is the calendar as
  specified.
* **Titan's drift**: the eight-circad rule's 688.3 circads against the
  688.3006-circad year is one circad in 1700 years, before the lengthening of
  the Martian year the page describes.

Checked against:

* The Titan worked example: the conjunction of 2002-12-18 is 13 Aries 209,
  Julian Circad 144 096, on a Solis at solar noon; year 209 begins at
  Julian Circad 143 856 on 22 April 2002 near 21:49:32 UTC; the page's
  201 × 688 + 8 × 696 = 143 856.
* Titan's month tables: every month length of Table 3-2's common and leap
  years, and every entry of Table 3-3's solar positions for the four kinds
  of year, found among the years the rule produces.
* The Galilean epochs: each moon's first circad of 2002 begins at its
  Table 2-9 conjunction to within a millisecond, and is 1 Januarius 2002, a
  Solis.
* The structure: Table 2-7's per-decade totals, Table 2-5's names and month
  lengths, and Ganymede's Junius of 24 circads in Table 2-6.

No date in any of these calendars other than the page's own calibrations was
found published, so nothing else could be checked.

## Sources

| Key | What it was used for | Read |
| --- | --- | --- |
| `gangale-titan` | Titan's solar day and circad, the months, both intercalations, the phasing tables, the calibration and epoch; Table 3-2 at `t2003darian_titan.htm` | yes, retrieved 2026-09-26 |
| `gangale-jupiter` | The Galilean circads and week names, both families' months and intercalations, the 2002 and 1609 epochs; Table 2-6 at `t1998jup_gregorian.htm`, Tables 2-12 to 2-15 at `t1998jup_io.htm` … `t1998jup_ca.htm` | yes, retrieved 2026-09-26 |
| `astronomical-almanac-2002` | The opposition of Saturn and the superior conjunction of Titan the Titan page cites | no |
| — | NASA Ames' Jupiter Ephemeris Generator 1.2, from which Table 2-9 was computed | no |

## Code

* `crates/hc-planetary/src/circad.rs`: the engine. Tests:
  `every_rule_round_trips_through_circad_numbers`,
  `the_month_lengths_add_up_to_the_year_lengths`,
  `the_calendar_trait_round_trips_and_rejects_what_does_not_exist`.
* `crates/hc-planetary/src/titan.rs`: `darian-titan`. Tests:
  `the_published_calibration_reproduces`,
  `the_leap_rule_puts_eight_leap_years_before_year_209`,
  `the_mean_year_is_688_3_circads_as_the_arithmetic_gives`,
  `the_month_tables_are_table_3_2`,
  `the_solar_phasing_is_table_3_3`,
  `the_circad_agrees_with_the_bodies_table`.
* `crates/hc-planetary/src/galilean.rs`: the Gregorian-based family. Tests:
  `each_calendar_begins_2002_at_its_table_2_9_conjunction`,
  `the_ten_year_sequences_are_table_2_7`,
  `the_months_are_table_2_5`,
  `io_2003_begins_on_the_worked_example_day`,
  `the_circads_agree_with_the_bodies_table`.
