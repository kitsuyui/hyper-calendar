# India's state holidays under the Negotiable Instruments Act

Backs the regions of `hc-holiday`'s `INDIA` table: each state's own days
off for the banks, scoped to its ISO 3166-2 code.

## What it is

India's nationwide holidays are few — Republic Day, Independence Day and
Gandhi Jayanti — and the central government's list of the Department of
Personnel and Training (DoPT) gives the rest for its own offices. Every
other day off is a state's: each state government notifies, every year, the
days that are holidays for the purposes of section 25 of the Negotiable
Instruments Act, 1881, on which the banks in the state close, and usually
the same days as general holidays for its own offices. The notifications
are published in each state's gazette, as PDFs, and none was read.

The Reserve Bank of India publishes the result for the banks: "Holidays
under Negotiable Instruments Act", a list for each of its 34 regional
offices and each year from 2001 [rbi-ni-act-holidays]. A day is listed at
an office when it is such a holiday there. The list gives one description
per date for all the offices together, joining with slashes the names the
day has wherever it is a holiday; of the 842 dates of 2019 to 2026, only
27 December 2024 is described differently at two offices, Bengaluru and
Kohima. It lists no Sunday but one, Shillong's Beh Dienkhlam of 14 July
2019. Asked for all the offices at once, the page gives a table of dates
against offices, with a mark where the day is a holiday and one
description a date, so it does not tell either which name is whose.

## How it works

A state's day, as carried, is a day the Reserve Bank lists at the state's
office, or at every one of the state's offices where it has several.

Its name is the description at the state's office, split at each `/` that
is not inside brackets, and never at a comma or a dash: "Dasara/Dusshera
(Vijaya Dashmi) (Aaso sud-10)/Mahanavami, Ayudhapooja/Durga Puja" has four
parts, "Mahanavami, Ayudhapooja" one of them, and "Dussehra
(Mahanavami/Vijayadashmi)" is one part. A part is left out of the state's
name only where the list itself shows that the office does not keep it:

1. in some year the part is the whole description of a date, and the
   office lists no date of that year that bears it, nor the day before or
   after that date, unless every office that keeps that date keeps the
   next day too: a feast set by the moon's sighting can fall a day apart
   at two offices, under two spellings;
2. the part is in no year the whole description of a date the office
   lists; and
3. a part the office is shown keeping, as the whole description of some
   date it lists, is left in the name.

Otherwise the description is carried whole. Where a state has several
offices, a part stays if it stays at every one. When no part is left out,
the text is the Reserve Bank's as printed, spacing and all ("Losoong /
Namsoong"). A day several states keep under the same name is one rule,
scoped to all of them.

**Worked example: Assam, 15 April 2022.** The description is "Good
Friday/Bengali New Year’s Day (Nababarsha)/Himachal Day/Vishu/Bohag Bihu",
listed at 28 offices, Guwahati among them.

1. "Good Friday" is the whole description of 19 April 2019, which Guwahati
   does not list; Guwahati lists no other date of 2019 that bears it, and
   neither 18 nor 20 April. Guwahati lists no Good Friday in any year but
   2022's, which shares its date with Bohag Bihu. The part is left out.
2. "Bohag Bihu" is the whole description of 16 April 2022, which Guwahati
   lists: Assam keeps it, and the third condition holds.
3. Nothing shows Guwahati not keeping the other three parts, so they stay:
   Assam's day is "Bengali New Year’s Day (Nababarsha)/Himachal Day/Vishu/
   Bohag Bihu". The same description is Madhya Pradesh's whole, Bhopal
   keeping Good Friday every year.

