# The Balinese Pawukon and the Javanese pasaran

Backs the identifiers `balinese-pawukon` and `javanese-pasaran` in
`hc-calendars-regional`.

## What it is

**The Pawukon.** A 210-day cycle kept in Bali, with its origins in Balinese
Hinduism, divided at the same time into ten concurrent weeks of one, two,
three, … ten days [wikipedia-pawukon]. A day is named by where it stands
in several of them at once, and the religious calendar keys off their
coincidences: Galungan is the day that is Buda (the fourth day of the
seven-day week), Kliwon (of the five-day week) and in the *wuku* Dungulan
[idntimes-galungan-2018], so it comes round every 210 days — on
17 June 2026, for instance [detik-galungan-2026]. The seven-day week
carries thirty named weeks, the *wuku*, and 30 × 7 = 210 is the whole
cycle [wikipedia-pawukon].

**The pasaran.** The five-day market week of Java, Legi, Pahing, Pon, Wage
and Kliwon, each with a *krama* (formal) name beside it — Manis, Pait,
Pethak, Cemèng, Asih — and named, it is said, for the markets that once
met on one day in five [wikipedia-javanese-calendar]. It is the same
cycle as the Pawukon's five-day week under Javanese names: the Balinese
Umanis is the Javanese Manis. Run against the seven-day week, which Java
took with Islam, it gives the 35-day *wetonan*, and a day's *weton* is the
pair: Tuesday 6 May 2008 was Selasa Wage [wikipedia-javanese-calendar].
A person's weton is the pair they were born on, and it is used to judge
character, to match marriage partners and to choose auspicious days; a
weton recurs every 35 days, the *selapan*, which is kept as a small
birthday [idwiki-weton]. The weton every Indonesian schoolchild is taught
is that of the proclamation of independence on 17 August 1945, Jumat Legi
[wikipedia-javanese-calendar].

**No year.** Neither cycle counts anything larger than itself. Pawukon
cycles are unnumbered, so the calendar has no epoch, and any anchor is a
choice [wikipedia-pawukon]; the same is true of the wetonan. Bali and Java
count years in other calendars — the Balinese Saka year, the Javanese
lunar year of Sultan Agung — which this document does not cover.

## How it works

**The regular weeks.** 210 = 2 × 3 × 5 × 7, so the three-, five-, six- and
seven-day weeks divide it and simply repeat: on day *d* of the Pawukon,
counted from 0, the three-day week is `d mod 3 + 1`, the six-day week
`d mod 6 + 1`, the seven-day week `d mod 7 + 1` and the *wuku*
`⌊d / 7⌋ + 1`. The five-day week is offset by one: it is
`(d + 1) mod 5 + 1`, counting Umanis as 1, so day 0 is Paing
[reingold2018code, `bali-triwara-from-fixed`, `bali-sadwara-from-fixed`,
`bali-saptawara-from-fixed`, `bali-pancawara-from-fixed`,
`bali-week-from-fixed`].

**The irregular weeks.** 210 is not a multiple of 4, 8 or 9, and the
tradition pads each of the three [wikipedia-pawukon]:

- The nine-day week repeats its first day three times at the start: days 0
  to 3 are all Dangu, and it runs regularly from day 4, closing exactly at
  day 209 since 207 = 23 × 9. The rule is
  `max(0, d − 3) mod 9 + 1` [reingold2018code, `bali-sangawara-from-fixed`].
- The eight-day week holds its seventh day, Kala, for three days in the
  *wuku* Dungulan, days 70, 71 and 72:
  `max(6, (d − 70) mod 210 + 4) mod 8 + 1` [reingold2018code,
  `bali-asatawara-from-fixed`].
- The four-day week has no rule of its own: it is the eight-day week
  reduced modulo four, so it holds its third day, Jaya, over the same
  three days [reingold2018code, `bali-caturwara-from-fixed`].

**The derived weeks.** The ten-, two- and one-day weeks are not counted at
all. Each day of the five- and seven-day weeks carries an *urip*, a ritual
weight: Umanis 5, Paing 9, Pon 7, Wage 4, Kliwon 8; Redite 5, Soma 4,
Anggara 3, Buda 7, Wraspati 8, Sukra 6, Saniscara 9. Add the two and one,
and reduce modulo ten: that value *v* fixes the ten-day week; the day is
Luang in the one-day week and Pepet in the two-day week when *v* is even,
and Menga, with no one-day name, when it is odd [reingold2018code,
`bali-dasawara-from-fixed`, `bali-dwiwara-from-fixed`,
`bali-luang-from-fixed`; wikipedia-pawukon;
sejarahharirayahindu-dasawara]. The same weights, under the Javanese
names, are the *neptu*: Ahad 5, Senen 4, Selasa 3, Rebo 7, Kemis 8,
Jemuwah 6, Setu 9; Legi 5, Pahing 9, Pon 7, Wage 4, Kliwon 8, and a
weton's neptu is the two added [idwiki-weton].

