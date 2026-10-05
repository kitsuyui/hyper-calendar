# The Tranquility calendar: thirteen months from the Moon landing, in Omni, 1989

## What it is

The Tranquility calendar is a perennial calendar that Jeff Siggins
described in *Omni* in July 1989. It has thirteen months of 28 days named
for scientists in alphabetical order, Archimedes to Mendel; its year begins
on 1 Archimedes, 21 July in the Gregorian calendar; and its years count
from Moon Landing Day, 20 July 1969, the years After Tranquility (A.T.).
Nobody adopted it. The article was read in a transcription at
mithrandir.com, whose certificate has expired, and so through the Internet
Archive's copy [siggins1989-tranquility]; the printed issue was not read.

## How it works

The article: the thirteen months make "a total of 364 days. One extra day is
added on at the end of the year to make 365. For leap years, a second extra
day is added." The last day of each year is **Armstrong Day**, the
anniversary of Moon Landing Day, 20 July; the leap day is **Aldrin Day**,
which "falls between the twenty-seventh and twenty-eighth of Hippocrates
(February 29)", every four years less "the leap days in every 400 years
that are dropped", the Gregorian rule. "The first of every month is a Friday
and the eleventh a Monday", so the two added days lie outside the week.
Moon Landing Day "stands alone. Not part of any month or year"; year 1 A.T.
begins the next day.

Worked example. 1 Archimedes 1 A.T. is 21 July 1969, a Monday in the
unbroken week and the calendar's own Friday. Armstrong Day 1 A.T. is 20
July 1970, and Armstrong Day 20 A.T. is 20 July 1989, as the article says.
3 A.T. holds February 1972, a leap year: 27 Hippocrates is 28 February,
Aldrin Day is 29 February and 28 Hippocrates is 1 March. 131 A.T. holds
February 2100, one of the dropped centennial leap days, and has no Aldrin
Day.

## What is carried

- **`tranquility`**, years 1 to 99 999 A.T. under the era code `at`, from 1
  Archimedes 1 A.T. Armstrong Day is numbered 29 Mendel and Aldrin Day 29
  Hippocrates, the only way they fit a year-month-day shape (as
  `international-fixed` numbers Year Day), and are flagged
  `outside-the-week` by `to_fields`. The date does not derive an ordering,
  because in a leap year's Hippocrates 29 falls before 28.
- The usage is unrecorded: a proposal that nobody adopted.
- Not carried, the article being the source: the time before Tranquility,
  which the article names Before Tranquility and numbers no year of, so
  Moon Landing Day and every day before it are refused as before the epoch;
  and the Tranquility clock, the hours Before and After Tranquility, which
  the article gives and which is not yet done.

## Accuracy

Exact: the article's rules are the definition, and the leap rule is the
Gregorian one applied to the February each year contains. Anchored to the
article's own dates: 20 July 1970 is Armstrong Day 1 A.T. and 20 July 1989
Armstrong Day 20 A.T. (`the_articles_dates`); the first of each month is a
Friday and the eleventh a Monday
(`the_first_of_every_month_is_a_friday_and_the_eleventh_a_monday`); and
every day round-trips. The weekday is the calendar's own, not the unbroken
week's, so a date's `weekday()` is none for the two added days.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [siggins1989-tranquility] | Every rule and quotation above | Yes, the transcription through the Internet Archive's copy of 16 May 2022, 2026-10-04 and 2026-10-05; the printed issue was not read |

## Code

`crates/hc-calendars-solar/src/tranquility.rs` (`MONTHS`, `ERA`,
`ARMSTRONG_DAY`, `ALDRIN_DAY`, `GREGORIAN_OFFSET`, `TranquilityCalendar`),
over the shared `common::perennial_*` 13 × 28 arithmetic and
`common::gregorian_year_begun_on`. Anchors: `the_articles_dates`,
`the_first_of_every_month_is_a_friday_and_the_eleventh_a_monday`,
`the_fields_carry_the_era_and_the_flag`, `every_day_round_trips`.