**Worked example: Maharashtra, 1 May 2026.** The description is
"Maharashtra Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of
Pandit Raghunath Murmu", listed at 28 offices, Mumbai, Belapur and Nagpur
among them; Bhubaneswar has no holiday in May 2026. The list shows none of
the four parts as another state's at the three offices, so by the list
alone the day keeps its whole description. Maharashtra's own list for
2026 (PHD-1125/C.R.199/Japuk (29) of 5 December 2025), as The Live Nagpur
reports it [livenagpur-maharashtra-holidays-2026], names "Maharashtra Din"
and "Buddha Purnima" on 1 May, so the day is "Maharashtra Din/Buddha
Pournima", in the Reserve Bank's spelling. For 2026 each of Maharashtra's
days is the parts its list names: "Gudhi Padwa" of "Gudhi Padwa/Ugadi
Festival/Telugu New Year's Day/Sajibu Nongmapanba (Cheiraoba)/1st
Navratra" on 19 March, "Dr. Babasaheb Ambedkar Jayanti" alone on 14 April,
and so on. Its Moharram of 26 June, "Muharram (Yaom-EShahadath)/Last Day
of Moharam/Ashoora", matches none of the parts, and stays as the list
gives it. The election day of 15 January 2026, which the state's list does
not have, is "Election to Municipal Corporations in Maharashtra", the part
that names the state.

**Worked example: Maharashtra, Id-E-Milad 2025.** It is listed on 5
September at Belapur and Nagpur and on 8 September at Mumbai: the state
kept the day on the 5th and moved it to the 8th for Mumbai. Neither date is
at every office, and a state code cannot scope a district, so neither is
carried.

## What is carried

The twenty-eight states and union territories with an office of their own
but Chandigarh, for 2019 to 2026: each day as above, as a rule of `INDIA`
for its year, scoped to the states that keep it under that name and of
`Kind::Bank`.
Andhra Pradesh's, Arunachal Pradesh's and Nagaland's are from 2023, the
first year the offices at Vijayawada, Itanagar and Kohima have a list.
Every year before a state's first, and 2027, which the Reserve Bank had
not published, and every later year, is a gap, reported once per state as
"Holidays under the Negotiable Instruments Act": the days are declared
year by year, so a year whose list was not read has none that can be
given, and must not look like a year without them. A day the nationwide
table also has, as Republic Day, is both the nationwide entry and the
state's.

An election day at a state's one office is left out, since the list does
not say how far in the state it runs: the Legislative Assembly polls of
Tamil Nadu (23 April 2026), West Bengal (29 April 2026), Assam (6 April
2021), Uttarakhand (14 February 2022), Chhattisgarh (17 November 2023),
Mizoram (7 November 2023) and Jharkhand (13 November 2024), the municipal
and local polls of Jaipur (11 September 2026), Tripura (25 November 2021),
Shimla (2 May 2023), Nagaland (26 June 2024), Arunachal Pradesh (15
December 2025) and Jharkhand (23 February 2026), Shillong's "General
Elections, 2025" (21 February 2025), and
the general elections of 2019 and 2024 at the one-office states. Where a
state has several offices, an election day at every one of them is
carried, and one at some of them is not, as Jammu and Kashmir's Assembly
polls, listed at Srinagar on 25 September 2024 and at Jammu on 1 October.

