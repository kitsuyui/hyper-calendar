# The Burmese calendar: the eras of the Myanmar Era and the record's exceptions

Backs the identifier `burmese` in `hc-calendars-regional`, and the Thingyan
days of `hc-holiday`'s Myanmar table.

## What it is

**The calendar.** The Burmese calendar (မြန်မာပြက္ခဒိန်) is the lunisolar
calendar of Myanmar, on which the Buddhist full-moon holidays, the Thingyan
New Year festival and the traditional festivals fall, and which the
country's wall calendars print beside the civil date. It has twelve lunar
months, Tagu (တန်ခူး) first and Tabaung (တပေါင်း) last, of 29 and 30 days
alternately, the 29-day months called *yet-ma-son la* and the 30-day months
*yet-son la* [wikipedia-burmese-calendar]. The days of a month are counted
in two halves: waxing 1 to 15, the 15th being the civil full moon, and
waning 1 to 14 or 15, the last day of the month being the civil new moon
[yannaingaye2013, wikipedia-burmese-calendar]. A date is spoken by its
month, its half and its day: Nayon waxing 3, 1374 ME.

| Month | Burmese | Days |
| --- | --- | --- |
| 1 Tagu | တန်ခူး | 29 |
| 2 Kason | ကဆုန် | 30 |
| 3 Nayon | နယုန် | 29, or 30 in a big watat year |
| — First Waso | ပထမ ဝါဆို | 30, in a watat year only |
| 4 Waso | ဝါဆို | 30 |
| 5 Wagaung | ဝါခေါင် | 29 |
| 6 Tawthalin | တော်သလင်း | 30 |
| 7 Thadingyut | သီတင်းကျွတ် | 29 |
| 8 Tazaungmon | တန်ဆောင်မုန်း | 30 |
| 9 Nadaw | နတ်တော် | 29 |
| 10 Pyatho | ပြာသို | 30 |
| 11 Tabodwe | တပို့တွဲ | 29 |
| 12 Tabaung | တပေါင်း | 30 |

**Watat and yat-ngyin.** Twelve such months are 354 days, eleven short of
the year, so every second or third year takes an intercalary month, and the
year is then a *watat* (ဝါထပ်) year. The extra month is a *second Waso*: a
thirty-day month is inserted before Waso, and it is the inserted one, First
Waso (ပထမ ဝါဆို), that is the extra, the regular Waso becoming Second Waso
(ဒုတိယ ဝါဆို). Some watat years take an intercalary day as well, the
*yat-ngyin* (ရက်ငင်), which is added at the end of Nayon and makes that
month 30 days; a year with both is a *big watat* year of 385 days and a
year with only the month a *little watat* year of 384 days [yannaingaye2013,
wikipedia-burmese-calendar]. The day is never inserted in a year that has no
month — the rule that Thailand's calendar reverses, as
[thai-lunar.md](thai-lunar.md) says [wikipedia-burmese-calendar, citing
Irwin]. Waso's full moon opens the rains retreat, and in a watat year it is
Second Waso's.

**The eras.** The year count is the Myanmar Era (မြန်မာသက္ကရာဇ်, ME),
also called the Kawza era, whose year 0 began on 22 March 638 CE by the
reckoning of King Popa Sawrahan [wikipedia-burmese-calendar]. Its year is
not the tropical year but the *Sūrya Siddhānta*'s sidereal year of
1 577 917 828⁄4 320 000 = 365.258 756 5 days, the same constant that
[hindu-calendars.md](hindu-calendars.md) carries for the Indian systems,
with a mean lunation of 1 577 917 828⁄53 433 336 = 29.530 588 days
[yannaingaye2013, after Irwin]. How the calendar was computed changed
three times, and Yan Naing Aye, whose arithmetic this library carries,
divides the era into five stretches by what was in force [yannaingaye2013]:

| Era | Years ME | Years CE | Intercalation rule |
| --- | --- | --- | --- |
| Makaranta 1, "Poppasaw's epoch" | 0–797 | 638–1436 | The Metonic cycle |
| Makaranta 2, "the epoch of Mohnyin, King of Ava" | 798–1099 | 1436–1738 | The Metonic cycle |
| Thandeikta | 1100–1216 | 1738–1855 | The Metonic cycle, with a different full-moon offset |
| The British period | 1217–1311 | 1855–1950 | The excess days at the New Year, over the first four months |
| The Calendar Advisory Board | 1312 on | 1950 on | The excess days, over the first eight months |

Makaranta (မာကရန္တ) is the old reckoning, which Wikipedia's article, after
Ōhashi, traces to the *Thuriya Theiddanta* (the *Sūrya Siddhānta*) and says
placed its intercalary months and days on a nineteen-year Metonic schedule;
Thandeikta (သန္ဒိဋ္ဌ) is the reform of the monk Nyaunggan Sayadaw, which
kept mean months and years but changed their lengths and re-fixed the
Metonic schedule "to prevent further divergence between the solar and
luni-solar years" [wikipedia-burmese-calendar, citing Irwin 1909]. The two
sources disagree on when: Yan Naing Aye's table runs Thandeikta from
1100 ME (1738), Wikipedia says the system was proposed in 1200 ME (1838) and
fully adopted in 1853 (1215 ME). This library follows the arithmetic it
carries, so its "Thandeikta" era is Yan Naing Aye's 1100–1216. The *Sūrya
Siddhānta* excess-day method "was started using only in 1217 ME due to the
dominance of the old system that uses the 19-year Metonic cycle", and the
third era "uses the calculation method by the Myanmar Calendar Advisory
Board" [yannaingaye2013], the board that sits today in the Ministry of
Religious Affairs and Culture and keeps the calendar in step with the solar
year [wikipedia-burmese-calendar].

**The New Year is solar.** The year does not begin at a new moon. Its
boundary is the *atat* moment, the instant the mean Sun enters Aries by the
calendar's own year, which falls in the middle of April; the festival of
Thingyan (သင်္ကြန်) runs from the *akya* moment about two days before it,
through one or two *akyat* days, to the *atat* day, and the New Year's day
is the day after [yannaingaye2013]. So Tagu — and, when the year runs late,
Kason — is cut in two: the days before the New Year belong to the old year
as *Hnaung Tagu* (နှောင်းတန်ခူး, late Tagu), and the days from the New
Year's day on are *Oo Tagu* (ဦးတန်ခူး, early Tagu), and a Burmese date has
to say which [yannaingaye2013]. Thingyan is the country's principal holiday
and the government gazettes its days each year.

## How it works

Everything below is Yan Naing Aye's statement of the arithmetic
[yannaingaye2013], in his symbols where they help: SY the solar year, LM
the lunar month, MO the epoch, and Julian Day Numbers throughout, since
that is what the source uses. This library evaluates the same expressions in
floating point.

**The constants.**

| Constant | Value | What it is |
| --- | --- | --- |
| SY | 1 577 917 828⁄4 320 000 = 365.258 756 5 days | The solar year, Irwin's value |
| LM | 1 577 917 828⁄53 433 336 = 29.530 587 95 days | The mean lunation |
| MO | JD 1 954 168.050 623 | The *atat* moment that opened 0 ME, "estimated by substituting a few known values and taking the average" of recently published New Year times; the day is JD 1 954 168 = 22 March 638 Julian |
| 3739 | years | Added to the Myanmar year to give the elapsed years of the Kali Yuga, whose epoch comes out as JD 588 465.56 |

**The excess days and the watat decision.** The *atat* moment of year *y*
is SY·*y* + MO. The Sun's excess over whole lunations since the Kali Yuga
epoch at that moment is the *excess days*,

```text
ed = (SY · (y + 3739)) mod LM
```

Under the two excess-day eras a year is watat when the excess, counted over
the first NM months of the year, has grown past a lunation:

```text
TA = (SY/12 − LM) · (12 − NM)      if ed < TA then ed = ed + LM
TW = LM − (SY/12 − LM) · NM        watat if ed ≥ TW
```

