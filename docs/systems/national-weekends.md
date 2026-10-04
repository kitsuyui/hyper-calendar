# National weekends: the days each country and exchange keeps, and the year each is first read

A holiday table says which days are holidays. Business-day arithmetic also
needs the weekend, and the weekend is a law that changes. This document says,
for every national table and every exchange of `hc-holiday`, which days are
the weekend, from which date the sources read state them, and what is a gap
before.

## What it is

The weekend is the days of the week on which the offices of the country, or
the exchange, do not work: a statute's or an authority's, for the public
service or for every employee, and often not the same days as the private
sector's statutory weekly rest, which many labour codes fix on Sunday alone
and leave the second day to the employer. The library carries the public
service's, the days the table's holidays are counted against, and says so in
the policy's note; the Saturday that a shop or a school keeps open is not a
weekend day.

A table used to give Saturday and Sunday, or Friday and Saturday, with no
first year, and so in every year back to the start of the calendar. No source
reads that far: Korea's five-day week came with 2004, Japan's civil service
took its Saturdays in 1992, the United States' executive agencies had a
Monday-to-Friday workweek by the law of 1966. A weekend before the sources
read is a gap, as a holiday is
([ADR 0013](../adr/0013-a-year-the-sources-do-not-reach-is-a-gap.md),
[ADR 0015](../adr/0015-a-region-may-keep-a-weekend-of-its-own.md)).

## How it works

A table's `weekend` is a list of `WeekendPolicy` values, each the days, the
first and the last day it is in force and the regions it is the law of.
Every national table and every exchange begins with a policy that has no
days, built with `WeekendPolicy::unread()`, which says that the law of those
days was not read, and goes on with the policies of the regimes the sources
give, built with `WeekendPolicy::of(days).from(year)` or `.from_day(year,
month, day)` and ended by the next regime's first day. They are all in
`hc_holiday::countries::weekends`, the one place the data is.

`RuleSet::weekend_in(region, day)` takes the policies in force on the day
whose regions the asked region is or lies within, and the nearest wins. A day
that no policy of a table that states a weekend covers is unread, and
`weekend_in` answers `None` for it, where it used to fall back to Saturday and
Sunday; only a table that states no weekend at all keeps them, as a
caller's own table may, and as the tables of the traditions and of the United
Nations' days do, whose days belong to no working week. The engine writes one
gap, `unread-weekend`, for each year in which a day is unread. A calendar's
`is_complete` is about the holidays and does not count it;
`HolidayCalendar::holiday_gaps` is `gaps` without it, and
`HolidayCalendar::weekend_is_read` says whether a day's law was read.

Worked example, Japan. The Public Service's complete two-day weekend began on
1 May 1992, as the Japanese Wikipedia's article on the 週休二日制 has it, and
the 一般職の職員の勤務時間、休暇等に関する法律 gives the national civil
servants Saturday and Sunday as their rest days (article 6(1)). So:

- Thursday 30 April 1992 is a day whose weekend law was not read:
  `weekend_in(None, ...)` is `None`, `is_weekend` says no, `weekend_is_read`
  says no, and `add_business_days` for a walk that reaches it is `None`
  (`HC_ERR_OUT_OF_RANGE` at the boundary).
- Saturday 2 May 1992 is a weekend day, and so is every Saturday and Sunday
  after it.
- A walk from Friday 1 January 1999 in South Korea's table is refused: its
  weekend is read from 2026, and the table does not say what Korea's was
  before.

## What is carried

The first column is the first date the policy is in force: the first day of
the year where a source gives only the year, and the first of the month where
it gives only the month, unless the note of the policy in
`countries/weekends.rs` says otherwise. *Basis* is `dated` where a source
read dates a regime and `read in 2026` where none does and the weekend is the
one the sources read give now, read in the year they were checked: the
years before 2026 are a gap for those 150-odd tables, and a later reading of
an instrument or a news report lowers each first year. Each row's note, with its sources, is the doc comment
of its static in `countries/weekends.rs`.

### The national tables and the exchanges

