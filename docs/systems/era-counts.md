# Year counts over another calendar's year

Backs the identifiers `spanish-era`, `masonic-anno-lucis`,
`masonic-anno-inventionis`, `masonic-anno-depositionis`,
`masonic-anno-ordinis`, `ada`, `philip-era`, `bostran-era` and
`era-fascista` in `hc-calendars-solar`.

## What it is

Each of these is a year number written on another calendar's days. None
has months of its own except the Bostran era, whose Macedonian months are
laid on the Julian year; what each adds is an epoch, a year boundary and
the question of who wrote it when.

* **The Spanish era**, *Æra Hispanica*, counted from "1 January 38 BC"
  and was "used in medieval Iberia", the reason for 38 BC "unclear"
  [wikipedia-spanish-era]. It "is found in inscriptions and was current
  with the chroniclers on the peninsula … used in Spain as late as the
  14th century and in Portugal until 1422, when it was officially
  abandoned" [grumel-eras-historical]. The kingdoms dropped it one by one:
  Catalonia in 1180, Aragon in 1349/1350, Valencia in 1358, Castile in
  1382/1383, Portugal in 1420/1422, Navarre in the early fifteenth century
  [wikipedia-spanish-era].
* **The Masonic years** are one count per rite: *Anno Lucis* of the
  Ancient Craft Masons, the common year plus 4000; *Anno Inventionis* of
  the Royal Arch, plus 530; *Anno Depositionis* of the Royal and Select
  Masters, plus 1000; *Anno Ordinis* of the Knights Templar, less 1118
  [lodge43-masonic-calendar]. Anno Lucis "was adopted in the 18th century
  as a simplification of the Anno Mundi era" [wikipedia-anno-lucis]. The
  Scottish Rite's *Anno Mundi* is the Hebrew year, beginning in September,
  and is `hebrew`'s [lodge43-masonic-calendar].
* **After the Development of Agriculture**: "in 1978, artist and
  intellectual Merlin Stone advocated that feminists adopt a new dating
  system, according to which 1978 was 9978 ADA" [wikipedia-ada].
* **The Era of Philip**: "in the *Handy Tables* Ptolemy uses as epoch the
  era of Philip (noon, −323 November 12) and not the era of Nabonassar
  (noon, −746 February 26), as he did in the *Almagest*" [chabas2013].
* **The Bostran era** "commemorated the Emperor Trajan's establishment of
  Arabia as a province. Its point of departure was March 22, a.d. 106"
  [grumel-eras-historical]. Its oldest inscription is of AD 107, and it
  was written "as late as AD 735", "almost never identified explicitly" in
  the later period [wikipedia-bostran-era].
* **The Era Fascista** was "introduced in 1926 (Anno IV) and officialized
  in 1927 (Anno V)", "abandoned in most of Italy with the fall of the
  Fascist regime in 1943 (Anno XXI), but continued to be used in the rump
  Republic of Salò until the death of Mussolini in April 1945 (Anno
  XXIII)" [wikipedia-era-fascista].

## How it works

**The pure offsets.** The Spanish era is the Julian year plus 38 — "Era
941 would be equivalent to AD 903" [wikipedia-spanish-era] — from 1 January,
the Julian days and leap years unchanged. The Masonic years and ADA are the
Gregorian year plus 4000, 530, 1000, −1118 and 8000: 2010 is "6010 A.L.",
"2540 A.I.", "3010 A:.Dep:" and "892 A.O." [lodge43-masonic-calendar], and
2026 is AL 6026 and 10026 ADA [wikipedia-anno-lucis, wikipedia-ada]. None
of the sources for the Masonic years or ADA names a new year other than
the common one, so the count changes on 1 January; a rite that keeps
another would be its own identifier.

**The Era of Philip.** The same Egyptian wandering year as `egyptian`,
counted from another 1 Thoth. From 26 February 747 BC to 12 November 324 BC
is 154 760 days, exactly 424 years of 365, so 1 Thoth of Philip 1 is
1 Thoth of Nabonassar 425 and every day keeps its month and day. *Worked
example*: Censorinus's 1 Thoth of 887 Nabonassar, 20 July AD 139, Julian
Day 1 772 028 [richards2013, §15.2.1], is 1 Thoth 463 of Philip.

