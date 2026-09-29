# Canada's federal and provincial holidays

Canada's holidays are set in fourteen jurisdictions: the federal
Parliament for the employers it regulates, and each province and
territory for the rest. This document covers `hc-holiday`'s `CANADA`
table: the federal list asked for no region, and each province's and
territory's list, the federal days its text keeps and its own, as rules
scoped to its ISO 3166-2 code.

## What it is

The Canada Labour Code, s. 166, lists the general holidays of federally
regulated employers — banks, airlines, telecommunications, the federal
public service among them: New Year's Day, Good Friday, Victoria Day,
Canada Day, Labour Day, the National Day for Truth and Reconciliation
from 2021, Thanksgiving, Remembrance Day, Christmas Day and Boxing Day.
Every other employer is under its province's or territory's
employment-standards law, and each of those has its own list: most keep
most of the federal days, several add one of their own in February or
August, and several leave federal days out — Ontario keeps neither
Remembrance Day nor the National Day for Truth and Reconciliation, and
New Brunswick, Newfoundland and Labrador, Nova Scotia and Prince Edward
Island keep neither Victoria Day nor Thanksgiving [ca-on-esa] [ca-nl-lsa]
[ca-ns-lsc].

## How it works

Ontario's Employment Standards Act, 2000, s. 1(1), lists "Family Day,
being the third Monday in February" among its public holidays; 2007,
c. 16, Sched. A, s. 1 added it, in force 3 December 2007, so the first
was 18 February 2008 [ca-on-esa]. `CANADA` asked for `CA-ON` gives
Family Day on Monday 16 February 2026, a day off; asked for `CA-QC` it
does not, and the day is a business day in Montreal. The same Act has no
August holiday: the "Civic Holiday" of the first Monday of August is a
municipal and customary day in Ontario, not a statutory one, and
`CA-ON` gives nothing on 3 August 2026.

## What is carried

- **The federal days**, asked for no region: the Canada Labour Code's
  list, for the employers it covers.
- **The federal days a province's text leaves out**, each excepted from
  the province (`HolidayRule::except_in`), from the lists read in 2026:
  | Federal day | Not in the list of |
  | --- | --- |
  | Good Friday | CA-QC, whose article 60 gives Good Friday or Easter Monday at the employer's choice |
  | Victoria Day | CA-NB, CA-NL, CA-NS, CA-PE; CA-QC keeps the Monday as the Journée nationale des patriotes |
  | Canada Day | CA-NL, whose 1 July is Memorial Day |
  | National Day for Truth and Reconciliation | CA-AB, CA-NB, CA-NL, CA-NS, CA-NU, CA-ON, CA-QC, CA-SK; CA-BC, CA-MB, CA-NT, CA-PE and CA-YT keep it under their own lists |
  | Thanksgiving | CA-NB, CA-NL, CA-NS, CA-PE |
  | Remembrance Day | CA-MB, CA-ON, CA-QC; CA-NS, whose Code leaves it to a Remembrance Day Act not read, a gap every year |
  | Boxing Day | CA-AB, CA-BC, CA-MB, CA-NB, CA-NL, CA-NS, CA-NT, CA-NU, CA-PE, CA-QC, CA-SK, CA-YT |

  The lists are the texts as they stand when read, so each day a province
  leaves out is a gap before 2026 in that province (`Rule::NO_DAY` read
  from 2026): an earlier text, not read, may have kept it. British
  Columbia's National Day for Truth and Reconciliation and Manitoba's
  Orange Shirt Day are carried from 2021, the federal day's first year,
  and are gaps until 2026. Prince Edward Island's and the Northwest
  Territories' are carried from 2022 and Yukon's from 2023, the first
  30 September after the acts that added them (Prince Edward Island's of
  17 November 2021, Yukon's of 24 November 2022; the Northwest Territories'
  public service alone had the day in 2021). Yukon's Boxing Day is a gap
  before 2023, the year of the text read, and Quebec's Victoria Day before
  2003, when its Monday took the patriotes' name. Newfoundland and
  Labrador's Memorial Day, 1 July, is carried from 2026, a gap before.
