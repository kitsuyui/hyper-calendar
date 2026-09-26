# The Vira Nirvana Samvat: the Jain era on the amānta months

Backs the identifier `vira-nirvana-samvat` in `hc-calendars-indic`.

## What it is

The Jain era counted from the nirvāṇa of Mahāvīra, the twenty-fourth
Tīrthaṅkara, "a calendar era beginning on 7 October 527 BCE"
[wikipedia-vira-nirvana-samvat]. The *Tiloya-paṇṇatti* of Yati-vṛṣabha
(sixth century) and Jinasena's *Harivaṃśa* (783 CE) give the interval from
the nirvāṇa to the Śaka era as 605 years and 5 months — and the
*Harivaṃśa* 10 days more [wikipedia-vira-nirvana-samvat, citing
sircar1965, not read]. Jain calendars print the year beside the Vikram
Samvat today: the Oshwal Association of the United Kingdom's calendar for
2025 is headed "Vir Samvat 2551 | Vikram Samvat 2081" until Diwali and
"Vir Samvat 2552 | Vikram Samvat 2082" after it [oshwal-2025].

**Two readings of the nirvāṇa, one era.** The *Tiloya-paṇṇatti* itself
records other intervals beside 605 years — 461, 9 785 and 14 793 years —
and Sagarmal Jain argues from Mahāvīra's relation to Candragupta Maurya
and from the inscriptions for 467 BCE, while noting that scholars of both
traditions, the Digambara Jugal Kishore Mukhtar and the Śvetāmbara
Muni Kalyāṇa Vijaya, "have also upheld this 527 B.C. date"
[jain-sagarmal-nirvana]. The roadmap had planned a second calendar for a
Digambara epoch of 662 BCE; no source read gives that date, or any
Digambara era distinct from the one above, so there is one identifier and
not two ([policy.md §5](../policy.md) names a disagreement, and none was
found). 510 BCE is given for the Digambara on web pages that cite
nothing, and is not carried.

## How it works

**The year.** The year begins on Kārtika śukla pratipadā, the first day of
the bright fortnight of Kārtika, the day after Dīpāvalī, the night of the
nirvāṇa: the Oshwal calendar's "New Year's Day, 22 Wednesday, Kartik Sud
Ekam" of 2025, two days after "Mahavirswami Nirvan Kalyanak & Diwali" on
the 20th [oshwal-2025]. The year number moves on "with Kartika Krishna to
Kartika Shukla transition", and is the Kārtikādi Vikram Samvat plus 470
[wikipedia-vira-nirvana-samvat].

**The months** are the amānta months, new moon to new moon, as
[hindu-calendars.md](hindu-calendars.md) sets them out: Kārtika to
Āśvina, each named by the saṅkrānti it holds, with an intercalary month
where one holds none. The Oshwal calendar dates its festivals by their
tithis in those months — "Aaso Vad Chaudas" is the fourteenth of the dark
half of Āśvina, the amānta month that ends at the Dīpāvalī new moon.

**Against the Śaka year.** The *Rashtriya Panchang*'s Śaka year begins at
Chaitra, five months after Kārtika. So the Kārtika of Śaka year *s* opens
Vira Nirvana Samvat *s* + 605, and Chaitra to Āśvina, which begin Śaka
*s* + 1, are still in it: the Vira Nirvana year is the Śaka year plus 605
from Kārtika to Phālguna and plus 604 from Chaitra to Āśvina, which is the
605 years and 5 months of the sources.

**Worked example.** What is the Vira Nirvana date of 10 April 2025, the
Oshwal calendar's "Mahavirswami Janma Kalyanak, Chaitra Sud Teras"? The
*Rashtriya Panchang*'s reckoning puts that day in Chaitra of Śaka 1947,
tithi 13, the thirteenth of the bright half. Chaitra is before Kārtika, so
the year is 1947 + 604 = 2551; Chaitra is the sixth month counted from
Kārtika. The date is 13 Chaitra 2551, which is how the calendar heads it.

## What is carried

- **Identifier** `vira-nirvana-samvat`, `ViraNirvanaCalendar::RASHTRIYA`:
  the amānta calendar of `hindu-lunar` read at the Central Station's
  sunrise with the Lahiri ayanamsa, the *Rashtriya Panchang*'s reckoning.
  The day begins at sunrise and is named by the civil day on whose
  sunrise it begins, `DayBoundary::Sunrise(DayNaming::ByStart)`, as the
  amānta calendar's is [reingold2018code, `hindu-lunar-from-fixed`].
  `ViraNirvanaCalendar::new` takes any other amānta calendar — another
  city's sunrise, for a local almanac.
