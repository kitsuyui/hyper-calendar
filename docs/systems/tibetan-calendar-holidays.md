# Holidays on the Tibetan calendar: Mongolia, Bhutan and the *düchen*

Backs the lunar days of the `MONGOLIA` (`MN`) and `BHUTAN` (`BT`) tables in
`hc-holiday` and its `buddhist-tibetan` tradition table, the rule shape
`Rule::TibetanDay` with its `TibetanMonth`, and
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

Janson's "usually" is not always. The Tibetan Nuns Project's list for
2024 keeps Chökhor Düchen, "Fourth day of the sixth lunar month", on 9
July [tnp-losar], and in the Phugpa calendar that is a day of the *leap*
month 6, which runs from 6 July to 4 August before the regular month 6 —
and the first of its two fourth days besides. The regular month's fourth
is 8 August. So for the *düchen* of `buddhist-tibetan` neither rule is
taken on trust: `TibetanMonth::Unrepeated(n)` is month *n* in a year that
has it once, and a Gregorian year that either copy of a repeated month
reaches is not answered, as a skipped or repeated day is not. Mongolia's
and Bhutan's days keep `Regular(n)`, which their lists and laws bear out.

**Which day.** A calendar day takes the number of the lunar day current at
its dawn, so now and then a number is skipped and another is repeated
on two days ([tibetan-phugpa.md](tibetan-phugpa.md)). Where a
holiday on such a number is kept is not settled by the sources read:

- Janson gives a general rule from Berzin — a skipped date's holiday on the
  day before, a repeated date's on the first of its two days — and adds
  that he has "not checked them against published calendars" [janson2014,
  §11].
- Henning's computed almanacs do otherwise: a festival on a repeated date
  is marked on the second of the two days, and one on a skipped date is
  not marked, as the Birth of the Buddha is not in 1966, 1975 and 1990
  [kalacakra-org-archive]. The Tibetan Nuns Project's Chökhor Düchen of 9
  July 2024, the first of two fourth days, follows Berzin's rule
  [tnp-losar]. Both are functions of `hc-calendars-regional`'s
  `tibetan_almanac`, `berzin_day` and `henning_almanac_day`
  ([tibetan-almanac.md](tibetan-almanac.md)).

Each is a named convention of `Rule::TibetanDay`, its `when`
(`hc_holiday::TibetanDayRule`): `Unsettled`, a gap; `Berzin`, the day
`berzin_day` gives; and `HenningAlmanac`, the day `henning_almanac_day`
gives, or none for a skipped number, which is an answer and not a gap.
Henning's almanacs settle the month as well: in a year that doubles the
month they mark the festival in both, the almanac for 2024 marking the
Turning of the Wheel on 10 July, in the leap month 6, and on 8 August, in
the regular one [kalacakra-org-archive]. The tables are three, one per
convention: `buddhist-tibetan`, `buddhist-tibetan-berzin` and
`buddhist-tibetan-henning`.
- In 2022 the third day of the first spring month was skipped. iKon.mn
  reported, from an unnamed source, three days off from 2 February, the
  Government having not yet discussed it [ikon-tsagaan-sar-2022].
- In 2025 the first day was skipped. The Government's resolution counted
  the holiday as the second and third days, "шинийн 2, 3-ны өдөр", on
  Saturday 1 and Sunday 2 March, and gave rest days on 3 to 5 March to be
  worked back within the year [mn-resolution-2025-109]. Berzin's rule would
  have put the first day on Friday 28 February, the *bituun*, which the
  resolution does not.

No Bhutanese list read falls on a skipped or repeated day, and neither
Mongolia's law nor its resolutions state a rule. So in the countries'
tables a Gregorian year in which a holiday's number is skipped or repeated
is not answered: `Rule::TibetanDay` with `TibetanDayRule::Unsettled`
reports it as a gap, and the days of the holiday that are not affected are
still given. For deciding which Gregorian year that
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
| Thimphu Drubchoe, in `BT-15` | 6th of month 8 | 2025, 2026 | every other year |
| Thimphu Tshechu, three days, in `BT-15` | 10th to 12th of month 8 | 2025, 2026 | every other year |
| Winter Solstice | the day the Bhutanese mean Sun reaches 250° | 2025, 2026 | every other year, by `bhutanese_winter_solstice` |

The Mongolian years are those of the laws that added the days. Chinggis
Khaan Day's law of 8 November 2012 came before that year's day, which
the calendar puts on 14 November. The Bhutanese days are the present lists' and no years are
claimed for them, as for the table's Gregorian days.

Not carried:

- the rest days of 3–5 March 2025, a transfer to be worked back, like the
  Government's other transfers;
