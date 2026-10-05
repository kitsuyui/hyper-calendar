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

**What a year is read for.** A system carries two facts about its years
besides the label (ADR 0013). `valid_from` is the year it was
*established*, before which a source says it did not exist; `valid_until`
is its last year; outside them the system is absent, which is an answer
(`outside-validity`, `FiscalError::OutsideValidity`). `read_from` is the first
year the sources read reach: every year from the establishment to it is a
*gap* (`gap`, `FiscalError::NotRead`), and so is every year in an `unread`
span, a stretch after it that no source reaches either. A system whose
establishment no source gives has no `valid_from`, and is a gap before its
`read_from` at every label. The first year read is the first whole year of the
earliest instrument, edition or page read: an Act in force from 1 April 1947
begins with 1947 for a year that starts on 1 April, one in force on
23 March begins with the next year, and a page of 2026 that states a year
begins with 2026. Asking the engine about a day answers with the system that
was in force, or with the gap, or with nothing where no system covers the
year at all (the half year of 1 January to 30 June 1843 in the United
States). `projected_span` and `projected_label_at` run the rule over any year
and say nothing of whether it was in force.

*Worked example: the United States in 1800, 1843 and 1844.* The Treasury's
letter of 15 December 1842 asks for estimates for "the half calendar year
ending 30th June, 1843" and for "the fiscal year ending 30th June, 1844"
[us-hdoc-27-16], so the first July year is FY1844, 1 July 1843 to 30 June
1844, and the six months of 1843 before it are in no system. The
calendar-year system before the Act of 26 August 1842 is carried to 1842 and
read for 1842 only, from a Treasury report on "the first half of the year
1842" [us-sdoc-27-371]. Asked about 1 June 1800 the engine answers `gap` for
the calendar-year system, which was in force and was not read, and
`outside-validity` for the July and the October systems, which did not exist
yet. Asked about 1 June 1843 it answers `outside-validity` for all three.
Asked about 1 June 1844 it answers FY1844, 1 July 1843 to 30 June 1844.

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
and past, a `sources` string and a `sources_checked` date (2026-10-04). Where
a profile has more than one system the kind says which: `Government`,
`PersonalTax`, `CorporateDefault` or `Academic`. "Start year" and "end year"
below are the label convention; the labels in brackets are the first year
carried as established (E) and the first year the sources read reach (R), a
label of the system's own calendar. A system with an R and no E has no
establishment in any source read, and its years before R are a gap.

| Code | Country | Systems carried |
| --- | --- | --- |
| AU | Australia | Government: 1 July, start year (R 1902) |
| BR | Brazil | Government: 1 January, start year (R 1965) |
| CA | Canada | Government: 1 April, start year (E and R 1907) |
| CN | China | Government: 1 January, start year (R 1992) |
| DE | Germany | Government: 1 January, start year (R 1970) |
| EG | Egypt | Government: 1 January, start year (R 1974, to 1979); 1 July, start year (E and R 1980) |
| ET | Ethiopia | Government: Hamle 1 of the Ethiopic calendar, end year (R 2014) |
| FR | France | Government: 1 January, start year (R 2006) |
| GB | United Kingdom | Government: 1 April, start year (E 1854, R 1855); personal tax: 6 April, start year (R 2007); England and Wales before 1752: 25 March in proleptic Gregorian dates, start year (E and R 1155, to 1750) |
| HK | Hong Kong | Government: 1 April, start year (R 2025); year of assessment: 1 April, start year (R 1947) |
| IN | India | Government: 1 April, start year (E 1867, R 1868) |
| IR | Iran | Government: 1 Farvardin on `persian-arithmetic-33`, start year (R 1350) |
| JP | Japan | Government: 1 July, start year (E and R 1875, to 1884); 1 April, start year (E and R 1886, with 1921 to 1946 unread); the school year, 1 April (R 1947) |
| NP | Nepal | Government: Shrawan 1 of the Bikram Sambat, start year (R 2083); needs the `indic` feature; without the `indic` feature the facade's `hc_fiscal_year_on` and `hc_fiscal_year_span` answer `no-data` |
| NZ | New Zealand | Government: 1 July, start year (R 2026); personal tax: 1 April, end year (R 2025) |
| PK | Pakistan | Government: 1 July, start year (R 2026) |
| RU | Russia | Government: 1 January, start year (R 2000) |
| SE | Sweden | Government: 1 January, start year (R 1921, to 1922); 1 July, start year (E and R 1923, to 1994); 1 January, start year (E and R 1997); company accounts: 1 January (R 1977) |
| SG | Singapore | Government: 1 April, start year (R 2026); personal tax basis period: 1 January (R 2026) |
| TH | Thailand | Government: 1 October of the Buddhist calendar, end year (E and R 2505); 1 January, start year (E and R 2484, to 2503) |
| US | United States | Government: 1 October, end year (E and R 1977); 1 July, end year (E and R 1844, to 1976); 1 January (R 1842, to 1842) |
| ZA | South Africa | Government: 1 April, start year (R 2026); personal tax: 1 March, end year (R 1985) |

