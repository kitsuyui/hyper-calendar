# The Yazidi year: Serêsal and the Eastern calendar

Backs the identifier `yazidi` in `hc-calendars-solar`.

## What it is

The Yazidis of northern Iraq, and their diaspora, date their religious
festivals by what Kreyenbroek calls "the Seleucid or 'Eastern' calendar,
which in this century is thirteen days behind the Gregorian or 'Western'
one" — that is, the Julian calendar under the Syriac month names, Nisan
for April, as the Christians of the same region kept it
[kreyenbroek1995, p. 164 n. 53]. Their year begins in Nisan, "the first
month of the Yezidi year" [kreyenbroek1995, p. 150], with *Serêsal*
(*Sersal*, "head of the year"), also *Çarşema Sor*, "Red Wednesday", or
*Çarşema Serê Nîsanê*, "the Wednesday at the head of Nisan": "The Yezidi
New Year (Serêsal) is celebrated on the first Wednesday of Nisan (April)"
[kreyenbroek1995, p. 151; rodziewicz2020]. Since Eastern 1 April is
Gregorian 14 April from 1900 to 2099, that is the first Wednesday on or
after 14 April Gregorian [wikipedia-yazidi-new-year, citing bozarslan2021]:
19 April in 2023, 17 April in 2024, 15 April in 2026 [nlka2023, rudaw2024].
The day is a Wednesday because Wednesday is the day of the week the
Yazidis hold sacred, the day of the creation of the world and of Tawûsî
Melek [rodziewicz2020].

The year number the community prints at the feast is the Gregorian year
plus 4750: 6764 in 2014, 6773 on 19 April 2023, 6774 on 17 April 2024
[yezidisinternational-sersal, nlka2023, kurdistanwatch2024]. It is the
same count as the modern Assyrian calendar's, whose epoch of 4750 BC was
fixed in 1955 ([assyrian.md](assyrian.md)); nothing read says when or from
where Yazidis adopted it, and no source read gives a Yazidi year in the
Seleucid era, the count from 312 BC. The "Seleucid calendar" of the
sources is the scheme of months, not a year number.

## How it works

**Serêsal.** For a Yazidi year *Y*, take the Julian year *Y* − 4750, find
its 1 April, and go forward to the first Wednesday on or after it. That
day is 1 of year *Y*; the day before it is the last day of *Y* − 1.

**The year's length.** Because the new year is fixed by a weekday, each
year is a whole number of weeks: 364 days, or 371 in the year before a
Julian April whose first Wednesday falls late enough — roughly every fifth
or sixth year, since 365.25 days a year leave a day and a quarter over
each year, and 1.25 × 7 ≈ 8.75 years are needed to make up a week. 14 April
2021 was itself a Wednesday, so 6771 began on it; the next Serêsal was
20 April 2022, and 6771 had 371 days.

**Worked example.** What is the Yazidi date of 6 October 2024? The Julian
date is 23 September 2024, the first day of the Feast of the Assembly,
which Kreyenbroek dates 23–30 September in the Seleucid calendar
[kreyenbroek1995, p. 151]. The Julian year is 2024, so the candidate year
is 2024 + 4750 = 6774, whose Serêsal is the first Wednesday on or after
Julian 1 April 2024 — Gregorian 14 April 2024, a Sunday — so Wednesday
17 April 2024. 6 October is after it, so the year is 6774, and the day of
the year is the days from 17 April to 6 October plus one: 173. So year
6774, day 173, Eastern 23 September.

**Why not months.** A month-and-day shape cannot hold this year. Eastern
1–6 April fall before Serêsal in some years and on or after it in others,
so a year of the count would carry those dates twice when its Serêsal is
early and its successor's late, and not at all in the opposite case. The
library therefore carries the year and the day of the year from Serêsal,
and the Eastern date beneath as a derived value, which is what a festival
dated "23 September (Seleucid)" needs. No table of Yazidi month names was
found in the sources read — Kreyenbroek names only Nisan — so no month
cycle is declared.

## What is carried

- **Identifier** `yazidi`, with the year and the extra field `day-of-year`,
  1 on Serêsal through 364 or 371, and the informative extras
  `eastern-month` and `eastern-day`, the Julian month and day, which
  `YazidiDate::eastern_date` also gives. The shape declares a
  `day-of-year` cycle of 364 or 371 positions and the seven-day week, and
  no month.
