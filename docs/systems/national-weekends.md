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

A weekend holds from a date, not from the start of the calendar: Japan's
civil service took its Saturdays in 1992, the United States' executive
agencies had a Monday-to-Friday workweek by the law of 1966, and the sources
read reach no further back than they say. A weekend before the sources
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
`weekend_in` answers `None` for it, never Saturday and Sunday by default;
only a table that states no weekend at all keeps them, as a caller's own
table may, and as the tables of the traditions and of the United Nations'
days do, whose days belong to no working week. The engine writes one
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

A table is *dated* where a source read dates a regime of its weekend, and
*read from 2026 only* where none does and the weekend is the one the sources
read give now, read in the year they were checked: the years before 2026 are a
gap for those tables, and a later reading of an instrument or a news report
lowers each first year. Where a read source and the table's reading of 2026
disagree, the dated source is carried for its own years and the note of the
static says so (Afghanistan, Djibouti).

### The national tables and the exchanges

The first column is the first date the policy is in force: the first day of
the year where a source gives only the year, and the first of the month where
it gives only the month, unless the note of the policy in
`countries/weekends.rs` says otherwise. The note of each static carries its
sources with their URLs and the date they were retrieved, and the last column
gives the same sources' keys in [`references.bib`](../references.bib).

#### Tables with a dated weekend (72)

