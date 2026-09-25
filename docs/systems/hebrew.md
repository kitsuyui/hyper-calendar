# The Hebrew calendar: the molad, the nineteen-year cycle and the four dehiyyot

Backs the identifier `hebrew` in `hc-calendars-lunar`.

## What it is

The calendar of Jewish religious life, and one of the two civil calendars of
the State of Israel: a lunisolar calendar of twelve or thirteen months whose
days begin at sunset, whose year begins in autumn with Tishrei, and whose
years are counted in the era of creation (*Anno Mundi*, AM). 1 Tishrei AM 1
is Monday 7 October 3761 BCE in the proleptic Julian calendar
[reingold2018code, `hebrew-epoch`; wikipedia-hebrew-calendar], which in this
library is RD −1 373 427. The count is retrospective: nothing was dated in
AM 1, and the calendar's own rules were not in force for most of the years
it numbers.

Two things distinguish it from the calendars around it. It is **fixed**:
since late antiquity the months have not been declared from the sighting of
the crescent, as they were in the time of the Second Temple and the
Sanhedrin, but computed from the *molad*, the mean conjunction, by
arithmetic on integers that anyone can repeat. And it is **postponed**: the
first day of the year is moved off certain weekdays and away from certain
molad times by four rules, the *dehiyyot*, so that the festivals fall where
the law needs them and every year has one of six permitted lengths.

Tradition attributes the fixed calendar to the patriarch Hillel II in the
year 670 of the Seleucid era, 358/9 CE; the attribution first appears in a
responsum of Hai Gaon of 992. Modern scholarship reads it narrowly: a
Cairo Geniza letter of 835/6 shows festivals kept on days the present
calendar does not give, and the calendar did not reach its exact modern
form until the years 922–924; Sacha Stern takes Hai Gaon to attribute only
the nineteen-year cycle to Hillel [wikipedia-hillel-ii]. The earliest
complete statement of the rules read for this document is Maimonides',
*Hilchot Kiddush HaChodesh* (the Laws of the Sanctification of the New
Month) in the *Mishneh Torah*, of about 1178 [maimonides-kiddush-hachodesh],
and it is the rules as he states them that this library carries.

## How it works

### The molad and its parts

The month is taken as exactly 29 days, 12 hours and 793 *ḥalakim* (parts),
an hour being 1 080 parts — a number chosen, Maimonides says, because it
divides evenly by 2, 3, 4, 5, 6, 8, 9 and 10 [maimonides-kiddush-hachodesh,
ch. 6]. A day is 25 920 parts and a month 765 433 parts, so the interval is
29.530 594 days. Hours are counted from six in the evening, the
conventional start of the Hebrew day, and the molad of a month is the
*mean* conjunction: the true one can be up to about fifteen hours from it.

The first molad, the molad of Tishrei of AM 1, is the one the tradition
calls *BaHaRaD* after its Hebrew numerals: day 2 (Monday), 5 hours and 204
parts after the evening [maimonides-kiddush-hachodesh, ch. 6]. Every other
molad is that plus a whole number of months. In the library's terms the
molad of Tishrei AM 1 is 876 parts before the midnight that begins RD
−1 373 427 [reingold2018code, `molad`].

### The nineteen-year cycle

Twelve lunar months fall 10 days, 21 hours and 204 parts short of a solar
year of 365¼ days [maimonides-kiddush-hachodesh, ch. 6], so a thirteenth
month is added in seven years of every nineteen: the 3rd, 6th, 8th, 11th,
14th, 17th and 19th of the cycle [maimonides-kiddush-hachodesh, ch. 6].
Nineteen years then hold 235 months. The closed form is that year *y* is
leap when (7*y* + 1) mod 19 < 7 [reingold2018code, `hebrew-leap-year?`],
and the number of months from the epoch to Tishrei of year *y* is
⌊(235*y* − 234) / 19⌋ [reingold2018code,
`hebrew-calendar-elapsed-days`].

The extra month is a second Adar. Maimonides' month order runs Nisan,
Iyyar, Sivan, Tammuz, Av, Elul, Tishrei, Marḥeshvan, Kislev, Ṭevet, Shevaṭ,
Adar, with the leap year's added month *Adar I* before an *Adar II*
[maimonides-kiddush-hachodesh, ch. 8]; the year number changes at Tishrei.
Which Adar is the "real" one — the one Purim falls in — is the second, so
in the Talmud's sense the added month is the first. This library numbers
the months from Tishrei in its public interface and, following Reingold
and Dershowitz, from Nisan internally; the next section says how the two
meet.

