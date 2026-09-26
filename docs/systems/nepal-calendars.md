# Nepal's calendars: the Bikram Sambat as gazetted, and Nepal Sambat

Backs the identifiers `bikram-sambat` and `nepal-sambat` in
`hc-calendars-indic`. Both are built on what
[hindu-calendars.md](hindu-calendars.md) explains and this document does
not repeat: the tithi and the day it belongs to, the amānta month and how
it takes its name, the adhika and kṣaya months, the sidereal signs and the
saṅkrānti, the *Sūrya Siddhānta*'s Sun, the ayanāṃśa, and the four Indian
rules for the civil day a solar month begins on. What is Nepal's — whose
Sun, which day, which names, which year — is here.

## What it is

**Bikram Sambat.** The Vikrama era, in India a lunisolar year kept for
religious dates, is in Nepal a solar civil calendar in general use, with
Baisakh as its first month and Chait its last: the Rana government made it
the official calendar in 1901, which began as 1958 VS
[wikipedia-vikram-samvat]; the Nepal Sambat article dates the change to
1903 and has the Śaka era lingering on coins until 1912
[wikipedia-nepal-sambat]. The months are the Sun's stay in the twelve
sidereal signs, 29 to 32 days each, and the year is the Gregorian year
plus 57 from Baisakh 1 in mid-April, which is New Year's Day. What fixes
the civil day each month begins on is not a rule in a text but a
publication. Each year the Government of Nepal decides the public,
festival and substitute holidays and the office hours of the coming year
"from Baisakh 1 to the end of Chait" for every government office and
public body, and the Ministry of Home Affairs publishes the decision as a
notice in Part 5 of the *Nepal Rajpatra*, the government gazette. The
notice's first section, "वर्षभरिका शनिबार बिदा", the year's Saturday
holidays, lists every Saturday of the year by its Bikram Sambat date, and
its list of festivals names a weekday for each
[np-moha-holidays-2082, np-moha-holidays-2083]. Behind the notice stands
the official pañcāṅga: the Nepal Panchanga Nirnayak Bikash Samiti, under
the Ministry of Culture, Tourism and Civil Aviation, is "the national body
of Nepal government for decisions, amendments, approval and publication"
of religious, astrological and festival matters, and no pañcāṅga or
calendar may be published without its approval [npns-samiti]. The
Samiti's tables were not read; this library takes the months from the
holiday notices, which are the government's own dated statement of them.

**Nepal Sambat.** The lunisolar calendar of the Newar people of the
Kathmandu Valley, established on 20 October 879 in the reign of
Rāghavadeva, and the official calendar of Nepal from then until the end of
the Malla dynasty in 1769, on coins, inscriptions, decrees, chronicles and
manuscripts. It stayed in some official use under the Shahs — a treaty
with Tibet is dated Nepal Sambat 895, 1775 — and was displaced by the Śaka
era and then, in 1903, by the Bikram Sambat [wikipedia-nepal-sambat]. Its
revival is recent and stepwise: on 18 November 1999 the government
declared Sankhadhar Sakhwa, the calendar's traditional founder, a
national hero; on 25 October 2011 it decided to bring Nepal Sambat into
use as the national calendar and formed a task force on how; Lalitpur
Metropolitan City has dated its documents in it beside the Bikram Sambat
since the year 1140, mid-2020; and from 11 November 2023 it appears in
official government documents alongside the Bikram Sambat
[wikipedia-nepal-sambat]. The same article's chronology puts 2008 down as
the year Nepal became a republic (Nepal Sambat 1128) and records no
recognition of the calendar in that year. The calendar is kept mainly by
the Newars, for festivals and rituals; its year opens with Mha Puja, the
Newar rite of worship of the self, on the first day of the waxing moon
during the Swanti festival [wikipedia-nepal-sambat, wikipedia-mha-puja].

## How it works

**The gazetted months.** A notice does not say how long a month is; its
Saturdays do. Where the last Saturday of one month and the first Saturday
of the next are seven days apart, the days between them are counted and
the month's length follows; twelve lengths make the year; and the weekday
the notice gives for New Year's Day — item 2.1 (क), "नव वर्ष - वैशाख १
गते", a Tuesday in 2083 — together with the Gregorian date that weekday
falls on, puts a Gregorian date on every first of the month. The four
notices read are:

