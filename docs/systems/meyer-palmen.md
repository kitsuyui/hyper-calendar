# The Meyer–Palmen Solilunar Calendar: two remainders and a sixty-year cycle

Backs the identifier `meyer-palmen` in `hc-calendars-lunar`.

## What it is

A lunisolar calendar proposed by Peter Meyer in March 1999, built on a
general rule for long years that Karl Palmen had posted to the CALNDR-L
mailing list on 24 February 1999 [meyer-mpslc]. Its months are lunations
and its thirteenth month keeps New Year's Day near the March equinox, as
in the Hebrew calendar, but nothing in it is observed or computed from the
sky: two remainders over fixed constants decide every year. Palmen calls
the class it belongs to the "YLM" calendars, after the three constants
[palmen-mpslc-properties]. It has been adopted by nobody; its sources are
its authors' pages on Meyer's site, Hermetic Systems.

## How it works

**The date.** Four numbers, cycle-year-month-day: "the day number ranges
from 1 through 31, the month number ranges from 1 through 13, the year
number ranges from 1 through 60 and the cycle number is an integer"
[meyer-mpslc]. A typical date is 89-49-06-09.

**The months.** "All odd-numbered months have 29 days and all even
numbered months have 30 days, except that the 13th month in a long year
has either 30 or 31 days" [meyer-mpslc]. They are named Aristarchus,
Bruno, Copernicus, Dee, Eratosthenes, Flamsteed, Galileo, Hypatia,
Ibrahim, Julius, Khayyam, Lilius and Meton. A short year is 354 days, a
long year 384 or 385.

**The two rules.** With Y = 6840, L = 2519 and M = 1328 [meyer-mpslc]:

1. Year y of cycle c is long if and only if ((60·c + y)·L) mod Y < L.
2. A long year has a 31-day Meton if and only if
   (⌊(60·c + y)·L / Y⌋·M) mod L < M.

The quotient in rule 2 is n for the nth long year, counting the first long
year of cycle 000, year 03, as the first, so rule 2 is (n·M) mod L < M
[meyer-mpslc; palmen-mpslc-properties]. Both rules are the same device
twice: ⌊k·L/Y⌋ rises by one at exactly the long years, so it counts them,
and ⌊n·M/L⌋ counts the long Metons.

**The era and the epoch.** 114 cycles, 6840 years, are an *era*, after
which the structure repeats; an era has 2519 long years, 1328 of them with
a 31-day Meton, 84 599 months and 2 498 258 days, which is 356 894 weeks
exactly, and "000-01-01-01 MP ... corresponds to Julian day number
207,227 (-4145-04-08 CE)", a Sunday, so every era begins on a Sunday
[meyer-mpslc]. The mean year is 365.242 397 66 days and the mean month
29.530 585 468.

**Worked example.** Wednesday 11 August 1999, the total solar eclipse, is
Julian Day 2 451 402, 2 244 175 days after the epoch. The mean year puts
it in the continuous year 1 + ⌊2 244 175 · 6840 / 2 498 258⌋ = 6145,
which is 60·102 + 25: cycle 102, year 25. Before that year lie 6144 years,
⌊6144·2519/6840⌋ = 2262 of them long and ⌊2262·1328/2519⌋ = 1192 of those
with a 31-day Meton, so it begins 354·6144 + 30·2262 + 1192 = 2 244 028
days after the epoch, on 17 March 1999. The eclipse is 147 days into the
year: two pairs of months, Aristarchus to Dee, are 118 days, and the 29 of
Eratosthenes make 147, so it is the first day of the sixth month,
Flamsteed: 102-25-06-01, as Meyer gives it. The year itself is long, since
6145·2519 mod 6840 = 335 < 2519, and its Meton has 31 days, since it is
the 2263rd long year and 2263·1328 mod 2519 = 97 < 1328; Palmen's table
has the same two remainders, 335 and 97, and the length 385
[palmen-mpslc-properties].

## What is carried

- **Identifier** `meyer-palmen`: the date's year is the continuous count
  60·c + y that both rules take, so 102-25 is year 6145 and 000-01 is
  year 1; `cycle` and `year-of-cycle` are fields beside it, and
  `MeyerPalmenDate::from_cycle` takes the four numbers as Meyer writes
  them. The era code is `mp`, named "MP", Meyer's suffix.
- **Months** 1 to 12, and Meton as month 13 in a long year; the shape
  declares twelve months with a thirteenth, named as Meyer names them.
  `is_leap_year` is the long year.
- **Range** from the first year of the era before era 0, cycle −114
  (10 986 BC), to the last of era 2, cycle 341 (AD 16 375). The day begins
  at midnight: the sources give Julian Day Numbers and civil dates and no
  other boundary. `usage` is unrecorded: a proposal.
- **Not carried:** the Goddess Lunar Calendar and Denis Elliott's calendar,
  its relatives in Palmen's comparison, and the YLM variant with different
  L in the two rules that Palmen suggests [palmen-mpslc-properties].

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The three correspondence tables on Meyer's page, JDN 0–11, 2 415 016–2 415 025 and 2 488 341–2 488 349, every row | `meyers_correspondence_tables_agree_row_by_row` | all 31 rows |
| The listed dates: 1795-03-20 as 099-01-01-01, 1999-03-17, 1999-08-11, eras 1 and 2 on Sundays 2695-04-07 and 9535-04-07, 1999-04-29 as 102-25-02-15 | `the_dates_meyer_lists_among_the_properties` | all |
| Meyer's converter output for the year 102-25: 385 days, Meton 5 March to 4 April 2000 | `the_year_102_25_is_the_one_meyers_converter_lists` | all |
| Palmen's table for 102-25 to 102-44: both remainders, the lengths, the New Year's Days | `palmens_table_of_remainders_lengths_and_new_years` | all 20 years |
| An era's 6840 years, 2519 long, 1328 long Metons, 84 599 months, 2 498 258 days, whole weeks; the structure repeats by era | `an_era_has_the_years_months_days_and_weeks_meyer_states` | all |
| Meyer's frequency table of the 4001 New Year's Days of 0–4000 CE, 6 March to 7 April | `new_years_days_are_distributed_as_meyers_table_counts_them` | all 33 counts |
| Every day of a thousand years, and the whole range in steps, round-trips | `every_day_of_a_thousand_years_round_trips`, `the_whole_range_round_trips` | all |

Meyer's statement that "all New Year's Days" fall from 6 March to 7 April
is measured on 0–4000 CE and holds there; outside the Common Era it does
not, and is not claimed: New Year's Day 000-01 itself is 8 April.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [meyer-mpslc] | The definition: date, months and names, both rules and constants, the era, the base JDN; the properties and dates listed; the correspondence tables; the New Year's Day table | Yes, 2026-09-26 |
| [palmen-mpslc-properties] | The YLM class; the table of remainders, lengths and New Year's Days for 102-25 to 102-44 | Yes, 2026-09-26 |
| [meyer-mp102-25] | Every day of the year 102-25 from Meyer's converter | Yes, 2026-09-26 |

Palmen's CALNDR-L posts of 1 and 24 February 1999, which both pages
cite, were not read.

## Code

`crates/hc-calendars-lunar/src/meyer_palmen.rs`. Anchors:
`meyers_correspondence_tables_agree_row_by_row`,
`palmens_table_of_remainders_lengths_and_new_years`,
`new_years_days_are_distributed_as_meyers_table_counts_them`. The rules
are `is_long_year` and `has_long_meton`; the year boundary `new_year`.