Reingold and Dershowitz's code gives the value *v* as a number only.
Wikipedia names each day of the ten-day week by an urip and takes the day
whose urip equals *v*, counting 0 as 10; on its list that makes *v* =
1 Pandita, 2 Pati, 3 Suka, 4 Duka, 5 Sri, 6 Manuh, 7 Manusa, 8 Raja,
9 Dewa and 10 Raksasa [wikipedia-pawukon] — the ten names in their
customary order [banwiki-dasawara], so *v* is the place in that list. A
Balinese published reading of Galungan, where *v* = 8 + 7 + 1 − 10 = 6,
names the day Manuh [idntimes-galungan-2018], which agrees with that
assignment. The Balinese source that states the formula lists the days in
a different order of urip — Raksasa 1, Manuh 2, Manusa 3, Duka 4, Pandita
5, Sri 6, Pati 7, Raja 8, Dewa 9, Suka 10
[sejarahharirayahindu-dasawara] — by which Galungan would be Sri, against
the published reading.

**The wetonan.** The pasaran runs against the ordinary week, and since 5
and 7 are coprime every pair occurs once in 35 days. A weekday *w* (0 for
Ahad) and a pasaran *p* (0 for Legi) sit at position `(21p + 15w) mod 35`
of the cycle: 21 is a multiple of 7 that leaves 1 modulo 5, and 15 a
multiple of 5 that leaves 1 modulo 7. Reingold and Dershowitz use the same
35-day subcycle to search for a Pawukon day [reingold2018code,
`bali-on-or-before`].

**Anchors.** Reingold and Dershowitz start a Pawukon cycle at Julian Day
Number 146, fixed day −1 721 279 — 26 May 4713 BCE in the proleptic Julian
calendar — the first cycle to begin on a positive Julian Day Number
[reingold2018code, `bali-epoch`; wikipedia-pawukon]. Its day 0 is
Redite, a Sunday, and it has stayed aligned with the Gregorian weekday:
Tuesday 5 January 2021 is day 184 of the cycle that began on 5 July 2020,
and Anggara, a Tuesday [wikipedia-pawukon]. Fixed day 0, 31 December 1 BCE
in the proleptic Gregorian calendar, is day 119 of a cycle and, under the
five-day week above, Umanis, a Sunday: in Javanese, Ahad Legi, the first
position of the wetonan. So the wetonan position of a fixed day *n* is
simply `n mod 35`.

**Worked example: 17 August 1945.** Its fixed day is 710 260.

*The weton.* 710 260 = 35 × 20 293 + 5, so the day is at position 5 of the
wetonan: weekday 5 mod 7 = 5, Jemuwah (Friday), and pasaran 5 mod 5 = 0,
Legi. The weton is Jemuwah Legi — Jumat Legi in Indonesian — and its neptu
is 6 + 5 = 11.

*The Pawukon.* 710 260 − (−1 721 279) = 2 431 539 = 210 × 11 578 + 159, so
it is day 159. Then:

| Week | Rule | Value | Name |
| --- | --- | --- | --- |
| *wuku* | ⌊159 / 7⌋ + 1 | 23 | Menail |
| seven-day | 159 mod 7 + 1 | 6 | Sukra |
| five-day | 160 mod 5 + 1 | 1 | Umanis |
| three-day | 159 mod 3 + 1 | 1 | Pasah |
| six-day | 159 mod 6 + 1 | 4 | Paniron |
| eight-day | max(6, 89 + 4) mod 8 + 1 | 6 | Brahma |
| four-day | (6 − 1) mod 4 + 1 | 2 | Laba |
| nine-day | 156 mod 9 + 1 | 4 | Nohan |
| ten-day | (6 + 5 + 1) mod 10 | 2 | Pati |
| two-day, one-day | 2 is even | — | Pepet; Luang |