| Code | Country | First read | Weekend | Basis |
| --- | --- | --- | --- | --- |
| AD | Andorra | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| AE | United Arab Emirates | 1999 | Thursday and Friday from 1999, Friday and Saturday from 1 September 2006, Saturday and Sunday from 2022; Sharjah Friday to Sunday from 2022 | dated |
| AF | Afghanistan | 2026 | Friday from 2026 | read in 2026 |
| AG | Antigua and Barbuda | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| AL | Albania | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| AM | Armenia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| AO | Angola | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| AR | Argentina | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| AT | Austria | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| AU | Australia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| AZ | Azerbaijan | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BA | Bosnia and Herzegovina | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BB | Barbados | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BD | Bangladesh | 1 April 1982 | Friday from 1 April 1982; Friday and Saturday from 9 September 2005 | dated |
| BE | Belgium | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BF | Burkina Faso | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BG | Bulgaria | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BH | Bahrain | 1 March 1990 | Thursday and Friday from 1 March 1990; Friday and Saturday from 1 September 2006 | dated |
| BI | Burundi | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BJ | Benin | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BN | Brunei | 2019 | Friday and Sunday from 2019 | dated |
| BO | Bolivia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BR | Brazil | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BS | Bahamas | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BT | Bhutan | 2016 | Saturday and Sunday from 2016 | dated |
| BW | Botswana | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| BY | Belarus | 2000 | Saturday and Sunday from 2000 | dated |
| BZ | Belize | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CA | Canada | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CD | Democratic Republic of the Congo | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CG | Republic of the Congo | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CH | Switzerland | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CI | Côte d'Ivoire | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CL | Chile | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CM | Cameroon | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CN | China | 25 March 1995 | Sunday from 25 March 1995; Saturday and Sunday from 1 May 1995 | dated |
| CO | Colombia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CR | Costa Rica | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CU | Cuba | 2014 | Sunday from 2014 | dated |
| CV | Cabo Verde | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CY | Cyprus | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| CZ | Czechia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| DE | Germany | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| DJ | Djibouti | 2026 | Friday from 2026 | read in 2026 |
| DK | Denmark | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| DM | Dominica | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| DO | Dominican Republic | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| DZ | Algeria | 1975 | Saturday and Sunday from 1975; Thursday and Friday from 1976; Friday and Saturday from 14 August 2009 | dated |
| EC | Ecuador | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| EE | Estonia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| EG | Egypt | 2006 | Friday from 2006; Friday and Saturday from 21 January 2006 | dated |
| ES | Spain | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| ET | Ethiopia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| FI | Finland | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| FJ | Fiji | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| FM | Micronesia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| FR | France | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| GA | Gabon | 2022 | Sunday from 2022 | dated |
| GB | United Kingdom | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| GD | Grenada | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| GE | Georgia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| GH | Ghana | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| GM | The Gambia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| GN | Guinea | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| GQ | Equatorial Guinea | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| GR | Greece | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| GT | Guatemala | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| GW | Guinea-Bissau | 2026 | Sunday from 2026 | read in 2026 |
| GY | Guyana | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| HK | Hong Kong | 3 July 2006 | Saturday and Sunday from 3 July 2006 | dated |
| HN | Honduras | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| HR | Croatia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| HT | Haiti | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| HU | Hungary | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| ID | Indonesia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| IE | Ireland | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| IL | Israel | 1952 | Saturday from 1952 | dated |
| IN | India | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| IQ | Iraq | 1 March 2005 | Friday and Saturday from 1 March 2005 | dated |
| IR | Iran | 2026 | Friday from 2026 | read in 2026 |
| IS | Iceland | 1983 | Saturday and Sunday from 1983 | dated |
| IT | Italy | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| JM | Jamaica | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| JO | Jordan | 1999 | Thursday and Friday from 1999; Friday and Saturday from 8 January 2000 | dated |
| JP | Japan | 1 May 1992 | Saturday and Sunday from 1 May 1992 | dated |
| KE | Kenya | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| KG | Kyrgyzstan | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| KH | Cambodia | 2026 | Sunday from 2026 | read in 2026 |
| KI | Kiribati | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| KM | Comoros | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| KN | Saint Kitts and Nevis | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| KP | North Korea | 2026 | Sunday from 2026 | read in 2026 |
| KR | South Korea | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| KW | Kuwait | 2007 | Thursday and Friday from 2007; Friday and Saturday from 1 September 2007 | dated |
| KZ | Kazakhstan | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| LA | Laos | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| LB | Lebanon | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| LC | Saint Lucia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| LI | Liechtenstein | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| LK | Sri Lanka | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| LR | Liberia | 2016 | Sunday from 2016 | dated |
| LS | Lesotho | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| LT | Lithuania | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| LU | Luxembourg | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| LV | Latvia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| LY | Libya | 2006 | Friday and Saturday from 2006 | dated |
| MA | Morocco | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MC | Monaco | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MD | Moldova | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| ME | Montenegro | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MG | Madagascar | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MH | Marshall Islands | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MK | North Macedonia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| ML | Mali | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MM | Myanmar | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MN | Mongolia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MO | Macau | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MR | Mauritania | 2007 | Friday and Saturday from 2007; Saturday and Sunday from 1 October 2014 | dated |
| MT | Malta | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MU | Mauritius | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MV | Maldives | 2013 | Friday and Saturday from 2013 | dated |
| MW | Malawi | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MX | Mexico | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| MY | Malaysia | 25 November 2013 | Saturday and Sunday from 25 November 2013; the states Kedah, Kelantan and Terengganu Friday and Saturday from the same day; Johor Saturday and Sunday from 1995, Friday and Saturday 2014 to 2024; a gap before | dated |
| MZ | Mozambique | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| NA | Namibia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| NE | Niger | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| NG | Nigeria | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| NI | Nicaragua | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| NL | Netherlands | 1 April 1965 | Saturday and Sunday from 1 April 1965 | dated |
| NO | Norway | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| NP | Nepal | 2026 | Saturday from 2026, Saturday and Sunday from 6 April 2026 | read in 2026 |
| NR | Nauru | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| NZ | New Zealand | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| OM | Oman | 2013 | Thursday and Friday from 2013; Friday and Saturday from 1 May 2013 | dated |
| PA | Panama | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| PE | Peru | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| PG | Papua New Guinea | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| PH | Philippines | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| PK | Pakistan | 9 June 2022 | Saturday and Sunday from 9 June 2022 | dated |
| PL | Poland | 1951 | Sunday from 1951; not read from 1973; Saturday and Sunday from 2001 | dated |
| PS | Palestine | 2026 | Friday and Saturday from 2026 | read in 2026 |
| PT | Portugal | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| PW | Palau | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| PY | Paraguay | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| QA | Qatar | 2003 | Thursday and Friday from 2003; Friday and Saturday from 1 August 2003 | dated |
| RO | Romania | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| RS | Serbia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| RU | Russia | 1940 | Sunday from 1940; not read from 1967; Saturday and Sunday from 1968 | dated |
| RW | Rwanda | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SA | Saudi Arabia | 2013 | Thursday and Friday from 2013; Friday and Saturday from 29 June 2013 | dated |
| SB | Solomon Islands | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SC | Seychelles | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SD | Sudan | 21 January 1998 | Friday from 21 January 1998; Friday and Saturday from 26 January 2008 | dated |
| SE | Sweden | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SG | Singapore | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SI | Slovenia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SK | Slovakia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SL | Sierra Leone | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SM | San Marino | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SN | Senegal | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SO | Somalia | 2025 | Friday from 2025 | dated |
| SR | Suriname | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SS | South Sudan | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SV | El Salvador | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| SY | Syria | 1 February 2004 | Friday and Saturday from 1 February 2004 | dated |
| SZ | Eswatini | 2026 | Sunday from 2026 | read in 2026 |
| TD | Chad | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| TG | Togo | 2022 | Sunday from 2022 | dated |
| TH | Thailand | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| TJ | Tajikistan | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| TL | Timor-Leste | 2013 | Sunday from 2013 | dated |
| TM | Turkmenistan | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| TN | Tunisia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| TO | Tonga | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| TR | Türkiye | 2 January 1924 | Friday from 2 January 1924; Sunday from 2 June 1935; Saturday and Sunday from 1 July 1974 | dated |
| TT | Trinidad and Tobago | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| TV | Tuvalu | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| TW | Taiwan | 1998 | not read from 1998; Saturday and Sunday from 2001 | dated |
| TZ | Tanzania | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| UA | Ukraine | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| UG | Uganda | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| US | United States | 6 September 1966 | Saturday and Sunday from 6 September 1966 | dated |
| UY | Uruguay | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| UZ | Uzbekistan | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| VA | Vatican City | 2011 | Sunday from 2011 | dated |
| VC | Saint Vincent and the Grenadines | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| VE | Venezuela | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| VN | Vietnam | 3 May 2017 | Saturday and Sunday from 3 May 2017 | dated |
| VU | Vanuatu | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| WS | Samoa | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| YE | Yemen | 2013 | Thursday and Friday from 2013; Friday and Saturday from 15 August 2013 | dated |
| ZA | South Africa | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| ZM | Zambia | 2026 | Saturday and Sunday from 2026 | read in 2026 |
| ZW | Zimbabwe | 2026 | Saturday and Sunday from 2026 | read in 2026 |


