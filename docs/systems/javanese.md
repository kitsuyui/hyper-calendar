# The Javanese calendar: Sultan Agung's lunar year, the windu and the kurup

Backs the identifiers `javanese`, `javanese-yogyakarta` and `javanese-aboge`
in `hc-calendars-lunar`. The five-day week that runs beside it is
`javanese-pasaran`, written up in
[pawukon-and-pasaran.md](pawukon-and-pasaran.md).

## What it is

Until 1633 Java counted years in the Śaka era on a solar reckoning. In that
year Sultan Agung of Mataram replaced the solar year with the lunar year of
the Hijri calendar, twelve months of 29 and 30 days under the Arabic
month order, and kept the Śaka year number: the year then current, 1555
Śaka, became 1555 Jawa, not 1043 of the Hijra [tanaya1971, karjanto2020,
wikipedia-javanese-calendar]. The era therefore marks no founding event;
it is the Śaka count carried on under another year. Latin writers call it
*Anno Javanico*, AJ [wikipedia-javanese-calendar, karjanto2020].

The first day was Friday, Jumat Legi, 1 Sura 1555, "1 Muharram 1043 of the
Hijra, 8 July 1633" [tanaya1971]; Karjanto and Beauducel give the same day
[karjanto2020]. The decree ran across Java and Madura except Banten, which
was not Mataram's [tanaya1971]; the Indonesian Wikipedia adds Batavia and
Blambangan [idwiki-kalender-jawa]. The Hijri year is the Javanese year less
512 [tanaya1971].

The calendar is kept today beside the Gregorian and Islamic ones, for
cultural and religious purposes and the choice of auspicious days; the
Kraton of Yogyakarta prints one [karjanto2020]. Two groups keep it
differently, and each has an identifier here: the Sultanate of Yogyakarta
between 1821 and 1866, and the Aboge communities today.

## How it works

### Months and days

Twelve months, alternately 30 and 29 days from Sura, as the Hijri months
are; the twelfth, Besar, has a thirtieth day in a long year
[tanaya1971, wikipedia-javanese-calendar]. The names are Javanese forms of
the Arabic ones, some Sanskrit or Malay (Pasa, Sela, Besar), with
alternatives in use; the Javanese script is as Wikipedia prints it
[wikipedia-javanese-calendar, idwiki-kalender-jawa, tanaya1971]:

| # | Name | Script | Also | Arabic | Days |
| --- | --- | --- | --- | --- | --- |
| 1 | Sura | ꦱꦸꦫ | | Muḥarram | 30 |
| 2 | Sapar | ꦱꦥꦂ | | Ṣafar | 29 |
| 3 | Mulud | ꦩꦸꦭꦸꦢ꧀ | Rabingulawal | Rabīʿ I | 30 |
| 4 | Bakda Mulud | ꦧꦏ꧀ꦢꦩꦸꦭꦸꦢ꧀ | Rabingulakir | Rabīʿ II | 29 |
| 5 | Jumadilawal | ꦗꦸꦩꦢꦶꦭꦮꦭ꧀ | | Jumādā I | 30 |
| 6 | Jumadilakir | ꦗꦸꦩꦢꦶꦭꦏꦶꦂ | | Jumādā II | 29 |
| 7 | Rejeb | ꦉꦗꦼꦧ꧀ | | Rajab | 30 |
| 8 | Ruwah | ꦫꦸꦮꦃ | Arwah | Shaʿbān | 29 |
| 9 | Pasa | ꦥꦱ | Siyam | Ramaḍān | 30 |
| 10 | Sawal | ꦱꦮꦭ꧀ | | Shawwāl | 29 |
| 11 | Sela | ꦱꦼꦭ | Apit, Dulkangidah | Dhū al-Qaʿda | 30 |
| 12 | Besar | ꦧꦼꦱꦂ | Kaji | Dhū al-Ḥijja | 29 or 30 |

