# The Thai lunar calendar, and the Buddhist year as printed

Backs the identifier `thai-lunar` in `hc-calendars-regional`, and
`printed_year` and `printed_to_fixed` of the `buddhist` module in
`hc-calendars-solar`.

## What it is

**The lunar calendar.** ปฏิทินจันทรคติไทย is the calendar on which
Thailand's Buddhist holy days fall and which every Thai wall calendar prints
beside the civil date. It has twelve numbered months, เดือนอ้าย (month 1) and
เดือนยี่ (month 2) and then เดือนสาม to เดือนสิบสอง, the odd months of 29
days and the even of 30 [wikipedia-th-thai-lunar]. The days of a month are
counted in two halves: the waxing fortnight, ขึ้น 1 ค่ำ to ขึ้น 15 ค่ำ, and the
waning, แรม 1 ค่ำ to แรม 14 ค่ำ in a 29-day month or แรม 15 ค่ำ in a 30-day
month. A month therefore begins the day after new moon by the calendar's own
reckoning, and its full moon, ขึ้น 15 ค่ำ, is the day the holy days fall on.

Three kinds of year keep the months in step with the seasons
[wikipedia-th-thai-lunar, payutto-dictionary]:

| Year type | Thai | Days | What is added |
| --- | --- | --- | --- |
| Normal | ปกติมาส ปกติวาร | 354 | Nothing |
| *Adhikavāra* | อธิกวาร | 355 | A 30th day, แรม 15 ค่ำ, in month 7 |
| *Adhikamāsa* | อธิกมาส | 384 | A second month 8 of 30 days, เดือน 8 หนแรก or เดือน 8 แรก, between month 7 and the regular month 8, which is then เดือน 8 หลัง |

A year is never both. Thailand's rule is the opposite of Burma's, where only
a year with the extra month may also take the extra day [eade2000, p. 199].

Which years are which is decided by the astronomical reckoning called
*suriyayatra* (สุริยยาตร), the Thai form of the Indian *Sūrya Siddhānta*
arithmetic, whose year has an epoch of 25 March 638 CE (the Chulasakarat
era) [eade2000, p. 195]. Eade's account of the rules is in the next section;
his own conclusion, after Prasert na Nagara and Luang Wisandarunkon, is that
the published calendar has not always followed them, and this library does
not compute them.

**The holy days.** Makha Bucha (วันมาฆบูชา) is the full moon of month 3;
Visakha Bucha (วันวิสาขบูชา) of month 6; Asalha Bucha (วันอาสาฬหบูชา) of
month 8; and Khao Phansa (วันเข้าพรรษา), the start of the rains retreat, is
แรม 1 ค่ำ of month 8, the day after Asalha Bucha. In an *adhikamāsa* year
Makha and Visakha Bucha move a month later, to the full moons of months 4
and 7, and Asalha Bucha is the full moon of the *second* month 8 — the later
of the two rains-retreat dates the Mahāvagga allows, as Eade notes
[eade2000, p. 198]. All four are public holidays, and the Bank of Thailand
publishes their dates a year ahead in its list of financial-institution
holidays [bot-fiholiday, bot-fpg3-2565, bot-fpg8-2566, bot-fpg5-2567,
bot-31-2568, bot-37-2569]. Those lists are the published calendar this
library carries.

**The year number.** Thailand counts years in the Buddhist Era (พุทธศักราช,
BE), 543 ahead of the Common Era: 2026 is 2569 BE. On the civil calendar the
year has begun on 1 January since 1941. Before that:

- From 1 April 1889 the civil reckoning was solar, with Gregorian months,
  the year beginning on 1 April and counted in the Rattanakosin era
  (รัตนโกสินทรศก, RS) from the founding of Bangkok: 1 April 1889 opened
  RS 108 [proclamation-new-day-rs107].
- A proclamation of 21 February RS 131 replaced the era with the Buddhist
  Era from the next 1 April, which opened 2456 BE (1913); the year still
  began on 1 April [proclamation-counting-rs131].
