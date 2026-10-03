# Where each national table of the Americas and Europe begins

Each national table of `hc-holiday` answers from one year and reports every
year before it as a gap. This document says which year that is for the
tables of the Americas and Europe, and why. The tables of Africa, the Middle
East, Asia and Oceania are written up where they are.

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

`tests/first_years.rs` checks for every row above that the first year has
days, that the years before it (one, ten and a hundred before, and 1500)
have none, and that the year just before is a gap. It pins Bolivia's 2026
from the decree, the 2017 and Güemes gaps of Argentina, Saint Vincent's 2020,
Venezuela's 1972, San Marino's two first years, and the cantons' names.

The first years are judgements about what the sources support, and they are
conservative: a table's days of earlier years may well be what the table
would have said, and the table says only that they were not read. Where
a source was a secondary list (Wikipedia for Chile, Latvia, Russia, Romania)
the first year is the earliest year it dates.

## Sources

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

`crates/hc-holiday/src/countries/mod.rs` has `read_all`; each table's
`rules` static is wrapped in it in `americas.rs`, `europe.rs`, `andorra.rs`,
`bolivia.rs` and `mexico.rs`. The tests are `tests/first_years.rs`, and the
dated years each table encodes are pinned in `tests/countries.rs`.
