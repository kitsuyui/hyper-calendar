# Rāhu kālam, Yamaganda and Gulika kālam

Backs `hc-calendars-indic::kalam`, and `hc-calendars-indic::muhurta` for
the fifteenths of the day, Abhijit and Dur Muhurtam (below). No calendar
identifier is registered: these are periods of a day, not calendars. The two conventions are two
functions, `kalam::by_sunrise` and `kalam::by_fixed_day`, and two entries
of the table `KalamConvention::ALL` under the identifiers
`rahu-kalam-sunrise` and `rahu-kalam-fixed`, by which the boundary's
`hc_kalam` selects one ([policy.md](../policy.md) §5).

## What it is

A Hindu almanac (*pañcāṅga*) marks three periods of each day as unsuitable
for starting anything new: **Rāhu kālam**, **Yamaganda** and **Gulika
kālam** (Drik Panchang spells the last "Gulikai Kalam"). Each is an eighth
of the day, and which eighth it is depends only on the weekday. Almanacs
such as Drik Panchang print all three for every day and place
[drik-day-panchang-2025], and a South Indian temple guide calls its fixed
times "the standard daily timings followed widely across South India"
[tirumala-kalam-table].

The same pages print two periods by another division of the day, into
fifteen *muhūrtas*: **Abhijit**, auspicious, and **Dur Muhurtam**,
inauspicious. "Abhijit Muhurat is the 8th Muhurat out of 15 Muhurats
which prevail between the sunrise and the sunset. The time interval
between the sunrise and the sunset is divided into 15 equal parts and the
middle portion of fifteen parts is known as Abhijit Muhurat", and it "is
not suitable on Wednesday as it forms a malefic Muhurta on this weekday"
[drik-abhijit-muhurat]; the day pages print "None" for it on every
Wednesday [drik-day-panchang-2025]. Dur Muhurtam is one or two muhūrtas
by the weekday, and on Tuesdays the second falls in the night.

## How it works

The day is cut into eight equal parts, numbered from the start of the day,
and each period takes the part its weekday gives it:

| Weekday | Rāhu kālam | Yamaganda | Gulika kālam |
| --- | --- | --- | --- |
| Sunday | 8 | 5 | 7 |
| Monday | 2 | 4 | 6 |
| Tuesday | 7 | 3 | 5 |
| Wednesday | 5 | 2 | 4 |
| Thursday | 6 | 1 | 3 |
| Friday | 4 | 7 | 2 |
| Saturday | 3 | 6 | 1 |

Rāhu kālam never takes the first part [wikipedia-rahukaalam]. The three
never share a part.

There are two readings of "the day", and they give different times:

- **Sunrise to sunset at the place** (`by_sunrise`). Drik Panchang
  calls this the *Yamardha* method, an eighth of the daytime, and says it
  is the most common of several [drik-rahu-kalam]. The parts are longer in
  summer and shorter in winter, and they differ from town to town.
- **06:00 to 18:00 of the clock** (`by_fixed_day`). Every part is an
  hour and a half, so the times are the same every day of the year and in
  every place. Wikipedia states the rule this way [wikipedia-rahukaalam],
  and temple tables print it, saying that the real times move "by a few
  minutes depending on your local sunrise" [tirumala-kalam-table].

**Worked example.** Drik Panchang gives sunrise at New Delhi on Wednesday
1 January 2025 as 07:14 IST and sunset as 17:36 [drik-day-panchang-2025]:
622 minutes of daylight, so a part is 77.75 minutes. Wednesday's Rāhu kālam
is the fifth part: it starts 4 × 77.75 = 311 minutes after sunrise, at
12:25, and ends at 13:42.75. The page prints 12:25 to 13:43. Yamaganda is
the second part, 08:31.75 to 09:49.5, printed as 08:32 to 09:49. Gulika is
the fourth, 11:07.25 to 12:25, printed as 11:07 to 12:25. On the fixed day
the same Wednesday's Rāhu kālam is 06:00 + 4 × 1.5 h = 12:00 to 13:30.

**The muhūrtas.** The daylight, sunrise to sunset, is cut into fifteen
equal muhūrtas, and the night, sunset to the next sunrise, into fifteen
more. Abhijit is the eighth of the day, and none on a Wednesday. Dur
Muhurtam takes these by the weekday:

| Weekday | Dur Muhurtam |
| --- | --- |
| Sunday | the 14th of the day |
| Monday | the 9th and the 12th of the day |
| Tuesday | the 4th of the day and the 7th of the night |
| Wednesday | the 8th of the day |
| Thursday | the 6th and the 12th of the day |
| Friday | the 4th and the 9th of the day |
| Saturday | the 1st and the 2nd of the day |

No statement of the Dur Muhurtam rule was read. The table is the
muhūrtas the times of Drik Panchang's New Delhi pages of 1 to 7 January
2025 fall in, one page for each weekday [drik-day-panchang-2025], as the
Yamaganda and Gulika columns above are, and the pages of 8 to 31 January
are the test of it.

**Worked example.** On Wednesday 1 January 2025 the pages' 07:14 to 17:36
is 622 minutes of daylight, so a muhūrta is 41.47 minutes. The eighth
begins 7 × 41.47 = 290.3 minutes after sunrise, at 12:04.3, and ends at
12:45.8: the page prints "Dur Muhurtam 12:04 PM to 12:46 PM" and
"Abhijit None". On Tuesday 7 January the night runs from sunset at 17:40
to sunrise at 07:15 on the 8th, 815 minutes, so a night muhūrta is 54.33
minutes; the seventh begins 6 × 54.33 = 326 minutes after sunset, at
23:06, and ends at 00:00: the page's second Dur Muhurtam, "11:06 PM to
12:00 AM, Jan 08".

## What is carried

- `Kalam`, a table of the three periods with the part each takes on each
  weekday: `Kalam::RAHU`, `Kalam::YAMAGANDA` and `Kalam::GULIKA`, with the
  identifiers `rahu-kalam`, `yamaganda` and `gulika-kalam`.
- `by_sunrise`: the period on a local day at a
  place, in Universal Time, from `hc-astro`'s sunrise and sunset. Where the
  Sun does not rise or set that day it returns `MissingSolarEvent` rather
  than a time: there is no daylight to divide.
- `by_fixed_day`: the period on a day as two readings
  of the local clock. The zone is the caller's, since the rule is stated in
  clock time.
- `KalamConvention`, the table of the two conventions, `SUNRISE` and
  `FIXED_DAY`, with the identifiers `rahu-kalam-sunrise` and
  `rahu-kalam-fixed`, the clock each span is read on, and the function.

- `muhurta::muhurta`: a muhūrta of the day or of the night, `Muhurta`
  with a `Half`, on a local day at a place, in Universal Time;
  `Muhurta::day` and `Muhurta::night` take its number, 1 to 15, and give
  `None` for any other;
  `abhijit`, the eighth of the day or `None` on a Wednesday; and
  `dur_muhurtam`, the one or two of `DUR_MUHURTAM` for the weekday. Each
  returns `MissingSolarEvent` where the Sun does not rise or set. The
  names, `ABHIJIT_NAME` and `DUR_MUHURTAM_NAME`, are the English pages',
  and `ABHIJIT_NAME_DEVANAGARI` and `DUR_MUHURTAM_NAME_DEVANAGARI` the Hindi
  edition's labels, अभिजित मुहूर्त and दुर्मुहूर्त [drik-day-panchang-2025].
  `WIKIPEDIA_NAMES` and `Muhurta::wikipedia_name` name all thirty, fifteen
  of the day from Rudra and fifteen of the night from Girīśa, as English
  Wikipedia's "Muhurta" tabulates them in transliteration
  [wikipedia-muhurta]; the article asks for citations for the table and
  names no text, so they are a secondary source's names, and its
  Devanagari column, which disagrees with its own transliteration twice,
  is not carried. It names the eighth of the day Vidhi, where Drik
  Panchang has Abhijit.

Not carried: the other methods Drik Panchang names (*Month Rahu*, *Khanda
Rahu*, *Vaar Rahu*, *Muhurta Rahu*), whose rules were not read, and the
night-time periods, which Drik Panchang says Rāhu kālam does not have
[drik-rahu-kalam]. Nor, not yet, the other periods the day pages print —
Brahma Muhurta, Pratah and Sayahna Sandhya, Vijaya, Godhuli and Nishita
Muhurta, Amrit Kalam and Varjyam — whose rules were not read. Neither
this document nor [hindu-calendars.md](hindu-calendars.md) names an
authority's margin of caution for any of these periods: Drik Panchang
prints whole minutes, and no other almanac's times were read.