with NM = 4 under the British and NM = 8 under the Board, so that TW is
22.269 45 days in the third era. Under the three Metonic eras the year is
watat when its remainder on division by 19 is 2, 5, 7, 10, 13, 15 or 18,
which the source writes as ⌊((7*y* + 2) mod 19) / 12⌋ = 1, and the excess
days are still computed because the full moon needs them. Wikipedia's
history of the Metonic schedule gives four different sets of remainders
over the centuries, this one as the set "from 1740"
[wikipedia-burmese-calendar]; the source applies it to the whole of
0–1216 ME and repairs the difference with the exceptions below.

**The full moon of Second Waso.** In a watat year the anchor of the whole
year is the day of Second Waso's full moon, four and a half lunations after
the mean new moon at the *atat*:

```text
w = round(SY · y + MO − ed + 4.5 · LM + WO)
```

WO is an offset that differs by era, −1.1 to 797 ME, −1.1 for 798–1099,
−0.85 for 1100–1216, −1.0 for 1217–1311 and −0.5 from 1312. The source
does not say how the offsets were chosen; they read as fitted to the
record, era by era.

**Little or big.** Let *w*₁ be the full moon of the nearest earlier watat
year, *yd* years back. The interval *w* − *w*₁ is *yd* common years of
354 days plus 30 or 31 days; if the remainder on division by 354 is 30 the
year is a little watat year, if 31 it is a big one and Nayon takes the
yat-ngyin [yannaingaye2013]. Any other remainder means the record's two
full moons are not consistent with the rule, which the source treats as an
error in the record and this library reports rather than accepts.

**The first day of Tagu** of any year, watat or not, is counted from the
previous watat year's full moon:

```text
tg1 = w₁ + 354 · yd − 102
```

The source states the figure without explaining it. It is arithmetic: in a
watat year the full moon of Second Waso is the 133rd day (Tagu 29, Kason
30, Nayon 29, First Waso 30, then waxing 15) or, with a yat-ngyin, the
134th, and the year has 384 or 385 days, so from the full moon to the last
day of the year is 252 days either way, and 354 − 102 = 252. From tg1 the
months follow in the order and lengths of the table above, and a day *k*
days after tg1 that runs past the year's 354, 384 or 385 days is the late
Tagu or Kason of the same year.

**Thingyan.** The *atat* time is SY·*y* + MO, and the *akya* time is
2.169 918 982 days before it, the length of the Thingyan "currently
recognized by the Myanmar Calendar Advisory Board", or 2.1675 days in the
years of the kings; each rounds to a day, *akyo* is the day before *akya*,
the *akyat* days are those between *akya* and *atat*, and the New Year's day
is the day after *atat* [yannaingaye2013]. The library applies the modern
length from 1312 ME.

**The exceptions, as data.** Where the record departs from the rule, the
source carries the departure as a table entry, and so does this library:
a full-moon exception moves *w* by a day or two for one year, and a watat
exception overrides the watat decision for one year.

| Era | Full-moon exceptions (year, shift) | Watat exceptions |
| --- | --- | --- |
| Makaranta 1 | 205 +1, 246 +1, 471 +1, 572 −1, 651 +1, 653 +2, 656 +1, 672 +1, 729 +1, 767 −1 | — |
| Makaranta 2 | 813, 849, 851, 854, 927, 933, 936, 938, 949, 952, 963, 968, 1039, all −1 | — |
| Thandeikta | 1120 +1, 1126 −1, 1150 +1, 1172 −1, 1207 +1 | 1201 is watat, 1202 is not |
| British | 1234 +1, 1261 −1 | 1263 is watat, 1264 is not |
| Board | 1377 +1 | 1344 is watat, 1345 is not |

