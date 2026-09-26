# The lectionary cycles: Years A, B and C, Years I and II, and the Propers

Backs `hc-holiday`'s `lectionary` module.

## What it is

A lectionary appoints the passages of scripture read at each service. The
Roman Catholic Lectionary for Mass of 1969, the *Ordo Lectionum Missae*,
arranged the Sundays in three years, "designated A, B, or C", each built
round one synoptic gospel, Matthew, Mark and Luke, and the weekdays in a
two-year cycle, "Cycle I and Cycle II" [wikipedia-lectionary;
wikipedia-ordo-lectionum-missae]. The Revised Common Lectionary of 1992,
which the Consultation on Common Texts prepared for Protestant and
Anglican churches, keeps the Roman Sunday cycle and its calendar: "The
Revised Common Lectionary and its earlier edition of 1983 continue the
pattern of the Roman Catholic Lectionary for Mass of 1969" [cct-rcl,
§19]. The RCL names the Sundays after Trinity Sunday by numbered Propers,
Proper 3 to Proper 29, with the Roman numbering of the Sundays in
Ordinary Time beside them in brackets [cct-rcl, table].

Which readings fall on a day is therefore a question with two parts: which
year of the cycle it is, which is arithmetic, and which Sunday of the year
it is, which the calendar decides. This library answers both and carries
no readings, which are the churches' copyrighted texts.

## How it works

**When a year begins.** Every cycle turns at the First Sunday of Advent,
the "Sunday between November 27 and December 3" [cct-rcl, table]. A
liturgical year is named here by the civil year that holds its Easter, as
the Liturgy Office of England and Wales heads its table of moveable feasts
[liturgyoffice-moveable]: the year from 30 November 2025 to 28 November
2026 is 2026.

**The Sunday cycle.** "Year A always begins on the First Sunday of Advent
in years which can be evenly divided by three (e.g., 1992, 1995, etc.)"
[cct-rcl, §8]. So a liturgical year *L* is Year A when *L* − 1 is
divisible by three, B when the remainder is 1, C when it is 2.

**The weekday cycle.** "Odd-numbered years are Cycle I; even-numbered ones
are Cycle II" [wikipedia-lectionary]. Which year is meant — the civil year
of a date, or the liturgical year — the sentence does not say. The Liturgy
Office's table does: it heads 2026 with "A" and "II", and gives 30 November
2025 as the Advent that begins it [liturgyoffice-moveable]. So the cycle
is the parity of the liturgical year, and it turns at Advent with the
Sunday cycle.

**The Propers.** The RCL's calendar dates Proper 29, Christ the King, as
the "Sunday between November 20 and November 26", and Propers 3 to 28 as
the "Second through Twenty-Sixth Sunday after Pentecost" before it; "When
Easter is as early as March 22, the numbered Proper for the Sunday
following Trinity Sunday is Proper 3" [cct-rcl, table and its note]. So a
Sunday after Trinity Sunday is Proper 29 less the weeks it falls before
Proper 29. The bracketed Ordinary Time number is the Proper plus five, and
the Liturgy Office's dates for the Roman Sundays in Ordinary Time — the
8th on 22–28 May, the 9th on 29 May–4 June, the 34th, Christ the King, on
20–26 November [liturgyoffice-sundays] — are the same windows.

**Worked example: 2025–26.** The First Sunday of Advent of 2025 is the
Sunday between 27 November and 3 December: 30 November. The liturgical
year it begins is 2026. 2025 is divisible by three (675 × 3), so it is
Year A; 2026 is even, so the weekdays are Year II. Easter 2026 is 5 April,
Pentecost 24 May, Trinity Sunday 31 May. Christ the King is the Sunday
between 20 and 26 November 2026, the 22nd, Proper 29. The first Sunday
after Trinity, 7 June, is 24 weeks earlier, so it is Proper 5 [Ordinary
10]. The Liturgy Office's table gives A and II for 2026 and 29 November
2026 as the next Advent [liturgyoffice-moveable].