- the Blessed Rainy Day, which both lists put on 23 September on
  different Bhutanese days, so it is solar; no rule was read for it. The
  Winter Solstice, on 2 January in both lists, is carried: Janson defines
  it as the day the mean solar longitude of the Bhutanese calendar reaches
  250°, after Henning's program, and says it "will be 3 January for the
  first time in 2020" [janson2014, Appendix A.4]. The instant the mean Sun
  reaches 250°, by Janson's rule for the almanac's special days, gives
  Henning's almanacs' days and times of 2001–2020 and both lists'
  2 January; it is `hc-calendars-regional`'s
  `tibetan_almanac::bhutanese_winter_solstice`
  ([tibetan-almanac.md](tibetan-almanac.md)), and the table predicts the
  day with it outside the lists' years, marked approximate;
- Dassain, Vijaya Dashami. The crate's Indian rule, Āśvina śukla 10 in the
  afternoon, puts it on 20 October 2026, a day before the list. A sunrise
  or midday tithi at Kathmandu or at the Indian station gives both lists'
  days, 2 October 2025 and 21 October 2026, but no source read says which
  convention the Ministry follows, and two years cannot choose it;
- those two, then, are taken from the lists, and another year is a gap.

### Bhutan's districts

The lists give two festivals for Thimphu only, with their Bhutanese
dates as the national days have theirs: Thimphu Drubchoe on 28 September
2025 and 17 September 2026, the 6th of the 8th month, and Thimphu Tshechu on
2–4 October 2025 and 21–23 September 2026, the 10th to the 12th
[moha-bt-calendar-2025; moha-bt-calendar-2026]. They are carried in the
Thimphu district, `BT-15`, as the national lunar days are: the lists' days
in their years, and elsewhere the prediction on `tibetan-bhutan`, marked
approximate. The Ministry changed the dates of both in 2021 by a
notification whose page gives no dates [moha-bt-notification-2021], which
is why a prediction is only that. Of the other districts' tshechus the
lists say that their days are "confirmed by the respective Dzongkhag
Administration".

| Code | Dzongkhag | Day | Instrument | First year |
| --- | --- | --- | --- | --- |
| BT-11 | Paro | tshechu: the Dzongkhag's confirmation, not read | | not carried |
| BT-12 | Chukha | as Paro | | not carried |
| BT-13 | Haa | as Paro | | not carried |
| BT-14 | Samtse | as Paro | | not carried |
| BT-15 | Thimphu | Thimphu Drubchoe, 6th of month 8; Thimphu Tshechu, 10th–12th of month 8 | the Ministry's lists for 2025 and 2026 | 2025 read; predicted before and after |
| BT-21 | Tsirang | as Paro | | not carried |
| BT-22 | Dagana | as Paro | | not carried |
| BT-23 | Punakha | as Paro | | not carried |
| BT-24 | Wangdue Phodrang | as Paro | | not carried |
| BT-31 | Sarpang | as Paro | | not carried |
| BT-32 | Trongsa | as Paro | | not carried |
| BT-33 | Bumthang | as Paro | | not carried |
| BT-34 | Zhemgang | as Paro | | not carried |
| BT-41 | Trashigang | as Paro | | not carried |
| BT-42 | Mongar | as Paro | | not carried |
| BT-43 | Pemagatshel | as Paro | | not carried |
| BT-44 | Lhuntse | as Paro | | not carried |
| BT-45 | Samdrup Jongkhar | as Paro | | not carried |
| BT-GA | Gasa | as Paro | | not carried |
| BT-TY | Trashiyangtse | as Paro | | not carried |

The district names are CLDR 48's English ones. Each Dzongkhag
Administration's confirmation is the source still to be read.

## Accuracy

**The Bhutanese lists** (`the_bhutanese_calendar_rule_reproduces_both_lists`).
The rule for each of the eight days gives exactly the list's day in 2025
and in 2026, and so does the rule for each of Thimphu's four
(`bhutan_predicts_thimphu_s_festivals_on_the_bhutanese_calendar`). Every day of the Ministry's two calendars is the arithmetic's
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

**The *düchen*** (`the_tibetan_duchen_are_the_tibetan_nuns_projects_2024_dates`,
`a_skipped_or_repeated_tibetan_day_is_a_gap_and_not_a_guess`,
`the_festivals_henning_marks_are_on_the_phugpa_days`). Losar,
Saga Dawa Düchen, the Universal Prayer Day and Lhabab Düchen of 2024 are
the Tibetan Nuns Project's 10 February, 23 May, 22 June and 22 November;
its Chökhor Düchen of 9 July is the leap-month day above, and 2024 reports
it as a gap and gives neither 9 July nor 8 August. Over 2000–2100 one of
the five is a gap 27 times, in 26 years. The four festivals Henning's
almanacs mark that the list does not — the Demonstration of Miracles on
the first fifteen days of the year, the Revelation of the Kalacakra Tantra
on 3/15, the Birth of the Buddha on 4/7 and the entry into the womb on
6/15 — fall on the almanacs' days of 2024 to 2026: 23 April and 14 May
2024, 28 February, 12 May, 2 June and 9 August 2025, 18 February, 1 May,
23 May and 29 July 2026.