- **Each province's and territory's own days**, scoped to its code, as
  `Kind::Public`: from the year a source read gives as the day's first,
  the years before absent; and where no source gives one, read from 2026,
  the years before a gap (`HolidayRule::read_from`). The table below says
  which.
- **Family Day and the August Monday.** Family Day, the third Monday of
  February, is carried from the year each province's law begins it:
  Alberta's from 1990, Saskatchewan's from 2007 and New Brunswick's from
  2018, the year its Act came into force. Ontario and Manitoba have no
  Civic Holiday on the first Monday of August in their laws, and none is
  carried there; the five provinces and territories that keep a day on
  that Monday have it under the name their laws give it.
- **Not yet carried, and why:**
  - the Northwest Territories' statute, published as PDF only: its days
    are the Department's list, which names the August Monday only as the
    "First Monday in August";
  - the Sunday rules that differ from the federal one: Quebec's 2 July
    when 1 July is a Sunday, and its 25 June when 24 June is, for the
    employees for whom Sunday is not a working day;
  - Yukon's Heritage Day and the other days kept by one government's
    public service only (Newfoundland and Labrador's St Patrick's,
    St George's, Discovery, Orangemen's and Regatta days among them);
  - the weekend moves the provincial laws make, where they differ from
    the federal one the table applies to the Monday days.

### The thirteen

