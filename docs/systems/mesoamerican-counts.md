# The Maya and Aztec counts

Backs the identifiers `maya-longcount`, `maya-longcount-gmt2`, `maya-tzolkin`,
`maya-haab`, `maya-round`, `aztec-tonalpohualli` and `aztec-xiuhpohualli` in
`hc-calendars-regional`.

## What it is

Two families of day counts kept across Mesoamerica, read today from carved
monuments and painted books rather than from any living civil calendar.

**The Maya calendars.** A Classic Maya inscription dates an event in several
counts at once. The **Long Count** is a tally of days since a mythological
zero, written as five places, `baktun.katun.tun.uinal.kin`; the **Tzolkʼin**
is a 260-day cycle of thirteen numbers against twenty named days; the
**Haabʼ** is a 365-day year of eighteen named months of twenty days and five
days of Wayebʼ; and the **Calendar Round** is the Tzolkʼin and the Haabʼ
read together, a pair that recurs every 18 980 days, about 52 years
[wikipedia-maya-calendar]. The counts are older than the Maya: the earliest
contemporaneous Long Count yet found, Stela 2 at Chiapa de Corzo in Chiapas,
is carved in an Epi-Olmec rather than a Maya style and reads 7.16.3.2.13,
in 36 BCE, with Tres Zapotes Stela C at 7.16.6.16.18 in 32 BCE
[wikipedia-long-count, wikipedia-chiapa-de-corzo]. The Long Count had
fallen out of use by Landa's time and is retained by scholars for
calculation [martin2012]; the 260-day count never stopped, and is still kept by
communities in the Guatemalan highlands for planting, divination and ritual
[wikipedia-tzolkin, wikipedia-maya-calendar].

**The Aztec calendars.** The Mexica of Tenochtitlan and their neighbours
kept the same two cycles under Nahuatl names: the **tonalpohualli**, 260
days of thirteen numbers against twenty day-signs, and the
**xiuhpohualli**, 365 days of eighteen twenty-day *veintenas* and five
*nemontemi* days "left for contemplation"; the two realign every 52 years,
the *xiuhmolpilli* or binding of the years [wikipedia-aztec-calendar,
wikipedia-xiuhpohualli, wikipedia-tonalpohualli]. They are known from the
painted books — the Codex Borbonicus, the Codex Magliabechiano, the
Codex Telleriano-Remensis — and from Sahagún's *Florentine Codex*
[wikipedia-tonalpohualli, wikipedia-aztec-calendar]. There is no Aztec long
count.

**Why the correlation is the whole question.** Nothing inside any of these
counts names a Western date. A Maya day is fixed to a Julian Day Number by
one integer, the **correlation constant**: the Julian Day Number of Long
Count 0.0.0.0.0. Some thirty values have been proposed, from Bowditch's
394 483 to Weitzel's 774 078 [wikipedia-long-count], and the Aztec cycles
are fixed by a separate anchor of their own. Every Western date in this
document and in the code is therefore a date *under a stated correlation*,
and the library registers the correlations it carries as separate
calendars rather than as a parameter.

## How it works

**The Long Count.** Five mixed-radix places: 20 kʼin to a winal (uinal), 18
winal to a tun of 360 days, 20 tun to a kʼatun of 7 200 days, 20 kʼatun to a
bʼakʼtun of 144 000 days. The 18 is the only irregular place; it is what
keeps the tun close to a year [wikipedia-long-count]. Reingold and
Dershowitz write the conversion as `(from-radix count (list 20 20 18 20))`
added to the epoch, and the inverse as `to-radix` [reingold2018code]. The
epoch is `(fixed-from-jd 584283)`, which they gloss as 11 August −3113, that
is 11 August 3114 BCE in the proleptic Gregorian calendar, 6 September in
the Julian, a Monday [reingold2018code, wikipedia-long-count]. The Maya
wrote the epoch itself as 13.0.0.0.0, and the same digits came round again
on 21 December 2012 [wikipedia-long-count, famsi-vanstone-2012].

