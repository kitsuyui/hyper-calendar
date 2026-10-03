# The Tibetan almanac: the five components and the columns after them

Backs `hc-calendars-regional`'s `tibetan_almanac` module, which computes
what a Tibetan almanac prints beside the date, over the true dates
`hc-calendars-lunar`'s `tibetan` computes and makes public
([tibetan-variants.md](tibetan-variants.md)). The
arithmetic of the date itself is in [tibetan-phugpa.md](tibetan-phugpa.md)
and is not repeated here.

## What it is

**The five components.** The Tibetan calendar is called "the five
components" (*lnga-bsdus*), as the Indian *pañcāṅga* is: the day of the
week, the lunar day, the lunar mansion, the *yoga* and the *karaṇa*
[janson2014, §10]. An almanac prints them for every calendar day, with the
numbers they come from: the true weekday, the Moon's longitude at
daybreak, the true Sun, the *yoga* "longitude" and the mean Sun. It prints
the planets on the days of full and new Moon, and names the years, months
and days in the Chinese-style cycles of element and animal. The
description is Janson's, from Henning's book and the Men-Tsee-Khang
almanac of 2013, with "certainly minor variations between different
almanacs" [janson2014, §10].

**Henning's computed almanacs.** Edward Henning publishes a computed
almanac for every year of the Phugpa, the Tsurphu and the Bhutanese
calendars, six hundred years of each, with the programs that make them:
for each day the weekday, the mansion, the *yoga*, the *karaṇa*, the four
numbers after them and the Sun in signs [kalacakra-org-archive;
kalacakra-org-software]. They are the almanac this module is checked
against, and they choose among the conventions below: his Phugpa and
Bhutanese almanacs take Minling Lochen's exact anomaly increment, his
Tsurphu almanacs the *karaṇa* Sun as well
([tibetan-variants.md](tibetan-variants.md)).

**Other things the almanac names.** The years have a second name, in the
Indian cycle of sixty names beginning with Prabhava (*rab byung*), and a
second count, from the ascent of the first Tibetan king in 127 BCE
[janson2014, §4]. The Bhutanese almanac names its weekdays one ahead of the
world's and dates its winter solstice by the mean Sun [janson2014,
Appendix A.4]. The Mongolian names its months by season and its years and
months by colour and animal [janson2014, Appendix A.3]. And a festival fixed
to a Tibetan date has to go somewhere when the date is skipped or
repeated [janson2014, §11].

## How it works

The quantities are Janson's, for lunar day *d* of true month *n*, all
rational. Longitudes are in lunar mansions, 27 to the circle.

**The day's columns** [janson2014, §10, items (i)–(ix)]:

| Column | Rule |
| --- | --- |
| true weekday | (true_date + 2) mod 7, the end of the lunar day |
| Moon at the end of the lunar day | true_sun + *d*/30 of a circle |
| Moon at daybreak | that, less frac(true_date) mansions: the Moon taken to move one mansion a day |
| lunar mansion | the Moon at daybreak's whole part |
| true Sun | Janson's `true_sun`, for the end of the lunar day, "regarded as valid also for the calendar day" |
| *yoga* longitude | the Moon at daybreak and the true Sun, added, mod 27 |
| *yoga* | its whole part |
| *karaṇa* | from the half-day of the month at daybreak |
| mean Sun | in signs, degrees and minutes |

**The *karaṇa*.** Each lunar day has two halves and each half one of eleven
*karaṇas*: half-days 1, 58, 59 and 60 the four fixed ones, the other 56 the
seven changing ones in turn, (*H* − 1) amod 7. Janson is not sure where the
halves divide, adding that "Henning divides each lunar day into two halves
of equal lengths" [janson2014, §10, item (viii)]. Henning's almanacs bear
out halves equal in elongation: the half-day at daybreak is ⌊60 × (the Moon
at daybreak − the true Sun)⌋ + 1, in revolutions. Halving the lunar day at its
midpoint in time gives another *karaṇa* than his on 135 of the 1 448
Phugpa days read, and the elongation on none.

**The first of two days with one number.** No lunar day ends in it, so the
almanac prints the end of the calendar day, `x;60,0`, for the weekday
[janson2014, §10]. For the Moon, Janson reads the Men-Tsee-Khang almanac as
the Moon at the end of the lunar day less one mansion, and Henning's Phugpa
and Bhutanese almanacs agree; his Tsurphu almanacs instead take the Moon at
the end of the lunar day before and move it on one mansion a day to
daybreak, which matches every such day of 2012–2014 in them. No text read
states that second rule; it is this library's reading of the pages, and
`ExtraDayMoon` names it.

