# The Pax calendar: thirteen months and a leap week, from Colligan's proposal

## What it is

The Pax calendar is a perennial calendar that James A. Colligan proposed in
1930, and that nobody adopted. Every year has whole weeks, 52 or 53, and
"the first day of every week, month and year would be Sunday". There are
thirteen months of 28 days: the eleven Gregorian months from January to November,
then *Columbus*, then December. In a leap year a seven-day month, *Pax*,
stands between Columbus and December. English Wikipedia's article "Pax
Calendar" gives the rule, and its tables of the Gregorian dates of Pax New
Year's Day [wikipedia-pax-calendar]. Colligan's own proposal of 1930 and his
*An unchangeable calendar without blank days* (University of San Francisco,
1933), which the article cites, were not read.

## How it works

The year carries the Gregorian number. The leap week is added "to 71 of the
400 years in the cycle": the years whose last two digits are divisible by
six (00 included) or are 99, unless the year is divisible by 400, which is
the Gregorian mean year. A common year has 364 days and a leap year 371.
A year begins on the Sunday within nine days of the Gregorian 1 January,
from 18 December to 6 January, so a year's New Year's Day is the previous
year's plus 364 days, or 371 if the previous year was a leap year. The months are
numbered 1 to 14, with Pax month 13 and December month 14 in every year, so
that each month keeps one number and one name; month 13 exists only in a
leap year.

Worked example. Pax 2026 begins on Sunday 28 December 2025. 2026 is not a
leap year (26 is not divisible by six), so Pax 2027 begins 364 days later,
on 27 December 2026, as the article's table has it. Pax 2006 is a leap year: it begins on 25 December 2005 and
has 371 days, so Pax 2007 begins on 31 December 2006. Within Pax 2006
Columbus, month 12, begins on 29 October 2006, Pax on 26 November, a week
long, and December, month 14, on 3 December.

## What is carried

- **`pax`**, years 1 to 99 999, counted by the rule in both directions from
  the anchor, Sunday 1 January 1928, the first row of the article's table; the rule speaks of a year's last two digits, so it is read
  for the positive years only. Asking for month 13 in a common year is a
  `month-out-of-range` refusal.
- The usage is unrecorded: the calendar is a proposal adopted by nobody.
- Not carried: Colligan's own statement of the rule, which was not read and
  would replace the article's; a Pax weekday that differs from the unbroken
  week's, since every year begins on a real Sunday.

## Accuracy

Exact: the rule as the article quotes it is the definition. Anchored to all
171 New Year's Days of the article's three blocks of tables, 1928 to 2054,
2091 to 2112 and 2291 to 2312, each of which falls on a Sunday and agrees
with the library; the test `the_published_new_years_days` carries all 171
rows, and `the_leap_rule_is_colligans` holds the 71 leap years in 400 and
`every_year_begins_on_a_sunday_near_the_gregorian_new_year` the Sunday and
the 18 December to 6 January range in every year. The article is a
secondary source; no printed source for the rule was read.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wikipedia-pax-calendar] | The months, the leap rule, the Sunday, and the 171 Gregorian dates of Pax New Year's Day | Yes, 2026-10-04; as wikitext 2026-10-05; secondary |
| Colligan, the proposal of 1930, and *An unchangeable calendar without blank days*, 1933 | The rule at first hand | Not read |

## Code

`crates/hc-calendars-solar/src/pax.rs` (`MONTHS`, `PAX`, `DECEMBER`,
`is_leap_year`, `new_year`, `PaxCalendar`), over the shared
`common::perennial_*` 13 × 28 arithmetic. Anchors:
`the_published_new_years_days`, `the_leap_rule_is_colligans`,
`every_year_begins_on_a_sunday_near_the_gregorian_new_year`,
`pax_exists_only_in_a_leap_year`, `every_day_round_trips`.
