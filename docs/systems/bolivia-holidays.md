# Bolivia's national and departmental holidays

Bolivia's national holidays are a supreme decree's, and each of its nine
departments has an efeméride, a day of its own. This document covers
`hc-holiday`'s `BOLIVIA` table and its regions.

## What it is

Decreto Supremo 21060 of 29 August 1985, article 67, made the holidays
"con suspensión de actividades públicas y privadas" the Sundays, the
national days "y en cada Departamento la fecha de su efeméride", and its
article 71 forbade any authority to suspend work on other days; the
decree is known here from the preambles of the later decrees that quote
it [bo-dp-205] [bo-ds-4219]. Decreto Supremo 2750 of 2016 is the
national calendar the table carries, and Decreto Supremo 5019 of
13 September 2023 extended its Sunday rule to the departmental
efemérides: one that falls on a Sunday is kept on the Monday [bo-ds-5019].
No instrument read lists the nine days; each is dated by the instruments
that name it — a decree that moves it for one year, a law that creates
it, and before 1985 the law or decree that declared it a holiday for its
year alone — and by the Ministry of Labour's notice each year, which is
published as PDF.

## How it works

Decreto Presidencial 205 of 13 July 2009 made 15 July 2009, for the
bicentenary of La Paz's revolution of 1809, a departmental holiday "por
única vez", and added that "queda vigente el feriado departamental de La
Paz del día 16 de julio" [bo-dp-205]. So `BOLIVIA` asked for `BO-L` gives
15 and 16 July 2009, and 16 July every year after, days off in the
department; asked for `BO-C`, Cochabamba, it gives neither, and gives
Cochabamba's own efeméride on 14 September. Oruro's
10 February falls on a Sunday in 2030, and from 2024 the Sunday rule
reaches it: `BO-O` in 2030 gives Monday 11 February as its substitute.

## What is carried

- **The national days** of Decreto Supremo 2750, nationwide, from 2017,
  its first whole year; every earlier year is a gap. For 2026, Decreto
  Supremo 5521 of 13 January 2026 moves 22 January to Friday 23 January and
  Sunday 21 June to Monday 22 June (art. 4) and makes Friday 5 June and
  Friday 7 August national holidays (art. 3) [bo-ds-5521-pixilegal]
  [bo-ds-5521-infoleyes]; the table has those four dates, and neither 22
  January nor 21 June, in 2026. The decrees that move or add days in other
  years, 2017 to 2025 and 2027 on, were not read: each of those years has a
  gap, "the year's decree of moved and additional holidays".
- **Four departments' days**, `Kind::Public`, from the first instrument
  read that dates them, with the Sunday rule from 2024; the years before
  are a gap where that instrument presupposes the day (La Paz, Oruro,
  Tarija) and absent where it sets it (Pando):
  La Paz's 16 July from 2009, Oruro's 10 February from 2014 (6 February in
  2013), Tarija's 15 April from 2020 and Pando's 11 October from 2025.
- **Santa Cruz's Día Departamental de la Autonomía**, 4 May, from 2011,
  as `Kind::Observance`: its law makes it a departmental holiday "sin
  suspensión de actividades" [bo-scz-ld-21]; the years before are a gap,
  the law repealing a departmental decree of 2009 that was not read.
- **Six departments' efemérides dated before 1985**, `Kind::Public`,
  from 1986, the first whole year of Decreto Supremo 21060 of 29 August
  1985, whose article 67 makes each department's efeméride a holiday every
  year, with the Sunday rule from 2024; the years before a gap. Before
  1985 each was declared a holiday for one year at a time, by a law or a
  supreme decree for "the 25th of this month", and those declarations are
  what date the day: Chuquisaca's 25 May, Cochabamba's 14 September,
  Potosí's 10 November, Santa Cruz's and Pando's 24 September and Beni's
  18 November. The years of those declarations are not carried on their
  own: each is a gap with the rest of the years before 1986.
- **Not yet carried, and why:** Cochabamba's 14 August, which a
  departmental law of 2019 or 2020 adds beside its 14 September, found in
  no form and known from the press, and Beni's 10 November, which
  Ley Departamental 003 of 2010 adds "de cada año", published as PDF and
  known from the press: each a gap from its year, `Rule::UNREAD` scoped to
  the department. Also not carried: the Gran Chaco region's 12 August (Decreto Supremo
  4568), a region smaller than a department; the bicentenary days of
  2025 (Decreto Supremo 5328) and Santa Cruz's extra day of 25 September
  2026 (Decreto Supremo 5711), known from the press only.

### The nine departments