| Code | State or union territory | Reserve Bank office | Days carried, 2019 to 2026 |
| --- | --- | --- | --- |
| IN-AN | Andaman and Nicobar Islands | none | not carried: no office |
| IN-AP | Andhra Pradesh | Vijayawada | —, —, —, —, 14, 20, 16, 20 |
| IN-AR | Arunachal Pradesh | Itanagar | —, —, —, —, 16, 16, 17, 18 |
| IN-AS | Assam | Guwahati | 17, 18, 17, 15, 18, 19, 19, 19 |
| IN-BR | Bihar | Patna | 18, 19, 21, 16, 19, 20, 18, 20 |
| IN-CG | Chhattisgarh | Raipur | 16, 16, 17, 14, 19, 19, 17, 17 |
| IN-CH | Chandigarh | Chandigarh | not yet carried: whose days the office's list gives — the union territory's, Punjab's or Haryana's, all three governments sitting in the city — was not established |
| IN-DH | Dadra and Nagar Haveli and Daman and Diu | none | not carried: no office |
| IN-DL | Delhi | New Delhi | 14, 14, 15, 12, 17, 16, 14, 16 |
| IN-GA | Goa | Panaji | 17, 14, 16, 14, 17, 17, 15, 17 |
| IN-GJ | Gujarat | Ahmedabad | 19, 20, 18, 17, 18, 17, 19, 19 |
| IN-HP | Himachal Pradesh | Shimla | 16, 15, 17, 16, 19, 18, 18, 19 |
| IN-HR | Haryana | none of its own (Chandigarh) | not carried, as for IN-CH |
| IN-JH | Jharkhand | Ranchi | 23, 22, 24, 21, 24, 22, 24, 21 |
| IN-JK | Jammu and Kashmir | Jammu, Srinagar | 21, 22, 22, 20, 24, 23, 19, 21 |
| IN-KA | Karnataka | Bengaluru | 25, 20, 22, 18, 20, 23, 21, 22 |
| IN-KL | Kerala | Thiruvananthapuram, Kochi | 17, 16, 21, 17, 19, 18, 16, 22 |
| IN-LA | Ladakh | none | not carried: no office |
| IN-LD | Lakshadweep | none | not carried: no office |
| IN-MH | Maharashtra | Mumbai, Belapur, Nagpur | 23, 21, 23, 19, 23, 22, 18, 22 |
| IN-ML | Meghalaya | Shillong | 21, 25, 19, 18, 19, 25, 22, 24 |
| IN-MN | Manipur | Imphal | 19, 20, 22, 19, 21, 25, 20, 19 |
| IN-MP | Madhya Pradesh | Bhopal | 16, 18, 16, 16, 18, 19, 18, 21 |
| IN-MZ | Mizoram | Aizawl | 18, 18, 18, 18, 22, 22, 24, 26 |
| IN-NL | Nagaland | Kohima | —, —, —, —, 14, 16, 16, 15 |
| IN-OD | Odisha | Bhubaneswar | 17, 18, 16, 18, 19, 24, 21, 20 |
| IN-PB | Punjab | none of its own (Chandigarh) | not carried, as for IN-CH |
| IN-PY | Puducherry | none | not carried: no office |
| IN-RJ | Rajasthan | Jaipur | 15, 15, 16, 15, 17, 17, 17, 16 |
| IN-SK | Sikkim | Gangtok | 25, 25, 26, 26, 25, 23, 27, 27 |
| IN-TN | Tamil Nadu | Chennai | 20, 20, 22, 18, 22, 22, 20, 22 |
| IN-TR | Tripura | Agartala | 15, 18, 22, 16, 19, 23, 22, 24 |
| IN-TS | Telangana | Hyderabad | 20, 22, 22, 17, 22, 22, 18, 20 |
| IN-UK | Uttarakhand | Dehradun | 17, 20, 19, 17, 21, 22, 22, 21 |
| IN-UP | Uttar Pradesh | Lucknow, Kanpur | 21, 21, 23, 20, 24, 24, 23, 25 |
| IN-WB | West Bengal | Kolkata | 21, 21, 23, 17, 25, 24, 25, 27 |

"No office" means the list has no office there, and the state's
notification, which was not read, is the source to read. The codes are
CLDR 48's regular ones: Telangana is `IN-TS`, `IN-TG` being deprecated,
and so are `IN-OR`, `IN-CT` and `IN-UT` beside `IN-OD`, `IN-CG` and
`IN-UK`.

Not carried:

- *The states' own lists* for their offices, their general holidays, which
  the notifications give and which were not read.
- *A holiday on a Sunday*, which the Reserve Bank does not list, but for
  Shillong's of 14 July 2019.
- *Which of a joined description's parts is a state's*, where the list
  does not show it and the state's own list was not read: a day carried
  whole can name another state's festival beside the state's own, as
  Karnataka's and Uttarakhand's 1 November 2025, "Kannada
  Rajyothsava/Igas-Bagwal". Maharashtra's lists for 2019 to 2025, and
  every other state's, are gazette PDFs and were not read; the secondary
  reports of Maharashtra's earlier lists found were not reliable, one of
  2019 dating Diwali Amavasya to 26 October and another of 2021 Buddha
  Purnima to 8 April, neither of which the Reserve Bank lists.
