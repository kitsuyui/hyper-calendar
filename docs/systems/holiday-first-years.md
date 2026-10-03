# Where each national table of the Americas, Europe, Africa, the Middle East, Oceania and Asia begins

Each national table of `hc-holiday` answers from one year and reports every
year before it as a gap. This document says which year that is for the
tables of the Americas, Europe, Africa, the Middle East, Oceania and Asia, and why.

## What it is

A holiday table is built from the law or list a person could read. A statute
read as it stands in 2026 says what the days are in 2026; it does not say
what they were in 1850, or in 1990, unless the text carries its own history.
ADR 0013 gives a rule two facts: `valid_from`, the year the day was
established, before which it is absent, and `read_from`, the first year the
sources read answer for, before which the engine reports a gap. Audit 10
(a1) found that 135 of the 195 national tables answered every year back to
1500 with every day present and no gap, because most rules carried neither.

## How it works

`countries::read_all(first, rules)` takes a table's rules and sets
`read_from(first)` on each that has none of its own. A rule that begins
later than `first` stays absent before it, which is an answer; a rule that
has its own `read_from` keeps it. Brazil is read from 2003, the first whole
year after Lei nº 10.607 of 19 December 2002, whose wording of article 1 of
Lei nº 662 of 1949 is the text read. Asked for 1990, `BRAZIL` gives no day
and one gap for each of the thirteen rules that apply then (Our Lady of
Aparecida began in 1980, Black Awareness Day only in 2024, so it is absent),
each naming the rule and carrying the table's `sources`; asked for 2003, it
gives the days. Asked for 1949, the year of the law, it still gives gaps: that
law was read only as the law of 2002 words it.

A year's first value is the first *whole* year the earliest instrument or
list read states. Where the instrument took effect part-way through a
year, the next year is the first (Brazil's law of 19 December 2002, Cuba's
Code of 17 June 2014); where only the year of enactment is known, that
year is. Where only one recent list was read — Peru's, Finland's,
Cyprus's, Malta's — the first year is the list's own.

