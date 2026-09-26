# Ember and Rogation Days: the Prayer Book, *Common Worship* and the Roman rubrics of 1960

Backs `hc-holiday`'s tradition tables `ember-bcp1662`,
`ember-common-worship` and `rogation-roman-1960`.

## What it is

The **Ember Days** are four groups of three fast days, a Wednesday, a
Friday and a Saturday, one group in each season of the year. The Western
Church kept them in the weeks after the First Sunday in Lent, Pentecost,
Holy Cross Day (14 September) and St Lucy's Day (13 December),
remembered in English as "Lent, Whitsun, Holyrood, and Lucie"
[wikipedia-ember-days]. The Church of England now keeps them in the week
before an ordination [cw-rules].

The **Rogation Days** are days of prayer for the crops. The Roman Rite
has two kinds. The *Greater Litanies* fall on 25 April. The *Lesser
Litanies*, or Rogations, fall on the three days before the Ascension
[rubrics-1960, nos. 80 and 87]. The Book of Common Prayer and *Common
Worship* keep only the three days before the Ascension
[bcp1662-vigils; cw-rules].

The churches do not agree on which weeks the Ember Days fall in, so the
library carries one table per church ([policy.md](../policy.md) §5):

- **The Book of Common Prayer of 1662.** "The Ember Days at the Four
  Seasons, being the Wednesday, Friday and Saturday after: 1. The First
  Sunday in Lent 2. The Feast of Pentecost 3. September 14 4. December
  13" [bcp1662-vigils].
- ***Common Worship*, the Church of England's rules of today.** "Ember
  Days should be kept, under the bishop's directions, in the week before
  an ordination". "Traditionally they have been observed on the
  Wednesdays, Fridays and Saturdays within the weeks before the Third
  Sunday of Advent, the Second Sunday of Lent and the Sundays nearest to
  29 June and 29 September" [cw-rules].
- **The Roman Rite under the Code of Rubrics of 1960.** It keeps the
  Greater and Lesser Litanies [rubrics-1960].

## How it works

**The Easter-relative days.** The Prayer Book and *Common Worship* date
the Lent week from the First Sunday in Lent, which is four days after Ash
Wednesday and so 42 days before Easter. The Wednesday, Friday and
Saturday after it are 39, 37 and 36 days before Easter. The Rogation Days
are the Monday, Tuesday and Wednesday before the Ascension: 36, 37 and 38
days after Easter. The Prayer Book's Whitsun week is the Wednesday, Friday
and Saturday after Pentecost: 52, 54 and 55 days after Easter. All of
these follow the Gregorian computus.

**"After September 14" and "after December 13".** The phrase has two
readings. It can mean one week: the first Wednesday after the day, then
the Friday and Saturday after that Wednesday. Or it can be read day by
day: the first Wednesday, the first Friday and the first Saturday after
the day, each counted separately. The two readings agree when the 14th
falls on a Saturday, Sunday, Monday or Tuesday. They part when it falls on
a Wednesday, Thursday or Friday. For a Thursday 14 September, as in 2023,
the week reading gives the 20th, 22nd and 23rd, while the day-by-day
reading gives the 15th, 16th and 20th. Wikipedia states the older Western
rule as a week: when the 14th falls on a Tuesday or a Wednesday, the
Ember Days are "the immediately following Wednesday, Friday, and
Saturday", so a Tuesday gives the 15th, 17th and 18th
[wikipedia-ember-days]. The library takes the week reading, on that
secondary statement. The Prayer Book's own table is ambiguous between
the two, and no Prayer Book almanac of a year that separates them was
read.

**The *Common Worship* weeks.** Each is the week before a Sunday, and the
days are the Wednesday, Friday and Saturday before that Sunday: 4, 2 and 1
days before it. The Second Sunday of Lent is 35 days before Easter. The
Sunday nearest 29 June falls between 26 June and 2 July; the Sunday
nearest 29 September falls between 26 September and 2 October. The Third
Sunday of Advent is two weeks after the First, whose window is 27
November to 3 December [cct-rcl], so it falls between 11 and 17 December.

**The Greater Litanies.** "The Greater Litanies are fixed on 25th April;
but if Easter Sunday or Monday after Easter falls on that day, they are
transferred to the following Tuesday" [rubrics-1960, no. 80]. So Easter
on 25 April, as in 2038, moves them to the 27th. Easter on 24 April, as
in 2011, puts Easter Monday on the 25th and moves them to the 26th.

**Worked example: 2025.** Easter 2025 was 20 April.

1. *Lent.* Ash Wednesday is Easter − 46 = 5 March. The First Sunday in
   Lent is 9 March. Both churches keep Wednesday 12, Friday 14 and
   Saturday 15 March. For *Common Worship* that is the week before the
   Second Sunday of Lent, 16 March.
2. *Rogation.* The Ascension is Easter + 39 = 29 May. The Rogation Days
   are Monday 26, Tuesday 27 and Wednesday 28 May in all three tables.
3. *The Greater Litanies.* 25 April 2025 is a Friday, and Easter Monday
   was the 21st, so the day stays on 25 April.
4. *Summer.* In the Prayer Book, Pentecost is Easter + 49 = 8 June, so
   the days are Wednesday 11, Friday 13 and Saturday 14 June. In *Common
   Worship*, 29 June 2025 is itself a Sunday, so the days are the 25th,
   27th and 28th.
