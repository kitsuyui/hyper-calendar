# The Tibetan calendar: the Phugpa arithmetic

Backs the identifier `tibetan` in `hc-calendars-lunar`.

## What it is

**The calendar.** The Tibetan calendar descends from the Indian tradition
through the *Kālacakra Tantra*, translated into Tibetan in the eleventh
century — the traditional date is 1027, when the first sixty-year cycle
begins — and standard in Tibet from the second half of the thirteenth
[janson2014, §1]. Its months are lunar, new moon to new moon, but numbered
by the solar months they correspond to, and its days are numbered by the
lunar days they correspond to. Neither correspondence is exact, so now and
then two months carry the same number, the first of them being the leap
month, and now and then a day number is skipped, or two calendar days carry
the same number, the first of them being the leap or *extra* day; there are
never skipped months [janson2014, §1]. A year is twelve or thirteen months
and 354, 355, 383, 384 or 385 days long [janson2014, §3].

**The traditions.** Different schools follow different rules for the
details, and two are in general use: the *Phugpa* (*phug-lugs*), begun in
1447 by Phugpa Lhundrub Gyatso, used by the Tibetan government from at
least 1696 to 1959, by the Gelug, Sakya, Nyingma and Shangpa Kagyu
traditions, by the Dalai Lama, and in the almanacs published at
Dharamsala; and the *Tsurphu* (*mtshur-lugs*), also from 1447, of the Karma
Kagyu, published from Rumtek. Bhutan and Mongolia keep versions of their
own. The versions "frequently differ by a day or a month" [janson2014, §1
and Appendix A]. This document and the library carry the Phugpa version,
which Janson calls the standard one.

**Years.** A Tibetan year is named in the sixty-year cycle by element,
gender and animal — Wood, Fire, Earth, Iron or Water, each for two years,
male then female; Mouse, Ox, Tiger, Rabbit, Dragon, Snake, Horse, Sheep,
Monkey, Bird, Dog, Pig — so that 2007 is Fire–female–Pig, or simply
Fire–Pig. It is also placed in the Indian cycle of sixty names beginning
with Prabhava (*rab byung*), the cycles numbered from 1027, and 2007 is the
21st year of the 17th cycle, which began in 1987 [janson2014, §4]. For a
number, Tibetans and Westerners commonly use the Gregorian year in which
the Tibetan year begins, which is what this library does; a count from
127 BCE, the traditional ascent of the first king, is also in use, and
gives the year beginning in 2024 as 2151 [janson2014, §4]. The year begins
with month 1 — with the leap month 1 when there is one, as in 2000 — and
its first day is Losar (*lo-gsar*), the New Year, which at present falls
between 5 February and the first week of March [janson2014, §1 and §4].

**Days.** The Tibetan calendar day runs from dawn to dawn and is 24 hours
long; Henning fixes its start at mean daybreak, 5 a.m. local mean solar
time [janson2014, §3 and Remark 6, citing henning2007, pp. 10–11]. The
lunar day (*tshes-zhag*, Sanskrit *tithi*) is the time in which the Moon's
elongation from the Sun grows by 1/30 of a circle, 12°; it varies between
about 21.5 and 25.7 hours, and there are thirty in every month, day 1
beginning at new moon [janson2014, §3 and Remark 15].

## How it works

Everything below is Svante Janson's statement of the Phugpa calculations
[janson2014], in his notation and with his equation numbers, from the epoch
of the *Kālacakra Tantra*, month 3 of the year 806. The calendar uses two
formulas for each quantity, a linear *mean* one and a corrected *true* one;
"true" means the calendar's own corrected value, not the astronomical one,
from which it now differs by about 2° of elongation and 36° of solar
longitude [janson2014, §3 and §12].

**Months.** The calendar rests on the relation 67 lunar months = 65 solar
months, taken as exact (5.1), so there are two leap months in every 65
solar months and the leap months recur in a cycle of 65 years, each of the
twelve numbers being doubled exactly twice in it [janson2014, §5]. Every
month has a *true month* count *n* from the epoch. For month *M* of year
*Y*, with the epoch *Y*₀ = 806, *M*₀ = 3 and the epoch's intercalation
index β* = 61:

```text
M' = 12 (Y − 806) + M − 3                              (5.2)
ix = (2 M' + 61) mod 65                                 (5.7)
a leap month M is inserted when ix = 48 or 49           (5.8)
n  = ⌊(67 M' + 61 + 17) / 65⌋ − [leap]                  (5.10)
```

