# Regional weekends: the states and emirates that keep a weekend of their own

Most of a country keeps one weekend, and a holiday table says which. Two
countries have a region whose law gives another: Malaysia's states that
keep Friday, and the Government of Sharjah. This document covers what the
sources read say of them, how the engine scopes a weekend and the way a
holiday is moved off it to a region
([ADR 0015](../adr/0015-a-region-may-keep-a-weekend-of-its-own.md)), and
what was looked for in other countries and not found.

## What it is

The weekend is the days of the week on which the offices a law governs do
not work. A statute or an authority sets it, for a country or for part of
one.

**Malaysia.** The Holidays Act 1951 defines the weekly holiday as Sunday
or, in the States where Friday is observed as the weekly holiday, Friday
[mylaw-holidays-act-1951]. The Unfederated Malay States kept Friday before
independence [parlimen81-weekend-history]. Today Kedah, Kelantan and
Terengganu keep Friday and Saturday, and the other states and the Federal
Territories Saturday and Sunday [mkn-johor-weekend-2025,
wikipedia-public-holidays-malaysia]. Johor changed twice in living memory:

| From | Johor's weekend | Source |
| --- | --- | --- |
| before 1994 | Friday and Saturday, "the rest days that Johor had prior to 1994" | [jakartapost-johor-weekend-2013] |
| 1994 | Saturday and Sunday; the day in 1994 is not given | [jakartapost-johor-weekend-2013] |
| 1 January 2014 | Friday and Saturday, by Sultan Ibrahim's decree reported on 25 November 2013, to let Muslims attend Friday prayers | [jakartapost-johor-weekend-2013, rtm-johor-weekend-2025] |
| 1 January 2025 | Saturday and Sunday, by the Regent's announcement of 7 October 2024 | [rtm-johor-weekend-2025, mkn-johor-weekend-2025] |

Perlis kept Friday before independence and keeps Saturday and Sunday now;
the date it moved is not read (see Accuracy).

**The United Arab Emirates.** The federal government's weekend is
Saturday and Sunday from 1 January 2022 [uae-public-sector-working-hours].
The Sharjah Executive Council gave the Government of Sharjah's offices a
four-day week, Monday to Thursday, with Friday, Saturday and Sunday off,
from the same day, as its circular of December 2021 says
[khaleejtimes-sharjah-weekend-2021, gulfnews-sharjah-weekend-2021]. The
public sector's page says it still holds in August 2026
[uae-public-sector-working-hours]. It is the weekend of the emirate's
government and of its government schools; Sharjah's private sector is under
the federal Labour Law.

## How it works

A weekend policy names its days, the years it is in force, to the day where
the source gives one, and the regions it is the law of. Asked for a day in a
region, the engine takes the policies in force on that day whose regions
contain the region, the nearest winning, and a table's own policy, the one
with no regions, if none does ([ADR 0015](../adr/0015-a-region-may-keep-a-weekend-of-its-own.md)).
A substitution policy is chosen in the same way, by year.

Worked example: Hari Raya Haji 2025 and Awal Muharram 2025 in Kedah.

1. The Prime Minister's Department's list gives Hari Raya Haji on Saturday
   7 June 2025 and Awal Muharram on Friday 27 June 2025.
2. Kedah's weekend from 25 November 2013 is Friday and Saturday. A holiday
   on the weekly holiday is moved to the day after (Holidays Act 1951,
   section 3), and the Saturday is off already, so the Friday holiday is
   moved to Sunday 29 June. The Saturday holiday stays: the sources read
   give Kedah no replacement for it.
3. Kelantan and Terengganu are the other way round: Friday's 27 June stays
   and Saturday's 7 June is moved to Sunday 8 June.
4. A caller who asks for no region, or for Selangor, gets neither move: the
   country's Sunday rule moves Sunday holidays to the Monday, and these
   fall on a Friday and a Saturday.
5. One business day after Thursday 5 March 2026 is Friday 6 March for the
   country and Sunday 8 March for Kedah, in the library; at the boundary the
   walk for Kedah is refused, because Kedah's own days were not read (see
   "Asking the weekend at the boundary").

## Asking the weekend at the boundary

