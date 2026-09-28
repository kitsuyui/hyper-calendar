# Year counts over another calendar's year

Backs the identifiers `spanish-era`, `masonic-anno-lucis`,
`masonic-anno-inventionis`, `masonic-anno-depositionis`,
`masonic-anno-ordinis`, `ada`, `roman-auc-capitoline`,
`cheondogyo-podeok`, `persian-imperial`, `philip-era`, `bostran-era`,
`era-fascista`, `masonic-anno-lucis-march`, `egyptian-ptolemy` and
`philip-era-ptolemy` in `hc-calendars-solar`, and `huangdi-era`,
`huangdi-era-tongmenghui`, `huangdi-era-liu-shipei` and
`huangdi-era-jiangsu` in `hc-calendars-regional`.

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
* **The Capitoline count *ab urbe condita*** is the count of the *fasti
  Capitolini*, one year behind Varro's: "AUC 1 in Augustus' fasti is
  753/752 BC", and the fasti "give to the start of the republic a date of
  244 AUC" where Varro has 245 [wikipedia-fasti]. Solinus used it: the
  consuls C. Pompeius Gallus and Q. Veranius held office "urbis conditae
  anno octingentesimo primo", in Olympiad 207, and Rome was founded in the
  first year of Olympiad 7 [solinus-mommsen1895]. Varro's count is
  `roman-auc`.
* **The Cheondogyo year, 포덕 (布德)**, counts from 1860, the year Choe
  Je-u received his revelation at Yongdamjeong, and is still used within
  the religion [wikipedia-ko-cheondogyo]. The church's newspaper dates its
  articles by it: "포덕 167년(2026) 7월 19일" [chondogyo-sinmun-2026].
* **The Iranian imperial year**: "On March 10, 1976 (20 Esfand 1354), Shah
  Mohammad Reza Pahlavi introduced the 'Imperial calendar' that measured
  the first year from 559 BC", the beginning of Cyrus's reign; it was
  "reversed … on September 2, 1978 (11 Shahrivar 2537, which became
  11 Shahrivar 1357)" [wikipedia-iranian-calendars].
* **The years of the Yellow Emperor, 黃帝紀元**, were the count of the
  reformers and revolutionaries of the last Qing decade, against the reign
  and against 康有為's 孔子紀年. They did not agree on the epoch: 嚴復's
  1898 is 4386, 劉師培's 1903 is 4614, 《江蘇》's 1903 is 4394, the
  同盟會's 1908 is 4605, and 宋教仁's 1905 is 4603, which the
  revolutionaries adopted and Sun Yat-sen's telegram used
  [zhang-xinbin-huangdi]. The Hubei Military Government dated its gazette
  by 宋教仁's count after the Wuchang rising [wikipedia-zh-huangdi-era],
  and Sun ended the count: "以黄帝纪年四千六百九年十一月十三日为中华民国元旦"
  [zhang-xinbin-huangdi]. The 同盟會's 4605 is the count of its address at
  the Yellow Emperor's tomb, `huangdi-era-tongmenghui`; its paper 《民報》
  counted as 宋教仁 did, and Wikipedia (zh)'s table heads that 2698 BC
  column with the paper's name, so the paper's years are `huangdi-era`
  [wikipedia-zh-huangdi-era].
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
of these sources names a new year other than the common one, so the
count changes on 1 January.

**The year of light from 1 March.** French Freemasonry kept another:
"L'année maçonnique a la même longueur que l'année grégorienne, mais
débute le 1er mars", the Gregorian year plus 4000 with the months known by
number [frwiki-calendrier-maconnique]; "le premier mois de l'année
maçonnique en cours est mars", January and February being "le 11ème mois
de l'année précédente" and "le douzième" [rer-calendrier-maconnique].
Bernheim calls it "the March = 1st month code", "frequently used on
documents issued in France or by French Brethren", and the classical code
of eighteenth-century French Freemasonry [bernheim1986, §1.2.1]. It is
`masonic-anno-lucis-march`, a competing convention of
`masonic-anno-lucis` (policy §5). *Worked example*: 21 February 1802 is in
the Masonic year that began on 1 March 1801, 1801 + 4000 = 5801, whose
twelfth month is February: "le 21e Jour du 12e mois de l'an Maçonnique
5801 ... means 21 February 1802, not 1801" [bernheim1986]. Bernheim's
"June = 1st month code", from the regulations of one lodge at
Saint-Pierre de la Martinique of 1750 and "very seldom used", is not
carried: he gives no day for its first month to begin on.