- *A day at one office of several*, as the worked example's.
- *The years before 2019*, and before 2023 in Andhra Pradesh: the
  Reserve Bank's lists for them were not read, and are reported gaps, back
  to the year the state was formed — Telangana 2014 (the Andhra Pradesh
  Reorganisation Act, 2014, commenced 2 June 2014), Gujarat and
  Maharashtra 1960 (the Bombay Reorganisation Act, 1960, in effect 1 May
  1960), Kerala and Andhra Pradesh 1956 (the States Reorganisation Act,
  1956, effective 1 November 1956), Chhattisgarh, Uttarakhand and
  Jharkhand 2000 (formed 1, 9 and 15 November 2000), Sikkim 1975 (a state
  of India from 26 April 1975), Meghalaya, Mizoram and Arunachal Pradesh
  1971 (the North-Eastern Areas (Reorganisation) Act, 1971, in effect 30
  December 1971), Nagaland 1963 (inaugurated 1 December 1963) and Goa 1961
  (under Indian rule from 19 December 1961) — or for the older states to
  1882, the Negotiable Instruments Act, 1881, having commenced on 1 March
  1882 (Wikipedia's pages on each Act and "Maharashtra Day", and
  [wikipedia-indian-state-formation], secondary, read 2026-09-29). Assam,
  Delhi, Himachal Pradesh, Jammu and Kashmir, Manipur and Tripura were
  provinces or princely states before they were states, and the year each
  first had holidays under the Act was not established, so their gap runs
  back to 1882 too. Before that year the lists are absent. The states and
  union territories not carried keep the national days and report their
  own as a gap.

## Accuracy

The dates are the Reserve Bank's, and no computation is involved: the test
checks a day of each kind against the list, the scoping, the gap after
2026 and that no day carried is a Sunday but the list's one. Where the list
is wrong the table is. A name is the list's text, less the parts the list
shows the state not keeping; the three conditions leave a part in whenever
the list does not decide, so a name carried whole can still name another
state's festival beside the state's own. It is the list's, not a claim
that the state keeps that festival. Of the 4,139 state days carried, 322
have parts left out, 12 of them Maharashtra's for 2026 by its own list.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [rbi-ni-act-holidays] | Every day carried, its description, the offices, and which parts of a description the list shows a state not keeping | Yes, 2026-09-29, each office's list for 2019 to 2026 through the page's form, and 2019 to 2024 again the same day, every row the same; the all-offices table for May 2026 and the Mumbai and Bhubaneswar lists for May 2026 again the same day |
| [livenagpur-maharashtra-holidays-2026] | The names of Maharashtra's days in 2026 | Yes, 2026-09-29, secondary; the notification it reports, a gazette PDF, not read |
| [wikipedia-indian-state-formation] | The years the newer states and union territories were formed, before which their holidays are absent | Yes, 2026-09-29, secondary |
| `hc-holiday`'s India table | The DoPT's nationwide list, beside which the states' days stand | This repository |

## Code

`crates/hc-holiday/src/countries/india.rs`: `STATE_DAYS`, built by
`lists_read` for each state's years, a gap outside those read from the
state's formation, and by `nia` for each day and the states that keep it
under that name, joined to the nationwide rules of `asia.rs` by
`rule::joined` into the `RULES` that `INDIA` evaluates.

`STATE_DAYS` is generated by `scripts/india-rbi-holidays.py`, which asks
the Reserve Bank's form for each office's list for each year, 272
requests, reads them over HTTP into memory, saving nothing, and applies
this document's rules: the days at every one of a state's offices, the
elections left out, the parts left out of a name, and Maharashtra's names
for 2026 from the table in the script. `python3
scripts/india-rbi-holidays.py` rewrites the table, `--check` exits 1 when
it is stale, and `--renamed` prints each name cut beside the description
it was cut from. The script needs the network and cannot run offline,
which is why CI does not run it. The rest of `india.rs`, its documentation,
the states' constants, `nia` and `lists_read`, is written by hand, and a
state added to the script needs its constants there. On 2026-09-29 the
script's `--check` passed against the live lists.

Anchors: `crates/hc-holiday/tests/india_states.rs`,
`a_state_keeps_the_days_the_reserve_bank_lists_for_it`,
`a_state_s_day_is_its_own`,
`the_states_are_carried_for_2019_to_2026_and_are_a_gap_before_and_after`,
`no_state_day_falls_on_a_sunday_but_the_one_the_list_has` and
`a_name_keeps_the_parts_the_list_does_not_show_another_state_s`.
