# Holidays on the Tibetan calendar: Mongolia and Bhutan

Backs the lunar days of the `MONGOLIA` (`MN`) and `BHUTAN` (`BT`) tables in
`hc-holiday`, the rule shape `Rule::TibetanDay` with its `TibetanMonth`, and
the calendar systems `CalendarSystem::MONGOLIAN` and
`CalendarSystem::TIBETAN_BHUTAN`, which name the `mongolian` and
`tibetan-bhutan` calendars of `hc-calendars-lunar`. The calendars
themselves are written up in [tibetan-variants.md](tibetan-variants.md);
this document is about dating holidays in them.

## What it is

**Mongolia.** Article 4.1 of the Law on Public Holidays and Days of
Observance (18 December 2003, as amended) lists the days "publicly rested",
and three of them are dated in the lunar calendar (*билгийн тооллын*)
[mn-law-public-holidays]:

- 4.1.3, **Tsagaan Sar**: "хаврын тэргүүн сарын шинийн 1, 2, 3", the first,
  second and third days of the first spring month;
- 4.1.8, **Chinggis Khaan Day**: "өвлийн тэргүүн сарын шинийн 1", the first
  day of the first winter month, added by the law of 8 November 2012;
- 4.1.10, **Buddha's Birthday**: "зуны тэргүүн сарын шинийн 15", the
  fifteenth day of the first summer month, added by the law of 20 December
  2019.

The lunar calendar Mongolia keeps is the New Genden version of the Tibetan,
whose months are named by season, month 1 beginning spring, so the first
summer month is month 4 and the first winter month month 10 [janson2014,
Appendix A.3, citing Sanders and Bat-Iredüi, not read here]; Janson names
Chinggis Khaan Day "the first day in the first winter month (month 10)". The law states the day; the Government settles each year's
rest days by resolution when it has to, as in 2025 [mn-resolution-2025-109].

**Bhutan.** No statute lists Bhutan's holidays. The Ministry of Home
Affairs publishes each year a calendar printing the Bhutanese day over every
Gregorian day, with a list of government holidays whose Dzongkha text gives
each its Bhutanese month and day [moha-bt-calendar-2025;
moha-bt-calendar-2026]. Janson describes the same division: some holidays
are fixed Bhutanese dates, "New Year and some dates connected to Buddha and
Buddhism", some are Gregorian, and the Winter Solstice is "calculated
according to the Bhutanese calendar", usually on 2 January [janson2014,
Appendix A.4].

## How it works

A holiday dated on the Tibetan calendar is a day number, 1 to 30, of a
month of a version of the calendar, and falls on the calendar day that
bears the number. Two things make that less simple than it sounds.

**Which month.** A year can repeat a month number. Holidays "are usually
not celebrated in leap months" [janson2014, §11], so a holiday is in the
regular month of its number: the second of the two in the Mongolian
version, whose leap month comes first, and the first in the Bhutanese,
whose leap month comes after. The New Year is the exception: it is the
first day of the year even when the year opens with a leap month 1
[janson2014, Remark 17, verified for the Phugpa Losar of 2000]. So Losar
and Tsagaan Sar are dated in the *first* month of the year, which in the
Mongolian year 2006 is the leap month 1, from 30 January, and not the
regular month 1, from 1 March. `TibetanMonth` says which: `First`,
`Regular(n)` or `Leap(n)`.

**Which day.** A calendar day takes the number of the lunar day current at
its dawn, so now and then a number is skipped and another is repeated
on two days ([tibetan-phugpa.md](tibetan-phugpa.md)). Where a
holiday on such a number is kept is not settled by the sources read:

- Janson gives a general rule from Berzin — a skipped date's holiday on the
  day before, a repeated date's on the first of its two days — and adds
  that he has "not checked them against published calendars" [janson2014,
  §11].
- In 2022 the third day of the first spring month was skipped. iKon.mn
  reported, from an unnamed source, three days off from 2 February, the
  Government having not yet discussed it [ikon-tsagaan-sar-2022].
