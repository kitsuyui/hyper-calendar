# The Hanke–Henry Permanent Calendar: the ISO year cut into months

Backs the identifier `hanke-henry` in `hc-calendars-solar`.

## What it is

A calendar reform proposed by Richard Conn Henry, an astronomer at Johns
Hopkins, and revised with the economist Steve H. Hanke: every year
identical, every date on the same weekday for ever, the year always
beginning on Monday 1 January, and the seasons kept by a whole extra week
every five or six years rather than a day every four
[hankehenry-calendar]. It has been adopted by nobody. It has also changed:
Henry's proposal of 2004 put the leap week, then called *Newton*, between
June and July, and the year began on a Sunday until the start was moved to
Monday for 2024 [wikipedia-hanke-henry]. This library carries the proposal
as its authors state it now.

## How it works

**The months.** "The first two months of each quarter are made up of 30
days, and the third is made up of 31 days" [hankehenry-calendar]: 30, 30,
31 four times, 91 days a quarter, 364 a year, 52 weeks exactly. January,
February, April, May, July, August, October and November have 30 days;
March, June, September and December have 31. The year "begins on Monday,
January 1" [hankehenry-calendar], so every quarter begins on a Monday and
ends on a Sunday, and "if, for example, your birthday is March 7, it will
always fall on a Thursday" [hankehenry-qanda].

**The leap week.** A seven-day "mini-month" called *Xtr* is added "at the
end of December" in some years [hankehenry-calendar], and the authors give
the test, which they credit to Irv Bromberg: "if the corresponding
Gregorian year begins or ends on a Thursday, that year contains an Xtr
month" [hankehenry-qanda]. Those are the years with 53 ISO 8601 weeks.
Since every year is 364 or 371 days and begins on a Monday, and the
authors put 1 January 2024 on the Gregorian 1 January, the year begins on
the Monday that opens ISO week 1 — the Monday nearest 1 January — and the
Hanke–Henry year is the ISO week-numbering year, cut into months instead
of numbered weeks. There are 71 leap weeks in 400 years, the mean year is
the Gregorian one, and a date is "never more than five days off" its
Gregorian namesake [hankehenry-qanda].

**Worked example.** What is Saturday 26 September 2026? ISO week 1 of
2026 begins on Monday 29 December 2025, so that is 1 January 2026 here, and
26 September is 271 days later. Two quarters are 182 days, leaving 89 into
the third: July has 30 and August 30, so this is day 29 of September, the
thirtieth: 30 September 2026. 2026 begins on a Thursday, so it has *Xtr*,
28 December 2026 to 3 January 2027 Gregorian, and 1 January 2027 is
Monday 4 January.

## What is carried

- **Identifier** `hanke-henry`: year, month and day, *Xtr* as month 13, as
  the authors' converter numbers it [hankehenry-qanda]. The shape declares
  the month cycle as twelve positions with a thirteenth in a leap year,
  named January to December and Xtr; a locale's Gregorian month names
  serve the twelve.
- **Weekday.** `HankeHenryDate::weekday` is a function of the month and
  day; *Xtr* is in the week, so it is always the weekday of the unbroken
  seven-day cycle, unlike the blank days of the World Calendar and the
  International Fixed Calendar.
- **Range** years 1 to 99 999, proleptic before 2024. `usage` is
  unrecorded: a proposal.
- **Not carried:** the 2004 form with *Newton* between June and July and
  the Sunday-start form, which their authors superseded and which are known
  here only from Wikipedia; and the time half of the proposal, Universal
  Time everywhere, which is not a calendar.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The Xtr years 1970–3001 are exactly those the authors list, 2026 to 3001 on the calendar page and 1970 to 2195 in the converter | `the_xtr_years_are_the_ones_the_authors_list` | all 1 032 years |
| Xtr where the Gregorian year begins or ends on a Thursday; the ISO long years and ISO week 1; 71 in 400 years | `xtr_is_where_the_gregorian_year_begins_or_ends_on_a_thursday` | all |
| 1 January 2018 and 2024 are the Gregorian ones; every year begins on a Monday | `every_year_begins_on_monday_1_january` | all |
| 7 March is always a Thursday; the calendar's weekday is the unbroken week's | `a_date_falls_on_the_same_weekday_every_year` | all |
| No date is more than five days from its Gregorian namesake, 2024–2423 | `a_date_is_never_more_than_five_days_from_its_gregorian_namesake` | all |
| Round trips over the whole range and on 2 800 consecutive days from December 2019, across two Xtr weeks | `every_day_round_trips` | all |

Henry's own page at Johns Hopkins refused to be read for this document,
so the placement of *Xtr* rests on the calendar site alone.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [hankehenry-calendar] | The months, the 364-day year, the list of Xtr years, Monday 1 January | Yes, 2026-09-26 |
| [hankehenry-qanda] | The Thursday test, the 2024 start, the 7 March example, the five-day bound; the converter's numbering and Xtr years | Yes, 2026-09-26 |
| [wikipedia-hanke-henry] | The superseded 2004 and Sunday-start forms | Yes, 2026-09-26 |

## Code

`crates/hc-calendars-solar/src/hanke_henry.rs`. Anchors:
`the_xtr_years_are_the_ones_the_authors_list`,
`every_year_begins_on_monday_1_january`,
`a_date_falls_on_the_same_weekday_every_year`. The year boundary is
`new_year`; the leap rule `is_leap_year`.
