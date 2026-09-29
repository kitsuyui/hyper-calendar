# Spain's national and autonomous-community holidays

Spain's public holidays are set in two layers: a national list in a royal
decree, part of which each autonomous community may replace, and a yearly
choice by each community, which the Ministry of Labour publishes for all
nineteen at once. This document covers `hc-holiday`'s `SPAIN` table and
its regions, the seventeen communities and the two autonomous cities.

## What it is

Article 37.2 of the Estatuto de los Trabajadores caps the fiestas
laborales at fourteen a year, two of them local, and lets the Government
move them to Mondays and the communities replace some national ones with
their own. Real Decreto 2001/1983 carries it out [es-rd-2001-1983]:

- **Article 45.1** lists the national days, "de carácter retribuido y no
  recuperable". The yearly resolutions quoted below sort them in three:
  the days in its paragraphs a) to c), which no community may replace,
  and those of paragraph d), which a community may keep or replace with a
  day of its own tradition: Epiphany, Maundy Thursday, and St Joseph's Day
  (19 March) or St James's Day (25 July), at the community's choice.
- **Article 45.2**: when a national day falls on a Sunday, its rest is
  taken on the following Monday, and **45.3** lets a community replace that
  Monday too with a day of its own.
- **Article 45.4**: each community tells the Ministry its list before
  30 September, and the Ministry publishes the lists in the BOE.
- **Article 46**: up to two local days a year in each municipality, which
  the regional or provincial bulletin publishes.

The result is one resolution a year of the Dirección General de Trabajo
(the Dirección General de Empleo until the one for 2018) whose annex is a
table: a row per date, a column per community, and a mark in each cell —
`*` for a national day no community may replace, `**` for a national day
the community did not replace, `***` for a day of the community's own
[es-fiestas-2013] [es-fiestas-2026]. Its notes add what the communities'
decrees say beyond the table: the Canary Islands' island days, the Aran
Valley's day, and which day is the recoverable one where a community
keeps a thirteenth.

## How it works

The resolution for 2026 [es-fiestas-2026], published on 28 October 2025,
gives Madrid and Catalonia these days besides the ones every community
keeps:

| Date | Madrid (`ES-MD`) | Catalonia (`ES-CT`) |
| --- | --- | --- |
| Thursday 2 April | Jueves Santo `**` | — |
| Monday 6 April | — | Lunes de Pascua `***` |
| Saturday 2 May | Fiesta de la Comunidad de Madrid `***` | — |
| Wednesday 24 June | — | San Juan `***` |
| Friday 11 September | — | Fiesta Nacional de Cataluña `***` |
| Monday 2 November | Día siguiente a Todos los Santos `**` | — |
| Monday 7 December | Lunes siguiente al Día de la Constitución Española `**` | — |
| Saturday 26 December | — | San Esteban `***` |

All Saints' Day and Constitution Day fall on Sundays in 2026. Madrid keeps
both Monday rests of article 45.2; Catalonia replaced them with Easter
Monday and St Stephen's Day, and replaced Maundy Thursday with St John's
Day. So Thursday 2 April is a day off in Madrid and a working day in
Barcelona, and Monday 6 April the reverse. A calendar asked for `ES-MD` in
2026 returns the ten nationwide days and Madrid's four.

## What is carried

- **The nationwide days**, as rules with no region: New Year's Day,
  Epiphany, Good Friday, Labour Day, the Assumption, the National Day, All
  Saints' Day, Constitution Day, the Immaculate Conception and Christmas
  Day, from 1990. The text of article 45 read is the version of Real
  Decreto 1346/1989, "en vigor a partir del 08/11/1989"; the versions of
  1983 and of Real Decreto 2403/1985 were not read, so every year before
  1990 is a gap for each day [es-rd-2001-1983]. Epiphany is a day of
  article 45.1 d) that a community may replace, and every community kept
  it in every resolution read: it is nationwide in 1994, 2013–2015 and
  2018–2026, the years whose resolution was read, and a gap in every
  other year.
- **Each community's other days**, scoped to its ISO 3166-2 code, as the
  resolutions list them: its own days (`***`), the replaceable national
  days it kept (`**`, Maundy Thursday in seventeen of the nineteen, St
  Joseph's or St James's Day in some), and the Monday rests it kept. Each
  is a `Rule::Listed` entry over a `Listing` of the community's rows, for
  the years whose resolution was read in the BOE's HTML text: 1994 (for
  the seventeen communities; Ceuta and Melilla, not yet autonomous cities,
  have no column), 2013 to 2015 and 2018 to 2026. All are `Kind::Public`: the resolutions list fiestas
  laborales, and a recoverable one is still a day off.