- In 2025 the first day was skipped. The Government's resolution counted
  the holiday as the second and third days, "шинийн 2, 3-ны өдөр", on
  Saturday 1 and Sunday 2 March, and gave rest days on 3 to 5 March to be
  worked back within the year [mn-resolution-2025-109]. Berzin's rule would
  have put the first day on Friday 28 February, the *bituun*, which the
  resolution does not.

No Bhutanese list read falls on a skipped or repeated day. So a Gregorian
year in which a holiday's number is skipped or repeated is not answered:
`Rule::TibetanDay` reports it as a gap, and the days of the holiday that
are not affected are still given. For deciding which Gregorian year that
is, a skipped day lies on the calendar day its lunar day ends in, the day
before the next number's.

**Worked example: Tsagaan Sar 2022.** Month 1 of the Mongolian year 2022
is regular, and its first two lunar days end on Wednesday 2 and Thursday 3
February, which are days 1 and 2. Lunar day 3 ends on 3 February as well,
before the dawn of the 4th, so the 4th is day 4 and no calendar day is day
3. The table gives Tsagaan Sar on 2 and 3 February and reports the third
day as a gap for 2022.

**Which Gregorian year.** A Tibetan year begins between late January and
March, so a date of Tibetan year *t* falls in Gregorian *t* or, for a late
month, *t* + 1: Bhutan's Traditional Day of Offering, the 1st of the 12th
month, is in January of the next Gregorian year, 30 January 2025 for the
year 2024. A Gregorian year is answered from the Tibetan years numbered one
less and the same.

**Taken as read, or predicted.** The days of the years whose dates were
read are carried as read and marked exact. Every other year is the
calendar's prediction, marked `Confidence::Approximate`: the days are
settled each year by the Government or the Ministry, and the Bhutanese
Government's calendar of 2003 disagreed with the arithmetic (below).

## What is carried

| Holiday | Date | Read, exact | Predicted |
| --- | --- | --- | --- |
| Tsagaan Sar, three days | days 1–3 of the first month, `mongolian` | 2025 (two days), 2026 | every other year |
| Buddha's Birthday | 15th of month 4 | — | from 2020 |
| Chinggis Khaan Day | 1st of month 10 | — | from 2012 |
| Traditional Day of Offering | 1st of month 12, `tibetan-bhutan` | 2025, 2026 | every other year |
| Losar, two days | days 1 and 2 of the first month | 2025, 2026 | every other year |
| Death Anniversary of Zhabdrung | 10th of month 3 | 2025, 2026 | every other year |
| Lord Buddha's Parinirvana | 15th of month 4 | 2025, 2026 | every other year |
| Birth Anniversary of Guru Rinpoche | 10th of month 5 | 2025, 2026 | every other year |
| First Sermon of Lord Buddha | 4th of month 6 | 2025, 2026 | every other year |
| Descending Day of Lord Buddha | 22nd of month 9 | 2025, 2026 | every other year |

The Mongolian years are those of the laws that added the days. Chinggis
Khaan Day's law of 8 November 2012 came before that year's day, which
the calendar puts on 14 November. The Bhutanese days are the present lists' and no years are
claimed for them, as for the table's Gregorian days.

Not carried:

- the rest days of 3–5 March 2025, a transfer to be worked back, like the
  Government's other transfers;
- the Winter Solstice and the Blessed Rainy Day, which both lists put on
  2 January and 23 September on different Bhutanese days. Janson defines
  the solstice as the day the mean solar longitude of the Bhutanese
  calendar reaches 250°, after Henning's program, and says it "will be
  3 January for the first time in 2020" [janson2014, Appendix A.4]. Taking
  the mean Sun of his (7.5) at the end of each lunar day gives 2 January
  in 2025 and 2026 but 3 January already in 2001, so that reading is not
  his. Henning's page could not be reached to settle it, and no rule was
  read for the Rainy Day;
- Dassain, Vijaya Dashami. The crate's Indian rule, Āśvina śukla 10 in the
  afternoon, puts it on 20 October 2026, a day before the list. A sunrise
  or midday tithi at Kathmandu or at the Indian station gives both lists'
  days, 2 October 2025 and 21 October 2026, but no source read says which
  convention the Ministry follows, and two years cannot choose it;