Read on 2026-09-29. CanLII refused the connection, as did the statute
sites of New Brunswick, Quebec, Nova Scotia, Prince Edward Island, Yukon
and Nunavut; Alberta, Saskatchewan and the Northwest Territories publish
their statutes as PDF only. So British Columbia's, Manitoba's, Ontario's
and Newfoundland and Labrador's statutes and New Brunswick's Family Day
Act were read on their own sites, and Quebec's Loi sur les normes du
travail and Loi sur la fête nationale and New Brunswick's Employment
Standards Act in the Internet Archive's captures of those sites; Yukon's,
Nova Scotia's and Prince Edward Island's in the captures of CanLII's
copies (Nova Scotia's and Prince Edward Island's current to 2022, checked
against the governments' pages of 2026 and 2024); the rest rest on the
governments' own pages or on secondary sources, which the entries name. The whole lists
of British Columbia's s 1 "statutory holiday", Manitoba's s 21(1)
"general holiday", Ontario's s 1(1) "public holiday", Newfoundland and
Labrador's s 14(1) "public holiday", and Alberta's, Saskatchewan's and
Nunavut's pages were re-read the same day for the federal days each keeps.

| Code | Province or territory | Days carried | Instrument | First year, and the years before | Not carried |
| --- | --- | --- | --- | --- | --- |
| CA-AB | Alberta | Family Day, third Monday of February | Employment Standards Code, RSA 2000, c E-9, as the Government's page states it [ca-ab-page]; first year from Wikipedia | established 1990 (Wikipedia, first celebrated) | Heritage Day, an optional day |
| CA-BC | British Columbia | Family Day, second Monday of February 2013–2018, third Monday from 2019; British Columbia Day, first Monday of August | Family Day Act and Regulation [ca-bc-family-day]; British Columbia Day Act and Employment Standards Act, s 1 [ca-bc-esa] | Family Day established 2013 (Family Day Act 2012), third Monday from 2019; British Columbia Day 2026, earlier years: gap | |
| CA-MB | Manitoba | Louis Riel Day, third Monday of February | The Employment Standards Code, s 21(1)(a.1), from S.M. 2007, c. 18 [ca-mb-esc] | established 2008 (S.M. 2007, c. 18) | Terry Fox Day, which names the first Monday of August and makes it no holiday |
| CA-NB | New Brunswick | Family Day, third Monday of February; New Brunswick Day, first Monday of August | An Act Respecting Family Day, in force 1 January 2018 [ca-nb-family-day]; Employment Standards Act, s 1, consolidated to 12 June 2026 [ca-nb-esa-2026] | Family Day established 2018 (the Act, in force 1 January 2018); New Brunswick Day 2026, earlier years: gap | |
| CA-NL | Newfoundland and Labrador | Memorial Day, 1 July, in place of Canada Day; the Labour Standards Act, s 14, keeps New Year's Day, Good Friday, Remembrance Day, Memorial Day, Labour Day and Christmas Day [ca-nl-lsa] | Labour Standards Act, RSNL 1990, c L-2, s 14 | 2026; earlier years: gap | the provincial public service's days |
| CA-NS | Nova Scotia | Nova Scotia Heritage Day, third Monday of February | Labour Standards Code, s 2(ga) [ca-ns-lsc-2022], as amended in force 1 January 2015, from the Government's page [ca-ns-lsc] | established 2015 (in force 1 January 2015) | Remembrance Day, under a Remembrance Day Act not read |
| CA-NT | Northwest Territories | National Indigenous Peoples Day, 21 June; Civic Holiday, first Monday of August; National Day for Truth and Reconciliation, 30 September | Employment Standards Act, SNWT 2007, c 13, s 22, not read; the Department's list [ca-nt-ece] and news release of 2022; the June date from Wikipedia [ca-nt-secondary] | National Indigenous Peoples Day established 2001 (Wikipedia); Civic Holiday 2026, earlier years: gap; 30 September from 2022 | |
| CA-NU | Nunavut | Nunavut Day, 9 July; Civic Holiday, first Monday of August | Labour Standards Act, not read; the Labour Standards Compliance Office's fact sheet [ca-nu-lsco]; Nunavut Day's date and year from Wikipedia | Nunavut Day established 2001 (Wikipedia); Civic Holiday 2026, earlier years: gap | |
| CA-ON | Ontario | Family Day, third Monday of February | Employment Standards Act, 2000, s 1(1), from 2007, c. 16, Sched. A [ca-on-esa] | established 2008 (2007, c. 16, Sched. A) | |
| CA-PE | Prince Edward Island | Islander Day, second Monday of February 2009, third Monday from 2010; National Day for Truth and Reconciliation, 30 September | Employment Standards Act, s 6(1), current to 1 June 2022 [ca-pe-esa-2022], and the Government's page of 2024; Islander Day's first year from Wikipedia [ca-pe-secondary] | Islander Day established 2009 (Wikipedia); 30 September from 2022 (SPEI 2021, c 33) | |
| CA-QC | Quebec | Saint-Jean-Baptiste Day, 24 June; National Patriots' Day (Journée nationale des patriotes), the Monday before 25 May; Good Friday or Easter Monday, at the employer's choice, both observances | Loi sur la fête nationale, arts. 1 and 2 [ca-qc-fn]; Loi sur les normes du travail, art. 60 [ca-qc-lnt]; the Premier's communiqué of 24 November 2002 for Décret 1322-2002 [ca-qc-patriotes] | Saint-Jean-Baptiste Day 1979, the first year of the Act of 1978, 1978 and before: gap; the patriotes from 2003, Victoria Day's Monday a gap before; Good Friday or Easter Monday 2026, earlier years: gap | 25 June when 24 June is a Sunday, for some employees |
| CA-SK | Saskatchewan | Family Day, third Monday of February; Saskatchewan Day, first Monday of August | The Saskatchewan Employment Act, as the Government's page states it [ca-sk-page]; Family Day's first year from Wikipedia | Family Day established 2007 (Wikipedia); Saskatchewan Day 2026, earlier years: gap | |
| CA-YT | Yukon | National Indigenous Peoples Day, 21 June; Discovery Day, third Monday of August; National Day for Truth and Reconciliation, 30 September | Employment Standards Act, s 1, in force from 20 April 2023 [ca-yt-esa-2023], and the Government's page of 2026, for the dates | National Indigenous Peoples Day established 2017 (SY 2017, c 1); Discovery Day 2026, earlier years: gap; 30 September from 2023 (SY 2022, c 18) | Heritage Day, for the public service only |

## Accuracy

`crates/hc-holiday/tests/canada_provinces.rs` holds each day to its
source: its date in its first year and in 2026, its kind and region,
nothing the year before; Ontario's Family Day absent in 2007 and Quebec's
Fête nationale a gap in 1978; Quebec's patriotes on 19 May 2003 and
18 May 2026, and its Good Friday and Easter Monday of 2026 as observances;
the day of 30 September in Prince Edward Island and the Northwest
Territories from 2022 and in Yukon from 2023; British Columbia's move of 2019; Islander
Day on 9 February 2009 and 15 February 2010; no August holiday in Ontario
or Manitoba; no Family Day in New Brunswick in 2017; and each province's
federal days in 2026, with a gap in 2025 for each it leaves out.
`canada_federal_and_provincial_holidays` in
`crates/hc-holiday/tests/countries.rs` keeps the older checks. What a
reader should know: half the days rest on secondary sources, and the
days whose first year no source gave start in 2026, though most are far
older, with the years before reported as gaps; the statutes, once
readable, would give both. The first years from Wikipedia are secondary.

## Sources

| Key | Used for | Read |
| --- | --- | --- |
| [ca-on-esa] | Ontario's public holidays and Family Day's amendment | Yes, 2026-09-29, e-Laws |
| [ca-bc-family-day] | Family Day in British Columbia, before and after 2018 | Yes, 2026-09-29, BC Laws point in time |
| [ca-bc-esa] | British Columbia Day and the statutory holidays | Yes, 2026-09-29, BC Laws |
| [ca-mb-esc] | Louis Riel Day and Terry Fox Day | Yes, 2026-09-29, Manitoba's consolidation |
| [ca-nb-family-day] | New Brunswick's Family Day | Yes, 2026-09-29, the bill as tabled |
| [ca-nb-esa] | New Brunswick's public holidays | A search excerpt only |
| [ca-nl-lsa] | Newfoundland and Labrador's holidays | Yes, 2026-09-29, the House of Assembly's site |
| [ca-ns-lsc] | Nova Scotia's holidays and Heritage Day | The Government's pages, 2026-09-29 |
| [ca-ab-page], [ca-sk-page], [ca-nu-lsco] | Alberta's, Saskatchewan's and Nunavut's holidays | The governments' pages, 2026-09-29 |
| [ca-nt-secondary], [ca-pe-secondary], [ca-qc-secondary], [ca-yt-secondary] | The days of the jurisdictions whose statutes could not be read at first; the June date and first year in the Northwest Territories, Islander Day's first year | Secondary, 2026-09-29 |
| [ca-qc-lnt], [ca-qc-fn], [ca-qc-patriotes] | Quebec's article 60, its Fête nationale and the patriotes | Yes, 2026-09-29, the Internet Archive's captures of LégisQuébec and of the Premier's site |
| [ca-nb-esa-2026] | New Brunswick's public holidays | Yes, 2026-09-29, the Internet Archive's capture of laws.gnb.ca |
| [ca-ns-lsc-2022], [ca-pe-esa-2022], [ca-yt-esa-2023] | Nova Scotia's, Prince Edward Island's and Yukon's lists | Yes, 2026-09-29, the Internet Archive's captures of CanLII's copies, with the governments' pages |
| [ca-nt-ece] | The Northwest Territories' list and its day of 30 September | Yes, 2026-09-29, the Department's and the Government's pages |

## Code

`crates/hc-holiday/src/countries/canada.rs`: the federal rules first in
`CA_RULES`, each excepting the provinces whose list leaves it out, then
`not_kept`'s gaps before 2026 for those (Yukon's from 2023, Quebec's
Victoria Day from 2003), the days of 30 September that Prince Edward
Island and the territories added, Quebec's choice of Good Friday or
Easter Monday and its patriotes, then the provinces' own days,
built by `provincial` and `provincial_fixed`, with `read_from` on the days
without an established year, and one region and one citation constant per
jurisdiction. The exchange `XTSE` keeps its own list. Anchors: the tests
in `crates/hc-holiday/tests/canada_provinces.rs`.