- **Gaps.** The resolutions for 2016 and 2017 publish the annex as an
  image [es-fiestas-2016-2017], which was not read, so those years are a
  gap for every community day, and so is every year after 2026. Of the
  resolutions before 2013, found in the BOE for every year from 1990 to
  2012 [es-fiestas-1990-2012], only those for 1994 and 1995 give the annex
  as text: 1994's is carried, and 1995's lists each community's dates
  without their names, which are not carried. For 1995 to 2012 each of
  the seventeen reports one gap, "The community's days of 1995 to 2012";
  the years before 1994, and before 2013 in Ceuta and Melilla, are a gap
  for every community day. A day a read resolution does not list is
  absent in that year.
- **Not yet carried, and why:**
  - the islands' own days in the Canaries (one per island, by the
    community's decree, which the resolutions quote in a note) and the
    Aran Valley's day, 17 June, in place of St Stephen's Day: neither is an
    ISO 3166-2 subdivision, and the table has no finer scope;
  - the municipalities' two local days of article 46, published in each
    community's or province's bulletin, of which none was read;
  - the resolutions of 1990 to 1993 and 1996 to 2012, whose annexes the
    BOE's text omits or gives as an image or PDF, the dates of 1995,
    without their names, and the images of 2016 and 2017.

### The nineteen communities

"Own days" are the `***` days, "kept" the replaceable national days the
community kept, each with the years it appears in the resolutions read;
a year missing inside a run is one in which the day fell on a Sunday and
was not listed, or was replaced. "Mondays" counts the Monday rests
(article 45.2) the community kept across the twelve years. The instrument
for every row is the year's resolution [es-fiestas-2013] …
[es-fiestas-2026], and for 1994 [es-fiestas-1994]. The table's years are
those of 2013 to 2026; 1994's are listed after it. The first year carried
is 1994 for the seventeen communities and 2013 for Ceuta and Melilla, the
years before a gap, and 1995 to 2012 a gap.

| Code | Community | Own days | Kept | Mondays |
| --- | --- | --- | --- | --- |
| ES-AN | Andalucía | Día de Andalucía, 28 February (2013–2015, 2018–2026) | Jueves Santo (all years) | 17 |
| ES-AR | Aragón | San Jorge/Día de Aragón, 23 April (all years but 2023, when it was a Sunday and the Monday was listed) | Jueves Santo (all years) | 18 |
| ES-AS | Asturias | Día de Asturias, 8 September (2014–2015, 2018, 2020–2023, 2025–2026; the Monday in 2013, 2019 and 2024) | Jueves Santo (all years) | 20 |
| ES-CB | Cantabria | Día de las Instituciones de Cantabria, 28 July (2018, 2020–2023, 2025–2026); La Bien Aparecida, 15 September (2014–2015, 2018, 2020–2023, 2025–2026); Lunes de Pascua (2013, 2015, 2019–2020, 2024) | Jueves Santo (2013–2015, 2019–2026); Santiago Apóstol (2013–2014, 2019, 2024) | 5 |
| ES-CE | Ceuta | Fiesta del Sacrificio (all years, on the dates listed); Nuestra Señora de África, 5 August (2023–2026; kept as `**` in 2022); Día de Ceuta, 2 September (2019–2023, 2026) | Jueves Santo (all years) | 7 |
| ES-CL | Castilla y León | Fiesta de Castilla y León, 23 April (all years but 2023) | Jueves Santo (all years); Santiago Apóstol (2023) | 17 |
| ES-CM | Castilla-La Mancha | Día de Castilla-La Mancha, 31 May (2013, 2018–2019, 2021–2025); Corpus Christi (2013–2015, 2019–2026); Lunes de Pascua (2014–2015, 2019–2020, 2026) | Jueves Santo (all years); San José (2020) | 4 |
| ES-CN | Canarias | Día de Canarias, 30 May (all years but 2021) | Jueves Santo (all years) | 7 |
| ES-CT | Cataluña | Lunes de Pascua (all years); San Juan, 24 June (2013–2015, 2019–2026); Fiesta Nacional de Cataluña, 11 September (all years but 2022); San Esteban, 26 December (all years but 2021); Lunes de Pascua Granada (2022) | none: Maundy Thursday replaced every year | 0 |
| ES-EX | Extremadura | Día de Extremadura, 8 September (2014–2015, 2018, 2020–2023, 2025–2026; the Monday in 2013 and 2019); Martes de Carnaval (2023–2024) | Jueves Santo (all years); San José (2021) | 17 |
| ES-GA | Galicia | Día de las Letras Gallegas, 17 May (2013–2014, 2018–2019, 2021–2025); San Juan, 24 June (2013, 2020, 2022, 2026); Día siguiente a San José (2015) | Jueves Santo (all years); Santiago Apóstol/Día Nacional de Galicia, 25 July (all years but 2021; own in 2014 and 2022); San José (2019–2021, 2026) | 1 |
| ES-IB | Illes Balears | Día de les Illes Balears, 1 March (2013–2014, 2018–2019, 2021–2025; the Monday in 2026); Lunes de Pascua (all years but 2014 and 2025); San Esteban (2013–2014, 2019–2020, 2025–2026) | Jueves Santo (all years) | 5 |
| ES-MC | Región de Murcia | Día de la Región de Murcia, 9 June (2014–2015, 2018–2023, 2025–2026) | Jueves Santo (all years); San José (2013–2015, 2018–2021, 2024–2026) | 10 |
| ES-MD | Comunidad de Madrid | Fiesta de la Comunidad de Madrid, 2 May (all years but 2021, when the Monday was listed); Corpus Christi (2014–2015); Santiago Apóstol (2022) | Jueves Santo (all years); San José (2015, 2021; moved to 18 March in 2013, to Monday 20 March in 2023); Santiago Apóstol (2024–2025) | 10 |
| ES-ML | Melilla | Fiesta del Sacrificio (all years, on the dates listed); Fiesta del Eid Fitr (2022–2023, 2025–2026); Estatuto de Autonomía de la Ciudad de Melilla, 13 March (2020–2021) | Jueves Santo (all years); San José (2013–2015) | 8 |
| ES-NC | Navarra | Lunes de Pascua (all years); Santiago Apóstol (2022) | Jueves Santo (all years); San José (2014–2015, 2019–2021, 2026); Santiago Apóstol (2013, 2015, 2023–2025) | 5 |
| ES-PV | País Vasco | Lunes de Pascua (all years); Día del País Vasco-Euskadiko Eguna, 25 October (2013–2014); V Centenario Vuelta al Mundo, 6 September (2022); Santiago Apóstol (2022) | Jueves Santo (all years); Santiago Apóstol (2013, 2015, 2019–2020, 2023–2026); San José (2015, 2019–2021, 2026) | 0 |
| ES-RI | La Rioja | Día de La Rioja, 9 June (all years but 2024, when the Monday was listed); Lunes de Pascua (2013–2015, 2019–2026) | Jueves Santo (all years) | 7 |
| ES-VC | Comunitat Valenciana | Día de la Comunitat Valenciana, 9 October (all years but 2022); Lunes de Pascua (all years); San Juan (2019–2026); Lunes de Fallas (2013) | San José (all years but 2023); Jueves Santo (2022 only) | 1 |