### The four dehiyyot

Rosh Hashanah, 1 Tishrei, would fall on the day of the molad of Tishrei if
nothing intervened. Four rules move it, each stated by Maimonides in
chapter 7, and the first two are applied before the year lengths are
known, the last two after:

1. **Lo ADU Rosh** (7:1). Rosh Hashanah may not fall on Sunday, Wednesday
   or Friday. When the molad falls on one of those days, the year begins
   on the next day. The reason usually given, from the Talmud (Rosh
   Hashanah 20a, not read here), is that Yom Kippur on 10 Tishrei must
   not abut the Sabbath — it would on Friday or Sunday — and Hoshana
   Rabbah on 21 Tishrei must not fall on it; Maimonides gives it as the
   received tradition.
2. **Molad Zaken** (7:2). When the molad falls at or after noon — 18 hours
   from six in the evening — the year begins on the following day, and
   then Lo ADU Rosh is applied again to that day. Maimonides' reason is
   that the calendar reckons the mean motion, and the postponements keep
   the day of the fixed calendar at the day the true conjunction, and so
   the crescent, could be seen [maimonides-kiddush-hachodesh, 7:7].
3. **GaTaRaD** (7:4). In a common year, when the molad falls on a Tuesday
   at or after 9 hours and 204 parts, the year begins on Thursday. Without
   it the year would run 356 days, which is not a permitted length. The
   arithmetic: the next year's molad is twelve months later, 354 days,
   8 hours and 876 parts, and 354 days is 50 weeks and 4 days, so a molad
   of Tuesday 9h 204p puts the next one at Saturday 18h 0p exactly —
   noon. Molad Zaken pushes that year to Sunday, Lo ADU Rosh to Monday,
   and Tuesday to Monday is 356 days. Beginning this year on Thursday
   instead makes it 354.
4. **BeTuTaKaPoT** (7:5). In the year after a leap year, when the molad
   falls on a Monday at or after 15 hours and 589 parts, the year begins
   on Tuesday. Without it the *preceding* year would run 382 days. The
   preceding molad is thirteen months earlier, 383 days, 21 hours and 589
   parts; 383 days is 54 weeks and 5 days, so a molad of Monday 15h 589p
   puts it at Tuesday 18h 0p, noon again, whence Molad Zaken gives
   Wednesday and Lo ADU Rosh Thursday. Thursday to Monday is 382 days,
   too short for a leap year; Tuesday makes it 383.

Reingold and Dershowitz apply the last two not by inspecting the molad but
by their consequence: compute the provisional new years of three
consecutive years with the first two rules alone, and if the coming year
would be 356 days long delay this one by two days, or if the past year
would be 382 days long delay it by one [reingold2018code,
`hebrew-year-length-correction`]. The two formulations are the same rule,
and the module uses theirs.

### The six year lengths

A common year runs 353, 354 or 355 days and a leap year 383, 384 or 385,
called *deficient*, *regular* and *complete* [maimonides-kiddush-hachodesh,
8:5 and 8:8]. The difference is in two months only: Marḥeshvan (called Ḥeshvan
here) has 29 days except in a complete year, when it has 30, and Kislev
has 30 except in a deficient year, when it has 29. Every other month is
fixed, and the year's length is the number of days from one Rosh Hashanah
to the next.

| Public number | Month | Days | Internal number |
| --- | --- | --- | --- |
| 1 | Tishrei | 30 | 7 |
| 2 | Ḥeshvan | 29, or 30 in a complete year | 8 |
| 3 | Kislev | 30, or 29 in a deficient year | 9 |
| 4 | Ṭevet | 29 | 10 |
| 5 | Shevaṭ | 30 | 11 |
| 5, leap | Adar I, in a leap year only | 30 | 12 |
| 6 | Adar, or Adar II in a leap year | 29 | 12, or 13 in a leap year |
| 7 | Nisan | 30 | 1 |
| 8 | Iyyar | 29 | 2 |
| 9 | Sivan | 30 | 3 |
| 10 | Tammuz | 29 | 4 |
| 11 | Av | 30 | 5 |
| 12 | Elul | 29 | 6 |