- the three, then, are taken from the lists, and another year is a gap.

## Accuracy

**The Bhutanese lists** (`the_bhutanese_calendar_rule_reproduces_both_lists`).
The rule for each of the eight days gives exactly the list's day in 2025
and in 2026. Every day of the Ministry's two calendars is the arithmetic's
([tibetan-variants.md](tibetan-variants.md)), so this is a check that the
dates read from the Dzongkha lists are the ones the rule states.

**Bhutan, Losar 2003**
(`bhutans_losar_of_2003_is_predicted_by_the_arithmetic_a_day_after_the_governments`).
Henning reports that the Government's calendar had the first day of the
first month on both 3 and 4 March 2003, where the arithmetic makes 3 March
a repeated 30th of the leap 12th month and Losar 4 March; Janson has "no
explanation for this discrepancy" [janson2014, Appendix A.13]. The table
predicts Losar 2003 on 4 and 5 March, marked approximate. The Government's
holidays of that year were not read.

**Mongolia** (`mongolia_dates_its_lunar_holidays_in_the_mongolian_calendar`).
The rule gives the days read for 2025 and 2026, and those MONTSAME reported
for 2020, 24–26 February, and 2021, 12–14 February, which are predicted
years [montsame-tsagaan-sar-2020; montsame-tsagaan-sar-2021;
montsame-tsagaan-sar-2026]. The New Year of the calendar is Janson's for
every year 2000–2030 ([tibetan-variants.md](tibetan-variants.md)). No
official date of Buddha's Birthday or of Chinggis Khaan Day was read; they
are predictions only.

**The gaps.** From 2027 to 2100 a lunar day of the Mongolian table is a gap
in 15 years — Tsagaan Sar in 11 — and of the Bhutanese in 23
(`mongolia_reports_a_skipped_or_repeated_lunar_day_as_a_gap_and_moves_nothing`,
`bhutan_predicts_its_bhutanese_calendar_days_beyond_the_lists`).

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [mn-law-public-holidays] | Articles 4.1.3, 4.1.8 and 4.1.10, and the dates of the laws that added them | Yes, 2026-09-26, legalinfo.mn |
| [mn-resolution-2025-109] | Tsagaan Sar 2025: the first day skipped, the second and third on the weekend, rest days 3–5 March | Yes, 2026-09-26, legalinfo.mn |
| [ikon-tsagaan-sar-2022] | Tsagaan Sar 2022, its third day skipped | Yes, 2026-09-26 |
| [montsame-tsagaan-sar-2020], [montsame-tsagaan-sar-2021], [montsame-tsagaan-sar-2026] | Tsagaan Sar 2020, 2021 and 2026 | Yes, 2026-09-26 |
| [moha-bt-calendar-2025], [moha-bt-calendar-2026] | The holiday lists and their Bhutanese dates | Yes, 2026-09-26 (the transcription in `hc-calendars-lunar`'s tests) |
| [janson2014] | The months by season, holidays not in leap months, the New Year in a leap month, Berzin's rule for skipped and repeated dates, the Bhutanese holidays and Winter Solstice, Losar 2003 | Yes, from the TeX source |
| [kalacakra-org] | Henning's Bhutanese program and holiday list | Not reachable on 2026-09-26 for this document; cited through Janson |
| Berzin, *Tibetan Astro Science* (1986) | The rule for skipped and repeated dates | Not read; cited through Janson |

## Code

- `crates/hc-holiday/src/rule.rs`: `Rule::TibetanDay`, `TibetanMonth`,
  `CalendarSystem::MONGOLIAN`, `CalendarSystem::TIBETAN_BHUTAN`, and the
  unit test `a_tibetan_day_is_a_gap_where_its_number_is_skipped_or_repeated`.
- `crates/hc-holiday/src/countries/asia.rs`: `MONGOLIA` and `BHUTAN`.
- `crates/hc-holiday/tests/countries.rs`: the tests named above.