Saudi Arabia's table is read from 2025, the Labour Law's executive
regulation as published in April 2025; its Founding Day began in 2022 by
Royal Order A/371 and its National Day in 2005. Asked for 2021 it gives no
day and nine gaps, the eight Eid and Arafah days and National Day, none of
them read for that year; Founding Day is absent, for the order had not been
made. Asked for 2023 it gives no day and ten gaps, Founding Day's among them,
as Aparecida's years before 2003 are; asked for 2025 it gives all ten days
and no gap. Tunisia's table is read from 1961, the year of its earliest
decree read, and every rule begins in 1961 or later, so before 1961 it gives
no day and no gap. Where the helper of a table (Ghana's, Rwanda's,
Somalia's) had set `valid_from` to the table's first year, as if the days had
been established then, it no longer does: `read_all` gives the first year and
the days of earlier years are gaps (Ghana, 2018: no day, ten gaps; 2019:
thirteen days).

Where an Act's text was read as a revised edition or as amended to a date,
the first year is that of the edition or the amendment; where only a list was
read (Iran's, Uganda's, Tanzania's, Zambia's, Zimbabwe's, Malawi's, Palau's)
it is that of the list, 2026, for no instrument with a date was read, and
reading the Act or decree would lower it. Australia has two first years: 2000
for the days every state keeps, from the two Acts read, and 2026, set on the
rule itself (`read_from` wins over `read_all`), for the day of one state or
territory.

## What is carried

| Code | Table | First year | Why |
| --- | --- | --- | --- |
| AD | Andorra | 2024 | The Government's decrees approving the work calendars of 2024 to 2026 (Decrets 487/2023, 409/2024 and 340/2025); Law 31/2018 leaves the days to the yearly decree and no earlier decree was read |
| AG | Antigua And Barbuda | 2006 | The Public Holidays (Amendment) Act 2005, published on 15 September 2005, from its first full year; the revised Schedule it replaced was not carried |
| AR | Argentina | 2011 | Decreto 1584/2010, in force from 2011; Ley 24.445 (1995) and the earlier years were not read, and 2017, Güemes's day in 2016 and 2017 are gaps because Decreto 52/2017 was not read |
| BB | Barbados | 1998 | The Public Holidays Act, Cap. 352 (L.R.O. 1998); the Act's notes cite amendments of 1974 to 1998 without saying which entry each made |
| BO | Bolivia | 2017 | Decreto Supremo 2750 of 1 May 2016, from its first whole year; the departments' days are read from their own instruments, 2026 from Decreto Supremo 5521, and every other year's decree that moves or adds days is a gap |
| BR | Brazil | 2003 | Lei nº 662 of 1949, article 1, in the wording of Lei nº 10.607 of 19 December 2002, from the first whole year after it; the wording before it was not read |
| BS | Bahamas | 1973 | The Public Holidays Act (Ch. 36, L.R.O. 1/2017), whose Schedule dates no entry before Independence Day of 1973; the other days are undated |
| BZ | Belize | 2020 | The Government's notices of 2020 to 2026; the Holidays Act, Chapter 289, could not be read |
| CL | Chile | 1981 | The first change that the laws read date, the Day of National Liberation; the older holidays (Ley 2.977 of 1915) are known only through Wikipedia |
| CO | Colombia | 1984 | Ley 51 de 1983, from its first full year |
| CR | Costa Rica | 2020 | Article 148 of the Código de Trabajo as reformed by Ley 9803 (2020); the text before the reform was not read |
| CU | Cuba | 2015 | Law 116 of 2013, in force from 17 June 2014, from its first whole year; the Government's grant of Good Friday in 2012 and 2013 is not carried, and 2014 is a gap |
| DM | Dominica | 2021 | The Government's lists of 2021 to 2026; the Act's Schedule of 1990 is undated beyond that and the Labour Day order was not found |
| DO | Dominican Republic | 1998 | Ley 139-97, in force from 27 June 1997, from its first whole year; the earlier law was not read |
| EC | Ecuador | 2017 | The law of Registro Oficial 906 of 20 December 2016, from 2017; the Código del Trabajo's earlier text was not read |
| GD | Grenada | 2017 | The Bank Holidays (Amendment) Act No. 2 of 2017, the latest amendment of the Act as revised to Act 19 of 1999 that was read; the years between 2000 and 2016 were not read |
| GT | Guatemala | 2018 | Decreto 19-2018, in force from 18 October 2018, and the Constitutional Court's ruling of 2020; article 127 of the Código de Trabajo was not read, so the years before are a gap |
| GY | Guyana | 2012 | The Public Holidays Act, Chapter 19:07, L.R.O. 1/2012; the yearly lists were read for 2017, 2018 and 2024 to 2026 |
| HN | Honduras | 1959 | The Código del Trabajo, Decreto 189 of 1959, article 339, in the edu-honduras.info copy, whose date of revision was not found |
| HT | Haiti | 1985 | The Code du travail, décret du 24 février 1984, articles 109 to 111, from its first whole year |
| JM | Jamaica | 1961 | The Schedule of the Holidays (Public General) Act, whose earliest dated entry is Labour Day of 1961, which replaced Empire Day |
| KN | Saint Kitts And Nevis | 2003 | The Public Holidays Act, Cap. 23.23, in the revised edition to 31 December 2002, from the first year after it |
| LC | Saint Lucia | 2005 | The Bank Holidays Act in the Revised Laws (2023), as amended to Act 5 of 2004, from the first year after it |
| MX | Mexico | 2006 | Article 74 of the Ley Federal del Trabajo as reformed in 2006; the text before the reform was not read |
| NI | Nicaragua | 1997 | Ley 185, La Gaceta 205 of 30 October 1996, from its first whole year |
| PA | Panama | 2008 | The Código de Trabajo, articles 46 and 47, as amended by Ley 70 of 28 December 2007, from 2008 |
| PE | Peru | 2026 | The only list read, the Spanish Wikipedia's for 2026 (Decreto Legislativo 713 and its amendments were not read) |
| PY | Paraguay | 1990 | Ley 8/90; nothing earlier is stated |
| SR | Suriname | 2007 | S.B. 2007 no. 98, the earliest amendment of the Besluit Vrije Dagen 1971 read |
| SV | El Salvador | 1995 | The Código de Trabajo, article 190, in the text of Decreto Legislativo 408 of 1995 |
| TT | Trinidad and Tobago | 1996 | The Public Holidays and Festivals Act (Chap. 19:05, updated to 31 December 2016), from the year of the latest dated entry of its Schedule, Spiritual Baptist Liberation Shouter Day of 1996; Labour Day and Republic Day carry no first year |
| UY | Uruguay | 1997 | Ley 16.805 of 24 December 1996, from its first whole year; the years before were not read |
| VC | Saint Vincent And The Grenadines | 2019 | The Prime Minister's Office's list for 2019, the earliest read; 2020 is a gap because its list was not read |
| VE | Venezuela | 2013 | The Ley Orgánica del Trabajo, los Trabajadores y las Trabajadoras (Gaceta Oficial Extraordinaria 6.076 of 7 May 2012), from its first whole year; the five days of the Ley de Fiestas Nacionales (Gaceta Oficial 29.541 of 22 June 1971) are read from 1972 |
| GB | United Kingdom | 1971 | The Banking and Financial Dealings Act 1971 |
| IE | Ireland | 1998 | The Organisation of Working Time Act 1997, section 21 and the Second Schedule, from the first whole year after it |
| FR | France | 2017 | Article L3133-1 of the Code du travail, in force since 10 August 2016, from the first whole year; the versions before it were not read |
| DE | Germany | 1990 | The Einigungsvertrag of 1990; the Länder's laws read are Brandenburg's of 1991 (to 2015) and Berlin's, and the other fourteen were not read |
| IT | Italy | 1949 | Legge 27 maggio 1949, n. 260 |
| PT | Portugal | 2016 | The Código do Trabalho article 234 in the wording of Lei n.º 8/2016, from 2016; Lei n.º 23/2012, which suspended four holidays for 2013 to 2015, was not read |
| NL | Netherlands | 2011 | The Algemene termijnenwet, article 3, in force from 10 October 2010, from the first whole year |
| BE | Belgium | 1975 | The royal decree of 18 April 1974, from its first whole year |
| AT | Austria | 1968 | The Feiertagsruhegesetz 1957 as amended by BGBl. Nr. 264/1967, in force from 26 July 1967, from the first whole year; the Arbeitsruhegesetz of 1983 keeps the list |
| SE | Sweden | 1989 | Lag (1989:253) om allmänna helgdagar |
| NO | Norway | 1995 | LOV-1995-02-24-12, with lov om 1. og 17. mai (LOV-1947-04-26-1) |
| DK | Denmark | 2024 | Lov nr. 214 of 6 March 2023, in force on 1 January 2024: the statute that lists the other helligdage was not read, so every earlier year is a gap, Store bededag's last year included |
| FI | Finland | 2026 | The only list read, the Finnish Wikipedia's "Pyhäpäivä"; the statutes were not read |
| PL | Poland | 2011 | The ustawa of 18 January 1951 as consolidated in 2025 and amended in 2010 (Epiphany from 2011); the amendments between 1951 and 2010, such as those of 1989 and 1990, were not read |
| CZ | Czechia | 2001 | Zákon č. 245/2000 Sb., from its first whole year |
| GR | Greece | 2022 | Νόμος 4808/2021, from its first whole year |
| HU | Hungary | 2013 | The Labour Code of 2012 (2012. évi I. törvény), in force from 1 July 2012, from its first whole year |
| RO | Romania | 2012 | The Codul muncii, article 139, as the Wikipedia list dates its amendments, the earliest being Legea 147/2012 |
| RU | Russia | 1991 | The changes Wikipedia dates from 1991, on no statute read; the Labour Code's article 112 is read for 2005 onward |
| UA | Ukraine | 2015 | The Kodeks zakoniv pro pratsyu, article 73, with the amendments read from law 238-VIII (2015); the text before it was not read |
| HR | Croatia | 2002 | The consolidated text of the Act in NN 136/2002, from 2002 |
| SK | Slovakia | 2021 | The Act's version in force from 1 November 2025 and the years Wikipedia dates, the earliest being 2021 |
| SI | Slovenia | 1992 | The ZPDPD, Uradni list RS 26/91, from its first whole year |
| IS | Iceland | 1998 | Lög nr. 32/1997 um frið vegna helgihalds and lög nr. 88/1971, article 6, in the Lagasafn of 1 September 2026, from the first year after 1997 |
| BG | Bulgaria | 2017 | The Labour Code article 154(2) as amended by SG 105/2016, in force from 1 January 2017 |
| CY | Cyprus | 2026 | The only list read, the Greek Wikipedia's, and the bank holidays law as CyLaw gives it undated |
| EE | Estonia | 1994 | The Public Holidays and Days of National Importance Act of 1994 and the act of 27 January 1998 |
| LV | Latvia | 1995 | The earliest year the Latvian Wikipedia dates among the amending laws |
| LT | Lithuania | 1990 | The Law on Holidays of 1990 |
| AL | Albania | 1993 | Law 7651 of 21 December 1992, from its first whole year |
| ME | Montenegro | 2007 | The Law on State and Other Holidays of 2007 |
| MK | North Macedonia | 2007 | The Law on Holidays as amended in 2007 |
| RS | Serbia | 2002 | The Law on State and Other Holidays (Official Gazette 43/2001), from its first whole year |
| BA | Bosnia and Herzegovina | see below | Each entity's own law: the Federation's days from 2016 (the Ministry's notices) and its Independence Day and Statehood Day from the laws of 1995, Republika Srpska's from 2007 (Official Gazette 43/07), Brčko's from 2002 (Official Gazette of the District 19/02) |
| BY | Belarus | 1991 | The earliest year the Russian Wikipedia dates, Independence Day of 27 July 1991 |
| LU | Luxembourg | 2019 | The law of 25 April 2019 and the Inspection du travail's list |
| MT | Malta | 2026 | The National Holidays and Other Public Holidays Act (Cap. 252) as read undated; the amendments' years are not carried |
| MD | Moldova | 2009 | The earliest change the sources date, Christmas by the new style from 2009; the Code's text was not read |
| LI | Liechtenstein | 1986 | The Labour Act, article 18(2), as amended by LGBl. 1986 Nr. 85 |
| MC | Monaco | 1966 | Law 798 of 18 February 1966 |
| SM | San Marino | 2014 | Law 152 of 30 October 2013 for the civil holidays; the religious days are read from 2025, the first of the Central Bank's calendars read |
| VA | Vatican City | 2011 | The Governorate's General Regulation of 21 November 2010, in force from 2011 |
| AE | United Arab Emirates | 2025 | Cabinet Resolution No. 27 of 2024, in force from 1 January 2025; the Resolution of 2019 it repealed was not read |
| AO | Angola | 2011 | Lei n.º 10/11 of 16 February 2011 |
| AU | Australia | 2000 | The Holidays Act 1983 (Qld) and the Statutory Holidays Act 2000 (Tas), the two Acts read, the later of which began in 2000, for the days every state keeps; 2026 for a day of one state or territory, whose own Act was not read (the NSW Government's list and secondary sources) |
| BF | Burkina Faso | 2015 | Loi n° 079-2015/CNT |
| BH | Bahrain | 2012 | Law 36 of 2012, article 64 |
| BI | Burundi | 2021 | Décret n° 100/150 of 7 June 2021, which amends the décret of 2006 |
| BJ | Benin | 1990 | Loi n° 90-019 of 27 July 1990 |
| BW | Botswana | 2006 | The Public Holidays Act, Cap. 03:07 (Act 17 of 2006) |
| CD | Democratic Republic of the Congo | 2014 | Ordonnance n° 14/010 of 14 May 2014 |
| CG | Republic of the Congo | 1994 | Loi n° 2-94 of 1 March 1994 |
| CI | Côte d'Ivoire | 1996 | Décret n° 96-205 of 7 March 1996 |
| CM | Cameroon | 1976 | Loi n° 73/5 of 7 December 1973 as loi n° 76/8 of 8 July 1976 amended it |
| CV | Cabo Verde | 1991 | Lei n.º 16/IV/91 of 30 December 1991 |
| DJ | Djibouti | 1978 | Arrêté n° 77-347/PR/MI of 4 October 1977 and the arrêtés that rectify it, from 1978, its first full year |
| DZ | Algeria | 1963 | Law 63-278 of 26 July 1963, articles 1, 3 and 4 |
| EG | Egypt | 2026 | Labour Law No. 14 of 2025, in force from 1 September 2025, so 2026 is its first whole year |
| ET | Ethiopia | 2024 | Proclamation No. 1334/2024 of 14 August 2024 |
| FJ | Fiji | 1985 | The Public Holidays Act (Cap. 101), 1985 edition |
| FM | Micronesia | 2014 | The Code of the Federated States of Micronesia (2014), title 1, chapter 6 |
| GA | Gabon | 2024 | The Ministry of Labour's communiqué of August 2024, the oldest dated source read; décret n° 00727 of 1998 was not read |
| GH | Ghana | 2019 | The 2019 list of public holidays, the oldest list read |
| GM | The Gambia | 2021 | The Office of the President's declarations of 2021 to 2026 |
| GN | Guinea | 2022 | Décret D/2022/0526 of 2 November 2022 |
| GQ | Equatorial Guinea | 2007 | Decreto núm. 9/2007 of 5 February 2007 |
| GW | Guinea-Bissau | 2023 | Decreto n.º 1/2023 of 18 January 2023, read as a newspaper quotes it |
| IL | Israel | 1951 | The Hours of Work and Rest Law of 1951, sections 2(b) and 7, the oldest text read for the days of rest |
| IQ | Iraq | 2024 | Official Holidays Law No. 12 of 2024 |
| IR | Iran | 2026 | The lists of Wikipedia, "Public holidays in Iran", retrieved 2026-09-22, the only list read; no instrument with a date was read |
| JO | Jordan | 2007 | The Prime Minister's Official Bulletin No. 6 of 2007 |
| KE | Kenya | 2022 | The Revised Edition 2022 of the Public Holidays Act, Cap. 110, the edition read |
| KI | Kiribati | 1977 | The Public Holidays Ordinance (Cap. 81), 1977 revised edition |
| KM | Comoros | 2026 | Décret n° 25-147 of 19 December 2025, from 2026, its first full year |
| KW | Kuwait | 2010 | Law 6 of 2010, article 68 |
| LB | Lebanon | 2005 | Decree 15215 of 27 September 2005 |
| LR | Liberia | 2012 | The President's holiday proclamations of 2012 to 2026, the oldest dated list read |
| LS | Lesotho | 1996 | The Public Holidays Act 1995 (Act No. 7 of 1995), in its first year, 1996 |
| LY | Libya | 2012 | Law No. 5 of 2012 |
| MA | Morocco | 1977 | Décret n° 2-77-169 of 28 February 1977, which the later décrets amend |
| MG | Madagascar | 2023 | The décret of 4 January 2023, the oldest of the yearly décrets read |
| MH | Marshall Islands | 1988 | The Public Holidays Act 1988 |
| ML | Mali | 2005 | Loi n° 05-040 of 22 July 2005 |
| MR | Mauritania | 1992 | Loi n° 92-018 of 7 December 1992 |
| MU | Mauritius | 2017 | The Prime Minister's Office's General Notice No. 814 of 2016 for the public holidays of 2017, the oldest dated list read; the Act (22 of 1968) was read as amended to 2019 |
| MW | Malawi | 2026 | The Public Holidays Act Cap. 18:05 and the 2026 list; the Act's own date was not read |
| MZ | Mozambique | 2023 | Lei n.º 13/2023 of 25 August 2023; the table under Lei n.º 23/2007 was read only as a secondary source reports it |
| NA | Namibia | 1990 | The Public Holidays Act 26 of 1990 |
| NE | Niger | 2023 | The Council of Ministers' text of 2023 restoring 3 August, the oldest dated source read; loi n° 97-20 of 1997 was not read |
| NG | Nigeria | 2004 | The Laws of the Federation of Nigeria 2004 text of the Public Holidays Act, Cap. P40, the edition read |
| NR | Nauru | 2023 | The Government Gazette of 30 December 2022, the list for 2023, the oldest dated list read |
| NZ | New Zealand | 2010 | Employment New Zealand's list of 2010 to 2025, the oldest dated list read; the Acts were not read |
| OM | Oman | 2020 | Royal Decree 56/2020, read with Royal Decree 88/2022 |
| PG | Papua New Guinea | 1982 | The 1982 revised edition of the Public Holidays Act 1953 (Chapter 321) |
| PS | Palestine | 2025 | The Council of Ministers' 2025 tables |
| PW | Palau | 2026 | The Palau National Code Annotated, Supp. 12, whose date was not read, retrieved 2026-09-23 |
| QA | Qatar | 2025 | Emiri Decision No. 57 of 2025; Cabinet Decision No. 6 of 2008, which a decision of 2025 amends, was read only as amended |
| RW | Rwanda | 2017 | Presidential Order n° 54/01 of 24 February 2017 |
| SA | Saudi Arabia | 2025 | The Labour Law's executive regulation, article 24, as published in April 2025; the regulation's own date was not read |
| SB | Solomon Islands | 1996 | The 1996 edition of the Public Holidays Act (Cap. 151) |
| SC | Seychelles | 1991 | The Public Holidays Act, Cap. 190, 1991 edition |
| SD | Sudan | 2025 | The Council of Ministers' announcements of December 2025 to August 2026 |
| SL | Sierra Leone | 2020 | The Government Notices of 2020, the oldest dated list read; the Act of 1960 was read as an undated revised-edition copy |
| SN | Senegal | 1974 | Loi n° 74-52 of 4 November 1974 |
| SO | Somalia | 2025 | Law No. 36 of 24 December 2024, the Labour Code of 2024, from 2025 |
| SS | South Sudan | 2022 | The Labour Act, 2017 (Act No. 64), read with the Ministry of Labour's calendar for 2022, from which the days are carried |
| SY | Syria | 2025 | Decree No. 188 of 2025 |
| SZ | Eswatini | 1998 | EswatiniLII's consolidation of the Public Holidays Act 1938 as at 1 December 1998 |
| TD | Chad | 1997 | Décret n° 97-413 of 30 September 1997 |
| TG | Togo | 1987 | Loi n° 87-08 of 9 June 1987 |
| TN | Tunisia | 1961 | Décret 61-144 of 1961 and the decrees after it, as the French Wikipedia chronicles them |
| TO | Tonga | 2020 | The Public Holidays Act, Chapter 8.11, 2020 Revised Edition |
| TR | Türkiye | 1981 | Law 2429 of 17 March 1981, whose text and amendments were read |
| TV | Tuvalu | 2022 | The Public Holidays Act, Cap. 4.50, 2022 Revised Edition |
| TZ | Tanzania | 2026 | The 2026 list, the only dated list read; the Act's own date was not read |
| UG | Uganda | 2026 | The 2026 list of the Consulate in Arusha, the only dated list read; no Act was read |
| VU | Vanuatu | 2006 | The Public Holidays Act [Cap. 114], Consolidated Edition 2006 |
| WS | Samoa | 2023 | The Public Holidays Act 2008 as revised to 31 December 2023 |
| YE | Yemen | 2000 | Law No. 2 of 2000 |
| ZA | South Africa | 1995 | The Public Holidays Act 36 of 1994, in force 1 January 1995 |
| ZM | Zambia | 2026 | HONO's 2026 list, the only dated list read; the Act's own date was not read |
| ZW | Zimbabwe | 2026 | General Notice 1361 of 2025, the 2026 list; the Act's own date was not read |

Four tables are read from more than one year. Venezuela's five days of the
Ley de Fiestas Nacionales are read from 1972 and the Labour Law's from 2013;
San Marino's civil holidays (law 152 of 2013) from 2014 and its religious
days from 2025; Bosnia and Herzegovina's three entities each from their own
law, which is why its nationwide set is empty (the Federation 2016, with its
Independence Day and Statehood Day from 1995; Republika Srpska 2007; Brčko
District 2002); and Bolivia's departmental days from the instruments cited
on each, which `docs/systems/bolivia-holidays.md` lists.

The United States, Spain, Switzerland and Canada's provinces had their first
years before audit 10: the United States' federal days begin at the year each
statute set them (`valid_from`) and its states' days at the year of the code
read for each; Spain's
nationwide days from 1990, the first whole year of article 45 as read;
Switzerland's from 2023, the latest first year of a cantonal law read, and
each canton from its own law; Canada's provincial days from 2026, the year
of the texts read. Canada's federal list, which the audit's task set aside,
still answers every year and is a follow-up.

### Asia

The tables of `asia.rs`, and Bhutan's Thimphu days in `bhutan.rs`, each
wrapped in `read_all` with these first years. A year before
them answers no day and reports a gap; `tests/asia_first_years.rs` checks
the first year and the one before for every row.

| Code | Table | First year | Why |
| --- | --- | --- | --- |
| AF | Afghanistan | 2023 | The first Gregorian year wholly under the Islamic Emirate's calendars; the Republic's list was not read |
| AM | Armenia | 2022 | The Law on Holidays and Remembrance Days as it stands, read through Wikipedia's reproductions; the longer New Year break before 2022 and the amendments of 2002 to 2021 are not given by the sources |
| AZ | Azerbaijan | 2007 | Labour Code art. 105 as amended on 8 December 2006, whose five-day Novruz is the first form read; National Salvation Day and Armed Forces Day (1998), State Flag Day (2010) and Victory Day (2021) keep their own years |
| BD | Bangladesh | 2025 | The Ministry of Public Administration's notification of 21 October 2024; the Hijri days of 2025 and 2026 are the notifications' own dates, and the tabular calendar answers from 2027 |
| BN | Brunei | 2023 | The Prime Minister's Office's circular for 2023, the earliest read |
| BT | Bhutan | 2025 | The Ministry of Home Affairs' list for 2025, the earliest read; Thimphu's festivals in `BT-15` likewise, with 2021 a gap in any case |
| CN | China | 1999 and 2008 | The statutory days from the 1999 revision of the 放假办法, which was not read, so 1998 and before are a gap; the State Council's arrangements from 2008. The table was already bounded before audit 10 |
| GE | Georgia | 2011 | The Labour Code adopted on 17 December 2010 (matsne.gov.ge); which of its 29 amendments of 2012 to 2026 changed art. 30 was not established |
| HK | Hong Kong | 1998 | The Schedule to Cap. 149 in force from 18 September 1998, the earliest version read; 1997 rests on the Chinese Wikipedia alone |
| ID | Indonesia | 2020 | The SKB for 2020, the earliest read |
| IN | India | 2025 | The DoPT's Office Memorandum of 9 July 2024 for 2025; the 2017 and 2018 memoranda were read only as GConnect reproduces them. The states' days keep their own first years (2019 or 2023) |
| KG | Kyrgyzstan | 2005 | The Labour Code of 4 August 2004, from its first whole year |
| KH | Cambodia | 2021 | The sub-decree for 2021, the earliest read; 2023's was not read and is a gap |
| KP | North Korea | 2020 | The wall calendar of 2020 as the Institute for Peace and Unification Studies transcribes it |
| KR | South Korea | 1949 | The 1949 decree (제124호); the rules carry their own dates as well |
| KZ | Kazakhstan | 2002 | The Law of 13 December 2001, from its first whole year |
| LA | Laos | 2018 | Decree No. 386 of 15 December 2017, from its first whole year |
| LK | Sri Lanka | 2023 | The Gazette order for 2023 (No. 2287/4), the earliest read; the Act of 1971 and the orders before 2023 were not read |
| MM | Myanmar | 2026 | The list read, Wikipedia's "Public holidays in Myanmar" as of 2026-09-22, secondary and undated; Deepavali's notices of 2020 to 2025 answer in their own years |
| MN | Mongolia | 2004 | The Law of 18 December 2003, from its first whole year; the five days of Naadam and Tsagaan Sar from 2014 (the amendments of 2014 and 2013 are noted in the consolidated text without their content), 10 July from the law of 28 June 2022 |
| MO | Macau | 2001 | Executive Order 60/2000 (Boletim Oficial No. 40/2000, October 2000), from its first whole year; the Bulletin's own page was not reachable |
| MV | Maldives | 2016 | The Monetary Authority's list for 2016, the earliest read |
| MY | Malaysia | 2020 | The Prime Minister's Department's list for 2020, the earliest read; the weekend law of the states keeps its own years |
| NP | Nepal | 2023 | The Home Ministry's notice for 2080 BS, which began on 14 April 2023; the days of January to April fall in 2024, which is their first year |
| PH | Philippines | 2012 | Proclamation No. 295 for 2012, the first of the annual proclamations read; the Administrative Code as amended by Republic Act 9849 (2009) was not read for 2010 and 2011 |
| PK | Pakistan | 2026 | The Cabinet Division's list for 2026 as Business Recorder reports it; Iqbal Day's years of 2014 and 2022 to 2025 are gaps with it |
| SG | Singapore | 2020 | The Ministry of Manpower's public-holiday release for 2020, the earliest read; the releases of 2017 and 2018 (Deepavali) are pinned as the rule's days only |
| TH | Thailand | 1992 | The Bank of Thailand's list for 1992, the earliest read |
| TJ | Tajikistan | 2012 | The Law on Holidays of 2 August 2011, from its first whole year |
| TL | Timor-Leste | 2006 | Law No. 10/2005 of 10 August 2005, from its first whole year |
| TM | Turkmenistan | 2010 | The Labour Code of 18 April 2009, from its first whole year |
| TW | Taiwan | 2012 | The 紀念日及節日實施辦法 as of 25 September 2012 and the DGPA calendars of 2012 to 2014; the swaps begin in 2017 |
| UZ | Uzbekistan | 2024 | The Labour Code of 28 October 2022, in force from 30 April 2023, from its first whole year; 1997 to 1999 are the rules' own dates, pinned as the rules' |
| VN | Vietnam | 2021 | The Labour Code 45/2019, in force from 1 January 2021; the Hùng Kings' Festival, a holiday from 2007 under the Code of 2012 that was not read, is a gap from 2007 to 2020 |

### The Russian republics that keep no day

Karelia (1999), Khakassia (2005), Mari El (2023) and Udmurtia (2020), whose
laws were read and give no day off, are each a rule scoped to the republic
with no day and `read_from` the year given, so that every earlier year is a
gap naming the republic; see `docs/systems/russia-transfers.md`. The test is
`the_republics_that_keep_no_day_are_a_gap_before_their_law` in
`tests/russia_republics.rs`.

### Bolivia, 2026

Decreto Supremo 5521 of 13 January 2026, read as pixilegal.com and
Infoleyes reproduce it, moves the holiday of Thursday 22 January to Friday
23 January and that of Sunday 21 June to Monday 22 June (article 4) and
makes Friday 5 June, after Corpus Christi, and Friday 7 August, after
Independence Day, national holidays for 2026 (article 3). The table
carries those four dates for 2026 and its ordinary dates for 2027
onward; neither 22 January nor 21 June is a holiday in 2026. The yearly
decrees of the other years, 2017 to 2025 and 2027 on, were not read, and
each of those years has a gap that says so.

### Switzerland's cantonal names

The three days every canton keeps — New Year's Day, Ascension and
Christmas — were given in German for every canton. They carry the names the
cantonal law read uses: Geneva's art. 1 ("1er Janvier", "Ascension",
"Noël"), Jura's arts. 3 and 4 ("Nouvel-An", "l'Ascension", "Noël"),
Neuchâtel's art. 3 ("le 1er janvier", "l'Ascension", "le jour de Noël") and
Ticino's art. 6 ("Capodanno", "Ascensione", "Natale"), each read on the
cantonal legislation site's HTML on 2026-10-03. The French texts of
Fribourg, Vaud and Valais are served only as scripted pages that could
not be read in HTML, so those three cantons carry no local name for the
trio and the English name stands.

## Accuracy

`tests/first_years.rs` checks for every row of the first table above that the first year has
days, that the years before it (one, ten and a hundred before, and 1500)
have none, and that the year just before is a gap. It pins Bolivia's 2026
from the decree, the 2017 and Güemes gaps of Argentina, Saint Vincent's 2020,
Venezuela's 1972, San Marino's two first years, and the cantons' names.

`tests/first_years_mea_oceania.rs` does the same for the 80 tables of the
Middle East, Africa and Oceania (first year answers a day, the year before
is a gap and not a complete answer, 1700 has no day, and every rule names
an establishment or a first year read), and pins Saudi Arabia's and Israel's
days that began before their table's first year. The existing country tests
that asserted a year before a table's first moved to later years, laid out
the same way where the weekday mattered (2033 and 2039 for 2022, 2030 for
2024), or dropped the anchor.

The first years are judgements about what the sources support, and they are
conservative: a table's days of earlier years may well be what the table
would have said, and the table says only that they were not read. Where
a source was a secondary list (Wikipedia for Chile, Latvia, Russia, Romania)
the first year is the earliest year it dates.

## Sources

For the Middle East, Africa and Oceania, each table's `sources` string names
what was read, with its retrieval date; they are not repeated here. Read
for this change on 2026-10-03: Wikipedia's "Public holidays in Iran" and
"Public holidays in Kenya" (en.wikipedia.org), which give no year for the law
of either, so Iran's first year is that of its list and Kenya's that of its
Revised Edition; and, for the Victorian and Western Australian days of 2026
that the Australian tests pin, workcalc.com.au's "Victoria Public Holidays"
and "WA Public Holidays" (https://workcalc.com.au/victoria-public-holidays/
and https://workcalc.com.au/wa-public-holidays/, a private aggregator,
secondary), which give Labour Day on 9 March, the King's Birthday on 8 June
and Melbourne Cup Day on 3 November, and Western Australia Day on 1 June.


Each instrument is cited in its table's `sources` string and, for the ones
read in a form `references.bib` keys, there. Read on 2026-10-03: Decreto
Supremo 5521 [bo-ds-5521-pixilegal] [bo-ds-5521-infoleyes], Geneva's Loi sur
les jours fériés, art. 1, Neuchâtel's Loi sur le dimanche et les jours
fériés, art. 3, Jura's Loi sur les jours fériés officiels et le repos
dominical, arts. 3 and 4, and Ticino's LALL, art. 6. The Spanish Wikipedia's
"Anexo:Días festivos en Perú", read the same day, lists the year 2026 alone
and says nothing of when Flag Day, Air Force Day, the Battle of Junín or the
Battle of Ayacucho became holidays. Not read: the Gaceta Oficial's PDFs and
the French texts of Fribourg, Vaud and Valais.

## Code

The tables of the Middle East, Africa and Oceania are in
`countries/africa_middle_east.rs` and `oceania.rs`, each section's first year
a `*_READ_FROM` constant passed to `read_all`; their tests are
`tests/first_years_mea_oceania.rs`.


`crates/hc-holiday/src/countries/mod.rs` has `read_all`; each table's
`rules` static is wrapped in it in `americas.rs`, `europe.rs`, `andorra.rs`,
`bolivia.rs`, `mexico.rs`, `asia.rs` and `bhutan.rs`. The tests are `tests/first_years.rs`, and the
dated years each table encodes are pinned in `tests/countries.rs`.