Where each comes from: the third era's single full-moon exception is the
source's own finding — "when we check the results of the above equation
for all the past full moon days of Second Waso in the third era, all the
full moon days are consistent except ME 1377", whose full moon fell a day
later than the rule [yannaingaye2013]. For the earlier eras the source
tabulates the exceptions without saying where each was read; its references
are Irwin's *The Burmese & Arakanese Calendars* (1909), Ohn Kyaing's
*Myanmar Patkadain Thutaythana Kyan* (1964) and Tin Naing Toe's
*Myanmar–English Calendar* (1999), none read here, and the three watat
pairs 1201/1202, 1263/1264 and 1344/1345 are each a watat year moved one
year earlier than the rule places it. The source's own code, maintained
since, carries the same five tables unchanged as of its version of
9 September 2026 [yan9a-mmcal].

**Worked example: 1374 ME (2012–2013).** The source's own example, which
the test `the_sources_worked_example_of_1374_me_is_reproduced` holds.

1. *Is it watat?* 1374 is in the third era: NM = 8, WO = −0.5.
   ed = (365.258 756 5 × 5113) mod 29.530 588 = 24.1096 days; TA = 3.6306,
   so nothing is added; TW = 22.2695, and 24.1096 ≥ 22.2695, so 1374 is a
   watat year. No watat exception names it.
2. *The full moon of Second Waso.* SY × 1374 + MO = 501 865.531 4 +
   1 954 168.050 6 = 2 456 033.582 0; subtract ed and add 4.5 × LM − 0.5:
   2 456 033.582 0 − 24.109 6 + 132.887 6 − 0.5 = 2 456 141.860; rounded,
   w = JD 2 456 142, which is Thursday 2 August 2012. No full-moon
   exception names 1374.
3. *Little or big?* 1373 has ed = 13.2179 < TW and is common. 1372 has
   ed = 31.8568 and is watat, with w₁ = JD 2 455 404 (26 July 2010), so
   yd = 2. w − w₁ = 738 = 2 × 354 + 30: the remainder is 30, so 1374 is a
   little watat year and Nayon keeps 29 days.
4. *The first day of Tagu.* tg1 = 2 455 404 + 708 − 102 = JD 2 456 010,
   23 March 2012. The months then run Tagu 29, Kason 30, Nayon 29, First
   Waso 30, Waso 30, and Waso waxing 15 is day 29 + 30 + 29 + 30 + 15 =
   133 of the year, JD 2 456 142 — the full moon of step 2, as it must be.
5. *A day.* 23 May 2012 is JD 2 456 071, day 62 from tg1; Tagu and Kason
   take 59, so it is the third day of Nayon: Nayon waxing 3, 1374 ME, which
   is the source's example.
6. *The New Year.* The *atat* of 1374 is SY × 1374 + MO = JD 2 456 033.58,
   rounded to JD 2 456 034 = 16 April 2012, so the New Year's day is
   17 April 2012. The days from tg1 on 23 March to 16 April are therefore
   not 1374's at all: they are Hnaung Tagu of 1373, and 17 April 2012 is
   Tagu waning 11, 1374 ME. At the other end, the *atat* of 1375 falls on
   16 April 2013, which is Hnaung Tagu waxing 6, 1374 ME, and 17 April
   2013 opens 1375.

## What is carried

- **Identifier** `burmese` in `hc-calendars-regional`, with the month as
  `Month { ordinal, leap }`, First Waso being `Month::leap(4)` and
  preceding the regular Waso; the day counted 1 to 30 straight through,
  with the fortnight day and the phase (waxing, full moon, waning, new
  moon) as extra fields; and a `late` flag for Hnaung Tagu and Hnaung
  Kason, so that the two stretches of Tagu a year apart are distinct
  dates.
- **The five eras as data**: `ERAS` holds each era's first and last year,
  its full-moon offset, its NM (−1 for the Metonic rule) and its two
  exception tables, and nothing about an era lives anywhere else.