The rational true month 67*M*'/65 + 61/65 is rounded down when ix < 48 and
up when ix ≥ 48, except that a leap month always rounds down (5.9), so
that the leap month and the regular month of the same number get
consecutive counts. The inverse, with β = 184 − β* = 123 (5.15):

```text
x = ⌈(65 n + 123) / 67⌉,  M = x amod 12,  Y = ⌈x / 12⌉ − 1 + 806   (5.19)–(5.21)
leap = (65 n + 123) mod 67 ∈ {1, 2}                                (5.22)
```

From these follow two rules that are not traditional but exact
[janson2014, §5.5]: *Y* is a leap year if and only if
(24 *Y* + 33) mod 65 ≥ 41 (5.41), and its leap month is then
1 + ⌊(64 − (24 (*Y* − 806) − 123) mod 65) / 2⌋ (5.34). So 2024 is a leap
year with a leap month 6, 2000 with a leap month 1, and 2023 is not.

**The mean motions.** Everything is a rational number, written in the
almanacs in mixed radices and here as the fraction it is. For lunar day *d*
of true month *n*, at the end of the lunar day (7.1), (7.5), (7.11):

| Quantity | Formula | Constants |
| --- | --- | --- |
| mean date, a Julian Date | *n*·*m*₁ + *d*·*m*₂ + *m*₀ | *m*₁ = 167 025⁄5 656 (29.530 587 days), *m*₂ = *m*₁/30 = 11 135⁄11 312, *m*₀ = 2 015 501 + 4 783⁄5 656 |
| mean Sun, in revolutions | *n*·*s*₁ + *d*·*s*₂ + *s*₀ | *s*₁ = 65⁄804, *s*₂ = *s*₁/30 = 13⁄4 824, *s*₀ = 743⁄804 |
| Moon's anomaly, in revolutions | *n*·*a*₁ + *d*·*a*₂ + *a*₀ | *a*₁ = 253⁄3 528, *a*₂ = 1⁄28, *a*₀ = 475⁄3 528 |

The Julian Date is Janson's addition: the tradition counts the date modulo
7, as a day of the week, and he adds 2 015 501 ≡ −2 (mod 7) to the
traditional *m*₀ so that the integer part is the JD outright (Remark 8).
The anomaly is measured from apogee. The almanacs' *a*₂ = 1⁄28 is a rounded
value; the exact (1 + *a*₁)/30 = 3 781⁄105 840 proposed by Minling Lochen
Dharmashri and used by Henning (7.24) moves the calendar on about one day
in 4 100, "a little less than one day in 10 years", the next such day being
19 November 2025 [janson2014, Remark 14].

**The corrections.** Two short tables stand in for a sine, linearly
interpolated between integers and extended by symmetry, tab(2*h* − *i*) =
tab(*i*) and tab(2*h* + *i*) = −tab(*i*):

```text
moon_tab(i), i = 0..7, period 28:  0, 5, 10, 15, 19, 22, 24, 25     (7.18)
sun_tab(i),  i = 0..3, period 12:  0, 6, 10, 11                     (7.21)
moon_equ = moon_tab(28 · anomaly_moon)                               (7.17)
sun_equ  = sun_tab(12 · (mean_sun − 1/4))                            (7.19), (7.20)
true_date = mean_date + moon_equ/60 − sun_equ/60                     (7.22)
```

**Days.** A calendar day is labelled by the lunar day current at its
beginning, which is to say that a lunar day gives its number to the
calendar day in which it ends: JD = ⌊true_date⌋ (8.1). If two lunar days
end in the same calendar day, the day takes the first one's number and the
second's number is skipped; if no lunar day ends in a calendar day, the day
takes the number of the next one, so that number is repeated, and the first
of the two is the extra day, marked "Extra" in the almanacs [janson2014,
§6]. New moon is by definition the end of lunar day 30 and full moon the
end of lunar day 15, so unless the day is skipped the calendar's new moon
is day 30 and its full moon day 15 [janson2014, §6]. No sunrise is ever
computed: the true date is a local Julian Date that takes integer values at
mean dawn rather than at noon UT (Remark 6). The first day of a month is
the day after day 30 of the month before, whether or not that day 30 is
skipped, and Losar is the day after day 30 of the regular month 12 of the
year before [janson2014, §8].