- The Calendar Years Act of 2483 (พระราชบัญญัติปีประดิทิน พุทธศักราช ๒๔๘๓,
  enacted 6 September 1940) made the year run from 1 January to
  31 December from 2484 on, so 2483 ran from 1 April to 31 December 1940
  and had nine months [calendar-years-act-2483].

So a Thai document printed between 1889 and 1940 dates January to March a
year lower than the modern rule gives, and before April 1913 in another era:
1 January 1920 was printed 2462, and 21 February RS 131 was in 1913. Before
1 April 1889 official dates were lunar, in the Chulasakarat era, and this
document does not cover them.

## How it works

**The year type fixes every month.** Given a year's type, its months are:

| Month | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 แรก | 8 | 9 | 10 | 11 | 12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Normal | 29 | 30 | 29 | 30 | 29 | 30 | 29 | — | 30 | 29 | 30 | 29 | 30 |
| *Adhikavāra* | 29 | 30 | 29 | 30 | 29 | 30 | **30** | — | 30 | 29 | 30 | 29 | 30 |
| *Adhikamāsa* | 29 | 30 | 29 | 30 | 29 | 30 | 29 | **30** | 30 | 29 | 30 | 29 | 30 |

Nothing a year type changes comes before month 7. So once the day of
ขึ้น 1 ค่ำ เดือนอ้าย is known, the type places every day of the year, and the
next year begins 354, 355 or 384 days later.

**Reading the type off the holy days.** Because the holy days are full
moons of fixed months, the intervals between them are fixed by the type:

| Interval | Normal | *Adhikavāra* | *Adhikamāsa* |
| --- | --- | --- | --- |
| Makha Bucha to Visakha Bucha | 88 | 88 | 89 |
| Visakha Bucha to Asalha Bucha | 59 | 60 | 59 |

The counts follow from the table above. In a normal year Makha Bucha is
day 15 of the 29-day month 3, so 14 days remain in it, then 30 (month 4),
29 (month 5) and 15 into month 6: 88. In an *adhikamāsa* year the same
count runs from day 15 of the 30-day month 4, so 15 + 29 + 30 + 15 = 89.
Visakha to Asalha is 15 + 29 + 15 = 59, or 60 when month 7 has its 30th
day, or 14 + 30 + 15 = 59 across the extra month 8. So a year's three
published dates name its type, and no two types give the same pair.

The step from one year's Makha Bucha to the next is a fourth check: it
equals the year's length when Makha Bucha stays in the same month, and
the length less 29 when it moves from month 4 back to month 3 (an
*adhikamāsa* year followed by another type), or plus 29 when it moves from
month 3 to month 4 (a year followed by an *adhikamāsa* one). Between 2024
and 2027 the steps are 354, 384 and 355 days: normal, then *adhikavāra*
followed by *adhikamāsa*, then *adhikamāsa* followed by normal.

Where a Bank list gives only the weekday that stood in for a holy day on a
Saturday or Sunday — the Monday after, or in 1996 the Thursday before —
the two intervals and the step decide which weekend day it was, and in
every such year exactly one candidate passes all three.