**The planets** [janson2014, Appendix D]. The general day is JD −
2 424 972, days from Henning's epoch of 1 April 1927. A planet's particular
day is (general day × 100, × 10 or × 1 for Mercury, Venus and the outer
three, plus an epoch value) mod *R*, *R* = 8 797, 2 247, 687, 4 332 and
10 766 days, and the mean heliocentric longitude is that over *R*. The
mean solar longitude is 18 382⁄6 714 405 of a circle a day from
1 − 458 772⁄6 714 405. For Mercury and Venus the Sun is the slow
longitude and the planet the step index, for the others the other way
round; the slow longitude less its birth-sign gives an equation from a
four-value table, and the true slow longitude against the step index a
final correction from a fourteen-value one. Rāhu's head is at −*x*⁄6 900 of
a circle, *x* = 30 (*n* + 187) + *d* with *n* counted from 1927.

**The years' names.** Year *Y* is (*Y* − 6) amod 60 in the Prabhava cycle,
whose sixty names in Tibetan and Sanskrit are Janson's table, from Henning;
the count from 127 BCE is *Y* + 127 [janson2014, §4 and Appendix B].

**The cycles of element and animal** [janson2014, Appendix E]. A day's
element is ⌈JD/2⌉ amod 5 in the order Wood, Fire, Earth, Iron, Water, its
gender male when JD is odd, its animal (JD + 2) amod 12 from the Mouse. A
month's animal and element follow one of two rules: the Phugpa's, whose
Tiger is month 11 of the year before and whose Tiger month takes the
element after the year's, and the Tsurphu's, month 1 the Tiger and the
elements running on from month to month and year to year, "(Y − 2 +
⌊(M − 1)/2⌋) amod 5", which is the Chinese rule and the Mongolian one.

**The astrological attributes** [janson2014, Appendix E]. The
Chinese-style system gives a lunar day an animal, an element, one of the
eight trigrams (*spar kha*) and one of the nine numbers (*sme ba*): the
animal is (*D* + 6*M* + 8) amod 12 from the Mouse, so that an odd month
begins with the Tiger and an even one with the Monkey (E.9); the element
is the month's advanced by the day, "(x + D) amod 5"; the trigram is
(*D* + 6*A* + 6) amod 8 and the number (*D* + 3*A*) amod 9, *A* the
month's animal from the Mouse as 1 (E.10, E.11), so that a Tiger month
begins with *li* and with 1, white. A leap month has its regular month's.
The calendar day has its trigram, (JD + 2) amod 8, and its number,
(−JD) amod 9, one less each day, which Janson reports Henning's book
computing as "10 − ((JD + 1) amod 9)" (Remark 36). In the Indian system
the weekday and the lunar mansion each have one of four elements, earth,
fire, water and wind, and the day has the pair, whose ten combinations
Henning's book names ("see [7, p. 204]"). Henning's almanacs print, after
the mansion, the two elements, weekday first; on the second line, after
the *karaṇa*, the lunar day's animal, trigram and number; and after the
solar day's element and animal, one of the twenty-eight Chinese
mansions and, in the Phugpa and Bhutanese, a number. No text read states
those last two: the mansions run in a cycle of twenty-eight days, *Jiao*
where JD is 17 mod 28, the same count as the Japanese almanac's
二十八宿, and the number runs one more each day, (JD − 1) amod 9, the
other way from Janson's rule, and turns at neither solstice in the years
read. That is two conventions of one attribute, and each is a function
(docs/policy.md §5): `janson_day_number` and `henning_almanac_day_number`.

**Mongolia** [janson2014, Appendix A.3]. The months are named as the
beginning, middle and end of the four seasons from the first spring
month, and "often the element is replaced by the corresponding colour"
in the names of years, months and days: green or blue for Wood, red,
yellow, white, and dark blue or black for Water, Mongolians using the
colour in parentheses.