**The sixty-year names.** Year *Y* is number (*Y* − 3) amod 60 in the
Chinese cycle, whose position mod 10 gives the element (two years each) and
the gender (odd positions male), and mod 12 the animal; it is number
(*Y* − 6) amod 60 in the Prabhava cycle ⌈(*Y* − 1026)/60⌉ [janson2014, §4].

**Worked example: Losar 2024.** The year of the Wood–Dragon, whose Losar
the test `losar_falls_on_the_published_days_of_recent_years` holds.

1. *The month before.* Losar is the day after day 30 of the regular month
   12 of 2023. *M*' = 12 × (2023 − 806) + 12 − 3 = 14 613;
   ix = (2 × 14 613 + 61) mod 65 = 29 287 mod 65 = 37, not 48 or 49, so it
   is not a leap month; *n* = ⌊(67 × 14 613 + 78)/65⌋ = ⌊979 149/65⌋ =
   ⌊15 063.83⌋ = 15 063.
2. *The mean quantities* at *d* = 30, *n* = 15 063: mean_date = 15 063 ×
   167 025⁄5 656 + 30 × 11 135⁄11 312 + 2 015 501 + 4 783⁄5 656 =
   2 460 350.608 0; mean_sun = frac(15 063 × 65⁄804 + 30 × 13⁄4 824 +
   743⁄804) = 631⁄804 = 0.784 8; anomaly_moon = frac(15 063 × 253⁄3 528 +
   30⁄28 + 475⁄3 528) = 713⁄1 764 = 0.404 2.
3. *The corrections.* 28 × 0.404 2 = 11.317 5, which is past 7, so
   moon_tab(14 − 11.317 5) = moon_tab(2.682 5) = 10 + 0.682 5 × 5 = 13.413.
   12 × (0.784 8 − 0.25) = 6.418, which is past 6, so
   sun_equ = −sun_tab(0.418) = −0.418 × 6 = −2.507.
4. *The true date.* 2 460 350.608 0 + 13.413/60 + 2.507/60 =
   2 460 350.873; its integer part, JD 2 460 350, is Friday 9 February
   2024, the calendar day in which lunar day 30 ends. Losar is the next
   day, JD 2 460 351, Saturday 10 February 2024 — the day the Tibetan
   Nuns Project and the almanacs it follows gave for the Wood Dragon year
   2151 [tnp-losar].
5. *The name.* (2024 − 3) amod 60 = 41: position 41 mod 10 = 1 is the
   first Wood year, male; 41 mod 12 = 5 is the Dragon. Wood–male–Dragon.
   In the Prabhava cycle it is (2024 − 6) amod 60 = 38, the 38th year of
   cycle ⌈998/60⌉ = 17.

**Worked example: a skipped and an extra day.** Janson's Table 8 lists the
skipped and repeated days of 2012 for four traditions; for the Phugpa,
month 1 has day 5 repeated and day 19 skipped. Month 1 of 2012 has
*n* = 14 916. Lunar day 4 ends at true_date 2 455 983.961, JD 2 455 983
(25 February); lunar day 5 ends at 2 455 985.032, JD 2 455 985
(27 February). No lunar day ends in JD 2 455 984, 26 February, so that
day takes the next number: 26 February is 5, the extra day, and
27 February is 5 again. Lunar day 18 ends at 2 455 998.081 and lunar day
19 at 2 455 998.985, both in JD 2 455 998, 11 March, which is therefore
day 18; there is no day 19, and 12 March is day 20. The module marks the
26th `leap_day` and refuses a date 19 of that month.

## What is carried

- **Identifier** `tibetan` in `hc-calendars-lunar`, English name "Tibetan
  (Phugpa)", with the month as `Month { ordinal, leap }`, the leap month
  preceding the regular month of the same number; the day 1 to 30; and
  `leap_day` for the first of two calendar days with the same number. A
  date reads `2000-1L-1` or `2024-6-15x`.
- **Exact arithmetic.** Every constant is the rational Janson states, held
  as a reduced `i128` fraction, and every true date is computed exactly and
  floored; nothing is floating point. The tables are his (7.18) and (7.21),
  the anomaly increment the almanacs' 1⁄28.
- **The epoch** of 806, month 3, β* = 61, and the inverse's β = 123, so
  that the same calendar is produced as from Henning's epoch of 1927 or the
  almanacs' of 1987, as Janson shows (Remark 5).