`hc_holiday_is_weekend(code, region, fixed)` (JavaScript `holidayIsWeekend`)
answers 1 or 0 for the law in force on the day in the region, as
`RuleSet::weekend_in` gives it: Kedah's Friday is 1 from 25 November 2013
and the country's is 0; a table that states no weekend law at all, a tradition's or a list of days, keeps Saturday and
Sunday. A day on which the region's law was not read, the `unread-weekend`
gap, is `HC_ERR_OUT_OF_RANGE`, which is what the business-day arithmetic has
always answered for it: no weekend of no days. The region is the same
argument as `hc_holiday_is_day_off`'s and a region of the table's country with
only a weekend law is one; a code that is no subdivision is refused as
`unknown`.

The weekend is read from the table and the region alone, so the answer does
not depend on whether the region's holidays were read: Kedah's weekend is
read and its days are not, and the weekend is answered. The days are what
the other calls need, and they are refused for it (ADR 0013):

- `hc_holiday_is_day_off` answers 1 for a day with an entry that stops work,
  and 0 only where the table knows the day. A gap of a kind that stops work
  in the day's year (a rule's calendar ended, its year not read), or the
  region's own days not read, whatever the kind, which is every year for
  Kedah, is
  `HC_ERR_NO_DATA`, and a day whose region's weekend law was not read
  `HC_ERR_OUT_OF_RANGE`. The refusal is the year's, not the day's: a day the
  table lists is a day off whatever else is open, and one it does not list is
  open as long as a holiday it could not place may be that very day.
- `hc_holiday_add_business_days` and `hc_holiday_business_days_between`
  refuse on the first day the walk reaches that is open in this way, with the
  same two codes, so that a count is never made on a guess. Only the kinds
  the arithmetic counts open a day: Louisiana's Mardi Gras, a day of the
  state's offices (kind `government`), does not (ADR 0010); a China workday
  that was not announced opens a weekend day.
- `hc_holiday_next` and `hc_holiday_previous` give the first holiday after a
  day and the last before it, and refuse when a gap of a wanted kind lies in
  any year from the day's to the found entry's.
- `hc_holidays_on` writes the substitute days of a region that has only a
  weekend law, such as Kedah's Sunday for Awal Muharram 2025,
  with the region in column 10. It does not write the gap
  `unread-subdivision` for those regions, as it writes none for the many
  subdivisions the table does not list; the gap `unread-weekend` it does
  write, in the years the law was not read.

## What is carried

`MALAYSIA` (`hc-holiday`, `countries/weekends.rs`, `MY`, and
`countries/asia.rs`, `MY_SUBSTITUTION`):

| Region | Weekend | Years | Substitute for a holiday on |
| --- | --- | --- | --- |
| the table | not read | to 24 November 2013 | — |
| the table | Saturday and Sunday | from 25 November 2013 | a Sunday, to the Monday |
| `MY-01` Johor | Saturday and Sunday | 1995 to 2013 | a Sunday, to the Monday |
| `MY-01` Johor | not read | to 1994 | — |
| `MY-01` Johor | Friday and Saturday | 1 January 2014 to 31 December 2024 | a Friday, to the Sunday |
| `MY-02` Kedah | Friday and Saturday | from 25 November 2013 | a Friday, to the Sunday; 2014 on |
| `MY-03` Kelantan, `MY-11` Terengganu | Friday and Saturday | from 25 November 2013 | a Saturday, to the Sunday; 2014 on |
| `MY-02`, `MY-03`, `MY-09` Perlis, `MY-11` | not read | to 24 November 2013 | — |

`UNITED_ARAB_EMIRATES` (`countries/weekends.rs`, `AE`): `AE-SH`
Sharjah, Friday, Saturday and Sunday from 1 January 2022. The emirate has
no substitution law of its own in the table, as the federation has none.

Every other region of both tables has the table's weekend. The weekend is
column 14 of `hc_holiday_tables`: the table's weekend laws, separated by
`;`, each four fields separated by `/`.

| Field | Holds |
| --- | --- |
| days | the weekend days as ISO 8601 weekday numbers joined by `+`, Monday 1 to Sunday 7 (`5+6` is Friday and Saturday), or `unread` for years whose law was not read |
| first | the first day in force, `YYYY-MM-DD`, empty for none |
| last | the last day in force, `YYYY-MM-DD`, empty for none |
| regions | the ISO 3166-2 codes it is the weekend of, joined by `,`, empty for the whole table |