## What is carried

- **Functions** `first_sunday_of_advent`, `liturgical_year`,
  `sunday_cycle` (`SundayCycle::A`, `B`, `C`), `roman_weekday_cycle`
  (`WeekdayCycle::I`, `II`) and `rcl_proper`, which answers for a Sunday
  after Trinity Sunday up to Christ the King, and `None` otherwise.
- **Range** the liturgical years 1583 to 4099, whose Easter the Gregorian
  computus gives. The Roman Lectionary is of 1969 and the RCL begins with
  Advent 1992 [cct-rcl, §8]; a letter for an earlier year is the rule
  applied backwards.
- **Not carried.**
  - *The readings*, which are the Holy See's and the Consultation on
    Common Texts' texts under their own copyright.
  - *The Propers of the Sundays after the Epiphany.* The RCL's calendar
    prints the Sixth to Eighth Sundays after the Epiphany as Propers 1
    to 3 for the churches that use them there, "except when this is the
    Last Sunday after the Epiphany" [cct-rcl, table]; which churches do
    is a choice the table leaves to each.
  - *The Roman numbering of the Sundays in Ordinary Time*, which the
    Liturgy Office tabulates year by year [liturgyoffice-moveable]; after
    Trinity Sunday it is the Proper plus five.
  - *The ordo*, the day-by-day choice between a Sunday and a feast; see
    `roman_calendar` for what that means and why it is out of scope.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The First Sunday of Advent and its year for every year 1992–2021 in the RCL's table [cct-rcl] | `the_rcl_table_of_advents_and_years_1992_to_2021` | 30 of 30 |
| The Sunday and weekday cycles of 2020–2030, at Easter, and the change at Advent 2025 [liturgyoffice-moveable] | `the_liturgy_office_cycles_of_2020_to_2030` | 11 of 11 |
| The worked example: 2026 is A and II, 7 June 2026 Proper 5, 22 November Proper 29 | `the_worked_example_of_2025_to_2026` | yes |
| Proper 29 on the Sunday 20–26 November, 1990–2040 [cct-rcl] | `proper_29_is_the_sunday_between_20_and_26_november` | all |
| The Sunday after Trinity is Proper 3 when Easter is 22 March, as in 1818 [cct-rcl]; never below 3 or above 8 in 1583–2500 | `an_easter_on_22_march_makes_the_sunday_after_trinity_proper_3` | yes |
| The Propers fall in the Liturgy Office's windows of Ordinary 8–34, 2000–2060 [liturgyoffice-sundays] | `the_propers_match_the_roman_sundays_in_ordinary_time_windows` | all |

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [cct-rcl] | §8, Year A in Advent of years divisible by three and the table of Advents and years 1992–2021; §19, the RCL and the Roman Lectionary; the table of the Christian year with the Advent and Proper 29 windows and the note on Proper 3 | Yes, the PDF, 2026-09-26 |
| [liturgyoffice-moveable] | The Sunday and weekday cycles of 2020–2060 by liturgical year, and the Advent Sundays | Yes, 2026-09-26 |
| [liturgyoffice-sundays] | The date windows of the Roman Sundays in Ordinary Time | Yes, 2026-09-26 |
| [wikipedia-lectionary] | The three-year and two-year cycles, odd years Cycle I | Yes, 2026-09-26 (secondary) |
| [wikipedia-ordo-lectionum-missae] | The Lectionary's editions of 1969 and 1981 | Yes, 2026-09-26 (secondary) |

The *Ordo Lectionum Missae*'s own praenotanda, which state the cycles for
the Roman Rite, were not read.

## Code

`crates/hc-holiday/src/lectionary.rs`. Anchors:
`the_rcl_table_of_advents_and_years_1992_to_2021`,
`the_liturgy_office_cycles_of_2020_to_2030`,
`the_propers_match_the_roman_sundays_in_ordinary_time_windows`.
