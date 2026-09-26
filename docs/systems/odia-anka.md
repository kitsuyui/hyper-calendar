# The Odia Anka: the Gajapati's regnal years from Suniā

Backs the identifier `odia-anka` in `hc-calendars-indic`.

## What it is

The *aṅka* (Odia ଅଙ୍କ, *Onko* in Sewell and Dikshit's spelling) is the
regnal count of the kings of Orissa, instituted by the Eastern Ganga kings
[wikipedia-anka-year] and still declared every year by the Gajapati
Maharaja of Puri. Sewell and Dikshit found it in use in the 1890s in the
temple of Jagannātha at Puri and in the zamindari tracts of Parlakimedi,
Peddakimedi and Chinnakimedi, each of which named its years after its own
zamindar [sewell1896, Art. 64]. Today the count is the Gajapati's: on the
Suniā festival of 2024 "Gajapati Maharaja will enter 68 'Anka'"
[odishatv-sunia-2024], in 2025 "Gajapati Dibyasingha Deb entered his 69th
Anka (regnal year), ushering in the 1433 Sal" [odishatv-sunia-2025], and
in 2026 his 71st, with 1434 Sal [odishatv-sunia-2026,
kalingatv-sunia-2026]. In 2011 the declaration was "52 anka 1419"
[puriwaves-sunia-52].

The Gajapati today is Dibyasingha Deb, who succeeded his father,
Birakishore Deb, and is given as in office from 7 July 1970
[wikipedia-dibyasingha-deba, citing a Sambad article of 2017 that was not
read].

## How it works

**The numbers that are dropped.** "In their notation, the years whose
numeral is 6, or whose numerals end with 6 or (except 10), are dropped …
the years succeeding the 5th and 19th Onkos of a prince or zamindar are
called the 7th and 21st Onkos respectively" [sewell1896, Art. 64]. The
OCR text reads "end with 6 or (except 10)", and the list Sewell and
Dikshit give a few lines later settles what was lost: to value an Onko
year one excludes "the 1st — possibly, the 6th, 16th, 20th, 26th, 30th,
36th, 40th, 46th, 50th, 56th". So every number ending in 6 is dropped, and
every number ending in 0 except 10. The present-day articles and Wikipedia
say the same [kalingatv-sunia-2026, wikipedia-anka-year], and the printed
numbers agree with it: 67 was not followed by 70 but by 68, 69 and 71
[odishatv-sunia-2024, odishatv-sunia-2025, odishatv-sunia-2026].

**The first number.** Sewell and Dikshit explain the missing 1: "when a
prince dies in the middle of an Onko year, his successor's 1st Onko which
commences on his accession to the throne, does not run its full term of a
year, but ends on the [12th] day of Bhadrapada-suddha following;
consequently the last regnal year of the one and the first of the other
together occupy only one year, and one year is dropped in effect". In a
footnote they record that "some say that the first year is also dropped",
and call this a misunderstanding of the same arrangement. Either way the
first *full* year of a reign, opened by the first Suniā after the
accession, is the 2nd Anka; Wikipedia's table of regnal years puts it the
same way, regnal year 1 against Anka 2 [wikipedia-anka-year]. Sewell and
Dikshit's list of later princes dates exactly that day, "the commencement
of the 2nd Onkos of their respective reigns".

**The New Year.** The Onko reckoning "begins the year on the 12th of
Bhadrapada-suddha, calling that day the 12th not the 1st. In other words,
the year changes its numerical designation every 12th day of
Bhadrapada-suddha", and "its months are purnimanta"; a footnote adds "On
the 11th according to some, but all the evidence tends to shew that the
year begins on the 12th" [sewell1896, Art. 64]. That day is Suniā, the
Odia financial new year [wikipedia-odia-calendar], kept "coinciding with
the Bhadraba Shukla Dwadasi Tithi" [odishatv-sunia-2025] — the same day as
the Vāmana Janma rites at the Jagannātha temple [odishatv-sunia-2024].

**Beyond fifty-nine.** Sewell and Dikshit also record that "the Onko
years are not counted above 59, but the years succeeding 59 begin with a
second series, thus 'second 1', 'second 2'". That is not the Puri count
of today, which has passed 59 without restarting — 67, 68, 69 and 71 —
and it is not carried.

**The Amli year beside it.** The year the Gajapati declares with the Anka,
"1433 Sal" in 2025, is the Amli era of Orissa, which also "commences …
on Bhadrapada sukla 12th", with its epoch in A.D. 592–93
[sewell1896, Arts. 52 and 71]; Wikipedia calls it the Utkaḷiya era,
"began on 592 CE on Bhādra sukḷa dvādasi", 1434 in 2026
[wikipedia-odia-calendar]. Its number is the Gregorian year of its Suniā
less 592. Sewell and Dikshit add that the Amli months are solar, each
beginning "from the moment when the sun enters a new sign"; that calendar
is not this one.

**Worked example.** What is the Anka of 15 September 2024, the day the
2024 article is dated [odishatv-sunia-2024]?

1. The amānta calendar of `hindu-lunar` puts the sunrise tithi of
   15 September 2024 at śukla 12 of nija Bhādrapada, Śaka 1946, and no
   earlier day of that fortnight carries 12: this is Suniā, and a new Anka
   begins.
2. Dibyasingha Deb's first Suniā after his accession in July 1970 fell in
   Śaka 1892, so 1946 opens his 1946 − 1892 + 1 = 55th full year.
3. The 55th number that is not dropped, counting from 2: from 2 to 68 are
   67 numbers, of which 6, 16, 20, 26, 30, 36, 40, 46, 50, 56, 60 and 66
   — twelve — are dropped, leaving 55. The 55th is 68.
4. The Amli year is 2024 − 592 = 1432.

The article's "68 'Anka' … 1432 'Sal'" is the answer. The day before,
14 September, is still the 67th Anka.

## What is carried

- **Identifier** `odia-anka`, `OdiaAnkaCalendar::PURI`: the Anka of
  Dibyasingha Deb over the pūrṇimānta calendar of `hindu-lunar-purnimanta`,
  read at the Central Station's sunrise with the Lahiri ayanamsa, as the
  crate's other lunisolar calendars are. The day begins at sunrise and is
  named by the civil day on whose sunrise it begins, the pūrṇimānta
  calendar's own boundary, `DayBoundary::Sunrise(DayNaming::ByStart)`
  [reingold2018code, `hindu-lunar-from-fixed`]. `OdiaAnkaCalendar::new` takes
  another pūrṇimānta calendar — Puri's sunrise, for a local almanac — and
  another `Reign`.
- **The mapping** as integers, not as a year count:
  `anka_of_regnal_year` and `regnal_year_of_anka` in the module turn the
  number of full years (1 from the first Suniā) into the Anka and back,
  and the second answers `None` for 1, 6, 16, 20 and every other dropped
  number. A dropped number is not a year: `is_leap_year` and `to_fixed`
  refuse it with `YearOutOfRange`. Nothing in the module computes an Anka
  by subtracting one from another.
- **Dates** are the Anka, the pūrṇimānta month numbered from Chaitra as
  `hindu-lunar-purnimanta` numbers it, an intercalary month flagged, and
  the tithi 1–30 with `leap_day` for a repeated one. A year runs from
  Bhādrapada śukla 12 to Bhādrapada śukla 11 a year later, so Bhādrapada
  appears at both ends of it — śukla 12 to 15 at the start, the dark half
  and śukla 1 to 11 at the end — and never with the same tithi twice. Era
  code `dibyasingha-deb`. Two derived extras: `regnal-year`, the full year
  of the reign, and `amli-year`, the Amli year that opened on the same
  Suniā; both are ignored on input.
- **The New Year's day, where the almanac could differ.** Two cases the
  sources read do not settle, and the library's rule for each:
  - *An intercalary Bhādrapada*, in 1974, 1993, 2012, 2031 and 2050 of the
    reign. The year opens in the nija month, as festivals are not marked
    in the adhika month [wikipedia-adhik-maas] and as Drik Panchang puts
    the same tithi's Vāmana Jayanti of 2012 in nija Bhādrapada, on
    26 September at the place it computed for [drik-vamana-jayanti-2012];
    the Central Station's reading gives 27 September. No source read dates
    a Suniā in such a year.
  - *A śukla 12 that holds no sunrise*, as in 1970 at the Central Station.
    The year opens on the first day of the fortnight whose sunrise tithi
    is 12 or later — the day of śukla 13 — so that no tithi of Bhādrapada
    appears twice in one year. An almanac that keeps Suniā on the day in
    which the 12th is current, the day of śukla 11, would open the year a
    day earlier.
- **Range** from Suniā of 1970 (13 September by the rule above), the
  first day of the 2nd Anka, to the pūrṇimānta calendar's last day in
  2300. The days from the accession to that Suniā, the Anka Sewell and
  Dikshit call the 1st, are not carried: the day of the accession rests on
  a source not read, and the fragment is the same days as the last Anka of
  Birakishore Deb, which is not carried either. The reign has not ended,
  so the count runs on after today as the Japanese era of the present
  reign does; when it ends, a new reign is a new `Reign`, and the days
  after it will have been an extension.
- **Not carried.** The Anka of earlier kings — Sewell and Dikshit give the
  first day of the 2nd Anka of Mukundadeva (2 September 1797),
  Ramachandradeva (22 September 1817), Virakesvaradeva (4 September 1854)
  and Divyasimhadeva (8 September 1859), but not the day any reign ended —
  and of the zamindars; the Ganjam series that restarts after 59; the Amli
  and Vilayati calendars with their solar months, which
  [calendars.md](../calendars.md) lists.

## Accuracy

| Check | Source | Test | Result |
| --- | --- | --- | --- |
| The numbers 1, 6, 16, 20, 26, 30, 36, 40, 46, 50, 56 are dropped and 10 is kept; regnal years 1–30 are Anka 2 to 37 | [sewell1896], [wikipedia-anka-year] | `the_dropped_numbers_are_those_sewell_and_dikshit_list`, `the_first_thirty_regnal_years_are_wikipedias_table` | all |
| The mapping and its inverse agree for every regnal year to 1 000 | — | `the_mapping_and_its_inverse_agree` | all |
| Suniā — nija Bhādrapada śukla 12 — on 2 September 1797, 22 September 1817, 4 September 1854 and 8 September 1859, Sewell and Dikshit's first days of the 2nd Onko | [sewell1896] | `suniya_falls_on_sewell_and_dikshits_days` | four of four |
| 52 Anka and 1419 Sal in 2011; 68 and 1432 from 15 September 2024; 69 and 1433 from 4 September 2025; 71 and 1434 in 2026, where 23 September is Suniā | [puriwaves-sunia-52], [odishatv-sunia-2024], [odishatv-sunia-2025], [odishatv-sunia-2026], [kalingatv-sunia-2026] | `the_printed_ankas_of_2011_and_2024_to_2026` | all; the day before each Suniā is the previous Anka |
| Every day of three Anka years round-trips through the date and the fields | — | `every_day_of_three_years_converts_and_converts_back` | all |
| Dropped numbers, out-of-range days and the wrong era are refused | — | `dropped_numbers_and_foreign_eras_are_refused` | all |

The press gives the Suniā day of 2024 and 2025 by the article's date
("today"), which agrees with the computed nija Bhādrapada śukla 12. The
two 2026 articles are dated 23 and 24 September; the computed Suniā is
23 September. Sewell and Dikshit's four dates are the Orissa court's,
reckoned with the old almanac's Sun and Moon at Puri, and the modern
reckoning at the Central Station reproduces all four; a tithi ending
within minutes of sunrise could still move a Suniā by a day between the
two.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [sewell1896] | Art. 64: the Onko reckoning, its dropped numbers, the successor's first Onko, the year from Bhādrapada śukla 12 with pūrṇimānta months, the series restarting after 59, the first days of four reigns' 2nd Onko; Arts. 52 and 71: the Amli year | Yes, 2026-09-26, in the Internet Archive's OCR text |
| [wikipedia-anka-year] | The Eastern Ganga origin; the dropped numbers; regnal year 1 as Anka 2 and the table to regnal year 30; Suniā on Bhādra śukla dvādaśī | Yes, 2026-09-26; its sources (Tripathi 1962, Kulke 1974, Panda 2008) were not read |
| [wikipedia-odia-calendar] | Suniā as the Odia financial new year; the Utkaḷiya era from 592 CE on Bhādra śukla dvādaśī, 1434 in 2026 | Yes, 2026-09-26 |
| [wikipedia-dibyasingha-deba] | Dibyasingha Deb in office from 7 July 1970, succeeding Birakishore Deb | Yes, 2026-09-26; the Sambad article it cites was not read |
| [odishatv-sunia-2024] | "68 'Anka'" and "1432 'Sal'" on Suniā, Bhādrapada śukla 12, 15 September 2024; the Vāmana Janma rites the same day | Yes, 2026-09-26 |
| [odishatv-sunia-2025] | The 69th Anka and 1433 Sal on 4 September 2025, "coinciding with the Bhadraba Shukla Dwadasi Tithi" | Yes, 2026-09-26 |
| [odishatv-sunia-2026] | The 71st Anka and 1434 Sal, article of 24 September 2026 | Yes, 2026-09-26 |
| [kalingatv-sunia-2026] | The 71st Anka, article of 23 September 2026; "numbers ending with 6 and 0 are excluded" | Yes, 2026-09-26 |
| [puriwaves-sunia-52] | "52 anka 1419", undated | Yes, 2026-09-26 |
| [wikipedia-adhik-maas] | Festivals are not marked in the adhika month | Yes, 2026-09-26 |
| [reingold2018code] | `hindu-lunar-from-fixed`: a fixed day's date read at its own sunrise | Not read for this document; the boundary is the pūrṇimānta calendar's, as [hindu-calendars.md](hindu-calendars.md) cites it |
| [drik-vamana-jayanti-2012] | Vāmana Jayanti, Bhādrapada śukla 12, on 26 September 2012 in nija Bhādrapada, computed for Tokyo, where the fetch was placed | Yes, 2026-09-26 |

## Code

`crates/hc-calendars-indic/src/odia_anka.rs` (`OdiaAnkaCalendar`,
`OdiaAnkaDate`, `Reign`, `DIBYASINGHA_DEB`, `anka_of_regnal_year`,
`regnal_year_of_anka`, `suniya_of`) over `hindu_purnimanta.rs` and
`hindu_lunar.rs`. Anchors: `suniya_falls_on_sewell_and_dikshits_days`,
`the_printed_ankas_of_2011_and_2024_to_2026`,
`the_first_thirty_regnal_years_are_wikipedias_table`; the structure:
`the_dropped_numbers_are_those_sewell_and_dikshit_list`,
`the_mapping_and_its_inverse_agree`,
`every_day_of_three_years_converts_and_converts_back`,
`dropped_numbers_and_foreign_eras_are_refused`.