| Year | *Nepal Rajpatra* | Dated | Baisakh 1 |
| --- | --- | --- | --- |
| 2080 | Khaṇḍa 72, No. 65, Part 5 | 2079-12-02 | Friday 14 April 2023 |
| 2081 | Khaṇḍa 73, No. 54, Part 5 | 2080-10-29 | Saturday 13 April 2024 |
| 2082 | Khaṇḍa 74, No. 59, Part 5 | 2081-11-15 | Monday 14 April 2025 |
| 2083 | Khaṇḍa 75, No. 67, Part 5 | 2082-11-18 | Tuesday 14 April 2026 |

Each year ends on the day before the next notice's Baisakh 1, so four
notices fix the first day of 48 months and the length of 47: the end of
Chait 2083 waits on the 2084 notice. The one Gregorian date a notice
names, Christmas Day, "क्रिसमस डे (डिसेम्बर २५)- पुस १० गते बिहीबार" in
the 2082 notice, ties the Bikram Sambat dates to the Gregorian ones
without going through the weekday [np-moha-holidays-2080,
np-moha-holidays-2081, np-moha-holidays-2082, np-moha-holidays-2083].

**The reckoning outside them.** Outside those four years the months have
to be computed, and the question is by which Sun and which rule. The first
answer tried was the modern one, the true Sun of `hc-astro` in the Lahiri
ayanāṃśa, as the *Rashtriya Panchang* computes it, with a rule of the
Indian kind: the saṅkrānti's day, unless the saṅkrānti fell after some
hour. No hour works. The Meṣa saṅkrānti of 13 April 2024 fell at
21:21:04 Nepal time and the gazette keeps Baisakh 2081 on the 13th; the
Siṃha and Kanyā saṅkrāntis of 16 August and 16 September 2024 fell at
19:59:20 and 19:58:11 and the gazette begins Bhadau and Asoj 2081 on the
17th; the Siṃha saṅkrānti of 17 August 2023 fell at 13:49:46 and the
gazette begins Bhadau 2080 on the 18th. (These instants, and those
below, are the library's, recomputed to the second after `hc-astro`
took its ΔT from the observed table; a figure given to the minute
elsewhere is the instant rounded to the nearest minute.) That is the roadmap row's finding that "no hour-of-day
rule fits the gazette", and it rules out the Vikrami rule of
`hindu-solar-vikrami` in particular, which keeps a saṅkrānti before the
next sunrise on its day and would have begun all three of the late months
a day early. The second answer is the Sun of the *Sūrya Siddhānta*, whose
saṅkrāntis fall hours from the modern ones and which the traditional
almanacs still compute by. With it, 47 of the 48 months the notices fix
begin on the civil day, midnight to midnight, in which their saṅkrānti
falls, at whatever hour: the library's `CivilDay`. The hour matters. Ten
of the 48 saṅkrāntis fall between midnight and sunrise at Kathmandu —
Asar 2080 at 01:57:10, Bhadau 2080 at 04:48:58, Asoj 2080 at 05:16:55,
Pus 2080 at 01:07:10, Falgun 2081 at 01:41:52, Jestha 2082 at 04:17:32,
Kartik 2082 at 04:18:34, Mangsir 2082 at 01:45:23, Chait 2082 at
03:36:34 and Magh 2083 at 03:23:02 — and the gazette begins every one of
those months on the civil day of the saṅkrānti. (An earlier version of
this document counted eight, leaving out Jestha and Kartik 2082, whose
saṅkrāntis fall 57 minutes and an hour and three quarters before
sunrise; the test
`the_sankrantis_between_midnight_and_sunrise_are_gazetted_on_their_civil_day`
now holds the list.) The rule Reingold and Dershowitz give for the
Siddhānta's solar calendar, `hindu-solar-from-fixed`, begins a month on
the day at whose *next* sunrise the Sun stands in the new sign, which
they name the Orissa rule and which is the library's `SunriseDay`
[reingold2018code]; it would have begun each of those ten months a day
early. So the reckoning is the Siddhānta's Sun with the civil-day rule at
Kathmandu, and the civil day is Kathmandu's local mean time, 85.32° east
and so 5 h 41 min ahead of Universal Time, where Nepal Standard Time is
5 h 45 min ahead [wikipedia-time-in-nepal]. The saṅkrānti nearest to
midnight in the four years, Kanyā of 2083, falls at 23:54:44 Nepal time
on 17 September 2026, 23:50:44 by Kathmandu's mean time, so the four
minutes move none of them. The Siddhānta
itself is evaluated at Ujjain's meridian, as the book evaluates it, and
the module notes that any clock from five to six hours ahead of Universal
Time gives the same 48 days.