**The day.** "Days in the Javanese calendar, like the Islamic calendar,
begin at sunset" [wikipedia-javanese-calendar, citing oey2001]. Tanaya
makes the distinction exactly: the *dina*, the weekday and the pasaran, is
counted from sunrise, day then night, while the *tanggal*, the date, is
counted from the moon's appearing on its first evening, "from its night and
then its day" [tanaya1971]. So a date begins at the sunset before the
civil day that carries its name: *malam 1 Suro* is the evening before
1 Sura, and the press gives 1 Sura 1959 as Friday 27 June 2025 and its eve
as Thursday 26 June [kompas-suro-1959]. In this library's terms the
boundary is sunset and the day is named by the civil day it ends on, as
for the Hijri calendars.

### Years and the windu

Eight years make a *windu*. Each year is *wastu*, 354 days, or *wuntu*,
355, the extra day being the thirtieth of Besar; in every windu the second,
fifth and eighth years are wuntu [tanaya1971]. The years are named, and
Tanaya derives the names from the *abjad* numerals of the weekday each year
opened on in the first windu, counted from Friday; the two years named Jim
became Jimawal and Jimakir, the first and last [tanaya1971]:

| # | Name | Krama | Days |
| --- | --- | --- | --- |
| 1 | Alip | Purwana | 354 |
| 2 | Ehe | Karyana | 355 |
| 3 | Jimawal | Anama | 354 |
| 4 | Je | Lalana | 354 |
| 5 | Dal | Ngawanga | 355 |
| 6 | Be | Pawaka | 354 |
| 7 | Wawu | Wasana | 354 |
| 8 | Jimakir | Swasana | 355 |

The lengths are Tanaya's [tanaya1971], the Indonesian Wikipedia's, which
cites the *Almanak van Nederlandsch-Indië* of 1837 [idwiki-kalender-jawa],
and Karjanto and Beauducel's Table 6 for the first two kurup
[karjanto2020]; the krama names are Wikipedia's and Karjanto's
[wikipedia-javanese-calendar, karjanto2020]. A windu is 5 × 354 + 3 × 355
= 2835 days, exactly 81 wetonan of 35 days, so every windu opens on the
same weekday and the same pasaran [tanaya1971, karjanto2020]. The name of a
year is (AJ + 6) mod 8, 1 being Alip and 0 Jimakir [tanaya1971].

Four windu — Adi, Kunthara, Sangara, Sancaya — make 32 years, 11 340 days,
which is also 54 Pawukon cycles of 210 days, so after four windu the
date, weekday, pasaran and *wuku* all return together [tanaya1971]. The
windu of a year is (AJ + 6) mod 32: 1 to 8 Sangara, 9 to 16 Sancaya, 17 to
24 Adi, 25 to 32 Kunthara; Tanaya's example is 1708, windu Adi and year Ehe
[tanaya1971]. 1555 was windu Kunthara, and its first day fell in wuku
Kulawu [tanaya1971]. Tanaya's two tables of a windu's month openings
alternate between windus beginning on wuku Langkir (Adi, Sangara) and on
wuku Kulawu (Kunthara, Sancaya), which is Karjanto and Beauducel's
*lambang* [tanaya1971, karjanto2020]. Other spellings: Kuntara, Sengara
[karjanto2020, wikipedia-javanese-calendar].

### The kurup

The windu has three long years in eight, 45 in 120; the tabular Hijri
calendar has eleven in thirty, 44 in 120. "So that the Javanese reckoning
stays in harmony with the Hijri, whenever the Javanese reckoning has run 15
windu, 120 years, one day is taken from the last wuntu year, which is made
wastu" [tanaya1971]. The 120 years are a *kurup* (Arabic *ḥurūf*, letter),
and 15 × 2835 − 1 = 42 524 days, the length of four thirty-year Hijri
cycles [tanaya1971, wikipedia-javanese-calendar, karjanto2020]. Each
kurup moves the weekday on which its Alip years open back by one, and is
named for it: Alip Jam'iyah (Friday), Kamsiyah (Thursday), Arba'iyah
(Wednesday), Salasiyah (Tuesday), Isnainiyah (Monday), Ahadiyah (Sunday),
Sabtiyah (Saturday) [tanaya1971]. The contracted names join the year, the
weekday and the pasaran: Alip Rebo Wage is *Aboge*, Alip Selasa Pon
*Asapon*, Alip Senen Pahing *Anenhing* [karjanto2020,
idwiki-kalender-jawa].

