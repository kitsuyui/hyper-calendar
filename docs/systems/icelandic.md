# The Old Icelandic calendar: the misseri, the sumarauki and the two rules

Backs the identifiers `icelandic`, `icelandic-julian`,
`icelandic-almanac`, `icelandic-friday`, `icelandic-julian-friday` and
`icelandic-medieval` in `hc-calendars-solar`.

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

**The Almanac's leap week.** "In the printed Icelandic Almanac, which
has been published since 1837, the leap week was inserted last in the
summer until 1928", so that "the Gregorian dates in Table 1 for S4–S6
were shifted to 22–28 July, 21–27 August and 20–26 September" in a leap
year; "this affects only the last three summer months ... the reckoning
by weeks was not affected" [janson2011, §7.1]. The year is the same
length and begins on the same day; only the seven days move, from after
the *aukanætur* to after Haustmánuður.

**Winter from a Friday.** "While there is agreement that summer begins
on a Thursday, there are two different traditions for the beginning of
winter: Friday or Saturday." The learned literature and Grágás specify
Saturday, "however, winter was reckoned from a Friday (one day before the
beginning of winter as shown in Table 1) from the 16th century until the
Icelandic Almanac began to be published in 1837, when the Saturday
reckoning was revived"; it "is first documented in 1508", and "the law
made in 1700 ... explicitly reckons winter from a Friday"
[janson2011, §2.1]. That law moved "the beginning of winter, which until
now has been on that Friday that is between the 9th and 18th October" to
"that Friday that is between the 19th and 28th October" [janson2011,
§3.4]. Winter weeks then "begin on Saturdays (or Fridays, see Section
2.1)" [janson2011, §2.2]. What Janson dates by the Friday is the
beginning of winter and its weeks, not of Gormánuður, which "comes on a
Saturday" in *Bókarbót*.

**The day.** "The day in Iceland in the Middle Ages was reckoned from
sunrise during summer and from dawn during winter (when the sun rises
late in Iceland)", and it began in the morning: "Day comes before night
throughout the Icelandic calendar" (*Bókarbót*) [janson2011, §2.4].

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

**Worked example, the Almanac.** 1838 is a leap year: summer began on
Thursday 19 April 1838, the next on Thursday 25 April 1839, 371 days on.
`icelandic` counts Heyannir from Sunday 29 July, after the eleven extra
days; the Almanac counts it from Sunday 22 July, after the four
*aukanætur*, in Janson's window of 22–28 July. Haustmánuður then ends
on Friday 19 October, the *sumarauki* runs from Saturday 20 to Friday
26 October, and winter begins on Saturday 27 October 1838, 180 days
before the next summer, as in `icelandic`. Under the Friday reckoning
the same winter begins on Friday 26 October.

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
- **`icelandic-almanac`**, the Almanac's year: the rule of 1700, with
  fourteen positions, the leap week *Sumarauki* the eighth after
  Haustmánuður, 7 days in a leap year and absent in a common one, and
  the winter months ninth to fourteenth. The fields' season, week and
  *sumarauki* flag are as above; the weeks are `icelandic`'s.
- **`icelandic-friday`** and **`icelandic-julian-friday`**, the two rules
  with winter from the Friday before the Saturday: the dates are
  `icelandic`'s and `icelandic-julian`'s, and `season` and `week` turn at
  the Friday, so winter's 26th week has six days and summer's last day
  after its full weeks is its one *veturnótt*, week 0. Janson does not
  say how many *veturnætur* the Friday reckoning had; week 0 is this
  library's name for the days after the last full week, as for the
  Saturday.
- **`icelandic-medieval`**, the Julian rule with the medieval day,
  `DayBoundary::Daybreak(DayNaming::ByStart)`: sunrise in summer and dawn
  in winter, the day named by the civil day it begins on. The boundary
  names the convention; which moment begins a given day is the season's
  and the place's, and is not computed.
- **Usage** `icelandic-julian` until 16 November 1700 (Julian), from an
  undated start in the eleventh or twelfth century; `icelandic` from
  28 November 1700, in use today through the Almanac;
  `icelandic-almanac` from the First Day of Summer 1837 to the last day
  of 1927, since 1928 — itself a leap year — is the year "until 1928"
  leaves open; `icelandic-julian-friday` until 16 November 1700 from an
  undated start, since "the Friday beginning is first documented in
  1508" and whether it is older is contested; `icelandic-friday` from
  28 November 1700 to the last day of 1836; `icelandic-medieval` until
  16 November 1700 at the latest, since no source read dates when the day
  came to be reckoned from midnight.