The "public number" is what `HebrewDate.month` carries, with Adar I as
`Month::leap(5)`, CLDR's `M05L`. Writing the intercalary month as the leap
repetition of Shevaṭ rather than of Adar is a choice of representation:
the month that follows Adar I is the Adar that carries Purim, and a single
`Month` type serves this calendar and the Chinese one, where the leap
month likewise follows the month it is named after. The internal number
is Reingold and Dershowitz's, Nisan first, Adar II as 13
[reingold2018code, `nisan`, `tishri`, `adar`, `adarii`].

### Worked example: Rosh Hashanah 5784

5784 stands eighth in its cycle (5784 mod 19 = 8), so it is a leap year.

1. **Months to the molad of Tishrei.** ⌊(235 × 5784 − 234) / 19⌋ =
   ⌊1 359 006 / 19⌋ = 71 526.
2. **The molad.** Each month is 29 days and 13 753 parts. 71 526 months
   add 71 526 × 29 = 2 074 254 days and 71 526 × 13 753 = 983 697 078
   parts; with BaHaRaD's 5 604 parts that is 983 702 682 parts, which is
   37 951 days and 12 762 parts. So the molad falls 2 074 254 + 37 951 =
   2 112 205 days after the epoch's Monday began, at 12 762 parts =
   11 hours 882 parts after six in the evening — about 05:49 in the
   morning. 2 112 205 is 4 more than a multiple of 7, and four days after
   a Monday is a Friday.
3. **Molad Zaken.** 11h 882p is before 18h; no postponement.
4. **Lo ADU Rosh.** Friday is forbidden; Rosh Hashanah moves to Saturday,
   2 112 206 days after the epoch.
5. **GaTaRaD** applies to common years only, and 5784 is leap.
   **BeTuTaKaPoT** applies to the year after a leap year, and 5783 is
   common. In the module's formulation: the provisional start of 5785 is
   2 112 589 days, 383 after this one, not 356; the provisional start of
   5783 is 2 111 851, 355 before, not 382. No correction.
6. **The date.** RD −1 373 427 + 2 112 206 = 738 779, which is Saturday
   16 September 2023 — the day Hebcal begins Rosh Hashanah 5784 at the
   preceding sunset [hebcal-5784]. The molad of Tishrei 5785 falls on a
   Thursday at 9h 391p and nothing moves it, so 5785 begins on Thursday
   3 October 2024, 383 days later: 5784 is a *deficient* leap year, with
   Ḥeshvan and Kislev both of 29 days.

For the two rules that did not bite, the module computes: in 5745 the
molad of Tishrei fell on Tuesday at 17h 976p in a common year, so
GaTaRaD moved Rosh Hashanah to Thursday 27 September 1984; in 5766, the
year after the leap year 5765, it fell on Monday at 16h 876p, so
BeTuTaKaPoT moved it to Tuesday 4 October 2005. Those two dates are the
module's own and were not checked against a published table for this
document.

### How far the arithmetic is from the sky

The molad interval, 765 433 / 25 920 = 29.530 594 days, is 0.46 seconds
longer than the mean synodic month of 29.530 588 861 days
[reingold2018code, `mean-synodic-month`], so the molad falls behind the
true mean conjunction by about a day in 15 000 years. Wikipedia puts the
drift accumulated since the Talmudic era at about 97 minutes
[wikipedia-hebrew-calendar]. The solar drift is much larger: the mean
year, 235/19 of a molad interval, is 365.246 82 days against a tropical
year of 365.242 19, an excess of 0.004 63 days a year, so the festivals
move later through the seasons by about a day every 216 years
[wikipedia-hebrew-calendar]. All four figures are derived here from the
constants named; only the 216 years and the 97 minutes are also quoted.

## What is carried

- **Identifier** `hebrew`, in `hc-calendars-lunar`, with the date as
  `HebrewDate { year, month, day }`, the era `AM`, months numbered from
  Tishrei with Adar I as `Month::leap(5)`, and the day beginning at
  sunset (`DayBoundary::Sunset`).
- **Range** AM 1 to AM 9 999: RD −1 373 427 (7 October 3761 BCE Julian)
  to the last day of Elul 9 999 (25 September 6239). The lower bound is
  the epoch; the upper is the library's usual one and has no source. The
  arithmetic is applied to every year in the range as if the rules had
  always been in force, which before the tenth century they were not
  (see above): a date this library gives for AM 3000 is what the fixed
  calendar would have said, not what anyone kept.
