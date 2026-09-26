# The Seleucid era and its neighbours

Backs the identifiers `seleucid-syrian`, `antioch-caesarean-era`,
`antioch-caesarean-era-september` and `gaza-era` in `hc-calendars-solar`
and `arsacid-era` in `hc-calendars-regional`.

## What it is

**The Seleucid era.** "The era of the Seleucids among the Greco-Syrians
got its name from Nicator Seleucus … who instituted it to commemorate the
beginning of his empire (312 b.c.)". It "was first employed in the
lunar-solar calendar in use among the Macedonians that began with the first
lunar month (… Dios) following the autumnal equinox. The beginning of the
era was placed on Dios 1, 312 b.c. When the Greco-Syrians received from
Rome the solar calendar with its fixed years of 365 days plus one day every
fourth year, they fixed the beginning of the year and consequently that of
the era as October 1, and later, c. a.d. 460, as September 1, in order to
align it with the Byzantine indiction; but this did not affect the Oriental
Syrians not subject to Constantinople" [grumel-eras-historical]. It is the
era of the Zabad inscription of 512, of John of Ephesus and of the Syriac
chroniclers to Michael the Syrian in the twelfth century, and it is found
on Church of the East tombstones in Central Asia into the fourteenth
[wikipedia-seleucid-era]. The lunisolar count at Babylon is `babylonian`.

**The calendar of Antioch.** The Julian year "received from Rome" was
written with the Macedonian month names. The *hemerologia*, conversion
tables of the ninth-century manuscripts, give it as the calendar "of the
Hellenes", which "identifies with the so-called calendar of Antioch", and
"arguably, this was the most widespread calendar in the Diocese of the East
during the fifth century" [bultrighini2021]. Its months run "starting with
Aydunaios = January" and Gorpiaios "should be followed by Hyperberetaios,
Dios, and Apellaios" [bultrighini2021]: Audynaios, Peritios, Dystros,
Xanthikos, Artemisios, Daisios, Panemos, Loos, Gorpiaios, Hyperberetaios,
Dios, Apellaios, January to December. This is the "Syro-Macedonian" year
of the roadmap.

**The Caesarean era of Antioch.** "A good number of the local eras
commemorated the granting of autonomy to various cities, either by Pompey
or Caesar"; among the Caesarean ones, "Antioch (the most important of all),
Dios (later Oct.) 1, 49 (Sept. 1, following the adjustment to the Byzantine
indiction, c. 460); several Syriac writers begin this era Oct. 1, 48 b.c."
[grumel-eras-historical]. Evagrius, writing in its 641st year, dates by it
throughout his *Ecclesiastical History*, calling it "the era of Antioch"
and "the era of Theopolis" [evagrius-walford1846].

**The era of Gaza.** Among the Pompeian eras, "Gaza on Oct. 28, 61 b.c.,
following the introduction of a fixed year — an era in use until the 7th
century" [grumel-eras-historical]. The city had a calendar of its own,
which the Leiden and Vatican *hemerologia* tabulate [bultrighini2021].

**The Arsacid era.** "The Parthians established their own dynastic era,
beginning with the vernal equinox (in Babylon with 1 Nisan = 14 April) 247
BCE", proved by "a Babylonian tablet equating the Seleucid year 208 with 144
of the Arsacid era"; in Mesopotamia a date could carry both, "first
mentioning the Royal (i.e. the Arsacid) reckoning and then the 'former' or
'ancient' (i.e., the Seleucid year)" [iranica-arsacid-era]. In Iranian
contexts it was written "with Zoroastrian month and day names"
[iranica-arsacid-era], a form Grumel dates from "Ferverdin 1, 248 b.c.,
which corresponded to January 22" [grumel-eras-historical].

## How it works

**The Syro-Macedonian month is the Julian month.** Every equation read
puts the Antiochene month on the Julian one, day for day:

