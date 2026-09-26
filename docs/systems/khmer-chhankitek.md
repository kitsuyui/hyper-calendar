# The Khmer *Chhankitek*, and the *suryayatra* arithmetic of Thailand, Laos and Cambodia

Backs the identifier `khmer` in `hc-calendars-regional` and the shared
module `southeast_asian`, which holds the month layout that `khmer` and
`thai-lunar` both use and the *suryayatra* quantities `khmer` computes its
years from. It also records why `lao`, `sinhalese-lunar` and `shan`, the
other members of the family on the roadmap, are not carried.

## What it is

**The calendar.** ចន្ទគតិ, *Chhankitek* — the word means "lunar calendar" —
is the lunisolar calendar of Cambodia. Cambodians use the Gregorian calendar
for civil purposes and this one for religious ones [tum-chhankitek]; the
Royal Government's annual sub-decree on public holidays dates Visak Bochea,
Pchum Ben and the Water Festival on it [ibc-kh-holidays-2024-2025,
andersen-kh-holidays-2026], and each year's New Year announcement gives the
lunar date of every day of the festival [freshnews-songkran-2022 …
freshnews-songkran-2026].

It has twelve months of 29 and 30 days in turn, Migasir first
[wikipedia-km-chankitek, tum-chhankitek]:

| Month | Khmer | UNGEGN | Days |
| --- | --- | --- | --- |
| 1 | មិគសិរ | Mĭkôsĕr | 29 |
| 2 | បុស្ស | Bŏss | 30 |
| 3 | មាឃ | Méakh | 29 |
| 4 | ផល្គុន | Phâlkŭn | 30 |
| 5 | ចេត្រ | Chétr | 29 |
| 6 | ពិសាខ | Pĭsakh | 30 |
| 7 | ជេស្ឋ | Chésth | 29, or 30 in a leap-day year |
| 8 | អាសាឍ | Asath | 30; in a leap-month year twice, បឋមាសាឍ and ទុតិយាសាឍ, 30 each |
| 9 | ស្រាពណ៍ | Srapôn | 29 |
| 10 | ភទ្របទ | Phôtrôbât | 30 |
| 11 | អស្សុជ | Âssŏch | 29 |
| 12 | កត្តិក | Kâtdĕk | 30 |

The Khmer forms are Khmer Wikipedia's [wikipedia-km-chankitek]; the UNGEGN
romanisations are English Wikipedia's [wikipedia-month], which prints month 6
as វិសាខ/ពិសាខ and month 8 as ឤសាឍ, with the deprecated independent vowel
U+17A4, where Khmer Wikipedia, the New Year announcements and Tum write
ពិសាខ and អាសាឍ. The days of a month are counted in two halves, 1 to
15 កើត (*keit*, waxing) and 1 to 14 or 15 រោច (*roaj*, waning), with no
day skipped or repeated [tum-chhankitek, wikipedia-km-chankitek].

**Three kinds of year.** Khmer Wikipedia names them បកតិមាស បកតិវារៈ, twelve
months and 354 days; អធិកមាស បកតិវារៈ, thirteen months with Asath twice, 384
days; and បកតិមាស អធិកវារៈ, twelve months with a 30-day Jesth, 355 days
[wikipedia-km-chankitek]. Tum calls the second *Adhikameas* and the third
*Chhantrea Thimeas* or *Adhikavereak*, and states the two rules that make
the Khmer calendar simpler to lay out than the Indian ones: the extra month
is always Ashadha, doubled as បឋមាសាឍ and ទុតិយាសាឍ, and the extra day is
always added to Jyestha; and "only one type of leap year can occur at a
time" [tum-chhankitek]. This is the Thai layout month for month: the Thai
calendar doubles its month 8, Āṣāḍha, and gives its month 7, Jyeṣṭha, a
30th day, and never both, as [thai-lunar.md](thai-lunar.md) sets out.
Gislén and Eade describe Thailand, Laos and Cambodia under one heading, with
one set of rules, and with the Thai, Lao and Khmer month names side by side
[gisleneade2019, pp. 421–422].

