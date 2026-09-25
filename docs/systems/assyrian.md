# The modern Assyrian calendar and its 4750 BC epoch

Backs the identifier `assyrian` in `hc-calendars-solar`.

## What it is

The calendar of the modern Assyrian community, in Iraq, Iran, Syria and
the diaspora, on which the new year *Kha b-Neesan* — the first of Nisan,
the revived *Akitu* — is kept on 1 April and the years are counted from
4750 BC: 1 April 2026 opened the year 6776 [syriacpress2026, aina2001]. It
is the Gregorian year with its months renamed after the Syriac months,
which carry the Akkadian names of the Babylonian calendar, and begun in
April rather than January.

The reckoning was made in the 1950s. A series of articles in the Assyrian
nationalist magazine *Gilgamesh*, published in Tehran, argued for a
national calendar: Nimrod Simono wrote on the Akitu festival in 1952, and
Jean Alkhas, in the April 1955 issue, number 34, fixed 4750 BC as the
epoch, on the word of a French archaeologist he did not name who had told
him a cuneiform tablet of that year recorded the calming of the great
flood and the beginning of life [wikipedia-assyrian-calendar, citing
paulissian1999 and daniel2001]. The Assyrian International News Agency
gives the epoch as "the date of the building of the first temple of Ashur"
[aina2001]. Neither claim has an archaeological basis; the date is a
construction, and it has been the community's calendar since.

It is not the calendar of the ancient Assyrians, who named each year for
its *limmu* eponym official and kept lunar months; nor is it the Seleucid
era, from 312 BC, in which Syriac-speaking Christians, Assyrians among
them, long dated documents and which some still use
[wikipedia-assyrian-calendar, citing coakley2013].

## How it works

**The year.** AINA states the rule: "In the Gregorian calendar, the
Assyrian year is 4750 + the Gregorian Year. Example: 4750 + 2012 A.D. =
6762 Assyrian Year (A.Y.). This is true if the date is after April 1,
before that the year is one less (i.e., 6761). This is because the
Assyrian year begins on April 1" [aina2001]. So the year *Y* runs from
1 Neesan, Gregorian 1 April of *Y* − 4750, to 31 Adar, Gregorian 31 March
of *Y* − 4749.

**The months.** AINA's table, in the calendar's order from Neesan:

| Month | Name | Gregorian month | Days |
| --- | --- | --- | --- |
| 1 | Neesan | April | 30 |
| 2 | Yar | May | 31 |
| 3 | Khzeeran | June | 30 |
| 4 | Tammuz | July | 31 |
| 5 | Tabakh (Ab) | August | 31 |
| 6 | Eelool | September | 30 |
| 7 | Tishrin Qamaya | October | 31 |
| 8 | Tishrin Treyana | November | 30 |
| 9 | Kanoon Qamaya | December | 31 |
| 10 | Kanoon Treyana | January | 31 |
| 11 | Eshwat | February | 28 or 29 |
| 12 | Adar | March | 31 |

*Qamaya* and *Treyana* are "first" and "second": the two Tishrins and the
two Kanoons are the double months of the Syriac calendar. Each month has
the days of the Gregorian month it is, so the leap day is 29 Eshwat, and
the year *Y* is long when the Gregorian year *Y* − 4749, the one holding
its February, is a leap year.

**Worked example.** What is the Assyrian date of 25 September 2026?
September is after 1 April, so the year is 2026 + 4750 = 6776; September is
the sixth month from April, Eelool; so 25 Eelool 6776. And 15 January 2027?
January is before 1 April, so the year is 2027 + 4750 − 1 = 6776 still;
January is the tenth month, Kanoon Treyana; so 15 Kanoon Treyana 6776. The
year 6776 ends on 31 Adar, 31 March 2027, and 6777 opens on 1 April 2027.

## What is carried