Years in these systems are labelled by the system: Iran's by Solar Hijri
year, Ethiopia's by Ethiopic year, Thailand's by Buddhist Era year, Nepal's
by Bikram Sambat year.

**Countries read from one page or instrument, 39.** Each is one government
year with the first year the page reaches (R). The authority `unread` is a
page read that states the year, with the instrument that fixes it not read.
The calendar-year ones are named for their year, and the label convention of
the others is the span a ministry's documents lead with the start year of.

| Code | Country | Year | R | Authority |
| --- | --- | --- | --- | --- |
| AO | Angola | 1 January | 2026 | unread |
| AR | Argentina | 1 January | 1993 | statute |
| BD | Bangladesh | 1 July | 1974 | statute |
| BH | Bahrain | 1 January | 2026 | unread |
| BS | Bahamas | 1 July | 2025 | unread |
| CL | Chile | 1 January | 2026 | statute |
| CO | Colombia | 1 January | 1997 | statute |
| CR | Costa Rica | 1 January | 2020 | statute |
| CY | Cyprus | 1 January | 2015 | statute |
| CZ | Czechia | 1 January | 2001 | statute |
| ES | Spain | 1 January | 2005 | statute |
| FJ | Fiji | 1 August | 2025 | unread |
| GH | Ghana | 1 January | 2026 | unread |
| HU | Hungary | 1 January | 2012 | statute |
| ID | Indonesia | 1 January | 2004 | statute |
| IL | Israel | 1 January | 2026 | unread |
| IQ | Iraq | 1 January | 2026 | unread |
| IS | Iceland | 1 January | 2016 | statute |
| IT | Italy | 1 January | 2010 | statute |
| JM | Jamaica | 1 April | 2015 | statute |
| JO | Jordan | 1 January | 2026 | unread |
| KR | South Korea | 1 January | 2007 | statute |
| KZ | Kazakhstan | 1 January | 2026 | unread |
| LB | Lebanon | 1 January | 2026 | unread |
| LV | Latvia | 1 January | 2003 | statute |
| MN | Mongolia | 1 January | 2012 | statute |
| MZ | Mozambique | 1 January | 2026 | unread |
| NG | Nigeria | 1 January | 2026 | unread |
| OM | Oman | 1 January | 2026 | unread |
| PH | Philippines | 1 January | 1988 | statute |
| QA | Qatar | 1 January | 2016 | unread |
| TN | Tunisia | 1 January | 2026 | unread |
| TR | Turkey | 1 January | 2004 | statute |
| TW | Taiwan | 1 January | 2022 | statute |
| TZ | Tanzania | 1 July | 2024 | unread |
| UA | Ukraine | 1 January | 2011 | statute |
| UZ | Uzbekistan | 1 January | 2014 | statute |
| VN | Vietnam | 1 January | 2017 | statute |
| WS | Samoa | 1 July | 2025 | unread |