**Berzin's rule** (`berzin_s_rule_keeps_a_skipped_day_on_the_day_before_and_a_repeated_one_on_the_first`).
Over 2000–2050 every day `buddhist-tibetan` gives, `buddhist-tibetan-berzin`
gives too, and every gap of a skipped or repeated number is answered; the
gaps left are the years that double the month, 2024's sixth among them.
The Birth of the Buddha of 1990, whose 7th is skipped, is 30 May, the 6th.

**Henning's almanacs** (`henning_s_almanac_marks_both_months_and_no_skipped_day`).
`buddhist-tibetan-henning`, on `tibetan-lochen`, gives every festival of
the almanacs for 1990 and 2024 to 2026 on the almanacs' day: in 2024 the
Turning of the Wheel on 10 July, the second of two fourth days of the leap
month 6, and on 8 August, and the entry into the womb on 21 July and
19 August; in 1990 no Birth of the Buddha. It reports no gap in any year
of 1960–2045.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [mn-law-public-holidays] | Articles 4.1.3, 4.1.8 and 4.1.10, and the dates of the laws that added them | Yes, 2026-09-26, legalinfo.mn |
| [mn-resolution-2025-109] | Tsagaan Sar 2025: the first day skipped, the second and third on the weekend, rest days 3–5 March | Yes, 2026-09-26, legalinfo.mn |
| [ikon-tsagaan-sar-2022] | Tsagaan Sar 2022, its third day skipped | Yes, 2026-09-26 |
| [montsame-tsagaan-sar-2020], [montsame-tsagaan-sar-2021], [montsame-tsagaan-sar-2026] | Tsagaan Sar 2020, 2021 and 2026 | Yes, 2026-09-26 |
| [moha-bt-calendar-2025], [moha-bt-calendar-2026] | The holiday lists and their Bhutanese dates | Yes, 2026-09-26 (the transcription in `hc-calendars-lunar`'s tests) |
| [moha-bt-notification-2021] | That the Ministry changed the dates of Thimphu Drubchoe and Tshechu in 2021 | Yes, 2026-09-29; the notification's dates are not on the page |
| [tnp-losar] | The *düchen* of 2024 and their lunar dates, Chökhor Düchen in the leap month 6 | Yes, 2026-09-26 |
| [janson2014] | The months by season, holidays not in leap months, the New Year in a leap month, Berzin's rule for skipped and repeated dates, the Bhutanese holidays and Winter Solstice, Losar 2003 | Yes, from the TeX source |
| [kalacakra-org] | Henning's Bhutanese program and holiday list | Read over plain HTTP 2026-09-29, for [tibetan-almanac.md](tibetan-almanac.md), which gives the solstice's 18;45 |
| [kalacakra-org-archive] | Henning's computed almanacs: where a festival on a skipped or repeated date is marked; the festivals, their English words and their days in 1990 and 2024–2026, both months of 2024 | Yes, 2026-09-29, Phugpa 1960–2045, and again 2026-09-29 for 1990 and 2024–2026 |
| Berzin, *Tibetan Astro Science* (1986) | The rule for skipped and repeated dates | Not read; cited through Janson |

## Code

- `crates/hc-holiday/src/rule.rs`: `Rule::TibetanDay`, `TibetanDayRule`, `TibetanMonth`,
  `CalendarSystem::MONGOLIAN`, `CalendarSystem::TIBETAN_BHUTAN`, and the
  unit test `a_tibetan_day_is_a_gap_where_its_number_is_skipped_or_repeated`.
- `crates/hc-holiday/src/countries/asia.rs`: `MONGOLIA` and `BHUTAN`;
  `crates/hc-holiday/src/countries/bhutan.rs`: `THIMPHU_DAYS`, joined to
  `BHUTAN`'s nationwide rules.
- `crates/hc-holiday/src/traditions.rs`: `BUDDHIST_TIBETAN` and
  `BUDDHIST_TIBETAN_BERZIN`, on `tibetan`, and `BUDDHIST_TIBETAN_HENNING`,
  on `tibetan-lochen`.
- `crates/hc-holiday/src/countries/bhutan.rs`: `winter_solstice`, the
  Winter Solstice outside the lists.
- `crates/hc-holiday/tests/countries.rs`, `tests/traditions.rs` and
  `tests/provincial_days.rs`: the tests named above, and
  `bhutan_keeps_thimphu_s_festivals_in_thimphu`.