"All years" means all twelve read of 2013 to 2026.

The resolution for 1994 [es-fiestas-1994] marks each day "Nal." or "X"
and has seventeen columns. Besides the nationwide days, Epiphany among
them, it gives the communities these:

| Code | Days of 1994 |
| --- | --- |
| ES-AN | Día de Andalucía, 28 February; Jueves Santo, 31 March; Monday rests of 2 May and 26 December |
| ES-AR | Jueves Santo; Día de Aragón, 23 April; Santiago Apóstol, 25 July; Monday rest of 26 December |
| ES-AS | Jueves Santo; Día de Asturias, 8 September; Monday rests of 2 May and 26 December |
| ES-CB | Santiago Apóstol; Día de las Instituciones, 28 July; Nuestra Señora Bien Aparecida, 15 September; Monday rest of 26 December |
| ES-CL | San José, 19 March; Jueves Santo; Fiesta de la Comunidad Autónoma de Castilla y León, 23 April; Santiago Apóstol |
| ES-CM | San José; Jueves Santo; Día de la Región de Castilla-La Mancha, 31 May; Monday rest of 2 May |
| ES-CN | Día de Canarias, 30 May; Santiago Apóstol; Monday rests of 2 May and 26 December |
| ES-CT | Lunes de Pascua, 4 April; Lunes de Pascua Granada, 23 May; San Juan, 24 June; San Esteban, 26 December |
| ES-EX | Jueves Santo; Día de Extremadura, 8 September; Monday rests of 2 May and 26 December |
| ES-GA | San José; Jueves Santo; Día de las Letras Gallegas, 17 May; Santiago Apóstol |
| ES-IB | Jueves Santo; Santiago Apóstol; Segunda fiesta de Navidad, 26 December; "En sustitución del día 25 de diciembre", 27 December |
| ES-MC | San José; Jueves Santo; Día de la Región, 9 June; Monday rest of 2 May |
| ES-MD | Jueves Santo; Santiago Apóstol; Monday rests of 2 May and 26 December |
| ES-NC | San José; Jueves Santo; Lunes de Pascua; Santiago Apóstol |
| ES-PV | San José; Jueves Santo; Lunes de Pascua; Santiago Apóstol |
| ES-RI | Jueves Santo; Lunes de Pascua; Día de La Rioja, 9 June; Santiago Apóstol |
| ES-VC | San José; Jueves Santo; Lunes de Pascua; Segundo día de Navidad, 26 December |

