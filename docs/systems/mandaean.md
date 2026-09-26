# The Mandaean calendar: the Parwanaia and the years after Adam

Backs the identifier `mandaean` in `hc-calendars-solar`.

## What it is

The calendar of the Mandaeans of southern Iraq and Khuzestan, on which
their feasts, fasts and inauspicious (*mbattal*) days fall. It is a year of
exactly 365 days: twelve months of thirty days, each named for a sign of
the zodiac, and five intercalary days, the *Parwanaia* or *Panja*, "which
fall between the 30th day of Shumbulta and the 1st day of Qaina" — after
the eighth month, not at the year's end [drower1937, p. 83]. Häberl
identifies it with the Sasanian civil calendar, quoting Taqizadeh's
judgement that it is the "only true continuance" of that calendar and
Stern's that its months are "exactly coterminous" with the late Sasanian
Zoroastrian ones, and finds its structure "faithfully maintained since
472 CE at the latest" [haberl2021].

Because nothing is ever intercalated, the year slips a day against the
seasons at every Gregorian leap year. Drower, writing in January 1935,
found the new year, *Dehwa Rabba*, on 8 August, "in the midst of the summer
heat, Qam Daula the First of Winter", and Panja, the spring baptismal
feast, on 5 April [drower1937, pp. 84, 90]; Häberl has the new year on
18 July from 2016 to 2019 and 17 July from 2020 to 2023 [haberl2021].

The months have a second set of names, Babylonian-derived — Shabat, Adar,
Nisan and so on — which Drower notes "do not correspond in season to their
Jewish or Turkish namesakes" [drower1937, p. 84], and are grouped into four
seasons of three, each divided into First, Middle and Last (*Awwal, Misai,
Akhir*): Awwal Gita, "the first of summer", is the first day of the seventh
month, Aria. Years are not numbered in use: each is named for the weekday
it began on, so that Drower's 1934–35 was "the year of Arba Habshaba", the
Year of Wednesday, and Häberl's manuscript colophons date by that name and
a Hijri year [drower1937, p. 84; haberl2021].

## How it works

**The month table.** In the calendar's own order, with Drower's forms:

| Position | Month | Zodiac | Babylonian name | Season |
| --- | --- | --- | --- | --- |
| 1 | Daula | Aquarius | Shabat | Awwal Sitwa, first of winter |
| 2 | Nuna | Pisces | Adar | Misai Sitwa |
| 3 | Umbara | Aries | Nisan | Akhir Sitwa |
| 4 | Taura | Taurus | Ayar | Awwal Abhar, first of spring |
| 5 | Silmia | Gemini | Siwan | Misai Abhar |
| 6 | Sartana | Cancer | Tammuz | Akhir Abhar |
| 7 | Aria | Leo | Ab | Awwal Gita, first of summer |
| 8 | Shumbulta | Virgo | Ellul | Misai Gita |
| 9 | *Parwanaia* | — | — | the five intercalary days |
| 10 | Qaina | Libra | Tishrin | Akhir Gita |
| 11 | Arqba | Scorpio | Mashrwan | Awwal Paiz, first of autumn |
| 12 | Hatia | Sagittarius | Kanun | Misai Paiz |
| 13 | Gadia | Capricorn | Tabit | Akhir Paiz |

The position column is this library's, which counts the Parwanaia as a
ninth position so that the thirteen fit one cycle in order; Mandaeans
count Qaina as the ninth month and Gadia as the twelfth, and the module
gives that number as `traditional_month`. Häberl's transliterations
(Dawla, Nuna, Embra, Tawra, Selmi, Sartana, Arya, Šombolta, Qayna, Arqwa,
Hetya, Gadya; Parwanaya) are the same months.

**Days of the year.** Positions 1–8 occupy days 0–239 of the year,
thirty each; the Parwanaia days 240–244; positions 10–13 days 245–364. The
year is 365 days, so any two dates are a fixed number of days apart, and
converting is counting.

**The era.** The *Ginza Rabba* gives the world an epoch of 480,000 years
from the creation of Adam, in seven ages; Häberl works the count through
the chronology and finds that the new year of 18 July 2019 "corresponded
to 1 Šabat / Dawla AA 481,343", AA being years after Adam, and that AA 1
falls in February of 479,004 BCE by the proleptic Gregorian calendar
[haberl2021]. That is the year number carried here. It is a scholar's
reckoning of a scriptural era, published once, and not a number Mandaeans
write on documents; it is carried because a calendar needs a year number
and this is the only one in print, and the era code `aa` marks it.

**Worked example.** What is the Mandaean date of 25 September 2026? The
last new year was 1 Daula 481,350 AA, on 16 July 2026: from Häberl's
18 July 2019 (481,343), seven years of 365 days, less the leap days of
29 February 2020 and 2024, is 16 July 2026. From 16 July to 25 September
is 71 days, so the day of the year is 71: position 71 ÷ 30 + 1 = 3, Umbara,
day 71 − 60 + 1 = 12. So 12 Umbara 481,350 AA. The year began on a
Thursday, so it is a Year of Thursday (*Hamshabba*). Panja of that year
begins 240 days after 16 July 2026: 13 March 2027.