| Equation | Source |
| --- | --- |
| "the calends of October, on the first day of the month Hyperberetaios", 232 | P.Dura 30, in [bultrighini2021] |
| "the eighth day before the ides of November, that is, on the sixth of the month Dios", 249 | P.Euphr. 6, in [bultrighini2021] |
| "the 6th day before the nones of October, on the second day of the month Hyperberetaios", 251 | P.Dura 29, in [bultrighini2021] |
| the spring equinox, 21 March, "according to the Syrians of Antioch and the Macedonians, 21 Dystros" | Theophilus, *Easter Letter*, in [bultrighini2021] |
| "the fourteenth day of the month Gorpiaeus, which the Romans call September" | Evagrius 2.12 [evagrius-walford1846] |
| "the ninth day of the month Panemus, which the Romans call July"; "the first of the month Xanthicus, or April"; "the first of the month Lous, or August"; "Artemisius or May"; "Apellaeus, called by the Latins December" | Evagrius 4.1; 4.9; 4.9; 4.5; 4.19 [evagrius-walford1846] |
| "the last day of the month Hyperberetaeus" for the earthquake of 588 | Evagrius 6.8 [evagrius-walford1846] |

Twenty-first Dystros on 21 March in every year, as Theophilus states it,
puts 1 Dystros on 1 March and so the Julian 29 February in Peritios; the
last day of Hyperberetaios is the thirty-first. The one equation Bultrighini
lists against the *hemerologia* here, P.Dura 32's "29th of the month
Xanthikos, the day before the calends of May", agrees too if its unusual
πρὸ δύο means 29 April, as she reports Sijpesteijn suggesting
[bultrighini2021].

**The year of an era.** Each era is that month and day with a year number
that changes on a fixed Julian day. Year *N* begins on the new-year day of
the Julian year *N* − *k*, where *k* is the era's offset, so a day of the
Julian year *Y* is in year *Y* + *k* from the new-year day and in *Y* + *k*
− 1 before it:

| Era | New year | Year 1 begins | *k* |
| --- | --- | --- | --- |
| Seleucid, Syrian form | 1 October | 1 October 312 BC | 312 |
| Antioch, from 1 October | 1 October | 1 October 49 BC | 49 |
| Antioch, from 1 September | 1 September | 1 September 49 BC | 49 |
| Gaza | 28 October | 28 October 61 BC | 61 |

The Julian year is in astronomical numbering, where 312 BC is −311, so
the Seleucid year is AD + 312 from October and AD + 311 from January.
Grumel does not say how the numbers met when Antioch's year moved to
1 September. The September year here keeps every day from October to
August in the year the October count gives it and moves September from the
end of one year to the start of the next, the reading under which the year
is aligned "with the Byzantine indiction" [grumel-eras-historical], whose
year begins on 1 September [wikipedia-byzantine-calendar], without
renumbering eleven months of it; Evagrius's dates below bear it out.

*Worked example, the Seleucid era.* The Zabad inscription is dated "in the
year 823 on the 24th day of month Gorpiaios" [wikipedia-zabad-inscription],
"24 September, 512 AD" [wikipedia-seleucid-era]. Gorpiaios is September;
24 September lies before 1 October, so the year is 512 + 311 = 823.

*Worked example, Antioch.* Evagrius dates the earthquake under Leo "in the
five hundred and sixth year of the free prerogatives of the city, about the
fourth hour of the night, on the fourteenth day of the month Gorpiaeus,
which the Romans call September, on the eve of the Lord's day, in the
eleventh cycle of the indiction" [evagrius-walford1846]. From 1 September
the year is the Julian year plus 49: 506 is 457, and 14 September 457 was a
Saturday, the eve of Sunday, in the eleventh indiction, which began on
1 September 457. Counted from 1 October the same number would be 14
September 458, a Sunday in the twelfth indiction. Evagrius's other
equations — Justin's accession on 9 Panemos 566, 9 July 518; Severus's
flight in Gorpiaios 567, September 518; Justinian proclaimed on 1 Xanthikos
575, 1 April 527 — agree with the September year, and the second of them
only with it.

*Worked example, Gaza.* A mosaic at Kissufim is dated "in the month of
Loos, on the 11th (day), year 636, indiction 9", which the editors put on
4 August 576 [csla-e03137]. From 28 October the Gaza year is the Julian
year plus 61 and before it plus 60: 4 August 576 is in 636.

**The Arsacid era at Babylon** is the Babylonian lunisolar year with the
Seleucid number less 64: 1 Nisannu of Seleucid year 65 is 1 Nisannu of
Arsacid year 1, the new year of 247 BCE, and SE 208 is AE 144
[iranica-arsacid-era]. The Macedonian reckoning of the same count began in
the autumn, half a year before the Babylonian, which is why Artabanus III's
letter of CE 21, received "in the year 268 according to the royal
reckoning, in the year 333 according to the ancient numbering"
[iranica-arsacid-era], differs by 65: an autumn-to-spring date is one
Seleucid year later by the Macedonian count than by the Babylonian.