**Ptolemy's day.** The *Almagest*'s era of Nabonassar begins "at noon on
26 February 747 BC" [wikipedia-nabonassar], and "the astronomical day had
begun at noon ever since Ptolemy chose to begin the days for his
astronomical observations at noon", at "the meridian of Alexandria"
[wikipedia-julian-day, quoting Herschel]; in Ptolemy's time "the Egyptian
calendar day started at dawn" [vangent-almagest]. `egyptian` and
`philip-era` keep the civil midnight, and `egyptian-ptolemy` and
`philip-era-ptolemy` are the same days under Ptolemy's
`DayBoundary::Noon(DayNaming::ByStart)`: the day 1 Thoth 1 runs from the
noon of the civil 1 Thoth 1, 26 February 747 BC, to the next noon, as the
Julian Day 1 448 638 does. The naming follows from the era's beginning at
the noon of its first civil day and from van Gent's calculator, which
reads a time as "local time since noon" of its Egyptian date; no source
read states it in words.

**The 포덕 year.** The Gregorian year less 1859, from 1 January. The
newspaper's Sunday sermons of "포덕 166년 12월 28일" and "포덕 167년 1월
11일" are Sundays on the Gregorian calendar, 28 December 2025 and
11 January 2026, so the months are the Gregorian ones and the year turns
between them [chondogyo-sinmun-2026]. *Worked example*: 19 July 2026 is
2026 − 1859 = 포덕 167.

**The imperial year.** The Solar Hijri year plus 1180 on the Solar Hijri
days: the source's table pairs 1355 with 2535, 21 March 1976 to 20 March
1977 [wikipedia-iranian-calendars]. Its sentence "the year changed from
1355 to 2535" cannot be taken literally, since on 10 March 1976 the year
was 1354: that day, 20 Esfand 1354, became 20 Esfand 2534. *Worked
example*: 2 September 1978 is 11 Shahrivar 1357, so 11 Shahrivar
1357 + 1180 = 2537, the date the source gives for the reversal.

**The years of the Yellow Emperor.** Each count is the Chinese lunisolar
year renamed, turning at 正月初一: the Wikipedia table gives each count's
year for 1903–1911 "農曆新年前" and "農曆新年後" [wikipedia-zh-huangdi-era],
and the dated documents are dated in lunar months. *Worked example*: the
Chinese year that began on 30 January 1911 is 4548 in the continuous count
from 2637 BC, and 4609 by 宋教仁's, 61 more; its eleventh month began on
20 December 1911, so 4609年11月13日 is 1 January 1912, the Republic's first
day. 劉師培 signed his essay "黄帝降生四千六百一十四年闰五月十七日"
[zhang-xinbin-huangdi]; 1903 is 4614 by his count, and the Chinese year of
1903 has a leap fifth month, so his date exists.

