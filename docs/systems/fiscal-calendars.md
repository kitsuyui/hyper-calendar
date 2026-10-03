# Fiscal, tax and academic years, and the 52/53-week calendars

Backs the crate `hc-fiscal`: its modules `year_system`, `quarters`,
`countries`, `academic` and `retail`, reached through the facade's `fiscal`
feature as `hyper_calendar::hc_fiscal`. No calendar identifier is
registered: a fiscal year is a way of naming years on a calendar that
exists already. What the crate does name is the country codes of
`countries::ALL` and `academic::ALL`, the three week-year identifiers
`nrf-4-5-4`, `iso-8601-week-year` and `last-saturday-of-december-4-4-5`,
and the five calendars a start may be dated in: `gregory`,
`persian-arithmetic-33`, `ethiopic`, `buddhist` and `bikram-sambat`
([policy.md](../policy.md) §5). The WebAssembly module and the C library
export it in the `fiscal` layer: `hc_fiscal_profiles`, `hc_fiscal_year_on`,
`hc_fiscal_year_span`, `hc_week_year_systems` and `hc_week_year_on`, whose
lines `hyper_calendar::fiscal_lines` writes once for both and whose columns
the boundary READMEs give.

## What it is

Institutions agree on what day it is and disagree on what year it is. A
government budgets in a **fiscal year**, a revenue authority assesses
individuals in a **tax year**, a school teaches in an **academic year**, and
none of these need begin on 1 January. Two things make the label unsafe.

**The name.** The federal fiscal year of the United States "begins on
October 1 of each year and ends on September 30 of the following year"
[cornell-31-usc-1102], and a fiscal year's short name is often the calendar
year it ends in, so "FY24" is 2023–2024 [wikipedia-fiscal-year]. Japan's
年度 begins on 1 April [japan-zaisei-ho-art11], and the code names it for the
year it begins in, which the Act does not say. Both are written "FY2024",
and the two years so written share only six months, April to September 2024.
The convention belongs to a system, not to a country. New Zealand's tax year
runs from 1 April to 31 March [ird-nz-tax-year]; Inland Revenue's guide for
the year from 1 April 2024 is titled 2025, as a search result showed (the
guide is a PDF and was not opened). South Africa's year of assessment, from
1 March to the end of February, is named for the year it ends in
[sars-tax-year]. The United Kingdom's tax year is named for the year it
begins in: "the tax year 2007-08" is the one beginning 6 April 2007
[uk-income-tax-act-2007-s4].

**The calendar.** "1 April" is not a complete rule until the calendar is
named. Iran's year begins at Nowruz, which the Solar Hijri calendar fixes by
the March equinox before or after noon at Tehran [heydari-malayeri2004, §1],
and lands on 20 or 21 March [wikipedia-solar-hijri-calendar]. Ethiopia's
begins on Hamle 1, which is 8 July [wikipedia-ethiopian-calendar;
indexmundi-ethiopia-fiscal-year]. Thailand names its budget year in
Buddhist Era years: the year from 1 October 2024 to 30 September 2025 is
2568 [thai-wikipedia-budget-year].

The **52/53-week year** is the other family. A retailer wants this week
compared with the same week last year, so its year is a whole number of weeks,
52 or sometimes 53, and ends on a chosen weekday. The National Retail
Federation publishes the retail year as a 4-5-4 calendar of four weeks, five
weeks and four weeks, so that comparable months hold the same number of
Saturdays and Sundays [nrf-4-5-4-calendar]. The United States Treasury
Regulations give two ways to fix such a year's end, "whatever date this same
day of the week last occurs in a calendar month" and "whatever date this same
day of the week falls that is the nearest to the last day of the calendar
month" [cornell-26-cfr-1-441-2]. ISO 8601's week-numbering year is a
52/53-week year without periods [wikipedia-iso-week-date].

## How it works

**A year system.** A `YearSystem` is a start (a month and a day in a named
calendar), a label convention (`LabelledByStartYear` or `LabelledByEndYear`,
with no default: policy §5) and a validity range given in the system's own
labels, not in Gregorian years. For a label *L*, the calendar year the cycle
starts in is *L* under the start convention and *L* − 1 under the end
convention. The first day of year *L* is the start date in that calendar
year; its last day is the day before the first day of year *L* + 1. The year
of a day *d* is found the other way: take the start calendar's year *y*
containing *d*; if *d* falls before that year's start date the cycle began
in *y* − 1; the label is that cycle year, plus one under the end convention.
A start that does not exist in a year (30 February) is an error and is
never clamped.