| Code | Exchange | First read | Trading week off |
| --- | --- | --- | --- |
| BVMF | B3 (São Paulo) | 2021 | Saturday and Sunday from 2021 |
| MISX | Moscow Exchange | 2023 | Saturday and Sunday from 2023 |
| XAMS | Euronext Amsterdam | 2021 | Saturday and Sunday from 2021 |
| XASX | Australian Securities Exchange | 2026 | Saturday and Sunday from 2026 |
| XBKK | Stock Exchange of Thailand | 2022 | Saturday and Sunday from 2022 |
| XBOM | BSE (Bombay Stock Exchange) | 2020 | Saturday and Sunday from 2020 |
| XBRU | Euronext Brussels | 2021 | Saturday and Sunday from 2021 |
| XCSE | Nasdaq Copenhagen | 2025 | Saturday and Sunday from 2025 |
| XDUB | Euronext Dublin | 2021 | Saturday and Sunday from 2021 |
| XETR | Frankfurt Stock Exchange (Xetra) | 2026 | Saturday and Sunday from 2026 |
| XHEL | Nasdaq Helsinki | 2025 | Saturday and Sunday from 2025 |
| XHKG | Stock Exchange of Hong Kong (HKEX) | 2026 | Saturday and Sunday from 2026 |
| XICE | Nasdaq Iceland | 2025 | Saturday and Sunday from 2025 |
| XIDX | Indonesia Stock Exchange | 2020 | Saturday and Sunday from 2020 |
| XIST | Borsa İstanbul | 2019 | Saturday and Sunday from 2019 |
| XJPX | Tokyo Stock Exchange (JPX) | 2026 | Saturday and Sunday from 2026 |
| XJSE | Johannesburg Stock Exchange | 2024 | Saturday and Sunday from 2024 |
| XKLS | Bursa Malaysia | 2020 | Saturday and Sunday from 2020 |
| XKRX | Korea Exchange | 2009 | Saturday and Sunday from 2009 |
| XLIS | Euronext Lisbon | 2021 | Saturday and Sunday from 2021 |
| XLON | London Stock Exchange | 2026 | Saturday and Sunday from 2026 |
| XMAD | Bolsa de Madrid (BME) | 2023 | Saturday and Sunday from 2023 |
| XMEX | Bolsa Mexicana de Valores | 2019 | Saturday and Sunday from 2019 |
| XMIL | Euronext Milan (Borsa Italiana) | 2021 | Saturday and Sunday from 2021 |
| XNAS | Nasdaq | 2026 | Saturday and Sunday from 2026 |
| XNSE | National Stock Exchange of India | 2020 | Saturday and Sunday from 2020 |
| XNYS | New York Stock Exchange | 1953 | Saturday and Sunday from 1953 |
| XNZE | NZX | 2021 | Saturday and Sunday from 2021 |
| XOSL | Euronext Oslo (Oslo Børs) | 2021 | Saturday and Sunday from 2021 |
| XPAR | Euronext Paris | 2021 | Saturday and Sunday from 2021 |
| XPHS | Philippine Stock Exchange | 2020 | Saturday and Sunday from 2020 |
| XSAU | Saudi Exchange (Tadawul) | 25 February 2006 | Thursday and Friday from 25 February 2006; Friday and Saturday from 29 June 2013 |
| XSES | Singapore Exchange | 2020 | Saturday and Sunday from 2020 |
| XSHE | Shenzhen Stock Exchange | 2015 | Saturday and Sunday from 2015 |
| XSHG | Shanghai Stock Exchange | 2014 | Saturday and Sunday from 2014 |
| XSTO | Nasdaq Stockholm | 2025 | Saturday and Sunday from 2025 |
| XSWX | SIX Swiss Exchange | 2026 | Saturday and Sunday from 2026 |
| XTAE | Tel Aviv Stock Exchange | 2024 | Friday and Saturday from 2024; Saturday and Sunday from 4 January 2026 |
| XTAI | Taiwan Stock Exchange | 2023 | Saturday and Sunday from 2023 |
| XTSE | Toronto Stock Exchange | 2025 | Saturday and Sunday from 2025 |
| XWAR | Warsaw Stock Exchange (GPW) | 2019 | Saturday and Sunday from 2019 |
| XWBO | Wiener Börse | 2019 | Saturday and Sunday from 2019 |