**The Capitoline count.** The Julian year plus 752, from 1 January. *Worked
example*: Gallus and Veranius were consuls in AD 49 [wikipedia-fasti],
which Solinus calls AUC 801 [solinus-mommsen1895]; 49 + 752 = 801. Varro's
count makes the same year 802. Solinus checks his figure by the
Olympiads: 206 Olympiads of four years, plus one, less the 24 years of the
first six Olympiads, leaves 801. The Olympiad years begin in summer and
the AUC years in January, so the check is by the year, not the day.

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
| `roman-auc-capitoline` | Julian | + 752, from 1 January | AUC 1 (752 BC) on | Unrecorded, as `roman-auc` is |
| `cheondogyo-podeok` | Gregorian | − 1859, from 1 January | 포덕 1 (1860) on | Attested, undated |
| `persian-imperial` | Solar Hijri, 33-year rule | + 1180, from 1 Farvardin | 1181 (A.P. 1) to the base's end | 10 March 1976 to 1 September 1978 |
| `huangdi-era` | Chinese lunisolar | + 61 on the continuous count, from 正月初一 | 1645 to 2150, `chinese`'s | Unrecorded: dated by documents |
| `huangdi-era-tongmenghui` | Chinese lunisolar | + 60 | 1645 to 2150 | Unrecorded |
| `huangdi-era-liu-shipei` | Chinese lunisolar | + 74 | 1645 to 2150 | Unrecorded |
| `huangdi-era-jiangsu` | Chinese lunisolar | − 146 | 1645 to 2150 | Unrecorded |
| `philip-era` | Egyptian wandering year | − 424 from Nabonassar | Philip 1 (324 BC) to `egyptian`'s end | Unrecorded: no span in the sources |
| `bostran-era` | Julian days, Macedonian months | from 22 March 106 | Years 1–9 999 | Unrecorded as days; attested 107–735 |
| `era-fascista` | Gregorian | from 29 October 1922 | Anno I–XXIII, to 28 October 1945 | Unrecorded as days; written Anno IV–XXI, XXIII in Salò |
| `masonic-anno-lucis-march` | Gregorian days, months numbered from March | + 4000, from 1 March | A.L. 1 (from 1 March 3999 BC) on | Attested, undated |
| `egyptian-ptolemy` | `egyptian`, the day from noon at Alexandria | as `egyptian` | as `egyptian` | Unrecorded |
| `philip-era-ptolemy` | `philip-era`, the day from noon at Alexandria | as `philip-era` | as `philip-era` | Unrecorded |

The nine pure offsets on a solar year are one table, `year_counts::ALL`,
read by one calendar type, whose `Base` is the Julian, the Gregorian or
the 33-year Solar Hijri calendar, the last being the equinox calendar in
force from A.P. 1178 to 1634 [heydari-malayeri2004] and so over the whole
period of the imperial year. The imperial year begins where its base
does, at 1181, since the 33-year calendar has no year before A.P. 1. The
four counts of the Yellow Emperor are one table, `huangdi::ALL`, over
`chinese`, and convert what it converts; the Holocene, Minguo and Juche years share their arithmetic
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

**Not carried.** 嚴復's "开国自黄帝至今四千三百八十六年", a count of years
elapsed in one article and not a dating [zhang-xinbin-huangdi]; the
Wikipedia table's rows from 2997 BC and 2999 BC, which cite nothing, and
its epochs in years BC for 劉師培's and 《江蘇》's counts, which contradict
its own year numbers and 張新斌's, which are followed
[wikipedia-zh-huangdi-era]; 孔子紀年, year 1 in 551 BC, whose year boundary
no source read gives [wikipedia-zh-kongzi-era]. The Spanish era's year beginning at 25 December once the
Anno Domini came in [wikipedia-spanish-era], which would be its own
identifier and has no dated source; the other provincial eras of the
*hemerologia* — Tyre, Sidon, Heliopolis, Lycia, Crete, Cyprus, Ephesus,
Bithynia, Gaza's own, Ascalon and Seleucia Pieria among the sixteen
calendars the Florence, Leiden and Vatican manuscripts tabulate
[wikipedia-bostran-era, citing Samuel 1972] — whose epochs, new years and
leap days no readable source gives: Kubitschek's edition (1915), Samuel's
*Greek and Roman Chronology* (1972, pp. 171–178) and Bultrighini's chapter
of 2021, which is read only as a PDF, would give them; the June code of
the Masonic year, above.

## Accuracy

Every conversion is exact integer arithmetic over its base calendar.