**The Tzolkʼin.** Thirteen numbers and twenty names advance together, one
step each per day, so a position is a pair (number, name) and the cycle
closes after lcm(13, 20) = 260 days. The names, in the sixteenth-century
Yucatec spelling this library uses: Imix, Ik, Akbal, Kan, Chicchan, Cimi,
Manik, Lamat, Muluc, Oc, Chuen, Eb, Ben, Ix, Men, Cib, Caban, Etznab,
Cauac, Ahau [wikipedia-tzolkin]. The ordinal of a pair within the cycle is
`(number − 1 + 39 (number − name)) mod 260` [reingold2018code,
`mayan-tzolkin-ordinal`]: 39 is what solves the two congruences at once,
since 39 ≡ 0 (mod 13) and 39 ≡ −1 (mod 20). The epoch 0.0.0.0.0 is
4 Ahau, ordinal 159, so the Tzolkʼin cycle containing the epoch began 159
days before it [reingold2018code, `mayan-tzolkin-epoch`].

**The Haabʼ.** Eighteen months of twenty days — Pop, Uo, Zip, Zotz, Tzec,
Xul, Yaxkin, Mol, Chen, Yax, Zac, Ceh, Mac, Kankin, Muan, Pax, Kayab, Cumku
— and the five days of Uayeb, 365 in all, with no leap day ever
[wikipedia-haab]. Days are counted **from zero**: the first day of a month
is its "seating", 0 Pop, and the last is 19 Pop [wikipedia-haab]. The epoch
is 8 Cumku, ordinal 348, so the Haabʼ year containing the epoch began 348
days before it [reingold2018code, `mayan-haab-epoch`].

**The Calendar Round.** Since 365 ≡ 105 (mod 260) and gcd(105, 260) = 5,
a Tzolkʼin position and a Haabʼ position can fall on the same day only when
their day counts agree modulo 5: one pairing in five occurs, 18 980 of the
94 900 conceivable, and the round closes after 18 980 days = 52 Haabʼ =
73 Tzolkʼin [reingold2018code, `mayan-calendar-round-on-or-before`;
wikipedia-maya-calendar]. The same congruence is why only four of the
twenty names can open a Haabʼ year: the **year bearer**, the Tzolkʼin name
of 0 Pop, is always Ik, Manik, Eb or Caban [wikipedia-maya-calendar;
reingold2018code, `mayan-year-bearer-from-fixed`].

**The GMT correlations.** Goodman proposed 584 280 and Martínez Hernández
584 281 [wikipedia-long-count]; Thompson worked from Landa's *Relación* —
the Colonial Maya New Year 12 Kʼan 1 Pop on 16 July 1553, and the kʼatun
ending 11.16.0.0.0 in 1539 — first to 584 285 with a "break of a day"
between the Classic and Colonial calendars, then to 584 284 without it,
and finally, following Martínez Hernández's point of 1926 that Landa's
informants had spoken some years before 1553 and a leap year had not been
allowed for, to **584 283** [martin2012]. That last value is the one the
literature calls "the GMT correlation"; 584 285, kept alive by Thompson's
earlier work and by Lounsbury, is listed beside it as "Thompson
(Lounsbury)" [wikipedia-long-count]. Bricker and Bricker find only 584 283
consistent with the Calendar Round dates that Landa, the Chronicle of
Oxkutzcab and the books of Chilam Balam pair with Julian dates, and it is
the value favoured by those who hold that the highland daykeepers' count is
unbroken [wikipedia-long-count, martin2012]. High-precision AMS radiocarbon
dating of a carved lintel from Temple I at Tikal places its carving between
658 and 696 CE and "strongly supports" the GMT correlation [kennett2013].
Martin and Skidmore reopen the question from the other side: Santa Elena
Poco Uinic Stela 3 records what appears to be an eclipse at 9.17.19.13.16
5 Kib 14 Chʼen, the total solar eclipse of 16 July 790 (JDN 2 009 802), and
2 009 802 − 1 425 516 gives **584 286** [martin2012]. Which of these is
right is a claim about history, not arithmetic; this library therefore
registers 584 283 as `maya-longcount` and 584 285 as `maya-longcount-gmt2`,
two calendars with two names, so that a caller can ask both and see the
two-day difference rather than get one of them silently
([policy.md §5](../policy.md)).