The tradition tables and the United Nations' days state no weekend and keep
Saturday and Sunday as a default; no source reads a weekend for them.

Malaysia and the United Arab Emirates also have regions with a weekend of
their own, [regional-weekends.md](regional-weekends.md).

## Accuracy

- **A date given only as a year** begins the regime on 1 January of it, and a
  month on its first day. The report of a change often says nothing more
  (Jordan's January 2000, Libya's 2006, Egypt's February 2006), so the weekend
  of the first days of such a year is as late or as early as the report.
- **The first regime of a change is read from the report that states it.**
  A news report of a change says what the weekend was before; it does not say
  how long, so the regime is carried from the year of the report and not
  before (Qatar from 2003, Saudi Arabia from 2013, Kuwait from 2007).
- **A phased or partial regime** is carried from its first date and the
  note says so: Hong Kong's civil service five-day week began on 3 July 2006
  and reached about 70 per cent of civil servants by 2012; China's departments
  had until 1 January 1996; Hungary's, Poland's and Taiwan's regimes with free
  Saturdays in turn are not weekends and their years are not read.
- **The weekend of the public service is not the private sector's.** Where a
  source read gives only the statutory weekly rest of private employment, and
  that is Sunday, it is not the weekend carried; the table's own weekend is
  carried from 2026 and the disagreement is in the note (Gabon, Equatorial
  Guinea, Honduras, Colombia, Bolivia, Croatia, Ireland and others).
- **Sources are secondary for most of the tables read in 2026.** The page
  read for them is the live English Wikipedia article "Workweek and weekend",
  retrieved 2026-10-04, which does not date a regime. The search budget of the
  session that gathered the sources ran out before most history could be
  looked for, and the first years of those tables are the year of the reading.
  A table whose weekend a source read contradicts is listed with its reading
  in the note.

## Sources

Each regime's source is named in its note in `countries/weekends.rs`: news
reports of the decisions (Gulf News, Arab News, Al Jazeera, the Sudan Tribune,
The National, Yemen Post), official pages (the Hong Kong Government's press
release of 2 July 2006, the Irish Statute Book, the Althingi, the e-Gov law
database, the Government of the United Arab Emirates' portal), laws (5 U.S.C.
§ 6101, the Algemene termijnenwet) and Wikipedia's articles on the workweek
and on the days free from work in Poland, Russia, Turkey and Japan, which are
secondary and say so. No PDF was opened. The exchanges' trading weeks are the
exchanges' own pages where one could be read and Wikipedia's article on the
exchange where not.

## Code

- `crates/hc-holiday/src/countries/weekends.rs`: the policies, one static per
  table, with its sources in its doc comment.
- `crates/hc-holiday/src/rule.rs`: `WeekendPolicy::{of, unread, from,
  from_day, until, until_day, in_regions}`, `RuleSet::{weekend_in, weekend_on,
  weekend_unread_in, weekend_first_year, states_a_weekend}`, `NO_WEEKEND`.
- `crates/hc-holiday/src/engine.rs`: the `unread-weekend` gap,
  `HolidayCalendar::{is_complete, holiday_gaps, weekend_is_read}`.
- `crates/hc-holiday/tests/national_weekends.rs`: the regimes of every table
  pinned, and the sweep that every table is unread before its first date.