The record of the kurup is not the rule, because the first change came
late:

| Kurup | Years | Length | First day | Alip years open on |
| --- | --- | --- | --- | --- |
| Jam'iyah | Alip 1555 – Jimakir 1674 | 120 | 8 July 1633 | Jumat Legi |
| Kamsiyah | Alip 1675 – Ehe 1748 | 74 | 11 December 1749 | Kemis Kliwon |
| Arba'iyah, Aboge | Jimawal 1749 – Jimakir 1866 | 118 | 28 September 1821 | Rebo Wage |
| Salasiyah, Asapon | Alip 1867 – Jimakir 1986 | 120 | 24 March 1936 | Selasa Pon |
| Isnainiyah, Anenhing | Alip 1987 – Jimakir 2106 | 120 | 26 August 2052 | Senen Pahing |

The years and weekdays are Tanaya's, and he names Sabtiyah as running
from Alip 2227 to Jimakir 2346 [tanaya1971]; the first days are Karjanto
and Beauducel's Table 8, which this library reproduces
[karjanto2020]. Wikipedia gives 24 March 1936 and 26 August 2052 too, and
25 August 2052, a Sunday, as the last day of Asapon
[wikipedia-javanese-calendar].

- **1748.** "At the end of the year 1748, by order of His Majesty
  Susuhunan Pakubuwana V of Surakarta, the count of the year was changed by
  stepping over one day: after Thursday 29 Besar of the year Ehe 1748, the
  count went on with Friday 1 Sura of the year Jimawal 1749" [tanaya1971,
  quoting a primbon]. Ehe 1748, a wuntu year, was made wastu, and
  Arba'iyah began two years into a windu. Some scholars, Tanaya reports,
  held that the change was two years late and belonged at Alip 1747
  [tanaya1971, karjanto2020]; it is. In this library's arithmetic 1 Sura
  1747 falls on 21 October 1819 and 1 Muḥarram 1235 of the civil tabular
  Hijri calendar on 20 October, the day behind that Karjanto and Beauducel
  say the experts of the time noticed [karjanto2020].
- **1936.** The Surakarta government's order of 15 December 1935,
  published in *Kabar Paprentahan* no. 24 as no. 1467 A/I, which Tanaya
  quotes: on the report of the Radyapustaka society that the change of
  kurup "has been fixed at every 120 years, one day forward", and a note
  from the Governor's office of 18 December 1934, that the next change
  falls at 1746 + 120 = 1866, "so that the year Jimakir 1866 is not used
  as a wuntu year", 1 Sura Alip 1867 "moves one day forward, falling on
  24 March 1936, a Selasa Pon" [tanaya1971]. The Pangulu's reckoners
  agreed and the Susuhunan assented.

Tanaya's rule is the whole of it from 1749: the 120 years are counted from
1747, so every kurup since ends at a Jimakir, and the 1935 order states
the rule forward. Karjanto and Beauducel hold that the months of the next
kurup "have not yet been decided" [karjanto2020], but what they mean is the
court's month adjustments below, which the rule does not include.

At the opening of every kurup since Jam'iyah, this library's arithmetic
puts 1 Sura on the first of Muḥarram of the civil tabular Hijri calendar
([hijri.md](hijri.md)) in the year AJ − 512: 1043, 1163, 1237 (at Jimawal
1749), 1355 and 1475. Wikipedia says the same of 1355 and 1475
[wikipedia-javanese-calendar]. Inside a kurup the two drift a day apart
and back, since three long years in eight and eleven in thirty place the
long years differently.