**Months, quarters, halves.** Fiscal month *n* runs from the start
day-of-month in the *n*-th month on from the start, to the day before the
same day-of-month a month later. With a start on the first of a month this
is the calendar months. With the United Kingdom's 6 April it gives tax
months from the 6th to the 5th: month 1 of 2024-25 is 6 April to 5 May 2024
(30 days) and month 12 is 6 March to 5 April 2025. A quarter is three
fiscal months, a half is two quarters. A calendar whose year is not twelve
months of equal standing returns `PeriodsNotDefined`: the Ethiopic year has
Pagumen, and no source read says which quarter it belongs to.

**A week year.** A `WeekYearSystem` ends on an anchor weekday chosen
relative to the last day of an anchor month, either as the last such
weekday of the month (`LastWeekdayOfMonth`) or the one nearest the month's
last day (`WeekdayNearestMonthEnd`). The start is the day after the
previous year's end. The nearest weekday never ties: the distances to the
previous and the next occurrence sum to seven. A year is 52 or 53 weeks. A
quarter's thirteen weeks split into three periods, 4-4-5, 4-5-4 or 5-4-4
(`PeriodShape`), and in a 53-week year the extra week is added to period
12. The NRF states the rule for the extra week as a count: lay out the 52
weeks, and if four or more days of January are left in the 53rd week, add
that week [nrf-4-5-4-calendar]. ISO 8601 numbers weeks from Monday; week 1
has the year's first Thursday, equivalently 4 January; and a week belongs
to the year its Thursday is in [wikipedia-iso-week-date]. That is the
same as a year ending on the Sunday nearest 31 December, which is how the
crate writes `iso-8601-week-year`.

**Worked example.** Take 1 November 2023.

*Japan.* The start is 1 April, named by the year it begins in. The day is
after 1 April 2023, so the label is 2023, the span is 1 April 2023 to
31 March 2024 (366 days, as it holds 29 February 2024), and the day is the
215th: April to October hold 30 + 31 + 30 + 31 + 31 + 30 + 31 = 214 days.
It is fiscal month 7, in quarter 3, October to December.

*United States.* The start is 1 October, named by the year it ends in. The
day is after 1 October 2023, so the cycle began in 2023 and the label is
2024; the span is 1 October 2023 to 30 September 2024 (366 days), the day
is the 32nd, and it is in quarter 1, October to December. Thailand's budget
year puts the same days in the Buddhist calendar: 1 November 2023 is in
the cycle of 2566 and so in budget year 2567. The year the United States
writes FY2024 begins 183 days before the year Japan writes FY2024
(1 October 2023 against 1 April 2024) and ends six months earlier.

*The NRF retail year 2023* (named by the year it starts in). 31 January
2023 is a Tuesday: the Saturday before is 28 January, three days off, and
the one after is 4 February, four days off, so the previous year ends on
28 January 2023 and this one starts on Sunday 29 January 2023. 31 January
2024 is a Wednesday: 27 January is four days off and 3 February three, so
the year ends on Saturday 3 February 2024. That is 371 days, 53 weeks. The
periods are 4, 5, 4, 4, 5, 4, 4, 5, 4, 4, 5 and 5 weeks; period 12 is the
five weeks from Sunday 31 December 2023 to 3 February 2024. 1 November 2023
is in week 40 (it begins on Sunday 29 October), period 10 (29 October to
25 November) and quarter 4.

*Iran.* The year labelled 1404 begins on 1 Farvardin 1404. The 33-year
rule gives 21 March 2025, the date the Solar Hijri table gives
[wikipedia-solar-hijri-calendar]; Birashk's 2 820-year cycle gives 20 March.

## What is carried

**Systems carried.** Every value is data read by one shared algorithm. A
`FiscalProfile` is a country code, a list of `YearSystem` values, current
and past, a `sources` string and a `sources_checked` date (2026-09-21;
Nepal 2026-09-23). Where a profile has more than one system the kind says
which: `Government`, `PersonalTax`, `CorporateDefault` or `Academic`. "Start
year" and "end year" below are the label convention; years in brackets are
the labels over which the system is carried.

