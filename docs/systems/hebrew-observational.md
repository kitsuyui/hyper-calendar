# The observational Hebrew calendar: Reingold and Dershowitz's prediction at Haifa

Backs the identifier `hebrew-observational` in `hc-calendars-lunar`.

## What it is

Before the fixed calendar of [hebrew.md](hebrew.md), the Hebrew months were
not computed but **declared**. Maimonides, describing the practice the
fixed calendar replaced, says the months are lunar and that the new
crescent is first seen in the west in the evening, a day or so after the
conjunction; a month of 29 days is one whose thirtieth night showed the
crescent, and one of 30 days follows when it did not
[maimonides-kiddush-hachodesh-1-4, 1:1, 1:3, 1:4]. Fixing the new month
was not for every individual but for the court, which calculated whether
the crescent would be seen, examined the witnesses who reported it, and
sent messengers to tell the people, and it was done only in the Land of
Israel [maimonides-kiddush-hachodesh-1-4, 1:5, 1:7, 1:8].

The year was made leap by the court too, by adding a second Adar, and
Maimonides names three grounds: the spring equinox, the ripening of the
barley and the blooming of the fruit trees. The first suffices by itself:
when the court calculated that the equinox would fall on the sixteenth of
Nisan or later, the month that would have been Nisan became the second
Adar, "and thus Pesach will fall in the spring". The barley and the trees
together could make a year leap even when the equinox came earlier, and
the state of the roads and bridges could in need
[maimonides-kiddush-hachodesh-1-4, 4:1–4:5].

So the calendar of the Second Temple period and of the Sanhedrin after it
was a record of decisions, and no table of those decisions survives to be
carried. What can be computed is a **prediction**: the calendar the rule
would give if every month began on the first evening the crescent could be
seen, and every year were made leap by the equinox alone. Reingold and
Dershowitz give one in the published code of *Calendrical Calculations*,
with Haifa as its sample location [reingold2018code,
`observational-hebrew-first-of-nisan`, `fixed-from-observational-hebrew`,
`observational-hebrew-from-fixed`, `classical-passover-eve`,
`hebrew-location`]. It is the same kind of thing as the observational
Hijri prediction of [hijri.md](hijri.md): a forecast of an observation,
never a record of a declaration.

## How it works

### The month

A month begins on a day whose eve shows the crescent at Haifa, 32.82° N,
35° E, at sea level [reingold2018code, `hebrew-location`], by the same test
as the observational Hijri calendar: Shaukat's criterion, judged when the
Sun is 4.5° below the horizon — the Moon past conjunction and short of
first quarter, an arc of light of at least 10.6°, an altitude of more than
4.1° [reingold2018code, `visible-crescent`, `shaukat-criterion`]. The
first day of the month containing a day is the last such day on or before
it (`phasis-on-or-before`); the first day of the next month to begin on or
after a day is the first such day on or after it (`phasis-on-or-after`),
found by stepping from 29 days after the last conjunction when the day is
four or more days past it, or when the crescent was already seen the
evening before.

### The year

1 Nisan in the spring of a Gregorian year is the first month start on or
after a day fixed by the equinox [reingold2018code,
`observational-hebrew-first-of-nisan`]: fourteen days before the day of the
March equinox, when the equinox falls before sunset at Haifa that day, and
thirteen days before it otherwise, the equinox after sunset belonging to
the next day. Fifteen days on, then, is the equinox's own day or later: the
fifteenth of Nisan is never before the equinox, which is Maimonides'
condition, that the equinox not fall on the sixteenth or later, read in
the other direction. The year runs twelve months from one such 1 Nisan to
the next, or thirteen when the next is thirteen lunations away, and the
thirteenth is Adar II.

The year number is the fixed calendar's, by the Anno Mundi era: the year
that holds a given 1 Nisan is the fixed calendar's year on that day, and
it changes at the seventh month, Tishrei, as the fixed calendar's does
[reingold2018code, `observational-hebrew-from-fixed`]. A month is numbered
by rounding the days since 1 Nisan over 29.5, so the months are Nisan 1 to
Elul 6, Tishrei 7 to Adar 12, and Adar II 13 in a year of thirteen. To go
the other way, the code finds 1 Nisan from the fixed calendar's Nisan of
the right year, sixty days on, as a way of naming the Gregorian year, and
looks for the month start on or before the middle of the month counted
from it [reingold2018code, `fixed-from-observational-hebrew`]. The eve of
Passover, 14 Nisan, is 1 Nisan plus thirteen days [reingold2018code,
`classical-passover-eve`].

### Worked example: Nisan and Passover of 2024