**The eras.** The Buddhist Era is the one Cambodians most use, 544 ahead of
the Common Era [tum-chhankitek]. It changes in the middle of the lunar year,
at the full moon of Pisakh: the New Year announcement for 2025 says that the
Buddhist Era was 2568 "until Sunday, 15 keit of Pisakh", and from Monday,
1 roaj of Pisakh, 2569 [freshnews-songkran-2025], and the announcements for
2022, 2023, 2024 and 2026 say the same of their years. Tum's appendix
instead has the Buddhist year begin on the *Laeung Sak* day of the New Year
festival [tum-chhankitek]; this document follows the announcements. The
Jolak Sakaraj (the Chulasakarat, the era of 638 CE) and the Moha Sakaraj
(the Śaka era) are in use too, and they, the twelve animals and the ten
*sak* change at the solar New Year in April [tum-chhankitek,
freshnews-songkran-2025].

**The New Year is solar.** Khmer New Year falls on 13 or 14 April and lasts
three or four days, and it is not dated by the lunar calendar: it falls
between 4 keit of Chaet and 4 keit of Pisakh [tum-chhankitek], and each
year's announcement gives the lunar date of each of its days
[freshnews-songkran-2025].

## How it works

**Whose arithmetic.** Tum's "Khmer Chhankitek Calendar" gives the rules that
decide a year's type as the Cambodian *hora* compute them, from Roath Kim
Soeun's *Pratitin Soryakkatik-Chankatik 1900–1999*, which this library has
not read, in the Buddhist Era [tum-chhankitek]. Gislén and Eade give the
same arithmetic as the *suryayatra* rules of the Thai calendar, in the
Chulasakarat era [gisleneade2019, pp. 422–423], which Eade's 2000 paper
also states [eade2000]. The two are the same computation shifted by the
era: Tum's Buddhist year *b* is Gislén and Eade's Chulasakarat year
*b* − 1182, and every quantity below agrees for every year of the range,
which `the_buddhist_era_form_is_the_chulasakarat_form` holds.

**The quantities of the solar New Year**, for the Chulasakarat year *y*
[gisleneade2019, eqs. 4–6 and 9]:

```text
ahargana     h = ⌊(292207 · y + 373) / 800⌋ + 1     days from the epoch to the New Year's day
kammacabala  k = 800 − ((292207 · y + 373) mod 800) the Khmer kromatopol; ≤ 207 is a 366-day solar year
avoman       a = (11 · h + 650) mod 692             the excess of tithis over days, in 692nds
tithi        t = ⌊(703 · h + 650) / 692⌋ mod 30     the lunar day on which the solar year begins
```

292 207 / 800 = 365.258 75 days is the sidereal solar year the system uses.
Tum writes the same quantities in the Buddhist Era: *ahakun* =
⌊(292207 · *b* + 499) / 800⌋ + 4, *avoman* = (11 · *ahakun* + 25) mod 692
and *bodithey* = (⌊(11 · *ahakun* + 25) / 692⌋ + *ahakun* + 29) mod 30,
which are *h* + 431 739, *a* and *t*. His prose writes the avoman formula
in the year rather than in the *ahakun*, but his own table of 2000–2020
(2000: *ahakun* 929 222, *avoman* 627, *bodithey* 11) requires the *ahakun*
[tum-chhankitek].

**The leap month.** The year has a second Asath when the solar year begins
late in the lunar one: when *t*, the *bodithey*, is 25 or more or 5 or less
— after 24 Chaet or before 6 Pisakh. Two pairs of consecutive years are
exceptions: a year of 24 followed by one of 6 has the leap month, and a year
of 25 followed by one of 5 does not [tum-chhankitek, gisleneade2019 p. 422].
Eade's statement for Thailand is the same, tithi 25–29 of Caitra or 1–5 of
Vaiśākha ([thai-lunar.md](thai-lunar.md)). Gislén and Eade's prose says
"before or on 6 Vaisakha", which their own 24–6 exception contradicts; Tum's
bound is followed.