| Code | Country | Systems carried |
| --- | --- | --- |
| AU | Australia | Government: 1 July, start year |
| BR | Brazil | Government: 1 January, start year |
| CA | Canada | Government: 1 April, start year |
| CN | China | Government: 1 January, start year |
| DE | Germany | Government: 1 January, start year |
| EG | Egypt | Government: 1 July, start year |
| ET | Ethiopia | Government: Hamle 1 of the Ethiopic calendar, end year (from 2002) |
| FR | France | Government: 1 January, start year |
| GB | United Kingdom | Government: 1 April, start year (from 1753); personal tax: 6 April, start year; England and Wales before 1752: 25 March in proleptic Gregorian dates, start year (to 1750) |
| HK | Hong Kong | Government: 1 April, start year |
| IN | India | Government: 1 April, start year |
| IR | Iran | Government: 1 Farvardin on `persian-arithmetic-33`, start year (from 1366) |
| JP | Japan | Government: 1 April, start year (from 1947); the school year, 1 April (from 1947) |
| NP | Nepal | Government: Shrawan 1 of the Bikram Sambat, start year (from 2076); needs the `indic` feature |
| NZ | New Zealand | Government: 1 July, start year; personal tax: 1 April, end year |
| PK | Pakistan | Government: 1 July, start year |
| RU | Russia | Government: 1 January, start year |
| SE | Sweden | Government: 1 January, start year (from 1997) and 1 July, start year (1923 to 1994); company accounts: 1 January |
| SG | Singapore | Government: 1 April, start year; personal tax basis period: 1 January |
| TH | Thailand | Government: 1 October of the Buddhist calendar, end year (from 2505); 1 January, start year (2484 to 2504) |
| US | United States | Government: 1 October, end year (from 1977); 1 July, end year (to 1976) |
| ZA | South Africa | Government: 1 April, start year; personal tax: 1 March, end year |

Years in these systems are labelled by the system: Iran's by Solar Hijri
year, Ethiopia's by Ethiopic year, Thailand's by Buddhist Era year, Nepal's
by Bikram Sambat year.

**Academic years, 8 countries.** Each carries an `Authority`, and
`is_national_rule` is true only for a statute or regulation.

| Code | School year starts | Authority | University year starts |
| --- | --- | --- | --- |
| JP | 1 April | regulation | 1 April, per institution |
| GB | 1 September | per region | 1 September, per institution |
| FR | 1 September | regulation | 1 September, per institution |
| DE | 1 August | per region | 1 October, per region |
| AU | 28 January | per region | 1 February, per institution |
| NZ | 28 January | regulation | 1 February, per institution |
| IN | 1 April | per region | none carried |
| US | 15 August | per institution | 15 August, per institution |

Where the authority is not national the entry is the modal choice and says
so in its note; a date there is not a claim about any district.

**Week years.** `nrf-4-5-4` (Saturday nearest 31 January, 4-5-4, start
year), `iso-8601-week-year` (Sunday nearest 31 December, end year, no
periods) and `last-saturday-of-december-4-4-5` (last Saturday of December,
4-4-5, end year).

**The boundary.** `hyper_calendar::fiscal_lines` writes one line per year
system for `hc_fiscal_profiles` (18 cells: the country, the table, the kind,
the names, the authority, the start in its calendar, the label convention,
the validity bounds, the note, the date the sources were checked and the
sources), for `hc_fiscal_year_on` (18 cells) and for `hc_fiscal_year_span`
(9 cells); one line per week-year system for `hc_week_year_systems` (10 cells)
and `hc_week_year_on` (14). A year line carries the label convention and the
start's calendar as cells, never defaulted, since "FY2024" begins in 2024 in
Japan and on 1 October 2023 in the United States. Its status is `in-force`,
`outside-validity`, where the system was not in force in that year, or
`outside-calendar-range`, where the start's calendar does not reach the day,
and the cells after the status are then empty. A country or kind not carried
is `HC_ERR_UNKNOWN`; a country with no system of the kind asked is
`HC_ERR_NO_DATA`. Nepal, whose year starts on 1 Shrawan of the Bikram Sambat,
is present only in a build that has the `calendars` layer too. The columns are
in the boundary READMEs.

**Holes.** The crate leaves a day in no year where history did. The 92 days
of 1 July to 30 September 1976, the transition quarter, are in no United
States fiscal year; 1 July 1995 to 31 December 1996 is in no Swedish budget
year; and 1751 and 1752 are in neither English system.