1. **The equinox.** `hc-astro` puts the March equinox of 2024 at 03:06 UT
   on 20 March. The Sun set at Haifa that day at 15:52 UT. The equinox came
   before sunset, so the search starts fourteen days earlier, on 6 March.
2. **The last conjunction** before 6 March was at 22:59 UT on 9 February,
   26 days earlier. Four days or more have passed, so the search steps from
   9 February plus 29 days, 9 March.
3. **The evenings.** The eve of 10 March, the evening of 9 March: the Moon
   is 350° from the Sun, still before conjunction, and fails. The evening
   of 10 March, at 16:02 UT with the Sun 4.5° down: the conjunction was at
   09:00 UT that morning, the elongation is 4.2°, the arc of light 4.6°
   and the altitude −0.7°, so it fails twice over. The evening of
   11 March, at 16:03 UT: elongation 18.4°, arc of light 18.4°, altitude
   13.5°, and it passes.
4. **The answer.** 1 Nisan is Tuesday 12 March 2024, the eve of Passover,
   14 Nisan, Monday 25 March, and the first day of Passover Tuesday
   26 March, six days after the equinox.

The fixed calendar made 5784 a leap year, and its 1 Nisan is Tuesday
9 April 2024, four weeks later, with Pesach from the evening of 22 April
[hebcal-5784]. On the prediction's reckoning 12 March was already Nisan;
on the fixed calendar's it was 2 Adar II. The prediction's own leap year
comes a year later: from 12 March 2024 to its next 1 Nisan, 31 March 2025,
is 384 days, thirteen months, so its 5785 has Adar I and Adar II where the
fixed calendar's 5785 has one Adar. The figures are from the module's own
functions.

## What is carried

- **Identifier** `hebrew-observational`, in `hc-calendars-lunar`, with the
  date as the fixed calendar's `HebrewDate { year, month, day }`: the era
  code `am`, months numbered from Tishrei, Adar I as `Month::leap(5)` —
  here in the years the prediction makes leap, not the years the fixed
  calendar does — and the day beginning at sunset and named by the civil
  day it ends on, `DayBoundary::Sunset(DayNaming::ByEnd)`, since the
  crescent that opens a month is judged on the eve of its first day.
  English and Hebrew month names are the fixed calendar's, from `hc-i18n`.
- **Beside it:** `first_of_nisan` and `classical_passover_eve` by
  Gregorian year, `is_leap_year` by Hebrew year, and the site, `SITE`,
  which is Haifa with Shaukat's criterion.
- **Range** 1 January 383 BCE to 31 December 2100 (proleptic Gregorian).
  The lower bound is where this library's crescent computation has been
  measured against a published table of first visibilities, the
  Babylonian calendar's comparison with Parker and Dubberstein
  ([babylonian.md](babylonian.md)); the upper is where the observational
  Hijri prediction stops. Both are this library's choice, not the
  source's. Outside the range a date is refused.
- **Computed, not tabulated.** Every month start is a crescent test from
  `hc-astro`; every year start is an equinox from `hc-astro` and a
  crescent test. The fixed calendar is consulted only to number the years
  and to name the Gregorian year of a Nisan, never to decide a month.
- **Not carried, and why.**
  - Any declared date. None was found.
  - The barley and the fruit trees, and the roads and the bridges, which
    Maimonides lists as further grounds for a leap year 
    [maimonides-kiddush-hachodesh-1-4, 4:3–4:5]. A crop is not computable.
  - The rule that a month is never longer than thirty days
    [maimonides-kiddush-hachodesh-1-4, 1:4]. The prediction judges each
    evening by itself, so a month runs 31 days when a first evening that
    just clears the criterion is followed thirty evenings later by one that
    just misses: four times in 1900–2100. The published code's main
    version keeps such months, its `month-length` typed `1..31`, and it has
    an alternative that caps months at thirty
    (`alt-fixed-from-observational-hebrew`,
    `alt-observational-hebrew-from-fixed`, `early-month?`), which is not
    carried. **The rule here is the main version's: a predicted month of 31
    days is allowed, and its 31st is a date that converts both ways.** The
    observational Hijri prediction follows the same rule, for the same
    reason, and [hijri.md](hijri.md) counts how often it happens there by
    place and criterion; Shawwāl 1464, from 16 September 2042, is 31 days
    long at Haifa in both.
  - The court's own reckoning of the equinox, which Maimonides gives by
    the *tekufah* of his chapter 9, not read here; the prediction uses the
    true equinox.
  - The Karaite calendar, which is a calendar of its own and is Researching
    in the roadmap, [calendars.md](../calendars.md).

## Accuracy