- **The day boundary** at 05:00 local mean solar time, `DAWN`, reported
  through `Calendar::day_boundary` so that a caller placing an instant
  knows when the Tibetan day turns.
- **The year's names**: `year_name` gives element, gender and animal,
  `prabhava` the cycle and the year in it; the Sanskrit and Tibetan names
  of the sixty years are not carried.
- **The range** 1000 to 3000, Losar of 1000 to the day before Losar of 3001.
  The bounds are this library's: the Phugpa rules date from 1447 and the
  arithmetic from 806, and no source names 1000 or 3000; the lower bound
  keeps the range inside the era the first *rab byung* cycle opens, and the
  upper is where the other arithmetic calendars here stop.
- **Not carried**: the Tsurphu, Mongolian and Bhutanese versions and the
  *Kālacakra* *karaṇa* calculation, as below; Henning's exact *a*₂; the
  almanac's other components — the five *lnga-bsdus* are the day of week,
  the lunar day, the lunar mansion, the *yoga* and the *karaṇa*, and the
  almanac also prints the true date's fraction, the true solar longitude
  and the planets [janson2014, §10 and Appendix D]; the holidays of
  Henning's Appendix II and the rule that a holiday on a skipped date moves
  to the day before [janson2014, §11]; the 127 BCE year count, which is
  *Y* + 127.

**What the other traditions would change.** Janson's Appendix A. All four
versions share the mean motions *m*₁, *s*₁, *a*₁ and differ in their epoch
values *m*₀, *s*₀, *a*₀; recomputed to the common epoch of JD 2 015 531
(23 March 806 Julian), the Tsurphu mean date is 0.046 day later than the
Phugpa and its mean Sun 4.78° further on, so the two dates differ on
average one day in twenty, "typically one or two days in a month", and when
they differ the Phugpa date is the larger by one [janson2014, Appendix A.13].
The Tsurphu leap rule is the simpler *ix* = 0 or 1, with β* = 59 from its
1732 epoch (JD 2 353 745, *m*₀ = 2 353 745 + 1 795 153⁄7 635 600,
*s*₀ = −5 983⁄108 540, *a*₀ = 207⁄392), which puts its leap months about
seven months from the Phugpa's — month 8 where the Phugpa has month 1 in
2000 — and moves Losar by a month in 2003, 2006, 2011, 2014, 2022 and 2030
and by a day in 2008 and 2025, when the Tsurphu Losar is 1 March rather
than 28 February [janson2014, Appendix A.2 and Tables 7–9]. The
Mongolian version has the Tsurphu leap months and differs from it by a few
days a year; the Bhutanese changes all three epoch values and numbers its
leap months differently; the *karaṇa* calculation of the *Kālacakra Tantra*
uses other solar constants, and some Tsurphu almanacs have used its solar
equation in the true date, which shifts a skipped or repeated day about
five times a year [janson2014, Appendix A]. Each is a parameter set over
this engine — an epoch, three epoch values and a leap rule — and none is
carried until its almanacs can be checked.

## Accuracy

**What the tests check.** Janson's own datelines: 31 December 2007 is
"Sunday 23, month 11, Fire–Pig year" and 8 January 2014 "Wednesday 8,
month 11, Water–Snake year" [janson2014, title page]; 2007 is the 21st year
of the 17th Prabhava cycle [janson2014, §4]; and Losar 2000 was "Sunday,
February 6th, which was the first day of leap month 1", as the Dalai Lama's
monastery reported it [janson2014, Remark 17, citing Salden]
(`the_sources_own_dates_and_losar_of_2000_are_reproduced`). The Losars of
2023–2026 — 21 February 2023, 10 February 2024, 28 February 2025,
18 February 2026 — and Saga Dawa Düchen, the fifteenth of the fourth
month, on 23 May 2024, are the days the Tibetan Nuns Project published for
the Water Hare, Wood Dragon, Wood Snake and Fire Horse years [tnp-losar],
which names no almanac, and the Central Tibetan Administration's Losar
greeting names 2152 as the Wood Snake year [tibet-net-losar-2152]
(`losar_falls_on_the_published_days_of_recent_years`). The leap years of
2000–2030 follow (5.41), 2024's leap month is 6, and the inverse
(5.19)–(5.22) is exact over nine hundred consecutive months
(`leap_years_follow_the_sixty_five_year_rule`). Every day of 1990–2030
round-trips, at least one day is skipped and one repeated in the span, and
every year has one of the five lengths, thirteen months exactly when (5.41)
says so (`every_day_of_four_decades_round_trips_and_years_have_the_five_lengths`).