- **Computed, not tabulated.** Nothing is looked up: the leap years, the
  molad, the postponements, the year lengths and the month lengths are
  all arithmetic, so the calendar is exact and its metadata says it is
  not astronomical. `molad` returns the mean conjunction of any month as
  a moment, for callers that announce it.
- **Beside the calendar**, in the same module: 15 Nisan as `passover`;
  the count of the Omer, day 1 on 16 Nisan to day 49 on 5 Sivan, as
  `omer_day` and `omer_weeks_and_days`; and *birkat hachama*, the
  blessing of the sun said when Shmuel's *tekufah* of Nisan returns to
  its hour and weekday every 28 Julian years, 10 227 days, always on a
  Wednesday, anchored to 8 April 2009. The 28-year cycle is arithmetic on
  the 365¼-day year; the anchor and the dates 7 April 1897, 8 April 1981
  and 8 April 2037 that the tests hold are stated by the module as widely
  reported and rest on no source named here.
- **Not carried, and why.**
  - The observational calendar of the Second Temple period and of the
    Sanhedrin, in which each month was declared on the testimony of
    witnesses to the crescent and the leap month by decision. There is no
    rule to compute, only records, and no record is carried.
  - The calendar between its fixing and its final form: the years before
    922–924 in which, the Geniza evidence shows, the festivals were not
    where the present rules put them [wikipedia-hillel-ii]. This library
    projects the present rules back and says so.
  - The Samaritan calendar, a sibling of this one on the priesthood's own
    conjunction computation and epoch, without the dehiyyot. It is a
    separate row of the roadmap, `samaritan`, and a calendar of its own.
  - The Karaite calendar, which keeps the observational rule.
  - The sabbatical (*shemittah*) year count and the Jubilee, which are
    year classifications and not dates.
  - The *tekufot* other than as `birkat_hachama` uses them, and any true
    astronomy: the module calls nothing in `hc-astro`.

## Accuracy

Exact, in the sense that the rules are arithmetic and the module is those
rules; the question is whether it is the *right* arithmetic, and that is
what the anchors check.

| Measure | Result | Test |
| --- | --- | --- |
| 1 Tishrei 5784 = Saturday 16 September 2023 [hebcal-5784] | Reproduced, both ways | `rosh_hashanah_5784_was_the_sixteenth_of_september_2023` |
| 15 Nisan 5784 = Tuesday 23 April 2024 [hebcal-5784] | Reproduced, both ways | `passover_5784_was_the_twenty_third_of_april_2024` |
| 5784 a deficient leap year of 383 days, Ḥeshvan and Kislev 29 | As stated | `the_year_5784_was_a_deficient_leap_year_of_383_days` |
| The closed form (7*y* + 1) mod 19 < 7 against the listed years 3, 6, 8, 11, 14, 17, 19, over 50 cycles | 950 of 950 | `the_closed_form_matches_the_listed_leap_years` |
| Rosh Hashanah never on Sunday, Wednesday or Friday, AM 1–9 999 | 9 999 of 9 999 | `rosh_hashanah_never_falls_on_sunday_wednesday_or_friday` |
| Every year one of the six lengths, AM 1–9 999 | 9 999 of 9 999 | `every_year_takes_one_of_the_six_permitted_lengths` |
| Deficient, regular and complete differ only in Ḥeshvan and Kislev, and all three occur, 5700–5799 | As stated | `deficient_regular_and_complete_years_differ_only_in_heshvan_and_kislev`, `all_three_kinds_of_year_occur` |
| Adar I only in a leap year, always 30 days, between Shevaṭ and Adar II | As stated | `adar_one_exists_only_in_a_leap_year_and_always_has_thirty_days`, `adar_one_falls_between_shevat_and_adar_two`, `asking_for_adar_one_in_a_common_year_is_refused` |
| 235 months in every cycle | 10 cycles checked | `the_metonic_cycle_closes_after_two_hundred_and_thirty_five_months` |
| The molad interval is 29.530 594 days and Rosh Hashanah lies between half a day before and 2½ days after the molad, 5700–5799 | As stated | `the_molad_advances_by_one_mean_synodic_month`, `the_molad_stays_within_a_day_of_rosh_hashanah` |
| Round trips over 40 000 days from 5700, and 5 000 days at each end of the range | All | `the_calendar_round_trips_over_forty_thousand_modern_days`, `the_calendar_round_trips_near_the_epoch_and_the_end_of_the_range` |
| The Omer: 49 days from 16 Nisan to 5 Sivan, spoken as weeks and days | As stated | `the_omer_runs_forty_nine_days_from_sixteen_nisan_to_five_sivan`, `the_omer_is_forty_nine_days_long_in_every_year`, `the_omer_speaks_weeks_and_days` |
| Birkat hachama on a Wednesday every 10 227 days; 1897-04-07, 1981-04-08, 2009-04-08, 2037-04-08 | As stated; the dates unsourced | `birkat_hachama_falls_on_a_wednesday_every_twenty_eight_years`, `the_recent_and_next_birkat_hachama_are_the_published_ones` |

