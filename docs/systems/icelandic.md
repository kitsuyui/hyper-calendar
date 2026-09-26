# The Old Icelandic calendar: the misseri, the sumarauki and the two rules

Backs the identifiers `icelandic` and `icelandic-julian` in
`hc-calendars-solar`.

## What it is

Iceland's own calendar, the *misseristal* ("reckoning of half-years"), was
"used as the civil calendar by the general population from the 10th to the
18th century, and occasionally later; it is still included in the
Icelandic Almanac" [janson2011, §1]. Its year is 52 weeks, 364 days, in two
*misseri*, summer and winter; a leap week, *sumarauki* ("summer
increase"), makes a leap year of 371 days, so that "every year begins on
the same day of the week", and so does every month. Time was reckoned
chiefly by weeks — "the standard way of reckoning time was by using
weeks" — and the thirty-day months were little used; dating by month and
day "has never been used in Iceland" [janson2011, §2.2–2.3]. The First Day
of Summer, *sumardagurinn fyrsti*, always a Thursday, is still a public
holiday [janson2011, §3.5].

## How it works

**The year.** Summer begins on the First Day of Summer, a Thursday, and has
three months of thirty days, then four extra days, *aukanætur* — eleven
in a leap year, with the *sumarauki* — then three months more; winter
begins on a Saturday, the First Day of Winter, and has six months of
thirty. Summer is 184 or 191 days and winter 180 [janson2011, §1, Table 1].
The months and the weekdays they always begin on are:

| | Summer | Begins | | Winter | Begins |
| --- | --- | --- | --- | --- | --- |
| S1 | Harpa | Thursday | W1 | Gormánuður | Saturday |
| S2 | Skerpla | Saturday | W2 | Ýlir | Monday |
| S3 | Sólmánuður | Monday | W3 | Mörsugur | Wednesday |
| | *aukanætur* (+ *sumarauki*) | Wednesday | W4 | Þorri | Friday |
| S4 | Heyannir | Sunday | W5 | Góa | Sunday |
| S5 | Tvímánuður | Tuesday | W6 | Einmánuður | Tuesday |
| S6 | Haustmánuður | Thursday | | | |

**The leap week.** Nothing is intercalated by count. The First Day of
Summer is fixed to a seven-day window of the church calendar, and a year
has *sumarauki* exactly when the next First Day of Summer is 53 weeks
away. From the twelfth century "the First Day of Summer always fell in the
week 9–15 April" of the Julian calendar [janson2011, §3.2]; when Iceland
changed calendar in November 1700 — Saturday 16 November (Julian) followed
by Sunday 28 November — the window became "the Thursday in the period
19–25 April" of the Gregorian calendar, "which has remained the rule until
the present" [janson2011, §3.3–3.4]. The Althingi shifted the window by
ten days, not by the eleven the calendars then differed by, so the two
rules are genuinely different calendars: they agree through the
sixteenth and seventeenth centuries, and part at Midsummer 1702, "when
there would have been sumarauki in the Julian version, but not in the
Gregorian"; the First Day of Summer 1703 was Thursday 19 April
(Gregorian), where the Julian rule gives 15 April (Julian), 26 April
[janson2011, §3.4]. Under the Julian rule the calendar repeats every 28
years, with five leap weeks, in years 3, 8, 14, 20 and 25 of the solar
cycle; under the Gregorian every 400, with 71 [janson2011, §5–6]. The
First Day of Winter "is always 180 days before the next First Day of
Summer" [janson2011, §4].

**Counting.** Janson advises counting the first three months forward from
the First Day of Summer and the last nine back from the next one, which
places the leap week without asking whether there is one [janson2011, §4].
Weeks are counted from the start of each *misseri*: summer weeks from
Thursdays, 1 to 26 (27 in a leap year), "ignoring the last two days, which
are called *veturnætur*"; winter weeks from Saturdays, 1 to 26, "with the
last week incomplete" [janson2011, §2.2].

**Worked example.** What is Saturday 26 September 2026? 19 April 2026 is a
Sunday, so the First Day of Summer is Thursday 23 April 2026; 19 April 2027
is a Monday, so the next is Thursday 22 April 2027, 364 days on — no
*sumarauki*. 26 September is 156 days after 23 April, past the first 90,
so count back: it is 208 days before 22 April 2027. Six whole months of
30 back is 180, leaving 28 more into the seventh month from the end,
Haustmánuður, whose 30 days end 180 days before summer: 26 September is
its 28th day from the end, day 3 of Haustmánuður. In weeks it is day 157 of summer, the
Saturday of the 23rd week of summer. The First Day of Winter is 180 days
before 22 April 2027: Saturday 24 October 2026.

## What is carried

- **Identifiers** `icelandic`, the Gregorian rule, and `icelandic-julian`,
  the Julian one — two names because the two rules disagree about real
  years (policy §5). Both run proleptically over years 1 to 99 998.
- **Dates** are year, position and day: the thirteen positions are the
  twelve months in their modern names with the extra days fourth, named
  *Aukanætur*, 4 or 11 days, of which days 5 to 11 are the *sumarauki*.
  The year is numbered by the calendar year its summer begins in —
  "there is no special numbering of the Icelandic years" — and begins with
  summer, which Janson chooses "somewhat arbitrarily" [janson2011, §1,
  §2.1]. The shape names the thirteen positions in Icelandic.
- **The week reckoning** `to_fields` adds: `season`, 1 summer or 2
  winter; `week`, the week of the *misseri*, 0 on the two *veturnætur*;
  and `sumarauki`, 1 on the leap-week days.
- **Usage** `icelandic-julian` until 16 November 1700 (Julian), from an
  undated start in the eleventh or twelfth century; `icelandic` from
  28 November 1700, in use today through the Almanac.
- **Not carried**, with the reason:
  - *The Almanac's leap week at the end of summer* until 1928, which moved
    Heyannir to Haustmánuður by a week in leap years and did not affect the
    week reckoning [janson2011, §7.1]. A third identifier if a caller needs
    those printed dates.
  - *Winter from a Friday*, the popular reckoning from the sixteenth
    century to 1837 [janson2011, §2.1].
  - *The confusion of 1702–1703*, when many places took the First Day of
    Winter from an earlier form of the decision [janson2011, §3.4, n. 47].
  - *The earlier calendars* — the 364-day year without intercalation and
    Þorsteinn surtr's week every seventh summer, c. 955 — whose rules are
    not recoverable [janson2011, §3.1].
  - *The day boundary*: the medieval day began at sunrise or dawn
    [janson2011, §2.4]; midnight is kept.

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| First Days of Summer 1700 (11 April Jul. = 22 April Greg.), 1701 (21 April), 1703 (19 April Greg.; 15 April Jul. by the old rule), 2009 (23 April), 2024–2026 | `the_first_day_of_summer_is_the_thursday_in_the_window` | all |
| Janson's closed forms (5.4), 15 − ((y + ⌊y/4⌋) mod 7) April Julian, and (6.4), 25 − ((y + ⌊y/4⌋ − ⌊y/100⌋ + ⌊y/400⌋ + 5) mod 7) April Gregorian | `the_first_day_of_summer_follows_jansons_formulas` | years 1–3000 |
| The rules agree 1496–1702, differ in 1495 and 1703; *sumarauki* in 1702 only under the Julian rule | `the_two_rules_agree_from_1496_and_part_in_1702` | all |
| Julian rule: leap years 3, 8, 14, 20, 25 of the solar cycle; *rímspillir* in 1119, 1147 … 1679 | `the_julian_rule_has_five_leap_weeks_in_the_solar_cycle` | 1100–1699 |
| Gregorian rule: 71 leap weeks in 1700–2099; the leap years are those with summer on 19 April, or 20 April before a leap year; *rímspillir* in 1719 … 2079; no leap week 1697–1702; none in 1899 | `the_gregorian_rule_has_71_leap_weeks_in_400_years` | all |
| Every month begins on its weekday in Table 1; Þorri 19–26 January and Heyannir 23–30 July | `every_month_begins_on_its_own_weekday` | all |
| Summer and winter weeks and the *veturnætur* of 2026 | `the_seasons_and_weeks_are_the_almanacs` | all |
| Every day of thirty years from 1690 under both rules, and a sample of the whole range, round-trip | `every_day_round_trips_under_both_rules` | all |

Janson's text also states that the last year before 1700 in which the two
rules would have differed is 1495, which the tests confirm. Reingold and
Dershowitz's *Calendrical Calculations* was not read; its published Lisp
code was read for comparison after the module was written, and agrees on
the Gregorian rule and the First Day of Winter. It dates by season, week
and weekday, numbers the *veturnætur* as a 27th (or 28th) week of summer
where Janson ignores them, and has no Julian form.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [janson2011] | Everything above: the structure, the months and their weekdays, the two rules, the change of 1700, the formulas, the leap-week and *rímspillir* tables, the variations | Yes, the DiVA copy, 2026-09-26 |
| [reingold2018code] | Comparison with Reingold and Dershowitz's Icelandic functions | The `calendar.l` section, 2026-09-26; no code taken |
| [reingold2018] | Chapter 6 | Not read |

## Code

`crates/hc-calendars-solar/src/icelandic.rs`. Anchors:
`the_first_day_of_summer_is_the_thursday_in_the_window`,
`the_two_rules_agree_from_1496_and_part_in_1702`,
`the_gregorian_rule_has_71_leap_weeks_in_400_years`. The rule is
`Rule::summer_raw`; the counting `Rule::to_fixed` and `Rule::from_fixed`.