**The Bostran era.** "Twelve months of 30 days with five epagomenal days
at the end of the year"; "the first day of the first month, Xanthikos,
corresponded to 22 March in the Julian calendar"; "a leap year came once
every four years in the Bostran calendar starting from the second year.
Thus years 2, 6, 10 etc. were leap years with a sixth epagomenal day"
[wikipedia-bostran-era]. The three statements fit: year *N* runs from
22 March of AD 105 + *N*, and contains the Julian 29 February of AD 106 +
*N*, which is a leap year exactly when *N* is 2 modulo 4. *Worked example*:
year 2 begins on 22 March 107; its twelve months end on 15 March 108,
because 29 February 108 falls inside Dystros; its six epagomenal days are
16 to 21 March; and year 3 begins on 22 March 108. The months are the
Macedonian ones in their order from Xanthikos: Xanthikos, Artemisios,
Daisios, Panemos, Loios, Gorpiaios, Hyperberetaios, Dios, Apellaios,
Audnaios, Peritios, Dystros [wikipedia-ancient-macedonian-calendar].

**The Era Fascista.** "Day 1 of Anno I … corresponded to 29 October 1922",
and each Anno begins on the anniversary [wikipedia-era-fascista]. *Worked
example*: 28 October 1936 is 1936 − 1922 = Anno XIV; 29 October 1936 is
1936 − 1921 = Anno XV. October is therefore split: its 29th to 31st open
an Anno and its 1st to 28th close the one before.

## What is carried

| Identifier | Base | Year | Range | Usage |
| --- | --- | --- | --- | --- |
| `spanish-era` | Julian | + 38, from 1 January | Era 1 (38 BC) on | Unrecorded as days; the kingdoms' years as data, `year_counts::SPANISH_ERA_ABANDONMENT` |
| `masonic-anno-lucis` | Gregorian | + 4000 | A.L. 1 on | Attested, undated |
| `masonic-anno-inventionis` | Gregorian | + 530 | A.I. 1 on | Attested, undated |
| `masonic-anno-depositionis` | Gregorian | + 1000 | A.Dep. 1 on | Attested, undated |
| `masonic-anno-ordinis` | Gregorian | − 1118 | A.O. 1, 1119, on | Attested, undated |
| `ada` | Gregorian | + 8000 | ADA 1 on | Unrecorded: a proposal |
| `philip-era` | Egyptian wandering year | − 424 from Nabonassar | Philip 1 (324 BC) to `egyptian`'s end | Unrecorded: no span in the sources |
| `bostran-era` | Julian days, Macedonian months | from 22 March 106 | Years 1–9 999 | Unrecorded as days; attested 107–735 |
| `era-fascista` | Gregorian | from 29 October 1922 | Anno I–XXIII, to 28 October 1945 | Unrecorded as days; written Anno IV–XXI, XXIII in Salò |

The six pure offsets are one table, `year_counts::ALL`, read by one
calendar type; the Holocene, Minguo and Juche years share their arithmetic
through `common::offset_to_fixed` and keep their own modules for their own
date types. Every count starts at its year 1 and refuses the years before
it rather than write a number nobody wrote.

**Why years and not days.** A period of use is a pair of days. The sources
for the Spanish era, the Bostran era and the Era Fascista give years —
"1349/1350", "as late as AD 735", "in April 1945" — and a year is not a
day, so these three record no period (`tests/usage.rs` lists them) and
carry the years as data or constants instead: `SPANISH_ERA_ABANDONMENT`
with `Abandonment::abandoned_by`, which answers `None` for a year the
source leaves open; `era_fascista::FIRST_WRITTEN_ANNO`,
`LAST_ANNO_IN_ITALY` and `LAST_ANNO`.

**Not carried.** The Spanish era's year beginning at 25 December once the
Anno Domini came in [wikipedia-spanish-era], which would be its own
identifier and has no dated source; Ptolemy's day from noon, which the
Egyptian calendars here do not model either; the other provincial eras of
the *hemerologia*; a Masonic year boundary other than 1 January.