**Nepal Sambat's months.** The months are the amānta months of
`hindu-lunar`, each under its Newar name, and the year begins with
Kachhalā. Wikipedia's table of the months names each by its full moon and
the Gregorian months it falls in; it has no column of Hindu month names,
and the pairing below is read off the full moons: Kachhalā's is Kārtik
Purnimā, so Kachhalā is Kārtika [wikipedia-nepal-sambat].

| # | Month | Amānta month | Full moon | Gregorian |
| --- | --- | --- | --- | --- |
| 1 | Kachhalā कछला | Kārtika | Saki Milā Punhi, Kārtik Purnimā | Oct–Nov |
| 2 | Thinlā थिंला | Mārgaśīrṣa | Yomari Punhi, Dhānya Purnimā | Nov–Dec |
| 3 | Pwanhelā प्वँहेला | Pauṣa | Milā Punhi, Paush Purnimā | Dec–Jan |
| 4 | Silā सिला | Māgha | Si Punhi, Māghi Purnimā | Jan–Feb |
| 5 | Chilā चिला | Phālguna | Holi Punhi, Phāgu Purnimā | Feb–Mar |
| 6 | Chaulā चौला | Chaitra | Lhuti Punhi, Bālāju Purnimā | Mar–Apr |
| 7 | Bachhalā बछला | Vaiśākha | Swānyā Punhi, Baisākh Purnimā | Apr–May |
| 8 | Tachhalā तछला | Jyeṣṭha | Jyā Punhi, Gaidu Purnimā | May–Jun |
| 9 | Dilā दिला | Āṣāḍha | Dillā Punhi, Guru Purnimā | Jun–Jul |
| 10 | Gunlā गुंला | Śrāvaṇa | Gun Punhi, Janāi Purnimā | Jul–Aug |
| 11 | Yanlā ञंला | Bhādrapada | Yenyā Punhi, Bhādra Purnimā | Aug–Sep |
| 12 | Kaulā कौला | Āśvina | Katin Punhi, Kojāgrat Purnimā | Sep–Oct |

The intercalary month is Analā, whatever its place in the year, "added
every three years"; the reduced month, Nhanlā, comes "roughly in every
two decades" and leaves eleven months in the year; a year runs from 353
to 355 days, or 383 to 385 with the intercalary month
[wikipedia-nepal-sambat]. The library carries the intercalary month as
the amānta calendar's adhika month, `Month::leap(n)` before the ordinary
month of the same number, and the reduced month as its kṣaya month, the
name that does not exist that year. The tithis keep their Newar
fortnights, *thwa* the bright and *gā* the dark, as the day numbers 1–15
and 16–30.

**The year count.** Year *N* begins in the autumn of Gregorian year
*N* + 879: New Year's Day of 1134 was 4 November 2013
[wikipedia-mha-puja], and the year Lalitpur began dating in, "1140, i.e.
mid 2020", is the one that opened in the autumn of 2019
[wikipedia-nepal-sambat]. The same arithmetic puts the epoch, 20 October
879, at the opening of a year 0, which is the library's inference and not
a statement in the source. Against the Śaka years of `hindu-lunar`, whose
year turns at Chaitra, Kachhalā to Chilā of year *N* fall in Śaka
*N* + 801 and Chaulā to Kaulā in Śaka *N* + 802.