**The leap day.** The year has a 30-day Jesth when the avoman is 137 or less
in a common solar year, or 126 or less in a solar leap year: 365 days add
555 to the avoman, 366 add 566, and 692 − 555 = 137, 692 − 566 = 126
[gisleneade2019, p. 423; tum-chhankitek]. Tum adds Roath Kim Soeun's case
"avoman 137 followed by 0": the year of 137 is then a normal year and the
year of 0 takes the day [tum-chhankitek].

**When both fall in one year.** The two rules can call for both in the same
year, and a Khmer year cannot have both. Tum's rule, from Roath Kim Soeun,
is that the year keeps the month and the day moves to the next year
[tum-chhankitek]. Gislén and Eade say of Thailand that "there are quite
complicated rules on how to move the intercalary day to one of the adjacent
years", and set out Faraut's rule in a simpler equivalent form, which keeps
the weekdays flowing from one lunar year into the next: in their example of
Chulasakarat 20–39 it puts both moved days in the year after, and it also
moves two whole lunar years a day earlier, which Faraut calls "l'exception
One". They show a variant that would put one of the days in the year before,
and suspect that in practice the day was sometimes moved always forward or
always back [gisleneade2019, pp. 423, 426, 428–429]. This library carries
Tum's rule, which has no whole-year shift, and Cambodia's published dates
bear it out (see Accuracy).

So, for the Buddhist year *b*: a leap-month year if the month rule calls for
it; otherwise a leap-day year if the day rule does, or if the year before was
called for both; otherwise a normal year.

**The layout.** A year runs from 1 keit Migasir, in November or December, to
the end of Kadeuk, through the months of the table above, and the next year
begins 354, 355 or 384 days later. Tum's epoch is 1 January 1900, 1 keit of
Bos [tum-chhankitek], so 1 keit Migasir of the year the rules call 2444 was
29 days earlier, 3 December 1899. The year the rules number *b* is the one
whose Pisakh, and so whose full moon of Visak Bochea, falls in the Gregorian
year *b* − 544.

**Worked example: 2568 (2024).**

1. *The New Year quantities.* 292207 × 2568 + 499 = 750 388 075, which
   is 937 985 × 800 + 75: *ahakun* = 937 985 + 4 = 937 989, and the
   kammacabala is 800 − 75 = 725, more than 207, so the solar year is
   common. 11 × 937 989 + 25 = 10 317 904 = 14 910 × 692 + 184: the avoman
   is 184. The *bodithey* is (14 910 + 937 989 + 29) mod 30 = 952 928 mod 30
   = 8.
2. *The type.* 8 is between 6 and 24, so no leap month; 184 is more than
   137, so no leap day of its own. The year before, 2567, has *bodithey* 26
   and avoman 310: a leap month and no leap day, so nothing moves into
   2568. 2568 is a normal year of 354 days.
3. *The first day.* 2567, a leap-month year, began on 24 November 2022, so
   2568 began 384 days later, on 13 December 2023.
4. *The New Year.* Migasir, Bos, Meak and Phalkun take 29 + 30 + 29 + 30 =
   118 days, so 1 keit Chaet is 9 April 2024 and 13 April is 5 keit Chaet,
   the day the announcement gives for the New Year of 2024
   [freshnews-songkran-2024].
5. *Visak Bochea.* Chaet has 29 days, so 1 keit Pisakh is 8 May and 15 keit
   Pisakh is 22 May 2024, a Wednesday: the sub-decree's Visak Bochea
   [ibc-kh-holidays-2024-2025], and the announcement's "Wednesday, 15 keit
   of Pisakh", the last day of 2567 in the Buddhist Era.
6. *Pchum Ben.* 15 roaj of Photrobot is day 29 + 30 + 29 + 30 + 29 + 30 +
   29 + 30 + 29 + 30 = 295 of the year, 2 October 2024, the middle day of
   the sub-decree's 1–3 October.

The next year, 2569, has avoman 47 in a common solar year and so its own
leap day: Jesth 2025 has 30 days, and the year 355.

## What is carried