`MY` writes
`6+7///;unread//1994-12-31/MY-01;5+6/2014-01-01/2024-12-31/MY-01;unread//2013-11-24/MY-02,MY-03,MY-09,MY-11;5+6/2013-11-25//MY-02,MY-03,MY-11`.
Where several entries cover a day the one for the nearest region wins, and a
region with none of its own has the entry with no regions; a table that
writes nothing keeps Saturday and Sunday. The JavaScript binding reads the
column into `HolidayTable.weekend`, a list of `{ days, first, last,
regions }` with `days` null for `unread`.

The regions an entry names are listed in column 9 of the same line: a
table's regions are `RuleSet::answered_regions`, the subdivisions its rules,
its weekend laws and its substitution policies are scoped to, sorted and once
each. `MY` lists `MY-01;MY-02;MY-03;MY-09;MY-11` and `AE` lists `AE-SH`, though
no rule of either is scoped to one, so that a menu built from column 9 offers
them; `hc_holidays_in_year`, `hc_holiday_is_day_off`, `hc_holiday_is_weekend`
and every other export that takes a `region` accept each of them, and
`hc_place_name` names each, being CLDR subdivisions. Column 13, the
subdivisions whose days were read, does not list them: the law of their
weekend was read, their holidays were not.

Not carried, each with its reason:

- **Other Malaysian states.** Selangor, Penang, Melaka and the rest keep
  the country's weekend, which the sources read state for the present
  and not for earlier years; the table's own policy applies to all of
  them from 25 November 2013, the report's day, and the years before are a
  gap.
- **Malaysia's states' own holidays.** See the section on them below.
- **Aceh.** One English-language article says Aceh keeps Monday to
  Thursday and Saturday [wikipedia-workweek-and-weekend]; it cites nothing, and the
  national rule for the civil service, Presidential Regulation 21 of 2023
  (a PDF, not opened), is Monday to Friday. No Aceh instrument was read as
  HTML, so Aceh keeps the country's weekend, which is a gap in what is
  claimed, not a statement.
- **India's states.** The central government's weekend is Saturday and
  Sunday, and a state's second or fourth Saturday is an office practice
  (the Government of Kerala notifies the second Saturday as a closed
  holiday). The notifications found are PDFs, which were not opened, and no
  state's law of a weekend was read. Not carried; the rotating Saturdays are
  also not a weekend but a day off of another shape.
- **Iraq's Kurdistan Region.** A news article and payroll guides say its
  weekend is Friday and Saturday, as Iraq's: nothing differs, so nothing is
  scoped.
- **Sharjah's private sector and its schools.** The Labour Law governs
  them and the sources read give the government's weekend only.
- **Saudi Arabia, Brunei, Iran, Israel and the rest.** Their weekends are
  countries', already in each table.

## Accuracy

The weekend of each region is checked against the sources the dated
examples in `crates/hc-holiday/tests/regional_weekends.rs` use. The
sources are news reports and an official notice, not the instruments: the
Sultan's decree, the Regent's announcement and the Sharjah Executive
Council's circular were not read, and the Holidays Act 1951 was read in a
secondary reproduction.

- **Kedah, Kelantan, Terengganu.** The earliest source that names them as
  the states with Friday and Saturday is the Jakarta Post's of 25 November
  2013, and the policy runs from that day; the three have kept Friday since
  before independence and when they added Saturday is not read. Before it
  their weekend is a gap, not Saturday and Sunday.
- **Perlis.** The Jakarta Post does not name it among the Friday-Saturday
  states and the National Security Council's notice of 31 December 2024 says
  that only the three do, so it keeps Saturday and Sunday from 25 November
  2013. When it moved off Friday is not settled: a blog quoting Mahathir's
  memoir says 1994, and a blog's account of a Perlis fatwa of 30 July 2009
  asks to keep Friday. Its years to 24 November 2013 are a gap.
- **Johor.** To 1994 a gap; 1995 to 2013 its own, Saturday and
  Sunday, which the Jakarta Post gives as the weekend it was changed from;
  the dates of 2014 and 2025 are three sources' agreeing.
- **The replacement days** are carried from secondary sources that agree
  with each other and with the Act's section 3 where it applies: the
  Wikipedia articles, English and Malay, and an education portal's rule for
  the school calendar. For a Friday holiday in Kelantan and Terengganu,
  which the Act would move to a Saturday that is off already, no extra day
  is carried; for a Saturday holiday in Kedah and Johor none. The states'
  gazettes, which would settle both, were not read.