### Worked example: 1 Sura 1959

1959 + 6 = 1965. 1965 mod 8 = 5, the year Dal; 1965 mod 32 = 13, the windu
Sancaya. 1959 is inside Salasiyah, which opened on Tuesday 24 March 1936,
Selasa Pon, 92 years earlier: 11 windu and four years, Alip, Ehe, Jimawal
and Je. That is 11 × 2835 + 354 + 355 + 354 + 354 = 32 602 days, and
24 March 1936 + 32 602 days is 27 June 2025. 32 602 = 7 × 4657 + 3, so the
weekday is three on from Tuesday, a Friday; 32 602 = 5 × 6520 + 2, so the
pasaran is two on from Pon, Kliwon. 1 Sura 1959 Dal is Jumat Kliwon,
27 June 2025, as Tanaya's table has Dal open in Salasiyah [tanaya1971] and
as the Kompas calendar prints it [kompas-suro-1959].

### The competing reckonings

**Yogyakarta, 1749 to 1866.** The Sultanate of Yogyakarta, which the
Treaty of Giyanti had divided from Surakarta in 1755, did not follow the
change of 1748: it ran Kamsiyah for its full 120 years, to Jimakir 1794,
so that the two courts' dates stood a day apart for 46 years. Sultan
Hamengkubuwana VI then agreed that Arba'iyah would end with Jimakir 1866,
as Surakarta's did, and the two reckonings agree from 1 Sura 1795,
16 May 1866 [karjanto2020, table 9; idwiki-kalender-jawa, which cites van
Dorp's almanac of 1865, not read, vandorp1865]. That is `javanese-yogyakarta`.

**Aboge.** The Aboge communities of Banyumas, Purbalingga, Cilacap and
Probolinggo never took the change of 1936 and still open every Alip year on
Rebo Wage, so they begin the fast and keep Idul Fitri a day or two after
everyone else [idwiki-kalender-jawa]. In 2017 the Aboge of Cikakak,
Banyumas, prayed the Idul Fitri prayer on Tuesday 27 June; their year was
Je, whose 1 Muḥarram fell on Selasa Pahing [tempo-aboge-2017]. In 2024
their fast began on Rebo Wage, 13 March, and Idul Fitri was Jumat Wage,
12 April, by the formula *Waljiro*: Sawal opens on the first weekday and
the second pasaran counted from the year's first day [detik-aboge-2024].
In 2025 the fast began on Sunday Pon, 2 March, and Idul Fitri was Selasa
Pon, 1 April [kompas-aboge-2025]. All of these are Tanaya's Arba'iyah
table carried on without a change of kurup [tanaya1971]. That is
`javanese-aboge`, which is `javanese` until 29 Besar Jimakir 1866 and a
day later from then on; it stays in Arba'iyah and never drops another day,
so from 1987 it is two days later.

**The Surakarta court's Garebeg adjustments.** Sultan Agung's first
Garebeg Mulud, 12 Mulud of the Dal year 1559, fell on Senen Pon, and 1 Sura
of every Dal year under Jam'iyah fell on the same weton [tanaya1971]. When
the kurup moved the day, the Surakarta court kept the Dal Garebeg on Senen
Pon by changing month lengths: under Kamsiyah, Sapar of Dal was given 30
days and Mulud 29; under Arba'iyah, Besar of Je 30 and Sapar of Dal 30,
with Mulud and Jumadilawal of Dal 29; under Salasiyah, at first, Sawal and
Besar of Je and Sapar of Dal 30, with Mulud, Jumadilawal and Rejeb 29.
Tanaya adds that the last practice "seems not to have been continued" and
that the true Monday Garebeg under Salasiyah is the one of a Je year, as in
1902, Senen Legi [tanaya1971]. A reckoner writing in 1831 says that in a
Dal year one "must use the plain date, not the date as changed by the
State" [tanaya1971, quoting Bagus Ngarfah]. Karjanto and Beauducel's
Tables 5 and 6, and Beauducel's program `weton.m`, carry a version of these
adjustments: Sapar 30 and Mulud 29 in Dal under Kamsiyah, and under
Arba'iyah and Salasiyah a long Je and a short Dal [karjanto2020,
beauducel2020weton].

