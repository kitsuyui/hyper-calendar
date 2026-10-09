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
sources read answer for, before which the engine reports a gap. A rule that
carries neither answers every year back to 1500 with its day present and no
gap; so each table of the Americas, Europe, Africa, the Middle East, Oceania
and Asia is wrapped in `read_all` with its first year.

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
decree read; the days that decree lists are read from 1961 and no source read
dates their establishment, so before 1961 they are a gap, and the days a
later decree added (Evacuation Day, Women's Day) are absent before it. Where
the helper of a table (Ghana's, Rwanda's, Somalia's) sets no `valid_from` to
the table's first year, as if the days had been established then: `read_all`
gives the first year and the days of earlier years are gaps (Ghana, 2018: no
day, ten gaps; 2019: thirteen days).

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
| AD | Andorra | 2024 | The Government's decrees approving the work calendars of 2024 to 2026 (Decrets 487/2023, 409/2024 and 340/2025); Law 31/2018 leaves the days to the yearly decree and no earlier decree was read. Read: Llei 31/2018, article 62, from the Cambra de Comerç's copy (ccis.ad), retrieved 2026-09-22, and the decrets 487/2023, 409/2024 and 340/2025 in the BOPA's HTML (bopa.ad), retrieved 2026-09-29. |
| AG | Antigua And Barbuda | 2006 | The Public Holidays (Amendment) Act 2005, published on 15 September 2005, from its first full year; the revised Schedule it replaced was not carried. Read: the Act and its Amendment Acts of 2005, 2014 and 2019 as laws.gov.ag publishes them, retrieved 2026-09-23. |
| AR | Argentina | 2011 | Decreto 1584/2010, in force from 2011; Ley 24.445 (1995) and the earlier years were not read, and 2017, Güemes's day in 2016 and 2017 are gaps because Decreto 52/2017 was not read. Read: Decreto 1584/2010 at servicios.infoleg.gob.ar, retrieved 2026-09-22, and Ley 27.399; Ley 25.370 and Ley 27.258 known only through the Spanish Wikipedia. |
| BB | Barbados | 1998 | The Public Holidays Act, Cap. 352 (L.R.O. 1998); the Act's notes cite amendments of 1974 to 1998 without saying which entry each made. Read: the Act as barbadoslawcourts.gov.bb publishes it, retrieved 2026-09-22. |
| BO | Bolivia | 2017 | Decreto Supremo 2750 of 1 May 2016, from its first whole year; the departments' days are read from their own instruments, 2026 from Decreto Supremo 5521, and every other year's decree that moves or adds days is a gap. Read: the Decretos Supremos at lexivox.org, retrieved 2026-09-22; Decreto Supremo 405 of 2010 known only as the Ministry of Labour reported it. |
| BR | Brazil | 2003 | Lei nº 662 of 1949, article 1, in the wording of Lei nº 10.607 of 19 December 2002, from the first whole year after it; the wording before it was not read. Read: the laws on the Câmara dos Deputados' legislation site, retrieved 2026-09-26; of Lei nº 14.759 only its title and publication. |
| BS | Bahamas | 1973 | The Public Holidays Act (Ch. 36, L.R.O. 1/2017), whose Schedule dates no entry before Independence Day of 1973; the other days are undated. Read: the Act as laws.bahamas.gov.bs publishes it, retrieved 2026-09-22. |
| BZ | Belize | 2020 | The Government's notices of 2020 to 2026; the Holidays Act, Chapter 289, could not be read. Read: the Press Office's notices at pressoffice.gov.bz, retrieved 2026-09-22; the Act is known from them and from Wikipedia. |
| CL | Chile | 1981 | The first change that the laws read date, the Day of National Liberation; the older holidays (Ley 2.977 of 1915) are known only through Wikipedia. Read: Leyes 19.668, 20.215, 20.299, 20.983 and 21.357 at LeyChile, retrieved 2026-09-26; the table's comment says Ley 2.977's holidays are known only through Wikipedia, which its sources line, listing the law at LeyChile, does not bear out; Leyes 3.810, 18.432, 20.148, 20.629 and 20.663 not read. |
| CO | Colombia | 1984 | Ley 51 de 1983, from its first full year. Read: article 1 of the law at funcionpublica.gov.co (gestor normativo), retrieved 2026-09-22. |
| CR | Costa Rica | 2020 | Article 148 of the Código de Trabajo as reformed by Ley 9803 (2020); the text before the reform was not read. Known through the Ministry of Labour's communiqués and La Nación's reports; whether the laws' own texts were read is not recorded. |
| CU | Cuba | 2015 | Law 116 of 2013, in force from 17 June 2014, from its first whole year; the Government's grant of Good Friday in 2012 and 2013 is not carried, and 2014 is a gap. Read: articles 94 to 100 in the Ministry of Justice's 2014 edition (minjus.gob.cu), retrieved 2026-09-22. |
| DM | Dominica | 2021 | The Government's lists of 2021 to 2026; the Act's Schedule of 1990 is undated beyond that and the Labour Day order was not found. Read: the Act as dominica.gov.dm publishes it and the Government's lists of 2021 to 2026, retrieved 2026-09-23. |
| DO | Dominican Republic | 1998 | Ley 139-97, in force from 27 June 1997, from its first whole year; the earlier law was not read. Read: the law as the Suprema Corte de Justicia publishes it (justia.com), articles 1 to 4. |
| EC | Ecuador | 2017 | The law of Registro Oficial 906 of 20 December 2016, from 2017; the Código del Trabajo's earlier text was not read. Read: the law as the Municipality of Santo Domingo published it; the dates from El Universo and the Spanish Wikipedia. |
| GD | Grenada | 2017 | The Bank Holidays (Amendment) Act No. 2 of 2017, the latest amendment of the Act as revised to Act 19 of 1999 that was read; the years between 2000 and 2016 were not read. Read: the Act at grenadaparliament.gd and Amendment Act No. 2 of 2017 at laws.gov.gd, retrieved 2026-09-23. |
| GT | Guatemala | 2018 | Decreto 19-2018, in force from 18 October 2018, and the Constitutional Court's ruling of 2020; article 127 of the Código de Trabajo was not read, so the years before are a gap. Not read: article 127 of the Código (the ministry's and the congress's sites refused access), known through Wikipedia; Decreto 19-2018 read as cuasiabogadosgt.blogspot.com transcribes it, retrieved 2026-09-26. |
| GY | Guyana | 2012 | The Public Holidays Act, Chapter 19:07, L.R.O. 1/2012; the yearly lists were read for 2017, 2018 and 2024 to 2026. Read: the Act as the Ministry of Legal Affairs publishes it (mola.gov.gy), retrieved 2026-09-22. |
| HN | Honduras | 1959 | The Código del Trabajo, Decreto 189 of 1959, article 339, in the edu-honduras.info copy, whose date of revision was not found. Read: the Código in the edu-honduras.info copy, retrieved 2026-09-23. |
| HT | Haiti | 1985 | The Code du travail, décret du 24 février 1984, articles 109 to 111, from its first whole year. Read: the Code in the CEPAL copy (oig.cepal.org), retrieved 2026-09-22; the décrets of 1989 and 2024 known only through a law firm's article (HDIT Cabinet Volmar). |
| JM | Jamaica | 1961 | The Schedule of the Holidays (Public General) Act, whose earliest dated entry is Labour Day of 1961, which replaced Empire Day. Read: the Schedule as the Ministry of Labour and Social Security publishes it (mlss.gov.jm), retrieved 2026-09-22. |
| KN | Saint Kitts And Nevis | 2003 | The Public Holidays Act, Cap. 23.23, in the revised edition to 31 December 2002, from the first year after it. Read: the revised edition as the St. Kitts and Nevis Law Commission publishes it, retrieved 2026-09-23. |
| LC | Saint Lucia | 2005 | The Bank Holidays Act in the Revised Laws (2023), as amended to Act 5 of 2004, from the first year after it. Read: the Revised Laws (2023) through the Internet Archive's copy of the Attorney General's Chambers' text, retrieved 2026-09-23. |
| MX | Mexico | 2006 | Article 74 of the Ley Federal del Trabajo as reformed in 2006; the text before the reform was not read. Read: article 74 in a secondary copy (conceptosjuridicos.com), retrieved 2026-09-26, the Cámara de Diputados' text being unreachable. |
| NI | Nicaragua | 1997 | Ley 185, La Gaceta 205 of 30 October 1996, from its first whole year. Read: articles 66 to 68 at the Asamblea Nacional's Normas Jurídicas (legislacion.asamblea.gob.ni), retrieved 2026-09-23. |
| PA | Panama | 2008 | The Código de Trabajo, articles 46 and 47, as amended by Ley 70 of 28 December 2007, from 2008. Whether the Código's text was read is not recorded: the table cites the Gaceta Oficial, a ministry consulta and Wikipedia's list (retrieved 2026-09-22). |
| PE | Peru | 2026 | The only list read, the Spanish Wikipedia's for 2026 (Decreto Legislativo 713 and its amendments were not read). Not read: Decreto Legislativo 713 and its amendments (El Peruano and gob.pe refused access); the 2026 list is the Spanish Wikipedia's. |
| PY | Paraguay | 1990 | Ley 8/90; nothing earlier is stated. Read: the laws at the Biblioteca y Archivo Central del Congreso (bacn.gov.py), retrieved 2026-09-23. |
| SR | Suriname | 2007 | S.B. 2007 no. 98, the earliest amendment of the Besluit Vrije Dagen 1971 read. Read: the Besluit and its amendments in the SRIS copies (sris.sr), retrieved 2026-09-23. |
| SV | El Salvador | 1995 | The Código de Trabajo, article 190, in the text of Decreto Legislativo 408 of 1995. Read: the Código and its reform decrees at the Asamblea Legislativa's Índice Legislativo, retrieved 2026-09-23. |
| TT | Trinidad and Tobago | 1996 | The Public Holidays and Festivals Act (Chap. 19:05, updated to 31 December 2016), from the year of the latest dated entry of its Schedule, Spiritual Baptist Liberation Shouter Day of 1996; Labour Day and Republic Day carry no first year. Read: the Act as laws.gov.tt publishes it, retrieved 2026-09-22. |
| UY | Uruguay | 1997 | Ley 16.805 of 24 December 1996, from its first whole year; the years before were not read. Read: the laws on IMPO (impo.com.uy), retrieved 2026-09-26. |
| VC | Saint Vincent And The Grenadines | 2019 | The Prime Minister's Office's list for 2019, the earliest read; 2020 is a gap because its list was not read. Read: the Prime Minister's Office's list for 2026 at pmoffice.gov.vc, retrieved 2026-09-23, and those for 2019 and 2021 to 2025 through the Internet Archive's copies. |
| VE | Venezuela | 2013 | The Ley Orgánica del Trabajo, los Trabajadores y las Trabajadoras (Gaceta Oficial Extraordinaria 6.076 of 7 May 2012), from its first whole year; the five days of the Ley de Fiestas Nacionales (Gaceta Oficial 29.541 of 22 June 1971) are read from 1972. Read: the Ley Orgánica at tugacetaoficial.com and Acceso a la Justicia, retrieved 2026-09-23, and the Ley de Fiestas Nacionales in the Justia copy of the gazette. |
| GB | United Kingdom | 1971 | The Banking and Financial Dealings Act 1971. Read: the Act on legislation.gov.uk, retrieved 2026-09-26; the royal proclamations up to 2023 not read. |
| IE | Ireland | 1998 | The Organisation of Working Time Act 1997, section 21 and the Second Schedule, from the first whole year after it. Read: the Act as the Law Reform Commission's Revised Acts publishes it and the Workplace Relations Commission's page, retrieved 2026-09-26. |
| FR | France | 2017 | Article L3133-1 of the Code du travail, in force since 10 August 2016, from the first whole year; the versions before it were not read. Read: the article on Légifrance, retrieved 2026-09-26; loi n° 81-893 not read. |
| DE | Germany | 1990 | The Einigungsvertrag of 1990; the Länder's laws read are Brandenburg's of 1991 (to 2015) and Berlin's, and the other fourteen were not read. Read: the Einigungsvertrag at gesetze-im-internet.de, Brandenburg's Feiertagsgesetz at BRAVORS and Berlin's as berlin.de lists it, retrieved 2026-09-26. |
| IT | Italy | 1949 | Legge 27 maggio 1949, n. 260. Read: the laws on Normattiva, retrieved 2026-09-26. |
| PT | Portugal | 2016 | The Código do Trabalho article 234 in the wording of Lei n.º 8/2016, from 2016; Lei n.º 23/2012, which suspended four holidays for 2013 to 2015, was not read. Read: article 234 on the Procuradoria-Geral Distrital de Lisboa's site (pgdlisboa.pt), retrieved 2026-09-26, the Diário da República's own page being unreadable. |
| NL | Netherlands | 2011 | The Algemene termijnenwet, article 3, in force from 10 October 2010, from the first whole year. Read: the Act and the Besluit of 2013 on wetten.overheid.nl, retrieved 2026-09-26. |
| BE | Belgium | 1975 | The royal decree of 18 April 1974, from its first whole year. Read: the arrêté royal on Justel, retrieved 2026-09-26; the loi du 4 janvier 1974 itself not read. |
| CH | Switzerland | 2023 for the nationwide days; 2000 for the cantons whose oldest text read is of that year | The Arbeitsgesetz (SR 822.11), Art. 20a Abs. 1, in the consolidation of 1 September 2023, whose first whole year is 2023; each canton's own days from the text of its law read, the laws before them not read. Read: the Arbeitsgesetz on Fedlex and the cantons' collections, retrieved 2026-09-29; Jura, Schwyz and Zurich as the German Wikipedia and the Canton of Zurich's page give them (secondary); the Bundesverfassung Art. 110 Abs. 3 not read. |
| ES | Spain | 1990 | The Real Decreto 2001/1983, arts. 45 and 46, as amended by the Real Decreto 1346/1989, from the first whole year after it; the communities' days are the resolutions' of 2013 to 2015 and 2018 to 2026, every other year a gap. Read: the Real Decreto on the BOE and the resolutions in the BOE's HTML text, retrieved 2026-09-29; the resolutions for 2016 and 2017, whose annexes are images, not read. |
| AT | Austria | 1968 | The Feiertagsruhegesetz 1957 as amended by BGBl. Nr. 264/1967, in force from 26 July 1967, from the first whole year; the Arbeitsruhegesetz of 1983 keeps the list. Read: the Arbeitsruhegesetz on jusline.at (secondary; the RIS being unreachable) and the federal laws of 1965 and 1957 as the RIS scans give them, retrieved 2026-09-26. |
| SE | Sweden | 1989 | Lag (1989:253) om allmänna helgdagar. Read: the Act on riksdagen.se, retrieved 2026-09-26. |
| NO | Norway | 1995 | LOV-1995-02-24-12, with lov om 1. og 17. mai (LOV-1947-04-26-1). Read: both laws on Lovdata, retrieved 2026-09-26. |
| DK | Denmark | 2024 | The royal resolution of 6 March 2023 (Lovtidende A 2023 nr. 270), which abolishes store bededag from 1 January 2024, and Lov nr. 214 of 2023 (§ 7 nr. 1, the enumerated list in lukkeloven § 2 stk. 1 from 2024), both read in full. No statute read defines the other holidays, so every earlier year is a gap, store bededag's last year, 2023, included |
| FI | Finland | 2026 | The only list read, the Finnish Wikipedia's "Pyhäpäivä"; the statutes were not read. Not read: laki 388/1937 (the Finlex page did not display its text), laki 272/1944 and kirkkolaki; the list is the Finnish Wikipedia's. |
| PL | Poland | 2011 | The ustawa of 18 January 1951 and its amendments of 1989 (Independence Day, and 15 August and 11 November on the list), 1990 (3 May, and 22 July removed), 2010 (Epiphany from 2011) and 2024 (Christmas Eve from 2025), read as summarised fetches of the ELI text; the consolidated text of 2025 was not read |
| CZ | Czechia | 2001 | Zákon č. 245/2000 Sb., from its first whole year. Read: the Act on Zákony pro lidi (the version of 13 May 2026), retrieved 2026-09-26. |
| GR | Greece | 2022 | Νόμος 4808/2021, from its first whole year. Read: Νόμος 4808/2021, article 60, on taxheaven.gr (secondary; the ΦΕΚ not read) and the Ministry's circular, retrieved 2026-09-26. |
| HU | Hungary | 2013 | The Labour Code of 2012 (2012. évi I. törvény), in force from 1 July 2012, from its first whole year. Whether the Code's text was read is not recorded: the table cites it and Wikipedia's list (retrieved 2026-09-22). |
| RO | Romania | 2012 | The Codul muncii, article 139, as the Wikipedia list dates its amendments, the earliest being Legea 147/2012. Whether the Code's text was read is not recorded: the table cites it with five amending laws as Wikipedia's list dates them (retrieved 2026-09-22). |
| RU | Russia | 1991 | The changes Wikipedia dates from 1991, on no statute read; the Labour Code's article 112 is read for 2005 onward. Read: article 112 and the Government's decrees as ConsultantPlus and Garant publish them, retrieved 2026-09-23; the years 1991 to 2002 from Wikipedia, no statute read. |
| UA | Ukraine | 2015 | The Kodeks zakoniv pro pratsyu, article 73, with the amendments read from law 238-VIII (2015); the text before it was not read. Read: the Kodeks's consolidated text of 31 July 2026 at zakon.rada.gov.ua and the cards of the amending laws. |
| HR | Croatia | 2002 | The consolidated text of the Act in NN 136/2002, from 2002. Read: the Act of 1996 as consolidated in NN 136/2002 on narodne-novine.nn.hr, retrieved 2026-09-26; the current text on zakon.hr (secondary). |
| SK | Slovakia | 2021 | The Act's version in force from 1 November 2025 and the years Wikipedia dates, the earliest being 2021. Read: the Act on Slov-Lex, in the version in force from 1 November 2025, retrieved 2026-09-26. |
| SI | Slovenia | 1992 | The ZPDPD, Uradni list RS 26/91, from its first whole year. Read: the ZPDPD as racunovodstvo.net consolidates it (secondary; the PISRS consolidation not read), retrieved 2026-09-26. |
| IS | Iceland | 1998 | Lög nr. 32/1997 um frið vegna helgihalds and lög nr. 88/1971, article 6, in the Lagasafn of 1 September 2026, from the first year after 1997. Read: both laws in the Lagasafn on althingi.is, retrieved 2026-09-26. |
| BG | Bulgaria | 2017 | The Labour Code article 154(2) as amended by SG 105/2016, in force from 1 January 2017. Whether the Labour Code's text was read is not recorded: the table cites it as gov.bg and the Bulgarian Wikipedia give it (retrieved 2026-09-22). |
| CY | Cyprus | 2026 | The only list read, the Greek Wikipedia's, and the bank holidays law as CyLaw gives it undated. Read: the bank holidays law on CyLaw, retrieved 2026-09-26; no statute listing the public holidays was found. |
| EE | Estonia | 1994 | The Public Holidays and Days of National Importance Act of 1994 and the act of 27 January 1998. Read: both Acts as the Estonian Wikipedia reproduces them with their amendment marks, retrieved 2026-09-22 (secondary). |
| LV | Latvia | 1995 | The earliest year the Latvian Wikipedia dates among the amending laws. Read: the consolidated law in force from 18 March 2025 at likumi.lv, retrieved 2026-09-22; the years from the Latvian Wikipedia. |
| LT | Lithuania | 1990 | The Law on Holidays of 1990. Not read: the Law on Holidays of 1990; the Labour Code's article 123 is known as the Lithuanian Wikipedia's list gives it (retrieved 2026-09-22). |
| AL | Albania | 1993 | Law 7651 of 21 December 1992, from its first whole year. Whether the text of Law 7651 was read is not recorded: the table cites it through festazyrtare.al and Wikipedia (retrieved 2026-09-22). |
| ME | Montenegro | 2007 | The Law on State and Other Holidays of 2007. Read: the Law as paragraf.me reproduces it, retrieved 2026-09-22 (secondary). |
| MK | North Macedonia | 2007 | The Law on Holidays as amended in 2007. Read: the Law as the Macedonian Wikisource reproduces it, retrieved 2026-09-22. |
| RS | Serbia | 2002 | The Law on State and Other Holidays (Official Gazette 43/2001), from its first whole year. Read: the Law as paragraf.rs reproduces it, retrieved 2026-09-22 (secondary). |
| BA | Bosnia and Herzegovina | see below | Each entity's own law: the Federation's days from 2016 (the Ministry's notices) and its Independence Day and Statehood Day from the laws of 1995, Republika Srpska's from 2007 (Official Gazette 43/07), Brčko's from 2002 (Official Gazette of the District 19/02). Read: the Federation's Ministry notices at fbihvlada.gov.ba, Republika Srpska's law at Paragraf Lex and the Serbian Wikisource, and Brčko's law at Paragraf Lex with the Assembly's decisions at skupstinabd.ba, retrieved 2026-09-23; the Federation's law and the 1995 laws known only as the notices and a school's page cite them. |
| BY | Belarus | 1991 | The earliest year the Russian Wikipedia dates, Independence Day of 27 July 1991. Read: the President's page of state holidays (president.gov.by), retrieved 2026-09-22; Decree 157's own text is not recorded as read, and the years are from the Russian Wikipedia. |
| LU | Luxembourg | 2019 | The law of 25 April 2019 and the Inspection du travail's list. Read: the Inspection du travail's page (itm.public.lu), retrieved 2026-09-22, which reproduces article L. 232-2 and the law; the law's own text is not recorded as read. |
| MT | Malta | 2026 | The National Holidays and Other Public Holidays Act (Cap. 252) as read undated; the amendments' years are not carried. Read: the Act as legislation.mt publishes it, retrieved 2026-09-22. |
| MD | Moldova | 2009 | The earliest change the sources date, Christmas by the new style from 2009; the Code's text was not read. Not read: the Code's text (legis.md); article 111 is known as zilelibere.md, a private aggregator, lists it (retrieved 2026-09-22). |
| LI | Liechtenstein | 1986 | The Labour Act, article 18(2), as amended by LGBl. 1986 Nr. 85. Read: the Arbeitsgesetz at gesetze.li, retrieved 2026-09-22. |
| MC | Monaco | 1966 | Law 798 of 18 February 1966. Read: the loi n° 798 as Legimonaco publishes it, retrieved 2026-09-22. |
| SM | San Marino | 2014 | Law 152 of 30 October 2013 for the civil holidays; the religious days are read from 2025, the first of the Central Bank's calendars read. Read: Legge 152 of 2013 from the Consiglio Grande e Generale's archive, retrieved 2026-09-22, and the Central Bank's calendars for 2025 and 2026. |
| VA | Vatican City | 2011 | The Governorate's General Regulation of 21 November 2010, in force from 2011. Read: the Regolamento generale at vatican.va and the ULSA's copy through the Internet Archive, retrieved 2026-09-23. |
| AE | United Arab Emirates | 2025 | Cabinet Resolution No. 27 of 2024, in force from 1 January 2025; the Resolution of 2019 it repealed was not read. Not read: Cabinet Resolution No. 27 of 2024 (uaelegislation.gov.ae refused access), known as legal-wires.com summarises it (secondary), nor Federal Decree-Law No. 33 of 2021, known as Gulf News reports it. |
| AO | Angola | 2011 | Lei n.º 10/11 of 16 February 2011. Read: the laws at AngoLEX (angolex.com), retrieved 2026-09-23. |
| AU | Australia | 2000 | The Holidays Act 1983 (Qld) and the Statutory Holidays Act 2000 (Tas), the two Acts read, the later of which began in 2000, for the days every state keeps; 2026 for a day of one state or territory, whose own Act was not read (the NSW Government's list and secondary sources). Read: the Queensland and Tasmanian Acts on legislation.qld.gov.au and legislation.tas.gov.au, retrieved 2026-09-26; the other states' Acts and the text of the Fair Work Act not read. |
| BF | Burkina Faso | 2015 | Loi n° 079-2015/CNT. Read: the loi at academiedepolice.bf, retrieved 2026-09-23; the law adopted on 9 January 2026 not read, known from press reports. |
| BH | Bahrain | 2012 | Law 36 of 2012, article 64. Known through the Labour Market Regulatory Authority and HONO, a private aggregator (secondary), retrieved 2026-09-22; whether the Law's own text was read is not recorded. |
| BI | Burundi | 2021 | Décret n° 100/150 of 7 June 2021, which amends the décret of 2006. Read: the décret in the Presidency's scan, through the Internet Archive, retrieved 2026-09-23. |
| BJ | Benin | 1990 | Loi n° 90-019 of 27 July 1990. Read: the lois at the Secrétariat général du Gouvernement's documenthèque (sgg.gouv.bj), retrieved 2026-09-23, of loi n° 90-019 the first page, articles 1 to 3. |
| BW | Botswana | 2006 | The Public Holidays Act, Cap. 03:07 (Act 17 of 2006). Read: the Act in the NATLEX copy, retrieved 2026-09-22. |
| CD | Democratic Republic of the Congo | 2014 | Ordonnance n° 14/010 of 14 May 2014. Read: the ordonnances at droitcongolais.info, retrieved 2026-09-23. |
| CG | Republic of the Congo | 1994 | Loi n° 2-94 of 1 March 1994. Read: the loi at the Secrétariat général du Gouvernement (sgg.cg), retrieved 2026-09-23. |
| CI | Côte d'Ivoire | 1996 | Décret n° 96-205 of 7 March 1996. Read: the décret of 2011 as loidici.biz reproduces it, retrieved 2026-09-22 (secondary); décret n° 96-205 itself is not recorded as read. |
| CM | Cameroon | 1976 | Loi n° 73/5 of 7 December 1973 as loi n° 76/8 of 8 July 1976 amended it. Read: loi n° 73/5 in the Ministry of Public Service's collection as the Internet Archive holds it, retrieved 2026-09-23; loi n° 76/8 known as Camerlex describes it. |
| CV | Cabo Verde | 1991 | Lei n.º 16/IV/91 of 30 December 1991. Read: the Lei in the Boletim Oficial at governo.cv, retrieved 2026-09-23. |
| DJ | Djibouti | 1978 | Arrêté n° 77-347/PR/MI of 4 October 1977 and the arrêtés that rectify it, from 1978, its first full year. Read: the arrêtés in the Journal officiel's eJO (journalofficiel.dj), retrieved 2026-09-23. |
| DZ | Algeria | 1963 | Law 63-278 of 26 July 1963, articles 1, 3 and 4. Read: articles 1, 3 and 4 of law 63-278 as legal-doctrine.com gives them (secondary), retrieved 2026-09-26. |
| EG | Egypt | 2026 | Labour Law No. 14 of 2025, in force from 1 September 2025, so 2026 is its first whole year. Read: Labour Law No. 14 of 2025 in Wikisource's text of the promulgating articles; the law's article on paid holidays and the official gazettes not read, known through press reports. |
| ET | Ethiopia | 2024 | Proclamation No. 1334/2024 of 14 August 2024. Read: the Proclamation as the Ministry of Justice publishes it (justice.gov.et), retrieved 2026-09-26. |
| FJ | Fiji | 1985 | The Public Holidays Act (Cap. 101), 1985 edition. Read: the Act and its Amendment Acts in PacLII's copies as the Internet Archive holds them (captured 2013-07-15 and 2009-01-08), and the Ministry of Information's lists of 2019 to 2026. |
| FM | Micronesia | 2014 | The Code of the Federated States of Micronesia (2014), title 1, chapter 6. Read: the Code and the Public Laws at fsmlaw.org, retrieved 2026-09-23. |
| GA | Gabon | 2024 | The Ministry of Labour's communiqué of August 2024, the oldest dated source read; décret n° 00727 of 1998 was not read. Not read: décret n° 00727 of 1998; the Ministry of Labour's communiqués read as the press reproduces them (secondary), retrieved 2026-09-26. |
| GH | Ghana | 2019 | The 2019 list of public holidays, the oldest list read. Not read: any instrument; the list is Wikipedia's and the Ghana News Agency's report (retrieved 2026-09-22). |
| GM | The Gambia | 2021 | The Office of the President's declarations of 2021 to 2026. Read: the Office of the President's declarations at op.gov.gm, retrieved 2026-09-26. |
| GN | Guinea | 2022 | Décret D/2022/0526 of 2 November 2022. Not read: the décret's own text, known as Guinée Nondi and Africa Guinée reproduce it, retrieved 2026-09-23. |
| GQ | Equatorial Guinea | 2007 | Decreto núm. 9/2007 of 5 February 2007. Read: the Decreto in the Boletín Oficial del Estado (boe.gob.gq), retrieved 2026-09-25. |
| GW | Guinea-Bissau | 2023 | Decreto n.º 1/2023 of 18 January 2023, read as a newspaper quotes it. Not read: Decreto n.º 1/2023, known as O Democrata and VOA Português quote its list (secondary). |
| IL | Israel | 1951 | The Hours of Work and Rest Law of 1951, sections 2(b) and 7, the oldest text read for the days of rest |
| IQ | Iraq | 2024 | Official Holidays Law No. 12 of 2024. Read: the Law from the Ministry of Justice's copy (moj.gov.iq), retrieved 2026-09-22. |
| IR | Iran | 2026 | The lists of Wikipedia, "Public holidays in Iran", retrieved 2026-09-22, the only list read; no instrument with a date was read |
| JO | Jordan | 2007 | The Prime Minister's Official Bulletin No. 6 of 2007. Read: the Official Bulletins on pm.gov.jo, retrieved 2026-09-26. |
| KE | Kenya | 2022 | The Revised Edition 2022 of the Public Holidays Act, Cap. 110, the edition read |
| KI | Kiribati | 1977 | The Public Holidays Ordinance (Cap. 81), 1977 revised edition. Read: the Ordinance and its Amendment Acts in PacLII's copies as the Internet Archive holds them (captured 2015-08-28 and 2024-12-23), and the Orders for 2025 and 2026 at president.gov.ki. |
| KM | Comoros | 2026 | Décret n° 25-147 of 19 December 2025, from 2026, its first full year. Read: the décret at the Union's legal portal (munganyo.km), retrieved 2026-09-25. |
| KW | Kuwait | 2010 | Law 6 of 2010, article 68. Read: article 68 as the Public Authority of Manpower published it in English and Kuwait Up To Date summarises it, retrieved 2026-09-22. |
| LB | Lebanon | 2005 | Decree 15215 of 27 September 2005. Read: the Presidency of the Council of Ministers' reproduction (pcm.gov.lb) of Decree 15215 and its amendments, retrieved 2026-09-22. |
| LR | Liberia | 2012 | The President's holiday proclamations of 2012 to 2026, the oldest dated list read. Read: the proclamations as the Ministry of Foreign Affairs' press releases give them, retrieved 2026-09-26, and the Decent Work Act 2015 in the Ministry of Labour's printing. |
| LS | Lesotho | 1996 | The Public Holidays Act 1995 (Act No. 7 of 1995), in its first year, 1996. Read: the Act in CommonLII's copy as the Internet Archive holds it. |
| LY | Libya | 2012 | Law No. 5 of 2012. Read: Law No. 5 of 2012 as the Libyan Legal Society's archive reproduces it, retrieved 2026-09-23. |
| MA | Morocco | 1977 | Décret n° 2-77-169 of 28 February 1977, which the later décrets amend. Not read: décret n° 2-77-169; the amending décrets are known as the French Wikipedia cites them (retrieved 2026-09-22). |
| MG | Madagascar | 2023 | The décret of 4 January 2023, the oldest of the yearly décrets read. Read: the décrets at the Centre National de Législation (cnlegis.gov.mg), retrieved 2026-09-23. |
| MH | Marshall Islands | 1988 | The Public Holidays Act 1988. Read: the Act in the Nitijela's consolidation as the Internet Archive holds it (captured 2025-04-04), retrieved 2026-09-23. |
| ML | Mali | 2005 | Loi n° 05-040 of 22 July 2005. Read: the loi in the Journal officiel (sgg-mali.ml), retrieved 2026-09-23. |
| MR | Mauritania | 1992 | Loi n° 92-018 of 7 December 1992. Read: the loi from the Ministère de la Fonction Publique et du Travail (fonctionpublique.gov.mr), retrieved 2026-09-23. |
| MU | Mauritius | 2017 | The Prime Minister's Office's General Notice No. 814 of 2016 for the public holidays of 2017, the oldest dated list read; the Act (22 of 1968) was read as amended to 2019 |
| MW | Malawi | 2026 | The Public Holidays Act Cap. 18:05 and the 2026 list; the Act's own date was not read. Read: the Act in the NATLEX copy, retrieved 2026-09-22. |
| MZ | Mozambique | 2023 | Lei n.º 13/2023 of 25 August 2023; the table under Lei n.º 23/2007 was read only as a secondary source reports it. Read: Lei n.º 13/2023 at the Tribunal Supremo (ts.gov.mz), retrieved 2026-09-23. |
| NA | Namibia | 1990 | The Public Holidays Act 26 of 1990. Read: the Act in the Legal Assistance Centre's annotated statutes (lac.org.na), retrieved 2026-09-22. |
| NE | Niger | 2023 | The Council of Ministers' text of 2023 restoring 3 August, the oldest dated source read; loi n° 97-20 of 1997 was not read. Not read: loi n° 97-20 (the NATLEX record refused access); the Council of Ministers' texts known as Le Sahel and the Agence Nigérienne de Presse report them. |
| NG | Nigeria | 2004 | The Laws of the Federation of Nigeria 2004 text of the Public Holidays Act, Cap. P40, the edition read. Read: the Act as the Policy and Legal Advocacy Centre reproduces it (placng.org), retrieved 2026-09-26; the Amendment Act 2019 and the Minister's declarations not read. |
| NR | Nauru | 2023 | The Government Gazette of 30 December 2022, the list for 2023, the oldest dated list read. Read: the Government Gazettes at ronlaw.gov.nr, retrieved 2026-09-23. |
| NZ | New Zealand | 2010 | Employment New Zealand's list of 2010 to 2025, the oldest dated list read; the Acts were not read. Not read: the Holidays Act 2003 and the other Acts (legislation.govt.nz refused access); the dates were checked against Employment New Zealand's page, retrieved 2026-09-26. |
| OM | Oman | 2020 | Royal Decree 56/2020, read with Royal Decree 88/2022. Read: Royal Decrees 88/2022 and 15/2025 in Decree's translations (decree.om), retrieved 2026-09-22, and Royal Decree 56/2020 on the same site. |
| PG | Papua New Guinea | 1982 | The 1982 revised edition of the Public Holidays Act 1953 (Chapter 321). Read: the Act in PacLII's consolidation and the 1982 revised edition as the Internet Archive holds them, retrieved 2026-09-23. |
| PS | Palestine | 2025 | The Council of Ministers' 2025 tables. Read: the Council of Ministers' tables at the Palestinian National Information Centre (info.wafa.ps), retrieved 2026-09-23. |
| PW | Palau | 2026 | The Palau National Code Annotated, Supp. 12, whose date was not read, retrieved 2026-09-23. Read: the Code in PacLII's copy as the Internet Archive holds it (captured 2025-12-06), retrieved 2026-09-23. |
| QA | Qatar | 2025 | Emiri Decision No. 57 of 2025; Cabinet Decision No. 6 of 2008, which a decision of 2025 amends, was read only as amended. Read: the Emiri Decision and the Cabinet Decision as Al Meezan publishes them (almeezan.qa), retrieved 2026-09-22. |
| RW | Rwanda | 2017 | Presidential Order n° 54/01 of 24 February 2017. Read: the Order in the Laws.Africa copy on RwandaLII, retrieved 2026-09-23. |
| SA | Saudi Arabia | 2025 | The Labour Law's executive regulation, article 24, as published in April 2025; the regulation's own date was not read. Read: article 24 as the Ministry of Human Resources and Social Development publishes it (hrsd.gov.sa), retrieved 2026-09-26; the ministerial decision issuing it not read. |
| SB | Solomon Islands | 1996 | The 1996 edition of the Public Holidays Act (Cap. 151). Read: the Act in PacLII's consolidation as the Internet Archive holds it (captured 2024-12-22), retrieved 2026-09-23; the Ministry's Public Notices, PDFs, not read. |
| SC | Seychelles | 1991 | The Public Holidays Act, Cap. 190, 1991 edition. Read: the Act and the Amendment Act of 2014 in the Public Service Bureau's scan (psb.gov.sc), retrieved 2026-09-23. |
| SD | Sudan | 2025 | The Council of Ministers' announcements of December 2025 to August 2026. Read: the Council of Ministers' announcements at sudan.gov.sd, retrieved 2026-09-26, and the Labour Act 1997 in an unofficial English translation. |
| SL | Sierra Leone | 2020 | The Government Notices of 2020, the oldest dated list read; the Act of 1960 was read as an undated revised-edition copy |
| SN | Senegal | 1974 | Loi n° 74-52 of 4 November 1974. Read: the loi as the 'Manuel du travailleur' on NATLEX gives it, retrieved 2026-09-22. |
| SO | Somalia | 2025 | Law No. 36 of 24 December 2024, the Labour Code of 2024, from 2025. Read: the Labour Code in the Ministry of Labour and Social Affairs' scan and English version (molsa.gov.so). |
| SS | South Sudan | 2022 | The Labour Act, 2017 (Act No. 64), read with the Ministry of Labour's calendar for 2022, from which the days are carried |
| SY | Syria | 2025 | Decree No. 188 of 2025. Read: Decree No. 188 of 2025 as SANA published it (sana.sy), retrieved 2026-09-23. |
| SZ | Eswatini | 1998 | EswatiniLII's consolidation of the Public Holidays Act 1938 as at 1 December 1998. Read: EswatiniLII's consolidation as the Internet Archive holds it (captured 2025-09-16). |
| TD | Chad | 1997 | Décret n° 97-413 of 30 September 1997. Read: décret n° 97-413 in Légitchad's text as NATLEX holds it, through the Internet Archive, retrieved 2026-09-23; décret n° 273/PR/MFPTDS/2019 not read. |
| TG | Togo | 1987 | Loi n° 87-08 of 9 June 1987. Read: loi n° 87-08 in a copy of jo.gouv.tg's scan downloaded 2026-09-23, the link now being broken. |
| TN | Tunisia | 1961 | Décret 61-144 of 1961 and the decrees after it, as the French Wikipedia chronicles them. Read: décret n° 2021-223 as legislation-securite.tn and jurisitetunisie.com publish it, retrieved 2026-09-22; décret 61-144 known only through the French Wikipedia's chronicle. |
| TO | Tonga | 2020 | The Public Holidays Act, Chapter 8.11, 2020 Revised Edition. Read: the Act at ago.gov.to and the Prime Minister's Office's releases at pmo.gov.to, retrieved 2026-09-23. |
| TR | Türkiye | 1981 | Law 2429 of 17 March 1981, whose text and amendments were read |
| TV | Tuvalu | 2022 | The Public Holidays Act, Cap. 4.50, 2022 Revised Edition. Read: the Acts at tuvalu-legislation.tv, retrieved 2026-09-23. |
| TZ | Tanzania | 2026 | The 2026 list, the only dated list read; the Act's own date was not read. Read: the Act as tanzanialaws.com reproduces it, retrieved 2026-09-22; the gazette notices not read. |
| UG | Uganda | 2026 | The 2026 list of the Consulate in Arusha, the only dated list read; no Act was read |
| VU | Vanuatu | 2006 | The Public Holidays Act [Cap. 114], Consolidated Edition 2006. Read: the Act at moia.gov.vu, retrieved 2026-09-23. |
| WS | Samoa | 2023 | The Public Holidays Act 2008 as revised to 31 December 2023. Read: the Act, revised to 31 December 2023, at ag.gov.ws, retrieved 2026-09-23. |
| YE | Yemen | 2000 | Law No. 2 of 2000. Read: Law No. 2 of 2000 from the Public Prosecution's legislation library (agoyemen.net), retrieved 2026-09-23. |
| ZA | South Africa | 1995 | The Public Holidays Act 36 of 1994, in force 1 January 1995. Read: the Act on gov.za, retrieved 2026-09-26; section 2A not read. |
| ZM | Zambia | 2026 | HONO's 2026 list, the only dated list read; the Act's own date was not read. Read: the Act as zambialaws.com and ZambiaLII summarise it, retrieved 2026-09-22 (secondary); the gazette notices not read. |
| ZW | Zimbabwe | 2026 | General Notice 1361 of 2025, the 2026 list; the Act's own date was not read. Read: General Notice 1361 of 2025 as SmartHR Solutions Zimbabwe reproduces it, retrieved 2026-09-22 (secondary). |

Four tables are read from more than one year. Venezuela's five days of the
Ley de Fiestas Nacionales are read from 1972 and the Labour Law's from 2013;
San Marino's civil holidays (law 152 of 2013) from 2014 and its religious
days from 2025; Bosnia and Herzegovina's three entities each from their own
law, which is why its nationwide set is empty (the Federation 2016, with its
Independence Day and Statehood Day from 1995; Republika Srpska 2007; Brčko
District 2002); and Bolivia's departmental days from the instruments cited
on each, which `docs/systems/bolivia-holidays.md` lists.

The United States, Spain, Switzerland and Canada's provinces carry their first
years in their own tables: the United States' federal days begin at the year
each statute set them (`valid_from`) and its states' days at the year of the
code read for each; Spain's nationwide days from 1990, the first whole year of
article 45 as read; Switzerland's from 2023, the latest first year of a
cantonal law read, and each canton from its own law; Canada's provincial days
from 2026, the year of the texts read. Canada's federal list is read from
1985, the year of the Canada Labour Code as consolidated (the Canada row of
the table below), and a province's text read in 2026 leaves a federal day out
from that year only: before 2026 the day is a gap on the days the federal
rule places.

### Asia

The tables of `asia.rs`, and Bhutan's Thimphu days in `bhutan.rs`, each
wrapped in `read_all` with these first years. A year before
them answers no day and reports a gap; `tests/asia_first_years.rs` checks
the first year and the one before for every row.

| Code | Table | First year | Why |
| --- | --- | --- | --- |
| AF | Afghanistan | 2023 | The first Gregorian year wholly under the Islamic Emirate's calendars; the Republic's list was not read. Not read: the Ministry of Information and Culture's calendar of 1444 AH; the Ministry of Labour's notices read as the press reports them (alemarahdari.af, Pajhwok, Bakhtar), retrieved 2026-09-26. |
| AM | Armenia | 2022 | The Law on Holidays and Remembrance Days as it stands, read through Wikipedia's reproductions; the longer New Year break before 2022 and the amendments of 2002 to 2021 are not given by the sources. Read: as the Armenian Wikipedia and Wikipedia reproduce it, retrieved 2026-09-22 (secondary). |
| AZ | Azerbaijan | 2007 | Labour Code art. 105 as amended on 8 December 2006, whose five-day Novruz is the first form read; National Salvation Day and Armed Forces Day (1998), State Flag Day (2010) and Victory Day (2021) keep their own years. Read: article 105 as the Cabinet of Ministers' page (nk.gov.az) gives it, retrieved 2026-09-22. |
| BD | Bangladesh | 2025 | The Ministry of Public Administration's notification of 21 October 2024; the Hijri days of 2025 and 2026 are the notifications' own dates, and the tabular calendar answers from 2027. Read: the notifications at mopa.gov.bd, retrieved 2026-09-23. |
| BN | Brunei | 2023 | The Prime Minister's Office's circular for 2023, the earliest read. Read: the circulars at jpm.gov.bn, retrieved 2026-09-23. |
| BT | Bhutan | 2025 | The Ministry of Home Affairs' list for 2025, the earliest read; Thimphu's festivals in `BT-15` likewise, with 2021 a gap in any case. Read: the Ministry of Home Affairs' calendars for 2025 and 2026 (moha.gov.bt), read again 2026-09-26. |
| CN | China | 1999 and 2008 | The statutory days from the 1999 revision of the 放假办法, which was not read, so 1998 and before are a gap; the State Council's arrangements from 2008. Read: the 2007, 2013 and 2024 texts of the 放假办法 on gov.cn; the 1949 and 1999 texts not read. |
| GE | Georgia | 2011 | The Labour Code adopted on 17 December 2010 (matsne.gov.ge); which of its 29 amendments of 2012 to 2026 changed art. 30 was not established. Read: article 30 as the Legislative Herald of Georgia (matsne.gov.ge) publishes it in English, retrieved 2026-09-22. |
| HK | Hong Kong | 1998 | The Schedule to Cap. 149 in force from 18 September 1998, the earliest version read; 1997 rests on the Chinese Wikipedia alone. Read: the Ordinance in the current version and those in force from 18 September 1998 and 24 February 2012, as HKLII reproduces them, retrieved 2026-09-26. |
| ID | Indonesia | 2020 | The SKB for 2020, the earliest read. Read: the SKBs for 2020 to 2027, from setda.kalteng.go.id and the Coordinating Ministry for Human Development and Culture (kemenkopmk.go.id), retrieved 2026-09-26. |
| IN | India | 2025 | The DoPT's Office Memorandum of 9 July 2024 for 2025; the 2017 and 2018 memoranda were read only as GConnect reproduces them. The states' days keep their own first years (2019 or 2023). The DoPT's PDFs could not be reached and were read as staffnews.in reproduces them (secondary), retrieved 2026-09-27. |
| KG | Kyrgyzstan | 2005 | The Labour Code of 4 August 2004, from its first whole year. Read: the Labour Code of 2025 on isito.kg, retrieved 2026-09-22, and that of 2004 as continent-online.com publishes it. |
| KH | Cambodia | 2021 | The sub-decree for 2021, the earliest read; 2023's was not read and is a gap. Read: the sub-decrees as pressocm.gov.kh posted them, read 2026-09-29, and those of 2025 to 2027 as the images Commerce Cambodia and DAP News posted and the Ministry of Labour's page, retrieved 2026-09-23. |
| KP | North Korea | 2020 | The wall calendar of 2020 as the Institute for Peace and Unification Studies transcribes it. Read: not the wall calendar, which was not seen, but the Institute's transcription of it (secondary), re-read 2026-09-27. |
| KR | South Korea | 1949 | The 1949 decree (제124호); the rules carry their own dates as well. Read: the decree as in force from 11 May 2026 and its earlier texts, and the 1949 to 1975 texts as Wikisource carries them; the 1976 decree (제8235호) not read. |
| KZ | Kazakhstan | 2002 | The Law of 13 December 2001, from its first whole year. Read: the Law as Параграф (prg.kz) consolidates it, to the law of 11 June 2026, retrieved 2026-09-22. |
| LA | Laos | 2018 | Decree No. 386 of 15 December 2017, from its first whole year. Read: the Decree in the Lao Official Gazette (laoofficialgazette.gov.la). |
| LK | Sri Lanka | 2023 | The Gazette orders for 2023 to 2027 (Nos. 2287/4 to 2493/5) are cited by number; they are PDFs and were not opened. The Act of 1971 and the orders before 2023 were not read. |
| MM | Myanmar | 2026 | The list read, Wikipedia's "Public holidays in Myanmar" as of 2026-09-22, secondary and undated; Deepavali's notices of 2020 to 2025 answer in their own years |
| MN | Mongolia | 2004 | The Law of 18 December 2003, from its first whole year; the five days of Naadam and Tsagaan Sar from 2014 (the amendments of 2014 and 2013 are noted in the consolidated text without their content), 10 July from the law of 28 June 2022. Read: the Law with the notes of its amending laws at legalinfo.mn, retrieved 2026-09-23 and 2026-09-26. |
| MO | Macau | 2001 | Executive Order 60/2000 (Boletim Oficial No. 40/2000, October 2000), from its first whole year; the Bulletin's own page was not reachable. Known through gov.mo's page on Executive Order 60/2000, retrieved 2026-09-22; the Boletim Oficial's own page was not reachable. |
| MV | Maldives | 2016 | The Monetary Authority's list for 2016, the earliest read. Read: the Employment Act in the Labour Relations Authority's unofficial English translation (lra.gov.mv) and the Monetary Authority's lists, retrieved 2026-09-23. |
| MY | Malaysia | 2020 | The Prime Minister's Department's list for 2020, the earliest read; the weekend law of the states keeps its own years. Read: the Prime Minister's Department's schedules for 2020 to 2027 through web.archive.org, retrieved 2026-09-26. |
| NP | Nepal | 2023 | The Home Ministry's notice for 2080 BS, which began on 14 April 2023; the days of January to April fall in 2024, which is their first year. Read: the Home Ministry's notices of public holidays in the Nepal Rajpatra for 2080 to 2083 BS, retrieved 2026-09-23 and read 2026-09-29. |
| PH | Philippines | 2012 | Proclamation No. 295 for 2012, the first of the annual proclamations read; the Administrative Code as amended by Republic Act 9849 (2009) was not read for 2010 and 2011. Read: the annual proclamations for 2012 to 2027 and Republic Act 10966 as LawPhil reproduces them, retrieved 2026-09-28; the Administrative Code as amended not read, known as the proclamations cite it. |
| PK | Pakistan | 2026 | The Cabinet Division's list for 2026 as Business Recorder reports it; Iqbal Day's years of 2014 and 2022 to 2025 are gaps with it. Not read: the Cabinet Division's list, a scanned image; known through Business Recorder, retrieved 2026-09-26 (secondary). |
| SG | Singapore | 2020 | The Ministry of Manpower's public-holiday release for 2020, the earliest read; the releases of 2017 and 2018 (Deepavali) are pinned as the rule's days only. Read: the Ministry of Manpower's press releases for 2020 to 2027, retrieved 2026-09-26. |
| TH | Thailand | 1992 | The Bank of Thailand's list for 1992, the earliest read. Read: the Bank of Thailand's lists as the Internet Archive keeps them, and its notifications for 2023 to 2026. |
| TJ | Tajikistan | 2012 | The Law on Holidays of 2 August 2011, from its first whole year. Read: the Law at the National Centre of Legislation (ncz.tj), retrieved 2026-09-22. |
| TL | Timor-Leste | 2006 | Law No. 10/2005 of 10 August 2005, from its first whole year. Read: the leis in the Jornal da República (mj.gov.tl/jornal) and FAOLEX. |
| TM | Turkmenistan | 2010 | The Labour Code of 18 April 2009, from its first whole year. Read: the Labour Code at the Ombudsman's copy (ombudsman.gov.tm), retrieved 2026-09-22. |
| TW | Taiwan | 2012 | The 紀念日及節日實施辦法 as of 25 September 2012 and the DGPA calendars of 2012 to 2014; the swaps begin in 2017. Read: the 條例 and the 辦法's versions of 2012 and 2014 on the 全國法規資料庫, and the DGPA's pages, retrieved 2026-09-26. |
| UZ | Uzbekistan | 2024 | The Labour Code of 28 October 2022, in force from 30 April 2023, from its first whole year; 1997 to 1999 are the rules' own dates, pinned as the rules'. Read: article 208 as lex.uz publishes it, retrieved 2026-09-22. |
| VN | Vietnam | 2021 | The Labour Code 45/2019, in force from 1 January 2021; the Hùng Kings' Festival, a holiday from 2007 under the Code of 2012 that was not read, is a gap from 2007 to 2020. Read: the Labour Code 45/2019 and the notices on 23 September 2026, mostly as the government portals and law databases reproduce them (secondary). |

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

### The United States, Japan, Canada and Bosnia and Herzegovina

Four tables say what is and is not known for the years before their sources:

| Code | Table | What is carried | Why |
| --- | --- | --- | --- |
| US | United States | Thanksgiving Day is a gap from 1870 to 1941 and the fourth Thursday of November from 1942; Armistice Day is the 11th of November from 1938 to 1953 | The Act of 28 June 1870 made "any day appointed or recommended by the President" for thanksgiving a holiday in the District of Columbia, and the Congressional Research Service's R41990 [crs-r41990-federal-holidays] gives the other dates; the Presidents' proclamations of 1870 to 1941, which named the day each year, were not read. The days before 1870 are absent: no federal holiday existed. Read: R41990 at everycrsreport.com, retrieved 2026-10-04; the Act's own text is not read. The in-lieu-of rule is carried from 1952 for a Sunday holiday (Executive Order 10358 of 9 June 1952, closing the offices on the Monday [eo-10358-presidency]) and from 1960 for a Saturday one (Pub. L. 86-362 of 22 September 1959, after which no holiday fell on a Saturday in 1959; the Act itself is a PDF and was not read, its Saturday content resting on a search summary and the Legal Information Institute's source note to § 6103(b)); Executive Order 9636 of 1945, which 10358 superseded, is not read, so a Sunday holiday before 1952 keeps its Sunday. |
| JP | Japan | Every year to 1948 is a gap, for the days of the 休日ニ関スル件 of 1927 and the regime before it | The 国民の祝日に関する法律 begins on 20 July 1948 and repealed the Ordinance; its days were not read. Read: the Act's text as e-Gov's API serves it, retrieved 2026-09-29; the 1927 Ordinance is not read. |
| CA | Canada | The ten federal general holidays are read from 1985 | The Canada Labour Code as R.S.C. 1985, c. L-2 consolidates it, section 166; the Code's earlier texts were not read, and the years the Wikipedia articles give for Victoria Day (1952), Remembrance Day (1931) and Thanksgiving (1957) are not establishments of the days. Read: Justice Laws' section 166, retrieved 2026-10-04; the Wikipedia articles are secondary and not checked against the statutes. |
| BA | Bosnia and Herzegovina | A request with no entity is a gap in every year; each entity's own list is read from its own law | No state-level law of public holidays was found among the regulations the state lists on Paragraf Lex; the entities' laws set them. Read: the regulation list on Paragraf Lex, retrieved 2026-10-04, which is weak evidence of absence. |

### The exchanges

Each exchange's table is read from the first year of the lists its `sources`
names, `read_from_year` in `exchanges.rs`: the NYSE's and Nasdaq's (which
share the NYSE's rules) from 2026, Euronext's from 2021, Borsa İstanbul's from 2019, the Korea Exchange's from
2009. A closure limited to the one year it happened in (the NYSE's closure
after the September 11 attacks, Hurricane Sandy, President Bush's funeral) is
read in that year, and a list that begins before the table's first year
keeps its own. An exchange that closes on its country's days includes the
country's table from the same year (`Include::read_from`), and before it the
engine reports `unread-included-holidays`. The trading week of each exchange is in
[national-weekends.md](national-weekends.md).

### The United Nations' days and weeks

Each of the 238 days and 11 weeks has a first year, `valid_from`, and is absent
before it. Where the sources do not read the years from that year on, the years
before `read_from` are a gap the engine reports, not an absence, and the cases
below say which applies. Where a page read gives the first observance, that
year; where it gives only the date the resolution was
adopted, the year of the first occurrence of the day's date on or after it
(derived, and said so in the table below); where only the year of the
proclamation is known, the day is established that year and read from the
next (`read_from`), the proclamation's own year being a gap; and where only
the session of the General Assembly that adopted the resolution is known,
from the session's year and read two years after. A day with no resolution
and no date read (Zero Discrimination Day's agency, the weeks the agencies
set) is read from 2026. A day an earlier body observed on the same date
before the United Nations proclaimed it (Wetlands, Family Remittances, Elder
Abuse, Kiswahili, the Day against Female Genital Mutilation) begins with the
proclamation: the precursor's year is not the day's. The years were read
from the United Nations' observance pages and, where those give no year,
Wikipedia's articles (secondary); the resolutions' own texts, PDFs, were not
read, and the number of rows with only a proclamation year is higher than
with a first observance.

The 28 days whose first year was once the session's year are dated from
pages read on 2026-10-05 [un-days-first-observances]; the adoption date is
the meetings coverage's or a news report's, and the first observance is
derived unless a page read states it:

| Day | Resolution, adopted | First year | Basis |
| --- | --- | --- | --- |
| Clean Energy, 26 Jan | A/RES/77/327, 25 Aug 2023 | 2024 | derived; the observance of 26 Jan 2024 is reported by SEforALL |
| Wetlands, 2 Feb | A/RES/75/317, 30 Aug 2021 | 2022 | UNEP's page: the first year observed as a UN day |
| Arabian Leopard, 10 Feb | A/RES/77/295, 12 Jun 2023 | 2024 | derived; a news report calls 10 Feb 2024 the first |
| Violent Extremism, 12 Feb | A/RES/77/243, 20 Dec 2022 | 2023 | UNOCT and ReliefWeb call 12 Feb 2023 the first |
| Tourism Resilience, 17 Feb | A/RES/77/269, 6 Feb 2023 | 2023 | derived |
| Disarmament Awareness, 5 Mar | A/RES/77/51, 7 Dec 2022 | 2023 | the Secretary-General's message of 5 Mar 2023: "first-ever" |
| Glaciers, 21 Mar | A/RES/77/158, 14 Dec 2022 | 2025 | the resolution names 2025; launched 21 Mar 2025 |
| Detained and Missing Staff, 25 Mar | none; the Convention is A/RES/49/59 | 1986 | the United Nations memorial page: observed since 1986 |
| Rwanda Reflection, 7 Apr | A/RES/58/234, 23 Dec 2003 | 2004 | the United Nations' page: 7 Apr 2004 the first |
| Wellness, 15 Apr | A/RES/80/249, 10 Mar 2026 | 2026 | reported observed on 15 Apr 2026 |
| Mother Earth, 22 Apr | A/RES/63/278, 22 Apr 2009 | 2010; 2009 a gap | adopted on the day itself |
| Tuna, 2 May | A/RES/71/124, 7 Dec 2016 | 2017 | derived |
| Plant Health, 12 May | A/RES/76/256, 29 Mar 2022 | 2022 | FAO: the inaugural celebration, 12 May 2022 |
| Biological Diversity, 22 May | A/RES/55/201, 20 Dec 2000 | 2001 | the day moved from 29 Dec; the President's message of 22 May 2001 |
| Food Safety, 7 Jun | A/RES/73/250, 20 Dec 2018 | 2019 | the day's background page: "As of 2019" |
| Play, 11 Jun | A/RES/78/268, 25 Mar 2024 | 2024 | the inaugural day, 11 Jun 2024 |
| Elder Abuse, 15 Jun | A/RES/66/127, 19 Dec 2011 | 2012 | derived; INPEA's day dates from 2006 |
| Family Remittances, 16 Jun | A/RES/72/281, 12 Jun 2018 | 2018 | the daily briefing of 14 Jun 2018; IFAD's day dates from 2015 |
| Sustainable Gastronomy, 18 Jun | A/RES/71/246, 21 Dec 2016 | 2017 | derived |
| Solstice, 21 Jun | A/RES/73/300, 20 Jun 2019 | 2019 | derived |
| Women in Diplomacy, 24 Jun | A/RES/76/269, 20 Jun 2022 | 2022 | derived |
| Deafblindness, 27 Jun | A/RES/79/294, 16 Jun 2025 | 2025 | the first day marked 27 Jun 2025 |
| MSME, 27 Jun | A/RES/71/279, 6 Apr 2017 | 2017 | the first day, 27 Jun 2017 (ICSB) |
| Rural Development, 6 Jul | A/RES/78/326, 6 Sep 2024 | 2025 | the day's page counts 2026 as the second |
| Kiswahili, 7 Jul | A/RES/78/312, 1 Jul 2024 | 2024 | derived; UNESCO's day dates from 2022 |
| Sand and Dust Storms, 12 Jul | A/RES/77/294, 8 Jun 2023 | 2023 | UNCCD: the first observance, 12 Jul 2023 |
| Hope, 12 Jul | A/RES/79/270, 4 Mar 2025 | 2025 | derived |
| Migrants, 18 Dec | A/RES/55/93, 4 Dec 2000 | 2000 | derived |

The two days of the 80th session adopted on 31 August 2026, Greening the
Planet (A/RES/80/300, 22 April) and Safe, Secure and Trustworthy Artificial
Intelligence (A/RES/80/302, 31 August), begin in 2027, a news report saying
so for the second; Women Searchers of Missing Persons (A/RES/80/301, 19
December) and the Elimination of Child, Early and Forced Marriage
(A/RES/80/309, adopted 4 September 2026, 27 November) are a gap in 2026 and
read from 2027, a news report giving 2027 for the second where the date
alone would make 2026. These adoption dates rest on news pages and search
summaries: the meetings coverage on press.un.org did not load.

### The traditions

A tradition's table is read from the first year of the calendar it dates its
days in, where no source dates the tradition itself:

| Code | First year | Why |
| --- | --- | --- |
| `christian-western` | 1583 | The Gregorian calendar's first whole year; its movable days are offsets from the Gregorian computus |
| `christian-orthodox`, `christian-armenian-jerusalem` | 326 | The Julian computus' first year |
| `christian-orthodox-revised-julian`, `name-days-greek-movable` | 1925 | The Church of Greece took the Revised Julian calendar on 10/23 March 1924 [wikipedia-revised-julian-calendar]; 1925 is the first whole year |
| `christian-armenian` | 1583 | Gregorian computus |
| `ethiopian-orthodox` | 401 | The Ethiopic calendar's use begins in 400 (`hc-calendars-solar`'s usage record) |
| `coptic-orthodox` | 326 | The Julian computus; the Coptic calendar's use begins in 284 |
| `jewish` | 359 | The Hebrew calendar's fixed rules, in use from 358 |
| `buddhist-east-asian` (the three Gregorian days) | 1873 | The Meiji reform of the calendar |
| `shinto` | 1873 | The same |
| `kyuchu-saishi` | 2020 | The schedule of the Reiwa era, the first whole year of it |
| `plough-days` | 1753 | The accounts are of England after 1752 |
| `unlucky-fridays`, `sacred-wednesdays`, `balinese-pawukon-days` | 2000 | The first year of the range the book's code was run for as the check, and the test's |
| `yazidi` | 1901 | Kreyenbroek's "in this century", the twentieth |
| `qumran-festivals` | −133 | 134 BCE, the earliest the Hasmonean settlement at Qumran is dated ([wikipedia-qumran], secondary) |
| `name-days-bulgarian-movable` | 2010 | The first year of the table of dates Bulgarian Wikipedia gives |

Read: the pages named in each row and the calendars' own usage records; no tradition's authority was read for these years. The other tables answer only where their calendars do. `tests/first_years_tables.rs` holds
every table to its row: nothing is answered before the first year, and the
year the row gives answers.

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
days that began before their table's first year. The country tests assert
years at or after a table's first year, laid out the same way where the weekday
matters (2033 and 2039 for 2022, 2030 for 2024).

The first years are judgements about what the sources support, and they are
conservative: a table's days of earlier years may well be what the table
would have said, and the table says only that they were not read. Where
a source was a secondary list (Wikipedia for Chile, Latvia, Russia, Romania)
the first year is the earliest year it dates.

## Sources

Each row of the tables above ends with what was read for it. "Read" means
the instrument's own text, at the site and on the date the row names, which is
the table's `sources` string in `crates/hc-holiday/src/countries`; "as X
reproduces it", "secondary" and "known through" mean a reproduction, a summary
or a report of the instrument, and "not read" means it is cited by a source
that was read, or its site refused access. A row that says that whether the
text was read "is not recorded" is one whose `sources` string and comment
name the instrument without saying how it was reached. Each table's
`sources_checked` date is the last day its sources were checked. Read on
2026-10-03, beside what the rows name: Wikipedia's "Public holidays in Iran" and
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

`crates/hc-holiday/src/exchanges.rs` has `read_from_year`, which wraps each
exchange's rules the way `read_all` does the countries' and reads a closure
limited to one year, and a list that begins earlier, from their own years;
`Include::read_from` in `rule.rs` reads an exchange's inclusion of its
country's days; `international.rs` gives each day and week its own years;
`traditions.rs` wraps each tradition's rules in `read_all` with the first year
of the table above. `tests/first_years_tables.rs` holds the exchanges, the
international days and the traditions to their first years,
`tests/gap_windows.rs` the days a gap leaves open, and
`tests/national_weekends.rs` every table's first weekend date.