- **Identifier** `khmer`, in `hc-calendars-regional`. The month is
  `Month { ordinal, leap }`, Migasir 1 to Kadeuk 12, the first Asath of a
  leap-month year, បឋមាសាឍ, being `Month::leap(8)` and ទុតិយាសាឍ
  `Month::regular(8)`, as `thai-lunar`, the Hindu and the Burmese calendars
  here have it. The day is counted 1 to 30 straight through the month, with
  the half and the day within it as the extra fields `waning` and
  `fortnight-day`.
- **The year number** is the Buddhist year the rules take, so that a year
  is one run of months, Migasir to Kadeuk, as in `thai-lunar`. It is the
  Buddhist Era Cambodia prints from 1 roaj Pisakh to the end of Kadeuk; from
  Migasir to 15 keit Pisakh the printed year is one less.
  `KhmerDate::printed_year` gives the printed year and the extra field
  `printed-year` carries it, and the date's display writes it, as
  «១៥កើត ខែពិសាខ ព.ស.២៥៦៨» for 11 May 2025. The animal year, the *sak*
  and the Jolak Sakaraj, which change at the solar New Year, are not
  carried.
- **The range** is the years 2444 to 2744, 3 December 1899 to 6 December
  2200. The lower bound is Tum's epoch. Within it, the three readings of the
  rule that were compared agree on every year: Tum's statement, which tests
  the Gregorian year the New Year falls in where Gislén and Eade test the
  kammacabala; the kammacabala form this library computes; and the
  maintained JavaScript implementation [momentkh], which also carries a
  moved day on through a run of leap-month years, a run that no year of the
  range has. Outside it they part — in 1818 and in 2272 — and nothing here
  was checked against a date so far from the present.
- **The shared engine.** `southeast_asian` holds the year types, the month
  layout that both Cambodia and Thailand use, the walk from a first day
  over a run of typed years, and the *suryayatra* quantities. `thai-lunar`
  gives it a table of published year types, `khmer` the rule.
- **Not carried:**
  - *Faraut's weekday rule*, which Gislén and Eade set out for Thailand
    [gisleneade2019, pp. 428–429]. Faraut's *Astronomie Cambodgienne*
    (1910) was not read, and every Cambodian date read, from 1913 to 2027,
    falls where Tum's rule, with no whole-year shift, puts it. It is a
    second convention, and if a source shows a calendar that kept it, it
    gets its own identifier.
  - *The solar New Year* — the *Songkran* moment, the *Vonobot* and *Laeung
    Sak* days — which the announcements date and which needs the true Sun
    of the *suryayatra*, not the mean quantities above.
  - *The years before 1900 and after 2200*, as above.
  - *Lao* (`lao`). Gislén and Eade treat Laos with Thailand and Cambodia,
    and name Phetsarath's "Le calendrier lao" (1956) and Dupertuis' "Le
    calcul du calendrier laotien" (1981) as its detailed treatments
    [gisleneade2019, p. 421]; neither was read. What is missing is the rule
    Laos uses when the month and the day fall in one year — Tum's, Faraut's,
    or another — and a published Lao date in a year where the choice shows;
    until then a Lao date would be a Khmer or a Thai date given another
    name. Gislén and Eade's Lao month names, in Latin letters, are the only
    Lao vocabulary read.
  - *Sinhalese lunar* (`sinhalese-lunar`). Sri Lanka's Poya days are fixed
    each year by the Holidays Act orders, and no source read gives a rule
    that reproduces them; `hc-holiday`'s Sri Lanka table already carries the
    orders for 2023–2027, Poya days and *adhi* Poya days included.
  - *Tai, Shan and Dai* (`shan`). Gislén and Eade's table of month numbers
    gives Caitra as month 5 in the central (Sukhothai) numbering, 6 at Keng
    Tung and 7 at Chiang Mai [gisleneade2019, Table 5], so the northern
    numberings run one and two ahead of the Thai one, not of the Burmese; it
    gives no Shan or Dai numbering, and no source read says which year rule
    — Burmese or Thai — the Tai calendars follow.

## Accuracy

The reference is what Cambodia published, and every date read is
reproduced:

| Check | Test | Result |
| --- | --- | --- |
| The lunar date of every New Year day of 2022–2026 as the announcements give it, with its weekday | `the_new_years_of_2022_to_2026_fall_on_the_announced_lunar_days` | 16 of 16 |
| The day the Buddhist Era changed in 2022–2026, 1 roaj Pisakh, with the weekday of the 15 keit before it | `the_buddhist_era_changes_at_the_full_moon_of_pisakh` | 5 of 5 |
| Visak Bochea, the Royal Ploughing Ceremony, Pchum Ben and the Water Festival of 2024–2027, as the sub-decrees give them | `the_sub_decreed_days_of_2024_to_2027_are_reproduced`; `hc-holiday`'s `cambodia_dates_its_lunar_days_where_the_khmer_calendar_puts_them` | 16 of 16 in this crate; every lunar day of the 2025–2027 table there, 24 of 24 |
| Meak Bochea, Visak Bochea, Pchum Ben and the Water Festival of 2015 and 2019, as timeanddate.com listed them | `the_holidays_of_2015_and_2019_are_reproduced` | 8 of 8 |
| Khmer Wikipedia's example: Sunday 3 December 2017, 15 keit Migasir, printed 2561 | `a_date_reads_as_a_khmer_calendar_writes_it` | yes |
| The dates Tum lists as checked, from Khmer Wikipedia and other pages: 2 October 1913, 20 November 1947, 18 February 1951, 23–25 September 1969, 26 May 2005, 29 September 2008, and the two whose Gregorian year he finds misprinted, 11 January 1945 and 31 October 1988 | `tums_checked_dates_are_reproduced` | 10 of 10 lunar dates; for 18 February 1951 the page gives a Monday, which was a Sunday, and BE 2493, one less than the 2494 printed by the change at Pisakh |
| Tum's table of *ahakun*, avoman and *bodithey* for 2000–2020, and the year types of 2001–2020 | `tums_table_of_2000_to_2020_is_reproduced` | 21 rows and 20 types |
| Gislén and Eade's worked example, Chulasakarat 1238: *ahargana* 452 191, kammacabala 161, avoman 655, tithi 19 | `the_worked_example_of_chulasakarat_1238_is_reproduced` | yes |
| Every day of the range round-trips, the days are consecutive, and every year's months sum to its type | `every_day_of_the_range_round_trips`, `month_lengths_follow_the_year_type` | 109 942 days |

**What the published dates test.** Every date after 1900 tests the count of
leap days and months since the epoch, since one missing day shifts
everything after it. The dates of 2015 test the two rarer rules as well.
2014 has avoman 137 and 2015 avoman 0, the one 137-then-0 pair in the
range, and 2015 is also called for the month. By the rules, 2014 is
normal, 2015 takes the month, and its day moves into Jesth 2016. Without
the 137-then-0 case 2014 would take a day too, and Visak Bochea would fall
on 3 May 2015 and 23 May 2024. With the day moved back into 2014 rather
than forward, the dates of 2015 would move a day later. The published 2 May
2015 and 22 May 2024 hold both rules.

**Where the readings disagree.** Tum's appendix begins the Buddhist year at
*Laeung Sak* and the announcements at 1 roaj Pisakh; the announcements
decide it here. timeanddate.com's lists name no source, and the Internet
Archive's copies of its 2013 and 2014 pages give the 2015 dates, so they
were not used. The sub-decree for 2026 is No. 167 of 18 September 2025 in
Andersen's summary [andersen-kh-holidays-2026] and of 5 September 2025 in
`hc-holiday`'s citation; the dates agree.