**Web almanacs that shorten Ehe.** Wikipedia's list of year lengths makes
Ehe 354 days and Jimawal 355, against every other source read
[wikipedia-javanese-calendar], and two almanac sites agree with it: they
put 1 Sura 1957 on 19 July 2023 [kidemang-almanak, kalenderku-2024] and
30 Besar 1957 on 7 July 2024 [kalenderku-2024]. Under Tanaya's rule 19 July
2023 is 30 Besar 1956, as the Radyapustaka Museum's calendar expert gave it
that week [detik-besar-1956], and Jimawal 1957 has no 30 Besar. The two
agree from 1 Sura 1958 onwards. None of the three names a text, so this is
recorded as a disagreement and not registered as a convention.

## What is carried

- **Identifiers** `javanese`, the reckoning of Surakarta with Tanaya's
  rule, which is the standard one, Asapon since 1936; `javanese-yogyakarta`,
  the same with Kamsiyah run to Jimakir 1794; `javanese-aboge`, the same
  with Arba'iyah never ended. All three are one parameterised calendar,
  `JavaneseCalendar`, whose only data is its list of kurup, each with its
  first year and the year whose last day it drops.
- **Range**: 1 Sura 1555, 8 July 1633, to the end of Jimakir 2346, the last
  year of Sabtiyah, the last kurup Tanaya names [tanaya1971]: 6 December
  2401 in `javanese` and `javanese-yogyakarta`, 11 December 2401 in
  `javanese-aboge`. Isnainiyah onwards is the 1935 rule projected, not a
  decision anyone has made yet.
- **Fields**: the year AJ under the era code `aj`, the month 1 to 12, the
  day, and three extra fields: `taun`, the year in the windu, 1 Alip to 8
  Jimakir; `windu`, 1 Adi to 4 Sancaya; `kurup`, 1 Jam'iyah to 7 Sabtiyah.
  The three are derived from the date and ignored when a date is read back.
- **Names**: the months, the years and the windu as the calendar's own, in
  Latin script, with the months' Javanese script as data; the kurup names.
- **The year is leap** when it is wuntu, 355 days. A wuntu year that
  closes a kurup is made wastu [tanaya1971] and is not leap.
- **The day** begins at sunset and is named by the civil day it ends on.
- **Period of use**: from 8 July 1633 for `javanese` and `javanese-aboge`,
  kept today; `javanese-yogyakarta` until 15 May 1866, after which its dates
  are Surakarta's.
- **Not carried**: the Śaka lunisolar reckoning before 1633; the Surakarta
  court's Garebeg adjustments; the *wuku*, which is `balinese-pawukon`'s,
  and the pasaran and weton, which are `javanese-pasaran`'s — a test holds
  this calendar's dates to both; the *masa-wuku* and the *pranata mangsa*;
  the six-, eight- and nine-day weeks; the *dina mulya* and other
  observances; any kurup after Sabtiyah.

## Accuracy

The arithmetic is exact: the rule is the definition, integer arithmetic on
the fixed day. What the tests check against a source outside the code:

| Check | Source | Result |
| --- | --- | --- |
| 1 Sura 1555 is 8 July 1633, a Friday, Jumat Legi, wuku Kulawu, windu Kunthara, and 1 Muḥarram 1043 civil | [tanaya1971, karjanto2020] | Holds |
| The first day of each kurup: 8 July 1633, 11 December 1749, 28 September 1821, 24 March 1936, 26 August 2052; Yogyakarta's 16 May 1866 | [karjanto2020, wikipedia-javanese-calendar] | Holds |
| Thursday 29 Besar Ehe 1748 followed by Friday 1 Sura Jimawal 1749 | [tanaya1971] | Holds |
| 24 March 1936 a Selasa Pon; 25 August 2052 a Sunday, the last of Asapon | [tanaya1971, wikipedia-javanese-calendar] | Holds |
| The weton of 1 Sura of every year, against Tanaya's four kurup tables: Surakarta 1555–1986, Yogyakarta 1675–1986, Aboge 1555–2346 | [tanaya1971, karjanto2020] | Holds on all 1536 |
| The weekday, pasaran and wuku of the first of every month of Salasiyah, from Tanaya's two tables of 96, over the fifteen windus they head: 1440 first days | [tanaya1971] | 1412 hold; the other 28 are four misprints in the seven windus one table serves, below |
| Garebeg Mulud of Dal 1559 on Senen Pon, of Je 1902 on Senen Legi; 4 Sela Dal 1831 a Rebo Wage | [tanaya1971] | Holds |
| 13 Sura 1682, 7 October 1756, a Kemis Pahing | [karjanto2020] | Holds |
| 1 Sura 1959 Jumat Kliwon 27 June 2025, 30 Sura Setu Wage 26 July | [kompas-suro-1959] | Holds |
| 19 July 2023, Rebo Legi, 30 Besar 1956 | [detik-besar-1956] | Holds |
| Every kurup since 1749 opens on 1 Muḥarram (AJ − 512) of the civil tabular Hijri calendar, and a full kurup is 42 524 days | [tanaya1971, wikipedia-javanese-calendar] | Holds |
| Aboge: 1 Sawal 1950 Tuesday 27 June 2017, its year opening on Selasa Pahing; 1 Pasa 1957 Rebo Wage 13 March 2024, 1 Sawal Jumat Wage 12 April; 1 Pasa 1958 Ahad Pon 2 March 2025, 1 Sawal Selasa Pon 1 April | [tempo-aboge-2017, detik-aboge-2024, kompas-aboge-2025] | Holds |
| The windu and year of 1708, Adi and Ehe, by Tanaya's rule | [tanaya1971] | Holds |

**The four misprints.** Tanaya's two month tables, as the Yayasan Sastra
Lestari transcribes them, give one weekday-pasaran-wuku triple for the
first of every month of Salasiyah's windus. Four of the 192 cells disagree
with the rule, all four in the table of the Kunthara and Sancaya windus, and
each is a misprint by its own evidence: two give a pasaran neptu of 3,
which no pasaran has (Sapar of Je, Rejeb of Alip); one gives a Jumat where
the same entry in the Adi and Sangara table, which must carry the same
weton, gives Ahad (Jumadilawal of Je); and one gives wuku 17 where the
other table's 12, plus the fifteen wuku that separate the two tables,
gives 27 (Ruwah of Alip). The test holds every entry as printed, over
every windu it serves, and asserts that exactly those four cells
disagree.

**Known disagreements.**

- Karjanto and Beauducel's Tables 5 and 6 and `weton.m` carry the court's
  Dal adjustments, above, so they differ from the plain rule during the Dal
  years of Kamsiyah and during the Je and Dal years of Arba'iyah and
  Salasiyah: `weton.m` puts 27 June 2025 on 30 Besar 1958 and 1 Sura 1959
  on the 28th [beauducel2020weton], where the Kompas calendar has the
  27th [kompas-suro-1959]. Their prose also says that "Jimawal and Wawu are
  always leap years", which their own Table 6 contradicts
  [karjanto2020].
- `weton.m` drops Surakarta's day at the end of Arba'iyah from Besar of Be
  1864, leaving that month 28 days, rather than from Jimakir 1866, and the
  example in its header, 12 Pasa 1900 for 3 December 1968, is a day off its
  own table, which gives 13 Pasa [beauducel2020weton]. This library gives
  13 Pasa too.
- Karjanto and Beauducel abbreviate the first kurup A'ahgi
  [karjanto2020]; by the naming rule it would be Alip Jumat Legi.