**Not carried: every other country.** The crate carries 22 countries' fiscal
years. Every other country is NOT YET CARRIED (reason: no source for any of
them has been read); the holiday tables of `hc-holiday` carry
195 countries (its README) against these 22. `countries::by_code` of an absent
country is `None`, which means "not carried", not "no offset". The same holds
for the school and university years of every country but the 8.

**Not carried, with the reason for each.**

- A company's own fiscal year, and any retailer's published calendar as a
  named system (not yet done; no source for any named company's calendar
  was read, beyond one 10-K cited in Sources). The caller writes it as a
  `WeekYearSystem`.
- The 13-period retail calendar of four-week periods (not yet done; no
  source read).
- Sub-national fiscal years, such as a state's or a province's (not yet
  done; no source read).
- Fiscal and tax years before the validity ranges above, and the
  transitional periods inside them: the stub half-year of 1923 in Sweden,
  Thailand's nine-month B.E. 2483, England's 1751 and 1752 (not yet done;
  for each, the entry's note says what was not found).
- Singapore's Year of Assessment, which follows the basis period, and the
  personal tax years of every country but the United Kingdom, New Zealand,
  Singapore and South Africa (not yet done; no source read).
- The `kharaji` fiscal year, the Ottoman *mâlî* year of 1677 to 1840 and the
  Ptolemaic financial year, which `docs/calendars.md` lists as researching
  (primary sources not read; see that document's rows).
- Which quarter of an Ethiopic year Pagumen is in (no source read says),
  so `PeriodsNotDefined`.

## Accuracy

**Exact where arithmetic.** A Gregorian-anchored year is integer arithmetic.
`every_government_system_round_trips_a_long_span_of_days` takes every
government system over 1950 to 2050, or its validity range if narrower,
checks that each year is 353 to 367 days and that the label of its first
and last day is its label. `consecutive_years_abut_without_a_gap_or_an_overlap`
and `every_day_of_a_long_run_lands_in_exactly_one_year` check the tiling
over 1800 to 2200 and 1990 to 2040. `fraction_elapsed` is the one float
and claims a day.

**The starts, against a source read.** Each start below was compared on
2026-10-03 with a page read directly; the first column is the system and
the last the result.

| System | Source read | Result |
| --- | --- | --- |
| United States, 1 October, end year; the 1976 transition quarter | [cornell-31-usc-1102]; [wikipedia-fiscal-year] | agrees |
| United Kingdom government, 1 April | [uk-interpretation-act-1978-sch1] ("the twelve months ending with 31st March") | agrees |
| United Kingdom tax, 6 April, start year | [uk-income-tax-act-2007-s4] | agrees |
| Japan, 1 April | [japan-zaisei-ho-art11] | start agrees; the start-year naming is in no page read |
| India, 1 April | [india-general-clauses-act-3-21] | agrees |
| Canada, 1 April | [canada-faa-s2] | agrees; the Act does not name the year, the table follows the Estimates |
| Germany, calendar year | [germany-bho-4] | agrees |
| Russia, calendar year | [russia-budget-code-art12]: the heading "Финансовый год" on article 12, whose text was not shown | start from [wikipedia-fiscal-year] |
| South Africa, 1 March, end year | [sars-tax-year] | agrees |
| New Zealand tax, 1 April, end year | [ird-nz-tax-year] | dates agree; the end-year naming rests on a search result, not a page opened |
| Thailand, 1 October, end year; first used for 2505 from 1 October 2504 | [pridi-budget-year-history]; [thai-wikipedia-budget-year] | agrees; the second dates the Act to 2502 and gives no first year |
| Iran, 1399 to 1410 | [wikipedia-solar-hijri-calendar] | 12 of 12 agree; see below |
| Ethiopia, 8 July | [indexmundi-ethiopia-fiscal-year]; [wikipedia-ethiopian-calendar] | agrees |
| Nepal, 16 July | [wikipedia-fiscal-year] | agrees for 2024; the code's other years not checked |
| Australia, 1 July | [treasury-au-reporting-periods]; [wikipedia-fiscal-year] | agrees |
| Brazil, China, Egypt, France, Hong Kong, New Zealand Crown, Pakistan, Singapore | [wikipedia-fiscal-year], secondary, which gives the calendar year or the date for each government | agrees |
| Sweden | [wikipedia-fiscal-year] | not checked: the page gives individuals' and organisations' years, not the state budget year, its 1997 change or the 1995/96 year, which no page read gives |

No start disagreed with the code. Two secondary statements did not agree
with each other: [wikipedia-fiscal-year] says Iran's fiscal year "usually
starts on 21st or 22 March", and its own Solar Hijri page gives 20 or 21 in
every year from 1399 to 1410. The code follows the second, which is the
table.

**The calendar anchors.**
`the_iranian_fiscal_year_matches_the_published_gregorian_starts` reproduces
the table's twelve year starts. The probe of 2026-10-03 compared
`persian-arithmetic-33` with the equinox calendar `persian` of
`hc-calendars-equinox` on every Nowruz from AP 1178 to 1634: 457 of 457 agree,
which is Borkowski's range as [heydari-malayeri2004, §8] reports it (Borkowski
himself not read, [borkowski1996]). Birashk's 2 820-year cycle,
`persian-arithmetic`, differs from the equinox calendar in 13 of those years:
1210, 1243, 1404, 1437, 1470, 1503, 1532, 1536, 1565, 1569, 1598, 1602 and
1631. Over 1100 to 1177 and 1635 to 1700 the 33-year rule differs from the
equinox calendar in 1111, 1144, 1177, 1635 and 1668. So the entry is exact
against the model from AP 1366 to 1634 and an extrapolation after it, and
`is_approximate` stays true. "Exact" there means agreement with `persian`,
whose own tolerance and undecided years are in
[solar-hijri.md](solar-hijri.md); none of the ten years it cannot call falls
from 1366 to 1634.

**Retail.** Against Target's Form 10-K, "fiscal 2023 (a 53-week year)" ended 3
February 2024 and fiscal 2022 ended 28 January 2023
[sec-target-10k-fiscal-2023]: the crate gives 29 January 2023 to 3 February
2024 and 30 January 2022 to 28 January 2023. The NRF page names 2006, 2012,
2017 and 2023 as 53-week years and gives the week 29 January to 4 February
2017 [nrf-4-5-4-calendar]: the crate's long years from 1990 to 2060 are 1995,
2000, 2006, 2012, 2017, 2023, 2028, 2034, 2040, 2045, 2051 and 2056, so it
agrees on those four and has no other long year from 2006 to 2023, and its
retail 2017 starts on Sunday 29 January 2017. The NRF's count rule (four or
more days of January in the 53rd week) and the crate's nearest-Saturday rule
give the same 52-or-53 answer on every year from 1950 to 2100, measured. The
Treasury Regulation's own November 2001 example, 24 November for the last
Saturday and 1 December for the nearest, is a test
(`the_regulations_own_november_2001_example_distinguishes_the_rules`)
[cornell-26-cfr-1-441-2]. The ISO year is checked against
`hc_calendars_solar::iso_week` for week 1 and weeks in the year over 1800 to
2199, and day by day from 1995 to 2035; the ISO source here is secondary
[wikipedia-iso-week-date], and its examples 2021-01-01 as 2020-W53-5 and
2019-12-30 as 2020-W01-1 are tests.