| Check | Test | Result |
| --- | --- | --- |
| Era 941 is AD 903; the epoch is 1 January 38 BC | `the_spanish_era_is_the_julian_year_plus_thirty_eight` | Holds |
| The kingdoms' years, and the order the source gives them in | `the_kingdoms_dropped_the_spanish_era_in_the_years_the_source_gives` | Holds |
| The Lodge's examples for 2010, and AL 6026 for 2026 | `the_masonic_years_are_the_lodges_worked_examples` | 5 of 5 |
| 1978 is 9978 ADA and 2026 is 10026 | `ada_is_the_common_year_plus_eight_thousand` | Holds |
| Solinus's consuls of AD 49 in AUC 801; one year less than `roman-auc` everywhere | `solinus_puts_the_consuls_of_ad_49_in_auc_801` | Holds |
| 포덕 166 on 28 December 2025, 167 on 11 January and 19 July 2026, all Sundays | `the_podeok_year_turns_on_the_first_of_january` | 3 of 3 |
| 2535 from 21 March 1976; 20 Esfand 2534 on 10 March 1976; 11 Shahrivar 2537 on 2 September 1978 | `the_imperial_year_is_the_solar_hijri_year_plus_1180` | Holds |
| 4609年11月13日 is 1 January 1912, 4609年9月10日 31 October 1911, 1905 is 4603 | `sun_yat_sen_made_4609_11_13_the_first_day_of_the_republic` | Holds |
| 劉師培's 4614年閏五月十七日 exists and is in 1903 | `liu_shipei_signed_in_the_leap_fifth_month_of_4614` | Holds |
| 《江蘇》's 1903 as 4394, the 同盟會's 1908 as 4605, the table's 1903 as 4600 and 4601 | `the_other_counts_are_the_sources_years` | Holds |
| Each count is `chinese` renamed on a sample of days and on every new year and its eve, 1645–2150 | `every_count_is_the_chinese_calendar_renamed` | Holds |
| The Era of Philip begins on 12 November 324 BC, Nabonassar 425 | `the_epoch_is_the_twelfth_of_november_324_bc` | Holds |
| … keeps every Egyptian month and day; Censorinus's day is 1 Thoth 463 | `a_day_keeps_its_egyptian_month_and_day_and_loses_424_years` | Holds |
| 1 Xanthikos is 22 March in every Bostran year | `every_new_year_is_the_twenty_second_of_march` | 9 999 of 9 999 |
| The sixth epagomenal day is the day before 1 Xanthikos | `the_sixth_epagomenal_day_is_the_day_before_the_new_year` | Holds |
| Anno I from 29 October 1922; the source's coin and sundial; Anni IV, XXI and XXIII in 1926, 1943 and April 1945 | `anno_one_begins_on_the_twenty_ninth_of_october_1922`, `the_dated_objects_carry_the_anno_the_rule_gives` | Holds |
| The year of light from 1 March: 28 February 2019 is 28/12/6018 and 1 March 2019 1/1/6019; 21/12/5801 is 21 February 1802; the 12th month of 5772 is February 1773; 1 January 1872 is 1/11/5871; 3 January 2017 is the 3rd of the 11th month | `the_sources_dates_decode_as_they_say` | 6 of 6 |
| … and `masonic-anno-lucis`'s year from March to December, a year less and the month ten more in January and February | `the_year_agrees_with_anno_lucis_from_march_to_december` | Holds |
| Ptolemy's day begins at the noon of its civil day and keeps the morning after; the dates are `egyptian`'s and `philip-era`'s | `the_day_begins_at_noon_and_is_named_by_its_start`, `the_dates_are_the_civil_calendars` | Holds |

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
| [lodge43-masonic-calendar] | The four Masonic years, their rites and the examples for 2010 | Yes, 2026-09-26; re-read 2026-09-29, no new year other than the common one stated |
| [frwiki-calendrier-maconnique] | The year of light from 1 March, its numbered months, the examples of 2019 | Yes, 2026-09-29, secondary |
| [rer-calendrier-maconnique] | March the first month, January and February the eleventh and twelfth of the year before, 3 January 2017 | Yes, 2026-09-29 |
| [bernheim1986] | "The March = 1st month code", its use in France and its decodings; the June code of Martinique, 1750 | Yes, 2026-09-29, in the Wayback Machine's HTML copy |
| [wikipedia-julian-day] | Ptolemy's day from noon and the meridian of Alexandria, quoting Herschel | Yes, 2026-09-29; Toomer's *Almagest*, which it cites, was not read |
| [vangent-almagest] | The Egyptian civil day from dawn in Ptolemy's time, the epoch at mean noon at Alexandria, the calculator's "local time since noon" | Yes, 2026-09-29 |
| [wikipedia-anno-lucis] | Anno Lucis in the 18th century and AL 6026 | Yes, 2026-09-26 |
| [wikipedia-ada] | Merlin Stone's proposal and its two examples | Yes, 2026-09-26; Stone's own writing was not read |
| [chabas2013] | The epochs of Philip and Nabonassar | Yes, 2026-09-26; Tihon and Mercier's edition was not read |
| [richards2013] | Censorinus's 1 Thoth 887 Nabonassar, 20 July 139, JDN 1 772 028 | Yes, 2026-09-26 |
| [wikipedia-bostran-era] | The months, the epagomenal days and the leap years; the attestations | Yes, 2026-09-26; Mercier 2001 was not read |
| [wikipedia-ancient-macedonian-calendar] | The Macedonian months, their order and spelling | Yes, 2026-09-26 |
| [wikipedia-fasti] | The Capitoline count, AD 49 as the consulship of Gallus and Veranius, AUC 802 by Varro | Yes, 2026-09-27; Greswell 1854, which it cites, was not read |
| [solinus-mommsen1895] | Solinus 1.29–30: AUC 801, Olympiad 207, the foundation in Olympiad 7.1 | Yes, 2026-09-27 |
| [wikipedia-era-fascista] | The epoch, the years of use, the coin and the sundial | Yes, 2026-09-26; the decrees were not read |
| [wikipedia-ko-cheondogyo] | 1860 as 포덕 원년, its use within the religion | Yes, 2026-09-28 |
| [chondogyo-sinmun-2026] | Articles dated 포덕 166년 12월 28일, 포덕 167년 1월 11일 and 포덕 167년(2026) 7월 19일 | Yes, 2026-09-28; the site's header reads "포덕166년 2026.09.28", against its own articles, and is not followed |
| [wikipedia-iranian-calendars] | The imperial year: its epoch, introduction and reversal, and the table of 1355–1357 | Yes, 2026-09-28; Gheissari 2010 and Molavi 2002, which it cites, were not read |
| [heydari-malayeri2004] | The 33-year rule's agreement with the equinox from A.P. 1178 to 1634 | Yes, as `persian_33` cites it |
| [zhang-xinbin-huangdi] | The five epochs, 劉師培's dated signature, Sun Yat-sen's telegram | Yes, 2026-09-28, in the Internet Archive's copy of 20 January 2023 |
| [wikipedia-zh-huangdi-era] | The Hubei gazette of 4609年9月10日, the table of 1903–1911 and its lunar-new-year boundary | Yes, 2026-09-28 |
| [wikipedia-zh-kongzi-era] | 孔子紀年's epoch, 551 BC | Yes, 2026-09-28 |

## Code

`crates/hc-calendars-solar/src/year_counts.rs`,
`crates/hc-calendars-regional/src/huangdi.rs`,
`crates/hc-calendars-solar/src/philip_era.rs`,
`crates/hc-calendars-solar/src/bostran.rs`,
`crates/hc-calendars-solar/src/era_fascista.rs`,
`crates/hc-calendars-solar/src/masonic_march.rs` and
`crates/hc-calendars-solar/src/ptolemaic_day.rs`; the anchors are the
tests named above.