- **Months** 1 for Kārtika through 12 for Āśvina, an intercalary month as
  `Month::leap(n)` before the ordinary one; the day is the tithi, 1 to 30,
  with `leap_day` for a repeated one, exactly as `hindu-lunar` has it. Era
  code `vira-nirvana`. English names are the *Rashtriya Panchang*'s
  transliteration used for `hindu-lunar`, in the Jain year's order; the
  Oshwal calendar's Gujarati forms (Kartik, Posh, Maha, Fagan, Chaitra,
  Vaishakh, Jeth, Ashadh, Shravan, Bhadarvo, Aaso) are not used because it
  prints no Mārgaśīrṣa. The Devanagari names are the module's `MONTHS`.
- **Range** the amānta engine's, Chaitra śukla 1 of 1700 to the day
  before that of 2300.
- **Shared arithmetic.** The Kārtikādi year over the amānta months is the
  same for Nepal Sambat, which differs only in its offset and its sunrise;
  both use the crate's `kartikadi` module.
- **Not carried.** A Digambara epoch, for the reason above; the scholarly
  redatings of the nirvāṇa, which no calendar is dated in; the Jain
  festivals as named days, which are the dates the tests read; the
  observance rules — a festival kept on the day its tithi holds at a given
  hour — which belong to a holiday rule.

## Accuracy

| Check | Source | Test | Result |
| --- | --- | --- | --- |
| New Year's Day of 2552 is Kārtika śukla 1, 22 October 2025, and the day before is in Āśvina 2551 | [oshwal-2025] | `new_years_day_2552_was_kartika_shukla_1_on_22_october_2025` | yes |
| Pauṣa śukla 15 on 13 January 2025, Chaitra śukla 13 on 10 April, Bhādrapada śukla 4 on 27 August, all in 2551; Kārtika śukla 15 on 5 November 2025 in 2552 | [oshwal-2025] | `the_festivals_of_2025_fall_in_2551_on_the_printed_tithis` | all four |
| 2544 began at the Diwali of 2017 | [wikipedia-vira-nirvana-samvat] | `year_2544_began_at_the_diwali_of_2017` | yes |
| The year is the Śaka year plus 605 from Kārtika and plus 604 before, with the amānta tithi unchanged, over four years | the rule | `the_year_is_the_saka_year_plus_605_from_kartika_and_604_before` | every fifth day |
| Every day of 2549–2551 round-trips; the adhika Śrāvaṇa of 2023 falls in 2549 | — | `every_day_of_three_years_converts_and_converts_back` | all |

The Oshwal calendar is published in the United Kingdom and does not say
for which almanac or place its tithis are reckoned; the days checked agree
with the Central Station's reckoning, but a tithi that ends near sunrise
can be read a day apart by an almanac computed elsewhere, as
[hindu-calendars.md](hindu-calendars.md) describes for the engine.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wikipedia-vira-nirvana-samvat] | The epoch of 527 BCE; the 605 years and 5 months of the *Tiloya-paṇṇatti* and the *Harivaṃśa*; the Kārtikādi Vikram Samvat plus 470; the year turning at Kārtika; 2544 from the Diwali of 2017 | Yes, 2026-09-26 |
| [sircar1965] | The same, as Wikipedia cites it | Not read |
| [jain-sagarmal-nirvana] | The other intervals of the *Tiloya-paṇṇatti*; the argument for 467 BCE; Mukhtar and Kalyāṇa Vijaya upholding 527 BCE | A summary of the paper, 2026-09-26; the paper itself was not read |
| [oshwal-2025] | The year numbers 2551 and 2552, New Year's Day 2025 and the festivals' tithis | Yes, the PDF, 2026-09-26 |
| [reingold2018code] | `hindu-lunar-from-fixed`: a fixed day's date read at its own sunrise | Yes, 2026-09-26 |

## Code

`crates/hc-calendars-indic/src/vira_nirvana.rs` (`ViraNirvanaCalendar`,
`ViraNirvanaDate`, `SAKA_OFFSET`, `MONTHS`) over `kartikadi.rs` and
`hindu_lunar.rs`. Anchors:
`new_years_day_2552_was_kartika_shukla_1_on_22_october_2025`,
`the_festivals_of_2025_fall_in_2551_on_the_printed_tithis`,
`year_2544_began_at_the_diwali_of_2017`; the structure:
`the_year_is_the_saka_year_plus_605_from_kartika_and_604_before`,
`every_day_of_three_years_converts_and_converts_back`.