- **The range** 1 to 3000 ME, from the New Year's day of 1 ME (23 March 639
  Julian, 26 March 639 proleptic Gregorian) to the day before the New
  Year's day of 3001 ME (13 May 3639). The bounds are
  this library's: the source's arithmetic runs from 0 ME, and its third
  era is open-ended; the library refuses 0 ME because the days of Hnaung
  Tagu before the first New Year would need a year −1 it does not compute,
  and stops at 3000 because a rule fitted to the record has no claim on the
  far future.
- **The Thingyan**: `thingyan(year)` gives *akyo*, *akya*, the last *akyat*,
  *atat* and the New Year's day, which `hc-holiday`'s Myanmar table uses.
- **The consistency flag**: a watat year whose full moon is neither 30 nor
  31 days past the previous one, over the common years between, is
  reported as `inconsistent` rather than silently typed.
- **Not carried**: the Arakanese calendar, which puts the intercalary day in
  Tagu rather than Nayon, and the Thai and Cambodian reckonings, which
  place the day in a year of its own [wikipedia-burmese-calendar]; the
  planetary and astrological content of the Thingyan beyond its days; the
  Metonic schedules Wikipedia gives for other centuries, which the source
  replaces by one schedule plus exceptions; and any year the source's
  consistency check marks, which is reported rather than repaired.

## Accuracy

**What the tests check.** The source's worked example is reproduced
exactly, as above: the watat status of 1372–1374, both full moons, the
first day of Tagu and the date of 23 May 2012
(`the_sources_worked_example_of_1374_me_is_reproduced`). The five exception
tables are exercised at 1201/1202, 1263/1264, 1344/1345 and 1377
(`the_exceptions_of_the_record_are_applied`). The full moons of 1386 ME —
Kason 22 May, Waso 20 July, Thadingyut 17 October and Tazaungmon
15 November 2024 — are the days Myanmar's 2024 public holidays fell on, and
the Thingyan of 1386 ME ran from *akyo* on 13 April to the New Year's day
on 17 April 2024
(`the_full_moons_of_1386_me_fall_where_the_published_calendar_puts_them`,
`the_thingyan_of_1386_me_ran_from_13_to_17_april_2024`). The 2024 dates
were compared, on 2026-09-25, with Office Holidays' list for 2024
[officeholidays-myanmar-2024], which names no source; the government's own
notification for 2024 was not reachable, and the Ministry of Immigration
and Population's holiday pages [moip-public-holidays] serve their lists by
script and returned nothing to read. Every day of 2000–2030 round-trips
through the calendar, and a sample of days across the whole range does
(`every_day_of_three_decades_round_trips`).

**Re-read on 2026-09-25.** The module's dates for 1387 and 1388 ME were
compared with the lists that could be read that day. The Thadingyut
holidays of 25–27 October 2026 and the Tazaungdaing holidays of
23–24 November 2026 on the Myanmar National Portal
[myanmar-national-portal-holidays], and Siam Commercial Bank Myanmar's
list of the 2026 public holidays [scb-myanmar-holidays-2026] — Tabaung's
full moon 2 March, Kason's 30 April, Waso's 29 July, Thadingyut's 25–27
October, Tazaungmon's 23–24 November 2026 — all agree with the module,
which makes 1388 ME a big watat year with First Waso and puts Second
Waso's full moon on 29 July 2026. Office Holidays' 2025 list agrees for
Tabaung (13 March), Thingyan (13–16 April, New Year 17 April), Kason
(11 May), Thadingyut (6 October) and Tazaungmon (4 November 2025)
[officeholidays-myanmar-2024].

**Known disagreement.** For the full moon of Waso the module gives
Wednesday 9 July 2025 and Tuesday 12 July 2022; Office Holidays gives
Thursday 10 July 2025 and Calendar Labs' table Wednesday 13 July 2022
[officeholidays-myanmar-2024, calendarlabs-waso], each a day later. Both
later dates are the days of the astronomical full moon, which is what a
secondary list may have used; the source's own code, in its version of
9 September 2026, carries no exception for 1384 or 1387 ME [yan9a-mmcal];
and no official notification for either year could be read. The
disagreement is recorded here and not resolved. If the gazette kept
10 July 2025, the module's way to carry it is a full-moon exception
(1387, +1) in the third era's table, as 1377's is.