**No published anchor exists for this calendar.** No declared date of the
Second Temple calendar survives in a form that could be checked, and no
table of the prediction itself was read: the book's sample values, which
the published code says are in its Appendix C, were not. What the tests
check is that the module is the rule, and how far the rule is from the
fixed calendar where both can be computed.

| Measure | Result | Test |
| --- | --- | --- |
| 15 Nisan never before the equinox's day, and the month before it too early, 1900–2100 and 383–283 BCE | All | `the_fifteenth_of_nisan_is_the_first_on_or_after_the_equinox` |
| The eve of Passover is 14 Nisan and 1 Nisan is a month start, every year of 1900–2100 | All | `passover_eve_is_the_fourteenth_of_the_first_month` |
| Month lengths, the 2 486 months of 1900–2100 | 1 171 of 29 days, 1 311 of 30, 4 of 31 (from 21 July 1917, 24 July 1933, 24 July 1971 and 16 September 2042), none shorter; the 31st of each of the four converts both ways | `months_run_twenty_nine_or_thirty_days_but_for_a_few_of_thirty_one` |
| Years of 12 or 13 months, 353–356 or 383–386 days, and Adar I exactly in the thirteen-month years, AM 5750–5789 and 3661–3699 | All | `years_run_twelve_or_thirteen_months` |
| Round trips over four years of modern days and at both ends of the range | All | `the_calendar_round_trips_over_four_years_of_days`, `the_calendar_round_trips_at_both_ends_of_its_range` |
| The worked example: 1 Nisan on 12 March 2024, Passover eve on 25 March, 5784 of twelve months and 5785 of thirteen | Reproduced | `nisan_2024_is_worked_in_the_document` |
| 1 Nisan against the fixed calendar's, 201 springs of 1900–2100 | The same day in 48, one day later in 58, two days later in 44, 27 to 29 days earlier in 51 | `the_prediction_and_the_fixed_calendar_and_the_crate_says_how_far_apart` |
| Leap years against the fixed calendar's, AM 5661–5860 | 99 of 200 agree | the same test |

**What the comparison with the fixed calendar measures.** The fixed
calendar's 1 Nisan is fixed by the molad and its postponements, and
begins on or up to two days before the day the crescent would first be
seen; that is the 150 springs within two days. In the other 51 the fixed
calendar begins Nisan a lunation later than the equinox rule would: its
nineteen-year cycle, whose mean year is 0.0046 days longer than the
tropical year [hebrew.md](hebrew.md), has carried its Passover later
through the seasons since the cycle was fixed, so that it now makes a year
leap one year earlier than the equinox rule does in about five of its
seven leap years. That is why the leap years agree in only half the
years. It is a measure of the fixed calendar's drift from the rule it
replaced, not an error of either.

The one statement in the module that no source supports, and that stands
as the module's own, is the range, which is a choice.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [reingold2018code] | `hebrew-location`, `observational-hebrew-first-of-nisan`, `fixed-from-observational-hebrew`, `observational-hebrew-from-fixed`, `classical-passover-eve`, `phasis-on-or-before`, `phasis-on-or-after`, `visible-crescent`, `shaukat-criterion`; the capped alternative `alt-fixed-from-observational-hebrew`, `alt-observational-hebrew-from-fixed`, `early-month?`, `month-length`, named as not carried | Yes, 2026-09-26 |
| [reingold2018] | The book's account of the calendar and its sample values in Appendix C | Not read |
| [maimonides-kiddush-hachodesh-1-4] | The months declared by the court on witnesses' testimony, in the Land of Israel; months of 29 and 30 days; the three grounds for a leap year and the equinox on 16 Nisan or later | Yes, 2026-09-26, in Eliyahu Touger's English translation on Sefaria, chapters 1 and 4 |
| [hebcal-5784] | 1 Nisan and Pesach of 5784 on the fixed calendar | Yes, 2026-09-25 |

Maimonides' *tekufah* in chapter 9 is cited above as not read.

## Code

`crates/hc-calendars-lunar/src/hebrew_observational.rs`: `first_of_nisan`,
`classical_passover_eve`, `is_leap_year`, `from_fixed`, `to_fixed`,
`ObservationalHebrewCalendar`, and `HAIFA` and `SITE`. The crescent test and
the two month searches are `ObservationSite::crescent_visible_on_the_eve_of`,
`month_start_on_or_before` and `month_start_on_or_after` in
`islamic_observational.rs`, shared with the observational Hijri
prediction. Anchors, such as they are: `nisan_2024_is_worked_in_the_document`,
`the_fifteenth_of_nisan_is_the_first_on_or_after_the_equinox`,
`the_prediction_and_the_fixed_calendar_and_the_crate_says_how_far_apart`.