- **A region's own holidays are not read.** Asking for `MY-02` gives the
  federal days moved by Kedah's rule and the gap `UNREAD_SUBDIVISION`: the
  states' own days are in PDFs on a host that refuses requests.
- **Sharjah.** The 2021 circular's date and the August 2026 page agree;
  no change since 2022 is reported.

## The Malaysian states' own days

Not carried: no source for them could be read. The Prime Minister's
Department's yearly lists, which give each state's own days, the sultans'
birthdays, the founding days and Thaipusam among them, are PDFs on
kabinet.gov.my (`hka_2020.pdf` to `HKA_2027.pdf`), which refuses requests,
and PDFs are not opened here. The Attorney General's Chambers' text of the
Holidays Act 1951 is at commonlii.org, which answers a bot check, and the
states' own gazettes were not found as HTML. The kabinet.gov.my page
`hari-kelepasan-am`, which links the PDFs, refused the connection too.

What is readable as HTML is secondary and undated. The Wikipedia article
on the public holidays in Malaysia
[wikipedia-public-holidays-malaysia] tabulates the states' days with a
date each (Johor's Sultan's Birthday 23 March, Terengganu's 26 April,
Selangor's 11 December, Perlis's 17 May, Penang's George Town World
Heritage Site day 7 July and so on) and gives no year for any. A Sultan's
birthday holiday moves with the reign or the ruler's decision, so a date
with no year of its own is a guess about every year. The commercial
calendar sites that list each state's days for 2026 are no better as
sources, and one search result's summary of them gives 31 July 2026 for the
Sultan of Pahang's Birthday where the article says 25 July. Nothing was
carried from either. Each state's own
days stay the gap `UNREAD_SUBDIVISION`, which the engine reports for
every year asked for, and the missing source is the state's gazette or the
Department's list for the year.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| `jakartapost-johor-weekend-2013` | Johor's weekend to 1994, 1994–2013 and from 2014; the three states then keeping Friday and Saturday | directly, HTML |
| `rtm-johor-weekend-2025` | Johor from 2025; the three states | directly, HTML |
| `mkn-johor-weekend-2025` | Johor from 2025; the three states as the only ones keeping Friday and Saturday | directly, HTML |
| `mylaw-holidays-act-1951` | the weekly holiday, section 3's replacement day | directly, a secondary reproduction |
| `wikipedia-public-holidays-malaysia`, `pendidik2u-cuti-berganti` | the replacement days of the Friday-Saturday states | directly, secondary |
| `parlimen81-weekend-history` | Friday before independence; the 1994 moves, as a blog quotes Mahathir | directly, a blog; not relied on for a date |
| `khaleejtimes-sharjah-weekend-2021`, `gulfnews-sharjah-weekend-2021`, `uae-public-sector-working-hours` | Sharjah's weekend from 2022 | directly, HTML |
| `wikipedia-workweek-and-weekend` | Aceh's reported week, not carried | directly, secondary |

## Code

- `crates/hc-holiday/src/rule.rs`: `WeekendPolicy::regions`,
  `SubstitutionPolicy::{regions, avoid}`, `RuleSet::{weekend_in,
  weekend_on, weekend_unread_in, weekend_regions, substitution_regions,
  answered_regions, substitution_in_region}`, `UNREAD_WEEKEND`.
- `crates/hc-holiday/src/engine.rs`: `HolidayCalendar::{is_weekend,
  weekend_is_read, add_business_days, business_days_between}` and the gap;
  `day_off`, `business_day`, `try_add_business_days`,
  `try_business_days_between`, `try_next_of`, `try_previous_of` and
  `Unanswered`, which say why a day is open.
- `crates/hyper-calendar/src/holiday_lines.rs`: `holiday_tables`, column 9
  from `answered_regions`, and `weekend_cell`, column 14; the one test of
  both is `crates/hyper-calendar/tests/holiday_table_regions.rs`;
  `is_weekend`, `is_day_off`, `add_business_days`, `business_days_between`,
  `next_holiday_line`, `previous_holiday_line` and `holidays_on`.
- `crates/hc-holiday/tests/regional_weekends.rs`: the dated examples above;
  `crates/hc-holiday/tests/open_days.rs`: the refusals, over a table whose
  gaps and weekends are the ones each case needs.