## Accuracy

The muhūrtas: against the same pages of 1 to 31 January 2025, Abhijit's
start and end on every day that has one and its absence on the five
Wednesdays, the first Dur Muhurtam of every day, and the second of 2, 3,
4, 6, 7, 13 and 14 January, each within a minute of the printed minute;
Drik Panchang's example, Abhijit from 24 minutes before the middle of a
twelve-hour day to 24 after, within half a minute.

The division is exact. The times are as good as the sunrise and sunset
beneath them. Against Drik Panchang's New Delhi pages of 1 to 7 January
2025, one day of each weekday [drik-day-panchang-2025], every start of all
three periods is within 0.55 minutes of the printed minute. The fixed-day
times agree exactly with Wikipedia's Monday example (07:30 to 09:00) and
with the temple table's times for all three periods on all seven weekdays
[tirumala-kalam-table].

## Sources

- [wikipedia-muhurta]: the thirty muhūrtas' names. Read 2026-09-29;
  secondary, the table marked as needing citations.
- [wikipedia-rahukaalam]: the division into eight, Rāhu kālam's part for
  each weekday, and the fixed 06:00 to 18:00 day with its times. Read
  2026-09-27. It cites Grimes (1996) and Dalal (2010), not read.
- [drik-rahu-kalam]: the Yamardha method, and that Rāhu kālam is a daytime
  period only. Read 2026-09-27. It does not give the weekday parts.
- [drik-day-panchang-2025]: the pages for New Delhi of 1 to 7 January 2025,
  with sunrise, sunset and all three periods. The Yamaganda and Gulika
  columns of the table above are the parts these pages' times fall in.
  Read 2026-09-27. The pages of 1 to 31 January for Abhijit and Dur
  Muhurtam, and the Hindi edition's labels for them, read 2026-09-29;
  the Dur Muhurtam table is the muhūrtas the first seven days' times fall
  in.
- [drik-abhijit-muhurat]: Abhijit as the eighth of the fifteen muhūrtas
  from sunrise to sunset, the middle one, its example of a twelve-hour
  day, and that it is not taken on a Wednesday. Read 2026-09-29.
- [tirumala-kalam-table]: the fixed-day times of all three periods for each
  weekday. Read 2026-09-27.
- [drik-day-panchang-hi-2026]: the Hindi day pañcāṅga of 27 September
  2026, for the three periods' Hindi names only. Read 2026-09-28.

## Code

`crates/hc-calendars-indic/src/kalam.rs`. The tests that anchor it:
`the_periods_of_a_january_week_start_when_drik_panchang_says`,
`a_monday_rahu_kalam_is_seven_thirty_to_nine_on_the_fixed_day`,
`the_fixed_day_table_is_the_eighths_of_six_to_six`,
`the_three_never_share_a_part_and_rahu_never_takes_the_first` and
`there_is_no_kalam_where_the_sun_does_not_rise`; and
`crates/hc-calendars-indic/src/muhurta.rs`, whose tests are
`the_muhurtas_of_january_2025_are_where_drik_panchang_prints_them`,
`the_muhurtas_are_named_as_wikipedia_tabulates_them`,
`a_number_outside_one_to_fifteen_names_no_muhurta`,
`abhijit_straddles_the_middle_of_the_day`,
`every_weekday_has_a_dur_muhurtam_and_only_tuesdays_is_at_night` and
`there_is_no_muhurta_where_the_sun_does_not_rise`. No boundary export
writes the muhūrtas yet.

The WebAssembly and C export `hc_kalam` writes the three periods of a day
by either convention, from `hyper_calendar::panchanga_lines`, each named
in a locale by `hc_i18n::reckonings`: in English as Drik Panchang prints
them, and in Hindi as its Hindi day pañcāṅga labels them, राहुकाल,
यमगण्ड and गुलिक काल [drik-day-panchang-hi-2026].