| Code | Country | First read | Weekend | Sources |
| --- | --- | --- | --- | --- |
| AE | United Arab Emirates | 1999 | Thursday and Friday from 1999, Friday and Saturday from 1 September 2006, Saturday and Sunday from 2022; Sharjah Friday to Sunday from 2022 | `gulfnews-uae-weekend-2006`, `thenational-weekend-uae-history-2021`, `uae-gov-public-sector-hours`, `khaleejtimes-sharjah-weekend-2021`, `gulfnews-sharjah-weekend-2021` |
| AF | Afghanistan | 2010 | Friday from 2010; Thursday and Friday from 2 December 2010; not read 2019 to 2025; Friday from 2026 | `aan-afghanistan-smog-holiday-2010`, `rferl-afghanistan-thursday-2010`, `pajhwok-afghanistan-thursday-2011`, `dailytimes-afghanistan-weekend-2018`, `wikipedia-workweek-and-weekend` |
| AM | Armenia | 21 June 2005 | Saturday and Sunday from 21 June 2005 | `arlis-armenia-labour-code-2004` |
| AU | Australia | 14 May 2015 | Saturday and Sunday from 14 May 2015 | `fwo-aps-enterprise-award-2015` |
| BD | Bangladesh | 1 April 1982 | Friday from 1 April 1982; Friday and Saturday from 9 September 2005 | `arabnews-bangladesh-weekend-2005`, `financialexpress-bangladesh-weekend` |
| BH | Bahrain | 1 March 1990 | Thursday and Friday from 1 March 1990; Friday and Saturday from 1 September 2006 | `gulfnews-bahrain-weekend-2006`, `gulfnews-bahrain-weekend-sought-2006` |
| BN | Brunei | 2019 | Friday and Sunday from 2019 | `tradegov-brunei-business-customs-2019` |
| BO | Bolivia | 26 December 2010 | Saturday and Sunday from 26 December 2010 | `lexivox-bolivia-ds-751-2010`, `lexivox-bolivia-ds-29000-2007` |
| BT | Bhutan | 2016 | Saturday and Sunday from 2016 | `thebhutanese-no-school-saturday-2016` |
| BY | Belarus | 2000 | Saturday and Sunday from 2000 | `zakony-by-trudovoj-kodeks-136`, `pravo-by-working-time-2022` |
| CD | Democratic Republic of the Congo | 2024 | Sunday from 2024; Saturday and Sunday from 1 August 2024 | `fonctionpublique-rdc-horaires-2024` |
| CL | Chile | 16 March 2005 | Saturday and Sunday from 16 March 2005 | `leychile-chile-dfl-29-2004` |
| CN | China | 25 March 1995 | Sunday from 25 March 1995; Saturday and Sunday from 1 May 1995 | `upi-china-five-day-week-1995`, `lehmanlaw-china-state-council-hours-1995` |
| CU | Cuba | 2014 | Sunday from 2014 | `gaceta-cuba-ley-116` |
| CZ | Czechia | 10 June 1968 | Saturday and Sunday from 10 June 1968 | `zakonyprolidi-cz-vyhlaska-63-1968` |
| DE | Germany | 1 March 2006 | Saturday and Sunday from 1 March 2006 | `gesetze-de-azv-2006` |
| DJ | Djibouti | 2017 | Friday and Saturday from 2017 | `embassy-djibouti-hours-2017`, `jo-djibouti-decret-2025-165`, `lanation-djibouti-horaires-2026`, `jo-djibouti-code-travail-2006` |
| DZ | Algeria | 1975 | Saturday and Sunday from 1975; Thursday and Friday from 1976; Friday and Saturday from 14 August 2009 | `jeuneafrique-algeria-weekend-2009` |
| EC | Ecuador | 6 October 2010 | Saturday and Sunday from 6 October 2010 | `derechoecuador-losep-2010` |
| EG | Egypt | 2006 | Friday from 2006; Friday and Saturday from 21 January 2006 | `amcham-egypt-work-week-2006`, `wikipedia-workweek-and-weekend` |
| ES | Spain | 1 March 2019 | Saturday and Sunday from 1 March 2019 | `boe-es-jornada-aGE-2019` |
| FI | Finland | 1 April 1969 | Saturday and Sunday from 1 April 1969 | `fiwiki-tyoaika` |
| GA | Gabon | 2022 | Sunday from 2022 | `leap-gabon-code-travail-2021` |
| GR | Greece | 1981 | Saturday and Sunday from 1981 | `karagilanis-greece-law-1157-1981` |
| HK | Hong Kong | 3 July 2006 | Saturday and Sunday from 3 July 2006 | `infogov-hk-five-day-week-2006` |
| ID | Indonesia | 1 October 1995 | Saturday and Sunday from 1 October 1995 | `pasalid-indonesia-keppres-68-1995` |
| IL | Israel | 1952 | Saturday from 1952 | `neto-israel-hours-of-work-and-rest-law` |
| IN | India | 3 June 1985 | Saturday and Sunday from 3 June 1985 | `7thpaycommission-india-om-1985`, `hellobanker-india-five-day-week-1985` |
| IQ | Iraq | 1 March 2005 | Friday and Saturday from 1 March 2005 | `aljazeera-iraq-saturday-2005`, `thenational-weekend-arab-world-2021` |
| IR | Iran | 1991 | Friday from 1991 | `irandataportal-iran-labour-law` |
| IS | Iceland | 1983 | Saturday and Sunday from 1983 | `althingi-iceland-law-88-1971` |
| JO | Jordan | 2000 | Thursday and Friday from 2000; Friday and Saturday from 8 January 2000 | `wfn-jordan-weekend-2000`, `thenational-weekend-arab-world-2021` |
| JP | Japan | 1 May 1992 | Saturday and Sunday from 1 May 1992 | `nikkei-japan-full-two-day-weekend-1992` |
| KE | Kenya | 25 September 2023 | Saturday and Sunday from 25 September 2023 | `src-kenya-part-time-2023` |
| KR | South Korea | 1 July 2005 | Saturday and Sunday from 1 July 2005 | `wikisource-korea-civil-servant-regulation` |
| KW | Kuwait | 2007 | Thursday and Friday from 2007; Friday and Saturday from 1 September 2007 | `arabnews-kuwait-weekend-2007` |
| LA | Laos | 19 December 2017 | Saturday and Sunday from 19 December 2017 | `laopost-laos-decree-386-2017` |
| LI | Liechtenstein | 2009 | Saturday and Sunday from 2009 | `gesetze-li-staatspersonalverordnung-2008` |
| LR | Liberia | 2016 | Sunday from 2016 | `wageindicator-liberia-weekly-rest` |
| LY | Libya | 2006 | Friday and Saturday from 2006 | `thenational-weekend-arab-world-2021`, `wikipedia-workweek-and-weekend` |
| MA | Morocco | 20 July 2005 | Saturday and Sunday from 20 July 2005 | `lavieeco-morocco-decree-2005` |
| MG | Madagascar | 1 August 2009 | Saturday and Sunday from 1 August 2009 | `lexxika-madagascar-decree-2009-969` |
| MN | Mongolia | 1 July 1999 | Saturday and Sunday from 1 July 1999 | `legalinfo-mongolia-labour-law-1999`, `legalinfo-mongolia-labour-law-2021` |
| MR | Mauritania | 6 April 2005 | Saturday and Sunday from 6 April 2005; Friday and Saturday from 23 December 2007; Saturday and Sunday from 1 October 2014 | `irin-mauritania-weekend-2005`, `cath-mauritania-friday-2007`, `cridem-mauritania-weekend-2014`, `jeuneafrique-mauritania-weekend-2014` |
| MV | Maldives | 2013 | Friday and Saturday from 2013 | `gazette-maldives-office-hours-2024`, `lloydsbanktrade-maldives-opening-hours`, `wikipedia-workweek-and-weekend` |
| MY | Malaysia | 25 November 2013 | Saturday and Sunday from 25 November 2013; the states Kedah, Kelantan and Terengganu Friday and Saturday from the same day; Johor Saturday and Sunday from 1995, Friday and Saturday 2014 to 2024; a gap before | `jakartapost-johor-weekend-2013`, `rtm-johor-weekend-2025`, `mkn-johor-weekend-2025`, `mylaw-holidays-act-1951`, `parlimen81-weekend-history` |
| NL | Netherlands | 1 April 1965 | Saturday and Sunday from 1 April 1965 | `wetten-nl-termijnenwet`, `nlwiki-vrije-zaterdag` |
| NP | Nepal | 2022 | Saturday from 2022; Saturday and Sunday from 15 May 2022; Saturday from 15 June 2022; Saturday and Sunday from 6 April 2026 | `onlinekhabar-nepal-two-day-weekend-2022`, `kathmandupost-nepal-two-day-weekend-2022`, `kathmandupost-nepal-sunday-rollback-2022`, `himalayantimes-nepal-weekend-revoked-2022`, `spotlightnepal-nepal-two-day-weekend-2026`, `onlinekhabar-nepal-two-day-weekend-2026` |
| OM | Oman | 2013 | Thursday and Friday from 2013; Friday and Saturday from 1 May 2013 | `alriyadh-oman-weekend-2013`, `thenational-oman-weekend-2013`, `gulfnews-oman-weekend-2013` |
| PE | Peru | 4 January 1996 | Saturday and Sunday from 4 January 1996 | `infopublic-peru-dl-800` |
| PH | Philippines | 27 December 1991 | Saturday and Sunday from 27 December 1991 | `ecodal-philippines-omnibus-rules-book-v`, `elibrary-philippines-csc-mc-21-1991` |
| PK | Pakistan | 9 June 2022 | Saturday and Sunday from 9 June 2022 | `geo-pakistan-saturday-2022`, `radiopk-four-day-week-2026` |
| PL | Poland | 1951 | Sunday from 1951; not read from 1973; Saturday and Sunday from 2001 | `infor-poland-days-off-act-1951`, `plwiki-days-off-poland`, `plwiki-wolna-sobota-prl` |
| PS | Palestine | 2007 | Thursday and Friday from 2007; Friday and Saturday from 1 July 2007 | `unispal-chronology-2007-07`, `gpc-palestine-employee-guide` |
| PT | Portugal | 1 August 2014 | Saturday and Sunday from 1 August 2014 | `pgdlisboa-portugal-ltfp-2014` |
| QA | Qatar | 2003 | Thursday and Friday from 2003; Friday and Saturday from 1 August 2003 | `arabnews-qatar-weekend-2003` |
| RU | Russia | 1940 | Sunday from 1940; not read from 1967; Saturday and Sunday from 1968 | `ruwiki-workweek`, `consultant-russia-tk-111` |
| SA | Saudi Arabia | 2013 | Thursday and Friday from 2013; Friday and Saturday from 29 June 2013 | `spa-saudi-weekend-2013` |
| SD | Sudan | 21 January 1998 | Friday from 21 January 1998; Friday and Saturday from 26 January 2008 | `sudantribune-sudan-weekend-2008`, `mondaq-sudan-hours-1998`, `sudanhorizon-eid-2026` |
| SG | Singapore | 1 September 2004 | Saturday and Sunday from 1 September 2004 | `mfa-singapore-five-day-workweek-2004` |
| SK | Slovakia | 10 June 1968 | Saturday and Sunday from 10 June 1968 | `zakonyprolidi-cz-vyhlaska-63-1968` |
| SO | Somalia | 2025 | Friday from 2025 | not re-read (see the note) |
| SY | Syria | 1 February 2004 | Friday and Saturday from 1 February 2004 | `addustour-syria-saturday-2003`, `mondaq-syria-hours-1998` |
| TG | Togo | 2022 | Sunday from 2022 | `lqdd-togo-weekly-rest` |
| TL | Timor-Leste | 2013 | Sunday from 2013 | `jornal-timor-leste-lei-4-2012` |
| TN | Tunisia | 17 September 2012 | Saturday and Sunday from 17 September 2012 | `legislation-securite-tn-decret-2012-1710` |
| TR | Türkiye | 2 January 1924 | Friday from 2 January 1924; Sunday from 2 June 1935; Saturday and Sunday from 1 July 1974 | `tdv-turkey-hafta-tatili` |
| TW | Taiwan | 2001 | not read 1998 to 2000; Saturday and Sunday from 2001 | `zhwiki-two-day-weekend-taiwan`, `taiwan-panorama-weekend-1998` |
| US | United States | 6 September 1966 | Saturday and Sunday from 6 September 1966 | `cornell-5-usc-6101` |
| VA | Vatican City | 2011 | Sunday from 2011 | `vatican-governorate-regulation-2010` |
| VN | Vietnam | 2021 | Saturday and Sunday from 2021 | `vn-holiday-notices`, `vn-labour-code-2019`, `ecotravelvietnam-business-hours-2024` |
| YE | Yemen | 2013 | Thursday and Friday from 2013; Friday and Saturday from 15 August 2013 | `yemenpost-yemen-weekend-2013`, `alkhaleej-yemen-weekend-2013`, `albawaba-yemen-weekend-2012` |

