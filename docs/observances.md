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
FixedInCalendar { cal, m, d } Eid al-Fitr, 1 Shawwal in islamic-umalqura
                              Rosh Hashanah, 1 Tishrei in hebrew
SolarTerm { term }            春分の日 / 秋分の日, the equinoxes
                              Nowruz, the March equinox at Tehran
EasterRelative { offset }     Good Friday (-2), Easter Monday (+1)
                              Ash Wednesday (-46), Pentecost (+49)
LunarPhase { phase, after }   Vesak, the full moon of Vesakha
Computed(fn)                  the few that really are bespoke
```

Rules then pass through **observance modifiers**, which are themselves data:

- `SubstituteIfWeekend` — the UK and US "observed" shift, Japan's 振替休日.
- `BridgeBetween` — Japan's 国民の休日 (a working day trapped between two
  holidays becomes one).
- `ValidFrom` / `ValidUntil` — a holiday that was created or abolished. Every
  rule carries these, because "is today a holiday in Japan" has a different
  answer in 1990 and 2020 and a library that ignores that is wrong for history.
- `Region` — subdivision scoping, for German *Länder*, US states, Swiss
  cantons, Canadian provinces.
- `Kind` — public holiday, bank holiday, school holiday, observance without a
  day off, religious day of obligation.

Because the rule set is data, a caller can supply their own table — a company
calendar, a school year, a fictional setting — and get the same engine.

## What the engine will not do

- **It will not claim a Hijri holiday is exact.** Eid dates depend on lunar
  crescent visibility decided per country, sometimes on the night before. The
  library returns the Umm al-Qura or tabular computation and flags it
  `Approximate`. It is a good prediction, not an announcement.
- **It will not invent substitution rules it cannot cite.** A country's
  weekend-substitution behaviour is law, and laws differ.
- **It will not pretend a holiday list is current.** Every country table
  carries the date its sources were checked.

## Stage 1 — Religious and traditional cycles

The cross-cutting ones, because national tables depend on them.

| Tradition | Coverage | Status |
| --- | --- | --- |
| Christian, Gregorian computus | Easter and the ~20 feasts keyed to it; Advent, Christmas, Epiphany, Candlemas, All Saints | Done |
| Christian, Julian computus (Orthodox) | Orthodox Easter and its cycle, Julian-dated fixed feasts | Done |
| Islamic | Ras as-Sanah, Ashura, Mawlid, Isra and Miraj, Ramadan, Laylat al-Qadr, Eid al-Fitr, Eid al-Adha, Day of Arafah | Done — flagged `Approximate` |
| Jewish | Rosh Hashanah, Yom Kippur, Sukkot, Shemini Atzeret, Simchat Torah, Hanukkah, Tu BiShvat, Purim, Passover, Lag BaOmer, Shavuot, Tisha B'Av, and the Omer count | Done |
| Buddhist | Vesak, Magha Puja, Asalha Puja, Vassa, Bodhi Day, Nirvana Day, Parinirvana | Partial — regional dating differs; Theravada and Mahayana variants separated |
| Hindu | Diwali, Holi, Navaratri, Dussehra, Ganesh Chaturthi, Janmashtami, Maha Shivaratri, Raksha Bandhan, Makar Sankranti, Ram Navami | Planned — needs `hindu-lunar` |
| Sikh | Vaisakhi, Guru Nanak Gurpurab, Hola Mohalla, Bandi Chhor Divas | Planned |
| Jain | Mahavir Jayanti, Paryushana, Diwali | Planned |
| Bahá'í | The nine holy days, the Fast, Ayyám-i-Há, Naw-Rúz | Planned |
| Zoroastrian | Nowruz, Mehregan, Yalda, Sadeh, the Gahambars | Planned |
| Shinto | 初詣, 節分, 七五三, and the major 祭 with fixed dates | Partial |
| Chinese folk | 春節, 元宵, 清明, 端午, 七夕, 中元, 中秋, 重陽, 冬至, 除夕 | Done |
| Pagan / Wheel of the Year | Samhain, Yule, Imbolc, Ostara, Beltane, Litha, Lughnasadh, Mabon | Planned |
| Secular international | UN observance days, Earth Day, International Workers' Day, Pi Day | Planned |

## Stage 2 — National public holidays

Ordered by how well the sources can be cited, not by importance.

**Done**

| Country | Note |
| --- | --- |
| Japan 🇯🇵 | Complete from 1948 (祝日法) to the present, including every amendment: the 1973 振替休日, the 1985 国民の休日, ハッピーマンデー in 2000 and 2003, 海の日, 山の日, the 2019 即位礼正殿の儀 and the 2020–21 Olympic moves. 春分の日 and 秋分の日 are computed from the equinox, not tabulated. |
| United States 🇺🇸 | Federal holidays with the Saturday/Sunday observed rule, including Juneteenth from 2021 |
| United Kingdom 🇬🇧 | England and Wales, Scotland and Northern Ireland as separate regions, with royal one-offs |
| Germany 🇩🇪 | Federal plus all 16 *Länder* |
| France 🇫🇷 | Métropole plus Alsace-Moselle |
| China 🇨🇳 | Statutory holidays keyed to the `chinese` calendar |
| Taiwan 🇹🇼 | |
| South Korea 🇰🇷 | Including the substitute-holiday rules |
| India 🇮🇳 | Central government gazetted holidays |
| Canada 🇨🇦 | Federal plus provincial |
| Australia 🇦🇺 | Federal plus states |
| Brazil 🇧🇷 | |
| Saudi Arabia 🇸🇦 | Umm al-Qura based |
| Israel 🇮🇱 | |
| Thailand 🇹🇭 | Including the Buddhist lunar holidays |
| Vietnam 🇻🇳 | |
| Indonesia 🇮🇩 | |
| Mexico 🇲🇽 | |
| Spain 🇪🇸, Italy 🇮🇹, Netherlands 🇳🇱, Poland 🇵🇱, Russia 🇷🇺, Turkey 🇹🇷, Egypt 🇪🇬, Nigeria 🇳🇬, South Africa 🇿🇦, Argentina 🇦🇷, Singapore 🇸🇬, Malaysia 🇲🇾, Philippines 🇵🇭, New Zealand 🇳🇿, Switzerland 🇨🇭, Austria 🇦🇹, Belgium 🇧🇪, Sweden 🇸🇪, Norway 🇳🇴, Denmark 🇩🇰, Finland 🇫🇮, Ireland 🇮🇪, Portugal 🇵🇹, Greece 🇬🇷, Czechia 🇨🇿, Hungary 🇭🇺, Romania 🇷🇴, Ukraine 🇺🇦 | Core national list |

**Planned** — every remaining UN member state and observer, plus the
subdivisions that have their own legal holidays. Tracked as one issue per
country so that each lands with a citable source.

**Researching** — countries whose holiday dates are announced annually by
decree rather than fixed in law (much of the Gulf, parts of South Asia). These
will be supported as *predictions* with an explicit `Approximate` flag and a
hook for supplying the announced table.

## Stage 3 — Beyond public holidays

| Category | Status |
| --- | --- |
| Business-day calculation (weekend rules by country, including Friday–Saturday and Sunday-only weekends) | Done |
| Trading-day calendars for major exchanges | Planned |
| School terms | Out of scope — too local and too volatile |
| Name days (Catholic, Orthodox, Nordic) | Planned |
| Anniversaries and commemorations without a day off | Partial |

## Adding a country

1. Add a `CountryRules` entry with its rules and a `sources_checked` date.
2. Cite the statute or the government gazette in a comment. A holiday without a
   source is a rumour.
3. Test at least five specific dates across at least two different years,
   including one that exercises the substitution rule.
4. Move its row to **Done**.