| Code | Department | Day carried | Instrument | First year, and the years before | Not carried |
| --- | --- | --- | --- | --- | --- |
| BO-B | Beni | 18 November | Decreto Supremo 4774 of 17 November 1957, and the Ley de 17 de noviembre de 1942 for the centenary [bo-beni-1942-1957] | 1986; earlier years: gap | 10 November, a gap from 2010 |
| BO-C | Cochabamba | 14 September | Ley de 14 de septiembre de 1950, and Decreto Supremo 27723 of 2004, whose preamble dates the efeméride [bo-cochabamba-1950-2004] | 1986; earlier years: gap | 14 August, a gap from 2019 |
| BO-H | Chuquisaca | 25 May | Decreto Supremo 1186 of 24 May 1948 and Decreto Supremo 8772 of 1969 [bo-chuquisaca-1948-1969] | 1986; earlier years: gap | |
| BO-L | La Paz | 16 July; 15 July 2009 once | Decreto Presidencial 205 of 13 July 2009 [bo-dp-205] | 2009; earlier years: gap | |
| BO-N | Pando | 24 September; Battle of Bahía, 11 October | Decreto Supremo 8934 of 1969 and the Ley de 21 de septiembre de 1950 [bo-santa-cruz-pando-1950-1969]; Ley 1606 of 13 November 2024 [bo-l-1606] | 24 September 1986, earlier years: gap; the Battle of Bahía established 2025 (Ley 1606 of 2024) | |
| BO-O | Oruro | 10 February; 6 February in 2013 | Decreto Supremo 1484 of 6 February 2013 [bo-ds-1484] | 2013; earlier years: gap | |
| BO-P | Potosí | 10 November | Ley de 20 de octubre de 1909, and Decretos Supremos 1797 of 1949 and 3870 of 1954 [bo-potosi-1909-1954] | 1986; earlier years: gap | |
| BO-S | Santa Cruz | 24 September; Día Departamental de la Autonomía, 4 May, an observance | Decreto Supremo 8933 of 1969 and the Ley de 21 de septiembre de 1950 [bo-santa-cruz-pando-1950-1969]; Ley Departamental 21 of 23 September 2010 [bo-scz-ld-21] | 24 September 1986, 4 May 2011; earlier years: gap | |
| BO-T | Tarija | 15 April | Decreto Supremo 4219 of 14 April 2020 [bo-ds-4219] | 2020; earlier years: gap | the Gran Chaco's 12 August |

## Accuracy

`crates/hc-holiday/tests/local_days.rs` holds each day to its instrument
in its first year and after, nothing before, a gap before La Paz's,
Oruro's and Tarija's and none before Pando's, La Paz's day in La Paz alone,
and the Sunday rule's effect in 2019 and 2030; the six efemérides dated
before 1985 in 1986 and 2026, a gap in 1985, and moved off a Sunday in
2024 (Potosí) and 2025 (Chuquisaca); and the gaps of Cochabamba's
14 August from 2019 and Beni's 10 November from 2010. Every instrument
was read in a copy, secondary: lexivox's, and for Decretos Supremos
1186, 1797, 3870 and 4774 derechoteca's, each quoted as its raw HTML
reads (derechoteca's first through a summarising fetch, then verified
against the raw text the same day); the Gaceta Oficial publishes PDF
only. The efemérides carried
from 1986 take their dates from declarations of 1942 to 1969 and, for
Cochabamba, a preamble of 2004: each day is the anniversary of the
event it keeps, which does not move, but no instrument after 1985 read
names Chuquisaca's, Potosí's, Santa Cruz's, Pando's or Beni's date.
Decreto Supremo 21060 itself is known from the preambles that quote it;
lexivox's page for it shows only its table of contents. Decreto Ley 7317
of 1965, which the Ministry still cites for Cochabamba's day, was seen
only as a summary. That a
day carried from the year of the instrument read is older — La Paz's is a
holiday under Decreto Supremo 21060 of 1985 at the latest — is known, but
no text giving its date before was read.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [bo-dp-205] | La Paz's 16 July and 15 July 2009; article 67 of Decreto Supremo 21060, as its preamble quotes it | Yes, 2026-09-29, lexivox |
| [bo-ds-1484] | Oruro's 10 February and its move in 2013 | Yes, 2026-09-29, lexivox |
| [bo-ds-4219] | Tarija's 15 April | Yes, 2026-09-29, lexivox |
| [bo-l-1606] | Pando's 11 October | Yes, 2026-09-29, lexivox |
| [bo-scz-ld-21] | Santa Cruz's 4 May | Yes, 2026-09-29, lexivox |
| [bo-ds-5521-pixilegal], [bo-ds-5521-infoleyes] | The 2026 moves and additions | Yes, 2026-10-03, in the HTML texts of pixilegal.com and Infoleyes (secondary; the Gaceta Oficial's PDF not read) |
| [bo-ds-5019] | The Sunday rule for the departmental efemérides | Yes, 2026-09-22, lexivox, as the table's `sources` cites it |
| [bo-chuquisaca-1948-1969], [bo-cochabamba-1950-2004], [bo-potosi-1909-1954], [bo-santa-cruz-pando-1950-1969], [bo-beni-1942-1957] | The dates of the six efemérides carried from 1986 | Yes, 2026-09-29, lexivox and derechoteca |

## Code

`crates/hc-holiday/src/countries/bolivia.rs`: the national rules, then the
departments', built by `departmental`, the six dated before 1985 read from
`DS_21060_FIRST_YEAR`, and the two gaps built by `undated`, with one
region and one citation constant per department. Anchors: the Bolivian tests in
`crates/hc-holiday/tests/local_days.rs`.
