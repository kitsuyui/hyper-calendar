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
day has wherever it is a holiday, and lists no Sunday.

## How it works

A state's day, as carried, is a day the Reserve Bank lists at the state's
office, or at every one of the state's offices where it has several. The
description is the Reserve Bank's, whole.

**Worked example: Maharashtra, 2025 and 2026.** The Reserve Bank has three
offices in Maharashtra: Mumbai, Belapur and Nagpur.

1. *19 February*, "Chhatrapati Shivaji Maharaj Jayanti", is listed at all
   three in both years, and at no office outside the state: it is
   Maharashtra's, and a business day in Uttar Pradesh.
2. *1 May 2026* is listed at all three with the description "Maharashtra
   Din/Buddha Pournima/May Day (Labour Day)/Birth Anniversary of Pandit
   Raghunath Murmu", four names joined for all the offices that keep the
   day. The list does not say which name is whose, so the description is
   carried whole.
3. *Id-E-Milad 2025* is listed on 5 September at Belapur and Nagpur and on
   8 September at Mumbai: the state kept the day on the 5th and moved it to
   the 8th for Mumbai. Neither date is at every office, and a state code
   cannot scope a district, so neither is carried.

## What is carried

The thirteen largest states, for 2025 and 2026: each day as above, as a
rule of `INDIA` for its year, scoped to its state and of `Kind::Bank`. The
years before 2025 are not carried; 2027, which the Reserve Bank had not
published, and every later year is a gap, reported once per state as
"Holidays under the Negotiable Instruments Act". A day the nationwide table
also has, as Republic Day, is both the nationwide entry and the state's.

An election day at a state's one office is left out, since the list does
not say how far in the state it runs: the Legislative Assembly polls of
Tamil Nadu (23 April 2026) and West Bengal (29 April 2026), and "Elections
in the jurisdiction of Jaipur Municipal Corporation" (11 September 2026).
Where a state has several offices, an election day at every one of them is
carried.

| Code | State or union territory | Reserve Bank office | Days carried, 2025 and 2026 |
| --- | --- | --- | --- |
| IN-AN | Andaman and Nicobar Islands | none | not carried: no office |
| IN-AP | Andhra Pradesh | Vijayawada | 16 and 20 |
| IN-AR | Arunachal Pradesh | Itanagar | not yet carried |
| IN-AS | Assam | Guwahati | not yet carried |
| IN-BR | Bihar | Patna | 18 and 20 |
| IN-CG | Chhattisgarh | Raipur | not yet carried |
| IN-CH | Chandigarh | Chandigarh | not yet carried: whose days the office's list gives — the union territory's, Punjab's or Haryana's, all three governments sitting in the city — was not established |
| IN-DH | Dadra and Nagar Haveli and Daman and Diu | none | not carried: no office |
| IN-DL | Delhi | New Delhi | not yet carried |
| IN-GA | Goa | Panaji | not yet carried |
| IN-GJ | Gujarat | Ahmedabad | 19 and 19 |
| IN-HP | Himachal Pradesh | Shimla | not yet carried |
| IN-HR | Haryana | none of its own (Chandigarh) | not carried, as for IN-CH |
| IN-JH | Jharkhand | Ranchi | not yet carried |
| IN-JK | Jammu and Kashmir | Jammu, Srinagar | not yet carried |
| IN-KA | Karnataka | Bengaluru | 21 and 22 |
| IN-KL | Kerala | Thiruvananthapuram, Kochi | 16 and 22 |
| IN-LA | Ladakh | none | not carried: no office |
| IN-LD | Lakshadweep | none | not carried: no office |
| IN-MH | Maharashtra | Mumbai, Belapur, Nagpur | 18 and 22 |
| IN-ML | Meghalaya | Shillong | not yet carried |
| IN-MN | Manipur | Imphal | not yet carried |
| IN-MP | Madhya Pradesh | Bhopal | 18 and 21 |
| IN-MZ | Mizoram | Aizawl | not yet carried |
| IN-NL | Nagaland | Kohima | not yet carried |
| IN-OD | Odisha | Bhubaneswar | 21 and 20 |
| IN-PB | Punjab | none of its own (Chandigarh) | not carried, as for IN-CH |
| IN-PY | Puducherry | none | not carried: no office |
| IN-RJ | Rajasthan | Jaipur | 17 and 16 |
| IN-SK | Sikkim | Gangtok | not yet carried |
| IN-TN | Tamil Nadu | Chennai | 20 and 22 |
| IN-TR | Tripura | Agartala | not yet carried |
| IN-TS | Telangana | Hyderabad | 18 and 20 |
| IN-UK | Uttarakhand | Dehradun | not yet carried |
| IN-UP | Uttar Pradesh | Lucknow, Kanpur | 23 and 25 |
| IN-WB | West Bengal | Kolkata | 25 and 27 |

"Not yet carried" means the Reserve Bank's list for the office was read
for 2019 to 2026 and the state's days are waiting on the same treatment;
"no office" means the list has no office there, and the state's
notification, which was not read, is the source to read. The codes are
CLDR 48's regular ones: Telangana is `IN-TS`, `IN-TG` being deprecated,
and so are `IN-OR`, `IN-CT` and `IN-UT` beside `IN-OD`, `IN-CG` and
`IN-UK`.

Not carried:

- *The states' own lists* for their offices, their general holidays, which
  the notifications give and which were not read.
- *A holiday on a Sunday*, which the Reserve Bank does not list.
- *A day at one office of several*, as the worked example's.
- *The years before 2025* in the thirteen states: the lists back to 2019
  were read, and carrying them is a matter of size, not of a missing
  source.

## Accuracy

The dates are the Reserve Bank's, and no computation is involved: the test
checks a day of each kind against the list, the scoping, the gap after
2026 and that no day carried is a Sunday. Where the list is wrong the table
is. The description can name another state's festival beside the state's
own, as the worked example shows; it is the list's, not a claim that the
state keeps that festival.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [rbi-ni-act-holidays] | Every day carried, its description, the offices | Yes, 2026-09-29, each office's list for 2019 to 2026 through the page's form |
| `hc-holiday`'s India table | The DoPT's nationwide list, beside which the states' days stand | This repository |

## Code

`crates/hc-holiday/src/countries/india.rs`: `STATE_DAYS`, built by `nia` for
each day and `not_read` for each state's gap, joined to the nationwide rules
of `asia.rs` by `countries::joined` into the `RULES` that `INDIA`
evaluates.

Anchors: `crates/hc-holiday/tests/india_states.rs`,
`a_state_keeps_the_days_the_reserve_bank_lists_for_it`,
`a_state_s_day_is_its_own`,
`the_states_are_carried_for_2025_and_2026_and_are_a_gap_after` and
`no_state_day_falls_on_a_sunday`.