**Measured on 2026-09-25 against the paper's tables.** Janson prints the
Phugpa Losar for every year of the last and current sixty-year cycles,
1927–2046, in his Table 1: the module reproduces all 120. His Table 7
gives the leap months 2000–2019 — 1, 10, 6, 3, 11, 8, 4, 1 in 2000, 2002,
2005, 2008, 2010, 2013, 2016 and 2019 — and his Table 8 every skipped and
repeated day of 2012: the module reproduces both, the second exactly as
worked above (month 1: 5 repeated, 19 skipped; month 2: 9 repeated, 12
and 25 skipped, 27 repeated). Those tables are Janson's computation of the
same arithmetic, so this is agreement with the source, not with an almanac;
Janson himself checked his calculations against Men-Tsee-Khang's almanac
for 2013 [janson2014, §1], and this library has not.

**Known disagreements.** None with a published date. The almanacs' 1⁄28
and Henning's exact *a*₂ give different calendars on about one day in
4 100, the next being 19 November 2025 [janson2014, Remark 14]; the
module carries the almanacs' value, and the Losar dates above do not
depend on the choice.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [janson2014] | The whole arithmetic: the epoch, (5.1)–(5.41), the mean motions and tables (7.1)–(7.24), the day rule (8.1), Remarks 5, 6, 8, 14, 15 and 17, the sixty-year cycle, the mean year, Appendix A on the other traditions, and Tables 1, 7 and 8 | Yes, 2026-09-25, from the TeX source on arXiv, the PDF not being renderable here; the module read the PDF 2026-09-22. Equation numbers are those of the arXiv version |
| [henning2007] | Mean daybreak at 5 a.m., the Phugpa and Tsurphu histories and epoch data, the holidays of Appendix II | Not read; cited through Janson |
| [kalacakra-org] | Henning's epoch data, calendar archive and open-source Phugpa and Tsurphu programs | Not read: on 2026-09-25 the host presented a certificate for another domain |
| [tnp-losar] | Losar 2023–2027 with the year names, Saga Dawa Düchen 2024 | Yes, 2026-09-25 |
| [tibet-net-losar-2152] | The Central Tibetan Administration's name for the year beginning in 2025 | Yes, 2026-09-25; the page carries no Gregorian date |
| Salden, *The story of Losar* | Losar 2000 on Sunday 6 February | Not read; Janson's citation, the page gone by 2013 |
| Men-Tsee-Khang, *Tibetan Annual Almanac 2013* | Janson's own check | Not read |

Statements the module documentation made before this write-up that no
source read here supports, trimmed in it and recorded so that they are not
mistaken for sourced: that the Gregorian numbering is how Tibetans
"commonly" number the year — Janson says it is common "especially among
Westerners" and that Tibetans use both it and the 127 BCE count; and the
range 1000–3000, which is this library's choice. The equation, remark and
section numbers the module cites were checked against the TeX source's
order and agree.

## Code

`crates/hc-calendars-lunar/src/tibetan.rs`: the constants `EPOCH_YEAR`,
`EPOCH_MONTH`, `EPOCH_INDEX`, `INVERSE_CONSTANT`, `DAWN`, the rationals
`M1`, `M2`, `M0`, `S1`, `S2`, `S0`, `A1`, `A2`, `A0` and the tables
`MOON_TABLE` and `SUN_TABLE`; the functions `true_month_count`,
`month_of_count`, `is_leap_year`, `leap_month_of`, `new_year`,
`year_name`, `prabhava`, `from_fixed` and `to_fixed`. Anchors:
`the_day_begins_at_mean_daybreak`,
`the_sources_own_dates_and_losar_of_2000_are_reproduced`,
`losar_falls_on_the_published_days_of_recent_years`,
`leap_years_follow_the_sixty_five_year_rule`,
`every_day_of_four_decades_round_trips_and_years_have_the_five_lengths`,
`impossible_dates_are_refused`. The Tsurphu, Mongolian and Bhutanese rows
of the roadmap in [calendars.md](../calendars.md) wait on this engine.
