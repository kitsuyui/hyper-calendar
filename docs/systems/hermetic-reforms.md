# Four reforms from Hermetic Systems: the 33-year calendars, the Hermetic Leap Week, Week and Month, and Tabot

Backs the identifiers `dee-cecil`, `dee`, `hermetic-leap-week`,
`week-and-month` and `tabot` in `hc-calendars-solar`.

## What they are

Proposals for calendar reform published on Peter Meyer's Hermetic Systems
site, each a single rule over the Gregorian months or the seven-day week,
none adopted by anyone:

- **The 33-year calendars.** The Julian months with a leap year whenever the
  year's remainder on division by 33 is not zero and is a multiple of 4,
  which Meyer attributes to John Dee, adviser to Elizabeth I on the reform
  of 1582, in two correlations: the *Dee–Cecil* calendar, named for William
  Cecil, "who preferred removing ten days from the calendar rather than
  Dee's eleven", and the *Dee* calendar a day earlier [meyer-dee-cecil].
  Duncan Steel's account of the proposal's history is on the same site
  [steel-33-year]. Simon Cassidy argued for the same rule in 1996, as the
  "Anni-Domini" rule of the eight leap years in "the 33-year traditional
  life of Jesus" [cassidy-33-year].
- **The Hermetic Leap Week Calendar**, Meyer's of 8 January 2007: 52 or 53
  weeks a year, twelve months of five, four and four weeks
  [meyer-hermetic-leap-week], with Karl Palmen's direct form of its leap
  rule [palmen-hermetic-leap-week].
- **The Week and Month Calendar**, Palmen's of 2018: the ISO week-numbering
  year cut into months of four or five weeks named Alpha to Epsilon, so that
  a date needs no day of the month [palmen-week-and-month].
- **The Tabot calendar**, created by Mark Moore (Ras Mahitema Selassie) for
  Tabot Ministries and first published on paper in 1997, whose rules Meyer
  fixed in 2006 at the ministry's request: the year from 2 November, the
  coronation day of Haile Selassie in 1930 [meyer-tabot].

## How they work

**The 33-year rule.** "A year is a leap year if (and only if) the year
number, when divided by 33, yields a non-zero remainder which is a multiple
of 4": the 4th, 8th, …, 32nd years of each cycle, eight in 33, 12 053 days,
a mean year of 365.2424̄ days [meyer-dee-cecil]. Years are numbered
astronomically; the Dee–Cecil 1-1-1 is Julian Day Number 1 721 426, the
Gregorian 1 January 1, and the Dee 1-1-1 is 1 721 425. Meyer's page states
the rule's consequence: "days back to March 1, 1980, and days forward to
February 28, 2016, have the same dates in the Dee-Cecil Calendar as in the
Gregorian" [meyer-dee-cecil]. Cassidy's words for his rule — "February will
have 29 days whenever the A.D. year-number, reduced modulo 33, is non-zero
and divisible by 4" — and his claim that it "has been in effect since March
1st. 1980 on a probationary trial period of 36 years", with the Gregorian
and Anni-Domini leap years the same "from 1981-2015" as over 1585–1619
[cassidy-33-year], make his calendar the Dee–Cecil calendar, on the same
days: the roadmap's `cassidy-33` is not a calendar of its own. His further
proposal, to make 2000 a common year and so apply "Dee's extra day of
correction", is the Dee calendar from 1 March 2000.

*Worked example.* 2016 is 3 mod 33, so the Dee–Cecil calendar has no
29 February 2016 and calls the Gregorian 29 February its 1 March. 2017 is
4 mod 33 and leap, so its 29 February is the Gregorian 28 February 2017,
and from 1 March 2017 the two agree again, until 2020 (7 mod 33).

**The Hermetic Leap Week rule.** Meyer groups the years into *hexades* of
five or six years whose third year alone is leap, one hexade beginning in
year 1, a hexade beginning in year Y being short when `Y · 71 mod 100 < 26`;
Palmen proved that the leap years are then exactly those with
`(71 · Y + 203) mod 400 < 71`, 71 in 400 years, a mean year of the
Gregorian 365.2425 days [meyer-hermetic-leap-week; palmen-hermetic-leap-week].
Year 1 begins on JDN 1 721 419, Monday 25 December 1 BC, "the first Monday
after the northern winter solstice of the year 0 CE". The months are
Arcturus, Bellatrix, Canopus, Deneb, Elnath, Fomalhaut, Girtab, Hadar,
Izar, Jabbah, Kochab and Lesath, of 5, 4, 4 weeks a quarter, Lesath taking
the leap week. Meyer writes a date by month (LPM) or by week of the year
(LPW) [meyer-hermetic-leap-week].