**What the source measured.** Against timeanddate.com's astronomical full
moons for the seventeen watat years 1350–1396 ME, the source reports an
average difference of 1.4 hours between its computed full-moon time and
the astronomical one [yannaingaye2013]; this library did not repeat the
measurement.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [yannaingaye2013] | Every constant, the excess-day and Metonic rules, the full-moon formula and its offsets, the little/big decision, tg1, the Thingyan lengths, the five era tables with their exceptions, the 1377 finding, the 1374 example, and the references to Irwin, Kyaing and Toe | Yes, 2026-09-25; the module read it 2026-09-22 |
| [yan9a-mmcal] | The author's maintained code, whose era tables were checked against the article's | Yes, 2026-09-25: the JavaScript (version 20260909) and C++ (version 20250726) sources |
| [irwin1909] | The constants and the Makaranta and Thandeikta history, through the two sources above | Not read |
| [wikipedia-burmese-calendar] | The month names in Burmese script, the 29- and 30-day months, the intercalary day's placement and the rule that it needs the month, the Arakanese and Thai placements, the Makaranta and Thandeikta history with its 1838 and 1853 dates, the Metonic remainder sets, the Calendar Advisory Board, the epoch of 22 March 638 | Yes, 2026-09-25; the module read it 2026-09-22 |
| [hindu-calendars.md](hindu-calendars.md) | The *Sūrya Siddhānta*'s sidereal year, the same constant | This repository |
| [officeholidays-myanmar-2024] | The 2024 and 2025 holiday dates compared above; the 10 July 2025 disagreement | Yes, 2026-09-25 |
| [scb-myanmar-holidays-2026] | The 2026 holiday dates compared above | Yes, 2026-09-25 |
| [myanmar-national-portal-holidays] | The Thadingyut and Tazaungdaing holidays of 2026 | Yes, 2026-09-25 |
| [calendarlabs-waso] | The 13 July 2022 disagreement, and 2023, 2024 and 2027 in agreement | Yes, 2026-09-25 |
| [moip-public-holidays] | The government's holiday lists for 2025 and 2026 | Not readable: the pages returned no list |

Statements the module documentation made before this write-up that no
source read here supports, trimmed or qualified in it and recorded so that
they are not mistaken for sourced: that the year is "sidereal" — the
article calls it the solar year, and the word rests only on the constant's
identity with the *Sūrya Siddhānta*'s; that the source "reconciles" the
calendar with the published calendars "era by era" — the article says so
of the third era and tabulates the earlier eras' exceptions without saying
against what; that "the kings" used the Metonic cycle "to 1216 ME" — the
article's own boundary, which Wikipedia dates differently; and that a gap
of neither 30 nor 31 days is one "the source flags as an error in the
record" — the article says only that such a case would be disputed. The
Burmese script of the month names was read from Wikipedia by the module's
author and not re-checked glyph by glyph here.

## Code

`crates/hc-calendars-regional/src/burmese.rs`: the constants `SOLAR_YEAR`,
`LUNAR_MONTH`, `EPOCH_JULIAN_DATE` and `KALI_YUGA_OFFSET`, the table `ERAS`
of `EraRule`, and the functions `watat`, `year_info`, `new_year_day`,
`thingyan`, `month_length`, `from_fixed` and `to_fixed`. Anchors:
`the_sources_worked_example_of_1374_me_is_reproduced`,
`the_exceptions_of_the_record_are_applied`,
`the_full_moons_of_1386_me_fall_where_the_published_calendar_puts_them`,
`every_day_of_three_decades_round_trips`, `impossible_dates_are_refused`,
`the_thingyan_of_1386_me_ran_from_13_to_17_april_2024`. Myanmar's holidays
on the calendar are in `crates/hc-holiday`, `MM` in `countries/asia.rs`,
through `CalendarSystem::BURMESE` and `burmese::thingyan`; the Latin month
names are in `hc-i18n`.