Two published dates anchor the calendar, both from Hebcal for 5784: Rosh
Hashanah from sunset on Friday 15 September 2023, so its first day is
Saturday the 16th, and Pesach from sunset on Monday 22 April 2024, so
15 Nisan is Tuesday the 23rd [hebcal-5784]. Hebcal is a calendar service,
not an authority; it is used here as an independent implementation of the
same rules, and two dates of one year are a thin check on a calendar
whose behaviour changes with the dehiyyot. The structural tests — the
forbidden weekdays and the six lengths over all 9 999 years — are what
carry the weight: a mistake in any of the four rules breaks one of them.
The molad test asserts the interval to six decimals and the bound on the
postponement, not the moment against any published molad.

Statements in the module that no source in the table below supports, and
that stand as the module's own: the birkat hachama anchor and dates named
above; and the range's upper bound.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [maimonides-kiddush-hachodesh] | Chapter 6: the molad interval and the part, the solar year and the deficit of a lunar year, BaHaRaD, the seven leap years. Chapter 7: the four dehiyyot with their thresholds (halachot 1, 2, 4 and 5) and the reason (7). Chapter 8: the six year lengths, the two months that vary and the month order (halachot 5 and 8) | Yes, 2026-09-25, in Sefaria's English translation |
| [reingold2018] | The arithmetic: the closed forms and the year-length formulation of the last two dehiyyot | Not read directly; the published code was |
| [reingold2018code] | `hebrew-epoch`, `hebrew-leap-year?`, `last-month-of-hebrew-year`, `molad`, `hebrew-calendar-elapsed-days`, `hebrew-year-length-correction`, `hebrew-new-year`, `days-in-hebrew-year`, `long-marheshvan?`, `short-kislev?`, `last-day-of-hebrew-month`, `fixed-from-hebrew`, `hebrew-from-fixed`, `mean-synodic-month`, and the month constants | Yes, 2026-09-25 |
| [hebcal-5784] | Rosh Hashanah and Pesach of 5784; that 5784 has Adar I and Adar II | Yes, 2026-09-25 |
| [wikipedia-hebrew-calendar] | The epoch's Julian date; the six year lengths and their names; the mean year of 365.2468 days and the day per 216 years; the 97 minutes of molad drift | Yes, 2026-09-25 |
| [wikipedia-hillel-ii] | The tradition of 358/9 CE and its source; the Geniza letter of 835/6; the final form by 922–924; Stern's reading | Yes, 2026-09-25 |

The Talmud's reason for Lo ADU Rosh (Rosh Hashanah 20a) is cited above as
not read.

## Code

`crates/hc-calendars-lunar/src/hebrew.rs`. The first two dehiyyot are in
`elapsed_days` (the 6 480 parts added before truncation are Molad Zaken;
the weekday test is Lo ADU Rosh) and the last two in `new_year_delay`;
`new_year` applies all four. `days_in_year`, `year_kind`, `is_long_heshvan`
and `is_short_kislev` derive the year type; `internal_month` and
`public_month` translate between the Tishrei-first public numbering and
the Nisan-first internal one. Anchors:
`rosh_hashanah_5784_was_the_sixteenth_of_september_2023`,
`passover_5784_was_the_twenty_third_of_april_2024`,
`rosh_hashanah_never_falls_on_sunday_wednesday_or_friday`,
`every_year_takes_one_of_the_six_permitted_lengths`. The Omer and birkat
hachama functions are in the same module; the holidays that sit on the
calendar are in `hc-holiday`. Hebrew and English month names are in
`hc-i18n`.