Of them the instrument was read for Argentina, Bangladesh, Chile (a copy),
Colombia, Costa Rica, Cyprus, Czechia, Hungary, Iceland, Indonesia, Italy,
Jamaica, Latvia, Mongolia, the Philippines, South Korea, Spain, Taiwan,
Turkey, Ukraine, Uzbekistan and Vietnam (each entry's `sources` names the
instrument). Qatar's calendar year from 2016 is a page of the Atlantic
Council's; Fiji, the Bahamas, Samoa and Tanzania are the ministries' own
pages; and the twelve of Angola, Bahrain, Ghana, Iraq, Israel, Jordan,
Kazakhstan, Lebanon, Mozambique, Nigeria, Oman and Tunisia rest on two
reference pages that agree (a PwC tax summary's tax year and a Wikipedia
infobox's fiscal year) and are read from 2026.

**Academic years, 8 countries.** Each carries an `Authority`, and
`is_national_rule` is true only for a statute or regulation. R is the first
year the sources read reach.

| Code | School year starts | Authority | R | University year starts | R |
| --- | --- | --- | --- | --- | --- |
| JP | 1 April | regulation | 1947 | 1 April, per institution | 2026 |
| GB | 1 September | per region | 2026 | 1 September, per institution | 2026 |
| FR | 1 September | regulation | 2026 | 1 September, per institution | 2026 |
| DE | 1 August (E 1967) | per region | 1967 | 1 October, per region | 2026 |
| AU | 28 January | per region | 2027 | 1 February, per institution | 2026 |
| NZ | 28 January | unread | 2026 | 1 February, per institution | 2026 |
| IN | 1 April | per region | 2026 | none carried | |
| US | 15 August | per institution | 2023 | 15 August, per institution | 2026 |

Where the authority is not national the entry is the modal choice and says
so in its note; a date there is not a claim about any district. A start
date that no page read gives is the table's representative date and the
entry's note says so: 1 September for the United Kingdom's school and
university years, France's university year and its school year's day, 28
January for Australia and New Zealand, 1 February for the two universities,
15 August for the United States.

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
of 1 July to 30 September 1976, the transition quarter, and the half year of
1 January to 30 June 1843 are in no United States fiscal year; 1 July 1995 to
31 December 1996 and 1 January to 30 June 1923 are in no Swedish budget year;
the nine months of 1 July 1885 to 31 March 1886 are in no Japanese one; the
half year of 1 January to 30 June 1980 is in no Egyptian one; B.E. 2504 is in
no Thai one; and 1751 and 1752 are in neither English system.

**Not carried: every country not listed.** The crate carries 61 countries'
fiscal years, 39 of them from one page or instrument each. Every other
country is NOT YET CARRIED: the surveys of 2026-10-04 read no page that
states a government year for it, or read one whose label convention for a
year that does not begin on 1 January is not evidenced (Kenya, Rwanda,
Malawi, Namibia, Botswana, Kuwait, Brunei, Bhutan, Laos, Myanmar, Uganda,
Mauritius, Afghanistan and Trinidad and Tobago among them), or only a PwC
tax summary's corporate year, which is not a government year (the countries
of Europe other than those listed, and most of the Americas). The
websearch and several legislation portals were unavailable for part of the
survey, so each of these is a country not yet read and not one with no
fiscal year. The holiday tables of `hc-holiday` carry 195 countries (its
README). `countries::by_code` of an absent country is `None`, which means
"not carried", not "no offset". The same holds for the school and university
years of every country but the 8.

**Not carried, with the reason for each.**

- A company's own fiscal year, and any retailer's published calendar as a
  named system (not yet done; no source for any named company's calendar
  was read, beyond one 10-K cited in Sources). The caller writes it as a
  `WeekYearSystem`.
- The 13-period retail calendar of four-week periods (not yet done; no
  source read).
- Sub-national fiscal years, such as a state's or a province's, among them
  the Australian colonies' moves to 30 June in 1870 to 1904 as Wikipedia
  gives them (not yet done; no instrument read).
- The years before each system's first year read, and the systems before
  the ones above whose labels or dates no page gives: Japan's calendar-year
  and October years of 明治2 to 明治7, the Indian 1 May year before 1867, the
  Canadian 1 July year to 1906, the English year to 5 April of 1854, the
  Thai April years before B.E. 2481 and the October year of B.E. 2481 or
  2482, the German 1 April year of the Reichshaushaltsordnung, the Soviet
  1 October year to 1930, Egypt's years of 1913, 1926 and 1946 (a think
  tank's account), Nepal's March-to-February year of 1951 and Qatar's
  31 March year-end (not yet done; no page read gives their years or
  labels).