- The web almanacs' long Jimawal, above.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [tanaya1971] | The whole rule: the months, wastu and wuntu, the year and windu names and the two mod rules, the kurup and their names, the 1748 and 1935 changes quoted, the seven kurup to 2346, the first-day tables of every kurup, the month tables of Salasiyah, the day from night, the Garebeg adjustments, Bagus Ngarfah's plain dates, AJ − 512 | Yes, 2026-09-26, the Yayasan Sastra Lestari transcription through a Wayback Machine copy of 21 April 2026, sastra.org refusing the connection |
| [karjanto2020] | The history, the kurup table with first days, Yogyakarta's table, 13 Sura 1682, the windu and lambang names, the tables of month and year lengths as they carry them | Yes, 2026-09-26, arXiv v1 |
| [beauducel2020weton] | The program behind Karjanto and Beauducel's tables, as a cross-check | Yes, 2026-09-26; BSD-licensed, read and not copied |
| [wikipedia-javanese-calendar] | The months in Latin and Javanese script, the krama year names, sunset, 24 March 1936, 25 and 26 August 2052, 1 Muḥarram 1355 and 1475 | Yes, 2026-09-26 |
| [idwiki-kalender-jawa] | The Asapon year table, the Yogyakarta reckoning, the Aboge communities | Yes, 2026-09-26 |
| [kompas-suro-1959] | 1 and 30 Sura 1959 | Yes, 2026-09-26 |
| [detik-besar-1956] | 30 Besar 1956 from the Radyapustaka Museum's expert | Yes, 2026-09-26 |
| [kidemang-almanak], [kalenderku-2024] | The almanacs that shorten Ehe | Yes, 2026-09-26 |
| [tempo-aboge-2017], [detik-aboge-2024], [kompas-aboge-2025] | The Aboge dates | Yes, 2026-09-26 |
| [oey2001] | Days from sunset, as Wikipedia cites it | Not read |
| [vandorp1865] | Yogyakarta's reconciliation, as the Indonesian Wikipedia cites it | Not read |
| [danudji2006] | A printed 120-year table of Asapon, cited by Wikipedia for the kurup dates | Not read |
| [proudfoot2006], [proudfoot2007] | The Muslim calendars of Southeast Asia; the kurup matching the Islamic cycle, as Karjanto and Beauducel cite it | Not read |
| [ricklefs1978], [ricklefs1993] | The Javanese historical tradition; the Śaka reckoning before 1633, as Wikipedia and Karjanto and Beauducel cite it | Not read |
| [gislen-eade-2019] | The calendars of Malaysia and Indonesia | Not read: the PDF link served an HTML page |

A primary source that would replace the secondary ones here: van Dorp's
almanac of 1865 for Yogyakarta, and a current Kraton almanac for the
present kurup.

## Code

`crates/hc-calendars-lunar/src/javanese.rs`. Anchors:
`the_first_day_is_friday_8_july_1633_and_1_muharram_1043`,
`every_kurup_opens_where_the_sources_put_it`,
`pakubuwana_v_stepped_over_a_day_after_29_besar_1748`,
`the_1935_order_moved_1_sura_1867_to_24_march_1936`,
`every_kurup_opens_on_the_tabular_first_of_muharram`,
`the_dated_days_in_tanaya_and_karjanto`,
`the_printed_calendars_of_recent_years`,
`the_aboge_communities_dates`,
`the_web_almanacs_long_jimawal_is_not_followed`,
`the_windu_and_year_names_follow_tanayas_rules`,
`the_kurup_lists_are_well_formed`, `the_range_ends_with_jimakir_2346`, and
the round trips over the whole range. The weton and the wuku are checked
in `crates/hc-calendars-regional/tests/javanese.rs`:
`the_first_day_of_every_year_has_tanayas_weton`,
`the_first_day_was_jumat_legi_in_wuku_kulawu`,
`the_months_of_two_windu_open_as_tanaya_tabulates`,
`the_published_days_carry_their_weton`.
