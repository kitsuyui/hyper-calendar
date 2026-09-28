# The provincial days of Solomon Islands and Vanuatu

Backs the regions of `hc-holiday`'s `SOLOMON_ISLANDS` and `VANUATU` tables:
each province's own day off, scoped to its ISO 3166-2 code.

## What it is

Both countries are divided into provinces, and each province keeps a day
of its own beside the national holidays of its Public Holidays Act.

- **Solomon Islands.** Section 6 of the Public Holidays Act (Cap. 151) lets
  the Minister appoint public holidays for a province, and the Minister for
  Home Affairs appoints the provinces' days each year, in the same notice
  in the Gazette as the national days. The notice for 2026, dated
  3 November 2025, appeared in the Extra-Ordinary Gazette of 5 November
  2025; it was read as the Island Sun reported it, the Gazette being a PDF
  [islandsun-sb-holidays-2026]. The Act itself is `SOLOMON_ISLANDS`'s
  source and was read by that table's author.
- **Vanuatu.** The Public Holidays Act [Cap. 114] does not name the
  provinces' days. The Government's list of holidays gives six of type
  "Provincial Holiday", one for each province, on a fixed date
  [govvu-holidays].

## How it works

The Solomon Islands notice gives each province's day, and the day it is to
be observed when that differs.

**Worked example: Guadalcanal and Malaita, 2026.** The notice gives
Guadalcanal Province's holiday as 1 August, "with July 31 to be observed as
the public holiday", and Malaita's as 15 August with 14 August observed.
1 and 15 August 2026 are Saturdays; the observed days are the Fridays
before. The table carries the observed days, 31 July and 14 August, and
Saturday 1 August is not a holiday in Guadalcanal. The notices of 2018 and
2020 that `SOLOMON_ISLANDS` cites keep a Saturday national holiday on the
Friday before in the same way, a practice the Act does not state.

A Vanuatu provincial day is its fixed date every year: Shefa Day is
18 June, a Thursday in 2026.

## What is carried

Every day below, as a rule of the country's table scoped to its province,
of `Kind::Public`. Solomon Islands' days are the notice's for 2026 alone:
a later year reports each as a gap, since the days are appointed each
year, and the years before the notice read are not carried. Vanuatu's are
carried from 2020, the year the Government's list read gives, its Good
Friday being 10 April and its Ascension 21 May; no earlier list was read.
Neither country's days move off a weekend: Solomon Islands' notice gives
the observed day itself, and Vanuatu's section 3 Sunday rule is for the
Act's own holidays.

| Code | Province | Day | Instrument | First year |
| --- | --- | --- | --- | --- |
| SB-CE | Central | 29 June 2026 | notice of 3 November 2025 | 2026 |
| SB-CH | Choiseul | 25 February 2026 | notice of 3 November 2025 | 2026 |
| SB-CT | Capital Territory (Honiara) | none: the notice appoints no day | | |
| SB-GU | Guadalcanal | 31 July 2026, for 1 August | notice of 3 November 2025 | 2026 |
| SB-IS | Isabel | 2 June 2026 | notice of 3 November 2025 | 2026 |
| SB-MK | Makira-Ulawa | 3 August 2026 | notice of 3 November 2025 | 2026 |
| SB-ML | Malaita | 14 August 2026, for 15 August | notice of 3 November 2025 | 2026 |
| SB-RB | Rennell and Bellona | 20 July 2026 | notice of 3 November 2025 | 2026 |
| SB-TE | Temotu | 8 June 2026 | notice of 3 November 2025 | 2026 |
| SB-WE | Western | 7 December 2026 | notice of 3 November 2025 | 2026 |
| VU-MAP | Malampa | Malampa Day, 10 October | the Government's list | 2020 |
| VU-PAM | Penama | Penama Day, 16 September | the Government's list | 2020 |
| VU-SAM | Sanma | Sanma Day, 24 September | the Government's list | 2020 |
| VU-SEE | Shefa | Shefa Day, 18 June | the Government's list | 2020 |
| VU-TAE | Tafea | Tafea Day, 8 October | the Government's list | 2020 |
| VU-TOB | Torba | Torba Day, 2 October | the Government's list | 2020 |

Not carried:

- *Solomon Islands' days in other years.* The notices of 2017 and 2019,
  for 2018 and 2020, which `SOLOMON_ISLANDS` cites, are PDFs whose
  provincial days were not read; no other year's notice was found.
- *What the provinces' own instruments say.* No provincial ordinance of
  Solomon Islands and no provincial council's resolution of Vanuatu was
  read, and the Vanuatu Department of Labour's brochure, which lists the
  same six days, is a PDF read only by the national table's author.

## Accuracy

Nothing is computed: each date is the source's. The Solomon Islands days
are a newspaper's report of the Gazette, secondary, and the Vanuatu days a
list of the Government's that names no instrument.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [islandsun-sb-holidays-2026] | Solomon Islands' provincial days of 2026 and the observed days | Yes, 2026-09-29 |
| [govvu-holidays] | Vanuatu's six provincial days and the year of the list | Yes, 2026-09-29 |

## Code

`crates/hc-holiday/src/countries/solomon_islands.rs`: the listing
`APPOINTED` and `PROVINCIAL_DAYS`, built by `provincial`;
`crates/hc-holiday/src/countries/vanuatu.rs`: `PROVINCIAL_DAYS`, built by
`provincial`. Each is joined to the nationwide rules of `oceania.rs` by
`countries::joined` into the `RULES` the country's table evaluates.

Anchors: `crates/hc-holiday/tests/provincial_days.rs`,
`solomon_islands_keeps_the_provincial_days_of_2026` and
`vanuatu_keeps_each_provincial_day_in_its_province_from_2020`.