- The transitional periods themselves: Sweden's half year of 1923 and
  eighteen months of 1995/96, Japan's 明治18年度, Thailand's nine-month
  B.E. 2483 and 2504, the Canadian nine months of 1906 to 1907, and
  England's 1751 and 1752 (not yet done; the entries' notes say what was
  not found).
- Singapore's Year of Assessment, which follows the basis period, and the
  personal tax years of every country but the United Kingdom, New Zealand,
  Singapore, South Africa and Hong Kong (not yet done; no source read).
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
2026-10-04 with a page read; the first column is the system, the second the
source read and the last the result. A page is read through the fetch tool,
which summarises it, so a quotation is the summary's.

| System | Source read | Result |
| --- | --- | --- |
| United States, 1 October, end year; the 1976 transition quarter | [cornell-31-usc-1102]; [us-crs-98-325] | agrees; FY1977 is the first October year |
| United States, 1 July, to FY1976; the calendar year before it | [us-hdoc-27-16]; [us-sdoc-27-371]; [us-crs-98-325] | the first July year is FY1844, and the Act of 26 August 1842 was not read |
| United Kingdom government, 1 April | [uk-interpretation-act-1978-sch1]; [uk-public-revenue-act-1854]; [uk-exchequer-audit-act-1866]; [uk-hansard-1856-budget] | agrees; established by the Act of 1854, which only permits the accounts to 31 March, and first reported for 1855-56 |
| United Kingdom tax, 6 April, start year | [uk-income-tax-act-2007-s4] | agrees |
| Japan, 1 April; the July year before it | [jp-natarchives-kaikei-nendo]; [jp-kaikeiho-1889]; [jp-egov-zaiseiho] | agrees; April from 明治19年度 (1886), July from 明治8年度 (1875); the revision of the 会計法 in 1921 was not read |
| Japan, the school year | [jp-egov-gakko-kyoiku-hou-shikou-kisoku]; [jp-mext-shogakko-rei-1900] | art. 59 agrees; applied from 1 April 1947 |
| India, 1 April | [india-general-clauses-act-3-21]; [in-arthapedia-financial-year] | agrees; the year is dated to 1867 and no instrument is named |
| Canada, 1 April | [canada-faa-s2]; [ca-hoc-procedure-and-practice-ch18]; [ca-hoc-annotated-standing-orders] | agrees; the July year ended in 1906; neither title (Estimates, Public Accounts) was read |
| Germany, calendar year | [germany-bho-4]; [de-hgrg-1969] | agrees; in force 1 January 1970 |
| France, calendar year | [fr-lolf-2001-692]; [fr-cc-2001-448-dc] | agrees; the LOLF's budget year from 2006 |
| Russia, calendar year | [russia-budget-code-art12] | agrees in substance; the wording of art. 12 was unstable between reads |
| Brazil, calendar year | [br-lei-4320-1964]; [br-lei-4320-lexml] | agrees |
| Sweden | [se-ku-1993-94-18]; [se-riksdagsordning-2014]; [se-prop-1994-95-100]; [se-riksgaldskontoret-1925]; [se-bokforingslag-1976]; [se-bokforingslag-1999] | agrees; set by the Riksdag Act, a statute |
| South Africa, 1 March, end year | [sars-tax-year]; [za-income-tax-act-94-1983] | agrees; the government year's definition was not read |
| New Zealand tax, 1 April | [ird-nz-tax-year] | dates agree; the page does not name the year |
| Thailand, 1 October, end year; first used for 2505 from 1 October 2504 | [pridi-budget-year-history]; [thai-wikipedia-budget-year]; [wikipedia-thai-solar-calendar] | agrees; the two disagree on whether the first October year was B.E. 2481 or 2482 |
| Iran, 1399 to 1410 | [wikipedia-solar-hijri-calendar]; [ir-public-accounting-law-1366]; [ir-public-accounting-law-1349] | 12 of 12 agree; the Act of 1349 (read on one site) is the first year read, 1350 |
| Ethiopia, 8 July | [et-chilot-efy-2014-budget]; [indexmundi-ethiopia-fiscal-year]; [wikipedia-ethiopian-calendar] | agrees; Proclamation 648/2009 is a PDF |
| Nepal, 16 July | [wikipedia-fiscal-year]; [np-myrepublica-budget-history] | agrees for 2024; the Act of 2076 is a PDF |
| Egypt, 1 July | [eg-albawab-fiscal-year]; [eg-countrystudies-public-finance]; [eg-law-53-1973] | agrees; July from 1980, calendar year under the law of 1973 |
| Australia, 1 July | [treasury-au-reporting-periods]; [wikipedia-australian-financial-year] | agrees; no instrument read |
| Hong Kong, 1 April | [hk-inland-revenue-ordinance-1947]; [hk-budget-2025-public-finance]; [hk-ird-budget-2026-27] | agrees; the Public Finance Ordinance's definition was not read |
| China, calendar year | [cn-budget-law-1994]; [cn-budget-law-current]; [cn-state-budget-regulation-1991] | agrees; article 10 in 1994, article 18 since the amendment of 2014 |
| Singapore, 1 April | [sg-legal-wires-financial-procedure-act-1966]; [wikipedia-income-tax-in-singapore]; [wikipedia-fiscal-year] | dates from secondary pages; the Act's wording was not read |
| Pakistan, 1 July | [pk-finance-division]; [wikipedia-fiscal-year] | dates from a secondary page |
| The academic years | [uk-education-act-1996-s579]; [fr-code-education-l521-1]; [de-wikipedia-hamburger-abkommen]; [de-wikipedia-schuljahr]; [au-vic-school-term-dates]; [au-nsw-term-dates-2027]; [au-wa-term-dates]; [nz-wikipedia-education]; [us-ecs-instructional-time] | the Japanese school year and the French calendar's legal basis agree; no page gives the date of the United Kingdom's, France's, New Zealand's or the United States' |