- **Identifier** `assyrian`, with the year, the month 1–12 from Neesan and
  the day, under the era code `AY`, and the extra field `gregorian-year`.
  The month names are declared with the shape in AINA's forms; the Syriac
  script and the scholarly transliterations (Nīsān, ʾĪyār, Ḥzīrān, Tammūz,
  ʾĀb, ʾĪlūl, Tešrīn Qḏīm and ʾḤrāy, Kānōn Qḏīm and ʾḤrāy, Šḇāṭ, ʾĀḏar) are a
  locale's.
- **Year number** the Gregorian year plus 4750 from 1 April and one less
  before it, `EpochForward` from year 1 in 4750 BC. `usage` begins on
  1 Neesan 6705, 1 April 1955, the Kha b-Neesan of the article that fixed
  the epoch; earlier years report `Standing::Proleptic`.
- **Range** year 1 to 999 999, 1 Neesan 1 (1 April 4750 BC, proleptic
  Gregorian) to 31 Adar 999 999.
- **Not carried:**
  - *The ancient Assyrian calendar* of *limmu* years and lunar months,
    which the roadmap lists separately and which is a reconstruction.
  - *The Seleucid era*, which is a year count on the same months and
    would be its own identifier if a source were read for its use.
  - *The Akitu festival's length and rites*, which are an observance.

## Accuracy

The reference is the Gregorian calendar and the published equivalences:

| Check | Test | Result |
| --- | --- | --- |
| 1 April 2026 opened 6776 [syriacpress2026]; 2012 was 6762 [aina2001]; 1 April 2019 was 1 Neesan 6769 | `kha_b_neesan_is_the_first_of_april_and_the_year_is_gregorian_plus_4750` | 3 of 3 |
| 1 January 2026 is 1 Kanoon Treyana 6775 and 31 March 2026 the last day of 6775 [aina2001] | `january_to_march_belong_to_the_year_that_began_the_previous_april` | all |
| Every month has its Gregorian month's days; 6773 (February 2024) is long, 6774 short, 6749 (2000) long, 6649 (1900) short | `the_months_are_the_gregorian_ones_from_april_and_eshwat_takes_the_leap_day` | all |
| The epoch is 1 April 4750 BC; the reckoning dates from 1955 | `the_epoch_is_4750_bc_and_the_reckoning_dates_from_1955` | all |
| Every sampled day of a 3.5-million-day span is the Gregorian day under the mapped month and year; every day of 6770–6777 round-trips | `every_day_of_a_wide_range_round_trips`, `every_single_day_of_a_leap_cycle_round_trips` | 36 083 and 2 922 days |

There is no published table to disagree with. The one thing the sources
leave open is whether every community counts January to March in the
lower year; AINA's statement is the only explicit one read, and the
calendar follows it.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [aina2001] | The epoch, the year rule including January to March, 1 Neesan on 1 April, the month names | Yes, 2026-09-25, over http |
| [wikipedia-assyrian-calendar] | The *Gilgamesh* articles, the French archaeologist, the Seleucid era, the Syriac month table | Yes, 2026-09-25 |
| [syriacpress2026] | 1 April 2026 as 6776 | Yes, 2026-09-25 |
| [paulissian1999] | The 1952 and 1955 articles | Not read; cited by Wikipedia |
| [daniel2001] | Alkhas's archaeologist | Not read; the copy at learnassyrian.com is gone |
| [coakley2013] | The Syriac month names, the Seleucid era | Not read; cited by Wikipedia |

Paulissian's article is the primary account of the calendar's making, and
reading it would let the 1955 date be cited directly.

## Code

`crates/hc-calendars-solar/src/assyrian.rs`. Anchors:
`kha_b_neesan_is_the_first_of_april_and_the_year_is_gregorian_plus_4750`,
`january_to_march_belong_to_the_year_that_began_the_previous_april`,
`the_epoch_is_4750_bc_and_the_reckoning_dates_from_1955`. The month names
are the module's `MONTHS`, the offset `YEAR_OFFSET`, the era `ERA`.