**The Aztec counts and Caso's correlation.** The tonalpohualli works
exactly as the Tzolkʼin, with the signs Cipactli, Ehecatl, Calli,
Cuetzpalin, Coatl, Miquiztli, Mazatl, Tochtli, Atl, Itzcuintli, Ozomatli,
Malinalli, Acatl, Ocelotl, Cuauhtli, Cozcacuauhtli, Ollin, Tecpatl,
Quiahuitl, Xochitl [wikipedia-tonalpohualli]; the xiuhpohualli as the
Haabʼ, except that its days are counted from 1 and the sources disagree
about which *veintena* opens the year — Sahagún begins with Atlcahualo,
Durán with Izcalli [wikipedia-xiuhpohualli]. Reingold and Dershowitz number
the months with Izcalli first and Xocotlhuetzi eleventh, and this library
follows them. Their anchor, which they name as Caso's, is the **fall of
Tenochtitlan on 13 August 1521 (Julian)** as the day 1 Coatl of the
tonalpohualli and day 2 of Xocotlhuetzi in the xiuhpohualli
[reingold2018code, `aztec-correlation`, `aztec-tonalpohualli-correlation`,
`aztec-xihuitl-correlation`; caso1971; wikipedia-fall-of-tenochtitlan]. The
tonalpohualli cycle containing that day began 104 days before it, and the
xiuhpohualli year 201 days before it. Aztec years are named by a **year
bearer** too, but taken from the other end: the tonalpohualli day of the
last day of the eighteenth month, day 20 of Tititl, which can only be
Tochtli, Acatl, Tecpatl or Calli [reingold2018code,
`aztec-xiuhmolpilli-from-fixed`; wikipedia-aztec-calendar]. Whether the
xiuhpohualli was ever corrected for the quarter-day drift is disputed;
the reconstructions that assume a correction run against what most
scholars hold, that no Mesoamerican calendar had a leap day, and this
library carries the uncorrected 365-day year [wikipedia-aztec-calendar,
azteccalendar].

**Worked example: Chiapa de Corzo Stela 2.** The stela reads 7.16.3.2.13
[wikipedia-long-count, wikipedia-chiapa-de-corzo].

1. Days since 0.0.0.0.0: 7 × 144 000 + 16 × 7 200 + 3 × 360 + 2 × 20 + 13
   = 1 008 000 + 115 200 + 1 080 + 40 + 13 = **1 124 333**.
2. Tzolkʼin: (1 124 333 + 159) mod 260 = 252; number 252 mod 13 + 1 = 6,
   name 252 mod 20 + 1 = 13, Ben: **6 Ben**. Check by the ordinal formula:
   (6 − 1 + 39 × (6 − 13)) mod 260 = (5 − 273) mod 260 = 252.
3. Haabʼ: (1 124 333 + 348) mod 365 = 116; month 116 ÷ 20 = 5, so the sixth
   month, Xul; day 116 mod 20 = 16: **16 Xul**.
4. Can they pair? The Tzolkʼin count is (epoch − 159 + 252) and the Haabʼ
   count (epoch − 348 + 116); the difference is 325, divisible by 5, so
   6 Ben 16 Xul occurs. Its ordinal in the round is
   (−232 + 365 × 325) mod 18 980 = 118 393 mod 18 980 = 4 513, and
   1 124 333 ÷ 18 980 = 59 remainder 4 513: round **59**.
5. Julian Day Number under 584 283: 584 283 + 1 124 333 = **1 708 616**,
   which is 6 December 36 BCE in the proleptic Gregorian calendar and
   8 December in the Julian — the "December 6, 36 BCE" the literature
   prints [wikipedia-long-count]. Under 584 285 it is JDN 1 708 618,
   8 December 36 BCE (Gregorian); under 584 286, 9 December.