## What is carried

| Identifier | Crate | Days | Year | Range |
| --- | --- | --- | --- | --- |
| `seleucid-syrian` | `hc-calendars-solar` | Julian, under the Antiochene month names | From 1 October, SE 1 from 1 October 312 BC | Years 1 to 9 999 |
| `antioch-caesarean-era` | the same | the same | From 1 October, year 1 from 1 October 49 BC | Years 1 to 9 999 |
| `antioch-caesarean-era-september` | the same | the same | From 1 September, year 1 from 1 September 49 BC | Years 1 to 9 999 |
| `gaza-era` | the same | Julian, under the Julian month names | From 28 October, year 1 from 28 October 61 BC | Years 1 to 9 999 |
| `arsacid-era` | `hc-calendars-regional` | `babylonian`'s, sunset to sunset | From 1 Nisannu, the Seleucid year less 64 | AE 1 to 322, `babylonian`'s SE 65 to 386 |

The four Julian eras are one table, `syro_macedonian::ALL`, read by one
calendar type. A date's month is the Julian month's ordinal, 1 for
Audynaios or January through 12 for Apellaios or December, whatever month
the year begins in, as `byzantine` numbers its months; the new year is a
change of year number, not of month numbering. The days before the
Greco-Syrians took up the Julian year, which no source read dates, are the
arithmetic continued backwards: the Seleucid era began on a lunisolar year,
and at Babylon that year is `babylonian`'s.

**Not carried.**

- *The Seleucid era from 1 September*, the Greco-Syrian year after c. 460
  that Grumel describes. It would be its own identifier; no dated example
  in it was read.
- *Antioch from 1 October 48 BC*, the year of "several Syriac writers"
  [grumel-eras-historical], one less than `antioch-caesarean-era`; no dated
  example was read.
- *The Gaza calendar's own months.* The *hemerologia* equate 23 September
  with 26 Gorpiaios of Gaza, a papyrus of 538 has "the sixth day before the
  ides of May, the fifteenth of the month Artemisios", and an epitaph of 576
  has 24 Apellaios of Gaza for 4 Audynaios of Elousa [bultrighini2021].
  With thirty-day months these put 1 Dios on 28 October, as the era's epoch
  does, and five epagomenal days after Loos, 24 to 28 August, but where the
  leap day fell is in none of the texts read; Meimaris's and Kubitschek's
  tables, which would say, were not read. The days are therefore carried as
  Julian days.
- *The Arsacid era on the Zoroastrian months.* Grumel's epoch, 1 Ferverdin
  of 248 BC on 22 January, is consistent with his Persian Seleucid epoch,
  1 Ferverdin of 311 BC on 7 February: 63 wandering years of 365 days fall
  16 days behind 63 Julian ones. But the one dated example read does not
  fit it. The stele of Xwāsak is dated "year 426, month of Spandārmat, day
  of Mihr [= 14 September 215]" [iranica-arsacid-era]. By Grumel's epoch
  day 16 of Spandārmat of year 426 is 18 September 178, or 23 September
  with the five extra days after Ābān; if 426 is a transposition of 462, as
  215 + 247 suggests, it is 9 or 14 September 214, a year early; 14
  September 215 needs an epoch a year later than Grumel's, 22 January 247
  BC, and the extra days after Ābān. Henning's edition, which would settle
  the year, was not read, and neither source says where the extra days
  stood, so the form stays on the roadmap as a competing convention.
- *The Pompeian eras and those of Tyre, Sidon and Ascalon.* Grumel gives
  years and no year start for all but Tyre; for Tyre he gives the eras of
  274 and 116 BC and a year from 19 October [grumel-eras-historical], but
  the Council of Chalcedon's "year 574, on the tenth of the month Peritios,
  according to the Romans on the twenty-fifth of February", 449
  [bultrighini2021], puts year 1 in 126/125 BC, not 116. See the roadmap.

## Accuracy

The Julian eras are exact integer arithmetic over `julian`; the Arsacid era
is `babylonian`'s computation, with its measured agreement with Parker and
Dubberstein ([babylonian.md](babylonian.md)).