5. *September.* In the Prayer Book, the 14th is a Sunday, so the days
   are Wednesday 17, Friday 19 and Saturday 20. In *Common Worship*, the
   Sunday nearest 29 September is the 28th, so the days are the 24th,
   26th and 27th.
6. *December.* In the Prayer Book, 13 December is a Saturday, so the
   days are Wednesday 17, Friday 19 and Saturday 20. In *Common Worship*,
   the First Sunday of Advent is 30 November and the Third is 14
   December, so the days are the 10th, 12th and 13th.

The two churches agree in Lent and part in the other three seasons.

## What is carried

- **`ember-bcp1662`**: the Prayer Book's four Ember weeks and its three
  Rogation Days, fifteen days a year, with the September and December
  weeks read as one week each.
- **`ember-common-worship`**: *Common Worship*'s four traditional weeks
  and the Rogation Days, fifteen days a year.
- **`rogation-roman-1960`**: the Greater Litanies, moved off Easter
  Sunday and Monday, and the Lesser Litanies.
- **Every entry is religious and none is a day off.** They are computed
  on the Gregorian computus, so they cover the years it gives, 1583–4099.

Not carried:

- ***Common Worship*'s first rule, the week before an ordination.** The
  bishop chooses the ordination date, and no table of those dates was
  read. A diocese that keeps its Ember Days before an ordination keeps
  them on other days than this table's.
- **The Prayer Book's vigils**, and its note moving a vigil off a Sunday.
  They are the eves of feasts, not Ember or Rogation Days.
- **The Roman Ember Days of 1960.** The rubrics read name them, but the
  text read does not give the rule for the September week. Wikipedia
  gives John XXIII's change, which the library does not carry.
- **The Lesser Litanies moved by a local Ordinary.** No. 87 allows it,
  and the choice is local.
- **The Episcopal Church's set**, "the Wednesday, Friday, and Saturday
  after Holy Cross Day" [wikipedia-ember-days]. It would be a table of its
  own, and no Episcopal Church source was read.
- **The Roman practice after 1969.** Bishops' conferences set "the time
  and manner" of observance [wikipedia-ember-days], so there is no single
  rule to compute.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The Prayer Book's fifteen days of 2025, worked from the rule; a Tuesday 14 September (2027) and a Wednesday (2022) | `the_prayer_book_ember_and_rogation_days` | yes |
| A Thursday 14 September (2023): one week, not three separate days | `the_prayer_book_september_week_is_one_week_after_holy_cross` | yes |
| *Common Worship*'s weeks of 2025, worked from the rule, and the Sunday nearest a Monday 29 June (2026) | `the_common_worship_traditional_ember_weeks` | yes |
| *Common Worship*'s June and September days of 2026 as the Church of England's *Daily Prayer* marks them, and the days it does not mark [cofe-daily-prayer-2026]; its December days of 2024 as the Diocese of London's calendar prints them [london-kalendar-2024-25] | `the_common_worship_ember_days_the_church_and_a_diocese_printed` | 9 of 9, and none of the unmarked days |
| The Greater Litanies moved by Easter Sunday (2038) and Easter Monday (2011) | `the_greater_litanies_leave_easter_and_easter_monday` | yes |

The *Common Worship* anchors are the only published dates. The Church's
*Daily Prayer* pages for December 2026 did not yet show the day's
observance when read, and the pages for 2025 were no longer online. The
Diocese of London's calendar prints 2020's weekdays under 2025's headings
from January onwards, so only its December 2024 page is used. No
published almanac of the Prayer Book's Ember Days or of the Roman
Litanies was read. Those tables rest on their rules alone, worked by hand
for the years above.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [bcp1662-vigils] | The Prayer Book's Ember Days after Lent 1, Pentecost, 14 September and 13 December, and the Rogation Days | Yes, the PDF, 2026-09-26 and 2026-09-27 |
| [cw-rules] | *Common Worship*'s Ember Days before an ordination, its traditional weeks, and its Rogation Days | Yes, 2026-09-26 and 2026-09-27 |
| [rubrics-1960] | Nos. 80 and 87, the Greater and Lesser Litanies | Yes, in English translation, 2026-09-26; the Latin not read |
| [wikipedia-ember-days] | The older Western rule for the September week, John XXIII's change, the Episcopal Church's set, the Roman practice after 1969 | Yes, 2026-09-26 and 2026-09-27 (secondary) |
| [cct-rcl] | The window of the First Sunday of Advent | Yes, the PDF, 2026-09-26 |
| [cofe-daily-prayer-2026] | The Ember Days of June and September 2026 | Yes, 2026-09-27 |
| [london-kalendar-2024-25] | The Ember Days of December 2024 | Yes, the PDF, 2026-09-27 |

## Code

`crates/hc-holiday/src/traditions.rs`: `EMBER_BCP1662`,
`EMBER_COMMON_WORSHIP`, `ROGATION_ROMAN_1960` and `greater_litanies`.
Anchors in `crates/hc-holiday/tests/traditions.rs`:
`the_common_worship_ember_days_the_church_and_a_diocese_printed`,
`the_prayer_book_ember_and_rogation_days`,
`the_prayer_book_september_week_is_one_week_after_holy_cross`,
`the_common_worship_traditional_ember_weeks`,
`the_greater_litanies_leave_easter_and_easter_monday`.