The same steps take a Classic Palenque date, Kʼinich Janaabʼ Pakal's birth
at 9.8.9.13.0 8 Ajaw 13 Pop [wikipedia-pakal], to 1 357 100 days, Tzolkʼin
ordinal 59 (8 Ahau), Haabʼ ordinal 13 (13 Pop), and JDN 1 941 383 under
584 283: 24 March 603 proleptic Gregorian, 21 March Julian — the "24 March
603" the literature gives, which is therefore a Gregorian date under
584 283; 584 285 makes it 26 March.

**Worked example: the fall of Tenochtitlan.** 13 August 1521 Julian is
fixed day 555 403 (23 August 1521 in the proleptic Gregorian calendar).
The tonalpohualli count is 555 403 − (555 403 − 104) = 104: number
104 mod 13 + 1 = 1, sign 104 mod 20 + 1 = 5, Coatl: **1 Coatl**. The
xiuhpohualli count is 201: month 201 ÷ 20 + 1 = 11, Xocotlhuetzi, day
201 mod 20 + 1 = 2: **2 Xocotlhuetzi**. The year began at 1 Izcalli on
24 January 1521 (Julian) and its twentieth day of Tititl falls on
18 January 1522, whose tonalpohualli is 3 Calli, so the year is *3 Calli*
by Reingold and Dershowitz's rule; no source read here states the year
name independently.

## What is carried

- **`maya-longcount`** — the Long Count under 584 283, `MayaLongCountCalendar::GMT`,
  the default. The five places are `DateFields::extra` fields `baktun`,
  `katun`, `tun`, `uinal`, `kin`; `year` mirrors the baktun. Range
  0.0.0.0.0 to 19.19.19.17.19, twenty baktun: RD −1 137 142 to
  RD 1 742 857, 11 August 3114 BCE to 12 October 4772 CE (proleptic
  Gregorian). Earlier days answer `BeforeEpoch`, later ones
  `AfterSupportedRange`; a place outside its radix, `DayOutOfRange`.
- **`maya-longcount-gmt2`** — the same under 584 285, `MayaLongCountCalendar::GMT_PLUS_TWO`;
  every fixed day two later. `MayaLongCountCalendar::with_correlation`
  accepts any other constant — 584 286, say — but such a calendar reports
  the identifier `maya-longcount` and is not registered.
