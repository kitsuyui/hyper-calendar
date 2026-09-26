# The Yerm lunar calendar: months of 30 and 29 nights in yerms of 17 and 15

Backs the identifier `yerm` in `hc-calendars-lunar`.

## What it is

A rule-based lunar calendar Karl Palmen devised in February 1998, which
"abandons any pretence to follow the seasons, allowing the length of the
'year' to be modified to suit a simple and accurate tracking of the lunar
month" [palmen-yerm]. The unit above the month is the *yerm*, from "YEaR
Moon", which has nothing to do with the solar year. Palmen notes that
Isaac Newton, around 1700, sketched a lunar calendar for reckoning Easter
with the same 17, 15, 17 grouping of months [palmen-yerm]. It has been
adopted by nobody.

## How it works

**The rules**, as Palmen states them [palmen-yerm]:

1. "In each yerm, the odd numbered months have 30 nights and the even
   numbered months 29 nights."
2. "In each cycle, the yerms have 17 months, except those whose number is
   divisible by 3, which have 15 months."
3. "Each cycle has 52 yerms and the present cycle began at noon on 11
   November 1996 in the Gregorian Calendar."

A yerm therefore begins and ends with a 30-night month, and a new yerm
begins wherever two 30-night months meet. A 17-month yerm is 502 nights and
a 15-month one 443; three yerms, 17, 17 and 15, are 49 months and 1447
nights, "exactly 14 days short of four years with a leap day", and a 17- and
a 15-month yerm together are exactly 135 weeks [palmen-yerm]. Seventeen such
groups are 51 yerms; the fifty-second, of 17 months, is "an additional 17
month yerm ... inserted after every 17 of these basic three-yerm cycles"
[palmen-yerm], because 52 is not divisible by 3. A cycle is 850 months and
25 101 nights, a mean month of 29.530 588 2 days. Stated another way, and
independently: counting months from the first of a cycle's last yerm as 0,
month m has 30 nights when m·451 mod 850 < 451
[palmen-mpslc-properties].

**The day.** "The dates (or nights) of the calendar begin at 12 midday
clock time, so that the night is not interrupted by a date change"
[palmen-yerm].

**The count.** Palmen numbers the cycles from Julian Day 1 948 379, noon of
16 May 622 Julian, "just two months before the start of the Islamic AH
era", so that the present cycle is cycle 21, and gives the conversion from
a Julian Day as a chain of divisions by 25 101, 1447, 502, 59 and 30, each
taking the quotient and keeping the remainder [palmen-yerm]. He writes
dates as cycle-yerm(month(night, "21-05(03(30".

**Worked example.** Palmen's own: the afternoon of 10 June 2002. Its Julian
Day is 2 452 436, 504 057 nights after the epoch. 504 057 = 20·25 101 +
2037: cycle 21. 2037 = 1·1447 + 590: yerm 1 + 3·1 = 4. 590 = 1·502 + 88:
yerm 5. 88 = 1·59 + 29: month 1 + 2·1 = 3. 29 = 0·30 + 29: month 3,
night 29 + 1 = 30. That is 21-05(03(30, as Palmen gives it, the last night
of a 30-night month.

## What is carried

- **Identifier** `yerm`: the date's year is the yerm counted continuously
  from Palmen's epoch, 52·(cycle − 1) + yerm, so 21-05 is yerm 1045;
  `cycle` and `yerm-of-cycle` are fields beside it, and
  `YermDate::from_cycle` takes the four numbers as Palmen writes them.
  There is no era.
- **Months** 1 to 15 or 17, numbered; English names them "Month 1" to
  "Month 17", as Palmen writes "Month 2". The shape declares fifteen months
  with seventeen in some yerms, and the week.
- **The day boundary is noon**, `DayBoundary::Noon(DayNaming::ByStart)`:
  a yerm date maps to the civil day on whose noon it begins: the night that begins at noon on 11 November 1996
  is fixed day 11 November 1996, and the morning of 12 November still
  belongs to it. This is how Palmen's tables give the dates ("begins noon
  2016-09-02") and how the Julian Day Number maps its own noon-to-noon
  days onto fixed days, so Palmen's Julian Day arithmetic carries over
  unchanged [palmen-yerm]. A caller converting a wall-clock time before
  noon takes the day before, which `DayBoundary::civil_day_offset` gives
  as −1.
- **`is_leap_year`** is true for the fifty-second yerm of a cycle, the one
  Palmen inserts to correct the mean month; a 17-month yerm is not leap,
  since two in three have 17 months.
- **Range** cycle 1 to cycle 150, noon of 16 May 622 Julian to AD 10 931;
  Palmen's cycle count starts at 1 and says nothing of earlier cycles.
  `usage` is unrecorded: a proposal.
- **Not carried:** the lunar week of Moonnight to Soonnight Palmen used in
  the first months, the full-moon weekend, the "natural yerm" and the
  59-yerm variant he considers, and the Basic Yerm Calendar of 17, 17, 15
  without the fifty-second yerm. The calendar makes no claim about the
  seasons and is not tested against them; nor is it tested against the
  moon, where Palmen's table gives the hours by which each month starts
  after the dark moon.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| 11 November 1996 as 21-01(01(01 and 2002-06-10 pm as 21-05(03(30 | `the_present_cycle_and_palmens_worked_date` | both |
| 2002-06-10 at 13:00 and 2002-06-11 at 11:00 are both 21-05(03(30; 2002-06-10 at 11:00 is the night before | `a_night_is_named_by_the_civil_day_on_whose_noon_it_begins` | all 3 |
| The new cycles 17 to 22, 1721-12-19 to 2065-08-02, and their weekdays | `the_cycles_begin_where_palmen_lists_them` | all 6 |
| The new yerms 20-25 to 21-24 and their weekdays | `the_new_yerms_are_the_ones_palmen_tabulates` | all 52 |
| The first days of the months of yerms 21-16 to 21-18 in the moon table | `the_months_of_yerms_16_to_18_begin_where_palmen_lists_them` | all 49 |
| 52 yerms, 850 months, 25 101 nights; 1447 nights to three yerms, 135 weeks to a 17 and a 15 | `a_cycle_is_52_yerms_850_months_and_25_101_nights`, `the_yerm_arithmetic_palmen_remarks_on` | all |
| The month lengths against Palmen's second statement, m·451 mod 850 < 451, over three cycles | `the_long_months_follow_palmens_451_in_850` | all 2550 months |
| Every night of two cycles, and the whole range in steps, round-trips | `every_night_of_two_cycles_round_trips`, `the_whole_range_round_trips_and_its_edges_are_refused` | all |

The module's conversion from a fixed day is Palmen's algorithm, and the
conversion to one his inverse; the tables above were built by Palmen
separately, from the rule that three yerms are two weeks short of four
Julian years, and checked by him with a date converter, so the agreement is
between his algorithm and his tables.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [palmen-yerm] | The rules, the noon boundary, the epoch and cycle numbering, the conversion algorithm, the tables of months, yerms and cycles, the worked date | Yes, 2026-09-26 |
| [palmen-mpslc-properties] | The 451-in-850 statement of the month lengths | Yes, 2026-09-26 |

The online converter at the-light.com that the page names was not used.

## Code

`crates/hc-calendars-lunar/src/yerm.rs`. Anchors:
`the_new_yerms_are_the_ones_palmen_tabulates`,
`the_months_of_yerms_16_to_18_begin_where_palmen_lists_them`,
`the_long_months_follow_palmens_451_in_850`. The rule is
`months_in_yerm`; the yerm boundary `new_yerm`.