## What is carried

- **Identifier** `mandaean`, with the year, the position 1–13 and the day,
  under the era code `aa`. The thirteen position names are declared with
  the shape in Drower's forms; the Babylonian names are the module's
  `BABYLONIAN_NAMES` and `MandaeanDate::babylonian_name`; the weekday the
  year is named for is `year_weekday`; `parwanaia` and `new_year` give the
  feast days.
- **Epoch** 1 Daula 1 AA, computed as 18 July 2019 less 481,342 × 365 days,
  which lands in February of astronomical year −479,003 (479,004 BCE), as
  Häberl states.
- **Range** year 1 to 999 999 AA.
- **Not carried:**
  - *The seasons and their thirds*, which are names for the months already
    carried.
  - *The feast days* as named days: Dehwa Hnina on 18 Taura, Ashuriyah on
    1 Sartana, Dehwa Daimana on 1 Hatia, the mbattal days and the rest
    [drower1937, pp. 60, 85–92, 211]. They are fixed dates on this
    calendar and are `hc-holiday`'s `mandaean` table, which Drower's
    Dehwa Hnina of 23 November 1932 and 1935 checks.
  - *The day boundary.* Drower describes the new year's vigil from sunset
    and the thirty-six hours indoors; nothing read states where the
    calendar day begins, so midnight is left as the default.
  - *Any correction.* Drower expected one "before another eighty years
    shall have elapsed" to keep Panja at the spring flood [drower1937,
    p. 92]; Häberl's dates show none was made.

## Accuracy

The references are the published equivalences, and every one read is
reproduced:

| Check | Test | Result |
| --- | --- | --- |
| The new year on 18 July 2016–2019 and 17 July 2020–2023; 18 July 2019 is 1 Daula 481,343; the year from Wednesday 18 July 2018 begins on a Wednesday [haberl2021] | `haberls_new_years_of_2016_to_2023_and_his_year_number` | 8 of 8, and the year number |
| Wikipedia's 2024 dates, 16 July and Parwanaya from 13 March [wikipedia-mandaean-calendar] | the same | 2 of 2 |
| The new year on 8 August 1934 and 1935; 29 January 1935 as 25 Sartana in the Year of Wednesday; Awwal Gita on 4 February 1935; Panja on 5 April 1932–1935 and 4 April 1936 [drower1937] | `drowers_dates_of_the_1930s` | 10 of 10 |
| The epoch in February 479,004 BCE | `the_epoch_is_the_creation_of_adam_in_february_479004_bc` | year and month |
| Every day of two years round-trips in order; every sampled day of a four-million-day span round-trips | `every_single_day_of_two_years_round_trips_in_order`, `every_day_of_a_wide_range_round_trips` | 730 and 44 944 days |

The known disagreement is Petermann's record of 1854, which Drower quotes:
Awwal Gita on 23 February, Awwal Paiz on 28 May, Awwal Sitwa on 26 August
and Awwal Abhar on 24 November [drower1937, p. 92]. Run back from 1934 at
365 days a year, the present reckoning gives 23 February for the first and
29 May, 27 August and 25 November for the other three — one day later —
and the test `petermanns_1854_dates_differ_by_a_day_from_the_present_reckoning`
records it. Drower herself counts "nineteen days between the Awwal Gita of
1854 and that of 1935", which agrees with the present reckoning; whether
Petermann's other three dates reflect a day boundary, a slip in his notes
or a day the calendar has since gained is not something the sources read
settle, and the library follows the twentieth- and twenty-first-century
dates.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [drower1937] | The year's structure and the month table (pp. 83–85), the month names in the forms used here, the 1930s dates (pp. 82, 84, 90), Petermann's 1854 record (p. 92), the feasts and mbattal days (pp. 60, 85–92, 211) | Yes, the archive.org text, 2026-09-25, and the feasts again 2026-09-26 |
| [haberl2021] | The Sasanian identification, the 2016–2023 new years, the Year of Wednesday, the era after Adam and its anchor and epoch | Yes, the author's proof, 2026-09-25; the figures in it are set in a font the text extraction had to be mapped for, and every figure used here was checked against a known one |
| [wikipedia-mandaean-calendar] | The 2024 festival dates, as a check | Yes, 2026-09-25 |

Petermann's *Reisen im Orient* (1861), the source of the 1854 dates, was
not read; Drower's quotation of it is what is cited.

## Code

`crates/hc-calendars-solar/src/mandaean.rs`. Anchors:
`haberls_new_years_of_2016_to_2023_and_his_year_number`,
`drowers_dates_of_the_1930s`,
`the_epoch_is_the_creation_of_adam_in_february_479004_bc`; the recorded
disagreement:
`petermanns_1854_dates_differ_by_a_day_from_the_present_reckoning`. The
month names are the module's `MONTHS`, the Parwanaia position `PARWANAIA`,
the era `ERA`.
