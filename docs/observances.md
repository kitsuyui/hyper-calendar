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
Computed(fn)                  the few that really are bespoke
```

`WeekdayOnOrAfter`, `WeekdayOnOrBefore` and `Offset` were added while writing
the national tables; each is still a pure rule value, and each removed a
`Computed` that would otherwise have been needed.

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
- `Region` — subdivision scoping, for German *Länder*, US states, Swiss
  cantons, Canadian provinces.
- `Kind` — public holiday, bank holiday, school holiday, observance without a
  day off, religious day of obligation.
- `WeekendPolicy` — which days are the weekend, over stated years, because
  Saudi Arabia (2013), the United Arab Emirates (2022) and Nepal (2026) all
  changed theirs inside living memory.

Because the rule set is data, a caller can supply their own table — a company
calendar, a school year, a fictional setting — and get the same engine.

Only **six** rules in the whole crate are `Computed`: Ireland's St Brigid's
Day, the Dutch royal day, US Inauguration Day, Mexico's presidential handover,
Israel's Yom HaAtzmaut and New Zealand's statutory Matariki schedule. Japan
needs none.

## What the engine will not do

- **It will not claim a Hijri holiday is exact.** Eid dates depend on lunar
  crescent visibility decided per country, sometimes on the night before. The
  library returns the Umm al-Qura or tabular computation and flags it
  `Approximate`. It is a good prediction, not an announcement.
- **It will not invent substitution rules it cannot cite.** A country's
  weekend-substitution behaviour is law, and laws differ. Two dozen of the
  tables below carry no substitution policy at all, because their countries
  have none in calendar-expressible form.
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

The cross-cutting ones, because national tables depend on them.

| Tradition | Coverage | Status |
| --- | --- | --- |
| Christian, Gregorian computus | Easter and the feasts keyed to it — 22 offsets from Septuagesima to the Sacred Heart — plus Advent, Christmas, Epiphany, Candlemas, All Saints | Done |
| Christian, Julian computus (Orthodox) | Orthodox Pascha and its cycle, Julian-dated fixed feasts; the Julian date and the civil date are both available | Done |
| Islamic | Ras as-Sanah, Ashura, Mawlid, Isra and Miraj, Mid-Sha'ban, Ramadan, Laylat al-Qadr, Eid al-Fitr, Eid al-Adha, Day of Arafah | Done — flagged `Approximate` |
| Jewish | Rosh Hashanah, Fast of Gedaliah, Yom Kippur, Sukkot, Hoshana Rabbah, Shemini Atzeret, Simchat Torah, Hanukkah, Tenth of Tevet, Tu BiShvat, Purim, Shushan Purim, Passover, Lag BaOmer, Shavuot, Seventeenth of Tammuz, Tisha B'Av | Done — the Omer count itself lives in `hc-calendars-lunar::hebrew`; the Sabbath postponements of the minor fasts are not modelled |
| Buddhist | Vesak, Magha Puja, Asalha Puja, Vassa, Pavarana, Bodhi Day, Nirvana Day, Buddha's Birthday in both reckonings | Partial — the Theravada full moons are approximated from the Chinese lunisolar calendar and flagged `Approximate`; the Mahayana dates are exact |
| Chinese folk | 除夕, 春節, 元宵, 清明, 端午, 七夕, 中元, 中秋, 重陽, 臘八, 冬至 | Done |
| Hindu | Diwali, Holi, Navaratri, Dussehra, Ganesh Chaturthi, Janmashtami, Maha Shivaratri, Raksha Bandhan, Makar Sankranti, Ram Navami | Planned — needs `hindu-lunar` |
| Sikh | Vaisakhi, Guru Nanak Gurpurab, Hola Mohalla, Bandi Chhor Divas | Planned |
| Jain | Mahavir Jayanti, Paryushana, Diwali | Planned |
| Bahá'í | The nine holy days, the Fast, Ayyám-i-Há, Naw-Rúz | Planned |
| Zoroastrian | Nowruz, Mehregan, Yalda, Sadeh, the Gahambars | Planned |
| Shinto | 初詣, 節分, 七五三, and the major 祭 with fixed dates | Planned — 節分 and the other 雑節 are in `hc-seasons`, but no Shinto rule table exists yet |
| Pagan / Wheel of the Year | Samhain, Yule, Imbolc, Ostara, Beltane, Litha, Lughnasadh, Mabon | Planned |
| Secular international | UN observance days established by General Assembly resolution, and UNESCO international days | Planned — each entry cites its resolution |

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
| New Zealand 🇳🇿 | Including mondayisation from 2014 and the statutory Matariki schedule |
| Brazil 🇧🇷 | Including Consciência Negra from 2024 |
| Mexico 🇲🇽 | Including the 2006 Monday reform and the six-yearly presidential handover |
| Saudi Arabia 🇸🇦 | Umm al-Qura based, with the 2013 weekend change |
| United Arab Emirates 🇦🇪 | With the 2022 move to a Saturday–Sunday weekend |
| Israel 🇮🇱 | Hebrew-dated and therefore exact, with Yom HaAtzmaut's Sabbath-avoidance rule |
| Thailand 🇹🇭 | Including the Buddhist lunar holidays, flagged `Approximate` |
| Vietnam 🇻🇳 | Keyed to the `vietnamese` calendar |
| Indonesia 🇮🇩 | |
| Spain 🇪🇸, Italy 🇮🇹, Netherlands 🇳🇱, Poland 🇵🇱, Türkiye 🇹🇷, Egypt 🇪🇬, Nigeria 🇳🇬, South Africa 🇿🇦, Singapore 🇸🇬, Malaysia 🇲🇾, Philippines 🇵🇭, Switzerland 🇨🇭, Austria 🇦🇹, Belgium 🇧🇪, Sweden 🇸🇪, Norway 🇳🇴, Denmark 🇩🇰, Finland 🇫🇮, Portugal 🇵🇹, Greece 🇬🇷, Czechia 🇨🇿 | Core national list |

**Partial**

| Country | Note |
| --- | --- |
| India 🇮🇳 | The three national holidays plus the gazetted days this crate can compute. Holi, Diwali, Dussehra, Janmashtami, Mahavir Jayanti and Guru Nanak's Birthday await `hindu-lunar` |
| Nepal 🇳🇵 | Only the days whose Bikram Sambat date maps to a near-fixed Gregorian one, each flagged `Approximate`. The table exists chiefly for the one-day weekend, which ran until April 2026 |

**Planned** — Russia 🇷🇺, Ukraine 🇺🇦, Hungary 🇭🇺, Romania 🇷🇴 and Argentina 🇦🇷
first, then every remaining UN member state and observer, plus the
subdivisions that have their own legal holidays. Tracked as one issue per
country so that each lands with a citable source. Argentina waits on a rule
shape for Decreto 1584/2010, which moves a holiday to the Monday before or
after depending on the weekday it falls on; Russia and Ukraine wait on a
source the author could read in the original.

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