**Eade's rule, which is not used.** Eade gives the *suriyayatra* rules as
Prasert and Wisandarunkon state them [eade2000, pp. 195–197]. The extra
month is called by the astronomical New Year: if the day of *thaloengsok*
(เถลิงศก, the Mean Sun's entry into Aries) falls on tithi 25 to 29 of
Caitra or 1 to 5 of Vaiśākha, the year is *adhikamāsa*. The extra day is
called by two quantities of the New Year computation: the *kammacubala*
(กัมมัชพล) of 207 or less marks a solar leap year, and the year has an extra
day when its *avoman* (อวมาน) is 126 or less in a solar leap year or 137 or
less in a common one — with the subsidiary Thai rule that a year with an
extra month may not also take the extra day, which then passes to the year
after. Eade tabulates the types of CS 1320–1340 (1958–1978) and shows that
the monastic (*Sasana*) rule for the extra day, from the *nakṣatra* at the
start of the rains retreat, does not reproduce them. Why this library does
not compute the rule is in the next section.

**The printed year.** For a day from 1 April 1889 on, take its Gregorian
year; before 1941, if the month is January to March, take the year before.
If the day is before 1 April 1913 the printed year is that year less 1781
in the Rattanakosin era; otherwise it is that year plus 543 in the Buddhist
Era. Reading back, a Rattanakosin year is valid only from RS 108 to 131, a
Buddhist year only from 2456, and January to March of 2483 do not exist.

**Worked example: 2569 BE (2026).** The Bank's notification for 2569 gives
Makha Bucha on Tuesday 3 March, Visakha Bucha on Sunday 31 May with the
Monday off in its place, and Asalha Bucha on Wednesday 29 July
[bot-31-2568]. From 3 March to 31 May is 89 days and from 31 May to
29 July is 59, so the year is *adhikamāsa*; the step to the next Makha
Bucha, Sunday 21 February 2027 [bot-37-2569], is 355 days, which is 384
less 29, as an *adhikamāsa* year followed by a normal one requires.

Now derive Visakha Bucha from the type alone. Makha Bucha is ขึ้น 15 ค่ำ
เดือน 4, the fifteenth day of a 30-day month, so 15 days of month 4 remain;
month 5 has 29 and month 6 has 30; Visakha Bucha is the fifteenth day of
month 7. That is 15 + 29 + 30 + 15 = 89 days after 3 March: 28 more days
of March, 30 of April, and the 31st of May. Sunday 31 May 2026, as
published. Asalha Bucha is then 14 remaining days of the 29-day month 7,
the 30 days of เดือน 8 แรก, and 15 into เดือน 8 หลัง: 59 days later, on
Wednesday 29 July, and Khao Phansa is Thursday 30 July. The module's test
`the_adhikamasa_year_2569_doubles_month_8` holds these days, with the year
beginning on 21 November 2025 and the next on 10 December 2026, 384 days
later.

## What is carried

- **Identifier** `thai-lunar`, in `hc-calendars-regional`, with the month as
  `Month { ordinal, leap }`: the extra เดือน 8 แรก of an *adhikamāsa* year is
  `Month::leap(8)` and the regular เดือน 8 หลัง, in which Asalha Bucha falls,
  is `Month::regular(8)`, the convention of the Hindu and Burmese calendars
  here. The day is counted 1 to 30 straight through the month, with the
  fortnight and the day within it as the extra fields `waning` and
  `fortnight-day`; the module documentation gives the layout. The functions
  `makha_bucha`, `visakha_bucha`, `asalha_bucha` and `khao_phansa` give the
  holy days, and `hc-holiday` dates Thailand's public holidays on them.
- **Year number** the Buddhist Era of the Gregorian year the year's Makha
  Bucha falls in. A year runs from ขึ้น 1 ค่ำ เดือนอ้าย, in November or
  December, to the end of month 12, so the number changes some weeks before
  the civil year does. That is this library's convention, chosen so that a
  year is one run of months; it is not how a Thai document dates a December
  day, and the older reckonings changed the year at month 5 or at Songkran.
- **Range** 2535 to 2570 BE, ขึ้น 1 ค่ำ เดือนอ้าย of 2535 on 7 December 1991
  to the end of month 12 of 2570 on 28 November 2027, and the first six
  months of 2571, to 23 May 2028, which no year type can change. Month 7 of
  2571 on, and whether it doubles month 8, wait on a type Thailand has not
  published and are refused with `AfterSupportedRange`; every earlier day is
  refused with `BeforeEpoch`. A year is added when the Bank publishes it.
- **The year types as data.** The table `YEAR_TYPES` holds one type a year,
  read off the published holy days as the previous section describes. The
  published days, with the type they name, are these. A date in parentheses
  is the weekday the list gave in place of a holy day on a Saturday or
  Sunday; the day before it is the weekend day the checks leave. To 2006
  the lists name Khao Phansa rather than Asalha Bucha, so a stand-in in the
  last column of those years is Khao Phansa's, and the Asalha Bucha shown
  is the day before the Khao Phansa it stood in for.

  | BE | CE | Type | Makha Bucha | Visakha Bucha | Asalha Bucha |
  | --- | --- | --- | --- | --- | --- |
  | 2535 | 1992 | N | 18 Feb | 16 May (Mon 18 May) | 14 Jul |
  | 2536 | 1993 | M | 7 Mar (Mon 8 Mar) | 4 Jun | 2 Aug |
  | 2537 | 1994 | N | 25 Feb | 24 May | 22 Jul (Khao Phansa Sat 23 Jul; Mon 25 Jul) |
  | 2538 | 1995 | N | 14 Feb | 13 May (Mon 15 May) | 11 Jul |
  | 2539 | 1996 | M | 3 Mar (Thu 29 Feb) | 31 May | 29 Jul |
  | 2540 | 1997 | V | 21 Feb | 20 May | 19 Jul (Khao Phansa Sun 20 Jul; Mon 21 Jul) |
  | 2541 | 1998 | N | 11 Feb | 10 May (Mon 11 May) | 8 Jul |
  | 2542 | 1999 | M | 1 Mar | 29 May (Mon 31 May) | 27 Jul |
  | 2543 | 2000 | V | 19 Feb (Mon 21 Feb) | 17 May | 16 Jul |
  | 2544 | 2001 | N | 8 Feb | 7 May | 5 Jul |
  | 2545 | 2002 | M | 26 Feb | 26 May (Mon 27 May) | 24 Jul |
  | 2546 | 2003 | N | 16 Feb (Mon 17 Feb) | 15 May | 13 Jul |
  | 2547 | 2004 | M | 5 Mar | 2 Jun | 31 Jul |
  | 2548 | 2005 | V | 23 Feb | 22 May | 21 Jul |
  | 2549 | 2006 | N | 13 Feb | 12 May | 10 Jul |
  | 2550 | 2007 | M | 3 Mar | 31 May | 29 Jul |
  | 2551 | 2008 | N | 21 Feb | 19 May | 17 Jul |
  | 2552 | 2009 | V | 9 Feb | 8 May | 7 Jul |
  | 2553 | 2010 | M | 28 Feb | 28 May | 26 Jul |
  | 2554 | 2011 | N | 18 Feb | 17 May | 15 Jul |
  | 2555 | 2012 | M | 7 Mar | 4 Jun | 2 Aug |
  | 2556 | 2013 | N | 25 Feb | 24 May | 22 Jul |
  | 2557 | 2014 | N | 14 Feb | 13 May | 11 Jul |
  | 2558 | 2015 | M | 4 Mar | 1 Jun | 30 Jul |
  | 2559 | 2016 | V | 22 Feb | 20 May | 19 Jul |
  | 2560 | 2017 | N | 11 Feb | 10 May | 8 Jul |
  | 2561 | 2018 | M | 1 Mar | 29 May | 27 Jul |
  | 2562 | 2019 | N | 19 Feb | 18 May | 16 Jul |
  | 2563 | 2020 | V | 8 Feb | 6 May | 5 Jul |
  | 2564 | 2021 | M | 26 Feb | 26 May | 24 Jul |
  | 2565 | 2022 | N | 16 Feb | 15 May | 13 Jul |
  | 2566 | 2023 | M | 6 Mar | 3 Jun | 1 Aug |
  | 2567 | 2024 | N | 24 Feb | 22 May | 20 Jul |
  | 2568 | 2025 | V | 12 Feb | 11 May | 10 Jul |
  | 2569 | 2026 | M | 3 Mar | 31 May | 29 Jul |
  | 2570 | 2027 | N | 21 Feb | 20 May | 18 Jul |

  N is normal, V *adhikavāra*, M *adhikamāsa*: thirteen M and seven V in the
  thirty-six. Eleven of the 108 days were published as stand-ins; the
  module's test names seven of the weekend days they resolve to, among them
  3 March 1996, 16 May 1992, 19 February 2000 and 16 February 2003, and
  the calendar gives the other four as the table shows.
- **The printed year**, in `hc-calendars-solar`, as `printed_year(rd)`, which
  gives `(PrintedEra, year)` for any day from 1 April 1889, and
  `printed_to_fixed(era, year, month, day)`, which reads a dateline back.
  The calendar `buddhist` itself keeps the modern year for every date, as
  Thai references do when they give a historical date in BE; only the
  printed form is a function of the day.
- **Not carried:**
  - *Eade's suriyayatra rule*, and with it any year outside the table. The
    reason the module and the roadmap give is that the rule as Eade states
    it reproduces his own year types for CS 1320–1340 but was found to
    disagree with eight of the thirty-six years of Thai Wikipedia's table of
    Makha Bucha dates for 1996–2031 [wikipedia-th-makha-bucha], and that the
    table itself departs from the Bank's published dates in 1997, 2025, 2026
    and 2027, so how far the rule is from the published calendar is not
    settled. That comparison was made when the module was written and its
    working is not in the repository, so the count of eight cannot be
    checked from what is carried; the four departures of the Wikipedia table
    can be, and are, in the accuracy section. The rule also needs the
    Chulasakarat year and the Thai sine tables, and a rule that the
    published calendar is known to depart from would give a date with no
    way to say whether it is the one Thailand kept. So the calendar refuses
    rather than extrapolates.
  - *The lunar reckoning before 1 April 1889*, in the Chulasakarat era, for
    the printed year: `printed_year` refuses it. The Thai lunar calendar of
    those years is the same calendar as `thai-lunar`, but no published year
    types are carried for it.
  - *Other Buddhist eras*: the Burmese, Sinhalese, Khmer and Lao eras share
    the name with different epochs and, in several cases, a lunisolar year;
    `buddhist` is the Thai reckoning only, whose offset is 543.

## Accuracy

The reference is the published calendar itself, so the measure is whether
every published holy day is reproduced, and it is:

| Check | Test | Result |
| --- | --- | --- |
| Makha, Visakha and Asalha Bucha (Khao Phansa to 2006) of every year 2535–2570 as the Bank published them | `every_published_holiday_is_reproduced` | 108 of 108: 97 given as the day, 11 as the weekday that stood in, each of which resolves to a weekend day within four days of it |
| Khao Phansa is the day after Asalha Bucha | the same | 36 of 36 |
| The seven weekend days that the neighbouring years settle | `the_weekend_days_resolve_as_the_neighbouring_years_require` | 7 of 7 |
| Every year's months sum to its type's length and the next year begins there; thirteen M and seven V | `month_lengths_follow_the_year_type` | 36 of 36 |
| Every day of the range round-trips and the days are consecutive | `every_day_of_the_range_round_trips` | 13 318 days |
| Thailand's four public holidays on the calendar, exact for 1992–2027 | `hc-holiday`'s `thailand_keeps_its_buddhist_days_on_the_thai_lunar_calendar` | 21 dates and 4 substitute Mondays |
| The printed year: RS 108 from 1 April 1889, 2456 from 1 April 1913, 2483 nine months long, 2484 from 1 January 1941, and datelines round-trip weekly to 1960 | `buddhist`'s `a_document_before_1941_printed_january_to_march_a_year_lower`, `the_rattanakosin_era_ran_from_1889_to_march_1913`, `the_year_2483_was_nine_months_long`, `a_printed_dateline_round_trips` | all |

Known disagreements are with a secondary source, not the published one.
Thai Wikipedia's table of Makha Bucha dates for 2539–2574 BE
[wikipedia-th-makha-bucha] gives 22 February 1997, 13 February 2025,
4 March 2026 and 22 February 2027, each a day after the Bank's date, which
is why the module does not use it. As read on 2026-09-25 the table also
gives 4 March 1996, the Monday after the Sunday 3 March that the Bank's
Thursday stand-in and the neighbouring years fix; the module counts four
departures and does not name this one.

On 2026-09-25 the sources were re-read as far as they could be. The Bank's
archived lists for 1992, 2007 and 2022, the Royal Gazette copy of the
notification for 2024 and the header of the one for 2027 agree with the
table above; the 2027 dates themselves were read from press reports of the
notice, as the gazette PDF's text does not extract, and the archived lists
for 1993–2006 and 2008–2021 now redirect to the Bank's current site, so
those years rest on the module's reading of 2026-09-23.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [bot-fiholiday] | The Makha, Visakha and Khao Phansa (to 2006) or Asalha Bucha (from 2007) dates of 2535–2565 BE, and the 2007 note that Asalha Bucha replaced Khao Phansa on the advice of the Office of National Buddhism | 1992, 2007 and 2022 re-read 2026-09-25 through the Internet Archive; the other years read 2026-09-23 and now unreachable there |
| [bot-fpg3-2565] | The dates of 2566 BE (2023) | Read 2026-09-23; on 2026-09-25 through press reports only |
| [bot-fpg8-2566] | The dates of 2567 BE (2024) | Yes, the Royal Gazette copy, 2026-09-25 |
| [bot-fpg5-2567] | The dates of 2568 BE (2025) | Read 2026-09-23; on 2026-09-25 through press reports only |
| [bot-31-2568] | The dates of 2569 BE (2026) and the worked example | Read 2026-09-23; on 2026-09-25 the Bank's covering letter, which gives its gazette citation |
| [bot-37-2569] | The dates of 2570 BE (2027) | The gazette PDF retrieved 2026-09-25, its header legible and its body not; the dates through press reports of it |
| [eade2000] | The *suriyayatra* rules, the CS 1320–1340 year types, the Thai and Burmese exclusion rules, the Mahāvagga's two dates for the rains retreat | Yes, 2026-09-25, from the Siam Society's scan by OCR |
| [payutto-dictionary] | The structure of the year types under อธิกมาส | Cited by the module; not reachable on 2026-09-25 (HTTP 403) |
| [wikipedia-th-thai-lunar] | The month names and lengths, the fortnights, the placing of the extra month and day | Yes, 2026-09-25 |
| [wikipedia-th-makha-bucha] | The table of Makha Bucha dates the module disagrees with | Yes, 2026-09-25 |
| [proclamation-new-day-rs107] | The solar reckoning from 1 April 1889, the year from 1 April, the Rattanakosin era | Yes, 2026-09-25, in the Wikisource transcription, which cites กฎหมายไทย เล่ม 3 and no gazette page |
| [proclamation-counting-rs131] | The Buddhist Era from 1 April 2456 | Not read directly; the date and the gazette page are as Thai Wikipedia's ปฏิทินสุริยคติไทย cites them |
| [calendar-years-act-2483] | The year from 1 January 2484 and the nine months of 2483 | Yes, 2026-09-25, in the Wikisource transcription, which gives the gazette citation |

The module's statement that the *suriyayatra* rule "reproduces his own year
types but not the Buddhist holidays Thailand actually kept", and the count
of eight disagreeing years, rest on a computation made when the module was
written and not retained; no source in this table states either. The
convention that the older reckonings changed the year at month 5 or at
Songkran is the module's, and this document names no source for it.

## Code

`crates/hc-calendars-regional/src/thai_lunar.rs`, with the table
`YEAR_TYPES` and the constants `FIRST_YEAR`, `LAST_YEAR` and
`MONTHS_KNOWN_AFTER_LAST_YEAR`. Anchors:
`every_published_holiday_is_reproduced`,
`the_weekend_days_resolve_as_the_neighbouring_years_require`,
`the_adhikamasa_year_2569_doubles_month_8`,
`month_lengths_follow_the_year_type`, `the_range_ends_where_the_table_does`.
The printed year is `crates/hc-calendars-solar/src/buddhist.rs`,
`PrintedEra`, `printed_year` and `printed_to_fixed`, anchored by
`a_document_before_1941_printed_january_to_march_a_year_lower`,
`the_rattanakosin_era_ran_from_1889_to_march_1913`,
`the_year_2483_was_nine_months_long` and `a_printed_dateline_round_trips`.
Thailand's holidays on the calendar are in `crates/hc-holiday`, `TH` in
`countries/asia.rs`, through `CalendarSystem::THAI_LUNAR`.
