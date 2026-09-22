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
Offset { base, days }         除夕, Seollal's eve, Tết's first days
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
  day off, religious day of obligation.
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
  Lunar New Year makeup days, Vietnam's Tết span, Thailand's Songkran makeup
  days and Indonesia's *cuti bersama* are decided year by year by a ministry.
  The statutory days are listed; the bridging days are not.
- **It will not tabulate what it cannot compute.** India's Hindu, Sikh and
  Jain gazetted holidays, Singapore's and Malaysia's Deepavali and Indonesia's
  Nyepi are absent because the calendars they are dated in do not exist in
  `hc-calendars-lunar` yet.
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
| Hindu | Makar Sankranti, Maha Shivaratri, Holika Dahan and Holi, Ugadi, Rama Navami, Mahavir Jayanti, Mesha Sankranti, Akshaya Tritiya, Buddha Purnima, Guru Purnima, Raksha Bandhan, Krishna Janmashtami, Ganesh Chaturthi, Navaratri, Durga Ashtami, Vijaya Dashami, Diwali, Guru Nanak Jayanti | Done — each on the part of the day its tithi must hold, at the national almanac's sunrise; reproduces the Rashtriya Panchang's festival lists for 2023–2025 |
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
| China 🇨🇳 | Statutory holidays keyed to the `chinese` calendar, across the 1999, 2007 and 2024 revisions |
| Taiwan 🇹🇼 | Including the 2025 restoration of three commemorative holidays |
| South Korea 🇰🇷 | Keyed to the `dangi` calendar, with all three extensions of the 대체공휴일 and the collision rule |
| Canada 🇨🇦 | Federal plus the provincial days fixed by statute |
| Australia 🇦🇺 | National plus all six states and both territories |
| New Zealand 🇳🇿 | Including mondayisation from 2014, and Matariki over 2022–2035 — the Act schedules to 2052, and a calendar past 2035 reports it as a gap rather than dropping it |
| Brazil 🇧🇷 | Including Consciência Negra from 2024 |
| Mexico 🇲🇽 | Including the 2006 Monday reform and the six-yearly presidential handover |
| Saudi Arabia 🇸🇦 | Umm al-Qura based, with the 2013 weekend change |
| United Arab Emirates 🇦🇪 | With the 2022 move to a Saturday–Sunday weekend |
| Israel 🇮🇱 | Hebrew-dated and therefore exact, with Yom HaAtzmaut's Sabbath-avoidance rule |
| Iran 🇮🇷 | Civil holidays on the astronomical `persian` calendar, so exact — Nowruz 1404 on 21 March 2025, where the arithmetic cycle says the 20th; the lunar Hijri days flagged `Approximate`, since Iran declares them on its own sighting; Friday weekend |
| Hungary 🇭🇺 | With Good Friday from 2017; the annual rearrangement of working days by decree is not carried |
| Romania 🇷🇴 | Orthodox Easter and Pentecost by the Julian computus; the additions of the last decade by year, Epiphany and Saint John from 2024 |
| Russia 🇷🇺 | The New Year holidays as they grew to 1–8 January; no substitution, because the Government transfers days off by decree every year and the statutory default almost never applies |
| Argentina 🇦🇷 | From Decreto 1584/2010 in 2011: the *inamovibles* where they fall, the *trasladables* on the decree's Mondays to 2016 and by the weekday rule of Ley 27.399 from 2018; the annual tourist holidays not carried; Holy Thursday and the days of the Jewish and Islamic faiths as observances |
| Colombia 🇨🇴 | Ley 51 de 1983 from 1984: ten holidays to the following Monday, eight where they fall |
| Kenya 🇰🇪 | The Public Holidays Act's Part I with the Sunday rule of section 4; Idd-ul-Azha and Diwali as the religious days Parts II and III make them, not days off for all; the Idd days approximate |
| Morocco 🇲🇦 | The decrees' eleven fixed days and the four feasts, three of them two days, approximate; Yennayer from 2024 and Unity Day from 2026 by year |
| Pakistan 🇵🇰 | The state holidays, the Hijri ones approximate; Iqbal Day only in the years it was a holiday; the notification's extra Eid days not carried |
| Peru 🇵🇪 | The sixteen days of the 2026 list from 2024, where they fall; the laws of the four recent additions not read, so nothing before is stated |
| Ukraine 🇺🇦 | The 2023 list — 8 May, 15 July, 1 October, Christmas on 25 December alone — with the earlier dates by year; the next-working-day rule of article 67; the martial-law suspension of days off is noted, not modelled |
| Thailand 🇹🇭 | Including the Buddhist lunar holidays, flagged `Approximate` |
| Vietnam 🇻🇳 | Keyed to the `vietnamese` calendar |
| Indonesia 🇮🇩 | |
| Spain 🇪🇸, Italy 🇮🇹, Netherlands 🇳🇱, Poland 🇵🇱, Türkiye 🇹🇷, Egypt 🇪🇬, Nigeria 🇳🇬, South Africa 🇿🇦, Singapore 🇸🇬, Malaysia 🇲🇾, Philippines 🇵🇭, Switzerland 🇨🇭, Austria 🇦🇹, Belgium 🇧🇪, Sweden 🇸🇪, Norway 🇳🇴, Denmark 🇩🇰, Finland 🇫🇮, Portugal 🇵🇹, Greece 🇬🇷, Czechia 🇨🇿 | Core national list |

**Partial**

| Country | Note |
| --- | --- |
| India 🇮🇳 | The three national holidays and the gazetted list — Holi, Ram Navami, Mahavir Jayanti, Buddha Purnima, Janmashtami, Dussehra, Diwali and Guru Nanak's Birthday now computed on `hindu-lunar`, the Hijri days approximate |
| Myanmar 🇲🇲 | The full moons, National Day, the Kayin New Year and Deepavali on the Burmese calendar (`burmese`), Thingyan from the calendar's own akya and atat moments; the gazette's annual extensions not carried; Eid al-Adha approximate |
| Nepal 🇳🇵 | Only the days whose Bikram Sambat date maps to a near-fixed Gregorian one, each flagged `Approximate`. The table exists chiefly for the one-day weekend, which ran until April 2026 |

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
| Trading-day calendars for major exchanges | Planned |
| School terms | Out of scope — too local and too volatile |
| Name days and the sanctorale | Planned — as named authorities rather than one list, since the Roman calendar was recast in 1969 and the Swedish *namnsdagslängd* was revised in 1901, 1993 and 2001 ([policy.md](policy.md) §5) |
| Anniversaries and commemorations without a day off | Partial — `Kind::Observance` exists and a handful of tables use it. A general commemoration list is **out of scope** under [policy.md](policy.md) §10: no authority defines which commemorations belong, so its coverage could never be stated. Individual ones enter through whichever authority proclaims them |

## Adding a country

1. Add a `CountryRules` entry with its rules and a `sources_checked` date.
2. Cite the statute or the government gazette in a comment. A holiday without a
   source is a rumour.
3. Test at least five specific dates across at least two different years,
   including one that exercises the substitution rule — or, where the country
   has none, one that proves a weekend holiday stays where it falls.
4. Move its row to **Done**.