The Australian states give Term 1 of 2027 on 28 January, 1 February and 3 or
10 February; the Education Act 1996 s. 579 defines the school year by "the
first school term to begin after July"; and the one page on New Zealand's
term dates says they are set by individual schools, so the entry does not
claim that a statute or a regulation fixes them. Those are recorded in the
entries' notes.

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

- Inland Revenue's page for New Zealand's tax year gives the dates of "the
  1 April 2024 to 31 March 2025 tax year" and does not name it, so the
  end-year label rests on a search result, and the entry says so.
- The Public Revenue and Consolidated Fund Charges Act 1854 s. 2 is
  permissive ("if they shall see fit"), so the table is read from 1855,
  the first year Hansard reports as ending on 31 March.
- Law 53 of 1973, art. 2 as first issued, has Egypt's year 1 January to
  31 December; the July year is from 1980, and the six months of 1980 before
  it are in neither.
- The Budget Law of China has the clause in article 10 in 1994 and in
  article 18 since 2014.
- Sweden's budget year is set by the Riksdag Act, so its authority is a
  statute; the months 1 May, 1 July and 1 September for a company's
  *brutet räkenskapsår* are in Bokföringslag (1976:125) § 12, and the law of
  1999 names none.
- The Thai pages disagree on whether the October year began in B.E. 2481 or
  2482; neither gives its labels, and the entry does not carry it. B.E. 2504
  can only have been nine months long if 2505 began on 1 October 2504 and
  the calendar year came before it, so the calendar-year system ends at
  2503.
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

Read directly as HTML on 2026-10-03 or, for the entries added or changed
afterwards, on 2026-10-04, unless noted; the fetch tool summarises a page.

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

Read on 2026-10-04:

- [jp-natarchives-kaikei-nendo], [jp-kaikeiho-1889], [jp-egov-zaiseiho],
  [jp-crd-shin-nendo]: the years of Japan's 会計年度 from 明治2 to 明治19, the
  会計法 of 1889 and the 財政法 of 1947; [jp-egov-gakko-kyoiku-hou-shikou-kisoku],
  [jp-mext-shogakko-rei-1900], [jp-mext-gakusei-hyakunen-koutou-kyouiku]:
  the school years.