#### Tables read from 2026 only (123)

For these tables no source read dates the weekend law, so the weekend is the
one the live English Wikipedia article "Workweek and weekend" gives,
retrieved 2026-10-04 and secondary, read from 2026, the year it was checked;
every year before is a gap, and a later reading of an instrument or a news
report lowers each first year.

- Saturday and Sunday from 2026 (119): AD Andorra, AG Antigua and Barbuda, AL Albania, AO Angola, AR Argentina, AT Austria, AZ Azerbaijan, BA Bosnia and Herzegovina, BB Barbados, BE Belgium, BF Burkina Faso, BG Bulgaria, BI Burundi, BJ Benin, BR Brazil, BS Bahamas, BW Botswana, BZ Belize, CA Canada, CG Republic of the Congo, CH Switzerland, CI Côte d'Ivoire, CM Cameroon, CO Colombia, CR Costa Rica, CV Cabo Verde, CY Cyprus, DK Denmark, DM Dominica, DO Dominican Republic, EE Estonia, ET Ethiopia, FJ Fiji, FM Micronesia, FR France, GB United Kingdom, GD Grenada, GE Georgia, GH Ghana, GM The Gambia, GN Guinea, GQ Equatorial Guinea, GT Guatemala, GY Guyana, HN Honduras, HR Croatia, HT Haiti, HU Hungary, IE Ireland, IT Italy, JM Jamaica, KG Kyrgyzstan, KI Kiribati, KM Comoros, KN Saint Kitts and Nevis, KZ Kazakhstan, LB Lebanon, LC Saint Lucia, LK Sri Lanka, LS Lesotho, LT Lithuania, LU Luxembourg, LV Latvia, MC Monaco, MD Moldova, ME Montenegro, MH Marshall Islands, MK North Macedonia, ML Mali, MM Myanmar, MO Macau, MT Malta, MU Mauritius, MW Malawi, MX Mexico, MZ Mozambique, NA Namibia, NE Niger, NG Nigeria, NI Nicaragua, NO Norway, NR Nauru, NZ New Zealand, PA Panama, PG Papua New Guinea, PW Palau, PY Paraguay, RO Romania, RS Serbia, RW Rwanda, SB Solomon Islands, SC Seychelles, SE Sweden, SI Slovenia, SL Sierra Leone, SM San Marino, SN Senegal, SR Suriname, SS South Sudan, SV El Salvador, TD Chad, TH Thailand, TJ Tajikistan, TM Turkmenistan, TO Tonga, TT Trinidad and Tobago, TV Tuvalu, TZ Tanzania, UA Ukraine, UG Uganda, UY Uruguay, UZ Uzbekistan, VC Saint Vincent and the Grenadines, VE Venezuela, VU Vanuatu, WS Samoa, ZA South Africa, ZM Zambia, ZW Zimbabwe.
- Sunday from 2026 (4): GW Guinea-Bissau, KH Cambodia, KP North Korea, SZ Eswatini.

