# New Zealand's provincial anniversary days

Backs the regions of `hc-holiday`'s `NEW_ZEALAND` table: the anniversary
day of each of the nineteenth-century provinces, scoped to the ISO 3166-2
region that bears its name.

## What it is

The Holidays Act 2003 lists among New Zealand's public holidays the
anniversary day of each province, but does not date it. Employment New
Zealand states the position: the Act "lists the holidays but does not
generally set dates for them", and the provincial anniversary days "are
observed locally by custom and practice and are generally prescribed by
regional or city councils" [employment-nz-anniversary-dates]. The provinces
are those of the nineteenth century, abolished in 1876, and they "are not
determined by present-day districts or regions".

Employment New Zealand lists the day each province observes, year by year:
2026 and 2027 on its current page, 2010 to 2025 on its page of previous
years [employment-nz-anniversary-dates, employment-nz-previous-years]. The
Act itself was not read: legislation.govt.nz and NZLII refused this
session's requests.

## How it works

"Provincial anniversary days are generally observed on the Monday nearest
to the actual day" [employment-nz-anniversary-dates]. A date three days or
fewer after a Monday goes back to it; one three days or fewer before a
Monday goes forward. The exceptions the list states are Taranaki's second
Monday of March, "to avoid Easter"; Hawke's Bay's Friday before Labour Day,
itself the fourth Monday of October; Marlborough's first Monday after
Labour Day; Christchurch Show Day, "the second Friday after the first
Tuesday in November", for northern and central Canterbury; South
Canterbury's Dominion Day, the fourth Monday of September; and Southland's
Easter Tuesday.

**Worked example: 2026.**

1. *Auckland*, 29 January. 29 January 2026 is a Thursday: the Monday before
   is three days back, the Monday after four days on, so the day is Monday
   26 January, as the list gives it.
2. *Nelson*, 1 February, a Sunday: Monday 2 February.
3. *Canterbury.* The first Tuesday of November 2026 is the 3rd; the Fridays
   after it are the 6th and the 13th, so Show Day is Friday 13 November.
4. *Southland.* Easter 2026 is 5 April, so Easter Tuesday is 7 April.
5. *Hawke's Bay.* Labour Day is Monday 26 October, so the day is Friday
   23 October, and Marlborough's is Monday 2 November.

## What is carried

Each province's rule, as a rule of `NEW_ZEALAND` scoped to its region, from
2010, the first year of the list read. None moves off a weekend, the
Monday and Friday rules keeping each off it. Every day is `Kind::Public`.

| Region | Province | Day | Rule | First year |
| --- | --- | --- | --- | --- |
| NZ-AUK | Auckland | Auckland Anniversary Day, 29 January | the Monday nearest | 2010 |
| NZ-BOP | Bay of Plenty | not carried: no province of that name, and no source read says which province's day the region keeps | | |
| NZ-CAN | Canterbury | Canterbury Anniversary Day, Christchurch Show Day | the second Friday after the first Tuesday of November | 2010 |
| NZ-CAN | South Canterbury | Dominion Day, the fourth Monday of September: not carried, as no region code names South Canterbury | | |
| NZ-CIT | Chatham Islands | Chatham Islands Anniversary Day, 30 November | the Monday nearest | 2010 |
| NZ-GIS | Gisborne | not carried, as for Bay of Plenty | | |
| NZ-HKB | Hawke's Bay | Hawke's Bay Anniversary Day, 1 November | the Friday before Labour Day | 2010 |
| NZ-MBH | Marlborough | Marlborough Anniversary Day, 1 November | the first Monday after Labour Day | 2010 |
| NZ-MWT | Manawatū-Whanganui | not carried, as for Bay of Plenty | | |
| NZ-NSN | Nelson | Nelson Anniversary Day, 1 February | the Monday nearest | 2010 |
| NZ-NTL | Northland | not carried, as for Bay of Plenty | | |
| NZ-OTA | Otago | Otago Anniversary Day, 23 March | the Monday nearest | 2010 |
| NZ-STL | Southland | Southland Anniversary Day, 17 January | the Monday nearest in 2010 and 2011, Easter Tuesday from 2012 | 2010 |
| NZ-TAS | Tasman | not carried, as for Bay of Plenty | | |
| NZ-TKI | Taranaki | Taranaki Anniversary Day, 31 March | the second Monday of March | 2010 |
| NZ-WGN | Wellington | Wellington Anniversary Day, 22 January | the Monday nearest | 2010 |
| NZ-WKO | Waikato | not carried, as for Bay of Plenty | | |
| NZ-WTC | West Coast | Westland Anniversary Day, 1 December | the Monday nearest | 2010 |

Not carried, beyond the rows marked:

- *The years before 2010*, for which no list and no council's resolution
  was read.
- *The local variation the list notes*: Westland's day "varies throughout
  Westland, but Greymouth observes the official day", and for Otago "there
  is no easily determined single day of local observance"; the rules are
  the days the list gives.
- *The day each region without a province of its name keeps.* The
  secondary accounts that give Northland, Waikato, Bay of Plenty and
  Gisborne Auckland's day, and Tasman Nelson's, were not taken, as no
  official source read says it.

## Accuracy

The reference is Employment New Zealand's list: every observed day of the
eleven provinces carried, 2010 to 2027, 198 days, is reproduced by the
rules, and the list's weekday agrees with the date in every row. The page
warns that its dates "may contain unintentional errors" and refers the
reader to the local council; no council was read. A rule predicts the years
after 2027, and a council may change its day.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [employment-nz-anniversary-dates] | The rules, the 2026 and 2027 days, the Act's silence on dates, the provinces not being regions | Yes, 2026-09-29 |
| [employment-nz-previous-years] | The days of 2010 to 2025; Southland's change in 2012 | Yes, 2026-09-29 |

## Code

`crates/hc-holiday/src/countries/new_zealand.rs`: `ANNIVERSARY_DAYS`, built
by `anniversary` with the moves of `NEAREST_MONDAY`, joined to the
nationwide rules of `oceania.rs` by `countries::joined` into the `RULES`
that `NEW_ZEALAND` evaluates.

Anchors: `crates/hc-holiday/tests/new_zealand_anniversaries.rs`,
`every_anniversary_day_of_the_list_is_reproduced` (the 198 days),
`no_anniversary_day_is_carried_before_the_list_begins`,
`an_anniversary_day_is_its_region_s_alone` and
`southland_moved_to_easter_tuesday_in_2012`.