- **`maya-tzolkin`**, **`maya-haab`**, **`maya-round`** — the three
  cycles, anchored to 584 283 **only**: their epochs are derived from
  `EPOCH`, and reading the same fixed day through them beside
  `maya-longcount-gmt2` gives a Calendar Round two positions on (Chiapa de
  Corzo's day comes out 8 Men 18 Xul). The Tzolkʼin and Haabʼ are unbounded
  and run backwards through the epoch; `maya-round` is bounded to the Long
  Count's twenty baktun so that its round number stays meaningful beside
  it. The Haabʼ's generic `day` field is **1-based** as the trait
  requires, so 0 Pop is `day: 1` in `DateFields` and `day: 0` in
  `HaabPosition`.
- **`aztec-tonalpohualli`**, **`aztec-xiuhpohualli`** — the two Aztec
  cycles under Caso's anchor, `CORRELATION` = RD 555 403, unbounded in
  both directions. Xiuhpohualli days are 1-based in the position and the
  field alike.
- **What `year` means.** For the cycles, `year` is the **round number**:
  complete cycles elapsed since the cycle that contains the anchor began —
  159 days before 0.0.0.0.0 for the Tzolkʼin, 348 for the Haabʼ, 0 for the
  Calendar Round; 104 days before 13 August 1521 for the tonalpohualli and
  201 for the xiuhpohualli. The day before the Maya epoch is still in
  Tzolkʼin round 0; RD −1 137 302 is the first day of round −1. The crate
  documentation explains why the round is carried at all: the trait
  demands a bijection, and a bare position is not one.
- **Leap years.** None of these counts has one, and none has a counted
  year to ask about: the `year` field of the Tzolkʼin, the Haabʼ, the
  Calendar Round, the tonalpohualli and the xiuhpohualli is a position in
  a round, and the Long Count has no year at all. Each therefore answers
  `Calendar::is_leap_year` with `UnsupportedField("year")`, and the dynamic
  `days_in_year` passes that answer through rather than probing for a
  first month; a caller should treat it as "the question does not apply".
- **Not carried, and why.**
  - *The 819-day count*, four stations of 819 days tied to colours and
    directions, which Linden and Bricker extend to twenty stations,
    16 380 days, to fit the synodic periods of the visible planets
    [linden2023, wikipedia-maya-calendar]. It is pure arithmetic beside
    these four and is a row of its own in the roadmap, `maya-819`.
  - *The Lords of the Night*, the nine-day cycle [wikipedia-maya-calendar],
    and the Maya and Aztec *year bearers* as fields: both are derivable
    from what is carried (the year bearer is the Tzolkʼin of 0 Pop, or the
    tonalpohualli of 20 Tititl), and neither is exposed.
  - *The other Mesoamerican 365-day years* — the Zapotec, Purépecha and
    Zoque years — and the *Mixtec year bearer*, which names the same solar
    year one number lower than the Aztec one: the same arithmetic under
    other name tables and other anchors, listed in the roadmap as
    `zapotec-yza`, `purepecha-year`, `zoque-hame` and `mixtec-year`, and
    kept out of `aztec-xiuhpohualli` because reading a Mixtec codex
    through it would give a silently wrong year name.
  - *The living highland counts* (Kʼicheʼ, Kaqchikel, Ixil, Mixe): their
    modern anchor differs between communities and is the whole question;
    the roadmap row `cholqij` is still researching it.
  - *A leap day* in the xiuhpohualli, for the reason given above.
  - *A time of day* for the change of Tzolkʼin and Haabʼ: Martin and
    Skidmore's argument turns on the two changing at different hours
    [martin2012]; this library, like Reingold and Dershowitz, changes
    everything at the day boundary.

## Accuracy

Every count is exact integer arithmetic; the only question is the
anchor. The tests hold these published readings:

| Reading | Source | Test |
| --- | --- | --- |
| 0.0.0.0.0 = JDN 584 283 = 11 August 3114 BCE (Gregorian) = 6 September (Julian) | [reingold2018code], [wikipedia-long-count] | `the_correlation_puts_the_epoch_where_the_constant_says` |
| 0.0.0.0.0 = 4 Ahau 8 Cumku, round 0 | [reingold2018code] | `the_epoch_is_four_ahau_eight_cumku` |
| 13.0.0.0.0 = 21 December 2012 under 584 283, 23 December under 584 285 | [wikipedia-long-count], [famsi-vanstone-2012] | `the_thirteenth_baktun_ended_on_the_twenty_first_of_december_2012`, `the_thirteenth_baktun_lands_where_each_correlation_says` |
| 13.0.0.0.0 = 4 Ahau 3 Kankin | [famsi-vanstone-2012] | `the_thirteenth_baktun_ended_on_four_ahau_three_kankin` |
| 21 September 2026 = 13.0.13.17.2 | [azteccalendar] | `a_published_modern_long_count_matches` |
| 13 August 1521 (Julian) = 1 Coatl, 2 Xocotlhuetzi | [reingold2018code] | `the_correlation_is_the_fall_of_tenochtitlan`, `tenochtitlan_fell_on_one_coatl_two_xocotlhuetzi` |
| 21 September 2026 = 8 Ehecatl, 14 Tititl | [azteccalendar] | `a_published_modern_date_matches` |

Beyond the anchors, the tests check that exactly 18 980 of the 94 900
Tzolkʼin–Haabʼ pairings are accepted, that every day of a whole Calendar
Round round-trips through all three Maya cycles and both Aztec ones, that
the two correlations differ by exactly two days over a million-day sample,
and that the uncorrected xiuhpohualli returns to the same position after
400 × 365 days, 97 days short of 400 Gregorian years.

Two readings not held by a test were checked while writing this document:
Chiapa de Corzo Stela 2 and Pakal's birth come out on the proleptic
Gregorian days the literature prints, as the worked examples show; and on
25 September 2026 azteccalendar.com printed 12 Miquiztli, 18 Tititl and
13.0.13.17.6, four days on from the test's 21 September values in all
three counts [azteccalendar].

**The caveat, stated plainly.** Agreement with a converter that uses the
same constant is not evidence for the constant. 584 283 is the majority
choice, supported by the Colonial Calendar Round dates and by the Tikal
radiocarbon result; 584 285 is in wide use; 584 286 has a published
astronomical argument. A Western date this library prints for a Maya
inscription can be two or three days from the truth if the majority is
wrong, and a Western date for an Aztec one rests on a single anchor
reported for a single day in 1521. The Aztec year bearer, the month that
opens the year and the existence of a correction are all contested in the
sources, and the library carries one reconstruction, Caso's as Reingold
and Dershowitz tabulate it.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [reingold2018] | The scheme: radices, ordinals, epochs, Caso's anchor, the month numbering | Not read directly; the published code was |
| [reingold2018code] | The exact definitions: `mayan-epoch` (584 283), `fixed-from-mayan-long-count`, `mayan-tzolkin-ordinal`, `mayan-tzolkin-epoch`, `mayan-haab-epoch`, `mayan-calendar-round-on-or-before`, `mayan-year-bearer-from-fixed`, `aztec-correlation` (13 August 1521 Julian), `aztec-tonalpohualli-correlation` (1 Coatl), `aztec-xihuitl-correlation` (11, 2), `aztec-xiuhmolpilli-from-fixed` | Yes, 2026-09-25 |
| [wikipedia-long-count] | The places and the 18; the table of proposed constants, Goodman 584 280, Martínez Hernández 584 281, GMT 584 283, Thompson (Lounsbury) 584 285; Bricker and Bricker on the Colonial dates; the Tikal lintel; Chiapa de Corzo Stela 2 as 7.16.3.2.13 or 8.7.3.2.13, 6 December 36 BCE, Epi-Olmec; 11 August 3114 BCE a Monday | Yes, 2026-09-25 |
| [wikipedia-maya-calendar] | Who used the counts and when; the Calendar Round's 18 980 days; the four year bearers Ik, Manik, Eb, Caban; the 819-day count; the Lords of the Night | Yes, 2026-09-25 |
| [wikipedia-tzolkin] | The twenty names in the sixteenth-century and revised spellings; the count's survival in highland Guatemala | Yes, 2026-09-25 |
| [wikipedia-haab] | The nineteen months in both spellings; the seating as day 0; no leap day | Yes, 2026-09-25 |
| [wikipedia-chiapa-de-corzo] | Stela 2 as the oldest Long Count date found, December 36 BCE | Yes, 2026-09-25 |
| [wikipedia-pakal] | 9.8.9.13.0 8 Ajaw 13 Pop = 24 March 603 | Yes, 2026-09-25; the article does not say whether its Western dates are Julian or Gregorian, and the arithmetic shows them to be Gregorian under 584 283 |
| [martin2012] | The history of 584 285, 584 284 and 584 283 from Landa's 12 Kʼan 1 Pop of 16 July 1553; the Poco Uinic eclipse and 584 286; the Long Count's disuse after the Classic | Yes, 2026-09-25, from the PDF's text; the figures were not seen |
| [kennett2013] | The Tikal lintel dated 658–696 CE, "strongly supports" the GMT correlation | The abstract only, 2026-09-25, in Europe PMC; the article's pages did not open |
| [linden2023] | The 819-day count extended to twenty stations, 16 380 days | The abstract only, 2026-09-25 |
| [caso1971] | The Aztec anchor as Reingold and Dershowitz attribute it | Not read directly; the bibliographic details from the publisher's listing |
| [wikipedia-aztec-calendar] | The day-signs; Sahagún's and Durán's first months; the four year bearers Tochtli, Acatl, Tecpatl, Calli; the *xiuhmolpilli*; 13 August 1521 = 1 Coatl; Caso as the basis of later reconstructions; the leap-day dispute | Yes, 2026-09-25 |
| [wikipedia-xiuhpohualli] | The eighteen *veintenas* and the nemontemi; the disagreement over the year's first month; the vague year | Yes, 2026-09-25 |
| [wikipedia-tonalpohualli] | The twenty signs; the codices; who used it | Yes, 2026-09-25 |
| [wikipedia-fall-of-tenochtitlan] | 13 August 1521 as a Julian date | Yes, 2026-09-25; it gives no Aztec date for the day |
| [famsi-vanstone-2012] | 13.0.0.0.0 4 Ajaw 3 Kʼankʼin as 21 or 23 December 2012; the count continuing past 13 | Yes, 2026-09-25 |
| [azteccalendar] | The converter the tests cite; its statement that it uses Caso's correlation and no leap-year correction; the reading of 25 September 2026 | Yes, 2026-09-25 |

The Smithsonian's *Living Maya Time* converter
(`maya.nmai.si.edu/calendar/maya-calendar-converter`) answered HTTP 500 on
2026-09-25 and was not read; Landa's *Relación*, the Chronicle of
Oxkutzcab and the codices are known here only through the sources above.

Statements in the module documentation for which this document names no
source: that Sahagún's informants dated the fall of Tenochtitlan 1 Coatl,
2 Xocotlhuetzi (the pairing is Reingold and Dershowitz's, attributed by them
to Caso; the *Florentine Codex* was not read); that the "Nuttall–Ochoa
model" is the name of the competing reconstruction (Wikipedia names Ochoa
and Medina correlations, not Nuttall); that the radiocarbon result supports
584 283 as a number rather than "the GMT correlation" by name, which is all
the abstract says; that the Calendar Round "is what most dated monuments
actually carry"; that the day and month spellings are those of Reingold and
Dershowitz's tables (the tables were not read; Wikipedia's older spelling is
the same); and, in the roadmap row, that Stela 2's two highest places are
reconstructed — the Long Count article offers a second reading, 8.7.3.2.13,
which differs in exactly those two places, and says no more.

