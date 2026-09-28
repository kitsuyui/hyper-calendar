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
it — and by the Ministry of Labour's notice each year, which is
published as PDF.

## How it works

Decreto Presidencial 205 of 13 July 2009 made 15 July 2009, for the
bicentenary of La Paz's revolution of 1809, a departmental holiday "por
única vez", and added that "queda vigente el feriado departamental de La
Paz del día 16 de julio" [bo-dp-205]. So `BOLIVIA` asked for `BO-L` gives
15 and 16 July 2009, and 16 July every year after, days off in the
department; asked for `BO-C`, Cochabamba, it gives neither. Oruro's
10 February falls on a Sunday in 2030, and from 2024 the Sunday rule
reaches it: `BO-O` in 2030 gives Monday 11 February as its substitute.

## What is carried

- **The national days** of Decreto Supremo 2750, nationwide, as before.
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
- **Not yet carried, and why:** the days no instrument read dates. Their
  dates are known from the Ministry of Labour's notices and the press, but
  the instruments that create them were not read:
  - Chuquisaca's 25 May;
  - Cochabamba's 14 September (Decreto Ley 7317 of 1965, cited) and
    14 August (a departmental law of 2019–2020, cited);
  - Potosí's 10 November;
  - Santa Cruz's 24 September and Pando's 24 September;
  - Beni's 18 November, and 10 November (a departmental law of 2010,
    reported).
  Also not carried: the Gran Chaco region's 12 August (Decreto Supremo
  4568), a region smaller than a department; the bicentenary days of
  2025 (Decreto Supremo 5328) and Santa Cruz's extra day of 25 September
  2026 (Decreto Supremo 5711), known from the press only.

### The nine departments

| Code | Department | Day carried | Instrument | First year, and the years before | Not carried |
| --- | --- | --- | --- | --- | --- |
| BO-B | Beni | none | | | 18 November, 10 November |
| BO-C | Cochabamba | none | | | 14 September, 14 August |
| BO-H | Chuquisaca | none | | | 25 May |
| BO-L | La Paz | 16 July; 15 July 2009 once | Decreto Presidencial 205 of 13 July 2009 [bo-dp-205] | 2009; earlier years: gap | |
| BO-N | Pando | Battle of Bahía, 11 October | Ley 1606 of 13 November 2024 [bo-l-1606] | established 2025 (Ley 1606 of 2024) | 24 September |
| BO-O | Oruro | 10 February; 6 February in 2013 | Decreto Supremo 1484 of 6 February 2013 [bo-ds-1484] | 2013; earlier years: gap | |
| BO-P | Potosí | none | | | 10 November |
| BO-S | Santa Cruz | Día Departamental de la Autonomía, 4 May, an observance | Ley Departamental 21 of 23 September 2010 [bo-scz-ld-21] | 2011; earlier years: gap | 24 September |
| BO-T | Tarija | 15 April | Decreto Supremo 4219 of 14 April 2020 [bo-ds-4219] | 2020; earlier years: gap | the Gran Chaco's 12 August |

## Accuracy

`crates/hc-holiday/tests/local_days.rs` holds each day to its instrument
in its first year and after, nothing before, a gap before La Paz's,
Oruro's and Tarija's and none before Pando's, La Paz's day in La Paz alone,
and the Sunday rule's effect in 2019 and 2030. Every instrument was read in
lexivox's copy, secondary: the Gaceta Oficial publishes PDF only. That a
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
| [bo-ds-5019] | The Sunday rule for the departmental efemérides | Yes, 2026-09-22, lexivox, as the table's `sources` cites it |

## Code

`crates/hc-holiday/src/countries/bolivia.rs`: the national rules, then the
departments', built by `departmental`, with one region and one citation
constant per department. Anchors: the Bolivian tests in
`crates/hc-holiday/tests/local_days.rs`.
