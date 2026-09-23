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
NthWeekday { month, n, day }  US Thanksgiving, 4th Thursday of November
                              Japan's 成人の日, 2nd Monday of January
LastWeekday { month, day }    UK Spring Bank Holiday, last Monday of May
WeekdayOnOrAfter { m, d, wd } Midsommardagen, the Saturday on or after 20 June
WeekdayOnOrBefore { m, d, wd} Buß- und Bettag, the Wednesday before 23 Nov
FixedInCalendar { cal, m, d } Eid al-Fitr, 1 Shawwal in islamic-umalqura
                              Rosh Hashanah, 1 Tishrei in hebrew
SolarTerm { term, meridian }  春分の日 / 秋分の日, the equinoxes at UTC+9
                              清明節, at the Beijing meridian
EasterRelative { comp, offs } Good Friday (-2), Easter Monday (+1)
                              Ash Wednesday (-46), Pentecost (+49)
LunarPhase { phase, after }   the first full moon on or after a fixed date
Tithi { m, tithi, prevails }  Rāma Navamī, Chaitra śukla 9 at midday
Sankranti { sign, ayanamsa }  Makar Sankranti, the Sun's entry into Makara
Nakshatra { n, sign, tithi }  Thaipusam, Puṣya in Thai
Offset { base, days }         除夕, Seollal's eve
MovedByWeekday { base, moves } a Monday-holiday law: Argentina's trasladables, Colombia's Ley Emiliani
Tabulated { fn, first, last } Matariki, gazetted through a stated last year
Computed(fn)                  the few that really are bespoke
```

`WeekdayOnOrAfter`, `WeekdayOnOrBefore`, `Offset` and `Tabulated` were added
while writing the national tables; each is still a pure rule value. The first
three each removed a `Computed` that would otherwise have been needed, and
the fourth gives a published table a last year, so that running out of table
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
- `Region` — subdivision scoping, used today for German *Länder*, Canadian
  provinces, Australian states and territories, Spanish autonomous
  communities and the single US entry `US-DC`. Swiss cantons and US states
  are *not* modelled: the vocabulary carries them, the tables do not.
- `Kind` — public holiday, bank holiday, school holiday, observance without a
  day off, religious day of obligation, and a weekend day made a working day
  (China's 调休上班).
- `WeekendPolicy` — which days are the weekend, over stated years, because
  Saudi Arabia (2013), the United Arab Emirates (2022) and Nepal (2026) all
  changed theirs inside living memory.

Because the rule set is data, a caller can supply their own table — a company
calendar, a school year, a fictional setting — and get the same engine.

Only **five** statutes in the whole crate are `Computed`: Ireland's St
Brigid's Day, the Dutch royal day (under two monarchs), US Inauguration Day,
Mexico's presidential handover and Israel's Yom HaAtzmaut. New Zealand's
Matariki is `Tabulated`, with the last gazetted year stated. Japan needs
none.

## What the engine will not do

- **It will not claim a Hijri holiday is exact.** Eid dates depend on lunar
  crescent visibility decided per country, sometimes on the night before. The
  library returns the Umm al-Qura or tabular computation and flags it
  `Approximate`. It is a good prediction, not an announcement.
- **It will not invent substitution rules it cannot cite.** A country's
  weekend-substitution behaviour is law, and laws differ. Most of the tables
  below carry no substitution policy at all, because their countries have
  none in calendar-expressible form; `supported.md` shows which.
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
  absent because the Balinese Saka calendar it is dated in does not exist in
  the crate yet. India's Hindu, Sikh and Jain gazetted holidays and
  Singapore's and Malaysia's Deepavali were absent for the same reason until
  `hindu-lunar` and `nanakshahi` arrived, and are computed now.
- **It will not pretend a holiday list is current.** Every country table
  carries the date its sources were checked.
- **It will not answer outside the span it evaluated.** Business-day
  arithmetic that walks off the end of a `HolidayCalendar` returns `None`.

## Stage 1 — Religious and traditional cycles

> A rule can be dated in any calendar, not a fixed list of them.
> `CalendarSystem` was a closed enum of eight, so a feast kept in the
> Ethiopic, Coptic, Solar Hijri or Badíʿ calendar could not be expressed —
> not "was not yet", *could not be*. It is now an open struct keyed to a
> registry identifier, and the Ethiopian Orthodox table is there to prove
> it rather than to describe it.

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
| Brazil 🇧🇷 | Including Consciência Negra from 2024 |
| Mexico 🇲🇽 | Including the 2006 Monday reform and the six-yearly presidential handover |
| Saudi Arabia 🇸🇦 | Umm al-Qura based, with the 2013 weekend change |
| United Arab Emirates 🇦🇪 | With the 2022 move to a Saturday–Sunday weekend |
| Israel 🇮🇱 | Hebrew-dated and therefore exact, with Yom HaAtzmaut's Sabbath-avoidance rule |
| Iran 🇮🇷 | Civil holidays on the astronomical `persian` calendar, so exact — Nowruz 1404 on 21 March 2025, where the arithmetic cycle says the 20th; the lunar Hijri days flagged `Approximate`, since Iran declares them on its own sighting; Friday weekend |
| Albania 🇦🇱 | Law 7651's fifteen holidays with both Easters, approximate Eids and the weekend rule that gives each holiday its own working day after |
| Montenegro 🇲🇪 | The 2007 law's two-day holidays with Njegoš Day from 2021, the Sunday rule, and each community's religious days as `Kind::Religious` |
| North Macedonia 🇲🇰 | The 1998 law as amended in 2007, the Sunday rule, the religious and ethnic communities' days off, Duhovden as the Friday before Pentecost |
| Serbia 🇷🇸 | The 2001 law with its 2007 and 2011 amendments: state holidays with the Sunday rule, the Orthodox days fixed, the working holidays as observances, the other confessions' days as `Kind::Religious` |
| Belarus 🇧🇾 | Decree 157's ten non-working days with Radunitsa on the ninth day after the Orthodox Easter and the years of 2 January and 3 July; the working state holidays as observances |
| Luxembourg 🇱🇺 | The eleven legal holidays with Europe Day from 2019 and Good Friday as the banks' alone; the compensatory day not carried |
| Malta 🇲🇹 | Cap. 252's five national and nine public holidays; nothing moves |
| Liechtenstein 🇱🇮 | The Labour Act's thirteen days equal to Sundays; the five bank days and the two collective-agreement days as bank and observance |
| Monaco 🇲🇨 | Law 798's twelve days, six of them to the Monday after a Sunday; the Prince's Day on 19 November since 1952 |
| San Marino 🇸🇲 | The 2013 rewriting of the 1990 calendar's civil days, the religious days as the Central Bank's calendars give them, 24 and 31 December as bank days |
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
| Bolivia 🇧🇴 | Decreto Supremo 2750 with the Sunday rule and the four days it excepts; departmental holidays and yearly bridges not carried |
| Chile 🇨🇱 | Every move a rule of its own: Ley 19.668's Mondays, Ley 20.299's Fridays, the computed 2 January and 17/20 September days, the solstice at Chile's meridian; Arica's day as `CL-AP` |
| Ecuador 🇪🇨 | Art. 65 as reformed in 2016: the moves per holiday, the weekend-only moves of the three excepted days, and the 2/3 November pair as the Government resolved it |
| Uruguay 🇺🇾 | Ley 16.805 as amended in 2001: paid holidays public, common ones bank, Tourism Week's six days, three holidays to the adjacent Monday |
| Jamaica 🇯🇲 | The Holidays (Public General) Act's Schedule: Sunday moves, Labour Day off a Saturday too, a Sunday Christmas giving the 26th and 27th; a Sunday Boxing Day's Monday only from the Minister's 2021 appointment |
| Trinidad and Tobago 🇹🇹 | Chap. 19:05 with section 3(2)'s next free day for a Sunday or for two holidays at once; Eid-ul-Fitr and Divali approximate; Carnival Monday and Tuesday as observances; African Emancipation Day from 2024 |
| Barbados 🇧🇧 | Cap. 352's First Schedule: the Monday after a Sunday, and the Tuesday for Emancipation Day off a Sunday or a Monday and for Christmas off a Sunday |
| Bahamas 🇧🇸 | Ch. 36's Sunday proviso and the Saturday practice, both to the next free weekday; Majority Rule Day from 2014 and National Heroes Day from 2013; no Tuesday-to-Thursday moves, which the Act does not have |
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
| Algeria 🇩🇿 | Law 63-278 as amended: the five civil days, Yennayer from 2018, the Eids two days until 2022 and three from law 23-10 of 2023, approximate; the Christian and Jewish community days as religious; the weekend by its three eras |
| Tunisia 🇹🇳 | Decree 2021-223's list for the public service, Aïd el-Fitr three days and Aïd el-Idha two, approximate; the decrees of 1961 to 2021 by their years |
| Senegal 🇸🇳 | Law 74-52 with Easter and Pentecost on their Sundays, the Monday after a Sunday Korité or Tabaski only, and the Grand Magal from 2012 |
| Côte d'Ivoire 🇨🇮 | Decree 96-205 as rewritten in 2011: twelve days and the day after a Sunday national holiday, Labour Day, Aïd el-Fitr, Christmas or Tabaski, from 2011; the two lendemain days approximate |
| Pakistan 🇵🇰 | The state holidays, the Hijri ones approximate; Iqbal Day only in the years it was a holiday; the notification's extra Eid days not carried |
| Peru 🇵🇪 | The sixteen days of the 2026 list from 2024, where they fall; the laws of the four recent additions not read, so nothing before is stated |
| Ukraine 🇺🇦 | The 2023 list — 8 May, 15 July, 1 October, Christmas on 25 December alone — with the earlier dates by year; the next-working-day rule of article 67; the martial-law suspension of days off is noted, not modelled |
| Thailand 🇹🇭 | Including the Buddhist lunar holidays, flagged `Approximate` |
| Vietnam 🇻🇳 | Article 112 of the 2019 Labour Code, Hùng Kings' day on the `vietnamese` calendar, and article 111(3)'s next working day for a fixed holiday on a weekend; Tết, the second National Day holiday and the working days swapped for a Saturday from the civil-service notices for 2021 to 2026, the Saturdays worked as working days; Vietnamese Culture Day from 2026 |
| Indonesia 🇮🇩 | |
| Spain 🇪🇸, Italy 🇮🇹, Netherlands 🇳🇱, Poland 🇵🇱, Türkiye 🇹🇷, Egypt 🇪🇬, Nigeria 🇳🇬, South Africa 🇿🇦, Singapore 🇸🇬, Malaysia 🇲🇾, Philippines 🇵🇭, Switzerland 🇨🇭, Austria 🇦🇹, Belgium 🇧🇪, Sweden 🇸🇪, Norway 🇳🇴, Denmark 🇩🇰, Finland 🇫🇮, Portugal 🇵🇹, Greece 🇬🇷, Czechia 🇨🇿 | Core national list |

**Partial**

| Country | Note |
| --- | --- |
| India 🇮🇳 | The three national holidays and the gazetted list — Holi, Ram Navami, Mahavir Jayanti, Buddha Purnima, Janmashtami, Dussehra, Diwali and Guru Nanak's Birthday now computed on `hindu-lunar`, the Hijri days approximate |
| Myanmar 🇲🇲 | The full moons, National Day, the Kayin New Year and Deepavali on the Burmese calendar (`burmese`), Thingyan from the calendar's own akya and atat moments; the gazette's annual extensions not carried; Eid al-Adha approximate |
| Nepal 🇳🇵 | The public holidays on a fixed date, from the Ministry of Home Affairs' notices for 2082 and 2083 BS: New Year, Republic Day, Constitution Day, Prithvi Jayanti, Maghe Sankranti, Martyrs' Day and Democracy Day on `bikram-sambat`, and Labour Day, Christmas and Women's Day on their Gregorian dates, as the notices give them. The festivals on `hindu-lunar` read at Kathmandu — Buddha Jayanti, Janai Purnima, Janmashtami, Ghatasthapana, Dashain and Tihar for as many days as each year's notice gives, Dhanya Purnima, Sonam and Gyalpo Lhosar, Maha Shivaratri — with Tamu Lhosar on Pus 15; the part of the day each tithi holds is fitted to the notices of 2080–2083 BS, so they are approximate, as are the two Eids. Chhath, which no single rule fits, and the holidays for one community or region are not carried. The one-day weekend until April 2026 |
| Sri Lanka 🇱🇰 | The Holidays Act orders for 2023–2027, every day of each: the full-moon Poya days, *adhi* ones included, Thai Pongal, Maha Shivarathri, the Sinhala and Tamil New Year, the three Muslim days and Deepavali as the gazettes date them, a year beyond reported as a gap; Independence Day, May Day, Christmas and Good Friday by rule. No computed rule reproduces the Poya days, so none is used |
| Bangladesh 🇧🇩 | The general and executive-order holidays of the Ministry of Public Administration's notifications for 2025 and 2026: the civil days on their Gregorian dates, July Mass Uprising Day from 2025, Pohela Boishakh and the hill districts' Chaitra Sankranti (from 2026) on `bangladeshi`; the two Eids with the executive order's days around them, Eid-e-Miladunnabi, Ashura, Shab-e-Barat, Shab-e-Qadr and Jumatul Bida approximate; Janmashtami, the Durga Puja's Navami and Bijoya Dashami and Buddha Purnima as the notifications date them, a year beyond reported as a gap; the Friday–Saturday weekend, nothing moved off it; the optional holidays not carried |
| Mongolia 🇲🇳 | The Law on Public Holidays and Days of Observance, art. 4.1, and the Labour Law, art. 97.1: New Year, Women's Day, Children's Day, Naadam's six days, Republic Day from 2016 and 29 December from 2011; Tsagaan Sar, Buddha's Birthday and Chinggis Khaan Day are on the Mongolian lunar calendar, which the crate does not have, and are reported as gaps in every year; nothing moves off the Saturday–Sunday weekend |

**Planned** — every remaining UN member state and observer, plus the
subdivisions that have their own legal holidays. Tracked as one issue per
country so that each lands with a citable source.

**Researching** — countries whose holiday dates are announced annually by
decree rather than fixed in law (much of the Gulf, parts of South Asia). These
are supported as *predictions* with an explicit `Approximate` flag; the hook
for supplying an announced table is simply to build a `RuleSet` of your own,
which the engine takes on the same terms as its own.

## Stage 3 — Beyond public holidays

| Category | Status |
| --- | --- |
| Business-day calculation (weekend rules by country, including Friday–Saturday, Thursday–Friday and one-day weekends) | Done |
| Trading-day calendars for major exchanges | In progress — `hc_holiday::exchanges`, rule sets keyed by Market Identifier Code from each exchange's own published calendar ([ADR 0008](adr/0008-exchange-calendars-are-rule-sets.md)): New York (`XNYS`) and Nasdaq (`XNAS`), Toronto (`XTSE`), Frankfurt on Xetra (`XETR`), Sydney (`XASX`), Euronext's Amsterdam, Brussels, Dublin, Lisbon, Milan, Oslo and Paris (`XAMS`, `XBRU`, `XDUB`, `XLIS`, `XMIL`, `XOSL`, `XPAR`) from its 2021–2026 tables, B3 in São Paulo (`BVMF`) from its 2021–2026 market calendars, and Nasdaq's Copenhagen, Stockholm, Helsinki and Iceland (`XCSE`, `XSTO`, `XHEL`, `XICE`) from its Nordic calendar for 2025–2027; Tokyo (`XJPX`), Hong Kong (`XHKG`), Seoul (`XKRX`, checked against its closure lists for 2009–2029) Shanghai (`XSHG`, on China's annual arrangements, checked against its notices for 2014–2026) and Taipei (`XTAI`, checked against its schedules for 2023–2026) as their countries' tables plus the exchanges' own days, through `RuleSet::includes`; SIX in Zurich (`XSWX`) from its market holidays for 2026 and 2027, 1 August and 26 December not shown by either year and not carried; London (`XLON`) as the bank holidays of England and Wales, the region its inclusion of the United Kingdom's table names, with its two half days, from its business-days table for August 2026 to January 2029 |
| School terms | Out of scope — too local and too volatile |
| Name days and the sanctorale | In progress — as named authorities rather than one list, since the Roman calendar was recast in 1969 and the Swedish *namnsdagslängd* was revised in 1901, 1993 and 2001 ([policy.md](policy.md) §5). The General Roman Calendar is done (`roman-general`, `hc_holiday::roman_calendar`): the 2002 Missal's calendar with each of the Holy See's decrees since, 2014–2025, every celebration with its rank, precedence not applied. National and diocesan calendars and the name-day lists are not yet carried |
| Anniversaries and commemorations without a day off | Partial — `Kind::Observance` exists and a handful of tables use it. A general commemoration list is **out of scope** under [policy.md](policy.md) §10: no authority defines which commemorations belong, so its coverage could never be stated. Individual ones enter through whichever authority proclaims them |

## Adding a country

1. Add a `CountryRules` entry with its rules and a `sources_checked` date.
2. Cite the statute or the government gazette in a comment. A holiday without a
   source is a rumour.
3. Test at least five specific dates across at least two different years,
   including one that exercises the substitution rule — or, where the country
   has none, one that proves a weekend holiday stays where it falls.
4. Move its row to **Done**.