**Whose sunrise.** The Newar calendar is kept in the Kathmandu Valley,
and the registered calendar reads the day at Kathmandu's sunrise,
27°42′36″ N 85°19′12″ E at sea level, with the Lahiri ayanāṃśa. The
Samiti's almanac, which would show where Nepal's pandits read a tithi
that turns within minutes of sunrise, was not read, so the calendar is
registered as this reading and no other, and `NepalSambatCalendar::new`
takes any other place and ayanāṃśa.

**Worked example: Pus and Magh 2082 from the notice.** Section 1 of the
2082 notice lists the Saturdays of पुस as 5, 12, 19, 26 and of माघ as 3,
10, 17, 24 [np-moha-holidays-2082]. Pus 26 and Magh 3 are Saturdays a
week apart, so the days between them are Pus 27, 28, 29, 30 and Magh 1,
2: Pus has 30 days, and Magh 1, five days after a Saturday, is a
Thursday — which item 2.1 (ठ) states outright, "माघी पर्व/माघे सङ्क्रान्ति
- माघ १ गते बिहीबार". Christmas, item 2.1 (ञ), is "पुस १० गते बिहीबार",
Thursday 25 December 2025, so Pus 1 is Tuesday 16 December, Pus 30 is
Wednesday 14 January 2026 and Magh 1 is Thursday 15 January 2026. The
Siddhānta's Makara saṅkrānti falls at 21:10 Nepal time on 14 January,
which the civil-day rule makes Magh 1 — the one month of the 48 the
reckoning misses, a day early; the modern Sun's saṅkrānti, at 15:19 the
same afternoon, would miss it too under any rule that keeps an afternoon
saṅkrānti on its day. The test
`a_day_the_reckoning_puts_in_magh_is_pus_30_by_the_gazette` holds the
14th on both sides. For a month the reckoning gets right, the 2083
notice's वैशाख 5, 12, 19, 26 and जेठ 2 make Baisakh 2083 a month of 31
days, the days between the two Saturdays being Baisakh 27 to 31 and
Jeth 1 [np-moha-holidays-2083].

**Worked example: Mha Puja of Nepal Sambat 1134.** The Moon overtook the
Sun at 18:35 Nepal time on 3 November 2013; Kathmandu's sunrise on
4 November was at 06:17, and the tithi in progress then was the first, so
4 November is śukla pratipadā and 3 November, whose sunrise fell in the
thirtieth tithi, was the last day of the month before. The month that
began on 4 November holds the Vṛścika saṅkrānti of 16 November, so it is
amānta Kārtika and its Newar name is Kachhalā; the month before was
Āśvina, Kaulā. Kachhalā 1 opens the year, and 2013 − 879 gives 1134: Mha
Puja, 4 November 2013, Nepal Sambat 1134, as Wikipedia's infobox dates it
[wikipedia-mha-puja]. The instants are this library's own computation,
the conjunction being the return of the Moon's elongation to zero, as
[hindu-calendars.md](hindu-calendars.md) explains a month must begin.

## What is carried

- **`bikram-sambat`**, as `BikramSambatCalendar`: `GAZETTED`, the first
  day of each of the 48 months the four notices fix, in the Gregorian
  calendar; `RECKONING`, a `HinduSolarCalendar` with the Vikrami month
  names, the `CivilDay` rule, the era "Bikram Sambat" at Gregorian plus
  57, Kathmandu and `SolarModel::SuryaSiddhanta`; and the calendar, which
  takes a month's first day from `GAZETTED` where there is one and from
  `RECKONING` where there is not, so that 2080–2083 BS (14 April 2023 to
  13 April 2027) are gazetted and every other year is computed. The date
  is a `HinduSolarDate` of year, month 1–12 and day; the day boundary is
  midnight; there are no leap years and no leap months, and `from_fields`
  refuses one. `MONTHS_DEVANAGARI` are the gazette's spellings, वैशाख to
  चैत; `MONTHS` are the Nepali forms the Vikram Samvat article lists
  beside the Sanskrit names, Baisakh, Jeth, Asar, Saaun, Bhadau, Aasoj,
  Kattik, Mangsir, Push, Maagh, Falgun, Chait [wikipedia-vikram-samvat].