- **Year number** the Gregorian year plus 4750 at Serêsal, `EpochForward`
  from year 1 in 4750 BC. `usage` is unrecorded, since nothing read says
  when the count was adopted.
- **Range** year 1 to 999 999, Serêsal of year 1 (April of 4750 BC,
  proleptic Julian) to the day before Serêsal of year 1 000 000.
- **Not carried:**
  - *A month table*, for want of a source; the Eastern months are
    `julian`'s and are reachable through the extras.
  - *The Seleucid era* as a year count, for want of a source that a Yazidi
    year is written in it. It would be its own identifier.
  - *The festivals* as named days. They are `hc-holiday`'s `yazidi` table
    on the Julian calendar: Serêsal, the Forty Days of Summer from
    10 Haziran, the Feast of the Assembly of 23–30 September, and the
    three-day winter fast before Bêlinde on 1 December [kreyenbroek1995,
    pp. 151–155]. The village tiwafs, the Feast of the Dead and
    Khidr-Ilyas, which the source reports only as "said to fall" on their
    dates, and the feasts on the Islamic calendar are not carried.
  - *The day boundary.* The celebrations begin on the Tuesday evening,
    the Yazidi day beginning at sunset [wikipedia-yazidi-new-year]; no
    source read states the calendar day's boundary as a rule, so midnight
    is left as the default.

## Accuracy

The reference is the rule and the published dates:

| Check | Test | Result |
| --- | --- | --- |
| 19 April 2023 is Serêsal 6773 [nlka2023]; 17 April 2024 is 6774 [rudaw2024, kurdistanwatch2024]; 15 April 2026; 16 April 2014 is 6764 [yezidisinternational-sersal]; each Serêsal of 6773–6776 is a Wednesday in the first week of Eastern April | `sersal_is_the_first_wednesday_of_eastern_nisan` | all |
| Serêsal of every year 1900–2099 is the first Wednesday on or after 14 April Gregorian, and on or after 13 April in 1850 | `this_century_it_is_the_first_wednesday_on_or_after_14_april_gregorian` | 200 years |
| The years are 364 or 371 days; 6771 is 371; 17 to 19 of 6700–6800 are long; the day before a Serêsal is day 364 or 371 of the year before | `the_year_is_364_or_371_days_and_the_day_before_sersal_ends_it` | all |
| The Eastern date beneath is the Julian one: Serêsal 6774 is 4 April 2024 Julian, and 6 October 2024 is 23 September | `the_eastern_date_beneath_is_the_julian_one` | all |
| Every day of 6766–6776 round-trips in order; every sampled day of a 3.5-million-day span round-trips | `every_single_day_of_a_decade_round_trips_in_order`, `every_day_of_a_wide_range_round_trips` | 4 018 and 36 083 days |

There is no published table to disagree with. The gap is the month table
and the origin of the year count, both of which a community-published
calendar would supply.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [kreyenbroek1995] | The Eastern (Seleucid) calendar thirteen days behind the Gregorian, Nisan as the first month, Serêsal on the first Wednesday of Nisan, the Feast of the Assembly's dates, and the other feasts `hc-holiday` carries | Yes, the archive.org text, 2026-09-25, and pp. 150–156 again 2026-09-26 |
| [rodziewicz2020] | The first Wednesday of Nisan; the sanctity of Wednesday | Abstract only, 2026-09-25 |
| [wikipedia-yazidi-new-year] | The first Wednesday on or after 14 April Gregorian; the Tuesday-evening start | Yes, 2026-09-25 |
| [bozarslan2021] | The same rule | Not read; cited by Wikipedia |
| [nlka2023] | 19 April 2023 as 6773 | Yes, 2026-09-25 |
| [rudaw2024] | 17 April 2024 | Yes, 2026-09-25 |
| [kurdistanwatch2024] | 6774 in 2024 | The search-engine excerpt, 2026-09-25 |
| [yezidisinternational-sersal] | 6764 in 2014 | Yes, 2026-09-25 |

## Code

`crates/hc-calendars-solar/src/yazidi.rs`. Anchors:
`sersal_is_the_first_wednesday_of_eastern_nisan`,
`this_century_it_is_the_first_wednesday_on_or_after_14_april_gregorian`,
`the_eastern_date_beneath_is_the_julian_one`. The rule is `new_year`; the
offset `YEAR_OFFSET`; the Eastern date `YazidiDate::eastern_date`.