#### Examined and not carried

Each country below was looked for in a dated source on 2026-10-05 and stays
in the group above, for the reason given; a source that says only that the
week has five days, without naming them, dates nothing.

- **Austria (AT)**: the Beamten-Dienstrechtsgesetz 1979, § 48(2a), keeps Saturdays, Sundays and holidays duty-free for federal officials unless service requires otherwise (JUSLINE's mirror, state of 4 October 2026), but the mirror does not date the sentence, and the Federal Legal Information System, which does, did not answer.
- **Belgium (BE)**: the 45-hour week over five days (agreed in 1955, law of 1964) is the private sector's history in a secondary article and names no day.
- **Brazil (BR)**: Decreto 1.590 of 10 August 1995 gives the federal public administration eight hours a day and forty a week and names no day of the week.
- **Burkina Faso (BF), Comoros (KM), Guinea (GN), Côte d'Ivoire (CI)**: the pages read give the hours of Monday to Thursday and of Friday, not a weekend or a five-day week.
- **Cambodia (KH)**: the Labour Law of 1997, article 147, gives the weekly rest "in principle" on Sunday; the civil servants' five-day week (a sub-decree of 1 April 2016) names no days in the page read.
- **Canada (CA)**: the Treasury Board's collective agreement for one bargaining group (expiring 18 April 2026) has a normal workweek of 37.5 hours "Monday through Friday", but it does not date the five-day week's beginning.
- **Croatia (HR), Montenegro (ME), Serbia (RS), Bosnia and Herzegovina (BA), Slovenia (SI)**: the labour laws read give the weekly rest on Sunday and the day before or after it, on Sunday as a rule, or for twenty-four hours, and name Saturday nowhere.
- **France (FR), Monaco (MC)**: the weekly rest is "in principle" Sunday in the texts read; none names Saturday.
- **Georgia (GE), Azerbaijan (AZ), Myanmar (MM), Venezuela (VE), Uruguay (UY), New Zealand (NZ)**: the texts read give a five-day week or two rest days a week, with no day named (Georgia's Law on Public Service of 2015, article 60, in force 1 January 2017; Azerbaijan's Labour Code, in force since 1999; Venezuela's LOTTT, article 173, of 2012; New Zealand's five-day week of 1936 in a public-service history).
- **Hungary (HU)**: the Kttv, § 89(1), fixes a general schedule of Monday to Thursday and Friday (promulgated on 30 December 2011), but the text read is consolidated and amended and its day of entry into force was not read.
- **Kazakhstan (KZ), Kyrgyzstan (KG), Uzbekistan (UZ), Tajikistan (TJ), Turkmenistan (TM), Ukraine (UA)**: the labour codes read make Sunday the common day off and leave the second day of a five-day week to the schedule of the organisation (Kazakhstan, Labour Code of 23 November 2015, article 84; Kyrgyzstan, Labour Code of 2025, article 65; Uzbekistan, Labour Code of 28 October 2022, in force 30 April 2023, article 207; Tajikistan, Labour Code of 23 July 2016, article 86; Turkmenistan, Labour Code, article 76; Ukraine, Labour Code of 1971, article 67), so Saturday is the custom and no text read.
- **Latvia (LV)**: the Labour Law (in force 1 June 2002) names Sunday as the general rest day, and its § 133(4) mentions state-budget institutions with a Monday-to-Friday week, but the amendment's date was not read.
- **Lebanon (LB)**: Parliament's vote of July 2017 for Saturday and Sunday (Lebanon Files, 15 July 2017) and the Minister of Social Affairs' announcement of 19 April 2018 "turning Saturday into a holiday" (Blog Baladi) disagree about when the Saturday became a holiday.
- **Luxembourg (LU)**: the Grand-Ducal Regulation of 12 November 2011 spreads the State's five working days "du lundi au samedi".
- **Macau (MO)**: the Estatuto dos Trabalhadores da Administração Pública de Macau (Decreto-Lei 87/89/M) could not be read, the Official Gazette's host not answering.
- **North Korea (KP)**: the Socialist Labour Law makes Sundays and State holidays days of rest in a page that is no longer served (the site had expired), and no article or date could be read again.
- **Romania (RO)**: article 137 of the Labour Code gives the weekly rest "de regulă sâmbăta și duminica" in a mirror of the republished text of 2011 that has been amended since, and the date of the wording was not read.
- **Sri Lanka (LK)**: the Holidays Act, No. 29 of 1971, in force on 26 August 1971, makes every Sunday and Poya day a public and bank holiday; the Sunday Times says that the Saturday and Sunday weekend returned under Sirimavo Bandaranaike's government (1970 to 1977) after the poya weekend of 1965, without a date.
- **Sweden (SE), Norway (NO)**: the sources read date the five-day week of the general labour market (Sweden 1971 or 1973, the two sources disagreeing; Norway 1968) and not the state administration's.
- **Thailand (TH)**: the Prime Minister's Office announcement of 6 August 2491 (1948) gives Sunday and Saturday from noon (read on Wikisource); the announcement (No. 12) of 2502 (1959) that makes Saturday and Sunday the weekly holidays is named in pages read, but its text and its date were not, and eleven amendments between the two were not read.
- **United Kingdom (GB)**: the Financial Secretary's speech of 19 April 1956 (Hansard) offers the Civil Service a five-day week "if it is ratified ... from 1st July"; its ratification was not read.
- **Vanuatu (VU)**: the Employment Act (in force 30 May 1983) gives the weekly rest "normally" on Sunday, the private sector's rest; the table's own reading is Saturday and Sunday.
- **Others**: Albania, Andorra, Argentina, Bulgaria, Colombia, Costa Rica, Cyprus, Denmark, El Salvador, Estonia, Ethiopia, Fiji, Guatemala, Honduras, Ireland, Italy, Jamaica, Malta, Mexico, Moldova, Nicaragua, North Macedonia, Panama, Paraguay, San Marino, Switzerland, Tanzania, Trinidad and Tobago, Uganda, the Dominican Republic and the small states of the Caribbean and the Pacific: the pages read give hours without days, or are not served as text (script pages, PDFs only, hosts that did not answer), or give a date without a day. Their regimes stay a gap before 2026 until a source is read.

#### The exchanges

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
  that is Sunday, it is not the weekend carried for a table whose own weekend
  is Saturday and Sunday; that table is carried from 2026 and the
  disagreement is in the list of tables examined and not carried
  (Equatorial Guinea, Honduras, Colombia, Croatia, Vanuatu and others). A table
  that carries Sunday as its weekend does so from the Code that says so (Cuba,
  Gabon, Liberia, Togo, Timor-Leste, Vatican City).
- **A labour law that gives the working week as five days and no days** dates
  nothing: Azerbaijan, Georgia, Venezuela and Uruguay stay as they are. A law
  that names Sunday as the common day off and leaves the second day to the
  schedule (the labour codes of Kazakhstan, Kyrgyzstan, Uzbekistan, Tajikistan,
  Turkmenistan and Ukraine) names no Saturday either.
- **A source that the table's reading of 2026 contradicts is carried for its
  own years.** Afghanistan's Thursday and Friday of 2010 to 2018 and
  Djibouti's Friday and Saturday from 2017 differ from the Wikipedia page's
  reading, and Algeria's Saturday and Sunday of 1975 is the one regime that no
  page read confirms: a news report of 2009 gives the Thursday and Friday it
  replaced.
- **A date the source gives of a report, not of the instrument,** is the
  policy's first day, and the note says so (Laos, Kenya, Palestine's first
  regime). Where the instrument's own day of effect is not on the page read,
  the instrument's date is taken and the note says so (Bolivia, Morocco,
  Philippines).