- **`nepal-sambat`**, as `NepalSambatCalendar::KATHMANDU`: a renaming of
  `HinduLunarCalendar::new(KATHMANDU, Ayanamsa::LAHIRI)`, every
  conversion going through it, so the two can never disagree about a
  tithi. The date is `NepalSambatDate` — year, month 1 for Kachhalā to 12
  for Kaulā, `leap_month` for Analā, the tithi 1–30 as the day, and
  `leap_day` for the second day to carry a tithi; `new_year` gives the
  first day of a year's first Kachhalā, the intercalary one in a year
  that has one; the day boundary is sunrise. `MONTHS_DEVANAGARI` and
  `MONTHS_NEWA` are the article's spellings in Devanagari and the
  Prachalit script. `new` takes another place and ayanāṃśa.
- **Range.** Both calendars answer over the amānta and solar engines'
  Gregorian 1700 to 2299: `bikram-sambat` from 9 April 1700 to 18 April
  2300, `nepal-sambat` from 21 March 1700 to 22 March 2300, and refuse
  outside.
- **Gazetted versus computed.** Only the 48 first days are tabulated;
  everything else, including every Bikram Sambat month outside 2080–2083
  and every Nepal Sambat date, is computed from the Siddhānta's
  arithmetic or the sky. The reckoning agrees with the gazette in 47 of
  the 48 months and misses Magh 2082, so a computed month's first or last
  day is to be read with that one miss in mind; more gazetted years would
  replace the reckoning in them, and are what the roadmap asks for.
- **Not carried, and why:**
  - *The Samiti's pañcāṅga*, the official almanac the notices rest on:
    not read, and its site was not reachable when this document was
    written. It would settle how Nepal reads a tithi at sunrise and what
    happened in Magh 2082.
  - *Lalitpur's solar Nepal Sambat*, the calendar of fixed Gregorian
    dates devised from year 1141 (2020) for administrative use, whose
    months run from 20 October. The article's table does place the leap
    day — Chaulā has 29 days in regular years and 30 in leap years,
    between Chilā ending on 17 or 18 March and Bachhalā beginning on 17
    or 18 April, which puts the extra day in the Chaulā that falls in a
    Gregorian leap year — but it cites the scheme to a calendar-maker's
    site and a blog, states the leap rule only as following "a similar
    pattern" to the Gregorian one, and names no body that keeps it
    [wikipedia-nepal-sambat]. That, and not a missing month for the leap
    day, is the module's stated reason for not carrying it; the
    municipality's own notice would be the source that changes it.
  - *The Malla-period dates* the calendar was made for: before 1700, out
    of the amānta engine's range.
  - *The Newar names of the tithis* (Pāru for pratipadā, and so on), and
    the festival rules — Mha Puja, Yomari Punhi, the Swanti days — which
    `hc-holiday`'s Nepal rules date on `hindu-lunar` and `bikram-sambat`.

## Accuracy

The Bikram Sambat's reference is the gazette itself, the four notices;
the Nepal Sambat's is the four New Year's Days that Wikipedia's Mha Puja
infobox dates, and the internal constraints the Nepal Sambat article
states. The tests assert:

| Check | Test | Result |
| --- | --- | --- |
| Every Saturday of 2080–2083 BS is one the notices list, and no other day is: 209 Saturdays over 48 months | `bikram_sambat::the_saturdays_are_the_ones_the_notices_list` | 209 of 209 |
| Baisakh 1 falls on the weekday item 2.1 (क) gives — Friday, Saturday, Monday, Tuesday — and on 14 April 2023, 13 April 2024, 14 April 2025, 14 April 2026 | `the_new_years_days_fall_on_the_weekdays_the_notices_give` | all |
| Christmas 2025, "पुस १० गते बिहीबार", is Pus 10, 2082, a Thursday | `christmas_2025_is_pus_10` | yes |
| The reckoning begins each gazetted month on the gazette's day | `the_reckoning_misses_the_gazette_once_in_forty_eight_months` | 47 of 48; Magh 2082 a day early |
| The ten Siddhānta saṅkrāntis between midnight and sunrise begin their months on the civil day, as the gazette does, and not a day earlier as `SunriseDay` would | `the_sankrantis_between_midnight_and_sunrise_are_gazetted_on_their_civil_day` | 10 of 10 |
| 14 January 2026 is Pus 30 by the calendar and Magh 1 by the reckoning alone | `a_day_the_reckoning_puts_in_magh_is_pus_30_by_the_gazette` | both |
| Every day of 2020–2030 round-trips across the gazetted years and the computed ones on either side | `every_day_round_trips_across_the_gazetted_years_and_beyond` | all |
| Months of 29 to 32 days and years of 365 or 366, 2000–2099 BS | `months_run_twenty_nine_to_thirty_two_days_and_years_365_or_366` | all |
| Mha Puja of 2013, 2014, 2016 and 2017 — 4 November, 24 October, 31 October, 20 October — is Kachhalā 1 of 1134, 1135, 1137 and 1138, and the day before is Kaulā of the year before | `nepal_sambat::new_years_day_is_mha_puja` | 4 of 4 |
| 1 July 2020 is in Nepal Sambat 1140, the year Lalitpur began dating in | `the_year_lalitpur_began_dating_in_is_1140` | yes |
| Each month's full moon falls in one of the two Gregorian months the article's table gives, 1130–1159 | `each_full_moon_falls_in_the_gregorian_months_the_table_gives` | 360 of 360 |
| Years run 353–355 or 383–385 days, 1100–1199, with at least one intercalary year | `years_run_353_to_355_days_or_383_to_385` | all |
| Every day of 1138–1140 round-trips; the intercalary months are Tachhalā 1138 and Kaulā 1140 | `every_day_of_three_years_converts_and_converts_back` | all |
| The tithi, its repetition and the intercalary flag of every day of 2024–2026 are the amānta calendar's at the same sunrise | `a_date_keeps_its_tithi_from_the_amanta_calendar_at_the_same_sunrise` | all |

**Known disagreements**, stated as the module documentation states them:

- *Magh 2082.* The Siddhānta's Makara saṅkrānti falls at 21:10 Nepal
  time on 14 January 2026 and the gazette begins Magh on the 15th — twice
  over, since it also dates Maghe Sankranti to Magh 1, a Thursday. No
  published explanation was found, one month is not enough to infer a
  rule from, and the reckoning does not try; the calendar follows the
  gazette because the gazetted day is there.
- *The modern Sun.* No rule of the hour-of-day kind reproduces the four
  years with the true Sun and the Lahiri ayanāṃśa, as the cases above
  show; the library does not register a modern-Sun Bikram Sambat.
- *A tithi at sunrise.* The Nepal Sambat is read at Kathmandu's sunrise
  and the Samiti's almanac was not read, so on a day when a tithi turns
  within minutes of sunrise the almanac may put the day in the other
  tithi. The four Mha Puja dates all fall with the conjunction hours
  clear of sunrise.
- *The month names' pairing.* Kachhalā is Kārtika because its full moon
  is Kārtik Purnimā, and so on down the table; the article gives the full
  moons, not the Hindu month names, and the pairing is the module's
  reading of them.