In the three cycles a Balinese date is usually written in, it is Sukra
Umanis Menail: the Friday that is Umanis, in the *wuku* Menail. The five-day
week says Umanis where the pasaran says Legi, the same day under two
names.

## What is carried

- **Identifiers** `balinese-pawukon` and `javanese-pasaran`, in
  `hc-calendars-regional`, unbounded in both directions: a cycle has no
  first or last day. Neither is dated as a calendar in use, since no
  source read dates the start of either.
- **The date** is a position and a *round*, the number of complete cycles
  since the anchor: `PawukonDate { round, day }` with the day 0 to 209,
  and `WetonDate { round, dina, pasaran }`. Nothing in Bali or Java counts
  rounds; they exist so that a date round-trips through a fixed day, which
  every calendar here must do, and the `year` field carries them.
- **The Pawukon's fields**: the *wuku* as the month (1 to 30), the
  seven-day week as the day, and the other eight counted weeks — two- to
  ten-day except the seven — as extra fields. The one-day week is not a
  field; `is_luang` reads it off the ten-day week's parity. Every Balinese
  name of the ten weeks and the thirty *wuku* is carried, with the urip of
  the five- and seven-day weeks. The seven-day week is declared as the
  weekday, so a locale's weekday names serve it.
- **The wetonan's fields**: no month; the position in the 35-day cycle,
  1 to 35, as the day, and the weekday, the pasaran and the neptu as extra
  fields. The Javanese *ngoko* names of both weeks and their neptu are
  carried.
- **The anchors**: Julian Day Number 146 for the Pawukon, as Reingold and
  Dershowitz give it; fixed day 0 for the wetonan, which is the same
  five-day cycle's Legi. The two are written separately in the code and a
  test holds them to each other.
- **Not carried**: the Pawukon's holy days and named coincidences —
  Galungan, Kuningan, Tumpek, Kajeng Kliwon, which Reingold and Dershowitz
  compute as `tumpek` and `kajeng-keliwon` [reingold2018code] — since they
  are observances, the province of `hc-holiday`; the urip and meanings of
  the ten-day week's own days and the rest of the divination; the *krama*
  pasaran names; any year count, Balinese or Javanese. The Javanese lunar
  calendar the pasaran belongs to is written up in
  [javanese.md](javanese.md), and its tests hold its days to the pasaran
  and the *wuku* here.

## Accuracy

The arithmetic is exact: integer arithmetic on the fixed day, no
approximation. What the tests check against a source outside the code:

| Check | Test | Result |
| --- | --- | --- |
| Five Galungan dates 210 days apart, 28 February 2024 to 17 June 2026, are Buda Kliwon Dungulan, day 73 | `galungan_always_falls_on_buda_kliwon_dungulan` | Holds |
| Galungan, 17 June 2026, is on every week as the published reading gives it, the ten-day week Manuh | `galungan_is_manuh_in_the_ten_day_week` | Holds |
| 5 January 2021, Wikipedia's worked day, is day 184 and on every week as the article gives it, the ten-day week Dewa | `wikipedias_worked_day_comes_out_on_all_ten_weeks` | Holds |
| The first and sixth days of the cycle are Sri and Manuh in the ten-day week, as the article works them | `the_first_and_sixth_days_are_sri_and_manuh` | Holds |
| The seven-day week is the Gregorian weekday, Redite Sunday, over 500 days | `the_seven_day_week_never_slipped_against_the_gregorian_one` | Holds |
| The anchor is Julian Day Number 146, a Sunday that is day 0 of Sinta | `the_epoch_is_julian_day_one_hundred_and_forty_six` | Holds |
| 17 August 1945 is Jemuwah Legi, neptu 11 | `indonesian_independence_was_declared_on_jumat_legi` | Holds |
| The pasaran and the Pawukon's five-day week agree on every day tested, and their weights are the same numbers | `the_pasaran_is_the_balinese_pancawara_under_other_names` | Holds |

The remaining tests check the shape of each cycle: the nine-day week's
start, the eight-day week's pause, the four-day week's inheritance of it,
the ten-day week's construction from the urip, the parity of the one- and
two-day weeks, and that every day of a cycle round-trips.

