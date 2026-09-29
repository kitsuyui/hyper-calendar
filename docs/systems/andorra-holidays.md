# Andorra's national and parish holidays

Andorra's work calendar is a Government decree, one a year, and each of
its seven parishes adds days of its own. This document covers
`hc-holiday`'s `ANDORRA` table and its regions, the parishes `AD-02` to
`AD-08`.

## What it is

Llei 31/2018 de relacions laborals, article 62, gives workers the
holidays of the work calendar, which the Government approves each year by
decree. The decree's article 3 adds "els dies que fixin els comuns de les
parròquies fins a un màxim de quatre", counting in parishes divided into
quarts the days of each quart, and says that each comú publishes its days
in the BOPA [ad-calendar-decrees]. The comuns do so each year, in an avís,
decret or edicte of the comú, and keep the days "de compliment obligatori,
retribuïdes i no recuperables" for construction, industry, offices,
banks, transport and the services not tied to tourism; a tourism worker's
day may be moved by agreement [ad-parish-instruments]. Several parishes
keep some days in one village or quarter only: Canillo's veïnats, the
villages of La Massana and Ordino, Encamp's two villages.

## How it works

Sant Julià de Lòria's decree for 2026, of 11 December 2025, fixes its
parish days as Sant Julià, its patron, on 7 January; the Diada de
Canòlich, its patroness, on Saturday 30 May; and the Monday and Tuesday
of its Festa Major, 27 and 28 July. So `ANDORRA` asked for `AD-06` in 2026
gives those four days, off; asked for `AD-07`, Andorra la Vella, it gives
that parish's own, Sant Joan on 24 June and the Festa Major of 1 to
3 August, and not Sant Julià's.

## What is carried

- **The national days** of the Government's calendar, nationwide, as the
  table carried them before this document.
- **Each parish's days kept in the whole parish**, `Kind::Public`, as the
  comú's instruments for 2024, 2025 and 2026 date them: one `Listing` for
  the six parishes that have such days, read by a rule per day for those
  three years. Any other year is a gap until its instrument is read, the
  years before 2024 included: the comuns fixed days then too. Encamp, whose
  instruments for those years keep no day in the whole parish, has a rule
  of no day for 2024–2026 (`Rule::unlisted`), and is a gap in any other
  year like the rest.
- **Not yet carried, and why:** the days a comú keeps in one village,
  quarter or veïnat, which need a scope finer than a parish: all of
  Encamp's (Encamp village's festa del poble and Sant Roc, and Sant Pere
  in Pas de la Casa), and in Canillo, La Massana and Ordino all but the
  day in the table. The instruments before 2024 were not read.

### The seven parishes

Read on 2026-09-29 in the BOPA's HTML files. The dates are those of the
instruments for 2024, 2025 and 2026, the same each year unless the row
gives three.

| Code | Parish | Days carried | Instrument | First year, and the years before | Not carried |
| --- | --- | --- | --- | --- | --- |
| AD-02 | Canillo | Sant Roc, 16 August | Comú de Canillo, avisos of 30 November 2023, 10 October 2024 and 4 December 2025 | 2024; earlier years: gap | the veïnats' days: Sant Pere and El Tarter (29 June), Soldeu, Canillo, Ransol, Aldosa and Els Plans (25 July), Sant Bartomeu de Soldeu (24 August), and from 2026 Prats, El Forn and El Vilar |
| AD-03 | Encamp | none in the whole parish | Comú d'Encamp, avisos of 15 December 2023, 25 November 2024 and 21 November 2025 | 2024–2026; any other year: gap | the festa del poble d'Encamp (three days in June) and Sant Roc, in Encamp village only; Sant Pere, in Pas de la Casa only |
| AD-04 | La Massana | Sant Antoni, 17 January | Comú de la Massana, decrets of 30 November 2023, 13 November 2024 and 13 November 2025 | 2024; earlier years: gap | the villages' festes majors: Sispony, Pal, Anyós, La Massana, Erts, L'Aldosa, Arinsal |
| AD-05 | Ordino | St. Pere, 29 June | Comú d'Ordino, decrets of 22 December 2023, 28 November 2024 and 27 November 2025 | 2024; earlier years: gap | the villages' days: Ordino, Sornàs, Ansalonga, La Cortinada, Llorts |
| AD-06 | Sant Julià de Lòria | Sant Julià, 7 January; Diada de Canòlich (25 May 2024, 31 May 2025, 30 May 2026); Monday and Tuesday of the Festa Major (29–30 July 2024, 28–29 July 2025, 27–28 July 2026) | Comú de Sant Julià de Lòria, decrets of 6 December 2023, 12 December 2024 and 11 December 2025 | 2024; earlier years: gap | the shops' closure on Canòlich, by edicte |
| AD-07 | Andorra la Vella | Festa del Poble (Sant Joan), 24 June; Festa Major (3–5 August 2024, 2–4 August 2025, 1–3 August 2026) | Comú d'Andorra la Vella, edictes of 15 November 2023, 19 November 2024 and 25 November 2025 | 2024; earlier years: gap | |
| AD-08 | Escaldes-Engordany | Sant Miquel d'Engolasters, 8 May (2025 and 2026); the parish's foundation day (16 June 2024, 15 June 2025, 14 June 2026); Sant Jaume, 25 July; Santa Anna, 26 July | Comú d'Escaldes-Engordany, decrets of 5 January 2024, 30 December 2024 and 22 December 2025 | 2024; earlier years: gap | |

## Accuracy

`crates/hc-holiday/tests/local_days.rs` holds a date of every parish day
to its instrument, Escaldes-Engordany's 8 May absent in 2024, nothing in
Encamp or nationwide, every parish complete in 2025 and a gap in 2023
and 2027.
The comuns fix their days one year at a time, and the patterns the dates
show — Canòlich on the last Saturday of May, Andorra la Vella's Festa Major
on the first Saturday to Monday of August, Escaldes-Engordany's foundation
day on the Sunday on or after 14 June under a Consell agreement of 1997
that was not read — are not rules any instrument states, so no year after
2026 is predicted. Two instruments were corrected by errata, Canillo's for
2024 and Ordino's for 2026, in days the table does not carry. The decrees'
limit of four days seems to count the quarters' days; Canillo and La
Massana list more than four in all.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [ad-calendar-decrees] | The Government's decrees 487/2023, 409/2024 and 340/2025, article 3 | Yes, 2026-09-29, BOPA |
| [ad-parish-instruments] | Each comú's instruments for 2024 to 2026 | Yes, 2026-09-29, BOPA |

## Code

`crates/hc-holiday/src/countries/andorra.rs`: the national rules, then
the parishes', built by `parish` over the listing `PARISH_DAYS`, with one
region and one citation constant per parish. Anchors: the Andorran tests
in `crates/hc-holiday/tests/local_days.rs`.