**Where the sources differ from each other or from the rule.**

- Tøndering's page names 1404 and 1437 as the only two disagreements between
  AP 1244 and 1531 of the 2 820-year cycle with an astronomical calendar
  [tondering-persian-calendar]; the page does not say which astronomical rule
  it used. The crate's equinox calendar disagrees with the cycle in four years
  of that span (1404, 1437, 1470 and 1503), and the comment of the Iran entry
  says so.
- The NRF page names 2006, 2012, 2017 and 2023 as 53-week years; it links
  calendars to 2028 that were not opened (they are PDF). 2028 is the rule's
  answer, and the test says so.
- The NRF page does not state "the Saturday nearest 31 January". It states
  the count rule, which the crate's nearest-Saturday rule reproduces (above).

## Sources

Read directly as HTML on 2026-10-03 unless noted.

- [cornell-31-usc-1102]: the text of 31 U.S.C. § 1102 and its credit to
  Pub. L. 93-344.
- [wikipedia-fiscal-year]: the country list, the label convention for the
  United States and India, the transition quarter of 1976, the United
  Kingdom's 25 March. Secondary.
- [uk-income-tax-act-2007-s4], [uk-interpretation-act-1978-sch1]: the tax
  year of 6 April and the financial year of 1 April, revised text.
- [uk-calendar-act-1750]: section 1 (the year from 1 January 1752, 14
  September after 2 September) and section 6 (rents, leases and the time of
  payment not accelerated).