- [in-arthapedia-financial-year]: India's 1867. [hk-inland-revenue-ordinance-1947],
  [hk-hku-public-finance-ordinance], [hk-budget-2025-public-finance],
  [hk-ird-budget-2026-27]: Hong Kong. [sg-legal-wires-financial-procedure-act-1966],
  [wikipedia-income-tax-in-singapore]: Singapore.
  [wikipedia-thai-solar-calendar]: B.E. 2483. [wikipedia-australian-financial-year]:
  the colonies and the Commonwealth. [pk-finance-division]: Pakistan.
  [cn-budget-law-1994], [cn-budget-law-current], [cn-state-budget-regulation-1991]: China.
- [us-hdoc-27-16], [us-sdoc-27-371], [us-crs-98-325]: the United States
  before 1977. [ca-hoc-procedure-and-practice-ch18],
  [ca-hoc-annotated-standing-orders]: Canada in 1906. [br-lei-4320-1964],
  [br-lei-4320-lexml]: Brazil. [uk-public-revenue-act-1854],
  [uk-exchequer-audit-act-1866], [uk-hansard-1856-budget],
  [uk-hansard-1854-financial-statement], [wikipedia-old-style-and-new-style-dates],
  [wikipedia-lady-day]: the United Kingdom. [de-hgrg-1969], [de-bho-1969]:
  Germany. [fr-lolf-2001-692], [fr-cc-2001-448-dc], [wikipedia-fr-lolf]:
  France. [russia-budget-code-art12], [wikipedia-ru-financial-year]:
  Russia. [se-ku-1993-94-18], [se-ku-1995-96-21], [se-prop-1994-95-100],
  [se-riksdagsordning-2014], [se-riksgaldskontoret-1925],
  [se-riksdag-document-list-1909-1920], [se-bokforingslag-1976],
  [se-bokforingslag-1999]: Sweden.
- [eg-law-53-1973], [eg-countrystudies-public-finance], [eg-albawab-fiscal-year]:
  Egypt. [sars-tax-year], [za-income-tax-act-94-1983],
  [za-treasury-pfma]: South Africa. [ir-public-accounting-law-1349],
  [ir-public-accounting-law-1366]: Iran. [et-chilot-efy-2014-budget]:
  Ethiopia. [np-myrepublica-budget-history]: Nepal.
- [uk-education-act-1996-s579], [fr-code-education-l521-1],
  [de-wikipedia-hamburger-abkommen], [de-wikipedia-schuljahr],
  [au-vic-school-term-dates], [au-nsw-term-dates-2027], [au-wa-term-dates],
  [nz-wikipedia-education], [us-ecs-instructional-time]: the academic years.
- The 39 countries of the table above are read from the pages and
  instruments their entries' `sources` name; the PwC Worldwide Tax Summaries
  and Wikipedia pages behind the twelve are named there too.

## Code

`crates/hc-fiscal/src/year_system.rs` (the core type, `StartCalendar`,
`YearStart`, `LabelConvention`, `Authority`), `quarters.rs`, `countries.rs`
and `countries/more.rs` (the 39 countries read from one page or instrument),
`academic.rs` and `retail.rs`; the lines are `crates/hyper-calendar/src/fiscal_lines.rs`.
The crate's tests pass (2026-10-04). The tests that anchor it:
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
`only_japan_and_france_claim_a_national_school_year`,
`a_year_the_system_was_in_force_in_but_no_source_reaches_is_a_gap`,
`the_us_federal_year_was_the_calendar_year_until_the_act_of_1842`,
`japans_april_year_began_in_1886_and_the_july_year_before_it_in_1875`,
`a_system_the_notes_say_existed_is_a_gap_before_the_year_read_not_absent`,
`egypt_moved_from_the_calendar_year_to_july_in_1980`,
`no_system_answers_a_year_before_the_first_year_its_sources_reach` and
`a_year_no_source_reaches_is_a_gap_and_not_an_answer`.
`crates/hyper-calendar/tests/facade.rs` checks that the `fiscal` feature
reaches the crate.