Checked for this document and not by a test: Wikipedia's table of all
210 days gives, on every day, the names the module gives on all ten weeks
and the *wuku*, save for spelling — Soma for Coma, Keliwon for Kliwon,
Warigadian, Dunggulan and Parangbakat for Warigadean, Dungulan and
Prangbakat; and 6 May 2008 is Selasa Wage [wikipedia-pawukon,
wikipedia-javanese-calendar]. Reingold and Dershowitz's book, which would
say what their numbering of the ten-day week means, was not read; their
code carries no names.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [reingold2018] | The Pawukon's arithmetic as a whole, §10.6 per the module | Not read directly; the published code was |
| [reingold2018code] | `bali-epoch`, `bali-day-from-fixed`, the ten `bali-…-from-fixed` functions, `bali-week-from-fixed`, `bali-on-or-before`, `kajeng-keliwon`, `tumpek` | Yes, 2026-09-26 |
| [wikipedia-pawukon] | The ten weeks and their names, the padding of the four-, eight- and nine-day weeks, the urip and the ten-day rule, the anchor's Julian date, the 5 January 2021 example, the table of all 210 days | Yes, 2026-09-26; the article cites no source |
| [idntimes-galungan-2018] | Galungan as Buda Kliwon Dungulan, and all ten week names of that day | Yes, 2026-09-26; it cites no source |
| [detik-galungan-2026] | Galungan on 17 June 2026, from the Ministry of Religious Affairs' circular | Yes, 2026-09-26 |
| [kemenag-b253-2025] | The circular itself | Not read: the ministry's host refused the connection on 2026-09-26 |
| [banwiki-dasawara] | The ten-day week's names in their customary order | Yes, 2026-09-26 |
| [sejarahharirayahindu-dasawara] | The ten-day formula, (urip + urip + 1) mod 10, and the days in the order of their own urip | Yes, 2026-09-26; a blog, cited for want of a better one read |
| [wikipedia-javanese-calendar] | The pasaran's names and *krama* names, the wetonan, Selasa Wage, Jumat Legi of 17 August 1945 as taught in schools | Yes, 2026-09-26 |
| [idwiki-weton] | The neptu of both weeks, the weton's uses, the 35-day *selapan* | Yes, 2026-09-26; it names the *Primbon Betaljemur Adammakna*, not read |
| [idwiki-proklamasi] | 17 August 1945 a Friday | Yes, 2026-09-26; it gives no pasaran |
| The other four Galungan dates in the test, 2024 and 2025 | The test's other rows | Not re-read; they follow from 2026 by multiples of 210 days |

Statements the module documentation and data made before this write-up
that no source read here supports, recorded so that they are not mistaken
for sourced. The first two are trimmed from the module header; the others
stand in the constants' documentation and the name tables:

- That the weton is used for "commemorating a death every 35 days". The
  source read describes the 35-day recurrence as a birthday observance.
- That Jumat Legi of 17 August 1945 is "repeated in every Indonesian
  account of the date". Wikipedia says it is widely taught; the Indonesian
  article on the proclamation gives the weekday only.
- That "other regional variants exist" for the pasaran names and none
  could be sourced: the *krama* names are sourced above, and no regional
  variant was found.
- The spelling Coma for the seven-day week's Monday, where the sources
  read spell it Soma.
- The Javanese weekday names as the module spells them — Ahad, Senen,
  Selasa, Rebo, Kemis, Jemuwah, Setu — which no source read lists together;
  the Indonesian article uses the Indonesian names, and Wikipedia writes
  Jemuah.

## Code

`crates/hc-calendars-regional/src/balinese_pawukon.rs` and
`crates/hc-calendars-regional/src/javanese_pasaran.rs`. Anchors:
`galungan_always_falls_on_buda_kliwon_dungulan`,
`the_seven_day_week_never_slipped_against_the_gregorian_one`,
`the_epoch_is_julian_day_one_hundred_and_forty_six`,
`galungan_is_manuh_in_the_ten_day_week`,
`wikipedias_worked_day_comes_out_on_all_ten_weeks`,
`indonesian_independence_was_declared_on_jumat_legi`,
`the_epoch_is_ahad_legi`,
`the_pasaran_is_the_balinese_pancawara_under_other_names`. The cycles'
shapes: `the_nine_day_week_does_not_start_until_day_four`,
`the_eight_day_week_pauses_for_three_days_in_dungulan`,
`the_four_day_week_inherits_the_eight_day_weeks_pause`,
`the_ten_day_week_is_built_from_urip_not_from_the_day_number`,
`the_one_and_two_day_weeks_are_the_ten_day_weeks_parity`,
`the_wetonan_is_thirty_five_days_and_visits_every_pair`,
`neptu_adds_the_two_weights`.