| Check | Test | Result |
| --- | --- | --- |
| The Antiochene months are the Julian months, as the papyri, Theophilus and Evagrius equate them | `the_antiochene_months_are_the_julian_months_the_sources_equate` | 12 of 12 |
| Zabad: 24 Gorpiaios 823 is 24 September 512; SE 1 begins on 1 October 312 BC | `the_zabad_inscription_is_twenty_four_gorpiaios_823` | Holds |
| Evagrius's earthquake: 14 Gorpiaios 506 from 1 September is 14 September 457, a Saturday in the eleventh indiction | `evagrius_dates_the_earthquake_under_leo_by_the_september_year` | Holds |
| Evagrius's accession, flight, proclamation, death and earthquake of 518–588 | `evagrius_dates_the_sixth_century_by_the_september_year` | 5 of 5 in the September year; the flight of 518 fails the October one |
| Antioch's year 1 from 1 October 49 BC, as Grumel gives it | `the_october_year_of_antioch_begins_in_49_bc` | Holds (year level: no dated example in this convention was read) |
| Kissufim: Loos 636 is August 576 | `the_kissufim_mosaic_is_in_gaza_636` | Holds (year level: the Gaza day of the month is not carried) |
| Every day of every era round-trips, and each refuses the day before its year 1 | `every_era_round_trips_and_starts_at_year_one` | Holds |
| AE = SE − 64; SE 208 is AE 144 | `the_arsacid_year_is_the_seleucid_less_sixty_four` | Holds |
| 1 Nisannu AE 1 | `the_arsacid_era_begins_with_nisannu_of_se_65` | 15 April 247 BCE here, a day after the 14 April the source gives |

The last row is a day's disagreement of the size and direction of
`babylonian`'s own with Parker and Dubberstein, 941 of whose 5 664 months
the moonlag criterion begins a day later than their table; the article does
not say where its "14 April" comes from, and their row for SE 65 was not
checked.
The Gaza synagogue's mosaic of "the month of Luos, year 569", which a
secondary source puts at "approximately July–August of the year 508"
[wikipedia-gaza-synagogue], is August 509 by the epoch; the page does not
say how it converted, and it is not used.

**What a primary source would settle.** The *hemerologia* in Kubitschek's
edition, or Meimaris's *Chronological Systems*, for the Gaza months and
leap day and for Tyre; an inscription dated in Antioch's October year, and
one in the Seleucid era from 1 September; Henning 1952 for the Xwāsak year.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [grumel-eras-historical] | The Seleucid era and its October and September years; the Antioch, Gaza, Tyre, Sidon and Ascalon eras; the Arsacid epoch on the Persian year | Yes, 2026-09-26 |
| [bultrighini2021] | The calendar of Antioch in the *hemerologia*, its month order, the papyri and Theophilus; Gaza's equations; the Chalcedon date of Tyre | Yes, 2026-09-26; Kubitschek 1915 and Meimaris 1992, which it cites, were not read |
| [evagrius-walford1846] | The era of Antioch in dates of 457–588 and the Julian month of each Macedonian one | Yes, 2026-09-26, in Walford's translation; the Greek was not read |
| [wikipedia-seleucid-era] | Zabad's 24 September 512; the era's later use | Yes, 2026-09-26; Kugener 1907 was not read |
| [wikipedia-zabad-inscription] | The inscription's "year 823 on the 24th day of month Gorpiaios" | Yes, 2026-09-26 |
| [csla-e03137] | The Kissufim mosaic, Loos 636, indiction 9, 4 August 576 | Yes, 2026-09-26; CIIP 3 was not read |
| [wikipedia-gaza-synagogue] | The synagogue mosaic of Luos 569 | Yes, 2026-09-26 |
| [wikipedia-byzantine-calendar] | The Byzantine year from 1 September | Yes, 2026-09-26, for `byzantine` |
| [iranica-arsacid-era] | The Babylonian epoch, SE 208 = AE 144, the double dates, the stele of Xwāsak | Yes, 2026-09-26, in the Wayback Machine's copy; Smith 1875 and Henning 1952 were not read |

## Code

`crates/hc-calendars-solar/src/syro_macedonian.rs` and
`crates/hc-calendars-regional/src/arsacid.rs`; the anchors are the tests
named above.