- **Sources are secondary for the tables read in 2026.** The page read for
  them is the live English Wikipedia article "Workweek and weekend",
  retrieved 2026-10-04, which does not date their regimes, so the first year
  of each is the year of the reading and every earlier year is a gap.

## Sources

Each regime's source is named in the note of its static in
`countries/weekends.rs`, with its URL and the date it was retrieved, and in
the last column of the dated table by its key in
[`references.bib`](../references.bib). They are laws and decrees (the
Arbeitszeitverordnung, the Keppres 68/1995, 5 U.S.C. § 6101, the Algemene
termijnenwet, Armenia's Labour Code, the Tunisian décret 2012-1710, Djibouti's
Décret 2025-165, the Althingi's law of 1971), officials' pages (the Hong Kong
Government's press release of 2 July 2006, the Government of the United Arab
Emirates' portal, Singapore's Ministry of Foreign Affairs, the Fair Work
Ombudsman's award, the Vatican Governorate's regulation), news reports of
decisions (Gulf News, Arab News, Al Jazeera, the Sudan Tribune, The National,
Yemen Post, Al Riyadh, IRIN, the United Nations' chronological review) and
Wikipedia's articles on the workweek and on the days free from work, which are
secondary and say so. A law that exists only as a PDF was not opened, and the
note says so; a page that the Internet Archive keeps and the live site refuses
is read in the archive's copy and named as such. The exchanges' trading weeks
are the exchanges' own pages where one could be read and Wikipedia's article
on the exchange where not.

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
