# Holidays and observances

The second half of requirement 3: public holidays by country, and religious or
cultural observances by tradition, listed in full and covered in stages.

## How this is modelled

A holiday is **data**, never code. `hc-holiday` provides one evaluator and a
rule vocabulary; every country and tradition is a table of rule values.

```text
Rule                          Example
────────────────────────────  ──────────────────────────────────────────────
FixedGregorian { month, day } New Year's Day, 1 January
NthWeekday { month, n, wd }   US Thanksgiving, 4th Thursday of November
                              Japan's 成人の日, 2nd Monday of January
LastWeekday { month, wd }     UK Spring Bank Holiday, last Monday of May
WeekdayOnOrAfter { m, d, wd } Midsommardagen, the Saturday on or after 20 June
WeekdayOnOrBefore { m, d, wd} Buß- und Bettag, the Wednesday before 23 Nov
FixedInCalendar { cal, m, d } Eid al-Fitr, 1 Shawwal in islamic-umalqura
                              Rosh Hashanah, 1 Tishrei in hebrew
SolarTerm { term, meridian }  春分の日 / 秋分の日, the equinoxes at UTC+9
                              清明節, at the Beijing meridian
EasterRelative { comp, offs } Good Friday (-2), Easter Monday (+1)
                              Ash Wednesday (-46), Pentecost (+49)
LunarPhase { phase, m, d }    the first full moon on or after a fixed date
Tithi { m, tithi, prevails }  Rāma Navamī, Chaitra śukla 9 at midday
Sankranti { sign, ayanamsa }  Makar Sankranti, the Sun's entry into Makara
Nakshatra { n, sign, tithi }  Thaipusam, Puṣya in Thai
Offset { base, days }         除夕, Seollal's eve
Span { from, to }             Dashain, Phūlpātī to Āśvina śukla 12
MovedByWeekday { base, moves } a Monday-holiday law: Argentina's trasladables, Colombia's Ley Emiliani
Tabulated { fn, first, last } Matariki, gazetted through a stated last year
Computed(fn)                  the few that really are bespoke
```

Each is a pure rule value. `WeekdayOnOrAfter`, `WeekdayOnOrBefore` and
`Offset` each spare a `Computed` that would otherwise be needed, and
`Tabulated` gives a published table a last year, so that running out of table
is reported as a gap rather than passing for a year without the holiday.

Rules then pass through **observance modifiers**, which are themselves data:

- `SubstitutionPolicy` — the UK and US "observed" shift, Japan's 振替休日. It
  carries which weekdays trigger it, which direction it moves in, whether the
  search steps past a day already taken (Japan's 2005 amendment), and whether
  two holidays landing on one day also trigger it (Korea's 대체공휴일).
- `BridgePolicy` — Japan's 国民の休日 (a working day trapped between two
  holidays becomes one).
- `ValidFrom` / `ValidUntil` — a holiday that was created or abolished. Every
  rule carries these, because "is today a holiday in Japan" has a different
  answer in 1990 and 2020 and a library that ignores that is wrong for history.
- `Region` — subdivision scoping, used for German *Länder*, Canadian
  provinces, Australian states and territories, the United Kingdom's three
  jurisdictions, French Alsace-Moselle, the three units of Bosnia and
  Herzegovina, Bangladesh's hill districts, and a few single places: `US-DC`,
  `MD-CU`, `GT-GU`, `SV-SS`, `NI-MN` and `CL-AP`. Swiss cantons, Spanish
  autonomous communities and US states are *not* modelled: the vocabulary
  carries them, the tables do not.
- `Kind` — public holiday, bank holiday, school holiday, observance without a
  day off, religious day of obligation, and a weekend day made a working day
  (China's 调休上班).
- `WeekendPolicy` — which days are the weekend, over stated years, because
  Saudi Arabia (2013), the United Arab Emirates (2022) and Nepal (2026) all
  changed theirs inside living memory.

Because the rule set is data, a caller can supply their own table — a company
calendar, a school year, a fictional setting — and get the same engine.

A rule is `Computed` only where its statute is a sentence rather than a
pattern: Ireland's St Brigid's Day, the Dutch royal day (under two monarchs),
US Inauguration Day, Mexico's presidential handover, Israel's Yom HaAtzmaut
and some thirty more in the national tables, most of them a move that
depends on the weekday or on another holiday. New Zealand's Matariki is
`Tabulated`, with the last gazetted year stated. Japan needs none.

## What the engine will not do

- **It will not claim a Hijri holiday is exact.** Eid dates depend on lunar
  crescent visibility decided per country, sometimes on the night before. The
  library returns the Umm al-Qura or tabular computation and flags it
  `Approximate`. It is a good prediction, not an announcement.
- **It will not invent substitution rules it cannot cite.** A country's
  weekend-substitution behaviour is law, and laws differ. Most of the tables
  below carry no substitution policy at all, because their countries have
  none in calendar-expressible form; [supported.md](supported.md) shows which.
- **It will not guess an annual administrative act.** China's 调休, Taiwan's
  swaps of a working day for a Saturday, Vietnam's Tết span and *làm bù*,
  Thailand's Songkran makeup days and Indonesia's *cuti bersama* are decided
  year by year by a ministry. Where those acts have been read they are data
  for their years: China's arrangements from 2008 to 2026, Russia's transfer
  decrees from 2013 to 2027, Taiwan's swaps from 2017 until they ended in
  2025 and Vietnam's notices from 2021 to 2026, the days off and the weekend
  days worked alike ([ADR 0009](adr/0009-a-working-day-is-an-entry.md)). A
  year past them is a gap. Elsewhere the statutory days are listed and the
  bridging days are not.
- **It will not tabulate what it cannot compute.** Indonesia's Nyepi is
  absent because the Balinese Saka calendar it is dated in is not in the
  crate. India's Hindu, Sikh and Jain gazetted holidays and Singapore's and
  Malaysia's Deepavali are computed on `hindu-lunar` and `nanakshahi`.
- **It will not pretend a holiday list is current.** Every country table
  carries the date its sources were checked.
- **It will not answer outside the span it evaluated.** Business-day
  arithmetic that walks off the end of a `HolidayCalendar` returns `None`.

## Stage 1 — Religious and traditional cycles

> A rule can be dated in any calendar, not a fixed list of them.
> `CalendarSystem` is an open struct keyed to a registry identifier, so a
> feast kept in the Ethiopic, Coptic, Solar Hijri or Badíʿ calendar is
> written in that calendar, and the Ethiopian Orthodox table is there to
> prove it rather than to describe it.

The cross-cutting ones, because national tables depend on them.

| Tradition | Coverage | Status |
| --- | --- | --- |
| Christian, Gregorian computus | Easter and the feasts keyed to it — 22 offsets from Septuagesima to the Sacred Heart — plus Advent, Christmas, Epiphany, Candlemas, All Saints | Done |
| Christian, Julian computus (Orthodox) | Orthodox Pascha and its cycle, Julian-dated fixed feasts; the Julian date and the civil date are both available | Done |
| Ethiopian Orthodox Tewahedo | Enkutatash, Meskel, Genna, Timkat and Debre Tabor dated in the Ethiopic calendar; the Bahire Hasab movable cycle from the Fast of Nineveh to Paraclete as *tewsak* offsets from Tinsae, which is the Julian-computus Pascha | Done — the calendrical date is given; where a feast is kept on a fixed Gregorian date by practice, as Genna is outside Lalibela in leap years, that is not modelled |
| Coptic Orthodox | The fourteen feasts of the Lord, Nayrouz, both Feasts of the Cross, the Apostles and St Mary's Dormition and Assumption dated in the Coptic calendar; the paschal cycle from the Fast of Nineveh to Pentecost on the Alexandrian computus | Done — Arabic names; the Coptic-language names are not carried |
| Islamic | Ras as-Sanah, Ashura, Mawlid, Isra and Miraj, Mid-Sha'ban, Ramadan, Laylat al-Qadr, Eid al-Fitr, Eid al-Adha, Day of Arafah | Done — flagged `Approximate` |
| Jewish | Rosh Hashanah, Fast of Gedaliah, Yom Kippur, Sukkot, Hoshana Rabbah, Shemini Atzeret, Simchat Torah, Hanukkah, Tenth of Tevet, Tu BiShvat, Purim, Shushan Purim, Passover, Lag BaOmer, Shavuot, Seventeenth of Tammuz, Tisha B'Av | Done — the Omer count itself lives in `hc-calendars-lunar::hebrew`; the Sabbath postponements of the minor fasts are not modelled |
| Buddhist | Vesak, Magha Puja, Asalha Puja, Vassa, Pavarana, Bodhi Day, Nirvana Day, Buddha's Birthday in both reckonings | Partial — the Theravada full moons are approximated from the Chinese lunisolar calendar and flagged `Approximate`; the Mahayana dates are exact |
| Chinese folk | 除夕, 春節, 元宵, 清明, 端午, 七夕, 中元, 中秋, 重陽, 臘八, 冬至 | Done |
| Hindu | Makar Sankranti, Maha Shivaratri, Holika Dahan and Holi, Ugadi, Rama Navami, Mahavir Jayanti, Mesha Sankranti, Akshaya Tritiya, Buddha Purnima, Guru Purnima, Raksha Bandhan, Krishna Janmashtami, Ganesh Chaturthi, Navaratri, Durga Ashtami, Vijaya Dashami, Diwali, Guru Nanak Jayanti, Thaipusam | Done — each on the part of the day its tithi must hold, at the national almanac's sunrise; reproduces the Rashtriya Panchang's festival lists for 2023–2025. Thaipusam is a nakṣatra rule, Puṣya in Thai, fitted to the days Malaysia and Mauritius gazetted for 2020–2026 |
| Sikh | The gurpurabs — Parkash, Gurgaddi, Joti Jot and Shaheedi of the Gurus, the Sahibzadas, the Guru Granth Sahib — Vaisakhi, Hola Mohalla, Bandi Chhor Divas | Done — on the Nanakshahi calendar of 2003 (`nanakshahi`, see [calendars.md](calendars.md)), fixed Gregorian dates, with the three days that calendar left lunar on the same rules as Holi, Diwali and Kartik Purnima. The SGPC's post-2010 dates, which are the Bikrami calendar's, are not carried |
| Jain | Mahavir Jayanti, Akshaya Tritiya, Paryushana and Samvatsari, Das Lakshana and Anant Chaturdashi, Diwali | Done — the Śvetāmbara eight days and the Digambara ten counted back from Samvatsari and Anant Chaturdashi on `hindu-lunar`; every entry approximate, since the sects' almanacs can differ from the national one by a day; Kshamavani not carried where its source contradicts itself |
| Bahá'í | The nine holy days — Naw-Rúz, the three days of Riḍván, the Declaration of the Báb, the Ascension of Bahá'u'lláh, the Martyrdom of the Báb, the Twin Holy Birthdays — with the Day of the Covenant and the Ascension of ʻAbdu'l-Bahá, and the first days of Ayyám-i-Há and of the Fast | Done — dated in the Badíʿ calendar as kept (`bahai`, see [calendars.md](calendars.md)), so exact through 19 March 2065 and a reported gap after, where the Bahá'í World Centre's table ends; the Twin Holy Birthdays follow the same table, 2015–2064 |
| Zoroastrian | Nowruz, Khordad Sal, the name-day feasts (Tiragan, Mehregan …), the six Gahambars, Zartosht No-Diso, Muktad | Done — the Parsi schedule of feasts as three tables, one per reckoning (`zoroastrian-fasli`, `zoroastrian-shahanshahi`, `zoroastrian-qadimi`, see [calendars.md](calendars.md)), every feast a fixed day of a fixed month, so exact. The Iranian community's dates on the civil calendar, and Sadeh and Yalda, which are Iranian festivals rather than days of the schedule, are not carried |
| Shinto | 初詣, 節分, 大祓, 七五三; the imperial court rites 宮中祭祀 as a second table | Done — `shinto` for the year as kept at a shrine and at home, 節分 as the eve of 立春 at the Japanese meridian; `kyuchu-saishi` for the Reiwa-era schedule of the 大祭, 小祭 and 旬祭, the equinox rites on 春分の日 and 秋分の日, 天長祭 bounded to its 2020 start. A shrine's own festival days are not carried |
| Pagan / Wheel of the Year | Samhain, Yule, Imbolc, Ostara, Beltane, Litha, Lughnasadh, Mabon | Done — two tables, northern and southern hemisphere; the quarter days on their Universal Time day, the cross-quarter days on their fixed dates; eve and nearest-weekend conventions not modelled |
| Secular international | The United Nations' international days — those of the General Assembly and of UNESCO, WHO, FAO and the other agencies on its list | Done — `international::UNITED_NATIONS`, every entry citing its resolution or designating body in `HolidayRule::source`; the seven rule-based days carry their rules; the weeks are not carried |

## Stage 2 — National public holidays

Ordered by how well the sources can be cited, not by importance.

**Done**

| Country | Note |
| --- | --- |
| Japan 🇯🇵 | Complete from 1948 (祝日法) to the present, including every amendment: the 1973 振替休日 (in force from 12 April, so 1973-02-12 is *not* a holiday), the 1985 国民の休日, ハッピーマンデー in 2000 and 2003, 海の日, 山の日, the 1959/1989/1990/1993 imperial one-offs, the 2019 即位礼正殿の儀 and the 2020–21 Olympic moves. 春分の日 and 秋分の日 are computed from the equinox at the Japan meridian, not tabulated. |
| United States 🇺🇸 | Federal holidays with the Saturday/Sunday observed rule, the Uniform Monday Holiday Act, Veterans Day's 1971–77 detour, Juneteenth from 2021, Inauguration Day for the capital region, and the two state-funeral days |
| United Kingdom 🇬🇧 | England and Wales, Scotland and Northern Ireland as separate regions, with the royal one-offs and the three jubilee moves of the Spring Bank Holiday |
| Ireland 🇮🇪 | Including St Brigid's Day and its conditional rule |
| Germany 🇩🇪 | Federal plus all 16 *Länder*, including Buß- und Bettag before and after 1995 |
| France 🇫🇷 | Métropole plus Alsace-Moselle, with 8 May's 1959–81 absence |
| Armenia 🇦🇲 | The 2001 law's thirteen non-working days with 27 January from 2026, its working holidays as observances; the Government's Merelots decisions not carried |
| Azerbaijan 🇦🇿 | Labour Code art. 105: Novruz's five days, two-day Eids approximate, the 2006 rest-day rule with Eid collisions that reproduces March 2026 |
| Georgia 🇬🇪 | Labour Code art. 30 with the Orthodox Easter; nothing moves |
| Kazakhstan 🇰🇿 | The 2001 law through the June 2026 move of Constitution Day to 15 March, with Nauryz, Republic Day, First President Day and 17 December by their years; art. 5's rest-day rule |
| Uzbekistan 🇺🇿 | Labour Code art. 208: the seven fixed days and the first day of each Hayit, approximate; a weekend holiday's day off to the next working day; the yearly decreed extras not carried |
| Kyrgyzstan 🇰🇬 | The 2004 code's art. 113 through 2024 with its transfer rule, 7 April from 2016 and 7–8 November from 2018; the 2025 code's art. 66 with the New Year and May holidays, the demoted state holidays as observances, and no transfer |
| Tajikistan 🇹🇯 | The Law on Holidays art. 3 with Navruz's four days and 1 May until 2016; Labour Code art. 89(5)'s transfer to the next working day |
| Turkmenistan 🇹🇲 | Labour Code art. 81: the nine days with Kurban Bayram's three, approximate; art. 81(2)'s Sunday rule; 2018's move of Independence Day and merger of Flag Day |
| Hong Kong 🇭🇰 | The seventeen general holidays with the statutory subset as `Kind::Public` and the 2021 phasing-in as years; Sunday and coincidence made up on the next free day, the 1983–2011 eve rule computed; complete from 1997 |
| Macau 🇲🇴 | Executive Order 60/2000 with the ten obligatory holidays of Law 7/2008 as `Kind::Public`; the public administration's compensatory rest days from 2019; the eves as observances |
| China 🇨🇳 | Statutory holidays keyed to the `chinese` calendar, across the 1999, 2007 and 2024 revisions; the State Council's arrangement for each year from 2008 to 2026, days off and working weekend days, with the three notices that changed a year |
| Taiwan 🇹🇼 | The 紀念日及節日實施條例 of May 2025 and the 辦法 before it: the making-up of a weekend holiday on the nearer working day and of the Lunar New Year days after them, Children's Day when 清明 falls on it, and the five days the 條例 added; the swaps of each office calendar from 2017 until they ended in 2025. Checked against the government office calendar for 2017–2027 |
| South Korea 🇰🇷 | Keyed to the `dangi` calendar, with every extension of the 대체공휴일 and the collision rule; 노동절 and the restored 제헌절 from 2026; the election days of 제2조제10호의2 from 2007 and the government-designated days from 2009, each for its year |
| Canada 🇨🇦 | Federal plus the provincial days fixed by statute |
| Australia 🇦🇺 | National plus all six states and both territories |
| New Zealand 🇳🇿 | Including mondayisation from 2014, and Matariki over 2022–2035 — the Act schedules to 2052, and a calendar past 2035 reports it as a gap rather than dropping it |
| Micronesia 🇫🇲 | Title 1, chapter 6: the national holidays with section 602's Friday and Monday, Veterans' day from 2004, Culture and Tradition Day from 2010, Presidents Day and the Veterans Day name from 2021 |
| Marshall Islands 🇲🇭 | The 1988 Act's Schedule as amended to 2015, the Friday holidays by weekday, section 902's Friday and Monday; General Election Day not carried |
| Nauru 🇳🇷 | Section 81 of the Public Service Act 2016 with its Monday, and Monday and Tuesday, for a weekend; the President's declared days tabulated from the 2023, 2024 and 2026 gazettes, other years a gap |
| Palau 🇵🇼 | 1 PNCA §§ 701–702: the nine legal holidays with the Friday and Monday rule, Family Day on the fourth Friday of November |
| Solomon Islands 🇸🇧 | Cap. 151's Schedule with the Sunday rule and the Tuesday after a Monday 26 December; the Sovereign's Birthday, the provincial days and the notices' Saturday-to-Friday practice not carried |
| Tonga 🇹🇴 | The Act's section 2 as the Prime Minister's Office applies it in 2024 and 2026: three days to a Monday by weekday, the royal birthdays of the reign off a Sunday, the rest where they fall |
| Tuvalu 🇹🇻 | Cap. 4.50's 2022 Schedule with the Monday for a weekend and the Tuesday for Tuvalu Day and Boxing Day; National Children's Day, dated from an undefined White Sunday, not carried |
| Vanuatu 🇻🇺 | Cap. 114's fourteen days with the Sunday rule and the Tuesday for a Monday Family Day, dated and named as the Government lists them; provincial days not carried |
| Samoa 🇼🇸 | The 2008 Act with the Monday, and Tuesday, for a Sunday, Independence Day on 1 June as the Ministry's calendar gives it, the three Mondays after the second Sundays of May, August and October |
| Brazil 🇧🇷 | Including Consciência Negra from 2024 |
| Mexico 🇲🇽 | Including the 2006 Monday reform and the six-yearly presidential handover |
| Saudi Arabia 🇸🇦 | Umm al-Qura based, with the 2013 weekend change |
| United Arab Emirates 🇦🇪 | With the 2022 move to a Saturday–Sunday weekend |
| Israel 🇮🇱 | Hebrew-dated and therefore exact, with Yom HaAtzmaut's Sabbath-avoidance rule |
| Iran 🇮🇷 | Civil holidays on the astronomical `persian` calendar, so exact — Nowruz 1404 on 21 March 2025, where the arithmetic cycle says the 20th; the lunar Hijri days flagged `Approximate`, since Iran declares them on its own sighting; Friday weekend |
| Albania 🇦🇱 | Law 7651's fifteen holidays with both Easters, approximate Eids and the weekend rule that gives each holiday its own working day after |
| Montenegro 🇲🇪 | The 2007 law's two-day holidays with Njegoš Day from 2022, the Sunday rule, and each community's religious days as `Kind::Religious` |
| North Macedonia 🇲🇰 | The 1998 law as amended in 2007, the Sunday rule, the religious and ethnic communities' days off, Duhovden as the Friday before Pentecost |
| Serbia 🇷🇸 | The 2001 law with its 2007 and 2011 amendments: state holidays with the Sunday rule, the Orthodox days fixed, the working holidays as observances, the other confessions' days as `Kind::Religious` |
| Bosnia and Herzegovina 🇧🇦 | No state law: the Federation's taken-over 1973 law with 1 March and 25 November, Republika Srpska's 2007 law with its 9 January as the notices give it to 2026 and the religious days as `Kind::Religious`, Brčko's 2002 law and the Assembly's religious days for 2017–2026, each a region with its own Sunday rule |
| Belarus 🇧🇾 | Decree 157's ten non-working days with Radunitsa on the ninth day after the Orthodox Easter and the years of 2 January and 3 July; the working state holidays as observances |
| Luxembourg 🇱🇺 | The eleven legal holidays with Europe Day from 2019 and Good Friday as the banks' alone; the compensatory day not carried |
| Malta 🇲🇹 | Cap. 252's five national and nine public holidays; nothing moves |
| Liechtenstein 🇱🇮 | The Labour Act's thirteen days equal to Sundays; the five bank days and the two collective-agreement days as bank and observance |
| Monaco 🇲🇨 | Law 798's twelve days, six of them to the Monday after a Sunday; the Prince's Day on 19 November since 1952 |
| San Marino 🇸🇲 | The 2013 rewriting of the 1990 calendar's civil days, the religious days as the Central Bank's calendars give them, 24 and 31 December as bank days |
| Vatican City 🇻🇦 | The Governorate's and the Curia's staff regulations: canon 1246's holy days of obligation, the Ascension and Corpus Christi on their Thursdays, Holy Week to Easter Tuesday and the regulations' own days; the Pope's election anniversary and name day for 2013–2026, a gap after; a Sunday-only week |
| Andorra 🇦🇩 | Law 31/2018 art. 62 and the yearly decree's fourteen national days, Carnival on its Monday; parish days not carried |
| Moldova 🇲🇩 | Art. 111 with the two Christmases, the Orthodox Easter and the Easter of the Blajini, Europe Day from 2017, Children's Day from 2024, and Chișinău's feast as `MD-CU` |
| Bulgaria 🇧🇬 | Labour Code art. 154: the list, the Orthodox Easter, 1 November for schools, and the weekend rule of 2017 that excepts the Easter days |
| Cyprus 🇨🇾 | The Republic's list with the Orthodox Easter and Easter Tuesday as a bank holiday; no substitution |
| Estonia 🇪🇪 | The act of 1998: national holiday, public holidays and the days of national importance as observances, with the act's own amendment years; no substitution |
| Latvia 🇱🇻 | The law as in force from 2025: holidays, the two one-off days, the remembrance days as observances, and the weekend rule that reaches three holidays only |
| Lithuania 🇱🇹 | Labour Code art. 123 with the years each day became a day off; no substitution |
| Croatia 🇭🇷 | The Act with the 2020 change: Statehood Day on 30 May, 25 June and 8 October demoted, Remembrance Day new; no substitution |
| Slovakia 🇸🇰 | The state holidays, each carried as a day off to its last year as one and an observance after; no substitution |
| Slovenia 🇸🇮 | The work-free days with the years the source gives, 2 January by its two spans; no substitution |
| Iceland 🇮🇸 | The act's list with the First Day of Summer and Commerce Day by their weekday rules, the two eves as half days; no substitution |
| Hungary 🇭🇺 | With Good Friday from 2017; the annual rearrangement of working days by decree is not carried |
| Romania 🇷🇴 | Orthodox Easter and Pentecost by the Julian computus; the additions of the last decade by year, Epiphany and Saint John from 2024 |
| Russia 🇷🇺 | The New Year holidays as they grew to 1–8 January; the Government's transfer decrees for 2013–2027, with the working Saturdays they create, and article 112's carry-over of a weekend holiday computed from them; checked against the production calendar for each of those years |
| Costa Rica 🇨🇷 | Art. 148 with the unpaid days and their years, and Ley 9875's 2020–2024 Mondays as computed rules that fall silent from 2025 |
| Dominican Republic 🇩🇴 | Ley 139-97 from its text: adjacent-Monday moves, the excluded days, Restoration Day fixed in inauguration years, a Sunday 1 May to Monday |
| Guatemala 🇬🇹 | Art. 127 with two half days and Guatemala City's Assumption as `GT-GU`; the tourism law's Monday moves for Army Day, and for 1 May and 20 October only until the 2020 ruling |
| Panama 🇵🇦 | Arts. 46 and 47: the Sunday rule, Ley 70's adjacent-Monday moves for 9 January and 28 November, 20 December from 2022, Flag Day as the public sector's |
| Cuba 🇨🇺 | Ley 116 arts. 94, 97 and 100: the four commemorations and five holidays, Good Friday from 2012, Christmas from 1998, the Sunday rest moved only for 1 May and 10 October; Sunday as the weekend |
| Belize 🇧🇿 | The Government's yearly notices under Chapter 289, the Act itself unread: Sunday to Monday, the Second Schedule's two days to the nearest Monday, the Tuesday and Wednesday moves the notices show |
| Guyana 🇬🇾 | Chapter 19:07 s. 3 with "if that day is a Sunday, the following day" and the section 6 days by the yearly lists; Phagwah, Deepavali and the two Islamic days approximate |
| Haiti 🇭🇹 | Art. 275-1's five national days, the 1984 code's list to 1988, the 1989 decree's seven from 1989, and the December 2024 decree's additions from 2025, Carnival Monday from noon as bank |
| Honduras 🇭🇳 | Art. 339 with Holy Week's three days; the October days on their dates to 2013 and as the Semana Morazánica from 2015, its Wednesday from noon as bank and 2020's in November; 2014 not carried |
| El Salvador 🇸🇻 | Art. 190 with San Salvador's 3 and 5 August as `SV-SS`, Father's Day from 2013 and Mother's Day for everyone from 2016 by their decrees, 7 July 2023; nothing moves |
| Nicaragua 🇳🇮 | Art. 66's nine days and art. 67's Santo Domingo days for Managua as `NI-MN`; art. 68's "será compensado" names no day, so nothing moves |
| Bolivia 🇧🇴 | Decreto Supremo 2750 with the Sunday rule and the four days it excepts; departmental holidays and yearly bridges not carried |
| Chile 🇨🇱 | Every move a rule of its own: Ley 19.668's Mondays, Ley 20.299's Fridays, the computed 2 January and 17/20 September days, the solstice at Chile's meridian; Arica's day as `CL-AP` |
| Ecuador 🇪🇨 | Art. 65 as reformed in 2016: the moves per holiday, the weekend-only moves of the three excepted days, and the 2/3 November pair as the Government resolved it |
| Uruguay 🇺🇾 | Ley 16.805 as amended in 2001: paid holidays public, common ones bank, Tourism Week's six days, three holidays to the adjacent Monday |
| Venezuela 🇻🇪 | LOTTT art. 184 with Carnival, Holy Week and the whole days of 24 and 31 December, and the 1971 Ley de Fiestas Nacionales' five days, 12 October renamed in 2002; the declared days not carried |
| Paraguay 🇵🇾 | Ley 7544/2025 with 14 May from 2012, 29 September from 1995 and 20 June from 2026; the 2026 decrees' three Monday moves and the additional days read for 2025 and 2026, earlier decreed moves not carried |
| Jamaica 🇯🇲 | The Holidays (Public General) Act's Schedule: Sunday moves, Labour Day off a Saturday too, a Sunday Christmas giving the 26th and 27th; a Sunday Boxing Day's Monday only from the Minister's 2021 appointment |
| Trinidad and Tobago 🇹🇹 | Chap. 19:05 with section 3(2)'s next free day for a Sunday or for two holidays at once; Eid-ul-Fitr and Divali approximate; Carnival Monday and Tuesday as observances; African Emancipation Day from 2024 |
| Barbados 🇧🇧 | Cap. 352's First Schedule: the Monday after a Sunday, and the Tuesday for Emancipation Day off a Sunday or a Monday and for Christmas off a Sunday |
| Bahamas 🇧🇸 | Ch. 36's Sunday proviso and the Saturday practice, both to the next free weekday; Majority Rule Day from 2014 and National Heroes Day from 2013; no Tuesday-to-Thursday moves, which the Act does not have |
| Antigua and Barbuda 🇦🇬 | Cap. 354 by the 2005 Schedule from 2006 and the 2019 one from 2020: the Sunday moves to the next free day, Independence Day off a Saturday too, and from 2020 Christmas and V. C. Bird Day off a Saturday; the Carnival Monday and Tuesday in August |
| Dominica 🇩🇲 | Chap. 19:10's Schedule, every Sunday in it, and section 9's next free day for two holidays at once; Labour Day on the first Monday of May from the 2021 lists |
| Grenada 🇬🇩 | Cap. 25's Schedule and "the Monday immediately next following" a Sunday, nothing more; Carnival Tuesday from noon as bank until 2023, Emancipation Day on 1 August and National Heroes' Day from 2025 |
| Saint Kitts and Nevis 🇰🇳 | Cap. 23.23's Schedule with the Sunday rule and a Sunday Christmas's Monday and Tuesday; Carnival Day and Culturama Day, proclaimed each year, not carried |
| Saint Lucia 🇱🇨 | The Bank Holidays Act's Schedule and its note: a Sunday or a second holiday to the next free day, so a Sunday 1 January gives the 3rd; Thanksgiving Day on the first Monday of October, approximate |
| Saint Vincent and the Grenadines 🇻🇨 | The Prime Minister's Office's lists, the Act itself unread: the Sunday moves they show, Carnival from the lists for 2021–2026, Spiritual Baptist Liberation Day from 2025 |
| Suriname 🇸🇷 | The Besluit Vrije Dagen 1971 as S.B. 2021 no. 27 left it, the 2007 and 2012 additions by year and 25 February for 2012–2020; Holi, Divali, the Ieds and Chinese New Year approximate; nothing moves |
| Argentina 🇦🇷 | From Decreto 1584/2010 in 2011: the *inamovibles* where they fall, the *trasladables* on the decree's Mondays to 2016 and by the weekday rule of Ley 27.399 from 2018; the annual tourist holidays not carried; Holy Thursday and the days of the Jewish and Islamic faiths as observances |
| Colombia 🇨🇴 | Ley 51 de 1983 from 1984: ten holidays to the following Monday, eight where they fall |
| Ethiopia 🇪🇹 | The national and Orthodox holidays on the Ethiopian calendar (`ethiopic`), where they are kept; Fasika by the Julian computus; the Islamic days approximate |
| Ghana 🇬🇭 | The Act as amended in 2019 and 2025, from 2019: Founders' Day and Kwame Nkrumah Memorial Day by their years, Republic Day back from 2025, Shaqq Day from 2026; the year-by-year Monday declarations not carried |
| Bahrain 🇧🇭 | The Council of Ministers' fourteen days under art. 64 of Law 36/2012: three of each Eid, two of Ashura, Hijri dates approximate; nothing moves |
| Jordan 🇯🇴 | The list with four days of Eid al-Fitr and five from Arafat, Christmas for all, the Christian employees' Eastern Easter days as `Kind::Religious`, the working commemorations as observances |
| Kuwait 🇰🇼 | Art. 68 of Law 6/2010's thirteen paid holidays with Arafat and Isra and Mi'raj, Hijri dates approximate |
| Oman 🇴🇲 | Royal Decree 88/2022 as amended in 2025: the single days with a weekend day compensated, the National Day pair's one computed day, the two Eid spans with a Friday start compensated, approximate; the 2013 weekend change |
| Qatar 🇶🇦 | Emiri Decision 57/2025: the two Eid spans approximate, National Day, Sport Day, the one-day bridge from 2025, the bank days of article 4 |
| Iraq 🇮🇶 | Law 12 of 2024 from the Gazette: eleven days with Ghadir and 16 March new, Christmas for all over 2020–2023, and article 2's Christian and Yazidi days as religious, the Julian-dated ones on the Julian calendar |
| Lebanon 🇱🇧 | Decree 15215 of 2005 from the Council of Ministers' own table: both Good Fridays and the Saturday they share, two-day Eids approximate, Labour Day alone moved off a Sunday, the May commemorations on their Sundays |
| Syria 🇸🇾 | Decree 188 of 2025 for the State's workers: both Easters on their Sundays, Liberation Day from 2025 and the Revolution from 2026, Nowruz from 2026 by Decree 13, Eids of three and four days approximate; the weekend by its two eras |
| Palestine 🇵🇸 | The Council of Ministers' tables for the Government sector: the Eids with their eves, the Eastern Easter for all, the Eastern and Western Christian employees' days as `Kind::Religious`; the Samaritan table not carried |
| Libya 🇱🇾 | Law 5 of 2012's table: Arafah and three days of each Eid approximate, the two days of 2011 from 2012; the Prime Minister's yearly decisions not carried; the 2006 weekend change |
| Yemen 🇾🇪 | Law 2 of 2000: five-day Eids approximate, the five national days, article 3(b)'s days as observances; article 4's replacement day not carried; the 2013 weekend change |
| Tanzania 🇹🇿 | Cap. 35's Schedule with two days of Eid al-Fitr, section 4's Saturday-and-Sunday rule, and the two presidential days kept every year |
| Uganda 🇺🇬 | Cap. 255's list with Luwum Day from 2016 and Heroes' Day from 2001, one day of each Eid; substitutes by designation not carried |
| Zambia 🇿🇲 | Cap. 272 with the three declared days by their years, the Monday and Tuesday holidays, the Act's Sunday-to-Monday rule |
| Zimbabwe 🇿🇼 | Chapter 10:21 as gazetted for 2026, Youth Day from 2018, the Easter block fixed, the Sunday proviso past a taken Monday |
| Kenya 🇰🇪 | The Public Holidays Act's Part I with the Sunday rule of section 4; Idd-ul-Azha and Diwali as the religious days Parts II and III make them, not days off for all; the Idd days approximate |
| Morocco 🇲🇦 | The decrees' eleven fixed days and the four feasts, three of them two days, approximate; Yennayer from 2024 and Unity Day from 2026 by year |
| Botswana 🇧🇼 | Cap. 03:07's fourteen days with section 2's three provisos: Sunday to Monday, the three second days from a Monday to the Tuesday, a Saturday Botswana Day to the Monday |
| Namibia 🇳🇦 | Act 26 of 1990's twelve days, the 2004 renaming, Genocide Remembrance Day from 2025 by proclamation, and a Sunday's Monday "unless that Monday is a public holiday" |
| Mauritius 🇲🇺 | The Act as amended in 2015 and 2019: the First Schedule's fixed and notified days, the latter approximate on the Chinese, Hijri and Hindu rules, and the Assumption or All Saints alternating from 2016 as computed rules |
| Malawi 🇲🇼 | Cap. 18:05's Schedule with Boxing Day from the yearly lists, and section 4's next free day for a Saturday or Sunday, the Saturday after Good Friday excepted |
| Rwanda 🇷🇼 | Presidential Order 54/01 of 2017 from 2017: fifteen days, Umuganura on the first Friday of August, the Eids approximate; article 4's next working day for a weekend or doubled holiday, one day for two, never past a taken day, 7 April never; the Ministry's two extra days of 2022–2023 |
| Burundi 🇧🇮 | Decree 100/150 of 2021's fourteen days, 8 June from 2021, the Eids approximate; article 4's discretionary Sunday move not carried |
| Seychelles 🇸🇨 | Cap. 190 as amended in 1994, 2014 and 2017: National Day from 1994 and the 2015 names, Liberation Day to 2016 and Easter Monday from 2017, section 4's next free day for a Sunday; the President's orders not carried |
| Mozambique 🇲🇿 | Article 105 of Lei 13/2023's nine days, where they fall — paragraph 6's Sunday sentence is published without its consequence; the tolerâncias de ponto not carried |
| Algeria 🇩🇿 | Law 63-278 as amended: the five civil days, Yennayer from 2018, the Eids two days until 2022 and three from law 23-10 of 2023, approximate; the Christian and Jewish community days as religious; the weekend by its three eras |
| Tunisia 🇹🇳 | Decree 2021-223's list for the public service, Aïd el-Fitr three days and Aïd el-Idha two, approximate; the decrees of 1961 to 2021 by their years |
| Senegal 🇸🇳 | Law 74-52 with Easter and Pentecost on their Sundays, the Monday after a Sunday Korité or Tabaski only, and the Grand Magal from 2012 |
| Côte d'Ivoire 🇨🇮 | Decree 96-205 as rewritten in 2011: twelve days and the day after a Sunday national holiday, Labour Day, Aïd el-Fitr, Christmas or Tabaski, from 2011; the two lendemain days approximate |
| Benin 🇧🇯 | Law 90-019's twelve days with article 3's national days as observances; the traditional religions on 10 January from 1998, and on the second Friday of January with the Thursday before from 2025; the Islamic days approximate; nothing moves |
| Burkina Faso 🇧🇫 | Law 079-2015/CNT from 2016 with its Sunday-to-Monday rule, Easter's Monday by it, and 15 May from 2024; the eleven days of the 2026 law with the rest as commemorations, a Sunday's Monday in 2026 a gap |
| Cabo Verde 🇨🇻 | Law 16/IV/91's seven days and Good Friday, Children's Day as a school day; 13 January from 2020 with 1992–2019 a gap; nothing moves |
| Guinea 🇬🇳 | Decree D/2022/0526 as the press reproduced it: twelve days, the lendemain days approximate, and the next working day for a weekend Independence Day, New Year's Day or Aïd el-Fitr from 2023 |
| Mali 🇲🇱 | Law 05-040's eleven days with the Birth and Baptism of Maouloud approximate, 14 January from 2023; article 2's declared extra days and Achoura not carried; nothing moves |
| Chad 🇹🇩 | Decree 97-413 of 1997: article 1's seven days off unpaid and unmoved, article 2's paid days with the Monday after a Sunday from 1997, 8 March from 2019 by decree 273 as the press gives it; 11 August 2010, replaced by a decree not read, a gap; the Islamic days approximate |
| Mauritania 🇲🇷 | Law 92-018's national day and seven legal holidays, the four Islamic days one day each and approximate, "Mouharram" read as its first day; the President's declared days not carried; nothing moves; Friday–Saturday weekend to 2013, Saturday–Sunday from 2014 |
| Djibouti 🇩🇯 | Arrêté 77-347 of 1977 as its 1981 rectificatif quotes it, from 1978: two days of each Eid, the first of Muharram, the Mouloud and Isra and Mi'raj approximate, Christmas by arrêté 77-609, Independence two days from 1981 and 28 June 1980 a gap; nothing moves; the Friday weekend of the Labour Code |
| Cameroon 🇨🇲 | Law 73/5 of 1973: four civil and six religious days, the Eids approximate, and the next day for a civil holiday on a Sunday or on another holiday, from 1974; the President's declared days not carried |
| Republic of the Congo 🇨🇬 | Law 2-94 of 1994's nine days with the Easter and Pentecost Mondays; nothing moves; 28 November and the Minister's declared days not carried |
| Democratic Republic of the Congo 🇨🇩 | Ordinances 14-010 and 23-042: 6 April from 2023, a Sunday holiday to the Saturday from 2014 to 2025, the Minister's moves for 2025 and 2026 as the communiqués give them and a gap from 2027 |
| Angola 🇦🇴 | Law 10/11 as amended by law 11/18: 23 March from 2019, the Sunday rule to 2017 and the Monday or Friday bridges from 28 September 2018; the celebration dates as observances |
| Pakistan 🇵🇰 | The state holidays, the Hijri ones approximate; Iqbal Day only in the years it was a holiday; the notification's extra Eid days not carried |
| Peru 🇵🇪 | The sixteen days of the 2026 list from 2024, where they fall; the laws of the four recent additions not read, so nothing before is stated |
| Ukraine 🇺🇦 | The 2023 list — 8 May, 15 July, 1 October, Christmas on 25 December alone — with the earlier dates by year; the next-working-day rule of article 67; the martial-law suspension of days off is noted, not modelled |
| Thailand 🇹🇭 | The Bank of Thailand's list with the Monday for a weekend holiday; Makha, Visakha and Asalha Bucha and Khao Phansa exact on the Thai lunar calendar (`thai-lunar`), a month later in an adhikamāsa year, for 1992–2027, the years Thailand has published, and gaps outside them; the Royal Ploughing Ceremony not carried |
| Vietnam 🇻🇳 | Article 112 of the 2019 Labour Code, Hùng Kings' day on the `vietnamese` calendar, and article 111(3)'s next working day for a fixed holiday on a weekend; Tết, the second National Day holiday and the working days swapped for a Saturday from the civil-service notices for 2021 to 2026, the Saturdays worked as working days; Vietnamese Culture Day from 2026 |
| Brunei 🇧🇳 | The Prime Minister's Office's circulars for 2023–2026: New Year, National Day, Armed Forces Day, the Sultan's Birthday and Christmas fixed, Chinese New Year on `chinese`, Isra' and Mi'raj, Awal Ramadhan, Nuzul Al-Qur'an, the three days of Hari Raya Aidil Fitri, Hari Raya Aidil Adha, the Islamic New Year and the Prophet's Birthday approximate; the Friday–Sunday weekend, a Friday or Sunday holiday replaced by the next working day as each circular names it |
| Timor-Leste 🇹🇱 | Law 10/2005 art. 2 as amended by Laws 3/2016 and 10/2023: Veterans' Day from 2017, 7 December renamed Memorial Day and National Heroes' Day on 31 December from 2016, National Women's Day from 2023; Good Friday and Corpus Christi by the Western computus, Idul Fitri and Idul Adha approximate; the commemorative dates not carried; the Labour Code's Sunday weekend, nothing moved |
| Maldives 🇲🇻 | Section 97 of the Employment Act, dated as the Maldives Monetary Authority's lists for 2016–2026: New Year, Labour Day, Independence Day's two days, Victory Day and Republic Day fixed; the first of Ramadan, Eid al-Fitr's three days, Hajj Day, Eid al-Adha's four (three in 2016), the Islamic New Year, National Day, the Prophet's Birthday and the Day the Maldives Embraced Islam approximate; the Friday–Saturday weekend, nothing moved off it; the President's declared government holidays not carried |
| Indonesia 🇮🇩 | The national days of the three ministries' joint decree: Chinese New Year on `chinese` from 2003, Labour Day from 2014, Pancasila Day from 2017, Vesak on the Chinese calendar and the Islamic days, all approximate; Nyepi and the *cuti bersama* not carried |
| Spain 🇪🇸, Italy 🇮🇹, Netherlands 🇳🇱, Poland 🇵🇱, Türkiye 🇹🇷, Egypt 🇪🇬, Nigeria 🇳🇬, South Africa 🇿🇦, Singapore 🇸🇬, Malaysia 🇲🇾, Philippines 🇵🇭, Switzerland 🇨🇭, Austria 🇦🇹, Belgium 🇧🇪, Sweden 🇸🇪, Norway 🇳🇴, Denmark 🇩🇰, Finland 🇫🇮, Portugal 🇵🇹, Greece 🇬🇷, Czechia 🇨🇿 | Core national list |

**Partial**

| Country | Note |
| --- | --- |
| India 🇮🇳 | The three national holidays and the gazetted list — Holi, Ram Navami, Mahavir Jayanti, Buddha Purnima, Janmashtami, Dussehra, Diwali and Guru Nanak's Birthday computed on `hindu-lunar`, the Hijri days approximate |
| Myanmar 🇲🇲 | The full moons, National Day, the Kayin New Year and Deepavali on the Burmese calendar (`burmese`), Thingyan from the calendar's own akya and atat moments; the gazette's annual extensions not carried; Eid al-Adha approximate |
| Nepal 🇳🇵 | The public holidays on a fixed date, from the Ministry of Home Affairs' notices for 2082 and 2083 BS: New Year, Republic Day, Constitution Day, Prithvi Jayanti, Maghe Sankranti, Martyrs' Day and Democracy Day on `bikram-sambat`, and Labour Day, Christmas and Women's Day on their Gregorian dates, as the notices give them. The festivals on `hindu-lunar` read at Kathmandu — Buddha Jayanti, Janai Purnima, Janmashtami, Ghatasthapana, Dashain and Tihar for as many days as each year's notice gives, Dhanya Purnima, Sonam and Gyalpo Lhosar, Maha Shivaratri — with Tamu Lhosar on Pus 15; the part of the day each tithi holds is fitted to the notices of 2080–2083 BS, so they are approximate, as are the two Eids. Chhath, which no single rule fits, and the holidays for one community or region are not carried. The one-day weekend until April 2026 |
| Papua New Guinea 🇵🇬 | Chapter 321's own days — New Year, the Easter weekend, Remembrance Day, Christmas and Boxing Day — with the Sunday rule; Independence Day, the Sovereign's Birthday and the other gazetted days not carried |
| Sri Lanka 🇱🇰 | The Holidays Act orders for 2023–2027, every day of each: the full-moon Poya days, *adhi* ones included, Thai Pongal, Maha Shivarathri, the Sinhala and Tamil New Year, the three Muslim days and Deepavali as the gazettes date them, a year beyond reported as a gap; Independence Day, May Day, Christmas and Good Friday by rule. No computed rule reproduces the Poya days, so none is used |
| Bangladesh 🇧🇩 | The general and executive-order holidays of the Ministry of Public Administration's notifications for 2025 and 2026: the civil days on their Gregorian dates, July Mass Uprising Day from 2025, Pohela Boishakh and the hill districts' Chaitra Sankranti (from 2026) on `bangladeshi`; the two Eids with the executive order's days around them, Eid-e-Miladunnabi, Ashura, Shab-e-Barat, Shab-e-Qadr and Jumatul Bida approximate; Janmashtami, the Durga Puja's Navami and Bijoya Dashami and Buddha Purnima as the notifications date them, a year beyond reported as a gap; the Friday–Saturday weekend, nothing moved off it; the optional holidays not carried |
| Mongolia 🇲🇳 | The Law on Public Holidays and Days of Observance, art. 4.1, and the Labour Law, art. 97.1: New Year, Women's Day, Children's Day, Naadam's six days, Republic Day from 2016 and 29 December from 2011; Tsagaan Sar, Buddha's Birthday and Chinggis Khaan Day are on the Mongolian lunar calendar, which the crate does not have, and are reported as gaps in every year; nothing moves off the Saturday–Sunday weekend |
| Madagascar 🇲🇬 | The yearly decrees for 2023–2026, the same thirteen days from 2023, 8 March for women only as an observance, the Eids approximate, 23 April 2025 by decree; 2024's election days and, from 2026, the Malagasy New Year and the day of culture are reported as gaps |
| Lesotho 🇱🇸 | Act 7 of 1995's eight unchanged days from 1996, nothing moved; Africa's Heroes' Day, the King's Birthday on 17 July and Boxing Day from the Embassy's 2026 list, the notices that changed the Schedule unread, so 1996–2025 are a gap for them |
| Cambodia 🇰🇭 | The Royal Government's sub-decrees for 2025, 2026 and 2027: the eleven days on a fixed date in all three, and Khmer New Year, Visak Bochea, the Royal Ploughing Ceremony, Pchum Ben and the Water Festival as the sub-decrees date them, a year beyond reported as a gap; the Labour Law's Sunday weekend, nothing moved off it under the 2021 amendment |
| Laos 🇱🇦 | Decree 386/ລບ of 2017, art. 4: New Year, Women's Day, Labour Day and National Day fixed, Lao New Year from the Prime Minister's Office's notices for 2024–2026, a year beyond reported as a gap; the Saturday–Sunday weekend with art. 3's compensatory day on the next working day from 2019; the Lao Women's Union's day, off for women alone, not carried |
| Bhutan 🇧🇹 | The Ministry of Home Affairs' government holiday lists for 2025 and 2026: the King's birthday, the Third and Fourth Druk Gyalpo's birthdays, the Coronation and National Day fixed; Losar, the Buddhist days, the Traditional Day of Offering, Dassain, the Winter Solstice and the Blessed Rainy Day as the lists date them, a year beyond reported as a gap; the Thimphu-only days and the district tshechus not carried; the Saturday–Sunday weekend, nothing moved off it |
| Fiji 🇫🇯 | The Ministry of Information's yearly lists for 2019–2026, every day of each as listed — Constitution Day to 2022, Girmit Day and Ratu Sir Lala Sukuna Day from 2023, the Prophet's Birthday and Diwali, and the weekend moves the lists make in some years and not others — a year outside reported as a gap; Good Friday, Easter Saturday and Easter Monday by rule from Cap. 101's Schedule |
| Kiribati 🇰🇮 | The Beretitenti's orders under Cap. 81 for 2025 (revised) and 2026, every day of each as ordered, the "in honour of" days included, a year outside reported as a gap; Good Friday and Easter Monday by rule; the Schedule days an order keeps without listing not carried |

**Planned** — the United Nations member states without a table,
Afghanistan, the Central African Republic, the Comoros, Equatorial Guinea,
Eritrea, Eswatini, Gabon, the Gambia, Guinea-Bissau, Liberia, Niger, North
Korea, São Tomé and Príncipe, Sierra Leone, Somalia, South Sudan, Sudan and
Togo, plus the subdivisions that have their own legal holidays. Tracked as
one issue per country so that each lands with a citable source.

Where a country's dates are announced each year by decree rather than fixed
in law, its table carries the notices that were read and reports any other
year as a gap, as Sri Lanka's, Cambodia's and Fiji's do, or predicts the date
with an explicit `Approximate` flag, as every Hijri-dated entry does. A caller
holding an announced table can build a `RuleSet` of their own, which the
engine takes on the same terms as its own.

## Stage 3 — Beyond public holidays

| Category | Status |
| --- | --- |
| Business-day calculation (weekend rules by country, including Friday–Saturday, Thursday–Friday and one-day weekends) | Done |
| Trading-day calendars for major exchanges | In progress — `hc_holiday::exchanges`, rule sets keyed by Market Identifier Code from each exchange's own published calendar ([ADR 0008](adr/0008-exchange-calendars-are-rule-sets.md)): New York (`XNYS`) and Nasdaq (`XNAS`), Toronto (`XTSE`), Frankfurt on Xetra (`XETR`), Sydney (`XASX`), Euronext's Amsterdam, Brussels, Dublin, Lisbon, Milan, Oslo and Paris (`XAMS`, `XBRU`, `XDUB`, `XLIS`, `XMIL`, `XOSL`, `XPAR`) from its 2021–2026 tables, B3 in São Paulo (`BVMF`) from its 2021–2026 market calendars, and Nasdaq's Copenhagen, Stockholm, Helsinki and Iceland (`XCSE`, `XSTO`, `XHEL`, `XICE`) from its Nordic calendar for 2025–2027; Tokyo (`XJPX`), Hong Kong (`XHKG`), Seoul (`XKRX`, checked against its closure lists for 2009–2029) Shanghai (`XSHG`, on China's annual arrangements, checked against its notices for 2014–2026) and Taipei (`XTAI`, checked against its schedules for 2023–2026) as their countries' tables plus the exchanges' own days, through `RuleSet::includes`; SIX in Zurich (`XSWX`) from its market holidays for 2026 and 2027, 1 August and 26 December not shown by either year and not carried; London (`XLON`) as the bank holidays of England and Wales, the region its inclusion of the United Kingdom's table names, with its two half days, from its business-days table for August 2026 to January 2029; Johannesburg (`XJSE`, South Africa's table plus the declared days, from its calendars for 2024–2026, its noon closes carried for 2023–2025), Mexico City (`XMEX`, Mexico's table plus the CNBV's four closing days, from its lists for 2019–2026), Warsaw (`XWAR`, Poland's table plus three days, from its lists for 2019–2027) and NZX (`XNZE`, New Zealand's table, from its memos for 2022–2026) through `RuleSet::includes`; Vienna (`XWBO`) from its lists for 2019–2027 and Madrid (`XMAD`) from BME's calendars for 2023–2026; Moscow (`MISX`, which trades on most of Russia's days off, from its announcements for 2023–2026), Tel Aviv (`XTAE`, Sunday–Thursday to 2025 and Monday–Friday from 2026, from its schedules for 2024–2027), Riyadh (`XSAU`, from its announcements for 2023–2026) and Istanbul (`XIST`, the Bayram days from its tables for 2019–2026) tabulated as listed, a year outside them a gap; Shenzhen (`XSHE`, China's table on the Shanghai rules, checked against its notices for 2015–2026) through `RuleSet::includes`; Bangkok (`XBKK`, from its holiday pages for 2022–2027), the National Stock Exchange of India and BSE (`XNSE`, `XBOM`, one list from their circulars and notices for 2020–2026, the Muhurat and weekend sessions included), Singapore (`XSES`, the Ministry of Manpower's holidays for 2020–2026 under SGX's stated rule, with SGX's half days), Kuala Lumpur (`XKLS`, from its calendar pages for 2020–2026), Jakarta (`XIDX`, from its calendars for 2020–2022 and 2024–2026, 2023 a gap) and Manila (`XPHS`, from its memoranda for 2020–2026) tabulated as listed, a year outside them a gap |
| School terms | Out of scope — too local and too volatile |
| Name days and the sanctorale | In progress — as named authorities rather than one list, since the Roman calendar was recast in 1969 and the Swedish *namnsdagslängd* was revised in 1901, 1993 and 2001 ([policy.md](policy.md) §5). The General Roman Calendar is done (`roman-general`, `hc_holiday::roman_calendar`): the 2002 Missal's calendar with each of the Holy See's decrees since, 2014–2025, every celebration with its rank, precedence not applied. National and diocesan calendars and the name-day lists are not yet carried |
| Anniversaries and commemorations without a day off | Partial — `Kind::Observance` exists and a handful of tables use it. A general commemoration list is **out of scope** under [policy.md](policy.md) §10: no authority defines which commemorations belong, so its coverage could never be stated. Individual ones enter through whichever authority proclaims them |

## Adding a country

1. Add a `CountryRules` entry with its rules and a `sources_checked` date,
   and register it in `countries::ALL`, whose length a test holds to the
   count the crate README states.
2. Cite the statute or the government gazette in a comment. A holiday without a
   source is a rumour.
3. Test at least five specific dates across at least two different years,
   including one that exercises the substitution rule — or, where the country
   has none, one that proves a weekend holiday stays where it falls.
4. Move its row to **Done**.