## Accuracy

Every conversion is exact integer arithmetic over its base calendar.

| Check | Test | Result |
| --- | --- | --- |
| Era 941 is AD 903; the epoch is 1 January 38 BC | `the_spanish_era_is_the_julian_year_plus_thirty_eight` | Holds |
| The kingdoms' years, and the order the source gives them in | `the_kingdoms_dropped_the_spanish_era_in_the_years_the_source_gives` | Holds |
| The Lodge's examples for 2010, and AL 6026 for 2026 | `the_masonic_years_are_the_lodges_worked_examples` | 5 of 5 |
| 1978 is 9978 ADA and 2026 is 10026 | `ada_is_the_common_year_plus_eight_thousand` | Holds |
| The Era of Philip begins on 12 November 324 BC, Nabonassar 425 | `the_epoch_is_the_twelfth_of_november_324_bc` | Holds |
| … keeps every Egyptian month and day; Censorinus's day is 1 Thoth 463 | `a_day_keeps_its_egyptian_month_and_day_and_loses_424_years` | Holds |
| 1 Xanthikos is 22 March in every Bostran year | `every_new_year_is_the_twenty_second_of_march` | 9 999 of 9 999 |
| The sixth epagomenal day is the day before 1 Xanthikos | `the_sixth_epagomenal_day_is_the_day_before_the_new_year` | Holds |
| Anno I from 29 October 1922; the source's coin and sundial; Anni IV, XXI and XXIII in 1926, 1943 and April 1945 | `anno_one_begins_on_the_twenty_ninth_of_october_1922`, `the_dated_objects_carry_the_anno_the_rule_gives` | Holds |

The Spanish era page's own second example, a document of 1137 dated "Era
millesima centesima LXXVI", is 39 years apart, not 38. A year of the
Incarnation begun on 25 March would put a January to March date of 1138 in
1137 and explain it, but the page does not say which style the document
used, so it is noted here and is not an anchor.

**What a primary source would settle.** The Bostran leap day rests on one
secondary source, which cites Mercier's study of 2001 [wikipedia-bostran-era]
and calls the calendar lunisolar while describing a solar one; the
arithmetic here is the one consistent reading of its three statements, and
Mercier, or the *hemerologia*, would confirm where the sixth day stands.
The Era Fascista's decrees of 1926 and 1927 would date its first use to the
day, and the instruments each kingdom dropped the Spanish era by would date
those changes; none was read.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [wikipedia-spanish-era] | The epoch, the offset and Era 941, the kingdoms' years, the year from 1 January and later 25 December, the 1137 document | Yes, 2026-09-26 |
| [grumel-eras-historical] | The Spanish era in Spain and Portugal; the Bostran epoch | Yes, 2026-09-26 |
| [lodge43-masonic-calendar] | The four Masonic years, their rites and the examples for 2010 | Yes, 2026-09-26 |
| [wikipedia-anno-lucis] | Anno Lucis in the 18th century and AL 6026 | Yes, 2026-09-26 |
| [wikipedia-ada] | Merlin Stone's proposal and its two examples | Yes, 2026-09-26; Stone's own writing was not read |
| [chabas2013] | The epochs of Philip and Nabonassar | Yes, 2026-09-26; Tihon and Mercier's edition was not read |
| [richards2013] | Censorinus's 1 Thoth 887 Nabonassar, 20 July 139, JDN 1 772 028 | Yes, 2026-09-26 |
| [wikipedia-bostran-era] | The months, the epagomenal days and the leap years; the attestations | Yes, 2026-09-26; Mercier 2001 was not read |
| [wikipedia-ancient-macedonian-calendar] | The Macedonian months, their order and spelling | Yes, 2026-09-26 |
| [wikipedia-era-fascista] | The epoch, the years of use, the coin and the sundial | Yes, 2026-09-26; the decrees were not read |

## Code

`crates/hc-calendars-solar/src/year_counts.rs`,
`crates/hc-calendars-solar/src/philip_era.rs`,
`crates/hc-calendars-solar/src/bostran.rs` and
`crates/hc-calendars-solar/src/era_fascista.rs`; the anchors are the tests
named above.