- **Not carried**, with the reason:
  - *The 1888 Almanac*, which "forgot to insert the leap week
    (sumarauki); this was corrected the following year" [janson2011,
    §7.1]: a misprint, not a rule. `icelandic-almanac` has the leap week
    in 1888.
  - *Winter from a Saturday "except at rímspillir when it begins on a
    Friday"*, which "a later 16th-century document" states [janson2011,
    §2.1]; a single document's rule, and Janson does not say where or how
    long it was kept.
  - *The confusion of 1702–1703*, when many places took the First Day of
    Winter from an earlier form of the decision [janson2011, §3.4, n. 47]:
    "in most places ... Friday 20 October", which is what
    `icelandic-friday` gives for 1702, where the first form of the
    decision gave 27 October.
  - *The earlier calendars* — the 364-day year without intercalation and
    Þorsteinn surtr's week every seventh summer, c. 955 — whose rules are
    not recoverable [janson2011, §3.1].

## Accuracy

| Check | Test | Result |
| --- | --- | --- |
| The 33 sample dates of *Calendrical Calculations*, 586 BCE to 2094, as its published code computes them [reingold2018code, `dates.l`]: `icelandic`, year, season, week and weekday | `every_sample_date_agrees_or_is_refused_or_is_a_known_difference` (`crates/hyper-calendar/tests/rd_sample_dates.rs`) | 31 of the 31 from AD 1; the 2 before refused |
| First Days of Summer 1700 (11 April Jul. = 22 April Greg.), 1701 (21 April), 1703 (19 April Greg.; 15 April Jul. by the old rule), 2009 (23 April), 2024–2026 | `the_first_day_of_summer_is_the_thursday_in_the_window` | all |
| Janson's closed forms (5.4), 15 − ((y + ⌊y/4⌋) mod 7) April Julian, and (6.4), 25 − ((y + ⌊y/4⌋ − ⌊y/100⌋ + ⌊y/400⌋ + 5) mod 7) April Gregorian | `the_first_day_of_summer_follows_jansons_formulas` | years 1–3000 |
| The rules agree 1496–1702, differ in 1495 and 1703; *sumarauki* in 1702 only under the Julian rule | `the_two_rules_agree_from_1496_and_part_in_1702` | all |
| Julian rule: leap years 3, 8, 14, 20, 25 of the solar cycle; *rímspillir* in 1119, 1147 … 1679 | `the_julian_rule_has_five_leap_weeks_in_the_solar_cycle` | 1100–1699 |
| Gregorian rule: 71 leap weeks in 1700–2099; the leap years are those with summer on 19 April, or 20 April before a leap year; *rímspillir* in 1719 … 2079; no leap week 1697–1702; none in 1899 | `the_gregorian_rule_has_71_leap_weeks_in_400_years` | all |
| Every month begins on its weekday in Table 1; Þorri 19–26 January and Heyannir 23–30 July | `every_month_begins_on_its_own_weekday` | all |
| Summer and winter weeks and the *veturnætur* of 2026 | `the_seasons_and_weeks_are_the_almanacs` | all |
| Every day of thirty years from 1690 under both rules, and a sample of the whole range, round-trip | `every_day_round_trips_under_both_rules` | all |
| The Almanac: in the 16 leap years of 1837–1927 Heyannir, Tvímánuður and Haustmánuður begin in 22–28 July, 21–27 August and 20–26 September, a week before `icelandic`'s, and the leap week ends the day before winter; 1838 worked in full | `the_almanac_puts_the_leap_week_last_in_summer` | all |
| The Almanac's weeks and seasons are `icelandic`'s on every day of 1836–1930, and its days round-trip | `the_almanacs_weeks_are_the_standard_weeks_and_every_day_round_trips` | all |
| Winter from a Friday: the Friday 20–27 October Gregorian and 10–17 October Julian, as the law of 1700 words it, 1500–1899; Friday 20 October 1702, "in most places" | `winter_from_a_friday_is_the_day_before_the_saturday` | all |
| The medieval day begins at daybreak and names its day by its start; every other form at midnight | `the_medieval_day_begins_at_daybreak` | holds |

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
| [janson2011] | Everything above: the structure, the months and their weekdays, the two rules, the change of 1700, the formulas, the leap-week and *rímspillir* tables, the variations | Yes, the DiVA copy, 2026-09-26; §2.1, §2.4, §3.4 n. 47 and §7.1 re-read in the same copy's text, 2026-09-29 |
| [reingold2018code] | Comparison with Reingold and Dershowitz's Icelandic functions | The `calendar.l` section, 2026-09-26; no code taken |
| [reingold2018] | Chapter 6 | Not read |

## Code

`crates/hc-calendars-solar/src/icelandic.rs`. Anchors:
`the_first_day_of_summer_is_the_thursday_in_the_window`,
`the_two_rules_agree_from_1496_and_part_in_1702`,
`the_gregorian_rule_has_71_leap_weeks_in_400_years`,
`the_almanac_puts_the_leap_week_last_in_summer`,
`winter_from_a_friday_is_the_day_before_the_saturday`,
`the_medieval_day_begins_at_daybreak`. The rule is `Rule::summer_raw`;
the counting `Rule::to_fixed` and `Rule::from_fixed`; the reckonings
`IcelandicCalendar::reckoning` and `IcelandicCalendar::first_day_of_winter`;
the Almanac `IcelandicAlmanacCalendar`. `DayBoundary::Daybreak` is in
`crates/hc-calendar/src/daystart.rs`.