*Worked example.* The hexade beginning in 2007 has the indicator
`07 · 71 = 497 → 97`, not below 26, so it is long, 2007 to 2012, and its
third year, 2009, is leap; Palmen's rule agrees, `(71 · 2009 + 203) mod 400
= 42`. 2007 begins on Monday 25 December 2006, so 8 January 2007 is
fourteen days in: 15 Arcturus 2007 (`2007-01-15 LPM`), the first day of
week 3 (`2007-03-1 LPW`), as Meyer dates his page.

**Week and Month.** The ISO weeks 1–4 are January, 5–9 February, 10–13
March, and so on in fours and fives to December, weeks 49 to 52 or 53; the
weeks of a month are Alpha, Beta, Gamma, Delta and Epsilon, and the year is
"the Gregorian year of its Thursdays" [palmen-week-and-month].

*Worked example.* Thursday 10 January 2019 is ISO 2019-W02-4, the second
week of January: "Thursday Beta January 2019", 2019-01-2-4.

**Tabot.** Months of 30 days, the twelfth of 35, and Ras, the fourth, of 31
in a long year N, "N+3 is divisible by 4 but N+31 is not divisible by 100"
or "N+331 is divisible by 400"; year 0 began on 2 November 1930
[meyer-tabot]. Ras begins on 31 January, so the rule is the Gregorian leap
rule applied to the year N + 1931 whose February Ras spans, and every
month begins on the same Gregorian date each year, as Meyer's table shows.
The months are Anbassa, Hymanot, Immanuel, Ras, Ta'Berhan, Manassa,
Danaffa, Negest, Tafari, Emru, Sawwara, and Negus & Dejazmatch; the
weekdays Ergat (Sunday), Tazajenat, Kedusenant, Ra'ee, Makrab, Mamlak and
Germa.

*Worked example.* 27 September 2006 falls before 2 November, so in the year
that began on 2 November 2005, 2005 − 1930 = 75. It is 329 days in: three
months of 30, Ras of 30 (75 + 3 is not divisible by 4), and six more of 30
make 300, and 29 more is the thirtieth of the eleventh month: Sawwara 30,
75, the date Meyer gives his page.

## What is carried

- **`dee-cecil`** and **`dee`**: year, month and day on the Gregorian month
  names, any year the Gregorian module converts. One struct, `DeeCalendar`,
  with the two correlations as constants.
- **`hermetic-leap-week`**: the month form, the stars' names in the shape,
  the week of the year and the weekday as the `week` and `day-of-week`
  fields; years −99 999 to 99 999, astronomical. It runs on the leap-week
  engine the Symmetry calendars use, `leap_week`, with its own constants.
- **`week-and-month`**: year, month, and a day of the month derived as
  `7 · (week − 1) + weekday`, because every calendar with months carries one;
  the week of the month, named Alpha to Epsilon in the shape's
  `week-of-month` cycle, and the weekday are extra fields. A naming of
  `iso8601-week`, on its range.
- **`tabot`**: year, month and day from year 0, the months and the weekdays
  named in the shape, the weekdays ordered from Monday as the crate orders
  the week. Moore's own publication and the ministry's site were not read.
- **Not carried:** Palmen's suggestions for Christmas and Easter in the Week
  and Month calendar, which are proposals about feasts, not the calendar;
  Dee's own eleven-day correction as a historical proposal, which the
  sources give no dated rule for beyond the Dee correlation itself; and the
  roadmap's `cassidy-33`, which is the Dee–Cecil calendar.
- `usage` is unrecorded for all five: proposals, and for Tabot a calendar
  the sources give no period of use for.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| Meyer's `IsDeeLeapYear`, `Dee2JDN` and `JDN2Dee`, run as published, over JDN 1 000 000 to 3 000 000 in both correlations | `meyers_conversion_functions_are_the_reference` | all 41 238 days |
| Dee–Cecil and Gregorian agree from 1 March 1980 to 28 February 2016 and part on 29 February 2016 | `dee_cecil_agrees_with_the_gregorian_calendar_from_1980_to_2016` | every day |
| Cassidy's spans 1981–2015 and 1585–1619 and his reductions of 1996, 2012 and 2016 | `cassidys_rule_is_this_rule_and_his_spans_hold` | all |
| 13 200 Dee years are a day short of 13 200 Gregorian ones | `thirteen_thousand_two_hundred_years_are_a_day_short_of_the_gregorian` | exact |
| Meyer's new years of 2007–2012 and his dated examples | `the_new_years_of_the_hexade_of_2007_are_meyers`, `the_dated_examples_are_meyers` | all |
| Palmen's list of the 71 leap years of a cycle; the hexade definition against the direct rule over 8 000 years | `the_leap_years_of_a_cycle_are_palmens_list`, `the_hexade_definition_and_the_direct_rule_agree` | all |
| Palmen's three dated examples; an Epsilon December exactly when Christmas is on Friday Delta December or later, 1600–2400 | `palmens_examples`, `an_epsilon_december_is_a_late_christmas` | all |
| Tabot: Sawwara 30, 75 and Sawwara 75's first day, a Kedusenant; every month's Gregorian first day over 500 years | `the_page_was_published_on_sawwara_30_75`, `every_month_begins_on_its_gregorian_date` | all |

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [meyer-dee-cecil] | The 33-year rule, both correlations, the 1980–2016 agreement, the conversion functions | Yes, 2026-09-26 |
| [cassidy-33-year] | Cassidy's rule, his spans and reductions, the 2000 proposal | Yes, 2026-09-26 |
| [steel-33-year] | The history of Dee's proposal | Yes, 2026-09-26 |
| [meyer-hermetic-leap-week] | The Hermetic Leap Week Calendar, its months, epoch and examples | Yes, 2026-09-26 |
| [palmen-hermetic-leap-week] | The direct leap rule, its proof and the list of leap years | Yes, 2026-09-26 |
| [palmen-week-and-month] | The Week and Month Calendar and its examples | Yes, 2026-09-26 |
| [meyer-tabot] | The Tabot calendar's rules, names and table | Yes, 2026-09-26 |

## Code

`crates/hc-calendars-solar/src/dee.rs`, `hermetic_leap_week.rs` (on
`leap_week.rs`), `week_and_month.rs` and `tabot.rs`. Anchors:
`meyers_conversion_functions_are_the_reference`,
`the_leap_years_of_a_cycle_are_palmens_list`, `palmens_examples`,
`the_page_was_published_on_sawwara_30_75`.