On 2026-09-25 the sources were re-read as far as they could be. The
Ministry's holiday listing names the four notices and their pages; the
2082 and 2083 notices were opened from those pages as scanned PDFs, and
their Saturday lists, New Year weekdays, Christmas and Maghe Sankranti
items agree with the tests' tables item for item; the 2080 and 2081 pages
were reached and their PDFs not opened, so those two years rest on the
module author's reading of 2026-09-23. The Samiti's site at npns.gov.np
refused the connection, and what this document says of it is from the
site at nepalpanchanga.com that carries its name. The Wikipedia articles
were re-read in their wikitext, Reingold and Dershowitz's code directly.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [np-moha-holidays] | The Ministry's listing of the four notices and the pages that carry them | Yes, 2026-09-25 |
| [np-moha-holidays-2080] | The Saturdays, New Year weekday and holidays of 2080 BS; *Rajpatra* Khaṇḍa 72, No. 65 | Page reached 2026-09-25, PDF not opened; the module's reading of 2026-09-23 |
| [np-moha-holidays-2081] | The same for 2081 BS; Khaṇḍa 73, No. 54 | Page reached 2026-09-25, PDF not opened; the module's reading of 2026-09-23 |
| [np-moha-holidays-2082] | The same for 2082 BS; Khaṇḍa 74, No. 59; Christmas as Pus 10 and Maghe Sankranti as Magh 1, Thursday; the worked example | Yes, 2026-09-25, pp. 3–5 of the scanned PDF |
| [np-moha-holidays-2083] | The same for 2083 BS; Khaṇḍa 75, No. 67; New Year on a Tuesday | Yes, 2026-09-25, pp. 1–2 of the scanned PDF |
| [npns-samiti] | The Samiti, its ministry, and its approval of every published calendar | The official site refused the connection on 2026-09-25; the site at nepalpanchanga.com read instead |
| [wikipedia-vikram-samvat] | The Rana adoption in 1901 as 1958 VS; the Nepali month names beside the Sanskrit ones; Baisakh first and Chait last | Yes, 2026-09-25 |
| [wikipedia-nepal-sambat] | The epoch, the Malla use, the 1903 replacement, the revival of 1999, 2011, 2020 and 2023; the table of months and full moons; Analā and Nhanlā; the year lengths; Lalitpur's solar calendar | Yes, 2026-09-25 |
| [wikipedia-mha-puja] | Mha Puja as Nepal Sambat's New Year's Day during Swanti; the dates of 2013, 2014, 2016 and 2017 | Yes, 2026-09-25; the four dates are the infobox's |
| [wikipedia-time-in-nepal] | Nepal Standard Time as UTC+05:45 | Yes, 2026-09-25 |
| [wikipedia-kathmandu] | Kathmandu's coordinates, as `places.rs` cites them | Not re-read; the module's reading of 2026-09-23 |
| [reingold2018code] | `hindu-solar-from-fixed`, the next-sunrise rule the book applies to the Siddhānta's solar calendar, and `ujjain` | Yes, 2026-09-25 |
| [reingold2018] | The Siddhānta's Sun, as [hindu-calendars.md](hindu-calendars.md) sets it out | Not read directly; the published code was |

Statements in the module documentation for which this document names no
source: that the epoch of 20 October 879 opens year 0, which is
arithmetic from the 1134 New Year and not a statement read anywhere; the
pairing of each Newar month with an amānta month, read off the full-moon
names; that Kathmandu's local mean time is 5 h 41 min ahead of Universal
Time, which is its longitude; and that the Siddhānta's saṅkrāntis are the
same 48 days under any clock from five to six hours ahead of Universal
Time, which is the module's own check. The Vikrami-rule cases and the
ten small-hours saṅkrāntis above are this library's computations, not
published ones.

## Code

`crates/hc-calendars-indic/src/bikram_sambat.rs` (`BikramSambatCalendar`,
`RECKONING`, `GAZETTED`, `MONTHS`, `MONTHS_DEVANAGARI`) and
`nepal_sambat.rs` (`NepalSambatCalendar`, `NepalSambatDate`,
`MONTHS_DEVANAGARI`, `MONTHS_NEWA`); the `CivilDay` rule is
`hindu_solar.rs`'s, the Sun `surya_siddhanta.rs`'s, Kathmandu
`places.rs`'s. Anchors: `the_saturdays_are_the_ones_the_notices_list`,
`the_new_years_days_fall_on_the_weekdays_the_notices_give`,
`christmas_2025_is_pus_10`,
`the_reckoning_misses_the_gazette_once_in_forty_eight_months`,
`the_sankrantis_between_midnight_and_sunrise_are_gazetted_on_their_civil_day`,
`a_day_the_reckoning_puts_in_magh_is_pus_30_by_the_gazette`,
`new_years_day_is_mha_puja`, `the_year_lalitpur_began_dating_in_is_1140`,
`each_full_moon_falls_in_the_gregorian_months_the_table_gives`,
`every_day_of_three_years_converts_and_converts_back`. The holiday rules
that date Nepal's festivals on these calendars are
`crates/hc-holiday/src/countries/asia.rs`, `NEPAL`.