**The Thai calendar, measured against the same rule.** Applied to the Thai
years 2535–2570 BE (1992–2027), the rule gives the year type the Bank of
Thailand's published holy days fix in 34 of the 36 years. It differs in
1994 and 1997, where the rule makes 1994 the leap-day year and Thailand's
dates make it 1997, so for three years the Thai calendar ran a day earlier
than the rule (`the_rule_gives_thailands_published_types_but_for_1994_and_1997`).
`thai-lunar` goes on carrying the published types; this is recorded, in
[thai-lunar.md](thai-lunar.md) too, because that document reports a larger
disagreement for Eade's statement of the rule, measured on a different
table.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [tum-chhankitek] | The rules for the year types from Roath Kim Soeun, with the *ahakun*, avoman and *bodithey* formulas, the 24/6, 25/5 and 137/0 cases and the moved day; the table for 2000–2020; the month layout, the epoch of 1 January 1900; the months and holy days; the checked dates; the eras | Yes, 2026-09-26, the Internet Archive's copies of the site's pages of 2008–2012; the PDF edition was not read |
| [soeun-pratitin] | The rules above, through Tum | Not read |
| [gisleneade2019] | The *suryayatra* formulas in the Chulasakarat era and the worked example of 1238; the leap-month and leap-day rules; Faraut's weekday rule; the family as one system; Table 5's month numbers; the Lao sources | Yes, 2026-09-26, the Internet Archive's copy of the NARIT PDF, by text extraction |
| [eade2000] | The Thai statement of the leap-month bound | This repository, [thai-lunar.md](thai-lunar.md) |
| [eade1995], [faraut1910], [phetsarath1956], [dupertuis1981] | The family, Faraut's rules and Laos, through [gisleneade2019] | Not read |
| [momentkh] | The third reading of the rule compared over the range | Yes, 2026-09-26, `momentkh.ts` at commit ff2bfd5, MIT licence |
| [wikipedia-km-chankitek] | The month names in Khmer script, the three year types by their Khmer names, the example of 3 December 2017 | Yes, 2026-09-26 |
| [wikipedia-month] | The UNGEGN romanisations of the month names | Yes, 2026-09-26 |
| [freshnews-songkran-2022], [freshnews-songkran-2023], [freshnews-songkran-2024], [freshnews-songkran-2025], [freshnews-songkran-2026] | The lunar dates of the New Year days and the day the Buddhist Era changed | Yes, 2026-09-26 |
| [kampucheathmey-songkran-2026] | The New Year of 2026 on Tuesday, 12 roaj Chaet | Yes, 2026-09-26 |
| [ibc-kh-holidays-2024-2025] | The sub-decreed days of 2024 and 2025 | Yes, 2026-09-26 |
| [andersen-kh-holidays-2026] | The sub-decreed days of 2026 | Yes, 2026-09-26 |
| `hc-holiday`'s Cambodia table | The sub-decreed days of 2025–2027 | This repository, read by that module 2026-09-23 |
| [timeanddate-kh-2015], [timeanddate-kh-2019] | The holidays of 2015 and 2019 | Yes, 2026-09-26, in the Internet Archive's copies |

## Code

`crates/hc-calendars-regional/src/southeast_asian.rs`: `YearType`,
`Fortnight`, the walk `Years`, and the quantities `ahargana`,
`kammacabala`, `is_solar_leap_year`, `avoman` and `new_year_tithi`.
`crates/hc-calendars-regional/src/khmer.rs`: `FIRST_YEAR`, `LAST_YEAR`,
`has_leap_month`, `has_leap_day_by_rule`, `year_type_by_rule`, `year_type`,
`KhmerDate` with `printed_year`, and `KhmerCalendar`. Anchors:
`the_new_years_of_2022_to_2026_fall_on_the_announced_lunar_days`,
`the_buddhist_era_changes_at_the_full_moon_of_pisakh`,
`the_sub_decreed_days_of_2024_to_2027_are_reproduced`,
`the_holidays_of_2015_and_2019_are_reproduced`,
`tums_checked_dates_are_reproduced`,
`tums_table_of_2000_to_2020_is_reproduced`,
`the_buddhist_era_form_is_the_chulasakarat_form`,
`the_readings_of_the_rule_agree_over_the_range`,
`the_worked_example_of_2568_is_reproduced`,
`the_rule_gives_thailands_published_types_but_for_1994_and_1997`,
`every_day_of_the_range_round_trips`; in `southeast_asian`,
`the_worked_example_of_chulasakarat_1238_is_reproduced`; in `hc-holiday`,
`cambodia_dates_its_lunar_days_where_the_khmer_calendar_puts_them`. The
English month names are in `hc-i18n`.