## Code

`crates/hc-calendars-regional/src/maya.rs` (`GMT_CORRELATION`,
`GMT_PLUS_TWO_CORRELATION`, `EPOCH`, `MayaLongCountCalendar`,
`MayaTzolkinCalendar`, `MayaHaabCalendar`, `MayaCalendarRoundCalendar`,
`calendar_round_ordinal`) and `aztec.rs` (`CORRELATION`,
`AztecTonalpohualliCalendar`, `AztecXiuhpohualliCalendar`). Anchors:
`the_correlation_puts_the_epoch_where_the_constant_says`,
`the_epoch_is_four_ahau_eight_cumku`,
`the_thirteenth_baktun_ended_on_the_twenty_first_of_december_2012`,
`the_thirteenth_baktun_ended_on_four_ahau_three_kankin`,
`the_thirteenth_baktun_lands_where_each_correlation_says`,
`a_published_modern_long_count_matches`,
`the_correlation_is_the_fall_of_tenochtitlan`,
`tenochtitlan_fell_on_one_coatl_two_xocotlhuetzi`,
`a_published_modern_date_matches`; the structure:
`only_one_tzolkin_haab_pairing_in_five_can_occur`,
`every_day_of_a_whole_calendar_round_round_trips`,
`the_two_correlations_differ_by_exactly_two_days`,
`both_cycles_round_trip_over_a_full_calendar_round`,
`the_vague_year_drifts_one_day_every_four_years`. The round-number
convention is explained in the crate documentation of
`hc-calendars-regional`; the roadmap rows for the counts not carried are in
[`calendars.md`](../calendars.md).
