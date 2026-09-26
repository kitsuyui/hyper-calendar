# The Archetypes Calendar: two remainders over an 1 803-year period, and the tweek

Backs the identifier `archetypes` in `hc-calendars-lunar`.

## What it is

An arithmetic lunisolar calendar by Peter Meyer, "an accurate lunisolar
calendar with connections to the Chinese Calendar": its months are meant to
begin at the dark moon, and its New Year's Day to fall where the Chinese
calendar's does, between 21 January and 21 February, which Meyer reports
it does on the same Gregorian date in 71.8% of the years 1900–2399 and
within a day in most of the rest [meyer-archetypes]. Its months are named
for the deities of the seven classical planets and five later ones, and its
week is replaced by the ten-day *tweek*. Nobody adopted it.

## How it works

**Months and years.** Odd months have 30 days and even ones 29 — Apollo,
Diana, Hermes, Aphrodite, Ares, Zeus, Chronos, Prometheus, Orpheus, Sophia,
Dionysus, Demeter — and a *long* year adds Persephone, of 30. In a *leap*
year Sophia has 30 days instead of 29. A year is therefore 354, 355, 384 or
385 days [meyer-archetypes].

**The two rules.** Years run in ARC periods of Y = 1 803; the year y has
position `p = ((y + 1360) mod Y) + 1`, and with L1 = 664 and L2 = 350 it is
long when `(p · L1 + (Y − 1)/2) mod Y < L1`, `(664p + 901) mod 1803 < 664`,
and leap when `(350p + 901) mod 1803 < 350`. A period has 664 long years,
350 leap years, 22 300 months and 658 532 days — a mean year of 365.24237
days and a mean month of 29.530583, and 94 076 whole weeks — and its year
types are symmetrical about the middle year [meyer-archetypes].

**The correlation.** "The ARC years 443 through 2245 constitute an ARC
period", and 443-1-1 is Julian Day Number 897 474, −2255-02-05 in the
proleptic Gregorian calendar; 1-1-1 is JDN 736 030 [meyer-archetypes].

**The tweek.** Each month is three tweeks, the first two of ten days and
the third of nine or ten, their days Sun Day, Mercury Day, Venus Day, Earth
Day, Mars Day, Jupiter Day, Saturn Day, Uranus Day, Neptune Day and Pluto
Day, so that the day's name is the last digit of the day of the month
[meyer-archetypes].

**Worked example.** What is 3 February 2011? The rule's form makes the
years of each kind a count: from year 443, the long years before the year
443 + k are `⌊(664k + 901) / 1803⌋` and the leap years
`⌊(350k + 901) / 1803⌋`. For 4709, k = 4 266: 1 571 long years and 828
leap years, so 4709 begins 354 · 4 266 + 30 · 1 571 + 828 = 1 558 122 days
after JDN 897 474, on JDN 2 455 596 — 3 February 2011, `4709-01-01`, a Sun
Day, as Meyer's table gives it. Its position is 661; 664 · 661 + 901 leaves
1 676 on division by 1 803 and 350 · 661 + 901 leaves 1 467, so it is
neither long nor leap, 354 days, and 4710 begins on 23 January 2012.

## What is carried

- **`archetypes`**: year, month and day, Persephone as month 13 with the
  shape's month cycle twelve or thirteen long and named as Meyer names
  them; the tweek and its day as the `tweek` and `day-of-tweek` fields and
  the `tweek-day` cycle, named. `is_leap_year` answers Meyer's *long* year,
  the one with the intercalary month; his *leap* year, with a 30-day
  Sophia, is `has_long_sophia`. Years −4 966 to 9 457, three periods
  before the anchored one and four after it.
- **Not carried:** Meyer's comparisons with the dark moon and the Chinese
  calendar, which measure the rules against the sky and against another
  calendar and are his, not re-measured here.
- `usage` is unrecorded: a proposal.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| 443-1-1 is JDN 897 474, −2255-02-05; 1-1-1 is JDN 736 030, −2697-01-30 | `the_correlation_is_meyers` | exact |
| The worked example of 4300: position 252, long, not leap | `the_worked_example_of_4300` | exact |
| The three runs of dated days of 2010, 2011 and 2012, with their JDNs and day names | `the_three_runs_of_dated_days` | all |
| The table of New Year's Days for 4699 to 4755, with its long and leap years | `the_new_years_of_4699_to_4755` | all 57 |
| 4400-01-01, 5400-12-29 and 4709-09-24 on the Gregorian dates Meyer gives; New Year's Day on 20 January in 3195, and within 21 January to 21 February over 4300–5200 | `the_dates_of_the_chinese_comparison` | all |
| 664 long and 350 leap years, 22 300 months, 658 532 days and 94 076 weeks a period; the symmetry about the middle year | `the_properties_of_an_arc_period` | exact |

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [meyer-archetypes] | The definition, the names, the correlation, the dated tables and the properties | Yes, 2026-09-26 |

## Code

`crates/hc-calendars-lunar/src/archetypes.rs`. Anchors:
`the_correlation_is_meyers`, `the_three_runs_of_dated_days`,
`the_new_years_of_4699_to_4755`. The year boundary is `new_year`; the two
rules `is_long_year` and `has_long_sophia`.