Each has twelve days in 1994, the Canaries too, and the names are carried
under the spellings of the later years where the day is the same (Día de
Aragón as San Jorge/Día de Aragón, Día de la Región as Día de la Región de
Murcia); Balearic Islands' and Valencia's second days of Christmas keep
their own names. The "X" mark does not tell a kept national day from the
community's own. The Eid days of Ceuta and Melilla are
listed on the dates each resolution gives; they are not computed.

## Accuracy

The table was not typed: a script read each resolution's annex from the
BOE's HTML text and wrote the rows, and `crates/hc-holiday/tests/spain_communities.rs`
checks the result against dates copied from the annexes by hand. The
script read the cells by their column and checked, for every year, that
each row has nineteen cells and every mark is `*`, `**` or `***`. The
markup misspells a community's cell id in 2019 (`illesbaleares` beside
`illesbalears`) and in 2020 (`headerCastaluna`), which is why the script
reads columns by position under the header row rather than by the cells'
`headers` attribute. Each community has twelve days in each year, thirteen where
its decree adds a recoverable one, and eleven in the Canaries, whose
twelfth is each island's own; the count was checked for every year.

The resolution for 2018 was corrected twice [es-fiestas-2018]: the first
correction changed only the legal basis and a note (the Murcia sentence
that names the recoverable day), the second marked 8 December for the
Canaries, which the table carries nationwide anyway. The corrections of
the 2013 and 2014 resolutions change only a decree's number in the legal
basis. The names are the resolutions' Spanish ones; where a name's
spelling changes between years (`Día de las Illes Balears` and `Día de les
Illes Balears`, `Día de Estremadura` in 2020, the four spellings of the
Fiesta del Sacrificio), one spelling is kept, and a row whose label joins
two communities' days (23 April 2018, "San Jorge Fiesta de Castilla y
León") is split between them.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [es-rd-2001-1983] | Articles 45 and 46: the national days, the Monday rest, the communities' replacement, the local days | Yes, 2026-09-26 and 2026-09-29, the BOE's consolidated text |
| [es-fiestas-2013] … [es-fiestas-2026] | Each year's annex: every community's days and marks, and the notes | Yes, 2026-09-29, the BOE's HTML text, for 2013–2015 and 2018–2026 |
| [es-fiestas-2018] | The 2018 resolution's two corrections | Yes, 2026-09-29 |
| [es-fiestas-2016-2017] | The 2016 and 2017 resolutions, whose annexes are images | The text read, the annexes not |
| [es-fiestas-1994] | The 1994 annex, with its correction | Yes, 2026-09-29, the BOE's HTML text |
| [es-fiestas-1990-2012] | The resolutions of 1990 to 2012 and what their text gives of the annex | Yes, 2026-09-29, the BOE's HTML text; the images and PDF not |

The Estatuto de los Trabajadores, article 37.2, is cited as the
resolutions quote it and was not read.

## Code

`crates/hc-holiday/src/countries/spain.rs`: the nationwide rules first in
`RULES`, built by `national`, read from `ARTICLE_45_READ`, with Epiphany
by `epiphany_where_read`, then each community's, built by `early` (the resolutions of
1994 and 2013–2015, with `UNREAD_1995_TO_2012` the gap between),
`early_city` (Ceuta's and Melilla's, 2013–2015) and `late` (2018–2026,
with 2016 and 2017 a gap); one
`Listing` per community, `ANDALUSIA_DAYS` to `VALENCIA_DAYS`; and the
table `SPAIN`. The exchange `XMAD` in `exchanges.rs` keeps its own list.

Anchors: every test in `crates/hc-holiday/tests/spain_communities.rs` —
each community's day in 2013 and 2026 with its kind and resolution, the
2012, 2016, 2017 and 2027 gaps, Ceuta's own day absent in 2015 and a
gap in 2012, Maundy
Thursday 2026 in seventeen communities and not in Catalonia or the
Valencian Community, the business days of 2 and 6 April 2026 in Madrid
and Catalonia, Andalusia's Monday of 7 December 2026, and the nineteen
codes; and `italy_and_spain` in `crates/hc-holiday/tests/countries.rs`
for the nationwide days.