**Bhutan** [janson2014, Appendix A.4; kalacakra-org, "The Bhutanese
calendar" and "Bhutan calendars"]. "Its day of week differs by one day":
the Bhutanese almanac names a day the one after the world's, (JD + 3) mod
7 in Janson's numbering from Saturday. The winter solstice holiday is
"the day the mean solar longitude reaches 250°", which Henning gives as
18;45 of the mean Sun rather than the Phugpa's 18;31,30. The instant is
Janson's rule for the special days of the almanac: the lunar-day count *L*
at which the mean Sun is *k* + 25⁄36 of a circle is (*k* + 25⁄36 − *s*₀)⁄*s*₂,
and its mean date *L m*₂ + *m*₀ [janson2014, §10]; the day is the calendar
day that mean date falls in.

**Festivals on skipped and repeated dates.** Janson gives Berzin's rule:
"If a holiday is fixed to a given date, and that date is skipped, the
holiday is on the preceding day. If the date appears twice, the holiday is
on the first of these", adding "I have not checked them against published
calendars" [janson2014, §11]. Henning's almanacs do otherwise: a festival
on a repeated date is marked on the second of the two days, one on a
skipped date is not marked at all, and one in a doubled month is marked in
both months, as his almanac for 2024 marks the Turning of the Wheel of the
Dharma on 10 July, the 4th of the leap month 6, and on 8 August, the 4th
of month 6. The two are `berzin_day` and
`henning_almanac_day`, two functions for two conventions (docs/policy.md
§5).

**Worked example: 11 February 2013, Losar of the Water Snake year**, in
Henning's Phugpa almanac, whose first line is "1: Mon. mon gre.
Water-Water; 11 Feb 2013 / phan tshun, gdab pa, Tiger, kham 7 / 2;6,31
22;14,22 21;26,54 16;41,17 9;15,17".

1. *The date.* Month 1 of 2013 has true month count *n* = 14 928 from 806,
   and lunar day 1 ends at true date 2 456 335.108 67 under the exact
   anomaly increment. JD 2 456 335 is 11 February; (2 456 335 + 2) mod 7 =
   2 is Monday, and 0.108 67 of a day is 6 *nāḍī* 31 *pala*: 2;6,31.
2. *The Sun and the Moon.* The true Sun is 21;26,54 mansions. The Moon at
   the end of lunar day 1 is a thirtieth of the circle on, 0;54 mansions:
   22;20,54. At daybreak, 0.108 67 of a day earlier, it is 0;6,31 of a
   mansion back, 22;14,22: mansion 22 from Aśvinī, Dhaniṣṭhā, *mon gre*.
3. *The yoga.* 22;14,22 + 21;26,54 = 43;41,16, less 27: 16;41, *yoga* 16,
   Vyatīpāta, *phan tshun*. The exact sum's last place is 17, as printed.
4. *The karaṇa.* The elongation at daybreak is 22;14,22 − 21;26,54 =
   0;47,28 mansions, 0.029 3 of a circle; sixty times that is 1.76, so the
   half-day is 2, the first changing *karaṇa*, Vava, *gdab pa*.
5. *The mean Sun* at the end of the lunar day is 0.792 50 of a circle, 9
   signs 15° 17′: 9;15,17.
6. *The attributes.* Monday is the Moon's day, water, and Dhaniṣṭhā a
   water mansion: "Water-Water". Month 1 of the Phugpa is the Dragon,
   *A* = 5, so lunar day 1 is (1 + 6 + 8) amod 12 = 3, the Tiger;
   (1 + 30 + 6) amod 8 = 5, *kham*; and (1 + 15) amod 9 = 7: "Tiger, kham
   7". JD 2 456 335 is 7 mod 28, the eighth mansion after *Jiao*, *Bi*, and
   (2 456 335 − 1) amod 9 = 9: "Bi 9". Janson's rule gives the day 8.

**Worked example: Mars on 6 January 2011**, Henning's "MARS sgos zhag =
525 - 20;12,3,2,100 - myur: 19;37,33,2,26" [kalacakra-org-software].

1. *The particular day.* JD 2 455 568 − 2 424 972 = 30 596; (30 596 +
   157) mod 687 = 525. The mean heliocentric longitude is 525⁄687 of a
   circle, 20;37,59 mansions.
2. *The equation.* The anomaly is 0.764 19 − 19⁄54 = 0.412 34; twelve
   times it is 4.948, which the table's symmetry takes to 1.052, where it
   reads 25 + 0.052 × 18 = 25.94. The true slow longitude is 20;37,59 −
   25.94⁄60 = 20;12,3.
3. *The correction.* The mean Sun is 0.694 22 of a circle; the step index
   less the true slow longitude is 0.946 03, 25.543 in mansions, where the
   final table reads −47 + 0.543 × 23 = −34.51. The fast longitude is
   20;12,3 − 34.51⁄60 = 19;37,32, a *pala* short of Henning's 19;37,33.

## What is carried

- **The columns of a day**: `almanac_day` for any `TibetanCalendar` — the
  Tibetan date and its true month; the weekday; the true weekday, `None` on
  the first of two days; the Moon at daybreak, the true Sun, the *yoga*
  longitude and the mean Sun, as exact rationals that print as the
  almanac does, `2;6,31`; and the mansion, *yoga*, *karaṇa* and half-day.
  The Tsurphu's Sun is the *karaṇa* one, `printed_mean_sun`, and its
  extra-day Moon the one of `extra_day_moon`. `lunar_day` gives the
  same quantities for a lunar day, a skipped one among them, for which the
  almanac prints the true weekday and the Sun.
- **Names**: `MANSIONS`, `YOGAS` and `KARANAS` in Sanskrit and Tibetan as
  Henning's Tsurphu almanacs print them, "Shatabhishaj/mon gru", in his
  ASCII transliteration and Wylie; his Phugpa almanacs print the Tibetan
  only, and *vishti* for *viSTi*. The twenty-seventh mansion's place is
  Abhijit's, *gro zhin*, as he prints it, where Indian lists have Śravaṇa.
  `WEEKDAYS` in English and Tibetan from Janson's table, *spen pa* for
  Saturday as his Appendix A.4 spells it.
- **The planets**: `planet_place` for the five planets, with the
  particular day, the mean heliocentric, true slow and fast longitudes,
  under Henning's Phugpa epoch of 1927; `rahu_head` for the Phugpa.
- **Years**: `RAB_BYUNG_NAMES` and `rab_byung_name`, the sixty names in
  Tibetan and Sanskrit as Janson's table prints them — *pramadi* stands for
  the 4th, 13th and 47th years there, and is kept; and `royal_year`, the
  count from 127 BCE.
- **Element, colour and animal**: `year_symbol`, `month_symbol` under
  `MonthCycle::Phugpa` or `MonthCycle::Tsurphu`, `day_symbol`; the
  year's and the day's are defined once, in `hc-calendars-lunar`'s
  `tibetan` module beside `year_name`, `prabhava` and `weekday`, and this
  module re-exports them with `Symbol`; `COLOURS`,
  the Mongolian use; `MONGOLIAN_COLOURS`, the words with their female
  forms as Gantumur's calendar writes them [gantumur-mongolian-calendar].
- **Mongolian months**: `mongolian_month`, the season and the month's
  place in it; `MONGOLIAN_MONTH_NAMES`, "Хаврын тэргүүн" to "Өвлийн сүүл",
  and `MONGOLIAN_LEAP_WORD`, илүү, as Gantumur's calendar writes them,
  "Зуны эхэн илүү сар" for a leap first summer month. That calendar is a
  program over Janson's arithmetic, a secondary source. `hc-i18n`'s `mn`
  entry writes `mongolian`'s months and dates with the same words
  (`data::MN_MONGOLIAN`): the month on its own as the calendar heads it,
  *Өвлийн сүүл илүү сар*, and a date in `mn.xml`'s Gregorian long date,
  *2026 оны хаврын тэргүүн сарын 1* for Tsagaan Sar, 18 February 2026, and
  *2024 оны өвлийн сүүл илүү сарын 1* in the leap twelfth month of 2024
  ([written-dates.md](written-dates.md)). No source read writes a whole
  lunar date, so its shape is the library's. Search results show Mongolian
  broadcasters writing "зуны эхэн илүү сарын шинийн 15" for a leap month's
  15th, on pages that could not be read; the *шинийн*, "of the new", that
  such a date puts before the day is not written.
- **Bhutan**: `bhutanese_weekday`; `bhutanese_winter_solstice`, the
  instant and so the day.
- **The attributes of a day** [janson2014, Appendix E]:
  `lunar_day_attributes`, the lunar day's animal, element, trigram and
  number under `MonthCycle::Phugpa` or `MonthCycle::Tsurphu`, `None` for a
  month or day out of range; `day_trigram`; `janson_day_number` and
  `henning_almanac_day_number`, the two counts of the calendar day's
  number; `chinese_mansion` and `CHINESE_MANSIONS`, in Henning's spelling,
  three *Wei* and two *Bi* among them as he prints them; `element_pair`,
  the weekday's and the mansion's elements, with `INDIAN_ELEMENTS`,
  `WEEKDAY_ELEMENTS` from Janson's table of the weekdays and
  `MANSION_ELEMENTS` as Henning's almanacs print them, Janson referring to
  Henning's book for the list; `TRIGRAMS` and `NINE_NUMBERS` with Janson's
  attributes of each. `hc_tibetan_almanac_day` writes them as lines, one
  kind each: `lunar-day-animal`, `lunar-day-element`, `lunar-day-trigram`
  and `lunar-day-number` for the lunar day that ends on the calendar day,
  none on the first of two days with one number, as Henning prints them;
  `day-trigram`, `day-number-janson` and `day-number-henning` for the
  calendar day, the two numbers as separate kinds (docs/policy.md §5) and
  Henning's not on the Tsurphu or Mongolian versions, whose almanacs print
  none; `chinese-mansion`; and `element-pair`, read at the weekday the
  almanac names the day by, the Bhutanese one on the Bhutanese versions.
  The Bhutanese versions' lunar day is read under the Phugpa's month
  cycle, with which Henning's Bhutanese almanacs of 2000 to 2020 agree, and
  has no element, since Janson gives no month rule for Bhutan. The lines'
  columns are in the WebAssembly module's README.
- **Festivals**: `HENNING_FESTIVALS`, the seven fixed-date festivals
  Henning's computed Phugpa almanacs mark in every year of 1960–2045 as
  read, with his English words; `berzin_day` and `henning_almanac_day`.
  Janson refers to the list of holidays in Henning's book, Appendix II,
  which was not read: whether it holds more is not known here, and the
  seven are what is carried. They are data in this crate. `hc-holiday`
  uses both rules: its `buddhist-tibetan-berzin` table dates a skipped or
  repeated festival by `berzin_day`, and `buddhist-tibetan-henning` dates
  the festivals as Henning's almanacs mark them, both months of a doubled
  one among them, by `henning_almanac_day`; `buddhist-tibetan`, from the
  Tibetan Nuns Project, reports a skipped or repeated date as a gap.
- **Not yet carried**: the planets under the Tsurphu, Bhutanese and
  Mongolian epochs, whose epoch values Henning's "Epoch data" gives but
  whose mean solar longitude Janson describes only for the Phugpa, citing
  Henning's pp. 341, not read; the *karaṇa* Moon an almanac prints beside
  the *siddhānta* one; the attributes of years and months that Janson
  gives with their tables — the power, life, body, fortune and spirit
  elements (E.2, Tables 18–23), the central, life and power numbers
  (E.3–E.5) and the Tsurphu month's number (E.8) — which no almanac read
  prints, Table 23 being the anchor they would be tested against; the
  Tsurphu calendars' count of the day's number from the solstices
  (Janson's Remark 37), which no page read prints; the names of the ten
  element pairs, in Henning's book, not read; the *svarodaya* emblems and
  the earth-lords, for which no source read gives a rule, Janson's
  Appendix E naming neither and Henning's book being unread; and the
  marks the Tsurphu almanacs print under some days, *zin phung*, *klu
  bzlog*, *klu thebs*, *yan kwong*, *nyi nag* and *ngan pa dgu 'dzom*, for
  which no source read gives a rule. The Bhutanese holidays as a table are
  `hc-holiday`'s `BHUTAN`.

## Accuracy

**Measured on 2026-09-29 against Henning's almanacs.** A program read
every day of Henning's computed Phugpa almanacs for the years beginning in
2012, 2013, 2014 and 2025 (1 448 days), his Tsurphu almanacs for 2012–2014
(1 093) and his Bhutanese almanac for 2019 (384), and compared each column
with `almanac_day` under `tibetan-lochen`, `tibetan-tsurphu-karana` and
`tibetan-bhutan-lochen`:

| | Phugpa | Tsurphu | Bhutanese |
| --- | ---: | ---: | ---: |
| Days | 1 448 | 1 093 | 384 |
| Date, extra day, mansion, *yoga*, *karaṇa* as printed | 1 448 | 1 093 | 384 |
| Every number to the printed *pala* | 1 432 | 904 | 382 |
| The rest within one *pala* | 16 | 189 | 2 |

The *pala* differences are in the last printed place of the Moon, the
*yoga* longitude and, in the Tsurphu, the true weekday. Henning's
programs keep the almanacs' mixed radices, whose last places for time
(707) and for longitude (67) differ, and Janson notes that the difference
"is usually ignored"; truncating the true date to a 67th of the last
place removes six of the Phugpa's sixteen and both Bhutanese ones, and
this module keeps the exact values rather than guess the rest. The test
`the_columns_are_those_of_hennings_almanacs` holds months 1 and 2 of the
Phugpa 2013, months 1 and 2 of the Tsurphu 2012 and month 1 of the
Bhutanese 2019 to within one *pala* and every name exactly;
`the_tsurphu_programs_worked_day_is_reproduced` holds the day Henning's
Tsurphu page works through. Under the almanacs' 1⁄28 instead, 1 112 of the
Phugpa days differ in some printed place, by up to a day, and 19 November
2025 in date.

**The attributes.** A program read every day of Henning's computed Phugpa
almanacs for 1960–2045 (31 391 days), his Bhutanese almanacs for 2000–2020
(7 677) and his Tsurphu almanacs for 2012–2014 (1 093) on 2026-09-29, and
compared the two elements, the Chinese mansion, the number after it and
the lunar day's animal, trigram and number — 40 783 lunar days, the
skipped ones' "Omitted" lines among them — with `element_pair`,
`chinese_mansion`, `henning_almanac_day_number` and
`lunar_day_attributes`, under `MonthCycle::Phugpa` for the Phugpa and
Bhutanese and `MonthCycle::Tsurphu` for the Tsurphu. Every one agreed; the
number after the mansion rose by one on every day of the Phugpa years,
through every solstice. `the_attributes_are_those_hennings_almanacs_print`
holds month 1 of the Phugpa 2013, of the Tsurphu 2013 and of the Bhutanese
2019, an omitted day and the doubled month 6 of 2024;
`the_lunar_day_attributes_follow_jansons_rules` Janson's statements of the
months' first trigrams and numbers; `the_calendar_days_numbers_run_both_ways`
Janson's rule against his report of Henning's book; and
`crates/hyper-calendar/tests/mansion_cycles.rs` the Chinese mansion
against `hc-almanac`'s 二十八宿 on every day of 1000–3000. The lunar day's
element, the calendar day's trigram and Janson's number are printed by no
almanac read and rest on his rules.

**The planets.** Henning's Mars of 6 January 2011 is reproduced to the
printed *pala* in the particular day and the true slow longitude, and a
*pala* short in the fast longitude (`marss_place_is_hennings_worked_one`).
No published place of the other four planets under the Phugpa epoch was
read, so they rest on the same arithmetic and tables.

**Far back.** Henning's search example finds Norzang Gyatso's data for the
Buddha's enlightenment, "Weekday: 1;38, Moon: 16;0, Sun: 2;30, Rāhu:
16;29", on Sunday 17 March 927 BCE: the arithmetic gives the 15th of month
4 there, 1;38,39, 15;59,53, 2;29,53 and Rāhu 16;29,36
(`the_enlightenment_day_of_hennings_search_is_reproduced`).

**Names and cycles.** The constitution of Mongolia came into force "from
the horse hour of the auspicious yellow horse day of the black tiger first
spring month of the water monkey year of the seventeenth 60-year cycle", 12
February 1992 [janson2014, Appendix A.3, citing Sanders and Bat-Iredüi,
not read]: the day, month and year have those colours, animals and
elements (`the_constitutions_day_month_and_year_have_their_colours`), as
Janson's "white tiger" first month of 1996 does. Henning's month headings,
"1 - Fire-male-Dragon" in the Phugpa 2013 and "1 - Wood-male-Tiger" in the
Tsurphu, and his solar days, "Earth-Monkey" for 11 February 2013, follow
the rules (`the_months_and_days_are_named_as_hennings_almanacs_name_them`).
The *rab byung* names of 1927, 1987, 2007, 2024 and 2046 are Janson's
table's, and the count from 127 BCE gives the 2130 of the Tibetan title of
the calendar for 2003 Janson cites, the Tibetan Nuns Project's 2151 for
the Wood Dragon year [tnp-losar] and the Central Tibetan Administration's
2152 [tibet-net-losar-2152] (`the_years_are_named_and_counted_as_janson_gives_them`).

**The Bhutanese weekday**: Henning's "5th May 2008. It is a Monday … in
Bhutan, it is a Tuesday", and his Bhutanese almanac's "1: Wed. …; 5 Feb
2019", a Tuesday (`the_bhutanese_weekday_is_one_ahead`). The Ministry of
Home Affairs' calendars print the Bhutanese day number under the world's
weekday columns and head the Sunday column *zla ba* [moha-bt-calendar-2026],
as read on 2026-09-26 for [tibetan-variants.md](tibetan-variants.md).

**The Bhutanese solstice** (`the_bhutanese_winter_solstice_is_the_mean_suns_250_degrees`).
Henning's Bhutanese almanacs print "Winter solstice, time: …" with the
weekday and time: 1 January 2001 at 2;51,38, 2 January 2002 at 4;7,52,
2 January 2011 at 1;34,1, 2 January 2017, 2018 and 2019 at 2;11,27, 3;27,41
and 4;43,56, and 3 January 2020 at 6;0,10, all reproduced to the *pala*;
2020 is the first 3 January, as Janson says. The Ministry's lists put the
holiday on 2 January in 2025 and 2026 [moha-bt-calendar-2025;
moha-bt-calendar-2026], as the rule does. Two of Henning's entries
disagree, with the rule and with his own columns: 2 January 2016 is printed
at 0;35,16 where the rule gives 0;55,13, and his mean Sun at the end of
that day's lunar day, 0;37 after daybreak, is 8;9,46, 249° 46′, not yet
250°; and the almanac for 2020 has the solstice on 1 January 2021 at
6;43,26, where its own mean Sun that day is 8;8,25, 248° 25′, and the rule
gives 2 January 2021 at 0;16,24. Both are recorded here, not carried.

**The drift of the Bhutanese solstice.** The mean Sun's year is the
calendar's, *m*₁ / *s*₁ = (167 025 / 5 656) / (65 / 804) = 6 714 405 /
18 382 days, 365.270 645, where the Gregorian calendar's is 365.2425: the
solstice comes 0.028 145 days later each Gregorian year, a day every 35½
years, 2.8 days a century. *Worked example*: Janson's first 3 January is
2020; 319 years × 0.028 145 = 8.98 days earlier, the solstice of the
winter 1700 ends in falls on 24 December 1700, as the rule gives it, and
it is 30 December in 1900, 4 January in 2100 and 30 January in 3000
(`the_bhutanese_winter_solstice_is_the_mean_suns_250_degrees`). So the
December date of a year before the 1920s is no error: it is the same
reckoning's solstice, drifted, of the winter that year ends in. The
solstice of a Gregorian year is the one that falls in it, and while the
day crosses 1 January, between 1923 and 1957, a year of 365 days between
two solstices 365.27 days apart holds none: 1923, 1927, 1931, 1935, 1938,
1942, 1946, 1949, 1953 and 1957, which `hc_bhutanese_winter_solstice`
refuses with `HC_ERR_NO_DATA` and in which the Bhutan table predicts no
Winter Solstice holiday. No source read says how the almanac labels the
solstice of such a winter.

**Festivals** (`a_festival_on_a_skipped_or_repeated_date_follows_each_rule`).
In 2024 the 4th of the leap month 6 is repeated, on 9 and 10 July: the
Tibetan Nuns Project kept Chökhor Düchen on 9 July [tnp-losar], as Berzin's
rule does, and Henning's almanac marks the Turning of the Wheel on 10 July.
In 1990 the 7th of month 4 is skipped: Berzin's rule keeps the Birth of the
Buddha on 30 May, the 6th, and Henning's almanac marks no day, as in 1966
and 1975. In 1961 and 1988 his almanac marks a repeated date's festival on
the second day. The Mongolian Government's resolution of 2025, whose first
day of the first month was skipped, kept the holiday on the 2nd and 3rd
only [mn-resolution-2025-109], which is neither rule
([tibetan-calendar-holidays.md](tibetan-calendar-holidays.md)).

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [janson2014] | The five components and the columns, §10; the holiday rule, §11; the years, §4 and Appendix B; Mongolia, Appendix A.3; Bhutan, Appendix A.4; the planets, Appendix D; the cycles and the attributes, Appendix E; the weekdays' elements, §9 | Yes, 2026-09-29, from the TeX source; §9's table and Appendix E again the same day in ar5iv's HTML rendering |
| [kalacakra-org-archive] | Henning's computed Phugpa, Tsurphu and Bhutanese almanacs, `tdata/pl_*.txt`, `ts_*.txt`, `bh_*.txt`: every column, the attributes, the festivals, the solstice | Yes, 2026-09-29, over plain HTTP: Phugpa 1960–2045, Tsurphu 2012–2014, Bhutanese 2000–2020 |
| [kalacakra-org-software] | "Open source Tibetan calendar software": the planets' worked example, the search example; "Open source Tsurphu calendar software": the Tsurphu day and the *karaṇa* Sun, the mansions' names | Yes, 2026-09-29 |
| [kalacakra-org] | "The Bhutanese calendar", "Bhutan calendars": the weekday, the solstice's 18;45; "Epoch data" | Yes, 2026-09-29 |
| [gantumur-mongolian-calendar] | The months' Mongolian names, илүү, the colour words | Yes, 2026-09-29; a computed calendar over Janson, secondary |
| [moha-bt-calendar-2025], [moha-bt-calendar-2026] | The Winter Solstice on 2 January; the weekday columns | Yes, 2026-09-26, for [tibetan-variants.md](tibetan-variants.md) |
| [tnp-losar] | Chökhor Düchen 2024 on 9 July; 2151 | Yes, 2026-09-25 |
| [henning2007] | Appendix I, the names; Appendix II, the holidays; p. 341, the Tsurphu planets | Not read; cited through Janson |
| Berzin, *Tibetan Astro Science* | The rule for skipped and repeated dates | Not read; cited through Janson |
| Sanders and Bat-Iredüi, *Colloquial Mongolian* | The constitution's date, the month names, the colours | Not read; cited through Janson |

## Code

`crates/hc-calendars-regional/src/tibetan_almanac.rs`: `almanac_day` and
`AlmanacDay`, `karana_of_half_day`, the tables `MANSIONS`, `YOGAS`,
`KARANAS`, `WEEKDAYS` and `RAB_BYUNG_NAMES`; `Planet`, `PlanetPlace`,
`general_day`, `mean_solar_longitude`, `planet_place`, `rahu_head`;
`rab_byung_name`, `royal_year`; `MonthCycle`, `month_symbol`, `COLOURS`,
`MONGOLIAN_COLOURS`, and the re-exports `Symbol`, `year_symbol` and
`day_symbol`, which `crates/hc-calendars-lunar/src/tibetan.rs` defines; `Season`,
`mongolian_month`, `MONGOLIAN_MONTH_NAMES`, `MONGOLIAN_LEAP_WORD`;
`bhutanese_weekday`, `bhutanese_winter_solstice`; `Trigram`, `TRIGRAMS`,
`NineNumber`, `NINE_NUMBERS`, `LunarDayAttributes`, `lunar_day_attributes`,
`day_trigram`, `janson_day_number`, `henning_almanac_day_number`,
`CHINESE_MANSIONS`, `chinese_mansion`, `INDIAN_ELEMENTS`,
`WEEKDAY_ELEMENTS`, `MANSION_ELEMENTS`, `element_pair`; `Festival`,
`HENNING_FESTIVALS`, `berzin_day`, `henning_almanac_day`. The quantities
come from `crates/hc-calendars-lunar/src/tibetan.rs`: `Ratio`, `M2`, `S2`,
`table`, `true_sun`, and `TibetanCalendar::mean_date`, `mean_sun`,
`karana_mean_sun`, `true_date`, `end_day`, `lunar_day_span` and `locate`,
under the registered calendars ([tibetan-variants.md](tibetan-variants.md)).
Anchors: `the_columns_are_those_of_hennings_almanacs`,
`the_tsurphu_programs_worked_day_is_reproduced`,
`the_karanas_follow_the_half_days`, `marss_place_is_hennings_worked_one`,
`the_enlightenment_day_of_hennings_search_is_reproduced`,
`the_years_are_named_and_counted_as_janson_gives_them`,
`the_constitutions_day_month_and_year_have_their_colours`,
`the_months_and_days_are_named_as_hennings_almanacs_name_them`,
`the_bhutanese_weekday_is_one_ahead`,
`the_bhutanese_winter_solstice_is_the_mean_suns_250_degrees`,
`a_festival_on_a_skipped_or_repeated_date_follows_each_rule`,
`the_attributes_are_those_hennings_almanacs_print`,
`the_lunar_day_attributes_follow_jansons_rules`,
`the_calendar_days_numbers_run_both_ways`,
`the_name_tables_are_distinct`; and
`crates/hyper-calendar/tests/mansion_cycles.rs`.