- [japan-zaisei-ho-art11]: 財政法 article 11, as a university's law
  database copies it.
- [india-general-clauses-act-3-21], [canada-faa-s2], [germany-bho-4],
  [russia-budget-code-art12]: the definitions named in the table.
- [sars-tax-year], [ird-nz-tax-year], [treasury-au-reporting-periods]: the
  tax years of South Africa and New Zealand; Australia's standard income
  year.
- [pridi-budget-year-history], [thai-wikipedia-budget-year]: Thailand's
  history of budget years.
- [indexmundi-ethiopia-fiscal-year] (the CIA World Factbook's line),
  [wikipedia-ethiopian-calendar]: Ethiopia.
- [wikipedia-solar-hijri-calendar]: the correspondence table of 1399 to 1410.
  [heydari-malayeri2004]: re-read as an HTML copy (ar5iv): the 33-year
  cycle, Borkowski's range, the case against the 2 820-year cycle, the
  Nowruz definition. [tondering-persian-calendar]: the two disagreements.
- [nrf-4-5-4-calendar]: the 4-5-4 rule, the count rule for the 53rd week,
  the 53-week years to 2023. [cornell-26-cfr-1-441-2]: the two anchor rules
  and the November 2001 example. [sec-target-10k-fiscal-2023]: Target's year.
  [wikipedia-iso-week-date]: ISO week dates, secondary.

Not read, though the code or an earlier document cites them: ISO 8601-1:2019
itself (clause 4.2.2.6); Borkowski 1996 [borkowski1996]; the Public
Accounting Act of Iran article 6; Proclamation 648/2009 of Ethiopia; the
Financial Procedures and Fiscal Responsibility Act 2076 of Nepal (a PDF),
whose 2080 to 2083 month lengths come from holiday notices also not read
here; Pub. L. 93-344 and 94-274 and the Act of 1842 (the credit line of
[cornell-31-usc-1102] and [wikipedia-fiscal-year] stand for them); the
Public Finance Ordinance of Hong Kong; the Financial Procedure Act of
Singapore and the Public Finance Management Act of South Africa (both
pages refused or unavailable); the New Zealand Public Finance Act 1989
(refused); Brazil's Lei 4.320 and China's Budget Law (the connection
failed); France's LOLF; Sweden's Bokföringslag and its budget propositions;
the Australian Income Tax Assessment Act 1997; every source behind the eight
academic years (the Education Acts, the *arrêtés*, the school laws, the
Education Commission of the States); and the Treasury Financial Reporting
Manual cited for the last-Saturday-of-December year.

## Code

`crates/hc-fiscal/src/year_system.rs` (the core type, `StartCalendar`,
`YearStart`, `LabelConvention`, `Authority`), `quarters.rs`, `countries.rs`,
`academic.rs` and `retail.rs`. The crate's tests pass (2026-10-03). The
tests that anchor it:
`japan_and_the_united_states_disagree_by_a_year_on_the_same_day`,
`the_two_years_written_fy2024_share_only_six_months`,
`the_transition_quarter_of_1976_belongs_to_no_fiscal_year`,
`every_other_day_of_the_twentieth_century_has_a_us_fiscal_year`,
`the_uk_tax_year_starts_on_the_sixth_of_april`,
`a_uk_tax_month_runs_from_the_sixth_to_the_fifth`,
`the_eighteen_month_swedish_transition_is_left_out_of_both_systems`,
`the_iranian_fiscal_year_matches_the_published_gregorian_starts`,
`the_iranian_fiscal_year_is_not_the_2820_year_cycle`,
`the_ethiopian_fiscal_year_is_named_by_the_year_it_ends_in`,
`the_ethiopian_fiscal_year_has_no_quarters`,
`nepal_starts_its_year_on_1_shrawan`,
`the_published_nrf_retail_years_all_come_out_right`,
`nrf_retail_2023_is_a_fifty_three_week_year`,
`the_iso_week_year_written_as_an_anchor_rule_matches_the_standard` and
`only_japan_france_and_new_zealand_claim_a_national_school_year`.
`crates/hyper-calendar/tests/facade.rs` checks that the `fiscal` feature
reaches the crate.
